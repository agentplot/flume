use crate::{lock, Log};
use flume_seams::{HeldState, PlaceId, Store, Versioned, Written};
use std::collections::BTreeMap;
use std::sync::Mutex;

type Interleaved = Box<dyn FnOnce() + Send>;

/// The store seam: each place's held state with its version, written by
/// compare-and-set, and a hook that runs another act just before the next
/// write, as two commands acting at once do.
pub struct FakeStore {
    state: Mutex<StoreState>,
    before_write: Mutex<Vec<Interleaved>>,
    root: String,
    pub log: Log,
}

#[derive(Default)]
struct StoreState {
    documents: BTreeMap<PlaceId, Versioned>,
    /// The lines of each place's `made.jsonl`, which go with its folder.
    made: BTreeMap<PlaceId, Vec<String>>,
    next: u64,
}

impl Default for FakeStore {
    fn default() -> Self {
        FakeStore::at("/host/state/flume")
    }
}

impl FakeStore {
    /// A store whose places' folders lie under `root`.
    pub fn at(root: &str) -> FakeStore {
        FakeStore {
            state: Mutex::default(),
            before_write: Mutex::default(),
            root: root.into(),
            log: Log::default(),
        }
    }

    /// `act` runs once, just before the next write is decided.
    pub fn before_next_write(&self, act: impl FnOnce() + Send + 'static) {
        lock(&self.before_write).push(Box::new(act));
    }

    pub fn held(&self, place: &PlaceId) -> Option<HeldState> {
        lock(&self.state)
            .documents
            .get(place)
            .map(|versioned| versioned.state.clone())
    }

    pub fn places(&self) -> Vec<PlaceId> {
        lock(&self.state).documents.keys().cloned().collect()
    }

    /// A line written to the place's `made.jsonl`, as a stage's run
    /// records what it made (129).
    pub fn record_made(&self, place: &PlaceId, line: &str) {
        lock(&self.state)
            .made
            .entry(place.clone())
            .or_default()
            .push(line.to_string());
    }

    /// Whether the place's folder holds a line of what was made.
    pub fn holds_made(&self, place: &PlaceId) -> bool {
        lock(&self.state).made.contains_key(place)
    }

    /// The held state set as it stands, past every check, for a scenario
    /// that needs the store in a state the core would reach.
    pub fn put(&self, place: &PlaceId, held: HeldState) {
        let mut state = lock(&self.state);
        state.next += 1;
        let version = state.next;
        state.documents.insert(
            place.clone(),
            Versioned {
                version,
                state: held,
            },
        );
    }
}

impl Store for FakeStore {
    fn read(&self, place: &PlaceId) -> Option<Versioned> {
        self.log.push(format!("read {place}"));
        lock(&self.state).documents.get(place).cloned()
    }

    fn read_every(&self) -> Vec<Versioned> {
        self.log.push("read_every");
        lock(&self.state).documents.values().cloned().collect()
    }

    fn write(&self, place: &PlaceId, at: Option<u64>, next: Option<&HeldState>) -> Written {
        let hooks: Vec<Interleaved> = std::mem::take(&mut *lock(&self.before_write));
        for hook in hooks {
            hook();
        }
        let mut state = lock(&self.state);
        let current = state
            .documents
            .get(place)
            .map(|versioned| versioned.version);
        let what = if next.is_some() { "write" } else { "remove" };
        let at_text = at.map_or("new".to_string(), |version| version.to_string());
        if current != at {
            self.log.push(format!("{what} {place} at {at_text} moved"));
            return Written::Moved;
        }
        self.log.push(format!("{what} {place} at {at_text}"));
        state.next += 1;
        let version = state.next;
        match next {
            Some(held) => {
                state.documents.insert(
                    place.clone(),
                    Versioned {
                        version,
                        state: held.clone(),
                    },
                );
            }
            None => {
                state.documents.remove(place);
                state.made.remove(place);
            }
        }
        Written::Written { version }
    }

    fn folder(&self, place: &PlaceId) -> String {
        format!(
            "{}/{}",
            self.root,
            place.0.trim_start_matches('/').replace('/', "%")
        )
    }

    fn made(&self, place: &PlaceId) -> Vec<String> {
        self.log.push(format!("made {place}"));
        lock(&self.state)
            .made
            .get(place)
            .cloned()
            .unwrap_or_default()
    }
}

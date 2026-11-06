use crate::{lock, Log};
use flume_seams::{
    HeldState, HostDocument, HostKey, HostVersioned, PlaceId, Store, Versioned, Written,
};
use std::collections::BTreeMap;
use std::sync::Mutex;

type Interleaved = Box<dyn FnOnce() + Send>;

/// A test of the held state a read hands out.
type Holds = Box<dyn Fn(&HeldState) -> bool + Send>;

/// The store seam: each place's held state with its version, written by
/// compare-and-set, the files written into its folder, the host's own
/// documents in memory beside them, and hooks that run another act just
/// before the next write, or just after a read hands out a held state a
/// test holds of, as two commands acting at once do.
pub struct FakeStore {
    state: Mutex<StoreState>,
    before_write: Mutex<Vec<Interleaved>>,
    after_read: Mutex<Vec<(Holds, Interleaved)>>,
    root: String,
    pub log: Log,
}

#[derive(Default)]
struct StoreState {
    documents: BTreeMap<PlaceId, Versioned>,
    /// The lines of each place's `made.jsonl`, which go with its folder.
    made: BTreeMap<PlaceId, Vec<String>>,
    /// The files written into each place's folder, by path, each with its
    /// text and its mode, which go with the folder (196).
    files: BTreeMap<PlaceId, BTreeMap<String, (String, u32)>>,
    /// The host's own documents, by key (153).
    host: BTreeMap<HostKey, HostVersioned>,
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
            after_read: Mutex::default(),
            root: root.into(),
            log: Log::default(),
        }
    }

    /// `act` runs once, just before the next write is decided.
    pub fn before_next_write(&self, act: impl FnOnce() + Send + 'static) {
        lock(&self.before_write).push(Box::new(act));
    }

    /// `act` runs once, just after the first read of a place that hands
    /// out a held state `when` holds of, so the reader goes on with what
    /// it read while `act` has moved the place on.
    pub fn after_read_of(
        &self,
        when: impl Fn(&HeldState) -> bool + Send + 'static,
        act: impl FnOnce() + Send + 'static,
    ) {
        lock(&self.after_read).push((Box::new(when), Box::new(act)));
    }

    /// Takes out and runs the first hook whose test holds of `held`.
    fn read_out(&self, held: &HeldState) {
        let act = {
            let mut hooks = lock(&self.after_read);
            hooks
                .iter()
                .position(|(when, _)| when(held))
                .map(|at| hooks.remove(at).1)
        };
        if let Some(act) = act {
            act();
        }
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

    /// The text of the file at `path` in a place's folder and its mode,
    /// as the last write left it; none where none was written (196).
    pub fn file(&self, path: &str) -> Option<(String, u32)> {
        lock(&self.state)
            .files
            .values()
            .find_map(|files| files.get(path).cloned())
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

impl FakeStore {
    /// The host's document under `key` as it stands, past the log.
    pub fn host_document(&self, key: &HostKey) -> Option<HostDocument> {
        lock(&self.state)
            .host
            .get(key)
            .map(|versioned| versioned.document.clone())
    }

    /// The host's document under `key` set as it stands, past every
    /// check, for a scenario that needs it so.
    pub fn put_host(&self, key: &HostKey, document: HostDocument) {
        let mut state = lock(&self.state);
        state.next += 1;
        let version = state.next;
        state
            .host
            .insert(key.clone(), HostVersioned { version, document });
    }
}

impl Store for FakeStore {
    fn read(&self, place: &PlaceId) -> Option<Versioned> {
        self.log.push(format!("read {place}"));
        let read = lock(&self.state).documents.get(place).cloned();
        if let Some(read) = &read {
            self.read_out(&read.state);
        }
        read
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
                state.files.remove(place);
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

    /// Refuses a path outside the place's folder, and otherwise keeps the
    /// text whole, readable by the host's user alone (196).
    fn write_file(&self, place: &PlaceId, path: &str, text: &str) -> Result<(), String> {
        self.log.push(format!("write_file {place} {path}"));
        let under = format!("{}/", self.folder(place));
        if !path.starts_with(&under) || path.split('/').any(|part| part == "..") {
            return Err(format!("{path} lies outside the place's folder"));
        }
        lock(&self.state)
            .files
            .entry(place.clone())
            .or_default()
            .insert(path.to_string(), (text.to_string(), 0o600));
        Ok(())
    }

    fn read_host(&self, key: &HostKey) -> Option<HostVersioned> {
        self.log.push(format!("read_host {key}"));
        lock(&self.state).host.get(key).cloned()
    }

    fn write_host(&self, key: &HostKey, at: Option<u64>, next: Option<&HostDocument>) -> Written {
        let mut state = lock(&self.state);
        let current = state.host.get(key).map(|versioned| versioned.version);
        let what = if next.is_some() {
            "write_host"
        } else {
            "remove_host"
        };
        let at_text = at.map_or("new".to_string(), |version| version.to_string());
        if current != at {
            self.log.push(format!("{what} {key} at {at_text} moved"));
            return Written::Moved;
        }
        self.log.push(format!("{what} {key} at {at_text}"));
        state.next += 1;
        let version = state.next;
        match next {
            Some(document) => {
                state.host.insert(
                    key.clone(),
                    HostVersioned {
                        version,
                        document: document.clone(),
                    },
                );
            }
            None => {
                state.host.remove(key);
            }
        }
        Written::Written { version }
    }
}

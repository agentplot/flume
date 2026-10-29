//! A place's held state, read with its version and written at it (73 to
//! 76, 102, 103): every act reads, decides and writes at the version it
//! read, and a write refused because the version moved is read again and
//! decided again, so two acts on one place leave it as one or the other
//! would have (75).

use crate::Engine;
use flume_seams::{HeldState, PlaceId, Versioned, Written};

/// What an act decided of a place's held state.
pub enum Step<T> {
    /// Write the state as the act left it, then answer `T`.
    Write(T),
    /// Write nothing, and answer `T`.
    Keep(T),
}

/// How many times an act reads again after a refused write before it
/// gives up.
const TRIES: usize = 32;

impl Engine {
    /// A place's held state and the version it was read at (102).
    pub fn held(&self, place: &PlaceId) -> Option<Versioned> {
        self.store.read(place)
    }

    /// A new place's first held state, written only if the place has
    /// none: its version, or none when held state stands (75, 103).
    pub fn create(&self, state: &HeldState) -> Option<u64> {
        match self.store.write(&state.place.id, None, Some(state)) {
            Written::Written { version } => Some(version),
            Written::Moved => None,
        }
    }

    /// Reads the place's held state, lets `decide` change it, and writes
    /// it at the version read; a refused write reads again and decides
    /// again. None when the place has no held state.
    pub fn update<T>(
        &self,
        place: &PlaceId,
        mut decide: impl FnMut(&mut HeldState) -> Step<T>,
    ) -> Option<T> {
        for _ in 0..TRIES {
            let read = self.store.read(place)?;
            let mut next = read.state.clone();
            match decide(&mut next) {
                Step::Keep(answer) => return Some(answer),
                Step::Write(answer) => {
                    if next == read.state {
                        return Some(answer);
                    }
                    self.stamp(&read.state, &mut next);
                    match self.store.write(place, Some(read.version), Some(&next)) {
                        Written::Written { .. } => return Some(answer),
                        Written::Moved => continue,
                    }
                }
            }
        }
        None
    }

    /// Stamps since when each stage stands in its state and the place
    /// waits for its caller, wherever an act changed them, so status says
    /// since when a place waits (56, 130).
    fn stamp(&self, was: &HeldState, next: &mut HeldState) {
        let now = self.host.now();
        for (index, stage) in next.stages.iter_mut().enumerate() {
            if was.stages.get(index).map(|was| was.state) != Some(stage.state) {
                stage.since = Some(now);
            }
        }
        let submission = &mut next.submission;
        if submission.waiting.is_none() {
            submission.waiting_since = None;
        } else if submission.waiting != was.submission.waiting {
            submission.waiting_since = Some(now);
        }
    }

    /// Removes the place's held state, and its folder with it, at the
    /// version read (62, 103).
    pub fn remove_held(&self, place: &PlaceId) -> bool {
        for _ in 0..TRIES {
            let Some(read) = self.store.read(place) else {
                return true;
            };
            if let Written::Written { .. } = self.store.write(place, Some(read.version), None) {
                return true;
            }
        }
        false
    }
}

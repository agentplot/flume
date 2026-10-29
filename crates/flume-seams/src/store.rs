//! The store seam's answers (102 to 104).

use crate::HeldState;
use serde::{Deserialize, Serialize};

/// A place's held state with the version it was read at (102).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Versioned {
    pub version: u64,
    pub state: HeldState,
}

/// write: done at a new version, or refused because the place's version
/// moved, or because a first write found held state already (103).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Written {
    Written { version: u64 },
    Moved,
}

/// The store seam (B.7), kept once for the host. The guarantee: it holds
/// nothing in a repository's content, and every flume process on the
/// host reaches the same store (104).
pub trait Store: Send + Sync {
    /// The place's held state and the version it was read at (102).
    fn read(&self, place: &crate::PlaceId) -> Option<Versioned>;

    /// Every place's held state (102).
    fn read_every(&self) -> Vec<Versioned>;

    /// The place's next held state, all or nothing, only if its held
    /// state is still at `at`; `at` none is a new place's first write,
    /// which succeeds only if it has none. `next` none removes the held
    /// state and the place's folder (103, 62).
    fn write(&self, place: &crate::PlaceId, at: Option<u64>, next: Option<&HeldState>) -> Written;

    /// The place's own folder beside its held state, for what its stages
    /// write for flume (123, design 2).
    fn folder(&self, place: &crate::PlaceId) -> String;

    /// Every line of `made.jsonl` in the place's own folder, in order and
    /// as written, where a stage's run records what it made; none where
    /// the file is absent. The lines go with the folder (128, 129, 62).
    fn made(&self, place: &crate::PlaceId) -> Vec<String>;
}

/// The name of the file in a place's own folder that a stage's run
/// records what it made in, one line per thing (129).
pub const MADE: &str = "made.jsonl";

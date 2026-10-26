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

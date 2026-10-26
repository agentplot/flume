//! The place seam's answers (78 to 83).

use crate::Revision;
use serde::{Deserialize, Serialize};

/// A place as the place seam is handed it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceRef {
    pub repository: String,
    pub location: String,
    pub branch: String,
    pub base: String,
}

/// make: a worktree on a new branch, or a refusal because the branch or
/// the location already exists (78).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MakeOutcome {
    Made,
    BranchExists,
    LocationExists,
    Refused { reason: String },
}

/// One readiness step's outcome and its words (79).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadyStep {
    pub step: String,
    pub passed: bool,
    pub words: String,
}

/// ready: each step run in order, stopping at the first that fails (79).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Readiness {
    pub steps: Vec<ReadyStep>,
}

impl Readiness {
    /// The step that failed, if one did (12).
    pub fn failed(&self) -> Option<&ReadyStep> {
        self.steps.iter().find(|step| !step.passed)
    }
}

/// What a place holds that keeps it from being removed (65).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Holds {
    pub uncommitted: Vec<String>,
    pub untracked: Vec<String>,
    pub lock: Option<String>,
}

impl Holds {
    pub fn any(&self) -> bool {
        !self.uncommitted.is_empty() || !self.untracked.is_empty() || self.lock.is_some()
    }
}

/// read: a place's branch, its head and what it holds (80).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceRead {
    Live {
        branch: String,
        head: Revision,
        holds: Holds,
    },
    Gone,
    CouldNotRead {
        reason: String,
    },
}

/// remove: the place removed with its teardown run, or the step that
/// failed (81).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Removed {
    Removed,
    Refused { step: String, words: String },
}

/// remove-branch: the branch removed, or the commits it holds whose
/// change the base lacks (82).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchRemoved {
    Removed,
    Kept { commits: Vec<Revision> },
    Gone,
}

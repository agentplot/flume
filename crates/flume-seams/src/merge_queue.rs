//! The merge queue seam's answers (99, 101, 117, 125, 9).

use crate::Revision;
use serde::{Deserialize, Serialize};

/// The outcome of a submission the merge queue decided (117).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Landed {
        commit: Revision,
        place_removed: bool,
        branch_removed: bool,
    },
    Failed {
        words: String,
    },
    Withdrawn,
    Refused {
        reason: String,
    },
}

/// A submission's state as the merge queue records it (99).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmissionState {
    Waiting,
    UnderTest,
    Landed { commit: Revision },
    Failed { words: String },
    Withdrawn,
}

impl SubmissionState {
    pub fn open(&self) -> bool {
        matches!(self, SubmissionState::Waiting | SubmissionState::UnderTest)
    }
}

/// A branch offered to the merge queue for a target, and what the merge
/// queue records of it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Submission {
    pub id: String,
    pub branch: String,
    pub target: String,
    pub state: SubmissionState,
}

/// read-submission: the newest submission of a branch, whoever made it,
/// or none, or that the merge queue could not be read (99).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmissionRead {
    None,
    Found { submission: Submission },
    CouldNotRead { reason: String },
}

/// Where the base's current revision lives, or why it cannot be read (9,
/// core.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaseRevision {
    At { revision: Revision },
    Refused { reason: String },
}

/// The merge queue seam (B.6), bound once for each repository. The
/// guarantee: landed is said only of a submission of that branch that
/// the merge queue itself landed, never inferred from what the base
/// holds, and submit removes a place only after its landing and only
/// through its repository's teardown (101).
pub trait MergeQueue: Send + Sync {
    /// The place's branch at `head` submitted to its base, and a wait
    /// until the merge queue decides (117).
    fn submit(&self, place: &crate::PlaceRef, head: &Revision) -> Verdict;

    /// The newest submission of `branch`, whoever made it (99).
    fn read_submission(&self, repository: &str, branch: &str) -> SubmissionRead;

    /// A wait on a submission still open until the merge queue decides,
    /// which keeps the queue moving (125).
    fn wait(&self, repository: &str, submission: &Submission) -> Verdict;

    /// Where `base`'s current revision lives, where work lands on it (9,
    /// core.7).
    fn base_revision(&self, repository: &str, base: &str) -> BaseRevision;
}

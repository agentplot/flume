//! The merge queue seam's answers (99, 101, 117, 125, 126, 133, 9).

use crate::Revision;
use serde::{Deserialize, Serialize};

/// The outcome of a submission as an answer reports it (117, 133).
/// `place_removed` and `branch_removed` are always false under a merge
/// queue that removes nothing of a place (113); they stay, since an
/// answer's form only gains fields (61).
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
    /// The submission conflicted with its base, naming the files
    /// contested (112).
    Conflict {
        files: Vec<String>,
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
    Landed {
        commit: Revision,
    },
    Failed {
        words: String,
    },
    /// It conflicted with its base, naming the files contested, and the
    /// base's revision it was tested against where the merge queue
    /// records one (112).
    Conflict {
        files: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        base: Option<Revision>,
        words: String,
    },
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
    /// The request it was taken in from, where the merge queue takes the
    /// base's work as requests, a forge's pull request by its number:
    /// its outcome is delivered to the request, so it is read from the
    /// merge queue's record as well as the place's inbox (124, 133, C.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
}

/// submit: the submission once the merge queue holds it, with no wait on
/// its verdict, or why the merge queue would not take it (117).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Submitted {
    Submitted {
        submission: Submission,
    },
    /// The merge queue opened or updated a request for the place's branch,
    /// a forge's pull request by its number, and holds no entry for it
    /// yet: an intake takes the request in later, on whichever host takes
    /// the base's requests in, and the submission is open and undecided
    /// until then (124, C.6).
    Requested {
        request: String,
    },
    /// The merge queue decided the submission as it took it in, and its
    /// answer gave the outcome, which it records delivered to the place,
    /// so no inbox gives it again (112, 133).
    Decided {
        submission: Submission,
        outcome: SubmissionOutcome,
    },
    Refused {
        reason: String,
    },
}

/// One outcome of a place's submission, delivered to the place once
/// (133). A withdrawal is never one: it is read from read-submission
/// (99).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmissionOutcome {
    Landed {
        commit: Revision,
    },
    Failed {
        words: String,
    },
    /// The files contested, the base's revision the merge queue tested
    /// against, and its words (112).
    Conflict {
        files: Vec<String>,
        base: Revision,
        words: String,
    },
}

/// inbox: the outcomes pending for a place, each given once, or why the
/// merge queue could not be read (133, 68).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Inbox {
    Outcomes { outcomes: Vec<SubmissionOutcome> },
    CouldNotRead { reason: String },
}

/// step: one round of the base's queue driven, or that the merge queue
/// moves on its own, or why it could not be read (125).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stepped {
    Stepped,
    MovesOnItsOwn,
    CouldNotRead { reason: String },
}

/// read-submission: the newest submission of a branch, whoever made it,
/// one taken in from a request opened from the branch among them, or
/// none, or that the merge queue could not be read (99).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmissionRead {
    None,
    Found { submission: Submission },
    CouldNotRead { reason: String },
}

/// Where the base's current revision lives, or why it cannot be read (9,
/// 126, core.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaseRevision {
    At { revision: Revision },
    Refused { reason: String },
}

/// The merge queue seam (B.6), bound once for each repository. The
/// guarantee: landed is said only of a submission of that branch that
/// the merge queue itself landed, never inferred from what the base
/// holds, and the merge queue removes nothing of a place (101, 113).
pub trait MergeQueue: Send + Sync {
    /// The place's branch at `head` submitted to its base, answered once
    /// the merge queue holds it, with no wait on its verdict (117).
    fn submit(&self, place: &crate::PlaceRef, head: &Revision) -> Submitted;

    /// The outcomes of the place's submissions pending for it, each
    /// given once (133).
    fn inbox(&self, place: &crate::PlaceRef) -> Inbox;

    /// The newest submission of `branch`, whoever made it (99).
    fn read_submission(&self, repository: &str, branch: &str) -> SubmissionRead;

    /// One round of `base`'s queue, so it moves where nothing else
    /// drives it (125).
    fn step(&self, repository: &str, base: &str) -> Stepped;

    /// Where `base`'s current revision lives, where work lands on it (9,
    /// 126, core.7).
    fn base_revision(&self, repository: &str, base: &str) -> BaseRevision;
}

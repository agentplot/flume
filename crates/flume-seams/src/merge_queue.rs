//! The merge queue seam's answers (99, 101, 117, 125, 126, 133, 9), and
//! the branches it opens and closes for base places (205, 206).

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

/// A branch the merge queue keeps that it could not read, passed over
/// with its words: what one branch says, or cannot, decides that branch
/// alone, and reads every other branch still (99, 133).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PassedOver {
    pub branch: String,
    pub words: String,
}

/// inbox: the outcomes pending for a place, each given once, from every
/// branch the merge queue could read, and each branch it passed over,
/// whose outcomes stay pending for a later inbox; or why the merge queue
/// could not be read (133, 68).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Inbox {
    Outcomes {
        outcomes: Vec<SubmissionOutcome>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        passed_over: Vec<PassedOver>,
    },
    CouldNotRead {
        reason: String,
    },
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

/// read-submission's answer across the branches the merge queue keeps:
/// what it read of the branch on every one it could read, and each one it
/// passed over with its words, which decides nothing of the others (99).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadAcross {
    pub read: SubmissionRead,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub passed_over: Vec<PassedOver>,
}

impl From<SubmissionRead> for ReadAcross {
    /// A read that passed no branch over.
    fn from(read: SubmissionRead) -> ReadAcross {
        ReadAcross {
            read,
            passed_over: vec![],
        }
    }
}

/// Where the base's current revision lives, or why it cannot be read, or
/// that the merge queue lands no such base, which the core then reads at
/// origin (9, 126, C.1, core.7).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaseRevision {
    At { revision: Revision },
    Refused { reason: String },
    Unlanded,
}

/// What a submission sends: the place's worktree head, from the place,
/// or a branch the merge queue lands on, named with its revision, from
/// the repository, with no worktree as its submitter (117, 203).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubmitHead {
    Worktree { head: Revision },
    Branch { branch: String, revision: Revision },
}

/// open: the branch made at its base's current revision as one the
/// merge queue lands on, and the revision it was made at; or why the
/// merge queue would not make it, in its words, nothing made (205).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchOpened {
    Made { revision: Revision },
    Refused { reason: String },
}

/// close: the branch removed into the branch its work landed in; or kept,
/// in the merge queue's words, a submission open on it or work of it its
/// base lacks among them; or already gone (204, 206).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchClosed {
    Removed,
    Kept { reason: String },
    Gone,
}

/// forecast: the conflicts a branch's head would meet were it submitted
/// to its base now, placed after every entry already submitted. A
/// forecast creates no entry, moves no branch, and is never read as a
/// verdict (203, 210).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Forecast {
    /// The head merges onto the base's revision `onto` and every entry
    /// ahead of it.
    Clear { head: Revision, onto: Revision },
    /// It does not: the files contested, the base's revision, each entry
    /// ahead it meets, none where it meets the base itself, and the merge
    /// queue's words.
    Conflict {
        head: Revision,
        onto: Revision,
        files: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        with: Vec<String>,
        words: String,
    },
    /// The merge queue could not form it, and why.
    CouldNotRead { reason: String },
}

impl Forecast {
    /// The head and the base's revision it was formed on, none where it
    /// could not be read.
    pub fn formed(&self) -> Option<(&Revision, &Revision)> {
        match self {
            Forecast::Clear { head, onto } | Forecast::Conflict { head, onto, .. } => {
                Some((head, onto))
            }
            Forecast::CouldNotRead { .. } => None,
        }
    }
}

/// The merge queue seam (B.6), bound once for each repository. The
/// guarantee: landed is said only of a submission of that branch that
/// the merge queue itself landed, never inferred from what the base
/// holds, and the merge queue removes nothing of a place (101, 113).
pub trait MergeQueue: Send + Sync {
    /// What `head` names submitted to the place's base, answered once the
    /// merge queue holds it, with no wait on its verdict: the place's
    /// branch at its worktree's head, from the place, or a branch named
    /// at its revision, from the repository (117, 203).
    fn submit(&self, place: &crate::PlaceRef, head: &SubmitHead) -> Submitted;

    /// The outcomes of the place's submissions pending for it, each
    /// given once (133).
    fn inbox(&self, place: &crate::PlaceRef) -> Inbox;

    /// The newest submission of each of `branches`, whoever made it, on
    /// the branches the merge queue could read, passing over each it
    /// could not, one answer for each branch in their order: the
    /// repository's records read once for them all, so a status of every
    /// place asks each repository once (58, 99, C.6).
    fn read_submissions(&self, repository: &str, branches: &[&str]) -> Vec<ReadAcross>;

    /// The newest submission of `branch` alone, as
    /// [`MergeQueue::read_submissions`] reads it (99).
    fn read_submission(&self, repository: &str, branch: &str) -> ReadAcross {
        self.read_submissions(repository, &[branch])
            .pop()
            .unwrap_or_else(|| {
                SubmissionRead::CouldNotRead {
                    reason: format!("the merge queue gave no answer for {branch}"),
                }
                .into()
            })
    }

    /// One round of `base`'s queue, so it moves where nothing else
    /// drives it (125).
    fn step(&self, repository: &str, base: &str) -> Stepped;

    /// Where `base`'s current revision lives, where work lands on it, or
    /// that the merge queue lands no such base (9, 126, core.7).
    fn base_revision(&self, repository: &str, base: &str) -> BaseRevision;

    /// `branch` made at `base`'s current revision as a branch the merge
    /// queue lands on, or refused in its words (205).
    fn open(&self, repository: &str, branch: &str, base: &str) -> BranchOpened;

    /// `branch`, one the merge queue lands on, removed into `into`, the
    /// branch its work landed in; or kept in its words, or gone (206).
    fn close(&self, repository: &str, branch: &str, into: &str) -> BranchClosed;

    /// The conflicts `branch`'s head would meet were it submitted to
    /// `base` now, placed after every entry already submitted; it creates
    /// no entry and moves nothing, and is never read as a verdict (203,
    /// 210).
    fn forecast(&self, repository: &str, branch: &str, base: &str) -> Forecast;
}

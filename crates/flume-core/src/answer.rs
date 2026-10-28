//! Every act's answer: one serde type, written as JSON, the same through
//! every surface, that only ever gains fields (60, 61). A refusal is an
//! answer of the same form (2).

use flume_seams::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Answer {
    /// Status of one place or every place (56).
    Status { places: Vec<PlaceStatus> },
    /// A launch's place, once its first stage is started, on hold or
    /// waiting (2, 6).
    Launched { place: PlaceStatus },
    /// The exact prompt a stage would be sent (29).
    Prompt {
        place: PlaceId,
        stage: String,
        text: String,
    },
    /// A caller's or an agent's act taken on a place.
    Acted {
        act: Act,
        place: PlaceId,
        status: Option<PlaceStatus>,
        tidy: Option<Tidied>,
    },
    /// The places a settle read, and what its tidies did (46).
    Settled { places: Vec<PlaceId>, tidy: Tidied },
    /// What a tidy removed and kept (64 to 67).
    Tidied { tidy: Tidied },
    /// The stop check's words, or none where the agent may go quiet
    /// (110).
    StopCheck { words: Option<String> },
    /// An act refused, naming why.
    Refused { refusal: Refusal },
}

impl Answer {
    pub fn refused(reason: impl Into<String>) -> Answer {
        Answer::Refused {
            refusal: Refusal {
                reason: reason.into(),
            },
        }
    }

    /// The refusal's reason, if the act was refused.
    pub fn refusal(&self) -> Option<&str> {
        match self {
            Answer::Refused { refusal } => Some(&refusal.reason),
            _ => None,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("an answer is always written")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Act {
    Retry,
    Stop,
    Drop,
    Done,
    Finish,
    Tell,
    Needs,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Refusal {
    pub reason: String,
}

/// Every fact of one place clause 56 lists, each read where it lives
/// when asked (58).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceStatus {
    pub place: PlaceId,
    pub repository: String,
    pub kind: String,
    pub version: u32,
    pub declared: Declared,
    pub name: String,
    pub branch: String,
    pub base: String,
    pub mark: Option<String>,
    pub lands: bool,
    pub phase: Phase,
    /// Each readiness step's outcome and words (12).
    pub readiness: Readiness,
    pub stages: Vec<StageStatus>,
    /// The stage the place most recently started.
    pub current: Option<String>,
    pub agent: Option<AgentStatus>,
    /// The branch's head the facts below were read at (25).
    pub head: Option<Revision>,
    /// Whether the current stage's deliverable exists.
    pub delivered: Option<bool>,
    /// The change's tasks and which are ticked.
    pub tasks: Option<Vec<Task>>,
    pub submission: Option<SubmissionStatus>,
    /// Each act of an agent's that flume refused (108).
    pub refused_acts: Vec<String>,
    /// What flume could not read, by what it is (31, 68).
    pub cannot_read: Vec<String>,
    /// Each end flume asked for that the agent did not take (31).
    pub failures: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageStatus {
    pub name: String,
    pub state: StageState,
    /// How many times the stage has started (48).
    pub starts: u32,
    /// How it ended, with what it delivered or the agent's words (43).
    pub end: Option<StageEnd>,
    /// The part a refused stage could not fill (27).
    pub refused_part: Option<String>,
    pub conversation: bool,
}

/// The current stage's agent (56).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentStatus {
    pub identity: String,
    pub stage: String,
    pub program: String,
    pub settings: Settings,
    pub state: AgentState,
    pub asking: Reading,
    pub own_work: Reading,
    /// Working past its stage's limit (118).
    pub past_limit: bool,
}

/// The place's submission as the merge queue holds it, and how often
/// flume submitted it (48, 56).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmissionStatus {
    pub read: SubmissionRead,
    pub submits: u32,
    pub send_backs: u32,
    /// The last outcome flume collected, or its refusal at submit, with
    /// the merge queue's words (133).
    pub verdict: Option<Verdict>,
    /// Why the place waits for its caller (112).
    pub waiting: Option<String>,
    /// The launch holds the submission (47).
    pub held: bool,
}

/// What a tidy removed and kept, a kept branch named once (67).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tidied {
    pub removed: Vec<PlaceId>,
    pub kept: Vec<KeptPlace>,
    pub branches_kept: Vec<BranchKept>,
    /// Places asked of whose work is not over.
    pub not_over: Vec<PlaceId>,
}

impl Tidied {
    pub fn add(&mut self, other: Tidied) {
        self.removed.extend(other.removed);
        self.kept.extend(other.kept);
        self.branches_kept.extend(other.branches_kept);
        self.not_over.extend(other.not_over);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeptPlace {
    pub place: PlaceId,
    pub why: Kept,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchKept {
    pub place: PlaceId,
    pub branch: String,
    pub commits: Vec<Revision>,
}

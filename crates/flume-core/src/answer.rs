//! Every act's answer: one serde type, written as JSON, the same through
//! every surface, that only ever gains fields (60, 61). A refusal is an
//! answer of the same form (2).

use flume_seams::*;
pub use flume_seams::{Act, WaitingOn, Waits};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Answer {
    /// Status of one place or every place, and of the host (56, 70).
    Status {
        places: Vec<PlaceStatus>,
        #[serde(default)]
        host: HostStatus,
    },
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
    /// A wait by place: the end of the stage's current start or of the
    /// submission it answers, delivered to the recipient by this wait or
    /// answered as a read of one already delivered; no end where the
    /// wait was still waiting when its timeout passed (43, design 9).
    Waited {
        place: PlaceId,
        recipient: String,
        end: Option<EndStatus>,
        /// This wait delivered the end; false for a read.
        delivered: bool,
    },
    /// Every end pending for the recipient that the inbox collected and
    /// delivered, of every place on the host, tidied places kept for
    /// their ends among them (43, design 9).
    Inbox {
        recipient: String,
        ends: Vec<Collected>,
    },
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

    /// The exit a command gives the answer (design 9): 1 for a refusal;
    /// for a wait, 0 for a stage delivered or finished or a submission
    /// landed, 2 for any other end, and 3 for still waiting at its
    /// timeout; 0 for every other answer.
    pub fn exit(&self) -> u8 {
        match self {
            Answer::Refused { .. } => 1,
            Answer::Waited { end: None, .. } => 3,
            Answer::Waited { end: Some(end), .. } => match &end.end.of {
                EndOf::Stage {
                    state: StageState::Delivered | StageState::Finished,
                    ..
                }
                | EndOf::Submission {
                    verdict: Verdict::Landed { .. },
                    ..
                } => 0,
                _ => 2,
            },
            _ => 0,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("an answer is always written")
    }
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
    /// The recipients of the place's ends, as its launch fixed them (7,
    /// 8, 43).
    pub recipients: Vec<String>,
    /// The words of each hold the launch put, by stage name (7, 130).
    pub hold_words: BTreeMap<String, String>,
    /// The move the place waits on its caller for, and since when; none
    /// where it waits on nobody (56, 130).
    pub waiting_on: Option<WaitingOn>,
    /// The place waited on its caller under no hold's words past its
    /// bound, read so by a settle (130).
    #[serde(default)]
    pub stalled: bool,
    /// Each hold on a stage or the submission, with its words and since
    /// when (47, 130).
    #[serde(default)]
    pub holds: Vec<Hold>,
    /// The place's priority, its caller's (7, 70).
    #[serde(default)]
    pub priority: i64,
    /// Every end of the place, with whom it was delivered to and when,
    /// and for whom it stands pending (43).
    #[serde(default)]
    pub ends: Vec<EndStatus>,
}

/// An end of a place as status gives it: what ended and what was read of
/// it, each delivery, and the recipients it stands pending for (43).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndStatus {
    #[serde(flatten)]
    pub end: EndHeld,
    pub pending_for: Vec<String>,
}

/// An end the inbox collected: the place it is of, by its location and
/// as its agents name it, and the end whole with its deliveries, this
/// one among them (43).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Collected {
    pub place: PlaceId,
    pub kind: String,
    pub name: String,
    pub end: EndStatus,
}

/// The host's agents running, its bound, the room left and the stages
/// waiting for room in the order they would start (56, 70); a host that
/// states no bound has no room to judge and no queue.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostStatus {
    pub running: u32,
    pub bound: Option<u32>,
    pub room: Option<u32>,
    pub queue: Vec<Queued>,
    /// What the terminal made and keeps for what it did not make, each
    /// named with what holds it (56, 137, 138).
    #[serde(default)]
    pub terminal_kept: Vec<String>,
}

/// A stage waiting for room on the host: it waits on the host, never on
/// its caller (70, 130).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Queued {
    pub place: PlaceId,
    pub stage: String,
    pub priority: i64,
    pub since: Option<Seconds>,
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
    /// The setting a stage was refused at its start for, with its layer
    /// (27, 32).
    #[serde(default)]
    pub refused_setting: Option<String>,
    pub conversation: bool,
}

/// The current stage's agent (56).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentStatus {
    pub identity: String,
    pub stage: String,
    pub program: String,
    pub settings: AgentSettings,
    pub state: AgentState,
    /// Whether the agent said it started under its start's identity
    /// (135).
    #[serde(default = "unknown")]
    pub started: Reading,
    pub asking: Reading,
    pub own_work: Reading,
    /// Working past its stage's limit (118).
    pub past_limit: bool,
    /// While the start is starting, what the terminal shows of the agent
    /// (35, 136).
    #[serde(default)]
    pub screen: Option<String>,
    /// The command a person runs in the place to start the agent, where
    /// the terminal printed it (C.5).
    #[serde(default)]
    pub printed: Option<String>,
    /// The agent's last words as its own record holds them, a failed
    /// turn's reason among them (43).
    #[serde(default)]
    pub last_words: Option<String>,
    /// What the agent's own record says of its start that no reading
    /// answers, a plugin of another version than flume's among it (135).
    #[serde(default)]
    pub notes: Vec<String>,
}

/// The settings an agent started with, each value as its layer wrote it,
/// and where each came from (32, 56).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSettings {
    #[serde(flatten)]
    pub settings: Settings,
    /// Each setting's layer and value as written, by `program`, `model`,
    /// `instructions` or `env.<NAME>`; a host's value before its `~` and
    /// variables are read.
    #[serde(default)]
    pub from: BTreeMap<String, Given>,
}

fn unknown() -> Reading {
    Reading::Unknown
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
    /// The submission is on hold (47).
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
    /// What the terminal closed and kept of what it made for each place
    /// removed (137, 138).
    #[serde(default)]
    pub terminal: Vec<TerminalTidied>,
}

impl Tidied {
    pub fn add(&mut self, other: Tidied) {
        self.removed.extend(other.removed);
        self.kept.extend(other.kept);
        self.branches_kept.extend(other.branches_kept);
        self.not_over.extend(other.not_over);
        self.terminal.extend(other.terminal);
    }
}

/// What the terminal closed of what it made for a place removed, and what
/// it kept, each named with what holds it (137, 138).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalTidied {
    pub place: PlaceId,
    pub closed: Vec<String>,
    pub kept: Vec<String>,
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

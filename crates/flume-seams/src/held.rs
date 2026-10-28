//! A place's held state: one document per place in the store (62, 73 to
//! 76, 103), enough on its own that a restart loses nothing (77). It
//! keeps no fact of a deliverable, a change or a submission between asks
//! (58); a stage's end keeps only what was reported at that moment (43).

use crate::{
    Binding, Declared, Holds, Identity, Kind, PlaceId, PlaceRef, Prompt, Readiness, Revision,
    Seconds, Secret, Settings, StopKind, Verdict,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeldState {
    pub place: PlaceRecord,
    pub phase: Phase,
    /// Each readiness step's outcome and words (12, 79).
    pub readiness: Readiness,
    pub stages: Vec<StageHeld>,
    /// The stage the place most recently started.
    pub current: Option<usize>,
    pub submission: SubmissionHeld,
    /// Each end flume asked for that the agent did not take (31).
    pub failures: Vec<String>,
    /// The place's priority, higher first: its launch's, changed by its
    /// caller after (7, 70).
    #[serde(default)]
    pub priority: i64,
}

/// What a launch fixed of a place (8, 21).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceRecord {
    pub id: PlaceId,
    pub repository: String,
    /// The kind as it resolved at the launch, its stages with it (21).
    pub kind: Kind,
    pub declared: Declared,
    pub name: String,
    pub branch: String,
    pub base: String,
    /// The base's revision the place was cut from (9).
    pub base_revision: Revision,
    pub location: String,
    /// The repository's binding as it resolved at the launch (21).
    pub binding: Binding,
    pub brief: Option<String>,
    pub mark: Option<String>,
    pub settings: BTreeMap<String, Settings>,
    pub hold: Vec<String>,
    pub hold_submission: bool,
    /// The words of each hold the launch put, by stage name (7, 130).
    #[serde(default)]
    pub hold_words: BTreeMap<String, String>,
    /// The recipients of the place's ends, as the launch named them (7,
    /// 8, 43).
    #[serde(default)]
    pub recipients: Vec<String>,
}

impl PlaceRecord {
    pub fn place_ref(&self) -> PlaceRef {
        PlaceRef {
            repository: self.repository.clone(),
            location: self.location.clone(),
            branch: self.branch.clone(),
            base: self.base.clone(),
        }
    }

    /// The place as its agents' identities name it, `<kind>--<name>`.
    pub fn short(&self) -> String {
        format!("{}--{}", self.kind.name, self.name)
    }
}

/// Where a place stands as a whole.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    /// Being made by a launch, the command named.
    Making {
        by: String,
    },
    Live,
    /// Its work is over (63); `kept` names what keeps it from removal.
    Over {
        over: Over,
        kept: Option<Kept>,
    },
    /// Removed, its branch and held state still to go (66).
    Gone {
        over: Over,
    },
}

/// Why a place's work is over (63).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Over {
    Landed { commit: Revision },
    Dropped,
    Done,
}

/// Why a place whose work is over still stands (64 to 66, 68).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kept {
    Holds { holds: Holds },
    Teardown { step: String, words: String },
    AgentLive { identity: String },
    Unreadable { reason: String },
}

/// Where a stage stands (vocabulary: stage state).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageState {
    NotReached,
    OnHold,
    WaitingOnReadiness,
    /// Due to start, waiting for room on the host (70).
    WaitingForRoom,
    Refused,
    /// Its agent opened, and not yet said it started (35, 135).
    Starting,
    Started,
    Begun,
    NeverBegun,
    Stuck,
    Lost,
    Stopped,
    Delivered,
    EndedShort,
    Finished,
}

impl StageState {
    /// The stage ended: delivered, ended short or finished.
    pub fn ended(self) -> bool {
        matches!(
            self,
            StageState::Delivered | StageState::EndedShort | StageState::Finished
        )
    }

    /// The stage runs: its agent started and its end not yet read.
    pub fn running(self) -> bool {
        matches!(
            self,
            StageState::Starting | StageState::Started | StageState::Begun | StageState::Stuck
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageHeld {
    pub name: String,
    pub state: StageState,
    /// How many times the stage has started (48).
    pub starts: u32,
    /// The part a refused stage could not fill (27).
    pub refused_part: Option<String>,
    pub end: Option<StageEnd>,
    pub start: Option<StartHeld>,
    /// Since when the stage has stood in its state, so since when it
    /// waits, on its caller or for room on the host (56, 70, 130).
    #[serde(default, alias = "waiting_since")]
    pub since: Option<Seconds>,
    /// What a stage waiting for room starts with, and its place in the
    /// host's queue (70).
    #[serde(default)]
    pub room: Option<RoomWait>,
}

/// A stage waiting for room on the host (70).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomWait {
    /// The order it came to wait in, host-wide, so equals in priority
    /// start in the order they came to wait.
    pub order: u64,
    /// The words it starts with: a fix round's or a retry's (49, 112).
    pub words: Option<String>,
}

/// A stage's end as reported when it ended (43).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageEnd {
    pub head: Option<Revision>,
    pub report: Option<String>,
    /// A stop-short's words, whole (42).
    pub words: Option<String>,
    /// Whether the stop-short needs an answer or found work (41, 131).
    #[serde(default)]
    pub needs: Option<StopKind>,
    /// For a stuck, lost or never-begun end, the agent's last words as
    /// its own record held them (43).
    #[serde(default)]
    pub last_words: Option<String>,
    /// For a stuck, lost or never-begun end, the commits on the place's
    /// branch since the start (43).
    #[serde(default)]
    pub commits: Vec<Revision>,
    /// For a never-begun end, what the terminal showed of the agent (43).
    #[serde(default)]
    pub screen: Option<String>,
}

/// One start of a stage and its agent.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartHeld {
    pub n: u32,
    pub started_at: Seconds,
    /// The branch's head when the stage started (25).
    pub start_head: Revision,
    /// A retry's or a failed submission's words.
    pub words: Option<String>,
    pub identity: Identity,
    pub secret: Secret,
    pub program: String,
    pub settings: Settings,
    /// The agent's own record, once a reading names it (77).
    pub record: Option<String>,
    pub prompt: Prompt,
    /// Started and not yet read ended.
    pub live: bool,
    pub begun_at: Option<Seconds>,
    /// flume's first reading of the agent quiet with nothing delivered
    /// (44).
    pub first_quiet: Option<Seconds>,
    pub last_head: Revision,
    pub last_head_move: Seconds,
    /// Reported working past its limit (118).
    pub past_limit: bool,
    /// The command a person runs in the place to start the agent, where
    /// the terminal printed it rather than ran it (C.5).
    #[serde(default)]
    pub printed: Option<String>,
}

/// A landing kind's submission as flume keeps it, with no wait held on
/// it (48, 111, 112, 124, 133).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmissionHeld {
    /// The last stage delivered and its submission is to be made.
    pub due: bool,
    /// How many times flume submitted the place (48).
    pub submits: u32,
    /// How many send-backs the place has taken (112).
    pub send_backs: u32,
    /// The submission flume last made, by the merge queue's id, so a
    /// withdrawal of it is told from a hand's (99, 112).
    #[serde(default)]
    pub id: Option<String>,
    /// The last outcome flume collected, or its refusal at submit.
    pub verdict: Option<Verdict>,
    /// Why the place waits for its caller's retry or drop (112, 130).
    pub waiting: Option<String>,
    /// Since when it has waited so (56, 130).
    #[serde(default)]
    pub waiting_since: Option<Seconds>,
    /// Why the place's inbox could not be read at the last settle, which
    /// acted on nothing (133, 68).
    #[serde(default)]
    pub unread: Option<String>,
}

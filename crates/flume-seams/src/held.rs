//! A place's held state: one document per place in the store (62, 73 to
//! 76, 103), enough on its own that a restart loses nothing (77). It
//! keeps no fact of a deliverable, a change or a submission between asks
//! (58); a stage's end keeps only what was reported at that moment (43).

use crate::{
    Binding, Declared, Holds, Identity, Kind, PlaceId, PlaceRef, Prompt, Readiness, Revision,
    Seconds, Secret, Settings, Verdict,
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
    Refused,
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
            StageState::Started | StageState::Begun | StageState::Stuck
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
}

/// A stage's end as reported when it ended (43).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageEnd {
    pub head: Option<Revision>,
    pub report: Option<String>,
    /// A stop-short's words, whole (42).
    pub words: Option<String>,
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
}

/// A landing kind's submission as flume keeps it (48, 111, 112, 124).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubmissionHeld {
    /// The last stage delivered and its submission is to be made.
    pub due: bool,
    /// How many times flume submitted the place (48).
    pub submits: u32,
    /// How many send-backs the place has taken (112).
    pub send_backs: u32,
    /// The command of flume's waiting on the merge queue's verdict (124).
    pub waiter: Option<String>,
    pub verdict: Option<Verdict>,
    /// Why the place waits for its caller's retry or drop (112).
    pub waiting: Option<String>,
}

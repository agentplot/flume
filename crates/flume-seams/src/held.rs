//! A place's held state: one document per place in the store (62, 73 to
//! 76, 103), enough on its own that a restart loses nothing (77). It
//! keeps no fact of a deliverable, a change or a submission between asks
//! (58); a stage's end keeps only what was reported at that moment (43).

use crate::{
    Binding, Declared, Given, Holds, Identity, Kind, KindSettings, PlaceId, PlaceRef, Prompt,
    Readiness, Revision, Seconds, Secret, Settings, StopKind, StreamHeld, Unsettled, Verdict,
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
    /// The clues last shown for the place, so a settle shows them again
    /// only when they change (139); never read to decide anything.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shown: Option<String>,
    /// Every end the place has had, in order, each recorded once by its
    /// key with what flume read of it, and each delivery of it; an end is
    /// pending for every recipient it has no delivery to (43).
    #[serde(default)]
    pub ends: Vec<EndHeld>,
    /// The role's address `<partition>/<role>` a role's run serves, by
    /// which a command run in it collects for the address (132); none
    /// for any other place.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Each hold on a stage or the submission, the launch's and those put
    /// after it, with its words and since when; a retry naming its stage
    /// or the submission ends it, and nothing else does (47, 49, 130).
    #[serde(default)]
    pub holds: Vec<Hold>,
    /// The move the place waits on its caller for, since when, and
    /// whether a settle read it stalled; none where it waits on nobody
    /// (56, 130).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiting: Option<Waiting>,
    /// How long the place waits on its caller under no hold's words
    /// before it is stalled: the launch's bound, five minutes where it
    /// sets none (130).
    #[serde(default = "stall_after")]
    pub stall_after: Seconds,
    /// Why a role's address this place has an end pending for was not
    /// woken at the last settle, by the address: the configuration naming
    /// nothing for it, or the wake's launch refused; gone once it is
    /// woken (132; core.28).
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub unwoken: BTreeMap<String, String>,
    /// Each move of a stage of the place, or of its intent at a start of
    /// the place's, from one account to another, in order (56, 145 to
    /// 147).
    pub moves: Vec<AccountMove>,
    /// Each start of the place's whose agent began without the settings
    /// its start recorded, in order, by its variables' names alone (56,
    /// 161).
    pub unsettled: Vec<UnsettledStart>,
}

/// A start whose agent began without the settings its start recorded:
/// its stage and number, when a settle read it so, and each variable that
/// differed by its name and whether the program did (56, 135, 161).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsettledStart {
    pub stage: String,
    pub start: u32,
    pub at: Seconds,
    #[serde(flatten)]
    pub unsettled: Unsettled,
}

/// How long a place waits on its caller under no hold's words before it
/// is stalled, where its launch sets no bound (130).
pub const STALL_AFTER: Seconds = 5 * 60;

fn stall_after() -> Seconds {
    STALL_AFTER
}

impl HeldState {
    /// The recipients each end stands pending for, in order, each named
    /// once (43).
    pub fn pending_for(&self) -> Vec<String> {
        let mut pending: Vec<String> = vec![];
        for end in &self.ends {
            for recipient in end.pending(&self.place.recipients) {
                if !pending.contains(&recipient) {
                    pending.push(recipient);
                }
            }
        }
        pending
    }

    /// The hold on the stage `name`, if it is held (47).
    pub fn hold_on(&self, name: &str) -> Option<&Hold> {
        self.holds
            .iter()
            .find(|hold| matches!(&hold.on, HoldOn::Stage { stage } if stage == name))
    }

    /// The hold on the submission, if it is held (47).
    pub fn submission_hold(&self) -> Option<&Hold> {
        self.holds.iter().find(|hold| hold.on == HoldOn::Submission)
    }
}

/// A hold on a stage or the submission, with its caller's words and
/// since when (47, 130).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hold {
    #[serde(flatten)]
    pub on: HoldOn,
    /// What the hold waits for, in its caller's words; the caller's to
    /// make good, since flume watches no event for it (130).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub words: Option<String>,
    pub since: Seconds,
}

/// What a hold holds: a stage, or the submission (47).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "on", rename_all = "snake_case")]
pub enum HoldOn {
    Stage { stage: String },
    Submission,
}

/// The move a place waits on its caller for, as the settle last read it,
/// and whether it read the place stalled on it (56, 130).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Waiting {
    #[serde(flatten)]
    pub on: WaitingOn,
    #[serde(default)]
    pub stalled: bool,
}

/// The move a place waits on its caller for, why, since when, and the
/// words that go with it: a hold's, the question, or the merge queue's
/// (56, 130).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaitingOn {
    /// A retry, or done.
    pub act: Act,
    /// The stage the retry names; none where it names the submission.
    pub stage: Option<String>,
    pub why: Waits,
    pub since: Option<Seconds>,
    pub words: Option<String>,
}

impl WaitingOn {
    /// Whether it is the same move as `other`, whatever its words.
    pub fn same_move(&self, other: &WaitingOn) -> bool {
        self.act == other.act && self.stage == other.stage && self.why == other.why
    }

    /// The move, in the words a stall's notice gives it (130).
    pub fn move_words(&self) -> String {
        match (self.act, &self.stage) {
            (Act::Done, _) => "done".into(),
            (_, Some(stage)) => format!("a retry of {stage}"),
            (_, None) => "a retry of the submission".into(),
        }
    }
}

/// A caller's act on a place.
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
    Priority,
    Hold,
}

/// Why a place waits on its caller (56, 130).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Waits {
    /// A stage, or the submission, on hold (47).
    OnHold,
    /// A stage ended short needing an answer, or having found work
    /// outside the kind's last stage (42, 131).
    Answer,
    /// A stage whose agent was lost (45).
    Lost,
    /// A stage whose prompt was never begun (38).
    NeverBegun,
    /// A first stage waiting on the place's readiness (12).
    Unready,
    /// A stage whose prompt asks for a part with nothing to fill it
    /// (27).
    Refused,
    /// A stage whose agent the caller stopped (51).
    Stopped,
    /// The place went back as often as its kind allows (23, 112).
    PastBound,
    /// The merge queue withdrew or refused the submission (112).
    Submission,
    /// The last stage delivered, and the kind does not land (114).
    Delivered,
}

/// An end of a place, recorded once by its key when a settle or an act
/// reads it, with what 43 names of it, and delivered once to each
/// recipient (43, 74).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndHeld {
    /// What ended, which is the end's key with what was read of it.
    #[serde(flatten)]
    pub of: EndOf,
    /// When flume read it.
    pub read_at: Seconds,
    /// Each delivery of it, to whom, when and by what.
    #[serde(default)]
    pub deliveries: Vec<Delivery>,
}

impl EndHeld {
    /// Whether this end is the one `other` keys.
    pub fn same(&self, other: &EndOf) -> bool {
        self.of.key() == other.key()
    }

    /// Whether the end was delivered to `recipient`.
    pub fn delivered_to(&self, recipient: &str) -> bool {
        self.deliveries
            .iter()
            .any(|delivery| delivery.recipient == recipient)
    }

    /// The recipients of `recipients` it stands pending for (43).
    pub fn pending(&self, recipients: &[String]) -> Vec<String> {
        recipients
            .iter()
            .filter(|recipient| !self.delivered_to(recipient))
            .cloned()
            .collect()
    }
}

/// What ended: a stage's start, ended, lost, never begun or stuck; a
/// submission's outcome from the merge queue; or a stall, each with what
/// flume read of it (43, 45, 130, 133).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "of", rename_all = "snake_case")]
pub enum EndOf {
    Stage {
        stage: String,
        /// The start's number (48).
        start: u32,
        /// How the start ended: delivered, ended short, finished, lost,
        /// never begun or stuck.
        state: StageState,
        /// For a stuck start, the first quiet reading it ran from (44).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        quiet_from: Option<Seconds>,
        /// What was read of it: the head or report, the agent's words and
        /// whether it needs an answer or found work, or what flume read of
        /// an agent lost, never begun or stuck.
        end: StageEnd,
    },
    Submission {
        /// The submit's number (48).
        submit: u32,
        /// The outcome, the commit landed as or the merge queue's words.
        verdict: Verdict,
    },
    Stall {
        /// Since when the place waited on its caller (130).
        since: Seconds,
        /// The move it waits on.
        waiting: String,
    },
}

impl EndOf {
    /// The end's key: what ended, without what was read of it.
    pub fn key(&self) -> String {
        match self {
            EndOf::Stage {
                stage,
                start,
                state,
                quiet_from,
                ..
            } => format!(
                "stage {stage} {start} {state:?} {}",
                quiet_from.map(|at| at.to_string()).unwrap_or_default()
            ),
            EndOf::Submission { submit, verdict } => {
                let outcome = match verdict {
                    Verdict::Landed { .. } => "landed",
                    Verdict::Failed { .. } => "failed",
                    Verdict::Conflict { .. } => "conflict",
                    Verdict::Withdrawn => "withdrawn",
                    Verdict::Refused { .. } => "refused",
                };
                format!("submission {submit} {outcome}")
            }
            EndOf::Stall { since, .. } => format!("stall {since}"),
        }
    }
}

/// A delivery of an end to one recipient: to whom, when, and by what
/// (43).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Delivery {
    pub recipient: String,
    pub at: Seconds,
    pub by: DeliveredBy,
}

/// What delivered an end: a notice through the terminal, a wait by
/// place, or the inbox (43).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveredBy {
    Notice,
    Wait,
    Inbox,
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
    /// The launch's settings by stage, kept since each later start reads
    /// the launch's layer as the launch gave it and the caller who gave
    /// it may be gone (8, 32).
    pub settings: BTreeMap<String, Settings>,
    /// The launch's settings for every stage's agent, kept as `settings`
    /// is (7, 8, 32).
    #[serde(default)]
    pub every_stage: Settings,
    /// The repository's settings for the kind, which it does not
    /// declare, as the launch read them at the base's revision (8, 21,
    /// 32).
    #[serde(default)]
    pub repository_settings: KindSettings,
    pub hold: Vec<String>,
    pub hold_submission: bool,
    /// The words of each hold the launch put, by stage name (7, 130).
    #[serde(default)]
    pub hold_words: BTreeMap<String, String>,
    /// The recipients of the place's ends, as the launch named them (7,
    /// 8, 43).
    #[serde(default)]
    pub recipients: Vec<String>,
    /// The stream the place runs in, fixed at its launch, and where it
    /// came from (8, 143).
    pub stream: StreamHeld,
    /// The intent the place serves, as its launch named it or its
    /// repository and base give it (147).
    pub intent: IntentKey,
}

/// An intent's key: the name a launch gave it, else the repository and
/// base the place is cut from, each with the digest that names the
/// intent's document on the host, the SHA-256 in hex of `name:<name>` or
/// `base:<repository id>:<base>` (147, 153).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "from", rename_all = "kebab-case")]
pub enum IntentKey {
    /// The launch named it.
    Launch { name: String, digest: String },
    /// The launch named none: the repository and base give it.
    Base {
        repository: String,
        base: String,
        digest: String,
    },
}

impl IntentKey {
    /// The digest that names the intent's document (153).
    pub fn digest(&self) -> &str {
        match self {
            IntentKey::Launch { digest, .. } | IntentKey::Base { digest, .. } => digest,
        }
    }
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
    /// Removed, its branch and held state still to go (66), since `at`.
    Gone {
        over: Over,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        at: Option<Seconds>,
    },
    /// Its worktree and branch removed, its document kept apart from a
    /// live place's for the ends pending for `pending_for`, since `at`
    /// (43, 62, 66).
    Tidied {
        over: Over,
        at: Seconds,
        pending_for: Vec<String>,
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
    /// A repetition of a repeating stage delivered its group, its agent
    /// to be ended, and a task of the change still unticked: the next
    /// repetition is due, and nothing of the stage has ended (140).
    Repeating,
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
    /// The setting a stage was refused at its start for, with its layer:
    /// a program no agent profile binds, or a variable its account names
    /// that the host user's login environment lacks (27, 32, 144).
    #[serde(default)]
    pub refused_setting: Option<String>,
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
    /// For a repeating stage between its repetitions, how many lines
    /// `made.jsonl` held when the first repetition whose lines no end yet
    /// carries started: the stage's end carries every line from there, so
    /// what an earlier repetition made is not lost (129, 140).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub made_from: Option<usize>,
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
    /// What the stage's run recorded it made since the start, each line
    /// as written (129).
    #[serde(default)]
    pub made: Vec<Made>,
    /// For a repetition of a repeating stage that ended short, stuck,
    /// lost or never begun, the task group it was given (142); boxed, so
    /// an end stays small beside a submission's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<GroupHeld>>,
    /// For a start never begun because its agent began without its
    /// settings a second time in a row, how it began otherwise, by its
    /// variables' names alone (35, 161); boxed, so an end stays small.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsettled: Option<Box<Unsettled>>,
}

/// One line a stage's run wrote of what it made (129): a JSON object
/// naming the thing, kept whole with every field as the run wrote it, or
/// a line that is no such object, kept as its text so the run's mistake
/// is seen and not dropped. flume builds, checks and rewrites no address
/// of it (core.23).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Made {
    Thing(serde_json::Map<String, serde_json::Value>),
    Text(String),
}

impl Made {
    /// A line of `made.jsonl` as flume carries it.
    pub fn read(line: &str) -> Made {
        match serde_json::from_str::<serde_json::Value>(line) {
            Ok(serde_json::Value::Object(thing))
                if thing.get("name").is_some_and(serde_json::Value::is_string) =>
            {
                Made::Thing(thing)
            }
            _ => Made::Text(line.to_string()),
        }
    }

    /// The thing's name, none for a line that names none.
    pub fn name(&self) -> Option<&str> {
        match self {
            Made::Thing(thing) => thing.get("name").and_then(serde_json::Value::as_str),
            Made::Text(_) => None,
        }
    }
}

/// One start of a stage and its agent.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartHeld {
    pub n: u32,
    pub started_at: Seconds,
    /// The branch's head when the stage started (25).
    pub start_head: Revision,
    /// How many lines `made.jsonl` in the place's folder held when the
    /// stage started: the lines after them are this start's (129).
    #[serde(default)]
    pub made_from: usize,
    /// A retry's or a failed submission's words.
    pub words: Option<String>,
    pub identity: Identity,
    pub secret: Secret,
    pub program: String,
    /// The settings it started with, each value as its layer wrote it
    /// (32, 56).
    pub settings: Settings,
    /// Each setting's layer and value as written, by `program`, `model`,
    /// `instructions` or `env.<NAME>`, a variable's from its account and
    /// its value [`crate::SET`], so no value is kept (32, 56, 144).
    #[serde(default)]
    pub from: BTreeMap<String, Given>,
    /// The account it runs under, none where its stream and the host
    /// name no pool (144, 145).
    pub account: Option<AccountHeld>,
    /// What its agent is to run under, which the agent's word that it
    /// started is compared with: its program, its account and each
    /// variable's digest keyed by its secret, never a value (161).
    pub start_record: StartRecord,
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
    /// For a repetition of a repeating stage, the task group it was
    /// given: the first holding a task unticked at `start_head`, or none
    /// where every task was ticked there (140).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupHeld>,
    /// The command of flume's opening the agent, as the host names it,
    /// until the terminal's open answers: while it lives, the terminal may
    /// still be bringing the program up, and no other command sends the
    /// prompt or answers the program's questions (34, 35, 37).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opening: Option<String>,
}

/// What a start's agent is to run under (161): the program its settings
/// resolved, the account it runs under, none where it has none, and, by
/// each variable's name, the variable's digest, HMAC-SHA256 keyed by the
/// start's secret over its name, a zero byte and its value, in hex. It
/// covers each variable of the start's resolved environment and
/// `FLUME_AGENT`, `FLUME_PLACE` and `FLUME_STAGE`; never `PATH` or `SHELL`,
/// which a terminal sets for its own sake, the login environment beneath
/// the settings, or `FLUME_SECRET`, which the started ack checks itself.
/// No value is kept, and no guessed value is tested against a digest
/// without the secret (core.30).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartRecord {
    pub program: String,
    pub account: Option<String>,
    pub env: BTreeMap<String, String>,
}

/// The account a start runs under: its pool, its name, and whether the
/// intent's affinity or the pool's strategy chose it (144, 145, 147).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountHeld {
    pub pool: String,
    pub name: String,
    pub from: AccountFrom,
}

/// What chose a start's account (145, 147).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountFrom {
    Affinity,
    Strategy,
}

/// A move from one account to another: of a stage, whose start went to
/// another account than its last, or of an intent, whose affinity moved;
/// why, and when. The agents after it start without the earlier
/// account's cache (56, 145 to 147).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountMove {
    #[serde(flatten)]
    pub on: MoveOn,
    pub from: String,
    pub to: String,
    pub why: MoveWhy,
    pub at: Seconds,
}

/// What moved between accounts: a stage, or the intent (147).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "on", rename_all = "snake_case")]
pub enum MoveOn {
    Stage { stage: String },
    Intent,
}

/// Why a start went to another account: the one before was spent, or at
/// its own bound (145, 146).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MoveWhy {
    Spent,
    Full,
}

impl MoveWhy {
    pub fn name(self) -> &'static str {
        match self {
            MoveWhy::Spent => "spent",
            MoveWhy::Full => "full",
        }
    }
}

/// The task group a repetition was given, as the change gave it at the
/// start's head: its place among the change's groups, its number, its
/// title and its tasks' texts (86, 140).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupHeld {
    /// Its index among the change's groups.
    pub index: usize,
    #[serde(default)]
    pub number: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub tasks: Vec<String>,
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
    /// The newest submission of the branch the merge queue held when the
    /// place was launched, by its id: an earlier place's of the same
    /// name, which this place never reads as its own (3, 99, 132).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
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

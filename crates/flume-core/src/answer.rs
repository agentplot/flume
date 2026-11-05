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
    /// A wake of a role's address: its run launched, or the run that
    /// stands for it, live or waiting, and nothing made (132).
    Woken {
        address: String,
        woke: Woke,
        place: PlaceStatus,
    },
    /// A stream's session shown in its view at a person's asking, and
    /// nothing flume holds changed (186; core.48).
    Opened(StreamOpened),
}

/// What an open showed (186): the stream, its multiplexer, the session
/// shown, its view, the title and the terminal app of a window, each
/// with where it came from, the command the window runs by its
/// variables' names, and what the window did.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamOpened {
    pub stream: String,
    pub multiplexer: ProfileHeld,
    pub session: SessionHeld,
    pub view: ViewHeld,
    /// None for the app's view, whose app titles its own window.
    #[serde(default)]
    pub title: Option<TitleHeld>,
    /// None for the app's view.
    #[serde(default)]
    pub terminal_app: Option<ProfileHeld>,
    /// None for the app's view.
    #[serde(default)]
    pub command: Option<ViewCommandStatus>,
    pub window: WindowOutcome,
}

/// The command a window runs to show a stream's session, by its program,
/// its arguments and its variables' names, never their values (187;
/// core.30).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewCommandStatus {
    pub program: String,
    pub arguments: Vec<String>,
    #[serde(default)]
    pub environment: Vec<String>,
}

/// What showing a stream's session did (186, 188).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowOutcome {
    /// The terminal app opened a window.
    Opened,
    /// No window: the line for a person to run.
    Printed { line: String },
    /// The multiplexer's own app showed the session.
    App,
}

/// What a wake of a role's address came to (132): its run launched by
/// this wake, or the run that stood, its agent live or the run waiting,
/// for room, on its caller or for another settle's launch to finish.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Woke {
    Launched,
    Live,
    Waiting,
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
    /// The stream the place runs in, fixed at its launch, and where it
    /// came from (56, 143).
    pub stream: StreamHeld,
    /// The place's label as it would resolve at a start now, with where
    /// it came from, none where nothing gives one (56, 180).
    #[serde(default)]
    pub label: Option<LabelHeld>,
    /// The intent the place serves, and where it came from (56, 147).
    pub intent: IntentKey,
    /// Each move of a stage of the place, or of its intent at a start of
    /// the place's, between accounts, in order, the agents after it
    /// starting without the earlier account's cache (56, 145 to 147).
    pub moves: Vec<MoveStatus>,
    /// Each start of the place's whose agent began without the settings
    /// its start recorded, each variable by its name alone and whether
    /// the program differed, never a value (56, 161).
    pub unsettled: Vec<UnsettledStart>,
    /// Each start of the place's that carried its stage's conversation and
    /// did not begin, with what happened: its stage started afresh after
    /// it (56, 146, S79).
    #[serde(default)]
    pub uncarried: Vec<UncarriedStart>,
    /// Every end of the place, with whom it was delivered to and when,
    /// and for whom it stands pending (43).
    #[serde(default)]
    pub ends: Vec<EndStatus>,
    /// The role's address the place is a run of, none for any other
    /// place (132).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// The clues show gives the place while it is live, with where the
    /// terminal shows them (139).
    #[serde(default)]
    pub clues: Option<PlaceClues>,
}

/// A move between accounts as status gives it: what moved, from, to,
/// why and when, and that the agents after it start without the earlier
/// account's cache (56, 147).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveStatus {
    #[serde(flatten)]
    pub moved: AccountMove,
    pub cache: Cache,
}

impl MoveStatus {
    pub fn of(moved: &AccountMove) -> MoveStatus {
        MoveStatus {
            moved: moved.clone(),
            cache: Cache::Lost,
        }
    }

    /// The move as a person reads it, an intent's for the program whose
    /// account moved where it names one, with whether a stage's
    /// conversation went with it where its agent stopped at its
    /// account's usage limit (56, 147, 179).
    pub fn words(&self) -> String {
        let moved = &self.moved;
        let what = match &moved.on {
            MoveOn::Stage { stage } => format!("{stage} moved"),
            MoveOn::Intent {
                program: Some(program),
            } => format!("the intent moved for {program}"),
            MoveOn::Intent { program: None } => "the intent moved".to_string(),
        };
        let conversation = match moved.carried {
            Some(true) => ", its conversation with it",
            Some(false) => ", its conversation stayed behind",
            None => "",
        };
        format!(
            "{what} from {} to {} at {}, {} {}{conversation}, its agents after starting without {}'s cache",
            moved.from,
            moved.to,
            moved.at,
            moved.from,
            match moved.why {
                MoveWhy::Spent => "spent",
                MoveWhy::Full => "at its bound",
            },
            moved.from
        )
    }
}

/// What a move between accounts did to the prompt cache its agents read
/// (147).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cache {
    Lost,
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
/// states no bound has no room to judge and no queue. Each of its
/// streams gives the same of its own (143).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostStatus {
    pub running: u32,
    pub bound: Option<u32>,
    pub room: Option<u32>,
    pub queue: Vec<Queued>,
    /// Each stream of the host, in the order its stages would start: the
    /// streams the configuration declares and any a live place still runs
    /// in (56, 143).
    pub streams: Vec<StreamStatus>,
    /// Each pool a stream of the host takes its accounts from, by name
    /// (56, 144 to 146).
    pub pools: Vec<PoolStatus>,
    /// What the terminal made and keeps for what it did not make, each
    /// named with what holds it (56, 137, 138).
    #[serde(default)]
    pub terminal_kept: Vec<String>,
    /// Each role's address an end stands pending for, with the run the
    /// host's configuration names for it, the run standing, and why it
    /// was not woken (132; core.28).
    #[serde(default)]
    pub addresses: Vec<AddressStatus>,
    /// The watch the terminal keeps, with its bound, or none (122).
    #[serde(default = "no_watch")]
    pub watch: Watch,
    /// Each terminal the host binds, its own first, then each stream's
    /// by the stream's name, each once: its profile, what it made and
    /// keeps for what it did not make, and its watch (56, 119, 190).
    #[serde(default)]
    pub terminals: Vec<TerminalStatus>,
    /// Each repository's base a live place stands on, with the count of
    /// its places show gives and where the terminal shows it (139).
    #[serde(default)]
    pub bases: Vec<BaseClues>,
    /// The host's configuration as it stands: whether its local layer was
    /// taken, every value with its layer, an account's variables by name
    /// and `<set>`, and each local key passed over with its pin (56, C.7;
    /// core.30, core.45).
    #[serde(default)]
    pub configuration: HostConfiguration,
}

fn no_watch() -> Watch {
    Watch::None
}

impl Default for HostStatus {
    fn default() -> HostStatus {
        HostStatus {
            running: 0,
            bound: None,
            room: None,
            queue: vec![],
            streams: vec![],
            pools: vec![],
            terminal_kept: vec![],
            addresses: vec![],
            watch: Watch::None,
            terminals: vec![],
            bases: vec![],
            configuration: HostConfiguration::default(),
        }
    }
}

/// A repository's base as show counts it (139): the count of its live
/// places running, asking, waiting on their caller and stalled, that
/// count in its one line, and where the terminal shows it, in the
/// terminal's words.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseClues {
    pub repository: String,
    pub base: String,
    pub count: BaseCount,
    pub line: String,
    pub shown_at: Option<String>,
}

/// A live place's clues as show gives them (139): its current stage and
/// that stage's state, the title `<stage> · <state>` its agent is shown
/// under, the name `<kind>--<name>` the agent is shown as, and where the
/// terminal shows them, in the terminal's words.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceClues {
    pub stage: String,
    pub state: String,
    pub title: String,
    pub display_agent: String,
    pub shown_at: ShownAt,
}

/// A role's address with ends pending for it (132): the run the host's
/// configuration names for it, none where it names none and nothing
/// wakes it; its run where one stands; each end pending for it; and why
/// the last settle did not wake it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressStatus {
    pub address: String,
    pub role: Option<RoleRun>,
    pub run: Option<PlaceId>,
    pub pending: Vec<PendingEnd>,
    pub unwoken: Option<String>,
}

/// An end pending for a role's address: the place it is of, its key, and
/// what ended in a notice's words (43, 132).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingEnd {
    pub place: PlaceId,
    pub key: String,
    pub what: String,
}

/// A stage waiting for room on the host: it waits on the host, never on
/// its caller, in its stream, short of the room it names (70, 130, 143).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Queued {
    pub place: PlaceId,
    pub stage: String,
    pub stream: String,
    /// The label its place resolves to now, none where nothing gives one
    /// (180).
    #[serde(default)]
    pub label: Option<String>,
    pub priority: i64,
    pub since: Option<Seconds>,
    pub wants: RoomWanted,
}

/// The room a stage waiting for room is short of (70, 143, 145, 181): the
/// host's, its own full or taken by stages that rank ahead; its stream's,
/// whose bound its agents running have reached; its stream's room past
/// what the stream's other labels keep free and do not use, naming those
/// labels; or its pool's, every
/// account that runs its program spent or at its own bound, with the
/// soonest time one stops being spent, none where none is spent. A stage
/// whose pool holds no account of its program waits on no room: it is
/// refused (4, 32, 145).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "room", rename_all = "kebab-case")]
pub enum RoomWanted {
    Host,
    Stream {
        stream: String,
        bound: u32,
    },
    KeptFree {
        stream: String,
        labels: Vec<String>,
    },
    Pool {
        pool: String,
        until: Option<Seconds>,
    },
}

/// `items` as a person reads a list: `a`, `a and b`, `a, b and c`.
pub(crate) fn and_list(items: &[String]) -> String {
    match items {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

impl RoomWanted {
    /// The want as a person reads it.
    pub fn words(&self) -> String {
        match self {
            RoomWanted::Host => "the host's room".into(),
            RoomWanted::Stream { stream, bound } => {
                format!("the stream {stream}'s room, at its bound of {bound}")
            }
            RoomWanted::KeptFree { stream, labels } => format!(
                "the stream {stream}'s room, the rest kept free for {}",
                and_list(labels)
            ),
            RoomWanted::Pool {
                pool,
                until: Some(until),
            } => format!("a usable account of the pool {pool}, the soonest spent until {until}"),
            RoomWanted::Pool { pool, until: None } => {
                format!("a usable account of the pool {pool}, each at its own bound")
            }
        }
    }
}

/// A stream of the host (143): the session its agents open in where
/// their launch names none, with where it came from, its multiplexer,
/// view and terminal app, each with where it came from, and the title a
/// window on it carries, its priority among streams, its agents running,
/// the bound it states, none where the host's alone holds it, its room,
/// none where it states no bound, its stages waiting for room in the
/// order they would start, its labels' shares, whether their reserves
/// are held, and why it refuses: its labels' fault, its multiplexer
/// naming a terminal profile flume does not carry, or, for a terminal
/// view, its terminal app naming a window profile flume does not carry,
/// that last also given alone, since it refuses the stream's open and
/// none of its stages; none where none holds (56, 70, 107, 181, 182, 186,
/// 188, 190; core.14).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamStatus {
    pub name: String,
    pub session: SessionHeld,
    pub multiplexer: ProfileHeld,
    pub view: ViewHeld,
    pub terminal_app: ProfileHeld,
    pub title: TitleHeld,
    pub priority: i64,
    pub running: u32,
    pub bound: Option<u32>,
    pub room: Option<u32>,
    pub queue: Vec<Queued>,
    #[serde(default)]
    pub labels: Vec<LabelStatus>,
    #[serde(default)]
    pub reserves: Reserves,
    #[serde(default)]
    pub refused: Option<String>,
    /// The terminal app's refusal alone, the last part of `refused` where
    /// it holds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_refused: Option<String>,
}

/// The title a window on a stream's part of its terminal carries, and
/// where it came from: the stream's `title`, else its name (143, 186).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TitleHeld {
    pub title: String,
    pub from: TitleFrom,
}

/// Where a stream's title came from (143, 186).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TitleFrom {
    Stream,
    StreamName,
}

/// A terminal the host binds (119, 190): the profile a stream takes it
/// by, none for the host's own where no profile flume carries is it,
/// what it made and keeps for what it did not make, each named with what
/// holds it, and the watch it keeps.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalStatus {
    pub profile: Option<String>,
    pub kept: Vec<String>,
    pub watch: Watch,
}

/// A label of a stream (181): its reserve, the slots of the stream it
/// keeps free while it runs fewer, its agents running, each counted by
/// the label it started under, and its stages waiting for room.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelStatus {
    pub name: String,
    pub reserve: u32,
    pub keep_free: u32,
    pub running: u32,
    pub waiting: u32,
}

/// Whether a stream's labels' reserves are held: held where it states a
/// bound, passed over where it states none, and none where it gives no
/// label a share (70, 181).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Reserves {
    Held,
    PassedOver,
    #[default]
    None,
}

/// A pool of the host (144, 145): its strategy and its accounts in
/// order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoolStatus {
    pub name: String,
    pub strategy: Strategy,
    pub accounts: Vec<AccountStatus>,
}

/// An account of a pool (144 to 146): the program it runs, flume's
/// agents running on it, its own bound, none where it states none, and
/// the time it is spent until, none where it is not spent; never a value
/// of its environment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountStatus {
    pub name: String,
    pub program: String,
    pub running: u32,
    pub bound: Option<u32>,
    pub spent_until: Option<Seconds>,
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
    /// For a repeating stage, the task group its current repetition works
    /// on, or the one its next would take, read from the change at the
    /// head when status is asked (56, 58, 142).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<GroupStatus>,
}

impl StageStatus {
    /// The stage as a person reads it: its name, and a repeating stage's
    /// group's place among the change's groups, `code 3/7` (139, 142).
    pub fn label(&self) -> String {
        label(&self.name, self.group.as_ref())
    }
}

/// A stage's name, and for a repeating stage its group's place among the
/// change's groups, `code 3/7` (139, 142).
pub fn label(stage: &str, group: Option<&GroupStatus>) -> String {
    match group {
        Some(group) => format!("{stage} {}/{}", group.place, group.of),
        None => stage.to_string(),
    }
}

/// A repeating stage's task group as status gives it: its place among
/// the change's groups at the head, counted from one, how many groups
/// the change holds there, and its number and title (56, 58, 142).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupStatus {
    pub place: usize,
    pub of: usize,
    #[serde(default)]
    pub number: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
}

/// The current stage's agent (56).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentStatus {
    pub identity: String,
    pub stage: String,
    pub program: String,
    pub settings: AgentSettings,
    /// The account it runs under, and whether the affinity or the
    /// strategy chose it; none where it runs under no account (56, 144,
    /// 145).
    pub account: Option<AccountHeld>,
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
    /// The dialog the agent's program shows a person before it takes its
    /// prompt, which flume does not answer, in its words as the terminal
    /// shows them: the place waits on a person to answer it in the
    /// agent's terminal (34, 35).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dialog: Option<String>,
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
    /// Its mode as its start passed it, none where no layer gave one; the
    /// mode as written and its layer are its settings' (56, 162, 169).
    #[serde(default)]
    pub mode: Option<ModeStatus>,
    /// For the agent of a start after its stage's agent stopped at its
    /// account's usage limit, that start's number and whether this agent
    /// carries its conversation, or why not, naming a variable and never
    /// a path; none for any other (56, 146, 179; core.30).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Conversation>,
    /// The session the agent opened in, with where it came from; none for
    /// an agent opened beside a pane and for a start written before (56,
    /// 107, 143).
    #[serde(default)]
    pub session: Option<SessionHeld>,
    /// The terminal the agent opened in, with where it came from; none
    /// for a start written before, whose agent is in the host's (56,
    /// 190).
    #[serde(default)]
    pub terminal: Option<ProfileHeld>,
}

/// An agent's mode at its start: `passed`, the value its profile passed,
/// as the start's record holds it; `entry`, the program whose entry of a
/// table of programs it took, none for one of flume's four given whole;
/// and `read_back`, whether its profile reads back the mode its program
/// began in, so its start was compared with it (56, 161, 169).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeStatus {
    pub passed: String,
    pub entry: Option<String>,
    pub read_back: bool,
}

/// The settings an agent started with, each value as its layer wrote it,
/// each variable its account gave by its name alone, its value
/// [`flume_seams::SET`], and where each came from (32, 56, 144).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentSettings {
    #[serde(flatten)]
    pub settings: Settings,
    pub env: BTreeMap<String, String>,
    /// Each setting's layer and value as written, by `program`, `model`,
    /// `instructions` or `env.<NAME>`, an environment variable's value
    /// [`flume_seams::SET`] and its layer `account`.
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
    /// The request the merge queue answered flume's last submit with,
    /// where it takes the base's work as requests: a forge's pull
    /// request by its number, which an intake takes in as the entry
    /// read-submission reads (124, C.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
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
    /// What each terminal closed and kept of what it made for each place
    /// removed, by the terminal (137, 138, 190).
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

/// What a terminal closed of what it made for a place removed, and what
/// it kept, each named with what holds it, under the name of the
/// terminal's profile: none for the host's own where no profile the
/// resolver carries gives it (137, 138, 190).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalTidied {
    pub place: PlaceId,
    #[serde(default)]
    pub profile: Option<String>,
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

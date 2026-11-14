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
    /// submission it answers, handed to the recipient by this wait, and
    /// confirmed by it where the wait confirms on receipt, or answered as
    /// a read of one the recipient confirmed; no end where the wait was
    /// still waiting when its timeout passed (43, design 9; weir 103).
    Waited {
        place: PlaceId,
        recipient: String,
        end: Option<EndStatus>,
        /// This wait handed the end over; false for a read.
        delivered: bool,
    },
    /// Every end pending for the recipient, of every place on the host,
    /// tidied places kept for their ends among them, each handed to it by
    /// the inbox and confirmed where the inbox confirms on receipt, and
    /// every place that waits on its caller and on this recipient, which
    /// marks nothing (43, 130, 193; design 9; weir 103).
    Inbox {
        recipient: String,
        ends: Vec<Collected>,
        #[serde(default)]
        waiting: Vec<WaitingPlace>,
        /// Every message pending at the recipient, in the order recorded,
        /// each handed to it by the inbox; none where the inbox names a
        /// place (43, 214).
        #[serde(default)]
        messages: Vec<CollectedMessage>,
    },
    /// Each delivery key a recipient confirmed, with when, and whether it
    /// had confirmed it before (43; weir 102).
    Confirmed {
        recipient: String,
        confirmed: Vec<KeyConfirmed>,
    },
    /// A wake of a role's address: its run launched, or the run that
    /// stands for it, live or waiting, and nothing made (132).
    Woken {
        address: String,
        woke: Woke,
        place: PlaceStatus,
    },
    /// A wake of a role's address the host's configuration gives a
    /// standing agent: nothing made, where the address lives, read at
    /// the ask, and each end pending for it (132; core.28).
    Held {
        address: String,
        standing: StandingStatus,
        pending: Vec<PendingEnd>,
    },
    /// A message to a role's address: its delivery's key, the address,
    /// what the act found of it, the run that stands for it where one
    /// does, and why its run was not woken where it was not (53, 132,
    /// 214, C.7).
    Messaged {
        key: String,
        address: String,
        found: Found,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        run: Option<PlaceId>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        why: Option<String>,
    },
    /// `flume kinds lock`'s act: the lock it wrote, each kind it recorded
    /// at its content's hash, and each it removed (155).
    Locked {
        lock_file: String,
        recorded: Vec<LockedKind>,
        removed: Vec<String>,
    },
    /// Each place's line asked alone, of the place named or of every
    /// place on the host, read from its held state, the change at its
    /// head and its live agent's asking mark alone (56, 57, 58).
    Lines { lines: Vec<PlaceLine> },
    /// A stream's session shown in its view at a person's asking, and
    /// nothing flume holds changed (186; core.48).
    Opened(StreamOpened),
    /// Every place on the host and every stream it declares, a line
    /// each, read from what flume keeps and each stream's terminal once
    /// (200; core.52).
    List {
        places: Vec<Listed>,
        streams: Vec<StreamListed>,
    },
}

/// A place's line in the list and its parts (200): `<kind>--<name>`, the
/// stage its line names, its state in its line's words, whom it waits on
/// and what keeps it, a part it has none of left out; `line` joins them
/// by ` · `. None of a place's long words is among them (56, 139).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Listed {
    pub place: PlaceId,
    pub repository: String,
    /// `<kind>--<name>`.
    pub name: String,
    /// `<kind>@<version>`.
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    pub state: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waits_on: Option<WaitsOn>,
    /// What keeps a place whose work is over, in a word (64 to 66, 68).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kept: Option<String>,
    pub line: String,
}

/// Whom a waiting place waits on: its caller for a move 130 names, the
/// host for room (70), the merge queue for its submission's outcome
/// (111), or, for a base place, the places on its branch (203).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WaitsOn {
    Caller,
    Host,
    /// A process runs the conversation its waiting stage's start would
    /// carry, or flume could not read whether one does (49, 207).
    Session,
    MergeQueue,
    /// A base place whose branch a place is live on, which lands there
    /// before the base place is submitted; never stalled for it (130,
    /// 203).
    Places,
}

impl WaitsOn {
    /// Whom the place waits on as its line says it.
    pub fn words(self) -> &'static str {
        match self {
            WaitsOn::Caller => "waits on its caller",
            WaitsOn::Host => "waits on the host",
            WaitsOn::Session => "waits on its session",
            WaitsOn::MergeQueue => "waits on the merge queue",
            WaitsOn::Places => "waits on the places on its branch",
        }
    }
}

/// A stream's line in the list and its parts (107, 186, 190, 199, 200):
/// the stream, its session as an open resolves it, its multiplexer with
/// where it came from, whether its session runs, or why no terminal was
/// asked, and each other session it names for a kind or a stage with
/// whether it runs; `line` joins them by ` · `.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamListed {
    pub stream: String,
    pub session: SessionHeld,
    pub multiplexer: ProfileHeld,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<SessionRuns>,
    /// Why no runs was asked: the stream's multiplexer names no profile
    /// flume carries (190).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncarried: Option<String>,
    /// Each session the stream names for a kind or a stage but its own, by
    /// name, read in the one runs its own is (199, 201).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sessions: Vec<SessionListed>,
    pub line: String,
}

/// A session a stream names for a kind or a stage, on the stream's line
/// in the list, with whether it runs, none where no terminal was asked
/// (199, 200, 201).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionListed {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs: Option<SessionRuns>,
}

/// What an open showed (186): the stream whose session it is, its
/// multiplexer, the session shown, its view, the title and the terminal
/// app of a window, each with where it came from, the kinds and stages
/// the stream names the session for (199), the command the window runs
/// by its variables' names, and what the window did.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamOpened {
    pub stream: String,
    pub multiplexer: ProfileHeld,
    pub session: SessionHeld,
    /// The kinds and stages the stream names the session for, each with
    /// its layer; none where it names it for none (199).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub named: Option<NamedSession>,
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

/// A kind `flume kinds lock` recorded: its identity and its content's
/// hash (155).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockedKind {
    pub kind: String,
    pub hash: String,
}

/// A delivery key confirmed: when the recipient confirmed it, and whether
/// it had before this call, the time then being its first (43; weir 102).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyConfirmed {
    pub key: String,
    pub at: Seconds,
    pub before: bool,
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

/// What a message to a role's address found of it (53, 132, 214): its
/// run launched by the message, its run's agent live, its run waiting,
/// for room, on its caller or for its launch, its run not woken, with
/// why, a standing agent of its source's, the address carried to another
/// host, or the address's batch handed into its session by the act.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Found {
    Woken,
    Live,
    Waiting,
    Unwoken,
    Standing,
    Carried,
    Handed,
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
    /// The repository's address, `<owner>/<name>`, as the launch read it
    /// from its clone's origin, by which the host's configuration names
    /// it; none where it had none (56, 143).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub kind: String,
    pub version: u32,
    pub declared: Declared,
    /// The kind's identity as its launch read it, `acme/review@2`; none
    /// for a place whose held state keeps no `kind_from` (19, 56).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// Where the kind came from, as its launch read it (19, 21, 56).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind_from: Option<KindFrom>,
    pub name: String,
    pub branch: String,
    pub base: String,
    /// Where the launch read the base's revision, none for a place
    /// launched before flume said (9, 56, C.1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_from: Option<BaseFrom>,
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
    /// Where the launch took them from, the answerer behind the place's
    /// stream with the source that declares it; none for a role's run
    /// and a place launched before flume kept it (7, 56, 193).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients_from: Option<RecipientsFrom>,
    /// The words of each hold the launch put, by stage name (7, 130).
    pub hold_words: BTreeMap<String, String>,
    /// The move the place waits on its caller for, and since when; none
    /// where it waits on nobody (56, 130).
    pub waiting_on: Option<WaitingOn>,
    /// The place waited on its caller under no hold's words past its
    /// bound, read so by a settle (130).
    #[serde(default)]
    pub stalled: bool,
    /// The stage that waits for room while a process runs the
    /// conversation its start would carry, or flume could not read
    /// whether one does, each process as the last settle's live read it,
    /// and since when it waits; none where no stage waits so (49, 56,
    /// 207; core.30).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waits_on_session: Option<SessionWait>,
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
    /// The place's one line, made from the facts above, the same as the
    /// line asked alone (56, 60; S14).
    #[serde(default)]
    pub line: PlaceLine,
    /// Whom a live place waits on, as the list gives it, the places on
    /// its branch among them for a base place; none where it waits on
    /// nobody (56, 130, 200, 203; core.14).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waits_on: Option<WaitsOn>,
    /// Whether the place has a worktree, given for a base place alone,
    /// which has none (9, 56, 202).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worktree: Option<bool>,
    /// The places being made or live whose base is a base place's
    /// branch, given for a base place alone (56, 130, 203).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_branch: Option<Vec<PlaceId>>,
    /// For a base place whose kind names a fix, the kind its fixes run as,
    /// as its launch resolved it, with where it came from (21, 56, 203).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fix_kind: Option<FixKind>,
    /// For such a base place, how many fixes it runs before it waits on
    /// its caller: its kind's bound, or flume's default for a stage's (23,
    /// 56, 203).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fix_limit: Option<u32>,
    /// For such a base place, the newest forecast a settle held of its
    /// branch, with when it was read, its head and its base's revision:
    /// clear, in conflict with its files and the entries ahead it meets,
    /// or unread with why; status forms none of its own (56, 58, 203, 210;
    /// core.15).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forecast: Option<ForecastHeld>,
    /// For a base place, each fix flume launched for it, with its number,
    /// its place, its cause, when it was launched and how it ended (56,
    /// 203).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixes: Vec<BaseFix>,
    /// For a place flume launched as a base place's fix, that base place,
    /// the fix's number and its cause (56, 203).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fix_of: Option<FixOf>,
}

/// A stage waiting for room while a process runs the conversation its
/// start would carry: the stage, each process by its id and when it
/// started as live last read it, or what live could not read, naming a
/// variable and never a path, and since when the stage waits (49, 56,
/// 207; core.30).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionWait {
    pub stage: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub processes: Vec<LiveProcess>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unread: Option<String>,
    pub since: Option<Seconds>,
}

/// A place's one line and its parts (56, 142): `<kind>--<name>`,
/// `<kind>@<version>`, its current stage with a repeating stage's group,
/// how many times that stage started, its change's tasks ticked at the
/// head, and its state in words; `full` joins them by ` · `, a part it
/// has none of left out, and `short` is `full` less its name and kind.
/// A full line asked for a session that is not the current stage's live
/// agent ends saying so; a short one never does (35, 140).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceLine {
    pub name: String,
    pub kind: String,
    pub stage: Option<String>,
    pub run: Option<u32>,
    pub tasks: Option<String>,
    pub state: String,
    pub full: String,
    pub short: String,
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
    /// account moved where it names one, off an account past its
    /// threshold with the window that held it and its end, with whether a
    /// stage's conversation went with it where its agent stopped at its
    /// account's usage limit (56, 147, 179, 185).
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
        let why = match (moved.why, &moved.window, moved.until) {
            (MoveWhy::Spent, ..) => "spent".to_string(),
            (MoveWhy::Threshold, Some(window), Some(until)) => {
                format!("past its threshold in {window} until {until}")
            }
            (MoveWhy::Threshold, ..) => "past its threshold".to_string(),
            (MoveWhy::Full, ..) => "at its bound".to_string(),
        };
        format!(
            "{what} from {} to {} at {}, {} {why}{conversation}, its agents after starting without {}'s cache",
            moved.from, moved.to, moved.at, moved.from, moved.from
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
/// it, its key as callers name it, the answer that answered a stop-short
/// once one did, with who gave it and when, each delivery, and the
/// recipients it stands pending for (42, 43).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndStatus {
    #[serde(flatten)]
    pub end: EndHeld,
    /// The end's key as callers name it, by which an answer names the
    /// stop-short it answers (42, 43).
    pub key: EndKey,
    pub pending_for: Vec<String>,
    /// For a stop-short's end, the time its agent was kept open for the
    /// answer until when the end was read; none where it was not kept
    /// open, and for every other end (43, 198).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_until: Option<Seconds>,
    /// The key of its delivery, `<kind>--<name>/<token>/<end key>`, by
    /// which a recipient confirms it, given only once the host's
    /// deliveries record it; none for an end with no recipient, one no
    /// settle has recorded yet, and one of a document that holds no token
    /// yet (43; weir 101).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery: Option<String>,
    /// When its delivery expires, by the host's clock, fixed when it was
    /// recorded; none where none is recorded (43; weir 104).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Seconds>,
    /// Each recipient that has not confirmed it, whether it is expired
    /// for it, and whether anything was ever confirmed under its name (43,
    /// 56; weir 104 to 106).
    #[serde(default)]
    pub owed: Vec<OwedTo>,
    /// Each recipient's confirmation of it, and when (43; weir 102).
    #[serde(default)]
    pub confirmed: Vec<ConfirmedBy>,
}

impl EndStatus {
    /// The end `end` of the place `place` as status gives it, its
    /// delivery read from `owing`, the host's deliveries as one read gave
    /// them: each hand-over of it, and the recipients it stands pending for
    /// (43, 198; weir 103, 105).
    pub fn of(end: &EndHeld, place: &PlaceRecord, owing: &Owing) -> EndStatus {
        let mut shown = end.clone();
        shown.deliveries = end.handovers(place, owing);
        // An end whose delivery is not recorded yet reads as its record
        // would: owed to each recipient the old form delivered nothing to,
        // each old delivery confirmed at its time (43, 61).
        let (owed, confirmed, expires_at) = match end.delivery(place, owing) {
            Some(delivery) => (
                delivery.owed.clone(),
                delivery.confirmed.clone(),
                Some(delivery.expires_at),
            ),
            None => {
                let mut confirmed: Vec<ConfirmedBy> = vec![];
                for delivery in &end.deliveries {
                    if !confirmed
                        .iter()
                        .any(|by| by.recipient == delivery.recipient)
                    {
                        confirmed.push(ConfirmedBy {
                            recipient: delivery.recipient.clone(),
                            at: delivery.at,
                        });
                    }
                }
                let owed = end
                    .pending(place, owing)
                    .into_iter()
                    .map(|recipient| OwedTo {
                        known: owing.known.contains(&recipient),
                        recipient,
                        expired: false,
                    })
                    .collect();
                (owed, confirmed, None)
            }
        };
        // Only a key the host's deliveries hold is named, so a recipient
        // never confirms a key flume has not recorded (43; weir 101, 102).
        let delivery = end
            .delivery(place, owing)
            .map(|delivery| delivery.key.clone());
        EndStatus {
            end: shown,
            key: end.of.end_key(),
            pending_for: owed
                .iter()
                .map(|owed: &OwedTo| owed.recipient.clone())
                .collect(),
            delivery,
            expires_at,
            owed,
            confirmed,
            open_until: match &end.of {
                EndOf::Stage {
                    state: StageState::EndedShort,
                    end,
                    ..
                } => end.open_until,
                _ => None,
            },
        }
    }
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
    /// This inbox confirmed it, as one that confirms on receipt does
    /// (43; weir 103).
    #[serde(default)]
    pub confirmed: bool,
    /// It is expired for the recipient: pending past the host's span
    /// since it was read (43; weir 104).
    #[serde(default)]
    pub expired: bool,
}

/// A message the inbox collected: its delivery's key, the address it
/// waits at, who sent it where its first line names one, when it was
/// recorded, its words, and whether this inbox confirmed it and it is
/// expired for the recipient (43, 214; weir 103, 104).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectedMessage {
    pub key: String,
    pub address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    pub at: Seconds,
    pub words: String,
    #[serde(default)]
    pub confirmed: bool,
    #[serde(default)]
    pub expired: bool,
}

/// An end expired for a recipient on the host, as status gives it: its
/// place, as `flume status` names it, the end's key, its delivery's key,
/// the recipient, whether anything was ever confirmed under its name, and
/// when it expired (43, 56; weir 104, 106).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpiredEnd {
    pub place: PlaceId,
    pub key: EndKey,
    pub delivery: String,
    pub recipient: String,
    pub known: bool,
    pub expires_at: Seconds,
}

/// A place the inbox gives as waiting on its recipient: one that waits on
/// its caller and that the recipient is a recipient of, was named by the
/// stop-short it waits on, or was given a stall of as the answerer behind
/// its stream; given whether anything of it was delivered or not, and
/// marked nothing for (43, 130, 193; design 9).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WaitingPlace {
    pub place: PlaceId,
    pub kind: String,
    pub name: String,
    pub stream: String,
    /// The first line of its brief, none where it has none.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brief: Option<String>,
    /// The move it waits on, since when and its words, and for an answer
    /// whether its agent is kept open for it and until when (130, 198).
    pub waiting_on: WaitingOn,
    /// The stop-short the move answers, with its words, whether it needs
    /// an answer or found work, its key and the address it named; none
    /// where the move answers none (41, 42).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_short: Option<StopShortWaited>,
    /// The hold on the move, with its words, since when and its bound,
    /// while it holds; none where none does (130).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hold: Option<Hold>,
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
    /// Each role's address an end stands pending for, and each the
    /// host's configuration gives a standing agent, with the run the
    /// configuration names for it, the run standing, why it was not
    /// woken, and where a standing agent lives (132, 190; core.28).
    #[serde(default)]
    pub addresses: Vec<AddressStatus>,
    /// The watch a caller keeps over the host, with the bound it stated,
    /// or none while none is kept (56, 119, 208).
    #[serde(default = "no_watch")]
    pub watch: Watch,
    /// When the newest settle a caller asked as its watch began, by the
    /// host's clock; none where the host holds no watch (56, 208).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watch_settled: Option<Seconds>,
    /// When the watch lapsed, twice its bound after its newest settle
    /// began, given while flume runs attended after one; none while a
    /// watch is kept or none was ever held (56, 119, 208).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watch_lapsed: Option<Seconds>,
    /// Each terminal the host binds, its own first, then each stream's
    /// by the stream's name, each once: its profile, what it made and
    /// keeps for what it did not make, and its watch, none (56, 122,
    /// 190).
    #[serde(default)]
    pub terminals: Vec<TerminalStatus>,
    /// Each repository's base a live place stands on, with the count of
    /// its places show gives and where the terminal shows it (139).
    #[serde(default)]
    pub bases: Vec<BaseClues>,
    /// What the terminal's own configuration lacks for flume's plugin,
    /// each in the terminal profile's words; none where it lacks nothing
    /// (72, 122).
    #[serde(default)]
    pub terminal_setup: Vec<String>,
    /// The host's configuration as it stands: whether its local layer was
    /// taken, every value with its layer, an account's variables by name
    /// and `<set>`, and each local key passed over with its pin (56, C.7;
    /// core.30, core.45).
    #[serde(default)]
    pub configuration: HostConfiguration,
    /// Each source the host declares, by name, read when asked (56, 159,
    /// 191; core.15).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<SourceStatus>,
    /// Each end expired for a recipient on the host, of every place and
    /// every tidied document, read from one read of the host's deliveries
    /// (43, 56; weir 104, 106; core.15).
    #[serde(default)]
    pub expired: Vec<ExpiredEnd>,
}

/// A source the host declares as status gives it (56, 191, 193): its
/// name, its file, the directory its packages lie in, the identity of
/// every package that directory holds, and why the file or the directory
/// cannot be read where either cannot; and its roles: the file its
/// declaration names for them, none where it names none, each address
/// it declares, each entry of it that declares nothing for its address,
/// with why, and why the roles file cannot be read where it cannot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceStatus {
    pub name: String,
    pub source_file: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kinds_dir: Option<String>,
    #[serde(default)]
    pub kinds: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cannot_read: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles_file: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles_refused: Vec<RoleRefused>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles_cannot_read: Option<String>,
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
            watch_settled: None,
            watch_lapsed: None,
            terminals: vec![],
            bases: vec![],
            terminal_setup: vec![],
            configuration: HostConfiguration::default(),
            sources: vec![],
            expired: vec![],
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

/// A role's address with ends pending for it, one a source gives a
/// standing agent, pending or not, or one whose run's stall reached no
/// one (130, 132, 193): the run a source declares
/// for it, none where it declares none; its run where one stands; each
/// end pending for it; why the last settle did not wake it, none for a
/// standing address; for a standing address, where it lives; for a
/// carried one, its entry; and, where no source declares anything for
/// it, why, so it wakes nothing; and, where its run stalled and its
/// stall reached no one, its run being its own stream's answerer or no
/// role standing behind its stream, that stall, so status names what no
/// one was told; each message pending at it, and each place whose work is
/// not over that reports to it: each read at the ask, and none in a
/// status written before (56, 130, 190, 191, 193, 214; core.14,
/// core.15).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressStatus {
    pub address: String,
    pub role: Option<RoleStatus>,
    pub run: Option<PlaceId>,
    pub pending: Vec<PendingEnd>,
    pub unwoken: Option<String>,
    #[serde(default)]
    pub standing: Option<StandingStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carried: Option<CarriedRole>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wakes_nothing: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stalled: Option<PendingEnd>,
    /// Each message pending at the address, in the order recorded (214).
    #[serde(default)]
    pub messages: Vec<PendingMessage>,
    /// Each place whose work is not over that names the address among
    /// its recipients (7, 214).
    #[serde(default)]
    pub reporting: Vec<PlaceId>,
}

/// A message pending at a role's address, as status gives it: its
/// delivery's key, who sent it where its first line names one, when it
/// was recorded, its words' first line, each hand-over with its way and
/// when, and whether it is expired for the address (43, 56, 214; weir
/// 104).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingMessage {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    pub at: Seconds,
    pub line: String,
    pub handovers: Vec<Delivery>,
    pub expired: bool,
}

/// A role's run as status gives it under its address: what its source's
/// entry declares for the run, and the stream it runs in, its address's,
/// which the form gives as `role.stream` though an entry names no stream
/// of its own (132, 143, 193; core.14).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoleStatus {
    #[serde(flatten)]
    pub run: RoleRun,
    pub stream: String,
}

/// Where a role's address a standing agent holds lives, read at the ask
/// and kept by nothing (56, 132, 190, C.7; core.15): the agent's name as
/// the terminal knows it, the stream it is sought in, the session there
/// with where that came from, the profile the stream's terminal names,
/// none where the host declares no such stream, and what the terminal
/// found.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StandingStatus {
    pub agent: String,
    pub stream: String,
    pub session: SessionHeld,
    pub terminal: Option<String>,
    pub found: StandingFound,
}

/// What the terminal found of a standing agent in its session (132,
/// C.5): the one agent of its name, at the session name a notice reaches
/// it by, with its state; none of that name; more than one, which takes
/// no notice; or why it could not read, the
/// host declaring no such stream or its terminal naming no profile flume
/// carries among the reasons.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StandingFound {
    One { at: String, state: AgentState },
    None,
    Several { count: usize },
    Unread { reason: String },
}

/// An end pending for a role's address: the place it is of, its key as
/// callers name it, and what ended in a notice's words (42, 43, 132).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingEnd {
    pub place: PlaceId,
    pub key: EndKey,
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
/// account that runs its program spent, past its threshold or at its own
/// bound, with the soonest time one is back, none where each is only at
/// its bound (185). A stage
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
            } => format!("a usable account of the pool {pool}, the soonest back at {until}"),
            RoomWanted::Pool { pool, until: None } => {
                format!("a usable account of the pool {pool}, each at its own bound")
            }
        }
    }
}

/// A stream of the host (143): its own session, the one its agents open
/// in where neither their launch nor its word for their kind or stage
/// names one, with where it came from; each session it names for its
/// kinds and their stages, with the kinds and stages it names it for and
/// each one's layer; each fault of those tables, with the refusal it
/// causes the starts that read it; its multiplexer, view and terminal
/// app, each with where it came from, and the title a window on it
/// carries, its priority among streams, its agents running,
/// the bound it states, none where the host's alone holds it, its room,
/// none where it states no bound, its stages waiting for room in the
/// order they would start, its labels' shares, whether their reserves
/// are held, and why it refuses: its labels' fault, its multiplexer
/// naming a terminal profile flume does not carry, or, for a terminal
/// view, its terminal app naming a window profile flume does not carry,
/// that last also given alone, since it refuses the stream's open and
/// none of its stages, none where none holds; and who stands behind it
/// (4, 56, 70, 107, 181, 182, 186, 188, 190, 193, 199, C.7; core.14).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamStatus {
    pub name: String,
    pub session: SessionHeld,
    #[serde(default)]
    pub sessions: Vec<NamedSession>,
    #[serde(default)]
    pub faults: Vec<TableFault>,
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
    /// Who stands behind the stream as the host's sources declare it:
    /// the one role, with its source, or the roles that each declare to;
    /// none where no role does (143, 193).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answerer: Option<StreamAnswerer>,
}

/// The title a window on a stream's part of its terminal carries, and
/// where it came from: for the stream's own session, the stream's
/// `title`, else its name, and for a session it names for a kind or a
/// stage, that session's name (143, 186, 199).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TitleHeld {
    pub title: String,
    pub from: TitleFrom,
}

/// Where a window's title came from (143, 186, 199).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TitleFrom {
    Stream,
    StreamName,
    /// The name of a session the stream names for a kind or a stage.
    Session,
}

/// A terminal the host binds (119, 190): the profile a stream takes it
/// by, none for the host's own where no profile flume carries is it,
/// what it made and keeps for what it did not make, each named with what
/// holds it, and its watch, none since no terminal keeps one or
/// settles; nothing of a terminal decides the host's watch (61, 122,
/// 208).
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

/// A pool of the host (144, 145, 185): its strategy, its threshold, 90
/// where it states none, and its accounts in order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PoolStatus {
    pub name: String,
    pub strategy: Strategy,
    #[serde(default = "threshold")]
    pub threshold: u32,
    pub accounts: Vec<AccountStatus>,
}

/// The threshold of a pool that states none (185).
fn threshold() -> u32 {
    crate::THRESHOLD
}

/// An account of a pool (144 to 146, 184, 185): the program it runs,
/// flume's agents running on it, its own bound, none where it states
/// none, the time it is spent until, none where it is not spent, and,
/// where that time is the hour 146 gives because flume read no reset in
/// its agent's words, those words, its reading, none where it names none,
/// and what holds it past its threshold, none where nothing does; never a
/// value of its environment or a reading's path.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountStatus {
    pub name: String,
    pub program: String,
    pub running: u32,
    pub bound: Option<u32>,
    pub spent_until: Option<Seconds>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spent_unread: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reading: Option<ReadingStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub past: Option<Past>,
}

/// An account's reading as status gives it (184): when the host read it,
/// whether it is fresh at the host's clock, and each of its windows; or,
/// where the account names one that cannot be read, why, in words that
/// name no path.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadingStatus {
    Read {
        read_at: Seconds,
        fresh: bool,
        windows: Vec<WindowStatus>,
    },
    Unread {
        unread: String,
    },
}

/// A window of an account's reading as status gives it (184, 185): its
/// name, how much of it is used, in hundredths of a percent and a
/// percentage in the JSON form, when it resets, none where the reading
/// gives none, and whether it holds the account past its threshold.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowStatus {
    pub name: String,
    #[serde(with = "as_percentage")]
    pub used: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resets_at: Option<Seconds>,
    pub past: bool,
}

/// What holds an account past its threshold: the window, and when it
/// stops holding it, its reset, or, where it names none, when the reading
/// stops being fresh (185).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Past {
    pub window: String,
    pub until: Seconds,
}

/// Hundredths of a percent as a percentage, a whole number where it is
/// one, as a reading's file writes it (184).
pub fn percentage(hundredths: u32) -> String {
    match hundredths % 100 {
        0 => (hundredths / 100).to_string(),
        _ => format!("{}", f64::from(hundredths) / 100.0),
    }
}

/// Hundredths of a percent given as a percentage in the JSON form, a
/// whole number where it is one (184).
mod as_percentage {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(hundredths: &u32, serializer: S) -> Result<S::Ok, S::Error> {
        match hundredths % 100 {
            0 => serializer.serialize_u32(hundredths / 100),
            _ => serializer.serialize_f64(f64::from(*hundredths) / 100.0),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
        let percent = f64::deserialize(deserializer)?;
        if !(0.0..=100.0).contains(&percent) {
            return Err(serde::de::Error::custom(format!(
                "{percent} is not a percentage from 0 to 100"
            )));
        }
        Ok((percent * 100.0).round() as u32)
    }
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
    /// Each answer the stage's current start took in its session, in the
    /// order it took them, each with the question it answered (42, 56,
    /// 198).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub answers: Vec<SessionAnswer>,
}

/// An answer a stage's start took in its session: the stop-short's words
/// it answered, whole, and the answer's words, the act it came by, when
/// flume took it and when it was sent and received there (42, 54, 56,
/// 198).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionAnswer {
    pub question: Option<String>,
    #[serde(flatten)]
    pub answer: AnswerHeld,
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
    /// For the agent of a conversation's stage read lost, one the terminal
    /// brought back in its last start's pane, that flume left it there,
    /// sent nothing; written only where it did (56, 77; core.10).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub left_in_pane: bool,
    /// Its program's own settings beyond the keys its settings give: how
    /// its profile passed them, that they were not read back, and each
    /// file a layer named for its program; none where no layer gave its
    /// program an entry (56, 195).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub own: Option<OwnStatus>,
    /// Its effort as its start passed it, with its layer, and the
    /// account's variable that outranks it; none where no layer gave one
    /// (56, 197).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<EffortStatus>,
}

/// An agent's effort at its start: `word`, as its layer wrote it, and
/// `layer`, the first of the eight that gave it; `passed`, the value its
/// profile passed, which takes its program's words as written; `read_back`,
/// whether its program read it back, which no profile does; and
/// `outranked_by`, the variable its program reads over the effort where
/// the agent's account's environment names it, by its name and never its
/// value, none otherwise (56, 197; core.30).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffortStatus {
    pub word: String,
    pub layer: Layer,
    pub passed: String,
    pub read_back: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outranked_by: Option<String>,
}

/// An agent's own settings at its start beyond their keys: `passed`, its
/// command's arguments naming the files it lists, none where it passed
/// no key; `read_back`, whether its program read them back, which no
/// release does; and `files`, each file a layer named for its program,
/// most specific first (56, 195, 196).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnStatus {
    pub passed: Option<String>,
    pub read_back: bool,
    #[serde(default)]
    pub files: Vec<OwnFileStatus>,
}

/// A file a layer named for an agent's program: its layer, its path as
/// written and as read, and each key its program's profile reserves left
/// out of it, unread, with the setting or the account that owns it (56,
/// 195).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnFileStatus {
    pub layer: Layer,
    pub written: String,
    pub read: String,
    #[serde(default)]
    pub left_out: Vec<Reserved>,
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
    /// `instructions`, `mode`, `keep_open` or `env.<NAME>`, an environment
    /// variable's value [`flume_seams::SET`] and its layer `account`.
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

/// A branch a tidy kept, named once: the commits it holds whose change
/// its base lacks, or, for a base place's branch the merge queue kept at
/// its close, the merge queue's words (67, 206).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchKept {
    pub place: PlaceId,
    pub branch: String,
    pub commits: Vec<Revision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

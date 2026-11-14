//! A place's held state: one document per place in the store (62, 73 to
//! 76, 103), enough on its own that a restart loses nothing (77). It
//! keeps no fact of a deliverable, a change or a submission between asks
//! (58); a stage's end keeps only what was reported at that moment (43).

use crate::{
    is_address, Binding, Declared, Forecast, Given, Holds, Identity, Kind, KindFrom, KindIdentity,
    KindSettings, LabelHeld, PassedOver, PlaceId, PlaceRef, ProfileHeld, Prompt, Readiness,
    RepositoryName, Revision, Seconds, Secret, SessionHeld, Settings, StopKind, StreamHeld,
    Unsettled, Verdict,
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
    /// The role's address `<stream>/<role>` a role's run serves, by
    /// which a command run in it collects for the address (132); none
    /// for any other place.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Each hold on a stage or the submission, the launch's and those put
    /// after it, with its words and since when, a retry naming its stage
    /// or the submission ending it and nothing else; and the hold on the
    /// move the place waits on, with its words, since when and its bound,
    /// ended by the move made, a retry or a drop, and lapsed at its bound
    /// (47, 49, 52, 130).
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
    /// woken at the last settle, by the address: no source declaring a run
    /// for it, or the wake's launch refused; gone once it is woken (132,
    /// 193; core.28).
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
    /// Each start of the place's that carried its stage's conversation and
    /// did not begin, in order, with what happened: its stage started
    /// afresh after it (56, 146, S79).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub uncarried: Vec<UncarriedStart>,
    /// Each terminal a start of the place's opened its agent in, each
    /// once, as it was first opened in, so the place is tidied in each
    /// whatever later starts of its stages record; none in a place
    /// written before, whose stages' starts say theirs (137, 190).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub terminals: Vec<ProfileHeld>,
    /// For a base place whose kind names a fix, the newest forecast a
    /// settle read of its branch against its base, and when, written only
    /// where its outcome, its head or its base's revision changed; none
    /// for any other place, and in a held state written before (56, 203,
    /// 210).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forecast: Option<ForecastHeld>,
    /// For a base place, each fix flume launched for it, in order of its
    /// number, each written before its launch and its end written by the
    /// fix's own settle; none for any other place, and in a held state
    /// written before (203).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixes: Vec<BaseFix>,
}

/// The newest forecast a settle read of a base place's branch, and when
/// (56, 203, 210).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForecastHeld {
    pub at: Seconds,
    pub forecast: Forecast,
}

/// Why flume launched a base place's fix: a forecast in conflict with
/// its base, by the head and the base's revision it was formed on; or a
/// failed or conflicting submission, by the merge queue's id. Each is
/// acted on once (203).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FixCause {
    Forecast { head: Revision, onto: Revision },
    Failed { submission: String },
    Conflicted { submission: String },
}

/// How a base place's fix ended, as its own settle wrote it onto its base
/// place: live while it stands, landed with its commit, dropped, or its
/// launch refused with the refusal's words (203).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "ended", rename_all = "snake_case")]
pub enum FixEnded {
    Live,
    Landed { commit: Revision },
    Dropped,
    Refused { reason: String },
}

/// One fix flume launched for a base place: its number from 1, its
/// place's name, its place, why, when it was written to be launched, and
/// how it ended (203).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseFix {
    pub number: u32,
    pub name: String,
    pub place: PlaceId,
    pub cause: FixCause,
    pub launched: Seconds,
    #[serde(flatten)]
    pub ended: FixEnded,
}

/// The kind a base place's fixes run as, as its launch resolved the name
/// its kind gives, and where it came from; written once, at the launch,
/// and never resolved again (21, 23, 203).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixKind {
    pub identity: KindIdentity,
    pub from: KindFrom,
}

/// The base place flume launched a fix for, the fix's number and why
/// (203).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixOf {
    pub place: PlaceId,
    pub number: u32,
    pub cause: FixCause,
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

/// A start that carried its stage's conversation and did not begin: its
/// stage and number, when a settle read it so, and the words saying what
/// happened, naming no session (146, 179, S79).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UncarriedStart {
    pub stage: String,
    pub start: u32,
    pub at: Seconds,
    pub words: String,
}

/// How long a place waits on its caller under no hold's words before it
/// is stalled, where its launch sets no bound (130).
pub const STALL_AFTER: Seconds = 5 * 60;

fn stall_after() -> Seconds {
    STALL_AFTER
}

impl HeldState {
    /// The recipients each end stands pending for, as `owing` read the
    /// host's deliveries, in order, each named once (43; weir 105).
    pub fn pending_for(&self, owing: &Owing) -> Vec<String> {
        let mut pending: Vec<String> = vec![];
        for end in &self.ends {
            for recipient in end.pending(&self.place, owing) {
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

    /// The hold on the move the place waits on, if it is held and has not
    /// lapsed (130).
    pub fn waiting_hold(&self) -> Option<&Hold> {
        self.holds
            .iter()
            .find(|hold| hold.on == HoldOn::Waiting && hold.lapsed.is_none())
    }
}

/// A hold on a stage, the submission or the move the place waits on, with
/// its caller's words and since when, and, on the waiting move, its bound
/// and when it lapsed (47, 130).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hold {
    #[serde(flatten)]
    pub on: HoldOn,
    /// What the hold waits for, in its caller's words; the caller's to
    /// make good, since flume watches no event for it (130).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub words: Option<String>,
    pub since: Seconds,
    /// For a hold on the waiting move, how long it holds from its since,
    /// none where it holds until the move is made (130).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound: Option<Seconds>,
    /// When the settle that read its bound passed lapsed it, after which it
    /// holds nothing and the place stalls again (130).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lapsed: Option<Seconds>,
}

/// What a hold holds: a stage, the submission (47), or the move the place
/// waits on its caller for (130).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "on", rename_all = "snake_case")]
pub enum HoldOn {
    Stage { stage: String },
    Submission,
    Waiting,
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
    /// A retry, or done; or a person's answer to the agent program's
    /// dialog, which is no act of flume's.
    pub act: Act,
    /// The stage the retry names; none where it names the submission.
    pub stage: Option<String>,
    pub why: Waits,
    pub since: Option<Seconds>,
    pub words: Option<String>,
    /// For an answer, the time the stage's agent is kept open for it
    /// until, while it is; none for any other move, and in a move written
    /// before (130, 198).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_until: Option<Seconds>,
    /// For an answer, when and why the stage's agent kept open for it was
    /// let go, once a settle recorded it; none for any other move, and in
    /// a move written before (130, 198).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<SessionClosed>,
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
            (Act::Answer, _) => "a person's answer to its agent's dialog".into(),
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
    /// A message to a place's agent, written `message` and read from
    /// `tell`, the verb it replaced, so held state that recorded an
    /// answer by a tell reads it as a message's (53; record 0069).
    #[serde(alias = "tell")]
    Message,
    Needs,
    Priority,
    Hold,
    /// A person's answer, in the agent's terminal, to a dialog its
    /// program shows before it takes its prompt: what a place waits on
    /// for a dialog, never an act a caller asks of flume (34, 35).
    Answer,
}

/// Why a place waits on its caller (56, 130).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Waits {
    /// A stage, or the submission, on hold (47).
    OnHold,
    /// A stage ended short needing an answer, or having found work that
    /// sends the place back to no fix round before it (42, 131).
    Answer,
    /// A stage whose agent was lost (45).
    Lost,
    /// A stage whose prompt was never begun (38).
    NeverBegun,
    /// A stage whose agent's program shows a person a dialog before it
    /// takes its prompt, which flume does not answer (34, 35).
    Dialog,
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
    /// A base place with no place on its branch live, whose submission
    /// its caller has not asked for (203).
    Unsubmitted,
}

/// An end of a place, recorded once by its key when a settle or an act
/// reads it, with what 43 names of it, its delivery to each recipient
/// recorded in the host's deliveries and owed until that recipient
/// confirms it (43, 74; weir 100, 102).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndHeld {
    /// What ended, which is the end's key with what was read of it.
    #[serde(flatten)]
    pub of: EndOf,
    /// When flume read it.
    pub read_at: Seconds,
    /// The old form's deliveries, to whom, when and by what, as an end
    /// written before its delivery was weir's held them: read and never
    /// added to, emptied once the host's deliveries hold them, and still
    /// written as `deliveries` so the form keeps the field (43, 61).
    #[serde(default)]
    pub deliveries: Vec<Delivery>,
    /// Each notice of it claimed before its send, to whom and when, so
    /// two settles at once send one notice; kept once the terminal
    /// answered it sent, taken back where it was not sent, and none in an
    /// end written before (43, 75).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub noticing: Vec<NoticeClaim>,
    /// For a stop-short needing an answer, the answer that answered it,
    /// once one did; none for any other end, and in an end written before
    /// (42).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<AnswerHeld>,
    /// For a stop-short needing an answer, the answer being taken for it
    /// now, claimed before its words are handed to the agent kept open or
    /// a fresh agent is started to carry them, so a second answer at once
    /// is refused with nothing typed; gone once the answer is recorded or
    /// not taken, and none in an end written before (42, 53, 75).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim: Option<AnswerClaim>,
}

/// What the host's deliveries hold of the ends of its places, as one read
/// of them gives it at one time, by each delivery's key: read once for a
/// settle, a wait, an inbox or a status (43; weir 105, 111; core.15).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Owing {
    pub deliveries: BTreeMap<String, EndDelivery>,
    /// Each name a recipient has confirmed under (weir 106).
    pub known: std::collections::BTreeSet<String>,
}

/// One end's delivery as the host's deliveries hold it at the time it was
/// read: its readers, when it expires, each reader that has not confirmed
/// it, each confirmation and each hand-over (43; weir 100 to 106).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EndDelivery {
    pub key: String,
    pub readers: Vec<String>,
    /// Its message as recorded, every reader's: the end's notice but the
    /// way to confirm it, which a notice adds for the recipient it goes
    /// to (43; weir 100; core.11).
    pub message: String,
    pub expires_at: Seconds,
    /// Each reader that has not confirmed it, in its readers' order.
    pub owed: Vec<OwedTo>,
    /// Each reader's confirmation, in the order added.
    pub confirmed: Vec<ConfirmedBy>,
    /// Each hand-over, to whom, when and by what, in the order added.
    pub handovers: Vec<Delivery>,
}

/// A recipient an end is owed to: whether its delivery has expired, and
/// whether anything was ever confirmed under the recipient's name (43, 56;
/// weir 104, 106).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwedTo {
    pub recipient: String,
    pub expired: bool,
    pub known: bool,
}

/// A recipient's confirmation of an end, and when (43; weir 102).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfirmedBy {
    pub recipient: String,
    pub at: Seconds,
}

/// A notice of an end claimed for a recipient before its send, and when,
/// and whether its send is still unanswered (43, 75).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoticeClaim {
    pub recipient: String,
    pub at: Seconds,
    /// True from the claim until the terminal answers the notice sent, so
    /// a settle that reads the claim meanwhile records no hand-over of a
    /// notice its send may yet fail to make; false, written by no field,
    /// once answered sent, and in a claim written before, which was read
    /// as answered. A claim a settle ended mid-send leaves sending still
    /// holds the recipient's notice, since the terminal may have typed it,
    /// and is recorded by no settle: the end waits for the recipient's
    /// inbox, a wait or its session's start (43, 75; weir 103).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub sending: bool,
}

/// An answer being taken for a stop-short: who gives it and the key it
/// named, as its command names them, when it was claimed, and until when
/// the claim holds, past which a later answer may take the stop-short, as
/// where the command that claimed it was ended before it let go (42, 75).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnswerClaim {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<EndKey>,
    pub at: Seconds,
    pub until: Seconds,
}

/// An answer to a stop-short: the caller's words, the act they came by,
/// when flume took them, who gave them and the key they named, and, where
/// they were handed to the session of the agent kept open for them, when
/// they were sent and received there (42, 53, 54, 198).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnswerHeld {
    pub words: String,
    pub act: Act,
    pub at: Seconds,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<Handed>,
    /// Who gave the answer, as the answering command's recipient named
    /// it; none where it named none, and in an answer written before (42).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub by: Option<String>,
    /// The key of the stop-short the answer named, taken only while the
    /// place waited on it; none where it named none, and in an answer
    /// written before (42).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<EndKey>,
}

/// An answer handed to the session of an agent kept open: when it was
/// last sent, and when the agent's own record read it received (38, 54).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Handed {
    pub sent_at: Seconds,
    pub received_at: Seconds,
}

/// An agent kept open for its answer let go: when a settle read it so,
/// why, when the agent was ended, none until it is, and whether an agent
/// the terminal brought back bare was left in its conversation's pane,
/// written only where one was (31, 77, 96, 134, 146, 198).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionClosed {
    pub at: Seconds,
    pub by: ClosedBy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<Seconds>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub left_in_pane: bool,
}

/// Why an agent kept open for its answer was let go: its time passed, its
/// caller stopped it, it was read stopped, its session ended by its own
/// record or an agent the terminal brought back bare stands in its pane,
/// or its account was spent (51, 77, 96, 134, 146, 198).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosedBy {
    Time,
    Stopped,
    AgentEnded,
    AccountSpent,
}

fn is_zero(count: &u32) -> bool {
    *count == 0
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

    /// The recipients of the end, of a place whose recipients are
    /// `recipients`: for a stop-short that named a role, that role's
    /// address alone; otherwise those, then, for a stall, the role the
    /// stop-short it waits on named and the answerer behind the place's
    /// stream, each once; and for a drop's or a done's end, those that are
    /// no role's address, each once, so no role's run is woken and no
    /// tidied document kept for a role alone (41, 43, 52, 114, 130, 132,
    /// 193).
    pub fn recipients(&self, recipients: &[String]) -> Vec<String> {
        match &self.of {
            EndOf::Stage { end, .. } => {
                if let Some(addressee) = &end.addressee {
                    return vec![addressee.clone()];
                }
            }
            EndOf::Over { .. } => {
                let mut every: Vec<String> = vec![];
                for recipient in recipients {
                    if !is_address(recipient) && !every.contains(recipient) {
                        every.push(recipient.clone());
                    }
                }
                return every;
            }
            EndOf::Submission { .. } | EndOf::Stall { .. } => {}
        }
        let mut every = recipients.to_vec();
        if let EndOf::Stall {
            answerer,
            stop_short,
            ..
        } = &self.of
        {
            let addressee = stop_short
                .as_ref()
                .and_then(|stop_short| stop_short.addressee.as_ref());
            for also in [addressee, answerer.as_ref()].into_iter().flatten() {
                if !every.contains(also) {
                    every.push(also.clone());
                }
            }
        }
        every
    }

    /// The end's delivery as the host's deliveries hold it, of the place
    /// `place`; none where none is recorded, as for an end with no
    /// recipient or of a document written before (43; weir 100).
    pub fn delivery<'a>(&self, place: &PlaceRecord, owing: &'a Owing) -> Option<&'a EndDelivery> {
        owing.deliveries.get(&place.delivery_key(&self.of)?)
    }

    /// The recipients the end stands pending for, of the place `place`:
    /// each reader of its delivery that has not confirmed it, as `owing`
    /// read it; or, where no delivery of it is recorded yet, each of its
    /// recipients the old form delivered nothing to, as its record would
    /// read (43, 130; weir 102, 105).
    pub fn pending(&self, place: &PlaceRecord, owing: &Owing) -> Vec<String> {
        match self.delivery(place, owing) {
            Some(delivery) => delivery
                .owed
                .iter()
                .map(|owed| owed.recipient.clone())
                .collect(),
            None => self
                .recipients(&place.recipients)
                .into_iter()
                .filter(|recipient| !self.delivered_to(recipient))
                .collect(),
        }
    }

    /// Whether the end stands pending for `recipient`, of the place
    /// `place` (43).
    pub fn pending_to(&self, place: &PlaceRecord, owing: &Owing, recipient: &str) -> bool {
        self.pending(place, owing)
            .iter()
            .any(|pending| pending == recipient)
    }

    /// Each hand-over of the end, to whom, when and by what, in the order
    /// added: its delivery's, or, where none is recorded yet, the old
    /// form's (43, 61; weir 103).
    pub fn handovers(&self, place: &PlaceRecord, owing: &Owing) -> Vec<Delivery> {
        match self.delivery(place, owing) {
            Some(delivery) => delivery.handovers.clone(),
            None => self.deliveries.clone(),
        }
    }

    /// Whether anything handed the end to `recipient`: a hand-over of its
    /// delivery, a notice claimed for it, or the old form's delivery (43,
    /// 132; weir 103; core.28).
    pub fn handed_to(&self, place: &PlaceRecord, owing: &Owing, recipient: &str) -> bool {
        self.handovers(place, owing)
            .iter()
            .any(|handover| handover.recipient == recipient)
            || self.delivered_to(recipient)
            || self
                .noticing
                .iter()
                .any(|claim| claim.recipient == recipient)
    }

    /// Whether a notice carries it to a recipient a pane holds, and a wake
    /// is owed for it: every end but a drop's or a done's, which stands
    /// pending until its recipient collects it by the inbox or a wait (43,
    /// 52, 114, 132).
    pub fn noticed(&self) -> bool {
        !matches!(self.of, EndOf::Over { .. })
    }
}

/// What ended: a stage's start, ended, lost, never begun or stuck; a
/// submission's outcome from the merge queue; a stall, each with what
/// flume read of it; or the place's work, by a drop or a done (43, 45,
/// 52, 114, 130, 133).
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
        /// How many answers the start had taken in its session when the
        /// end was read, so a stop-short after an answer is an end of its
        /// own; zero, written by no field, for every other end and in an
        /// end written before (42, 43).
        #[serde(default, skip_serializing_if = "is_zero")]
        answered: u32,
        /// What was read of it: the head or report, the agent's words and
        /// whether it needs an answer or found work, or what flume read of
        /// an agent lost, never begun or stuck; boxed, so an end stays
        /// small beside a submission's or a stall's.
        end: Box<StageEnd>,
    },
    Submission {
        /// The submit's number (48).
        submit: u32,
        /// The outcome, the commit landed as or the merge queue's words.
        verdict: Verdict,
    },
    Stall {
        /// Since when the place waited on its caller under no hold's
        /// words: since the move, or since the hold on it lapsed (130).
        since: Seconds,
        /// The move it waits on.
        waiting: String,
        /// The answerer behind the place's stream as its sources declared
        /// it when the stall was read, delivered the stall besides the
        /// place's recipients; none where no role stood behind the stream,
        /// where the place is that answerer's own run, or where an earlier
        /// flume wrote none (130, 132, 193).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        answerer: Option<String>,
        /// The stop-short the place waits on, where the move answers one,
        /// so a reader who never saw the place has the question whole;
        /// none where it answers none, or an earlier flume wrote none
        /// (41, 42, 130, 193); boxed, so an end stays small.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        stop_short: Option<Box<StopShortWaited>>,
        /// Whether the flume that recorded it read the answerer and the
        /// stop-short: false only for a stall an earlier flume recorded,
        /// which the first settle that reads it, while the place still
        /// waits on its caller under no hold's words, reads them for
        /// (130, 193).
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        answerer_read: bool,
        /// The words of the hold on the waiting move whose bound passed,
        /// stalling the place again; none for a stall under no hold
        /// (130, 193).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lapsed: Option<String>,
    },
    /// The end of the place's work by a drop or a done, recorded for those
    /// alone, in the write that makes the work over: a landing is its
    /// submission's end. No notice carries it, and it stands pending for
    /// the place's recipients that are no role's address (43, 52, 114).
    Over { over: Over },
}

/// The stop-short a stalled place waits on, as the stall read it: its
/// words, whether it needs an answer or found work, its end's key, and the
/// role's address it named, whole, none where it named none (41, 42, 43,
/// 130, 193).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopShortWaited {
    pub words: Option<String>,
    #[serde(default)]
    pub needs: Option<StopKind>,
    /// The key of the stop-short's end, as callers name it (42, 43).
    pub key: EndKey,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addressee: Option<String>,
}

impl EndOf {
    /// The end's key as held state records it, by which a settle records
    /// the end once: what ended, without what was read of it. A stage's
    /// names the answers its start had taken only where there are some, so
    /// every key written before reads as it did (43). Callers see it as
    /// [`EndKey`], made from the same parts.
    pub fn key(&self) -> String {
        match self {
            EndOf::Stage {
                stage,
                start,
                state,
                quiet_from,
                answered,
                ..
            } => {
                let key = format!(
                    "stage {stage} {start} {state:?} {}",
                    quiet_from.map(|at| at.to_string()).unwrap_or_default()
                );
                if *answered == 0 {
                    key
                } else {
                    format!("{key} {answered}")
                }
            }
            EndOf::Submission { submit, verdict } => {
                format!("submission {submit} {}", KeyOutcome::of(verdict).word())
            }
            EndOf::Stall { since, .. } => format!("stall {since}"),
            EndOf::Over { over } => format!("over {}", over.word()),
        }
    }

    /// The end's key as callers see it and name it, made from the parts
    /// [`EndOf::key`] is made from (42, 43).
    pub fn end_key(&self) -> EndKey {
        match self {
            EndOf::Stage {
                stage,
                start,
                state,
                answered,
                ..
            } => EndKey::Stage {
                stage: stage.clone(),
                start: *start,
                state: *state,
                answered: *answered,
            },
            EndOf::Submission { submit, verdict } => EndKey::Submission {
                submit: *submit,
                outcome: KeyOutcome::of(verdict),
            },
            EndOf::Stall { since, .. } => EndKey::Stall { since: *since },
            EndOf::Over { over } => EndKey::Over { over: over.clone() },
        }
    }
}

/// An end's key as callers see it and name it, one token:
/// `stage/<stage>/<start>/<state>`, with `/<answered>` after it where the
/// start had taken that many answers in its session when the end was read,
/// so a second stop-short of one start kept open is a key of its own;
/// `submission/<n>/<outcome>`; `stall/<since>`; or `over/<dropped|done>`,
/// a landing keyed by its submission alone. Each notice, collected
/// end, waiting place and status gives it, and an answer names the
/// stop-short it answers by it. It is read back from that token alone,
/// refused in any other form, naming the forms (42, 43, 198).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum EndKey {
    Stage {
        stage: String,
        start: u32,
        state: StageState,
        answered: u32,
    },
    Submission {
        submit: u32,
        outcome: KeyOutcome,
    },
    Stall {
        since: Seconds,
    },
    Over {
        over: Over,
    },
}

/// The forms an end's key takes, as a refusal names them (42, 43).
pub const END_KEY_FORMS: &str = "stage/<stage>/<start>/<state>, with /<answered> after it where its start had taken answers in its session, submission/<n>/<outcome>, stall/<since>, or over/<dropped|done>";

impl std::fmt::Display for EndKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EndKey::Stage {
                stage,
                start,
                state,
                answered,
            } => {
                write!(f, "stage/{stage}/{start}/{}", state.word())?;
                if *answered != 0 {
                    write!(f, "/{answered}")?;
                }
                Ok(())
            }
            EndKey::Submission { submit, outcome } => {
                write!(f, "submission/{submit}/{}", outcome.word())
            }
            EndKey::Stall { since } => write!(f, "stall/{since}"),
            EndKey::Over { over } => write!(f, "over/{}", over.word()),
        }
    }
}

impl std::str::FromStr for EndKey {
    type Err = String;

    /// Reads a key from its token, refusing any other form, a number
    /// written otherwise than the key writes it among them, naming the
    /// forms (42, 43).
    fn from_str(given: &str) -> Result<EndKey, String> {
        let refused = || format!("`{given}` is no end's key: a key is {END_KEY_FORMS}");
        let number = |part: &str| part.parse::<u64>().map_err(|_| refused());
        let small = |part: &str| part.parse::<u32>().map_err(|_| refused());
        let parts: Vec<&str> = given.split('/').collect();
        let key = match parts.as_slice() {
            ["stage", stage, start, state] | ["stage", stage, start, state, _]
                if !stage.is_empty() =>
            {
                EndKey::Stage {
                    stage: stage.to_string(),
                    start: small(start)?,
                    state: StageState::from_word(state).ok_or_else(refused)?,
                    answered: match parts.get(4) {
                        Some(answered) => small(answered)?,
                        None => 0,
                    },
                }
            }
            ["submission", submit, outcome] => EndKey::Submission {
                submit: small(submit)?,
                outcome: KeyOutcome::from_word(outcome).ok_or_else(refused)?,
            },
            ["stall", since] => EndKey::Stall {
                since: number(since)?,
            },
            ["over", "dropped"] => EndKey::Over {
                over: Over::Dropped,
            },
            ["over", "done"] => EndKey::Over { over: Over::Done },
            _ => return Err(refused()),
        };
        if key.to_string() != given {
            return Err(refused());
        }
        Ok(key)
    }
}

impl TryFrom<String> for EndKey {
    type Error = String;

    fn try_from(given: String) -> Result<EndKey, String> {
        given.parse()
    }
}

impl From<EndKey> for String {
    fn from(key: EndKey) -> String {
        key.to_string()
    }
}

/// A submission's outcome as its end's key names it (43).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyOutcome {
    Landed,
    Failed,
    Conflict,
    Withdrawn,
    Refused,
}

impl KeyOutcome {
    const ALL: [KeyOutcome; 5] = [
        KeyOutcome::Landed,
        KeyOutcome::Failed,
        KeyOutcome::Conflict,
        KeyOutcome::Withdrawn,
        KeyOutcome::Refused,
    ];

    /// The outcome of the merge queue's verdict.
    pub fn of(verdict: &Verdict) -> KeyOutcome {
        match verdict {
            Verdict::Landed { .. } => KeyOutcome::Landed,
            Verdict::Failed { .. } => KeyOutcome::Failed,
            Verdict::Conflict { .. } => KeyOutcome::Conflict,
            Verdict::Withdrawn => KeyOutcome::Withdrawn,
            Verdict::Refused { .. } => KeyOutcome::Refused,
        }
    }

    /// The outcome's word in a key.
    pub fn word(self) -> &'static str {
        match self {
            KeyOutcome::Landed => "landed",
            KeyOutcome::Failed => "failed",
            KeyOutcome::Conflict => "conflict",
            KeyOutcome::Withdrawn => "withdrawn",
            KeyOutcome::Refused => "refused",
        }
    }

    fn from_word(word: &str) -> Option<KeyOutcome> {
        KeyOutcome::ALL
            .into_iter()
            .find(|outcome| outcome.word() == word)
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

/// What handed an end over: a notice through the terminal, a wait by
/// place, the inbox, or a session's start collecting it (43, C.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveredBy {
    Notice,
    Wait,
    Inbox,
    SessionStart,
}

impl DeliveredBy {
    /// The way a hand-over records it by in the host's deliveries (43;
    /// weir 103).
    pub fn way(self) -> &'static str {
        match self {
            DeliveredBy::Notice => "notice",
            DeliveredBy::Wait => "wait",
            DeliveredBy::Inbox => "inbox",
            DeliveredBy::SessionStart => "session-start",
        }
    }

    /// The hand-over a way of the host's deliveries names; none for a way
    /// flume records none by.
    pub fn of_way(way: &str) -> Option<DeliveredBy> {
        [
            DeliveredBy::Notice,
            DeliveredBy::Wait,
            DeliveredBy::Inbox,
            DeliveredBy::SessionStart,
        ]
        .into_iter()
        .find(|by| by.way() == way)
    }
}

/// Where a launch took a place's recipients from, the first of these
/// that gave any: the recipients the launch named, `FLUME_RECIPIENT` in
/// the launching command's environment, the session the launching
/// command ran in as the terminal names it, or the answerer behind the
/// place's stream, with the source that declares it (7, 190, 193).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "from", rename_all = "kebab-case")]
pub enum RecipientsFrom {
    Launch,
    Environment,
    Session,
    Answerer {
        source: String,
    },
    /// A base place's fix takes its base place's recipients, who asked for
    /// the work (43, 203; core.54).
    BasePlace,
}

/// Where a launch read its base's revision (9, 56, 126, C.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BaseFrom {
    /// Where the merge queue's profile says the base lives, the merge
    /// queue landing it.
    MergeQueue,
    /// `origin/<base>`, after flume's own fetch.
    Origin,
    /// The host's own branch, the repository having no remote `origin`.
    NoOrigin,
    /// The host's own branch, `origin` holding no such branch.
    OriginLacks,
}

/// What a launch fixed of a place (8, 21).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceRecord {
    pub id: PlaceId,
    pub repository: String,
    /// The repository's address, `<owner>/<name>`, as the launch read it
    /// from its clone's origin; none where the clone had no forge origin
    /// or the place was launched before flume kept one. No other part of
    /// the origin's URL is kept (8, 56, 143, C.7; core.30).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The kind as it resolved at the launch, its stages with it (21).
    pub kind: Kind,
    pub declared: Declared,
    /// Where the kind came from, as the launch read it; none in a held
    /// state written before a launch kept it (19, 21).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind_from: Option<KindFrom>,
    pub name: String,
    pub branch: String,
    pub base: String,
    /// The base's revision the place was cut from (9).
    pub base_revision: Revision,
    /// Where the launch read the base's revision; none for a place
    /// launched before flume said (56, C.1).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_from: Option<BaseFrom>,
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
    /// The repository's settings for every kind, its `[agents]`, as the
    /// launch read them at the base's revision; none in a held state
    /// written before (21, 32, 195).
    #[serde(default)]
    pub repository_agents: Settings,
    pub hold: Vec<String>,
    pub hold_submission: bool,
    /// The words of each hold the launch put, by stage name (7, 130).
    #[serde(default)]
    pub hold_words: BTreeMap<String, String>,
    /// The recipients of the place's ends, as the launch fixed them (7,
    /// 8, 43, 193).
    #[serde(default)]
    pub recipients: Vec<String>,
    /// Where the launch took its recipients from, the first of the four
    /// that gave any; none for a role's run, which has none, and for a
    /// place launched before flume kept it (7, 56, 193).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients_from: Option<RecipientsFrom>,
    /// The stream the place runs in, fixed at its launch, and where it
    /// came from (8, 143).
    pub stream: StreamHeld,
    /// The intent the place serves, as its launch named it or its
    /// repository and base give it (147).
    pub intent: IntentKey,
    /// For a base place whose kind names a fix, the kind its fixes run
    /// as, resolved at its launch; none for any other place, and in a
    /// record written before (21, 23, 203).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixes_as: Option<FixKind>,
    /// For a place flume launched as a base place's fix, that base place,
    /// the fix's number and why; none for any other place (203).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fix_of: Option<FixOf>,
    /// The token the place's launch took, by which its ends' deliveries
    /// are keyed, so two launches of one place never share a key; none in
    /// a document written before, which its first settle's write gives
    /// one (43, C.7; weir 101).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
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
    /// The repository as the host's configuration names it: by the
    /// address its launch kept, else by its folder (143, C.7).
    pub fn repository_name(&self) -> RepositoryName {
        RepositoryName {
            address: self.address.clone(),
            folder: self.repository.clone(),
        }
    }

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

    /// The key of the delivery of its end `of`, `<kind>--<name>/<token>/<end
    /// key>`, so two launches of one place never share one; none while it
    /// holds no token, as a document written before (43, C.7; weir 101).
    pub fn delivery_key(&self, of: &EndOf) -> Option<String> {
        let token = self.token.as_ref()?;
        Some(format!("{}/{token}/{}", self.short(), of.end_key()))
    }

    /// The first line of its brief that holds anything, trimmed; none
    /// where it has no brief or a blank one (43; design 9).
    pub fn brief_line(&self) -> Option<&str> {
        self.brief
            .as_deref()?
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty())
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

impl Over {
    /// Its word: `landed`, `dropped` or `done` (43, 63).
    pub fn word(&self) -> &'static str {
        match self {
            Over::Landed { .. } => "landed",
            Over::Dropped => "dropped",
            Over::Done => "done",
        }
    }
}

/// Why a place whose work is over still stands (64 to 66, 68).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kept {
    Holds {
        holds: Holds,
    },
    Teardown {
        step: String,
        words: String,
    },
    AgentLive {
        identity: String,
    },
    Unreadable {
        reason: String,
    },
    /// A process of the host runs the conversation of its last
    /// conversation start, each as the agent profile's live read it, or
    /// what could not be read where it could not read whether one does
    /// (64, 207; core.18).
    SessionRuns {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        processes: Vec<LiveProcess>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unread: Option<String>,
    },
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
    const ALL: [StageState; 16] = [
        StageState::NotReached,
        StageState::OnHold,
        StageState::WaitingOnReadiness,
        StageState::WaitingForRoom,
        StageState::Refused,
        StageState::Starting,
        StageState::Started,
        StageState::Begun,
        StageState::NeverBegun,
        StageState::Stuck,
        StageState::Lost,
        StageState::Stopped,
        StageState::Delivered,
        StageState::Repeating,
        StageState::EndedShort,
        StageState::Finished,
    ];

    /// The state's word in an end's key, `ended-short` for
    /// [`StageState::EndedShort`] (43).
    pub fn word(self) -> &'static str {
        match self {
            StageState::NotReached => "not-reached",
            StageState::OnHold => "on-hold",
            StageState::WaitingOnReadiness => "waiting-on-readiness",
            StageState::WaitingForRoom => "waiting-for-room",
            StageState::Refused => "refused",
            StageState::Starting => "starting",
            StageState::Started => "started",
            StageState::Begun => "begun",
            StageState::NeverBegun => "never-begun",
            StageState::Stuck => "stuck",
            StageState::Lost => "lost",
            StageState::Stopped => "stopped",
            StageState::Delivered => "delivered",
            StageState::Repeating => "repeating",
            StageState::EndedShort => "ended-short",
            StageState::Finished => "finished",
        }
    }

    fn from_word(word: &str) -> Option<StageState> {
        StageState::ALL
            .into_iter()
            .find(|state| state.word() == word)
    }

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
    /// For a stage waiting after its agent stopped at its account's usage
    /// limit, or after a caller's retry of a conversation reported lost,
    /// that start, why it ended and whether the next may carry its
    /// conversation; none for any other wait, so no other start carries
    /// one (30, 49, 146, 179).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carry: Option<CarryFrom>,
    /// For a stage whose start would carry a conversation, or start a
    /// lost conversation afresh, each process of the host the agent
    /// profile's live last read running that conversation; none where it
    /// read none, and in a wait written before (49, 207).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub live: Vec<LiveProcess>,
    /// What the agent profile's live last could not read, so the stage
    /// waits as if a process ran its conversation, naming a variable and
    /// never a path; none where it read, and in a wait written before
    /// (207; core.30).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_unread: Option<String>,
}

/// A stage's wait to carry the conversation of a start whose agent
/// stopped at its account's usage limit or was reported lost: that start,
/// why it ended, whether the next start may carry its conversation, and
/// why not where it may not (49, 146, 179, S79).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CarryFrom {
    /// The start whose agent stopped at its account's usage limit, or
    /// whose conversation was reported lost.
    pub start: Identity,
    /// Why that start ended, spent in a wait written before.
    #[serde(default, skip_serializing_if = "CarryCause::is_spent")]
    pub cause: CarryCause,
    /// Whether the next start may carry its conversation.
    pub carry: bool,
    /// Why it does not, where it may not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
}

/// Why the start whose conversation a start carries ended: its agent
/// stopped at its account's usage limit, or its conversation was reported
/// lost and its caller retried it (49, 146, 179). Spent is written by no
/// field, so a wait or a conversation written before reads as spent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CarryCause {
    #[default]
    Spent,
    Lost,
}

impl CarryCause {
    pub fn name(self) -> &'static str {
        match self {
            CarryCause::Spent => "spent",
            CarryCause::Lost => "lost",
        }
    }

    pub fn is_spent(&self) -> bool {
        *self == CarryCause::Spent
    }
}

/// A process of the host running a conversation, as an agent profile's
/// live reads it: its id, and when it started by the host's clock (207).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveProcess {
    pub pid: u32,
    pub started: Seconds,
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
    /// The role's address, whole, a stop-short named as the one its words
    /// are for, which its end goes to in place of the place's recipients;
    /// none where it named none (41, 43, 132).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addressee: Option<String>,
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
    /// For a conversation's stage read lost while an agent the terminal
    /// brought back stands in its last start's pane, that flume left that
    /// agent there, sent nothing (43, 77; core.10).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub left_in_pane: bool,
    /// For a stop-short needing an answer whose agent was kept open for
    /// it, the time it is kept open until: its stop-short's time and its
    /// start's `keep_open`; none where it was not kept open, and in an end
    /// written before (198).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_until: Option<Seconds>,
    /// For an agent kept open, when and why a settle read it let go, once
    /// one did (198).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<SessionClosed>,
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
    /// `effort`, `instructions` or `env.<NAME>`, a variable's from its
    /// account and its value [`crate::SET`], so no value is kept (32, 56,
    /// 144, 197).
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
    /// Each role's address, whole, its prompt listed for a stop-short to
    /// name, in the prompt's order, read from the host's sources at the
    /// start; none where it listed none, and in a start written before
    /// (28, 41, 193).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub may_name: Vec<String>,
    /// The command of flume's opening the agent, as the host names it,
    /// until the terminal's open answers: while it lives, the terminal may
    /// still be bringing the program up, and no other command sends the
    /// prompt or answers the program's questions (34, 35, 37).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opening: Option<String>,
    /// The question the agent's program shows a person before it takes
    /// its prompt, which flume does not answer, as the terminal last read
    /// it: the place waits on a person while it shows (34, 35).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dialog: Option<Dialog>,
    /// For a start after its stage's agent stopped at its account's usage
    /// limit, or after a caller's retry of a conversation reported lost,
    /// whether it carries that agent's conversation, or why not; none for
    /// any other start (49, 56, 146, 179).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation: Option<Conversation>,
    /// The session its agent opened in, with where it came from; none
    /// for an agent opened beside a pane, and in a start written before
    /// (56, 107, 143).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<SessionHeld>,
    /// The label its place started under, with where it came from, which
    /// its agent counts toward until it ends; none where it had none, and
    /// in a start written before (56, 180, 181).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<LabelHeld>,
    /// The terminal its agent opened in, with where it came from; none in
    /// a start written before, which is the host's (56, 190).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal: Option<ProfileHeld>,
    /// Its program's own settings beyond the keys its settings hold: each
    /// file a layer named for its program, and how its command passed the
    /// keys; none where no layer gave its program an entry, and in a start
    /// written before (56, 195, 196).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub own: Option<OwnHeld>,
    /// How many answers its agent took in its session, each handed to it
    /// while it was kept open; zero, written by no field, in a start
    /// written before (42, 198).
    #[serde(default, skip_serializing_if = "is_zero")]
    pub answered: u32,
}

/// A start's own settings beyond its keys: each file a layer named for
/// its program, most specific first, and how its command passed the keys,
/// its arguments naming the files it lists, none where it passed none or
/// until it is opened (56, 195, 196).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnHeld {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<crate::OwnFile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passed: Option<String>,
}

/// Whether a start carries the conversation of its stage's start before
/// it, whose agent stopped at its account's usage limit or was reported
/// lost: that start's number, why it ended, and why not where it does not
/// (49, 56, 146, 179).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conversation {
    /// The number of the start whose agent stopped or was lost.
    pub from: u32,
    /// Why that start ended, spent in a conversation written before.
    #[serde(default, skip_serializing_if = "CarryCause::is_spent")]
    pub cause: CarryCause,
    pub carried: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
}

/// A question an agent's program shows a person before it takes its
/// prompt, its words as the terminal shows them, and since when flume
/// read it shown (34, 35).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dialog {
    pub words: String,
    pub since: Seconds,
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
/// without the secret (core.30). Its mode is the value the start's
/// profile passes, none where the settings give no mode, and in a record
/// written before (161, 162). Its session is the one the start resumes,
/// none where it carries no conversation, and in a record written before
/// (161, 179).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StartRecord {
    pub program: String,
    pub account: Option<String>,
    pub env: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

/// The account a start runs under: its pool, its name, and whether the
/// intent's affinity, the pool's strategy or the conversation it carries
/// chose it (144, 145, 147).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountHeld {
    pub pool: String,
    pub name: String,
    pub from: AccountFrom,
}

/// What chose a start's account (145, 147): the intent's affinity, the
/// pool's strategy, or, for a lost conversation's start, the account its
/// lost start ran under, which holds its conversation (145, 179).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountFrom {
    Affinity,
    Strategy,
    Conversation,
}

/// A move from one account to another: of a stage, whose start went to
/// another account than its last, or of an intent, whose affinity moved;
/// why, and when. The agents after it start without the earlier
/// account's cache (56, 145 to 147). A stage's move after its agent
/// stopped at its account's usage limit says whether its conversation
/// went with it, none for any other (56, 179). A move off an account past
/// its threshold names the window that held it and when that window
/// resets, none for any other and on a move recorded before (185).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountMove {
    #[serde(flatten)]
    pub on: MoveOn,
    pub from: String,
    pub to: String,
    pub why: MoveWhy,
    pub at: Seconds,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carried: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub until: Option<Seconds>,
}

/// What moved between accounts: a stage, or the intent's affinity for
/// the program whose account moved, none on a move recorded before an
/// intent kept an account per program (147).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "on", rename_all = "snake_case")]
pub enum MoveOn {
    Stage {
        stage: String,
    },
    Intent {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        program: Option<String>,
    },
}

/// Why a start went to another account: the one before was spent, past
/// its threshold, or at its own bound (145, 146, 185).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MoveWhy {
    Spent,
    Threshold,
    Full,
}

impl MoveWhy {
    pub fn name(self) -> &'static str {
        match self {
            MoveWhy::Spent => "spent",
            MoveWhy::Threshold => "threshold",
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
    /// withdrawal of it is told from a hand's (99, 112); for a request,
    /// the entry its intake took it in as, once read.
    #[serde(default)]
    pub id: Option<String>,
    /// The request the merge queue answered flume's last submit with, a
    /// forge's pull request by its number, where it takes the base's work
    /// as requests: the submission is the entry an intake takes it in as
    /// (124, C.6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    /// The newest submission of the branch the merge queue held when the
    /// place was launched, and again before each submit of flume's, by
    /// its id: an earlier place's of the same name, or the entry of the
    /// place's submission before, which this place never reads as its
    /// own (3, 99, 124, 132).
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
    /// Each branch the place's inbox passed over at the last settle, with
    /// its words, its outcomes left pending for a later one; the
    /// outcomes of every other branch were acted on (133).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub passed_over: Vec<PassedOver>,
}

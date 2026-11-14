//! What the core hands on of a place's trace, its records, the host's
//! reading and flume's own warnings, and the one interface it hands them
//! across (216 to 220; design 6). Nothing here names a crate or a format
//! of telemetry's, so the core names none (core.1, core.2): the binary
//! sends what it is handed, and the fakes keep it.

use crate::Seconds;
use std::collections::BTreeMap;

/// The signals a command sends, as the host's `[telemetry]` and
/// OpenTelemetry's own variables leave them on: none where flume sends
/// nothing (216).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Sends {
    pub traces: bool,
    pub metrics: bool,
    pub logs: bool,
}

impl Sends {
    /// Every signal.
    pub const ALL: Sends = Sends {
        traces: true,
        metrics: true,
        logs: true,
    };

    /// Whether any signal goes, so an agent is given its place's facts
    /// (219).
    pub fn any(self) -> bool {
        self.traces || self.metrics || self.logs
    }
}

/// An attribute's value as a signal names it: words, a number or a yes
/// or no.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Text(String),
    Number(i64),
    Flag(bool),
}

impl From<&str> for Value {
    fn from(text: &str) -> Value {
        Value::Text(text.to_string())
    }
}

impl From<String> for Value {
    fn from(text: String) -> Value {
        Value::Text(text)
    }
}

impl From<u32> for Value {
    fn from(number: u32) -> Value {
        Value::Number(number.into())
    }
}

impl From<bool> for Value {
    fn from(flag: bool) -> Value {
        Value::Flag(flag)
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Text(text) => f.write_str(text),
            Value::Number(number) => write!(f, "{number}"),
            Value::Flag(flag) => write!(f, "{flag}"),
        }
    }
}

/// A signal's attributes, by key.
pub type Attributes = BTreeMap<String, Value>;

/// One signal the core hands on, once the write that records it landed
/// (217): a span with the times the held state keeps, a record of a
/// change under the span it concerns, the host's reading at a settle of
/// every place, or a line of flume's own log (217, 218, 220).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Signal {
    /// A span that ended: the place's, a start's or a wait's, by its
    /// trace's 32 hex digits and its own 16, its parent's none for the
    /// place's own (217).
    Span {
        trace: String,
        span: String,
        parent: Option<String>,
        name: String,
        from: Seconds,
        to: Seconds,
        attributes: Attributes,
    },
    /// A change of a place, under the span it concerns, its body its
    /// event's name alone (217, 218).
    Record {
        trace: String,
        span: String,
        at: Seconds,
        event: String,
        attributes: Attributes,
    },
    /// The places standing and each stream's agents, read by a settle of
    /// every place (220).
    Host(HostReading),
    /// A line of flume's own log, in its own words (218).
    Warning { at: Seconds, words: String },
}

/// The host's places and streams as a settle of every place read them
/// (220, 121, 208).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HostReading {
    pub at: Seconds,
    /// The places standing, counted by kind, state and whom they wait on.
    pub places: Vec<PlacesStanding>,
    /// Each stream's agents running and its room.
    pub streams: Vec<StreamAgents>,
}

/// How many places of a kind stand in one state waiting on one party,
/// none where they wait on nobody (220).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlacesStanding {
    pub kind: String,
    pub state: String,
    pub on: Option<String>,
    pub count: u64,
}

/// A stream's agents running and the room left in it, none where neither
/// it nor the host states a bound (70, 220).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StreamAgents {
    pub stream: String,
    pub running: u32,
    pub room: Option<u32>,
}

/// Where the core hands its signals: the binary's sends them as its
/// command ends, a fake keeps them (216; design 6).
pub trait Telemetry: Send + Sync {
    /// The signals this command sends, chosen once as it began.
    fn sends(&self) -> Sends;

    /// Hand on one signal; nothing comes back (216).
    fn send(&self, signal: Signal);
}

/// Telemetry that sends nothing: an engine given none (216).
#[derive(Clone, Copy, Debug, Default)]
pub struct Off;

impl Telemetry for Off {
    fn sends(&self) -> Sends {
        Sends::default()
    }

    fn send(&self, _signal: Signal) {}
}

/// The repository, by its address, else its folder's last name (218).
pub const REPOSITORY: &str = "flume.repository";
/// The kind's name.
pub const KIND: &str = "flume.kind";
/// The kind's version.
pub const KIND_VERSION: &str = "flume.kind.version";
/// The place's name.
pub const PLACE: &str = "flume.place";
/// The base the place is cut from.
pub const BASE: &str = "flume.base";
/// The place's stream.
pub const STREAM: &str = "flume.stream";
/// The intent: the launch's name, else `base:<repository>:<base>`.
pub const INTENT: &str = "flume.intent";
/// The mark, where the place has one.
pub const MARK: &str = "flume.mark";
/// The stage a signal concerns.
pub const STAGE: &str = "flume.stage";
/// The start's number.
pub const START: &str = "flume.start";
/// A repeating stage's repetition.
pub const REPETITION: &str = "flume.repetition";
/// The agent's program.
pub const PROGRAM: &str = "flume.program";
/// The conversation's id, as the agent seam reads it.
pub const SESSION_ID: &str = "session.id";
/// The account, `<pool>/<account>`.
pub const ACCOUNT: &str = "flume.account";
/// How a start ended.
pub const START_END: &str = "flume.start.end";
/// Whether a start that ended short needs an answer or found work.
pub const START_NEEDS: &str = "flume.start.needs";
/// Whom a wait is on.
pub const WAIT_ON: &str = "flume.wait.on";
/// The move a wait is for.
pub const WAIT_MOVE: &str = "flume.wait.move";
/// Whether a hold stood over a wait.
pub const WAIT_HELD: &str = "flume.wait.held";
/// Whether a wait stalled.
pub const WAIT_STALLED: &str = "flume.wait.stalled";
/// A place's state in the list's words.
pub const STATE: &str = "flume.state";
/// A record's change.
pub const EVENT_NAME: &str = "event.name";

/// The keys a metric may carry, and no other: none names a place, an
/// intent, a mark, a conversation or an account (220; core.56).
pub const METRIC_KEYS: [&str; 8] = [
    REPOSITORY, KIND, STAGE, STREAM, START_END, WAIT_ON, WAIT_MOVE, STATE,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_sends_no_signal() {
        let off: &dyn Telemetry = &Off;
        assert_eq!(off.sends(), Sends::default());
        assert!(!off.sends().any());
        off.send(Signal::Warning {
            at: 1,
            words: "nothing keeps this".into(),
        });
        assert!(Sends::ALL.any());
        assert!(Sends {
            logs: true,
            ..Sends::default()
        }
        .any());
    }

    #[test]
    fn no_metric_key_names_a_place_intent_mark_conversation_or_account() {
        for key in [
            PLACE,
            INTENT,
            MARK,
            SESSION_ID,
            ACCOUNT,
            BASE,
            START,
            KIND_VERSION,
            START_NEEDS,
        ] {
            assert!(!METRIC_KEYS.contains(&key), "{key}");
        }
    }
}

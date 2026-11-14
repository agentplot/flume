//! The terminal seam's answers (94 to 98, 122, 132, 136 to 139, 187, 201,
//! 7, C.5), what its configuration lacks for flume's plugin (122), and an
//! agent's state.

use crate::Seconds;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// What an agent is doing (vocabulary: agent state).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentState {
    Working,
    Waiting,
    Asking,
    Stopped,
    Unknown,
}

/// open: an agent running in the place under its identity; a start held
/// at a question its program asks before it starts, or a program that
/// did not come up within the terminal's bound, each with what the
/// terminal shows; or a refusal (94, 35). `carried` says the command it
/// ran carried the prompt, as a terminal that cannot send runs it (38);
/// `printed` is the command a person runs in the place, where the
/// terminal prints it rather than runs it, which status gives with the
/// agent (C.5).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Opened {
    Opened {
        carried: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        printed: Option<String>,
    },
    /// Held at a question the program asks before it starts, which
    /// flume does not answer: the start waits on a person while the
    /// terminal reads it asking (34, 35).
    Held {
        screen: String,
    },
    /// The program did not come up within the terminal's bound (35).
    NotComeUp {
        screen: String,
    },
    Refused {
        reason: String,
    },
}

/// Where the terminal opens an agent: a split beside the pane the
/// settings name; in the terminal's part of the name given, a session
/// under a terminal that keeps sessions; or in the terminal's own (107,
/// 143). It names no terminal (core.1).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "in", rename_all = "snake_case")]
pub enum OpenIn {
    /// A split beside the pane the settings name (107).
    Beside { pane: String },
    /// The terminal's part of this name (107, 143).
    Session { name: String },
    /// The terminal's own (107).
    Own,
}

/// Where the session an agent opens in came from: the launch's
/// settings, its stream's word for its kind's stage, its stream's word
/// for its kind, its stream's `session`, its stream's name, or, for the
/// host's default stream naming none, the terminal's own (32, 107, 143,
/// 199). For a role's address a standing agent holds, the session it is
/// found in comes from the entry's own `session`, `role`, or else from
/// its stream as a start's does (132, C.7).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SessionFrom {
    Launch,
    StreamStage,
    StreamKind,
    Stream,
    StreamName,
    Own,
    Role,
}

/// The session an agent opens in, by its name, none for the terminal's
/// own, and where it came from (56, 107, 143).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionHeld {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub from: SessionFrom,
}

/// send: the text delivered as input, or a refusal; never left typed
/// (95).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sent {
    Sent,
    Refused { reason: String },
}

/// send to a recipient's session by its name: the text delivered as
/// input, no session found by that name, or a refusal; never left typed
/// (43, 95).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToSession {
    Sent,
    NotFound,
    Refused { reason: String },
}

/// close: the agent's end, or a refusal because it did not end (97).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Closed {
    Closed,
    Refused { reason: String },
}

/// A watch as status gives it: a caller settling every place on a bound
/// it states, or none (61, 208). No terminal keeps one or settles: a
/// terminal's answer in status gives its watch none (122).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Watch {
    Bound { seconds: Seconds },
    None,
}

/// tidy: what the terminal made to hold a gone place's agents, and the
/// places of its repository on its base, closed; and each thing kept,
/// named with what the terminal did not make that holds it (137, 138).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalTidy {
    pub closed: Vec<String>,
    pub kept: Vec<String>,
}

/// The count of the places of a repository on one base, by what they do
/// (139): running, asking, waiting on their caller, and stalled.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseCount {
    pub running: u32,
    pub asking: u32,
    pub waiting_on_caller: u32,
    pub stalled: u32,
}

impl std::fmt::Display for BaseCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} running · {} asking · {} waiting on caller · {} stalled",
            self.running, self.asking, self.waiting_on_caller, self.stalled
        )
    }
}

/// The clues show gives a terminal for a place: its current stage, that
/// stage's state as a word, the agent the stage runs if one is live, the
/// count of the places on its base, and the short form of the place's
/// line, `code 1/2 · run 1 · tasks 2/5 · working`, which a terminal shows
/// beside the agent; none in clues written before (139).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clues {
    pub stage: String,
    pub state: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<crate::Identity>,
    pub base: BaseCount,
    #[serde(default)]
    pub line: String,
}

/// show: the clues shown, the terminal showing none, or a failure, which
/// fails nothing (139).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Shown {
    Shown,
    ShowsNone,
    Failed { reason: String },
}

/// Where the terminal shows a place's clues, each named in the
/// terminal's own words: what holds the place's agents, the pane of its
/// live agent, and what holds the places of its repository on its base;
/// none where it holds none; and each thing that holds the places of its
/// repository on its base, one for each of the terminal's sessions that
/// has one, none where it holds none and in an answer written before.
/// Status gives them; the settle alone shows the clues, and a plugin of
/// the terminal's copies none (107, 139, C.5).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShownAt {
    #[serde(default)]
    pub place: Option<String>,
    #[serde(default)]
    pub agent: Option<String>,
    #[serde(default)]
    pub base: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bases: Vec<String>,
}

/// The command a person runs to see a part of the terminal: its program
/// by its full path, its arguments, and the variables it needs of its
/// environment with their values, which the window passes on and no
/// answer gives (187; core.30).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewCommand {
    pub program: String,
    pub arguments: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub environment: BTreeMap<String, String>,
}

/// attach: for a window, the command that shows the part, the part
/// started first where it did not run; for the terminal's own app, the
/// app told to show it; or a refusal where the terminal holds no part a
/// person can be shown or offers no such view (187).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Attached {
    Command { command: ViewCommand },
    Shown,
    Refused { reason: String },
}

/// runs, for each part asked: whether it runs now, with the name the
/// terminal knows it by; or that the terminal holds no part, or could
/// not read it, in its words (201).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionRuns {
    Runs { name: String },
    NotRunning { name: String },
    NoPart { reason: String },
    Unread { reason: String },
}

/// An agent a part of the terminal holds that carries a name: that name,
/// the session name `send_to` and `session_state` reach it by, and its
/// state as the terminal reads it (95, 96, 132, C.5).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedAgent {
    pub name: String,
    pub at: String,
    pub state: AgentState,
}

/// agents in a part: each agent the part holds that carries a name, none
/// for a part that does not run; or why the part could not be read, in
/// the terminal's words, a terminal that holds no agent a name finds
/// among them. The terminal decides nothing of names: two of one name
/// are both given, and the core counts them (132, C.5).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentsIn {
    Read { agents: Vec<NamedAgent> },
    Unread { reason: String },
}

/// What the seam's default `agents_in` answers: the terminal holds no
/// agent a name finds (132, C.5).
pub const NO_NAMED_AGENTS: &str = "the terminal holds no agent a name finds";

/// What the seam's default `relay` answers: the terminal offers no relay
/// (222).
pub const NO_RELAY: &str = "the terminal offers no relay";

/// A key flume names, as a person's input gives it to a relay (222): a
/// key by flume's own name, never a terminal's, so no caller names a
/// multiplexer's keys and a profile maps it to its terminal's own
/// (core.1). It is read from and written as its name: any of `ctrl+`,
/// `alt+` and `shift+`, in that order, before the key's own name.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Key {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub name: KeyName,
}

/// A key's own name, without its leading `ctrl+`, `alt+` and `shift+`
/// (222): one flume names, a function key by its number, or one
/// printable character.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyName {
    Enter,
    Tab,
    Esc,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Up,
    Down,
    Left,
    Right,
    Space,
    /// `f1` to `f12`.
    F(u8),
    /// One printable character, neither a control nor a space.
    Char(char),
}

/// flume's key names, as a refusal of any other gives them (222).
pub const KEY_NAMES: &str = "enter, tab, esc, backspace, delete, insert, home, end, pageup, pagedown, up, down, left, right, space, f1 to f12, or any one printable character, each led by any of ctrl+, alt+ and shift+ in that order";

impl KeyName {
    const NAMED: [(KeyName, &'static str); 15] = [
        (KeyName::Enter, "enter"),
        (KeyName::Tab, "tab"),
        (KeyName::Esc, "esc"),
        (KeyName::Backspace, "backspace"),
        (KeyName::Delete, "delete"),
        (KeyName::Insert, "insert"),
        (KeyName::Home, "home"),
        (KeyName::End, "end"),
        (KeyName::PageUp, "pageup"),
        (KeyName::PageDown, "pagedown"),
        (KeyName::Up, "up"),
        (KeyName::Down, "down"),
        (KeyName::Left, "left"),
        (KeyName::Right, "right"),
        (KeyName::Space, "space"),
    ];

    /// Every key name flume gives, bare: the named keys, then `f1` to
    /// `f12`; a printable character is any one (222).
    pub fn named() -> Vec<KeyName> {
        let mut named: Vec<KeyName> = Self::NAMED.iter().map(|(name, _)| *name).collect();
        named.extend((1..=12).map(KeyName::F));
        named
    }

    fn read(given: &str) -> Option<KeyName> {
        if given == "escape" {
            return Some(KeyName::Esc);
        }
        if let Some((name, _)) = Self::NAMED.iter().find(|(_, word)| *word == given) {
            return Some(*name);
        }
        if let Some(number) = given
            .strip_prefix('f')
            .and_then(|number| number.parse::<u8>().ok())
            .filter(|number| (1..=12).contains(number) && format!("f{number}") == given)
        {
            return Some(KeyName::F(number));
        }
        let mut chars = given.chars();
        match (chars.next(), chars.next()) {
            (Some(one), None) if !one.is_control() && !one.is_whitespace() => {
                Some(KeyName::Char(one))
            }
            _ => None,
        }
    }
}

impl std::fmt::Display for KeyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyName::F(number) => write!(f, "f{number}"),
            KeyName::Char(one) => write!(f, "{one}"),
            named => {
                let word = Self::NAMED
                    .iter()
                    .find(|(name, _)| name == named)
                    .map(|(_, word)| *word)
                    .unwrap_or_default();
                f.write_str(word)
            }
        }
    }
}

impl Key {
    /// A key by its own name alone, led by no `ctrl+`, `alt+` or `shift+`.
    pub fn bare(name: KeyName) -> Key {
        Key {
            ctrl: false,
            alt: false,
            shift: false,
            name,
        }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ctrl {
            f.write_str("ctrl+")?;
        }
        if self.alt {
            f.write_str("alt+")?;
        }
        if self.shift {
            f.write_str("shift+")?;
        }
        write!(f, "{}", self.name)
    }
}

impl std::str::FromStr for Key {
    type Err = String;

    /// Reads a key from flume's name for it, `escape` read as `esc`, and
    /// refuses any other name, naming it and flume's names (222).
    fn from_str(given: &str) -> Result<Key, String> {
        let mut rest = given;
        let mut lead = |word: &str| match rest.strip_prefix(word) {
            Some(after) if !after.is_empty() => {
                rest = after;
                true
            }
            _ => false,
        };
        let ctrl = lead("ctrl+");
        let alt = lead("alt+");
        let shift = lead("shift+");
        KeyName::read(rest)
            .map(|name| Key {
                ctrl,
                alt,
                shift,
                name,
            })
            .ok_or_else(|| format!("{given} is no key flume names: flume names {KEY_NAMES}"))
    }
}

impl TryFrom<String> for Key {
    type Error = String;

    fn try_from(given: String) -> Result<Key, String> {
        given.parse()
    }
}

impl From<Key> for String {
    fn from(key: Key) -> String {
        key.to_string()
    }
}

/// A person's input to a relay, as the person typed it: text, or keys by
/// flume's names typed in order (222). It is written `{"text": …}` or
/// `{"keys": [ … ]}`, the lines a relay's caller writes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Typed {
    Text(String),
    Keys(Vec<Key>),
}

/// A relay's read: what the terminal shows of the agent now, as a
/// person's view of it would show it; that the terminal holds the agent
/// no more; or the terminal's words where it did not answer (96, 136,
/// 222).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelayScreen {
    Shows { screen: String },
    Gone,
    Unread { reason: String },
}

/// A terminal's relay of one agent, the handle `Terminal::relay` gives
/// (222). It holds what the profile found at the ask, so each read is one
/// call. It makes and keeps nothing in the terminal, and dropping it ends
/// nothing there: the relay closes nothing it did not open, which is
/// nothing (221, 222).
pub trait Relay: Send {
    /// The bound the profile states within which a change of what the
    /// terminal shows is given again: the least time between two reads
    /// (222).
    fn every(&self) -> std::time::Duration;

    /// What the terminal shows of the agent now, as a person's view of it
    /// would; gone where the terminal holds the agent no more; or the
    /// terminal's words where it did not answer (96, 136, 222).
    fn screen(&mut self) -> RelayScreen;

    /// The person's input given to the agent as the person typed it:
    /// nothing typed cleared first and no receipt read, which no send of
    /// flume's is (95); or the terminal's refusal in its words (107,
    /// 222).
    fn typed(&mut self, input: &Typed) -> Result<(), String>;
}

/// The terminal seam (B.5), kept once for the host. The guarantee: open
/// never runs a second agent under an identity already live, and close
/// never reports an end that did not happen (98). It offers no operation
/// that settles, keeps watch over the host or answers a bound: whether
/// flume runs attended is read from the host's watch alone (119, 122,
/// 208).
pub trait Terminal: Send + Sync {
    /// An agent running `command` in the place under `identity`, where
    /// `at` says, and carrying the session name `session`, a role run's
    /// address, where one is given, so a send or a state read by that name
    /// reaches it; refused where a live agent carries that name already.
    /// What the terminal keeps of the start it keeps in the place's own
    /// `folder` (94, 98, 107, 128, 132).
    fn open(
        &self,
        place: &crate::PlaceRef,
        folder: &str,
        command: &crate::AgentCommand,
        identity: &crate::Identity,
        at: &OpenIn,
        session: Option<&str>,
    ) -> Opened;

    /// `text` delivered as input the agent receives, any text it holds
    /// typed cleared first, so no answer leaves text typed (95).
    fn send(&self, agent: &crate::Identity, text: &str) -> Sent;

    /// Whether the terminal delivers text to a running agent at all, as a
    /// tell's send does: where it does not, every send is refused with
    /// nothing typed, and no agent is kept open for an answer (53, 95,
    /// 198, C.5).
    fn sends(&self) -> bool;

    /// The agent's state: stopped only when its program no longer runs,
    /// unknown for anything the terminal cannot classify; asking where
    /// it reads the program waiting on a person, which, before the agent
    /// takes its prompt, is a dialog its program shows (35, 96).
    fn state(&self, agent: &crate::Identity) -> AgentState;

    /// `text` delivered as input to the recipient's session named
    /// `session`, as the terminal names sessions or as an agent's open
    /// named it, any text it holds typed cleared first; not found where
    /// the terminal holds no such session, never guessed (43, 95, 98).
    fn send_to(&self, session: &str, text: &str) -> ToSession;

    /// The state of the recipient's session named `session`, read as an
    /// agent's is; none where the terminal holds no such session (43, 96).
    fn session_state(&self, session: &str) -> Option<AgentState>;

    /// The agents the part `session` holds, the terminal's own where
    /// none, each that carries a name, by that name, with the session
    /// name `send_to` and `session_state` reach it by and its state as
    /// the terminal reads it; or why the part could not be read. A part
    /// that does not run holds none, and nothing is started; nothing is
    /// kept of what was read (132, C.5; core.13, core.15). The default
    /// answers not read: the terminal holds no agent a name finds.
    fn agents_in(&self, _session: Option<&str>) -> AgentsIn {
        AgentsIn::Unread {
            reason: NO_NAMED_AGENTS.into(),
        }
    }

    /// What the terminal shows of the agent, for status and a prompt
    /// never begun; none where it shows nothing (136, 35, 43).
    fn screen(&self, agent: &crate::Identity) -> Option<String>;

    /// For a start whose agent has not said it started: the folder-trust
    /// question its open was given, answered as open answers it where the
    /// terminal shows it now, since the program may ask it after open has
    /// returned; whether it was answered. A terminal that reads no screen
    /// answers nothing (34, 35).
    fn answer_trust(&self, _agent: &crate::Identity) -> bool {
        false
    }

    /// The agent's end (97).
    fn close(&self, agent: &crate::Identity) -> Closed;

    /// What the terminal made to hold the agents of `place`, now gone,
    /// closed, and what it made for the places of its repository on its
    /// base once it holds nothing of a live place; never anything it did
    /// not make (137, 138).
    fn tidy(&self, place: &crate::PlaceRef) -> TerminalTidy;

    /// Each thing the terminal made and keeps for what it did not make,
    /// named with what holds it, for status (56, 137, 138).
    fn kept(&self) -> Vec<String>;

    /// Clues beside the place's agents, display only: nothing flume
    /// decides reads them back (139).
    fn show(&self, place: &crate::PlaceRef, clues: &Clues) -> Shown;

    /// Where the terminal shows the place's clues, the agent's by
    /// `agent` where one is live, as it last recorded them, asking
    /// nothing of the terminal's program; none of each where it shows
    /// none (139).
    fn shown_at(&self, place: &crate::PlaceRef, agent: Option<&crate::Identity>) -> ShownAt;

    /// What the terminal's own configuration lacks for flume's plugin
    /// there, each in words a person acts on, which status gives when
    /// asked and flume says to no person by itself; none where it lacks
    /// nothing, and none by default, for a terminal with no plugin of
    /// flume's (72, 122).
    fn setup(&self) -> Vec<String> {
        vec![]
    }

    /// The session the calling command runs in, as the terminal names
    /// it, a launch's default recipient; none where it names none (7).
    fn session(&self) -> Option<String>;

    /// The part named `session`, the terminal's own where none, shown to
    /// a person in `view`: a terminal view answers a command or a
    /// refusal, and the app's view shown or a refusal (187).
    fn attach(&self, session: Option<&str>, view: crate::View) -> Attached;

    /// Whether each part of `sessions`, each named or the terminal's own
    /// where none, runs now, read together in one reading of the
    /// terminal, one answer for each in the order asked, starting
    /// nothing, within a bound the profile states (98, 200, 201).
    fn runs(&self, sessions: &[Option<&str>]) -> Vec<SessionRuns>;

    /// A relay of `agent`: a handle that reads what the terminal shows of
    /// it, as a person's view would, within the bound the handle states,
    /// and types a person's input into it with nothing cleared first and
    /// no wait for receipt; or a refusal in the terminal's words where it
    /// holds no such agent or offers no relay (221, 222). The default
    /// refuses with `NO_RELAY`, so a profile offers a relay only by
    /// building one. The handle makes nothing in the terminal, and
    /// dropping it ends nothing there (221).
    fn relay(&self, _agent: &crate::Identity) -> Result<Box<dyn Relay + '_>, String> {
        Err(NO_RELAY.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The seam-contracts scenario "Key names" (222).
    #[test]
    fn key_names_are_read_and_written_as_flume_names_them() {
        for (given, written) in [
            ("ctrl+c", "ctrl+c"),
            ("shift+tab", "shift+tab"),
            ("escape", "esc"),
            ("f12", "f12"),
            ("ctrl+alt+x", "ctrl+alt+x"),
            ("é", "é"),
            ("space", "space"),
        ] {
            let key: Key = given.parse().unwrap_or_else(|refused| panic!("{refused}"));
            assert_eq!(key.to_string(), written, "{given}");
            assert_eq!(
                serde_json::to_string(&key).unwrap(),
                format!("\"{written}\""),
                "serde writes {given} by its name"
            );
            let read: Key = serde_json::from_str(&format!("\"{given}\"")).unwrap();
            assert_eq!(read, key, "serde reads {given} as FromStr does");
        }
        for given in ["alt+ctrl+x", "f13", "hyper+x"] {
            let refused = given.parse::<Key>().expect_err(given);
            assert!(refused.starts_with(given), "{refused}");
            for name in [
                "enter",
                "esc",
                "pagedown",
                "f1 to f12",
                "printable",
                "ctrl+, alt+ and shift+",
            ] {
                assert!(refused.contains(name), "{given}: {refused} names {name}");
            }
            assert!(
                serde_json::from_str::<Key>(&format!("\"{given}\"")).is_err(),
                "serde refuses {given}"
            );
        }
        for named in KeyName::named() {
            let key = Key::bare(named);
            assert_eq!(key.to_string().parse::<Key>(), Ok(key.clone()), "{key}");
        }
        for refused in ["", "ctrl+", "f0", "f01", " ", "\t", "ab", "Enter"] {
            assert!(refused.parse::<Key>().is_err(), "{refused:?}");
        }
        assert_eq!(
            "ctrl++".parse::<Key>().map(|key| key.to_string()),
            Ok("ctrl++".into()),
            "a printable `+` led by ctrl+"
        );
    }

    /// A terminal of the test's own that builds every operation of the
    /// seam but relay.
    struct NoRelay;

    impl Terminal for NoRelay {
        fn open(
            &self,
            _: &crate::PlaceRef,
            _: &str,
            _: &crate::AgentCommand,
            _: &crate::Identity,
            _: &OpenIn,
            _: Option<&str>,
        ) -> Opened {
            Opened::Opened {
                carried: false,
                printed: None,
            }
        }
        fn send(&self, _: &crate::Identity, _: &str) -> Sent {
            Sent::Sent
        }
        fn sends(&self) -> bool {
            true
        }
        fn state(&self, _: &crate::Identity) -> AgentState {
            AgentState::Waiting
        }
        fn send_to(&self, _: &str, _: &str) -> ToSession {
            ToSession::Sent
        }
        fn session_state(&self, _: &str) -> Option<AgentState> {
            Some(AgentState::Waiting)
        }
        fn agents_in(&self, _: Option<&str>) -> AgentsIn {
            AgentsIn::Read { agents: vec![] }
        }
        fn screen(&self, _: &crate::Identity) -> Option<String> {
            Some("ready".into())
        }
        fn answer_trust(&self, _: &crate::Identity) -> bool {
            true
        }
        fn close(&self, _: &crate::Identity) -> Closed {
            Closed::Closed
        }
        fn tidy(&self, _: &crate::PlaceRef) -> TerminalTidy {
            TerminalTidy::default()
        }
        fn kept(&self) -> Vec<String> {
            vec![]
        }
        fn show(&self, _: &crate::PlaceRef, _: &Clues) -> Shown {
            Shown::Shown
        }
        fn shown_at(&self, _: &crate::PlaceRef, _: Option<&crate::Identity>) -> ShownAt {
            ShownAt::default()
        }
        fn setup(&self) -> Vec<String> {
            vec![]
        }
        fn session(&self) -> Option<String> {
            None
        }
        fn attach(&self, _: Option<&str>, _: crate::View) -> Attached {
            Attached::Shown
        }
        fn runs(&self, sessions: &[Option<&str>]) -> Vec<SessionRuns> {
            sessions
                .iter()
                .map(|_| SessionRuns::Runs { name: "own".into() })
                .collect()
        }
    }

    /// The seam-contracts scenario "A terminal that builds no relay"
    /// (222).
    #[test]
    fn a_terminal_that_builds_no_relay_refuses_it() {
        let agent = crate::Identity {
            repository: "r1".into(),
            place: "fix--one".into(),
            stage: "code".into(),
            start: 1,
        };
        match NoRelay.relay(&agent) {
            Err(refused) => assert_eq!(refused, "the terminal offers no relay"),
            Ok(_) => panic!("a terminal that builds no relay relayed"),
        }
        assert_eq!(NO_RELAY, "the terminal offers no relay");
    }
}

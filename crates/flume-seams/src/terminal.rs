//! The terminal seam's answers (94 to 98, 122, 136 to 139, 7), and an
//! agent's state.

use crate::Seconds;
use serde::{Deserialize, Serialize};

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
    /// flume does not answer (34, 35).
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

/// watch: the terminal settling its places on a bound it states, or no
/// watch (122).
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
/// stage's state as a word, the agent the stage runs if one is live, and
/// the count of the places on its base (139).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clues {
    pub stage: String,
    pub state: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<crate::Identity>,
    pub base: BaseCount,
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

/// The terminal seam (B.5), kept once for the host. The guarantee: open
/// never runs a second agent under an identity already live, and close
/// never reports an end that did not happen (98).
pub trait Terminal: Send + Sync {
    /// An agent running `command` in the place under `identity`, beside
    /// the pane the settings name, if any, and carrying the session name
    /// `session`, a role run's address, where one is given, so a send or
    /// a state read by that name reaches it; refused where a live agent
    /// carries that name already. What the terminal keeps of the start it
    /// keeps in the place's own `folder` (94, 98, 107, 128, 132).
    fn open(
        &self,
        place: &crate::PlaceRef,
        folder: &str,
        command: &crate::AgentCommand,
        identity: &crate::Identity,
        beside: Option<&str>,
        session: Option<&str>,
    ) -> Opened;

    /// `text` delivered as input the agent receives, any text it holds
    /// typed cleared first, so no answer leaves text typed (95).
    fn send(&self, agent: &crate::Identity, text: &str) -> Sent;

    /// The agent's state: stopped only when its program no longer runs,
    /// unknown for anything the terminal cannot classify (96).
    fn state(&self, agent: &crate::Identity) -> AgentState;

    /// `text` delivered as input to the recipient's session named
    /// `session`, as the terminal names sessions or as an agent's open
    /// named it, any text it holds typed cleared first; not found where
    /// the terminal holds no such session, never guessed (43, 95, 98).
    fn send_to(&self, session: &str, text: &str) -> ToSession;

    /// The state of the recipient's session named `session`, read as an
    /// agent's is; none where the terminal holds no such session (43, 96).
    fn session_state(&self, session: &str) -> Option<AgentState>;

    /// What the terminal shows of the agent, for status and a prompt
    /// never begun; none where it shows nothing (136, 35, 43).
    fn screen(&self, agent: &crate::Identity) -> Option<String>;

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

    /// The watch the terminal keeps, if any (122).
    fn watch(&self) -> Watch;

    /// The session the calling command runs in, as the terminal names
    /// it, a launch's default recipient; none where it names none (7).
    fn session(&self) -> Option<String>;
}

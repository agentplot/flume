//! The terminal seam's answers (94 to 98, 122, 136, 7), and an agent's state.

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

/// open: an agent running in the place under its identity, or a refusal
/// (94). `carried` says the command it ran carried the prompt, as a
/// terminal that cannot send runs it (38); `printed` is the command a
/// person runs in the place, where the terminal prints it rather than
/// runs it, which status gives with the agent (C.5).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Opened {
    Opened {
        carried: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        printed: Option<String>,
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

/// The terminal seam (B.5), kept once for the host. The guarantee: open
/// never runs a second agent under an identity already live, and close
/// never reports an end that did not happen (98).
pub trait Terminal: Send + Sync {
    /// An agent running `command` in the place under `identity`, beside
    /// the pane the settings name, if any; what the terminal keeps of the
    /// start it keeps in the place's own `folder` (94, 107, 128).
    fn open(
        &self,
        place: &crate::PlaceRef,
        folder: &str,
        command: &crate::AgentCommand,
        identity: &crate::Identity,
        beside: Option<&str>,
    ) -> Opened;

    /// `text` delivered as input the agent receives, any text it holds
    /// typed cleared first, so no answer leaves text typed (95).
    fn send(&self, agent: &crate::Identity, text: &str) -> Sent;

    /// The agent's state: stopped only when its program no longer runs,
    /// unknown for anything the terminal cannot classify (96).
    fn state(&self, agent: &crate::Identity) -> AgentState;

    /// What the terminal shows of the agent, for status and a prompt
    /// never begun; none where it shows nothing (136, 35, 43).
    fn screen(&self, agent: &crate::Identity) -> Option<String>;

    /// The agent's end (97).
    fn close(&self, agent: &crate::Identity) -> Closed;

    /// The watch the terminal keeps, if any (122).
    fn watch(&self) -> Watch;

    /// The session the calling command runs in, as the terminal names
    /// it, a launch's default recipient; none where it names none (7).
    fn session(&self) -> Option<String>;
}

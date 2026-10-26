//! The terminal seam's answers (94 to 98, 122), and an agent's state.

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
/// terminal that cannot send runs it (38).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Opened {
    Opened { carried: bool },
    Refused { reason: String },
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

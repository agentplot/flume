//! The agent seam's answers (88 to 93, 115, 116, 121), an agent's
//! identity and its start's secret (33, 41).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

/// The one name flume gives an agent: its repository, its place, its
/// stage and which start of the stage it is (33).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Identity {
    pub repository: String,
    /// The place as `<kind>--<name>`.
    pub place: String,
    pub stage: String,
    pub start: u32,
}

impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "flume:{}:{}:{}:{}",
            self.repository, self.place, self.stage, self.start
        )
    }
}

/// A secret flume gives one start alone, which a stop-short carries
/// (41).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Secret(pub String);

/// command: how to start an agent so it works in its place under its
/// identity (88). `prompt_args` carry the prompt as the command's own
/// arguments, for a terminal that cannot send to a running agent (38).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentCommand {
    pub program: String,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub prompt_args: Vec<String>,
}

/// A yes or no the agent's own record may not give (93).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Reading {
    Yes,
    No,
    Unknown,
}

/// received: whether the agent received a text as a prompt, read from
/// its own record, which the answer names (89, 77).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Received {
    Received { record: String },
    NotReceived { record: String },
    Unknown,
}

/// Whether the agent program offers an operation (115, 121).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Offered {
    Offered,
    NotOffered,
}

/// guard: each act the agent would have taken that submits, merges or
/// pushes its place's branch, refused and reported, or that the program
/// gives no way to (116, 108).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Guarded {
    Refused { acts: Vec<String> },
    NotOffered,
}

/// An agent's word that its stage cannot deliver without something, with
/// its start's identity and secret (41, 92).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopShort {
    pub identity: String,
    pub secret: Secret,
    pub words: String,
}

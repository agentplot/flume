//! The agent seam's answers (88 to 93, 115, 116, 121, 134, 135), an agent's
//! identity and its start's secret (33, 41). Every reading of an agent's
//! own record is handed the place's own folder with the identity, where
//! the agent profile keeps its marks (128).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

/// The one name flume gives an agent: its repository, its place, its
/// stage and which start of the stage it is (33). The repository is
/// named by its id, taken from its common git directory (C.4).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Identity {
    /// The repository's id.
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

/// The question the agent program asks before it works in a folder new
/// to it, and the key that answers it, which the terminal reads and
/// answers (34).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FolderTrust {
    /// Text the program shows while it asks.
    pub question: String,
    /// The input that answers yes.
    pub answer: String,
}

/// command: how to start an agent so it works in its place under its
/// identity (88). `prompt_args` carry the prompt as the command's own
/// arguments, for a terminal that cannot send to a running agent (38).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentCommand {
    pub program: String,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub prompt_args: Vec<String>,
    /// The program's folder-trust question and its answer, if it asks
    /// one (34).
    #[serde(default)]
    pub folder_trust: Option<FolderTrust>,
    /// The program's kind, by which a terminal that starts a known agent
    /// program names it; none where the profile names none (C.4).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
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

/// What a stop-short says of the stage: it cannot go on without an
/// answer, or it found work the stage cannot deliver past (41, 92, 131).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopKind {
    #[default]
    NeedsAnswer,
    FoundWork,
}

/// An agent's word that its stage cannot deliver without something, with
/// its start's identity and secret, and whether it needs an answer or
/// found work (41, 92).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StopShort {
    pub identity: String,
    pub secret: Secret,
    pub words: String,
    #[serde(default)]
    pub kind: StopKind,
}

/// The agent seam (B.4). The guarantee: each answer is what the agent's
/// own record shows, and an answer the record does not give is unknown,
/// never guessed from the terminal (93).
pub trait Agent: Send + Sync {
    /// How to start the agent with its settings in the place under its
    /// identity and secret, the prompt in `prompt_args` (88, 29).
    fn command(
        &self,
        settings: &crate::Settings,
        place: &crate::PlaceRef,
        identity: &Identity,
        secret: &Secret,
        prompt: &str,
    ) -> AgentCommand;

    /// Whether the agent under `agent` has said it started under its
    /// start's identity, with flume's guard and stop check in force, read
    /// from its record in the place's own `folder` (135, 35, 128).
    fn started(&self, folder: &str, agent: &Identity) -> Reading;

    /// Whether the agent under `agent` received `text` as a prompt, read
    /// from its record in the place's own `folder` (89, 128).
    fn received(&self, folder: &str, agent: &Identity, text: &str) -> Received;

    /// Whether it waits on a question it put to a person (90, 128).
    fn asking(&self, folder: &str, agent: &Identity) -> Reading;

    /// Whether own work it started still runs (91, 128).
    fn own_work(&self, folder: &str, agent: &Identity) -> Reading;

    /// Whether the agent's own record shows its session ended (134, 128).
    fn ended(&self, folder: &str, agent: &Identity) -> Reading;

    /// The agent's last words as its own record holds them, which a stuck,
    /// lost or never-begun end carries; none where the record holds none
    /// or cannot be read (43, 128).
    fn last_words(&self, folder: &str, agent: &Identity) -> Option<String>;

    /// What the agent's own record says of its start that no reading
    /// answers and status reports with the agent, such as the agent
    /// program's part of flume being of another version than the one
    /// that started it (135); none by default.
    fn notes(&self, _folder: &str, _agent: &Identity) -> Vec<String> {
        vec![]
    }

    /// The one way the agent tells flume it stops short, filled in for
    /// its place and stage, as its prompt gives it (92, 28): the same for
    /// every start of the stage, naming nothing of `agent` that differs
    /// between starts, so a repetition's prompt matches the one before it
    /// up to its group (141; core.31).
    fn stop_short(&self, agent: &Identity) -> String;

    /// Whether the program lets flume keep an agent about to go quiet at
    /// work, with words naming its deliverable (115).
    fn send_back(&self) -> Offered;

    /// Each act the agent would have taken that submits, merges or pushes
    /// its place's branch, refused and reported (116, 108, 128).
    fn guard(&self, folder: &str, agent: &Identity) -> Guarded;

    /// Whether the program tells flume at once each time the agent goes
    /// quiet, so that flume settles its place (121).
    fn on_quiet(&self) -> Offered;
}

//! The agent seam's answers (88 to 93, 115, 116, 121, 134, 135, 151, 161), an agent's
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
/// to it, and how its choices show and are chosen, which the terminal
/// reads off the screen and answers yes (34).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FolderTrust {
    /// The yes choice's text, which the program shows while it asks.
    pub question: String,
    /// The mark the program shows on the line of the choice in focus.
    pub focus: String,
    /// The key that moves the focus to the next choice.
    pub next: String,
    /// The key that confirms the choice in focus, sent only while the
    /// yes choice has it.
    pub confirm: String,
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

/// spent: whether the agent's own record shows its account reached a
/// usage limit, with the reset the record names by the host's clock, none
/// where it names none, and the words it gave; unknown where the record
/// cannot say (146, 151).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Spent {
    Yes {
        until: Option<crate::Seconds>,
        words: String,
    },
    No,
    Unknown,
}

/// started_without: how the agent began otherwise than its start
/// recorded, as its own word that it started shows it: each variable the
/// start's record names that its environment lacks, each it holds with
/// another value, both by name alone, whether it runs another program,
/// and the mode it began in where it is not the one its start asked,
/// none in a word written before (135, 161, 162).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unsettled {
    #[serde(default)]
    pub missing: Vec<String>,
    #[serde(default)]
    pub different: Vec<String>,
    #[serde(default)]
    pub program: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ModeBegan>,
}

/// The mode a start asked, as its profile passed it, and the one the
/// agent's word says it began in, none where its word names none (161,
/// 162).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeBegan {
    pub asked: String,
    pub began: Option<String>,
}

impl Unsettled {
    /// How the agent began otherwise, naming each variable and never a
    /// value, and both modes where it began in another (56, 161, 162).
    pub fn words(&self) -> String {
        let mut words = vec![];
        if !self.missing.is_empty() {
            words.push(format!("without {}", self.missing.join(", ")));
        }
        if !self.different.is_empty() {
            words.push(format!("with another {}", self.different.join(", ")));
        }
        if self.program {
            words.push("under another program".to_string());
        }
        if let Some(ModeBegan { asked, began }) = &self.mode {
            words.push(match began {
                Some(began) => format!("in the mode `{began}`, where its start asked `{asked}`"),
                None => format!("in a mode its word did not name, where its start asked `{asked}`"),
            });
        }
        if words.is_empty() {
            return "otherwise than its start recorded".to_string();
        }
        words.join(", ")
    }
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
    /// How to start the agent with its settings and `env`, its account's
    /// variables read against the login environment, in the place under
    /// its identity and secret, the prompt in `prompt_args` (88, 29, 144).
    fn command(
        &self,
        settings: &crate::Settings,
        env: &BTreeMap<String, String>,
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

    /// Whether the agent's own record shows its account reached a usage
    /// limit, and the reset it names, read in the place's own `folder`
    /// (151, 146, 128); unknown by default, for a program whose record
    /// says nothing of one. flume asks no program or its maker whether a
    /// limit has reset.
    fn spent(&self, _folder: &str, _agent: &Identity) -> Spent {
        Spent::Unknown
    }

    /// How the agent under `agent` began otherwise than its start
    /// recorded, as its own record in the place's own `folder` holds its
    /// word: none where it began as recorded, has said nothing yet, or
    /// the program checks nothing at its start (135, 161, 128).
    fn started_without(&self, _folder: &str, _agent: &Identity) -> Option<Unsettled> {
        None
    }

    /// The variables the agent program keeps from every process it
    /// starts, its hooks among them, whatever gives them: its word that it
    /// started cannot show them, so no start's record names them (161);
    /// none by default.
    fn withheld(&self) -> Vec<String> {
        vec![]
    }

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

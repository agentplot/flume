//! A kind as read-kinds gives it (18, 23), the declaration it came from
//! (19) and the repository's binding (105). The fields are the
//! statement's vocabulary, never the kind format's (core.4): a kinds
//! profile maps the format onto them.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A named list of stages in order, at a version (23).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Kind {
    pub name: String,
    pub version: u32,
    /// The prefix of its places' branches, `<prefix>/<name>`.
    pub branch_prefix: String,
    /// Whether its first stage starts in a place that could not be made
    /// ready (12).
    pub starts_unready: bool,
    pub lands: bool,
    pub stages: Vec<Stage>,
}

/// One step of a kind (23), as declared: every agent and every
/// deliverable it names is kept, so the core can refuse a stage that asks
/// for more than flume runs (4).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stage {
    pub name: String,
    /// The prompt's text, with its parts unfilled (27).
    pub prompt: String,
    /// A conversation has no deliverable and ends on its caller's finish
    /// (109).
    pub conversation: bool,
    pub deliverables: Vec<Deliverable>,
    pub agents: Vec<Settings>,
    /// Whether the stage waits on its caller's word (47).
    pub waits: bool,
    /// How long its agent may be quiet before the stage is stuck (44).
    pub quiet_limit: Option<u64>,
    /// On a kind's last stage, the stage a failed submission sends its
    /// place back to (112).
    pub send_back_to: Option<String>,
    /// On a kind's last stage, how many times a place may go back before
    /// it waits on its caller (23, 112).
    pub send_back_limit: Option<u32>,
}

/// What must exist for a stage to end, in one of the six forms (24), or
/// a form flume cannot read, which refuses the kind (4).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Deliverable {
    Commits,
    ChangeWritten,
    TasksTicked,
    ChangeArchived,
    /// A file at the path, relative to the place, or under `{{state}}`,
    /// the place's own folder beside its held state (123).
    Report {
        path: String,
    },
    /// A file inside the place at the path, in a commit.
    CommittedReport {
        path: String,
    },
    Unreadable {
        spelled: String,
    },
}

/// What an agent starts with beyond its prompt (32, 107).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub program: Option<String>,
    pub model: Option<String>,
    pub instructions: Option<String>,
    pub env: BTreeMap<String, String>,
    /// The pane the agent opens beside, named by the launch's settings.
    pub beside: Option<String>,
}

impl Settings {
    /// These settings with `over`'s fields set over them, field by field
    /// (32).
    pub fn under(&self, over: &Settings) -> Settings {
        let mut env = self.env.clone();
        env.extend(over.env.clone());
        Settings {
            program: over.program.clone().or_else(|| self.program.clone()),
            model: over.model.clone().or_else(|| self.model.clone()),
            instructions: over
                .instructions
                .clone()
                .or_else(|| self.instructions.clone()),
            env,
            beside: over.beside.clone().or_else(|| self.beside.clone()),
        }
    }
}

/// The declaration a kind resolved to (19).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Declared {
    Launch,
    Repository,
    Flume,
}

/// read-kinds: every kind a declaration holds, or why it could not be
/// read (84).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KindsRead {
    Read { kinds: Vec<Kind> },
    CouldNotRead { reason: String },
}

/// A repository's binding: the profile it names for each of the place,
/// change and merge-queue seams, or none (105, core.3).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Binding {
    pub place: Option<String>,
    pub change: Option<String>,
    pub merge_queue: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BindingRead {
    Read { binding: Binding },
    CouldNotRead { reason: String },
}

/// The kinds seam (B.2). The same revision always gives the same kinds,
/// reading changes nothing, and a launch's declaration is read the same
/// way (85).
pub trait Kinds: Send + Sync {
    /// Every kind the repository declares at `revision` (84, 20).
    fn read_kinds(&self, repository: &str, revision: &crate::Revision) -> KindsRead;

    /// Every kind a declaration a launch carries holds (19, 85).
    fn read_declaration(&self, declaration: &str) -> KindsRead;

    /// flume's own declaration: fix and change (22).
    fn flumes_own(&self) -> Vec<Kind>;

    /// The repository's binding, read at its default branch's current
    /// revision (20, 105).
    fn read_binding(&self, repository: &str) -> BindingRead;
}

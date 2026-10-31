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
    /// Settings its declaration gives every stage's agent and the
    /// stages it names, which a stage's own come before (23, 32).
    #[serde(default)]
    pub agents: KindSettings,
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
    /// What the stage makes besides its deliverable, each by the name its
    /// declaration gives it; none where it declares nothing (129, C.2).
    #[serde(default)]
    pub makes: Vec<String>,
    /// Whether the stage repeats per task group: a fresh agent for each
    /// group of its change's tasks in turn, in the same place (140); false
    /// where it declares nothing, and in a held state written before.
    #[serde(default)]
    pub repeats: bool,
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

/// What an agent starts with beyond its prompt and the environment its
/// account gives (32, 107, 144).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub program: Option<String>,
    pub model: Option<String>,
    pub instructions: Option<String>,
    /// The pane the agent opens beside, named by the launch's settings.
    pub beside: Option<String>,
    /// How far the agent may act without asking a person, as its layer
    /// gave it; none where the layer gives none, and in settings written
    /// before (162, 169).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ModeSetting>,
}

/// flume's four modes, which mean the same on every program (169).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Plan,
    Ask,
    Edits,
    Full,
}

impl Mode {
    /// The four, in order.
    pub const ALL: [Mode; 4] = [Mode::Plan, Mode::Ask, Mode::Edits, Mode::Full];

    /// The mode as a layer writes it.
    pub fn word(self) -> &'static str {
        match self {
            Mode::Plan => "plan",
            Mode::Ask => "ask",
            Mode::Edits => "edits",
            Mode::Full => "full",
        }
    }

    /// The mode `word` spells, none where it spells none of the four.
    pub fn from_word(word: &str) -> Option<Mode> {
        Mode::ALL.into_iter().find(|mode| mode.word() == word)
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.word())
    }
}

/// A mode as a layer gives it: one of flume's four, or a table of
/// programs, each entry as written, the agent's taken by its program's
/// name (162, 169). It is written as the word or as an object, and read
/// through [`ModeSetting::from_word`] and [`ModeSetting::from_programs`],
/// so held state, the launch's JSON form and every reader spell it alike.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "WrittenMode", into = "WrittenMode")]
pub enum ModeSetting {
    Flume(Mode),
    Programs(BTreeMap<String, String>),
}

/// A mode as JSON spells it, before it is read.
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum WrittenMode {
    Word(String),
    Programs(BTreeMap<String, String>),
}

impl TryFrom<WrittenMode> for ModeSetting {
    type Error = String;

    fn try_from(written: WrittenMode) -> Result<ModeSetting, String> {
        match written {
            WrittenMode::Word(word) => ModeSetting::from_word(&word),
            WrittenMode::Programs(programs) => ModeSetting::from_programs(programs),
        }
    }
}

impl From<ModeSetting> for WrittenMode {
    fn from(setting: ModeSetting) -> WrittenMode {
        match setting {
            ModeSetting::Flume(mode) => WrittenMode::Word(mode.word().to_string()),
            ModeSetting::Programs(programs) => WrittenMode::Programs(programs),
        }
    }
}

impl ModeSetting {
    /// One of flume's four, given bare; any other word refused naming it,
    /// since a program's own value goes under its program's name (169).
    pub fn from_word(word: &str) -> Result<ModeSetting, String> {
        Mode::from_word(word)
            .map(ModeSetting::Flume)
            .ok_or_else(|| {
                format!(
                    "the mode `{word}` is none of flume's four, plan, ask, edits and full: \
                 a program's own value goes under its program's name, as \
                 mode = {{ <program> = \"{word}\" }}"
                )
            })
    }

    /// A table of programs, each entry as written; one that names no
    /// program refused (169).
    pub fn from_programs(programs: BTreeMap<String, String>) -> Result<ModeSetting, String> {
        if programs.is_empty() {
            return Err("the mode's table of programs names no program".into());
        }
        Ok(ModeSetting::Programs(programs))
    }

    /// The mode as a layer would write it, `edits` or `{ agent =
    /// "careful", other = "edits" }`, its programs in order (56).
    pub fn written(&self) -> String {
        match self {
            ModeSetting::Flume(mode) => mode.word().to_string(),
            ModeSetting::Programs(programs) => {
                let entries: Vec<String> = programs
                    .iter()
                    .map(|(program, value)| format!("{program} = {value:?}"))
                    .collect();
                format!("{{ {} }}", entries.join(", "))
            }
        }
    }

    /// The entry of the agent whose program is named `program`: one of the
    /// four given whole, or the table's entry for it; none where the table
    /// names no entry for it, since the table is taken whole (169).
    pub fn entry(&self, program: &str) -> Option<ModeEntry> {
        match self {
            ModeSetting::Flume(mode) => Some(ModeEntry::Flume(*mode)),
            ModeSetting::Programs(programs) => programs.get(program).map(|value| {
                Mode::from_word(value)
                    .map_or_else(|| ModeEntry::Own(value.clone()), ModeEntry::Flume)
            }),
        }
    }
}

/// An agent's entry of a mode: one of flume's four, or a value of its
/// program's own, as written, which its profile maps or refuses (169).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModeEntry {
    Flume(Mode),
    Own(String),
}

impl std::fmt::Display for ModeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModeEntry::Flume(mode) => f.write_str(mode.word()),
            ModeEntry::Own(value) => f.write_str(value),
        }
    }
}

/// What stands in a record or an answer for an environment variable's
/// value: a value may be a secret, a token or a password, so no status,
/// log line, refusal or record of a start holds one, only that the
/// variable is set (32, 56).
pub const SET: &str = "<set>";

impl Settings {
    /// These settings with `over`'s fields set over them, field by field,
    /// each taken whole: a mode's table of programs is never merged with
    /// another's (32, 169).
    pub fn under(&self, over: &Settings) -> Settings {
        Settings {
            program: over.program.clone().or_else(|| self.program.clone()),
            model: over.model.clone().or_else(|| self.model.clone()),
            instructions: over
                .instructions
                .clone()
                .or_else(|| self.instructions.clone()),
            beside: over.beside.clone().or_else(|| self.beside.clone()),
            mode: over.mode.clone().or_else(|| self.mode.clone()),
        }
    }
}

/// Settings given for a kind's agents: for every stage, and for each
/// stage named, which come before those for every stage (32).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindSettings {
    pub every_stage: Settings,
    pub stages: BTreeMap<String, Settings>,
}

impl KindSettings {
    /// The settings for `stage`'s agent: the stage's own over those for
    /// every stage (32).
    pub fn for_stage(&self, stage: &str) -> Settings {
        match self.stages.get(stage) {
            Some(own) => self.every_stage.under(own),
            None => self.every_stage.clone(),
        }
    }

    /// Whether they give no setting, for every stage or for any.
    pub fn is_empty(&self) -> bool {
        self.every_stage == Settings::default() && self.stages.is_empty()
    }
}

/// Where a setting an agent started with came from, the first of the
/// six that gave it, or for an environment variable its account (32, 56,
/// 144).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Layer {
    /// The launch's settings (7).
    Launch,
    /// The host's configuration for the place's repository and kind.
    HostRepository,
    /// The host's configuration for the kind.
    HostKind,
    /// The repository's settings for a kind it does not declare.
    Repository,
    /// The kind's declaration (23).
    Kind,
    /// flume's own default.
    Default,
    /// The account the agent runs under, which alone gives its
    /// environment (144).
    Account,
}

impl Layer {
    /// The layer as status names it.
    pub fn name(self) -> &'static str {
        match self {
            Layer::Launch => "launch",
            Layer::HostRepository => "host-repository",
            Layer::HostKind => "host-kind",
            Layer::Repository => "repository",
            Layer::Kind => "kind",
            Layer::Default => "default",
            Layer::Account => "account",
        }
    }
}

impl std::fmt::Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// One setting's layer and its value as that layer wrote it, before an
/// account value's `~` and variables are read (32, 56).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Given {
    pub layer: Layer,
    pub value: String,
}

/// `from` with the value of every environment variable, `env.<NAME>`,
/// given as [`SET`], so the layers stand and no value does (32, 56).
pub fn unvalued_from(from: &BTreeMap<String, Given>) -> BTreeMap<String, Given> {
    from.iter()
        .map(|(key, given)| {
            let value = if key.starts_with("env.") {
                SET.to_string()
            } else {
                given.value.clone()
            };
            (
                key.clone(),
                Given {
                    layer: given.layer,
                    value,
                },
            )
        })
        .collect()
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
    Read {
        kinds: Vec<Kind>,
        /// The settings the declaration gives kinds it does not declare,
        /// by the kind's name (32, C.2).
        #[serde(default)]
        settings: BTreeMap<String, KindSettings>,
    },
    CouldNotRead {
        reason: String,
    },
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
    /// Every kind the repository declares at `revision`, and the
    /// settings it gives kinds it does not declare (84, 20, 32).
    fn read_kinds(&self, repository: &str, revision: &crate::Revision) -> KindsRead;

    /// Every kind a declaration a launch carries holds (19, 85).
    fn read_declaration(&self, declaration: &str) -> KindsRead;

    /// flume's own declaration: fix and change (22).
    fn flumes_own(&self) -> Vec<Kind>;

    /// The repository's binding, read at its default branch's current
    /// revision (20, 105).
    fn read_binding(&self, repository: &str) -> BindingRead;
}

//! The kinds profile (C.2; design 7): every declaration, a repository's
//! `.config/flume.toml` at a revision, a launch's file and flume's own
//! built in, read through `weir-kinds`, with no reader of flume's own
//! (core.4). The format's fields are mapped onto the seam's types, every
//! section the format does not name is ignored (23), and a declaration
//! weir refuses is answered as could not be read, naming every field that
//! fails (84). A kind's `agents` table, and a `[kinds.<kind>]` table that
//! gives settings to a kind declared elsewhere, are weir's too: each is
//! mapped onto the seam's settings for every stage and for each stage it
//! names, the latter returned apart from the kinds (32, C.2; weir 52,
//! 57, C.4); which declaration they join, and whether the stages they
//! name exist, are the core's to say. A prompt is a path relative to its declaration's file,
//! read beside it: at the same revision for a repository's, on disk for
//! a launch's, and from the binary for flume's own.
//!
//! The repository's binding is the declaration's `[profiles]` table at
//! the default branch: `origin/HEAD` after one fetch where the
//! repository has a remote, the local default branch otherwise (20,
//! 105). No other tool's file is read to choose a profile (core.3).

use crate::git::{tree_path, Git};
use flume_seams::{
    Binding, BindingRead, Deliverable, Kind, KindSettings, Kinds, KindsRead, Revision, Settings,
    Stage,
};
use std::collections::BTreeMap;
use std::path::Path;
use weir_kinds::toml::{Table, Value};
use weir_kinds::{End, Refusal};

/// Where a repository declares its kinds and its binding, in its tree.
pub const DECLARATION: &str = ".config/flume.toml";

/// The remote a repository's default branch is read from where it has
/// one.
const REMOTE: &str = "origin";

/// flume's own declaration and its prompts, built into the binary (22,
/// C.2).
const OWN: &str = include_str!("flume.toml");
const OWN_PROMPTS: &[(&str, &str)] = &[
    ("prompts/code.md", include_str!("prompts/code.md")),
    (
        "prompts/change-construct.md",
        include_str!("prompts/change-construct.md"),
    ),
    (
        "prompts/change-code.md",
        include_str!("prompts/change-code.md"),
    ),
    (
        "prompts/change-verify.md",
        include_str!("prompts/change-verify.md"),
    ),
];

/// The kinds profile over weir's kind format.
#[derive(Clone, Copy, Debug, Default)]
pub struct Weir;

impl Kinds for Weir {
    fn read_kinds(&self, repository: &str, revision: &Revision) -> KindsRead {
        let git = Git::at(repository);
        if let Err(reason) = git.commit(&revision.0) {
            return KindsRead::CouldNotRead { reason };
        }
        let text = match git.file(&revision.0, DECLARATION) {
            Ok(Some(text)) => text,
            Ok(None) => {
                return KindsRead::Read {
                    kinds: vec![],
                    settings: BTreeMap::new(),
                }
            }
            Err(reason) => return KindsRead::CouldNotRead { reason },
        };
        let directory = Path::new(DECLARATION)
            .parent()
            .map(|parent| parent.to_string_lossy().into_owned())
            .unwrap_or_default();
        read(&text, |path| {
            let at = tree_path(&directory, path)
                .ok_or_else(|| format!("{path} lies outside the repository"))?;
            git.file(&revision.0, &at)?
                .ok_or_else(|| format!("{at} is not in {revision}"))
        })
    }

    fn read_declaration(&self, declaration: &str) -> KindsRead {
        let file = Path::new(declaration);
        let text = match std::fs::read_to_string(file) {
            Ok(text) => text,
            Err(error) => {
                return KindsRead::CouldNotRead {
                    reason: format!("{declaration} could not be read: {error}"),
                }
            }
        };
        let directory = file.parent().unwrap_or(Path::new(""));
        read(&text, |path| {
            let at = directory.join(path);
            std::fs::read_to_string(&at)
                .map_err(|error| format!("{} could not be read: {error}", at.display()))
        })
    }

    fn flumes_own(&self) -> Vec<Kind> {
        match read(OWN, |path| {
            OWN_PROMPTS
                .iter()
                .find(|(own, _)| *own == path)
                .map(|(_, text)| text.to_string())
                .ok_or_else(|| format!("{path} is not built in"))
        }) {
            KindsRead::Read { kinds, .. } => kinds,
            KindsRead::CouldNotRead { reason } => {
                panic!("flume's own declaration is built in and reads: {reason}")
            }
        }
    }

    fn read_binding(&self, repository: &str) -> BindingRead {
        match default_revision(repository) {
            Ok(Some(revision)) => binding_at(repository, &revision),
            Ok(None) => BindingRead::Read {
                binding: Binding::default(),
            },
            Err(reason) => BindingRead::CouldNotRead { reason },
        }
    }
}

/// The revision of the repository's default branch: `origin/HEAD` after
/// one fetch where it has the remote, the local default branch otherwise;
/// `None` where it has no commit (20, 105).
pub fn default_revision(repository: &str) -> Result<Option<Revision>, String> {
    let git = Git::at(repository);
    let local = git
        .text(&["symbolic-ref", "--quiet", "--short", "HEAD"])
        .map(|name| name.trim().to_string())
        .ok();
    if git.remotes()?.iter().any(|remote| remote == REMOTE) {
        git.bytes(&["fetch", "--quiet", REMOTE])
            .map_err(|words| format!("the fetch of {REMOTE} failed: {words}"))?;
        let tracking = format!("refs/remotes/{REMOTE}/HEAD");
        if let Ok(commit) = git.commit(&tracking) {
            return Ok(Some(Revision(commit)));
        }
        if let Some(name) = &local {
            if let Ok(commit) = git.commit(&format!("refs/remotes/{REMOTE}/{name}")) {
                return Ok(Some(Revision(commit)));
            }
        }
        return Err(format!(
            "{REMOTE} names no default branch: {tracking} is not set"
        ));
    }
    match local {
        Some(name) => Ok(git.commit(&format!("refs/heads/{name}")).ok().map(Revision)),
        None => Ok(git.commit("HEAD").ok().map(Revision)),
    }
}

/// The `[profiles]` table of the repository's declaration at `revision`:
/// each seam it names, none where it names none or there is no
/// declaration (105, C.2, core.3).
pub fn binding_at(repository: &str, revision: &Revision) -> BindingRead {
    let git = Git::at(repository);
    let text = match git.file(&revision.0, DECLARATION) {
        Ok(Some(text)) => text,
        Ok(None) => {
            return BindingRead::Read {
                binding: Binding::default(),
            }
        }
        Err(reason) => return BindingRead::CouldNotRead { reason },
    };
    match profiles(&text) {
        Ok(binding) => BindingRead::Read { binding },
        Err(reason) => BindingRead::CouldNotRead {
            reason: format!("{DECLARATION} at {revision}: {reason}"),
        },
    }
}

/// The binding a declaration's `[profiles]` table names. The table is
/// flume's own, beside the kinds, so it is read whether or not the kinds
/// read.
fn profiles(text: &str) -> Result<Binding, String> {
    let root: Table = text
        .parse()
        .map_err(|error: weir_kinds::toml::de::Error| error.message().trim().to_string())?;
    let Some(table) = root.get("profiles") else {
        return Ok(Binding::default());
    };
    let Some(table) = table.as_table() else {
        return Err("profiles: must be a table naming a profile for each seam".into());
    };
    let mut problems = vec![];
    let mut named = |seam: &str| match table.get(seam) {
        None => None,
        Some(Value::String(name)) if !name.is_empty() => Some(name.clone()),
        Some(_) => {
            problems.push(format!("profiles.{seam}: must be a profile's name"));
            None
        }
    };
    let binding = Binding {
        place: named("place"),
        change: named("change"),
        merge_queue: named("merge_queue"),
    };
    if problems.is_empty() {
        Ok(binding)
    } else {
        Err(problems.join("; "))
    }
}

/// Every kind a declaration's text holds, its prompts read by `prompt`
/// from their paths, and the settings it gives kinds it does not
/// declare, or every field that fails it (84, 32).
fn read(text: &str, prompt: impl Fn(&str) -> Result<String, String>) -> KindsRead {
    let declaration = match weir_kinds::read_kinds(text) {
        Ok(declaration) => declaration,
        Err(refusal) => {
            return KindsRead::CouldNotRead {
                reason: refused(&refusal),
            }
        }
    };
    let mut problems = vec![];
    let kinds = declaration
        .kinds
        .values()
        .map(|kind| map_kind(kind, &prompt, &mut problems))
        .collect();
    let settings = declaration
        .settings
        .values()
        .map(|given| {
            let at = format!("kinds.{}.agents", given.name);
            (
                given.name.clone(),
                map_agents(&given.agents, &at, &mut problems),
            )
        })
        .collect();
    if problems.is_empty() {
        KindsRead::Read { kinds, settings }
    } else {
        KindsRead::CouldNotRead {
            reason: problems.join("; "),
        }
    }
}

fn refused(refusal: &Refusal) -> String {
    refusal
        .problems
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("; ")
}

fn map_kind(
    kind: &weir_kinds::Kind,
    prompt: &impl Fn(&str) -> Result<String, String>,
    problems: &mut Vec<String>,
) -> Kind {
    let at = format!("kinds.{}", kind.name);
    let version = u32::try_from(kind.version).unwrap_or_else(|_| {
        problems.push(format!("{at}.version: is more than flume counts"));
        0
    });
    Kind {
        name: kind.name.clone(),
        version,
        branch_prefix: kind.branch.clone(),
        starts_unready: kind.starts_unready,
        lands: kind.lands,
        agents: map_agents(&kind.agents, &format!("{at}.agents"), problems),
        stages: kind
            .stages
            .iter()
            .enumerate()
            .map(|(n, stage)| map_stage(stage, &format!("{at}.stages[{n}]"), prompt, problems))
            .collect(),
    }
}

fn map_stage(
    stage: &weir_kinds::Stage,
    at: &str,
    prompt: &impl Fn(&str) -> Result<String, String>,
    problems: &mut Vec<String>,
) -> Stage {
    let text = prompt(&stage.prompt).unwrap_or_else(|reason| {
        problems.push(format!("{at}.prompt: {reason}"));
        String::new()
    });
    let (conversation, deliverables) = match &stage.end {
        End::Conversation => (true, vec![]),
        End::Deliverable(deliverable) => (false, vec![map_deliverable(deliverable)]),
    };
    let send_back_limit = stage.max_send_backs.map(|times| {
        u32::try_from(times).unwrap_or_else(|_| {
            problems.push(format!("{at}.max_send_backs: is more than flume counts"));
            0
        })
    });
    Stage {
        name: stage.name.clone(),
        prompt: text,
        conversation,
        deliverables,
        agents: stage
            .agents
            .iter()
            .enumerate()
            .map(|(n, agent)| map_agent(agent, &format!("{at}.agents[{n}]"), problems))
            .collect(),
        waits: stage.wait,
        quiet_limit: stage.quiet.map(|quiet| quiet.as_secs()),
        send_back_to: stage.on_fail.clone(),
        send_back_limit,
    }
}

fn map_deliverable(deliverable: &weir_kinds::Deliverable) -> Deliverable {
    match deliverable {
        weir_kinds::Deliverable::Commits => Deliverable::Commits,
        weir_kinds::Deliverable::Change => Deliverable::ChangeWritten,
        weir_kinds::Deliverable::Tasks => Deliverable::TasksTicked,
        weir_kinds::Deliverable::Archived => Deliverable::ChangeArchived,
        weir_kinds::Deliverable::Report(path) => Deliverable::Report { path: path.clone() },
        weir_kinds::Deliverable::Committed(path) => {
            Deliverable::CommittedReport { path: path.clone() }
        }
    }
}

/// A kind's `agents` table: the settings for every stage, and for each
/// stage its `stages` table names (32; weir 52, C.4).
fn map_agents(
    agents: &weir_kinds::KindAgents,
    at: &str,
    problems: &mut Vec<String>,
) -> KindSettings {
    KindSettings {
        every_stage: map_agent(&agents.every_stage, at, problems),
        stages: agents
            .stages
            .iter()
            .map(|(stage, table)| {
                let at = format!("{at}.stages.{stage}");
                (stage.clone(), map_agent(table, &at, problems))
            })
            .collect(),
    }
}

/// An agent's settings, its table as written: `program`, `model`,
/// `instructions`, `env` and `beside` (32, 107); every other key is the
/// runner's, and ignored.
fn map_agent(agent: &Table, at: &str, problems: &mut Vec<String>) -> Settings {
    let mut text = |name: &str| match agent.get(name) {
        None => None,
        Some(Value::String(value)) => Some(value.clone()),
        Some(_) => {
            problems.push(format!("{at}.{name}: must be a string"));
            None
        }
    };
    let program = text("program");
    let model = text("model");
    let instructions = text("instructions");
    let beside = text("beside");
    let env = match agent.get("env") {
        None => Default::default(),
        Some(Value::Table(env)) => env
            .iter()
            .filter_map(|(name, value)| match value {
                Value::String(value) => Some((name.clone(), value.clone())),
                _ => {
                    problems.push(format!("{at}.env.{name}: must be a string"));
                    None
                }
            })
            .collect(),
        Some(_) => {
            problems.push(format!("{at}.env: must be a table of strings"));
            Default::default()
        }
    };
    Settings {
        program,
        model,
        instructions,
        env,
        beside,
    }
}

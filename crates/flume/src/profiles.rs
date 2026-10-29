//! The profiles of Part C built into the binary, one module each, bound
//! by the repository's binding and the host's (105, core.3). Each seam
//! has one marker below which a profile unit appends its module line;
//! the lists are append-only, so the units that write the profiles stay
//! apart (design 13).

use crate::host::config::{self, HostConfig};
use crate::host::history::GitHistory;
use crate::host::{self, RealHost};
use flume_core::{Defaults, Engine};
use flume_seams::{
    Agent, Binding, BranchRemoved, Change, Kinds, MakeOutcome, MergeQueue, Place, PlaceRead,
    PlaceRef, Readiness, ReadyStep, Removed, Resolver, Revision, Settings, Terminal,
};
use place::git_worktree::{self, GitWorktree, Login, PlaceData, Step};
use std::path::Path;
use std::sync::Arc;

// ---- the place seam's profiles: a profile unit appends its module line below ----
pub mod place {
    pub mod git_worktree;
}

// ---- the kinds seam's profiles: a profile unit appends its module line below ----
pub mod kinds {
    pub mod weir;
}

// ---- the change seam's profiles: a profile unit appends its module line below ----
pub mod change {
    pub mod openspec;
}

// ---- the agent seam's profiles: a profile unit appends its module line below ----

// ---- the terminal seam's profiles: a profile unit appends its module line below ----
pub mod terminal {
    pub mod herdr;
    pub mod printed;
}

// ---- the merge queue seam's profiles: a profile unit appends its module line below ----
pub mod merge_queue {
    pub mod sluice;
}

// ---- the store seam's profiles: a profile unit appends its module line below ----
pub mod store {
    pub mod state_dir;
}

/// The program an agent runs where no layer of its settings names one:
/// Claude Code (32, C.4; design 16).
pub const DEFAULT_PROGRAM: &str = "claude";

/// The send-backs a kind allows where it sets none (23, 112; design 16).
pub const SEND_BACKS: u32 = 3;

/// What the engine is built with where the statement leaves the value
/// to flume: Claude Code with no model, instructions or environment, and
/// three send-backs (32, 23; design 16).
pub fn defaults() -> Defaults {
    Defaults {
        settings: Settings {
            program: Some(DEFAULT_PROGRAM.into()),
            ..Settings::default()
        },
        send_backs: SEND_BACKS,
    }
}

/// The resolver from a binding's profile names to the profiles built in
/// (105, core.3): the kinds through weir's format, the place through
/// plain `git worktree` bound with the merge queue's data, the change
/// through OpenSpec, the merge queue through sluice, and the agent by
/// its program. A name it does not hold resolves to nothing, so the core
/// refuses naming it.
#[derive(Clone, Debug, Default)]
pub struct BuiltIn {
    /// The login environment the place's steps run in, the host user's
    /// where none is given.
    login: Option<Vec<(String, String)>>,
}

impl BuiltIn {
    /// The profiles built in, a place's steps run in the host user's
    /// login environment.
    pub fn new() -> BuiltIn {
        BuiltIn::default()
    }

    /// The profiles built in, a place's steps run in exactly `variables`.
    pub fn with_login(variables: &[(&str, &str)]) -> BuiltIn {
        BuiltIn {
            login: Some(
                variables
                    .iter()
                    .map(|(name, value)| (name.to_string(), value.to_string()))
                    .collect(),
            ),
        }
    }
}

impl Resolver for BuiltIn {
    fn kinds(&self) -> Arc<dyn Kinds> {
        Arc::new(kinds::weir::Weir)
    }

    fn place_profiles(&self) -> Vec<String> {
        vec![git_worktree::NAME.into()]
    }

    fn place(&self, repository: &str, binding: &Binding) -> Option<Arc<dyn Place>> {
        match binding.place.as_deref() {
            None | Some(git_worktree::NAME) => Some(Arc::new(BoundPlace {
                repository: repository.into(),
                merge_queue: binding.merge_queue.clone(),
                login: self.login.clone(),
            })),
            Some(_) => None,
        }
    }

    fn change(&self, profile: &str) -> Option<Arc<dyn Change>> {
        match profile {
            change::openspec::NAME => Some(Arc::new(change::openspec::OpenSpec)),
            _ => None,
        }
    }

    fn merge_queue(&self, profile: &str) -> Option<Arc<dyn MergeQueue>> {
        match profile {
            merge_queue::sluice::NAME => Some(Arc::new(merge_queue::sluice::Sluice::new())),
            _ => None,
        }
    }

    /// No agent profile is built in yet: Claude Code's, which answers to
    /// [`DEFAULT_PROGRAM`], comes with flume's plugin, and until then a
    /// launch is refused naming the program no profile runs (32, 105).
    fn agent(&self, _program: &str) -> Option<Arc<dyn Agent>> {
        None
    }
}

/// The place profile over plain `git worktree` for one repository, bound
/// with the data its merge queue profile declares for the place's base
/// at each call (design 9, 13): the readiness, the teardown and where the
/// base lives, read from the merge queue's policy at the base's current
/// revision, none of it where no merge queue is bound. The resolver
/// hands the one profile's data to the other, and neither calls the
/// other (core.2).
struct BoundPlace {
    repository: String,
    merge_queue: Option<String>,
    login: Option<Vec<(String, String)>>,
}

/// The step a place's data is named by where the merge queue's policy
/// could not be read.
const POLICY_STEP: &str = "the merge queue's policy";

impl BoundPlace {
    /// What the merge queue declares for the places on `base`.
    fn data(&self, base: &str) -> Result<PlaceData, String> {
        match self.merge_queue.as_deref() {
            Some(merge_queue::sluice::NAME) => {
                let declared = merge_queue::sluice::for_place(&self.repository, base)?;
                let mut readiness = vec![];
                if let Some(toolchain) = &declared.toolchain {
                    readiness.push(Step::toolchain("toolchain", toolchain));
                }
                if let Some(setup) = &declared.setup {
                    readiness.push(Step::run(setup, setup));
                }
                Ok(PlaceData {
                    readiness,
                    teardown: vec![Step::run(&declared.teardown, &declared.teardown)],
                    base_remote: declared.base_remote,
                })
            }
            _ => Ok(PlaceData::default()),
        }
    }

    /// The profile bound with `data`.
    fn bound(&self, data: PlaceData) -> GitWorktree {
        match &self.login {
            Some(variables) => {
                let variables: Vec<(&str, &str)> = variables
                    .iter()
                    .map(|(name, value)| (name.as_str(), value.as_str()))
                    .collect();
                GitWorktree::with_login(data, Login::given(&variables))
            }
            None => GitWorktree::new(data),
        }
    }
}

impl Place for BoundPlace {
    fn make(
        &self,
        repository: &str,
        revision: &Revision,
        branch: &str,
        location: &str,
    ) -> MakeOutcome {
        self.bound(PlaceData::default())
            .make(repository, revision, branch, location)
    }

    fn ready(&self, place: &PlaceRef, revision: &Revision) -> Readiness {
        match self.data(&place.base) {
            Ok(data) => self.bound(data).ready(place, revision),
            Err(words) => Readiness {
                steps: vec![ReadyStep {
                    step: POLICY_STEP.into(),
                    passed: false,
                    words,
                }],
            },
        }
    }

    fn read(&self, place: &PlaceRef) -> PlaceRead {
        self.bound(PlaceData::default()).read(place)
    }

    fn remove(&self, place: &PlaceRef) -> Removed {
        match self.data(&place.base) {
            Ok(data) => self.bound(data).remove(place),
            Err(words) => Removed::Refused {
                step: POLICY_STEP.into(),
                words,
            },
        }
    }

    fn remove_branch(&self, repository: &str, branch: &str, base: &str) -> BranchRemoved {
        match self.data(base) {
            Ok(data) => self.bound(data).remove_branch(repository, branch, base),
            // A base whose place cannot be read judges nothing: the branch
            // is kept (67).
            Err(_) => BranchRemoved::Kept { commits: vec![] },
        }
    }
}

/// The terminal profiles built in, by the name `[host] terminal` gives
/// them; the printed command where it names none (105, C.5).
pub fn terminal_profiles() -> Vec<String> {
    vec![terminal::printed::NAME.into(), terminal::herdr::NAME.into()]
}

/// The terminal profile `name` binds for the host, its records under
/// flume's state directory `state`, or the refusal naming the profiles
/// there are (105, C.5).
pub fn terminal(name: Option<&str>, state: &Path) -> Result<Arc<dyn Terminal>, String> {
    match name {
        None | Some(terminal::printed::NAME) => Ok(Arc::new(terminal::printed::Printed)),
        Some(terminal::herdr::NAME) => Ok(Arc::new(
            terminal::herdr::Herdr::new(
                terminal::herdr::Server::Session(terminal::herdr::SESSION.into()),
                state.join("terminal/herdr"),
            )
            .with_caller(terminal::herdr::caller()),
        )),
        Some(other) => Err(format!(
            "the host's configuration names the terminal {other}, and no terminal profile is named {other}; the terminal profiles there are: {}",
            terminal_profiles().join(", ")
        )),
    }
}

/// The core over the profiles built in, for one command on this host:
/// flume's state in `$XDG_STATE_HOME/flume`, the host's configuration in
/// `$XDG_CONFIG_HOME/flume/config.toml`, and the recipient
/// `$FLUME_RECIPIENT` names; a configuration that cannot be read is
/// refused, naming its fault (C.7; design 16).
pub fn core() -> Result<Engine, String> {
    core_over(
        &config::flume_state()?,
        HostConfig::of_host()?,
        host::recipient(),
        BuiltIn::new(),
    )
}

/// The core over `resolver`, the terminal `config` binds, the store in
/// flume's state directory `state`, git's history, and the host with
/// `config` and `recipient`, with flume's [`defaults`] (design 16).
pub fn core_over(
    state: &Path,
    config: HostConfig,
    recipient: Option<String>,
    resolver: BuiltIn,
) -> Result<Engine, String> {
    let terminal = terminal(config.terminal.as_deref(), state)?;
    Ok(Engine::new(
        Arc::new(resolver),
        terminal,
        Arc::new(store::state_dir::StateDir::at(state)),
        Arc::new(GitHistory),
        Arc::new(RealHost::new(state, config, recipient)),
        defaults(),
    ))
}

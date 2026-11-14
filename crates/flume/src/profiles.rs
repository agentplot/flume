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
    PlaceRef, Readiness, ReadyStep, Removed, Resolver, Revision, Settings, Terminal, Window,
};
use place::git_worktree::{self, GitWorktree, Login, PlaceData, Step};
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

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
pub mod agent {
    pub mod claude;
}

// ---- the terminal seam's profiles: a profile unit appends its module line below ----
pub mod terminal {
    pub mod common;
    pub mod herdr;
    pub mod printed;
    pub mod rex;
}

// ---- the merge queue seam's profiles: a profile unit appends its module line below ----
pub mod merge_queue {
    pub mod sluice;
}

// ---- the store seam's profiles: a profile unit appends its module line below ----
pub mod store {
    pub mod state_dir;
}

// ---- the window seam's profiles: a profile unit appends its module line below ----
pub mod window {
    pub mod ghostty;
    pub mod printed;
    pub mod terminal;
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
/// through OpenSpec, the merge queue through sluice, the agent by its
/// program, and the terminal and the window by their names. A name it
/// does not hold resolves to nothing, so the core refuses naming it.
#[derive(Clone, Debug, Default)]
pub struct BuiltIn {
    /// The login environment the place's steps run in, the host user's
    /// where none is given.
    login: Option<Vec<(String, String)>>,
    /// The terminal profiles built for this command, shared by its
    /// clones.
    terminals: Arc<Terminals>,
}

/// The terminal profiles a command built, by name, each at most once, so
/// the host's own is the very one the core is built with; and flume's
/// state directory their records go under, none until the core is built
/// over the resolver (105, 190; design 2).
#[derive(Default)]
struct Terminals {
    state: OnceLock<PathBuf>,
    built: Mutex<BTreeMap<String, Arc<dyn Terminal>>>,
}

impl fmt::Debug for Terminals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let built = self.built.lock().unwrap_or_else(PoisonError::into_inner);
        f.debug_struct("Terminals")
            .field("state", &self.state.get())
            .field("built", &built.keys().collect::<Vec<_>>())
            .finish()
    }
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
            terminals: Arc::default(),
        }
    }

    /// The same profiles, their terminals' records under flume's state
    /// directory `state`, as the core built over them keeps them, so a
    /// resolver that wraps them gives terminals by name before
    /// [`core_with`] builds the core over it (105, 190).
    pub fn over(self, state: &Path) -> BuiltIn {
        let _ = self.terminals.state.set(state.to_path_buf());
        self
    }

    /// The terminal profile `name` names, the printed command where none,
    /// its records under flume's state directory `state`: the one built
    /// before for this command, else built now and kept; or the refusal
    /// naming the profiles there are (105, 190).
    fn terminal_named(
        &self,
        name: Option<&str>,
        state: &Path,
    ) -> Result<Arc<dyn Terminal>, String> {
        let name = name.unwrap_or(terminal::printed::NAME);
        let mut built = self
            .terminals
            .built
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if let Some(built) = built.get(name) {
            return Ok(built.clone());
        }
        let profile = terminal(Some(name), state, self.login.as_deref())?;
        built.insert(name.into(), profile.clone());
        Ok(profile)
    }

    /// The login environment's `$XDG_DATA_HOME`, or its
    /// `$HOME/.local/share`, where the resolver was given one.
    fn login_data(&self) -> Option<PathBuf> {
        let variables = self.login.as_ref()?;
        let given = |name: &str| {
            variables
                .iter()
                .find(|(variable, value)| variable == name && !value.is_empty())
                .map(|(_, value)| PathBuf::from(value))
        };
        Some(given("XDG_DATA_HOME").unwrap_or_else(|| {
            given("HOME")
                .unwrap_or_else(std::env::temp_dir)
                .join(".local/share")
        }))
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
                own: self.login_data().map(|data| data.join("flume")),
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

    /// Claude Code's profile for the program [`DEFAULT_PROGRAM`] names,
    /// by that name or by a path to it, with flume's plugin; any other
    /// program resolves to nothing, and a launch naming it is refused
    /// (32, 105, C.4).
    /// The plugin is written under the login environment's
    /// `$XDG_DATA_HOME`, or its `$HOME/.local/share`, where the resolver
    /// was given one.
    fn agent(&self, program: &str) -> Option<Arc<dyn Agent>> {
        let name = Path::new(program).file_name()?.to_str()?;
        if name != agent::claude::NAME {
            return None;
        }
        let claude = match self.login_data() {
            None => agent::claude::Claude::of_host(),
            Some(data) => {
                let binary = std::env::current_exe()
                    .and_then(std::fs::canonicalize)
                    .unwrap_or_else(|_| "flume".into());
                agent::claude::Claude::new(&data, &binary)
            }
        };
        Some(Arc::new(claude))
    }

    /// The terminal profile named `profile`, built at most once for the
    /// command, the host's own the very one the core is built with; none
    /// for a name it does not hold, or before the core is built over it,
    /// with no state directory for the records (105, 190).
    fn terminal(&self, profile: &str) -> Option<Arc<dyn Terminal>> {
        if !terminal_profiles().iter().any(|name| name == profile) {
            return None;
        }
        let state = self.terminals.state.get()?;
        self.terminal_named(Some(profile), state).ok()
    }

    fn terminal_profiles(&self) -> Vec<String> {
        terminal_profiles()
    }

    /// The window profile named `profile`, Ghostty's started in the login
    /// environment the resolver was given where it was given one; none
    /// for a name it does not hold (105, 188).
    fn window(&self, profile: &str) -> Option<Arc<dyn Window>> {
        window(profile, self.login.as_deref())
    }

    fn window_profiles(&self) -> Vec<String> {
        window_profiles()
    }
}

/// The place profile over plain `git worktree` for one repository, bound
/// with the data its merge queue profile declares for the place's base
/// at each call (design 9, 13): the readiness, asked at the revision the
/// place's launch read, which a ready is given, so a retry readies as the
/// launch did; the teardown; and where the base lives; none of it where
/// no merge queue is bound (12, 20, C.1). The resolver hands the one
/// profile's data to the other, and neither calls the other (core.2). flume's own data directory, where every place goes
/// (record 0027), is the login environment's where the resolver was
/// given one, as the plugin's is.
struct BoundPlace {
    repository: String,
    merge_queue: Option<String>,
    login: Option<Vec<(String, String)>>,
    own: Option<PathBuf>,
}

/// The step a place's data is named by where the merge queue's policy
/// could not be read.
const POLICY_STEP: &str = "the merge queue's policy";

/// What the merge queue named `merge_queue` declares for the places of
/// `repository` on `base`, as the resolver binds the place profile with
/// it: the readiness, the merge queue's equipping of the place asked at
/// `revision`, the revision its launch read, where one is given, as one
/// step with the merge queue profile's reading of what it printed (12,
/// 20, 79); the teardown; and where the base lives; none of it where no
/// merge queue is bound (11).
pub fn place_data(
    repository: &str,
    merge_queue: Option<&str>,
    base: &str,
    revision: Option<&Revision>,
) -> Result<PlaceData, String> {
    match merge_queue {
        Some(merge_queue::sluice::NAME) => {
            let declared = merge_queue::sluice::for_place(repository, base, revision)?;
            let readiness = declared
                .equip
                .iter()
                .map(|equip| {
                    Step::read(
                        merge_queue::sluice::EQUIP,
                        equip,
                        merge_queue::sluice::equipped,
                    )
                })
                .collect();
            Ok(PlaceData {
                readiness,
                teardown: vec![Step::run(&declared.teardown, &declared.teardown)],
                base_remote: declared.base_remote,
            })
        }
        _ => Ok(PlaceData::default()),
    }
}

impl BoundPlace {
    /// What the merge queue declares for the places on `base`, the
    /// readiness asked at `revision` where one is given.
    fn data(&self, base: &str, revision: Option<&Revision>) -> Result<PlaceData, String> {
        place_data(
            &self.repository,
            self.merge_queue.as_deref(),
            base,
            revision,
        )
    }

    /// The profile bound with `data`.
    fn bound(&self, data: PlaceData) -> GitWorktree {
        let profile = match &self.login {
            Some(variables) => {
                let variables: Vec<(&str, &str)> = variables
                    .iter()
                    .map(|(name, value)| (name.as_str(), value.as_str()))
                    .collect();
                GitWorktree::with_login(data, Login::given(&variables))
            }
            None => GitWorktree::new(data),
        };
        match &self.own {
            Some(own) => profile.with_own(own),
            None => profile,
        }
    }
}

impl Place for BoundPlace {
    fn location(&self, repository: &str, place: &str) -> Result<String, String> {
        self.bound(PlaceData::default()).location(repository, place)
    }

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
        match self.data(&place.base, Some(revision)) {
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
        match self.data(&place.base, None) {
            Ok(data) => self.bound(data).remove(place),
            Err(words) => Removed::Refused {
                step: POLICY_STEP.into(),
                words,
            },
        }
    }

    fn remove_branch(&self, repository: &str, branch: &str, base: &str) -> BranchRemoved {
        match self.data(base, None) {
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
    vec![
        terminal::printed::NAME.into(),
        terminal::herdr::NAME.into(),
        terminal::rex::NAME.into(),
    ]
}

/// The terminal profile `name` binds for the host, its records under
/// flume's state directory `state`, its panes given exactly `login` for
/// the host user's login environment where it is given one, or the
/// refusal naming the profiles there are (105, C.5).
pub fn terminal(
    name: Option<&str>,
    state: &Path,
    login: Option<&[(String, String)]>,
) -> Result<Arc<dyn Terminal>, String> {
    match name {
        None | Some(terminal::printed::NAME) => Ok(Arc::new(terminal::printed::Printed)),
        Some(terminal::herdr::NAME) => {
            let mut herdr = terminal::herdr::Herdr::new(
                terminal::herdr::Server::Session(terminal::herdr::SESSION.into()),
                state.join("terminal/herdr"),
            )
            .with_caller(terminal::herdr::caller())
            .with_plugin(terminal::herdr::plugin_state(state));
            if let Some(login) = login {
                herdr = herdr.with_login(login);
            }
            Ok(Arc::new(herdr))
        }
        Some(terminal::rex::NAME) => {
            let mut rex = terminal::rex::Rex::new(state.join("terminal/rex"))
                .with_caller(terminal::rex::caller());
            if let Some(login) = login {
                rex = rex.with_login(login);
            }
            Ok(Arc::new(rex))
        }
        Some(other) => Err(format!(
            "the host's configuration names the terminal {other}, and no terminal profile is named {other}; the terminal profiles there are: {}",
            terminal_profiles().join(", ")
        )),
    }
}

/// The window profiles built in, by the name `terminal_app` gives them;
/// the printed window where none names one (105, 188, C.8).
pub fn window_profiles() -> Vec<String> {
    vec![
        window::ghostty::NAME.into(),
        window::terminal::NAME.into(),
        window::printed::NAME.into(),
    ]
}

/// The window profile `name` names, for the host's own system, Ghostty's
/// started in exactly `login` for the host user's login environment where
/// it is given one; none for a name no profile holds (105, 188, C.8).
pub fn window(name: &str, login: Option<&[(String, String)]>) -> Option<Arc<dyn Window>> {
    match name {
        window::printed::NAME => Some(Arc::new(window::printed::PrintedWindow)),
        window::ghostty::NAME => {
            let ghostty = window::ghostty::Ghostty::new();
            Some(Arc::new(match login {
                Some(login) => {
                    let login: Vec<(&str, &str)> = login
                        .iter()
                        .map(|(name, value)| (name.as_str(), value.as_str()))
                        .collect();
                    ghostty.with_login(&login)
                }
                None => ghostty,
            }))
        }
        window::terminal::NAME => Some(Arc::new(window::terminal::TerminalApp::new())),
        _ => None,
    }
}

/// The core over the profiles built in, for one command on this host:
/// flume's state in `$XDG_STATE_HOME/flume`, the host's configuration in
/// `$XDG_CONFIG_HOME/flume/config.toml` and `config.local.toml` over it,
/// and the recipient
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

/// The core over `resolver`, the terminal `config` binds, built through
/// `resolver` so the one it gives by that name is the core's own (190),
/// the store in flume's state directory `state`, git's history, and the
/// host with `config` and `recipient`, and the login environment
/// `resolver` was given where it was given one, with flume's
/// [`defaults`] (design 16).
pub fn core_over(
    state: &Path,
    config: HostConfig,
    recipient: Option<String>,
    resolver: BuiltIn,
) -> Result<Engine, String> {
    let login = resolver.login.clone();
    let resolver = resolver.over(state);
    let terminal = resolver.terminal_named(config.terminal.as_deref(), state)?;
    core_built(
        state,
        config,
        recipient,
        Arc::new(resolver),
        login,
        terminal,
    )
}

/// The core over any resolver, as [`core_over`] builds it over the
/// profiles built in: the terminal `config` binds, the one `resolver`
/// gives by that name where it gives one, so a start in the host's own
/// terminal reaches the core's own (190), the store in `state`, git's
/// history, and the host with `config`, `recipient` and exactly `login`
/// for the host user's login environment where it is given one (design
/// 16). A test binds a stand-in agent program through it.
pub fn core_with(
    state: &Path,
    config: HostConfig,
    recipient: Option<String>,
    resolver: Arc<dyn Resolver>,
    login: Option<Vec<(String, String)>>,
) -> Result<Engine, String> {
    let named = config.terminal.as_deref();
    let own = match resolver.terminal(named.unwrap_or(terminal::printed::NAME)) {
        Some(own) => own,
        None => terminal(named, state, login.as_deref())?,
    };
    core_built(state, config, recipient, resolver, login, own)
}

/// The core over `resolver` and the host's own `terminal`, as
/// [`core_with`] builds it.
fn core_built(
    state: &Path,
    config: HostConfig,
    recipient: Option<String>,
    resolver: Arc<dyn Resolver>,
    login: Option<Vec<(String, String)>>,
    terminal: Arc<dyn Terminal>,
) -> Result<Engine, String> {
    let mut host = RealHost::new(state, config, recipient);
    if let Some(login) = &login {
        host = host.with_login(login);
    }
    Ok(Engine::new(
        resolver,
        terminal,
        Arc::new(store::state_dir::StateDir::at(state)),
        Arc::new(GitHistory),
        Arc::new(host),
        defaults(),
    ))
}

/// The words `flume hook` takes, one per hook of the agent program's
/// plugin, and the status line its module pins (C.4; design 5).
pub fn hook_events() -> Vec<&'static str> {
    agent::claude::hook::EVENTS
        .iter()
        .map(|(word, _)| *word)
        .chain([agent::claude::status_line::EVENT])
        .collect()
}

/// Whether the hook for `event` reads its input on standard input: every
/// hook of the agent program's does, and the status line reads none
/// (design 5).
pub fn hook_reads_input(event: &str) -> bool {
    event != agent::claude::status_line::EVENT
}

/// What the agent program's hook for `event` answers to `input`, in the
/// agent this process's environment names, the core built by `build`
/// where the hook needs it: outside an agent flume started, nothing but
/// SessionStart's, what waits on the session the terminal names (43, C.4;
/// design 14).
pub fn hook(event: &str, input: &str, build: &dyn Fn() -> Result<Engine, String>) -> String {
    let binary = std::env::current_exe()
        .and_then(std::fs::canonicalize)
        .unwrap_or_else(|_| "flume".into());
    agent::claude::hook::hook(
        event,
        input,
        &agent::claude::hook::Start::of_process(),
        build,
        &binary,
    )
}

/// The started ack of the agent this process's environment names: its
/// session, transcript, source and plugin version, with the transcript's
/// length the ack measures, taken only where the start's record matches
/// it, the session among it where the start resumes one (135, 161, 179).
pub fn ack_started(
    session: &str,
    transcript: &str,
    source: &str,
    plugin_version: &str,
    build: &dyn Fn() -> Result<Engine, String>,
) -> Result<String, String> {
    let binary = std::env::current_exe()
        .and_then(std::fs::canonicalize)
        .unwrap_or_else(|_| "flume".into());
    agent::claude::hook::ack_started(
        &agent::claude::hook::Start::of_process(),
        &agent::claude::record::Started {
            session_id: session.into(),
            transcript_path: transcript.into(),
            source: source.into(),
            plugin_version: plugin_version.into(),
            from: 0,
        },
        build,
        &binary,
    )
}

/// The received ack of the agent this process's environment names: the
/// submitted prompt's digest, not taken at the start's first prompt where
/// the mode the session runs in, as its program names it, is not the one
/// its start recorded (89, 161, 162).
pub fn ack_received(
    digest: &str,
    mode: Option<&str>,
    build: &dyn Fn() -> Result<Engine, String>,
) -> Result<String, String> {
    let binary = std::env::current_exe()
        .and_then(std::fs::canonicalize)
        .unwrap_or_else(|_| "flume".into());
    agent::claude::hook::ack_received(
        &agent::claude::hook::Start::of_process(),
        digest,
        mode,
        "",
        "",
        build,
        &binary,
    )
    .map_err(agent::claude::hook::Unacked::words)
}

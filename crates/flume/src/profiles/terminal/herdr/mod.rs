//! The terminal profile over herdr (C.5; design 12; records 0016, 0018),
//! every command, state name, error code and field it reads checked
//! against the herdr the tree pins, 0.9.5 (core.29).
//!
//! open makes a pane in the place with the agent's environment set on
//! it, the host user's login environment under it, and a shell of the
//! start's own that reads no startup file, so nothing the host's shell
//! files do reorders the pane's `PATH`: a split beside the pane the settings name, by its label or id on
//! flume's server or as `herdr:<pane>@<socket>`; otherwise, in flume's
//! own herdr session, the place's tab `<kind>--<name>` in the workspace
//! of the place's repository and base, labelled `<repository> <base>`,
//! each made when first needed and recorded by id and server. The pane's
//! `PATH` is led by a folder of the start's own linking the program's
//! kind to the program the settings name, since herdr's agent start
//! types the kind's name into the pane's shell; agent start then runs
//! it with the command's arguments whole, under the agent's herdr name,
//! and nothing is typed (94, 107). Whatever agent start answers, the
//! folder-trust question is read off the pane's screen and answered yes
//! with the keys the command names, the focus moved to the yes choice and
//! confirmed only there, read again until it is gone; a start held at
//! any other question is held with the screen, and one herdr never sees
//! ready, with no trust question on its screen, is not come up with the
//! screen (34, 35). The
//! program in the pane's foreground must be the one the settings name,
//! or the start is closed and refused naming what ran (32). A pane is
//! held by its id and its server's socket, and found again by the
//! agent's herdr name where its id no longer answers, its new id
//! recorded before anything is sent (30, 98).
//!
//! send clears what the agent holds typed with one ctrl+c, then hands the
//! text to herdr's agent prompt (95). state reads herdr's working as
//! working, idle and done as waiting, blocked as asking, which the core
//! takes as asking only with the agent's mark (90), and herdr's unknown
//! as unknown; stopped only when the pane is gone, herdr lists it with a
//! `restore_error`, or herdr lists no agent in it and its foreground is
//! the shell (96). screen is the pane's visible lines (136); close ends
//! the pane and reads that it is gone (97). tidy closes a gone place's
//! tab, and its base's workspace once no tab of a live place and no pane
//! flume did not open is in it, keeping and naming either where a pane
//! flume did not open stands (137, 138). show titles the agent's pane
//! with its stage and state and gives the base's workspace its count as
//! the token `flume` (139), and says where it shows them, the place's
//! tab, its agent's pane and its base's workspace each as
//! `herdr:<id>@<socket>` from its records, for flume's herdr plugin to
//! refresh them (C.5). The watch is flume's herdr plugin's: while a
//! watch of it has renewed its lease within the lease's term, a minute,
//! the profile answers its bound, thirty seconds, and no watch
//! otherwise, reading the lease and starting nothing (119, 122; design
//! 10). The session a command runs in is `herdr:<pane>@<socket>`, its
//! pane on its server (7).
//!
//! A recipient's session is reached by its name (43, 132; records 0016,
//! 0018): `herdr:<pane>@<socket>` by that pane on that server, its id
//! alone, since a plain recipient's pane carries no name of flume's; and
//! a name an open gave an agent, a role run's address, by the pane its
//! live start holds, found again by that start's herdr name where the id
//! no longer answers, its new id recorded before anything is sent. A
//! notice goes by herdr's agent prompt into a session working or
//! waiting, the held input cleared first in one that waits, and is
//! refused, nothing typed, for a session asking or unread; a working
//! session is not interrupted to clear it, its program queuing the
//! notice as its next turn.
//!
//! Every herdr call runs under a bound, ten seconds where the call states
//! none, and one that does not answer within it is unknown or a refusal
//! naming the bound; each runs with the caller's `HERDR_*` stripped and
//! the recorded server's socket passed (98, core.29).

use flume_seams::{
    AgentCommand, AgentState, Closed, Clues, FolderTrust, Identity, Opened, PlaceRef, Sent, Shown,
    ShownAt, Terminal, TerminalTidy, ToSession, Watch,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

/// The profile's name in the host's configuration, `[host] terminal`
/// (105).
pub const NAME: &str = "herdr";

/// The bound on a herdr call that states none (98).
pub const BOUND: Duration = Duration::from_secs(10);

/// The bound flume's herdr plugin settles every place on while its watch
/// runs (122).
pub const WATCH_BOUND: flume_seams::Seconds = 30;

/// How long a lease of the plugin's watch is fresh after it was last
/// renewed; the watch renews it four times as often.
pub const LEASE_TERM: Duration = Duration::from_secs(60);

/// How long agent start waits for the program to be ready: room for
/// SessionStart hooks slow under load (record 0018).
pub const START_TIMEOUT: Duration = Duration::from_secs(120);

/// How long, once the folder-trust question is answered, herdr is waited
/// on for the agent to idle: herdr's own default for a start.
pub const TRUST_WAIT: Duration = Duration::from_secs(30);

/// How long apart the screen is read while the folder-trust question is
/// answered.
const TRUST_BEAT: Duration = Duration::from_millis(250);

/// How long the screen is read for the folder-trust question to be drawn
/// or the screen to hold still without it.
const TRUST_SETTLE: Duration = Duration::from_secs(5);

/// How long a key sent at the folder-trust question is given to show on
/// the screen before it is sent again, as one sent before the program
/// read its keys is lost.
const TRUST_REDRAW: Duration = Duration::from_secs(2);

/// How far past the timeout it passes a call that states one is bounded.
const PAST_TIMEOUT: Duration = Duration::from_secs(10);

/// The kinds herdr 0.9.5's agent start takes, each its program's
/// canonical name (`herdr agent`).
pub const KINDS: [&str; 24] = [
    "pi",
    "claude",
    "codex",
    "gemini",
    "cursor",
    "devin",
    "agy",
    "cline",
    "omp",
    "mastracode",
    "opencode",
    "copilot",
    "kimi",
    "kiro",
    "droid",
    "amp",
    "grok",
    "hermes",
    "kilo",
    "qodercli",
    "qwen",
    "letta",
    "maki",
    "muse",
];

/// The herdr session flume opens its places' panes in, where the settings
/// name no pane.
pub const SESSION: &str = "flume";

/// How often a reading of herdr is taken again while a server starts.
const POLL: Duration = Duration::from_millis(200);

/// The server open makes a place's pane on, where the settings name no
/// pane.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Server {
    /// The server at this socket.
    Socket(PathBuf),
    /// The named herdr session, its server started headless where none
    /// runs.
    Session(String),
}

/// A pane, by its id and its server's socket.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pane {
    pub id: String,
    pub socket: PathBuf,
}

impl Pane {
    /// The pane as flume names a session, `herdr:<pane>@<socket>` (7).
    pub fn session(&self) -> String {
        format!("herdr:{}@{}", self.id, self.socket.display())
    }

    /// The pane a session name `herdr:<pane>@<socket>` names.
    pub fn named(session: &str) -> Option<Pane> {
        let (id, socket) = session.strip_prefix("herdr:")?.split_once('@')?;
        (!id.is_empty() && !socket.is_empty()).then(|| Pane {
            id: id.into(),
            socket: socket.into(),
        })
    }
}

/// What a herdr call came to.
#[derive(Clone, Debug, PartialEq)]
pub enum Call {
    /// herdr answered: its `result`, or its text where it answers text.
    Answered(Value),
    /// herdr refused, with its code and its words.
    Refused { code: String, words: String },
    /// No answer within the bound, or herdr could not be run.
    Unanswered(String),
}

/// The terminal over herdr.
#[derive(Clone, Debug)]
pub struct Herdr {
    program: PathBuf,
    server: Server,
    records: PathBuf,
    bound: Duration,
    start_timeout: Duration,
    caller: Option<Pane>,
    login: Arc<OnceLock<BTreeMap<String, String>>>,
    plugin: Option<PathBuf>,
}

/// A pane as open recorded it for an agent: its id, its server, the
/// agent's herdr name and the place it was opened for.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Kept {
    pane: Pane,
    name: String,
    place: Option<String>,
}

/// Where a pane was found: there, gone, or not readable now.
enum Found {
    At(Pane),
    Gone,
    Unknown(String),
}

/// What a session name an open gave an agent names: the agent.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct Named {
    session: String,
    identity: Identity,
}

impl Herdr {
    /// The host's herdr: flume's own session, the panes, tabs and
    /// workspaces recorded under `$XDG_STATE_HOME/flume/terminal/herdr`,
    /// and the pane the calling command runs in, as herdr's environment
    /// names it.
    pub fn of_host() -> Result<Herdr, String> {
        let state = crate::host::config::flume_state()?;
        Ok(Herdr::new(
            Server::Session(SESSION.into()),
            state.join("terminal/herdr"),
        )
        .with_caller(caller())
        .with_plugin(plugin_state(&state)))
    }

    /// herdr opening on `server`, recording what it makes under
    /// `records`; the calling command in no pane.
    pub fn new(server: Server, records: impl Into<PathBuf>) -> Herdr {
        Herdr {
            program: "herdr".into(),
            server,
            records: records.into(),
            bound: BOUND,
            start_timeout: START_TIMEOUT,
            caller: None,
            login: Arc::default(),
            plugin: None,
        }
    }

    /// The state directory of flume's herdr plugin, whose watch's lease
    /// says the terminal keeps watch; none, no watch.
    pub fn with_plugin(mut self, plugin: Option<PathBuf>) -> Herdr {
        self.plugin = plugin;
        self
    }

    /// Whether a watch of flume's herdr plugin, on any herdr server, has
    /// renewed its lease within the lease's term: the lease's first line
    /// is the watch's pid and when it last renewed, in seconds since the
    /// epoch (design 10).
    pub fn watch_kept(&self) -> bool {
        let Some(plugin) = &self.plugin else {
            return false;
        };
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |since| since.as_secs());
        fs::read_dir(plugin.join("servers"))
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|server| fs::read_to_string(server.path().join("watch.lease")).ok())
            .filter_map(|lease| {
                let first = lease.lines().next()?.to_string();
                first.split_whitespace().nth(1)?.parse::<u64>().ok()
            })
            .any(|renewed| renewed <= now + 5 && now.saturating_sub(renewed) < LEASE_TERM.as_secs())
    }

    /// Exactly `variables` for the host user's login environment, in
    /// place of what the user's login shell sets.
    pub fn with_login(self, variables: &[(String, String)]) -> Herdr {
        let login = OnceLock::new();
        let _ = login.set(variables.iter().cloned().collect());
        Herdr {
            login: Arc::new(login),
            ..self
        }
    }

    /// The host user's login environment, read once (29).
    fn login(&self) -> &BTreeMap<String, String> {
        self.login.get_or_init(crate::host::login_environment)
    }

    /// The bound on each call, in place of ten seconds.
    pub fn with_bound(mut self, bound: Duration) -> Herdr {
        self.bound = bound;
        self
    }

    /// How long agent start waits for the program, in place of 120
    /// seconds; herdr takes 3001 milliseconds to 300 seconds.
    pub fn with_start_timeout(mut self, timeout: Duration) -> Herdr {
        self.start_timeout = timeout;
        self
    }

    /// The pane the calling command runs in, if any.
    pub fn with_caller(mut self, caller: Option<Pane>) -> Herdr {
        self.caller = caller;
        self
    }

    /// The pane the agent under `identity` runs in, as last recorded.
    pub fn pane(&self, identity: &Identity) -> Option<Pane> {
        self.kept(identity).map(|kept| kept.pane)
    }

    /// The workspace recorded for the place's repository and base, by id
    /// and server.
    pub fn workspace(&self, place: &PlaceRef) -> Option<(String, PathBuf)> {
        let record = read_json(&self.workspace_record(place))?;
        Some((
            record.get("workspace")?.as_str()?.into(),
            record.get("socket")?.as_str()?.into(),
        ))
    }

    /// The tab recorded for the place, by id and server.
    pub fn tab(&self, place: &PlaceRef) -> Option<(String, PathBuf)> {
        let record = read_json(&self.tab_record(&place.location))?;
        Some((
            record.get("tab")?.as_str()?.into(),
            record.get("socket")?.as_str()?.into(),
        ))
    }

    /// herdr run with `args` on the server at `socket`, under `bound`:
    /// its JSON `result`, its refusal, or no answer.
    pub fn call(&self, socket: &Path, args: &[&str], bound: Duration) -> Call {
        let (success, stdout, stderr) = match self.run(Some(socket), args, bound) {
            Ok(output) => output,
            Err(words) => return Call::Unanswered(words),
        };
        for text in [&stdout, &stderr] {
            let Ok(answer) = serde_json::from_str::<Value>(text.trim()) else {
                continue;
            };
            if let Some(error) = answer.get("error") {
                return Call::Refused {
                    code: field(error, "code"),
                    words: field(error, "message"),
                };
            }
            if let Some(result) = answer.get("result") {
                return Call::Answered(result.clone());
            }
        }
        if success {
            Call::Answered(Value::String(stdout))
        } else {
            Call::Refused {
                code: "failed".into(),
                words: format!("herdr {}: {}", unvalued(args), stderr.trim()),
            }
        }
    }

    /// herdr run with `args`, every `HERDR_*` of the caller's stripped and
    /// `socket` passed where one is named, ended once `bound` has passed:
    /// whether it succeeded, and its output and its errors.
    fn run(
        &self,
        socket: Option<&Path>,
        args: &[&str],
        bound: Duration,
    ) -> Result<(bool, String, String), String> {
        let mut command = Command::new(&self.program);
        command
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (name, _) in std::env::vars_os() {
            if name.to_string_lossy().starts_with("HERDR_") {
                command.env_remove(name);
            }
        }
        if let Some(socket) = socket {
            command.env("HERDR_SOCKET_PATH", socket);
        }
        let mut child = command
            .spawn()
            .map_err(|error| format!("herdr could not be run: {error}"))?;
        let read = |pipe: Option<Box<dyn Read + Send>>| {
            std::thread::spawn(move || {
                let mut text = String::new();
                if let Some(mut pipe) = pipe {
                    let _ = pipe.read_to_string(&mut text);
                }
                text
            })
        };
        let stdout = read(child.stdout.take().map(|pipe| Box::new(pipe) as _));
        let stderr = read(child.stderr.take().map(|pipe| Box::new(pipe) as _));
        let deadline = Instant::now() + bound;
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) if Instant::now() < deadline => {
                    std::thread::sleep(Duration::from_millis(10))
                }
                Ok(None) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err(format!(
                        "herdr {} did not answer within {} seconds",
                        args.first().copied().unwrap_or_default(),
                        bound.as_secs_f32()
                    ));
                }
                Err(error) => return Err(format!("herdr could not be waited on: {error}")),
            }
        };
        Ok((
            status.success(),
            stdout.join().unwrap_or_default(),
            stderr.join().unwrap_or_default(),
        ))
    }

    /// herdr's answer under the profile's bound.
    fn ask(&self, socket: &Path, args: &[&str]) -> Call {
        self.call(socket, args, self.bound)
    }

    fn agent_record(&self, identity: &Identity) -> PathBuf {
        self.records
            .join(format!("{}.json", identity.to_string().replace('/', "%")))
    }

    fn workspace_record(&self, place: &PlaceRef) -> PathBuf {
        self.records.join("workspaces").join(format!(
            "{}.json",
            digest(&format!("{}\n{}", place.repository, place.base))
        ))
    }

    fn tab_record(&self, location: &str) -> PathBuf {
        self.records
            .join("tabs")
            .join(format!("{}.json", digest(location)))
    }

    fn kept(&self, identity: &Identity) -> Option<Kept> {
        let record = read_json(&self.agent_record(identity))?;
        Some(Kept {
            pane: Pane {
                id: record.get("pane")?.as_str()?.into(),
                socket: record.get("socket")?.as_str()?.into(),
            },
            name: record
                .get("name")
                .and_then(Value::as_str)
                .map_or_else(|| herdr_name(identity), str::to_string),
            place: record
                .get("place")
                .and_then(Value::as_str)
                .map(str::to_string),
        })
    }

    fn keep(&self, identity: &Identity, kept: &Kept) -> Result<(), String> {
        write_json(
            &self.agent_record(identity),
            &json!({
                "pane": kept.pane.id,
                "socket": kept.pane.socket,
                "name": kept.name,
                "place": kept.place,
            }),
        )
    }

    /// Every pane flume opened, as its records name them.
    fn opened_panes(&self) -> BTreeSet<String> {
        fs::read_dir(&self.records)
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "json"))
            .filter_map(|entry| read_json(&entry.path()))
            .filter_map(|record| record.get("pane")?.as_str().map(str::to_string))
            .collect()
    }

    /// Every tab flume made for a place still recorded, by id.
    fn place_tabs(&self) -> BTreeMap<String, String> {
        fs::read_dir(self.records.join("tabs"))
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|entry| read_json(&entry.path()))
            .filter_map(|record| {
                Some((
                    record.get("tab")?.as_str()?.to_string(),
                    record.get("place")?.as_str()?.to_string(),
                ))
            })
            .collect()
    }

    /// The agent's pane: by its recorded id while herdr lists it there,
    /// else found again by the agent's herdr name, taken only once herdr
    /// lists the pane under that name, its new id recorded before
    /// anything is sent (30, 98; record 0018).
    fn found(&self, identity: &Identity) -> Found {
        let Some(mut kept) = self.kept(identity) else {
            return Found::Gone;
        };
        let pane = &kept.pane;
        match self.ask(&pane.socket, &["pane", "get", &pane.id]) {
            Call::Answered(info) if info["pane"]["pane_id"].as_str() == Some(pane.id.as_str()) => {
                return Found::At(pane.clone())
            }
            Call::Answered(_) => {}
            Call::Refused { code, .. } if code == "pane_not_found" => {}
            Call::Refused { code, .. } if code == "server_not_running" => return Found::Gone,
            Call::Refused { code, words } => {
                return Found::Unknown(format!("herdr refused ({code}): {words}"))
            }
            Call::Unanswered(words) => return Found::Unknown(words),
        }
        match self.ask(&pane.socket, &["agent", "get", &kept.name]) {
            Call::Answered(answer) => {
                let agent = &answer["agent"];
                let (Some(name), Some(id)) = (agent["name"].as_str(), agent["pane_id"].as_str())
                else {
                    return Found::Gone;
                };
                if name != kept.name {
                    return Found::Gone;
                }
                kept.pane.id = id.into();
                match self.keep(identity, &kept) {
                    Ok(()) => Found::At(kept.pane),
                    Err(reason) => Found::Unknown(reason),
                }
            }
            Call::Refused { code, .. } if code == "agent_not_found" => Found::Gone,
            Call::Refused { code, words } => {
                Found::Unknown(format!("herdr refused ({code}): {words}"))
            }
            Call::Unanswered(words) => Found::Unknown(words),
        }
    }

    fn session_record(&self, session: &str) -> PathBuf {
        self.records
            .join("sessions")
            .join(format!("{}.json", digest(session)))
    }

    /// The agent an open named `session`, as recorded.
    fn named(&self, session: &str) -> Option<Identity> {
        let record = read_json(&self.session_record(session))?;
        let named: Named = serde_json::from_value(record).ok()?;
        (named.session == session).then_some(named.identity)
    }

    /// The pane a session name reaches: `herdr:<pane>@<socket>` by its id
    /// alone while herdr lists it there, and a name an open gave an agent
    /// by the pane its live start holds, found again by its herdr name
    /// (43, 132; record 0018).
    fn reach(&self, session: &str) -> Found {
        if let Some(pane) = Pane::named(session) {
            return match self.ask(&pane.socket, &["pane", "get", &pane.id]) {
                Call::Answered(info)
                    if info["pane"]["pane_id"].as_str() == Some(pane.id.as_str()) =>
                {
                    Found::At(pane)
                }
                Call::Answered(_) => Found::Gone,
                Call::Refused { code, .. }
                    if code == "pane_not_found" || code == "server_not_running" =>
                {
                    Found::Gone
                }
                Call::Refused { code, words } => {
                    Found::Unknown(format!("herdr refused ({code}): {words}"))
                }
                Call::Unanswered(words) => Found::Unknown(words),
            };
        }
        match self.named(session) {
            Some(identity) => self.found(&identity),
            None => Found::Gone,
        }
    }

    /// The state of the agent in `pane`, as herdr reads it (96).
    fn pane_state(&self, pane: &Pane) -> AgentState {
        let info = match self.ask(&pane.socket, &["pane", "get", &pane.id]) {
            Call::Answered(info) => info,
            Call::Refused { code, .. }
                if code == "pane_not_found" || code == "server_not_running" =>
            {
                return AgentState::Stopped
            }
            Call::Refused { .. } | Call::Unanswered(_) => return AgentState::Unknown,
        };
        let info = info.get("pane").cloned().unwrap_or_default();
        if info
            .get("restore_error")
            .is_some_and(|error| !error.is_null())
        {
            return AgentState::Stopped;
        }
        if info.get("agent").is_some_and(|agent| !agent.is_null()) {
            return match info.get("agent_status").and_then(Value::as_str) {
                Some("working") => AgentState::Working,
                Some("idle" | "done") => AgentState::Waiting,
                Some("blocked") => AgentState::Asking,
                _ => AgentState::Unknown,
            };
        }
        let Call::Answered(processes) =
            self.ask(&pane.socket, &["pane", "process-info", "--pane", &pane.id])
        else {
            return AgentState::Unknown;
        };
        let processes = processes.get("process_info").cloned().unwrap_or_default();
        let shell = processes.get("shell_pid").and_then(Value::as_u64);
        let foreground = processes
            .get("foreground_process_group_id")
            .and_then(Value::as_u64);
        if shell.is_some() && shell == foreground {
            AgentState::Stopped
        } else {
            AgentState::Unknown
        }
    }

    /// The socket of the server open makes a place's pane on, its session's
    /// server started headless where none runs.
    fn socket(&self) -> Result<PathBuf, String> {
        let name = match &self.server {
            Server::Socket(socket) => return Ok(socket.clone()),
            Server::Session(name) => name,
        };
        let (_, listing, _) = self.run(None, &["session", "list", "--json"], self.bound)?;
        let sessions: Value = serde_json::from_str(listing.trim())
            .map_err(|error| format!("herdr session list --json: {error}"))?;
        let session = sessions
            .get("sessions")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find(|session| session.get("name").and_then(Value::as_str) == Some(name));
        let socket = match session.and_then(|session| session.get("socket_path")) {
            Some(Value::String(socket)) => PathBuf::from(socket),
            _ => return self.start_session(name),
        };
        if session.and_then(|session| session.get("running")) == Some(&Value::Bool(true)) {
            return Ok(socket);
        }
        self.start_session(name)
    }

    /// Starts the named session's server headless, apart from this
    /// command, and gives its socket once it answers.
    fn start_session(&self, name: &str) -> Result<PathBuf, String> {
        let mut command = Command::new(&self.program);
        command
            .args(["--session", name, "server"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        for (variable, _) in std::env::vars_os() {
            if variable.to_string_lossy().starts_with("HERDR_") {
                command.env_remove(variable);
            }
        }
        std::os::unix::process::CommandExt::process_group(&mut command, 0);
        command
            .spawn()
            .map_err(|error| format!("herdr's session {name} could not be started: {error}"))?;
        let deadline = Instant::now() + self.bound;
        loop {
            let (_, listing, _) = self.run(None, &["session", "list", "--json"], self.bound)?;
            let running = serde_json::from_str::<Value>(listing.trim())
                .ok()
                .and_then(|sessions| {
                    sessions
                        .get("sessions")?
                        .as_array()?
                        .iter()
                        .find(|session| {
                            session.get("name").and_then(Value::as_str) == Some(name)
                                && session.get("running") == Some(&Value::Bool(true))
                        })?
                        .get("socket_path")?
                        .as_str()
                        .map(PathBuf::from)
                });
            if let Some(socket) = running {
                return Ok(socket);
            }
            if Instant::now() >= deadline {
                return Err(format!(
                    "herdr's session {name} did not start within {} seconds",
                    self.bound.as_secs()
                ));
            }
            std::thread::sleep(POLL);
        }
    }

    /// The pane the settings name, by `herdr:<pane>@<socket>`, or by its
    /// label or id on flume's server.
    fn named_pane(&self, beside: &str) -> Result<Pane, String> {
        if let Some(pane) = Pane::named(beside) {
            return Ok(pane);
        }
        let socket = self.socket()?;
        let panes = self.answered(&socket, &["pane", "list"])?;
        panes
            .get("panes")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find(|pane| {
                pane.get("pane_id").and_then(Value::as_str) == Some(beside)
                    || pane.get("label").and_then(Value::as_str) == Some(beside)
            })
            .and_then(|pane| pane.get("pane_id").and_then(Value::as_str))
            .map(|id| Pane {
                id: id.into(),
                socket: socket.clone(),
            })
            .ok_or_else(|| format!("no herdr pane is named {beside}"))
    }

    /// herdr's result, or its refusal or silence in its words.
    fn answered(&self, socket: &Path, args: &[&str]) -> Result<Value, String> {
        match self.ask(socket, args) {
            Call::Answered(result) => Ok(result),
            Call::Refused { code, words } => Err(format!("herdr refused ({code}): {words}")),
            Call::Unanswered(words) => Err(words),
        }
    }

    /// The `--cwd` and `--env` arguments of a pane made for the agent in
    /// `start`, the start's own folder: the place, and the host user's
    /// login environment with the agent's environment over it, `PATH`
    /// led by `start`'s `bin`, linking the kind to the program, and
    /// `SHELL` the start's `shell`, which herdr runs as the pane's shell
    /// and which reads no startup file, so nothing reorders that `PATH`
    /// (29, 32, C.5).
    fn made(
        &self,
        place: &PlaceRef,
        command: &AgentCommand,
        start: &Path,
    ) -> Result<Vec<String>, String> {
        let login = self.login();
        let shell = start.join("shell");
        let script = pane_shell(login.get("SHELL").map_or("/bin/sh", String::as_str));
        fs::write(&shell, script)
            .and_then(|()| fs::set_permissions(&shell, fs::Permissions::from_mode(0o755)))
            .map_err(|error| format!("{}: {error}", shell.display()))?;
        let path = command
            .env
            .get("PATH")
            .cloned()
            .or_else(|| std::env::var("PATH").ok())
            .unwrap_or_else(|| "/usr/bin:/bin".into());
        let mut env: BTreeMap<String, String> = login
            .iter()
            .filter(|(name, _)| !UNSET.contains(&name.as_str()) && !name.starts_with("HERDR_"))
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect();
        env.extend(command.env.clone());
        env.insert("SHELL".into(), shell.display().to_string());
        env.insert(
            "PATH".into(),
            format!("{}:{path}", start.join("bin").display()),
        );
        let mut args = vec!["--cwd".to_string(), place.location.clone()];
        for (name, value) in env {
            args.extend(["--env".to_string(), format!("{name}={value}")]);
        }
        Ok(args)
    }

    /// A new pane for the agent in the place, made with `made`'s cwd and
    /// environment: a split beside the pane named, or in the place's tab
    /// of the workspace of its repository and base, each made and
    /// recorded when first needed (94, 107; record 0018).
    fn new_pane(
        &self,
        place: &PlaceRef,
        short: &str,
        beside: Option<&str>,
        made: &[String],
    ) -> Result<Pane, String> {
        let split = |pane: &Pane| -> Result<Pane, String> {
            let mut args: Vec<&str> = vec!["pane", "split", &pane.id, "--direction", "right"];
            args.extend(made.iter().map(String::as_str));
            args.push("--no-focus");
            let split = self.answered(&pane.socket, &args)?;
            pane_id(&split, "pane").map(|id| Pane {
                id,
                socket: pane.socket.clone(),
            })
        };
        if let Some(beside) = beside {
            return split(&self.named_pane(beside)?);
        }
        let socket = self.socket()?;
        let workspace = self.standing_workspace(place, &socket)?;
        let Some(workspace) = workspace else {
            let label = workspace_label(place);
            let mut args: Vec<&str> = vec!["workspace", "create"];
            args.extend(made.iter().map(String::as_str));
            args.extend(["--label", &label, "--no-focus"]);
            let created = self.answered(&socket, &args)?;
            let workspace = created["workspace"]["workspace_id"]
                .as_str()
                .ok_or_else(|| format!("herdr gave no workspace in {created}"))?;
            write_json(
                &self.workspace_record(place),
                &json!({
                    "workspace": workspace,
                    "socket": socket,
                    "repository": place.repository,
                    "base": place.base,
                    "label": label,
                }),
            )?;
            let tab = created["tab"]["tab_id"]
                .as_str()
                .ok_or_else(|| format!("herdr gave no tab in {created}"))?;
            self.answered(&socket, &["tab", "rename", tab, short])?;
            self.keep_tab(place, tab, workspace, &socket, short)?;
            return pane_id(&created, "root_pane").map(|id| Pane { id, socket });
        };
        if let Some((tab, _)) = self.tab(place) {
            let panes = self.answered(&socket, &["pane", "list", "--workspace", &workspace])?;
            let in_tab = panes["panes"]
                .as_array()
                .into_iter()
                .flatten()
                .find(|pane| pane["tab_id"].as_str() == Some(tab.as_str()))
                .and_then(|pane| pane["pane_id"].as_str());
            if let Some(id) = in_tab {
                return split(&Pane {
                    id: id.into(),
                    socket: socket.clone(),
                });
            }
        }
        let mut args: Vec<&str> = vec!["tab", "create", "--workspace", &workspace];
        args.extend(made.iter().map(String::as_str));
        args.extend(["--label", short, "--no-focus"]);
        let created = self.answered(&socket, &args)?;
        let tab = created["tab"]["tab_id"]
            .as_str()
            .ok_or_else(|| format!("herdr gave no tab in {created}"))?;
        self.keep_tab(place, tab, &workspace, &socket, short)?;
        pane_id(&created, "root_pane").map(|id| Pane { id, socket })
    }

    fn keep_tab(
        &self,
        place: &PlaceRef,
        tab: &str,
        workspace: &str,
        socket: &Path,
        short: &str,
    ) -> Result<(), String> {
        write_json(
            &self.tab_record(&place.location),
            &json!({
                "tab": tab,
                "workspace": workspace,
                "socket": socket,
                "label": short,
                "place": place.location,
            }),
        )
    }

    /// The workspace recorded for the place's repository and base where
    /// herdr still lists it on `socket`; its record dropped otherwise.
    fn standing_workspace(
        &self,
        place: &PlaceRef,
        socket: &Path,
    ) -> Result<Option<String>, String> {
        let Some((workspace, recorded)) = self.workspace(place) else {
            return Ok(None);
        };
        if recorded != socket {
            return Ok(None);
        }
        let listing = self.answered(socket, &["workspace", "list"])?;
        let listed = listing["workspaces"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|listed| listed["workspace_id"].as_str() == Some(workspace.as_str()));
        if listed {
            return Ok(Some(workspace));
        }
        let _ = fs::remove_file(self.workspace_record(place));
        Ok(None)
    }

    /// The pane's visible lines once they show the folder-trust question,
    /// or once they hold still without it, two reads a beat apart
    /// agreeing, so a dialog half drawn is not taken for what it shows;
    /// for up to [`TRUST_SETTLE`]. Where herdr said the program is held at
    /// a question, `blocked`, a screen without the question is read until
    /// the bound, as the dialog may be drawn late.
    fn settled(&self, pane: &Pane, trust: &FolderTrust, blocked: bool) -> String {
        let deadline = Instant::now() + TRUST_SETTLE;
        let mut last = self.visible(pane);
        while !asks(&last, trust) && Instant::now() < deadline {
            std::thread::sleep(TRUST_BEAT);
            let screen = self.visible(pane);
            if screen == last && !blocked {
                break;
            }
            last = screen;
        }
        last
    }

    /// Answers the folder-trust question in `pane` from its screen, within
    /// [`TRUST_WAIT`]: the next key while another choice has the focus,
    /// the confirming key only while the yes choice has it, each key sent
    /// once the screen shows what the last did, and read again until the
    /// question is gone. Nothing is ever confirmed on another choice, so
    /// "No, exit" is never answered. Where the question is not on the
    /// screen once it holds still, or by [`TRUST_SETTLE`] where herdr said
    /// the program is `blocked`, it was not asked (34; record 0018).
    fn trust(&self, pane: &Pane, trust: &FolderTrust, blocked: bool) -> Trusted {
        let deadline = Instant::now() + TRUST_WAIT;
        let mut screen = self.settled(pane, trust, blocked);
        if !asks(&screen, trust) {
            return Trusted::NotAsked(screen);
        }
        // The screen a key was last sent at, and when.
        let mut sent: Option<(String, Instant)> = None;
        loop {
            let Some(focused) = focused(&screen, trust) else {
                return Trusted::Answered;
            };
            if Instant::now() >= deadline {
                return Trusted::Unanswered(screen);
            }
            let waiting = sent
                .as_ref()
                .is_some_and(|(at, when)| *at == screen && when.elapsed() < TRUST_REDRAW);
            if !waiting {
                let key = if focused { &trust.confirm } else { &trust.next };
                // By the pane, which herdr answers whether or not it
                // lists an agent in it.
                let mut keys = vec!["pane", "send-keys", pane.id.as_str()];
                keys.extend(key.split_whitespace());
                if let Err(reason) = self.answered(&pane.socket, &keys) {
                    return Trusted::Unanswered(format!("{screen}\n{reason}"));
                }
                sent = Some((screen.clone(), Instant::now()));
            }
            std::thread::sleep(TRUST_BEAT);
            screen = self.visible(pane);
        }
    }

    /// The pane's visible lines, or herdr's words where it gave none.
    fn visible(&self, pane: &Pane) -> String {
        match self.ask(
            &pane.socket,
            &[
                "pane", "read", &pane.id, "--source", "visible", "--format", "text",
            ],
        ) {
            Call::Answered(Value::String(text)) => text,
            Call::Answered(other) => other.to_string(),
            Call::Refused { code, words } => format!("herdr refused ({code}): {words}"),
            Call::Unanswered(words) => words,
        }
    }

    /// The pane closed, its agent's record and the start's folder with it.
    fn abandon(&self, identity: &Identity, pane: &Pane, bin: &Path) {
        let _ = self.ask(&pane.socket, &["pane", "close", &pane.id]);
        let _ = fs::remove_file(self.agent_record(identity));
        if let Some(start) = bin.parent() {
            let _ = fs::remove_dir_all(start);
        }
    }

    /// Whether the program in the pane's foreground is `program`, by its
    /// path or, for a program a script starts, the script's path among its
    /// arguments; or what ran in its place (32; record 0018).
    fn runs(&self, pane: &Pane, program: &Path) -> Result<(), String> {
        let info = self.answered(&pane.socket, &["pane", "process-info", "--pane", &pane.id])?;
        let processes = info["process_info"]["foreground_processes"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        let wanted = program.canonicalize().unwrap_or_else(|_| program.into());
        let mut ran = vec![];
        for process in &processes {
            let cwd = process["cwd"].as_str().map(PathBuf::from);
            let executable = process["pid"]
                .as_u64()
                .and_then(|pid| executable(pid as u32));
            let mut paths: Vec<PathBuf> = executable.iter().cloned().collect();
            for arg in process["argv"].as_array().into_iter().flatten() {
                let Some(arg) = arg.as_str() else {
                    continue;
                };
                let path = Path::new(arg);
                if path.is_absolute() {
                    paths.push(path.into());
                } else if arg.contains('/') {
                    if let Some(cwd) = &cwd {
                        paths.push(cwd.join(path));
                    }
                }
            }
            if paths
                .iter()
                .filter_map(|path| path.canonicalize().ok())
                .any(|path| path == wanted)
            {
                return Ok(());
            }
            let cmdline = process["cmdline"].as_str().map(str::to_string);
            ran.push(match (executable, cmdline) {
                (Some(path), Some(cmdline)) => format!("{} ({cmdline})", path.display()),
                (Some(path), None) => path.display().to_string(),
                (None, Some(cmdline)) => cmdline,
                (None, None) => process.to_string(),
            });
        }
        Err(if ran.is_empty() {
            format!(
                "herdr shows nothing running in the pane {}, not {}",
                pane.id,
                program.display()
            )
        } else {
            format!(
                "the pane {} ran {} rather than {}, the program the settings name",
                pane.id,
                ran.join(", "),
                program.display()
            )
        })
    }

    /// The panes on `socket` that `within` takes, flume opened none of,
    /// by id.
    fn foreign_panes(
        &self,
        socket: &Path,
        workspace: &str,
        opened: &BTreeSet<String>,
        within: impl Fn(&Value) -> bool,
    ) -> Result<Vec<String>, String> {
        let panes = match self.ask(socket, &["pane", "list", "--workspace", workspace]) {
            Call::Answered(panes) => panes,
            // A workspace herdr closed holds no pane.
            Call::Refused { code, .. } if code == "workspace_not_found" => return Ok(vec![]),
            Call::Refused { code, words } => {
                return Err(format!("herdr refused ({code}): {words}"))
            }
            Call::Unanswered(words) => return Err(words),
        };
        Ok(panes["panes"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|pane| within(pane))
            .filter_map(|pane| pane["pane_id"].as_str())
            .filter(|id| !opened.contains(*id))
            .map(str::to_string)
            .collect())
    }

    /// The records of the gone place's agents dropped, their panes gone
    /// with its tab.
    fn forget_place(&self, place: &PlaceRef) {
        for entry in fs::read_dir(&self.records)
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            let of_place = read_json(&path)
                .is_some_and(|record| record["place"].as_str() == Some(place.location.as_str()));
            if of_place {
                let _ = fs::remove_file(path);
            }
        }
    }

    /// The base's recorded workspace closed once it holds no tab of a
    /// live place and no pane flume did not open: what was closed, none
    /// where it stands for a live place or is gone, or what was kept and
    /// what holds it (137, 138).
    fn tidy_workspace(
        &self,
        place: &PlaceRef,
        opened: &BTreeSet<String>,
    ) -> Result<Option<String>, String> {
        let Some((workspace, socket)) = self.workspace(place) else {
            return Ok(None);
        };
        if let Some(kept) = self.held_workspace(place, opened) {
            return Err(kept);
        }
        let label = workspace_label(place);
        match self.live_tabs(&workspace, &socket)? {
            Some(true) => return Ok(None),
            Some(false) => {}
            None => {
                // herdr closes a workspace with its last tab.
                let _ = fs::remove_file(self.workspace_record(place));
                return Ok(Some(format!("the workspace {label}")));
            }
        }
        match self.ask(&socket, &["workspace", "close", &workspace]) {
            Call::Answered(_) => {
                let _ = fs::remove_file(self.workspace_record(place));
                Ok(Some(format!("the workspace {label}")))
            }
            Call::Refused { code, .. } if code == "workspace_not_found" => {
                let _ = fs::remove_file(self.workspace_record(place));
                Ok(None)
            }
            Call::Refused { code, words } => Err(format!(
                "the workspace {label} ({workspace}): herdr refused ({code}): {words}"
            )),
            Call::Unanswered(words) => Err(format!("the workspace {label} ({workspace}): {words}")),
        }
    }

    /// Whether the workspace holds a tab flume made for a place still
    /// recorded; none where herdr no longer lists the workspace.
    fn live_tabs(&self, workspace: &str, socket: &Path) -> Result<Option<bool>, String> {
        let tabs = match self.ask(socket, &["tab", "list", "--workspace", workspace]) {
            Call::Answered(tabs) => tabs,
            Call::Refused { code, .. } if code == "workspace_not_found" => return Ok(None),
            Call::Refused { code, words } => {
                return Err(format!("herdr refused ({code}): {words}"))
            }
            Call::Unanswered(words) => return Err(words),
        };
        let places = self.place_tabs();
        Ok(Some(
            tabs["tabs"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|tab| tab["tab_id"].as_str())
                .any(|tab| places.contains_key(tab)),
        ))
    }

    /// The base's recorded workspace named with what holds it, where it
    /// holds no tab of a live place and a pane flume did not open stands
    /// in it; none otherwise.
    fn held_workspace(&self, place: &PlaceRef, opened: &BTreeSet<String>) -> Option<String> {
        let (workspace, socket) = self.workspace(place)?;
        if self.live_tabs(&workspace, &socket).ok()?? {
            return None;
        }
        let foreign = self
            .foreign_panes(&socket, &workspace, opened, |_| true)
            .ok()?;
        (!foreign.is_empty()).then(|| {
            format!(
                "the workspace {} ({workspace}), held by the pane {} flume did not open",
                workspace_label(place),
                foreign.join(", ")
            )
        })
    }
}

/// The pane the calling command runs in, as herdr's environment names it.
pub fn caller() -> Option<Pane> {
    let id = std::env::var("HERDR_PANE_ID").ok()?;
    let socket = std::env::var("HERDR_SOCKET_PATH").ok()?;
    (!id.is_empty() && !socket.is_empty()).then(|| Pane {
        id,
        socket: socket.into(),
    })
}

/// The state directory herdr gives flume's herdr plugin, beside flume's
/// own state directory `flume_state`: `$XDG_STATE_HOME/herdr/plugins/
/// agentplot.flume` where flume's is `$XDG_STATE_HOME/flume` (design 10).
pub fn plugin_state(flume_state: &Path) -> Option<PathBuf> {
    flume_state
        .parent()
        .map(|state| state.join("herdr/plugins").join(PLUGIN))
}

/// flume's herdr plugin's id (`plugins/herdr/herdr-plugin.toml`).
pub const PLUGIN: &str = "agentplot.flume";

/// The agent's herdr name, made from the identity: at most 32 of the
/// characters herdr's rule `[a-z][a-z0-9_-]{0,31}` allows, ending in a
/// digest of the whole identity, so it is unique on the server (C.5;
/// record 0018).
pub fn herdr_name(identity: &Identity) -> String {
    let words: String = format!("f-{}-{}-{}", identity.place, identity.stage, identity.start)
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-' {
                c
            } else {
                '-'
            }
        })
        .take(23)
        .collect();
    format!("{words}-{}", &digest(&identity.to_string())[..8])
}

/// The variables of the login environment a pane is not given: the
/// shell's own of the shell that read it, and those the pane's own
/// shell and flume set.
const UNSET: [&str; 6] = ["PATH", "SHELL", "PWD", "OLDPWD", "SHLVL", "_"];

/// The pane's shell for a host user whose login shell is `login`: zsh
/// with `-f`, bash with `--noprofile --norc`, any other `/bin/sh`, each
/// reading no startup file, `ENV` and `BASH_ENV` unset, and `SHELL` the
/// user's own again for what runs in it (C.5).
pub fn pane_shell(login: &str) -> String {
    let name = Path::new(login)
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let exec = match name.as_str() {
        "zsh" => format!("{} -f", quoted(login)),
        "bash" => format!("{} --noprofile --norc", quoted(login)),
        _ => "/bin/sh -i".to_string(),
    };
    format!(
        "#!/bin/sh\n# flume's pane shell: no startup file is read, so PATH stays as flume set it.\nunset ENV BASH_ENV\nSHELL={}\nexport SHELL\nexec {exec}\n",
        quoted(login)
    )
}

/// `text` quoted for a POSIX shell.
fn quoted(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
}

/// The label of the workspace of a place's repository and base,
/// `<repository> <base>`, the repository by its directory's name.
pub fn workspace_label(place: &PlaceRef) -> String {
    let repository = Path::new(&place.repository).file_name().map_or_else(
        || place.repository.clone(),
        |name| name.to_string_lossy().into_owned(),
    );
    format!("{repository} {}", place.base)
}

/// The first twelve hex digits of `text`'s SHA-256.
fn digest(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .take(6)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn read_json(path: &Path) -> Option<Value> {
    serde_json::from_str(&fs::read_to_string(path).ok()?).ok()
}

fn write_json(path: &Path, value: &Value) -> Result<(), String> {
    path.parent()
        .map_or(Ok(()), fs::create_dir_all)
        .and_then(|()| fs::write(path, value.to_string()))
        .map_err(|error| format!("{}: {error}", path.display()))
}

/// The program the settings name, by its path: as named where it holds a
/// `/`, else the first on `path` (32).
fn program_path(program: &str, path: &str) -> Option<PathBuf> {
    if program.contains('/') {
        return Some(PathBuf::from(program));
    }
    path.split(':')
        .filter(|dir| !dir.is_empty())
        .map(|dir| Path::new(dir).join(program))
        .find(|candidate| candidate.is_file())
}

/// The file a process runs, from the host's process table; none where it
/// cannot be read.
#[cfg(target_os = "macos")]
fn executable(pid: u32) -> Option<PathBuf> {
    let mut buffer = vec![0u8; libc::PROC_PIDPATHINFO_MAXSIZE as usize];
    // SAFETY: the buffer is as long as `PROC_PIDPATHINFO_MAXSIZE`, as
    // `proc_pidpath` asks.
    let length = unsafe {
        libc::proc_pidpath(
            pid as libc::c_int,
            buffer.as_mut_ptr().cast(),
            buffer.len() as u32,
        )
    };
    (length > 0).then(|| {
        buffer.truncate(length as usize);
        PathBuf::from(String::from_utf8_lossy(&buffer).into_owned())
    })
}

/// The file a process runs, from the host's process table; none where it
/// cannot be read.
#[cfg(not(target_os = "macos"))]
fn executable(pid: u32) -> Option<PathBuf> {
    fs::read_link(format!("/proc/{pid}/exe")).ok()
}

fn field(value: &Value, name: &str) -> String {
    value
        .get(name)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

/// `args` as words a refusal may carry: each `--env NAME=value` given as
/// `NAME=<set>`, since a value may be a secret (32, 56).
fn unvalued(args: &[&str]) -> String {
    let mut words = Vec::with_capacity(args.len());
    let mut env = false;
    for arg in args {
        if env {
            let name = arg.split_once('=').map_or(*arg, |(name, _)| name);
            words.push(format!("{name}={}", flume_seams::SET));
        } else {
            words.push((*arg).to_string());
        }
        env = *arg == "--env";
    }
    words.join(" ")
}

/// What came of the folder-trust question at a start.
enum Trusted {
    /// The screen held still without the question.
    NotAsked(String),
    /// The question was answered yes and is gone from the screen.
    Answered,
    /// The question still stands, with the screen.
    Unanswered(String),
}

/// Whether `screen` shows the folder-trust question: its yes choice.
fn asks(screen: &str, trust: &FolderTrust) -> bool {
    screen.lines().any(|line| line.contains(&trust.question))
}

/// Whether the yes choice has the focus on `screen`, or none where the
/// question is not shown.
fn focused(screen: &str, trust: &FolderTrust) -> Option<bool> {
    screen
        .lines()
        .find(|line| line.contains(&trust.question))
        .map(|line| !trust.focus.is_empty() && line.contains(&trust.focus))
}

/// The pane id under `key` of a herdr result.
fn pane_id(result: &Value, key: &str) -> Result<String, String> {
    result
        .get(key)
        .and_then(|pane| pane.get("pane_id"))
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("herdr gave no {key} in {result}"))
}

impl Terminal for Herdr {
    fn open(
        &self,
        place: &PlaceRef,
        folder: &str,
        command: &AgentCommand,
        identity: &Identity,
        beside: Option<&str>,
        session: Option<&str>,
    ) -> Opened {
        if self.state(identity) != AgentState::Stopped {
            return Opened::Refused {
                reason: format!("an agent is live under {identity}"),
            };
        }
        if let Some(name) = session {
            if let Some(holder) = self.named(name) {
                if holder != *identity && self.state(&holder) != AgentState::Stopped {
                    return Opened::Refused {
                        reason: format!("the live agent {holder} carries the session name {name}"),
                    };
                }
            }
        }
        let Some(kind) = command.kind.as_deref() else {
            return Opened::Refused {
                reason: format!(
                    "herdr starts a known agent program by its kind, and {} names none",
                    command.program
                ),
            };
        };
        if !KINDS.contains(&kind) {
            return Opened::Refused {
                reason: format!(
                    "herdr knows no kind {kind}, so {} cannot run under herdr; its kinds are {}",
                    command.program,
                    KINDS.join(", ")
                ),
            };
        }
        let path = command
            .env
            .get("PATH")
            .cloned()
            .or_else(|| std::env::var("PATH").ok())
            .unwrap_or_default();
        let Some(program) = program_path(&command.program, &path) else {
            return Opened::Refused {
                reason: format!("the program {} is on no folder of PATH", command.program),
            };
        };
        // A folder named by the identity's digest, since an identity's
        // `:` would split `PATH`.
        let start = Path::new(folder)
            .join("herdr")
            .join(digest(&identity.to_string()));
        let bin = start.join("bin");
        let link = bin.join(kind);
        let linked = fs::create_dir_all(&bin)
            .and_then(|()| match fs::remove_file(&link) {
                Err(error) if error.kind() != std::io::ErrorKind::NotFound => Err(error),
                _ => Ok(()),
            })
            .and_then(|()| std::os::unix::fs::symlink(&program, &link));
        if let Err(error) = linked {
            return Opened::Refused {
                reason: format!("{}: {error}", link.display()),
            };
        }
        let made = match self.made(place, command, &start) {
            Ok(made) => made,
            Err(reason) => return Opened::Refused { reason },
        };
        let pane = match self.new_pane(place, &identity.place, beside, &made) {
            Ok(pane) => pane,
            Err(reason) => return Opened::Refused { reason },
        };
        let kept = Kept {
            pane: pane.clone(),
            name: herdr_name(identity),
            place: beside.is_none().then(|| place.location.clone()),
        };
        if let Err(reason) = self.keep(identity, &kept) {
            self.abandon(identity, &pane, &bin);
            return Opened::Refused { reason };
        }
        let timeout = self.start_timeout.as_millis().to_string();
        let mut args: Vec<&str> = vec![
            "agent",
            "start",
            &kept.name,
            "--kind",
            kind,
            "--pane",
            &pane.id,
            "--timeout",
            &timeout,
            "--",
        ];
        args.extend(command.args.iter().map(String::as_str));
        let started = self.call(&pane.socket, &args, self.start_timeout + PAST_TIMEOUT);
        // herdr's answer is one signal and the screen another: the trust
        // question is read off the screen and answered whatever herdr
        // answered, since herdr may read the dialog ready, blocked or
        // neither, and may answer before it is drawn (34; record 0018).
        let trust = command
            .folder_trust
            .as_ref()
            .filter(|trust| !trust.question.is_empty());
        match started {
            Call::Answered(_) => {
                if let Some(trust) = trust {
                    if let Trusted::Unanswered(screen) = self.trust(&pane, trust, false) {
                        return Opened::Held { screen };
                    }
                }
            }
            Call::Refused { code, .. } if code == "agent_not_ready" => {
                let Some(trust) = trust else {
                    return Opened::Held {
                        screen: self.visible(&pane),
                    };
                };
                match self.trust(&pane, trust, true) {
                    Trusted::Answered => {}
                    Trusted::NotAsked(screen) | Trusted::Unanswered(screen) => {
                        return Opened::Held { screen }
                    }
                }
                let wait = TRUST_WAIT.as_millis().to_string();
                let waited = self.call(
                    &pane.socket,
                    &[
                        "agent",
                        "wait",
                        &kept.name,
                        "--until",
                        "idle",
                        "--timeout",
                        &wait,
                    ],
                    TRUST_WAIT + PAST_TIMEOUT,
                );
                if !matches!(waited, Call::Answered(_)) {
                    return Opened::Held {
                        screen: self.visible(&pane),
                    };
                }
            }
            Call::Refused { code, .. } if code == "timeout" => {
                let answered = trust.map(|trust| self.trust(&pane, trust, false));
                match answered {
                    Some(Trusted::Answered) => {}
                    Some(Trusted::Unanswered(screen)) => return Opened::Held { screen },
                    Some(Trusted::NotAsked(_)) | None => {
                        return Opened::NotComeUp {
                            screen: self.visible(&pane),
                        }
                    }
                }
            }
            Call::Refused { code, words } => {
                self.abandon(identity, &pane, &bin);
                let reason = if code == "agent_name_taken" {
                    format!("an agent is live under {identity}: herdr refused ({code}): {words}")
                } else {
                    format!("herdr refused ({code}): {words}")
                };
                return Opened::Refused { reason };
            }
            Call::Unanswered(words) => {
                self.abandon(identity, &pane, &bin);
                return Opened::Refused { reason: words };
            }
        }
        if let Err(reason) = self.runs(&pane, &program) {
            self.abandon(identity, &pane, &bin);
            return Opened::Refused { reason };
        }
        if let Some(name) = session {
            let named = Named {
                session: name.into(),
                identity: identity.clone(),
            };
            let written = serde_json::to_value(&named)
                .map_err(|error| error.to_string())
                .and_then(|value| write_json(&self.session_record(name), &value));
            if let Err(reason) = written {
                self.abandon(identity, &pane, &bin);
                return Opened::Refused { reason };
            }
        }
        Opened::Opened {
            carried: false,
            printed: None,
        }
    }

    fn send(&self, agent: &Identity, text: &str) -> Sent {
        let pane = match self.found(agent) {
            Found::At(pane) => pane,
            Found::Gone => {
                return Sent::Refused {
                    reason: format!("no agent runs under {agent}"),
                }
            }
            Found::Unknown(reason) => return Sent::Refused { reason },
        };
        for args in [
            vec!["agent", "send-keys", pane.id.as_str(), "ctrl+c"],
            vec!["agent", "prompt", pane.id.as_str(), text],
        ] {
            if let Err(reason) = self.answered(&pane.socket, &args) {
                return Sent::Refused { reason };
            }
        }
        Sent::Sent
    }

    fn state(&self, agent: &Identity) -> AgentState {
        match self.found(agent) {
            Found::At(pane) => self.pane_state(&pane),
            Found::Gone => AgentState::Stopped,
            Found::Unknown(_) => AgentState::Unknown,
        }
    }

    fn send_to(&self, session: &str, text: &str) -> ToSession {
        let pane = match self.reach(session) {
            Found::At(pane) => pane,
            Found::Gone => return ToSession::NotFound,
            Found::Unknown(reason) => return ToSession::Refused { reason },
        };
        let mut calls = vec![];
        match self.pane_state(&pane) {
            AgentState::Waiting => {
                calls.push(vec!["agent", "send-keys", pane.id.as_str(), "ctrl+c"])
            }
            AgentState::Working => {}
            AgentState::Stopped => return ToSession::NotFound,
            state => {
                return ToSession::Refused {
                    reason: format!(
                        "the session {session} reads {state:?}, so nothing was typed into it"
                    ),
                }
            }
        }
        calls.push(vec!["agent", "prompt", pane.id.as_str(), text]);
        for args in calls {
            if let Err(reason) = self.answered(&pane.socket, &args) {
                return ToSession::Refused { reason };
            }
        }
        ToSession::Sent
    }

    fn session_state(&self, session: &str) -> Option<AgentState> {
        match self.reach(session) {
            Found::At(pane) => match self.pane_state(&pane) {
                AgentState::Stopped => None,
                state => Some(state),
            },
            Found::Gone => None,
            Found::Unknown(_) => Some(AgentState::Unknown),
        }
    }

    fn screen(&self, agent: &Identity) -> Option<String> {
        let Found::At(pane) = self.found(agent) else {
            return None;
        };
        match self.ask(
            &pane.socket,
            &[
                "pane", "read", &pane.id, "--source", "visible", "--format", "text",
            ],
        ) {
            Call::Answered(Value::String(text)) => Some(text),
            _ => None,
        }
    }

    fn close(&self, agent: &Identity) -> Closed {
        let pane = match self.found(agent) {
            Found::At(pane) => pane,
            Found::Gone => {
                let _ = fs::remove_file(self.agent_record(agent));
                return Closed::Closed;
            }
            Found::Unknown(reason) => return Closed::Refused { reason },
        };
        match self.ask(&pane.socket, &["pane", "close", &pane.id]) {
            Call::Answered(_) => {}
            Call::Refused { code, .. } if code == "pane_not_found" => {}
            Call::Refused { code, words } => {
                return Closed::Refused {
                    reason: format!("herdr refused ({code}): {words}"),
                }
            }
            Call::Unanswered(words) => return Closed::Refused { reason: words },
        }
        match self.ask(&pane.socket, &["pane", "get", &pane.id]) {
            Call::Refused { code, .. } if code == "pane_not_found" => {
                let _ = fs::remove_file(self.agent_record(agent));
                Closed::Closed
            }
            Call::Unanswered(words) => Closed::Refused { reason: words },
            _ => Closed::Refused {
                reason: format!("herdr's pane {} still stands", pane.id),
            },
        }
    }

    fn tidy(&self, place: &PlaceRef) -> TerminalTidy {
        let mut tidy = TerminalTidy::default();
        let opened = self.opened_panes();
        if let Some((tab, socket)) = self.tab(place) {
            let record = read_json(&self.tab_record(&place.location)).unwrap_or_default();
            let label = record["label"].as_str().unwrap_or(&tab).to_string();
            let workspace = record["workspace"].as_str().unwrap_or_default().to_string();
            match self.foreign_panes(&socket, &workspace, &opened, |pane| {
                pane["tab_id"].as_str() == Some(tab.as_str())
            }) {
                Err(reason) => tidy.kept.push(format!(
                    "the tab {label} ({tab}), which herdr could not be read for: {reason}"
                )),
                Ok(foreign) if !foreign.is_empty() => tidy.kept.push(format!(
                    "the tab {label} ({tab}), held by the pane {} flume did not open",
                    foreign.join(", ")
                )),
                Ok(_) => match self.ask(&socket, &["tab", "close", &tab]) {
                    Call::Answered(_) => {
                        tidy.closed.push(format!("the tab {label}"));
                        let _ = fs::remove_file(self.tab_record(&place.location));
                    }
                    Call::Refused { code, .. } if code == "tab_not_found" => {
                        let _ = fs::remove_file(self.tab_record(&place.location));
                    }
                    Call::Refused { code, words } => tidy.kept.push(format!(
                        "the tab {label} ({tab}): herdr refused ({code}): {words}"
                    )),
                    Call::Unanswered(words) => {
                        tidy.kept.push(format!("the tab {label} ({tab}): {words}"))
                    }
                },
            }
        }
        self.forget_place(place);
        match self.tidy_workspace(place, &opened) {
            Ok(Some(closed)) => tidy.closed.push(closed),
            Ok(None) => {}
            Err(kept) => tidy.kept.push(kept),
        }
        tidy
    }

    fn kept(&self) -> Vec<String> {
        let opened = self.opened_panes();
        fs::read_dir(self.records.join("workspaces"))
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter_map(|entry| read_json(&entry.path()))
            .filter_map(|record| {
                let place = PlaceRef {
                    repository: record["repository"].as_str()?.into(),
                    base: record["base"].as_str()?.into(),
                    location: String::new(),
                    branch: String::new(),
                };
                self.held_workspace(&place, &opened)
            })
            .collect()
    }

    fn show(&self, place: &PlaceRef, clues: &Clues) -> Shown {
        let mut failures = vec![];
        let mut shown = false;
        if let Some(agent) = &clues.agent {
            if let Found::At(pane) = self.found(agent) {
                let title = format!("{} · {}", clues.stage, clues.state);
                match self.answered(
                    &pane.socket,
                    &[
                        "pane",
                        "report-metadata",
                        &pane.id,
                        "--source",
                        "flume",
                        "--title",
                        &title,
                        "--display-agent",
                        &agent.place,
                    ],
                ) {
                    Ok(_) => shown = true,
                    Err(reason) => failures.push(format!("the pane {}: {reason}", pane.id)),
                }
            }
        }
        if let Some((workspace, socket)) = self.workspace(place) {
            let token = format!("flume={}", clues.base);
            match self.answered(
                &socket,
                &[
                    "workspace",
                    "report-metadata",
                    &workspace,
                    "--source",
                    "flume",
                    "--token",
                    &token,
                ],
            ) {
                Ok(_) => shown = true,
                Err(reason) => failures.push(format!("the workspace {workspace}: {reason}")),
            }
        }
        if !failures.is_empty() {
            Shown::Failed {
                reason: failures.join("; "),
            }
        } else if shown {
            Shown::Shown
        } else {
            Shown::ShowsNone
        }
    }

    fn shown_at(&self, place: &PlaceRef, agent: Option<&Identity>) -> ShownAt {
        let named = |id: &str, socket: &Path| format!("herdr:{id}@{}", socket.display());
        ShownAt {
            place: self.tab(place).map(|(tab, socket)| named(&tab, &socket)),
            agent: agent
                .and_then(|agent| self.pane(agent))
                .map(|pane| pane.session()),
            base: self
                .workspace(place)
                .map(|(workspace, socket)| named(&workspace, &socket)),
        }
    }

    fn watch(&self) -> Watch {
        if self.watch_kept() {
            Watch::Bound {
                seconds: WATCH_BOUND,
            }
        } else {
            Watch::None
        }
    }

    fn session(&self) -> Option<String> {
        self.caller.as_ref().map(Pane::session)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The profile over records in a scratch directory of its own.
    fn over_records(name: &str) -> (Herdr, PathBuf) {
        let records =
            std::env::temp_dir().join(format!("flume-herdr-records-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&records);
        (
            Herdr::new(Server::Socket("/run/h.sock".into()), &records),
            records,
        )
    }

    #[test]
    fn a_refusals_words_name_each_variable_and_hold_no_value() {
        let words = unvalued(&[
            "pane",
            "split",
            "w1:p1",
            "--env",
            "CLAUDE_CODE_OAUTH_TOKEN=sk-ant-oat01-secret",
            "--env",
            "PATH=/bin",
            "--no-focus",
        ]);
        assert_eq!(
            words,
            "pane split w1:p1 --env CLAUDE_CODE_OAUTH_TOKEN=<set> --env PATH=<set> --no-focus"
        );
    }

    fn place() -> PlaceRef {
        PlaceRef {
            repository: "/r".into(),
            location: "/r/places/fix--one".into(),
            branch: "fix/one".into(),
            base: "main".into(),
        }
    }

    fn identity() -> Identity {
        Identity {
            repository: "r1".into(),
            place: "fix--one".into(),
            stage: "code".into(),
            start: 1,
        }
    }

    #[test]
    fn where_the_clues_show_is_read_from_the_records_alone() {
        // 139, C.5, design 10: the place's tab, its agent's pane and its
        // base's workspace, each as `herdr:<id>@<socket>` from what open
        // recorded, asking herdr nothing (the socket answers no server).
        let (herdr, records) = over_records("shown");
        let place = place();
        write_json(
            &herdr.tab_record(&place.location),
            &json!({"tab": "w1:t2", "socket": "/run/h.sock"}),
        )
        .unwrap();
        write_json(
            &herdr.workspace_record(&place),
            &json!({"workspace": "w1", "socket": "/run/h.sock"}),
        )
        .unwrap();
        herdr
            .keep(
                &identity(),
                &Kept {
                    pane: Pane {
                        id: "w1:p3".into(),
                        socket: "/run/h.sock".into(),
                    },
                    name: herdr_name(&identity()),
                    place: Some(place.location.clone()),
                },
            )
            .unwrap();
        assert_eq!(
            herdr.shown_at(&place, Some(&identity())),
            ShownAt {
                place: Some("herdr:w1:t2@/run/h.sock".into()),
                agent: Some("herdr:w1:p3@/run/h.sock".into()),
                base: Some("herdr:w1@/run/h.sock".into()),
            }
        );
        let _ = fs::remove_dir_all(records);
    }

    #[test]
    fn a_place_open_never_made_shows_nowhere() {
        let (herdr, _) = over_records("none");
        assert_eq!(
            herdr.shown_at(&place(), Some(&identity())),
            ShownAt::default()
        );
    }
}

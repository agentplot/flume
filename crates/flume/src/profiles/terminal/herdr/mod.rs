//! The terminal profile over herdr (C.5; design 12; record 0016), every
//! command, state name, error code and field it reads checked against
//! the herdr the tree pins, 0.9.5 (core.29).
//!
//! open makes a pane: a split beside the pane the settings name, by its
//! label or id on flume's server or as `herdr:<pane>@<socket>`; otherwise
//! a tab named `<kind>--<name>` in the workspace `flume` of flume's own
//! herdr session, a split in it where the place's tab stands. The pane is
//! held by its id and its server's socket, recorded by the agent's
//! identity; the agent gets a herdr name made from the identity for
//! display alone, never addressed by it, since herdr drops a name when
//! the pane's session changes (94, 107). What is typed into the pane's
//! shell is one line running a launcher file in the place's own folder,
//! since herdr refuses a line past the terminal's limit and an agent's
//! instructions pass it; open then reads the pane until the agent waits
//! or shows its folder-trust question, answered with the keys its
//! command names (34). send clears what the agent holds typed with one
//! ctrl+c, then hands the text to herdr's agent prompt (95). state reads
//! herdr's working as working, idle and done as waiting, blocked as
//! asking, which the core takes as asking only with the agent's mark
//! (90), and herdr's unknown as unknown; stopped only when the pane is
//! gone, herdr lists it with a `restore_error`, or herdr lists no agent
//! in it and its foreground is the shell (96). screen is the pane's
//! visible lines (136); close ends the pane and reads that it is gone
//! (97); there is no watch (122); and the session a command runs in is
//! `herdr:<pane>@<socket>`, its pane on its server (7).
//!
//! Every herdr call runs under a bound, ten seconds where the call states
//! none, and one that does not answer within it is unknown or a refusal
//! naming the bound; each runs with the caller's `HERDR_*` stripped and
//! the recorded server's socket passed (98, core.29).

use crate::shell;
use flume_seams::{
    AgentCommand, AgentState, Closed, Identity, Opened, PlaceRef, Sent, Terminal, Watch,
};
use serde_json::{json, Value};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// The profile's name in the host's configuration, `[host] terminal`
/// (105).
pub const NAME: &str = "herdr";

/// The bound on a herdr call that states none (98).
pub const BOUND: Duration = Duration::from_secs(10);

/// How long open reads a new pane for the agent's first wait or its
/// folder-trust question: herdr's own bound on an agent's start.
pub const OPEN_BOUND: Duration = Duration::from_secs(30);

/// The herdr session flume opens its places' panes in, where the settings
/// name no pane.
pub const SESSION: &str = "flume";

/// The workspace of flume's session that holds one tab per place.
pub const WORKSPACE: &str = "flume";

/// How often a reading of herdr is taken again while open waits.
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
    open_bound: Duration,
    caller: Option<Pane>,
}

impl Herdr {
    /// The host's herdr: flume's own session, the panes recorded under
    /// `$XDG_STATE_HOME/flume/terminal/herdr`, and the pane the calling
    /// command runs in, as herdr's environment names it.
    pub fn of_host() -> Result<Herdr, String> {
        Ok(Herdr::new(
            Server::Session(SESSION.into()),
            crate::host::config::flume_state()?.join("terminal/herdr"),
        )
        .with_caller(caller()))
    }

    /// herdr opening on `server`, recording each agent's pane under
    /// `records`; the calling command in no pane.
    pub fn new(server: Server, records: impl Into<PathBuf>) -> Herdr {
        Herdr {
            program: "herdr".into(),
            server,
            records: records.into(),
            bound: BOUND,
            open_bound: OPEN_BOUND,
            caller: None,
        }
    }

    /// The bound on each call, in place of ten seconds.
    pub fn with_bound(mut self, bound: Duration) -> Herdr {
        self.bound = bound;
        self
    }

    /// The pane the calling command runs in, if any.
    pub fn with_caller(mut self, caller: Option<Pane>) -> Herdr {
        self.caller = caller;
        self
    }

    /// The pane the agent under `identity` runs in, as open recorded it.
    pub fn pane(&self, identity: &Identity) -> Option<Pane> {
        let text = fs::read_to_string(self.record(identity)).ok()?;
        let record: Value = serde_json::from_str(&text).ok()?;
        Some(Pane {
            id: record.get("pane")?.as_str()?.into(),
            socket: record.get("socket")?.as_str()?.into(),
        })
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
                words: format!("herdr {}: {}", args.join(" "), stderr.trim()),
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

    fn record(&self, identity: &Identity) -> PathBuf {
        self.records
            .join(format!("{}.json", identity.to_string().replace('/', "%")))
    }

    fn keep(&self, identity: &Identity, pane: &Pane) -> Result<(), String> {
        let record = json!({ "pane": pane.id, "socket": pane.socket });
        fs::create_dir_all(&self.records)
            .and_then(|()| fs::write(self.record(identity), record.to_string()))
            .map_err(|error| format!("{}: {error}", self.records.display()))
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

    /// A new pane for the agent in the place: a split beside the pane
    /// named, or the place's tab in flume's workspace (94, 107).
    fn new_pane(
        &self,
        place: &PlaceRef,
        short: &str,
        beside: Option<&str>,
    ) -> Result<Pane, String> {
        let split = |pane: &Pane| -> Result<Pane, String> {
            let split = self.answered(
                &pane.socket,
                &[
                    "pane",
                    "split",
                    &pane.id,
                    "--direction",
                    "right",
                    "--cwd",
                    &place.location,
                    "--no-focus",
                ],
            )?;
            pane_id(&split, "pane").map(|id| Pane {
                id,
                socket: pane.socket.clone(),
            })
        };
        if let Some(beside) = beside {
            return split(&self.named_pane(beside)?);
        }
        let socket = self.socket()?;
        let workspaces = self.answered(&socket, &["workspace", "list"])?;
        let workspace = workspaces
            .get("workspaces")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find(|workspace| workspace.get("label").and_then(Value::as_str) == Some(WORKSPACE))
            .and_then(|workspace| workspace.get("workspace_id").and_then(Value::as_str))
            .map(str::to_string);
        let Some(workspace) = workspace else {
            let made = self.answered(
                &socket,
                &[
                    "workspace",
                    "create",
                    "--cwd",
                    &place.location,
                    "--label",
                    WORKSPACE,
                    "--no-focus",
                ],
            )?;
            if let Some(tab) = made
                .get("tab")
                .and_then(|tab| tab.get("tab_id"))
                .and_then(Value::as_str)
            {
                self.answered(&socket, &["tab", "rename", tab, short])?;
            }
            return pane_id(&made, "root_pane").map(|id| Pane { id, socket });
        };
        let panes = self.answered(&socket, &["pane", "list", "--workspace", &workspace])?;
        let tabs = self.answered(&socket, &["tab", "list", "--workspace", &workspace])?;
        let tab = tabs
            .get("tabs")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find(|tab| tab.get("label").and_then(Value::as_str) == Some(short))
            .and_then(|tab| tab.get("tab_id").and_then(Value::as_str));
        let in_tab = tab.and_then(|tab| {
            panes
                .get("panes")?
                .as_array()?
                .iter()
                .find(|pane| pane.get("tab_id").and_then(Value::as_str) == Some(tab))?
                .get("pane_id")?
                .as_str()
                .map(str::to_string)
        });
        if let Some(id) = in_tab {
            return split(&Pane {
                id,
                socket: socket.clone(),
            });
        }
        let made = self.answered(
            &socket,
            &[
                "tab",
                "create",
                "--workspace",
                &workspace,
                "--cwd",
                &place.location,
                "--label",
                short,
                "--no-focus",
            ],
        )?;
        pane_id(&made, "root_pane").map(|id| Pane { id, socket })
    }

    /// Reads the new pane until its agent waits, or answers the agent's
    /// folder-trust question once it shows, giving the agent its display
    /// name once herdr sees it; for up to the open's bound (34).
    fn come_up(&self, pane: &Pane, command: &AgentCommand, identity: &Identity) {
        let deadline = Instant::now() + self.open_bound;
        let mut named = false;
        let mut trusted = command.folder_trust.is_none();
        while Instant::now() < deadline {
            let Call::Answered(info) = self.ask(&pane.socket, &["pane", "get", &pane.id]) else {
                return;
            };
            let agent = info.get("pane").and_then(|pane| pane.get("agent"));
            if agent.is_some_and(|agent| !agent.is_null()) && !named {
                named = true;
                let _ = self.ask(
                    &pane.socket,
                    &["agent", "rename", &pane.id, &display_name(identity)],
                );
            }
            if let Some(trust) = command.folder_trust.as_ref().filter(|_| !trusted) {
                let visible = self.ask(
                    &pane.socket,
                    &[
                        "pane", "read", &pane.id, "--source", "visible", "--format", "text",
                    ],
                );
                if matches!(&visible, Call::Answered(Value::String(text)) if text.contains(&trust.question))
                {
                    let mut keys = vec!["pane", "send-keys", pane.id.as_str()];
                    keys.extend(trust.answer.split_whitespace());
                    let _ = self.ask(&pane.socket, &keys);
                    trusted = true;
                }
            }
            let status = info
                .get("pane")
                .and_then(|pane| pane.get("agent_status"))
                .and_then(Value::as_str);
            if named && trusted && matches!(status, Some("idle" | "done")) {
                return;
            }
            std::thread::sleep(POLL);
        }
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

/// A herdr name for display made from the identity: at most 32 of the
/// characters herdr's rule `[a-z][a-z0-9_-]{0,31}` allows, never the
/// agent's address (C.5).
pub fn display_name(identity: &Identity) -> String {
    let words = format!("f-{}-{}-{}", identity.place, identity.stage, identity.start);
    words
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-' {
                c
            } else {
                '-'
            }
        })
        .take(32)
        .collect()
}

/// The launcher file that starts the agent: into the place, the command's
/// environment over the pane's, and the program with its arguments (88).
pub fn launcher(place: &PlaceRef, command: &AgentCommand, identity: &Identity) -> String {
    let mut text = format!(
        "#!/bin/sh\n# {identity}, started by flume's herdr profile.\ncd {} || exit 1\n",
        shell::quote(&place.location)
    );
    for (name, value) in &command.env {
        text.push_str(&format!("export {name}={}\n", shell::quote(value)));
    }
    text.push_str("exec ");
    text.push_str(&shell::words(
        std::iter::once(command.program.as_str()).chain(command.args.iter().map(String::as_str)),
    ));
    text.push('\n');
    text
}

fn field(value: &Value, name: &str) -> String {
    value
        .get(name)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
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
    ) -> Opened {
        if self.state(identity) != AgentState::Stopped {
            return Opened::Refused {
                reason: format!("an agent is live under {identity}"),
            };
        }
        let file = Path::new(folder)
            .join("herdr")
            .join(format!("{}.sh", identity.to_string().replace('/', "%")));
        let written = file
            .parent()
            .map_or(Ok(()), fs::create_dir_all)
            .and_then(|()| fs::write(&file, launcher(place, command, identity)));
        if let Err(error) = written {
            return Opened::Refused {
                reason: format!("{}: {error}", file.display()),
            };
        }
        let pane = match self.new_pane(place, &identity.place, beside) {
            Ok(pane) => pane,
            Err(reason) => return Opened::Refused { reason },
        };
        if let Err(reason) = self.keep(identity, &pane) {
            let _ = self.ask(&pane.socket, &["pane", "close", &pane.id]);
            return Opened::Refused { reason };
        }
        let line = format!("sh {}", shell::quote(&file.to_string_lossy()));
        if let Err(reason) = self.answered(&pane.socket, &["pane", "run", &pane.id, &line]) {
            let _ = self.ask(&pane.socket, &["pane", "close", &pane.id]);
            return Opened::Refused { reason };
        }
        self.come_up(&pane, command, identity);
        Opened::Opened {
            carried: false,
            printed: None,
        }
    }

    fn send(&self, agent: &Identity, text: &str) -> Sent {
        let Some(pane) = self.pane(agent) else {
            return Sent::Refused {
                reason: format!("no agent runs under {agent}"),
            };
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
        let Some(pane) = self.pane(agent) else {
            return AgentState::Stopped;
        };
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

    fn screen(&self, agent: &Identity) -> Option<String> {
        let pane = self.pane(agent)?;
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
        let Some(pane) = self.pane(agent) else {
            return Closed::Closed;
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
                let _ = fs::remove_file(self.record(agent));
                Closed::Closed
            }
            Call::Unanswered(words) => Closed::Refused { reason: words },
            _ => Closed::Refused {
                reason: format!("herdr's pane {} still stands", pane.id),
            },
        }
    }

    fn watch(&self) -> Watch {
        Watch::None
    }

    fn session(&self) -> Option<String> {
        self.caller.as_ref().map(Pane::session)
    }
}

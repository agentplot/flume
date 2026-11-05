//! The terminal profile over herdr (C.5; design 12; records 0016, 0018),
//! every command, state name, error code and field it reads checked
//! against the herdr the tree pins, 0.9.5 (core.29).
//!
//! open makes a pane in the place with the agent's environment set on
//! it, the host user's login environment under it, and a shell of the
//! start's own that reads no startup file, so nothing the host's shell
//! files do reorders the pane's `PATH`: a split beside the pane the settings name, by its label or id on
//! flume's server or as `herdr:<pane>@<socket>`; otherwise, in the herdr
//! session the core names, or flume's own where it names the terminal's
//! own, the session's server found by `herdr session list` and started
//! headless where none runs, a name the pinned herdr refuses refusing the
//! open in herdr's words, the place's tab `<kind>--<name>` in that
//! session's workspace of the place's repository and base, labelled
//! `<repository> <base>`, each made when first needed and recorded by id
//! and server, a record written before records were kept by server read
//! as the record of the server it names (107, 138; record 0031). The pane's
//! `PATH` is led by a folder of the start's own linking the program's
//! kind to the program the settings name, since herdr's agent start
//! types the kind's name into the pane's shell; once the pane's screen
//! shows the shell's first output, its prompt, agent start runs it with
//! the command's arguments whole, under the agent's herdr name, and
//! nothing is typed (94, 107). Until the shell's line editor takes the
//! pane's terminal, herdr refuses a line past the terminal's canonical
//! limit, `agent_start_input_failed`: that is the shell not yet at its
//! prompt, and agent start is asked again within the start's bound, a
//! start still so answered at the bound not come up with the screen,
//! never refused. Whatever agent start answers, the
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
//! tab in every session it made one in, and its base's workspace in each
//! once no tab of a live place and no pane flume did not open is in it,
//! keeping and naming either where a pane flume did not open stands
//! (137, 138). show titles the agent's pane with its stage and state and
//! gives each of the base's workspaces, in every session, its count as
//! the token `flume` (139), and says where it shows them, the place's
//! tab and its base's workspace on its live agent's server, its agent's
//! pane, and each of the base's workspaces, each as `herdr:<id>@<socket>`
//! from its records, for flume's herdr plugin to refresh them (C.5). The watch is flume's herdr plugin's: while a
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

use crate::host::process::{executable, program_path};
use flume_seams::{
    AgentCommand, AgentState, Attached, Closed, Clues, FolderTrust, Identity, OpenIn, Opened,
    PlaceRef, Sent, Shown, ShownAt, Terminal, TerminalTidy, ToSession, Watch,
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

/// How long a start waits for the pane's shell to come to its prompt and
/// the program to be ready: room for a shell and SessionStart hooks slow
/// under load (record 0018).
pub const START_TIMEOUT: Duration = Duration::from_secs(120);

/// herdr's shortest bound on an agent start: its settle delay, three
/// seconds, and a millisecond more.
const SHORTEST_START: Duration = Duration::from_millis(3001);

/// herdr's word that the pane's terminal cannot yet take the start's
/// line: on macOS, a line past the terminal's canonical limit while it is
/// canonical, as it is until the shell's line editor takes it at its
/// prompt; nothing is typed (herdr 0.9.5's `validate_pty_submission`).
const NOT_YET_TAKEN: &str = "agent_start_input_failed";

/// How long apart agent start is asked again while herdr answers that
/// the pane's terminal cannot yet take its line.
const TAKEN_BEAT: Duration = Duration::from_millis(250);

/// How long, once the folder-trust question is answered, herdr is waited
/// on for the agent to idle: herdr's own default for a start.
pub const TRUST_WAIT: Duration = Duration::from_secs(30);

/// How long apart the screen is read while the folder-trust question is
/// answered.
const TRUST_BEAT: Duration = Duration::from_millis(250);

/// How long the screen is read for the folder-trust question to be drawn
/// or the program's screen to hold still without it.
const TRUST_SETTLE: Duration = Duration::from_secs(10);

/// How long a program's screen holds still without the folder-trust
/// question before it is read as not asking.
const TRUST_STILL: Duration = Duration::from_secs(1);

/// How long a screen still as it was when herdr answered is given for the
/// program to draw before it is read as not asking.
const TRUST_DRAW: Duration = Duration::from_secs(3);

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
/// name no pane and the core names no session of its own.
pub const SESSION: &str = "flume";

/// The variable the pinned herdr, 0.9.5, reads its named sessions' home
/// from, `$XDG_CONFIG_HOME/herdr/sessions/<name>`, `~/.config` where it is
/// unset; a test's profile sets it to a scratch home (core.29).
pub const SESSION_HOME: &str = "XDG_CONFIG_HOME";

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
    session_home: Option<PathBuf>,
}

/// A workspace or a tab open made, as its record names it: its id and
/// its server's socket, the record whole, and the file it is kept in.
#[derive(Clone, Debug)]
struct Record {
    id: String,
    socket: PathBuf,
    value: Value,
    path: PathBuf,
}

impl Record {
    /// The workspace or tab as flume names it to a plugin,
    /// `herdr:<id>@<socket>` (C.5).
    fn named(&self) -> String {
        format!("herdr:{}@{}", self.id, self.socket.display())
    }

    /// Its label as recorded, else its id.
    fn label(&self) -> &str {
        self.value["label"].as_str().unwrap_or(&self.id)
    }
}

/// A pane as open recorded it for an agent: its id, its server, the
/// agent's herdr name and the place it was opened for.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Kept {
    pane: Pane,
    name: String,
    place: Option<String>,
    /// The folder-trust question the open was given, which a settle
    /// answers where the program asks it after the open (34).
    trust: Option<FolderTrust>,
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
            session_home: None,
        }
    }

    /// herdr's named sessions placed under `home` rather than the host
    /// user's: every herdr call, and every session's server the profile
    /// starts, run with `home` as [`SESSION_HOME`], which the pinned herdr
    /// reads its sessions from; for tests, which reach no session of the
    /// host's (core.29).
    pub fn with_session_home(mut self, home: impl Into<PathBuf>) -> Herdr {
        self.session_home = Some(home.into());
        self
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

    /// How long a start waits for the pane's shell and the program, in
    /// place of 120 seconds; herdr's agent start takes 3001 milliseconds
    /// to 300 seconds.
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

    /// The workspace last recorded for the place's repository and base,
    /// on any server, by id and server.
    pub fn workspace(&self, place: &PlaceRef) -> Option<(String, PathBuf)> {
        self.workspaces(place).pop()
    }

    /// Each workspace recorded for the place's repository and base, one
    /// on each server, by id and server, the last made last.
    pub fn workspaces(&self, place: &PlaceRef) -> Vec<(String, PathBuf)> {
        self.base_records(place)
            .into_iter()
            .map(|record| (record.id, record.socket))
            .collect()
    }

    /// The place's tab last recorded, on any server, by id and server.
    pub fn tab(&self, place: &PlaceRef) -> Option<(String, PathBuf)> {
        self.tabs(place).pop()
    }

    /// Each of the place's tabs recorded, one on each server its agents
    /// opened on, by id and server, the last made last.
    pub fn tabs(&self, place: &PlaceRef) -> Vec<(String, PathBuf)> {
        self.tab_records(place)
            .into_iter()
            .map(|record| (record.id, record.socket))
            .collect()
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
        if let Some(home) = &self.session_home {
            command.env(SESSION_HOME, home);
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

    /// Where the workspace of the place's repository and base on the
    /// server at `socket` is recorded.
    fn workspace_record(&self, place: &PlaceRef, socket: &Path) -> PathBuf {
        self.records.join("workspaces").join(format!(
            "{}.json",
            digest(&format!(
                "{}\n{}\n{}",
                socket.display(),
                place.repository,
                place.base
            ))
        ))
    }

    /// Where a workspace was recorded before records were kept by
    /// server, which is read as the record of the server it names.
    fn workspace_record_before(&self, place: &PlaceRef) -> PathBuf {
        self.records.join("workspaces").join(format!(
            "{}.json",
            digest(&format!("{}\n{}", place.repository, place.base))
        ))
    }

    /// Where the place's tab on the server at `socket` is recorded.
    fn tab_record(&self, location: &str, socket: &Path) -> PathBuf {
        self.records.join("tabs").join(format!(
            "{}.json",
            digest(&format!("{location}\n{}", socket.display()))
        ))
    }

    /// Where a tab was recorded before records were kept by server,
    /// which is read as the record of the server it names.
    fn tab_record_before(&self, location: &str) -> PathBuf {
        self.records
            .join("tabs")
            .join(format!("{}.json", digest(location)))
    }

    /// Every record under `kind`, `workspaces` or `tabs`, its id under
    /// `key`, oldest first.
    fn records_of(&self, kind: &str, key: &str) -> Vec<Record> {
        let mut records: Vec<(std::time::SystemTime, Record)> =
            fs::read_dir(self.records.join(kind))
                .into_iter()
                .flatten()
                .filter_map(Result::ok)
                .filter_map(|entry| {
                    let path = entry.path();
                    let value = read_json(&path)?;
                    let made = entry
                        .metadata()
                        .and_then(|meta| meta.modified())
                        .unwrap_or(std::time::UNIX_EPOCH);
                    Some((
                        made,
                        Record {
                            id: value.get(key)?.as_str()?.into(),
                            socket: value.get("socket")?.as_str()?.into(),
                            value,
                            path,
                        },
                    ))
                })
                .collect();
        records.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.path.cmp(&b.1.path)));
        records.into_iter().map(|(_, record)| record).collect()
    }

    /// Each workspace recorded for the place's repository and base, one
    /// on each server, oldest first; one recorded before records were
    /// kept by server read as its server's.
    fn base_records(&self, place: &PlaceRef) -> Vec<Record> {
        let before = self.workspace_record_before(place);
        self.records_of("workspaces", "workspace")
            .into_iter()
            .filter(|record| {
                record.path == before
                    || (record.value["repository"].as_str() == Some(place.repository.as_str())
                        && record.value["base"].as_str() == Some(place.base.as_str()))
            })
            .collect()
    }

    /// The workspace recorded for the place's repository and base on the
    /// server at `socket`.
    fn base_record_on(&self, place: &PlaceRef, socket: &Path) -> Option<Record> {
        self.base_records(place)
            .into_iter()
            .find(|record| record.socket == socket)
    }

    /// Each of the place's tabs recorded, one on each server, oldest
    /// first; one recorded before records were kept by server read as its
    /// server's.
    fn tab_records(&self, place: &PlaceRef) -> Vec<Record> {
        let before = self.tab_record_before(&place.location);
        self.records_of("tabs", "tab")
            .into_iter()
            .filter(|record| {
                record.path == before
                    || record.value["place"].as_str() == Some(place.location.as_str())
            })
            .collect()
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
            trust: record
                .get("trust")
                .and_then(|trust| serde_json::from_value(trust.clone()).ok()),
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
                "trust": kept.trust,
            }),
        )
    }

    /// Every pane flume opened, as its records name them, by server and
    /// id.
    fn opened_panes(&self) -> BTreeSet<(PathBuf, String)> {
        fs::read_dir(&self.records)
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "json"))
            .filter_map(|entry| read_json(&entry.path()))
            .filter_map(|record| {
                Some((
                    record.get("socket")?.as_str()?.into(),
                    record.get("pane")?.as_str()?.to_string(),
                ))
            })
            .collect()
    }

    /// Every tab flume made for a place still recorded, by server and id.
    fn place_tabs(&self) -> BTreeSet<(PathBuf, String)> {
        self.records_of("tabs", "tab")
            .into_iter()
            .filter(|record| record.value.get("place").is_some())
            .map(|record| (record.socket, record.id))
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

    /// The socket of `server`: a session's, as `herdr session list`
    /// lists it running, its server started headless where none runs
    /// (107, C.5).
    fn socket_of(&self, server: &Server) -> Result<PathBuf, String> {
        match server {
            Server::Socket(socket) => Ok(socket.clone()),
            Server::Session(name) => match self.running(name)? {
                Some(socket) => Ok(socket),
                None => self.start_session(name),
            },
        }
    }

    /// The socket of the session `name` where `herdr session list` lists
    /// it running; none where it lists it stopped or not at all.
    fn running(&self, name: &str) -> Result<Option<PathBuf>, String> {
        let (_, listing, _) = self.run(None, &["session", "list", "--json"], self.bound)?;
        let sessions: Value = serde_json::from_str(listing.trim())
            .map_err(|error| format!("herdr session list --json: {error}"))?;
        Ok(sessions
            .get("sessions")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find(|session| {
                session.get("name").and_then(Value::as_str) == Some(name)
                    && session.get("running") == Some(&Value::Bool(true))
            })
            .and_then(|session| session.get("socket_path")?.as_str())
            .map(PathBuf::from))
    }

    /// Starts the named session's server headless, apart from this
    /// command, and gives its socket once it answers; a server that ends
    /// before herdr lists it running refuses in herdr's words, as a name
    /// the pinned herdr refuses does, its errors kept under the records'
    /// `servers` (core.29).
    fn start_session(&self, name: &str) -> Result<PathBuf, String> {
        let errors = self
            .records
            .join("servers")
            .join(format!("{}.log", digest(name)));
        let kept = errors
            .parent()
            .map_or(Ok(()), fs::create_dir_all)
            .and_then(|()| fs::File::create(&errors))
            .map_err(|error| format!("{}: {error}", errors.display()))?;
        let mut command = Command::new(&self.program);
        command
            .args(["--session", name, "server"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(kept);
        for (variable, _) in std::env::vars_os() {
            if variable.to_string_lossy().starts_with("HERDR_") {
                command.env_remove(variable);
            }
        }
        if let Some(home) = &self.session_home {
            command.env(SESSION_HOME, home);
        }
        std::os::unix::process::CommandExt::process_group(&mut command, 0);
        let mut child = command
            .spawn()
            .map_err(|error| format!("herdr's session {name} could not be started: {error}"))?;
        let deadline = Instant::now() + self.bound;
        loop {
            if let Some(socket) = self.running(name)? {
                return Ok(socket);
            }
            if let Ok(Some(_)) = child.try_wait() {
                // Another command's server may have come up meanwhile.
                if let Some(socket) = self.running(name)? {
                    return Ok(socket);
                }
                let words = fs::read_to_string(&errors).unwrap_or_default();
                let words = words
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty())
                    .collect::<Vec<_>>()
                    .join("; ");
                return Err(format!("herdr refused the session {name}: {words}"));
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
        let socket = self.socket_of(&self.server)?;
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
    /// environment: a split beside the pane named, or, on the server of
    /// the session `at` names or of flume's own, in the place's tab of the
    /// workspace of its repository and base, each made and recorded with
    /// its server when first needed (94, 107; records 0018, 0031).
    fn new_pane(
        &self,
        place: &PlaceRef,
        short: &str,
        at: &OpenIn,
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
        let socket = match at {
            OpenIn::Beside { pane } => return split(&self.named_pane(pane)?),
            OpenIn::Own => self.socket_of(&self.server)?,
            OpenIn::Session { name } => self.socket_of(&Server::Session(name.clone()))?,
        };
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
                &self.workspace_record(place, &socket),
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
        if let Some(tab) = self
            .tab_records(place)
            .into_iter()
            .find(|tab| tab.socket == socket)
            .map(|tab| tab.id)
        {
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
            &self.tab_record(&place.location, socket),
            &json!({
                "tab": tab,
                "workspace": workspace,
                "socket": socket,
                "label": short,
                "place": place.location,
            }),
        )
    }

    /// The workspace recorded for the place's repository and base on
    /// `socket` where herdr still lists it there; its record dropped
    /// otherwise.
    fn standing_workspace(
        &self,
        place: &PlaceRef,
        socket: &Path,
    ) -> Result<Option<String>, String> {
        let Some(record) = self.base_record_on(place, socket) else {
            return Ok(None);
        };
        let listing = self.answered(socket, &["workspace", "list"])?;
        let listed = listing["workspaces"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|listed| listed["workspace_id"].as_str() == Some(record.id.as_str()));
        if listed {
            return Ok(Some(record.id));
        }
        let _ = fs::remove_file(&record.path);
        Ok(None)
    }

    /// Waits until `deadline` for the pane's shell to come to its prompt:
    /// for its screen to show the shell's first output, which, the shell
    /// reading no startup file, is its prompt or what the shell says as
    /// it starts; herdr's answer, `timeout` where the screen shows
    /// nothing by then (record 0018).
    fn prompt(&self, pane: &Pane, deadline: Instant) -> Call {
        let wait = deadline.saturating_duration_since(Instant::now());
        let timeout = wait.as_millis().to_string();
        self.call(
            &pane.socket,
            &[
                "pane",
                "wait-output",
                &pane.id,
                "--source",
                "visible",
                "--regex",
                r"\S",
                "--timeout",
                &timeout,
            ],
            wait + PAST_TIMEOUT,
        )
    }

    /// The pane's visible lines once they show the folder-trust question,
    /// or once the program has drawn and its screen holds still without
    /// it, so neither a dialog half drawn nor one not yet drawn is taken
    /// for what the screen shows; for up to [`TRUST_SETTLE`]. herdr may
    /// call the program ready before it draws anything, so a screen still
    /// as it was at the first read is settled only after [`TRUST_DRAW`];
    /// where herdr said the program is held at a question, `blocked`, the
    /// question is waited for until the bound, as it may be drawn late.
    fn settled(&self, pane: &Pane, trust: &FolderTrust, blocked: bool) -> String {
        let began = Instant::now();
        let deadline = began + TRUST_SETTLE;
        let first = self.visible(pane);
        let mut last = first.clone();
        let mut still = Instant::now();
        while !asks(&last, trust) && Instant::now() < deadline {
            std::thread::sleep(TRUST_BEAT);
            let screen = self.visible(pane);
            if screen != last {
                still = Instant::now();
            } else if !blocked
                && still.elapsed() >= TRUST_STILL
                && (screen != first || began.elapsed() >= TRUST_DRAW)
            {
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
        opened: &BTreeSet<(PathBuf, String)>,
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
            .filter(|id| !opened.contains(&(socket.to_path_buf(), (*id).to_string())))
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

    /// The base's workspace `record` names closed once it holds no tab of
    /// a live place and no pane flume did not open: what was closed, none
    /// where it stands for a live place or is gone, or what was kept and
    /// what holds it (137, 138).
    fn tidy_workspace(
        &self,
        place: &PlaceRef,
        record: &Record,
        opened: &BTreeSet<(PathBuf, String)>,
    ) -> Result<Option<String>, String> {
        let (workspace, socket) = (&record.id, &record.socket);
        if let Some(kept) = self.held_workspace(record, opened) {
            return Err(kept);
        }
        let label = workspace_label(place);
        match self.live_tabs(workspace, socket)? {
            Some(true) => return Ok(None),
            Some(false) => {}
            None => {
                // herdr closes a workspace with its last tab.
                let _ = fs::remove_file(&record.path);
                return Ok(Some(format!("the workspace {label}")));
            }
        }
        match self.ask(socket, &["workspace", "close", workspace]) {
            Call::Answered(_) => {
                let _ = fs::remove_file(&record.path);
                Ok(Some(format!("the workspace {label}")))
            }
            Call::Refused { code, .. } if code == "workspace_not_found" => {
                let _ = fs::remove_file(&record.path);
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
                .any(|tab| places.contains(&(socket.to_path_buf(), tab.to_string()))),
        ))
    }

    /// The base's workspace `record` names, named with what holds it,
    /// where it holds no tab of a live place and a pane flume did not open
    /// stands in it; none otherwise.
    fn held_workspace(
        &self,
        record: &Record,
        opened: &BTreeSet<(PathBuf, String)>,
    ) -> Option<String> {
        let (workspace, socket) = (&record.id, &record.socket);
        if self.live_tabs(workspace, socket).ok()?? {
            return None;
        }
        let foreign = self
            .foreign_panes(socket, workspace, opened, |_| true)
            .ok()?;
        (!foreign.is_empty()).then(|| {
            format!(
                "the workspace {} ({workspace}), held by the pane {} flume did not open",
                record.label(),
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
        at: &OpenIn,
        session: Option<&str>,
    ) -> Opened {
        let beside = matches!(at, OpenIn::Beside { .. });
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
        let pane = match self.new_pane(place, &identity.place, at, &made) {
            Ok(pane) => pane,
            Err(reason) => return Opened::Refused { reason },
        };
        let kept = Kept {
            pane: pane.clone(),
            name: herdr_name(identity),
            place: (!beside).then(|| place.location.clone()),
            trust: command
                .folder_trust
                .clone()
                .filter(|trust| !trust.question.is_empty()),
        };
        if let Err(reason) = self.keep(identity, &kept) {
            self.abandon(identity, &pane, &bin);
            return Opened::Refused { reason };
        }
        // herdr types the start into the pane's shell, which must be at
        // its prompt: until its line editor takes the terminal, a line
        // past the terminal's canonical limit is refused, and under load a
        // new pane's shell comes to its prompt after herdr is first asked
        // (record 0018).
        let deadline = Instant::now() + self.start_timeout;
        match self.prompt(&pane, deadline) {
            Call::Answered(_) => {}
            Call::Refused { code, words } if code == "timeout" => {
                return Opened::NotComeUp {
                    screen: format!(
                        "{}\nthe pane's shell showed no prompt: herdr refused ({code}): {words}",
                        self.visible(&pane)
                    ),
                }
            }
            Call::Refused { code, words } => {
                self.abandon(identity, &pane, &bin);
                return Opened::Refused {
                    reason: format!("herdr refused ({code}): {words}"),
                };
            }
            Call::Unanswered(words) => {
                self.abandon(identity, &pane, &bin);
                return Opened::Refused { reason: words };
            }
        }
        // Asked again, with what is left of the start's bound and at least
        // herdr's shortest, while herdr answers that the terminal cannot
        // yet take the line, as the shell may draw its prompt, or say
        // something as it starts, before its line editor takes the
        // terminal; nothing was typed.
        let started = loop {
            let left = deadline
                .saturating_duration_since(Instant::now())
                .max(SHORTEST_START);
            let timeout = left.as_millis().to_string();
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
            match self.call(&pane.socket, &args, left + PAST_TIMEOUT) {
                Call::Refused { code, .. }
                    if code == NOT_YET_TAKEN && Instant::now() + TAKEN_BEAT < deadline =>
                {
                    std::thread::sleep(TAKEN_BEAT)
                }
                answer => break answer,
            }
        };
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
            // herdr's server drops a pending name at the start's deadline,
            // a moment before its command's own deadline, so a command
            // that polls in between answers that the name no longer owns
            // the terminal where it means the start timed out: both are
            // read off the screen alike (34; record 0018).
            Call::Refused { code, .. } if code == "timeout" || code == "agent_name_not_found" => {
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
            // The terminal took no line within the start's bound: the
            // shell never came to its prompt, and nothing was typed (35).
            Call::Refused { code, words } if code == NOT_YET_TAKEN => {
                return Opened::NotComeUp {
                    screen: format!("{}\nherdr refused ({code}): {words}", self.visible(&pane)),
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

    /// The folder-trust question the open was given, answered from the
    /// pane's screen as open answers it, where the screen shows it now
    /// (34; record 0018).
    fn answer_trust(&self, agent: &Identity) -> bool {
        let Some(trust) = self.kept(agent).and_then(|kept| kept.trust) else {
            return false;
        };
        let Found::At(pane) = self.found(agent) else {
            return false;
        };
        if !asks(&self.visible(&pane), &trust) {
            return false;
        }
        matches!(self.trust(&pane, &trust, false), Trusted::Answered)
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

    /// The place's tab closed on every server a record names, then the
    /// base's workspace on each of those servers, or on every server where
    /// the place had no tab, each once it holds no tab of a live place and
    /// no pane flume did not open (137, 138; record 0031).
    fn tidy(&self, place: &PlaceRef) -> TerminalTidy {
        let mut tidy = TerminalTidy::default();
        let opened = self.opened_panes();
        let tabs = self.tab_records(place);
        let servers: BTreeSet<PathBuf> = tabs.iter().map(|tab| tab.socket.clone()).collect();
        for record in &tabs {
            let (tab, socket) = (&record.id, &record.socket);
            let label = record.label().to_string();
            let workspace = record.value["workspace"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            match self.foreign_panes(socket, &workspace, &opened, |pane| {
                pane["tab_id"].as_str() == Some(tab.as_str())
            }) {
                Err(reason) => tidy.kept.push(format!(
                    "the tab {label} ({tab}), which herdr could not be read for: {reason}"
                )),
                Ok(foreign) if !foreign.is_empty() => tidy.kept.push(format!(
                    "the tab {label} ({tab}), held by the pane {} flume did not open",
                    foreign.join(", ")
                )),
                Ok(_) => match self.ask(socket, &["tab", "close", tab]) {
                    Call::Answered(_) => {
                        tidy.closed.push(format!("the tab {label}"));
                        let _ = fs::remove_file(&record.path);
                    }
                    Call::Refused { code, .. } if code == "tab_not_found" => {
                        let _ = fs::remove_file(&record.path);
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
        for record in self
            .base_records(place)
            .iter()
            .filter(|record| servers.is_empty() || servers.contains(&record.socket))
        {
            match self.tidy_workspace(place, record, &opened) {
                Ok(Some(closed)) => tidy.closed.push(closed),
                Ok(None) => {}
                Err(kept) => tidy.kept.push(kept),
            }
        }
        tidy
    }

    fn kept(&self) -> Vec<String> {
        let opened = self.opened_panes();
        self.records_of("workspaces", "workspace")
            .iter()
            .filter_map(|record| self.held_workspace(record, &opened))
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
        let token = format!("flume={}", clues.base);
        for record in self.base_records(place) {
            let (workspace, socket) = (&record.id, &record.socket);
            match self.answered(
                socket,
                &[
                    "workspace",
                    "report-metadata",
                    workspace,
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

    /// The place's tab on the live agent's server, else its newest; the
    /// agent's pane; the base's workspace on that tab's server, or its
    /// newest where the place has no tab; and each of the base's
    /// workspaces, on every server; each from the records alone (107,
    /// 139, C.5).
    fn shown_at(&self, place: &PlaceRef, agent: Option<&Identity>) -> ShownAt {
        let pane = agent.and_then(|agent| self.pane(agent));
        let tabs = self.tab_records(place);
        let tab = pane
            .as_ref()
            .and_then(|pane| tabs.iter().find(|tab| tab.socket == pane.socket))
            .or(tabs.last());
        let bases = self.base_records(place);
        let base = match tab {
            Some(tab) => bases.iter().find(|base| base.socket == tab.socket),
            None => bases.last(),
        };
        ShownAt {
            place: tab.map(Record::named),
            agent: pane.map(|pane| pane.session()),
            base: base.map(Record::named),
            bases: bases.iter().map(Record::named).collect(),
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

    /// Refused until the change that opens a stream's window builds it on
    /// the stream's own sessions (187).
    fn attach(&self, _session: Option<&str>, _title: &str) -> Attached {
        Attached::Refused {
            reason: "herdr's attach is not yet built".into(),
        }
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
        let socket = Path::new("/run/h.sock");
        write_json(
            &herdr.tab_record(&place.location, socket),
            &json!({"tab": "w1:t2", "socket": socket, "place": place.location}),
        )
        .unwrap();
        write_json(
            &herdr.workspace_record(&place, socket),
            &json!({"workspace": "w1", "socket": socket, "repository": "/r", "base": "main"}),
        )
        .unwrap();
        keep_pane(&herdr, "w1:p3", "/run/h.sock");
        assert_eq!(
            herdr.shown_at(&place, Some(&identity())),
            ShownAt {
                place: Some("herdr:w1:t2@/run/h.sock".into()),
                agent: Some("herdr:w1:p3@/run/h.sock".into()),
                base: Some("herdr:w1@/run/h.sock".into()),
                bases: vec!["herdr:w1@/run/h.sock".into()],
            }
        );
        let _ = fs::remove_dir_all(records);
    }

    /// The agent `identity()` recorded in the pane `id` on `socket`.
    fn keep_pane(herdr: &Herdr, id: &str, socket: &str) {
        herdr
            .keep(
                &identity(),
                &Kept {
                    pane: Pane {
                        id: id.into(),
                        socket: socket.into(),
                    },
                    name: herdr_name(&identity()),
                    place: Some(place().location),
                    trust: None,
                },
            )
            .unwrap();
    }

    #[test]
    fn a_place_in_two_sessions_shows_on_its_live_agents_server_and_its_base_on_each() {
        // 107, 139, design 2: the tab and the base's workspace on the live
        // agent's server, and each of the base's workspaces in bases.
        let (herdr, records) = over_records("two");
        let place = place();
        for (socket, workspace, tab) in [
            ("/run/flume.sock", "w1", "w1:t2"),
            ("/run/work.sock", "w4", "w4:t1"),
        ] {
            let socket = Path::new(socket);
            write_json(
                &herdr.tab_record(&place.location, socket),
                &json!({"tab": tab, "socket": socket, "place": place.location}),
            )
            .unwrap();
            write_json(
                &herdr.workspace_record(&place, socket),
                &json!({"workspace": workspace, "socket": socket, "repository": "/r", "base": "main"}),
            )
            .unwrap();
        }
        keep_pane(&herdr, "w1:p3", "/run/flume.sock");
        let shown = herdr.shown_at(&place, Some(&identity()));
        assert_eq!(
            (shown.place.as_deref(), shown.base.as_deref()),
            (
                Some("herdr:w1:t2@/run/flume.sock"),
                Some("herdr:w1@/run/flume.sock")
            )
        );
        let mut bases = shown.bases;
        bases.sort();
        assert_eq!(
            bases,
            vec![
                "herdr:w1@/run/flume.sock".to_string(),
                "herdr:w4@/run/work.sock".to_string()
            ]
        );
        assert_eq!(herdr.tabs(&place).len(), 2);
        let _ = fs::remove_dir_all(records);
    }

    #[test]
    fn records_written_before_are_read_as_their_servers() {
        // 138, design 2: a workspace and a tab recorded at the keys used
        // before records were kept by server are the records of the
        // server they name.
        let (herdr, records) = over_records("before");
        let place = place();
        write_json(
            &herdr.tab_record_before(&place.location),
            &json!({"tab": "w1:t2", "socket": "/run/h.sock", "place": place.location}),
        )
        .unwrap();
        write_json(
            &herdr.workspace_record_before(&place),
            &json!({"workspace": "w1", "socket": "/run/h.sock"}),
        )
        .unwrap();
        assert_eq!(
            herdr.shown_at(&place, None),
            ShownAt {
                place: Some("herdr:w1:t2@/run/h.sock".into()),
                agent: None,
                base: Some("herdr:w1@/run/h.sock".into()),
                bases: vec!["herdr:w1@/run/h.sock".into()],
            }
        );
        assert_eq!(
            herdr
                .base_record_on(&place, Path::new("/run/h.sock"))
                .map(|record| record.path),
            Some(herdr.workspace_record_before(&place))
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

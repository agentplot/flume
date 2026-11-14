//! The terminal profile over Rex (C.5's Rex rows and the paragraphs
//! beneath them; the change a-stream-runs-its-agents-in-rex), every
//! command, field and refusal it reads checked against the Rex the tree
//! pins from Rex's flake, 0.1.0-preview.14 at API revision 12 (core.22,
//! core.29).
//!
//! Each of the host's terminal sessions under Rex is a Rex server of
//! flume's own, listening at `<state>/terminal/rex/<session>.sock` with
//! its state in `<session>.state` beside it, flume's own session named
//! `flume`; a name that would put the socket past the 104 bytes a socket's
//! path holds, or outside that folder, is refused naming it. A session's
//! server is started by `server start --restore=false` where
//! `server.status` finds none, never stopped, and never the one the Rex app
//! runs for its user (107, 143; core.13). In each server, the places of a
//! repository on one base are a Rex session labelled `<repository> <base>`,
//! each place a window labelled `<kind>--<name>`, and each agent a block
//! labelled its Rex name; a second agent of a place splits its window, and
//! an agent beside a named block, `rex:<block>@<socket>`, splits beside
//! it, recording no window (94, 107, 199).
//!
//! open claims the identity in the host's state before it makes a block,
//! since Rex refuses no label twice (98, 132). The block runs, with no
//! shell, a script of the start's own, mode 0700, that removes itself and
//! execs `/usr/bin/env -i` with Rex's terminal variables, the login
//! environment, the agent's environment and flume's own, `PATH` led by a
//! folder of the start's own linking the program's kind to the program, so
//! no variable stands in any process's arguments (29, 32; core.30). Rex's
//! ready is the program's first Program Status report, a second signal and
//! never the start; the folder-trust question is answered from the screen
//! while it waits (34, 35). Once ready, the block's program is read from
//! the host's process table and must be the one the settings name (32).
//! Only a program the pinned Rex was read to take Program Status from runs
//! under it: Claude Code, by the kind `claude`.
//!
//! send clears held input with one ctrl+c only while the agent waits, then
//! types the text in pieces Rex takes and presses Enter (95). state is the
//! program's own report as Rex keeps it (96); screen the block's last
//! lines, as many as it has rows (136); close closes the block and reads
//! that it is gone (97). tidy closes a gone place's window in each server
//! it made one in, and its base's Rex session once nothing live and
//! nothing a person opened is in it, naming what keeps either (137, 138;
//! core.18). A recipient's session and the calling command's are
//! `rex:<block>@<socket>`; a role run's address reaches the block its live
//! start holds, and a standing agent the block its server lists under its
//! name (7, 43, 132). runs asks each part's server at once, starting none
//! (201). attach refuses both views and show shows none (139, 187).
//!
//! Every call is `rex --autostart=false -S unix://<socket> <words>`, the
//! calling process's `REX_*` stripped, under a bound, ten seconds where
//! the call states none: one that does not answer within it is killed and
//! read as unknown or refused naming the bound, and a nonzero exit is a
//! refusal in Rex's words (98; core.29).

use super::common::{
    asks, base_label, digest, focused, quoted, read_json, runs_program, write_json, Running,
};
use crate::host::process::{children, executable, program_path};
use flume_seams::{
    AgentCommand, AgentState, AgentsIn, Attached, Closed, Clues, FolderTrust, Identity, NamedAgent,
    OpenIn, Opened, PlaceRef, Sent, SessionRuns, Shown, ShownAt, Terminal, TerminalTidy, ToSession,
    View,
};
use serde_json::{json, Value};
#[cfg(test)]
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Read, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

/// The profile's name in the host's configuration, `[host] multiplexer`
/// and a stream's `multiplexer` (105, 190).
pub const NAME: &str = "rex";

/// The bound on a Rex call that states none (98, C.5).
pub const BOUND: Duration = Duration::from_secs(10);

/// The bound on each part's `server.status` that runs asks (201, C.5).
pub const RUNS_BOUND: Duration = Duration::from_secs(1);

/// How long open waits for the program's first Program Status report
/// (35, C.5).
pub const START_TIMEOUT: Duration = Duration::from_secs(120);

/// How often open reads the block while it waits for the program's
/// report, and the screen while it answers the folder-trust question.
const BEAT: Duration = Duration::from_millis(250);

/// How long the folder-trust question is answered for, once seen, before
/// the start is read never begun (34).
pub const TRUST_WAIT: Duration = Duration::from_secs(30);

/// How long a key sent at the folder-trust question is given to show on
/// the screen before it is sent again, as one sent before the program
/// read its keys is lost.
const TRUST_REDRAW: Duration = Duration::from_secs(2);

/// The session flume opens its places' blocks in where the settings name
/// no block and the core names no session of its own (107, 143).
pub const SESSION: &str = "flume";

/// The oldest API revision the profile reads Rex at, the pinned
/// release's (C.5; core.29).
pub const API_REVISION: u64 = 12;

/// The most of a text one `send` types, since Rex refuses a request over
/// 64 KiB (95, C.5).
pub const PIECE: usize = 60_000;

/// The longest path a socket may have, as macOS allows it (C.5).
pub const SOCKET_LIMIT: usize = 104;

/// The kinds whose programs the pinned Rex was read to take Program
/// Status reports from, Claude Code 2.1.322's natively as
/// `app=claude-code` (32, C.5).
pub const KINDS: [&str; 1] = ["claude"];

/// The flavor of a terminal block (C.5).
pub const FLAVOR: &str = "com.superlogical.terminal.shell";

/// The longest label a Rex name has: 119 characters, `-` and eight hex
/// digits (C.5).
const NAME_WORDS: usize = 119;

/// What Rex answers where no server listens at a socket.
const NO_SERVER: &str = "no Rex server is running at";

/// The program that runs a block's program under it on macOS, which
/// `process` names as the block's child (C.5).
const LOGIN: &str = "/usr/bin/login";

/// The variables of a block's own environment its start script keeps,
/// Rex's terminal variables (C.5).
pub const KEPT: [&str; 7] = [
    "TERM",
    "COLORTERM",
    "TERM_PROGRAM",
    "TERM_PROGRAM_VERSION",
    "REX_SERVER",
    "REX_SESSION",
    "REX_BLOCK",
];

/// The variables of the login environment the start script does not
/// give the program: the shell's own of the shell that read it, and those
/// the script sets itself.
const UNSET: [&str; 5] = ["PATH", "PWD", "OLDPWD", "SHLVL", "_"];

/// A block, by its id and its server's socket.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub id: String,
    pub socket: PathBuf,
}

impl Block {
    /// The block as flume names a session, `rex:<block>@<socket>` (7, 43).
    pub fn session(&self) -> String {
        format!("rex:{}@{}", self.id, self.socket.display())
    }

    /// The block a session name `rex:<block>@<socket>` names; none for a
    /// name in any other form (190).
    pub fn named(session: &str) -> Option<Block> {
        let (id, socket) = session.strip_prefix("rex:")?.split_once('@')?;
        (!id.is_empty() && socket.starts_with('/')).then(|| Block {
            id: id.into(),
            socket: socket.into(),
        })
    }
}

/// The block the calling command runs in, as Rex's environment names it,
/// `REX_BLOCK` on the server `REX_SERVER`, both set, the server a Unix
/// socket; none otherwise (7, C.5).
pub fn caller() -> Option<Block> {
    caller_of(
        std::env::var("REX_BLOCK").ok().as_deref(),
        std::env::var("REX_SERVER").ok().as_deref(),
    )
}

fn caller_of(block: Option<&str>, server: Option<&str>) -> Option<Block> {
    let (block, server) = (block?, server?);
    let socket = server.strip_prefix("unix://").unwrap_or(server);
    (!block.is_empty() && socket.starts_with('/')).then(|| Block {
        id: block.into(),
        socket: socket.into(),
    })
}

/// What a Rex call came to.
#[derive(Clone, Debug, PartialEq)]
pub enum Call {
    /// Rex answered: its JSON, null where it printed nothing.
    Answered(Value),
    /// Rex refused, in its words.
    Refused(String),
    /// No answer within the bound, or Rex could not be run.
    Unanswered(String),
}

impl Call {
    /// Whether Rex refused because no server listens at the socket.
    fn no_server(&self) -> bool {
        matches!(self, Call::Refused(words) if words.contains(NO_SERVER))
    }

    /// Whether Rex refused because it holds no such block.
    fn block_gone(&self, block: &str) -> bool {
        matches!(self, Call::Refused(words)
            if words.contains(&format!("block target \"{block}\" not found")))
    }

    /// Rex's answer, or its refusal or silence in its words.
    fn answer(self) -> Result<Value, String> {
        match self {
            Call::Answered(value) => Ok(value),
            Call::Refused(words) => Err(format!("Rex refused: {words}")),
            Call::Unanswered(words) => Err(words),
        }
    }
}

/// An agent's block as open recorded it: where it is, and what the
/// record holds besides.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Located {
    block: Block,
    session: Option<String>,
}

/// The terminal over Rex.
#[derive(Clone, Debug)]
pub struct Rex {
    program: PathBuf,
    records: PathBuf,
    bound: Duration,
    start_timeout: Duration,
    caller: Option<Block>,
    login: Arc<OnceLock<BTreeMap<String, String>>>,
}

impl Rex {
    /// Rex recording what it makes, and listening for its sessions, under
    /// `records`, flume's `<state>/terminal/rex`; the calling command in no
    /// block. Building it runs nothing (190).
    pub fn new(records: impl Into<PathBuf>) -> Rex {
        Rex {
            program: NAME.into(),
            records: records.into(),
            bound: BOUND,
            start_timeout: START_TIMEOUT,
            caller: None,
            login: Arc::default(),
        }
    }

    /// Rex run as `program`, in place of the `rex` on the `PATH`; for
    /// tests, which name the pinned one.
    pub fn with_program(mut self, program: impl Into<PathBuf>) -> Rex {
        self.program = program.into();
        self
    }

    /// The bound on each call that states none, in place of ten seconds.
    pub fn with_bound(mut self, bound: Duration) -> Rex {
        self.bound = bound;
        self
    }

    /// How long open waits for the program's first report, in place of
    /// 120 seconds.
    pub fn with_start_timeout(mut self, timeout: Duration) -> Rex {
        self.start_timeout = timeout;
        self
    }

    /// The block the calling command runs in, if any.
    pub fn with_caller(mut self, caller: Option<Block>) -> Rex {
        self.caller = caller;
        self
    }

    /// Exactly `variables` for the host user's login environment, in
    /// place of what the user's login shell sets.
    pub fn with_login(self, variables: &[(String, String)]) -> Rex {
        let login = OnceLock::new();
        let _ = login.set(variables.iter().cloned().collect());
        Rex {
            login: Arc::new(login),
            ..self
        }
    }

    /// The host user's login environment, read once (29).
    fn login(&self) -> &BTreeMap<String, String> {
        self.login.get_or_init(crate::host::login_environment)
    }

    /// The socket of the session `name`, flume's own where none, under the
    /// records' folder; refused, naming the session, where the name is
    /// empty, `.` or `..`, holds `/` or a NUL, or puts the socket's path
    /// past 104 bytes (107, C.5).
    pub fn socket(&self, name: Option<&str>) -> Result<PathBuf, String> {
        let name = name.unwrap_or(SESSION);
        if name.is_empty() || name == "." || name == ".." || name.contains(['/', '\0']) {
            return Err(format!(
                "the session {name:?} names no socket under {}: a Rex session's name is a file's \
                 name, and is not empty, . or .., and holds no / and no NUL",
                self.records.display()
            ));
        }
        let socket = self.records.join(format!("{name}.sock"));
        let length = socket.as_os_str().len();
        if length > SOCKET_LIMIT {
            return Err(format!(
                "the session {name}'s Rex socket {} is {length} bytes, past the {SOCKET_LIMIT} a \
                 socket's path holds",
                socket.display()
            ));
        }
        Ok(socket)
    }

    /// The block an agent of `identity` would be labelled with, its Rex
    /// name (C.5).
    pub fn name(identity: &Identity) -> String {
        rex_name(identity)
    }

    /// The block the agent under `identity` runs in, as last recorded.
    pub fn block(&self, identity: &Identity) -> Option<Block> {
        self.located(identity).map(|located| located.block)
    }

    /// The Rex session last recorded for the place's repository and base,
    /// on any server, by id and server.
    pub fn base_session(&self, place: &PlaceRef) -> Option<(String, PathBuf)> {
        self.base_records(place)
            .pop()
            .map(|record| (record.id, record.socket))
    }

    /// The place's window last recorded, on any server, by id and server.
    pub fn window(&self, place: &PlaceRef) -> Option<(String, PathBuf)> {
        self.window_records(place)
            .pop()
            .map(|record| (record.id, record.socket))
    }

    /// Rex run with `args` on the server at `socket`, under `bound`: its
    /// JSON answer, its refusal in its words, or no answer.
    pub fn call(&self, socket: &Path, args: &[&str], bound: Duration) -> Call {
        let mut command = Command::new(&self.program);
        command
            .arg("--autostart=false")
            .arg("-S")
            .arg(format!("unix://{}", socket.display()))
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (name, _) in std::env::vars_os() {
            if name.to_string_lossy().starts_with("REX_") {
                command.env_remove(name);
            }
        }
        std::os::unix::process::CommandExt::process_group(&mut command, 0);
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(error) => return Call::Unanswered(format!("rex could not be run: {error}")),
        };
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
                    if let Ok(group) = libc::pid_t::try_from(child.id()) {
                        // SAFETY: a signal to the process group this call
                        // made for its rex.
                        unsafe { libc::kill(-group, libc::SIGKILL) };
                    }
                    let _ = child.wait();
                    return Call::Unanswered(format!(
                        "rex {} did not answer within {} seconds",
                        words(args),
                        bound.as_secs_f32()
                    ));
                }
                Err(error) => {
                    return Call::Unanswered(format!("rex could not be waited on: {error}"))
                }
            }
        };
        let (stdout, stderr) = (
            stdout.join().unwrap_or_default(),
            stderr.join().unwrap_or_default(),
        );
        if !status.success() {
            let said = stderr.trim();
            let said = said.strip_prefix("Error: ").unwrap_or(said);
            return Call::Refused(if said.is_empty() {
                format!("rex {} exited with {status}", words(args))
            } else {
                said.to_string()
            });
        }
        let parsed = match stdout.trim() {
            "" => return Call::Answered(Value::Null),
            text => serde_json::from_str::<Value>(text).ok(),
        };
        Call::Answered(parsed.unwrap_or(Value::String(stdout)))
    }

    /// Rex's answer under the profile's bound.
    fn ask(&self, socket: &Path, args: &[&str]) -> Call {
        self.call(socket, args, self.bound)
    }

    /// The server of the session `name`, flume's own where none, by its
    /// socket: `server.status` at it, and, where no server listens there,
    /// `server start` restoring nothing; a server at an API revision below
    /// 12 refused in Rex's words (107, C.5; core.13).
    fn server(&self, name: Option<&str>) -> Result<PathBuf, String> {
        let socket = self.socket(name)?;
        let status = match self.ask(&socket, &["api", "call", "server.status"]) {
            call if call.no_server() => {
                let state = socket.with_extension("state");
                let state = state.to_string_lossy();
                self.ask(
                    &socket,
                    &[
                        "server",
                        "start",
                        "--state-dir",
                        &state,
                        "--restore=false",
                        "--json",
                    ],
                )
                .answer()?
            }
            call => call.answer()?,
        };
        match status["api_revision"].as_u64() {
            Some(revision) if revision >= API_REVISION => Ok(socket),
            revision => Err(format!(
                "the Rex server at {} answers API revision {}, {}, and flume reads Rex at revision \
                 {API_REVISION} or later",
                socket.display(),
                revision.map_or_else(|| "none".to_string(), |revision| revision.to_string()),
                status["rex_version"].as_str().unwrap_or("its version unsaid")
            )),
        }
    }

    fn agent_record(&self, identity: &Identity) -> PathBuf {
        self.records
            .join("agents")
            .join(format!("{}.json", digest(&identity.to_string())))
    }

    fn base_record(&self, place: &PlaceRef, socket: &Path) -> PathBuf {
        self.records.join("bases").join(format!(
            "{}.json",
            digest(&format!(
                "{}\n{}\n{}",
                socket.display(),
                place.repository,
                place.base
            ))
        ))
    }

    fn window_record(&self, place: &PlaceRef, socket: &Path) -> PathBuf {
        self.records.join("windows").join(format!(
            "{}.json",
            digest(&format!("{}\n{}", place.location, socket.display()))
        ))
    }

    /// Every record under `kind`, its id under `key`, oldest first.
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

    /// Each Rex session recorded for the place's repository and base, one
    /// on each server, oldest first.
    fn base_records(&self, place: &PlaceRef) -> Vec<Record> {
        self.records_of("bases", "session")
            .into_iter()
            .filter(|record| {
                record.value["repository"].as_str() == Some(place.repository.as_str())
                    && record.value["base"].as_str() == Some(place.base.as_str())
            })
            .collect()
    }

    /// Each of the place's windows recorded, one on each server, oldest
    /// first.
    fn window_records(&self, place: &PlaceRef) -> Vec<Record> {
        self.records_of("windows", "window")
            .into_iter()
            .filter(|record| record.value["place"].as_str() == Some(place.location.as_str()))
            .collect()
    }

    /// Every agent's record, with the file it is kept in.
    fn agent_records(&self) -> Vec<(PathBuf, Value)> {
        fs::read_dir(self.records.join("agents"))
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "json"))
            .filter_map(|entry| Some((entry.path(), read_json(&entry.path())?)))
            .collect()
    }

    /// Every block flume opened, as its records name them, by server and
    /// id.
    fn opened_blocks(&self) -> BTreeSet<(PathBuf, String)> {
        self.agent_records()
            .into_iter()
            .filter_map(|(_, record)| {
                Some((
                    record.get("socket")?.as_str()?.into(),
                    record.get("block")?.as_str()?.to_string(),
                ))
            })
            .collect()
    }

    /// The agent's block as its record names it; none where no record
    /// stands or it names no block yet.
    fn located(&self, identity: &Identity) -> Option<Located> {
        located_in(&read_json(&self.agent_record(identity))?)
    }

    /// The agent's state: stopped where no record stands, or where one
    /// claims the identity for an open whose process is gone; unknown
    /// while an open of a live process holds it; otherwise its block's
    /// (96, 98).
    fn agent_state(&self, identity: &Identity) -> AgentState {
        let Some(record) = read_json(&self.agent_record(identity)) else {
            return AgentState::Stopped;
        };
        match located_in(&record) {
            Some(located) => self.block_state(&located),
            None => {
                let claimer = record["claimed_by"]
                    .as_u64()
                    .and_then(|pid| i32::try_from(pid).ok());
                // SAFETY: signal 0 asks only whether the process stands.
                let alive = claimer.is_some_and(|pid| unsafe { libc::kill(pid, 0) } == 0);
                if alive {
                    AgentState::Unknown
                } else {
                    AgentState::Stopped
                }
            }
        }
    }

    /// The block's `process`: its answer, none where Rex holds no such
    /// block or no server listens at its socket, or why it could not be
    /// read (96, C.5).
    fn process(&self, located: &Located) -> Result<Option<Value>, String> {
        let call = self.block_call(located, "process");
        if call.block_gone(&located.block.id) || call.no_server() {
            return Ok(None);
        }
        call.answer().map(Some)
    }

    /// `block call <method>` on the block, in its Rex session where its
    /// record names one.
    fn block_call(&self, located: &Located, method: &str) -> Call {
        let mut args = vec!["block", "call"];
        if let Some(session) = &located.session {
            args.extend(["-s", session.as_str()]);
        }
        args.extend(["-b", located.block.id.as_str(), method]);
        self.ask(&located.block.socket, &args)
    }

    /// The block's state, as C.5's state row reads it, or why it could
    /// not be read (96, 98).
    fn read_block(&self, located: &Located) -> Result<AgentState, String> {
        let Some(process) = self.process(located)? else {
            return Ok(AgentState::Stopped);
        };
        let status = self.block_call(located, "program_status").answer()?;
        Ok(read_state(&process, &status))
    }

    /// The block's state, unknown where it could not be read (96, 98).
    fn block_state(&self, located: &Located) -> AgentState {
        self.read_block(located).unwrap_or(AgentState::Unknown)
    }

    /// The block's screen: the last lines of `capture --trim`, as many as
    /// `size` gives it rows (136, C.5).
    fn visible(&self, located: &Located) -> Result<String, String> {
        let mut args = vec!["capture"];
        if let Some(session) = &located.session {
            args.extend(["-s", session.as_str()]);
        }
        args.extend(["-b", located.block.id.as_str(), "--trim"]);
        let captured = match self.ask(&located.block.socket, &args).answer()? {
            Value::String(text) => text,
            Value::Null => String::new(),
            other => other.to_string(),
        };
        let rows = self
            .block_call(located, "size")
            .answer()?
            .get("rows")
            .and_then(Value::as_u64)
            .unwrap_or(24);
        Ok(last_rows(&captured, rows as usize))
    }

    /// `keys` pressed in the block, by `send-key`.
    fn keys(&self, located: &Located, keys: &[&str]) -> Result<(), String> {
        let mut args = vec!["send-key"];
        if let Some(session) = &located.session {
            args.extend(["-s", session.as_str()]);
        }
        args.extend(["-b", located.block.id.as_str()]);
        args.extend(keys);
        self.ask(&located.block.socket, &args).answer().map(|_| ())
    }

    /// `text` typed into the block in pieces of at most [`PIECE`] bytes,
    /// cut between characters, then Enter pressed; held input first
    /// cleared with one ctrl+c where `clear` (95, C.5).
    fn typed(&self, located: &Located, text: &str, clear: bool) -> Result<(), String> {
        if clear {
            self.keys(located, &["ctrl+c"])?;
        }
        for piece in pieces(text, PIECE) {
            let mut args = vec!["send"];
            if let Some(session) = &located.session {
                args.extend(["-s", session.as_str()]);
            }
            args.extend(["-b", located.block.id.as_str(), "--", piece]);
            self.ask(&located.block.socket, &args).answer()?;
        }
        self.keys(located, &["enter"])
    }

    /// The Rex session of the server at `socket` whose view holds
    /// `block`, by `session.list` and `session.view`.
    fn session_holding(&self, socket: &Path, block: &str) -> Result<String, String> {
        for session in self.sessions(socket)? {
            let view = self.view(socket, &session)?;
            if view_blocks(&view).iter().any(|(id, _)| id == block) {
                return Ok(session);
            }
        }
        Err(format!(
            "the Rex server at {} holds no block {block}",
            socket.display()
        ))
    }

    /// The Rex sessions `session.list` lists on the server at `socket`.
    fn sessions(&self, socket: &Path) -> Result<Vec<String>, String> {
        let listed = self
            .ask(socket, &["api", "call", "session.list"])
            .answer()?;
        Ok(listed["sessions"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|session| session["session_id"].as_str().map(str::to_string))
            .collect())
    }

    /// The view of the Rex session `session`; Rex's refusal where it holds
    /// none.
    fn view(&self, socket: &Path, session: &str) -> Result<Value, String> {
        self.ask(socket, &["api", "call", "-s", session, "session.view"])
            .answer()
    }

    /// The block's options for the agent's start: no shell, the place its
    /// folder, kept once its program exits, running `script` (94, C.5).
    fn block_layout(&self, label: &str, place: &PlaceRef, script: &Path) -> Value {
        json!({
            "block": {
                "flavor": FLAVOR,
                "label": label,
                "options": {
                    "shell": "none",
                    "cwd": place.location,
                    "exit": {"on_completion": false},
                    "command": [script.to_string_lossy()],
                },
            }
        })
    }

    /// A new block for the agent: a split beside the block named, or, on
    /// the server of the session `at` names or of flume's own, in the
    /// place's window of its base's Rex session, each made and recorded
    /// with its server when first needed (94, 107, 199, C.5).
    fn new_block(&self, place: &PlaceRef, at: &OpenIn, layout: &Value) -> Result<Located, String> {
        let split = |socket: &Path, session: &str, anchor: &str| -> Result<Located, String> {
            let args = json!({
                "anchor_id": anchor,
                "direction": "horizontal",
                "side": "after",
                "layout": layout,
            })
            .to_string();
            let split = self
                .ask(
                    socket,
                    &["api", "call", "-s", session, "session.new_split", &args],
                )
                .answer()?;
            Ok(Located {
                block: Block {
                    id: first_block(&split)?,
                    socket: socket.to_path_buf(),
                },
                session: Some(session.into()),
            })
        };
        let name = match at {
            OpenIn::Beside { pane } => {
                let Some(beside) = Block::named(pane) else {
                    return Err(format!(
                        "Rex opens beside a block named rex:<block>@<socket>, and {pane} names none"
                    ));
                };
                let session = self.session_holding(&beside.socket, &beside.id)?;
                return split(&beside.socket, &session, &beside.id);
            }
            OpenIn::Own => None,
            OpenIn::Session { name } => Some(name.as_str()),
        };
        let socket = self.server(name)?;
        let short = place_short(place);
        let Some(session) = self.standing_session(place, &socket)? else {
            let label = base_label(place);
            let args = json!({
                "label": label,
                "initial_windows": [{"window_label": short, "layout": layout}],
            })
            .to_string();
            let created = self
                .ask(&socket, &["api", "call", "session.create", &args])
                .answer()?;
            let session = created["session_id"]
                .as_str()
                .ok_or_else(|| format!("Rex gave no session in {created}"))?
                .to_string();
            write_json(
                &self.base_record(place, &socket),
                &json!({
                    "session": session,
                    "socket": socket,
                    "repository": place.repository,
                    "base": place.base,
                    "label": label,
                }),
            )?;
            let window = &created["initial_windows"][0];
            let id = window["window_id"]
                .as_str()
                .ok_or_else(|| format!("Rex gave no window in {created}"))?;
            self.keep_window(place, id, &session, &socket, &short)?;
            return Ok(Located {
                block: Block {
                    id: first_block(window)?,
                    socket,
                },
                session: Some(session),
            });
        };
        if let Some(window) = self
            .window_records(place)
            .into_iter()
            .find(|window| window.socket == socket)
        {
            let view = self.view(&socket, &session)?;
            let anchor = view["windows"]
                .as_array()
                .into_iter()
                .flatten()
                .find(|listed| listed["window_id"].as_str() == Some(window.id.as_str()))
                .and_then(|listed| window_blocks(listed).into_iter().next());
            if let Some((anchor, _)) = anchor {
                return split(&socket, &session, &anchor);
            }
        }
        let args = json!({
            "window_label": short,
            "focus": false,
            "layout": layout,
        })
        .to_string();
        let created = self
            .ask(
                &socket,
                &["api", "call", "-s", &session, "session.new_window", &args],
            )
            .answer()?;
        let id = created["window_id"]
            .as_str()
            .ok_or_else(|| format!("Rex gave no window in {created}"))?;
        self.keep_window(place, id, &session, &socket, &short)?;
        Ok(Located {
            block: Block {
                id: first_block(&created)?,
                socket,
            },
            session: Some(session),
        })
    }

    fn keep_window(
        &self,
        place: &PlaceRef,
        window: &str,
        session: &str,
        socket: &Path,
        label: &str,
    ) -> Result<(), String> {
        write_json(
            &self.window_record(place, socket),
            &json!({
                "window": window,
                "session": session,
                "socket": socket,
                "label": label,
                "place": place.location,
                "repository": place.repository,
                "base": place.base,
            }),
        )
    }

    /// The Rex session recorded for the place's repository and base on
    /// `socket` where `session.list` still lists it there; its record
    /// dropped otherwise.
    fn standing_session(&self, place: &PlaceRef, socket: &Path) -> Result<Option<String>, String> {
        let Some(record) = self
            .base_records(place)
            .into_iter()
            .find(|record| record.socket == socket)
        else {
            return Ok(None);
        };
        if self.sessions(socket)?.contains(&record.id) {
            return Ok(Some(record.id));
        }
        let _ = fs::remove_file(&record.path);
        Ok(None)
    }

    /// The identity claimed in the host's state for this open, by its
    /// record made only where none stands: one whose agent reads stopped
    /// is replaced, and one whose agent does not refuses the open (98).
    fn claim(&self, identity: &Identity, record: &Value) -> Result<(), String> {
        let path = self.agent_record(identity);
        for _ in 0..2 {
            match create_json(&path, record) {
                Ok(true) => return Ok(()),
                Ok(false) => {}
                Err(reason) => return Err(reason),
            }
            if self.agent_state(identity) != AgentState::Stopped {
                return Err(format!("an agent is live under {identity}"));
            }
            self.forget(identity);
        }
        Err(format!(
            "the record of {identity} at {} could not be claimed",
            path.display()
        ))
    }

    /// The agent's record and the start's folder it names removed.
    fn forget(&self, identity: &Identity) {
        let path = self.agent_record(identity);
        if let Some(start) = read_json(&path)
            .as_ref()
            .and_then(|record| record["start"].as_str())
        {
            let _ = fs::remove_dir_all(start);
        }
        let _ = fs::remove_file(path);
    }

    /// The block closed, the start's folder and the identity's record
    /// with it, for an open that failed after the block was made.
    fn abandon(&self, identity: &Identity, located: &Located) {
        let _ = self.ask(
            &located.block.socket,
            &close_args(located)
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        );
        self.forget(identity);
    }

    /// The live agent other than `identity` that carries the session name
    /// `name`, as its record names it (132).
    fn carrier(&self, name: &str, identity: &Identity) -> Option<Identity> {
        self.agent_records().into_iter().find_map(|(_, record)| {
            if record["named"].as_str() != Some(name) {
                return None;
            }
            let holder: Identity = serde_json::from_value(record["identity"].clone()).ok()?;
            (holder != *identity && self.agent_state(&holder) != AgentState::Stopped)
                .then_some(holder)
        })
    }

    /// The block a session name reaches: `rex:<block>@<socket>` by its id
    /// on its server, and a name an open gave an agent by the block its
    /// live start holds; none for any other name, with no call made (43,
    /// 132, 190).
    fn reach(&self, session: &str) -> Option<Located> {
        if let Some(block) = Block::named(session) {
            return Some(Located {
                block,
                session: None,
            });
        }
        self.agent_records().into_iter().find_map(|(_, record)| {
            (record["named"].as_str() == Some(session))
                .then(|| located_in(&record))
                .flatten()
        })
    }

    /// Reads the block until the program's first Program Status report,
    /// or until the bound: the folder-trust question on its screen
    /// answered meanwhile, an exit or the bound passing with no report the
    /// start never begun with the screen; the block's child, by its pid,
    /// once ready (34, 35, C.5).
    fn ready(&self, located: &Located, trust: Option<&FolderTrust>) -> Result<u32, Opened> {
        let deadline = Instant::now() + self.start_timeout;
        let never_begun = |words: &str| Opened::NotComeUp {
            screen: match self.visible(located) {
                Ok(screen) => format!("{screen}\n{words}"),
                Err(reason) => format!("{words}\n{reason}"),
            },
        };
        loop {
            match self.process(located) {
                Ok(None) => {
                    return Err(Opened::NotComeUp {
                        screen: format!("the block {} is gone", located.block.id),
                    })
                }
                Ok(Some(process)) if !process["last_exit"].is_null() => {
                    return Err(never_begun(&format!(
                        "the program exited before it reported its state to Rex: {}",
                        process["last_exit"]
                    )))
                }
                Ok(Some(process)) => {
                    if let Some(child) = process["child"]["pid"].as_u64() {
                        if let Call::Answered(status) = self.block_call(located, "program_status") {
                            if owned_records(&status, child).next().is_some() {
                                return Ok(child as u32);
                            }
                        }
                    }
                }
                Err(_) => {}
            }
            if Instant::now() >= deadline {
                return Err(never_begun(&format!(
                    "the program reported no state to Rex within {} seconds",
                    self.start_timeout.as_secs()
                )));
            }
            if let Some(trust) = trust {
                if let Ok(screen) = self.visible(located) {
                    if asks(&screen, trust) {
                        if let Trusted::Unanswered(screen) = self.trust(located, trust, screen) {
                            return Err(Opened::NotComeUp { screen });
                        }
                    }
                }
            }
            std::thread::sleep(BEAT);
        }
    }

    /// Answers the folder-trust question on `screen` within
    /// [`TRUST_WAIT`]: the next key while another choice has the focus,
    /// the confirming key only while the yes choice has it, each key sent
    /// once the screen shows what the last did, read again until the
    /// question is gone. Nothing is ever confirmed on another choice (34;
    /// core.12).
    fn trust(&self, located: &Located, trust: &FolderTrust, mut screen: String) -> Trusted {
        let deadline = Instant::now() + TRUST_WAIT;
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
                let keys: Vec<&str> = key.split_whitespace().collect();
                if let Err(reason) = self.keys(located, &keys) {
                    return Trusted::Unanswered(format!("{screen}\n{reason}"));
                }
                sent = Some((screen.clone(), Instant::now()));
            }
            std::thread::sleep(BEAT);
            screen = match self.visible(located) {
                Ok(screen) => screen,
                Err(reason) => return Trusted::Unanswered(reason),
            };
        }
    }

    /// Whether the block's child, `child`, or its child where that is
    /// `/usr/bin/login`, is `program`, read from the host's process table;
    /// or what ran in its place (32, C.5).
    fn runs(&self, located: &Located, child: u32, program: &Path) -> Result<(), String> {
        let processes: Vec<Running> =
            if executable(child).is_some_and(|path| path == Path::new(LOGIN)) {
                children(child)
                    .into_iter()
                    .map(|process| Running {
                        pid: process.pid,
                        args: process.args,
                        cwd: None,
                        cmdline: None,
                    })
                    .collect()
            } else {
                vec![Running {
                    pid: child,
                    ..Running::default()
                }]
            };
        match runs_program(&processes, program) {
            Ok(()) => Ok(()),
            Err(ran) if ran.is_empty() => Err(format!(
                "the host's process table shows nothing running in the Rex block {}, not {}",
                located.block.id,
                program.display()
            )),
            Err(ran) => Err(format!(
                "the Rex block {} ran {} rather than {}, the program the settings name",
                located.block.id,
                ran.join(", "),
                program.display()
            )),
        }
    }

    /// The start script's text for the program `bin/<kind>`, its
    /// environment the login environment with the agent's over it, `PATH`
    /// led by `bin` (29, 32, C.5; core.30).
    fn start_script(
        &self,
        script: &Path,
        link: &Path,
        command: &AgentCommand,
        bin: &Path,
    ) -> String {
        let path = command
            .env
            .get("PATH")
            .cloned()
            .or_else(|| std::env::var("PATH").ok())
            .unwrap_or_else(|| "/usr/bin:/bin".into());
        let mut env: BTreeMap<String, String> = self
            .login()
            .iter()
            .filter(|(name, _)| {
                !UNSET.contains(&name.as_str())
                    && !KEPT.contains(&name.as_str())
                    && !name.starts_with("REX_")
            })
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect();
        env.extend(command.env.clone());
        env.insert("PATH".into(), format!("{}:{path}", bin.display()));
        start_script(script, &env, link, &command.args)
    }
}

/// A Rex session or a window open made, as its record names it: its id
/// and its server's socket, the record whole, and the file it is kept in.
#[derive(Clone, Debug)]
struct Record {
    id: String,
    socket: PathBuf,
    value: Value,
    path: PathBuf,
}

impl Record {
    /// Its label as recorded, else its id.
    fn label(&self) -> &str {
        self.value["label"].as_str().unwrap_or(&self.id)
    }
}

/// What came of the folder-trust question at a start.
enum Trusted {
    /// The question was answered yes and is gone from the screen.
    Answered,
    /// The question still stands, with the screen.
    Unanswered(String),
}

/// The agent's block as its record `record` names it.
fn located_in(record: &Value) -> Option<Located> {
    Some(Located {
        block: Block {
            id: record.get("block")?.as_str()?.into(),
            socket: record.get("socket")?.as_str()?.into(),
        },
        session: record
            .get("session")
            .and_then(Value::as_str)
            .map(str::to_string),
    })
}

/// `block close` of the block, in its Rex session where one is named.
fn close_args(located: &Located) -> Vec<String> {
    let mut args = vec!["block".to_string(), "close".into()];
    if let Some(session) = &located.session {
        args.extend(["-s".into(), session.clone()]);
    }
    args.push(located.block.id.clone());
    args
}

/// `value` written as the record at `path` only where none stands there,
/// whole at once, so no reader finds it half written: written beside it,
/// then linked to its name, which fails where a record stands; whether it
/// was; the words naming the path where it could not be.
fn create_json(path: &Path, value: &Value) -> Result<bool, String> {
    static CLAIMS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let beside = path.with_extension(format!(
        "{}-{}.claim",
        std::process::id(),
        CLAIMS.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let failed = |error: std::io::Error| format!("{}: {error}", path.display());
    path.parent()
        .map_or(Ok(()), fs::create_dir_all)
        .and_then(|()| {
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(&beside)?;
            file.write_all(value.to_string().as_bytes())
        })
        .map_err(failed)?;
    let linked = fs::hard_link(&beside, path);
    let _ = fs::remove_file(&beside);
    match linked {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
        Err(error) => Err(failed(error)),
    }
}

/// The agent's Rex name: `f-<place>-<stage>-<start>` lowercased, each
/// character outside `[a-z0-9_-]` made `-`, cut to 119 characters, then
/// `-` and the first eight hex digits of the identity's SHA-256 (C.5).
pub fn rex_name(identity: &Identity) -> String {
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
        .take(NAME_WORDS)
        .collect();
    format!("{words}-{}", &digest(&identity.to_string())[..8])
}

/// The place's short name, `<kind>--<name>`, the last part of its folder.
fn place_short(place: &PlaceRef) -> String {
    Path::new(&place.location).file_name().map_or_else(
        || place.location.clone(),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// The start script's text: it removes `script`, itself, then execs
/// `/usr/bin/env -i` with Rex's terminal variables taken from its own
/// environment where set, each of `env`, then `link` and `args`, every
/// word quoted whole, so no variable stands in a process's arguments
/// (29, 32, C.5; core.30).
pub fn start_script(
    script: &Path,
    env: &BTreeMap<String, String>,
    link: &Path,
    args: &[String],
) -> String {
    let mut text = String::from(
        "#!/bin/sh\n# flume's start of an agent under Rex: it removes itself, then runs the \
         agent's program\n# with Rex's terminal variables and exactly the environment below.\n",
    );
    text.push_str(&format!(
        "rm -f -- {}\nexec /usr/bin/env -i \\\n",
        quoted(&script.to_string_lossy())
    ));
    for name in KEPT {
        text.push_str(&format!("  ${{{name}+\"{name}=${name}\"}} \\\n"));
    }
    for (name, value) in env {
        text.push_str(&format!("  {} \\\n", quoted(&format!("{name}={value}"))));
    }
    text.push_str(&format!("  {}", quoted(&link.to_string_lossy())));
    for arg in args {
        text.push_str(&format!(" \\\n  {}", quoted(arg)));
    }
    text.push('\n');
    text
}

/// `text` cut into pieces of at most `most` bytes, each cut between
/// characters, in order (95, C.5).
pub fn pieces(text: &str, most: usize) -> Vec<&str> {
    let mut pieces = vec![];
    let mut rest = text;
    while !rest.is_empty() {
        let mut cut = rest.len().min(most.max(4));
        while !rest.is_char_boundary(cut) {
            cut -= 1;
        }
        let (piece, after) = rest.split_at(cut);
        pieces.push(piece);
        rest = after;
    }
    pieces
}

/// The last `rows` lines of `captured`, the screen and scrollback Rex's
/// `capture --trim` gives (136, C.5).
pub fn last_rows(captured: &str, rows: usize) -> String {
    let lines: Vec<&str> = captured.lines().collect();
    lines[lines.len().saturating_sub(rows)..].join("\n")
}

/// The records of `program_status` whose owner is the block's child
/// `child`.
fn owned_records(status: &Value, child: u64) -> impl Iterator<Item = &Value> {
    status["records"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(move |record| record["owner"]["pid"].as_u64() == Some(child))
}

/// An agent's state from its block's `process` and `program_status`, as
/// C.5's state row reads them: stopped where the program exited; else
/// the newest record by `updated_at` whose owner is the block's child,
/// `working` working, `idle` and `done` waiting, `blocked` asking, `error`
/// waiting, and none unknown (96).
pub fn read_state(process: &Value, status: &Value) -> AgentState {
    if !process["last_exit"].is_null() {
        return AgentState::Stopped;
    }
    let Some(child) = process["child"]["pid"].as_u64() else {
        return AgentState::Unknown;
    };
    let newest = owned_records(status, child).max_by_key(|record| {
        record["updated_at"]
            .as_str()
            .and_then(|at| chrono::DateTime::parse_from_rfc3339(at).ok())
            .map(|at| at.timestamp_nanos_opt().unwrap_or_default())
            .unwrap_or_default()
    });
    match newest.and_then(|record| record["state"].as_str()) {
        Some("working") => AgentState::Working,
        Some("idle" | "done" | "error") => AgentState::Waiting,
        Some("blocked") => AgentState::Asking,
        _ => AgentState::Unknown,
    }
}

/// The first block id an answer gives under `block_ids`.
fn first_block(answer: &Value) -> Result<String, String> {
    answer["block_ids"][0]
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| format!("Rex gave no block in {answer}"))
}

/// Each block of a window of a view, by id and label.
fn window_blocks(window: &Value) -> Vec<(String, Option<String>)> {
    window["layers"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|layer| layer["blocks"].as_array().into_iter().flatten())
        .filter_map(|block| {
            Some((
                block["block_id"].as_str()?.to_string(),
                block["label"].as_str().map(str::to_string),
            ))
        })
        .collect()
}

/// Each block of a session's view, by id and label.
fn view_blocks(view: &Value) -> Vec<(String, Option<String>)> {
    view["windows"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(window_blocks)
        .collect()
}

/// `args` as words a refusal may carry: a JSON argument cut short.
fn words(args: &[&str]) -> String {
    args.iter()
        .map(|arg| {
            if arg.starts_with('{') || arg.len() > 80 {
                "…".to_string()
            } else {
                (*arg).to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

impl Terminal for Rex {
    fn open(
        &self,
        place: &PlaceRef,
        folder: &str,
        command: &AgentCommand,
        identity: &Identity,
        at: &OpenIn,
        session: Option<&str>,
    ) -> Opened {
        let refused = |reason: String| Opened::Refused { reason };
        let kind = match command.kind.as_deref() {
            Some(kind) if KINDS.contains(&kind) => kind,
            kind => {
                return refused(format!(
                    "{} reports no Program Status the pinned Rex was read to take, so it cannot \
                     run under Rex{}; the programs that run under Rex are those of the kinds {}",
                    command.program,
                    kind.map(|kind| format!(" (its kind is {kind})"))
                        .unwrap_or_default(),
                    KINDS.join(", ")
                ))
            }
        };
        match at {
            OpenIn::Own => {
                if let Err(reason) = self.socket(None) {
                    return refused(reason);
                }
            }
            OpenIn::Session { name } => {
                if let Err(reason) = self.socket(Some(name)) {
                    return refused(reason);
                }
            }
            OpenIn::Beside { pane } => {
                if Block::named(pane).is_none() {
                    return refused(format!(
                        "Rex opens beside a block named rex:<block>@<socket>, and {pane} names none"
                    ));
                }
            }
        }
        let path = command
            .env
            .get("PATH")
            .cloned()
            .or_else(|| std::env::var("PATH").ok())
            .unwrap_or_default();
        let Some(program) = program_path(&command.program, &path) else {
            return refused(format!(
                "the program {} is on no folder of PATH",
                command.program
            ));
        };
        let beside = matches!(at, OpenIn::Beside { .. });
        let name = rex_name(identity);
        let start = Path::new(folder)
            .join("rex")
            .join(digest(&identity.to_string()));
        let trust = command
            .folder_trust
            .clone()
            .filter(|trust| !trust.question.is_empty());
        let mut record = json!({
            "identity": identity,
            "name": name,
            "place": (!beside).then(|| place.location.clone()),
            "start": start,
            "trust": trust,
            "named": session,
            "claimed_by": std::process::id(),
        });
        if let Err(reason) = self.claim(identity, &record) {
            return refused(reason);
        }
        if let Some(name) = session {
            if let Some(holder) = self.carrier(name, identity) {
                let _ = fs::remove_file(self.agent_record(identity));
                return refused(format!(
                    "the live agent {holder} carries the session name {name}"
                ));
            }
        }
        let bin = start.join("bin");
        let link = bin.join(kind);
        let script = start.join("start");
        let written = fs::create_dir_all(&bin)
            .and_then(|()| fs::set_permissions(&start, fs::Permissions::from_mode(0o700)))
            .and_then(|()| match fs::remove_file(&link) {
                Err(error) if error.kind() != std::io::ErrorKind::NotFound => Err(error),
                _ => Ok(()),
            })
            .and_then(|()| std::os::unix::fs::symlink(&program, &link))
            .and_then(|()| {
                let _ = fs::remove_file(&script);
                fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .mode(0o700)
                    .open(&script)
            })
            .and_then(|mut file| {
                file.write_all(self.start_script(&script, &link, command, &bin).as_bytes())
            });
        if let Err(error) = written {
            self.forget(identity);
            return refused(format!("{}: {error}", start.display()));
        }
        let layout = self.block_layout(&name, place, &script);
        let located = match self.new_block(place, at, &layout) {
            Ok(located) => located,
            Err(reason) => {
                self.forget(identity);
                return refused(reason);
            }
        };
        record["block"] = json!(located.block.id);
        record["socket"] = json!(located.block.socket);
        record["session"] = json!(located.session);
        if let Err(reason) = write_json(&self.agent_record(identity), &record) {
            self.abandon(identity, &located);
            return refused(reason);
        }
        let child = match self.ready(&located, trust.as_ref()) {
            Ok(child) => child,
            Err(answer) => {
                self.abandon(identity, &located);
                return answer;
            }
        };
        if let Err(reason) = self.runs(&located, child, &program) {
            self.abandon(identity, &located);
            return refused(reason);
        }
        Opened::Opened {
            carried: false,
            printed: None,
        }
    }

    fn send(&self, agent: &Identity, text: &str) -> Sent {
        let Some(located) = self.located(agent) else {
            return Sent::Refused {
                reason: format!("no agent runs under {agent}"),
            };
        };
        let clear = match self.read_block(&located) {
            Err(reason) => return Sent::Refused { reason },
            Ok(AgentState::Stopped) => {
                return Sent::Refused {
                    reason: format!("no agent runs under {agent}"),
                }
            }
            Ok(state) => state == AgentState::Waiting,
        };
        match self.typed(&located, text, clear) {
            Ok(()) => Sent::Sent,
            Err(reason) => Sent::Refused { reason },
        }
    }

    /// Rex types into a running agent's block, so an agent is kept open
    /// for an answer under it (53, 95, 198, C.5).
    fn sends(&self) -> bool {
        true
    }

    fn state(&self, agent: &Identity) -> AgentState {
        self.agent_state(agent)
    }

    fn send_to(&self, session: &str, text: &str) -> ToSession {
        let Some(located) = self.reach(session) else {
            return ToSession::NotFound;
        };
        let clear = match self.read_block(&located) {
            Err(reason) => return ToSession::Refused { reason },
            Ok(AgentState::Waiting) => true,
            Ok(AgentState::Working) => false,
            Ok(AgentState::Stopped) => return ToSession::NotFound,
            Ok(state) => {
                return ToSession::Refused {
                    reason: format!(
                        "the session {session} reads {state:?}, so nothing was typed into it"
                    ),
                }
            }
        };
        match self.typed(&located, text, clear) {
            Ok(()) => ToSession::Sent,
            Err(reason) => ToSession::Refused { reason },
        }
    }

    fn session_state(&self, session: &str) -> Option<AgentState> {
        match self.block_state(&self.reach(session)?) {
            AgentState::Stopped => None,
            state => Some(state),
        }
    }

    fn agents_in(&self, session: Option<&str>) -> AgentsIn {
        let unread = |reason: String| AgentsIn::Unread { reason };
        let socket = match self.socket(session) {
            Ok(socket) => socket,
            Err(reason) => return unread(reason),
        };
        match self.ask(&socket, &["api", "call", "server.status"]) {
            Call::Answered(_) => {}
            call if call.no_server() => return AgentsIn::Read { agents: vec![] },
            call => return unread(call.answer().err().unwrap_or_default()),
        }
        let sessions = match self.sessions(&socket) {
            Ok(sessions) => sessions,
            Err(reason) => return unread(reason),
        };
        let mut agents = vec![];
        for id in sessions {
            let view = match self.view(&socket, &id) {
                Ok(view) => view,
                Err(reason) => return unread(reason),
            };
            for (block, label) in view_blocks(&view) {
                let Some(label) = label else {
                    continue;
                };
                let located = Located {
                    block: Block {
                        id: block,
                        socket: socket.clone(),
                    },
                    session: Some(id.clone()),
                };
                agents.push(NamedAgent {
                    name: label,
                    at: located.block.session(),
                    state: self.block_state(&located),
                });
            }
        }
        AgentsIn::Read { agents }
    }

    fn screen(&self, agent: &Identity) -> Option<String> {
        self.visible(&self.located(agent)?).ok()
    }

    /// The folder-trust question the open was given, answered from the
    /// block's screen as open answers it, where the screen shows it now
    /// (34).
    fn answer_trust(&self, agent: &Identity) -> bool {
        let Some(record) = read_json(&self.agent_record(agent)) else {
            return false;
        };
        let Some(trust) = serde_json::from_value::<FolderTrust>(record["trust"].clone()).ok()
        else {
            return false;
        };
        let Some(located) = located_in(&record) else {
            return false;
        };
        let Ok(screen) = self.visible(&located) else {
            return false;
        };
        asks(&screen, &trust) && matches!(self.trust(&located, &trust, screen), Trusted::Answered)
    }

    fn close(&self, agent: &Identity) -> Closed {
        let Some(located) = self.located(agent) else {
            if self.agent_state(agent) == AgentState::Stopped {
                self.forget(agent);
                return Closed::Closed;
            }
            return Closed::Refused {
                reason: format!("an open of {agent} is under way"),
            };
        };
        let args = close_args(&located);
        let call = self.ask(
            &located.block.socket,
            &args.iter().map(String::as_str).collect::<Vec<_>>(),
        );
        match call {
            Call::Answered(_) => {}
            call if call.block_gone(&located.block.id) || call.no_server() => {}
            call => {
                return Closed::Refused {
                    reason: call.answer().err().unwrap_or_default(),
                }
            }
        }
        match self.process(&located) {
            Ok(None) => {
                self.forget(agent);
                Closed::Closed
            }
            Ok(Some(_)) => Closed::Refused {
                reason: format!("the Rex block {} still stands", located.block.id),
            },
            Err(reason) => Closed::Refused { reason },
        }
    }

    /// The place's window closed on every server a record names, then the
    /// base's Rex session on each of those servers, or on every server
    /// where the place had no window, each once nothing live and nothing
    /// flume did not open is in it (137, 138; core.18).
    fn tidy(&self, place: &PlaceRef) -> TerminalTidy {
        let mut tidy = TerminalTidy::default();
        let opened = self.opened_blocks();
        let windows = self.window_records(place);
        let servers: BTreeSet<PathBuf> =
            windows.iter().map(|window| window.socket.clone()).collect();
        for record in &windows {
            match self.tidy_window(record, &opened) {
                Ok(Some(closed)) => tidy.closed.push(closed),
                Ok(None) => {}
                Err(kept) => tidy.kept.push(kept),
            }
        }
        for (path, record) in self.agent_records() {
            if record["place"].as_str() == Some(place.location.as_str()) {
                if let Some(start) = record["start"].as_str() {
                    let _ = fs::remove_dir_all(start);
                }
                let _ = fs::remove_file(path);
            }
        }
        for record in self
            .base_records(place)
            .iter()
            .filter(|record| servers.is_empty() || servers.contains(&record.socket))
        {
            match self.tidy_session(record, &self.opened_blocks()) {
                Ok(Some(closed)) => tidy.closed.push(closed),
                Ok(None) => {}
                Err(kept) => tidy.kept.push(kept),
            }
        }
        tidy
    }

    fn kept(&self) -> Vec<String> {
        let opened = self.opened_blocks();
        let mut kept: Vec<String> = self
            .records_of("windows", "window")
            .iter()
            .filter(|record| record.value["tidied"] == Value::Bool(true))
            .filter_map(|record| self.held_window(record, &opened).err())
            .collect();
        kept.extend(
            self.records_of("bases", "session")
                .iter()
                .filter_map(|record| self.held_session(record, &opened).ok().flatten()),
        );
        kept
    }

    /// Rex 0.1.0-preview.14 labels a window and a session, which the
    /// bolt's next unit does; this profile shows nothing (139).
    fn show(&self, _place: &PlaceRef, _clues: &Clues) -> Shown {
        Shown::ShowsNone
    }

    fn shown_at(&self, _place: &PlaceRef, _agent: Option<&Identity>) -> ShownAt {
        ShownAt::default()
    }

    fn session(&self) -> Option<String> {
        self.caller.as_ref().map(Block::session)
    }

    /// Refused for both views, nothing run: the pinned Rex has no terminal
    /// client, only its app shows a session, and telling the app to show
    /// one is the bolt's next unit (187, C.5).
    fn attach(&self, session: Option<&str>, view: View) -> Attached {
        let part = session.unwrap_or(SESSION);
        Attached::Refused {
            reason: match view {
                View::Terminal => format!(
                    "Rex 0.1.0-preview.14 has no terminal client, and only its app shows a \
                     session, so no window shows the Rex session {part}"
                ),
                View::App => format!(
                    "flume does not yet tell the Rex app to show a session, so the app is not \
                     shown the Rex session {part}"
                ),
            },
        }
    }

    /// `server.status` at each part's socket, every part at once, each
    /// bounded at one second: a server that answers runs, none at its
    /// socket does not, and anything else, or a name the profile refuses,
    /// is not read, in its words. Nothing is started (200, 201, C.5).
    fn runs(&self, sessions: &[Option<&str>]) -> Vec<SessionRuns> {
        std::thread::scope(|scope| {
            let asked: Vec<_> = sessions
                .iter()
                .map(|session| {
                    scope.spawn(move || {
                        let name = session.unwrap_or(SESSION).to_string();
                        let socket = match self.socket(*session) {
                            Ok(socket) => socket,
                            Err(reason) => return SessionRuns::Unread { reason },
                        };
                        match self.call(&socket, &["api", "call", "server.status"], RUNS_BOUND) {
                            Call::Answered(_) => SessionRuns::Runs { name },
                            call if call.no_server() => SessionRuns::NotRunning { name },
                            call => SessionRuns::Unread {
                                reason: call.answer().err().unwrap_or_default(),
                            },
                        }
                    })
                })
                .collect();
            asked
                .into_iter()
                .map(|asked| {
                    asked.join().unwrap_or_else(|_| SessionRuns::Unread {
                        reason: "the reading of Rex failed".into(),
                    })
                })
                .collect()
        })
    }
}

impl Rex {
    /// The place's window `record` names closed, unless a block flume did
    /// not open stands in it: what was closed, none where it was gone, or
    /// what was kept and what holds it (137, 138).
    fn tidy_window(
        &self,
        record: &Record,
        opened: &BTreeSet<(PathBuf, String)>,
    ) -> Result<Option<String>, String> {
        let label = record.label().to_string();
        if let Err(kept) = self.held_window(record, opened) {
            let mut value = record.value.clone();
            value["tidied"] = Value::Bool(true);
            let _ = write_json(&record.path, &value);
            return Err(kept);
        }
        let Some(session) = record.value["session"].as_str() else {
            let _ = fs::remove_file(&record.path);
            return Ok(None);
        };
        let args = json!({"window_id": record.id}).to_string();
        match self.ask(
            &record.socket,
            &["api", "call", "-s", session, "session.close_window", &args],
        ) {
            Call::Answered(_) => {
                let _ = fs::remove_file(&record.path);
                Ok(Some(format!("the window {label}")))
            }
            call if call.no_server() => {
                let _ = fs::remove_file(&record.path);
                Ok(None)
            }
            Call::Refused(words) if words.contains("not found") => {
                let _ = fs::remove_file(&record.path);
                Ok(None)
            }
            call => Err(format!(
                "the window {label} ({}): {}",
                record.id,
                call.answer().err().unwrap_or_default()
            )),
        }
    }

    /// The window `record` names, named with what holds it, where a block
    /// flume did not open stands in it; or why it could not be read.
    fn held_window(
        &self,
        record: &Record,
        opened: &BTreeSet<(PathBuf, String)>,
    ) -> Result<(), String> {
        let label = record.label();
        let Some(session) = record.value["session"].as_str() else {
            return Ok(());
        };
        let view = match self.ask(
            &record.socket,
            &["api", "call", "-s", session, "session.view"],
        ) {
            Call::Answered(view) => view,
            call if call.no_server() => return Ok(()),
            Call::Refused(words) if words.contains("not found") => return Ok(()),
            call => {
                return Err(format!(
                    "the window {label} ({}), which Rex could not be read for: {}",
                    record.id,
                    call.answer().err().unwrap_or_default()
                ))
            }
        };
        let foreign: Vec<String> = view["windows"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|window| window["window_id"].as_str() == Some(record.id.as_str()))
            .flat_map(window_blocks)
            .filter(|(id, _)| !opened.contains(&(record.socket.clone(), id.clone())))
            .map(|(id, _)| id)
            .collect();
        if foreign.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "the window {label} ({}), held by the block {} flume did not open",
                record.id,
                foreign.join(", ")
            ))
        }
    }

    /// The base's Rex session `record` names destroyed once it holds no
    /// window but those of live places the profile made and no block it
    /// did not open: what was closed, none where it stands for a live place
    /// or is gone, or what was kept and what holds it (137, 138).
    fn tidy_session(
        &self,
        record: &Record,
        opened: &BTreeSet<(PathBuf, String)>,
    ) -> Result<Option<String>, String> {
        let label = record.label().to_string();
        match self.held_session(record, opened) {
            Err(reason) => return Err(reason),
            Ok(Some(kept)) => return Err(kept),
            Ok(None) => {}
        }
        if self.live_windows(record)? {
            return Ok(None);
        }
        let args = json!({"session_id": record.id}).to_string();
        match self.ask(&record.socket, &["api", "call", "session.destroy", &args]) {
            Call::Answered(_) => {
                let _ = fs::remove_file(&record.path);
                Ok(Some(format!("the Rex session {label}")))
            }
            call if call.no_server() => {
                let _ = fs::remove_file(&record.path);
                Ok(None)
            }
            Call::Refused(words) if words.contains("not found") => {
                let _ = fs::remove_file(&record.path);
                Ok(None)
            }
            call => Err(format!(
                "the Rex session {label} ({}): {}",
                record.id,
                call.answer().err().unwrap_or_default()
            )),
        }
    }

    /// The window ids of live places the profile made on the server of
    /// `record`, as their records name them.
    fn place_windows(&self, socket: &Path) -> BTreeSet<String> {
        self.records_of("windows", "window")
            .into_iter()
            .filter(|window| window.socket == socket && window.value["tidied"] != Value::Bool(true))
            .map(|window| window.id)
            .collect()
    }

    /// Whether the base's Rex session holds a window of a live place the
    /// profile made; false where it is gone.
    fn live_windows(&self, record: &Record) -> Result<bool, String> {
        let view = match self.ask(
            &record.socket,
            &["api", "call", "-s", &record.id, "session.view"],
        ) {
            Call::Answered(view) => view,
            call if call.no_server() => return Ok(false),
            Call::Refused(words) if words.contains("not found") => return Ok(false),
            call => return Err(call.answer().err().unwrap_or_default()),
        };
        let places = self.place_windows(&record.socket);
        Ok(view["windows"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|window| window["window_id"].as_str())
            .any(|window| places.contains(window)))
    }

    /// The base's Rex session `record` names, named with what holds it,
    /// where it holds no window of a live place and a block flume did not
    /// open stands in it; none otherwise; or why it could not be read.
    fn held_session(
        &self,
        record: &Record,
        opened: &BTreeSet<(PathBuf, String)>,
    ) -> Result<Option<String>, String> {
        let label = record.label();
        let view = match self.ask(
            &record.socket,
            &["api", "call", "-s", &record.id, "session.view"],
        ) {
            Call::Answered(view) => view,
            call if call.no_server() => return Ok(None),
            Call::Refused(words) if words.contains("not found") => return Ok(None),
            call => {
                return Err(format!(
                    "the Rex session {label} ({}), which Rex could not be read for: {}",
                    record.id,
                    call.answer().err().unwrap_or_default()
                ))
            }
        };
        let places = self.place_windows(&record.socket);
        let windows: Vec<&Value> = view["windows"].as_array().into_iter().flatten().collect();
        if windows
            .iter()
            .filter_map(|window| window["window_id"].as_str())
            .any(|window| places.contains(window))
        {
            return Ok(None);
        }
        let foreign: Vec<String> = windows
            .into_iter()
            .flat_map(window_blocks)
            .filter(|(id, _)| !opened.contains(&(record.socket.clone(), id.clone())))
            .map(|(id, _)| id)
            .collect();
        Ok((!foreign.is_empty()).then(|| {
            format!(
                "the Rex session {label} ({}), held by the block {} flume did not open",
                record.id,
                foreign.join(", ")
            )
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The profile over records in a scratch folder of its own, short
    /// enough for its sockets, its rex a script that marks it was run;
    /// and the folder and that mark.
    fn over_records(name: &str) -> (Rex, PathBuf, PathBuf) {
        let scratch = PathBuf::from("/tmp").join(format!("frx-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&scratch);
        fs::create_dir_all(&scratch).expect("a scratch folder");
        let program = scratch.join("rex");
        let ran = scratch.join("ran");
        fs::write(
            &program,
            format!("#!/bin/sh\necho \"$@\" >> '{}'\nexit 1\n", ran.display()),
        )
        .expect("the stand-in rex");
        fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("executable");
        let rex = Rex::new(scratch.join("terminal/rex")).with_program(&program);
        (rex, scratch, ran)
    }

    fn place() -> PlaceRef {
        PlaceRef {
            repository: "/r/acme".into(),
            location: "/r/places/fix--one".into(),
            branch: "fix/one".into(),
            base: "main".into(),
        }
    }

    fn identity() -> Identity {
        Identity {
            repository: "0123456789ab".into(),
            place: "fix--one".into(),
            stage: "code".into(),
            start: 1,
        }
    }

    fn command(kind: &str) -> AgentCommand {
        AgentCommand {
            program: "/bin/sh".into(),
            args: vec![],
            env: BTreeMap::new(),
            prompt_args: vec![],
            folder_trust: None,
            kind: Some(kind.into()),
            files: BTreeMap::new(),
        }
    }

    #[test]
    fn a_session_name_the_socket_cannot_hold_is_refused() {
        // 107, C.5: a session's socket and its state folder under the
        // records' folder, flume's own named `flume`; a name that puts the
        // socket past 104 bytes, or outside the folder, refused naming it,
        // and an open in it refused with no server started.
        let (rex, scratch, ran) = over_records("names");
        let records = scratch.join("terminal/rex");
        assert_eq!(rex.socket(None), Ok(records.join("flume.sock")));
        assert_eq!(rex.socket(Some("work")), Ok(records.join("work.sock")));
        assert_eq!(
            rex.socket(Some("work"))
                .map(|socket| socket.with_extension("state")),
            Ok(records.join("work.state"))
        );
        let room = SOCKET_LIMIT - records.join(".sock").as_os_str().len();
        let longest = "w".repeat(room);
        assert!(rex.socket(Some(&longest)).is_ok(), "a name that fits");
        let long = "w".repeat(room + 1);
        for name in [long.as_str(), "", ".", "..", "../x", "a/b", "a\0b"] {
            let refused = rex.socket(Some(name)).expect_err(name);
            assert!(
                refused.contains(&format!("{name:?}")) || refused.contains(name),
                "{refused}"
            );
            let opened = rex.open(
                &place(),
                &scratch.join("state").to_string_lossy(),
                &command("claude"),
                &identity(),
                &OpenIn::Session { name: name.into() },
                None,
            );
            assert!(
                matches!(&opened, Opened::Refused { reason } if *reason == refused),
                "{opened:?}"
            );
        }
        assert!(!ran.exists(), "a rex command was run");
        assert!(
            !records.join("agents").exists(),
            "an identity was claimed for a refused open"
        );
        let _ = fs::remove_dir_all(scratch);
    }

    #[test]
    fn a_long_places_rex_name_is_128_characters_ending_in_its_digest() {
        // C.5: lowercased, each character outside [a-z0-9_-] made `-`,
        // cut to 119, then `-` and eight hex digits of the identity's
        // SHA-256.
        let who = Identity {
            place: format!("Change--A/Long.Name{}", "x".repeat(150)),
            ..identity()
        };
        let name = rex_name(&who);
        assert_eq!(name.len(), 128, "{name}");
        assert!(name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-'));
        assert!(name.starts_with("f-change--a-long-namex"), "{name}");
        let full = Sha256::digest(who.to_string().as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        assert!(name.ends_with(&format!("-{}", &full[..8])), "{name}");
        assert_eq!(
            rex_name(&identity()),
            format!("f-fix--one-code-1-{}", &full_of(&identity())[..8])
        );
        assert_eq!(Rex::name(&who), name);
    }

    fn full_of(identity: &Identity) -> String {
        Sha256::digest(identity.to_string().as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    }

    #[test]
    fn a_command_run_in_a_rex_block_names_its_block() {
        // 7, C.5: `REX_BLOCK` on the server `REX_SERVER`, both set, as
        // `rex:<block>@<socket>`, read back as that block; none where
        // either is missing or the server names no socket.
        let block = caller_of(
            Some("block:01a1d7f4-c986"),
            Some("unix:///tmp/x1/terminal/rex/work.sock"),
        )
        .expect("the block");
        assert_eq!(
            block.session(),
            "rex:block:01a1d7f4-c986@/tmp/x1/terminal/rex/work.sock"
        );
        assert_eq!(Block::named(&block.session()), Some(block.clone()));
        assert_eq!(
            caller_of(Some("block:1"), Some("/tmp/s.sock")).map(|block| block.session()),
            Some("rex:block:1@/tmp/s.sock".into())
        );
        for (id, server) in [
            (None, Some("unix:///tmp/s.sock")),
            (Some("block:1"), None),
            (Some(""), Some("unix:///tmp/s.sock")),
            (Some("block:1"), Some("tcp://host:7777")),
        ] {
            assert_eq!(caller_of(id, server), None, "{id:?} {server:?}");
        }
        let (rex, scratch, ran) = over_records("caller");
        assert_eq!(rex.session(), None);
        let rex = rex.with_caller(Some(block.clone()));
        assert_eq!(rex.session(), Some(block.session()));
        assert!(!ran.exists(), "a rex command was run");
        let _ = fs::remove_dir_all(scratch);
    }

    #[test]
    fn a_herdr_sessions_name_is_none_of_rexs() {
        // 7, 43, 190: a name in another terminal's form, or one no open
        // gave an agent, is none of Rex's: not found and no state, with no
        // call made, so the core asks the next terminal.
        let (rex, scratch, ran) = over_records("other");
        for name in [
            "herdr:w1-p2@/tmp/s",
            "rex:@/tmp/s",
            "rex:block:1@relative.sock",
            "east/ops",
            "",
        ] {
            assert_eq!(rex.send_to(name, "a notice"), ToSession::NotFound, "{name}");
            assert_eq!(rex.session_state(name), None, "{name}");
            assert_eq!(Block::named(name), None, "{name}");
        }
        assert!(!ran.exists(), "a rex command was run");
        let _ = fs::remove_dir_all(scratch);
    }

    #[test]
    fn the_start_script_holds_every_variable_and_no_argument_does() {
        // 29, 32, C.5; core.30: the block runs the script alone, which
        // holds every variable; run, it removes itself and execs the
        // program with Rex's terminal variables from its own environment,
        // the login environment with the agent's over it, PATH led by its
        // bin, and nothing else, each value and argument whole.
        let (rex, scratch, _) = over_records("script");
        let tricky = "it's \"$HOME\" `x`\nnext \u{e9}\u{1f600} \\ $";
        let rex = rex.with_login(&[
            ("HOME".into(), "/home/someone".into()),
            ("LOGIN_ONLY".into(), "from the login".into()),
            ("SHARED".into(), "the login's".into()),
            ("PWD".into(), "/elsewhere".into()),
            ("REX_BLOCK".into(), "block:a-stale-one".into()),
        ]);
        let start = scratch.join("state/rex/0123");
        let bin = start.join("bin");
        fs::create_dir_all(&bin).expect("the start's bin");
        let out = scratch.join("out");
        fs::create_dir_all(&out).expect("the program's output folder");
        let program = scratch.join("program");
        fs::write(
            &program,
            format!(
                "#!/bin/sh\nd='{}'\nfor v in A_VALUE LOGIN_ONLY SHARED TERM REX_BLOCK COLORTERM \
                 OUTSIDE PWD_SEEN PATH HOME; do eval \"if [ -n \\\"\\${{$v+x}}\\\" ]; then printf '%s' \\\"\\$$v\\\" > \\\"$d/$v\\\"; fi\"; done\n\
                 printf '%s\\0' \"$@\" > \"$d/args\"\n",
                out.display()
            ),
        )
        .expect("the program");
        fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("executable");
        let link = bin.join("claude");
        std::os::unix::fs::symlink(&program, &link).expect("the link");
        let mut command = command("claude");
        command.program = program.to_string_lossy().into();
        command.env = BTreeMap::from([
            ("A_VALUE".to_string(), tricky.to_string()),
            ("SHARED".to_string(), "the agent's".to_string()),
            ("PATH".to_string(), "/usr/bin:/bin".to_string()),
        ]);
        command.args = vec!["--flag".into(), tricky.into(), String::new()];
        let script = start.join("start");
        let text = rex.start_script(&script, &link, &command, &bin);
        assert!(text.contains("A_VALUE="), "{text}");
        let layout = rex.block_layout("f-fix--one-code-1-01234567", &place(), &script);
        assert_eq!(
            layout["block"]["options"]["command"],
            json!([script.to_string_lossy()])
        );
        assert!(
            !layout.to_string().contains("A_VALUE")
                && !layout.to_string().contains("from the login"),
            "a variable stands in the block's arguments: {layout}"
        );
        fs::write(&script, &text).expect("the script");
        fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).expect("executable");
        let status = Command::new(&script)
            .env_clear()
            .env("TERM", "xterm-256color")
            .env("REX_BLOCK", "block:01a1")
            .env("OUTSIDE", "from the server")
            .env("PATH", "/usr/bin:/bin")
            .status()
            .expect("the script runs");
        assert!(status.success());
        assert!(!script.exists(), "the script did not remove itself");
        let read = |name: &str| fs::read_to_string(out.join(name)).ok();
        assert_eq!(read("A_VALUE").as_deref(), Some(tricky));
        assert_eq!(read("LOGIN_ONLY").as_deref(), Some("from the login"));
        assert_eq!(read("SHARED").as_deref(), Some("the agent's"));
        assert_eq!(read("TERM").as_deref(), Some("xterm-256color"));
        assert_eq!(read("REX_BLOCK").as_deref(), Some("block:01a1"));
        assert_eq!(read("HOME").as_deref(), Some("/home/someone"));
        assert_eq!(read("COLORTERM"), None, "a kept variable unset was given");
        assert_eq!(
            read("OUTSIDE"),
            None,
            "the server's environment reached the program"
        );
        assert_eq!(
            read("PATH"),
            Some(format!("{}:/usr/bin:/bin", bin.display()))
        );
        assert_eq!(
            read("args").as_deref(),
            Some(format!("--flag\0{tricky}\0\0").as_str())
        );
        let _ = fs::remove_dir_all(scratch);
    }

    #[test]
    fn the_state_is_read_from_the_programs_newest_report() {
        // 96, C.5: stopped where the program exited; otherwise the newest
        // record by updated_at whose owner is the block's child, a record
        // of another owner never; working, waiting for idle, done and
        // error, asking for blocked, and unknown for none.
        let process = json!({"last_exit": null, "child": {"pid": 41, "path": LOGIN}});
        let status = |records: Value| json!({ "records": records });
        let record = |state: &str, owner: u64, at: &str| json!({"state": state, "owner": {"pid": owner, "name": "login"}, "updated_at": at});
        for (state, read) in [
            ("working", AgentState::Working),
            ("idle", AgentState::Waiting),
            ("done", AgentState::Waiting),
            ("blocked", AgentState::Asking),
            ("error", AgentState::Waiting),
            ("other", AgentState::Unknown),
        ] {
            assert_eq!(
                read_state(
                    &process,
                    &status(json!([record(state, 41, "2026-11-13T21:48:26-05:00")]))
                ),
                read,
                "{state}"
            );
        }
        assert_eq!(
            read_state(&process, &status(json!([]))),
            AgentState::Unknown
        );
        assert_eq!(
            read_state(
                &process,
                &status(json!([record("working", 99, "2026-11-13T21:48:26-05:00")]))
            ),
            AgentState::Unknown,
            "another owner's record"
        );
        let newest = status(json!([
            record("done", 41, "2026-11-14T02:48:27Z"),
            record("working", 41, "2026-11-13T21:48:26.5-05:00"),
            record("blocked", 99, "2026-11-14T03:00:00Z"),
        ]));
        assert_eq!(read_state(&process, &newest), AgentState::Waiting);
        let exited = json!({"last_exit": {"code": 0}, "child": {"pid": 41}});
        assert_eq!(read_state(&exited, &newest), AgentState::Stopped);
        assert_eq!(
            read_state(&json!({"last_exit": null}), &newest),
            AgentState::Unknown
        );
        let gone = Call::Refused("block target \"block:1\" not found".into());
        assert!(gone.block_gone("block:1") && !gone.block_gone("block:2"));
        let scoped = Call::Refused(
            "block target \"block:1\" not found: session scope: session \"s\" not found".into(),
        );
        assert!(scoped.block_gone("block:1"));
        let none = Call::Refused(
            "no Rex server is running at /tmp/x/terminal/rex/w.sock, and autostart is disabled"
                .into(),
        );
        assert!(none.no_server() && !none.block_gone("block:1"));
        assert!(!Call::Unanswered("rex did not answer".into()).no_server());
    }

    #[test]
    fn a_text_is_cut_between_characters_and_a_screen_is_its_last_rows() {
        // 95, 136, C.5: pieces of at most 60,000 bytes, each whole
        // characters, together the text in order; the screen the last
        // lines of what capture gives, as many as the block has rows.
        let text: String = "a\u{e9}\u{1f600}\n".repeat(15_000);
        assert!(text.len() > 100_000);
        let cut = pieces(&text, PIECE);
        assert!(cut.len() >= 2);
        assert!(cut
            .iter()
            .all(|piece| piece.len() <= PIECE && !piece.is_empty()));
        assert_eq!(cut.concat(), text);
        assert_eq!(pieces("", PIECE), Vec::<&str>::new());
        assert_eq!(pieces("short", PIECE), vec!["short"]);
        assert_eq!(
            pieces("\u{e9}\u{e9}\u{e9}", 5),
            vec!["\u{e9}\u{e9}", "\u{e9}"]
        );
        assert_eq!(last_rows("one\ntwo\nthree\nfour\n", 2), "three\nfour");
        assert_eq!(last_rows("one\ntwo", 24), "one\ntwo");
        assert_eq!(last_rows("", 24), "");
    }

    #[test]
    fn a_kind_that_reports_nothing_to_rex_is_refused() {
        // 32, C.5: a program the pinned Rex was not read to take Program
        // Status from refused naming it and claude, before any call and
        // with nothing claimed.
        let (rex, scratch, ran) = over_records("kind");
        let mut codex = command("codex");
        codex.program = "/opt/agents/codex".into();
        let mut none = command("claude");
        none.kind = None;
        for asked in [codex, none] {
            let opened = rex.open(
                &place(),
                &scratch.join("state").to_string_lossy(),
                &asked,
                &identity(),
                &OpenIn::Own,
                None,
            );
            let Opened::Refused { reason } = opened else {
                panic!("open gave {opened:?}");
            };
            assert!(
                reason.contains(&asked.program) && reason.contains("claude"),
                "{reason}"
            );
            if let Some(kind) = &asked.kind {
                assert!(reason.contains(kind.as_str()), "{reason}");
            }
        }
        assert!(!ran.exists(), "a rex command was run");
        assert!(!scratch.join("terminal").exists(), "something was recorded");
        let _ = fs::remove_dir_all(scratch);
    }

    #[test]
    fn attach_refuses_both_views_and_show_shows_none() {
        // 122, 139, 187, C.5: no terminal client, and the app's view the
        // bolt's next unit; show shows none and says nowhere, and setup
        // gives nothing; none of them calls Rex.
        let (rex, scratch, ran) = over_records("attach");
        let Attached::Refused { reason } = rex.attach(None, View::Terminal) else {
            panic!("a window was attached");
        };
        assert!(
            reason.contains("no terminal client")
                && reason.contains("only its app shows a session"),
            "{reason}"
        );
        assert!(matches!(
            rex.attach(Some("work"), View::App),
            Attached::Refused { reason } if reason.contains("work")
        ));
        let clues = Clues {
            stage: "code".into(),
            state: "working".into(),
            agent: Some(identity()),
            base: flume_seams::BaseCount::default(),
            line: "code · working".into(),
        };
        assert_eq!(rex.show(&place(), &clues), Shown::ShowsNone);
        assert_eq!(
            rex.shown_at(&place(), Some(&identity())),
            ShownAt::default()
        );
        assert_eq!(rex.setup(), Vec::<String>::new());
        assert!(!ran.exists(), "a rex command was run");
        let _ = fs::remove_dir_all(scratch);
    }
}

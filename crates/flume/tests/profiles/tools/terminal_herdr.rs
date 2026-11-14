//! The terminal profile over herdr (C.5; design 12; records 0016, 0018)
//! against the pinned herdr, on a server each test starts headless on a
//! scratch socket with a scratch home and a shell that reads no startup
//! file of the host's, so no herdr session of the host's is touched, no
//! `HERDR_*` of the caller's reaches it, and no agent program of the
//! host's is on a pane's `PATH`. The agent is a stand-in: a shell script
//! named `claude`, which herdr's agent start runs by that kind, which
//! reports its own state to herdr, keeps what it received, clears its
//! input on ctrl+c as an agent does, and, as its environment asks, shows
//! the folder-trust question, a warning, or a dialog a person answers,
//! first.
//!
//! The terminal seam's scenarios run against the profile; then the
//! folder-trust question answered (S39), the bypass warning held (S40), a
//! program that never comes up (S45), a pane found by name after a move
//! (S46), the environment from the pane, the program the settings name, a
//! name taken, the last place on a base (S47), two bases, a person's pane
//! (S48), titles that follow the stage (S49), a conversation beside a
//! named pane, a launch from a herdr pane, herdr's readings, a prompt
//! left typed (S43), a pane herdr could not bring back, long
//! instructions to a shell not yet at its prompt, a start the terminal
//! cannot yet take, a shell never at its prompt, a herdr call that never
//! answers, a quiet pane not ended,
//! a finished turn nobody looked at (S42), a launch and a retry each
//! sending its agent's prompt with no settle, a dialog that holds a start
//! waiting on a person, and an agent that began after its start was read
//! never begun, which a retry leaves be; a nix-wrapped Claude Code
//! started from the agent profile, instructions of many lines that herdr
//! refuses as an argument started from the agent profile by a file, a
//! role run's pane reached by its
//! address after a move, and a recipient on another server reached
//! through its own socket; what flume's herdr plugin's startup writes
//! herdr's configuration lacks, given by status while its server runs;
//! the pane's row following its place's line at a settle; and flume
//! unattended by the host's watch alone, whatever the plugin's state, no
//! herdr command run to decide it. Under a scratch
//! home of herdr's named sessions: two
//! streams' sessions (S82), a session name herdr refuses, a place tidied
//! in two sessions, records written before they were kept by server, and
//! attach: a stream's session started for a window's command, a client
//! that command runs, the own session by its name or refused by its
//! socket, a name herdr refuses, and herdr's app refused (187); and runs:
//! a session not listed left so, a running session and the own session,
//! a herdr that does not answer within the bound, and an own server held
//! by its socket (201); and agents_in: a standing agent found by its name
//! and reached, a session that does not run, an agent with no name, the
//! agent gone, and a herdr that refuses or does not answer (132, C.5).

use crate::common::*;
use flume::host::config::HostConfig;
use flume::host::RealHost;
use flume::profiles::store::state_dir::StateDir;
use flume::profiles::terminal::herdr::{
    herdr_name, Herdr, Pane, Server, NAME, SESSION, SESSION_HOME, START_TIMEOUT,
};
use flume_conformance::engine::world::{kind, stage, World};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::terminal::{self, TerminalHarness};
use flume_core::{Answer, Engine, Retry};
use flume_fakes::FakeChange;
use flume_seams::*;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// The stand-in agent, run as `claude`: herdr takes it for a Claude Code
/// agent by its kind, and it reports its state to herdr with the pane's
/// own `HERDR_*` left out, as herdr's reading of the screen would give
/// it, so herdr lists no agent once it exits.
/// Once up, it reads ctrl+c as a key, as Claude Code does, clearing what
/// the line held before it, never as a signal the shell would take while
/// it reads the line that follows.
const STAND_IN: &str = r#"#!/bin/sh
log="$STAND_IN_LOG/$HERDR_PANE_ID"
mkdir -p "$log"
printf '%s\0' "$@" > "$log/argv"
printf '%s\n' "$0" > "$log/program"
if [ -n "$STAND_IN_VALUE" ]; then printf '%s\n' "$STAND_IN_VALUE" > "$log/env"; fi
pane="$HERDR_PANE_ID"
report() {
  env -u HERDR_PANE_ID -u HERDR_TAB_ID -u HERDR_WORKSPACE_ID -u HERDR_ENV \
    "$HERDR_BIN_PATH" pane report-agent "$pane" --source flume-test --agent claude --state "$1" >/dev/null 2>&1
}
trap 'echo cleared >> "$log/events"' INT
if [ -n "$STAND_IN_TRUST" ]; then
  # Claude Code's folder-trust dialog: "No, exit" first and in focus, the
  # focus moved by the arrow keys and confirmed by enter, one key at a
  # time. STAND_IN_TRUST=late tells herdr it is held at a question and
  # draws the choices only a while later; =ready tells herdr it is ready
  # and draws the dialog a while later, =after only once open has
  # returned; =silent draws the dialog and
  # tells herdr nothing, as a dialog herdr does not know.
  focus=no
  draw() {
    printf '\033[2J\033[H Accessing workspace:\n Quick safety check\n'
    if [ "$focus" = no ]; then
      printf ' \342\235\257 No, exit\n   Yes, I trust this folder\n'
    else
      printf '   No, exit\n \342\235\257 Yes, I trust this folder\n'
    fi
  }
  case $STAND_IN_TRUST in
    late) printf '\033[2J\033[H Accessing workspace:\n'; report blocked; sleep 1; draw ;;
    ready) report idle; sleep 1; draw ;;
    after) report idle; sleep 12; draw ;;
    silent) draw ;;
    *) draw; report blocked ;;
  esac
  stty -icanon -echo min 1 2>/dev/null
  esc=$(printf '\033')
  while :; do
    key=$(dd bs=1 count=1 2>/dev/null)
    case $key in
      "$esc")
        case $(dd bs=1 count=2 2>/dev/null) in
          '[B'|OB) focus=yes ;;
          '[A'|OA) focus=no ;;
        esac
        draw ;;
      '')
        if [ "$focus" = yes ]; then printf '\033[2J\033[H'; echo trusted >> "$log/events"; break; fi
        echo refused >> "$log/events"; exit 1 ;;
    esac
  done
  stty icanon echo 2>/dev/null
fi
if [ -n "$STAND_IN_WARNING" ]; then
  printf '%s\n' "$STAND_IN_WARNING"
  report blocked
  while :; do sleep 1; done
fi
if [ -n "$STAND_IN_DIALOG" ]; then
  # A dialog of the program's own before it starts, as Claude Code's for a
  # repository's MCP servers, which a person answers with enter. It tells
  # herdr it is held at a question, or, with STAND_IN_DIALOG_SEEN=no, that
  # it is ready, as a dialog herdr does not know.
  printf '\033[2J\033[H%s\n' "$STAND_IN_DIALOG"
  if [ "$STAND_IN_DIALOG_SEEN" = no ]; then report idle; else report blocked; fi
  stty -icanon -echo min 1 2>/dev/null
  while :; do
    key=$(dd bs=1 count=1 2>/dev/null)
    if [ -z "$key" ]; then printf '\033[2J\033[H'; echo answered >> "$log/events"; break; fi
  done
  stty icanon echo 2>/dev/null
fi
: > "$log/started"
report idle
if [ -n "$STAND_IN_CAT" ]; then
  # Once the test says go, after open has read the program, a pane
  # running `cat` in raw mode, so every key a relay types reaches it as
  # bytes, a ctrl+c or a ctrl+d among them, and none ends it.
  while [ ! -e "$log/go" ]; do sleep 0.1; done
  stty raw -echo 2>/dev/null
  exec cat > "$log/cat"
fi
ctrl_c=$(printf '\003')
stty -isig 2>/dev/null
while :; do
  IFS= read -r line
  case $? in
    0)
      while :; do
        case $line in
          *"$ctrl_c"*) echo cleared >> "$log/events"; line=${line#*"$ctrl_c"} ;;
          *) break ;;
        esac
      done
      printf '%s\n' "$line" >> "$log/got" ;;
    1) exit 0 ;;
  esac
done
"#;

/// The folder-trust question as the stand-in shows it, and its answer.
const TRUST: &str = "Yes, I trust this folder";

/// The tight bound the tests give herdr's calls: herdr answers a local
/// call in milliseconds, and a frozen server is read within it.
const BOUND: Duration = Duration::from_secs(5);

/// herdr's shortest bound on an agent's start, for a program that never
/// comes up.
const SHORT_START: Duration = Duration::from_millis(3001);

/// The bound on an agent's start the seam's scenarios give herdr: room
/// for the stand-in on a loaded host, short for a program never up.
const HARNESS_START: Duration = Duration::from_secs(20);

/// How many short links this run has made, which names the next.
static LINKS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// A folder short enough for a link whose socket path fits a socket's
/// limit: the temporary folder, which a sandboxed run may write and which
/// sluice keeps short, or `/tmp` where the temporary folder is long.
fn short_folder() -> PathBuf {
    let temporary = std::env::temp_dir();
    if temporary.as_os_str().len() <= 80 {
        temporary
    } else {
        PathBuf::from("/tmp")
    }
}

/// A herdr server of the test's own, headless on a scratch socket.
struct TestServer {
    scratch: Scratch,
    socket: PathBuf,
    /// A short link to the scratch, where its path is too long for a
    /// socket's.
    link: Option<PathBuf>,
    child: Option<Child>,
}

/// The pinned herdr, by its own path, whose folder holds nothing else.
fn herdr_program() -> PathBuf {
    let path = std::env::var("PATH").unwrap_or_default();
    path.split(':')
        .map(|dir| Path::new(dir).join("herdr"))
        .find(|candidate| candidate.is_file())
        .and_then(|found| found.canonicalize().ok())
        .expect("the pinned herdr on PATH")
}

impl TestServer {
    fn start(label: &str) -> TestServer {
        let scratch = Scratch::new(label);
        for dir in ["home", "bin", "other", "zdot", "log", "records", "state"] {
            fs::create_dir_all(scratch.path(dir)).expect("a scratch directory");
        }
        let executable = |path: PathBuf, text: &str| {
            fs::write(&path, text).expect("a script written");
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
                .expect("the script made executable");
        };
        executable(scratch.path("bin/claude"), STAND_IN);
        executable(scratch.path("other/claude"), STAND_IN);
        // A shell that reads no startup file of the host's, so the pane's
        // `PATH` is the one flume sets.
        executable(scratch.path("bin/shell"), "#!/bin/sh\nexec /bin/sh -i\n");
        // Startup files of each shell that put another `claude` first.
        let first = format!(
            "PATH={}:$PATH\nexport PATH\n",
            scratch.path("other").display()
        );
        executable(scratch.path("bin/rc"), &first);
        for file in [".zshenv", ".zshrc", ".zprofile", ".bashrc", ".profile"] {
            fs::write(scratch.path("zdot").join(file), &first).expect("a startup file");
        }
        // A program the settings name that runs another in its place.
        executable(
            scratch.path("bin/wrapper"),
            &format!(
                "#!/bin/sh\nexec {} \"$@\"\n",
                scratch.path("other/claude").display()
            ),
        );
        let (socket, link) = if scratch.root.as_os_str().len() > 90 {
            let link = short_folder().join(format!(
                "h{}-{}",
                std::process::id(),
                LINKS.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            ));
            let _ = fs::remove_file(&link);
            std::os::unix::fs::symlink(&scratch.root, &link).unwrap_or_else(|error| {
                panic!(
                    "the server's socket needs a short link, its scratch {} being {} bytes, \
                     and {} cannot be made: {error}",
                    scratch.root.display(),
                    scratch.root.as_os_str().len(),
                    link.display()
                )
            });
            (link.join("s"), Some(link))
        } else {
            (scratch.path("s"), None)
        };
        let mut server = TestServer {
            scratch,
            socket,
            link,
            child: None,
        };
        server.run();
        // The operator's own workspace holds the focus, as on a host
        // where someone works, so no pane of flume's is looked at.
        server.herdr(&["workspace", "create", "--label", "home", "--focus"]);
        server
    }

    /// Starts the server, or starts it again on the same socket and home,
    /// with no folder on its `PATH` but herdr's own and the system's.
    fn run(&mut self) {
        let program = herdr_program();
        let path = format!(
            "{}:/usr/bin:/bin",
            program.parent().expect("herdr's folder").display()
        );
        let child = Command::new(&program)
            .arg("server")
            .env_clear()
            .env("PATH", path)
            .env("HOME", self.scratch.path("home"))
            .env("SHELL", self.scratch.path("bin/shell"))
            .env("TERM", "xterm-256color")
            // A shell that reads its input as UTF-8, as a host user's does.
            .env("LANG", "en_US.UTF-8")
            .env("HERDR_SOCKET_PATH", &self.socket)
            .current_dir(self.scratch.path("home"))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("herdr's server starts");
        self.child = Some(child);
        let deadline = Instant::now() + Duration::from_secs(20);
        while self.try_herdr(&["workspace", "list"]).is_err() {
            assert!(Instant::now() < deadline, "herdr's server did not answer");
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// Stops the server through its socket, its panes with it.
    fn stop(&mut self) {
        let _ = self.try_herdr(&["server", "stop"]);
        if let Some(mut child) = self.child.take() {
            let deadline = Instant::now() + Duration::from_secs(10);
            while child.try_wait().ok().flatten().is_none() && Instant::now() < deadline {
                std::thread::sleep(Duration::from_millis(100));
            }
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    /// The server stops answering, as a server that hangs does.
    fn freeze(&self) {
        self.signal(libc::SIGSTOP);
    }

    fn thaw(&self) {
        self.signal(libc::SIGCONT);
    }

    fn signal(&self, signal: libc::c_int) {
        if let Some(child) = &self.child {
            // SAFETY: a signal to the server process this test started.
            unsafe { libc::kill(child.id() as libc::pid_t, signal) };
        }
    }

    /// herdr's result for a call of the test's own, or its words.
    fn try_herdr(&self, args: &[&str]) -> Result<Value, String> {
        self.try_herdr_on(&self.socket, args)
    }

    /// herdr's result for a call of the test's own on the server at
    /// `socket`, or its words.
    fn try_herdr_on(&self, socket: &Path, args: &[&str]) -> Result<Value, String> {
        let mut command = Command::new("herdr");
        command.args(args);
        for (name, _) in std::env::vars_os() {
            if name.to_string_lossy().starts_with("HERDR_") {
                command.env_remove(name);
            }
        }
        let output = command
            .env("HERDR_SOCKET_PATH", socket)
            .env("HOME", self.scratch.path("home"))
            .stdin(Stdio::null())
            .output()
            .map_err(|error| error.to_string())?;
        let text = String::from_utf8_lossy(&output.stdout).into_owned();
        match serde_json::from_str::<Value>(text.trim()) {
            Ok(answer) if answer.get("result").is_some() => Ok(answer["result"].clone()),
            _ if output.status.success() => Ok(Value::String(text)),
            _ => Err(format!("{text}{}", String::from_utf8_lossy(&output.stderr))),
        }
    }

    fn herdr(&self, args: &[&str]) -> Value {
        self.herdr_on(&self.socket, args)
    }

    fn herdr_on(&self, socket: &Path, args: &[&str]) -> Value {
        self.try_herdr_on(socket, args)
            .unwrap_or_else(|words| panic!("herdr {}: {words}", args.join(" ")))
    }

    /// The profile on this server, the host user's login environment a
    /// scratch home and `/bin/sh`.
    fn profile(&self) -> Herdr {
        self.profile_with_login(&[("SHELL", "/bin/sh")])
    }

    /// The profile on this server, the host user's login environment
    /// `variables` over a scratch home and a UTF-8 locale.
    fn profile_with_login(&self, variables: &[(&str, &str)]) -> Herdr {
        let mut login = vec![
            (
                "HOME".to_string(),
                self.scratch.path("home").to_string_lossy().into_owned(),
            ),
            ("LANG".to_string(), "en_US.UTF-8".to_string()),
        ];
        login.extend(
            variables
                .iter()
                .map(|(name, value)| (name.to_string(), value.to_string())),
        );
        Herdr::new(
            Server::Socket(self.socket.clone()),
            self.scratch.path("records"),
        )
        .with_bound(BOUND)
        .with_login(&login)
    }

    /// The place `short` of the scratch repository on `base`.
    fn place_on(&self, short: &str, base: &str) -> PlaceRef {
        let location = self.scratch.path(&format!("repository/places/{short}"));
        fs::create_dir_all(&location).expect("the place's directory");
        PlaceRef {
            repository: self.scratch.path("repository").to_string_lossy().into(),
            location: location.to_string_lossy().into(),
            branch: format!("fix/{short}"),
            base: base.into(),
        }
    }

    fn place(&self, short: &str) -> PlaceRef {
        self.place_on(short, "main")
    }

    fn folder(&self) -> String {
        self.scratch.path("state/fix--one").to_string_lossy().into()
    }

    /// The stand-in's command, with `env` over its own and a `PATH` that
    /// holds no agent program of the host's.
    fn command(&self, env: &[(&str, &str)]) -> AgentCommand {
        let mut variables: BTreeMap<String, String> = BTreeMap::from([
            (
                "STAND_IN_LOG".to_string(),
                self.scratch.path("log").to_string_lossy().into_owned(),
            ),
            ("PATH".to_string(), "/usr/bin:/bin".to_string()),
        ]);
        variables.extend(
            env.iter()
                .map(|(name, value)| (name.to_string(), value.to_string())),
        );
        AgentCommand {
            program: self.scratch.path("bin/claude").to_string_lossy().into(),
            args: vec![],
            env: variables,
            prompt_args: vec![],
            folder_trust: Some(FolderTrust {
                question: TRUST.into(),
                focus: "\u{276f}".into(),
                next: "down".into(),
                confirm: "enter".into(),
            }),
            kind: Some("claude".into()),
            files: BTreeMap::new(),
        }
    }

    /// What the stand-in that started in `pane` keeps under `name`, line
    /// by line.
    fn kept(&self, pane: &str, name: &str) -> Vec<String> {
        fs::read_to_string(self.scratch.path("log").join(pane).join(name))
            .map(|text| text.lines().map(str::to_string).collect())
            .unwrap_or_default()
    }

    /// What the stand-in that started in `pane` keeps under `name`, line
    /// by line: on this server's own, or on a session's, whose stand-ins
    /// keep their logs under its socket's digest, since two servers give
    /// panes the same ids.
    fn kept_at(&self, pane: &Pane, name: &str) -> Vec<String> {
        if pane.socket == self.socket {
            return self.kept(&pane.id, name);
        }
        let log = self.session_log(&pane.socket).join(&pane.id).join(name);
        fs::read_to_string(log)
            .map(|text| text.lines().map(str::to_string).collect())
            .unwrap_or_default()
    }

    /// Where the stand-ins on the session's server at `socket` keep their
    /// logs.
    fn session_log(&self, socket: &Path) -> PathBuf {
        self.scratch
            .path("log")
            .join(digest(&socket.to_string_lossy()))
    }

    /// Waits up to ten seconds for `condition`.
    fn within(&self, what: &str, condition: impl Fn() -> bool) {
        let deadline = Instant::now() + Duration::from_secs(10);
        while !condition() {
            assert!(Instant::now() < deadline, "{what}");
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// herdr's agent status for the pane.
    fn status(&self, pane: &Pane) -> Option<String> {
        self.herdr(&["pane", "get", &pane.id])["pane"]["agent_status"]
            .as_str()
            .map(str::to_string)
    }

    /// The stand-in reports `state` to herdr, as its hooks would.
    fn report(&self, pane: &Pane, state: &str) {
        self.herdr(&[
            "pane",
            "report-agent",
            &pane.id,
            "--source",
            "flume-test",
            "--agent",
            "claude",
            "--state",
            state,
        ]);
    }

    /// The labels of the workspaces herdr lists.
    fn workspaces(&self) -> Vec<String> {
        self.herdr(&["workspace", "list"])["workspaces"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|workspace| workspace["label"].as_str().map(str::to_string))
            .collect()
    }

    /// The labels of the tabs herdr lists.
    fn tabs(&self) -> Vec<String> {
        self.herdr(&["tab", "list"])["tabs"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|tab| tab["label"].as_str().map(str::to_string))
            .collect()
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.thaw();
        self.stop();
        if let Some(link) = &self.link {
            let _ = fs::remove_file(link);
        }
    }
}

fn identity(start: u32) -> Identity {
    identity_of("fix--one", start)
}

fn identity_of(place: &str, start: u32) -> Identity {
    Identity {
        repository: "0123456789ab".into(),
        place: place.into(),
        stage: "code".into(),
        start,
    }
}

/// An agent opened in `place` under `who` and waiting, and its pane.
fn opened_in(
    server: &TestServer,
    profile: &Herdr,
    place: &PlaceRef,
    who: &Identity,
    command: &AgentCommand,
) -> Pane {
    opened_at(server, profile, place, who, command, &OpenIn::Own)
}

/// An agent opened in `place` under `who` where `at` says, and waiting,
/// and its pane.
fn opened_at(
    server: &TestServer,
    profile: &Herdr,
    place: &PlaceRef,
    who: &Identity,
    command: &AgentCommand,
    at: &OpenIn,
) -> Pane {
    let opened = profile.open(place, &server.folder(), command, who, at, None);
    assert!(
        matches!(opened, Opened::Opened { .. }),
        "open gave {opened:?}"
    );
    let pane = profile.pane(who).expect("the pane recorded");
    server.within("the stand-in did not come up waiting", || {
        profile.state(who) == AgentState::Waiting
    });
    pane
}

/// An agent opened under `identity` in the place `fix--one`, waiting.
fn opened(server: &TestServer, profile: &Herdr, who: &Identity, command: &AgentCommand) -> Pane {
    opened_in(server, profile, &server.place("fix--one"), who, command)
}

struct Harness {
    server: TestServer,
    seam: Herdr,
    caller: Pane,
    /// The pane each moved agent started in, by identity.
    moved: RefCell<BTreeMap<String, String>>,
    /// The home of the sessions attach starts and the standing agents
    /// stand in, the test's own.
    home: SessionHome,
}

impl Harness {
    fn new() -> Harness {
        let server = TestServer::start("herdr");
        let home = SessionHome::new(&["w"]);
        let caller = Pane {
            id: "w9:p9".into(),
            socket: server.socket.clone(),
        };
        let seam = server
            .profile()
            .with_start_timeout(HARNESS_START)
            .with_caller(Some(caller.clone()))
            .with_session_home(home.path())
            .with_plugin(Some(server.scratch.path("plugin")));
        Harness {
            server,
            seam,
            caller,
            moved: RefCell::default(),
            home,
        }
    }
}

impl TerminalHarness for Harness {
    type Seam = Herdr;

    fn seam(&self) -> &Herdr {
        &self.seam
    }

    fn shared(&self) -> Arc<dyn Terminal> {
        Arc::new(self.seam.clone())
    }

    fn place(&self) -> PlaceRef {
        self.server.place("fix--one")
    }

    fn folder(&self) -> String {
        self.server.folder()
    }

    fn command(&self) -> AgentCommand {
        self.server.command(&[])
    }

    fn sends(&self) -> bool {
        true
    }

    fn closes(&self) -> bool {
        true
    }

    fn stubborn(&self, _identity: &Identity) {
        self.server.freeze();
    }

    fn session(&self) -> Option<String> {
        Some(self.caller.session())
    }

    fn shows(&self) -> bool {
        true
    }

    fn leave_typed(&self, identity: &Identity, text: &str) {
        self.server
            .within("the stand-in did not come up waiting", || {
                self.seam.state(identity) == AgentState::Waiting
            });
        let pane = self.seam.pane(identity).expect("the pane recorded");
        self.server.herdr(&["pane", "send-text", &pane.id, text]);
    }

    fn submitted(&self, identity: &Identity) -> Vec<String> {
        let started = self.started_in(identity);
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let got = self.server.kept(&started, "got");
            if !got.is_empty() || Instant::now() >= deadline {
                return got;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    fn held(&self) -> Option<AgentCommand> {
        Some(
            self.server
                .command(&[("STAND_IN_WARNING", "WARNING: Bypass Permissions mode")]),
        )
    }

    fn never_up(&self) -> Option<AgentCommand> {
        let mut command = self.server.command(&[]);
        command.program = self
            .server
            .scratch
            .path("bin/missing")
            .to_string_lossy()
            .into();
        Some(command)
    }

    fn makes(&self) -> bool {
        true
    }

    fn person_pane(&self, place: &PlaceRef) {
        let (workspace, _) = self.seam.workspace(place).expect("the base's workspace");
        self.server.herdr(&[
            "tab",
            "create",
            "--workspace",
            &workspace,
            "--label",
            "mine",
            "--no-focus",
        ]);
    }

    fn move_pane(&self, identity: &Identity) {
        let pane = self.seam.pane(identity).expect("the pane recorded");
        self.moved
            .borrow_mut()
            .insert(identity.to_string(), pane.id.clone());
        self.server.herdr(&[
            "pane",
            "move",
            &pane.id,
            "--new-workspace",
            "--label",
            "moved",
            "--no-focus",
        ]);
    }

    fn shows_clues(&self) -> bool {
        true
    }

    /// A stand-in opened in a place of its own, as a person's or a
    /// caller's agent stands in a pane, by its pane on the server.
    fn recipient(&self) -> Option<String> {
        let who = identity_of("ops--caller", 1);
        let pane = opened_in(
            &self.server,
            &self.seam,
            &self.server.place("ops--caller"),
            &who,
            &self.server.command(&[]),
        );
        Some(pane.session())
    }

    fn received(&self, session: &str) -> Vec<String> {
        let pane = Pane::named(session).expect("a session the profile named");
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let got = self.server.kept_at(&pane, "got");
            if !got.is_empty() || Instant::now() >= deadline {
                return got;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    fn no_session(&self) -> String {
        Pane {
            id: "w9:p99".into(),
            socket: self.server.socket.clone(),
        }
        .session()
    }

    /// herdr 0.9.5 has no app of its own, and attaches a window's client
    /// alone (187).
    fn views(&self) -> Vec<View> {
        vec![View::Terminal]
    }

    /// The stream's session, named by one letter as the sessions' tests
    /// name theirs; the profile's own server is held by its socket, whose
    /// part herdr attaches by no name.
    fn parts(&self) -> Vec<Option<String>> {
        vec![Some("w".into())]
    }

    fn refuse_attach(&self, _view: View) -> bool {
        false
    }

    /// The stream's session `w`, which no server of the scratch home runs,
    /// and the profile's own part, held by a socket no session of the home
    /// lists (201).
    fn runs_parts(&self) -> Vec<(Option<String>, SessionRuns)> {
        vec![
            (
                Some("w".into()),
                SessionRuns::NotRunning { name: "w".into() },
            ),
            (
                None,
                SessionRuns::Unread {
                    reason: format!(
                        "herdr session list --json lists no session at {}",
                        self.server.socket.display()
                    ),
                },
            ),
        ]
    }

    /// The plugin's watch on this server, its lease renewed now, having
    /// written that herdr's configuration binds no key to the popup.
    /// What the plugin's startup on the test's server writes it lacks,
    /// in setup.json with the server's socket, which is present (122,
    /// C.5).
    fn setup(&self) -> Vec<String> {
        let words = "herdr's configuration, /h/config.toml, binds no key to flume's places";
        let server = self.server.scratch.path("plugin/servers/1");
        fs::create_dir_all(&server).expect("the plugin's server state");
        fs::write(
            server.join("setup.json"),
            serde_json::json!({ "socket": self.server.socket, "lacking": [words] }).to_string(),
        )
        .expect("setup.json");
        vec![words.to_string()]
    }

    /// A stand-in started in the session `session` of the harness's own
    /// home by `herdr agent start <name>`, as a host's standing agent
    /// stands in its session (132, C.5).
    fn named_agent(&self, session: &str, name: &str) -> bool {
        standing(&self.server, &self.home, session, Some(name));
        true
    }

    /// herdr relays a pane's screen and a person's keys (221, 222, C.5's
    /// relay row).
    fn relays(&self) -> bool {
        true
    }
}

impl Harness {
    /// The pane the agent's stand-in started in, which names its log.
    fn started_in(&self, identity: &Identity) -> String {
        self.moved
            .borrow()
            .get(&identity.to_string())
            .cloned()
            .unwrap_or_else(|| self.seam.pane(identity).expect("the pane recorded").id)
    }
}

#[test]
fn the_herdr_profile_passes_the_terminal_scenarios() {
    pass_all("terminal", &terminal::scenarios(), Harness::new);
}

#[test]
fn s39_the_folder_trust_question_is_answered_and_the_agent_comes_up() {
    let server = TestServer::start("herdr-trust");
    let profile = server.profile();
    let who = identity(1);
    let pane = opened(
        &server,
        &profile,
        &who,
        &server.command(&[("STAND_IN_TRUST", "1")]),
    );
    assert_eq!(
        server.kept(&pane.id, "events"),
        vec!["trusted".to_string()],
        "the stand-in read an answer to its question"
    );
    assert!(
        fs::metadata(server.scratch.path("log").join(&pane.id).join("started")).is_ok(),
        "the stand-in did not start once trusted"
    );
}

/// The folder-trust question drawn a while after herdr was told the
/// program is held at a question: the choices are waited for, read off
/// the screen and answered yes, never "No, exit" (34; record 0018).
#[test]
fn s39_a_folder_trust_question_drawn_late_is_answered_from_the_screen() {
    let server = TestServer::start("herdr-trust-late");
    let profile = server.profile();
    let who = identity(1);
    let pane = opened(
        &server,
        &profile,
        &who,
        &server.command(&[("STAND_IN_TRUST", "late")]),
    );
    assert_eq!(
        server.kept(&pane.id, "events"),
        vec!["trusted".to_string()],
        "the stand-in read yes to its question"
    );
}

/// The folder-trust question drawn a while after herdr read the program
/// ready: the screen is read until the program draws, and the question
/// answered yes (34; record 0018).
#[test]
fn s39_a_folder_trust_question_drawn_after_herdr_reads_ready_is_answered() {
    let server = TestServer::start("herdr-trust-ready");
    let profile = server.profile();
    let who = identity(1);
    let pane = opened(
        &server,
        &profile,
        &who,
        &server.command(&[("STAND_IN_TRUST", "ready")]),
    );
    assert_eq!(
        server.kept(&pane.id, "events"),
        vec!["trusted".to_string()],
        "the stand-in read yes to its question"
    );
}

/// The folder-trust question drawn only once open has returned: a settle
/// of the start, which its agent has not said it started, answers it from
/// the screen (34, 35; record 0018).
#[test]
fn s39_a_folder_trust_question_drawn_after_the_open_is_answered_at_a_settle() {
    let server = TestServer::start("herdr-trust-after");
    let profile = server.profile();
    let who = identity(1);
    let opened = profile.open(
        &server.place("fix--one"),
        &server.folder(),
        &server.command(&[("STAND_IN_TRUST", "after")]),
        &who,
        &OpenIn::Own,
        None,
    );
    assert!(
        matches!(opened, Opened::Opened { .. }),
        "open gave {opened:?}"
    );
    assert!(
        !profile.answer_trust(&who),
        "a question not yet asked was answered"
    );
    let pane = profile.pane(&who).expect("the pane recorded");
    let deadline = Instant::now() + Duration::from_secs(20);
    while !profile.answer_trust(&who) {
        assert!(Instant::now() < deadline, "the question was never answered");
        std::thread::sleep(Duration::from_millis(500));
    }
    assert_eq!(
        server.kept(&pane.id, "events"),
        vec!["trusted".to_string()],
        "the stand-in read yes to its question"
    );
}

/// The folder-trust question in a dialog herdr does not know, so its
/// agent start times out, or answers that the name no longer owns the
/// terminal where herdr's server dropped it first under load (the gate's
/// run of 272bc16): the question is read off the screen and
/// answered yes, and the agent comes up (34; record 0018).
#[test]
fn s39_a_folder_trust_question_herdr_does_not_see_is_answered_from_the_screen() {
    let server = TestServer::start("herdr-trust-silent");
    let profile = server.profile().with_start_timeout(SHORT_START);
    let who = identity(1);
    let pane = opened(
        &server,
        &profile,
        &who,
        &server.command(&[("STAND_IN_TRUST", "silent")]),
    );
    assert_eq!(
        server.kept(&pane.id, "events"),
        vec!["trusted".to_string()],
        "the stand-in read yes to its question"
    );
}

#[test]
fn s40_the_bypass_warning_holds_the_start_with_its_lines() {
    let server = TestServer::start("herdr-warning");
    let profile = server.profile();
    let who = identity(1);
    let warning = "WARNING: running in Bypass Permissions mode";
    let answer = profile.open(
        &server.place("fix--one"),
        &server.folder(),
        &server.command(&[("STAND_IN_WARNING", warning)]),
        &who,
        &OpenIn::Own,
        None,
    );
    let Opened::Held { screen } = answer else {
        panic!("open at the bypass warning gave {answer:?}");
    };
    assert!(screen.contains(warning), "the screen held: {screen}");
    let pane = profile.pane(&who).expect("the pane recorded");
    assert!(
        server.kept(&pane.id, "events").is_empty() && server.kept(&pane.id, "got").is_empty(),
        "something was typed into the warning"
    );
}

#[test]
fn s45_a_program_that_never_comes_up_is_not_come_up_with_the_screen() {
    let server = TestServer::start("herdr-timeout");
    let profile = server.profile().with_start_timeout(SHORT_START);
    let mut command = server.command(&[]);
    command.program = server.scratch.path("bin/missing").to_string_lossy().into();
    let began = Instant::now();
    let answer = profile.open(
        &server.place("fix--one"),
        &server.folder(),
        &command,
        &identity(1),
        &OpenIn::Own,
        None,
    );
    assert!(
        matches!(&answer, Opened::NotComeUp { screen } if screen.contains("claude")),
        "open of a program not there gave {answer:?}"
    );
    assert!(
        began.elapsed() < SHORT_START + Duration::from_secs(10),
        "open waited {:?}",
        began.elapsed()
    );
}

/// A role run's agent, opened under the role's address, moved into
/// another workspace: a notice to the address finds its pane by the
/// start's herdr name, records the new id and reaches it there (132;
/// record 0018).
#[test]
fn a_role_runs_pane_moved_to_another_workspace_is_reached_by_its_address() {
    let server = TestServer::start("herdr-role");
    let profile = server.profile();
    let who = identity(1);
    let opened = profile.open(
        &server.place("fix--one"),
        &server.folder(),
        &server.command(&[]),
        &who,
        &OpenIn::Own,
        Some("madswan/planner"),
    );
    assert!(
        matches!(opened, Opened::Opened { .. }),
        "open gave {opened:?}"
    );
    let pane = profile.pane(&who).expect("the pane recorded");
    server.within("the stand-in did not come up waiting", || {
        profile.session_state("madswan/planner") == Some(AgentState::Waiting)
    });
    server.herdr(&[
        "pane",
        "move",
        &pane.id,
        "--new-workspace",
        "--label",
        "moved",
        "--no-focus",
    ]);
    assert_eq!(
        profile.send_to("madswan/planner", "flume: fix--two code delivered"),
        ToSession::Sent,
        "the notice to the role's address"
    );
    let moved = profile.pane(&who).expect("the pane recorded");
    assert_ne!(
        moved.id, pane.id,
        "the moved pane's new id was not recorded"
    );
    server.within("the moved agent did not receive the notice", || {
        server.kept(&pane.id, "got") == vec!["flume: fix--two code delivered".to_string()]
    });
    assert_eq!(profile.close(&who), Closed::Closed);
    assert_eq!(profile.session_state("madswan/planner"), None);
    assert_eq!(
        profile.send_to("madswan/planner", "another"),
        ToSession::NotFound,
        "a notice to an address whose run has ended"
    );
}

/// A recipient on another herdr server than the one the profile opens
/// on: the notice goes through the recipient's own socket, and a
/// recipient asking takes nothing (43, 95, 98; record 0016).
#[test]
fn a_recipient_on_another_server_is_sent_to_through_its_own_socket() {
    let server = TestServer::start("herdr-mine");
    let theirs = TestServer::start("herdr-theirs");
    let profile = server.profile();
    let other = theirs.profile();
    let who = identity_of("ops--caller", 1);
    let pane = opened_in(
        &theirs,
        &other,
        &theirs.place("ops--caller"),
        &who,
        &theirs.command(&[]),
    );
    let session = pane.session();
    assert_eq!(
        profile.session_state(&session),
        Some(AgentState::Waiting),
        "the recipient's state read on its own server"
    );
    assert_eq!(
        profile.send_to(&session, "flume: fix--one code delivered"),
        ToSession::Sent
    );
    theirs.within("the recipient did not receive the notice", || {
        theirs.kept(&pane.id, "got") == vec!["flume: fix--one code delivered".to_string()]
    });
    theirs.report(&pane, "blocked");
    assert_eq!(
        profile.session_state(&session),
        Some(AgentState::Asking),
        "the recipient asking"
    );
    assert!(
        matches!(
            profile.send_to(&session, "another"),
            ToSession::Refused { .. }
        ),
        "a notice typed into a question"
    );
    assert_eq!(
        theirs.kept(&pane.id, "got"),
        vec!["flume: fix--one code delivered".to_string()],
        "what the asking recipient took"
    );
}

#[test]
fn s46_a_pane_moved_to_another_workspace_is_found_by_its_agents_name() {
    let server = TestServer::start("herdr-moved");
    let profile = server.profile();
    let who = identity(1);
    let pane = opened(&server, &profile, &who, &server.command(&[]));
    server.herdr(&[
        "pane",
        "move",
        &pane.id,
        "--new-workspace",
        "--label",
        "moved",
        "--no-focus",
    ]);
    assert_eq!(profile.state(&who), AgentState::Waiting, "the moved agent");
    let moved = profile.pane(&who).expect("the pane recorded");
    assert_ne!(
        moved.id, pane.id,
        "the moved pane's new id was not recorded"
    );
    let agent = server.herdr(&["agent", "get", &herdr_name(&who)]);
    assert_eq!(
        agent["agent"]["pane_id"].as_str(),
        Some(moved.id.as_str()),
        "the pane herdr lists under the agent's name"
    );
    assert_eq!(profile.send(&who, "go on"), Sent::Sent, "the send");
    server.within("the moved agent did not receive the send", || {
        server.kept(&pane.id, "got") == vec!["go on".to_string()]
    });
}

#[test]
fn the_environment_is_set_on_the_pane_and_nothing_is_typed() {
    let server = TestServer::start("herdr-env");
    let who = identity(1);
    let value = "a value with spaces, 'quotes' and $dollars";
    let profile = server.profile_with_login(&[("SHELL", "/bin/sh"), ("STAND_IN_VALUE", "login")]);
    let from_login = identity(2);
    let pane = opened(&server, &profile, &from_login, &server.command(&[]));
    assert_eq!(
        server.kept(&pane.id, "env"),
        vec!["login".to_string()],
        "the login environment's variable the started program read"
    );
    assert_eq!(profile.close(&from_login), Closed::Closed);
    let pane = opened(
        &server,
        &profile,
        &who,
        &server.command(&[("STAND_IN_VALUE", value)]),
    );
    assert_eq!(
        server.kept(&pane.id, "env"),
        vec![value.to_string()],
        "the variable the started program read"
    );
    let screen = profile.screen(&who).expect("a screen");
    assert!(
        !screen.contains("STAND_IN_VALUE") && !screen.contains("export"),
        "the environment was typed: {screen}"
    );
}

/// A nix-wrapped Claude Code, `bin/claude` a wrapper that sets a variable
/// and runs `bin/.claude` in its place, as devenv pins it: the agent
/// profile names the wrapped program, which herdr starts under the kind's
/// name, with the wrapper's variable, and the foreground check takes it
/// (record 0018; C.5).
#[test]
fn a_nix_wrapped_claude_starts_under_herdr_from_the_agent_profile() {
    use flume::profiles::agent::claude::Claude;
    let server = TestServer::start("herdr-wrapped");
    let nix = server.scratch.path("nix/bin");
    fs::create_dir_all(&nix).expect("the wrapper's folder");
    let wrapped = nix.join(".claude");
    fs::write(&wrapped, STAND_IN).expect("the wrapped program");
    fs::set_permissions(&wrapped, fs::Permissions::from_mode(0o755)).expect("executable");
    let wrapper = nix.join("claude");
    fs::write(
        &wrapper,
        format!(
            "#! /bin/sh -e\nexport STAND_IN_VALUE='set by the wrapper'\nexec \"{}\"  \"$@\"\n",
            wrapped.display()
        ),
    )
    .expect("the wrapper");
    fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o755)).expect("executable");
    let claude = Claude::new(
        &server.scratch.path("data"),
        &server.scratch.path("bin/flume"),
    );
    let settings = Settings {
        program: Some(wrapper.to_string_lossy().into()),
        ..Settings::default()
    };
    let env = server.command(&[]).env;
    let place = server.place("fix--one");
    let who = identity(1);
    let command = claude.command(
        &server.folder(),
        &settings,
        None,
        &BTreeMap::new(),
        None,
        &env,
        &place,
        &who,
        &Secret("s".into()),
        "go",
    );
    assert_eq!(
        Path::new(&command.program),
        wrapped.as_path(),
        "the program the agent profile names"
    );
    let profile = server.profile();
    let pane = opened(&server, &profile, &who, &command);
    assert_eq!(
        server.kept(&pane.id, "env"),
        vec!["set by the wrapper".to_string()],
        "the wrapper's variable the started program read"
    );
    let listed = server.herdr(&["pane", "get", &pane.id]);
    assert!(
        !listed["pane"]["agent"].is_null(),
        "herdr lists no agent in the pane: {listed}"
    );
}

/// Instructions of many lines, as a host's that holds a whole document
/// (deneb, 2026-11-05): as an argument herdr refuses them as
/// `invalid_agent_argument`, a newline or a tab being a control
/// character, and the refused start leaves no pane; from the agent
/// profile they go by a file of the start's own, which the agent starts
/// naming and which holds them whole (148, C.4, C.5; record 0038).
#[test]
fn instructions_of_many_lines_start_under_herdr_by_the_agent_profiles_file() {
    use flume::profiles::agent::claude::{Claude, STANDING_PROMPT_FLAG};
    let server = TestServer::start("herdr-lines");
    let profile = server.profile();
    let instructions =
        "## Capture\n\n- **What counts.** A decision the person makes.\n\t- and its record\n";
    let refused = identity(1);
    let mut command = server.command(&[]);
    command.args = vec!["--append-system-prompt".into(), instructions.into()];
    let answer = profile.open(
        &server.place("fix--one"),
        &server.folder(),
        &command,
        &refused,
        &OpenIn::Own,
        None,
    );
    assert!(
        matches!(&answer, Opened::Refused { reason } if reason.contains("invalid_agent_argument")),
        "instructions of many lines as an argument gave {answer:?}"
    );
    assert_eq!(
        profile.pane(&refused),
        None,
        "the refused start's pane kept"
    );
    assert!(
        !server.tabs().contains(&"fix--one".to_string()),
        "the refused start's pane stands"
    );

    let claude = Claude::new(
        &server.scratch.path("data"),
        &server.scratch.path("bin/flume"),
    );
    let stand_in = server.command(&[]);
    let settings = Settings {
        program: Some(stand_in.program.clone()),
        instructions: Some(instructions.into()),
        ..Settings::default()
    };
    let who = identity(2);
    let command = claude.command(
        &server.folder(),
        &settings,
        None,
        &BTreeMap::new(),
        None,
        &stand_in.env,
        &server.place("fix--one"),
        &who,
        &Secret("s".into()),
        "go",
    );
    let pane = opened(&server, &profile, &who, &command);
    let argv = fs::read(server.scratch.path("log").join(&pane.id).join("argv"))
        .expect("the arguments the agent started with");
    let argv = String::from_utf8_lossy(&argv).into_owned();
    let words: Vec<&str> = argv.split_terminator('\0').collect();
    let file = words
        .iter()
        .position(|word| *word == STANDING_PROMPT_FLAG)
        .and_then(|at| words.get(at + 1))
        .unwrap_or_else(|| panic!("the agent started with no standing prompt's file: {words:?}"));
    assert_eq!(
        fs::read_to_string(file).expect("the standing prompt's file"),
        instructions,
        "the instructions the file holds"
    );
}

#[test]
fn startup_files_that_put_another_claude_first_leave_the_program_the_settings_name() {
    let server = TestServer::start("herdr-startup");
    let rc = server.scratch.path("bin/rc").to_string_lossy().into_owned();
    let zdot = server.scratch.path("zdot").to_string_lossy().into_owned();
    let home = zdot.clone();
    let other = server.scratch.path("other").to_string_lossy().into_owned();
    let shells: Vec<&str> = ["/bin/zsh", "/bin/bash", "/bin/sh"]
        .into_iter()
        .filter(|shell| Path::new(shell).is_file())
        .collect();
    for (n, shell) in shells.iter().enumerate() {
        // The login shell's every startup file, by `HOME`, `ZDOTDIR`,
        // `ENV` and `BASH_ENV`, would put `other/claude` first.
        let profile = server.profile_with_login(&[
            ("SHELL", shell),
            ("HOME", &home),
            ("ZDOTDIR", &zdot),
            ("ENV", &rc),
            ("BASH_ENV", &rc),
        ]);
        let who = identity(n as u32 + 1);
        let pane = opened(
            &server,
            &profile,
            &who,
            &server.command(&[("ENV", &rc), ("BASH_ENV", &rc)]),
        );
        let program = server.kept(&pane.id, "program");
        assert!(
            program.len() == 1 && !program[0].starts_with(&other),
            "under {shell} the pane ran {program:?}, not the program the settings name"
        );
        let closed = profile.close(&who);
        assert_eq!(closed, Closed::Closed, "under {shell}");
    }
}

#[test]
fn a_program_that_runs_another_in_its_place_is_refused_naming_it() {
    let server = TestServer::start("herdr-program");
    let profile = server.profile();
    let who = identity(1);
    let mut command = server.command(&[]);
    command.program = server
        .scratch
        .path("bin/wrapper")
        .to_string_lossy()
        .into_owned();
    let answer = profile.open(
        &server.place("fix--one"),
        &server.folder(),
        &command,
        &who,
        &OpenIn::Own,
        None,
    );
    let other = server
        .scratch
        .path("other/claude")
        .to_string_lossy()
        .into_owned();
    assert!(
        matches!(&answer, Opened::Refused { reason } if reason.contains(&other)),
        "open where the program ran another gave {answer:?}"
    );
    assert_eq!(profile.pane(&who), None, "the refused start's pane kept");
    assert!(
        !server.tabs().contains(&"fix--one".to_string()),
        "the refused start's pane stands"
    );
}

#[test]
fn a_herdr_name_taken_refuses_the_open() {
    let server = TestServer::start("herdr-taken");
    let profile = server.profile();
    let who = identity(1);
    opened(&server, &profile, &who, &server.command(&[]));
    let record = server
        .scratch
        .path("records")
        .join(format!("{}.json", who.to_string().replace('/', "%")));
    fs::remove_file(record).expect("the agent's record");
    let answer = profile.open(
        &server.place("fix--one"),
        &server.folder(),
        &server.command(&[]),
        &who,
        &OpenIn::Own,
        None,
    );
    assert!(
        matches!(&answer, Opened::Refused { reason } if reason.contains("agent_name_taken")),
        "an open under a name herdr holds gave {answer:?}"
    );
    let name = herdr_name(&who);
    assert!(
        name.len() <= 32
            && name.starts_with(|c: char| c.is_ascii_lowercase())
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || "_-".contains(c)),
        "a name herdr's rule refuses: {name}"
    );
    assert_ne!(
        herdr_name(&who),
        herdr_name(&Identity {
            repository: "ba9876543210".into(),
            ..who.clone()
        }),
        "two repositories' agents under one name"
    );
}

#[test]
fn s47_the_last_place_on_a_base_closes_its_tab_and_workspace() {
    let server = TestServer::start("herdr-last");
    let profile = server.profile();
    let (one, two) = (server.place("fix--one"), server.place("fix--two"));
    let (first, second) = (identity_of("fix--one", 1), identity_of("fix--two", 1));
    opened_in(&server, &profile, &one, &first, &server.command(&[]));
    opened_in(&server, &profile, &two, &second, &server.command(&[]));
    let label = format!("{} main", "repository");
    assert_eq!(
        server.workspaces().iter().filter(|w| **w == label).count(),
        1,
        "the base's workspaces: {:?}",
        server.workspaces()
    );
    let tabs = server.tabs();
    assert!(
        tabs.contains(&"fix--one".to_string()) && tabs.contains(&"fix--two".to_string()),
        "the places' tabs: {tabs:?}"
    );
    let tidy = profile.tidy(&one);
    assert_eq!(
        tidy.closed,
        vec!["the tab fix--one".to_string()],
        "{tidy:?}"
    );
    assert!(
        server.workspaces().contains(&label),
        "the workspace went early"
    );
    let tidy = profile.tidy(&two);
    assert!(tidy.kept.is_empty(), "kept: {tidy:?}");
    assert_eq!(
        tidy.closed.last().map(String::as_str),
        Some(format!("the workspace {label}").as_str()),
        "{tidy:?}"
    );
    assert!(
        !server.workspaces().contains(&label),
        "the workspace stands"
    );
    assert_eq!(profile.kept(), Vec::<String>::new(), "kept");
}

#[test]
fn places_on_two_bases_open_in_two_workspaces() {
    let server = TestServer::start("herdr-bases");
    let profile = server.profile();
    let main = server.place_on("fix--one", "main");
    let bolt = server.place_on("fix--two", "bolt/the-surfaces");
    opened_in(
        &server,
        &profile,
        &main,
        &identity_of("fix--one", 1),
        &server.command(&[]),
    );
    opened_in(
        &server,
        &profile,
        &bolt,
        &identity_of("fix--two", 1),
        &server.command(&[]),
    );
    let workspaces = server.workspaces();
    for label in ["repository main", "repository bolt/the-surfaces"] {
        assert!(
            workspaces.contains(&label.to_string()),
            "no workspace {label}: {workspaces:?}"
        );
    }
    let (main_workspace, _) = profile.workspace(&main).expect("main's workspace");
    let (bolt_workspace, _) = profile.workspace(&bolt).expect("the bolt's workspace");
    assert_ne!(
        main_workspace, bolt_workspace,
        "one workspace for both bases"
    );
}

#[test]
fn s48_a_persons_pane_keeps_the_workspace_named() {
    let server = TestServer::start("herdr-person");
    let profile = server.profile();
    let place = server.place("fix--one");
    let who = identity(1);
    opened_in(&server, &profile, &place, &who, &server.command(&[]));
    let (workspace, _) = profile.workspace(&place).expect("the workspace");
    let mine = server.herdr(&[
        "tab",
        "create",
        "--workspace",
        &workspace,
        "--label",
        "mine",
        "--no-focus",
    ]);
    let mine = mine["root_pane"]["pane_id"].as_str().unwrap().to_string();
    assert_eq!(profile.close(&who), Closed::Closed, "close");
    let tidy = profile.tidy(&place);
    assert!(
        tidy.kept.iter().any(|kept| kept.contains(&mine)),
        "the person's pane named: {tidy:?}"
    );
    assert!(
        server.workspaces().contains(&"repository main".to_string()),
        "the person's workspace was closed"
    );
    assert!(
        server.try_herdr(&["pane", "get", &mine]).is_ok(),
        "the person's pane was closed"
    );
    assert!(
        profile.kept().iter().any(|kept| kept.contains(&mine)),
        "kept: {:?}",
        profile.kept()
    );
}

#[test]
fn s49_titles_follow_the_stage_and_a_clue_that_fails_fails_nothing() {
    let server = TestServer::start("herdr-clues");
    let place = server.place("fix--one");
    let who = identity(1);
    let pane = opened_in(
        &server,
        &server.profile(),
        &place,
        &who,
        &server.command(&[]),
    );
    let profile = server.profile().with_bound(Duration::from_secs(2));
    let clues = |state: &str| Clues {
        stage: "code".into(),
        state: state.into(),
        agent: Some(who.clone()),
        base: BaseCount {
            running: 1,
            ..BaseCount::default()
        },
        line: format!("code · run 1 · {state}"),
    };
    for state in ["starting", "waiting", "working", "asking"] {
        assert_eq!(profile.show(&place, &clues(state)), Shown::Shown, "{state}");
        let got = server.herdr(&["pane", "get", &pane.id]);
        assert_eq!(
            got["pane"]["title"].as_str(),
            Some(format!("code · {state}").as_str()),
            "the pane's title"
        );
        assert_eq!(got["pane"]["display_agent"].as_str(), Some("fix--one"));
        assert_eq!(
            got["pane"]["tokens"]["flume"].as_str(),
            Some(format!("code · run 1 · {state}").as_str()),
            "the pane's row, {got}"
        );
    }
    let (workspace, _) = profile.workspace(&place).expect("the workspace");
    let got = server.herdr(&["workspace", "get", &workspace]);
    assert_eq!(
        got["workspace"]["tokens"]["flume"].as_str(),
        Some("1 running · 0 asking · 0 waiting on caller · 0 stalled"),
        "the workspace's token"
    );
    server.freeze();
    let failed = profile.show(&place, &clues("working"));
    server.thaw();
    assert!(
        matches!(failed, Shown::Failed { .. }),
        "a clue to a server that does not answer gave {failed:?}"
    );
}

#[test]
fn a_conversation_opens_beside_a_named_pane() {
    let server = TestServer::start("herdr-beside");
    let profile = server.profile();
    let operator = server.herdr(&["workspace", "create", "--label", "operator", "--no-focus"]);
    let operator = operator["root_pane"]["pane_id"]
        .as_str()
        .unwrap()
        .to_string();
    server.herdr(&["pane", "rename", &operator, "operator"]);
    let beside = |start: u32, named: &str| {
        let who = identity(start);
        let answer = profile.open(
            &server.place("fix--one"),
            &server.folder(),
            &server.command(&[]),
            &who,
            &OpenIn::Beside {
                pane: named.to_string(),
            },
            None,
        );
        assert!(
            matches!(answer, Opened::Opened { .. }),
            "open gave {answer:?}"
        );
        let pane = profile.pane(&who).expect("the pane recorded");
        assert_ne!(pane.id, operator, "the agent took the operator's pane");
        assert_eq!(pane.socket, server.socket, "the pane's server");
        let tab_of = |id: &str| server.herdr(&["pane", "get", id])["pane"]["tab_id"].clone();
        assert_eq!(tab_of(&pane.id), tab_of(&operator), "the agent's tab");
        assert_eq!(profile.close(&who), Closed::Closed, "close");
    };
    beside(2, "operator");
    beside(
        3,
        &Pane {
            id: operator.clone(),
            socket: server.socket.clone(),
        }
        .session(),
    );
    let refused = profile.open(
        &server.place("fix--one"),
        &server.folder(),
        &server.command(&[]),
        &identity(4),
        &OpenIn::Beside {
            pane: "nobody".into(),
        },
        None,
    );
    assert!(
        matches!(&refused, Opened::Refused { reason } if reason.contains("nobody")),
        "open beside a pane nobody named gave {refused:?}"
    );
    assert!(
        !server.tabs().contains(&"fix--one".to_string()),
        "a conversation's open made the place a tab"
    );
}

/// The session a command run in a herdr pane names, written where
/// `FLUME_HERDR_SESSION_OUT` says: the half of
/// `a_command_run_in_a_herdr_pane_names_its_pane` that runs in the pane.
#[test]
fn session_child() {
    let Some(out) = std::env::var_os("FLUME_HERDR_SESSION_OUT") else {
        return;
    };
    let session = Herdr::of_host()
        .expect("the host's herdr")
        .session()
        .unwrap_or_default();
    fs::write(out, session).expect("the session written");
}

#[test]
fn a_command_run_in_a_herdr_pane_names_its_pane() {
    let server = TestServer::start("herdr-session");
    let made = server.herdr(&["workspace", "create", "--label", "caller", "--no-focus"]);
    let pane = made["root_pane"]["pane_id"].as_str().unwrap().to_string();
    let out = server.scratch.path("session");
    let line = format!(
        "FLUME_HERDR_SESSION_OUT={} XDG_STATE_HOME={} {} --exact terminal_herdr::session_child --nocapture",
        out.display(),
        server.scratch.path("state").display(),
        std::env::current_exe().unwrap().display()
    );
    server.herdr(&["pane", "run", &pane, &line]);
    let deadline = Instant::now() + Duration::from_secs(30);
    while !out.exists() {
        assert!(
            Instant::now() < deadline,
            "the command in the pane wrote nothing"
        );
        std::thread::sleep(Duration::from_millis(200));
    }
    std::thread::sleep(Duration::from_millis(200));
    assert_eq!(
        fs::read_to_string(&out).unwrap(),
        format!("herdr:{pane}@{}", server.socket.display()),
        "the session a launch from the pane names as its recipient"
    );
}

#[test]
fn herdrs_readings_as_the_profile_reads_them() {
    let server = TestServer::start("herdr-states");
    let profile = server.profile();
    let who = identity(1);
    let pane = opened(&server, &profile, &who, &server.command(&[]));
    assert_eq!(profile.send(&who, "do the work"), Sent::Sent, "the prompt");
    server.report(&pane, "working");
    assert_eq!(profile.state(&who), AgentState::Working, "working");
    server.report(&pane, "idle");
    server.within(
        "herdr did not read a finished turn nobody looked at as done",
        || server.status(&pane).as_deref() == Some("done"),
    );
    assert_eq!(
        profile.state(&who),
        AgentState::Waiting,
        "done, a finished turn nobody looked at (S42)"
    );
    server.report(&pane, "blocked");
    assert_eq!(profile.state(&who), AgentState::Asking, "blocked");
    server.report(&pane, "unknown");
    assert_eq!(profile.state(&who), AgentState::Unknown, "herdr's unknown");
    server.report(&pane, "idle");
    server.herdr(&["pane", "send-keys", &pane.id, "ctrl+d"]);
    server.within("an agent whose program ended does not read stopped", || {
        profile.state(&who) == AgentState::Stopped
    });
    assert!(
        server.try_herdr(&["pane", "get", &pane.id]).is_ok(),
        "the pane went with its agent"
    );
}

#[test]
fn s43_a_prompt_left_typed_is_cleared_before_each_sending() {
    let server = TestServer::start("herdr-typed");
    let profile = server.profile();
    let who = identity(1);
    let pane = opened(&server, &profile, &who, &server.command(&[]));
    server.herdr(&["pane", "send-text", &pane.id, "left typed by a hand"]);
    assert_eq!(
        profile.send(&who, "the prompt"),
        Sent::Sent,
        "the first sending"
    );
    server.within("the first sending was not received", || {
        server.kept(&pane.id, "got").len() == 1
    });
    server.herdr(&["pane", "send-text", &pane.id, "the prompt, left typed"]);
    assert_eq!(
        profile.send(&who, "the prompt"),
        Sent::Sent,
        "the second sending"
    );
    server.within("the second sending was not received", || {
        server.kept(&pane.id, "got").len() == 2
    });
    assert_eq!(
        server.kept(&pane.id, "got"),
        vec!["the prompt".to_string(), "the prompt".to_string()],
        "what the agent received (S43)"
    );
    assert_eq!(
        server.kept(&pane.id, "events"),
        vec!["cleared".to_string(), "cleared".to_string()],
        "the input cleared before each sending"
    );
}

#[test]
fn a_pane_herdr_could_not_bring_back_reads_stopped() {
    let mut server = TestServer::start("herdr-restore");
    let profile = server.profile();
    let who = identity(1);
    let place = server.place("fix--one");
    let pane = opened_in(&server, &profile, &place, &who, &server.command(&[]));
    server.stop();
    fs::remove_dir_all(&place.location).expect("the place removed");
    server.run();
    let restored = server.herdr(&["pane", "get", &pane.id]);
    assert!(
        restored["pane"]["restore_error"].is_string(),
        "herdr restored the pane: {restored}"
    );
    assert_eq!(
        profile.state(&who),
        AgentState::Stopped,
        "the unrestored pane"
    );
}

/// The host user's login shell as a loaded host runs it, slow to come to
/// its prompt, held at a gate until the test lets it go: named `bash`, so
/// the pane's shell runs it with `--noprofile --norc`, it says what it is
/// given, marks that it holds, and waits on the gate by the shell's own
/// `read`, so the pane's foreground holds nothing but the shell and its
/// terminal stays canonical, as it is until a line editor takes it; let
/// go, it becomes bash.
struct HeldShell {
    dir: PathBuf,
    /// The shell, as the host user's login environment names it.
    login: String,
}

impl HeldShell {
    fn new(server: &TestServer, says: &str) -> HeldShell {
        let dir = server.scratch.path("held");
        fs::create_dir_all(&dir).expect("the held shell's folder");
        let gate = dir.join("gate");
        let made = Command::new("mkfifo")
            .arg(&gate)
            .status()
            .expect("mkfifo runs");
        assert!(made.success(), "the gate made");
        let says = if says.is_empty() {
            String::new()
        } else {
            format!("printf '%s\\n' '{says}'\n")
        };
        let script = format!(
            "#!/bin/sh\n{says}: > '{}'\nread -r _ < '{}'\nexec bash \"$@\"\n",
            dir.join("holds").display(),
            gate.display()
        );
        let path = dir.join("bash");
        fs::write(&path, script).expect("the held shell written");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
            .expect("the held shell made executable");
        HeldShell {
            dir,
            login: path.to_string_lossy().into_owned(),
        }
    }

    /// Whether the shell holds at its gate.
    fn holds(&self) -> bool {
        self.dir.join("holds").exists()
    }

    /// Lets the shell go on to its prompt: the gate opened without
    /// blocking, so a shell that never reads it fails the test rather
    /// than hanging it.
    fn let_go(&self) {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            match fs::OpenOptions::new()
                .write(true)
                .custom_flags(libc::O_NONBLOCK)
                .open(self.dir.join("gate"))
            {
                Ok(mut gate) => {
                    gate.write_all(b"go\n").expect("the gate opened");
                    return;
                }
                Err(error) => {
                    assert!(
                        Instant::now() < deadline,
                        "no shell reads the gate: {error}"
                    );
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }
}

/// What open answered for the agent under `who`, opened in the place
/// `fix--one` in a pane whose shell `shell` holds: once open has made the
/// pane, the shell holds at its gate and the pane's screen `shows` as the
/// test asks, the shell is let go, open still under way.
fn opened_once_let_go(
    server: &TestServer,
    profile: &Herdr,
    shell: &HeldShell,
    who: &Identity,
    command: &AgentCommand,
    shows: impl Fn(&str) -> bool,
) -> Opened {
    let place = server.place("fix--one");
    let folder = server.folder();
    std::thread::scope(|scope| {
        let opening =
            scope.spawn(|| profile.open(&place, &folder, command, who, &OpenIn::Own, None));
        server.within(
            "open made no pane whose held shell shows what it says",
            || {
                opening.is_finished()
                    || (shell.holds() && profile.screen(who).is_some_and(|screen| shows(&screen)))
            },
        );
        if opening.is_finished() {
            let opened = opening.join().expect("open answers");
            panic!("open answered {opened:?} while the pane's shell held before its prompt");
        }
        shell.let_go();
        opening.join().expect("open answers")
    })
}

/// Instructions of 20,000 characters, opened in a pane whose shell has
/// not yet come to its prompt, its screen blank and its terminal taking
/// no line past its canonical limit, as under the gate's load on deneb
/// (2026-10-30): open waits for the prompt rather than starting the agent
/// into the shell, and the agent starts with them whole once the shell
/// comes to it (C.5; record 0018).
#[test]
fn instructions_of_twenty_thousand_characters_reach_the_agent_whole() {
    let server = TestServer::start("herdr-long");
    let shell = HeldShell::new(&server, "");
    let profile = server.profile_with_login(&[("SHELL", shell.login.as_str())]);
    let who = identity(1);
    let instructions: String =
        "keep the lockfile exactly as it stands: 'quotes', $HOME, `ticks` and é; "
            .chars()
            .cycle()
            .take(20_000)
            .collect();
    let mut command = server.command(&[]);
    command.args = vec!["--append-system-prompt".into(), instructions.clone()];
    let opened = opened_once_let_go(&server, &profile, &shell, &who, &command, |screen| {
        screen.trim().is_empty()
    });
    assert!(
        matches!(opened, Opened::Opened { .. }),
        "open gave {opened:?}"
    );
    let pane = profile.pane(&who).expect("the pane recorded");
    server.within("the stand-in did not come up waiting", || {
        profile.state(&who) == AgentState::Waiting
    });
    let argv = fs::read(server.scratch.path("log").join(&pane.id).join("argv"))
        .expect("the arguments the agent started with");
    let mut given = b"--append-system-prompt\0".to_vec();
    given.extend(instructions.as_bytes());
    given.push(0);
    let differ = argv.iter().zip(&given).position(|(a, b)| a != b);
    assert!(
        argv == given,
        "the arguments the agent started with differ: {} bytes for {}, first at {differ:?}: {:?}",
        argv.len(),
        given.len(),
        differ.map(|at| String::from_utf8_lossy(
            &argv[at.saturating_sub(20)..(at + 20).min(argv.len())]
        )
        .into_owned())
    );
}

/// The words a shell says as it starts, before its line editor takes the
/// terminal, as macOS's bash says the default shell is now zsh.
const SAYS: &str = "The shell starts.";

/// Instructions past the terminal's canonical limit, 1024 bytes on macOS,
/// and within Linux's, where herdr checks no line.
fn past_the_canonical_limit() -> String {
    "keep the lockfile as it stands; ".repeat(100)
}

/// A shell that says something as it starts and is held before its line
/// editor takes the terminal: open's wait for its first output is over,
/// and herdr answers agent_start_input_failed, the terminal not yet
/// taking the line; open asks again and the agent starts once the shell
/// comes to its prompt (C.5; record 0018).
#[test]
fn a_start_the_terminal_cannot_yet_take_is_asked_again_until_it_can() {
    let server = TestServer::start("herdr-not-yet");
    let shell = HeldShell::new(&server, SAYS);
    let profile = server.profile_with_login(&[("SHELL", shell.login.as_str())]);
    let who = identity(1);
    let mut command = server.command(&[]);
    let instructions = past_the_canonical_limit();
    command.args = vec!["--append-system-prompt".into(), instructions.clone()];
    let opened = opened_once_let_go(&server, &profile, &shell, &who, &command, |screen| {
        screen.contains(SAYS)
    });
    assert!(
        matches!(opened, Opened::Opened { .. }),
        "open gave {opened:?}"
    );
    let pane = profile.pane(&who).expect("the pane recorded");
    server.within("the stand-in did not come up waiting", || {
        profile.state(&who) == AgentState::Waiting
    });
    let argv = fs::read(server.scratch.path("log").join(&pane.id).join("argv"))
        .expect("the arguments the agent started with");
    assert_eq!(
        String::from_utf8_lossy(&argv),
        format!("--append-system-prompt\0{instructions}\0"),
        "the arguments the agent started with"
    );
}

/// A shell that never comes to its prompt within the start's bound: open
/// asks agent start again until the bound, and the start has not come up,
/// with the screen and, on macOS, where herdr checks the line, herdr's
/// words; it is never refused (35, C.5; record 0018).
#[test]
fn a_shell_never_at_its_prompt_is_not_come_up_and_never_refused() {
    let server = TestServer::start("herdr-never-prompt");
    let shell = HeldShell::new(&server, SAYS);
    let profile = server
        .profile_with_login(&[("SHELL", shell.login.as_str())])
        .with_start_timeout(SHORT_START);
    let mut command = server.command(&[]);
    command.args = vec!["--append-system-prompt".into(), past_the_canonical_limit()];
    let began = Instant::now();
    let answer = profile.open(
        &server.place("fix--one"),
        &server.folder(),
        &command,
        &identity(1),
        &OpenIn::Own,
        None,
    );
    let took = began.elapsed();
    let Opened::NotComeUp { screen } = &answer else {
        panic!("open of a pane whose shell never came to its prompt gave {answer:?}");
    };
    assert!(screen.contains(SAYS), "the screen given: {screen:?}");
    if cfg!(target_os = "macos") {
        assert!(
            screen.contains("agent_start_input_failed"),
            "herdr's words not given: {screen:?}"
        );
        assert!(
            took >= SHORT_START - Duration::from_millis(500),
            "open asked again for only {took:?}"
        );
    }
    assert!(
        took < SHORT_START + Duration::from_secs(10),
        "open waited {took:?}"
    );
}

#[test]
fn a_herdr_call_that_never_answers_is_refused_within_its_bound() {
    let server = TestServer::start("herdr-silent");
    let who = identity(1);
    opened(&server, &server.profile(), &who, &server.command(&[]));
    let profile = server.profile().with_bound(Duration::from_secs(2));
    server.freeze();
    let began = Instant::now();
    let sent = profile.send(&who, "the prompt");
    let took = began.elapsed();
    server.thaw();
    assert!(
        matches!(&sent, Sent::Refused { reason } if reason.contains("within 2 seconds")),
        "a send to a server that does not answer gave {sent:?}"
    );
    assert!(
        took < Duration::from_secs(4),
        "the send waited {took:?} on herdr"
    );
    assert_eq!(START_TIMEOUT, Duration::from_secs(120), "the start's bound");
}

/// The stand-in as the agent seam reads it: started once it wrote its
/// mark, a prompt received once it took the prompt's line; it asks
/// nothing and starts no own work.
struct StandIn {
    log: PathBuf,
    program: String,
    terminal: Herdr,
    /// Variables over the stand-in's own, as a test's dialog asks.
    env: BTreeMap<String, String>,
}

impl StandIn {
    fn kept(&self, agent: &Identity, name: &str) -> Option<String> {
        let pane = self.terminal.pane(agent)?;
        fs::read_to_string(self.log.join(pane.id).join(name)).ok()
    }
}

impl Agent for StandIn {
    fn command(
        &self,
        _folder: &str,
        _settings: &Settings,
        _mode: Option<&str>,
        _own: &BTreeMap<String, serde_json::Value>,
        _resume: Option<&str>,
        _env: &BTreeMap<String, String>,
        _place: &PlaceRef,
        _identity: &Identity,
        _secret: &Secret,
        prompt: &str,
    ) -> AgentCommand {
        let mut env = BTreeMap::from([
            (
                "STAND_IN_LOG".to_string(),
                self.log.to_string_lossy().into_owned(),
            ),
            ("PATH".to_string(), "/usr/bin:/bin".to_string()),
        ]);
        env.extend(self.env.clone());
        AgentCommand {
            program: self.program.clone(),
            args: vec![],
            env,
            prompt_args: vec![prompt.into()],
            folder_trust: None,
            kind: Some("claude".into()),
            files: BTreeMap::new(),
        }
    }

    fn started(&self, _folder: &str, agent: &Identity) -> Reading {
        match self.kept(agent, "started") {
            Some(_) => Reading::Yes,
            None => Reading::No,
        }
    }

    fn received(&self, _folder: &str, agent: &Identity, text: &str) -> Received {
        let record = format!("stand-in:{agent}");
        match self.kept(agent, "got") {
            Some(got) if got.lines().any(|line| line == text) => Received::Received { record },
            _ => Received::NotReceived { record },
        }
    }

    fn asking(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::No
    }

    fn own_work(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::No
    }

    fn ended(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::No
    }

    fn last_words(&self, _folder: &str, _agent: &Identity) -> Option<String> {
        None
    }

    fn stop_short(&self, _agent: &Identity) -> String {
        "to stop short, run flume needs".into()
    }

    fn stop_short_to(&self) -> String {
        "name it by flume needs --to".into()
    }

    fn send_back(&self) -> Offered {
        Offered::NotOffered
    }

    fn guard(&self, _folder: &str, _agent: &Identity) -> Guarded {
        Guarded::NotOffered
    }

    fn on_quiet(&self) -> Offered {
        Offered::NotOffered
    }
}

/// A repository of the scratch's bound in `world`'s fakes, with `stages`
/// as the kind `two`, each run by the stand-in.
fn bound(server: &TestServer, world: &World, stages: Vec<Stage>) -> String {
    bound_with(server, world, stages, &[])
}

/// A repository of the scratch's bound in `world`'s fakes, with `stages`
/// as the kind `two`, each run by the stand-in with `env` over its own.
fn bound_with(
    server: &TestServer,
    world: &World,
    stages: Vec<Stage>,
    env: &[(&str, &str)],
) -> String {
    let repository = server
        .scratch
        .path("repository")
        .to_string_lossy()
        .into_owned();
    world.git.init(&repository, "main");
    world.kinds.set_binding(
        &repository,
        Binding {
            place: Some("trees".into()),
            change: Some("plans".into()),
            merge_queue: Some("queue".into()),
        },
    );
    let stages = stages
        .into_iter()
        .map(|mut stage| {
            stage.agents = vec![Settings {
                program: Some("stand-in".into()),
                ..Settings::default()
            }];
            stage
        })
        .collect();
    world
        .kinds
        .set_kinds(&repository, vec![kind("two", false, stages)]);
    world.resolver.bind_agent(
        "stand-in",
        Arc::new(StandIn {
            log: server.scratch.path("log"),
            program: server.scratch.path("bin/claude").to_string_lossy().into(),
            terminal: server.profile(),
            env: env
                .iter()
                .map(|(name, value)| (name.to_string(), value.to_string()))
                .collect(),
        }),
    );
    world
        .place
        .locate_under(&server.scratch.path("data/flume").to_string_lossy());
    fs::create_dir_all(place_of(world, &repository).0).expect("the place's directory");
    repository
}

/// The place `two--one` of `repository`, where `world`'s fake place makes
/// it.
fn place_of(world: &World, repository: &str) -> PlaceId {
    PlaceId::new(
        world
            .place
            .location(repository, "two--one")
            .expect("the place's location"),
    )
}

/// An engine over `world`'s fakes with herdr for its terminal, the
/// host's multiplexer and the resolver's `herdr`, the store in the
/// scratch's state directory and the host's own clock (190).
fn engine(server: &TestServer, world: &World) -> Engine {
    let state = server.scratch.path("state/flume");
    let herdr: Arc<dyn Terminal> = Arc::new(server.profile());
    world.resolver.with_terminal(NAME, herdr.clone());
    let config = HostConfig {
        terminal: Some(NAME.into()),
        ..HostConfig::default()
    };
    Engine::new(
        world.resolver.clone(),
        herdr,
        Arc::new(StateDir::at(&state)),
        world.history.clone(),
        // The command's environment names a recipient, so a launch from
        // no pane is taken (7, 193).
        Arc::new(RealHost::new(&state, config, Some("caller".into()))),
        world.defaults.clone(),
    )
}

fn stage_state(server: &TestServer, world: &World, place: &PlaceId, name: &str) -> StageState {
    match engine(server, world).status(Some(place)) {
        Answer::Status { places, .. } => places[0]
            .stages
            .iter()
            .find(|stage| stage.name == name)
            .map(|stage| stage.state)
            .expect("the stage"),
        other => panic!("status answered {other:?}"),
    }
}

/// The kind `two` launched as `two--one` in the scratch repository and
/// settled until its first stage has begun.
fn begun(server: &TestServer, world: &World, repository: &str) -> PlaceId {
    let answer = engine(server, world).launch(&Launch {
        repository: repository.into(),
        kind: "two".into(),
        name: "one".into(),
        base: "main".into(),
        brief: Some("fix the bug".into()),
        ..Launch::default()
    });
    assert!(answer.refusal().is_none(), "the launch: {answer:?}");
    let id = place_of(world, repository);
    let deadline = Instant::now() + Duration::from_secs(60);
    while stage_state(server, world, &id, "code") != StageState::Begun {
        assert!(
            Instant::now() < deadline,
            "the code stage did not begin: {:?}",
            stage_state(server, world, &id, "code")
        );
        engine(server, world).settle(Some(&id));
    }
    id
}

#[test]
fn the_row_follows_the_line() {
    // The terminal-profiles spec's "The row follows the line" (139, C.5):
    // a change's repeating code stage keeps working while the change at
    // its head goes from 2 tasks ticked of 5 to 3; the settle then
    // reports the pane's title and its row as the token `flume`, the
    // short form of the place's line, against the pinned herdr's server,
    // and the next settle, with nothing changed, reports nothing.
    const FIVE: &[(&str, &str, usize)] = &[("1", "One", 4), ("2", "Two", 1)];
    let server = TestServer::start("herdr-row");
    let world = World::new();
    let mut code = stage(
        "code",
        Some(Deliverable::TasksTicked),
        "{{brief}} {{needs}}",
    );
    code.repeats = true;
    let repository = bound(&server, &world, vec![code]);
    // Launched with code held, its change written on its branch, then
    // code started at the head that holds it, at its first group.
    let answer = engine(&server, &world).launch(&Launch {
        repository: repository.clone(),
        kind: "two".into(),
        name: "one".into(),
        base: "main".into(),
        brief: Some("build it".into()),
        hold: vec!["code".into()],
        ..Launch::default()
    });
    assert!(answer.refusal().is_none(), "the launch: {answer:?}");
    let id = place_of(&world, &repository);
    let at = world
        .git
        .commit(&repository, "two/one", &["plans/one/tasks.md"]);
    world
        .change
        .set(&repository, &at, "one", FakeChange::groups(FIVE, 2));
    let retried = engine(&server, &world).retry(&id, Retry::Stage("code".into()), None, false);
    assert!(retried.refusal().is_none(), "the retry: {retried:?}");
    let deadline = Instant::now() + Duration::from_secs(60);
    while stage_state(&server, &world, &id, "code") != StageState::Begun {
        assert!(
            Instant::now() < deadline,
            "the code stage did not begin: {:?}",
            stage_state(&server, &world, &id, "code")
        );
        engine(&server, &world).settle(Some(&id));
    }
    let who = Identity {
        repository: flume_core::repository_id(&format!("{repository}/.git")),
        place: "two--one".into(),
        stage: "code".into(),
        start: 1,
    };
    let pane = server.profile().pane(&who).expect("the pane recorded");
    server.report(&pane, "working");
    let calls = server.scratch.path("calls");
    let logging = server.scratch.path("logging-herdr");
    fs::write(
        &logging,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{}'\nexec herdr \"$@\"\n",
            calls.display()
        ),
    )
    .expect("the logging herdr");
    fs::set_permissions(&logging, fs::Permissions::from_mode(0o755))
        .expect("the logging herdr made executable");
    let state = server.scratch.path("state/flume");
    let settle = || {
        let herdr: Arc<dyn Terminal> = Arc::new(server.profile().with_program(&logging));
        world.resolver.with_terminal(NAME, herdr.clone());
        Engine::new(
            world.resolver.clone(),
            herdr,
            Arc::new(StateDir::at(&state)),
            world.history.clone(),
            Arc::new(RealHost::new(
                &state,
                HostConfig {
                    terminal: Some(NAME.into()),
                    ..HostConfig::default()
                },
                None,
            )),
            world.defaults.clone(),
        )
        .settle(Some(&id));
        let reports: Vec<String> = fs::read_to_string(&calls)
            .unwrap_or_default()
            .lines()
            .filter(|call| call.contains("report-metadata"))
            .map(String::from)
            .collect();
        let _ = fs::remove_file(&calls);
        reports
    };
    settle();
    let head = world
        .git
        .commit(&repository, "two/one", &["plans/one/tasks.md"]);
    world
        .change
        .set(&repository, &head, "one", FakeChange::groups(FIVE, 3));
    let row = "code 1/2 · run 1 · tasks 3/5 · working";
    let reports = settle();
    assert!(
        reports.contains(&format!(
            "pane report-metadata {} --source flume --title code 1/2 · working --display-agent two--one --token flume={row}",
            pane.id
        )),
        "the settle's reports: {reports:?}"
    );
    assert_eq!(
        server.herdr(&["pane", "get", &pane.id])["pane"]["tokens"]["flume"].as_str(),
        Some(row),
        "the pane's row"
    );
    assert_eq!(
        settle(),
        Vec::<String>::new(),
        "the reports of a settle that changed nothing"
    );
}

#[test]
fn a_quiet_pane_with_nothing_delivered_is_not_ended() {
    let server = TestServer::start("herdr-quiet");
    let world = World::new();
    let repository = bound(
        &server,
        &world,
        vec![stage(
            "code",
            Some(Deliverable::Commits),
            "{{brief}} {{needs}}",
        )],
    );
    let id = begun(&server, &world, &repository);
    let code = Identity {
        repository: flume_core::repository_id(&format!("{repository}/.git")),
        place: "two--one".into(),
        stage: "code".into(),
        start: 1,
    };
    let pane = server.profile().pane(&code).expect("the pane recorded");
    for _ in 0..3 {
        engine(&server, &world).settle(Some(&id));
    }
    assert_eq!(
        stage_state(&server, &world, &id, "code"),
        StageState::Begun,
        "a quiet stage with nothing delivered (39, I1)"
    );
    assert!(
        server.try_herdr(&["pane", "get", &pane.id]).is_ok(),
        "the quiet agent's pane was closed"
    );
    assert_eq!(
        server.profile().state(&code),
        AgentState::Waiting,
        "the agent"
    );
}

#[test]
fn s42_a_finished_turn_nobody_looked_at_delivers_and_the_next_stage_starts() {
    let server = TestServer::start("herdr-done");
    let world = World::new();
    let repository = bound(
        &server,
        &world,
        vec![
            stage("code", Some(Deliverable::Commits), "{{brief}} {{needs}}"),
            stage("review", Some(Deliverable::Commits), "{{brief}} {{needs}}"),
        ],
    );
    let id = begun(&server, &world, &repository);
    let code = Identity {
        repository: flume_core::repository_id(&format!("{repository}/.git")),
        place: "two--one".into(),
        stage: "code".into(),
        start: 1,
    };
    let pane = server.profile().pane(&code).expect("the pane recorded");
    server.report(&pane, "working");
    world.git.commit(&repository, "two/one", &["fix.rs"]);
    server.report(&pane, "idle");
    server.within("herdr did not read the finished turn done", || {
        server.status(&pane).as_deref() == Some("done")
    });
    engine(&server, &world).settle(Some(&id));
    assert_eq!(
        stage_state(&server, &world, &id, "code"),
        StageState::Delivered,
        "the code stage"
    );
    assert!(
        server.try_herdr(&["pane", "get", &pane.id]).is_err(),
        "the code stage's agent was not ended"
    );
    assert!(
        matches!(
            stage_state(&server, &world, &id, "review"),
            StageState::Starting | StageState::Started | StageState::Begun
        ),
        "the review stage did not start"
    );
}

#[test]
fn a_launch_and_a_retry_each_send_their_agents_prompt_with_no_settle() {
    // On deneb a retry's agent came up ready in herdr and sat with no
    // prompt until a settle by hand: the watch's settle had sent it while
    // the retry still opened the agent, into a program not yet reading
    // its input. The command that opens an agent now sends its prompt
    // once its open answers, and nothing else sends while it opens, so
    // each agent here takes its prompt with no settle run (6, 37, 38, 46).
    let server = TestServer::start("herdr-sent");
    let world = World::new();
    let repository = bound(
        &server,
        &world,
        vec![stage(
            "code",
            Some(Deliverable::Commits),
            "{{brief}} {{needs}}",
        )],
    );
    let answer = engine(&server, &world).launch(&Launch {
        repository: repository.clone(),
        kind: "two".into(),
        name: "one".into(),
        base: "main".into(),
        brief: Some("fix the bug".into()),
        ..Launch::default()
    });
    assert!(answer.refusal().is_none(), "the launch: {answer:?}");
    let id = place_of(&world, &repository);
    let code = |start| Identity {
        repository: flume_core::repository_id(&format!("{repository}/.git")),
        place: "two--one".into(),
        stage: "code".into(),
        start,
    };
    let took = |pane: &Pane| {
        server
            .kept(&pane.id, "got")
            .iter()
            .any(|line| line.contains("fix the bug"))
    };
    let first = server.profile().pane(&code(1)).expect("the pane recorded");
    server.within("the launch's agent took no prompt", || took(&first));

    // The begun stage's agent is the caller's to end before a retry (50).
    let stopped = engine(&server, &world).stop(&id, false);
    assert!(stopped.refusal().is_none(), "the stop: {stopped:?}");
    let retried = engine(&server, &world).retry(&id, Retry::Stage("code".into()), None, false);
    assert!(retried.refusal().is_none(), "the retry: {retried:?}");
    let second = server.profile().pane(&code(2)).expect("the pane recorded");
    server.within("the retry's agent took no prompt", || took(&second));
    assert_eq!(
        server.kept(&second.id, "got").len(),
        1,
        "the retry's sendings"
    );
    engine(&server, &world).settle(Some(&id));
    assert_eq!(
        stage_state(&server, &world, &id, "code"),
        StageState::Begun,
        "the code stage at the first settle"
    );
}

/// A dialog as Claude Code draws one for a repository's MCP servers.
const MCP_DIALOG: &str = "New MCP server found in .mcp.json: docs";

/// The place's status, as the engine gives it.
fn place_status(server: &TestServer, world: &World, place: &PlaceId) -> flume_core::PlaceStatus {
    match engine(server, world).status(Some(place)) {
        Answer::Status { mut places, .. } => places.remove(0),
        other => panic!("status answered {other:?}"),
    }
}

/// The kind `two` launched as `two--one` in the scratch repository, its
/// place's id.
fn launched(server: &TestServer, world: &World, repository: &str) -> PlaceId {
    let answer = engine(server, world).launch(&Launch {
        repository: repository.into(),
        kind: "two".into(),
        name: "one".into(),
        base: "main".into(),
        brief: Some("fix the bug".into()),
        ..Launch::default()
    });
    assert!(answer.refusal().is_none(), "the launch: {answer:?}");
    place_of(world, repository)
}

#[test]
fn a_dialog_before_the_prompt_waits_on_a_person_with_its_words() {
    // On deneb a Claude Code agent stopped at its dialog approving a
    // repository's MCP server, before it took any prompt, and flume read
    // its stage never begun, saying nothing of the dialog. A dialog the
    // program shows a person before it takes its prompt now holds the
    // start, starting, the place waiting on a person with the dialog's
    // words for as long as it shows; once answered, the agent starts and
    // takes its prompt (34, 35, 37).
    let server = TestServer::start("herdr-dialog");
    let world = World::new();
    let mut code = stage("code", Some(Deliverable::Commits), "{{brief}} {{needs}}");
    code.quiet_limit = Some(1);
    let repository = bound_with(
        &server,
        &world,
        vec![code],
        &[("STAND_IN_DIALOG", MCP_DIALOG)],
    );
    let id = launched(&server, &world, &repository);
    let who = Identity {
        repository: flume_core::repository_id(&format!("{repository}/.git")),
        place: "two--one".into(),
        stage: "code".into(),
        start: 1,
    };
    let pane = server.profile().pane(&who).expect("the pane recorded");
    std::thread::sleep(Duration::from_millis(1500));
    engine(&server, &world).settle(Some(&id));
    let status = place_status(&server, &world, &id);
    assert_eq!(
        status.stages[0].state,
        StageState::Starting,
        "the stage past its limit at the dialog: {status:?}"
    );
    let waiting = status
        .waiting_on
        .clone()
        .expect("the place waits on nobody");
    assert_eq!(
        (waiting.act, waiting.why, waiting.stage.as_deref()),
        (Act::Answer, Waits::Dialog, Some("code")),
        "what the place waits on"
    );
    assert!(
        waiting
            .words
            .as_deref()
            .is_some_and(|words| words.contains(MCP_DIALOG)),
        "the dialog's words: {waiting:?}"
    );
    let dialog = status.agent.and_then(|agent| agent.dialog);
    assert!(
        dialog
            .as_deref()
            .is_some_and(|words| words.contains(MCP_DIALOG)),
        "the agent's dialog: {dialog:?}"
    );
    assert!(
        server.kept(&pane.id, "got").is_empty() && server.kept(&pane.id, "events").is_empty(),
        "something was typed into the dialog"
    );

    // A person answers it in the pane.
    server.herdr(&["pane", "send-keys", &pane.id, "enter"]);
    server.within("the stand-in did not take the answer", || {
        server.kept(&pane.id, "events") == vec!["answered".to_string()]
    });
    let deadline = Instant::now() + Duration::from_secs(60);
    while stage_state(&server, &world, &id, "code") != StageState::Begun {
        assert!(
            Instant::now() < deadline,
            "the code stage did not begin: {:?}",
            stage_state(&server, &world, &id, "code")
        );
        engine(&server, &world).settle(Some(&id));
    }
    assert!(
        server
            .kept(&pane.id, "got")
            .iter()
            .any(|line| line.contains("fix the bug")),
        "the agent took no prompt"
    );
    let status = place_status(&server, &world, &id);
    assert_eq!(status.waiting_on, None, "what the place waits on");
}

#[test]
fn an_agent_that_began_after_its_start_read_never_begun_is_begun_and_a_retry_leaves_it() {
    // On deneb a conversation's agent stopped at a dialog herdr read
    // ready, and its start was read never begun; a person answered it and
    // the agent began, but flume still held the stage never begun, and the
    // operator's retry ended the working session. The stage is now begun
    // once its agent says it started, and a retry is refused, forced or
    // not, naming the agent and the stop that ends it first (35, 37, 50).
    let server = TestServer::start("herdr-began");
    let world = World::new();
    let mut talk = stage("talk", None, "{{brief}} {{needs}}");
    talk.quiet_limit = Some(1);
    let repository = bound_with(
        &server,
        &world,
        vec![talk],
        &[
            ("STAND_IN_DIALOG", MCP_DIALOG),
            ("STAND_IN_DIALOG_SEEN", "no"),
        ],
    );
    let id = launched(&server, &world, &repository);
    let who = Identity {
        repository: flume_core::repository_id(&format!("{repository}/.git")),
        place: "two--one".into(),
        stage: "talk".into(),
        start: 1,
    };
    let pane = server.profile().pane(&who).expect("the pane recorded");
    std::thread::sleep(Duration::from_millis(1500));
    engine(&server, &world).settle(Some(&id));
    assert_eq!(
        stage_state(&server, &world, &id, "talk"),
        StageState::NeverBegun,
        "a start at a dialog herdr reads ready, past its limit"
    );

    // A person answers the dialog, and the agent says it started.
    server.herdr(&["pane", "send-keys", &pane.id, "enter"]);
    server.within("the stand-in did not take the answer", || {
        server.kept(&pane.id, "events") == vec!["answered".to_string()]
    });
    let deadline = Instant::now() + Duration::from_secs(60);
    while stage_state(&server, &world, &id, "talk") != StageState::Begun {
        assert!(
            Instant::now() < deadline,
            "the talk stage did not begin: {:?}",
            stage_state(&server, &world, &id, "talk")
        );
        engine(&server, &world).settle(Some(&id));
    }
    assert!(
        server
            .kept(&pane.id, "got")
            .iter()
            .any(|line| line.contains("fix the bug")),
        "the agent took no prompt"
    );
    assert_eq!(
        place_status(&server, &world, &id).waiting_on,
        None,
        "what the place waits on"
    );

    for force in [false, true] {
        let retried = engine(&server, &world).retry(&id, Retry::Current, None, force);
        let reason = retried
            .refusal()
            .unwrap_or_else(|| panic!("the retry, forced {force}, was taken: {retried:?}"));
        assert!(
            reason.contains(&who.to_string()) && reason.contains("flume stop"),
            "the refusal names not the agent and its stop: {reason}"
        );
    }
    assert!(
        server.try_herdr(&["pane", "get", &pane.id]).is_ok(),
        "the retry ended the working session"
    );
    assert!(
        server
            .profile()
            .pane(&Identity {
                start: 2,
                ..who.clone()
            })
            .is_none(),
        "a second agent was opened"
    );
}

/// `flume status --json` under the scratch host, the herdr terminal
/// bound, its terminal setup.
fn terminal_setup(server: &TestServer, flume: &Path) -> Value {
    let mut command = Command::new(flume);
    for (name, _) in std::env::vars_os() {
        let name = name.to_string_lossy().into_owned();
        if name.starts_with("FLUME_") || name.starts_with("HERDR_") || name.starts_with("REX_") {
            command.env_remove(name);
        }
    }
    let output = command
        .args(["status", "--json"])
        .current_dir(server.scratch.path("home"))
        .env("HOME", server.scratch.path("home"))
        .env("XDG_STATE_HOME", server.scratch.path("state"))
        .env("XDG_CONFIG_HOME", server.scratch.path("config"))
        .env("XDG_DATA_HOME", server.scratch.path("data"))
        .stdin(Stdio::null())
        .output()
        .expect("flume runs");
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "flume status: {text}");
    let answer: Value = serde_json::from_str(text.trim()).expect("status's JSON");
    answer["status"]["host"]["terminal_setup"].clone()
}

#[test]
fn a_startups_setup_is_given_by_status_while_its_server_runs() {
    // 122, C.5: the plugin's startup, run as herdr runs it at the pinned
    // herdr's server's start, reads a herdr configuration that binds no
    // key to the popup and names no row of flume's, writes what it lacks
    // into setup.json with the server's socket, and exits; `flume status
    // --json` gives those words as the host's terminal setup while the
    // server runs, and none once it has stopped.
    let mut server = TestServer::start("setup");
    let flume = Path::new(env!("CARGO_BIN_EXE_flume"));
    let plugin = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../plugins/herdr")
        .canonicalize()
        .expect("the herdr plugin");
    let plugin_state = server.scratch.path("state/herdr/plugins/agentplot.flume");
    fs::create_dir_all(server.scratch.path("config/flume")).expect("flume's configuration");
    fs::write(
        server.scratch.path("config/flume/config.toml"),
        "[host]\nterminal = \"herdr\"\n",
    )
    .expect("flume's configuration");
    let herdr_config = server.scratch.path("herdr.toml");
    fs::write(
        &herdr_config,
        "[[keys.command]]\nkey = \"prefix+m\"\ntype = \"plugin_action\"\ncommand = \"agentplot.sluice.board\"\n",
    )
    .expect("herdr's configuration");
    let path = format!(
        "{}:{}",
        flume.parent().expect("flume's folder").display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let mut command = Command::new("sh");
    for (name, _) in std::env::vars_os() {
        let name = name.to_string_lossy().into_owned();
        if name.starts_with("FLUME_") || name.starts_with("HERDR_") || name.starts_with("REX_") {
            command.env_remove(name);
        }
    }
    let ran = command
        .arg(plugin.join("bin/startup.sh"))
        .current_dir(server.scratch.path("home"))
        .env("PATH", path)
        .env("HOME", server.scratch.path("home"))
        .env("XDG_STATE_HOME", server.scratch.path("state"))
        .env("XDG_CONFIG_HOME", server.scratch.path("config"))
        .env("XDG_DATA_HOME", server.scratch.path("data"))
        .env("HERDR_PLUGIN_ROOT", &plugin)
        .env("HERDR_PLUGIN_STATE_DIR", &plugin_state)
        .env("HERDR_SOCKET_PATH", &server.socket)
        .env("HERDR_BIN_PATH", herdr_program())
        .env("HERDR_CONFIG_PATH", &herdr_config)
        .stdin(Stdio::null())
        .output()
        .expect("the startup runs");
    assert!(
        ran.status.success(),
        "the startup: {}",
        String::from_utf8_lossy(&ran.stderr)
    );
    let written: Value = fs::read_dir(plugin_state.join("servers"))
        .expect("the plugin's servers")
        .filter_map(Result::ok)
        .find_map(|server| fs::read_to_string(server.path().join("setup.json")).ok())
        .and_then(|text| serde_json::from_str(&text).ok())
        .expect("setup.json");
    assert_eq!(
        written["socket"].as_str(),
        Some(server.socket.to_string_lossy().as_ref()),
        "the server's socket in setup.json"
    );
    let lacking = written["lacking"].clone();
    let file = herdr_config.display().to_string();
    let words: Vec<&str> = lacking
        .as_array()
        .expect("an array")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert_eq!(words.len(), 2, "{lacking}");
    assert!(
        words[0].contains(&file) && words[0].contains("binds no key to flume's places"),
        "{}",
        words[0]
    );
    assert!(
        words[1].contains(&file) && words[1].contains("\"$flume\""),
        "{}",
        words[1]
    );
    assert_eq!(terminal_setup(&server, flume), lacking);
    server.stop();
    assert_eq!(terminal_setup(&server, flume), Value::Array(vec![]));
}

/// A lease an earlier release's watch kept on server `1` under `plugin`,
/// renewed `ago` seconds before now by this process, as that watch wrote
/// it; this release reads none.
fn lease_renewed(plugin: &Path, ago: u64) {
    let server = plugin.join("servers/1");
    fs::create_dir_all(&server).expect("the plugin's server state");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("the clock")
        .as_secs();
    fs::write(
        server.join("watch.lease"),
        format!("{} {}\n", std::process::id(), now - ago),
    )
    .expect("the lease");
}

/// The host's watch and when it lapsed, as status gives them.
fn watch_and_lapse(engine: &Engine) -> (Watch, Option<Seconds>) {
    match engine.status(None) {
        Answer::Status { host, .. } => (host.watch, host.watch_lapsed),
        other => panic!("status answered {other:?}"),
    }
}

#[test]
fn unattended_by_the_hosts_watch_alone() {
    // The terminal-profiles spec's "Unattended by the host's watch
    // alone" (119, 122, 208, C.5): the herdr profile bound against the
    // pinned herdr's server, the plugin's state holding an earlier
    // release's fresh lease beside it, every herdr command it runs
    // logged; a caller keeping watch with a bound of a minute keeps flume
    // unattended, three minutes with no such settle let it run attended,
    // and no herdr command was run to decide either.
    let server = TestServer::start("watch");
    let world = World::new();
    let plugin = server.scratch.path("plugin");
    lease_renewed(&plugin, 0);
    let calls = server.scratch.path("calls");
    let logging = server.scratch.path("logging-herdr");
    fs::write(
        &logging,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{}'\nexec herdr \"$@\"\n",
            calls.display()
        ),
    )
    .expect("the logging herdr");
    fs::set_permissions(&logging, fs::Permissions::from_mode(0o755))
        .expect("the logging herdr made executable");
    let herdr = server
        .profile()
        .with_plugin(Some(plugin))
        .with_program(&logging);
    let terminal: Arc<dyn Terminal> = Arc::new(herdr);
    world.resolver.with_terminal(NAME, terminal.clone());
    let engine = Engine::new(
        world.resolver.clone(),
        terminal,
        Arc::new(StateDir::at(server.scratch.path("state/flume"))),
        world.history.clone(),
        world.host.clone(),
        world.defaults.clone(),
    );
    assert_eq!(
        watch_and_lapse(&engine),
        (Watch::None, None),
        "no caller keeping watch beside a fresh lease: flume runs attended"
    );
    let settled = engine.settle_every(60);
    assert!(settled.refusal().is_none(), "{settled:?}");
    let began = world.host.now();
    assert_eq!(
        watch_and_lapse(&engine),
        (Watch::Bound { seconds: 60 }, None),
        "a caller keeping watch: flume runs unattended"
    );
    world.host.advance(180);
    assert_eq!(
        watch_and_lapse(&engine),
        (Watch::None, Some(began + 120)),
        "three minutes with no settle of the watch's: flume runs attended"
    );
    assert_eq!(
        fs::read_to_string(&calls).unwrap_or_default(),
        "",
        "herdr's calls"
    );
}

/// The longer of the two sockets the pinned herdr binds in a named
/// session's folder, `herdr/sessions/<name>` under the session home,
/// beside `herdr.sock`, whatever `HERDR_SOCKET_PATH` or
/// `HERDR_CLIENT_SOCKET_PATH` says.
const CLIENT_SOCKET: &str = "herdr-client.sock";

/// The longest path a socket's address holds on this host, its closing
/// nul left out.
fn socket_path_limit() -> usize {
    // SAFETY: an address of all zeros is a valid `sockaddr_un`.
    let address: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    address.sun_path.len() - 1
}

/// A test's own scratch home of herdr's named sessions, never the host's:
/// a short link in the run's own scratch (`short_folder`), `SLUICE_TMP`
/// under sluice, or in `/tmp` where the sessions' sockets would not fit
/// under that, to a folder of the test's, so two tests may each name a
/// session `work`. Its servers are stopped and its link removed when it
/// is dropped.
struct SessionHome {
    scratch: Scratch,
    link: PathBuf,
}

impl SessionHome {
    /// The home for a test that starts the sessions `names`: refused, with
    /// the bytes, where no folder the run may write leaves a session's
    /// socket room in a socket's address (record 0047, rule core.51).
    fn new(names: &[&str]) -> SessionHome {
        let scratch = Scratch::new("herdr-sessions");
        fs::create_dir_all(scratch.path("home")).expect("the session home");
        let limit = socket_path_limit();
        let named = format!(
            "c{}-{}",
            std::process::id(),
            LINKS.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        );
        let socket = |folder: &Path, name: &str| {
            folder
                .join(&named)
                .join(format!("herdr/sessions/{name}/{CLIENT_SOCKET}"))
        };
        let fits = |folder: &Path| {
            names
                .iter()
                .all(|name| socket(folder, name).as_os_str().len() <= limit)
        };
        let linked = |folder: &Path| {
            let link = folder.join(&named);
            let _ = fs::remove_file(&link);
            std::os::unix::fs::symlink(scratch.path("home"), &link).map(|()| link)
        };
        // The temporary folder where the sessions' sockets fit under it,
        // else `/tmp` where the run may write there, as a run whose
        // temporary folder is long needs.
        let short = short_folder();
        let link = [short.clone(), PathBuf::from("/tmp")]
            .iter()
            .filter(|folder| fits(folder))
            .find_map(|folder| linked(folder).ok())
            .unwrap_or_else(|| {
                let sockets = names
                    .iter()
                    .map(|name| {
                        let socket = socket(&short, name);
                        format!("{}, {} bytes", socket.display(), socket.as_os_str().len())
                    })
                    .collect::<Vec<_>>()
                    .join("; ");
                panic!(
                    "no folder this run may write, {} or /tmp, leaves herdr's sessions {} room \
                     within the {limit} bytes a socket's path holds on this host: {sockets}",
                    short.display(),
                    names.join(", ")
                )
            });
        SessionHome { scratch, link }
    }

    fn path(&self) -> &Path {
        &self.link
    }

    /// herdr's answer for `args`, run under this home, on the server at
    /// `socket` where one is named.
    fn herdr(&self, socket: Option<&Path>, args: &[&str]) -> Value {
        self.try_herdr(socket, args)
            .unwrap_or_else(|words| panic!("herdr {}: {words}", args.join(" ")))
    }

    /// herdr's answer for `args`, run under this home, on the server at
    /// `socket` where one is named, or its words.
    fn try_herdr(&self, socket: Option<&Path>, args: &[&str]) -> Result<Value, String> {
        let mut command = Command::new("herdr");
        command.args(args);
        for (name, _) in std::env::vars_os() {
            if name.to_string_lossy().starts_with("HERDR_") {
                command.env_remove(name);
            }
        }
        command
            .env(SESSION_HOME, &self.link)
            .env("HOME", self.scratch.path("home"));
        if let Some(socket) = socket {
            command.env("HERDR_SOCKET_PATH", socket);
        }
        let output = command
            .stdin(Stdio::null())
            .output()
            .map_err(|error| error.to_string())?;
        let text = String::from_utf8_lossy(&output.stdout).into_owned();
        match serde_json::from_str::<Value>(text.trim()) {
            Ok(answer) if answer.get("result").is_some() => Ok(answer["result"].clone()),
            Ok(answer) => Ok(answer),
            _ if output.status.success() => Ok(Value::String(text)),
            _ => Err(format!("{text}{}", String::from_utf8_lossy(&output.stderr))),
        }
    }

    /// The sessions herdr lists running under this home, by name, with
    /// their sockets.
    fn running(&self) -> BTreeMap<String, PathBuf> {
        self.try_herdr(None, &["session", "list", "--json"])
            .unwrap_or_default()["sessions"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|session| session["running"] == Value::Bool(true))
            .filter_map(|session| {
                Some((
                    session["name"].as_str()?.to_string(),
                    PathBuf::from(session["socket_path"].as_str()?),
                ))
            })
            .collect()
    }

    /// The socket of the session `name`'s server under this home, started
    /// headless where none runs, as a host's own session runs: with
    /// nothing on its `PATH` but herdr's own folder and the system's, the
    /// scratch's home and its shell that reads no startup file of the
    /// host's.
    fn start(&self, server: &TestServer, name: &str) -> PathBuf {
        if let Some(socket) = self.running().remove(name) {
            return socket;
        }
        let program = herdr_program();
        let path = format!(
            "{}:/usr/bin:/bin",
            program.parent().expect("herdr's folder").display()
        );
        let child = Command::new(&program)
            .args(["--session", name, "server"])
            .env_clear()
            .env("PATH", path)
            .env("HOME", server.scratch.path("home"))
            .env("SHELL", server.scratch.path("bin/shell"))
            .env("TERM", "xterm-256color")
            .env("LANG", "en_US.UTF-8")
            .env(SESSION_HOME, &self.link)
            .current_dir(server.scratch.path("home"))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("herdr's session server starts");
        // Stopped through its socket when the home is dropped.
        drop(child);
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            if let Some(socket) = self.running().remove(name) {
                return socket;
            }
            assert!(
                Instant::now() < deadline,
                "herdr's session {name} did not run"
            );
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// The labels herdr lists of `what`, `workspace` or `tab`, on the
    /// server at `socket`.
    fn labels(&self, socket: &Path, what: &str) -> Vec<String> {
        self.herdr(Some(socket), &[what, "list"])[format!("{what}s")]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|listed| listed["label"].as_str().map(str::to_string))
            .collect()
    }
}

impl Drop for SessionHome {
    fn drop(&mut self) {
        for socket in self.running().values() {
            let _ = self.try_herdr(Some(socket), &["server", "stop"]);
        }
        let deadline = Instant::now() + Duration::from_secs(10);
        while !self.running().is_empty() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(100));
        }
        let _ = fs::remove_file(&self.link);
    }
}

/// The first twelve hex digits of `text`'s SHA-256, as the profile names
/// its records.
fn digest(text: &str) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(text.as_bytes())
        .iter()
        .take(6)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[test]
fn the_pinned_herdr_keeps_its_named_sessions_under_the_session_home() {
    // core.29: the variable `with_session_home` sets is the one the pinned
    // herdr reads its named sessions' home from.
    let home = SessionHome::new(&[]);
    let listed = home.herdr(None, &["session", "list", "--json"]);
    let default = listed["sessions"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|session| session["default"] == Value::Bool(true))
        .unwrap_or_else(|| panic!("no default session in {listed}"));
    assert_eq!(
        default["session_dir"].as_str().map(PathBuf::from),
        Some(home.path().join("herdr")),
        "{listed}"
    );
}

#[test]
fn s82_two_streams_sessions_under_herdr() {
    // 107, S82: each fix's agent in the session the core names, each
    // session's server started headless, its workspace of the base
    // holding the fix's tab and its agent's pane.
    let server = TestServer::start("herdr-streams");
    let home = SessionHome::new(&["work", "personal"]);
    let profile = server.profile().with_session_home(home.path());
    let mut panes = BTreeMap::new();
    for (session, short) in [("work", "fix--one"), ("personal", "fix--two")] {
        let who = identity_of(short, 1);
        let pane = opened_at(
            &server,
            &profile,
            &server.place(short),
            &who,
            &server.command(&[]),
            &OpenIn::Session {
                name: session.into(),
            },
        );
        panes.insert(session, (short, pane));
    }
    let running = home.running();
    for (session, (short, pane)) in &panes {
        let socket = running
            .get(*session)
            .unwrap_or_else(|| panic!("herdr lists no session {session} running: {running:?}"));
        assert_eq!(&pane.socket, socket, "{short}'s pane's server");
        assert_eq!(
            home.labels(socket, "workspace"),
            vec!["repository main".to_string()],
            "{session}'s workspaces"
        );
        assert_eq!(
            home.labels(socket, "tab"),
            vec![short.to_string()],
            "{session}'s tabs"
        );
        assert_eq!(
            home.herdr(Some(socket), &["pane", "get", &pane.id])["pane"]["pane_id"].as_str(),
            Some(pane.id.as_str()),
            "{short}'s agent's pane in {session}"
        );
    }
    assert!(
        !server.tabs().iter().any(|tab| tab.starts_with("fix--")),
        "a fix's tab in flume's own session: {:?}",
        server.tabs()
    );
}

#[test]
fn a_session_name_herdr_refuses_refuses_the_open_in_its_words() {
    // 98, core.29: the name is herdr's to refuse, and nothing of flume's
    // is left open.
    let server = TestServer::start("herdr-bad-session");
    let home = SessionHome::new(&[]);
    let profile = server.profile().with_session_home(home.path());
    let place = server.place("fix--one");
    let who = identity(1);
    let opened = profile.open(
        &place,
        &server.folder(),
        &server.command(&[]),
        &who,
        &OpenIn::Session { name: "a/b".into() },
        None,
    );
    let Opened::Refused { reason } = opened else {
        panic!("open gave {opened:?}");
    };
    assert!(
        reason.contains("session name may only contain"),
        "the refusal is not herdr's: {reason}"
    );
    assert_eq!(profile.pane(&who), None, "the agent's pane recorded");
    assert_eq!(profile.tabs(&place), vec![], "a tab recorded");
    assert_eq!(profile.workspaces(&place), vec![], "a workspace recorded");
    assert_eq!(home.running().get("a/b"), None, "a session left running");
    assert!(
        !server.tabs().contains(&"fix--one".to_string()),
        "a tab made in flume's own session"
    );
}

#[test]
fn a_place_tidied_in_two_sessions_closes_its_tab_and_workspace_in_each() {
    // 137, 138: a place whose first agent opened in flume's own session
    // and its next in its stream's, `work`.
    let server = TestServer::start("herdr-two-tidy");
    let home = SessionHome::new(&["work"]);
    let profile = server.profile().with_session_home(home.path());
    let place = server.place("fix--one");
    opened_in(
        &server,
        &profile,
        &place,
        &identity(1),
        &server.command(&[]),
    );
    let pane = opened_at(
        &server,
        &profile,
        &place,
        &identity(2),
        &server.command(&[]),
        &OpenIn::Session {
            name: "work".into(),
        },
    );
    assert_eq!(profile.tabs(&place).len(), 2, "the place's tabs");
    assert_eq!(profile.workspaces(&place).len(), 2, "the base's workspaces");
    let tidy = profile.tidy(&place);
    assert!(tidy.kept.is_empty(), "kept: {tidy:?}");
    let mut closed = tidy.closed.clone();
    closed.sort();
    assert_eq!(
        closed,
        vec![
            "the tab fix--one".to_string(),
            "the tab fix--one".to_string(),
            "the workspace repository main".to_string(),
            "the workspace repository main".to_string(),
        ],
        "{tidy:?}"
    );
    assert!(
        !server.workspaces().contains(&"repository main".to_string())
            && !server.tabs().contains(&"fix--one".to_string()),
        "flume's own session keeps the place: {:?} {:?}",
        server.workspaces(),
        server.tabs()
    );
    assert!(
        home.labels(&pane.socket, "workspace").is_empty(),
        "work keeps the place's workspace"
    );
    assert_eq!(profile.tabs(&place), vec![], "the tabs' records");
    assert_eq!(
        profile.workspaces(&place),
        vec![],
        "the workspaces' records"
    );
    assert_eq!(profile.kept(), Vec::<String>::new(), "kept");
}

#[test]
fn a_place_recorded_before_records_were_kept_by_server_is_tidied_as_before() {
    // 138, design 2: a tab and a workspace at the keys flume used before
    // are the records of the server they name.
    let server = TestServer::start("herdr-before");
    let profile = server.profile();
    let place = server.place("fix--one");
    opened_in(
        &server,
        &profile,
        &place,
        &identity(1),
        &server.command(&[]),
    );
    let records = server.scratch.path("records");
    let socket = server.socket.display().to_string();
    let moves = [
        (
            records.join("tabs").join(format!(
                "{}.json",
                digest(&format!("{}\n{socket}", place.location))
            )),
            records
                .join("tabs")
                .join(format!("{}.json", digest(&place.location))),
        ),
        (
            records.join("workspaces").join(format!(
                "{}.json",
                digest(&format!("{socket}\n{}\n{}", place.repository, place.base))
            )),
            records.join("workspaces").join(format!(
                "{}.json",
                digest(&format!("{}\n{}", place.repository, place.base))
            )),
        ),
    ];
    for (now, before) in &moves {
        fs::rename(now, before).unwrap_or_else(|error| panic!("{} moved: {error}", now.display()));
    }
    assert_eq!(
        profile.tab(&place).map(|(_, socket)| socket),
        Some(server.socket.clone()),
        "the tab written before"
    );
    let tidy = profile.tidy(&place);
    assert_eq!(
        tidy.closed,
        vec![
            "the tab fix--one".to_string(),
            "the workspace repository main".to_string()
        ],
        "{tidy:?}"
    );
    assert!(
        !server.workspaces().contains(&"repository main".to_string()),
        "the workspace stands"
    );
    for (_, before) in &moves {
        assert!(!before.exists(), "{} is kept", before.display());
    }
}

/// The pinned herdr's full path, as attach is to give it, the links on
/// the way read.
fn pinned_herdr(command: &ViewCommand) -> (PathBuf, PathBuf) {
    (
        Path::new(&command.program)
            .canonicalize()
            .unwrap_or_else(|error| panic!("{}: {error}", command.program)),
        herdr_program(),
    )
}

/// The command attach answered, or a panic naming the answer.
fn command_of(attached: Attached) -> ViewCommand {
    match attached {
        Attached::Command { command } => command,
        other => panic!("attach gave {other:?}"),
    }
}

#[test]
fn a_streams_session_not_yet_running_is_started_for_attach() {
    // 187, S90: the session's server started headless, and the command
    // the pinned herdr's full path with `session attach w` and the
    // scratch home.
    let server = TestServer::start("herdr-attach");
    let home = SessionHome::new(&["w"]);
    let profile = server.profile().with_session_home(home.path());
    assert_eq!(home.running().get("w"), None, "w runs before attach");
    let command = command_of(profile.attach(Some("w"), View::Terminal));
    assert!(
        home.running().contains_key("w"),
        "herdr lists no w running: {:?}",
        home.running()
    );
    let (given, pinned) = pinned_herdr(&command);
    assert_eq!(given, pinned, "the herdr attach names");
    assert!(Path::new(&command.program).is_absolute(), "{command:?}");
    assert_eq!(command.arguments, ["session", "attach", "w"]);
    assert_eq!(
        command.environment,
        BTreeMap::from([(
            SESSION_HOME.to_string(),
            home.path().to_string_lossy().into_owned()
        )])
    );
    assert!(
        !command
            .environment
            .keys()
            .any(|name| name.starts_with("HERDR_")),
        "{command:?}"
    );
}

/// `command` run under a pseudo-terminal of 80 by 24 of the test's own,
/// its own session's leader with the terminal as its controlling one,
/// every `HERDR_*` of the caller's stripped and its environment set; what
/// it writes is kept, for a failure's words.
fn under_a_terminal(command: &ViewCommand) -> (Child, Arc<std::sync::Mutex<Vec<u8>>>) {
    use std::os::fd::{FromRawFd, OwnedFd};
    use std::os::unix::process::CommandExt;
    // SAFETY: the pseudo-terminal's calls each take the descriptor the
    // one before gave, checked; the child's hook calls only setsid and
    // ioctl, which are async-signal-safe.
    unsafe {
        let master = libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY);
        assert!(master >= 0, "a pseudo-terminal");
        assert_eq!(libc::grantpt(master), 0, "grantpt");
        assert_eq!(libc::unlockpt(master), 0, "unlockpt");
        let name = std::ffi::CStr::from_ptr(libc::ptsname(master))
            .to_string_lossy()
            .into_owned();
        let master = OwnedFd::from_raw_fd(master);
        let terminal = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&name)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        let size = libc::winsize {
            ws_row: 24,
            ws_col: 80,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };
        assert_eq!(
            libc::ioctl(
                std::os::fd::AsRawFd::as_raw_fd(&terminal),
                libc::TIOCSWINSZ,
                &size
            ),
            0,
            "the terminal's size set"
        );
        let mut run = Command::new(&command.program);
        run.args(&command.arguments)
            .envs(&command.environment)
            .env("TERM", "xterm-256color")
            .stdin(terminal.try_clone().expect("the terminal"))
            .stdout(terminal.try_clone().expect("the terminal"))
            .stderr(terminal);
        for (variable, _) in std::env::vars_os() {
            if variable.to_string_lossy().starts_with("HERDR_") {
                run.env_remove(variable);
            }
        }
        run.pre_exec(|| {
            libc::setsid();
            libc::ioctl(0, libc::TIOCSCTTY as _, 0);
            Ok(())
        });
        let child = run.spawn().expect("the command attach gave runs");
        let written = Arc::new(std::sync::Mutex::new(vec![]));
        let kept = written.clone();
        std::thread::spawn(move || {
            use std::io::Read;
            let mut master = fs::File::from(master);
            let mut buffer = [0u8; 4096];
            while let Ok(read @ 1..) = master.read(&mut buffer) {
                if let Ok(mut kept) = kept.lock() {
                    kept.extend_from_slice(&buffer[..read]);
                }
            }
        });
        (child, written)
    }
}

#[test]
fn a_client_the_command_runs_attaches_to_its_session() {
    // 187: the command, run under a terminal, is a client still running
    // two seconds on, and herdr lists `w` at the socket attach started,
    // no other server of `w` started.
    let server = TestServer::start("herdr-client");
    let home = SessionHome::new(&["w"]);
    let profile = server.profile().with_session_home(home.path());
    let command = command_of(profile.attach(Some("w"), View::Terminal));
    let started = home.running().get("w").cloned().expect("w running");
    let (mut client, written) = under_a_terminal(&command);
    std::thread::sleep(Duration::from_secs(2));
    let running = client.try_wait().expect("the client read");
    let _ = client.kill();
    let _ = client.wait();
    let written = String::from_utf8_lossy(&written.lock().expect("the output")).into_owned();
    assert_eq!(
        running, None,
        "the client ended within two seconds, writing {written:?}"
    );
    assert_eq!(
        home.running().get("w"),
        Some(&started),
        "w's server after the client"
    );
    assert_eq!(home.running().len(), 1, "{:?}", home.running());
}

#[test]
fn the_own_session_is_attached_by_its_name_or_refused_by_its_socket() {
    // 187: a profile whose own server is the session `flume` attaches it
    // by that name; one whose own server is held by its socket alone
    // refuses, herdr attaching by a session's name.
    let server = TestServer::start("herdr-own");
    let home = SessionHome::new(&[SESSION]);
    let own = Herdr::new(
        Server::Session(SESSION.into()),
        server.scratch.path("records-own"),
    )
    .with_bound(BOUND)
    .with_session_home(home.path());
    let command = command_of(own.attach(None, View::Terminal));
    assert_eq!(command.arguments, ["session", "attach", SESSION]);
    assert!(home.running().contains_key(SESSION), "{:?}", home.running());
    let Attached::Refused { reason } = server.profile().attach(None, View::Terminal) else {
        panic!("a socket's own part attached");
    };
    assert!(
        reason.contains("attaches a client by a session's name"),
        "{reason}"
    );
}

#[test]
fn a_name_herdr_refuses_refuses_attach_in_its_words() {
    // 187; core.29: as open is refused for that name.
    let server = TestServer::start("herdr-attach-bad");
    let home = SessionHome::new(&[]);
    let profile = server.profile().with_session_home(home.path());
    let Attached::Refused { reason } = profile.attach(Some("a/b"), View::Terminal) else {
        panic!("a name herdr refuses attached");
    };
    assert!(
        reason.contains("session name may only contain"),
        "the refusal is not herdr's: {reason}"
    );
    assert_eq!(home.running().get("a/b"), None, "a session left running");
}

#[test]
fn herdrs_app_is_refused_and_starts_no_server() {
    // 187, C.5, S90: herdr 0.9.5 has no app of its own.
    let server = TestServer::start("herdr-app");
    let home = SessionHome::new(&["w"]);
    let profile = server.profile().with_session_home(home.path());
    let Attached::Refused { reason } = profile.attach(Some("w"), View::App) else {
        panic!("herdr's app shown");
    };
    assert!(
        reason.contains("herdr 0.9.5 has no app of its own") && reason.contains("herdr-core"),
        "{reason}"
    );
    assert!(home.running().is_empty(), "{:?}", home.running());
}

/// A profile whose own server is the session `flume`, under `home`, the
/// host user's login environment `server`'s scratch home and `/bin/sh`.
fn own_session(server: &TestServer, home: &SessionHome) -> Herdr {
    Herdr::new(
        Server::Session(SESSION.into()),
        server.scratch.path("records-own"),
    )
    .with_bound(BOUND)
    .with_login(&[
        (
            "HOME".to_string(),
            server.scratch.path("home").to_string_lossy().into_owned(),
        ),
        ("LANG".to_string(), "en_US.UTF-8".to_string()),
        ("SHELL".to_string(), "/bin/sh".to_string()),
    ])
    .with_session_home(home.path())
}

/// Whether herdr's listing under `home` names a session `name`, running
/// or not.
fn listed(home: &SessionHome, name: &str) -> bool {
    home.herdr(None, &["session", "list", "--json"])["sessions"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|session| session["name"].as_str() == Some(name))
}

#[test]
fn runs_of_a_session_not_listed_reads_not_running_and_leaves_it_so() {
    // 201; core.13: runs starts no server, as attach would.
    let server = TestServer::start("herdr-runs-none");
    let home = SessionHome::new(&["w"]);
    let profile = server.profile().with_session_home(home.path());
    assert!(!listed(&home, "w"), "w listed before runs");
    assert_eq!(
        profile.runs(&[Some("w")]),
        vec![SessionRuns::NotRunning { name: "w".into() }]
    );
    assert!(!listed(&home, "w"), "w listed after runs");
    assert!(home.running().is_empty(), "{:?}", home.running());
}

#[test]
fn runs_reads_a_running_session_and_the_own_session_by_its_name() {
    // 107, 201: `w` and the own session `flume`, each started by an
    // agent's open in it, read running, the own by its name.
    let server = TestServer::start("herdr-runs");
    let home = SessionHome::new(&[SESSION, "w"]);
    let profile = own_session(&server, &home);
    opened_in(
        &server,
        &profile,
        &server.place("fix--one"),
        &identity_of("fix--one", 1),
        &server.command(&[]),
    );
    opened_at(
        &server,
        &profile,
        &server.place("fix--two"),
        &identity_of("fix--two", 1),
        &server.command(&[]),
        &OpenIn::Session { name: "w".into() },
    );
    assert_eq!(
        profile.runs(&[Some("w")]),
        vec![SessionRuns::Runs { name: "w".into() }]
    );
    assert_eq!(
        profile.runs(&[None]),
        vec![SessionRuns::Runs {
            name: SESSION.into()
        }]
    );
}

#[test]
fn runs_of_several_sessions_reads_them_in_one_listing() {
    // 200, 201: `w`, `x`, which no listing names, and the own session
    // asked in one call, through a herdr of the test's own that logs each
    // call and runs the pinned herdr, answer from one `session list`.
    let server = TestServer::start("herdr-runs-several");
    let home = SessionHome::new(&[SESSION, "w"]);
    let profile = own_session(&server, &home);
    opened_in(
        &server,
        &profile,
        &server.place("fix--one"),
        &identity_of("fix--one", 1),
        &server.command(&[]),
    );
    opened_at(
        &server,
        &profile,
        &server.place("fix--two"),
        &identity_of("fix--two", 1),
        &server.command(&[]),
        &OpenIn::Session { name: "w".into() },
    );
    let calls = server.scratch.path("calls");
    let logging = server.scratch.path("logging-herdr");
    fs::write(
        &logging,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{}'\nexec herdr \"$@\"\n",
            calls.display()
        ),
    )
    .expect("the logging herdr");
    fs::set_permissions(&logging, fs::Permissions::from_mode(0o755))
        .expect("the logging herdr made executable");
    let logged = own_session(&server, &home).with_program(&logging);
    assert_eq!(
        logged.runs(&[Some("w"), Some("x"), None]),
        vec![
            SessionRuns::Runs { name: "w".into() },
            SessionRuns::NotRunning { name: "x".into() },
            SessionRuns::Runs {
                name: SESSION.into()
            },
        ]
    );
    assert_eq!(
        fs::read_to_string(&calls).expect("the logged calls"),
        "session list --json\n",
        "herdr's calls"
    );
    assert!(!listed(&home, "x"), "x listed after runs");
}

#[test]
fn runs_of_a_herdr_that_does_not_answer_is_unread_within_two_seconds() {
    // 201; core.29: a herdr of the test's own that answers only after ten
    // seconds, its listing bounded at one.
    let scratch = Scratch::new("herdr-runs-silent");
    let silent = scratch.path("herdr");
    fs::write(&silent, "#!/bin/sh\nsleep 10\necho '{\"sessions\":[]}'\n")
        .expect("the stand-in herdr");
    fs::set_permissions(&silent, fs::Permissions::from_mode(0o755))
        .expect("the stand-in herdr made executable");
    let profile = Herdr::new(Server::Session(SESSION.into()), scratch.path("records"))
        .with_program(&silent)
        .with_session_home(scratch.path("home"));
    let began = Instant::now();
    let runs = profile.runs(&[Some("w")]);
    let took = began.elapsed();
    assert!(
        matches!(&runs[..], [SessionRuns::Unread { reason }] if reason.contains("did not answer within 1 seconds")),
        "runs of a herdr that does not answer gave {runs:?}"
    );
    assert!(
        took < Duration::from_secs(2),
        "runs waited {took:?} on herdr"
    );
}

#[test]
fn runs_of_an_own_server_held_by_its_socket_reads_the_session_listed_there() {
    // 201: the own part of a profile held by `w`'s socket runs as `w`; of
    // one held by a socket no session lists, unread naming the socket.
    let server = TestServer::start("herdr-runs-socket");
    let home = SessionHome::new(&["w"]);
    command_of(
        server
            .profile()
            .with_session_home(home.path())
            .attach(Some("w"), View::Terminal),
    );
    let socket = home.running().get("w").cloned().expect("w running");
    let held = Herdr::new(Server::Socket(socket), server.scratch.path("records-w"))
        .with_bound(BOUND)
        .with_session_home(home.path());
    assert_eq!(
        held.runs(&[None]),
        vec![SessionRuns::Runs { name: "w".into() }]
    );
    let unlisted = server.scratch.path("none.sock");
    let none = Herdr::new(
        Server::Socket(unlisted.clone()),
        server.scratch.path("records-none"),
    )
    .with_bound(BOUND)
    .with_session_home(home.path());
    match &none.runs(&[None])[..] {
        [SessionRuns::Unread { reason }] => assert!(
            reason.contains(&unlisted.display().to_string()),
            "the words name no socket: {reason}"
        ),
        other => panic!("runs of a socket no session lists gave {other:?}"),
    }
}

/// A stand-in in a workspace of its own on the server of the session
/// `session` under `home`, that server started first where none runs, as
/// a standing agent stands in a host's session: under the herdr name
/// `name` by herdr's agent start, or, where none is given, run by its
/// kind typed into the pane's shell, as a person starts an agent herdr
/// lists with no name; its pane once herdr lists its agent there and it
/// came up. Its log is kept under its server's socket (132, C.5).
fn standing(server: &TestServer, home: &SessionHome, session: &str, name: Option<&str>) -> Pane {
    let socket = home.start(server, session);
    let made = server.herdr_on(
        &socket,
        &[
            "workspace",
            "create",
            "--cwd",
            &server.scratch.path("home").to_string_lossy(),
            "--env",
            &format!(
                "PATH={}:/usr/bin:/bin",
                server.scratch.path("bin").display()
            ),
            "--env",
            &format!("SHELL={}", server.scratch.path("bin/shell").display()),
            "--env",
            &format!("STAND_IN_LOG={}", server.session_log(&socket).display()),
            "--label",
            name.unwrap_or("unnamed"),
            "--no-focus",
        ],
    );
    let pane = Pane {
        id: made["root_pane"]["pane_id"]
            .as_str()
            .unwrap_or_else(|| panic!("herdr gave no pane in {made}"))
            .to_string(),
        socket: socket.clone(),
    };
    // The pane's shell at its prompt first, as the profile's own start
    // waits for it.
    let timeout = HARNESS_START.as_millis().to_string();
    server.herdr_on(
        &socket,
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
    );
    match name {
        Some(name) => {
            let deadline = Instant::now() + HARNESS_START;
            loop {
                match server.try_herdr_on(
                    &socket,
                    &[
                        "agent",
                        "start",
                        name,
                        "--kind",
                        "claude",
                        "--pane",
                        &pane.id,
                        "--timeout",
                        &timeout,
                    ],
                ) {
                    Ok(_) => break,
                    // The pane's shell not yet at its prompt, as the
                    // profile's own start reads it.
                    Err(words)
                        if words.contains("agent_start_input_failed")
                            && Instant::now() < deadline =>
                    {
                        std::thread::sleep(Duration::from_millis(250))
                    }
                    Err(words) => panic!("herdr agent start {name}: {words}"),
                }
            }
        }
        None => {
            server.herdr_on(&socket, &["pane", "send-text", &pane.id, "claude"]);
            server.herdr_on(&socket, &["pane", "send-keys", &pane.id, "enter"]);
        }
    }
    server.within("herdr lists no agent in the standing pane", || {
        server.herdr_on(&socket, &["agent", "list"])["agents"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|agent| agent["pane_id"].as_str() == Some(pane.id.as_str()))
    });
    server.within("the standing stand-in did not come up", || {
        server
            .session_log(&socket)
            .join(&pane.id)
            .join("started")
            .exists()
    });
    pane
}

#[test]
fn a_standing_agent_is_found_by_its_name_and_reached() {
    // The terminal-profiles spec's "A standing agent found and reached"
    // (95, 132, C.5): a stand-in started in `w` by `herdr agent start
    // w-planner`, given by that name, waiting, at its pane on `w`'s
    // server, and a notice sent there taken.
    let server = TestServer::start("herdr-standing");
    let home = SessionHome::new(&["w"]);
    let profile = server.profile().with_session_home(home.path());
    let pane = standing(&server, &home, "w", Some("w-planner"));
    assert_eq!(
        profile.agents_in(Some("w")),
        AgentsIn::Read {
            agents: vec![NamedAgent {
                name: "w-planner".into(),
                at: pane.session(),
                state: AgentState::Waiting,
            }]
        }
    );
    assert_eq!(
        profile.send_to(&pane.session(), "flume: fix--one code delivered"),
        ToSession::Sent,
        "the notice to the standing agent"
    );
    server.within("the standing agent did not take the notice", || {
        server.kept_at(&pane, "got") == vec!["flume: fix--one code delivered".to_string()]
    });
}

#[test]
fn a_session_that_does_not_run_holds_no_agent_and_is_not_started() {
    // The terminal-profiles spec's "A session that does not run" (132;
    // core.13): `x`, which the listing does not name running, beside `w`,
    // which runs a standing agent.
    let server = TestServer::start("herdr-standing-none");
    let home = SessionHome::new(&["w", "x"]);
    let profile = server.profile().with_session_home(home.path());
    standing(&server, &home, "w", Some("w-planner"));
    assert_eq!(
        profile.agents_in(Some("x")),
        AgentsIn::Read { agents: vec![] }
    );
    assert!(
        !home.running().contains_key("x"),
        "x runs once its agents were asked: {:?}",
        home.running()
    );
    assert!(!listed(&home, "x"), "x listed once its agents were asked");
}

#[test]
fn an_agent_herdr_lists_with_no_name_is_not_given() {
    // The terminal-profiles spec's "An agent with no name" (C.5): beside
    // `w-planner`, a stand-in run by its kind typed into a pane of `w`,
    // which herdr lists with no name.
    let server = TestServer::start("herdr-standing-unnamed");
    let home = SessionHome::new(&["w"]);
    let profile = server.profile().with_session_home(home.path());
    let named = standing(&server, &home, "w", Some("w-planner"));
    let unnamed = standing(&server, &home, "w", None);
    let listed = server.herdr_on(&unnamed.socket, &["agent", "list"]);
    let nameless = listed["agents"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|agent| agent["pane_id"].as_str() == Some(unnamed.id.as_str()))
        .unwrap_or_else(|| panic!("herdr lists no agent in the unnamed pane: {listed}"));
    assert!(
        nameless.get("name").is_none_or(Value::is_null),
        "herdr lists a name for the agent started with none: {nameless}"
    );
    assert_eq!(
        profile.agents_in(Some("w")),
        AgentsIn::Read {
            agents: vec![NamedAgent {
                name: "w-planner".into(),
                at: named.session(),
                state: AgentState::Waiting,
            }]
        }
    );
}

#[test]
fn a_standing_agent_gone_is_not_given() {
    // The terminal-profiles spec's "The agent gone" (132): the stand-in's
    // pane closed, the session running still with another workspace.
    let server = TestServer::start("herdr-standing-gone");
    let home = SessionHome::new(&["w"]);
    let profile = server.profile().with_session_home(home.path());
    let pane = standing(&server, &home, "w", Some("w-planner"));
    server.herdr_on(
        &pane.socket,
        &["workspace", "create", "--label", "kept", "--no-focus"],
    );
    let named = |agents: &AgentsIn| match agents {
        AgentsIn::Read { agents } => Some(
            agents
                .iter()
                .filter(|agent| agent.name == "w-planner")
                .count(),
        ),
        AgentsIn::Unread { .. } => None,
    };
    assert_eq!(named(&profile.agents_in(Some("w"))), Some(1), "before");
    server.herdr_on(&pane.socket, &["pane", "close", &pane.id]);
    server.within("w-planner is given once its pane closed", || {
        named(&profile.agents_in(Some("w"))) == Some(0)
    });
    assert!(home.running().contains_key("w"), "w stopped");
}

#[test]
fn agents_in_of_a_herdr_that_refuses_or_does_not_answer_is_unread_in_its_words() {
    // 132; core.29: a herdr of the test's own that lists `w` and `slow`
    // running, refuses `agent list` on `w`'s socket and answers it on
    // `slow`'s only after ten seconds; and one whose listing answers only
    // after ten seconds; each call bounded at one.
    let scratch = Scratch::new("herdr-agents-unread");
    let scripted = scratch.path("herdr");
    fs::write(
        &scripted,
        r#"#!/bin/sh
case "$1 $2" in
  "session list")
    [ -n "$SLOW_LISTING" ] && sleep 10
    echo '{"sessions":[{"name":"w","running":true,"socket_path":"/nowhere/w.sock"},{"name":"slow","running":true,"socket_path":"/nowhere/slow.sock"}]}' ;;
  "agent list")
    case "$HERDR_SOCKET_PATH" in */slow.sock) sleep 10 ;; esac
    echo '{"error":{"code":"internal_error","message":"the server broke"}}'
    exit 1 ;;
esac
"#,
    )
    .expect("the stand-in herdr");
    fs::set_permissions(&scripted, fs::Permissions::from_mode(0o755))
        .expect("the stand-in herdr made executable");
    let profile = Herdr::new(Server::Session(SESSION.into()), scratch.path("records"))
        .with_program(&scripted)
        .with_bound(Duration::from_secs(1))
        .with_session_home(scratch.path("home"));
    assert_eq!(
        profile.agents_in(Some("w")),
        AgentsIn::Unread {
            reason: "herdr refused (internal_error): the server broke".into()
        }
    );
    let began = Instant::now();
    let slow = profile.agents_in(Some("slow"));
    assert!(
        matches!(&slow, AgentsIn::Unread { reason } if reason.contains("did not answer within 1 seconds")),
        "the agents of a server that does not answer gave {slow:?}"
    );
    assert!(
        began.elapsed() < Duration::from_secs(3),
        "agents_in waited {:?} on herdr",
        began.elapsed()
    );
    let silent = scratch.path("silent-herdr");
    fs::write(
        &silent,
        format!(
            "#!/bin/sh\nSLOW_LISTING=1 exec {} \"$@\"\n",
            scripted.display()
        ),
    )
    .expect("the silent herdr");
    fs::set_permissions(&silent, fs::Permissions::from_mode(0o755))
        .expect("the silent herdr made executable");
    let listing = profile.clone().with_program(&silent).agents_in(Some("w"));
    assert!(
        matches!(&listing, AgentsIn::Unread { reason } if reason.contains("did not answer within 1 seconds")),
        "the agents of a part whose listing does not answer gave {listing:?}"
    );
}

#[test]
fn relay_types_each_of_flumes_key_names_into_a_pane() {
    // 222, C.5's relay row at blueprints b54a9b4, core.29: each of flume's
    // key names, bare and led by ctrl+, alt+ and shift+, typed by the relay
    // into a pane of the pinned herdr running `cat`, herdr taking every
    // name but delete, insert, home, end, pageup and pagedown, which the
    // relay refuses, named, and which the pinned herdr itself still
    // refuses; and of `a`, `home` and `b` together, `cat` reads `a` and `b`.
    let server = TestServer::start("herdr-keys");
    let profile = server.profile();
    let who = identity(1);
    let pane = opened(
        &server,
        &profile,
        &who,
        &server.command(&[("STAND_IN_CAT", "1")]),
    );
    let log = server.scratch.path("log").join(&pane.id);
    fs::write(log.join("go"), "").expect("the stand-in told to run cat");
    server.within("the pane's cat did not run", || log.join("cat").exists());
    let mut relay = profile.relay(&who).expect("the pane is relayed");
    let six = ["delete", "insert", "home", "end", "pageup", "pagedown"];
    let mut bare: Vec<String> = KeyName::named().iter().map(ToString::to_string).collect();
    bare.extend(["a", "Z", "1", "+", "/", "é"].map(str::to_string));
    let mut names = vec![];
    for name in &bare {
        for lead in ["", "ctrl+", "alt+", "shift+"] {
            names.push(format!("{lead}{name}"));
        }
    }
    names.extend(["ctrl+alt+x", "ctrl+shift+tab", "ctrl+alt+shift+delete"].map(str::to_string));
    let mut refused = vec![];
    for name in &names {
        let key: Key = name.parse().unwrap_or_else(|refused| panic!("{refused}"));
        let cannot = six.iter().any(|one| name.ends_with(one));
        match relay.typed(&Typed::Keys(vec![key])) {
            Ok(()) => assert!(!cannot, "{name} was typed"),
            Err(words) if cannot => assert!(
                words.starts_with(&format!("{name} is a key herdr 0.9.5 cannot type")),
                "{name}: {words}"
            ),
            Err(words) => refused.push((name.clone(), words)),
        }
    }
    assert!(refused.is_empty(), "herdr refused {refused:#?}");
    for name in six {
        let words = server
            .try_herdr(&["pane", "send-keys", &pane.id, name])
            .expect_err(name);
        assert!(
            words.contains("invalid_key") || words.contains("unsupported key"),
            "the pinned herdr's refusal of {name}: {words}"
        );
    }
    let cat = log.join("cat");
    let before = fs::read(&cat).expect("what cat read").len();
    let around = Typed::Keys(
        ["a", "home", "b"]
            .map(|name| name.parse().unwrap())
            .to_vec(),
    );
    let words = relay.typed(&around).expect_err("home is refused");
    assert!(
        words.starts_with("home is a key herdr 0.9.5 cannot type"),
        "{words}"
    );
    server.within("cat did not read a and b", || {
        fs::read(&cat).is_ok_and(|read| read.len() >= before + 2)
    });
    assert_eq!(&fs::read(&cat).expect("what cat read")[before..], b"ab");
    assert!(
        matches!(relay.screen(), RelayScreen::Shows { .. }),
        "the pane reads gone once every key was typed"
    );
}

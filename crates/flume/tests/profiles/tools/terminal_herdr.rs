//! The terminal profile over herdr (C.5; design 12; records 0016, 0018)
//! against the pinned herdr, on a server each test starts headless on a
//! scratch socket with a scratch home and a shell that reads no startup
//! file of the host's, so no herdr session of the host's is touched, no
//! `HERDR_*` of the caller's reaches it, and no agent program of the
//! host's is on a pane's `PATH`. The agent is a stand-in: a shell script
//! named `claude`, which herdr's agent start runs by that kind, which
//! reports its own state to herdr, keeps what it received, clears its
//! input on ctrl+c as an agent does, and, as its environment asks, shows
//! the folder-trust question or a warning first.
//!
//! The terminal seam's scenarios run against the profile; then the
//! folder-trust question answered (S39), the bypass warning held (S40), a
//! program that never comes up (S45), a pane found by name after a move
//! (S46), the environment from the pane, the program the settings name, a
//! name taken, the last place on a base (S47), two bases, a person's pane
//! (S48), titles that follow the stage (S49), a conversation beside a
//! named pane, a launch from a herdr pane, herdr's readings, a prompt
//! left typed (S43), a pane herdr could not bring back, long
//! instructions, a herdr call that never answers, a quiet pane not ended
//! and a finished turn nobody looked at (S42); a nix-wrapped Claude Code
//! started from the agent profile, a role run's pane reached by its
//! address after a move, and a recipient on another server reached
//! through its own socket.

use crate::common::*;
use flume::host::config::HostConfig;
use flume::host::RealHost;
use flume::profiles::store::state_dir::StateDir;
use flume::profiles::terminal::herdr::{herdr_name, Herdr, Pane, Server, START_TIMEOUT};
use flume_conformance::engine::world::{kind, stage, World};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::terminal::{self, TerminalHarness};
use flume_core::{Answer, Engine};
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
: > "$log/started"
report idle
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
            std::os::unix::fs::symlink(&scratch.root, &link)
                .unwrap_or_else(|error| panic!("a short link at {}: {error}", link.display()));
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
        let mut command = Command::new("herdr");
        command.args(args);
        for (name, _) in std::env::vars_os() {
            if name.to_string_lossy().starts_with("HERDR_") {
                command.env_remove(name);
            }
        }
        let output = command
            .env("HERDR_SOCKET_PATH", &self.socket)
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
        self.try_herdr(args)
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
        }
    }

    /// What the stand-in that started in `pane` keeps under `name`, line
    /// by line.
    fn kept(&self, pane: &str, name: &str) -> Vec<String> {
        fs::read_to_string(self.scratch.path("log").join(pane).join(name))
            .map(|text| text.lines().map(str::to_string).collect())
            .unwrap_or_default()
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
    let opened = profile.open(place, &server.folder(), command, who, None, None);
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
}

impl Harness {
    fn new() -> Harness {
        let server = TestServer::start("herdr");
        let caller = Pane {
            id: "w9:p9".into(),
            socket: server.socket.clone(),
        };
        let seam = server
            .profile()
            .with_start_timeout(HARNESS_START)
            .with_caller(Some(caller.clone()));
        Harness {
            server,
            seam,
            caller,
            moved: RefCell::default(),
        }
    }
}

impl TerminalHarness for Harness {
    type Seam = Herdr;

    fn seam(&self) -> &Herdr {
        &self.seam
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
            let got = self.server.kept(&pane.id, "got");
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
        None,
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
/// agent start times out: the question is read off the screen and
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
        None,
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
        None,
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
        None,
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
        env: server.command(&[]).env,
        ..Settings::default()
    };
    let place = server.place("fix--one");
    let who = identity(1);
    let command = claude.command(&settings, &place, &who, &Secret("s".into()), "go");
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
        None,
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
        None,
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
            Some(named),
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
        Some("nobody"),
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

#[test]
fn instructions_of_twenty_thousand_characters_reach_the_agent_whole() {
    let server = TestServer::start("herdr-long");
    let profile = server.profile();
    let who = identity(1);
    let instructions: String =
        "keep the lockfile exactly as it stands: 'quotes', $HOME, `ticks` and é; "
            .chars()
            .cycle()
            .take(20_000)
            .collect();
    let mut command = server.command(&[]);
    command.args = vec!["--append-system-prompt".into(), instructions.clone()];
    let pane = opened(&server, &profile, &who, &command);
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
        _settings: &Settings,
        _place: &PlaceRef,
        _identity: &Identity,
        _secret: &Secret,
        prompt: &str,
    ) -> AgentCommand {
        AgentCommand {
            program: self.program.clone(),
            args: vec![],
            env: BTreeMap::from([
                (
                    "STAND_IN_LOG".to_string(),
                    self.log.to_string_lossy().into_owned(),
                ),
                ("PATH".to_string(), "/usr/bin:/bin".to_string()),
            ]),
            prompt_args: vec![prompt.into()],
            folder_trust: None,
            kind: Some("claude".into()),
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
        }),
    );
    server.place("two--one");
    repository
}

/// An engine over `world`'s fakes with herdr for its terminal, the store
/// in the scratch's state directory and the host's own clock.
fn engine(server: &TestServer, world: &World) -> Engine {
    let state = server.scratch.path("state/flume");
    Engine::new(
        world.resolver.clone(),
        Arc::new(server.profile()),
        Arc::new(StateDir::at(&state)),
        world.history.clone(),
        Arc::new(RealHost::new(&state, HostConfig::default(), None)),
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
    let id = PlaceId::new(format!("{repository}/places/two--one"));
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

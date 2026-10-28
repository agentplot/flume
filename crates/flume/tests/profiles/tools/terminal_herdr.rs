//! The terminal profile over herdr (C.5; design 12; record 0016) against
//! the pinned herdr, on a server each test starts headless on a scratch
//! socket with a scratch home, so no herdr session of the host's is
//! touched and no `HERDR_*` of the caller's reaches it. The agent is a
//! stand-in: a shell script herdr sees as a Claude Code agent by its
//! program's name, which reports its own state to herdr, keeps what it
//! received, and clears its input on ctrl+c as an agent does.
//!
//! The terminal seam's scenarios run against the profile; then a
//! conversation beside a named pane, a launch from a herdr pane, a quiet
//! pane not ended, done read as waiting (S42), a prompt left typed (S43),
//! a pane herdr could not bring back, instructions longer than the
//! terminal's line, the folder-trust question, and a herdr call that
//! never answers.

use crate::common::*;
use flume::host::config::HostConfig;
use flume::host::RealHost;
use flume::profiles::store::state_dir::StateDir;
use flume::profiles::terminal::herdr::{display_name, Herdr, Pane, Server};
use flume_conformance::engine::world::{kind, stage, World};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::terminal::{self, TerminalHarness};
use flume_core::{Answer, Engine};
use flume_seams::*;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// The stand-in agent: herdr takes the pane's foreground program for a
/// Claude Code agent by its name, and the script reports its state to
/// herdr with the pane's own `HERDR_*` left out, as herdr's reading of
/// the screen would give it, so herdr lists no agent once it exits.
const STAND_IN: &str = r#"log="$STAND_IN_LOG/$HERDR_PANE_ID"
mkdir -p "$log"
for arg in "$@"; do printf '%s\n' "${#arg}"; done > "$log/args"
pane="$HERDR_PANE_ID"
report() {
  env -u HERDR_PANE_ID -u HERDR_TAB_ID -u HERDR_WORKSPACE_ID -u HERDR_ENV \
    "$HERDR_BIN_PATH" pane report-agent "$pane" --source flume-test --agent claude --state "$1" >/dev/null 2>&1
}
trap 'echo cleared >> "$log/events"' INT
if [ -n "$STAND_IN_TRUST" ]; then
  printf 'Do you trust the files in this folder?\n'
  IFS= read -r answer
  echo "trust:$answer" >> "$log/events"
fi
if [ -n "$STAND_IN_WARNING" ]; then
  printf '%s\n' "$STAND_IN_WARNING"
fi
: > "$log/started"
report idle
while :; do
  IFS= read -r line
  case $? in
    0) printf '%s\n' "$line" >> "$log/got" ;;
    1) exit 0 ;;
  esac
done
"#;

/// The tight bound the tests give herdr's calls: herdr answers a local
/// call in milliseconds, and a frozen server is read within it.
const BOUND: Duration = Duration::from_secs(5);

/// A herdr server of the test's own, headless on a scratch socket.
struct TestServer {
    scratch: Scratch,
    socket: PathBuf,
    child: Option<Child>,
}

impl TestServer {
    fn start(label: &str) -> TestServer {
        let scratch = Scratch::new(label);
        for dir in ["home", "bin", "log", "records", "state"] {
            fs::create_dir_all(scratch.path(dir)).expect("a scratch directory");
        }
        fs::write(scratch.path("bin/stand-in.sh"), STAND_IN).expect("the stand-in written");
        std::os::unix::fs::symlink("/bin/sh", scratch.path("bin/claude"))
            .expect("the stand-in's program");
        let socket = scratch.path("s");
        let mut server = TestServer {
            scratch,
            socket,
            child: None,
        };
        server.run();
        // The operator's own workspace holds the focus, as on a host
        // where someone works, so no pane of flume's is looked at.
        server.herdr(&["workspace", "create", "--label", "home", "--focus"]);
        server
    }

    /// Starts the server, or starts it again on the same socket and home.
    fn run(&mut self) {
        let child = Command::new("herdr")
            .arg("server")
            .env_clear()
            .env("PATH", std::env::var_os("PATH").unwrap_or_default())
            .env("HOME", self.scratch.path("home"))
            .env("SHELL", "/bin/sh")
            .env("TERM", "xterm-256color")
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

    /// The profile on this server.
    fn profile(&self) -> Herdr {
        Herdr::new(
            Server::Socket(self.socket.clone()),
            self.scratch.path("records"),
        )
        .with_bound(BOUND)
    }

    fn place(&self, short: &str) -> PlaceRef {
        let location = self.scratch.path(&format!("repository/places/{short}"));
        fs::create_dir_all(&location).expect("the place's directory");
        PlaceRef {
            repository: self.scratch.path("repository").to_string_lossy().into(),
            location: location.to_string_lossy().into(),
            branch: "fix/one".into(),
            base: "main".into(),
        }
    }

    fn folder(&self) -> String {
        self.scratch.path("state/fix--one").to_string_lossy().into()
    }

    /// The stand-in's command, with `env` over its own.
    fn command(&self, env: &[(&str, &str)]) -> AgentCommand {
        let mut variables: BTreeMap<String, String> = env
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect();
        variables.insert(
            "STAND_IN_LOG".into(),
            self.scratch.path("log").to_string_lossy().into(),
        );
        AgentCommand {
            program: self.scratch.path("bin/claude").to_string_lossy().into(),
            args: vec![self
                .scratch
                .path("bin/stand-in.sh")
                .to_string_lossy()
                .into()],
            env: variables,
            prompt_args: vec![],
            folder_trust: None,
        }
    }

    /// What the stand-in in `pane` keeps under `name`, line by line.
    fn kept(&self, pane: &Pane, name: &str) -> Vec<String> {
        fs::read_to_string(self.scratch.path("log").join(&pane.id).join(name))
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
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.thaw();
        self.stop();
    }
}

fn identity(start: u32) -> Identity {
    Identity {
        repository: "0123456789ab".into(),
        place: "fix--one".into(),
        stage: "code".into(),
        start,
    }
}

/// An agent opened under `identity` and waiting, and its pane.
fn opened(server: &TestServer, profile: &Herdr, who: &Identity, command: &AgentCommand) -> Pane {
    let opened = profile.open(
        &server.place("fix--one"),
        &server.folder(),
        command,
        who,
        None,
    );
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

struct Harness {
    server: TestServer,
    seam: Herdr,
    caller: Pane,
}

impl Harness {
    fn new() -> Harness {
        let server = TestServer::start("herdr");
        let caller = Pane {
            id: "w9:p9".into(),
            socket: server.socket.clone(),
        };
        let seam = server.profile().with_caller(Some(caller.clone()));
        Harness {
            server,
            seam,
            caller,
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
        let pane = self.seam.pane(identity).expect("the pane recorded");
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let got = self.server.kept(&pane, "got");
            if !got.is_empty() || Instant::now() >= deadline {
                return got;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

#[test]
fn the_herdr_profile_passes_the_terminal_scenarios() {
    pass_all("terminal", &terminal::scenarios(), Harness::new);
}

#[test]
fn a_place_has_a_tab_in_flumes_workspace_and_a_conversation_opens_beside_a_named_pane() {
    let server = TestServer::start("herdr-panes");
    let profile = server.profile();
    let first = opened(&server, &profile, &identity(1), &server.command(&[]));
    let tabs = server.herdr(&["tab", "list"]);
    let tab = tabs["tabs"]
        .as_array()
        .unwrap()
        .iter()
        .find(|tab| tab["label"] == "fix--one")
        .expect("no tab named for the place");
    let workspace = server.herdr(&["workspace", "get", tab["workspace_id"].as_str().unwrap()]);
    assert_eq!(workspace["workspace"]["label"], "flume", "the workspace");
    let agent = server.herdr(&["agent", "get", &first.id]);
    assert_eq!(
        agent["agent"]["name"].as_str(),
        Some(display_name(&identity(1)).as_str()),
        "the agent's display name"
    );
    let name = display_name(&identity(1));
    assert!(
        name.len() <= 32
            && name.starts_with(|c: char| c.is_ascii_lowercase())
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || "_-".contains(c)),
        "a name herdr's rule refuses: {name}"
    );
    assert_eq!(profile.close(&identity(1)), Closed::Closed, "close");

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
    );
    assert!(
        matches!(&refused, Opened::Refused { reason } if reason.contains("nobody")),
        "open beside a pane nobody named gave {refused:?}"
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
fn a_prompt_left_typed_is_cleared_before_each_sending() {
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
        server.kept(&pane, "got").len() == 1
    });
    server.herdr(&["pane", "send-text", &pane.id, "the prompt, left typed"]);
    assert_eq!(
        profile.send(&who, "the prompt"),
        Sent::Sent,
        "the second sending"
    );
    server.within("the second sending was not received", || {
        server.kept(&pane, "got").len() == 2
    });
    assert_eq!(
        server.kept(&pane, "got"),
        vec!["the prompt".to_string(), "the prompt".to_string()],
        "what the agent received (S43)"
    );
    assert_eq!(
        server.kept(&pane, "events"),
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
    let opened = profile.open(&place, &server.folder(), &server.command(&[]), &who, None);
    assert!(
        matches!(opened, Opened::Opened { .. }),
        "open gave {opened:?}"
    );
    let pane = profile.pane(&who).expect("the pane recorded");
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
fn instructions_longer_than_the_terminals_line_reach_the_agent_whole() {
    let server = TestServer::start("herdr-long");
    let profile = server.profile();
    let who = identity(1);
    let instructions: String = "keep the lockfile exactly as it stands; "
        .chars()
        .cycle()
        .take(40_000)
        .collect();
    let mut command = server.command(&[]);
    command
        .args
        .extend(["--append-system-prompt".into(), instructions.clone()]);
    let pane = opened(&server, &profile, &who, &command);
    assert_eq!(
        server.kept(&pane, "args"),
        vec!["22".to_string(), "40000".to_string()],
        "the arguments' lengths the agent started with"
    );
    let screen = profile.screen(&who).expect("a screen");
    assert!(
        !screen.contains("keep the lockfile"),
        "the instructions were typed into the pane"
    );
    assert!(
        screen
            .replace('\n', "")
            .contains(&format!("sh {}/herdr/", server.folder())),
        "the screen shows no launcher run: {screen}"
    );
}

#[test]
fn the_folder_trust_question_is_answered_with_the_keys_the_command_names() {
    let server = TestServer::start("herdr-trust");
    let profile = server.profile();
    let who = identity(1);
    let mut command = server.command(&[("STAND_IN_TRUST", "1")]);
    command.folder_trust = Some(FolderTrust {
        question: "Do you trust the files in this folder?".into(),
        answer: "y enter".into(),
    });
    let pane = opened(&server, &profile, &who, &command);
    assert_eq!(
        server.kept(&pane, "events"),
        vec!["trust:y".to_string()],
        "the answer the agent read"
    );
}

#[test]
fn the_screen_of_a_start_held_at_a_dialog() {
    let server = TestServer::start("herdr-screen");
    let profile = server.profile();
    let who = identity(1);
    let warning = "WARNING: running in Bypass Permissions mode";
    opened(
        &server,
        &profile,
        &who,
        &server.command(&[("STAND_IN_WARNING", warning)]),
    );
    let screen = profile.screen(&who).expect("a screen");
    assert!(screen.contains(warning), "the screen: {screen} (S40)");
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
}

/// The stand-in as the agent seam reads it: started once it wrote its
/// mark, a prompt received once it took the prompt's line; it asks
/// nothing and starts no own work.
struct StandIn {
    log: PathBuf,
    program: String,
    script: String,
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
            args: vec![self.script.clone()],
            env: BTreeMap::from([(
                "STAND_IN_LOG".to_string(),
                self.log.to_string_lossy().into_owned(),
            )]),
            prompt_args: vec![prompt.into()],
            folder_trust: None,
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

    fn stop_short(&self, agent: &Identity) -> String {
        format!("to stop short, run flume needs as {agent}")
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
            script: server
                .scratch
                .path("bin/stand-in.sh")
                .to_string_lossy()
                .into(),
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
        repository: repository.clone(),
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
        repository: repository.clone(),
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

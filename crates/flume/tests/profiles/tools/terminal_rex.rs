//! The terminal profile over Rex (C.5) against the pinned Rex, on servers
//! each test starts under a state folder of its own, short enough that
//! every socket under it fits the 104 bytes a socket's path holds, and
//! stops at its end, so no Rex server of the host's, the Rex app's among
//! them, is touched and no `REX_*` of the caller's reaches one (core.29,
//! core.51).

use flume::host::config::HostConfig;
use flume::host::process::listed;
use flume::host::RealHost;
use flume::profiles::store::state_dir::StateDir;
use flume::profiles::terminal::rex::{rex_name, Block, Rex, NAME, PIECE};
use flume_conformance::engine::world::{kind, stage, World};
use flume_core::{Answer, Engine};
use flume_seams::*;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// How many state folders this run has made, which names the next.
static FOLDERS: AtomicU32 = AtomicU32::new(0);

/// The pinned Rex, by its own path: the first `rex` on the test's `PATH`,
/// which the devenv shell leads with the release devenv.lock pins. A run
/// with none fails naming it, and never passes without it (core.22,
/// core.51).
fn rex_program() -> PathBuf {
    let path = std::env::var("PATH").unwrap_or_default();
    path.split(':')
        .filter(|dir| !dir.is_empty())
        .map(|dir| Path::new(dir).join("rex"))
        .find(|candidate| candidate.is_file())
        .and_then(|found| found.canonicalize().ok())
        .expect(
            "the pinned rex on PATH: the devenv shell gives it where Rex's flake packages Rex, \
             and these tests run nowhere else",
        )
}

/// A folder short enough for a state folder whose sockets fit: the
/// temporary folder, which a sandboxed run may write and which sluice
/// keeps short, or `/tmp` where the temporary folder is long.
fn short_folder() -> PathBuf {
    let temporary = std::env::temp_dir();
    if temporary.as_os_str().len() <= 40 {
        temporary
    } else {
        PathBuf::from("/tmp")
    }
}

/// A state folder of the test's own, as flume's is, its Rex profile's
/// records and sockets under `terminal/rex`; every Rex server listening
/// under it stopped, and the folder removed, when it is dropped.
struct RexState {
    root: PathBuf,
}

impl RexState {
    fn new() -> RexState {
        let root = short_folder().join(format!(
            "x{}-{}",
            std::process::id(),
            FOLDERS.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("terminal/rex")).unwrap_or_else(|error| {
            panic!(
                "the Rex tests' state folder {} cannot be made: {error}",
                root.display()
            )
        });
        let root = root.canonicalize().expect("the state folder's path");
        RexState { root }
    }

    /// Where the profile keeps its records and its sessions' sockets.
    fn records(&self) -> PathBuf {
        self.root.join("terminal/rex")
    }

    /// The socket of the session `name`, as the profile names it.
    fn socket(&self, name: &str) -> PathBuf {
        self.records().join(format!("{name}.sock"))
    }

    /// The state folder of the session `name`'s server, beside its socket.
    fn server_state(&self, name: &str) -> PathBuf {
        self.records().join(format!("{name}.state"))
    }

    /// Starts the session `name`'s server as the profile does, restoring
    /// nothing, and gives what Rex answered.
    fn start_server(&self, name: &str) -> Value {
        let state = self.server_state(name);
        rex(
            &self.socket(name),
            &[
                "server",
                "start",
                "--state-dir",
                &state.to_string_lossy(),
                "--restore=false",
                "--json",
            ],
        )
    }

    /// Every socket a Rex server may listen on under the folder.
    fn sockets(&self) -> Vec<PathBuf> {
        let mut sockets = vec![];
        let mut folders = vec![self.root.clone()];
        while let Some(folder) = folders.pop() {
            for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
                let path = entry.path();
                let Ok(kind) = entry.file_type() else {
                    continue;
                };
                if kind.is_dir() {
                    folders.push(path);
                } else if path.extension().is_some_and(|ext| ext == "sock") {
                    sockets.push(path);
                }
            }
        }
        sockets
    }
}

impl Drop for RexState {
    fn drop(&mut self) {
        for socket in self.sockets() {
            stop_server(&socket);
        }
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// The server at `socket` stopped through its socket, and its process
/// ended where it does not stop, a frozen one woken first.
fn stop_server(socket: &Path) {
    let kept = server_pid(socket);
    if let Some(pid) = kept {
        // SAFETY: a signal to a server this test's folder holds.
        unsafe { libc::kill(pid as libc::pid_t, libc::SIGCONT) };
    }
    let pid = try_rex(socket, &["api", "call", "server.status"])
        .ok()
        .and_then(|status| status["pid"].as_i64())
        .or(kept);
    if let Some(pid) = pid {
        // SAFETY: a signal to a server this test's folder holds.
        unsafe { libc::kill(pid as libc::pid_t, libc::SIGCONT) };
    }
    let _ = try_rex(socket, &["server", "stop"]);
    if let Some(pid) = pid {
        let deadline = Instant::now() + Duration::from_secs(10);
        // SAFETY: signal 0 asks only whether the process stands.
        while unsafe { libc::kill(pid as libc::pid_t, 0) } == 0 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(100));
        }
        // SAFETY: a signal to a server this test's folder holds.
        unsafe { libc::kill(pid as libc::pid_t, libc::SIGKILL) };
    }
}

/// The process of the server listening at `socket`, by the pid a start
/// of the profile's or the test's answered, kept beside it.
fn server_pid(socket: &Path) -> Option<i64> {
    fs::read_to_string(socket.with_extension("pid"))
        .ok()?
        .trim()
        .parse()
        .ok()
}

/// Rex's answer for `args` on the server at `socket`, the caller's
/// `REX_*` stripped and no server started of Rex's choosing; or its
/// words where it refuses or does not answer within ten seconds.
fn try_rex(socket: &Path, args: &[&str]) -> Result<Value, String> {
    let mut command = Command::new(rex_program());
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
    let child = command.spawn().map_err(|error| error.to_string())?;
    let pid = child.id();
    let (sent, got) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = sent.send(child.wait_with_output());
    });
    let output = match got.recv_timeout(Duration::from_secs(10)) {
        Ok(output) => output.map_err(|error| error.to_string())?,
        Err(_) => {
            // SAFETY: a signal to the call this test started.
            unsafe { libc::kill(pid as libc::pid_t, libc::SIGKILL) };
            return Err(format!("rex {} did not answer", args.join(" ")));
        }
    };
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    if !output.status.success() {
        return Err(format!(
            "{text}{}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    if text.trim().is_empty() {
        return Ok(Value::Null);
    }
    serde_json::from_str(text.trim()).or(Ok(Value::String(text)))
}

/// Rex's answer for `args` on the server at `socket`.
fn rex(socket: &Path, args: &[&str]) -> Value {
    try_rex(socket, args).unwrap_or_else(|words| panic!("rex {}: {words}", args.join(" ")))
}

#[test]
fn the_pinned_rex_runs_at_api_revision_twelve_or_later() {
    // core.22, C.5: the release the tree pins starts a server of the
    // test's own on a socket under its state folder, restoring nothing,
    // and answers its status at API revision 12 or later.
    let state = RexState::new();
    let socket = state.socket("flume");
    assert!(
        socket.as_os_str().len() <= 104,
        "the state folder leaves its socket {} bytes",
        socket.as_os_str().len()
    );
    let started = state.start_server("flume");
    fs::write(socket.with_extension("pid"), started["pid"].to_string())
        .expect("the server's pid kept");
    assert!(started["pid"].as_i64().is_some(), "start gave {started}");
    let status = rex(&socket, &["api", "call", "server.status"]);
    let revision = status["api_revision"].as_u64().unwrap_or_default();
    assert!(revision >= 12, "server.status gave {status}");
    assert_eq!(status["pid"], started["pid"], "another server answered");
    assert_eq!(status["session_count"], 0, "the server restored sessions");
}

/// The stand-in agent, run as `claude`: it keeps what it was started
/// with and every line it took under its log, by its block, and reports
/// its state to Rex by OSC 7501 as Claude Code does, `app=claude-code`:
/// `idle` once up, `working` from a line's first moment, `done` once the
/// line is taken, or after `STAND_IN_WORK` seconds of work. It reads its
/// terminal a chunk at a time, unechoed, ctrl+c no signal and Enter no
/// line feed: a ctrl+c clears what it held typed, as Claude Code clears
/// its input, a line feed typed is kept in the line, and Enter ends it.
/// A line `report <state>` reports that state alone, `print <n>` prints
/// `n` numbered lines, and `exit` exits. As its environment asks, it
/// shows the folder-trust question first, reports nothing, or exits as
/// it starts.
const STAND_IN: &str = r#"#!/bin/sh
LC_ALL=C
export LC_ALL
log="$STAND_IN_LOG/${REX_BLOCK#block:}"
mkdir -p "$log"
printf '%s\0' "$@" > "$log/argv"
printf '%s\n' "$0" > "$log/program"
if [ -n "$A_SECRET" ]; then printf '%s' "$A_SECRET" > "$log/secret"; fi
if [ -n "$REX_BLOCK" ]; then printf '%s' "$REX_BLOCK" > "$log/block"; fi
report() { printf '\033]7501;state=%s:app=claude-code\033\\' "$1"; }
if [ -n "$STAND_IN_EXIT" ]; then echo "the stand-in exits"; exit 3; fi
if [ -n "$STAND_IN_TRUST" ]; then
  # Claude Code's folder-trust dialog: "No, exit" first and in focus, the
  # focus moved by the arrow keys and confirmed by enter, one key at a
  # time. STAND_IN_TRUST=after reports idle first and draws the dialog
  # only once open has returned.
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
    after) report idle; sleep 12; draw ;;
    *) draw ;;
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
if [ -n "$STAND_IN_SILENT" ]; then echo "the stand-in says nothing to Rex"; while :; do sleep 1; done; fi
: > "$log/started"
report idle
stty -icanon -isig -echo -icrnl min 1 time 0 2>/dev/null
ctrl_c=$(printf '\003')
cr=$(printf '\r')
pending=
while :; do
  chunk=$(dd bs=65536 count=1 2>/dev/null; printf x)
  chunk=${chunk%x}
  [ -z "$chunk" ] && exit 0
  pending=$pending$chunk
  while :; do
    case $pending in
      *"$cr"*)
        line=${pending%%"$cr"*}
        pending=${pending#*"$cr"}
        case $line in
          *"$ctrl_c"*) echo cleared >> "$log/events"; line=${line##*"$ctrl_c"} ;;
        esac
        case $line in
          "report "*) report "${line#report }"; continue ;;
          "print "*) i=0; while [ "$i" -lt "${line#print }" ]; do i=$((i + 1)); echo "row $i"; done; continue ;;
          exit) exit 0 ;;
        esac
        report working
        printf '%s\n' "$line" >> "$log/got"
        if [ -n "$STAND_IN_WORK" ]; then sleep "$STAND_IN_WORK"; fi
        report done ;;
      *"$ctrl_c"*) echo cleared >> "$log/events"; pending=${pending##*"$ctrl_c"} ;;
      *) break ;;
    esac
  done
done
"#;

/// The folder-trust question as the stand-in shows it, and its answer.
const TRUST: &str = "Yes, I trust this folder";

/// The tight bound the tests give Rex's calls: Rex answers a local call
/// in milliseconds, and a frozen server is read within it.
const BOUND: Duration = Duration::from_secs(3);

/// The bound on an agent's start the tests give the profile: room for
/// the stand-in on a loaded host, short for a program that never reports.
const START: Duration = Duration::from_secs(20);

/// A test's own folder for the Rex profile: its state, the stand-in and
/// the programs beside it, the stand-ins' logs, a scratch repository and
/// its places' folders.
struct Fixture {
    state: RexState,
}

impl Fixture {
    fn new() -> Fixture {
        let state = RexState::new();
        for dir in ["home", "bin", "other", "log", "repository", "folders"] {
            fs::create_dir_all(state.root.join(dir)).expect("a scratch folder");
        }
        let fixture = Fixture { state };
        fixture.executable("bin/claude", STAND_IN);
        fixture.executable("other/claude", STAND_IN);
        fixture.executable(
            "bin/wrapper",
            &format!(
                "#!/bin/sh\nexec {} \"$@\"\n",
                fixture.path("other/claude").display()
            ),
        );
        fixture
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.state.root.join(relative)
    }

    fn executable(&self, relative: &str, text: &str) {
        let path = self.path(relative);
        fs::write(&path, text).expect("a script written");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755))
            .expect("the script made executable");
    }

    /// The profile over the fixture's state, the host user's login
    /// environment a scratch home, a UTF-8 locale and `/bin/sh`.
    fn profile(&self) -> Rex {
        Rex::new(self.state.records())
            .with_program(rex_program())
            .with_bound(BOUND)
            .with_start_timeout(START)
            .with_login(&[
                (
                    "HOME".to_string(),
                    self.path("home").to_string_lossy().into_owned(),
                ),
                ("LANG".to_string(), "en_US.UTF-8".to_string()),
                ("SHELL".to_string(), "/bin/sh".to_string()),
            ])
    }

    /// The place `short` of the scratch repository on `base`.
    fn place_on(&self, short: &str, base: &str) -> PlaceRef {
        let location = self.path(&format!("repository/places/{short}"));
        fs::create_dir_all(&location).expect("the place's folder");
        PlaceRef {
            repository: self.path("repository").to_string_lossy().into(),
            location: location.to_string_lossy().into(),
            branch: format!("fix/{short}"),
            base: base.into(),
        }
    }

    fn place(&self, short: &str) -> PlaceRef {
        self.place_on(short, "main")
    }

    /// The place's own folder, where the profile keeps its starts.
    fn folder(&self, short: &str) -> String {
        self.path(&format!("folders/{short}"))
            .to_string_lossy()
            .into()
    }

    /// The stand-in's command, with `env` over its own and a `PATH` that
    /// holds no agent program of the host's.
    fn command(&self, env: &[(&str, &str)]) -> AgentCommand {
        let mut variables = BTreeMap::from([
            (
                "STAND_IN_LOG".to_string(),
                self.path("log").to_string_lossy().into_owned(),
            ),
            ("PATH".to_string(), "/usr/bin:/bin".to_string()),
        ]);
        variables.extend(
            env.iter()
                .map(|(name, value)| (name.to_string(), value.to_string())),
        );
        AgentCommand {
            program: self.path("bin/claude").to_string_lossy().into(),
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

    /// What the stand-in that started in `block` keeps under `name`.
    fn kept_text(&self, block: &str, name: &str) -> Option<String> {
        let id = block.strip_prefix("block:").unwrap_or(block);
        fs::read_to_string(self.path("log").join(id).join(name)).ok()
    }

    /// What the stand-in that started in `block` keeps under `name`,
    /// line by line.
    fn kept(&self, block: &str, name: &str) -> Vec<String> {
        self.kept_text(block, name)
            .map(|text| text.lines().map(str::to_string).collect())
            .unwrap_or_default()
    }

    /// What the stand-in in `block` took, once it took anything, waiting
    /// up to ten seconds.
    fn got(&self, block: &str) -> Vec<String> {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let got = self.kept(block, "got");
            if !got.is_empty() || Instant::now() >= deadline {
                return got;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    /// Rex's answer for `args` on the session `name`'s server.
    fn rex(&self, name: &str, args: &[&str]) -> Value {
        rex(&self.state.socket(name), args)
    }

    /// The pid of the session `name`'s server, kept beside its socket so
    /// the state's guard can wake it.
    fn server_pid(&self, name: &str) -> i32 {
        let socket = self.state.socket(name);
        let pid = self.rex(name, &["api", "call", "server.status"])["pid"]
            .as_i64()
            .expect("the server's pid");
        fs::write(socket.with_extension("pid"), pid.to_string()).expect("the pid kept");
        pid as i32
    }

    /// The session `name`'s server stops answering, as a server that
    /// hangs does; and its pid, to wake it.
    fn freeze(&self, name: &str) -> i32 {
        let pid = self.server_pid(name);
        // SAFETY: a signal to a server this test started under its state.
        unsafe { libc::kill(pid, libc::SIGSTOP) };
        pid
    }

    fn thaw(&self, pid: i32) {
        // SAFETY: a signal to a server this test started under its state.
        unsafe { libc::kill(pid, libc::SIGCONT) };
    }

    /// Starts the session `name`'s server where none runs.
    fn server(&self, name: &str) {
        if try_rex(&self.state.socket(name), &["api", "call", "server.status"]).is_err() {
            self.state.start_server(name);
        }
        self.server_pid(name);
    }

    /// A block the test opens in a Rex session of its own, labelled
    /// `label`, on the session `name`'s server, running `command`, and
    /// its Rex session.
    fn own_block(&self, name: &str, label: &str, command: &[&str]) -> (Block, String) {
        self.server(name);
        let layout = json!({
            "block": {
                "flavor": "com.superlogical.terminal.shell",
                "label": label,
                "options": {
                    "shell": "none",
                    "cwd": self.path("home"),
                    "exit": {"on_completion": false},
                    "command": command,
                },
            }
        });
        let created = self.rex(
            name,
            &[
                "api",
                "call",
                "session.create",
                &json!({"label": format!("the test's {label}"), "initial_windows": [{"window_label": label, "layout": layout}]}).to_string(),
            ],
        );
        let block = created["initial_windows"][0]["block_ids"][0]
            .as_str()
            .expect("the block")
            .to_string();
        (
            Block {
                id: block,
                socket: self.state.socket(name),
            },
            created["session_id"].as_str().expect("the session").into(),
        )
    }

    /// The stand-in opened by the test in a block of its own on the session
    /// `name`'s server, labelled `label`, with `env` over its own, and
    /// waiting once it reported.
    fn stand_in_block(&self, name: &str, label: &str, env: &[(&str, &str)]) -> Block {
        let script = self.path(&format!("folders/{label}-start"));
        let mut text = format!(
            "#!/bin/sh\nexec /usr/bin/env STAND_IN_LOG={} PATH=/usr/bin:/bin",
            self.path("log").display()
        );
        for (variable, value) in env {
            text.push_str(&format!(" {variable}={value}"));
        }
        text.push_str(&format!(" {}\n", self.path("bin/claude").display()));
        fs::create_dir_all(script.parent().expect("a folder")).expect("the folder");
        fs::write(&script, text).expect("the start");
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).expect("executable");
        let (block, _) = self.own_block(name, label, &[&script.to_string_lossy()]);
        let profile = self.profile();
        within("the test's stand-in did not come up waiting", || {
            profile.session_state(&block.session()) == Some(AgentState::Waiting)
        });
        block
    }

    /// The labels of the Rex sessions the session `name`'s server lists.
    fn sessions(&self, name: &str) -> Vec<String> {
        self.rex(name, &["api", "call", "session.list"])["sessions"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|session| session["label"].as_str().map(str::to_string))
            .collect()
    }

    /// Each window of the Rex session `session` on the session `name`'s
    /// server, by its label, with its blocks' ids.
    fn windows(&self, name: &str, session: &str) -> Vec<(String, Vec<String>)> {
        self.rex(name, &["api", "call", "-s", session, "session.view"])["windows"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|window| {
                (
                    window["label"].as_str().unwrap_or_default().to_string(),
                    window["layers"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .flat_map(|layer| layer["blocks"].as_array().into_iter().flatten())
                        .filter_map(|block| block["block_id"].as_str().map(str::to_string))
                        .collect(),
                )
            })
            .collect()
    }
}

/// Waits up to ten seconds for `condition`.
fn within(what: &str, condition: impl Fn() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !condition() {
        assert!(Instant::now() < deadline, "{what}");
        std::thread::sleep(Duration::from_millis(100));
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

/// An agent opened under `who` in `place` where `at` says, carrying
/// `session` where given, waiting; and its block.
fn opened_at(
    fixture: &Fixture,
    profile: &Rex,
    place: &PlaceRef,
    who: &Identity,
    command: &AgentCommand,
    at: &OpenIn,
    session: Option<&str>,
) -> Block {
    let short = Path::new(&place.location)
        .file_name()
        .expect("the place's name")
        .to_string_lossy()
        .into_owned();
    let opened = profile.open(place, &fixture.folder(&short), command, who, at, session);
    assert!(
        matches!(opened, Opened::Opened { .. }),
        "open gave {opened:?}"
    );
    let block = profile.block(who).expect("the block recorded");
    within("the stand-in did not come up waiting", || {
        profile.state(who) == AgentState::Waiting
    });
    block
}

/// An agent opened under `who` in the place `fix--one`, in flume's own
/// session, waiting; and its block.
fn opened(fixture: &Fixture, profile: &Rex, who: &Identity, command: &AgentCommand) -> Block {
    opened_at(
        fixture,
        profile,
        &fixture.place("fix--one"),
        who,
        command,
        &OpenIn::Own,
        None,
    )
}

#[test]
fn a_rex_call_that_never_answers_is_refused_within_its_bound() {
    // 98, C.5; core.29: an agent's server stopped so it answers nothing:
    // the send is refused naming the bound and the state reads unknown,
    // each within its bound, and nothing waits on Rex.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    opened(&fixture, &profile, &who, &fixture.command(&[]));
    let pid = fixture.freeze("flume");
    let began = Instant::now();
    let sent = profile.send(&who, "the prompt");
    let sending = began.elapsed();
    let began = Instant::now();
    let state = profile.state(&who);
    let reading = began.elapsed();
    fixture.thaw(pid);
    assert!(
        matches!(&sent, Sent::Refused { reason } if reason.contains("within 3 seconds")),
        "a send to a server that does not answer gave {sent:?}"
    );
    assert!(sending < BOUND * 2, "the send waited {sending:?} on Rex");
    assert_eq!(state, AgentState::Unknown);
    assert!(reading < BOUND * 2, "the state waited {reading:?} on Rex");
    assert_eq!(
        flume::profiles::terminal::rex::BOUND,
        Duration::from_secs(10)
    );
}

#[test]
fn a_call_from_inside_another_servers_block_reaches_its_own_server_alone() {
    // C.5; core.29: read from a process whose REX_SERVER, REX_SESSION and
    // REX_BLOCK name a block of another server, the profile's calls reach
    // the agent's server alone, and the other's block takes nothing.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let block = opened(&fixture, &profile, &who, &fixture.command(&[]));
    let other = fixture.stand_in_block("other", "theirs", &[]);
    let sessions = fixture.sessions("other");
    let server = format!("unix://{}", other.socket.display());
    // SAFETY: the variables a block gives its programs, set for this
    // test's reading and taken out after; the profile's calls strip them.
    std::env::set_var("REX_SERVER", &server);
    std::env::set_var("REX_BLOCK", &other.id);
    std::env::set_var("REX_SESSION", "session:theirs");
    let state = profile.state(&who);
    let sent = profile.send(&who, "for the agent");
    let screen = profile.screen(&who);
    std::env::remove_var("REX_SERVER");
    std::env::remove_var("REX_BLOCK");
    std::env::remove_var("REX_SESSION");
    assert_eq!(state, AgentState::Waiting);
    assert_eq!(sent, Sent::Sent);
    assert!(screen.is_some(), "the agent's screen was not read");
    assert_eq!(fixture.got(&block.id), vec!["for the agent".to_string()]);
    assert!(
        fixture.kept(&other.id, "got").is_empty(),
        "the other server's block took {:?}",
        fixture.kept(&other.id, "got")
    );
    assert_eq!(
        fixture.sessions("other"),
        sessions,
        "the other server changed"
    );
}

#[test]
fn a_sessions_server_is_started_where_none_answers() {
    // 107, C.5: an agent opened in the session `work`, no server running
    // at its socket: a server runs at <state>/terminal/rex/work.sock,
    // started restoring nothing, and the agent's block is in it.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let socket = fixture.state.socket("work");
    assert!(try_rex(&socket, &["api", "call", "server.status"]).is_err());
    let who = identity(1);
    let block = opened_at(
        &fixture,
        &profile,
        &fixture.place("fix--one"),
        &who,
        &fixture.command(&[]),
        &OpenIn::Session {
            name: "work".into(),
        },
        None,
    );
    fixture.server_pid("work");
    assert_eq!(block.socket, socket);
    let (session, at) = profile
        .base_session(&fixture.place("fix--one"))
        .expect("the base's Rex session");
    assert_eq!(at, socket);
    let windows = fixture.windows("work", &session);
    assert!(
        windows.iter().any(|(_, blocks)| blocks.contains(&block.id)),
        "the block is not in the work server's Rex session: {windows:?}"
    );
    assert!(
        try_rex(
            &fixture.state.socket("flume"),
            &["api", "call", "server.status"]
        )
        .is_err(),
        "flume's own server was started"
    );
}

#[test]
fn a_server_started_again_restores_nothing() {
    // 77, C.5: a session's server holding an agent stopped, then another
    // agent opened in that session: the server is started again with none
    // of the sessions it saved, the first agent reads stopped, and the
    // second opens in a new Rex session of its base.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let work = OpenIn::Session {
        name: "work".into(),
    };
    let place = fixture.place("fix--one");
    let first = identity(1);
    opened_at(
        &fixture,
        &profile,
        &place,
        &first,
        &fixture.command(&[]),
        &work,
        None,
    );
    let (before, _) = profile
        .base_session(&place)
        .expect("the base's Rex session");
    fixture.server_pid("work");
    fixture.rex("work", &["server", "stop"]);
    within("the work server did not stop", || {
        try_rex(
            &fixture.state.socket("work"),
            &["api", "call", "server.status"],
        )
        .is_err()
    });
    assert_eq!(profile.state(&first), AgentState::Stopped);
    let second = identity(2);
    opened_at(
        &fixture,
        &profile,
        &place,
        &second,
        &fixture.command(&[]),
        &work,
        None,
    );
    fixture.server_pid("work");
    let (after, _) = profile
        .base_session(&place)
        .expect("the base's Rex session");
    assert_ne!(
        after, before,
        "the second agent opened in the Rex session saved"
    );
    let listed = fixture.rex("work", &["api", "call", "session.list"]);
    let ids: Vec<&str> = listed["sessions"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|session| session["session_id"].as_str())
        .collect();
    assert_eq!(ids, vec![after.as_str()], "the server restored {listed}");
    assert_eq!(profile.state(&first), AgentState::Stopped);
}

#[test]
fn two_opens_of_one_identity_leave_one_block() {
    // 98, C.5: an identity's agent live, another open under it refused
    // naming it, and two opens at once leave one block of its Rex name;
    // the record holds no variable's value.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let command = fixture.command(&[("A_SECRET", "a-value-no-record-holds")]);
    opened(&fixture, &profile, &who, &command);
    let again = profile.open(
        &fixture.place("fix--one"),
        &fixture.folder("fix--one"),
        &command,
        &who,
        &OpenIn::Own,
        None,
    );
    assert!(
        matches!(&again, Opened::Refused { reason } if reason.contains(&who.to_string())),
        "a second open gave {again:?}"
    );
    let racing = identity(2);
    let answers: Vec<Opened> = std::thread::scope(|scope| {
        let opens: Vec<_> = (0..2)
            .map(|_| {
                scope.spawn(|| {
                    profile.open(
                        &fixture.place("fix--one"),
                        &fixture.folder("fix--one"),
                        &command,
                        &racing,
                        &OpenIn::Own,
                        None,
                    )
                })
            })
            .collect();
        opens
            .into_iter()
            .map(|open| open.join().expect("an open"))
            .collect()
    });
    assert_eq!(
        answers
            .iter()
            .filter(|answer| matches!(answer, Opened::Opened { .. }))
            .count(),
        1,
        "two opens at once gave {answers:?}"
    );
    let (session, _) = profile
        .base_session(&fixture.place("fix--one"))
        .expect("the Rex session");
    let view = fixture.rex("flume", &["api", "call", "-s", &session, "session.view"]);
    for named in [&who, &racing] {
        let label = rex_name(named);
        let count = view["windows"]
            .as_array()
            .into_iter()
            .flatten()
            .flat_map(|window| window["layers"].as_array().into_iter().flatten())
            .flat_map(|layer| layer["blocks"].as_array().into_iter().flatten())
            .filter(|block| block["label"].as_str() == Some(label.as_str()))
            .count();
        assert_eq!(count, 1, "blocks labelled {label}: {view}");
    }
    for entry in fs::read_dir(fixture.state.records().join("agents")).expect("the records") {
        let text = fs::read_to_string(entry.expect("a record").path()).expect("a record");
        assert!(!text.contains("a-value-no-record-holds"), "{text}");
    }
}

#[test]
fn places_on_two_bases_open_in_two_rex_sessions() {
    // 94, 107, C.5: places on main and bolt/the-surfaces of one
    // repository, each opened in flume's own session: its server holds
    // the Rex sessions `<repository> main` and `<repository>
    // bolt/the-surfaces`, each with its place's window and its agent's
    // block.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let main = fixture.place_on("fix--one", "main");
    let bolt = fixture.place_on("fix--two", "bolt/the-surfaces");
    let one = opened_at(
        &fixture,
        &profile,
        &main,
        &identity_of("fix--one", 1),
        &fixture.command(&[]),
        &OpenIn::Own,
        None,
    );
    let two = opened_at(
        &fixture,
        &profile,
        &bolt,
        &identity_of("fix--two", 1),
        &fixture.command(&[]),
        &OpenIn::Own,
        None,
    );
    let mut labels = fixture.sessions("flume");
    labels.sort();
    assert_eq!(
        labels,
        vec![
            "repository bolt/the-surfaces".to_string(),
            "repository main".to_string()
        ]
    );
    for (place, block, label) in [(&main, &one, "fix--one"), (&bolt, &two, "fix--two")] {
        let (session, _) = profile.base_session(place).expect("the base's Rex session");
        assert_eq!(
            fixture.windows("flume", &session),
            vec![(label.to_string(), vec![block.id.clone()])]
        );
    }
}

#[test]
fn a_second_agent_of_a_place_splits_its_window() {
    // 94, 107, C.5: a second agent of a place, opened while its first
    // runs, a split of the place's window, no second window made.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let place = fixture.place("fix--one");
    let first = opened_at(
        &fixture,
        &profile,
        &place,
        &identity(1),
        &fixture.command(&[]),
        &OpenIn::Own,
        None,
    );
    let second_who = Identity {
        stage: "review".into(),
        ..identity(1)
    };
    let second = opened_at(
        &fixture,
        &profile,
        &place,
        &second_who,
        &fixture.command(&[]),
        &OpenIn::Own,
        None,
    );
    let (session, _) = profile
        .base_session(&place)
        .expect("the base's Rex session");
    assert_eq!(
        fixture.windows("flume", &session),
        vec![("fix--one".to_string(), vec![first.id, second.id])]
    );
}

#[test]
fn an_agent_opens_beside_a_named_block() {
    // 107, C.5: a stage's settings naming the block rex:<block>@<socket>:
    // the agent opens in a split beside it, in its Rex session, and no
    // window is recorded; a block named in herdr's form refuses the open
    // naming it.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let (named, session) = fixture.own_block("w", "mine", &["/bin/sleep", "600"]);
    let place = fixture.place("fix--one");
    let block = opened_at(
        &fixture,
        &profile,
        &place,
        &identity(1),
        &fixture.command(&[]),
        &OpenIn::Beside {
            pane: named.session(),
        },
        None,
    );
    assert_eq!(block.socket, named.socket);
    assert_eq!(
        fixture.windows("w", &session),
        vec![("mine".to_string(), vec![named.id.clone(), block.id.clone()])]
    );
    assert_eq!(profile.window(&place), None, "a window was recorded");
    let refused = profile.open(
        &place,
        &fixture.folder("fix--one"),
        &fixture.command(&[]),
        &identity(2),
        &OpenIn::Beside {
            pane: "herdr:w1-p2@/tmp/s".into(),
        },
        None,
    );
    assert!(
        matches!(&refused, Opened::Refused { reason } if reason.contains("herdr:w1-p2@/tmp/s")),
        "{refused:?}"
    );
}

#[test]
fn no_variable_of_the_agent_stands_in_any_processs_arguments() {
    // 29, 32, C.5; core.30: an agent given A_SECRET sees it and Rex's
    // REX_BLOCK; no process of the host holds its value among its
    // arguments, the block's and the server's among them, and the start's
    // script is gone.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let secret = format!("a-secret-{}", std::process::id());
    let block = opened(
        &fixture,
        &profile,
        &who,
        &fixture.command(&[("A_SECRET", &secret)]),
    );
    assert_eq!(fixture.kept_text(&block.id, "secret"), Some(secret.clone()));
    assert_eq!(
        fixture.kept_text(&block.id, "block"),
        Some(block.id.clone())
    );
    let processes = listed().expect("the host's process table");
    let holding: Vec<_> = processes
        .iter()
        .filter(|process| process.args.iter().any(|arg| arg.contains(&secret)))
        .collect();
    assert!(holding.is_empty(), "processes hold the value: {holding:?}");
    let starts = Path::new(&fixture.folder("fix--one")).join("rex");
    let scripts: Vec<PathBuf> = fs::read_dir(&starts)
        .expect("the starts")
        .flatten()
        .map(|start| start.path().join("start"))
        .filter(|script| script.exists())
        .collect();
    assert!(scripts.is_empty(), "the start's script stands: {scripts:?}");
}

#[test]
fn a_program_that_never_reports_is_never_begun_with_the_screen() {
    // 35, S45, C.5: a program that writes no Program Status report within
    // the start's bound: the start never begun with the block's screen,
    // the block closed and nothing of the start left.
    let fixture = Fixture::new();
    let profile = fixture.profile().with_start_timeout(Duration::from_secs(4));
    let who = identity(1);
    let opened = profile.open(
        &fixture.place("fix--one"),
        &fixture.folder("fix--one"),
        &fixture.command(&[("STAND_IN_SILENT", "1")]),
        &who,
        &OpenIn::Own,
        None,
    );
    let Opened::NotComeUp { screen } = opened else {
        panic!("open of a program that never reports gave {opened:?}");
    };
    assert!(
        screen.contains("the stand-in says nothing to Rex"),
        "{screen}"
    );
    assert_eq!(profile.block(&who), None, "the record stands");
    assert_eq!(profile.state(&who), AgentState::Stopped);
    let (session, _) = profile
        .base_session(&fixture.place("fix--one"))
        .expect("the Rex session");
    let blocks: usize = fixture
        .windows("flume", &session)
        .iter()
        .map(|(_, blocks)| blocks.len())
        .sum();
    assert_eq!(blocks, 0, "the block was left open");
}

#[test]
fn a_program_that_exits_as_it_starts_is_never_begun_at_once() {
    // 35, C.5: a program that exits before it reports: never begun with
    // the screen at once, not at the bound.
    let fixture = Fixture::new();
    let profile = fixture
        .profile()
        .with_start_timeout(Duration::from_secs(60));
    let began = Instant::now();
    let opened = profile.open(
        &fixture.place("fix--one"),
        &fixture.folder("fix--one"),
        &fixture.command(&[("STAND_IN_EXIT", "1")]),
        &identity(1),
        &OpenIn::Own,
        None,
    );
    let took = began.elapsed();
    let Opened::NotComeUp { screen } = opened else {
        panic!("open of a program that exits gave {opened:?}");
    };
    assert!(screen.contains("the stand-in exits"), "{screen}");
    assert!(took < Duration::from_secs(30), "the open waited {took:?}");
    assert_eq!(profile.state(&identity(1)), AgentState::Stopped);
}

#[test]
fn a_program_that_runs_another_in_its_place_is_refused_naming_it() {
    // 32, C.5: the program the settings name execs another that reports
    // Program Status: the block closed and the open refused naming what
    // ran.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let mut command = fixture.command(&[]);
    command.program = fixture.path("bin/wrapper").to_string_lossy().into();
    let opened = profile.open(
        &fixture.place("fix--one"),
        &fixture.folder("fix--one"),
        &command,
        &who,
        &OpenIn::Own,
        None,
    );
    let Opened::Refused { reason } = opened else {
        panic!("open of a wrapper gave {opened:?}");
    };
    let other = fixture.path("other/claude");
    assert!(
        reason.contains(&other.to_string_lossy().into_owned()) && reason.contains("wrapper"),
        "{reason}"
    );
    assert_eq!(profile.state(&who), AgentState::Stopped);
    let (session, _) = profile
        .base_session(&fixture.place("fix--one"))
        .expect("the Rex session");
    let blocks: usize = fixture
        .windows("flume", &session)
        .iter()
        .map(|(_, blocks)| blocks.len())
        .sum();
    assert_eq!(blocks, 0, "the block was left open");
}

#[test]
fn s39_the_folder_trust_question_is_answered_and_the_agent_comes_up_under_rex() {
    // 34, S39, C.5: the question on the screen before the program reports
    // is answered yes from the screen, never "No, exit", and the agent
    // comes up.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let block = opened(
        &fixture,
        &profile,
        &identity(1),
        &fixture.command(&[("STAND_IN_TRUST", "1")]),
    );
    assert_eq!(
        fixture.kept(&block.id, "events"),
        vec!["trusted".to_string()]
    );
    assert!(fixture.kept_text(&block.id, "started").is_some());
}

#[test]
fn s39_a_folder_trust_question_drawn_after_the_open_is_answered_at_a_settle_under_rex() {
    // 34, 35, S39: the question drawn only once open has returned, its
    // agent not yet started: answer_trust answers it yes from the screen.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let opened = profile.open(
        &fixture.place("fix--one"),
        &fixture.folder("fix--one"),
        &fixture.command(&[("STAND_IN_TRUST", "after")]),
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
    let block = profile.block(&who).expect("the block recorded");
    let deadline = Instant::now() + Duration::from_secs(30);
    while !profile.answer_trust(&who) {
        assert!(Instant::now() < deadline, "the question was never answered");
        std::thread::sleep(Duration::from_millis(500));
    }
    assert_eq!(
        fixture.kept(&block.id, "events"),
        vec!["trusted".to_string()]
    );
}

#[test]
fn s43_a_prompt_left_typed_is_cleared_before_each_sending_under_rex() {
    // 95, S43; core.11: text left typed in a waiting agent's block, then
    // a prompt sent: the agent takes the prompt alone, what was typed
    // cleared first.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let block = opened(&fixture, &profile, &who, &fixture.command(&[]));
    fixture.rex("flume", &["send", "-b", &block.id, "--", "left typed "]);
    std::thread::sleep(Duration::from_millis(300));
    assert_eq!(profile.send(&who, "the prompt"), Sent::Sent);
    assert_eq!(fixture.got(&block.id), vec!["the prompt".to_string()]);
    assert_eq!(
        fixture.kept(&block.id, "events"),
        vec!["cleared".to_string()]
    );
    within("the agent did not wait again", || {
        profile.state(&who) == AgentState::Waiting
    });
    fixture.rex("flume", &["send", "-b", &block.id, "--", "again typed "]);
    std::thread::sleep(Duration::from_millis(300));
    assert_eq!(profile.send(&who, "the next"), Sent::Sent);
    within("the agent did not take the next", || {
        fixture.kept(&block.id, "got").len() == 2
    });
    assert_eq!(
        fixture.kept(&block.id, "got"),
        vec!["the prompt".to_string(), "the next".to_string()]
    );
}

#[test]
fn a_working_agent_is_sent_a_tell_with_no_ctrl_c() {
    // 95, C.5: a tell sent to an agent that reads working: no ctrl+c
    // pressed, and the agent takes the tell as its next input.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let block = opened(
        &fixture,
        &profile,
        &who,
        &fixture.command(&[("STAND_IN_WORK", "4")]),
    );
    assert_eq!(profile.send(&who, "the prompt"), Sent::Sent);
    within("the agent did not work", || {
        profile.state(&who) == AgentState::Working
    });
    assert_eq!(profile.send(&who, "a tell"), Sent::Sent);
    let deadline = Instant::now() + Duration::from_secs(20);
    while fixture.kept(&block.id, "got").len() < 2 {
        assert!(Instant::now() < deadline, "the tell was never taken");
        std::thread::sleep(Duration::from_millis(200));
    }
    assert_eq!(
        fixture.kept(&block.id, "got"),
        vec!["the prompt".to_string(), "a tell".to_string()]
    );
    assert_eq!(
        fixture.kept(&block.id, "events"),
        vec!["cleared".to_string()],
        "a ctrl+c was pressed while the agent worked"
    );
}

#[test]
fn a_text_past_sixty_thousand_bytes_arrives_whole() {
    // 95, C.5: a prompt of 150,000 bytes, non-ASCII and line feeds among
    // them, taken whole and in order as one prompt.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let block = opened(&fixture, &profile, &who, &fixture.command(&[]));
    let mut text = String::new();
    let mut row = 0;
    while text.len() < 150_000 {
        row += 1;
        text.push_str(&format!(
            "row {row} caf\u{e9} \u{1f600} of the long prompt\n"
        ));
    }
    text.push_str("the end");
    assert!(text.len() > 2 * PIECE);
    assert_eq!(profile.send(&who, &text), Sent::Sent);
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let got = fixture.kept_text(&block.id, "got").unwrap_or_default();
        if got.ends_with("the end\n") {
            assert_eq!(got, format!("{text}\n"), "the prompt as taken");
            break;
        }
        assert!(
            Instant::now() < deadline,
            "the prompt never arrived whole: {} bytes",
            got.len()
        );
        std::thread::sleep(Duration::from_millis(250));
    }
}

/// `line` typed into the block and Enter pressed, as a person types it.
fn typed(block: &Block, line: &str) {
    rex(&block.socket, &["send", "-b", &block.id, "--", line]);
    rex(&block.socket, &["send-key", "-b", &block.id, "enter"]);
}

#[test]
fn rexs_readings_as_the_profile_reads_them() {
    // 96, C.5: an agent reporting idle, working, blocked, done and error,
    // then exiting, read waiting, working, asking, waiting, waiting, then
    // stopped.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let block = opened(&fixture, &profile, &who, &fixture.command(&[]));
    assert_eq!(profile.state(&who), AgentState::Waiting);
    for (state, read) in [
        ("working", AgentState::Working),
        ("blocked", AgentState::Asking),
        ("done", AgentState::Waiting),
        ("error", AgentState::Waiting),
        ("idle", AgentState::Waiting),
    ] {
        typed(&block, &format!("report {state}"));
        within(&format!("{state} was not read {read:?}"), || {
            profile.state(&who) == read
        });
    }
    typed(&block, "exit");
    within("an exited agent was not read stopped", || {
        profile.state(&who) == AgentState::Stopped
    });
}

#[test]
fn the_screen_is_the_blocks_last_rows() {
    // 136, C.5: a block that printed more lines than it has rows: the
    // screen is its last lines, as many as its rows.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let block = opened(&fixture, &profile, &who, &fixture.command(&[]));
    typed(&block, "print 200");
    let rows = fixture.rex("flume", &["block", "call", "-b", &block.id, "size"])["rows"]
        .as_u64()
        .expect("the block's rows") as usize;
    within("the block did not print its rows", || {
        profile
            .screen(&who)
            .is_some_and(|screen| screen.lines().last() == Some("row 200"))
    });
    let screen = profile.screen(&who).expect("the screen");
    let lines: Vec<&str> = screen.lines().collect();
    assert_eq!(lines.len(), rows, "{screen}");
    assert_eq!(lines[0], format!("row {}", 200 - rows + 1));
}

#[test]
fn a_close_rex_cannot_make_is_refused() {
    // 97, 98, C.5: an agent's server stopped so its block cannot be
    // closed, and the agent closed: the close refused, and not reported.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    opened(&fixture, &profile, &who, &fixture.command(&[]));
    let pid = fixture.freeze("flume");
    let closed = profile.close(&who);
    let state = profile.state(&who);
    fixture.thaw(pid);
    assert!(matches!(closed, Closed::Refused { .. }), "{closed:?}");
    assert_ne!(state, AgentState::Stopped);
    assert!(profile.block(&who).is_some(), "the record was dropped");
    assert_eq!(profile.close(&who), Closed::Closed);
    assert_eq!(profile.state(&who), AgentState::Stopped);
}

#[test]
fn s47_the_last_place_on_a_base_closes_its_window_and_rex_session() {
    // 137, 138, S47; core.18: the last place on a base tidied: its window
    // and its base's Rex session closed, and the server still runs.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let place = fixture.place("fix--one");
    opened(&fixture, &profile, &who, &fixture.command(&[]));
    assert_eq!(profile.close(&who), Closed::Closed);
    let tidy = profile.tidy(&place);
    assert_eq!(
        tidy,
        TerminalTidy {
            closed: vec![
                "the window fix--one".to_string(),
                "the Rex session repository main".to_string()
            ],
            kept: vec![],
        }
    );
    assert!(
        fixture.sessions("flume").is_empty(),
        "{:?}",
        fixture.sessions("flume")
    );
    assert!(profile.kept().is_empty());
    assert_eq!(profile.tidy(&place), TerminalTidy::default());
}

#[test]
fn s48_a_persons_block_keeps_the_rex_session_named() {
    // 137, 138, S48; core.18: a person's block opened in a base's Rex
    // session, and the base's last place tidied: the place's window
    // closed, the Rex session kept, and kept naming it with the person's
    // block.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let place = fixture.place("fix--one");
    opened(&fixture, &profile, &who, &fixture.command(&[]));
    let (session, _) = profile
        .base_session(&place)
        .expect("the base's Rex session");
    let mine = fixture.rex(
        "flume",
        &[
            "api",
            "call",
            "-s",
            &session,
            "session.new_window",
            &json!({"window_label": "mine", "focus": false, "layout": {"block": {"flavor": "com.superlogical.terminal.shell", "options": {"shell": "none", "command": ["/bin/sleep", "600"], "exit": {"on_completion": false}}}}}).to_string(),
        ],
    )["block_ids"][0]
        .as_str()
        .expect("the person's block")
        .to_string();
    assert_eq!(profile.close(&who), Closed::Closed);
    let tidy = profile.tidy(&place);
    assert_eq!(tidy.closed, vec!["the window fix--one".to_string()]);
    assert_eq!(tidy.kept.len(), 1, "{tidy:?}");
    assert!(
        tidy.kept[0].contains("repository main") && tidy.kept[0].contains(&mine),
        "{tidy:?}"
    );
    let kept = profile.kept();
    assert_eq!(kept.len(), 1, "{kept:?}");
    assert!(kept[0].contains(&mine), "{kept:?}");
    assert_eq!(
        fixture.sessions("flume"),
        vec!["repository main".to_string()]
    );
}

#[test]
fn a_recipient_block_is_sent_to_and_read_by_its_name() {
    // 43, 95, C.5: a block the test opened, no agent of flume's, sent to
    // and read by rex:<block>@<socket>: waiting, it takes a notice, its
    // held input cleared; working, it is typed into with no ctrl+c;
    // asking, it is refused with nothing typed; gone, it is not found.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let block = fixture.stand_in_block("w", "recipient", &[("STAND_IN_WORK", "3")]);
    let name = block.session();
    assert_eq!(profile.session_state(&name), Some(AgentState::Waiting));
    rex(&block.socket, &["send", "-b", &block.id, "--", "held "]);
    std::thread::sleep(Duration::from_millis(300));
    assert_eq!(profile.send_to(&name, "a notice"), ToSession::Sent);
    assert_eq!(fixture.got(&block.id), vec!["a notice".to_string()]);
    within("the recipient did not work", || {
        profile.session_state(&name) == Some(AgentState::Working)
    });
    assert_eq!(profile.send_to(&name, "a second"), ToSession::Sent);
    let deadline = Instant::now() + Duration::from_secs(20);
    while fixture.kept(&block.id, "got").len() < 2 {
        assert!(
            Instant::now() < deadline,
            "the second notice was never taken"
        );
        std::thread::sleep(Duration::from_millis(200));
    }
    assert_eq!(
        fixture.kept(&block.id, "events"),
        vec!["cleared".to_string()]
    );
    within("the recipient did not wait again", || {
        profile.session_state(&name) == Some(AgentState::Waiting)
    });
    typed(&block, "report blocked");
    within("the recipient did not ask", || {
        profile.session_state(&name) == Some(AgentState::Asking)
    });
    assert!(matches!(
        profile.send_to(&name, "into a question"),
        ToSession::Refused { .. }
    ));
    std::thread::sleep(Duration::from_millis(500));
    assert_eq!(
        fixture.kept(&block.id, "got").len(),
        2,
        "something was typed"
    );
    rex(&block.socket, &["block", "close", &block.id]);
    assert_eq!(profile.session_state(&name), None);
    assert_eq!(profile.send_to(&name, "to nobody"), ToSession::NotFound);
}

#[test]
fn a_role_runs_address_reaches_its_block() {
    // 132, C.5: a role run opened under the address east/ops: a notice to
    // east/ops reaches its block, and no second open carries east/ops
    // while it lives.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let who = identity(1);
    let block = opened_at(
        &fixture,
        &profile,
        &fixture.place("fix--one"),
        &who,
        &fixture.command(&[]),
        &OpenIn::Own,
        Some("east/ops"),
    );
    assert_eq!(profile.session_state("east/ops"), Some(AgentState::Waiting));
    assert_eq!(profile.send_to("east/ops", "a notice"), ToSession::Sent);
    assert_eq!(fixture.got(&block.id), vec!["a notice".to_string()]);
    let second = profile.open(
        &fixture.place("ops--two"),
        &fixture.folder("ops--two"),
        &fixture.command(&[]),
        &identity_of("ops--two", 1),
        &OpenIn::Own,
        Some("east/ops"),
    );
    assert!(
        matches!(&second, Opened::Refused { reason } if reason.contains("east/ops")),
        "{second:?}"
    );
    assert_eq!(profile.close(&who), Closed::Closed);
    assert_eq!(profile.session_state("east/ops"), None);
}

#[test]
fn a_standing_agent_is_given_by_its_label() {
    // 132, 193, C.5: a block labelled ops runs a program that reports idle
    // in the server of the session east: the agents of east give ops
    // waiting at rex:<block>@<socket>, which send_to reaches; a part whose
    // server does not run gives none and starts nothing.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    let block = fixture.stand_in_block("east", "ops", &[]);
    let AgentsIn::Read { agents } = profile.agents_in(Some("east")) else {
        panic!("the agents of east were not read");
    };
    assert_eq!(
        agents,
        vec![NamedAgent {
            name: "ops".into(),
            at: block.session(),
            state: AgentState::Waiting,
        }]
    );
    assert_eq!(profile.send_to(&agents[0].at, "a notice"), ToSession::Sent);
    assert_eq!(fixture.got(&block.id), vec!["a notice".to_string()]);
    assert_eq!(
        profile.agents_in(Some("desk")),
        AgentsIn::Read { agents: vec![] }
    );
    assert!(
        !fixture.state.socket("desk").exists(),
        "a server was started for desk"
    );
}

#[test]
fn runs_reads_three_parts_at_once_and_starts_none() {
    // 200, 201, C.5: work's server runs, desk's does not, and the
    // terminal's own is stopped so it answers nothing: runs gives work
    // running, desk not running and flume not read within about a second,
    // and no server is started at desk's socket.
    let fixture = Fixture::new();
    let profile = fixture.profile();
    fixture.server("work");
    fixture.server("flume");
    let pid = fixture.freeze("flume");
    let began = Instant::now();
    let runs = profile.runs(&[Some("work"), Some("desk"), None]);
    let took = began.elapsed();
    fixture.thaw(pid);
    assert_eq!(
        runs[0],
        SessionRuns::Runs {
            name: "work".into()
        }
    );
    assert_eq!(
        runs[1],
        SessionRuns::NotRunning {
            name: "desk".into()
        }
    );
    assert!(
        matches!(&runs[2], SessionRuns::Unread { reason } if reason.contains("within 1 seconds")),
        "{runs:?}"
    );
    assert!(took < Duration::from_millis(2500), "runs took {took:?}");
    assert!(
        !fixture.state.socket("desk").exists(),
        "desk's server was started"
    );
    assert!(matches!(
        &profile.runs(&[Some("../x")])[0],
        SessionRuns::Unread { reason } if reason.contains("../x")
    ));
}

/// The stand-in as the agent seam reads it: started once it wrote its
/// mark, a prompt received once it took the prompt's line; it asks
/// nothing and starts no own work.
struct StandIn {
    fixture_log: PathBuf,
    program: String,
    terminal: Rex,
}

impl StandIn {
    fn kept(&self, agent: &Identity, name: &str) -> Option<String> {
        let block = self.terminal.block(agent)?;
        let id = block
            .id
            .strip_prefix("block:")
            .unwrap_or(&block.id)
            .to_string();
        fs::read_to_string(self.fixture_log.join(id).join(name)).ok()
    }
}

impl Agent for StandIn {
    fn command(
        &self,
        _folder: &str,
        _settings: &Settings,
        _mode: Option<&str>,
        _own: &BTreeMap<String, Value>,
        _resume: Option<&str>,
        _env: &BTreeMap<String, String>,
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
                    self.fixture_log.to_string_lossy().into_owned(),
                ),
                ("PATH".to_string(), "/usr/bin:/bin".to_string()),
            ]),
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

/// An engine over `world`'s fakes with the Rex profile for its terminal,
/// the host's multiplexer and the resolver's `rex`, the store in the
/// fixture's state and the host's own clock (190).
fn engine(fixture: &Fixture, world: &World) -> Engine {
    let state = fixture.path("flume");
    let rex: Arc<dyn Terminal> = Arc::new(fixture.profile());
    world.resolver.with_terminal(NAME, rex.clone());
    let config = HostConfig {
        terminal: Some(NAME.into()),
        ..HostConfig::default()
    };
    Engine::new(
        world.resolver.clone(),
        rex,
        Arc::new(StateDir::at(&state)),
        world.history.clone(),
        Arc::new(RealHost::new(&state, config, Some("caller".into()))),
        world.defaults.clone(),
    )
}

fn stage_state(fixture: &Fixture, world: &World, place: &PlaceId, name: &str) -> StageState {
    match engine(fixture, world).status(Some(place)) {
        Answer::Status { places, .. } => places[0]
            .stages
            .iter()
            .find(|stage| stage.name == name)
            .map(|stage| stage.state)
            .expect("the stage"),
        other => panic!("status answered {other:?}"),
    }
}

#[test]
fn a_done_agent_with_nothing_delivered_waits_and_no_settle_ends_it() {
    // 39, I1; core.9: the terminal-profiles scenario "A quiet block with
    // nothing delivered": a stage's agent under Rex reports done with no
    // deliverable and no stop-short; its state reads waiting, and no
    // settle ends the stage or its agent.
    let fixture = Fixture::new();
    let world = World::new();
    let repository = fixture.path("repository").to_string_lossy().into_owned();
    world.git.init(&repository, "main");
    world.kinds.set_binding(
        &repository,
        Binding {
            place: Some("trees".into()),
            change: Some("plans".into()),
            merge_queue: Some("queue".into()),
        },
    );
    let mut code = stage("code", Some(Deliverable::Commits), "{{brief}} {{needs}}");
    code.agents = vec![Settings {
        program: Some("stand-in".into()),
        ..Settings::default()
    }];
    world
        .kinds
        .set_kinds(&repository, vec![kind("two", false, vec![code])]);
    world.resolver.bind_agent(
        "stand-in",
        Arc::new(StandIn {
            fixture_log: fixture.path("log"),
            program: fixture.path("bin/claude").to_string_lossy().into(),
            terminal: fixture.profile(),
        }),
    );
    world
        .place
        .locate_under(&fixture.path("data").to_string_lossy());
    let id = PlaceId::new(
        world
            .place
            .location(&repository, "two--one")
            .expect("the place's location"),
    );
    fs::create_dir_all(&id.0).expect("the place's folder");
    let answer = engine(&fixture, &world).launch(&Launch {
        repository: repository.clone(),
        kind: "two".into(),
        name: "one".into(),
        base: "main".into(),
        brief: Some("fix the bug".into()),
        ..Launch::default()
    });
    assert!(answer.refusal().is_none(), "the launch: {answer:?}");
    let deadline = Instant::now() + Duration::from_secs(60);
    while stage_state(&fixture, &world, &id, "code") != StageState::Begun {
        assert!(
            Instant::now() < deadline,
            "the code stage did not begin: {:?}",
            stage_state(&fixture, &world, &id, "code")
        );
        engine(&fixture, &world).settle(Some(&id));
    }
    let code = Identity {
        repository: flume_core::repository_id(&format!("{repository}/.git")),
        place: "two--one".into(),
        stage: "code".into(),
        start: 1,
    };
    let profile = fixture.profile();
    let block = profile.block(&code).expect("the block recorded");
    within("the agent did not finish its turn", || {
        profile.state(&code) == AgentState::Waiting && !fixture.kept(&block.id, "got").is_empty()
    });
    for _ in 0..3 {
        engine(&fixture, &world).settle(Some(&id));
    }
    assert_eq!(
        stage_state(&fixture, &world, &id, "code"),
        StageState::Begun,
        "a quiet stage with nothing delivered (39, I1)"
    );
    assert!(
        try_rex(
            &block.socket,
            &["block", "call", "-b", &block.id, "process"]
        )
        .is_ok(),
        "the quiet agent's block was closed"
    );
    assert_eq!(profile.state(&code), AgentState::Waiting, "the agent");
}

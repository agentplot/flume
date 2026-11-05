//! `flume open` through the built binary under the pinned herdr (186 to
//! 190, S90, S91), on a walkthrough's host whose home is a short link, so
//! the sessions' sockets beneath its configuration home fit, and whose
//! `claude` is a stand-in that tells herdr it is ready and waits. The
//! window programs are stand-ins first on the binary's `PATH`: `open` and
//! `ghostty` for the ghostty profile and `osascript` for Terminal.app's,
//! each recording its words and environment and opening nothing. A
//! stream's view is opened before its places run and again once a fix's
//! agent runs in its session, another stream's under Terminal.app on
//! macOS and refused elsewhere, and the line printed where no terminal app
//! is named, nothing run (S90); and a stream under the printed command
//! beside one under herdr, status giving each agent's terminal and each
//! stream's multiplexer (S91).

use super::walk::*;
use crate::common::window::{shell_words, Run, StandIn};
use serde_json::{json, Value};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Stdio;

/// The stand-in for Claude Code under herdr: it tells herdr it is ready
/// with the pane's own `HERDR_*` left out, draws a line and waits, never
/// acking its start, so its stage stays not yet begun.
const CLAUDE: &str = r#"#!/bin/sh
pane="$HERDR_PANE_ID"
printf 'the stand-in waits\n'
env -u HERDR_PANE_ID -u HERDR_TAB_ID -u HERDR_WORKSPACE_ID -u HERDR_ENV \
  "$HERDR_BIN_PATH" pane report-agent "$pane" --source flume-test --agent claude --state idle >/dev/null 2>&1
while :; do sleep 1; done
"#;

/// The longest socket the pinned herdr binds for a session `w`, beneath
/// the configuration home.
const SESSION_SOCKET: &str = "config/herdr/sessions/w/herdr-client.sock";

/// How many short links this run has made, which names the next.
static LINKS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// The longest path a socket's address holds on this host, its closing
/// nul left out.
fn socket_path_limit() -> usize {
    // SAFETY: an address of all zeros is a valid `sockaddr_un`.
    let address: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    address.sun_path.len() - 1
}

/// The pinned herdr, by its own path.
fn pinned_herdr() -> PathBuf {
    let path = std::env::var("PATH").unwrap_or_default();
    path.split(':')
        .map(|dir| Path::new(dir).join("herdr"))
        .find(|candidate| candidate.is_file())
        .and_then(|found| found.canonicalize().ok())
        .expect("the pinned herdr on PATH")
}

/// Whether `program` is the pinned herdr, by its own path.
fn is_pinned_herdr(program: &str) -> bool {
    Path::new(program).canonicalize().ok() == Some(pinned_herdr())
}

/// A host under herdr: the walkthrough, its home's short link, the
/// window programs' stand-ins, and its sessions' servers stopped when it
/// is dropped.
struct Host {
    walk: Walk,
    link: PathBuf,
    open: StandIn,
    ghostty: StandIn,
    osascript: StandIn,
}

impl Host {
    fn new(label: &str) -> Host {
        let walk = Walk::without_stand_in(label);
        let named = format!(
            "o{}-{}",
            std::process::id(),
            LINKS.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        );
        let limit = socket_path_limit();
        let fits =
            |folder: &Path| folder.join(&named).join(SESSION_SOCKET).as_os_str().len() <= limit;
        let temporary = std::env::temp_dir();
        let folder = if fits(&temporary) {
            temporary
        } else {
            PathBuf::from("/tmp")
        };
        let link = folder.join(&named);
        let bytes = link.join(SESSION_SOCKET).as_os_str().len();
        assert!(
            bytes <= limit,
            "herdr's session w would bind {}, {bytes} bytes, past the {limit} a socket's path \
             holds on this host",
            link.join(SESSION_SOCKET).display()
        );
        let _ = std::fs::remove_file(&link);
        std::os::unix::fs::symlink(walk.home(), &link)
            .unwrap_or_else(|error| panic!("a short link at {}: {error}", link.display()));
        let walk = walk.with_home(link.clone());
        let scratch = walk.repository.scratch.root.clone();
        let claude = scratch.join("bin/claude");
        std::fs::write(&claude, CLAUDE).expect("the stand-in for claude");
        std::fs::set_permissions(&claude, std::fs::Permissions::from_mode(0o755))
            .expect("the stand-in made runnable");
        Host {
            open: StandIn::new(&scratch, "open"),
            ghostty: StandIn::new(&scratch, "ghostty"),
            osascript: StandIn::new(&scratch, "osascript"),
            walk,
            link,
        }
    }

    /// The configuration home the binary's herdr calls run under, which
    /// a window's command names.
    fn config_home(&self) -> String {
        self.link.join("config").to_string_lossy().into_owned()
    }

    /// herdr's answer for `args` under the host's configuration home, on
    /// the server at `socket` where one is named.
    fn herdr(&self, socket: Option<&str>, args: &[&str]) -> Value {
        let mut command = self.walk.command("herdr", &self.walk.repository.dir);
        command.args(args).stdout(Stdio::piped());
        if let Some(socket) = socket {
            command.env("HERDR_SOCKET_PATH", socket);
        }
        let output = command.output().expect("herdr runs");
        let answer: Value = serde_json::from_slice(&output.stdout).unwrap_or(Value::Null);
        answer.get("result").cloned().unwrap_or(answer)
    }

    /// The sessions herdr lists running under the host's configuration
    /// home, by name, with their sockets.
    fn running(&self) -> Vec<(String, String)> {
        self.herdr(None, &["session", "list", "--json"])["sessions"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|session| session["running"] == Value::Bool(true))
            .filter_map(|session| {
                Some((
                    session["name"].as_str()?.to_string(),
                    session["socket_path"].as_str()?.to_string(),
                ))
            })
            .collect()
    }

    /// The window `run` of the ghostty profile opened, by its words and
    /// environment: the command's program, its arguments, its home and the
    /// title.
    fn ghostty_window(&self, run: &Run) -> (String, Vec<String>, Option<String>, String) {
        let mut words = run.words.clone();
        let mut home = None;
        if cfg!(target_os = "macos") {
            assert_eq!(words[..2], ["-na", "Ghostty.app"], "{words:?}");
            let args = words
                .iter()
                .position(|word| word == "--args")
                .unwrap_or_else(|| panic!("no --args in {words:?}"));
            for pair in words[2..args].chunks(2) {
                assert_eq!(pair[0], "--env", "{words:?}");
                if let Some(value) = pair[1].strip_prefix("XDG_CONFIG_HOME=") {
                    home = Some(value.to_string());
                }
            }
            words.drain(..=args);
        } else {
            home = run.environment.get("XDG_CONFIG_HOME").cloned();
        }
        let title = words[0]
            .strip_prefix("--title=")
            .unwrap_or_else(|| panic!("no title first in {words:?}"))
            .to_string();
        assert_eq!(words[1], "-e", "{words:?}");
        (words[2].clone(), words[3..].to_vec(), home, title)
    }

    /// The runs of the ghostty profile's program on this host.
    fn ghostty_runs(&self, at_least: usize) -> Vec<Run> {
        if cfg!(target_os = "macos") {
            self.open.runs()
        } else {
            self.ghostty.runs_at_least(at_least)
        }
    }
}

impl Drop for Host {
    fn drop(&mut self) {
        for (_, socket) in self.running() {
            let _ = self.herdr(Some(&socket), &["server", "stop"]);
        }
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !self.running().is_empty() && std::time::Instant::now() < deadline {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let _ = std::fs::remove_file(&self.link);
    }
}

/// The place's status as the binary gives it.
fn place_status(host: &Host, place: &str) -> Value {
    host.walk.status(place)
}

#[test]
fn s90_a_streams_view_through_the_binary() {
    // 186 to 189, S90: `w`'s view opened before its places run and again
    // once a fix's agent runs in its session, each a ghostty window under
    // its title running the pinned herdr's attach; `o`'s under
    // Terminal.app on macOS and refused elsewhere.
    let host = Host::new("s90");
    host.walk.configure_whole(
        "[host]\nmultiplexer = \"herdr\"\nterminal_app = \"ghostty\"\n\
         [streams.w]\ntitle = \"Work Herdr\"\n\
         [streams.o]\nterminal_app = \"terminal\"\ntitle = \"Office Herdr\"\n",
    );
    let (first, refused) = host.walk.flume_ran(&["open", "w"]);
    assert!(!refused, "the first open: {first}");
    assert_eq!(first["opened"]["window"], json!("opened"), "{first}");
    assert_eq!(
        first["opened"]["title"],
        json!({"title": "Work Herdr", "from": "stream"}),
        "{first}"
    );
    assert_eq!(
        first["opened"]["command"]["environment"],
        json!(["XDG_CONFIG_HOME"]),
        "the answer names the variable alone: {first}"
    );
    assert!(
        !first.to_string().contains(&host.config_home()),
        "the answer gives the home: {first}"
    );
    assert!(
        host.running().iter().any(|(name, _)| name == "w"),
        "w's server: {:?}",
        host.running()
    );
    let place = host.walk.launch(
        "fix",
        "one",
        &["--brief", "fix the parser", "--stream", "w"],
    );
    let status = place_status(&host, &place);
    assert_eq!(
        status["agent"]["session"],
        json!({"name": "w", "from": "stream-name"}),
        "{status}"
    );
    assert_eq!(
        status["agent"]["terminal"],
        json!({"profile": "herdr", "from": "host"}),
        "{status}"
    );
    let (second, refused) = host.walk.flume_ran(&["open", "w"]);
    assert!(!refused, "the second open: {second}");
    let windows = host.ghostty_runs(2);
    assert_eq!(windows.len(), 2, "{windows:?}");
    for run in &windows {
        let (program, arguments, home, title) = host.ghostty_window(run);
        assert!(
            is_pinned_herdr(&program),
            "{program} is not the pinned herdr"
        );
        assert_eq!(arguments, ["session", "attach", "w"]);
        assert_eq!(home.as_deref(), Some(host.config_home().as_str()));
        assert_eq!(title, "Work Herdr");
    }
    let (office, refused) = host.walk.flume_ran(&["open", "o"]);
    if cfg!(target_os = "macos") {
        assert!(!refused, "o's open: {office}");
        let runs = host.osascript.runs();
        assert_eq!(runs.len(), 1, "{runs:?}");
        let words = &runs[0].words;
        let (line, title) = (&words[words.len() - 2], &words[words.len() - 1]);
        assert_eq!(title, "Office Herdr");
        let line = shell_words(line);
        assert_eq!(line[..2], ["exec", "/usr/bin/env"], "{line:?}");
        assert_eq!(line[2], format!("XDG_CONFIG_HOME={}", host.config_home()));
        assert!(is_pinned_herdr(&line[3]), "{line:?}");
        assert_eq!(line[4..], ["session", "attach", "o"]);
    } else {
        assert!(refused, "Terminal.app opened off macOS: {office}");
        assert!(
            office["refused"]["refusal"]["reason"]
                .as_str()
                .is_some_and(|reason| reason.contains("Terminal.app runs only on macOS")),
            "{office}"
        );
        assert!(host.osascript.runs().is_empty());
    }
}

#[test]
fn s90_the_printed_window_through_the_binary() {
    // 188, S90: no terminal app named, the line printed on a line of its
    // own, its variable by name, and nothing run.
    let host = Host::new("s90-printed");
    host.walk
        .configure_whole("[host]\nmultiplexer = \"herdr\"\n[streams.w]\nagents = 1\n");
    let printed = host.walk.person(&["open", "w"]);
    let line = printed
        .lines()
        .last()
        .and_then(|line| line.strip_prefix("run: "))
        .unwrap_or_else(|| panic!("no line to run in {printed}"));
    let words: Vec<&str> = line.split(' ').collect();
    assert_eq!(words[0], "XDG_CONFIG_HOME=<set>", "{line}");
    assert!(is_pinned_herdr(words[1]), "{line}");
    assert_eq!(words[2..], ["session", "attach", "w"]);
    assert!(!printed.contains(&host.config_home()), "{printed}");
    for stand_in in [&host.open, &host.ghostty, &host.osascript] {
        assert!(stand_in.runs_settled().is_empty(), "a window program ran");
    }
}

#[test]
fn s91_a_stream_under_the_printed_command_beside_one_under_herdr() {
    // 190, S91: `w`'s fix's agent in herdr's session `w`, `t`'s fix's
    // command printed, and status giving each agent's terminal and each
    // stream's multiplexer with where it came from.
    let host = Host::new("s91");
    host.walk.configure_whole(
        "[host]\nmultiplexer = \"herdr\"\n[streams.w]\nagents = 1\n\
         [streams.t]\nmultiplexer = \"printed\"\n",
    );
    let under_herdr = host.walk.launch(
        "fix",
        "one",
        &["--brief", "fix the parser", "--stream", "w"],
    );
    let printed = host.walk.launch(
        "fix",
        "two",
        &["--brief", "fix the parser", "--stream", "t"],
    );
    let (status, refused) = host.walk.flume_ran(&["status"]);
    assert!(!refused, "{status}");
    let status = &status["status"];
    let place = |location: &str| -> Value {
        status["places"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|place| place["place"] == location)
            .cloned()
            .unwrap_or_else(|| panic!("no {location} in {status}"))
    };
    let one = place(&under_herdr);
    assert_eq!(
        one["agent"]["terminal"],
        json!({"profile": "herdr", "from": "host"}),
        "{one}"
    );
    assert_eq!(
        one["agent"]["session"],
        json!({"name": "w", "from": "stream-name"}),
        "{one}"
    );
    assert!(one["agent"]["printed"].is_null(), "{one}");
    let two = place(&printed);
    assert_eq!(
        two["agent"]["terminal"],
        json!({"profile": "printed", "from": "stream"}),
        "{two}"
    );
    assert!(two["agent"]["printed"].is_string(), "{two}");
    let socket = host
        .running()
        .into_iter()
        .find(|(name, _)| name == "w")
        .map(|(_, socket)| socket)
        .unwrap_or_else(|| panic!("no session w: {:?}", host.running()));
    let tabs = host.herdr(Some(&socket), &["tab", "list"]);
    assert!(
        tabs["tabs"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|tab| tab["label"] == "fix--one"),
        "w's tabs: {tabs}"
    );
    let stream = |name: &str| -> Value {
        status["host"]["streams"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|stream| stream["name"] == name)
            .cloned()
            .unwrap_or_else(|| panic!("no stream {name} in {status}"))
    };
    assert_eq!(
        stream("w")["multiplexer"],
        json!({"profile": "herdr", "from": "host"})
    );
    assert_eq!(
        stream("t")["multiplexer"],
        json!({"profile": "printed", "from": "stream"})
    );
}

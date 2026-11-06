//! The walkthroughs with the pinned Claude Code under herdr, the
//! check-live layer (flume:live; design 18): flume's built binary with
//! `[host] terminal = "herdr"` on a scratch repository bound to
//! `git-worktree`, `openspec` and `sluice` and landing on a local bare
//! remote, its agents real Claude Code sessions in panes of flume's own
//! herdr session, which flume starts on a server of the walkthrough's
//! scratch home, never the host's, and a test settling as a caller would.
//! A fix lands (S1); a change runs construct, code and verify (S2); a
//! code stage whose agent ends its turn to wait on a suite it started, its
//! pane quiet with no deliverable, is not ended until it delivers (39,
//! S3); and a fix whose host gives the mode `ask` begins in it over the
//! repository's own default, its started ack taken (162, S66); a fix
//! whose code agent stops at its account's usage limit goes on in its own
//! conversation under the pool's other account, whose `projects` is a
//! link to the first's, and lands (S78); and a fix whose code agent's pane
//! holds the place's line, pinned under the prompt by flume's plugin's
//! module, while it works, and lands (56, 60, C.4); and a fix on a host
//! that names its people's settings file for every agent and a fork's
//! attribution for the repository, whose code agent's session runs the
//! file's hook beside flume's plugin's and commits with no trailer, and
//! lands (195, S100, S102, C.4).
//!
//! Run by hand, outside the gate, required by no branch, in the devenv
//! shell, so the pinned Claude Code and herdr lead `PATH`:
//!
//! ```text
//! FLUME_LIVE_CLAUDE_CODE_OAUTH_TOKEN=<token> devenv shell -- moon run flume:live
//! ```
//!
//! The token is a test account's, made by `claude setup-token` signed in
//! to that account; the host's own sign-in, configuration and accounts are
//! never read: each agent runs under the walkthrough's host's pool, whose
//! accounts each give a scratch `CLAUDE_CONFIG_DIR` and that token (144).
//! With no token the
//! walkthroughs fail at once, naming it. `FLUME_LIVE_MODEL` names a model
//! for the agents where the account's default is not wanted.

#[path = "../../common/mod.rs"]
mod common;

// The walkthroughs' host, of which these use a part.
#[allow(dead_code)]
#[path = "../tools/walk.rs"]
mod walk;

mod walkthroughs;

use flume::profiles::store::state_dir::StateDir;
use flume_seams::{PlaceId, StartHeld, Store};
use serde_json::Value;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use walk::Walk;

/// The variable the operator names a test account's token in.
pub const TOKEN: &str = "FLUME_LIVE_CLAUDE_CODE_OAUTH_TOKEN";

/// The variable the operator names the agents' model in, if any.
pub const MODEL: &str = "FLUME_LIVE_MODEL";

/// The repository's Claude Code settings: edits and shell commands taken
/// without a question, since no person answers one here, by the default
/// mode and by allowing every tool a fix or a change uses, so an agent in
/// `ask` asks nothing either; flume's own plugin still refuses a push or a
/// submit (108). A session begins in `default`, the pinned release's word
/// for `manual`, only where its start passed `--permission-mode manual`
/// over this default (162).
const SETTINGS: &str = r#"{
  "permissions": {
    "defaultMode": "acceptEdits",
    "allow": ["Bash", "Edit", "Write", "Read", "Glob", "Grep"]
  }
}
"#;

/// A suite that takes two minutes, as a slow one does.
const SUITE: &str = "sleep 120\necho 'the suite passed'\n";

/// A step that takes a minute, which an agent waits on in one foreground
/// call of its Bash tool: the pinned release refuses a bare `sleep` run
/// there, and runs a script whatever it holds.
const STEP: &str = "sleep 60\necho 'the step is done'\n";

/// One live walkthrough: its host, its herdr session's server stopped
/// and its home's short link removed when it ends.
pub struct Live {
    pub walk: Walk,
    link: PathBuf,
    /// The test account's token, which no answer of flume's may hold.
    token: String,
    /// Each account's scratch configuration home, in the pool's order.
    pub homes: Vec<PathBuf>,
}

impl Live {
    /// A walkthrough under herdr with the pinned Claude Code signed in by
    /// the operator's test token; refused, naming the token, without one.
    pub fn new(label: &str) -> Live {
        Live::with_fix_agents(label, "")
    }

    /// A walkthrough as `new` makes it, the host's `[kinds.fix.agents]`
    /// holding `lines` beside the operator's model, if any (32).
    pub fn with_fix_agents(label: &str, lines: &str) -> Live {
        Live::made(label, lines, 1, |_| String::new())
    }

    /// A walkthrough as `new` makes it, the host's configuration holding
    /// the tables `more` gives beside, from the walk, whose repository's
    /// location and scratch it may name (32, 143).
    pub fn with_host(label: &str, more: impl FnOnce(&Walk) -> String) -> Live {
        Live::made(label, "", 1, more)
    }

    /// A walkthrough as `new` makes it on a fill-first pool of two
    /// accounts, each signed in by the operator's test token with a scratch
    /// configuration home of its own, the second's `projects` a link to the
    /// first's, as a host shares one between its accounts (179).
    pub fn sharing_projects(label: &str) -> Live {
        Live::made(label, "", 2, |_| String::new())
    }

    /// A walkthrough on a fill-first pool of `accounts` accounts, the
    /// host's configuration holding what `more` gives beside.
    fn made(label: &str, lines: &str, accounts: usize, more: impl FnOnce(&Walk) -> String) -> Live {
        let token = std::env::var(TOKEN)
            .ok()
            .filter(|token| !token.trim().is_empty())
            .unwrap_or_else(|| {
                panic!(
                    "flume:live runs Claude Code signed in to a test account, never the host's own: \
                     set {TOKEN} to a token `claude setup-token` made for that account, then run \
                     `{TOKEN}=<token> devenv shell -- moon run flume:live`"
                )
            });
        let walk = Walk::without_stand_in(label);
        // herdr's socket lies beneath the home, which must be short.
        let link = PathBuf::from(format!("/tmp/flume-l-{}-{label}", std::process::id()));
        let _ = std::fs::remove_file(&link);
        std::os::unix::fs::symlink(walk.home(), &link).expect("a short link to the home");
        let walk = walk.with_home(link.clone());

        pinned_claude();
        let homes: Vec<PathBuf> = (1..=accounts)
            .map(|n| {
                let home = walk.repository.scratch.path(&match n {
                    1 => "claude-config".to_string(),
                    _ => format!("claude-config-{n}"),
                });
                std::fs::create_dir_all(&home).expect("the agents' Claude Code folder");
                std::fs::write(
                    home.join(".claude.json"),
                    "{\"hasCompletedOnboarding\": true, \"theme\": \"dark\"}\n",
                )
                .expect("the agents' Claude Code configuration");
                home
            })
            .collect();
        if let [first, others @ ..] = homes.as_slice() {
            if !others.is_empty() {
                std::fs::create_dir_all(first.join("projects")).expect("the first's projects");
            }
            for home in others {
                std::os::unix::fs::symlink(first.join("projects"), home.join("projects"))
                    .expect("a link to the first's projects");
            }
        }
        let model = std::env::var(MODEL)
            .ok()
            .filter(|model| !model.trim().is_empty());
        let accounts: Vec<String> = homes
            .iter()
            .enumerate()
            .map(|(at, home)| {
                let name = match at {
                    0 => "test-account".to_string(),
                    _ => format!("test-account-{}", at + 1),
                };
                format!(
                    "{{ name = {name:?}, program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?}, CLAUDE_CODE_OAUTH_TOKEN = {token:?} }} }}",
                    home.display().to_string()
                )
            })
            .collect();
        let mut text = format!(
            "[host]\nterminal = \"herdr\"\npool = \"live\"\n\n[pools.live]\nstrategy = \"fill-first\"\n\
             accounts = [{}]\n",
            accounts.join(", ")
        );
        for kind in ["fix", "change"] {
            let mut table = model
                .as_ref()
                .map(|model| format!("model = {model:?}\n"))
                .unwrap_or_default();
            if kind == "fix" {
                table.push_str(lines);
            }
            if !table.is_empty() {
                text.push_str(&format!("\n[kinds.{kind}.agents]\n{table}"));
            }
        }
        text.push_str(&more(&walk));
        walk.configure(&text);

        let repository = &walk.repository;
        repository.write(".claude/settings.json", SETTINGS);
        repository.write("suite.sh", SUITE);
        repository.write("step.sh", STEP);
        repository.commit("Claude Code's settings, the slow suite and the minute's step");
        repository.git(&["push", "--quiet", "origin", "main"]);
        Live {
            walk,
            link,
            token,
            homes,
        }
    }

    /// The socket of flume's own herdr session's server.
    fn socket(&self) -> String {
        let listing = self
            .walk
            .command("herdr", &self.walk.repository.dir)
            .args(["session", "list", "--json"])
            .output()
            .expect("herdr runs");
        let sessions: Value = serde_json::from_slice(&listing.stdout).unwrap_or(Value::Null);
        sessions["sessions"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|session| session["name"] == "flume")
            .and_then(|session| session["socket_path"].as_str())
            .unwrap_or_else(|| panic!("no herdr session flume: {sessions}"))
            .to_string()
    }

    /// herdr run with `args` on flume's own session's server.
    fn on_server(&self, args: &[&str]) -> std::process::Output {
        self.walk
            .command("herdr", &self.walk.repository.dir)
            .args(args)
            .env("HERDR_SOCKET_PATH", self.socket())
            .output()
            .expect("herdr runs")
    }

    /// herdr's answer on flume's own session's server, as JSON.
    pub fn herdr(&self, args: &[&str]) -> Value {
        let output = self.on_server(args);
        serde_json::from_slice::<Value>(&output.stdout)
            .map(|answer| answer["result"].clone())
            .unwrap_or_else(|_| {
                panic!(
                    "herdr {}: {}{}",
                    args.join(" "),
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                )
            })
    }

    /// The text the pane shows, as `pane read <pane> --source visible
    /// --format text` gives it on flume's own session's server.
    pub fn screen(&self, pane: &str) -> String {
        let output = self.on_server(&[
            "pane", "read", pane, "--source", "visible", "--format", "text",
        ]);
        assert!(
            output.status.success(),
            "herdr pane read {pane}: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    /// The labels of the workspaces flume's herdr session holds.
    pub fn workspaces(&self) -> Vec<String> {
        self.herdr(&["workspace", "list"])["workspaces"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|workspace| workspace["label"].as_str().map(str::to_string))
            .collect()
    }

    /// Settles the place every five seconds until `holds` holds of its
    /// status, for up to `minutes`; refused at once where a stage ended
    /// with no deliverable or a turn failed on the account.
    pub fn until(
        &self,
        place: &str,
        what: &str,
        minutes: u64,
        holds: impl Fn(&Value) -> bool,
    ) -> Value {
        self.until_every(place, what, minutes, Duration::from_secs(5), holds)
    }

    /// As `until`, settling every `pause`, for a state a place may pass
    /// through within seconds.
    pub fn until_every(
        &self,
        place: &str,
        what: &str,
        minutes: u64,
        pause: Duration,
        holds: impl Fn(&Value) -> bool,
    ) -> Value {
        self.walk.until_within(
            place,
            what,
            Duration::from_secs(minutes * 60),
            pause,
            |status| {
                // Checked before anything prints the status (32, 56).
                assert!(
                    !status.to_string().contains(&self.token),
                    "{place}'s status holds the value of the test account's token"
                );
                failing(place, status);
                holds(status)
            },
        )
    }

    /// Settles the place until it is tidied, its work landed.
    pub fn until_landed(&self, place: &str, minutes: u64) {
        self.until(place, "landed and tidied", minutes, Value::is_null);
    }

    /// flume's store in the walkthrough's home.
    fn store(&self) -> StateDir {
        StateDir::at(self.walk.home().join("state/flume"))
    }

    /// The place's own folder in flume's store, where its agents' records
    /// are.
    pub fn folder(&self, place: &str) -> String {
        self.store().folder(&PlaceId::new(place))
    }

    /// The start of the place's live agent, as flume holds it: its
    /// identity and secret among it, which the agent's environment carries
    /// to its hooks (30).
    pub fn live_start(&self, place: &str) -> StartHeld {
        let held = self
            .store()
            .read(&PlaceId::new(place))
            .unwrap_or_else(|| panic!("no held state of {place}"))
            .state;
        held.stages
            .into_iter()
            .filter_map(|stage| stage.start)
            .find(|start| start.live)
            .unwrap_or_else(|| panic!("no live start in {place}"))
    }

    /// The plugin's hook for `event` run as the pinned Claude Code runs it
    /// in the session of `start` in the place, with `input`, the variables
    /// the start gives its agent's hooks over the walkthrough's environment;
    /// it exits 0, as each of the plugin's hooks does.
    pub fn hook(&self, place: &str, start: &StartHeld, event: &str, input: &Value) {
        let mut hook = self
            .walk
            .command(env!("CARGO_BIN_EXE_flume"), Path::new(place))
            .args(["hook", event])
            .env("FLUME_AGENT", start.identity.to_string())
            .env("FLUME_SECRET", &start.secret.0)
            .env("FLUME_PLACE", place)
            .env("FLUME_STAGE", &start.identity.stage)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("flume's hook runs");
        hook.stdin
            .take()
            .expect("the hook's input")
            .write_all(input.to_string().as_bytes())
            .expect("the hook's input written");
        let output = hook.wait_with_output().expect("the hook ends");
        assert!(
            output.status.success(),
            "the hook {event}: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

impl Drop for Live {
    fn drop(&mut self) {
        let dir = &self.walk.repository.dir;
        let listing = self
            .walk
            .command("herdr", dir)
            .args(["session", "list", "--json"])
            .output();
        let socket = listing
            .ok()
            .and_then(|listing| serde_json::from_slice::<Value>(&listing.stdout).ok())
            .and_then(|sessions| {
                sessions["sessions"]
                    .as_array()?
                    .iter()
                    .find(|session| session["name"] == "flume" && session["running"] == true)?
                    .get("socket_path")?
                    .as_str()
                    .map(str::to_string)
            });
        if let Some(socket) = socket {
            let _ = self
                .walk
                .command("herdr", dir)
                .args(["server", "stop"])
                .env("HERDR_SOCKET_PATH", socket)
                .output();
        }
        let _ = std::fs::remove_file(&self.link);
    }
}

/// The walkthrough refused where the place's status shows a stage ended
/// with nothing delivered, or its agent's turn failed on the account, so
/// it does not wait out its limit.
fn failing(place: &str, status: &Value) {
    if status.is_null() {
        return;
    }
    for stage in status["stages"].as_array().into_iter().flatten() {
        let state = stage["state"].as_str().unwrap_or_default();
        assert!(
            !["never_begun", "stuck", "lost", "stopped", "refused"].contains(&state),
            "{place}'s stage {} is {state}: {status}",
            stage["name"]
        );
    }
    let words = status["agent"]["last_words"].as_str().unwrap_or_default();
    for refused in ["authentication_failed", "billing_error"] {
        assert!(
            !words.contains(refused),
            "Claude Code refused the test account's sign-in ({refused}): set {TOKEN} to a token \
             `claude setup-token` made for a test account; the agent's last words: {words}"
        );
    }
}

/// The pinned Claude Code, as `devenv.nix` states its version, is the
/// first `claude` on `PATH`, nix's wrapper, whose wrapped program and
/// variables the agent profile runs for the account's program `claude`.
fn pinned_claude() {
    let nix =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../devenv.nix"))
            .expect("devenv.nix");
    let pinned = nix
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("claudeVersion = \"")?
                .split('"')
                .next()
                .map(str::to_string)
        })
        .expect("devenv.nix pins Claude Code's version");
    let found = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
        .map(|dir| dir.join("claude"))
        .find(|candidate| candidate.is_file())
        .expect("a claude on PATH: run in the devenv shell");
    let version = Command::new(&found)
        .arg("--version")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
        .unwrap_or_default();
    assert!(
        version.starts_with(&pinned),
        "the first claude on PATH, {}, is {version:?}, not the pinned {pinned}: run in the devenv shell",
        found.display()
    );
}

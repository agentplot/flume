//! The walkthroughs with the pinned Claude Code under herdr, the
//! check-live layer (flume:live; design 18): flume's built binary with
//! `[host] terminal = "herdr"` on a scratch repository bound to
//! `git-worktree`, `openspec` and `sluice` and landing on a local bare
//! remote, its agents real Claude Code sessions in panes of flume's own
//! herdr session, which flume starts on a server of the walkthrough's
//! scratch home, never the host's, and a test settling as a caller would.
//! A fix lands (S1); a change runs construct, code and verify (S2); and a
//! code stage whose agent ends its turn to wait on a suite it started, its
//! pane quiet with no deliverable, is not ended until it delivers (39,
//! S3).
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
//! never read: each agent runs with a scratch `CLAUDE_CONFIG_DIR` and that
//! token, given by the walkthrough's host configuration. With no token the
//! walkthroughs fail at once, naming it. `FLUME_LIVE_MODEL` names a model
//! for the agents where the account's default is not wanted.

#[path = "../../common/mod.rs"]
mod common;

// The walkthroughs' host, of which these use a part.
#[allow(dead_code)]
#[path = "../tools/walk.rs"]
mod walk;

mod walkthroughs;

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use walk::Walk;

/// The variable the operator names a test account's token in.
pub const TOKEN: &str = "FLUME_LIVE_CLAUDE_CODE_OAUTH_TOKEN";

/// The variable the operator names the agents' model in, if any.
pub const MODEL: &str = "FLUME_LIVE_MODEL";

/// The repository's Claude Code settings: edits and shell commands taken
/// without a question, since no person answers one here; flume's own
/// plugin still refuses a push or a submit (108).
const SETTINGS: &str = r#"{
  "permissions": {
    "defaultMode": "acceptEdits",
    "allow": ["Bash", "Edit", "Write", "Read", "Glob", "Grep"]
  }
}
"#;

/// A suite that takes two minutes, as a slow one does.
const SUITE: &str = "sleep 120\necho 'the suite passed'\n";

/// One live walkthrough: its host, its herdr session's server stopped
/// and its home's short link removed when it ends.
pub struct Live {
    pub walk: Walk,
    link: PathBuf,
}

impl Live {
    /// A walkthrough under herdr with the pinned Claude Code signed in by
    /// the operator's test token; refused, naming the token, without one.
    pub fn new(label: &str) -> Live {
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

        let program = pinned_claude(&walk);
        let config = walk.repository.scratch.path("claude-config");
        std::fs::create_dir_all(&config).expect("the agents' Claude Code folder");
        std::fs::write(
            config.join(".claude.json"),
            "{\"hasCompletedOnboarding\": true, \"theme\": \"dark\"}\n",
        )
        .expect("the agents' Claude Code configuration");
        let model = std::env::var(MODEL)
            .ok()
            .filter(|model| !model.trim().is_empty());
        let mut text = String::from("[host]\nterminal = \"herdr\"\n");
        for kind in ["fix", "change"] {
            text.push_str(&format!(
                "\n[kinds.{kind}.agents]\nprogram = {:?}\n",
                program.display().to_string()
            ));
            if let Some(model) = &model {
                text.push_str(&format!("model = {model:?}\n"));
            }
            text.push_str(&format!(
                "\n[kinds.{kind}.agents.env]\nCLAUDE_CONFIG_DIR = {:?}\nCLAUDE_CODE_OAUTH_TOKEN = {token:?}\nDISABLE_AUTOUPDATER = \"1\"\nDISABLE_INSTALLATION_CHECKS = \"1\"\n",
                config.display().to_string()
            ));
        }
        walk.configure(&text);

        let repository = &walk.repository;
        repository.write(".claude/settings.json", SETTINGS);
        repository.write("suite.sh", SUITE);
        repository.commit("Claude Code's settings and the slow suite");
        repository.git(&["push", "--quiet", "origin", "main"]);
        Live { walk, link }
    }

    /// herdr's answer on flume's own session's server, as JSON.
    pub fn herdr(&self, args: &[&str]) -> Value {
        let dir = &self.walk.repository.dir;
        let listing = self
            .walk
            .command("herdr", dir)
            .args(["session", "list", "--json"])
            .output()
            .expect("herdr runs");
        let sessions: Value = serde_json::from_slice(&listing.stdout).unwrap_or(Value::Null);
        let socket = sessions["sessions"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|session| session["name"] == "flume")
            .and_then(|session| session["socket_path"].as_str())
            .unwrap_or_else(|| panic!("no herdr session flume: {sessions}"))
            .to_string();
        let output = self
            .walk
            .command("herdr", dir)
            .args(args)
            .env("HERDR_SOCKET_PATH", socket)
            .output()
            .expect("herdr runs");
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
        self.walk.until_within(
            place,
            what,
            Duration::from_secs(minutes * 60),
            Duration::from_secs(5),
            |status| {
                failing(place, status);
                holds(status)
            },
        )
    }

    /// Settles the place until it is tidied, its work landed.
    pub fn until_landed(&self, place: &str, minutes: u64) {
        self.until(place, "landed and tidied", minutes, Value::is_null);
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

/// The pinned Claude Code, as `devenv.nix` states its version, the first
/// `claude` on `PATH`; where a wrapper of nix's starts it, a link of the
/// walkthrough's own named `claude` to the program the wrapper runs, so
/// herdr reads the process by its name.
fn pinned_claude(walk: &Walk) -> PathBuf {
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
    let folder = found.parent().expect("claude's folder");
    let program = [".claude-wrapped", ".claude"]
        .iter()
        .map(|name| folder.join(name))
        .find(|wrapped| wrapped.is_file())
        .unwrap_or(found);
    let link = walk.repository.scratch.path("claude-code/claude");
    std::fs::create_dir_all(link.parent().expect("the link's folder")).expect("a folder");
    std::os::unix::fs::symlink(&program, &link).expect("the pinned Claude Code as claude");
    link
}

//! A walkthrough's host (design 18): flume's built binary under the
//! printed command, on a scratch repository bound to `git-worktree`,
//! `openspec` and `sluice`, its base landing on a local bare remote, and
//! the stand-in for Claude Code first on `PATH` as `claude`, its agents
//! signed in by a pool of one account whose `CLAUDE_CONFIG_DIR` is a
//! scratch folder and whose `CLAUDE_CODE_OAUTH_TOKEN` is a word of the
//! walk's own, which the stand-in keeps from its hooks as the release
//! does and sends nowhere (144, 161); or, for the check-live layer, no stand-in and the
//! host's terminal and account as its configuration names them. Where a
//! walkthrough asks, the stand-in for sluice is first on `PATH` too, the
//! pinned sluice answering its first submit decided, or each submit a
//! pull request that waits for the walkthrough to take it in. Every process
//! the walkthrough starts, flume, sluice, git and the stand-in, runs in the
//! scratch home with no multiplexer's session, no flume, sluice or Claude
//! Code session of the caller's, no credential and no forge (core.6,
//! core.22, core.29): the remote is a directory.

use crate::common::*;
use serde_json::Value;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

/// The stand-in's account's `CLAUDE_CODE_OAUTH_TOKEN`: no token of any
/// account's, only a word for the stand-in to keep from its hooks.
pub const STAND_IN_TOKEN: &str = "stand-in-token";

/// The gate: the check fails, with its words, while the tree holds
/// `broken`.
const CHECK: &str = "if [ -e broken ]; then\n  echo 'the parser test failed: broken is in the tree' >&2\n  exit 1\nfi\n";

/// sluice's policy: main gated by the check and landed on `origin`.
const POLICY: &str =
    "[tests]\nrunner = \"command\"\ncommand = \"sh check.sh\"\n\n[branches.main]\nremote = \"origin\"\n";

/// The repository's binding (105, C.1 to C.3, C.6).
const BINDING: &str =
    "[profiles]\nplace = \"git-worktree\"\nchange = \"openspec\"\nmerge_queue = \"sluice\"\n";

/// The variables of a caller's session that no process of a walkthrough
/// carries.
fn a_callers(name: &str) -> bool {
    [
        "HERDR_",
        "TMUX",
        "ZELLIJ",
        "SLUICE_",
        "FLUME_",
        "CLAUDE",
        "STAND_IN_",
    ]
    .iter()
    .any(|prefix| name.starts_with(prefix))
}

/// The stand-in's run: what it printed and how it ended.
pub struct Ran {
    pub out: String,
    pub code: i32,
}

/// One walkthrough's host and repository.
pub struct Walk {
    pub repository: Repository,
    pub remote: PathBuf,
    home: PathBuf,
    path: String,
    runs: std::cell::Cell<u32>,
    /// The stand-in's account's `CLAUDE_CONFIG_DIR`, none for the
    /// check-live layer, whose configuration names its own account.
    account: Option<PathBuf>,
}

impl Walk {
    /// A repository on `main` holding the binding, the gate and
    /// OpenSpec's root, pushed to its bare remote, which it tracks, and
    /// the stand-in first on `PATH` as `claude`.
    pub fn new(label: &str) -> Walk {
        Walk::made(label, true)
    }

    /// The walkthrough's repository and host with no stand-in on `PATH`:
    /// the agent program is the one the host's configuration names.
    // The check-live layer's, which the tools layer does not use.
    #[allow(dead_code)]
    pub fn without_stand_in(label: &str) -> Walk {
        Walk::made(label, false)
    }

    fn made(label: &str, stand_in: bool) -> Walk {
        let repository = Repository::new(label);
        repository.write(".config/sluice.toml", POLICY);
        repository.write(".config/flume.toml", BINDING);
        repository.write("check.sh", CHECK);
        repository.write(".gitignore", "places/\n");
        repository.write("openspec/config.yaml", "schema: spec-driven\n");
        repository.write("work.rs", "one\n");
        repository.commit("the binding, the gate and the specs' root");
        let remote = repository.bare_remote();
        repository.git(&["remote", "add", "origin", &remote.to_string_lossy()]);
        repository.git(&["fetch", "--quiet", "origin"]);
        repository.git(&["branch", "--quiet", "--set-upstream-to", "origin/main"]);

        let home = repository.scratch.path("home");
        let bin = repository.scratch.path("bin");
        for dir in [&home, &bin] {
            std::fs::create_dir_all(dir).expect("the walkthrough's folder");
        }
        if stand_in {
            std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_flume-stand-in"), bin.join("claude"))
                .expect("the stand-in as claude");
        }
        let flume = Path::new(env!("CARGO_BIN_EXE_flume"))
            .parent()
            .expect("flume's folder")
            .to_path_buf();
        let path = std::env::join_paths([bin, flume].into_iter().chain(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        )))
        .expect("a PATH")
        .to_string_lossy()
        .into_owned();
        let account = stand_in.then(|| repository.scratch.path("account-1"));
        if let Some(dir) = &account {
            std::fs::create_dir_all(dir).expect("the account's folder");
        }
        let walk = Walk {
            repository,
            remote,
            home,
            path,
            runs: std::cell::Cell::new(0),
            account,
        };
        walk.configure("");
        walk
    }

    /// The stand-in's account's `CLAUDE_CONFIG_DIR`, where its agents
    /// keep their transcripts.
    pub fn account(&self) -> &Path {
        self.account
            .as_deref()
            .expect("a walkthrough with the stand-in")
    }

    /// The walkthrough's home at `home`, a path to the same folder, as a
    /// short link is where a socket beneath the home must fit.
    // The check-live layer's, which the tools layer does not use.
    #[allow(dead_code)]
    pub fn with_home(mut self, home: PathBuf) -> Walk {
        self.home = home;
        self
    }

    /// The walkthrough's home.
    // The check-live layer's, which the tools layer does not use.
    #[allow(dead_code)]
    pub fn home(&self) -> &Path {
        &self.home
    }

    /// The stand-in for sluice first on `PATH` as `sluice`, so the
    /// first submission is answered decided.
    pub fn sluice_decides_the_first_submit(&self) {
        stand_in_sluice(&self.repository.scratch.path("bin"));
    }

    /// The stand-in for sluice on a base that takes pull requests first
    /// on `PATH` as `sluice`, so each submission is a pull request that
    /// waits to be taken in.
    pub fn sluice_takes_pull_requests(&self) {
        stand_in_pull_requests(&self.repository.scratch.path("bin"));
    }

    /// The place's pull request taken in as an entry, as the host that
    /// takes the base's requests in does: the entry's record.
    pub fn take_in_request(&self) -> Value {
        let output = self
            .command("sluice", &self.repository.dir)
            .arg("take-in")
            .output()
            .expect("the stand-in for sluice runs");
        assert!(
            output.status.success(),
            "sluice take-in: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice(&output.stdout).expect("the entry's record")
    }

    /// The host's configuration, `[host]` and beside it, with the
    /// stand-in's pool of one account, which the `default` stream takes
    /// its accounts from (144, C.7).
    pub fn configure(&self, text: &str) {
        let mut text = text.to_string();
        if let Some(account) = &self.account {
            text.push_str(&format!(
                "\n[streams.default]\npool = \"stand-in\"\n\n[pools.stand-in]\nstrategy = \"fill-first\"\naccounts = [{{ name = \"stand-in-1\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?}, CLAUDE_CODE_OAUTH_TOKEN = \"{STAND_IN_TOKEN}\" }} }}]\n",
                account.display().to_string()
            ));
        }
        self.configure_whole(&text);
    }

    /// The host's configuration as `text` gives it whole, its pools and
    /// accounts among it.
    pub fn configure_whole(&self, text: &str) {
        let dir = self.home.join("config/flume");
        std::fs::create_dir_all(&dir).expect("the configuration's folder");
        std::fs::write(dir.join("config.toml"), text).expect("the configuration");
    }

    /// A scratch folder for an account's `CLAUDE_CONFIG_DIR`, made.
    pub fn account_folder(&self, name: &str) -> PathBuf {
        let dir = self.repository.scratch.path(name);
        std::fs::create_dir_all(&dir).expect("the account's folder");
        dir
    }

    /// `program` with the walkthrough's environment, in `dir`.
    pub fn command(&self, program: &str, dir: &Path) -> Command {
        let mut command = Command::new(program);
        command.current_dir(dir);
        for (name, _) in std::env::vars() {
            if a_callers(&name) {
                command.env_remove(name);
            }
        }
        command
            .env("HOME", &self.home)
            .env("XDG_STATE_HOME", self.home.join("state"))
            .env("XDG_CONFIG_HOME", self.home.join("config"))
            .env("XDG_DATA_HOME", self.home.join("data"))
            .env("PATH", &self.path)
            .env("OPENSPEC_TELEMETRY", "0")
            .stdin(Stdio::null());
        command
    }

    /// flume's answer to `words`, as JSON, and whether it refused.
    pub fn flume_ran(&self, words: &[&str]) -> (Value, bool) {
        let output = self
            .command(env!("CARGO_BIN_EXE_flume"), &self.repository.dir)
            .args(words)
            .arg("--json")
            .output()
            .expect("flume runs");
        let answer = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
            panic!(
                "flume {}: {}{}",
                words.join(" "),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        });
        (answer, !output.status.success())
    }

    /// flume's answer to `words`, which it must not refuse.
    pub fn flume(&self, words: &[&str]) -> Value {
        let (answer, refused) = self.flume_ran(words);
        assert!(!refused, "flume {} refused: {answer}", words.join(" "));
        answer
    }

    /// flume's answer to `words` as a person reads it, which it must not
    /// refuse.
    pub fn person(&self, words: &[&str]) -> String {
        let output = self
            .command(env!("CARGO_BIN_EXE_flume"), &self.repository.dir)
            .args(words)
            .output()
            .expect("flume runs");
        let answer = String::from_utf8_lossy(&output.stdout).into_owned();
        assert!(
            output.status.success(),
            "flume {} refused: {answer}{}",
            words.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
        answer
    }

    /// Launches `<kind>--<name>` with `words` beside, and gives the
    /// place's location.
    pub fn launch(&self, kind: &str, name: &str, words: &[&str]) -> String {
        let mut launch = vec!["launch", "--kind", kind, "--name", name];
        launch.extend(words);
        self.flume(&launch);
        format!("{}/places/{kind}--{name}", self.repository.name())
    }

    /// Every place's status and the host's.
    pub fn everything(&self) -> Value {
        self.flume(&["status"])["status"].clone()
    }

    /// The place's status, null once it is tidied.
    pub fn status(&self, place: &str) -> Value {
        self.everything()["places"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|status| status["place"] == place)
            .cloned()
            .unwrap_or(Value::Null)
    }

    /// The host's part of status.
    pub fn host(&self) -> Value {
        self.everything()["host"].clone()
    }

    /// A settle of the place, as a caller makes between its commands.
    pub fn settle(&self, place: &str) {
        self.flume(&["settle", place]);
    }

    /// Settles the place and reads its status until `holds` holds of it,
    /// for up to two minutes, and gives that status.
    pub fn until(&self, place: &str, what: &str, holds: impl Fn(&Value) -> bool) -> Value {
        self.until_within(
            place,
            what,
            Duration::from_secs(120),
            Duration::from_millis(300),
            holds,
        )
    }

    /// Settles the place every `pause` and reads its status until `holds`
    /// holds of it, for up to `limit`, and gives that status.
    pub fn until_within(
        &self,
        place: &str,
        what: &str,
        limit: Duration,
        pause: Duration,
        holds: impl Fn(&Value) -> bool,
    ) -> Value {
        let began = Instant::now();
        loop {
            self.settle(place);
            let status = self.status(place);
            if holds(&status) {
                return status;
            }
            assert!(
                began.elapsed() < limit,
                "{place} never came to {what}: {status}"
            );
            std::thread::sleep(pause);
        }
    }

    /// The command the place's agent was printed with.
    pub fn printed(&self, place: &str) -> String {
        let status = self.status(place);
        status["agent"]["printed"]
            .as_str()
            .unwrap_or_else(|| panic!("no command printed for {place}: {status}"))
            .to_string()
    }

    /// The prompt the printed command carries, its last word, which
    /// follows the plugin's folder, unquoted.
    pub fn prompt(&self, place: &str) -> String {
        let printed = self.printed(place);
        let (_, after) = printed
            .split_once(" --plugin-dir ")
            .unwrap_or_else(|| panic!("no plugin in {printed}"));
        let (_, quoted) = after
            .split_once(' ')
            .unwrap_or_else(|| panic!("no prompt in {printed}"));
        quoted
            .strip_prefix('\'')
            .and_then(|quoted| quoted.strip_suffix('\''))
            .unwrap_or(quoted)
            .replace("'\\''", "'")
    }

    /// The printed command run as a person runs it, the stand-in acting
    /// `acts`; started, not waited on.
    pub fn start_agent(&self, place: &str, acts: Value) -> Child {
        self.start_agent_with(place, acts, &[])
    }

    /// The printed command run as a person runs it, the stand-in acting
    /// `acts` with the variables `env` adds; started, not waited on.
    pub fn start_agent_with(&self, place: &str, acts: Value, env: &[(&str, &str)]) -> Child {
        let printed = self.printed(place);
        self.runs.set(self.runs.get() + 1);
        let script = self
            .repository
            .scratch
            .path(&format!("script-{}.json", self.runs.get()));
        std::fs::write(&script, acts.to_string()).expect("the stand-in's script");
        self.command("sh", &self.repository.dir)
            .args(["-c", &printed])
            .env("STAND_IN_SCRIPT", &script)
            .envs(env.iter().copied())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("the printed command runs")
    }

    /// The printed command run to its end, the stand-in acting `acts`;
    /// it must end well.
    pub fn agent(&self, place: &str, acts: Value) -> String {
        self.agent_with(place, acts, &[])
    }

    /// The printed command run to its end, the stand-in acting `acts` with
    /// the variables `env` adds; it must end well.
    pub fn agent_with(&self, place: &str, acts: Value, env: &[(&str, &str)]) -> String {
        let ran = finish(self.start_agent_with(place, acts, env));
        assert_eq!(ran.code, 0, "the stand-in in {place}: {}", ran.out);
        ran.out
    }

    /// A file in the scratch folder, for a stand-in's `wait_for`.
    pub fn signal(&self, name: &str) -> PathBuf {
        self.repository.scratch.path(name)
    }

    /// A commit landed on the remote's main by someone else, as another
    /// host lands one.
    pub fn landed_elsewhere(&self, file: &str, text: &str) -> String {
        let elsewhere = Repository::clone_of(&self.remote, Scratch::new("elsewhere"));
        elsewhere.write(file, text);
        let head = elsewhere.commit("landed elsewhere");
        elsewhere.git(&["push", "--quiet", "origin", "main"]);
        head
    }

    /// The file at `path` on the remote's main, none where it holds none.
    pub fn on_remote(&self, path: &str) -> Option<String> {
        let output = Command::new("git")
            .arg("--git-dir")
            .arg(&self.remote)
            .args(["show", &format!("main:{path}")])
            .output()
            .expect("git runs");
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Every file on the remote's main.
    pub fn remote_files(&self) -> Vec<String> {
        git(&self.remote, &["ls-tree", "-r", "--name-only", "main"])
            .lines()
            .map(str::to_string)
            .collect()
    }

    /// The subjects of the remote's main, newest first.
    pub fn remote_log(&self) -> Vec<String> {
        git(&self.remote, &["log", "--format=%s", "main"])
            .lines()
            .map(str::to_string)
            .collect()
    }

    /// Whether the repository holds the branch.
    pub fn has_branch(&self, branch: &str) -> bool {
        !self
            .repository
            .git(&["branch", "--list", branch])
            .is_empty()
    }

    /// sluice's records of main's submissions, read in the scratch home
    /// with no session of the caller's.
    pub fn submissions(&self) -> Vec<Value> {
        let output = self
            .command("sluice", &self.repository.dir)
            .args(["status", "main", "--all", "--json"])
            .output()
            .expect("sluice runs");
        let status: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
            panic!(
                "sluice status: {}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        });
        status["submissions"]
            .as_array()
            .cloned()
            .unwrap_or_default()
    }
}

/// The stand-in for sluice written into `bin` as `sluice`: the pinned
/// sluice, as the test's own `PATH` finds it, with its first submit's
/// `--no-wait` dropped, so it answers as the pinned sluice answers a
/// no-wait submit on a base that takes pull requests where it finds the
/// submission decided as it takes it in: with the entry's record,
/// exiting as the outcome does, the outcome recorded delivered to the
/// worktree, so no inbox gives it (sluice's `report_delivering`; 112,
/// 133). Every other command, and every later submit, is the pinned
/// sluice's own.
pub fn stand_in_sluice(bin: &Path) {
    let pinned = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
        .map(|dir| dir.join("sluice"))
        .find(|path| path.is_file())
        .expect("the pinned sluice on PATH");
    std::fs::create_dir_all(bin).expect("the stand-in's folder");
    let path = bin.join("sluice");
    let script = format!(
        "#! /bin/sh -e\n\
         if [ \"$1\" = submit ] && mkdir '{}' 2>/dev/null; then\n  \
         for word do\n    \
         shift\n    \
         [ \"$word\" = --no-wait ] || set -- \"$@\" \"$word\"\n  \
         done\n\
         fi\n\
         exec '{}' \"$@\"\n",
        bin.join(".sluice-submitted").display(),
        pinned.display()
    );
    std::fs::write(&path, script).expect("the stand-in for sluice");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
        .expect("the stand-in for sluice made executable");
}

/// The pull request the stand-in for sluice on a base that takes pull
/// requests answers with.
pub const PULL_REQUEST: &str = "7";

/// The stand-in for sluice written into `bin` as `sluice`, for a base
/// that takes pull requests where the walkthrough has no forge: its
/// submit answers as the pinned sluice answers a no-wait submit whose
/// pull request intake has not taken in, `{pull_request, branch,
/// state}` with no entry, and makes none (C.6); `sluice take-in` has
/// the pinned sluice take the request in from the place that submitted
/// it, as an intake on the host that takes the base's requests in does;
/// status gives each entry of that place's branch as sluice records one
/// intake took in from a pull request, `pull/<n>` its `from.worktree`
/// and the branch its `from.head`; and inbox gives nothing, sluice having
/// delivered each outcome to the request (133). Every other command is
/// the pinned sluice's own.
pub fn stand_in_pull_requests(bin: &Path) {
    let on_path = |program: &str| {
        std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
            .map(|dir| dir.join(program))
            .find(|path| path.is_file())
            .unwrap_or_else(|| panic!("the pinned {program} on PATH"))
    };
    let (pinned, jq) = (on_path("sluice"), on_path("jq"));
    std::fs::create_dir_all(bin).expect("the stand-in's folder");
    let path = bin.join("sluice");
    let script = format!(
        r#"#! /bin/sh -e
state='{state}'
pinned='{pinned}'
mkdir -p "$state"
case "$1" in
submit)
  pwd > "$state/place"
  git rev-parse --abbrev-ref HEAD > "$state/branch"
  printf '%s\n' "$2" > "$state/base"
  rm -f "$state/taken"
  printf '{{"branch":"%s","pull_request":{number},"state":"waiting"}}\n' "$2"
  ;;
take-in)
  if [ ! -e "$state/place" ] || [ -e "$state/taken" ]; then
    echo 'sluice take-in: no pull request waits to be taken in' >&2
    exit 1
  fi
  : > "$state/taken"
  cd "$(cat "$state/place")"
  exec "$pinned" submit "$(cat "$state/base")" --no-wait --json
  ;;
inbox)
  printf '{{"outcomes":[]}}\n'
  ;;
status)
  code=0
  out=$("$pinned" "$@") || code=$?
  if [ -e "$state/branch" ] && printf '%s\n' "$out" |
    '{jq}' --arg branch "$(cat "$state/branch")" '
      if type == "object" and has("submissions") then
        .submissions |= map(
          if .from.worktree == $branch then
            (.from.worktree = "pull/{number}") | (.from.head = $branch) | del(.from.path)
          else . end)
      else . end' 2>/dev/null; then
    exit "$code"
  fi
  printf '%s\n' "$out"
  exit "$code"
  ;;
*)
  exec "$pinned" "$@"
  ;;
esac
"#,
        state = bin.join(".sluice-request").display(),
        pinned = pinned.display(),
        jq = jq.display(),
        number = PULL_REQUEST,
    );
    std::fs::write(&path, script).expect("the stand-in for sluice");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
        .expect("the stand-in for sluice made executable");
}

/// The stand-in's run waited on to its end.
pub fn finish(child: Child) -> Ran {
    let output = child.wait_with_output().expect("the printed command ends");
    Ran {
        out: format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
        code: output.status.code().unwrap_or(-1),
    }
}

/// The place's stage named `name`, as status gives it.
pub fn stage<'a>(status: &'a Value, name: &str) -> &'a Value {
    status["stages"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|stage| stage["name"] == name)
        .unwrap_or(&Value::Null)
}

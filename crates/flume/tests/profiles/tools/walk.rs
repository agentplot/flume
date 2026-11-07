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
//! pull request that waits for the walkthrough to take it in. With the
//! stand-in, the host user's login shell is the walk's own, which gives
//! the walk's `PATH`, so a place's steps, sluice's equip and release, run
//! the sluice the tree pins; and every sluice keeps its scratch in the
//! walk's home, which the gate's sandbox lets it write. Every process
//! the walkthrough starts, flume, sluice, git and the stand-in, runs in the
//! scratch home with no multiplexer's session, no flume, sluice or Claude
//! Code session of the caller's, no credential and no forge (core.6,
//! core.22, core.29): the remote is a directory, and sluice's checkouts
//! keep their scratch in the walkthrough's own folder. A walkthrough may
//! run under the pinned herdr instead (design 10): its agents in panes of
//! flume's own herdr session, started headless on the walkthrough's
//! scratch home, which it keeps behind a short link so the sockets of its
//! sessions fit, each start of the stand-in acting its own script, and
//! its sessions' servers stopped when it ends; and a test may start a
//! herdr server of its own, on a scratch socket and home, to run the
//! stand-in in one of its panes. Every herdr call a test makes is bounded
//! (core.29).

use crate::common::*;
use flume::profiles::place::git_worktree;
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

/// The variable naming the folder that stands for the host user's own,
/// under which the pinned sluice keeps a checkout's scratch rather than
/// under `/tmp/sluice-<uid>`, which the gate's sandbox refuses a process
/// a check starts; sluice's own tests name theirs by it (sluice record
/// 0063).
pub const USER_SCRATCH: &str = "SLUICE_USER_SCRATCH";

/// The test's own folder for the scratch of the checkouts sluice makes
/// for `repository`.
pub fn user_scratch(repository: &Repository) -> PathBuf {
    repository.scratch.path("sluice")
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
    /// The host user's login shell, the walk's own, with the stand-in;
    /// none for the check-live layer, whose host's own shell starts its
    /// terminal's panes.
    shell: Option<PathBuf>,
    runs: std::cell::Cell<u32>,
    /// The stand-in's account's `CLAUDE_CONFIG_DIR`, none for the
    /// check-live layer, whose configuration names its own account.
    account: Option<PathBuf>,
    /// The folder the stand-in reads each start's script from, which its
    /// account gives as `STAND_IN_SCRIPTS`, on a host under herdr.
    scripts: Option<PathBuf>,
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
        let shell = stand_in.then(|| login_shell(&repository.scratch.path("login"), &path));
        let account = stand_in.then(|| repository.scratch.path("account-1"));
        if let Some(dir) = &account {
            std::fs::create_dir_all(dir).expect("the account's folder");
        }
        let walk = Walk {
            repository,
            remote,
            home,
            path,
            shell,
            runs: std::cell::Cell::new(0),
            account,
            scripts: None,
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

    /// The walkthrough's home at `home`: a path to the same folder, as a
    /// short link is where a socket beneath the home must fit, or another
    /// walkthrough's, so its repositories share one host.
    pub fn with_home(mut self, home: PathBuf) -> Walk {
        self.home = home;
        self
    }

    /// The walkthrough's home, its configuration under `config/flume`.
    pub fn home(&self) -> &Path {
        &self.home
    }

    /// The walkthrough's `PATH`: the stand-in, then flume's own folder,
    /// then the test's.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The host user's login shell, the walk's own, which gives the walk's
    /// `PATH` to what a launch runs for a place.
    pub fn login_shell(&self) -> &Path {
        self.shell
            .as_deref()
            .expect("a walkthrough with the stand-in")
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
                "\n[streams.default]\npool = \"stand-in\"\n\n[pools.stand-in]\nstrategy = \"fill-first\"\naccounts = [{}]\n",
                self.stand_in_account("stand-in-1", account)
            ));
        }
        self.configure_whole(&text);
    }

    /// The host's configuration folder, `$XDG_CONFIG_HOME/flume`, which
    /// holds `config.toml` and `sources/` (191, C.7).
    pub fn config_dir(&self) -> PathBuf {
        self.home.join("config/flume")
    }

    /// The stand-in's account `name` as a pool's `accounts` lists it: its
    /// program `claude`, its `CLAUDE_CONFIG_DIR` the folder `home` and its
    /// `CLAUDE_CODE_OAUTH_TOKEN` the walk's word, and, on a host under
    /// herdr, `STAND_IN_SCRIPTS` the walk's folder of scripts (144).
    pub fn stand_in_account(&self, name: &str, home: &Path) -> String {
        let scripts = self
            .scripts
            .as_ref()
            .map(|dir| format!(", STAND_IN_SCRIPTS = {:?}", dir.display().to_string()))
            .unwrap_or_default();
        format!(
            "{{ name = {name:?}, program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?}, CLAUDE_CODE_OAUTH_TOKEN = \"{STAND_IN_TOKEN}\"{scripts} }} }}",
            home.display().to_string()
        )
    }

    /// The host's configuration as `text` gives it whole, its pools and
    /// accounts among it.
    pub fn configure_whole(&self, text: &str) {
        let dir = self.config_dir();
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
            .env(USER_SCRATCH, user_scratch(&self.repository))
            .stdin(Stdio::null());
        if let Some(shell) = &self.shell {
            command.env("SHELL", shell);
        }
        command
    }

    /// flume's binary with the walkthrough's environment in its
    /// repository, for `words`: a launch's environment names
    /// `$FLUME_RECIPIENT` as `caller`, since a launch from no session that
    /// names no recipient and finds no role behind its stream is refused
    /// (7, 193).
    fn flume_command(&self, words: &[&str]) -> Command {
        let mut command = self.command(env!("CARGO_BIN_EXE_flume"), &self.repository.dir);
        if words.contains(&"launch") {
            command.env("FLUME_RECIPIENT", "caller");
        }
        command
    }

    /// flume's answer to `words`, as JSON, and whether it refused.
    pub fn flume_ran(&self, words: &[&str]) -> (Value, bool) {
        Walk::answered(self.flume_command(words), words)
    }

    /// flume's answer to `words` as [`Walk::flume_ran`] gives it, run with
    /// `env` beside the walkthrough's and no `$FLUME_RECIPIENT`: a launch
    /// from the session `env` names, or from none where it names none (7,
    /// 193).
    pub fn flume_ran_from(&self, words: &[&str], env: &[(&str, &str)]) -> (Value, bool) {
        let mut command = self.command(env!("CARGO_BIN_EXE_flume"), &self.repository.dir);
        command.envs(env.iter().copied());
        Walk::answered(command, words)
    }

    /// `command`'s answer to `words`, as JSON, and whether it refused.
    fn answered(mut command: Command, words: &[&str]) -> (Value, bool) {
        let output = command
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
            .flume_command(words)
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
        git_worktree::location(
            &self.repository.name(),
            &format!("{kind}--{name}"),
            Some(&self.home.join("data/flume")),
        )
        .expect("the place's location")
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

    /// The session the printed command resumes, none where it resumes
    /// none (179).
    pub fn resumes(&self, place: &str) -> Option<String> {
        let printed = self.printed(place);
        let (_, after) = printed.split_once(" --resume ")?;
        after.split_whitespace().next().map(str::to_string)
    }

    /// The file the printed command names by `--settings`, none where it
    /// passes no own settings (195, 196).
    pub fn own_settings(&self, place: &str) -> Option<PathBuf> {
        let printed = self.printed(place);
        let (_, after) = printed.split_once(" --settings ")?;
        after
            .split_whitespace()
            .next()
            .map(|file| PathBuf::from(file.trim_matches('\'')))
    }

    /// The prompt the printed command carries, its last word, which
    /// follows the plugin's folder, the own settings' file and the session
    /// it resumes, if any, unquoted.
    pub fn prompt(&self, place: &str) -> String {
        let printed = self.printed(place);
        let (_, after) = printed
            .split_once(" --plugin-dir ")
            .unwrap_or_else(|| panic!("no plugin in {printed}"));
        let (_, mut quoted) = after
            .split_once(' ')
            .unwrap_or_else(|| panic!("no prompt in {printed}"));
        for flag in ["--settings ", "--resume "] {
            if let Some(given) = quoted.strip_prefix(flag) {
                quoted = given.split_once(' ').map_or(given, |(_, quoted)| quoted);
            }
        }
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
/// 133). While that submit waits on the rounds that decide it, its inbox
/// gives no outcome, as the pinned sluice's place inbox gives none of a
/// submission it decided as it took it in, whose record names the pull
/// request: a settle beside the submit, the Stop hook's or a caller's,
/// collects nothing the answer delivers. Every other command, and every
/// later submit, is the pinned sluice's own.
pub fn stand_in_sluice(bin: &Path) {
    let pinned = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
        .map(|dir| dir.join("sluice"))
        .find(|path| path.is_file())
        .expect("the pinned sluice on PATH");
    std::fs::create_dir_all(bin).expect("the stand-in's folder");
    let path = bin.join("sluice");
    let script = format!(
        r#"#! /bin/sh -e
pinned='{pinned}'
if [ "$1" = inbox ] && [ -d '{submitting}' ]; then
  printf '%s\n' '{{"outcomes": []}}'
  exit 0
fi
if [ "$1" = submit ] && mkdir '{submitted}' 2>/dev/null; then
  for word do
    shift
    [ "$word" = --no-wait ] || set -- "$@" "$word"
  done
  mkdir '{submitting}'
  status=0
  "$pinned" "$@" || status=$?
  rmdir '{submitting}'
  exit "$status"
fi
exec "$pinned" "$@"
"#,
        pinned = pinned.display(),
        submitted = bin.join(".sluice-submitted").display(),
        submitting = bin.join(".sluice-submitting").display(),
    );
    std::fs::write(&path, script).expect("the stand-in for sluice");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
        .expect("the stand-in for sluice made executable");
}

/// The walk's login shell, written into `dir` as `shell`: run as a login
/// shell, it runs `/bin/sh` with the rest of its words and `path` for its
/// `PATH`, so the login environment a place's steps run in names the
/// walk's tools, the pinned sluice among them, rather than whatever the
/// host user's own login shell names (11, 79; core.22). Its path.
fn login_shell(dir: &Path, path: &str) -> PathBuf {
    std::fs::create_dir_all(dir).expect("the login shell's folder");
    let shell = dir.join("shell");
    std::fs::write(
        &shell,
        format!(
            "#!/bin/sh\nPATH='{path}'\nexport PATH\n[ \"$1\" = -l ] && shift\nexec /bin/sh \"$@\"\n"
        ),
    )
    .expect("the login shell");
    std::fs::set_permissions(&shell, std::fs::Permissions::from_mode(0o755))
        .expect("the login shell made executable");
    shell
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

/// The seconds since the epoch now.
pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("a clock past the epoch")
        .as_secs()
}

/// The top of the UTC hour two hours on from `at`, well clear of any
/// walk's length, and the session limit naming it in the words the pinned
/// Claude Code writes, a time of day in en-US's 12-hour form (151, C.4).
pub fn session_limit_after(at: u64) -> (u64, String) {
    let reset = at - at % 3600 + 2 * 3600;
    let hour = reset % 86_400 / 3600;
    let clock = match hour {
        0 => "12am".to_owned(),
        1..=11 => format!("{hour}am"),
        12 => "12pm".to_owned(),
        _ => format!("{}pm", hour - 12),
    };
    (
        reset,
        format!("You've hit your session limit · resets {clock} (UTC)"),
    )
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

/// The bound on each herdr call a test makes of its own: herdr answers a
/// local call in milliseconds, and one that does not answer within this
/// fails the test naming it (core.29).
pub const HERDR_BOUND: Duration = Duration::from_secs(10);

/// What the pinned herdr's call `command` answers within [`HERDR_BOUND`]:
/// its `result` where it answers JSON holding one, else its text; its
/// words where it refuses, or the bound named where it does not answer
/// within it, the call then killed.
pub fn herdr_within(command: &mut Command) -> Result<Value, String> {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("herdr could not be run: {error}"))?;
    let read = |pipe: Option<Box<dyn std::io::Read + Send>>| {
        std::thread::spawn(move || {
            let mut text = String::new();
            if let Some(mut pipe) = pipe {
                let _ = pipe.read_to_string(&mut text);
            }
            text
        })
    };
    let out = read(child.stdout.take().map(|pipe| Box::new(pipe) as _));
    let err = read(child.stderr.take().map(|pipe| Box::new(pipe) as _));
    let began = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if began.elapsed() < HERDR_BOUND => {
                std::thread::sleep(Duration::from_millis(20));
            }
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!(
                    "herdr did not answer within {} seconds",
                    HERDR_BOUND.as_secs()
                ));
            }
        }
    };
    let (out, err) = (
        out.join().unwrap_or_default(),
        err.join().unwrap_or_default(),
    );
    match serde_json::from_str::<Value>(out.trim()) {
        Ok(answer) if answer.get("result").is_some() => Ok(answer["result"].clone()),
        _ if status.success() => Ok(Value::String(out)),
        _ => Err(format!("{out}{err}")),
    }
}

/// The pinned herdr, by its own path, the first on the test's `PATH`.
pub fn pinned_herdr() -> PathBuf {
    std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
        .map(|dir| dir.join("herdr"))
        .find(|candidate| candidate.is_file())
        .and_then(|found| found.canonicalize().ok())
        .expect("the pinned herdr on PATH")
}

/// How many short links this run has made, which names the next.
static LINKS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// A link to `target` short enough that `socket` beneath it fits a
/// socket's path on this host: in the temporary folder where it fits
/// there, else in `/tmp` where the run may write there; the test fails
/// naming what lacked room where neither may hold it (core.51).
pub fn short_link(target: &Path, socket: &str) -> PathBuf {
    // SAFETY: an address of all zeros is a valid `sockaddr_un`.
    let address: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    let limit = address.sun_path.len() - 1;
    let named = format!(
        "w{}-{}",
        std::process::id(),
        LINKS.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    );
    let temporary = std::env::temp_dir();
    [temporary.clone(), PathBuf::from("/tmp")]
        .into_iter()
        .filter(|folder| folder.join(&named).join(socket).as_os_str().len() <= limit)
        .find_map(|folder| {
            let link = folder.join(&named);
            let _ = std::fs::remove_file(&link);
            std::os::unix::fs::symlink(target, &link)
                .ok()
                .map(|()| link)
        })
        .unwrap_or_else(|| {
            let socket = temporary.join(&named).join(socket);
            panic!(
                "herdr's socket would be {}, {} bytes, past the {limit} a socket's path holds on \
                 this host, and /tmp cannot be written",
                socket.display(),
                socket.as_os_str().len()
            )
        })
}

/// A herdr server of the test's own, headless on a scratch socket with a
/// scratch home, its panes' shell `/bin/sh` and its `PATH` herdr's own
/// folder and the system's, so no herdr session of the host's is touched
/// and no `HERDR_*` of the caller's reaches it; stopped, its panes with
/// it, when dropped (core.29).
pub struct HerdrServer {
    pub scratch: Scratch,
    socket: PathBuf,
    link: PathBuf,
    child: Option<Child>,
}

impl HerdrServer {
    pub fn start(label: &str) -> HerdrServer {
        let scratch = Scratch::new(label);
        std::fs::create_dir_all(scratch.path("home")).expect("the server's home");
        let link = short_link(&scratch.root, "s");
        let socket = link.join("s");
        let program = pinned_herdr();
        let child = Command::new(&program)
            .arg("server")
            .env_clear()
            .env(
                "PATH",
                format!(
                    "{}:/usr/bin:/bin",
                    program.parent().expect("herdr's folder").display()
                ),
            )
            .env("HOME", scratch.path("home"))
            .env("SHELL", "/bin/sh")
            .env("TERM", "xterm-256color")
            .env("LANG", "en_US.UTF-8")
            .env("HERDR_SOCKET_PATH", &socket)
            .current_dir(scratch.path("home"))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("herdr's server starts");
        let server = HerdrServer {
            scratch,
            socket,
            link,
            child: Some(child),
        };
        let began = Instant::now();
        while server.try_herdr(&["workspace", "list"]).is_err() {
            assert!(
                began.elapsed() < Duration::from_secs(20),
                "herdr's server did not answer"
            );
            std::thread::sleep(Duration::from_millis(100));
        }
        server
    }

    /// herdr's answer to `args` on this server, within the bound.
    pub fn try_herdr(&self, args: &[&str]) -> Result<Value, String> {
        let mut command = Command::new(pinned_herdr());
        for (name, _) in std::env::vars_os() {
            if name.to_string_lossy().starts_with("HERDR_") {
                command.env_remove(name);
            }
        }
        herdr_within(
            command
                .args(args)
                .env("HERDR_SOCKET_PATH", &self.socket)
                .env("HOME", self.scratch.path("home")),
        )
    }

    /// herdr's answer to `args` on this server, which it must give.
    pub fn herdr(&self, args: &[&str]) -> Value {
        self.try_herdr(args)
            .unwrap_or_else(|words| panic!("herdr {}: {words}", args.join(" ")))
    }

    /// A pane of a new workspace in `cwd`: its id.
    pub fn pane_in(&self, cwd: &Path) -> String {
        let made = self.herdr(&[
            "workspace",
            "create",
            "--label",
            "stand-in",
            "--cwd",
            &cwd.to_string_lossy(),
        ]);
        made["root_pane"]["pane_id"]
            .as_str()
            .unwrap_or_else(|| panic!("no pane in {made}"))
            .to_string()
    }

    /// The pane as herdr reads it.
    pub fn pane(&self, pane: &str) -> Value {
        self.herdr(&["pane", "get", pane])["pane"].clone()
    }

    /// The text typed into the pane's agent as flume's herdr profile sends
    /// it: ctrl+c, then `agent prompt` (95).
    pub fn prompt(&self, pane: &str, text: &str) {
        self.herdr(&["agent", "send-keys", pane, "ctrl+c"]);
        self.herdr(&["agent", "prompt", pane, text]);
    }
}

impl Drop for HerdrServer {
    fn drop(&mut self) {
        let _ = self.try_herdr(&["server", "stop"]);
        if let Some(mut child) = self.child.take() {
            let deadline = Instant::now() + Duration::from_secs(10);
            while child.try_wait().ok().flatten().is_none() && Instant::now() < deadline {
                std::thread::sleep(Duration::from_millis(100));
            }
            let _ = child.kill();
            let _ = child.wait();
        }
        let _ = std::fs::remove_file(&self.link);
    }
}

/// A walkthrough's host under the pinned herdr (C.5; design 10): the
/// walk, its home a short link so the sockets the pinned herdr binds for
/// its sessions beneath its configuration home fit a socket's path, every
/// herdr call it makes itself bounded and passing the session's socket
/// (core.29), and each session's server running under its home stopped,
/// its panes with it, and its link removed when it is dropped. Made by
/// [`Walk::under_herdr`], it runs its agents in panes of flume's own herdr
/// session, whose server flume's herdr profile starts headless on the
/// walkthrough's scratch home, never the host's; the stand-in first on the
/// panes' `PATH` as `claude`, through the login environment the walk's
/// shell gives; and the walkthrough's account giving `STAND_IN_SCRIPTS`,
/// the walk's folder of each start's script, beside its scratch
/// `CLAUDE_CONFIG_DIR` and token.
pub struct UnderHerdr {
    pub walk: Walk,
    pub link: PathBuf,
}

impl std::ops::Deref for UnderHerdr {
    type Target = Walk;

    fn deref(&self) -> &Walk {
        &self.walk
    }
}

impl Walk {
    /// A walkthrough's host under the pinned herdr, its agents in flume's
    /// own session.
    pub fn under_herdr(label: &str) -> UnderHerdr {
        let mut walk = Walk::made(label, true);
        let scripts = walk.repository.scratch.path("scripts");
        std::fs::create_dir_all(&scripts).expect("the scripts' folder");
        walk.scripts = Some(scripts);
        let host = UnderHerdr::new(walk, own_session());
        host.configure("", "");
        host
    }
}

impl UnderHerdr {
    /// `walk` under herdr, its home a short link whose sockets fit for the
    /// session `longest`, the longest name it opens (core.51).
    pub fn new(walk: Walk, longest: &str) -> UnderHerdr {
        // The longest socket the pinned herdr binds for a session,
        // beneath the configuration home.
        let socket = format!("config/herdr/sessions/{longest}/herdr-client.sock");
        let link = short_link(walk.home(), &socket);
        UnderHerdr {
            walk: walk.with_home(link.clone()),
            link,
        }
    }

    /// The host's configuration: `[host]` naming herdr with `host`'s keys
    /// beside, then `rest`, with the stand-in's pool of one account.
    pub fn configure(&self, host: &str, rest: &str) {
        self.walk
            .configure(&format!("[host]\nmultiplexer = \"herdr\"\n{host}\n{rest}"));
    }

    /// The script the stand-in acts at the start `start` of the stage
    /// `stage`: its runs, each a list of acts, the first taking the start's
    /// prompt and each later one the next prompt typed into its session.
    pub fn script(&self, stage: &str, start: u32, runs: Value) {
        let file = self.scripts().join(format!("{stage}-{start}.json"));
        std::fs::write(file, runs.to_string()).expect("the stand-in's script");
    }

    /// The prompts the stand-in took at the start `start` of `stage`, in
    /// order, as its log keeps them.
    pub fn prompts(&self, stage: &str, start: u32) -> Vec<String> {
        self.log(stage, start)
            .lines()
            .filter_map(|line| line.strip_prefix("stand-in: took the prompt: "))
            .map(|prompt| serde_json::from_str(prompt).expect("a prompt as JSON"))
            .collect()
    }

    /// The stand-in's log at the start `start` of `stage`, empty where no
    /// agent of it ran.
    pub fn log(&self, stage: &str, start: u32) -> String {
        std::fs::read_to_string(self.scripts().join(format!("{stage}-{start}.log")))
            .unwrap_or_default()
    }

    fn scripts(&self) -> &Path {
        self.walk.scripts.as_deref().expect("a host under herdr")
    }

    /// herdr's answer to `args` under the host's configuration home, on
    /// the server at `socket` where one is named, within the bound: its
    /// JSON where it answers JSON, else its text, or its words where it
    /// refuses or does not answer.
    pub fn try_herdr(&self, socket: Option<&str>, args: &[&str]) -> Result<Value, String> {
        let mut command = self.walk.command("herdr", &self.walk.repository.dir);
        command.args(args);
        if let Some(socket) = socket {
            command.env("HERDR_SOCKET_PATH", socket);
        }
        herdr_within(&mut command).map(|answer| match answer {
            Value::String(text) => serde_json::from_str(&text).unwrap_or(Value::String(text)),
            answer => answer,
        })
    }

    /// herdr's answer to `args` as [`UnderHerdr::try_herdr`] gives it, or
    /// null where herdr refused or did not answer within the bound.
    pub fn herdr(&self, socket: Option<&str>, args: &[&str]) -> Value {
        self.try_herdr(socket, args).unwrap_or(Value::Null)
    }

    /// The sessions herdr lists running under the host's configuration
    /// home, by name, with their sockets.
    pub fn running(&self) -> Vec<(String, String)> {
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

    /// The socket of the session `name` herdr lists running under the
    /// host's configuration home.
    pub fn socket_of(&self, name: &str) -> String {
        self.running()
            .into_iter()
            .find(|(running, _)| running == name)
            .map(|(_, socket)| socket)
            .unwrap_or_else(|| panic!("no session {name}: {:?}", self.running()))
    }

    /// The labels of what herdr lists of `what`, `tab` or `workspace`, on
    /// the server at `socket`.
    pub fn labels(&self, socket: &str, what: &str) -> Vec<String> {
        self.herdr(Some(socket), &[what, "list"])[format!("{what}s")]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|listed| listed["label"].as_str().map(str::to_string))
            .collect()
    }

    /// The panes of flume's own session, as herdr lists them.
    pub fn panes(&self) -> Vec<Value> {
        let Some((_, socket)) = self
            .running()
            .into_iter()
            .find(|(name, _)| name == own_session())
        else {
            return vec![];
        };
        self.herdr(Some(&socket), &["pane", "list"])["panes"]
            .as_array()
            .cloned()
            .unwrap_or_default()
    }
}

/// The name of flume's own herdr session.
fn own_session() -> &'static str {
    flume::profiles::terminal::herdr::SESSION
}

impl Drop for UnderHerdr {
    fn drop(&mut self) {
        for (_, socket) in self.running() {
            let _ = self.try_herdr(Some(&socket), &["server", "stop"]);
        }
        let deadline = Instant::now() + Duration::from_secs(10);
        while !self.running().is_empty() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(100));
        }
        let _ = std::fs::remove_file(&self.link);
    }
}

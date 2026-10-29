//! The commands against the built binary on scratch repositories bound to
//! `git-worktree` and no merge queue (section 9; design 16; command-line
//! spec). Each command runs in a process of its own, its state and the
//! host's configuration under the test's scratch directories, nothing of
//! the caller's session in its environment.
//!
//! The launches run through this test binary re-run as flume: the same
//! commands over the profiles built in, beside a stand-in agent program
//! bound by its name, which reads its start and its end from marks the
//! test writes in the place's folder, and a stand-in merge queue for the
//! one scenario that needs a kind that lands. Every answer read back is
//! read through flume's own binary where the scenario allows.

mod common;

use common::{git, Repository, Scratch};
use flume::commands;
use flume::host::config::{self, HostConfig};
use flume::profiles::store::state_dir::StateDir;
use flume::profiles::{core_over, core_with, BuiltIn};
use flume_core::Answer;
use flume_seams::*;
use serde_json::Value;
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

/// The stand-in agent program's name.
const STAND_IN: &str = "stand-in";

/// The stand-in merge queue's name.
const STAND_IN_QUEUE: &str = "stand-in-queue";

/// The variable that makes this test binary run as flume, the command's
/// words in it as a JSON list.
const AS_FLUME: &str = "FLUME_TEST_AS_FLUME";

/// The stand-in agent: it says it started and received its prompt at
/// once, never asks or keeps own work, and reads its session ended once
/// the test writes the mark `stand-in/<stage>-<start>` in the place's
/// folder.
struct StandInAgent;

fn ended_mark(folder: &str, agent: &Identity) -> PathBuf {
    Path::new(folder)
        .join(STAND_IN)
        .join(format!("{}-{}", agent.stage, agent.start))
}

impl Agent for StandInAgent {
    fn command(
        &self,
        settings: &Settings,
        _place: &PlaceRef,
        _identity: &Identity,
        _secret: &Secret,
        prompt: &str,
    ) -> AgentCommand {
        let mut args = vec![];
        if let Some(model) = &settings.model {
            args.extend(["--model".to_string(), model.clone()]);
        }
        if let Some(instructions) = &settings.instructions {
            args.extend(["--instructions".to_string(), instructions.clone()]);
        }
        AgentCommand {
            program: settings.program.clone().unwrap_or_default(),
            args,
            env: settings.env.clone(),
            prompt_args: vec![prompt.to_string()],
            folder_trust: None,
        }
    }
    fn started(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::Yes
    }
    fn received(&self, _folder: &str, _agent: &Identity, _text: &str) -> Received {
        Received::Received {
            record: STAND_IN.into(),
        }
    }
    fn asking(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::No
    }
    fn own_work(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::No
    }
    fn ended(&self, folder: &str, agent: &Identity) -> Reading {
        if ended_mark(folder, agent).exists() {
            Reading::Yes
        } else {
            Reading::No
        }
    }
    fn last_words(&self, _folder: &str, _agent: &Identity) -> Option<String> {
        None
    }
    fn stop_short(&self, _agent: &Identity) -> String {
        "flume needs \"<words>\"".into()
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

/// The stand-in merge queue: the base where the repository holds it,
/// nothing submitted and nothing to collect.
struct StandInQueue;

impl MergeQueue for StandInQueue {
    fn submit(&self, _place: &PlaceRef, _head: &Revision) -> Submitted {
        Submitted::Refused {
            reason: "the stand-in queue takes nothing".into(),
        }
    }
    fn inbox(&self, _place: &PlaceRef) -> Inbox {
        Inbox::Outcomes { outcomes: vec![] }
    }
    fn read_submission(&self, _repository: &str, _branch: &str) -> SubmissionRead {
        SubmissionRead::None
    }
    fn step(&self, _repository: &str, _base: &str) -> Stepped {
        Stepped::MovesOnItsOwn
    }
    fn base_revision(&self, repository: &str, base: &str) -> BaseRevision {
        match flume::git::Git::at(repository).commit(base) {
            Ok(revision) => BaseRevision::At {
                revision: Revision::new(revision),
            },
            Err(reason) => BaseRevision::Refused { reason },
        }
    }
}

/// The profiles built in, with the stand-ins beside them.
struct StandIn(BuiltIn);

impl Resolver for StandIn {
    fn kinds(&self) -> Arc<dyn Kinds> {
        self.0.kinds()
    }
    fn place_profiles(&self) -> Vec<String> {
        self.0.place_profiles()
    }
    fn place(&self, repository: &str, binding: &Binding) -> Option<Arc<dyn Place>> {
        self.0.place(repository, binding)
    }
    fn change(&self, profile: &str) -> Option<Arc<dyn Change>> {
        self.0.change(profile)
    }
    fn merge_queue(&self, profile: &str) -> Option<Arc<dyn MergeQueue>> {
        match profile {
            STAND_IN_QUEUE => Some(Arc::new(StandInQueue)),
            _ => self.0.merge_queue(profile),
        }
    }
    fn agent(&self, program: &str) -> Option<Arc<dyn Agent>> {
        (program == STAND_IN).then(|| Arc::new(StandInAgent) as Arc<dyn Agent>)
    }
}

/// This test binary as flume, where the test that re-runs it asks: the
/// commands over the profiles built in and the stand-ins, the core built
/// as flume's binary builds it, the login environment its `HOME` alone.
#[test]
fn stand_in_flume() {
    let Ok(words) = std::env::var(AS_FLUME) else {
        return;
    };
    let words: Vec<String> = serde_json::from_str(&words).expect("the command's words");
    let build = || {
        let home = std::env::var("HOME").unwrap_or_default();
        core_with(
            &config::flume_state()?,
            HostConfig::of_host()?,
            flume::host::recipient(),
            Arc::new(StandIn(BuiltIn::with_login(&[("HOME", &home)]))),
            Some(vec![("HOME".into(), home)]),
        )
    };
    let outcome = commands::run(std::iter::once("flume".to_string()).chain(words), &build);
    let mut out = std::io::stdout().lock();
    writeln!(out, "{}", outcome.text).expect("the answer written");
    out.flush().expect("the answer written");
    std::process::exit(i32::from(outcome.code));
}

/// One host's scratch: flume's state, its configuration, a home.
struct Host {
    scratch: Scratch,
}

/// What a command printed and its exit status.
struct Ran {
    text: String,
    code: i32,
}

impl Ran {
    fn answer(&self) -> Answer {
        serde_json::from_str(self.text.trim())
            .unwrap_or_else(|error| panic!("an answer's JSON ({error}): {}", self.text))
    }
    fn json(&self) -> Value {
        serde_json::from_str(self.text.trim()).expect("the answer's JSON")
    }
}

impl Host {
    fn new(label: &str) -> Host {
        let scratch = Scratch::new(label);
        for dir in ["state", "config/flume", "home"] {
            std::fs::create_dir_all(scratch.path(dir)).expect("the host's directory");
        }
        Host { scratch }
    }

    fn state(&self) -> PathBuf {
        self.scratch.path("state/flume")
    }

    fn home(&self) -> PathBuf {
        self.scratch.path("home")
    }

    /// Writes the host's configuration.
    fn configure(&self, text: &str) {
        std::fs::write(self.scratch.path("config/flume/config.toml"), text)
            .expect("the configuration");
    }

    /// The command's process, run in `dir` with `env` set over nothing of
    /// the caller's flume or terminal session.
    fn command(&self, program: &Path, dir: &Path, env: &[(&str, &str)]) -> Command {
        let mut command = Command::new(program);
        command.current_dir(dir);
        for (name, _) in std::env::vars() {
            if name.starts_with("FLUME_") || name.starts_with("HERDR_") {
                command.env_remove(name);
            }
        }
        command
            .env("XDG_STATE_HOME", self.scratch.path("state"))
            .env("XDG_CONFIG_HOME", self.scratch.path("config"))
            .env("HOME", self.home());
        for (name, value) in env {
            command.env(name, value);
        }
        command
    }

    /// flume's own binary.
    fn flume(&self, dir: &Path, words: &[&str], env: &[(&str, &str)]) -> Ran {
        let output = self
            .command(Path::new(env!("CARGO_BIN_EXE_flume")), dir, env)
            .args(words)
            .output()
            .expect("flume runs");
        Ran {
            text: String::from_utf8_lossy(&output.stdout).into_owned(),
            code: output.status.code().unwrap_or(-1),
        }
    }

    /// This test binary as flume, with the stand-ins.
    fn stand_in(&self, dir: &Path, words: &[&str], env: &[(&str, &str)]) -> Ran {
        let me = std::env::current_exe().expect("the test binary");
        let output = self
            .command(&me, dir, env)
            .env(AS_FLUME, serde_json::to_string(words).expect("words"))
            .args([
                "--exact",
                "stand_in_flume",
                "--nocapture",
                "--test-threads=1",
                "-q",
            ])
            .output()
            .expect("the test binary runs");
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let text = stdout
            .split_once("running 1 test\n")
            .map_or(stdout.as_str(), |(_, after)| after)
            .to_string();
        Ran {
            text,
            code: output.status.code().unwrap_or(-1),
        }
    }

    /// The place's own folder in the store.
    fn folder(&self, place: &str) -> String {
        StateDir::at(self.state()).folder(&PlaceId::new(place))
    }

    /// Writes the stand-in agent's mark that its session ended.
    fn end_agent(&self, place: &str, stage: &str, start: u32) {
        let identity = Identity {
            repository: String::new(),
            place: String::new(),
            stage: stage.into(),
            start,
        };
        let mark = ended_mark(&self.folder(place), &identity);
        std::fs::create_dir_all(mark.parent().expect("a folder")).expect("the mark's folder");
        std::fs::write(mark, "").expect("the mark");
    }
}

/// A scratch repository bound to `git-worktree` and the merge queue
/// `queue`, none where it is none.
fn repository(label: &str, queue: Option<&str>) -> Repository {
    let repository = Repository::new(label);
    let mut binding = "[profiles]\nplace = \"git-worktree\"\n".to_string();
    if let Some(queue) = queue {
        binding.push_str(&format!("merge_queue = \"{queue}\"\n"));
    }
    repository.write(".config/flume.toml", &binding);
    repository.write(".gitignore", "places/\n");
    repository.commit("the binding");
    repository
}

/// A declaration the launch carries, beside its prompt: `change` with
/// construct, code and verify, and `fix` with code, each stage's agent
/// the stand-in and its deliverable commits; `fix` lands where `lands`.
fn kinds_file(host: &Host, lands: bool) -> String {
    let dir = host.scratch.path("kinds");
    std::fs::create_dir_all(dir.join("prompts")).expect("the kinds' folder");
    std::fs::write(
        dir.join("prompts/work.md"),
        "Do the work of the {{kind}} {{name}} in {{place}}. To stop short: {{needs}}\n",
    )
    .expect("the prompt");
    let stage = |kind: &str, name: &str| {
        format!(
            "[[kinds.{kind}.stages]]\nname = \"{name}\"\nprompt = \"prompts/work.md\"\ndeliverables = [\"commits\"]\nagents = [{{ program = \"{STAND_IN}\" }}]\n\n"
        )
    };
    let text = format!(
        "[kinds.change]\nversion = 1\nbranch = \"change\"\nstarts_unready = false\nlands = false\n\n{}{}{}[kinds.fix]\nversion = 1\nbranch = \"fix\"\nstarts_unready = true\nlands = {lands}\n\n{}",
        stage("change", "construct"),
        stage("change", "code"),
        stage("change", "verify"),
        stage("fix", "code"),
    );
    let file = dir.join("kinds.toml");
    std::fs::write(&file, text).expect("the declaration");
    file.to_string_lossy().into_owned()
}

/// The one place a status or a launch answers.
fn the_place(answer: &Answer) -> &flume_core::PlaceStatus {
    match answer {
        Answer::Launched { place } => place,
        Answer::Status { places, .. } if places.len() == 1 => &places[0],
        other => panic!("one place's answer: {other:?}"),
    }
}

/// Commits in the place, as its stage's agent would.
fn commit_in(place: &str, file: &str) {
    std::fs::write(Path::new(place).join(file), file).expect("the work");
    git(Path::new(place), &["add", file]);
    git(
        Path::new(place),
        &[
            "-c",
            "core.hooksPath=/dev/null",
            "commit",
            "--quiet",
            "-m",
            file,
        ],
    );
}

#[test]
fn the_same_answer_through_the_command_line_and_the_core() {
    // S14, 60: status through flume's binary is the core's own answer.
    let host = Host::new("same-answer");
    let repository = repository("same-answer", None);
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = format!("{}/places/fix--one", repository.name());
    let ran = host.flume(&repository.dir, &["status", &place, "--json"], &[]);
    assert_eq!(ran.code, 0, "{}", ran.text);
    let core = core_over(
        &host.state(),
        HostConfig::default(),
        None,
        BuiltIn::with_login(&[("HOME", &host.home().to_string_lossy())]),
    )
    .expect("the core over the same state");
    assert_eq!(ran.answer(), core.status(Some(&PlaceId::new(&place))));
    let person = host.flume(&repository.dir, &["status", &place], &[]);
    assert_eq!(person.code, 0);
    assert!(
        person.text.starts_with(&format!("{place}: fix@1")),
        "{}",
        person.text
    );
}

#[test]
fn a_refusal() {
    // S26, 61: done names a fix place, whose kind lands.
    let host = Host::new("refusal");
    let repository = repository("refusal", Some(STAND_IN_QUEUE));
    let kinds = kinds_file(&host, true);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let ran = host.flume(&repository.dir, &["done", "fix--one", "--json"], &[]);
    assert_eq!(ran.code, 1, "{}", ran.text);
    let place = format!("{}/places/fix--one", repository.name());
    let reason = ran
        .answer()
        .refusal()
        .map(str::to_string)
        .expect("a refusal");
    assert!(reason.contains(&place), "{reason}");
    let person = host.flume(&repository.dir, &["done", "fix--one"], &[]);
    assert_eq!(person.code, 1);
    assert!(person.text.starts_with("refused: "), "{}", person.text);
}

#[test]
fn needs_from_outside_an_agent() {
    // 41, C.4: no FLUME_AGENT, so nothing is asked of the core.
    let host = Host::new("needs");
    let repository = repository("needs", None);
    let ran = host.flume(&repository.dir, &["needs", "which design?", "--json"], &[]);
    assert_eq!(ran.code, 1, "{}", ran.text);
    let reason = ran
        .answer()
        .refusal()
        .map(str::to_string)
        .expect("a refusal");
    assert!(reason.contains("FLUME_AGENT"), "{reason}");
    let found = host.flume(
        &repository.dir,
        &["needs", "--found", "more work", "--json"],
        &[("FLUME_AGENT", "flume:x:fix--one:code:1")],
    );
    assert_eq!(found.code, 1, "{}", found.text);
    assert!(found.text.contains("FLUME_SECRET"), "{}", found.text);
    assert!(!host.state().join("places").exists(), "something changed");
}

#[test]
fn holds_and_recipients_at_the_launch() {
    // 7: a hold's words and a recipient named, then the variable's.
    let host = Host::new("holds");
    let repository = repository("holds", None);
    let kinds = kinds_file(&host, false);
    let first = host.stand_in(
        &repository.dir,
        &[
            "launch",
            "--kind",
            "change",
            "--name",
            "one",
            "--kinds",
            &kinds,
            "--hold",
            "verify=after review",
            "--recipient",
            "madswan/planner",
            "--json",
        ],
        &[("FLUME_RECIPIENT", "ignored/role")],
    );
    assert_eq!(first.code, 0, "{}", first.text);
    let second = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "change", "--name", "two", "--kinds", &kinds, "--json",
        ],
        &[("FLUME_RECIPIENT", "madswan/conductor")],
    );
    assert_eq!(second.code, 0, "{}", second.text);
    let one = host
        .flume(&repository.dir, &["status", "change--one", "--json"], &[])
        .answer();
    let one = the_place(&one);
    assert_eq!(one.recipients, vec!["madswan/planner".to_string()]);
    assert_eq!(
        one.hold_words,
        BTreeMap::from([("verify".to_string(), "after review".to_string())])
    );
    let two = host
        .flume(&repository.dir, &["status", "change--two", "--json"], &[])
        .answer();
    let two = the_place(&two);
    assert_eq!(two.recipients, vec!["madswan/conductor".to_string()]);
    assert!(two.hold_words.is_empty());
}

/// The current agent's stage, settings and their layers, as flume's
/// binary gives them.
fn current_agent(host: &Host, dir: &Path, place: &str) -> Value {
    let status = host.flume(dir, &["status", place, "--json"], &[]);
    assert_eq!(status.code, 0, "{}", status.text);
    status.json()["status"]["places"][0]["agent"].clone()
}

/// Moves the place's current stage on: its agent commits and its session
/// ends, and a settle starts the next.
fn deliver(host: &Host, dir: &Path, place: &str, stage: &str) {
    commit_in(place, &format!("{stage}.txt"));
    host.end_agent(place, stage, 1);
    let settled = host.stand_in(dir, &["settle", place, "--json"], &[]);
    assert_eq!(settled.code, 0, "{}", settled.text);
}

#[test]
fn agent_settings_at_the_launch() {
    // 7, 32: every stage's model, one stage's over it, and every stage's
    // variable, each at the launch layer.
    let host = Host::new("agent-settings");
    let repository = repository("agent-settings", None);
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch",
            "--kind",
            "change",
            "--name",
            "one",
            "--kinds",
            &kinds,
            "--agent",
            "model=opus",
            "--agent",
            "verify.model=sonnet",
            "--agent",
            "env.A=1",
            "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = format!("{}/places/change--one", repository.name());
    let launch = |model: &str| {
        serde_json::json!({
            "model": {"layer": "launch", "value": model},
            "env.A": {"layer": "launch", "value": "1"},
            "program": {"layer": "kind", "value": STAND_IN},
        })
    };
    for (stage, model) in [
        ("construct", "opus"),
        ("code", "opus"),
        ("verify", "sonnet"),
    ] {
        let agent = current_agent(&host, &repository.dir, &place);
        assert_eq!(agent["stage"], stage, "{agent}");
        assert_eq!(agent["settings"]["model"], model, "{agent}");
        assert_eq!(agent["settings"]["env"]["A"], "1", "{agent}");
        assert_eq!(agent["settings"]["from"], launch(model), "{agent}");
        if stage != "verify" {
            deliver(&host, &repository.dir, &place, stage);
        }
    }
}

#[test]
fn host_settings_for_a_kind() {
    // 32, C.7: the host's configuration gives change's agents a variable,
    // read at the start, held as written, handed to the agent read.
    let host = Host::new("host-settings");
    host.configure("[kinds.change.agents]\nenv = { WORK_HOME = \"~/work\" }\n\n[kinds.change.agents.stages.code]\nmodel = \"haiku\"\n");
    let repository = repository("host-settings", None);
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "change", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = format!("{}/places/change--one", repository.name());
    let agent = current_agent(&host, &repository.dir, &place);
    assert_eq!(agent["stage"], "construct");
    assert_eq!(agent["settings"]["env"]["WORK_HOME"], "~/work", "{agent}");
    assert_eq!(
        agent["settings"]["from"]["env.WORK_HOME"],
        serde_json::json!({"layer": "host-kind", "value": "~/work"})
    );
    assert!(agent["settings"]["from"].get("model").is_none(), "{agent}");
    let printed = agent["printed"].as_str().expect("the printed command");
    let expanded = format!("WORK_HOME={}/work", host.home().to_string_lossy());
    assert!(printed.contains(&expanded), "{printed}");
    deliver(&host, &repository.dir, &place, "construct");
    let agent = current_agent(&host, &repository.dir, &place);
    assert_eq!(agent["stage"], "code");
    assert_eq!(
        agent["settings"]["from"]["model"],
        serde_json::json!({"layer": "host-kind", "value": "haiku"})
    );
}

#[test]
fn a_place_named_from_inside_it() {
    // A place named by `<kind>--<name>` from inside its own worktree.
    let host = Host::new("from-inside");
    let repository = repository("from-inside", None);
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = repository.dir.join("places/fix--one");
    let ran = host.flume(&place, &["status", "fix--one", "--json"], &[]);
    assert_eq!(ran.code, 0, "{}", ran.text);
    let answer = ran.answer();
    assert_eq!(
        the_place(&answer).place,
        PlaceId::new(place.to_string_lossy())
    );
    let by_location = host.flume(
        &repository.dir,
        &["status", "places/fix--one", "--json"],
        &[],
    );
    assert_eq!(by_location.answer(), answer);
}

/// Every file under `dir` and its bytes.
fn files_under(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut found = vec![];
    let mut todo = vec![dir.to_path_buf()];
    while let Some(dir) = todo.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                todo.push(path);
            } else {
                found.push((path.clone(), std::fs::read(&path).unwrap_or_default()));
            }
        }
    }
    found.sort();
    found
}

#[test]
fn status_over_mcp() {
    // A scripted MCP session initializes `flume mcp`, lists its tools and
    // calls `status`: the tool's result is the JSON `flume status --json`
    // gives over the same state, and nothing is written (57, 60).
    let host = Host::new("mcp");
    let repository = repository("mcp", None);
    let kinds = kinds_file(&host, false);
    let launched = host.stand_in(
        &repository.dir,
        &[
            "launch", "--kind", "fix", "--name", "one", "--kinds", &kinds, "--json",
        ],
        &[],
    );
    assert_eq!(launched.code, 0, "{}", launched.text);
    let place = format!("{}/places/fix--one", repository.name());
    let status = host.flume(&repository.dir, &["status", &place, "--json"], &[]);
    let every = host.flume(&repository.dir, &["status", "--json"], &[]);
    let before = files_under(&host.state());

    let requests = [
        serde_json::json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "test", "version": "1"}}}),
        serde_json::json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
        serde_json::json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"}),
        serde_json::json!({"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "status", "arguments": {"place": place}}}),
        serde_json::json!({"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "status", "arguments": {}}}),
        serde_json::json!({"jsonrpc": "2.0", "id": 5, "method": "tools/call", "params": {"name": "status", "arguments": {"place": "/nowhere/places/fix--none"}}}),
        serde_json::json!({"jsonrpc": "2.0", "id": 6, "method": "settle"}),
    ];
    let input: String = requests
        .iter()
        .map(|request| format!("{request}\n"))
        .collect();
    let mut child = host
        .command(Path::new(env!("CARGO_BIN_EXE_flume")), &repository.dir, &[])
        .arg("mcp")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("flume mcp runs");
    child
        .stdin
        .take()
        .expect("its input")
        .write_all(input.as_bytes())
        .expect("the session written");
    let output = child.wait_with_output().expect("flume mcp ends");
    assert!(output.status.success());
    let replies: Vec<Value> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).expect("a JSON-RPC reply"))
        .collect();
    assert_eq!(replies.len(), 6, "one reply a request: {replies:?}");
    assert_eq!(replies[0]["id"], 1);
    assert_eq!(replies[0]["result"]["protocolVersion"], "2025-06-18");
    assert_eq!(replies[0]["result"]["serverInfo"]["name"], "flume");
    let tools = replies[1]["result"]["tools"].as_array().expect("the tools");
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0]["name"], "status");
    let text = |reply: &Value| {
        reply["result"]["content"][0]["text"]
            .as_str()
            .expect("the tool's text")
            .to_string()
    };
    assert_eq!(text(&replies[2]), status.text.trim());
    assert_eq!(replies[2]["result"]["isError"], false);
    assert_eq!(text(&replies[3]), every.text.trim());
    assert_eq!(replies[4]["result"]["isError"], true);
    assert_eq!(replies[5]["error"]["code"], -32601);
    assert_eq!(files_under(&host.state()), before, "status over MCP wrote");
}

//! The plugin's hooks run by the built binary (C.4; design 14, 17;
//! record 0016): a place launched by flume's own binary under the printed
//! command, its agent the command prints for Claude Code, and each hook
//! run as Claude Code 2.1.296 runs it, `flume hook <event>` with the
//! hook's input on standard input and the agent's environment, over a
//! fixture transcript on a scratch state directory. No Claude Code runs:
//! the test's own process stands in for the program the hook runs under,
//! a scratch `claude` on the hook's `PATH` naming it (161).

use super::agent_claude::fixture;
use crate::common::*;
use flume::profiles::place::git_worktree;
use serde_json::{json, Value};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// A host's scratch and a repository bound to `git-worktree` alone.
struct Host {
    repository: Repository,
}

/// One agent the printed command started, as its environment names it.
struct Agent {
    place: String,
    identity: String,
    secret: String,
    stage: String,
    plugin: PathBuf,
    transcript: PathBuf,
    /// The prompt the command carries, its last word.
    prompt: String,
    /// The `PATH` Claude Code hands its hooks: a scratch folder first whose
    /// `claude` is the program the hook runs under, this test's own.
    path: String,
}

/// What a command printed and its exit status.
struct Ran {
    out: String,
    code: i32,
}

impl Ran {
    fn json(&self) -> Value {
        serde_json::from_str(self.out.trim())
            .unwrap_or_else(|error| panic!("JSON ({error}): {}", self.out))
    }
}

/// A kind of two code stages that does not land, and a conversation.
const KINDS: &str = r#"[kinds.fix]
version = 1
branch = "fix"
starts_unready = true
lands = false

[[kinds.fix.stages]]
name = "code"
prompt = "prompts/work.md"
deliverables = ["commits"]
agents = [{}]

[[kinds.fix.stages]]
name = "review"
prompt = "prompts/work.md"
deliverables = ["commits"]
agents = [{}]

[kinds.talk]
version = 1
branch = "talk"
starts_unready = true
lands = false

[[kinds.talk.stages]]
name = "chat"
prompt = "prompts/work.md"
conversation = true
agents = [{}]
"#;

impl Host {
    fn new(label: &str) -> Host {
        let repository = Repository::new(label);
        repository.write(
            ".config/flume.toml",
            "[profiles]\nplace = \"git-worktree\"\n",
        );
        repository.commit("the binding");
        let host = Host { repository };
        for dir in [
            "state",
            "config/flume",
            "data",
            "home",
            "kinds/prompts",
            "bin",
        ] {
            std::fs::create_dir_all(host.path(dir)).expect("the host's directory");
        }
        // The hook's parent is this test's process, so the program the
        // start's record names, `claude`, is found as this test's own.
        std::os::unix::fs::symlink(
            std::env::current_exe().expect("this test's program"),
            host.path("bin/claude"),
        )
        .expect("the stand-in program");
        std::fs::write(
            host.path("kinds/prompts/work.md"),
            "Do the work of {{name}}. To stop short: {{needs}}\n",
        )
        .expect("the prompt");
        std::fs::write(host.path("kinds/kinds.toml"), KINDS).expect("the kinds");
        host
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.repository.scratch.path(relative)
    }

    /// flume's binary in the repository, `env` over nothing of the
    /// caller's flume, terminal or Claude Code session, `input` on its
    /// standard input.
    fn flume(&self, words: &[&str], env: &[(&str, &str)], input: &str) -> Ran {
        let mut command = Command::new(env!("CARGO_BIN_EXE_flume"));
        command.current_dir(&self.repository.dir);
        for (name, _) in std::env::vars() {
            if ["FLUME_", "HERDR_", "REX_", "CLAUDE"]
                .iter()
                .any(|prefix| name.starts_with(prefix))
            {
                command.env_remove(name);
            }
        }
        command
            .env("XDG_STATE_HOME", self.path("state"))
            .env("XDG_CONFIG_HOME", self.path("config"))
            .env("XDG_DATA_HOME", self.path("data"))
            .env("HOME", self.path("home"))
            .envs(env.iter().copied())
            .args(words)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn().expect("flume runs");
        child
            .stdin
            .take()
            .expect("its input")
            .write_all(input.as_bytes())
            .expect("the input written");
        let output = child.wait_with_output().expect("flume ends");
        Ran {
            out: String::from_utf8_lossy(&output.stdout).into_owned(),
            code: output.status.code().unwrap_or(-1),
        }
    }

    /// Launches `<kind>--one` and gives its agent, as the printed command
    /// names it.
    fn launch(&self, kind: &str) -> Agent {
        self.launch_with(kind, &[])
    }

    /// Launches `<kind>--one` with `more` words of the launch's own, and
    /// gives its agent.
    fn launch_with(&self, kind: &str, more: &[&str]) -> Agent {
        let kinds = self.path("kinds/kinds.toml");
        let kinds = kinds.to_string_lossy();
        let mut words = vec![
            "launch",
            "--kind",
            kind,
            "--name",
            "one",
            "--brief",
            "fix the parser",
            "--kinds",
            &kinds,
            "--recipient",
            "caller",
            "--json",
        ];
        words.extend(more);
        let ran = self.flume(&words, &[], "");
        assert_eq!(ran.code, 0, "the launch: {}", ran.out);
        let place = git_worktree::location(
            &self.repository.name(),
            &format!("{kind}--one"),
            Some(&self.path("data/flume")),
        )
        .expect("the place's location");
        self.agent(&place)
    }

    /// The place's current agent, from the command status prints.
    fn agent(&self, place: &str) -> Agent {
        let status = self.status(place);
        let agent = &status["places"][0]["agent"];
        let printed = agent["printed"].as_str().expect("a printed command");
        // The command's environment is in the file the line names, never
        // in the line (32, 56).
        let env = printed
            .split(' ')
            .map(|word| word.trim_matches('\''))
            .find(|word| word.ends_with(".env"))
            .map(|file| std::fs::read_to_string(file).expect("the command's environment"))
            .unwrap_or_else(|| panic!("no environment file in {printed}"));
        let word = |name: &str| {
            env.lines()
                .find_map(|line| line.strip_prefix(&format!("{name}=")))
                .map(|value| value.trim_matches('\'').to_string())
                .unwrap_or_else(|| panic!("{name} in {env}"))
        };
        let plugin = printed
            .split(' ')
            .skip_while(|word| *word != "--plugin-dir")
            .nth(1)
            .expect("the plugin's directory")
            .trim_matches('\'')
            .to_string();
        let prompt = printed
            .rsplit_once(" '")
            .map(|(_, quoted)| quoted.trim_end_matches('\'').replace("'\\''", "'"))
            .expect("the prompt, quoted");
        let identity = word("FLUME_AGENT");
        let stage = word("FLUME_STAGE");
        let transcript = self.path(&format!("sessions/{stage}.jsonl"));
        std::fs::create_dir_all(transcript.parent().expect("a folder")).expect("sessions");
        std::fs::write(&transcript, fixture("session.jsonl")).expect("the transcript");
        Agent {
            place: word("FLUME_PLACE"),
            identity,
            secret: word("FLUME_SECRET"),
            stage,
            plugin: PathBuf::from(plugin),
            transcript,
            prompt,
            path: format!(
                "{}:{}",
                self.path("bin").display(),
                std::env::var("PATH").unwrap_or_default()
            ),
        }
    }

    fn status(&self, place: &str) -> Value {
        let ran = self.flume(&["status", place, "--json"], &[], "");
        assert_eq!(ran.code, 0, "status: {}", ran.out);
        ran.json()["status"].clone()
    }

    /// The place's agent as status gives it.
    fn agent_status(&self, place: &str) -> Value {
        self.status(place)["places"][0]["agent"].clone()
    }

    /// The hook for `event` run as Claude Code runs it in `agent`'s
    /// session, with `input` over the fields every hook's input carries.
    fn hook(&self, agent: &Agent, event: &str, input: Value) -> Ran {
        self.hook_in(agent, event, input, &agent.env())
    }

    fn hook_in(
        &self,
        agent: &Agent,
        event: &str,
        mut input: Value,
        env: &[(String, String)],
    ) -> Ran {
        let common = json!({
            "session_id": "4e3c05c8-51c4-4449-8577-4f7038e394bf",
            "transcript_path": agent.transcript,
            "cwd": agent.place,
            "hook_event_name": event_name(event),
        });
        for (key, value) in common.as_object().expect("an object") {
            input
                .as_object_mut()
                .expect("an object")
                .entry(key)
                .or_insert(value.clone());
        }
        let env: Vec<(&str, &str)> = env
            .iter()
            .map(|(name, value)| (name.as_str(), value.as_str()))
            .collect();
        let ran = self.flume(&["hook", event], &env, &input.to_string());
        assert_eq!(ran.code, 0, "the hook {event} exits 0: {}", ran.out);
        ran
    }

    /// `flume hook status-line` in `env`, as the plugin's module runs it,
    /// its standard input held open and never written: it reads none, so
    /// it ends with it open (design 5).
    fn status_line(&self, env: &[(String, String)]) -> Ran {
        let mut command = Command::new(env!("CARGO_BIN_EXE_flume"));
        command.current_dir(&self.repository.dir);
        for (name, _) in std::env::vars() {
            if ["FLUME_", "HERDR_", "REX_", "CLAUDE"]
                .iter()
                .any(|prefix| name.starts_with(prefix))
            {
                command.env_remove(name);
            }
        }
        command
            .env("XDG_STATE_HOME", self.path("state"))
            .env("XDG_CONFIG_HOME", self.path("config"))
            .env("XDG_DATA_HOME", self.path("data"))
            .env("HOME", self.path("home"))
            .envs(env.iter().map(|(name, value)| (name, value)))
            .args(["hook", "status-line"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn().expect("flume runs");
        let input = child.stdin.take();
        let began = Instant::now();
        while child.try_wait().expect("flume's status").is_none() {
            assert!(
                began.elapsed() < Duration::from_secs(30),
                "status-line waited on its input"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
        drop(input);
        let output = child.wait_with_output().expect("flume ends");
        let ran = Ran {
            out: String::from_utf8_lossy(&output.stdout).into_owned(),
            code: output.status.code().unwrap_or(-1),
        };
        assert_eq!(ran.code, 0, "status-line exits 0: {}", ran.out);
        ran
    }

    /// Every file under flume's state directory and its text.
    fn state(&self) -> Vec<(PathBuf, Vec<u8>)> {
        let mut found = vec![];
        let mut todo = vec![self.path("state")];
        while let Some(dir) = todo.pop() {
            for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    todo.push(path);
                } else {
                    let text = std::fs::read(&path).unwrap_or_default();
                    found.push((path, text));
                }
            }
        }
        found.sort();
        found
    }
}

impl Agent {
    /// The environment Claude Code gives the agent's hooks: the four
    /// FLUME_* words, the plugin's root and its `PATH`.
    fn env(&self) -> Vec<(String, String)> {
        vec![
            ("FLUME_AGENT".into(), self.identity.clone()),
            ("FLUME_SECRET".into(), self.secret.clone()),
            ("FLUME_PLACE".into(), self.place.clone()),
            ("FLUME_STAGE".into(), self.stage.clone()),
            (
                "CLAUDE_PLUGIN_ROOT".into(),
                self.plugin.to_string_lossy().into_owned(),
            ),
            ("PATH".into(), self.path.clone()),
        ]
    }
}

/// The event Claude Code names for the word `flume hook` takes.
fn event_name(word: &str) -> &'static str {
    flume::profiles::agent::claude::hook::EVENTS
        .iter()
        .find(|(given, _)| *given == word)
        .map(|(_, name)| *name)
        .unwrap_or("Unknown")
}

/// The hook's answer as Claude Code reads it.
fn decision(ran: &Ran) -> Option<Value> {
    (!ran.out.trim().is_empty()).then(|| ran.json())
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

/// Reads status until `holds` holds of it, for up to `within`.
fn until(host: &Host, place: &str, within: Duration, holds: impl Fn(&Value) -> bool) -> Value {
    let began = Instant::now();
    loop {
        let status = host.status(place);
        if holds(&status) || began.elapsed() > within {
            return status;
        }
        std::thread::sleep(Duration::from_millis(250));
    }
}

#[test]
fn the_started_and_received_acks() {
    // SessionStart's ack makes the start started; UserPromptSubmit's acks
    // the submitted prompt by its digest; both clear the asking mark (35,
    // 89, 90, 135).
    let host = Host::new("acks");
    let agent = host.launch("fix");
    assert_eq!(host.agent_status(&agent.place)["started"], "no");

    let ran = host.hook(&agent, "session-start", json!({ "source": "startup" }));
    assert_eq!(decision(&ran), None);
    let status = host.agent_status(&agent.place);
    assert_eq!(status["started"], "yes", "{status}");
    assert_eq!(status["notes"], json!([]), "{status}");

    host.hook(
        &agent,
        "pre-tool-use",
        json!({ "tool_name": "AskUserQuestion", "tool_input": {}, "tool_use_id": "toolu_q" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "yes");
    host.hook(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": "go on  with\nthe parser" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "no");
    let record = flume::profiles::agent::claude::record::Record::named(
        &host
            .path("state/flume/places")
            .join(agent.place.trim_start_matches('/').replace('/', "%"))
            .to_string_lossy(),
        &agent.identity,
    );
    let wanted = flume::profiles::agent::claude::digest("go on with the parser");
    assert_eq!(record.received(&wanted), Some(true), "the prompt's digest");

    // An ack without the start's secret is refused, and writes nothing.
    let mut wrong = agent.env();
    wrong[1].1 = "not-the-secret".into();
    let before = host.state();
    host.hook_in(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": "x" }),
        &wrong,
    );
    assert_eq!(host.state(), before, "an ack without the secret wrote");
    let refused = host.flume(&["ack", "received", "--digest", "abc"], &[], "");
    assert_eq!(refused.code, 1, "{}", refused.out);
}

/// The agent's record in the place's own folder.
fn record_of(host: &Host, agent: &Agent) -> flume::profiles::agent::claude::record::Record {
    flume::profiles::agent::claude::record::Record::named(
        &host
            .path("state/flume/places")
            .join(agent.place.trim_start_matches('/').replace('/', "%"))
            .to_string_lossy(),
        &agent.identity,
    )
}

/// The host's configuration: a pool of one account whose environment
/// gives `CLAUDE_CONFIG_DIR` a scratch folder of the test's own.
fn with_an_account(host: &Host) -> PathBuf {
    let account = host.path("account-1");
    std::fs::create_dir_all(&account).expect("the account's folder");
    std::fs::write(
        host.path("config/flume/config.toml"),
        format!(
            "[host]\npool = \"work\"\n\n[pools.work]\nstrategy = \"fill-first\"\naccounts = [{{ name = \"work-1\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?} }} }}]\n",
            account.display().to_string()
        ),
    )
    .expect("the host's configuration");
    account
}

#[test]
fn a_variable_with_another_value() {
    // The start's record names CLAUDE_CONFIG_DIR; the hook's environment
    // holds another value: no started ack, the agent's record names the
    // variable as different and holds no value, a settle of the place is
    // started, which reads the start begun without its settings, and the
    // hook exits 0 (161).
    let host = Host::new("another-value");
    let account = with_an_account(&host);
    let agent = host.launch("fix");
    let mut env = agent.env();
    let other = host.path("account-elsewhere");
    env.push((
        "CLAUDE_CONFIG_DIR".into(),
        other.to_string_lossy().into_owned(),
    ));
    let ran = host.hook_in(
        &agent,
        "session-start",
        json!({ "source": "startup" }),
        &env,
    );
    assert_eq!(decision(&ran), None);
    assert_eq!(host.agent_status(&agent.place)["started"], "no");
    let record = record_of(&host, &agent);
    assert!(record.started().is_none(), "a started ack was written");
    let unsettled = record
        .unsettled()
        .expect("the word that it began otherwise");
    assert_eq!(unsettled.unsettled.different, vec!["CLAUDE_CONFIG_DIR"]);
    assert!(unsettled.unsettled.missing.is_empty() && !unsettled.unsettled.program);
    let written =
        std::fs::read_to_string(record.dir().join("unsettled.json")).expect("the word as written");
    for value in [account.to_string_lossy(), other.to_string_lossy()] {
        assert!(!written.contains(value.as_ref()), "a value in {written}");
    }
    let status = until(&host, &agent.place, Duration::from_secs(20), |status| {
        status["places"][0]["unsettled"]
            .as_array()
            .is_some_and(|read| !read.is_empty())
    });
    let read = &status["places"][0]["unsettled"][0];
    assert_eq!(read["different"], json!(["CLAUDE_CONFIG_DIR"]), "{status}");
    assert_eq!(read["start"], 1, "{status}");

    // The same variable as the start gave it: the ack is taken.
    let host = Host::new("same-value");
    let account = with_an_account(&host);
    let agent = host.launch("fix");
    let mut env = agent.env();
    env.push((
        "CLAUDE_CONFIG_DIR".into(),
        account.to_string_lossy().into_owned(),
    ));
    host.hook_in(
        &agent,
        "session-start",
        json!({ "source": "startup" }),
        &env,
    );
    assert_eq!(host.agent_status(&agent.place)["started"], "yes");
}

#[test]
fn a_credential_the_program_keeps_from_its_hooks() {
    // The account gives CLAUDE_CONFIG_DIR and CLAUDE_CODE_OAUTH_TOKEN;
    // Claude Code keeps the token from its hooks, so the hook's
    // environment holds CLAUDE_CONFIG_DIR alone. The start's record names
    // no token and the ack is taken (161).
    let host = Host::new("withheld");
    let account = host.path("account-1");
    std::fs::create_dir_all(&account).expect("the account's folder");
    std::fs::write(
        host.path("config/flume/config.toml"),
        format!(
            "[host]\npool = \"work\"\n\n[pools.work]\nstrategy = \"fill-first\"\naccounts = [{{ name = \"work-1\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?}, CLAUDE_CODE_OAUTH_TOKEN = \"not-a-token\" }} }}]\n",
            account.display().to_string()
        ),
    )
    .expect("the host's configuration");
    let agent = host.launch("fix");
    let mut env = agent.env();
    env.push((
        "CLAUDE_CONFIG_DIR".into(),
        account.to_string_lossy().into_owned(),
    ));
    host.hook_in(
        &agent,
        "session-start",
        json!({ "source": "startup" }),
        &env,
    );
    assert_eq!(host.agent_status(&agent.place)["started"], "yes");
    fn records(value: &Value, found: &mut Vec<Value>) {
        match value {
            Value::Object(fields) => {
                for (key, field) in fields {
                    if key == "start_record" {
                        found.push(field.clone());
                    } else {
                        records(field, found);
                    }
                }
            }
            Value::Array(items) => items.iter().for_each(|item| records(item, found)),
            _ => {}
        }
    }
    let mut found = vec![];
    for (_, text) in host.state() {
        if let Ok(document) = serde_json::from_slice::<Value>(&text) {
            records(&document, &mut found);
        }
    }
    let records = found;
    assert_eq!(records.len(), 1, "the start's record");
    let names: Vec<&String> = records[0]["env"]
        .as_object()
        .expect("the record's variables")
        .keys()
        .collect();
    assert_eq!(
        names,
        vec![
            "CLAUDE_CONFIG_DIR",
            "FLUME_AGENT",
            "FLUME_PLACE",
            "FLUME_STAGE"
        ]
    );
}

#[test]
fn another_program() {
    // The hook's parent runs another program than the `claude` its PATH
    // finds: no started ack, and the agent's record says the program
    // differed (161).
    let host = Host::new("another-program");
    let agent = host.launch("fix");
    let elsewhere = host.path("elsewhere");
    std::fs::create_dir_all(&elsewhere).expect("a folder");
    std::os::unix::fs::symlink("/bin/sh", elsewhere.join("claude")).expect("another program");
    let mut env = agent.env();
    env[5].1 = format!(
        "{}:{}",
        elsewhere.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    host.hook_in(
        &agent,
        "session-start",
        json!({ "source": "startup" }),
        &env,
    );
    assert_eq!(host.agent_status(&agent.place)["started"], "no");
    let unsettled = record_of(&host, &agent)
        .unsettled()
        .expect("the word that it began otherwise");
    assert!(unsettled.unsettled.program, "{unsettled:?}");
    assert!(unsettled.unsettled.missing.is_empty() && unsettled.unsettled.different.is_empty());
}

/// The start's mode as its record names it, from the place's held state.
fn recorded_mode(host: &Host, agent: &Agent) -> Option<String> {
    let status = host.status(&agent.place);
    status["places"][0]["agent"]["mode"]["passed"]
        .as_str()
        .map(str::to_string)
}

/// The start's first prompt submitted as Claude Code 2.1.296 runs
/// UserPromptSubmit, its input naming the session's mode where `mode` is
/// given.
fn first_prompt(host: &Host, agent: &Agent, mode: Option<&str>) -> Ran {
    let mut input = json!({ "prompt": agent.prompt });
    if let Some(mode) = mode {
        input["permission_mode"] = json!(mode);
    }
    host.hook(agent, "user-prompt-submit", input)
}

#[test]
fn another_mode() {
    // The start's record names bypassPermissions. SessionStart's input
    // names no mode, as 2.1.296's never does, and the started ack is taken
    // on the program and the variables. The start's first prompt's input
    // reads `default`, as Claude Code begins where the account's
    // configuration disables the mode asked: the prompt is blocked with
    // words naming both modes, so the agent takes no turn, no received ack
    // is written, the agent's record names the mode asked and the mode
    // begun as the record would name it with the started ack's session, a
    // settle of the place is started, which reads the start begun without
    // its settings, and the hook exits 0 (161, 162, S67).
    let host = Host::new("another-mode");
    let agent = host.launch_with("fix", &["--agent", "mode=full"]);
    assert_eq!(
        recorded_mode(&host, &agent).as_deref(),
        Some("bypassPermissions")
    );
    let ran = host.hook(&agent, "session-start", json!({ "source": "startup" }));
    assert_eq!(decision(&ran), None);
    assert_eq!(host.agent_status(&agent.place)["started"], "yes");
    let ran = first_prompt(&host, &agent, Some("default"));
    let blocked = decision(&ran).expect("the prompt blocked");
    assert_eq!(blocked["decision"], "block", "{blocked}");
    assert!(
        blocked["reason"].as_str().is_some_and(|reason| reason
            .contains("in the mode `manual`, where its start asked `bypassPermissions`")),
        "{blocked}"
    );
    let record = record_of(&host, &agent);
    assert!(!record.received_any(), "the prompt held back was acked");
    let unsettled = record
        .unsettled()
        .expect("the word that it began otherwise");
    assert_eq!(
        unsettled.unsettled.mode,
        Some(flume_seams::ModeBegan {
            asked: "bypassPermissions".into(),
            began: Some("manual".into()),
        })
    );
    assert!(
        unsettled.unsettled.missing.is_empty()
            && unsettled.unsettled.different.is_empty()
            && !unsettled.unsettled.program,
        "{unsettled:?}"
    );
    assert_eq!(
        Path::new(&unsettled.transcript_path),
        agent.transcript.as_path(),
        "the started ack's transcript"
    );
    let status = until(&host, &agent.place, Duration::from_secs(20), |status| {
        status["places"][0]["unsettled"]
            .as_array()
            .is_some_and(|read| !read.is_empty())
    });
    let read = &status["places"][0]["unsettled"][0];
    assert_eq!(
        read["mode"],
        json!({ "asked": "bypassPermissions", "began": "manual" }),
        "{status}"
    );
}

#[test]
fn manual_read_back_as_default() {
    // Claude Code 2.1.296 reads `--permission-mode manual` as its own
    // `default` and reports it so: the first prompt is taken (161, C.4).
    let host = Host::new("manual-mode");
    let agent = host.launch_with("fix", &["--agent", "mode=ask"]);
    assert_eq!(recorded_mode(&host, &agent).as_deref(), Some("manual"));
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let ran = first_prompt(&host, &agent, Some("default"));
    assert_eq!(decision(&ran), None);
    let record = record_of(&host, &agent);
    assert!(record.received_any(), "the first prompt was not acked");
    assert!(record.unsettled().is_none());
}

#[test]
fn no_mode_in_the_record() {
    // A start whose settings give no mode passes none and compares none,
    // whatever mode the account's configuration began the session in
    // (161, 162).
    let host = Host::new("no-mode");
    let agent = host.launch("fix");
    assert_eq!(recorded_mode(&host, &agent), None);
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let ran = first_prompt(&host, &agent, Some("acceptEdits"));
    assert_eq!(decision(&ran), None);
    let record = record_of(&host, &agent);
    assert!(record.received_any(), "the first prompt was not acked");
    assert!(record.unsettled().is_none());
}

#[test]
fn no_mode_in_the_input() {
    // The start's record names acceptEdits and its first prompt's input
    // names no mode: the prompt is blocked, no received ack, the agent's
    // record naming the mode asked and no mode begun (161).
    let host = Host::new("no-mode-input");
    let agent = host.launch_with("fix", &["--agent", "mode=edits"]);
    assert_eq!(recorded_mode(&host, &agent).as_deref(), Some("acceptEdits"));
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let ran = first_prompt(&host, &agent, None);
    assert_eq!(
        decision(&ran).map(|blocked| blocked["decision"].clone()),
        Some(json!("block"))
    );
    let record = record_of(&host, &agent);
    assert!(!record.received_any(), "the prompt held back was acked");
    let unsettled = record
        .unsettled()
        .expect("the word that it began otherwise");
    assert_eq!(
        unsettled.unsettled.mode,
        Some(flume_seams::ModeBegan {
            asked: "acceptEdits".into(),
            began: None,
        })
    );
}

#[test]
fn a_later_prompt_and_a_compaction_after_the_mode_moved() {
    // A start's first prompt taken in acceptEdits; once started, a person
    // moves its session's mode. A later prompt, a tell, compares no mode
    // and is acked received; the session compacts, SessionStart compares
    // no mode either, so the started ack is written again with the
    // session's transcript, and nothing is unsettled (161, 162).
    let host = Host::new("compaction-mode");
    let agent = host.launch_with("fix", &["--agent", "mode=edits"]);
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let ran = first_prompt(&host, &agent, Some("acceptEdits"));
    assert_eq!(decision(&ran), None);
    let ran = host.hook(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": "a tell", "permission_mode": "default" }),
    );
    assert_eq!(decision(&ran), None, "a later prompt blocked");
    let compacted = host.path("sessions/compacted.jsonl");
    std::fs::write(&compacted, fixture("session.jsonl")).expect("the transcript");
    host.hook(
        &agent,
        "session-start",
        json!({ "source": "compact", "transcript_path": compacted }),
    );
    let record = record_of(&host, &agent);
    let started = record.started().expect("the started ack");
    assert_eq!(started.source, "compact");
    assert_eq!(
        Path::new(&started.transcript_path),
        compacted.as_path(),
        "{started:?}"
    );
    assert!(
        record.unsettled().is_none(),
        "unsettled after the mode moved"
    );
    assert_eq!(
        std::fs::read_to_string(record.received_path())
            .unwrap_or_default()
            .lines()
            .count(),
        2,
        "both prompts acked received"
    );
    assert_eq!(host.agent_status(&agent.place)["started"], "yes");
}

#[test]
fn an_ack_by_hand_with_no_mode() {
    // `flume ack received` at the first prompt of a start whose record
    // names acceptEdits: with `--mode acceptEdits` it is taken; with no
    // `--mode` it acks nothing, naming the mode asked and no mode begun
    // (135, 161).
    let by_hand = |label: &str, mode: Option<&str>| {
        let host = Host::new(label);
        let agent = host.launch_with("fix", &["--agent", "mode=edits"]);
        let mut words = vec!["ack", "received", "--digest", "a1b2c3"];
        if let Some(mode) = mode {
            words.extend(["--mode", mode]);
        }
        let env = agent.env();
        let env: Vec<(&str, &str)> = env
            .iter()
            .map(|(name, value)| (name.as_str(), value.as_str()))
            .collect();
        let ran = host.flume(&words, &env, "");
        let record = record_of(&host, &agent);
        (ran, record, host)
    };

    let (ran, record, _host) = by_hand("ack-with-mode", Some("acceptEdits"));
    assert_eq!(ran.code, 0, "{}", ran.out);
    assert_eq!(
        record.received("a1b2c3"),
        Some(true),
        "the ack was not taken"
    );
    assert!(record.unsettled().is_none());

    let (ran, record, _host) = by_hand("ack-without-mode", None);
    assert_eq!(ran.code, 1, "{}", ran.out);
    assert!(!record.received_any(), "a received ack was written");
    assert_eq!(
        record.unsettled().map(|unsettled| unsettled.unsettled.mode),
        Some(Some(flume_seams::ModeBegan {
            asked: "acceptEdits".into(),
            began: None,
        }))
    );
    assert!(
        ran.out
            .contains("in a mode its word did not name, where its start asked `acceptEdits`"),
        "{}",
        ran.out
    );
}

#[test]
fn a_prompt_submitted_as_a_paste_is_acked_as_sent() {
    // A terminal sends the prompt as a paste, and Claude Code 2.1.296,
    // where the account's features turn paste tags on, submits it inside
    // `<pasted_content id="....">` tags, newlines around them: the ack is
    // the digest of the prompt as sent, so the stage begins (38, 89;
    // record 0016).
    let host = Host::new("paste-acks");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    host.hook(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": "\n\n<pasted_content id=\"fad0\">\nfix the  parser\nand commit it\n</pasted_content id=\"fad0\">\n" }),
    );
    let record = flume::profiles::agent::claude::record::Record::named(
        &host
            .path("state/flume/places")
            .join(agent.place.trim_start_matches('/').replace('/', "%"))
            .to_string_lossy(),
        &agent.identity,
    );
    let wanted = flume::profiles::agent::claude::digest("fix the parser and commit it");
    assert_eq!(record.received(&wanted), Some(true), "the prompt's digest");
}

#[test]
fn a_subagents_input_acks_nothing() {
    let host = Host::new("subagent-acks");
    let agent = host.launch("fix");
    host.hook(
        &agent,
        "session-start",
        json!({ "source": "startup", "agent_id": "a1b2", "agent_type": "Explore" }),
    );
    assert_eq!(host.agent_status(&agent.place)["started"], "no");
}

#[test]
fn a_bare_resume_acks_nothing() {
    // herdr's bare `claude --resume` carries no FLUME_* and loads no
    // plugin directory: its SessionStart acks nothing, and the start is
    // not started (77, S41).
    let host = Host::new("bare-resume");
    let agent = host.launch("fix");
    let before = host.state();
    let ran = host.hook_in(&agent, "session-start", json!({ "source": "resume" }), &[]);
    assert_eq!(decision(&ran), None);
    assert_eq!(host.state(), before, "a session flume did not start wrote");
    assert_eq!(host.agent_status(&agent.place)["started"], "no");
}

#[test]
fn a_plugin_of_another_version() {
    // The started ack names the plugin's version; one other than the
    // binary's is reported with the agent (135).
    let host = Host::new("plugin-version");
    let agent = host.launch("fix");
    let other = host.path("other-plugin");
    std::fs::create_dir_all(other.join(".claude-plugin")).expect("a plugin");
    std::fs::write(
        other.join(".claude-plugin/plugin.json"),
        r#"{"name": "flume", "version": "9.9.9"}"#,
    )
    .expect("its manifest");
    let mut env = agent.env();
    env[4].1 = other.to_string_lossy().into_owned();
    host.hook_in(
        &agent,
        "session-start",
        json!({ "source": "startup" }),
        &env,
    );
    let status = host.agent_status(&agent.place);
    assert_eq!(status["started"], "yes");
    let notes = status["notes"].to_string();
    assert!(notes.contains("9.9.9"), "{status}");
}

#[test]
fn an_agent_tries_to_push() {
    // S25: the guard refuses the push, status reports it, and the stage
    // goes on (116, 108).
    let host = Host::new("push");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let ran = host.hook(
        &agent,
        "pre-tool-use",
        json!({
            "tool_name": "Bash",
            "tool_input": { "command": "git add -A && git push origin HEAD" },
            "tool_use_id": "toolu_push",
        }),
    );
    let answer = decision(&ran).expect("a refusal");
    assert_eq!(answer["hookSpecificOutput"]["hookEventName"], "PreToolUse");
    assert_eq!(answer["hookSpecificOutput"]["permissionDecision"], "deny");
    let reason = answer["hookSpecificOutput"]["permissionDecisionReason"]
        .as_str()
        .unwrap_or_default();
    assert!(reason.contains("flume submits this place"), "{reason}");
    let status = host.status(&agent.place);
    let place = &status["places"][0];
    let acts = place["refused_acts"].to_string();
    assert!(acts.contains("git push"), "{status}");
    assert_eq!(place["current"], "code", "{status}");
}

#[test]
fn a_subagents_push_is_refused() {
    // The guard refuses a subagent's command as its agent's (C.4).
    let host = Host::new("subagent-push");
    let agent = host.launch("fix");
    let ran = host.hook(
        &agent,
        "pre-tool-use",
        json!({
            "tool_name": "Bash",
            "tool_input": { "command": "cd sub && sluice submit main --no-wait" },
            "tool_use_id": "toolu_sub",
            "agent_id": "a1b2",
            "agent_type": "general-purpose",
        }),
    );
    let answer = decision(&ran).expect("a refusal");
    assert_eq!(answer["hookSpecificOutput"]["permissionDecision"], "deny");
    let status = host.status(&agent.place);
    assert!(
        status["places"][0]["refused_acts"]
            .to_string()
            .contains("sluice submit"),
        "{status}"
    );
}

#[test]
fn an_agent_merges_the_base_for_a_conflict() {
    // A conflict's fix merges the base into the place's branch: never
    // refused, nothing recorded (112).
    let host = Host::new("merge");
    let agent = host.launch("fix");
    let ran = host.hook(
        &agent,
        "pre-tool-use",
        json!({
            "tool_name": "Bash",
            "tool_input": { "command": "git fetch origin && git merge 4f2a9c1" },
            "tool_use_id": "toolu_merge",
        }),
    );
    assert_eq!(decision(&ran), None);
    let status = host.status(&agent.place);
    assert_eq!(status["places"][0]["refused_acts"], json!([]), "{status}");
}

#[test]
fn a_dismissed_question_clears_its_mark() {
    // A question dismissed with no PostToolUse: the next prompt's hook
    // clears the mark, and asking reads no (90).
    let host = Host::new("dismissed");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    host.hook(
        &agent,
        "pre-tool-use",
        json!({ "tool_name": "AskUserQuestion", "tool_input": {"questions": []}, "tool_use_id": "toolu_ask" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "yes");
    host.hook(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": "never mind" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "no");

    // A plan put to a person is asking too, until the call's own
    // PostToolUse; another call's does not clear it.
    host.hook(
        &agent,
        "pre-tool-use",
        json!({ "tool_name": "ExitPlanMode", "tool_input": {"plan": "p"}, "tool_use_id": "toolu_plan" }),
    );
    host.hook(
        &agent,
        "post-tool-use",
        json!({ "tool_name": "Read", "tool_input": {}, "tool_response": {}, "tool_use_id": "toolu_other" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "yes");
    host.hook(
        &agent,
        "post-tool-use",
        json!({ "tool_name": "ExitPlanMode", "tool_input": {"plan": "p"}, "tool_response": {}, "tool_use_id": "toolu_plan" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "no");
}

#[test]
fn a_permission_prompt_is_asking() {
    // Claude Code asks a person's permission: PermissionRequest leaves the
    // mark, the permitted call's PostToolUse clears it (90).
    let host = Host::new("permission");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let call = json!({ "command": "rm -rf build" });
    let ran = host.hook(
        &agent,
        "permission-request",
        json!({ "tool_name": "Bash", "tool_input": call }),
    );
    assert_eq!(decision(&ran), None, "the hook decides no permission");
    assert_eq!(host.agent_status(&agent.place)["asking"], "yes");
    host.hook(
        &agent,
        "post-tool-use",
        json!({ "tool_name": "Bash", "tool_input": call, "tool_response": {}, "tool_use_id": "toolu_rm" }),
    );
    assert_eq!(host.agent_status(&agent.place)["asking"], "no");
}

#[test]
fn a_stop_failure_as_last_words() {
    // A usage limit ends the turn: StopFailure's reason is the agent's
    // last words, which status gives with the agent, and the agent's
    // record holds the failure's error, its words and when the hook ran,
    // from which the profile reads the usage limit and its reset (43,
    // 151, C.4).
    let host = Host::new("stop-failure");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let before = now();
    host.hook(
        &agent,
        "stop-failure",
        json!({
            "error": "rate_limit",
            "last_assistant_message": "You've hit your session limit · resets 5pm (UTC)",
        }),
    );
    let after = now();
    let status = host.agent_status(&agent.place);
    assert_eq!(
        status["last_words"], "You've hit your session limit · resets 5pm (UTC) (rate_limit)",
        "{status}"
    );
    let folder = host
        .path("state/flume/places")
        .join(agent.place.trim_start_matches('/').replace('/', "%"));
    let record = flume::profiles::agent::claude::record::Record::named(
        &folder.to_string_lossy(),
        &agent.identity,
    );
    let failure = record.failure().expect("the failure recorded");
    assert_eq!(failure.error, "rate_limit");
    assert_eq!(
        failure.words,
        "You've hit your session limit · resets 5pm (UTC)"
    );
    assert!(
        (before..=after).contains(&failure.at),
        "the hook ran at {}, between {before} and {after}",
        failure.at
    );
}

/// The host's clock, in seconds.
fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("after 1970")
        .as_secs()
}

#[test]
fn the_stop_check_sends_back_then_the_stop_settles() {
    // S24: the agent tries to stop with nothing made, and the stop is
    // blocked with words naming commits and `flume needs`. It commits; the
    // next stop is let through and starts `flume settle <place>
    // --on-quiet`, which settles the place once the agent is quiet (under
    // the printed command, once its session ended): the code stage ends at
    // its deliverable and the next starts, with no settle of the test's
    // (S29, 46, 115, 121).
    let host = Host::new("stop");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    host.hook(
        &agent,
        "user-prompt-submit",
        json!({ "prompt": agent.prompt }),
    );
    assert!(agent.prompt.contains("flume needs"), "{}", agent.prompt);
    let ran = host.hook(&agent, "stop", json!({ "stop_hook_active": false }));
    let answer = decision(&ran).expect("the stop blocked");
    assert_eq!(answer["decision"], "block");
    let reason = answer["reason"].as_str().unwrap_or_default();
    assert!(reason.contains("commits"), "{reason}");
    assert!(reason.contains("flume needs"), "{reason}");

    commit_in(&agent.place, "parser.rs");
    let ran = host.hook(&agent, "stop", json!({ "stop_hook_active": true }));
    assert_eq!(decision(&ran), None, "a stop with the deliverable made");
    host.hook(
        &agent,
        "session-end",
        json!({ "reason": "prompt_input_exit" }),
    );
    let status = until(&host, &agent.place, Duration::from_secs(20), |status| {
        status["places"][0]["current"] == "review"
    });
    let place = &status["places"][0];
    assert_eq!(place["current"], "review", "{status}");
    assert_eq!(place["stages"][0]["state"], "delivered", "{status}");
}

#[test]
fn a_conversations_agent_stops() {
    // The stop check never blocks a conversation's agent (109).
    let host = Host::new("conversation");
    let agent = host.launch("talk");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let ran = host.hook(&agent, "stop", json!({ "stop_hook_active": false }));
    assert_eq!(decision(&ran), None);
}

#[test]
fn a_session_the_operator_started() {
    // Outside an agent flume started, every hook allows what it was asked
    // and writes nothing (C.4).
    let host = Host::new("operator");
    let agent = host.launch("fix");
    let before = host.state();
    for (word, _) in flume::profiles::agent::claude::hook::EVENTS {
        let input = json!({
            "tool_name": "Bash",
            "tool_input": { "command": "git push" },
            "tool_use_id": "toolu_x",
            "prompt": "hello",
            "source": "startup",
            "error": "rate_limit",
            "reason": "other",
        });
        let ran = host.hook_in(&agent, word, input, &[]);
        assert_eq!(decision(&ran), None, "{word}");
    }
    assert_eq!(host.state(), before, "a hook wrote outside flume's agent");
}

/// The pane herdr's environment names in a session flume did not start,
/// on a server whose socket nothing listens on: herdr is asked nothing to
/// name the session (7, C.5).
fn pane_env(host: &Host, pane: &str) -> Vec<(String, String)> {
    vec![
        ("HERDR_PANE_ID".into(), pane.into()),
        (
            "HERDR_SOCKET_PATH".into(),
            host.path("no-herdr.sock").to_string_lossy().into_owned(),
        ),
    ]
}

/// The session herdr's environment names for `pane`, as flume names it.
fn pane_session(host: &Host, pane: &str) -> String {
    format!("herdr:{pane}@{}", host.path("no-herdr.sock").display())
}

/// The fix `fix--one` launched for the recipient `recipient` beside the
/// test's caller, its code agent started and its prompt received, stopped
/// short asking which file holds the log, and the place settled, so it
/// waits on a retry of code; then the host's configuration names herdr as
/// its terminal, whose environment names the calling session, the fix's
/// start keeping the terminal it recorded.
fn asks_for(host: &Host, recipient: &str) -> Agent {
    let agent = host.launch_with("fix", &["--recipient", recipient]);
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    first_prompt(host, &agent, None);
    let env = agent.env();
    let env: Vec<(&str, &str)> = env
        .iter()
        .map(|(name, value)| (name.as_str(), value.as_str()))
        .collect();
    let asked = host.flume(&["needs", "Which file holds the log?", "--json"], &env, "");
    assert_eq!(asked.code, 0, "the stop-short: {}", asked.out);
    let settled = host.flume(&["settle", &agent.place, "--json"], &[], "");
    assert_eq!(settled.code, 0, "the settle: {}", settled.out);
    let status = host.status(&agent.place);
    assert_eq!(
        status["places"][0]["waiting_on"]["why"], "answer",
        "{status}"
    );
    std::fs::write(
        host.path("config/flume/config.toml"),
        "[host]\nterminal = \"herdr\"\n",
    )
    .expect("the configuration");
    agent
}

/// The added context SessionStart gave, where it gave any.
fn context(ran: &Ran) -> Option<String> {
    if ran.out.trim().is_empty() {
        return None;
    }
    let output = ran.json();
    assert_eq!(
        output["hookSpecificOutput"]["hookEventName"], "SessionStart",
        "{output}"
    );
    Some(
        output["hookSpecificOutput"]["additionalContext"]
            .as_str()
            .expect("the added context")
            .to_string(),
    )
}

/// SessionStart run in a session flume did not start, in herdr's pane
/// `pane`, `more` over its environment, with the fixture's input.
fn session_start_in(host: &Host, agent: &Agent, pane: &str, more: &[(&str, &str)]) -> Ran {
    let mut env = pane_env(host, pane);
    env.extend(
        more.iter()
            .map(|(name, value)| (name.to_string(), value.to_string())),
    );
    let input: Value = serde_json::from_str(
        &fixture_text("session-start-another-session.json")
            .replace("__TRANSCRIPT__", &agent.transcript.to_string_lossy())
            .replace("__CWD__", &host.repository.dir.to_string_lossy()),
    )
    .expect("the fixture's JSON");
    host.hook_in(agent, "session-start", input, &env)
}

/// The plugin's own fixture `name`, as Claude Code 2.1.296 hands a hook
/// its input.
fn fixture_text(name: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../plugins/claude/tests/fixtures")
            .join(name),
    )
    .expect("the plugin's fixture")
}

#[test]
fn a_restarted_pane() {
    // S94, 43, C.4, design 9; weir 103: the claude-code-plugin spec's "A
    // restarted pane": a session flume did not start, begun again in the
    // pane a stop-short was handed to before, is given at its start the
    // place, the stop-short's words, its key and the three ways on; the
    // inbox it ran is the pane's and confirms on receipt, so the
    // stop-short reads handed to the pane by session_start and confirmed
    // by it, and the place waits as it did.
    let host = Host::new("restarted-pane");
    let session = pane_session(&host, "w1-p2");
    let agent = asks_for(&host, &session);
    let collected = host.flume(&["inbox", "--recipient", &session, "--json"], &[], "");
    assert_eq!(collected.code, 0, "the inbox: {}", collected.out);
    assert_eq!(
        collected.json()["inbox"]["ends"].as_array().map(Vec::len),
        Some(1),
        "{}",
        collected.out
    );
    let ran = session_start_in(&host, &agent, "w1-p2", &[]);
    let words = context(&ran).unwrap_or_else(|| panic!("no context given: {}", ran.out));
    for said in [
        &session,
        "fix--one (fix the parser), in the stream default, waits on a retry of code",
        "Its stop-short stage/code/1/ended-short needs an answer:",
        "  Which file holds the log?",
        "- answer it, naming its key: flume retry fix--one --stage code --answers stage/code/1/ended-short --words \"<answer>\"",
        "- hold the move with your words, saying who has it: flume hold fix--one \"<words>\" --waiting --for <time>",
        "- leave it: once it stalls, the answerer behind its stream has it.",
    ] {
        assert!(words.contains(said), "{said:?} not in:\n{words}");
    }
    assert!(
        words.contains("Ends delivered to this session now")
            && words.contains("[stage/code/1/ended-short]"),
        "the stop-short handed before not given:\n{words}"
    );
    assert_eq!(ran.code, 0, "{}", ran.out);
    let status = host.status(&agent.place);
    let end = status["places"][0]["ends"]
        .as_array()
        .expect("ends")
        .iter()
        .find(|end| end["key"] == "stage/code/1/ended-short")
        .unwrap_or_else(|| panic!("no stop-short: {status}"))
        .clone();
    let handed: Vec<&str> = end["deliveries"]
        .as_array()
        .expect("deliveries")
        .iter()
        .filter(|delivery| delivery["recipient"] == session.as_str())
        .filter_map(|delivery| delivery["by"].as_str())
        .collect();
    assert_eq!(handed, vec!["inbox", "session_start"], "{end}");
    assert!(
        end["confirmed"]
            .as_array()
            .expect("confirmed")
            .iter()
            .any(|confirmed| confirmed["recipient"] == session.as_str()),
        "{end}"
    );
    assert!(
        !end["pending_for"]
            .as_array()
            .expect("pending_for")
            .iter()
            .any(|pending| pending == session.as_str()),
        "{end}"
    );
}

#[test]
fn an_end_pending_for_the_pane_collected_at_its_start() {
    // 43, C.4, design 9: an end still pending for the pane is collected by
    // SessionStart's inbox, given with its key, and delivered.
    let host = Host::new("pending-for-pane");
    let session = pane_session(&host, "w1-p2");
    let agent = asks_for(&host, &session);
    let ran = session_start_in(&host, &agent, "w1-p2", &[]);
    let words = context(&ran).unwrap_or_else(|| panic!("no context given: {}", ran.out));
    assert!(
        words.contains(
            "- fix--one code needs an answer [stage/code/1/ended-short]; read: flume status fix--one"
        ),
        "{words}"
    );
    let again = host.flume(&["inbox", "--recipient", &session, "--json"], &[], "");
    assert_eq!(
        again.json()["inbox"]["ends"].as_array().map(Vec::len),
        Some(0),
        "collected again: {}",
        again.out
    );
}

#[test]
fn nothing_waits_on_the_session() {
    // 43, C.4: SessionStart in a session nothing is pending for and
    // nothing waits on gives nothing, and exits 0.
    let host = Host::new("nothing-waits");
    let agent = asks_for(&host, &pane_session(&host, "w1-p2"));
    let ran = session_start_in(&host, &agent, "w9-p9", &[]);
    assert_eq!(context(&ran), None, "{}", ran.out);
}

#[test]
fn a_recipient_in_the_environment() {
    // 43, C.4, design 9: SessionStart collects as the session the
    // terminal names, never as the environment's recipient, so the end
    // pending for that recipient stays pending for it.
    let host = Host::new("recipient-in-env");
    let agent = asks_for(&host, "planner");
    let ran = session_start_in(&host, &agent, "w9-p9", &[("FLUME_RECIPIENT", "planner")]);
    assert_eq!(context(&ran), None, "{}", ran.out);
    let status = host.status(&agent.place);
    let ends = status["places"][0]["ends"].as_array().expect("ends");
    assert!(
        ends.iter().any(|end| end["pending_for"]
            .as_array()
            .is_some_and(|named| named.contains(&json!("planner")))),
        "the end is not pending for planner: {status}"
    );
}

#[test]
fn flume_unreachable_at_a_sessions_start() {
    // 43, C.4: where the host's configuration cannot be read, SessionStart
    // in a session flume did not start gives nothing and exits 0.
    let host = Host::new("unreachable");
    let agent = asks_for(&host, &pane_session(&host, "w1-p2"));
    std::fs::write(host.path("config/flume/config.toml"), "[host\n").expect("the configuration");
    let ran = session_start_in(&host, &agent, "w1-p2", &[]);
    assert_eq!(context(&ran), None, "{}", ran.out);
}

#[test]
fn the_status_line_of_a_code_agent() {
    // In a code agent's session the hook prints the place's line asked
    // alone, one line, the same as status --line, and a session under an
    // identity that is not the stage's live start says so (35, 56, 57,
    // 60; S14; design 5).
    let host = Host::new("status-line");
    let agent = host.launch("fix");
    let line = host.flume(&["status", "fix--one", "--line"], &[], "");
    assert_eq!(line.code, 0, "status --line: {}", line.out);
    assert_eq!(line.out, "fix--one · fix@1 · code · run 1 · starting\n");
    assert_eq!(host.status_line(&agent.env()).out, line.out);

    // Working once a settle reads the started ack and the prompt
    // received.
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    first_prompt(&host, &agent, None);
    assert_eq!(host.flume(&["settle", &agent.place], &[], "").code, 0);
    assert_eq!(
        host.status_line(&agent.env()).out,
        "fix--one · fix@1 · code · run 1 · working\n"
    );

    let mut later = agent.env();
    later[0].1 = agent.identity.replace(":code:1", ":code:2");
    assert_ne!(later[0].1, agent.identity, "another start's identity");
    assert_eq!(
        host.status_line(&later).out,
        "fix--one · fix@1 · code · run 1 · working · this session is not its agent\n"
    );
}

#[test]
fn the_status_line_outside_an_agent_flume_started() {
    // Outside an agent flume started, or for a place the core cannot
    // read, it prints nothing and exits 0 (C.4; design 5).
    let host = Host::new("status-line-outside");
    let agent = host.launch("fix");
    assert_eq!(host.status_line(&[]).out, "");
    let env = agent.env();
    let without = |name: &str| -> Vec<(String, String)> {
        env.iter()
            .filter(|(given, _)| given != name)
            .cloned()
            .collect()
    };
    assert_eq!(host.status_line(&without("FLUME_PLACE")).out, "");
    assert_eq!(host.status_line(&without("FLUME_AGENT")).out, "");
    let mut gone = agent.env();
    gone[2].1 = format!("{}-gone", agent.place);
    assert_eq!(host.status_line(&gone).out, "");
    let mut unspelled = agent.env();
    unspelled[0].1 = "not-an-identity".into();
    assert_eq!(host.status_line(&unspelled).out, "");
}

#[test]
fn the_status_line_writes_nothing() {
    // Ten runs in a code agent's environment leave every file under the
    // scratch state directory as it was, the place's held state at its
    // version among them (57; core.16).
    let host = Host::new("status-line-reads");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let before = host.state();
    for _ in 0..10 {
        assert!(host
            .status_line(&agent.env())
            .out
            .starts_with("fix--one · "));
    }
    assert_eq!(host.state(), before, "status-line wrote flume's state");
}

/// The session the fixtures' inputs name.
const SESSION: &str = "4e3c05c8-51c4-4449-8577-4f7038e394bf";

/// A start that carries the conversation of the stage's start before it,
/// and where its hooks run.
struct Carried {
    agent: Agent,
    /// The resumed session's transcript, in the first account's home.
    transcript: PathBuf,
    /// Its environment as Claude Code gives its hooks, its account's
    /// `CLAUDE_CONFIG_DIR` among it.
    env: Vec<(String, String)>,
}

/// A fill-first pool of two accounts whose configuration homes are
/// scratch folders of the test's own, the second's `projects` a link to
/// the first's: the fix's code agent under work-1 acks in `SESSION`, its
/// transcript in work-1's home, and takes its prompt; its turn fails at
/// the session limit and its session ends; a settle reads it spent, and the stage's start under
/// work-2 resumes that session (146, 179, C.4).
fn carried(host: &Host) -> Carried {
    let first = host.path("home-1");
    let second = host.path("home-2");
    let folder = first.join("projects/-scratch-repository-places-fix--one");
    std::fs::create_dir_all(&folder).expect("the first home's projects");
    std::fs::create_dir_all(&second).expect("the second home");
    std::os::unix::fs::symlink(first.join("projects"), second.join("projects")).expect("the link");
    std::fs::write(
        host.path("config/flume/config.toml"),
        format!(
            "[host]\npool = \"work\"\n\n[pools.work]\nstrategy = \"fill-first\"\naccounts = [\n  {{ name = \"work-1\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?} }} }},\n  {{ name = \"work-2\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?} }} }},\n]\n",
            first.display().to_string(),
            second.display().to_string()
        ),
    )
    .expect("the host's configuration");
    let stopped = host.launch("fix");
    let transcript = folder.join(format!("{SESSION}.jsonl"));
    std::fs::write(&transcript, fixture("session.jsonl")).expect("the transcript");
    let mut env = stopped.env();
    env.push((
        "CLAUDE_CONFIG_DIR".into(),
        first.to_string_lossy().into_owned(),
    ));
    host.hook_in(
        &stopped,
        "session-start",
        json!({ "source": "startup", "transcript_path": transcript }),
        &env,
    );
    assert_eq!(host.agent_status(&stopped.place)["started"], "yes");
    host.hook_in(
        &stopped,
        "user-prompt-submit",
        json!({ "prompt": stopped.prompt, "transcript_path": transcript }),
        &env,
    );
    host.hook_in(
        &stopped,
        "stop-failure",
        json!({
            "error": "rate_limit",
            "last_assistant_message": "You've hit your session limit · resets 5pm (UTC)",
            "transcript_path": transcript,
        }),
        &env,
    );
    host.hook_in(
        &stopped,
        "session-end",
        json!({ "reason": "other", "transcript_path": transcript }),
        &env,
    );
    let settled = host.flume(&["settle", &stopped.place], &[], "");
    assert_eq!(settled.code, 0, "the settle: {}", settled.out);
    let status = host.agent_status(&stopped.place);
    assert_eq!(
        status["conversation"],
        json!({ "from": 1, "carried": true }),
        "{status}"
    );
    let agent = host.agent(&stopped.place);
    assert_ne!(agent.identity, stopped.identity, "{status}");
    let printed = status["printed"].as_str().unwrap_or_default();
    assert!(
        printed.contains(&format!("--resume {SESSION}")),
        "the carried start resumes no session: {printed}"
    );
    let mut env = agent.env();
    env.push((
        "CLAUDE_CONFIG_DIR".into(),
        second.to_string_lossy().into_owned(),
    ));
    Carried {
        agent,
        transcript,
        env,
    }
}

#[test]
fn a_carried_start_in_another_session() {
    // The hook runs for a start whose record names the session it
    // resumes, its input naming another: no started ack is written, the
    // agent's word says it came up in another session and names neither,
    // a settle of the place is started, and the hook exits 0. The record
    // keeps the session the agent runs and its transcript, as every such
    // word does, for reading its own work (161, 179).
    let host = Host::new("carried-elsewhere");
    let carried = carried(&host);
    let other = "9b1f6a2e-0c4d-4e8f-a1b2-c3d4e5f60718";
    let transcript = carried.transcript.with_file_name(format!("{other}.jsonl"));
    std::fs::write(&transcript, fixture("session.jsonl")).expect("its transcript");
    let ran = host.hook_in(
        &carried.agent,
        "session-start",
        json!({ "source": "resume", "session_id": other, "transcript_path": transcript }),
        &carried.env,
    );
    assert_eq!(decision(&ran), None);
    let record = record_of(&host, &carried.agent);
    assert!(record.started().is_none(), "a started ack was written");
    let word = record
        .unsettled()
        .expect("the word that it began otherwise");
    assert!(word.unsettled.session, "{word:?}");
    assert!(
        word.unsettled.missing.is_empty()
            && word.unsettled.different.is_empty()
            && !word.unsettled.program
            && word.unsettled.mode.is_none(),
        "{word:?}"
    );
    let words = word.unsettled.words();
    assert!(
        !words.contains(SESSION) && !words.contains(other),
        "{words}"
    );
    let written =
        std::fs::read_to_string(record.dir().join("unsettled.json")).expect("the word as written");
    assert!(
        !written.contains(SESSION),
        "the resumed session in {written}"
    );
    let status = until(
        &host,
        &carried.agent.place,
        Duration::from_secs(20),
        |status| {
            status["places"][0]["unsettled"]
                .as_array()
                .is_some_and(|read| !read.is_empty())
        },
    );
    let read = &status["places"][0]["unsettled"][0];
    assert_eq!(read["session"], true, "{status}");
    assert_eq!(read["start"], 2, "{status}");
    let read = read.to_string();
    assert!(!read.contains(SESSION) && !read.contains(other), "{read}");
}

#[test]
fn a_start_that_resumes_nothing() {
    // The hook runs for a start whose record names no session: it compares
    // none, and acks on the program and the variables as before, whatever
    // session its input names (161).
    let host = Host::new("resumes-nothing");
    let agent = host.launch("fix");
    host.hook(
        &agent,
        "session-start",
        json!({ "source": "startup", "session_id": "a-session-of-its-own" }),
    );
    let started = record_of(&host, &agent).started().expect("the started ack");
    assert_eq!(started.session_id, "a-session-of-its-own");
    assert!(record_of(&host, &agent).unsettled().is_none());
}

/// Fills the transcript at `path` to `length` bytes with one record of
/// the program's own.
fn fill_to(path: &Path, length: usize) {
    let mut text = std::fs::read_to_string(path).expect("the transcript");
    let pad = length - text.len() - r#"{"type":"system","content":""}"#.len() - 1;
    text.push_str(&format!(
        "{}\n",
        json!({ "type": "system", "content": "x".repeat(pad) })
    ));
    std::fs::write(path, &text).expect("the transcript");
    assert_eq!(text.len(), length);
}

#[test]
fn a_carried_starts_length_kept_at_a_compaction() {
    // The hook acks a carried start whose transcript holds 4,096 bytes,
    // and the ack records 4,096; the start compacts, its transcript grows
    // and SessionStart runs again with `source` `compact`: the ack still
    // records 4,096 (91, 179).
    let host = Host::new("carried-length");
    let carried = carried(&host);
    fill_to(&carried.transcript, 4096);
    let ran = host.hook_in(
        &carried.agent,
        "session-start",
        json!({ "source": "resume", "transcript_path": carried.transcript }),
        &carried.env,
    );
    assert_eq!(decision(&ran), None);
    assert_eq!(host.agent_status(&carried.agent.place)["started"], "yes");
    let record = record_of(&host, &carried.agent);
    let started = record.started().expect("the started ack");
    assert_eq!((started.session_id.as_str(), started.from), (SESSION, 4096));

    let mut text = std::fs::read_to_string(&carried.transcript).expect("the transcript");
    text.push_str(&fixture("session.jsonl"));
    std::fs::write(&carried.transcript, text).expect("the transcript grown");
    host.hook_in(
        &carried.agent,
        "session-start",
        json!({ "source": "compact", "transcript_path": carried.transcript }),
        &carried.env,
    );
    let started = record.started().expect("the started ack");
    assert_eq!(started.source, "compact");
    assert_eq!(started.from, 4096, "{started:?}");
}

/// `flume hook usage` run in `env` as the plugin's module runs it, its
/// input the engine's report `rate_limits`.
fn usage(host: &Host, env: &[(String, String)], rate_limits: Value) -> Ran {
    let env: Vec<(&str, &str)> = env
        .iter()
        .map(|(name, value)| (name.as_str(), value.as_str()))
        .collect();
    let ran = host.flume(
        &["hook", "usage"],
        &env,
        &json!({ "rateLimits": rate_limits }).to_string(),
    );
    assert_eq!(ran.code, 0, "hook usage exits 0: {}", ran.out);
    assert_eq!(ran.out, "", "hook usage printed");
    ran
}

/// work-1's document in the scratch state directory, none where there
/// is none.
fn work_1_document(host: &Host) -> Option<Value> {
    let text = std::fs::read_to_string(host.path("state/flume/accounts/work/work-1.json")).ok()?;
    let read: Value = serde_json::from_str(&text).expect("the document as JSON");
    Some(read["document"].clone())
}

/// A code agent flume launched under work-1, started in the environment
/// its account gives it, and that environment.
fn started_under_work_1(host: &Host) -> (Agent, Vec<(String, String)>) {
    let account = with_an_account(host);
    let agent = host.launch("fix");
    let mut env = agent.env();
    env.push((
        "CLAUDE_CONFIG_DIR".into(),
        account.to_string_lossy().into_owned(),
    ));
    host.hook_in(
        &agent,
        "session-start",
        json!({ "source": "startup" }),
        &env,
    );
    assert_eq!(host.agent_status(&agent.place)["started"], "yes");
    (agent, env)
}

#[test]
fn a_report_of_the_windows_is_the_accounts_reading() {
    // The claude-code-plugin spec's scenario "A report under an account":
    // the hook keeps the report beside the start's record and the core
    // takes it as work-1's reading, from the code agent's identity and
    // the session its started ack names, a use past 100 read as 100,
    // which status gives fresh (184, 185, 211).
    let host = Host::new("usage-account");
    let (agent, env) = started_under_work_1(&host);
    let seventeen = now() + 3 * 3600;
    let before = now();
    usage(
        &host,
        &env,
        json!([
            {"kind": "five_hour", "percentUsed": 42, "resetsAt": flume_seams::rfc3339_of(seventeen)},
            {"kind": "seven_day", "percentUsed": 107.5},
        ]),
    );
    let after = now();
    let document = work_1_document(&host).expect("work-1's document");
    let reading = &document["reading"];
    let taken_at = reading["taken_at"].as_str().expect("taken_at");
    let taken = flume_seams::rfc3339_read(taken_at).expect("RFC 3339");
    assert!((before..=after).contains(&taken), "{document}");
    assert_eq!(reading["agent"], json!(agent.identity), "{document}");
    assert_eq!(reading["session"], json!(SESSION), "{document}");
    assert_eq!(
        reading["windows"],
        json!([
            {"name": "five_hour", "used": 42, "resets_at": flume_seams::rfc3339_of(seventeen)},
            {"name": "seven_day", "used": 100},
        ]),
        "{document}"
    );
    assert!(
        record_of(&host, &agent)
            .dir()
            .join("windows.json")
            .is_file(),
        "the report kept beside the start's record"
    );
    let ran = host.flume(&["status", "--json"], &[], "");
    assert_eq!(ran.code, 0, "status: {}", ran.out);
    let status = ran.json();
    let account = &status["status"]["host"]["pools"][0]["accounts"][0];
    assert_eq!(account["name"], "work-1", "{status}");
    assert_eq!(account["reading"]["fresh"], true, "{status}");
    assert_eq!(
        account["reading"]["agent"],
        json!(agent.identity),
        "{status}"
    );
    assert_eq!(account["reading"]["read_at"], json!(taken), "{status}");
}

#[test]
fn a_window_in_no_form_keeps_nothing() {
    // The claude-code-plugin spec's scenario "A window in no such form":
    // each keeps nothing, work-1's reading stands, and the hook exits 0
    // (184).
    let host = Host::new("usage-no-form");
    let (agent, env) = started_under_work_1(&host);
    usage(
        &host,
        &env,
        json!([{"kind": "five_hour", "percentUsed": 30}]),
    );
    let standing = work_1_document(&host).expect("work-1's document");
    let kept = std::fs::read(record_of(&host, &agent).dir().join("windows.json"))
        .expect("the report kept");
    for window in [
        json!({"percentUsed": 42}),
        json!({"kind": "five_hour", "percentUsed": "42%"}),
        json!({"kind": "five_hour", "percentUsed": 42, "resetsAt": "at five"}),
    ] {
        usage(
            &host,
            &env,
            json!([{"kind": "seven_day", "percentUsed": 18}, window]),
        );
        assert_eq!(
            work_1_document(&host).as_ref(),
            Some(&standing),
            "work-1's reading after {window}"
        );
        assert_eq!(
            std::fs::read(record_of(&host, &agent).dir().join("windows.json"))
                .expect("the report kept"),
            kept,
            "the report kept after {window}"
        );
    }
}

#[test]
fn a_report_under_no_account_writes_no_accounts_document() {
    // The claude-code-plugin spec's scenario "A start under no account":
    // the report is kept beside the start's record, no account's
    // document is written, and the hook exits 0 (184).
    let host = Host::new("usage-no-account");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    usage(
        &host,
        &agent.env(),
        json!([{"kind": "five_hour", "percentUsed": 42}]),
    );
    assert!(
        record_of(&host, &agent)
            .dir()
            .join("windows.json")
            .is_file(),
        "the report kept beside the start's record"
    );
    assert!(
        !host.path("state/flume/accounts").exists(),
        "an account's document written"
    );
}

#[test]
fn hook_usage_outside_a_session_flume_started_writes_nothing() {
    // The claude-code-plugin spec's scenario "Outside a session flume
    // started": with no FLUME_AGENT and no FLUME_SECRET the hook prints
    // nothing, every file under the scratch state directory is as it was,
    // and it exits 0 (C.4).
    let host = Host::new("usage-outside");
    let agent = host.launch("fix");
    host.hook(&agent, "session-start", json!({ "source": "startup" }));
    let before = host.state();
    let env: Vec<(String, String)> = agent
        .env()
        .into_iter()
        .filter(|(name, _)| name != "FLUME_AGENT" && name != "FLUME_SECRET")
        .collect();
    usage(
        &host,
        &env,
        json!([{"kind": "five_hour", "percentUsed": 42}]),
    );
    usage(
        &host,
        &[],
        json!([{"kind": "five_hour", "percentUsed": 42}]),
    );
    assert_eq!(host.state(), before, "hook usage wrote flume's state");
}

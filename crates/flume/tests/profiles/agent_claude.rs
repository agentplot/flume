//! The agent profile for Claude Code (C.4; design 14; record 0016): the
//! agent seam's scenarios against it over fixture transcripts and hook
//! records of Claude Code 2.1.296, the release `devenv.nix` pins; the
//! command for a code stage; the plugin an agent loads, written once per
//! version; a prompt queued while the agent worked; a slash command's
//! prompt; and the folder-trust dialog the command names, against that
//! release's screen. The hooks themselves are driven through the built
//! binary in `hooks_claude.rs`.

use crate::common::*;
use flume::profiles::agent::claude::record::{Asking, Record, Started};
use flume::profiles::agent::claude::{
    collapsed, digest, written, Claude, NAME, PLUGIN, TRUST_ANSWER, TRUST_QUESTION, VERSION,
};
use flume::profiles::{BuiltIn, DEFAULT_PROGRAM};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::agent::{self, AgentHarness};
use flume_seams::*;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The fixtures, as Claude Code 2.1.296 writes them.
pub fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/profiles/fixtures/claude")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// The plugin as the repository holds it.
fn plugin_source() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/claude")
}

/// Appends `lines` to the transcript at `path`.
fn append(path: &Path, lines: &str) {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("the transcript");
    file.write_all(lines.as_bytes())
        .expect("the transcript written");
}

struct Harness {
    scratch: Scratch,
    seam: Claude,
}

impl Harness {
    fn new() -> Harness {
        let scratch = Scratch::new("claude");
        std::fs::create_dir_all(scratch.path("state/places/fix--one")).expect("the folder");
        let seam = Claude::new(&scratch.path("data"), Path::new("/opt/flume/bin/flume"));
        Harness { scratch, seam }
    }

    fn record(&self, identity: &Identity) -> Record {
        Record::at(&self.folder(), identity)
    }

    fn transcript(&self, identity: &Identity) -> PathBuf {
        self.scratch.path(&format!(
            "sessions/{}-{}.jsonl",
            identity.stage, identity.start
        ))
    }
}

impl AgentHarness for Harness {
    type Seam = Claude;

    fn seam(&self) -> &Claude {
        &self.seam
    }

    fn place(&self) -> PlaceRef {
        PlaceRef {
            repository: self.scratch.path("repository").to_string_lossy().into(),
            location: self
                .scratch
                .path("repository/places/fix--one")
                .to_string_lossy()
                .into(),
            branch: "fix/one".into(),
            base: "main".into(),
        }
    }

    fn folder(&self) -> String {
        self.scratch
            .path("state/places/fix--one")
            .to_string_lossy()
            .into()
    }

    /// The session starts and its SessionStart hook acks, as the
    /// plugin's hook records it; its transcript is the fixture's.
    fn start(&self, _command: &AgentCommand, identity: &Identity, folder: &str) {
        let transcript = self.transcript(identity);
        std::fs::create_dir_all(transcript.parent().expect("a folder")).expect("sessions");
        std::fs::write(&transcript, fixture("session.jsonl")).expect("the transcript");
        Record::at(folder, identity)
            .ack_started(&Started {
                session_id: "4e3c05c8-51c4-4449-8577-4f7038e394bf".into(),
                transcript_path: transcript.to_string_lossy().into(),
                source: "startup".into(),
                plugin_version: VERSION.into(),
            })
            .expect("the started ack");
    }

    /// Claude Code submits the text, and the UserPromptSubmit hook acks
    /// its digest.
    fn deliver(&self, identity: &Identity, text: &str) {
        self.record(identity)
            .ack_received(&digest(&collapsed(text)))
            .expect("the received ack");
    }

    fn ask(&self, identity: &Identity) {
        append(&self.transcript(identity), &fixture("question.jsonl"));
        self.record(identity)
            .mark_asking(&Asking {
                tool_use_id: Some("toolu_01AskWhichParser".into()),
                tool_name: "AskUserQuestion".into(),
                tool_input: serde_json::Value::Null,
            })
            .expect("the mark");
    }

    fn start_own_work(&self, identity: &Identity) {
        append(
            &self.transcript(identity),
            &fixture("background-started.jsonl"),
        );
    }

    /// The transcript and the acks become a folder each, which no reading
    /// can read as a file.
    fn hide_record(&self, identity: &Identity) {
        let transcript = self.transcript(identity);
        std::fs::remove_file(&transcript).expect("the transcript removed");
        std::fs::create_dir_all(&transcript).expect("a folder in its place");
        std::fs::create_dir_all(self.record(identity).received_path()).expect("the acks hidden");
    }

    fn try_push(&self, identity: &Identity) {
        self.record(identity)
            .refuse("git push: git push origin HEAD")
            .expect("the refusal");
    }

    fn end_session(&self, identity: &Identity) {
        self.record(identity)
            .mark_ended("prompt_input_exit")
            .expect("the mark");
    }

    fn say_last(&self, identity: &Identity, words: &str) {
        let line = serde_json::json!({
            "type": "assistant",
            "isSidechain": false,
            "message": {"role": "assistant", "type": "message", "content": [{"type": "text", "text": words}]},
            "version": "2.1.296",
        });
        append(&self.transcript(identity), &format!("{line}\n"));
    }
}

#[test]
fn the_profile_passes_the_agent_scenarios() {
    pass_all("agent", &agent::scenarios(), Harness::new);
}

fn identity(stage: &str) -> Identity {
    Identity {
        repository: "/repository".into(),
        place: "fix--one".into(),
        stage: stage.into(),
        start: 1,
    }
}

#[test]
fn the_command_for_a_code_stage() {
    // The command for a code stage: `claude` with the plugin, the model
    // and the instructions, the four FLUME_* words over the settings'
    // environment, and nothing of the caller's session (29, 32, 88, C.4).
    let harness = Harness::new();
    let settings = Settings {
        model: Some("opus".into()),
        instructions: Some("Keep each commit small.".into()),
        // A `PATH` holding no `claude`, so no wrapper of the host's is
        // read for the program named by its name.
        env: BTreeMap::from([
            ("CLAUDE_CONFIG_DIR".into(), "/home/me/.claude-work".into()),
            ("PATH".into(), "/nowhere".into()),
        ]),
        ..Settings::default()
    };
    let who = identity("code");
    let command = harness.seam.command(
        &settings,
        &harness.place(),
        &who,
        &Secret("s3cret".into()),
        "fix the parser",
    );
    assert_eq!(command.program, NAME);
    let plugin = harness.seam.plugin_dir().to_string_lossy().into_owned();
    assert_eq!(
        command.args,
        vec![
            "--plugin-dir".to_string(),
            plugin,
            "--model".into(),
            "opus".into(),
            "--append-system-prompt".into(),
            "Keep each commit small.".into(),
        ]
    );
    assert_eq!(
        command.env,
        BTreeMap::from([
            (
                "CLAUDE_CONFIG_DIR".to_string(),
                "/home/me/.claude-work".to_string()
            ),
            ("PATH".into(), "/nowhere".into()),
            ("FLUME_AGENT".into(), who.to_string()),
            ("FLUME_SECRET".into(), "s3cret".into()),
            ("FLUME_PLACE".into(), harness.place().location),
            ("FLUME_STAGE".into(), "code".into()),
        ]),
        "the environment holds the settings' and flume's words alone"
    );
    assert_eq!(command.prompt_args, vec!["fix the parser".to_string()]);

    // A program the settings name by its path runs as named.
    let by_path = Settings {
        program: Some("/opt/claude/bin/claude".into()),
        ..Settings::default()
    };
    let command = harness
        .seam
        .command(&by_path, &harness.place(), &who, &Secret("s".into()), "x");
    assert_eq!(command.program, "/opt/claude/bin/claude");
}

#[test]
fn the_resolver_binds_the_program_by_name_or_path() {
    let resolver = BuiltIn::new();
    assert!(resolver.agent(DEFAULT_PROGRAM).is_some());
    assert!(resolver.agent("/opt/claude/bin/claude").is_some());
    assert!(resolver.agent("codex").is_none());
}

#[test]
fn the_plugin_an_agent_loads() {
    // Two commands in one version name one plugin directory, written once,
    // whose files are plugins/claude's, each command naming `flume` named
    // by the binary's absolute path (design 14, 17).
    let harness = Harness::new();
    let place = harness.place();
    let who = identity("code");
    let first = harness
        .seam
        .command(&Settings::default(), &place, &who, &Secret("a".into()), "x");
    let dir = PathBuf::from(&first.args[1]);
    let manifest = dir.join(".claude-plugin/plugin.json");
    let written_at = std::fs::metadata(&manifest)
        .and_then(|meta| meta.modified())
        .expect("the plugin written");
    std::thread::sleep(std::time::Duration::from_millis(20));
    let second = harness.seam.command(
        &Settings::default(),
        &place,
        &identity("verify"),
        &Secret("b".into()),
        "y",
    );
    assert_eq!(first.args[1], second.args[1], "two plugin directories");
    assert_eq!(
        std::fs::metadata(&manifest)
            .and_then(|meta| meta.modified())
            .expect("the plugin"),
        written_at,
        "the plugin was written again"
    );
    assert!(
        dir.starts_with(harness.scratch.path("data/flume/plugins/claude")),
        "{}",
        dir.display()
    );
    let binary = Path::new("/opt/flume/bin/flume");
    for (path, text) in PLUGIN {
        let source = std::fs::read_to_string(plugin_source().join(path)).expect(path);
        assert_eq!(text, source, "{path} is not the repository's");
        let on_disk = std::fs::read_to_string(dir.join(path)).expect(path);
        assert_eq!(on_disk, written(&source, binary), "{path} as written");
    }
    let hooks = std::fs::read_to_string(dir.join("hooks/hooks.json")).expect("the hooks");
    assert!(
        hooks.contains("\"command\": \"/opt/flume/bin/flume\""),
        "{hooks}"
    );
    assert!(!hooks.contains("\"command\": \"flume\""), "{hooks}");
    // The manifest carries the binary's version, a release's where the
    // repository's says the workspace's, which the started ack reports
    // back (135).
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest).expect("the manifest"))
            .expect("the manifest as JSON");
    assert_eq!(manifest["version"], flume::VERSION, "{manifest}");

    // Every file of the plugin is one the binary writes: only the
    // project's own files are left out.
    let mut files = vec![];
    let mut todo = vec![plugin_source()];
    while let Some(dir) = todo.pop() {
        for entry in std::fs::read_dir(&dir)
            .expect("the plugin's files")
            .flatten()
        {
            let path = entry.path();
            let relative = path
                .strip_prefix(plugin_source())
                .expect("under the plugin")
                .to_string_lossy()
                .into_owned();
            if relative == "moon.yml" || relative == "tests" {
                continue;
            }
            if path.is_dir() {
                todo.push(path);
            } else {
                files.push(relative);
            }
        }
    }
    files.sort();
    let mut embedded: Vec<String> = PLUGIN.iter().map(|(path, _)| path.to_string()).collect();
    embedded.sort();
    assert_eq!(files, embedded);
}

#[test]
fn the_plugin_carries_flumes_version() {
    let manifest: serde_json::Value =
        serde_json::from_str(PLUGIN[0].1).expect("the plugin's manifest");
    assert_eq!(manifest["version"], VERSION);
    assert_eq!(manifest["name"], "flume");
}

#[test]
fn a_prompt_queued_while_the_agent_worked() {
    // Typed while the agent works, the prompt is queued: the transcript
    // holds its enqueue record, and received stays no until Claude Code
    // submits it and the UserPromptSubmit hook acks it (38, 89).
    let harness = Harness::new();
    let who = identity("code");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    append(&harness.transcript(&who), &fixture("queued.jsonl"));
    let text = "the reviewer asks for one more test";
    assert!(
        matches!(
            harness.seam.received(&folder, &who, text),
            Received::NotReceived { .. }
        ),
        "received before it was submitted"
    );
    harness.deliver(&who, "the reviewer  asks for\none more test\n");
    assert!(matches!(
        harness.seam.received(&folder, &who, text),
        Received::Received { .. }
    ));
}

#[test]
fn a_slash_command_prompt() {
    // A slash command Claude Code runs itself fires no UserPromptSubmit:
    // its transcript's turn, by its command's name and arguments,
    // whitespace collapsed, says it was received; a transcript not yet
    // written says not (38, 89).
    let harness = Harness::new();
    let who = identity("code");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    let prompt = "/compact keep the parser's notes";
    std::fs::remove_file(harness.transcript(&who)).expect("the transcript");
    assert!(matches!(
        harness.seam.received(&folder, &who, prompt),
        Received::NotReceived { .. }
    ));
    append(&harness.transcript(&who), &fixture("session.jsonl"));
    assert!(matches!(
        harness.seam.received(&folder, &who, prompt),
        Received::NotReceived { .. }
    ));
    append(&harness.transcript(&who), &fixture("slash-command.jsonl"));
    assert!(matches!(
        harness.seam.received(&folder, &who, prompt),
        Received::Received { .. }
    ));
    assert!(matches!(
        harness.seam.received(&folder, &who, "/compact other words"),
        Received::NotReceived { .. }
    ));
}

#[test]
fn own_work_ends_with_its_tasks_notification() {
    let harness = Harness::new();
    let who = identity("code");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    assert_eq!(harness.seam.own_work(&folder, &who), Reading::No);
    append(
        &harness.transcript(&who),
        &fixture("background-started.jsonl"),
    );
    assert_eq!(harness.seam.own_work(&folder, &who), Reading::Yes);
    append(
        &harness.transcript(&who),
        &fixture("background-finished.jsonl"),
    );
    assert_eq!(harness.seam.own_work(&folder, &who), Reading::No);
}

#[test]
fn a_question_answered_in_the_transcript_is_no_longer_asked() {
    // The mark stops counting once the transcript holds the call's
    // answer, though no hook cleared it (90).
    let harness = Harness::new();
    let who = identity("code");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    harness.ask(&who);
    assert_eq!(harness.seam.asking(&folder, &who), Reading::Yes);
    append(
        &harness.transcript(&who),
        &fixture("question-answered.jsonl"),
    );
    assert_eq!(harness.seam.asking(&folder, &who), Reading::No);
}

#[test]
fn the_trust_screen() {
    // The command names the folder-trust dialog of Claude Code 2.1.296 by
    // a line its screen shows, and the keys that answer it yes: the
    // dialog lists "No, exit" first and focused, so the answer moves down
    // to the trusting choice and confirms (34; record 0018).
    let harness = Harness::new();
    let command = harness.seam.command(
        &Settings::default(),
        &harness.place(),
        &identity("code"),
        &Secret("s".into()),
        "x",
    );
    let trust = command.folder_trust.expect("the folder-trust question");
    assert_eq!(trust.question, TRUST_QUESTION);
    assert_eq!(trust.answer, TRUST_ANSWER);
    let screen = fixture("trust-screen-2.1.296.txt");
    assert!(screen.contains(&trust.question), "{screen}");
    let no = screen.find("❯ No, exit").expect("the focused refusal");
    let yes = screen.find(&trust.question).expect("the trusting choice");
    assert!(no < yes, "the trusting choice is not below the focused one");
    assert_eq!(
        trust.answer.split(' ').collect::<Vec<_>>(),
        ["down", "enter"]
    );
    assert!(!screen.contains("Bypass Permissions"));
}

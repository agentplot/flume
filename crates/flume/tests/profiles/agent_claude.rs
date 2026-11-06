//! The agent profile for Claude Code (C.4; design 14; record 0016): the
//! agent seam's scenarios against it over fixture transcripts and hook
//! records of Claude Code 2.1.296, the release `devenv.nix` pins; the
//! command for a code stage, its instructions in a file of the start's
//! own, and for a stage in a mode; the map of
//! flume's modes and the release's own, and the mode read back; the
//! plugin an agent loads, written once per version; a prompt queued while
//! the agent worked; a slash command's
//! prompt; the folder-trust dialog the command names, against that
//! release's screen; the carry of a stopped start's conversation over
//! scratch configuration homes, one whose `projects` is a link to
//! another's and one of its own, and the command that resumes it; each reading of a carried start past
//! the length its started ack recorded; and live, over scratch homes'
//! session records and processes the tests start. The hooks themselves are driven through the built binary in
//! `hooks_claude.rs`.

use crate::common::*;
use flume::profiles::agent::claude::record::{self, Asking, Failure, Record, Started};
use flume::profiles::agent::claude::{
    as_received, digest, written, Claude, NAME, PLUGIN, STANDING_PROMPT_FLAG, TRUST_CONFIRM,
    TRUST_FOCUS, TRUST_NEXT, TRUST_QUESTION, VERSION,
};
use flume::profiles::agent::claude::{carry, live, mode, transcript};
use flume::profiles::{BuiltIn, DEFAULT_PROGRAM};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::agent::{self, AgentHarness, Modes};
use flume_seams::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

/// The fixtures, as Claude Code 2.1.296 writes them.
pub fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/profiles/fixtures/claude")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// The session every fixture's transcript names.
const SESSION: &str = "4e3c05c8-51c4-4449-8577-4f7038e394bf";

/// The folder under a home's `projects` the harness's transcripts are in,
/// one as 2.1.296 names a place's: the profile reads the names a started
/// ack gives, and computes none.
const PROJECT: &str = "-repository-places-fix--one";

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
    processes: Processes,
}

/// The processes a harness started, ended when it is dropped.
#[derive(Default)]
struct Processes(RefCell<Vec<Child>>);

impl Drop for Processes {
    fn drop(&mut self) {
        for mut child in self.0.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

/// A session no process of the host but the harness's runs: the test
/// process's id and a count.
fn new_session() -> String {
    static SESSIONS: AtomicUsize = AtomicUsize::new(0);
    format!(
        "live-{}-{}",
        std::process::id(),
        SESSIONS.fetch_add(1, Ordering::Relaxed)
    )
}

/// When the process `pid` started, as 2.1.296 records it in `procStart`:
/// `ps -o lstart=` under `LC_ALL=C` and `TZ=UTC`, trimmed.
fn proc_start(pid: u32) -> String {
    let output = Command::new("ps")
        .args(["-o", "lstart=", "-p", &pid.to_string()])
        .env("LC_ALL", "C")
        .env("TZ", "UTC")
        .output()
        .expect("ps");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// The process `pid` as live names it.
fn live_process(pid: u32) -> LiveProcess {
    LiveProcess {
        pid,
        started: live::started_at(&proc_start(pid)).expect("the process's start"),
    }
}

impl Harness {
    fn new() -> Harness {
        let scratch = Scratch::new("claude");
        std::fs::create_dir_all(scratch.path("state/places/fix--one")).expect("the folder");
        let seam = Claude::new(&scratch.path("data"), Path::new("/opt/flume/bin/flume"));
        Harness {
            scratch,
            seam,
            processes: Processes::default(),
        }
    }

    /// A process the harness runs until it is dropped, `sleep`.
    fn run_process(&self) -> u32 {
        self.keep(
            Command::new("sleep")
                .arg("600")
                .stdin(Stdio::null())
                .spawn()
                .expect("sleep"),
        )
    }

    /// A process the harness runs until it is dropped whose arguments are
    /// those of `claude --resume <session>`, as the host lists them: a
    /// shell named `claude` waiting on its input.
    fn run_resume(&self, session: &str) -> u32 {
        self.keep(
            Command::new("/bin/sh")
                .arg0("claude")
                .args(["-c", "read line", "claude", "--resume", session])
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .spawn()
                .expect("sh"),
        )
    }

    fn keep(&self, child: Child) -> u32 {
        let pid = child.id();
        self.processes.0.borrow_mut().push(child);
        pid
    }

    /// Ends the harness's process `pid` and waits for it.
    fn end_process(&self, pid: u32) {
        let mut processes = self.processes.0.borrow_mut();
        let at = processes
            .iter()
            .position(|child| child.id() == pid)
            .expect("a process of the harness");
        let mut child = processes.remove(at);
        child.kill().expect("the process ended");
        child.wait().expect("the process waited on");
    }

    /// `<home>/sessions/<pid>.json` as 2.1.296 writes it for the process
    /// `pid` running `session`, its `procStart` as given, none left out.
    fn session_record(
        &self,
        home: &Path,
        pid: u32,
        session: &str,
        started: Option<&str>,
    ) -> PathBuf {
        let mut record = serde_json::json!({
            "pid": pid,
            "sessionId": session,
            "cwd": self.place().location,
            "startedAt": 1_793_279_400_000u64,
            "kind": "interactive",
            "entrypoint": "cli",
        });
        if let Some(started) = started {
            record["procStart"] = started.into();
        }
        let path = home.join(format!("sessions/{pid}.json"));
        std::fs::create_dir_all(path.parent().expect("a folder")).expect("sessions");
        std::fs::write(&path, record.to_string()).expect("the record");
        path
    }

    /// The start's started ack, written again naming `session`.
    fn name_session(&self, identity: &Identity, session: &str) {
        let record = self.record(identity);
        let mut started = record.started().expect("a started ack");
        started.session_id = session.into();
        record.ack_started(&started).expect("the started ack");
    }

    /// A configuration home whose `sessions` stands and cannot be listed:
    /// a file in its place, which no user, the superuser among them, can
    /// list.
    fn unlistable(&self, name: &str) -> PathBuf {
        let home = self.home(name);
        std::fs::create_dir_all(&home).expect("the home");
        std::fs::write(home.join(live::SESSIONS), "").expect("the file");
        home
    }

    fn record(&self, identity: &Identity) -> Record {
        Record::at(&self.folder(), identity)
    }

    /// The start's transcript, in the configuration home of `work-1`, the
    /// account every start of the harness runs under.
    fn transcript(&self, identity: &Identity) -> PathBuf {
        self.home("work-1").join(format!(
            "projects/{PROJECT}/{}-{}.jsonl",
            identity.stage, identity.start
        ))
    }

    /// The scratch configuration home of the account `name`.
    fn home(&self, name: &str) -> PathBuf {
        self.scratch.path(&format!("homes/{name}"))
    }

    /// The configuration home of `name`, its `projects` a symbolic link to
    /// `work-1`'s, as a host shares one between its accounts.
    fn sharing(&self, name: &str) -> PathBuf {
        let home = self.home(name);
        if !home.join("projects").exists() {
            std::fs::create_dir_all(&home).expect("the home");
            std::os::unix::fs::symlink(self.home("work-1").join("projects"), home.join("projects"))
                .expect("the link");
        }
        home
    }

    /// The configuration home of `name`, its `projects` a folder of its own
    /// holding a copy of `earlier`'s transcript under the same names.
    fn apart(&self, name: &str, earlier: &Identity) -> PathBuf {
        let home = self.home(name);
        let copy = home.join(format!(
            "projects/{PROJECT}/{}",
            self.transcript(earlier)
                .file_name()
                .expect("a file")
                .to_string_lossy()
        ));
        std::fs::create_dir_all(copy.parent().expect("a folder")).expect("the home");
        std::fs::copy(self.transcript(earlier), &copy).expect("the copy");
        home
    }

    /// The carried start after `earlier`, its started ack naming
    /// `earlier`'s session and transcript, as `--resume` goes on in them,
    /// and the transcript's length now.
    fn carried(&self, earlier: &Identity) -> Identity {
        let next = Identity {
            start: earlier.start + 1,
            ..earlier.clone()
        };
        let path = self.transcript(earlier);
        Record::at(&self.folder(), &next)
            .ack_started(&Started {
                session_id: SESSION.into(),
                transcript_path: path.to_string_lossy().into(),
                source: "resume".into(),
                plugin_version: VERSION.into(),
                from: transcript::length(&path),
            })
            .expect("the started ack");
        next
    }
}

/// The environment of an account whose configuration home is `home`.
fn at_home(home: &Path) -> BTreeMap<String, String> {
    BTreeMap::from([(
        carry::CONFIG_DIR.to_string(),
        home.to_string_lossy().into_owned(),
    )])
}

/// Every entry under `dir`, by its path: a file's bytes, a link's target,
/// and a folder as itself.
fn tree(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut found = vec![];
    let mut todo = vec![dir.to_path_buf()];
    while let Some(dir) = todo.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let path = entry.path();
            let meta = std::fs::symlink_metadata(&path).expect("the entry");
            let held = if meta.is_symlink() {
                std::fs::read_link(&path)
                    .expect("the link")
                    .to_string_lossy()
                    .into_owned()
                    .into_bytes()
            } else if meta.is_dir() {
                todo.push(path.clone());
                b"folder".to_vec()
            } else {
                std::fs::read(&path).expect("the file")
            };
            found.push((path, held));
        }
    }
    found.sort();
    found
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
                session_id: SESSION.into(),
                transcript_path: transcript.to_string_lossy().into(),
                source: "startup".into(),
                plugin_version: VERSION.into(),
                from: 0,
            })
            .expect("the started ack");
    }

    /// Claude Code submits the text, and the UserPromptSubmit hook acks
    /// its digest.
    fn deliver(&self, identity: &Identity, text: &str) {
        self.record(identity)
            .ack_received(&digest(&as_received(text)))
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

    /// The StopFailure hook recorded `rate_limit` and a session limit's
    /// words, written as 2.1.296 builds them, at 13:10 UTC on 29 October
    /// 2026; the reset they name is 15:00 that day.
    fn hit_usage_limit(&self, identity: &Identity) -> Seconds {
        self.record(identity)
            .set_failure(&Failure {
                error: "rate_limit".into(),
                words: "You've hit your session limit \u{b7} resets 3pm (UTC)".into(),
                at: 1_793_279_400,
            })
            .expect("the failure");
        1_793_286_000
    }

    /// The SessionStart hook found the variable missing and wrote its
    /// word in place of the started ack, as the plugin's hook records it.
    fn begin_without(&self, identity: &Identity, variable: &str) {
        self.record(identity)
            .set_unsettled(&record::Unsettled {
                unsettled: Unsettled {
                    missing: vec![variable.to_string()],
                    ..Unsettled::default()
                },
                session_id: SESSION.into(),
                transcript_path: self.transcript(identity).to_string_lossy().into(),
                from: 0,
            })
            .expect("the word");
    }

    /// The four as the profile maps them, and 2.1.296's own six.
    fn modes(&self) -> Modes {
        Modes {
            four: BTreeMap::from([
                (Mode::Plan, "plan".into()),
                (Mode::Ask, "manual".into()),
                (Mode::Edits, "acceptEdits".into()),
                (Mode::Full, "bypassPermissions".into()),
            ]),
            own: [
                "acceptEdits",
                "auto",
                "bypassPermissions",
                "manual",
                "dontAsk",
                "plan",
            ]
            .map(String::from)
            .to_vec(),
        }
    }

    /// The SessionStart hook found the session begun in another mode and
    /// wrote its word in place of the started ack, both modes named.
    fn begin_in(&self, identity: &Identity, asked: &str, began: &str) {
        self.record(identity)
            .set_unsettled(&record::Unsettled {
                unsettled: Unsettled {
                    mode: Some(ModeBegan {
                        asked: asked.to_string(),
                        began: Some(began.to_string()),
                    }),
                    ..Unsettled::default()
                },
                session_id: SESSION.into(),
                transcript_path: self.transcript(identity).to_string_lossy().into(),
                from: 0,
            })
            .expect("the word");
    }

    /// `runs`' session named by a record in work-1's home, for a process
    /// the harness runs started at its `procStart`, and resumed by a
    /// `claude` under no home given; `unknown`'s read under a home whose
    /// `sessions` cannot be listed (207, C.4).
    fn live_sessions(&self, runs: &Identity, unknown: &Identity) -> Option<agent::LiveSessions> {
        let session = new_session();
        self.name_session(runs, &session);
        self.name_session(unknown, &new_session());
        let recorded = self.run_process();
        self.session_record(
            &self.home("work-1"),
            recorded,
            &session,
            Some(&proc_start(recorded)),
        );
        let resumed = self.run_resume(&session);
        let mut processes = vec![live_process(recorded), live_process(resumed)];
        processes.sort_by_key(|process| process.pid);
        Some(agent::LiveSessions {
            envs: vec![
                at_home(&self.home("work-1")),
                at_home(&self.sharing("work-2")),
            ],
            processes,
            unreadable: vec![
                at_home(&self.home("work-1")),
                at_home(&self.unlistable("work-9")),
            ],
        })
    }

    /// `work-2`, whose `projects` is a link to `work-1`'s, where `earlier`
    /// ran, and `work-3`, whose `projects` is a folder of its own holding a
    /// copy of its transcript (179, C.4).
    fn carry_accounts(&self, earlier: &Identity) -> Option<agent::CarryAccounts> {
        Some(agent::CarryAccounts {
            reaches: at_home(&self.sharing("work-2")),
            session: SESSION.into(),
            reaches_none: at_home(&self.apart("work-3", earlier)),
        })
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
    // and the instructions, in a file of the start's own beside its record
    // holding them as given, newlines and all, which no argument may hold
    // under herdr (148, C.5), the four FLUME_* words over its account's
    // environment, no `--permission-mode` where the settings give no mode,
    // and nothing of the caller's session (29, 32, 88, 144, 162, C.4).
    let harness = Harness::new();
    let instructions = "Keep each commit small.\n\n- **Before you finish.** List each decision.\n";
    let settings = Settings {
        model: Some("opus".into()),
        instructions: Some(instructions.into()),
        ..Settings::default()
    };
    // A `PATH` holding no `claude`, so no wrapper of the host's is read
    // for the program named by its name.
    let env = BTreeMap::from([
        ("CLAUDE_CONFIG_DIR".into(), "/home/me/.claude-work".into()),
        ("PATH".into(), "/nowhere".into()),
    ]);
    let who = identity("code");
    let command = harness.seam.command(
        &harness.folder(),
        &settings,
        None,
        None,
        &env,
        &harness.place(),
        &who,
        &Secret("s3cret".into()),
        "fix the parser",
    );
    assert_eq!(command.program, NAME);
    let plugin = harness.seam.plugin_dir().to_string_lossy().into_owned();
    let file = harness.record(&who).standing_prompt();
    assert_eq!(
        command.args,
        vec![
            "--plugin-dir".to_string(),
            plugin,
            "--model".into(),
            "opus".into(),
            STANDING_PROMPT_FLAG.into(),
            file.to_string_lossy().into_owned(),
        ]
    );
    assert_eq!(
        std::fs::read_to_string(&file).expect("the standing prompt's file"),
        instructions,
        "the instructions as the file holds them"
    );
    assert!(
        command
            .args
            .iter()
            .all(|arg| !arg.chars().any(char::is_control)),
        "an argument holds a control character: {:?}",
        command.args
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
        "the environment holds the account's and flume's words alone"
    );
    assert_eq!(command.prompt_args, vec!["fix the parser".to_string()]);
    assert!(
        !command.args.iter().any(|arg| arg == "--permission-mode"),
        "a mode passed where the settings give none"
    );

    // A program the settings name by its path runs as named.
    let by_path = Settings {
        program: Some("/opt/claude/bin/claude".into()),
        ..Settings::default()
    };
    let command = harness.seam.command(
        &harness.folder(),
        &by_path,
        None,
        None,
        &env,
        &harness.place(),
        &who,
        &Secret("s".into()),
        "x",
    );
    assert_eq!(command.program, "/opt/claude/bin/claude");
}

#[test]
fn the_command_passes_the_mode_it_is_given() {
    // The value the claim's resolution mapped goes to
    // `--permission-mode`, after the model; none is passed where no mode
    // is given (162, 169).
    let harness = Harness::new();
    let settings = Settings {
        model: Some("opus".into()),
        ..Settings::default()
    };
    let command = |mode: Option<&str>| {
        harness.seam.command(
            &harness.folder(),
            &settings,
            mode,
            None,
            &BTreeMap::new(),
            &harness.place(),
            &identity("code"),
            &Secret("s".into()),
            "x",
        )
    };
    assert_eq!(
        command(Some("acceptEdits")).args[2..],
        ["--model", "opus", "--permission-mode", "acceptEdits"]
    );
    assert!(
        !command(None)
            .args
            .iter()
            .any(|arg| arg == "--permission-mode"),
        "a mode passed where none was given"
    );
}

#[test]
fn the_command_for_a_stage_in_edits() {
    // Settings giving `edits` start `claude` with `--permission-mode
    // acceptEdits`: the entry the program's name takes, mapped by the
    // profile, passed by its command (S66, 169).
    let harness = Harness::new();
    let settings = Settings {
        mode: Some(ModeSetting::from_word("edits").expect("edits")),
        ..Settings::default()
    };
    let entry = settings
        .mode
        .as_ref()
        .and_then(|mode| mode.entry(NAME))
        .expect("the entry for claude");
    let value = harness.seam.mode(&entry).expect("edits mapped");
    // A `PATH` holding no `claude`, so the program runs by its name.
    let command = harness.seam.command(
        &harness.folder(),
        &settings,
        Some(&value),
        None,
        &BTreeMap::from([("PATH".into(), "/nowhere".into())]),
        &harness.place(),
        &identity("code"),
        &Secret("s".into()),
        "x",
    );
    assert_eq!(command.program, NAME);
    assert_eq!(command.args[2..], ["--permission-mode", "acceptEdits"]);
}

#[test]
fn each_of_the_four() {
    // Each of flume's four maps to the one choice of 2.1.296's that means
    // it (169, C.4).
    let seam = Harness::new().seam;
    let mapped: Vec<String> = Mode::ALL
        .iter()
        .map(|mode| seam.mode(&ModeEntry::Flume(*mode)).expect("mapped"))
        .collect();
    assert_eq!(
        mapped,
        ["plan", "manual", "acceptEdits", "bypassPermissions"]
    );
}

#[test]
fn its_own_value_under_its_name() {
    // `{ claude = "auto" }`'s entry passes `auto` as written, and `{ claude
    // = "yolo" }`'s is refused naming `yolo`, `claude` and the release's
    // six values (169, C.4).
    let seam = Harness::new().seam;
    let entry = |value: &str| {
        ModeSetting::from_programs(BTreeMap::from([(NAME.to_string(), value.to_string())]))
            .expect("a table")
            .entry(NAME)
            .expect("claude's entry")
    };
    assert_eq!(seam.mode(&entry("auto")), Ok("auto".to_string()));
    let words = seam.mode(&entry("yolo")).expect_err("yolo taken");
    for named in [
        "`yolo`",
        "`claude`",
        "acceptEdits",
        "auto",
        "bypassPermissions",
        "manual",
        "dontAsk",
        "plan",
    ] {
        assert!(words.contains(named), "{named} not named: {words}");
    }
}

#[test]
fn the_mode_read_back() {
    // 2.1.296 reports `--permission-mode manual` as `default`: the profile
    // reads it back as `manual`, and every other choice as itself (161,
    // C.4).
    assert_eq!(Harness::new().seam.mode_read_back(), Offered::Offered);
    assert_eq!(
        ["default", "acceptEdits", "plan"].map(mode::began),
        ["manual", "acceptEdits", "plan"]
    );
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
    let first = harness.seam.command(
        &harness.folder(),
        &Settings::default(),
        None,
        None,
        &BTreeMap::new(),
        &place,
        &who,
        &Secret("a".into()),
        "x",
    );
    let dir = PathBuf::from(&first.args[1]);
    let manifest = dir.join(".claude-plugin/plugin.json");
    let written_at = std::fs::metadata(&manifest)
        .and_then(|meta| meta.modified())
        .expect("the plugin written");
    std::thread::sleep(std::time::Duration::from_millis(20));
    let second = harness.seam.command(
        &harness.folder(),
        &Settings::default(),
        None,
        None,
        &BTreeMap::new(),
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
    // The module runs the same binary by its path (design 4).
    let module = std::fs::read_to_string(dir.join("hooks/line.ts")).expect("the module");
    assert!(
        module.contains("const FLUME = \"/opt/flume/bin/flume\";"),
        "{module}"
    );
    assert!(!module.contains("const FLUME = \"flume\";"), "{module}");
    // The manifest carries the binary's version, a release's where the
    // repository's says the workspace's, which the started ack reports
    // back (135).
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest).expect("the manifest"))
            .expect("the manifest as JSON");
    assert_eq!(manifest["version"], flume::VERSION, "{manifest}");

    // Every file of the plugin is one the binary writes: only the
    // project's own files are left out, its tasks, its tests, the engine
    // types it is typed against and its TypeScript configuration, and the
    // types a session of Claude Code lays beside a plugin it loads.
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
            if [
                "moon.yml",
                "tests",
                "engine-types",
                "tsconfig.json",
                ".claude-plugin/types",
            ]
            .contains(&relative.as_str())
            {
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
fn a_prompt_inside_paste_tags_is_the_prompt_sent() {
    // Claude Code 2.1.296 may submit a pasted prompt inside paste tags
    // carrying a four-hex-digit id: the prompt is compared with them taken
    // off, and text that only looks like a tag stays (89; record 0016).
    let sent = "Fix the parser.\n\nCommit it.";
    let submitted = "\n\n<pasted_content id=\"fad0\">\nFix the parser.\n\nCommit it.\n</pasted_content id=\"fad0\">\n";
    assert_eq!(as_received(submitted), as_received(sent));
    assert_eq!(as_received(sent), "Fix the parser. Commit it.");
    assert_eq!(
        as_received("see <pasted_content id=\"FAD0\"> and <b>"),
        "see <pasted_content id=\"FAD0\"> and <b>"
    );
    assert_eq!(
        as_received("words <pasted_content id=\"0a1b\">pasted</pasted_content id=\"0a1b\">"),
        "words pasted"
    );

    let harness = Harness::new();
    let who = identity("code");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    harness.deliver(&who, submitted);
    assert!(matches!(
        harness.seam.received(&folder, &who, sent),
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
    // a line its screen shows, the mark of the choice in focus and the
    // keys that move and confirm it: the dialog lists "No, exit" first
    // and focused, with or without the warning that the folder's settings
    // pre-approve tools, so the terminal moves the focus down to the
    // trusting choice before it confirms (34; record 0018).
    let harness = Harness::new();
    let command = harness.seam.command(
        &harness.folder(),
        &Settings::default(),
        None,
        None,
        &BTreeMap::new(),
        &harness.place(),
        &identity("code"),
        &Secret("s".into()),
        "x",
    );
    let trust = command.folder_trust.expect("the folder-trust question");
    assert_eq!(trust.question, TRUST_QUESTION);
    assert_eq!(trust.focus, TRUST_FOCUS);
    assert_eq!(trust.next, TRUST_NEXT);
    assert_eq!(trust.confirm, TRUST_CONFIRM);
    for name in [
        "trust-screen-2.1.296.txt",
        "trust-screen-settings-2.1.296.txt",
    ] {
        let screen = fixture(name);
        let yes = screen
            .lines()
            .find(|line| line.contains(&trust.question))
            .unwrap_or_else(|| panic!("no trusting choice in {name}: {screen}"));
        assert!(
            !yes.contains(&trust.focus),
            "the trusting choice is in focus at first in {name}"
        );
        let no = screen
            .lines()
            .position(|line| line.contains(&trust.focus) && line.contains("No, exit"))
            .unwrap_or_else(|| panic!("no focused refusal in {name}"));
        let at = screen
            .lines()
            .position(|line| line.contains(&trust.question))
            .expect("the trusting choice");
        assert!(
            no < at,
            "the trusting choice is not below the focused one in {name}"
        );
        assert!(!screen.contains("Bypass Permissions"));
    }
}

#[test]
fn the_command_of_a_carried_start() {
    // The command for a start resuming the session `s-1` holds `--resume
    // s-1` beside `--plugin-dir`, and one resuming none holds no
    // `--resume` (88, 179).
    let harness = Harness::new();
    let command = |resume: Option<&str>| {
        harness.seam.command(
            &harness.folder(),
            &Settings {
                model: Some("opus".into()),
                ..Settings::default()
            },
            None,
            resume,
            &BTreeMap::new(),
            &harness.place(),
            &Identity {
                start: 2,
                ..identity("code")
            },
            &Secret("s".into()),
            "x",
        )
    };
    let plugin = harness.seam.plugin_dir().to_string_lossy().into_owned();
    assert_eq!(
        command(Some("s-1")).args,
        [
            "--plugin-dir",
            &plugin,
            "--resume",
            "s-1",
            "--model",
            "opus"
        ]
    );
    assert_eq!(
        command(None).args,
        ["--plugin-dir", &plugin, "--model", "opus"]
    );
}

/// The words of a carry's no, which name `CLAUDE_CONFIG_DIR` and no path
/// of the harness's (core.30).
fn no_words(harness: &Harness, answer: Carry) -> String {
    let Carry::No { why } = answer else {
        panic!("carry answered {answer:?}");
    };
    assert!(why.contains("`CLAUDE_CONFIG_DIR`"), "{why}");
    let scratch = harness.scratch.path("");
    assert!(
        !why.contains('/') && !why.contains(scratch.to_string_lossy().as_ref()),
        "a path in {why}"
    );
    why
}

#[test]
fn a_conversation_carried_to_a_home_whose_projects_links_the_firsts() {
    // The stopped start's transcript is in work-1's home and work-2's
    // `projects` is a symbolic link to work-1's: carry answers yes with
    // the ack's session, and the next start's command resumes it. So does
    // work-1's own home, a pool of one account across its reset, and the
    // host user's `~/.claude` where the account names no
    // `CLAUDE_CONFIG_DIR`. Nothing in either home is written (179, C.4;
    // core.32).
    let harness = Harness::new();
    let who = identity("code");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    harness.hit_usage_limit(&who);
    let second = harness.sharing("work-2");
    let before = (tree(&harness.home("work-1")), tree(&second));
    let yes = Carry::Yes {
        session: SESSION.into(),
    };
    assert_eq!(harness.seam.carry(&folder, &who, &at_home(&second)), yes);
    assert_eq!(
        harness
            .seam
            .carry(&folder, &who, &at_home(&harness.home("work-1"))),
        yes
    );
    let command = harness.seam.command(
        &harness.folder(),
        &Settings::default(),
        None,
        Some(SESSION),
        &at_home(&second),
        &harness.place(),
        &Identity {
            start: 2,
            ..who.clone()
        },
        &Secret("s".into()),
        "carry on from where you stopped",
    );
    assert_eq!(command.args[2..], ["--resume", SESSION]);
    assert_eq!(
        (tree(&harness.home("work-1")), tree(&second)),
        before,
        "a home was written"
    );

    // No CLAUDE_CONFIG_DIR: the home is `.claude` under the host user's
    // own, here a link to work-1's; an empty value is none.
    let login = harness.scratch.path("login");
    std::fs::create_dir_all(&login).expect("the login home");
    std::os::unix::fs::symlink(harness.home("work-1"), login.join(".claude")).expect("the link");
    let record = Record::at(&folder, &who);
    assert_eq!(carry::carry(&record, &BTreeMap::new(), Some(&login)), yes);
    assert_eq!(
        carry::carry(
            &record,
            &BTreeMap::from([(carry::CONFIG_DIR.to_string(), String::new())]),
            Some(&login)
        ),
        yes
    );
    no_words(
        &harness,
        carry::carry(
            &record,
            &BTreeMap::new(),
            Some(&harness.scratch.path("elsewhere")),
        ),
    );
    no_words(&harness, carry::carry(&record, &BTreeMap::new(), None));
}

#[test]
fn two_homes_of_their_own() {
    // work-2's `projects` is a folder of its own holding no such
    // transcript, then a copy of it under the same names, which is another
    // file: carry answers no each time, naming `CLAUDE_CONFIG_DIR` and no
    // path (179; core.30, core.32).
    let harness = Harness::new();
    let who = identity("code");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    harness.hit_usage_limit(&who);
    let own = harness.home("work-2");
    std::fs::create_dir_all(own.join("projects").join(PROJECT)).expect("its own projects");
    let why = no_words(&harness, harness.seam.carry(&folder, &who, &at_home(&own)));
    assert_eq!(why, carry::UNREACHED);
    let copied = harness.apart("work-3", &who);
    no_words(
        &harness,
        harness.seam.carry(&folder, &who, &at_home(&copied)),
    );
}

#[test]
fn a_start_that_never_said_it_started() {
    // The stopped start has no started ack: carry answers no, saying its
    // conversation is not known; one whose ack's transcript is gone, that
    // it cannot be read (179).
    let harness = Harness::new();
    let who = identity("code");
    let folder = harness.folder();
    let second = harness.home("work-2");
    let why = no_words(
        &harness,
        harness.seam.carry(&folder, &who, &at_home(&second)),
    );
    assert!(why.starts_with("its conversation is not known"), "{why}");

    harness.start(&AgentCommand::default(), &who, &folder);
    std::fs::remove_file(harness.transcript(&who)).expect("the transcript removed");
    let why = no_words(
        &harness,
        harness
            .seam
            .carry(&folder, &who, &at_home(&harness.sharing("work-2"))),
    );
    assert_eq!(why, carry::UNREADABLE);
}

#[test]
fn a_background_task_the_stopped_agent_left() {
    // The stopped agent started a background shell and never finished it,
    // before the length the carried start's ack recorded: own work reads
    // no for the carried start, and yes for the stopped one; once the
    // carried agent starts work of its own, yes (31, 91, 179).
    let harness = Harness::new();
    let who = identity("code");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    harness.start_own_work(&who);
    let next = harness.carried(&who);
    assert_eq!(harness.seam.own_work(&folder, &next), Reading::No);
    assert_eq!(harness.seam.own_work(&folder, &who), Reading::Yes);
    append(
        &harness.transcript(&who),
        &fixture("background-started.jsonl"),
    );
    assert_eq!(harness.seam.own_work(&folder, &next), Reading::Yes);
}

#[test]
fn last_words_before_the_carried_agent_wrote_any() {
    // The carried agent stops before it writes a message, and the
    // transcript's last message is the stopped agent's: the carried
    // start's last words are none, and the stopped start's its own; once
    // the carried agent writes one, it is its last words (43, 179).
    let harness = Harness::new();
    let who = identity("code");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    harness.say_last(&who, "half way through the parser");
    let next = harness.carried(&who);
    assert_eq!(harness.seam.last_words(&folder, &next), None);
    assert_eq!(
        harness.seam.last_words(&folder, &who).as_deref(),
        Some("half way through the parser")
    );
    harness.say_last(&who, "the parser is done");
    assert_eq!(
        harness.seam.last_words(&folder, &next).as_deref(),
        Some("the parser is done")
    );
}

#[test]
fn a_question_and_a_slash_command_of_the_stopped_agent() {
    // The stopped agent's question was answered and its `/compact`
    // received before the carried start's length: the carried agent's
    // mark on the same call is not answered by them, and its own
    // `/compact` is not received until its own part holds it (89, 90,
    // 179).
    let harness = Harness::new();
    let who = identity("code");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    append(&harness.transcript(&who), &fixture("question.jsonl"));
    append(
        &harness.transcript(&who),
        &fixture("question-answered.jsonl"),
    );
    append(&harness.transcript(&who), &fixture("slash-command.jsonl"));
    let next = harness.carried(&who);
    let prompt = "/compact keep the parser's notes";
    assert!(matches!(
        harness.seam.received(&folder, &who, prompt),
        Received::Received { .. }
    ));
    assert!(matches!(
        harness.seam.received(&folder, &next, prompt),
        Received::NotReceived { .. }
    ));
    // The carried agent asks the same call again, in the session it
    // resumed.
    append(&harness.transcript(&who), &fixture("question.jsonl"));
    harness
        .record(&next)
        .mark_asking(&Asking {
            tool_use_id: Some("toolu_01AskWhichParser".into()),
            tool_name: "AskUserQuestion".into(),
            tool_input: serde_json::Value::Null,
        })
        .expect("the mark");
    assert_eq!(harness.seam.asking(&folder, &next), Reading::Yes);
    append(
        &harness.transcript(&who),
        &fixture("question-answered.jsonl"),
    );
    assert_eq!(harness.seam.asking(&folder, &next), Reading::No);
    append(&harness.transcript(&who), &fixture("slash-command.jsonl"));
    assert!(matches!(
        harness.seam.received(&folder, &next, prompt),
        Received::Received { .. }
    ));
}

#[test]
fn a_fresh_start_reads_its_whole_transcript() {
    // A start that carries nothing recorded a length of 0 at its started
    // ack: every reading reads its whole transcript, as before (91).
    let harness = Harness::new();
    let who = identity("code");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    harness.start_own_work(&who);
    harness.say_last(&who, "working on the parser");
    harness.ask(&who);
    append(&harness.transcript(&who), &fixture("slash-command.jsonl"));
    assert_eq!(
        harness.seam.own_work(&folder, &who),
        Reading::Yes,
        "own work"
    );
    assert_eq!(
        harness.seam.last_words(&folder, &who).as_deref(),
        Some("working on the parser")
    );
    assert_eq!(harness.seam.asking(&folder, &who), Reading::Yes);
    assert!(matches!(
        harness
            .seam
            .received(&folder, &who, "/compact keep the parser's notes"),
        Received::Received { .. }
    ));
    let record = Record::at(&folder, &who);
    assert_eq!(record.started().map(|started| started.from), Some(0));
}

#[test]
fn a_transcript_read_from_its_length() {
    // The records that begin at or after the length are read, a line cut
    // there left out; a file shorter than the length, which Claude Code
    // wrote again, is read whole (179).
    let harness = Harness::new();
    let who = identity("code");
    harness.start(&AgentCommand::default(), &who, &harness.folder());
    harness.say_last(&who, "the first");
    let path = harness.transcript(&who);
    let length = transcript::length(&path);
    harness.say_last(&who, "the second");
    let last = |from: u64| match transcript::Transcript::read_from(&path, from) {
        transcript::Read::Read(read) => read.last_message(),
        _ => panic!("the transcript unread"),
    };
    assert_eq!(last(0).as_deref(), Some("the second"));
    assert_eq!(last(length).as_deref(), Some("the second"));
    assert_eq!(last(length - 1).as_deref(), Some("the second"));
    assert_eq!(last(length + 1), None, "a line cut at the length was read");
    assert_eq!(
        last(transcript::length(&path) + 1).as_deref(),
        Some("the second"),
        "a shorter file was not read whole"
    );
    assert!(matches!(
        transcript::Transcript::read_from(&harness.scratch.path("none.jsonl"), 9),
        transcript::Read::NotWritten
    ));
    assert_eq!(transcript::length(&harness.scratch.path("none.jsonl")), 0);
}

/// The processes live reads, by a listing of the host's.
fn listed() -> Option<Vec<live::Process>> {
    live::processes()
}

#[test]
fn a_session_record_and_its_process() {
    // work-1's `sessions/<pid>.json` names the start's session with the
    // `pid` and `procStart` of a process the test runs: live answers runs
    // with that process, under work-1's home and under the login
    // environment's `~/.claude` where that is work-1's; once the process
    // ends, none, the record still on disk (207, C.4).
    let harness = Harness::new();
    let who = identity("talk");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    let session = new_session();
    harness.name_session(&who, &session);
    let home = harness.home("work-1");
    let pid = harness.run_process();
    let record = harness.session_record(&home, pid, &session, Some(&proc_start(pid)));
    let runs = Live::Runs(vec![live_process(pid)]);
    assert_eq!(harness.seam.live(&folder, &who, &[at_home(&home)]), runs);
    let login = harness.scratch.path("login");
    std::fs::create_dir_all(&login).expect("the login home");
    std::os::unix::fs::symlink(&home, login.join(".claude")).expect("the link");
    assert_eq!(
        live::live(
            &Record::at(&folder, &who),
            &[BTreeMap::new()],
            Some(&login),
            listed
        ),
        runs
    );
    // A home that keeps no `sessions` folder, or another session's record,
    // names nothing.
    assert_eq!(
        harness
            .seam
            .live(&folder, &who, &[at_home(&harness.home("work-2"))]),
        Live::None
    );
    let other = identity("code");
    harness.start(&AgentCommand::default(), &other, &folder);
    harness.name_session(&other, &new_session());
    assert_eq!(
        harness.seam.live(&folder, &other, &[at_home(&home)]),
        Live::None
    );
    // A start that never said it started names no session.
    assert_eq!(
        harness
            .seam
            .live(&folder, &identity("verify"), &[at_home(&home)]),
        Live::None
    );

    harness.end_process(pid);
    assert_eq!(
        harness.seam.live(&folder, &who, &[at_home(&home)]),
        Live::None
    );
    assert!(record.is_file(), "the record was removed");
}

#[test]
fn a_reused_process_id() {
    // The record names a `pid` the host runs, started at another time
    // than its `procStart`: live answers none. A record with no
    // `procStart` counts where its process runs, as 2.1.296 reads its own
    // (207).
    let harness = Harness::new();
    let who = identity("talk");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    let session = new_session();
    harness.name_session(&who, &session);
    let home = harness.home("work-1");
    let pid = harness.run_process();
    harness.session_record(&home, pid, &session, Some("Thu Jan  1 00:00:00 1970"));
    assert_eq!(
        harness.seam.live(&folder, &who, &[at_home(&home)]),
        Live::None
    );
    harness.session_record(&home, pid, &session, None);
    assert_eq!(
        harness.seam.live(&folder, &who, &[at_home(&home)]),
        Live::Runs(vec![live_process(pid)])
    );
}

#[test]
fn a_resume_under_a_home_not_given() {
    // A process `claude --resume <session>` runs under a home the profile
    // is not given: live answers runs with that process, given any homes
    // or none; one resuming another session counts nothing (207, S117).
    let harness = Harness::new();
    let who = identity("talk");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    let session = new_session();
    harness.name_session(&who, &session);
    harness.run_resume(&new_session());
    assert_eq!(harness.seam.live(&folder, &who, &[]), Live::None);
    let pid = harness.run_resume(&session);
    let runs = Live::Runs(vec![live_process(pid)]);
    assert_eq!(harness.seam.live(&folder, &who, &[]), runs);
    assert_eq!(
        harness
            .seam
            .live(&folder, &who, &[at_home(&harness.home("work-1"))]),
        runs
    );
    harness.end_process(pid);
    assert_eq!(harness.seam.live(&folder, &who, &[]), Live::None);
}

#[test]
fn a_sessions_folder_that_cannot_be_read() {
    // A home's `sessions` stands and cannot be listed: live answers
    // unknown, naming `CLAUDE_CONFIG_DIR` and no path, whatever another
    // home holds; and where the host's processes cannot be listed,
    // unknown naming them (207; core.30).
    let harness = Harness::new();
    let who = identity("talk");
    let folder = harness.folder();
    harness.start(&AgentCommand::default(), &who, &folder);
    let blocked = harness.unlistable("work-9");
    let answer = harness.seam.live(
        &folder,
        &who,
        &[at_home(&harness.home("work-1")), at_home(&blocked)],
    );
    let Live::Unknown { what } = answer else {
        panic!("live answered {answer:?}");
    };
    assert_eq!(what, live::UNREADABLE);
    assert!(what.contains("`CLAUDE_CONFIG_DIR`"), "{what}");
    assert!(!what.contains('/'), "a path in {what}");
    assert_eq!(
        live::live(
            &Record::at(&folder, &who),
            &[at_home(&harness.home("work-1"))],
            None,
            || None
        ),
        Live::Unknown {
            what: live::UNLISTED.into()
        }
    );
}

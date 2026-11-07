//! The stand-in for Claude Code driven through one stage (design 18): the
//! command the printed terminal gives run as a person runs it, the
//! stand-in first on `PATH` as `claude`. Its session acks its start and
//! its prompt through the plugin's hooks, its transcript is written as
//! the pinned Claude Code writes one, a push it tries is refused by the
//! guard, and its stop starts the settle that moves the place on to the
//! next stage once its session ended, with no settle of the test's (35,
//! 89, 108, 116, 121, S29); while its session runs it names it in its
//! home's `sessions/<pid>.json`, as the pinned Claude Code does, and
//! removes the record at its exit (207). A session it is told to resume
//! and finds no transcript of ends it as the pinned Claude Code ends
//! (design 8; C.4). In a pane of a herdr server of the test's own it
//! takes each prompt as the pinned herdr's `agent prompt` types it, its
//! session kept open between a script's two runs, and reports its state,
//! herdr reading it working while a turn runs and idle once it ends; and
//! each start reads its own script by its stage and number from the
//! folder `STAND_IN_SCRIPTS` names, keeping its log beside it (design 10;
//! 37, 38, 53, 54; core.29).

use super::walk::*;
use flume::host::process::listed;
use flume::profiles::agent::claude::live::{started_at, SESSIONS};
use serde_json::{json, Value};

/// A kind of two stages that does not land.
const KINDS: &str = r#"[kinds.review]
version = 1
branch = "review"
starts_unready = true
lands = false

[[kinds.review.stages]]
name = "code"
prompt = "prompts/work.md"
deliverables = ["commits"]
agents = [{}]

[[kinds.review.stages]]
name = "check"
prompt = "prompts/work.md"
deliverables = ["commits"]
agents = [{}]
"#;

#[test]
fn the_stand_in_drives_one_stage() {
    let walk = Walk::new("stand-in");
    let kinds = walk.repository.scratch.path("kinds/kinds.toml");
    std::fs::create_dir_all(kinds.parent().expect("a folder").join("prompts"))
        .expect("the kinds' folder");
    std::fs::write(&kinds, KINDS).expect("the kinds");
    std::fs::write(
        walk.repository.scratch.path("kinds/prompts/work.md"),
        "Do the work of {{name}}: {{brief}}. To stop short: {{needs}}\n",
    )
    .expect("the prompt");
    let place = walk.launch(
        "review",
        "one",
        &[
            "--brief",
            "fix the parser",
            "--kinds",
            &kinds.to_string_lossy(),
        ],
    );
    let prompt = walk.prompt(&place);
    assert!(prompt.contains("fix the parser"), "{prompt}");

    let midway = walk.signal("midway");
    let started = walk.signal("started");
    let agent = walk.start_agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "bash": "git push origin HEAD" },
            { "bash": "git add parser.rs && git commit --quiet -m 'the parser'" },
            { "bash": format!("touch '{}'", started.display()) },
            { "wait_for": midway },
        ]),
    );
    // Midway through its stage the agent has started under its start's
    // identity, and the push it tried is refused and reported.
    let began = std::time::Instant::now();
    while !started.exists() {
        assert!(
            began.elapsed() < std::time::Duration::from_secs(60),
            "the stand-in never came to its commit"
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let status = walk.status(&place);
    assert_eq!(status["agent"]["started"], "yes", "{status}");
    assert_eq!(
        status["agent"]["settings"]["from"]["env.CLAUDE_CONFIG_DIR"],
        serde_json::json!({"layer": "account", "value": "<set>"}),
        "{status}"
    );
    assert_eq!(status["agent"]["asking"], "no", "{status}");
    assert!(
        status["refused_acts"].to_string().contains("git push"),
        "{status}"
    );
    // Its session is named in its home's `sessions/<pid>.json`: the
    // process that runs it, started at the record's `procStart`, and the
    // session its transcript is named for.
    let records: Vec<std::path::PathBuf> = std::fs::read_dir(walk.account().join(SESSIONS))
        .expect("the running sessions' folder")
        .flatten()
        .map(|entry| entry.path())
        .collect();
    assert_eq!(records.len(), 1, "{records:?}");
    let record: Value =
        serde_json::from_str(&std::fs::read_to_string(&records[0]).expect("the record"))
            .expect("the record as JSON");
    let pid = record["pid"].as_u64().expect("a pid");
    assert_eq!(
        records[0].file_name().and_then(|name| name.to_str()),
        Some(format!("{pid}.json").as_str())
    );
    let process = listed()
        .expect("the host's processes")
        .into_iter()
        .find(|process| u64::from(process.pid) == pid)
        .expect("the record's process runs");
    assert_eq!(
        record["procStart"].as_str().and_then(started_at),
        Some(process.started),
        "{record}"
    );
    let session = record["sessionId"].as_str().expect("a session").to_string();
    std::fs::write(&midway, "").expect("the signal");
    let ran = finish(agent);
    assert_eq!(ran.code, 0, "{}", ran.out);
    assert!(
        ran.out.contains(&format!("session {session} in ")),
        "{}",
        ran.out
    );
    assert!(!records[0].exists(), "the record left at the exit");
    assert!(ran.out.contains("Bash refused"), "{}", ran.out);
    assert!(ran.out.contains("session ended"), "{}", ran.out);

    // The transcript holds the prompt queued and as the user's turn, and
    // each tool call with its result.
    let projects = walk.account().join("projects");
    let transcript = std::fs::read_dir(&projects)
        .expect("the transcripts' folder")
        .flatten()
        .flat_map(|project| {
            std::fs::read_dir(project.path())
                .into_iter()
                .flatten()
                .flatten()
        })
        .map(|file| std::fs::read_to_string(file.path()).expect("a transcript"))
        .next()
        .expect("a transcript");
    let records: Vec<Value> = transcript
        .lines()
        .map(|line| serde_json::from_str(line).expect("a JSON record"))
        .collect();
    assert_eq!(records[0]["operation"], "enqueue");
    assert_eq!(records[0]["content"], prompt.as_str());
    assert!(
        records
            .iter()
            .any(|record| record["type"] == "user"
                && record["message"]["content"] == prompt.as_str()),
        "the user's turn"
    );
    let uses = records
        .iter()
        .filter_map(|record| record["message"]["content"].as_array())
        .flatten()
        .filter(|block| block["type"] == "tool_use")
        .count();
    let results = records
        .iter()
        .filter_map(|record| record["message"]["content"].as_array())
        .flatten()
        .filter(|block| block["type"] == "tool_result")
        .count();
    assert_eq!((uses, results), (4, 4), "{transcript}");

    // The stop's settle, once the session ended, ends the stage at its
    // deliverable and starts the next; the test settles nothing.
    let began = std::time::Instant::now();
    let status = loop {
        let status = walk.status(&place);
        if status["current"] == "check" || began.elapsed() > std::time::Duration::from_secs(90) {
            break status;
        }
        std::thread::sleep(std::time::Duration::from_millis(300));
    };
    assert_eq!(status["current"], "check", "{status}");
    assert_eq!(stage(&status, "code")["state"], "delivered", "{status}");
    assert_eq!(
        status["agent"]["started"], "no",
        "the next stage's agent: {status}"
    );
}

#[test]
fn the_stand_in_refuses_a_session_it_finds_no_transcript_of() {
    // Design 8, C.4: `--resume <session>` where its home's `projects`
    // holds no transcript of the session ends the stand-in as the pinned
    // Claude Code ends, with its words and exit status 1, before any hook
    // and with no session begun or transcript written.
    let walk = Walk::new("stand-in-resume");
    let output = walk
        .command(env!("CARGO_BIN_EXE_flume-stand-in"), &walk.repository.dir)
        .env("CLAUDE_CONFIG_DIR", walk.account())
        .args(["--resume", "no-such-session", "the prompt"])
        .output()
        .expect("the stand-in runs");
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "No conversation found with session ID: no-such-session\n"
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    assert!(
        !walk.account().join("projects").exists(),
        "a transcript written"
    );
}

/// The records of the one transcript the stand-in wrote under `home`.
fn transcript_under(home: &std::path::Path) -> Vec<Value> {
    let text = std::fs::read_dir(home.join("projects"))
        .expect("the transcripts' folder")
        .flatten()
        .flat_map(|project| {
            std::fs::read_dir(project.path())
                .into_iter()
                .flatten()
                .flatten()
        })
        .map(|file| std::fs::read_to_string(file.path()).expect("a transcript"))
        .next()
        .expect("a transcript");
    text.lines()
        .map(|line| serde_json::from_str(line).expect("a JSON record"))
        .collect()
}

/// Waits up to a minute until `holds` holds, failing naming `what`.
fn until_it(what: &str, holds: impl Fn() -> bool) {
    let began = std::time::Instant::now();
    while !holds() {
        assert!(
            began.elapsed() < std::time::Duration::from_secs(60),
            "never came to {what}"
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[test]
fn the_stand_in_under_herdr_takes_its_later_prompt_and_reports_its_state() {
    // Design 10; 37, 38, 53, 54; core.29: started in a herdr pane with no
    // prompt, as a terminal that types the prompt starts it, the stand-in
    // reports itself idle; the first prompt, typed as flume's herdr profile
    // sends one, a newline within it, makes the first run's acts, herdr
    // reading it working while the run waits; once the turn ends herdr
    // reads it idle and its session stays open for the second run; the
    // second prompt makes the second run's acts, and the session then
    // ends, herdr listing no agent in the pane.
    let server = HerdrServer::start("stand-in-herdr");
    let work = server.scratch.path("work");
    let home = server.scratch.path("account");
    let bin = server.scratch.path("bin");
    for dir in [&work, &home, &bin] {
        std::fs::create_dir_all(dir).expect("a scratch folder");
    }
    std::os::unix::fs::symlink(env!("CARGO_BIN_EXE_flume-stand-in"), bin.join("claude"))
        .expect("the stand-in as claude");
    let midway = server.scratch.path("midway");
    let script = server.scratch.path("script.json");
    std::fs::write(
        &script,
        json!([
            [
                { "write": { "path": "first.txt", "text": "one\n" } },
                { "wait_for": midway },
            ],
            [{ "write": { "path": "second.txt", "text": "two\n" } }],
        ])
        .to_string(),
    )
    .expect("the script");
    let out = server.scratch.path("out");
    let launcher = server.scratch.path("launcher");
    std::fs::write(
        &launcher,
        format!(
            "#!/bin/sh\nexport CLAUDE_CONFIG_DIR='{}' STAND_IN_SCRIPT='{}'\nexec '{}' > '{}' 2>&1\n",
            home.display(),
            script.display(),
            bin.join("claude").display(),
            out.display()
        ),
    )
    .expect("the launcher");
    std::fs::set_permissions(
        &launcher,
        <std::fs::Permissions as std::os::unix::fs::PermissionsExt>::from_mode(0o755),
    )
    .expect("the launcher made runnable");
    let pane = server.pane_in(&work);
    server.herdr(&["pane", "run", &pane, &launcher.to_string_lossy()]);
    let printed = || std::fs::read_to_string(&out).unwrap_or_default();
    let reads = |state: &str| {
        let read = server.pane(&pane);
        read["agent"] == "claude" && read["agent_status"] == state
    };
    until_it("the stand-in up and idle", || reads("idle"));
    assert!(!printed().contains("took the prompt"), "{}", printed());

    server.prompt(&pane, "the first\nprompt");
    until_it("the first run working", || reads("working"));
    assert!(work.join("first.txt").is_file(), "{}", printed());
    std::fs::write(&midway, "").expect("the signal");
    until_it("the first turn ended", || {
        printed().contains("the turn ended") && reads("idle")
    });
    assert!(!printed().contains("session ended"), "{}", printed());
    assert!(!work.join("second.txt").exists(), "{}", printed());

    server.prompt(&pane, "the second");
    until_it("the session ended", || printed().contains("session ended"));
    until_it("herdr listing no agent in the pane", || {
        server.pane(&pane)["agent"].is_null()
    });
    assert!(work.join("second.txt").is_file(), "{}", printed());
    let records = transcript_under(&home);
    let turns: Vec<&Value> = records
        .iter()
        .filter(|record| record["type"] == "user" && record["message"]["content"].is_string())
        .map(|record| &record["message"]["content"])
        .collect();
    assert_eq!(
        turns,
        [&json!("the first\nprompt"), &json!("the second")],
        "the user's turns"
    );
    let queued: Vec<&Value> = records
        .iter()
        .filter(|record| record["operation"] == "enqueue")
        .map(|record| &record["content"])
        .collect();
    assert_eq!(
        queued,
        [&json!("the first\nprompt"), &json!("the second")],
        "the prompts queued"
    );
    let ids: std::collections::BTreeSet<&str> = records
        .iter()
        .filter_map(|record| record["uuid"].as_str())
        .collect();
    assert_eq!(
        ids.len(),
        records
            .iter()
            .filter(|record| record["uuid"].is_string())
            .count(),
        "each record its own id"
    );
}

#[test]
fn each_start_reads_its_own_script_from_the_scripts_folder() {
    // Design 10: where `STAND_IN_SCRIPTS` names a folder, each start reads
    // `<stage>-<n>.json` there by its `FLUME_STAGE` and the number its
    // `FLUME_AGENT` ends in, over the file `STAND_IN_SCRIPT` names, and
    // keeps its log beside it as `<stage>-<n>.log`, each prompt it took
    // among its lines; a start the folder holds no script of acts none.
    let walk = Walk::new("stand-in-scripts");
    let scripts = walk.repository.scratch.path("scripts");
    std::fs::create_dir_all(&scripts).expect("the scripts' folder");
    for start in [1, 2] {
        std::fs::write(
            scripts.join(format!("code-{start}.json")),
            json!([{ "write": { "path": format!("start-{start}.txt"), "text": "made\n" } }])
                .to_string(),
        )
        .expect("a start's script");
    }
    let named = walk.repository.scratch.path("named.json");
    std::fs::write(
        &named,
        json!([{ "write": { "path": "named.txt", "text": "made\n" } }]).to_string(),
    )
    .expect("the named script");
    let run = |start: u32, prompt: &str| {
        let output = walk
            .command(env!("CARGO_BIN_EXE_flume-stand-in"), &walk.repository.dir)
            .env("CLAUDE_CONFIG_DIR", walk.account())
            .env("STAND_IN_SCRIPTS", &scripts)
            .env("STAND_IN_SCRIPT", &named)
            .env("FLUME_STAGE", "code")
            .env("FLUME_AGENT", format!("flume:r:fix--one:code:{start}"))
            .arg(prompt)
            .output()
            .expect("the stand-in runs");
        assert!(output.status.success(), "{output:?}");
    };
    run(2, "the second start's prompt");
    run(1, "the first start's\nprompt");
    run(3, "a start with no script");
    for start in [1, 2] {
        assert!(
            walk.repository
                .dir
                .join(format!("start-{start}.txt"))
                .is_file(),
            "start {start} made its own acts"
        );
    }
    assert!(
        !walk.repository.dir.join("named.txt").exists(),
        "STAND_IN_SCRIPT read over the folder"
    );
    let log = |start: u32| {
        std::fs::read_to_string(scripts.join(format!("code-{start}.log"))).expect("a start's log")
    };
    assert!(
        log(1).contains("stand-in: took the prompt: \"the first start's\\nprompt\"\n"),
        "{}",
        log(1)
    );
    assert!(
        log(2).contains("stand-in: took the prompt: \"the second start's prompt\"\n"),
        "{}",
        log(2)
    );
    assert!(log(3).contains("session ended"), "{}", log(3));
    assert!(!log(3).contains("Write done"), "{}", log(3));
}

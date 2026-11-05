//! The stand-in for Claude Code driven through one stage (design 18): the
//! command the printed terminal gives run as a person runs it, the
//! stand-in first on `PATH` as `claude`. Its session acks its start and
//! its prompt through the plugin's hooks, its transcript is written as
//! the pinned Claude Code writes one, a push it tries is refused by the
//! guard, and its stop starts the settle that moves the place on to the
//! next stage once its session ended, with no settle of the test's (35,
//! 89, 108, 116, 121, S29). A session it is told to resume and finds no
//! transcript of ends it as the pinned Claude Code ends (design 8; C.4).

use super::walk::*;
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
    std::fs::write(&midway, "").expect("the signal");
    let ran = finish(agent);
    assert_eq!(ran.code, 0, "{}", ran.out);
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

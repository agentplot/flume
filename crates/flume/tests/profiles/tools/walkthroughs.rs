//! The walkthroughs (design 18): flume's built binary under the printed
//! command, on a scratch repository bound to `git-worktree`, `openspec`
//! and `sluice`, landing on a local bare remote, each agent the stand-in
//! for Claude Code run as a person runs the printed command, and the test
//! settling as a caller would between its commands. A kind runs through
//! on its own: a fix lands through flume's own submission (S1); a failed
//! submission and a conflicting one each go back to code with sluice's
//! words and land on the second (112); a change runs construct, code and
//! verify, its report committed in the archived change (S2, S35); verify
//! finding work sends the place back to code (131); and a host's bound
//! queues a launch and starts the waiting ones by priority (S37).

use super::walk::*;
use serde_json::{json, Value};

/// The acts of a code stage that writes `file` and commits it.
fn commits(file: &str, text: &str) -> Value {
    json!([
        { "write": { "path": file, "text": text } },
        { "bash": format!("git add -A && git commit --quiet -m 'write {file}'") },
    ])
}

/// Settles the place until it is tidied, its work landed.
fn until_landed(walk: &Walk, place: &str) {
    walk.until(place, "landed and tidied", Value::is_null);
}

/// The stand-in writing the change `name` as OpenSpec's spec-driven
/// schema has it, two tasks unticked, and committing it.
fn construct(name: &str) -> Value {
    let at = format!("openspec/changes/{name}");
    json!([
        { "bash": format!("openspec new change {name}") },
        { "write": { "path": format!("{at}/proposal.md"), "text": "## Why\n\nThe tool cannot read its input.\n\n## What Changes\n\n- Add a parser.\n" } },
        { "write": { "path": format!("{at}/specs/parser/spec.md"), "text": "## ADDED Requirements\n\n### Requirement: Parsing\nThe tool SHALL parse its input.\n\n#### Scenario: Plain input\n- **WHEN** the input is a word\n- **THEN** the parse yields the word\n" } },
        { "write": { "path": format!("{at}/design.md"), "text": "## Context\n\nA parser of words.\n" } },
        { "write": { "path": format!("{at}/tasks.md"), "text": "## 1. Parser\n\n- [ ] 1.1 Write the parser\n- [ ] 1.2 Test the parser\n" } },
        { "bash": format!("git add -A && git commit --quiet -m 'the change {name}'") },
    ])
}

/// The stand-in's verify: the report written, the change archived with
/// it, and committed.
fn verify_holds(name: &str) -> Value {
    json!([
        { "write": { "path": format!("openspec/changes/{name}/verify.md"), "text": "# Verify\n\nEvery requirement met, every task ticked.\n" } },
        { "bash": format!("openspec archive {name} --yes") },
        { "bash": format!("git add -A && git commit --quiet -m 'archive {name}'") },
    ])
}

#[test]
fn a_fix_lands_through_flumes_own_submission() {
    // S1: the fix's agent commits, tries to push and to submit, both
    // refused; flume submits the place, sluice lands it on the remote, and
    // flume tidies the place and its branch.
    let walk = Walk::new("walk-fix");
    let place = walk.launch("fix", "one", &["--brief", "fix the parser"]);
    let out = walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "bash": "git add -A && git commit --quiet -m 'fix the parser'" },
            { "bash": "git push origin HEAD" },
            { "bash": "sluice submit main --no-wait" },
        ]),
    );
    assert_eq!(out.matches("Bash refused").count(), 2, "{out}");
    until_landed(&walk, &place);

    // One submission, flume's: the agent's own was refused.
    let submissions = walk.submissions();
    assert_eq!(submissions.len(), 1, "{submissions:?}");
    let entry = &submissions[0];
    assert_eq!(entry["state"], "landed", "{entry}");
    assert_eq!(entry["from"]["path"], place.as_str(), "{entry}");
    assert!(entry["from"].get("session").is_none(), "{entry}");
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n")
    );
    assert!(
        walk.remote_log()[0].contains("fix/one"),
        "{:?}",
        walk.remote_log()
    );
    assert!(!std::path::Path::new(&place).exists(), "the place is left");
    assert!(!walk.has_branch("fix/one"), "the place's branch is left");
}

#[test]
fn a_failed_submission_goes_back_to_code_and_lands_on_the_second() {
    // 112: the gate fails the first submission; the code stage starts
    // again with sluice's notice in its prompt, its agent fixes it, and
    // the second submission lands.
    let walk = Walk::new("walk-failed");
    let place = walk.launch("fix", "two", &["--brief", "fix the parser"]);
    walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "write": { "path": "broken", "text": "not yet\n" } },
            { "bash": "git add -A && git commit --quiet -m 'fix the parser'" },
        ]),
    );
    let status = walk.until(&place, "its code stage started again", |status| {
        stage(status, "code")["starts"] == 2 && !status["agent"].is_null()
    });
    assert_eq!(status["submission"]["send_backs"], 1, "{status}");
    let prompt = walk.prompt(&place);
    // sluice's notice, as sluice words it, names the failure and where
    // to read why.
    assert!(prompt.contains("[sluice] #1 failed on main"), "{prompt}");
    walk.agent(
        &place,
        json!([{ "bash": "git rm --quiet broken && git commit --quiet -m 'the parser test passes'" }]),
    );
    until_landed(&walk, &place);
    let states: Vec<Value> = walk
        .submissions()
        .iter()
        .map(|entry| entry["state"].clone())
        .collect();
    assert_eq!(states.len(), 2, "{states:?}");
    assert!(states.contains(&json!("landed")), "{states:?}");
    assert!(walk.on_remote("parser.rs").is_some());
    assert_eq!(walk.on_remote("broken"), None);
    assert!(!walk.has_branch("fix/two"));
}

#[test]
fn a_conflicting_submission_merges_the_base_and_lands() {
    // 112: a commit lands on the base meanwhile, touching the file the
    // place's work touches; the submission conflicts, the code stage
    // starts again with the files contested and the base's revision, the
    // stand-in merges that revision and commits, and the place lands.
    let walk = Walk::new("walk-conflict");
    let place = walk.launch("fix", "three", &["--brief", "rework the work"]);
    let base = walk.landed_elsewhere("work.rs", "the base's own\n");
    walk.agent(&place, commits("work.rs", "the place's own\n"));
    walk.until(&place, "its code stage started again", |status| {
        stage(status, "code")["starts"] == 2 && !status["agent"].is_null()
    });
    let prompt = walk.prompt(&place);
    assert!(prompt.contains("work.rs"), "{prompt}");
    assert!(prompt.contains(&base), "the base's revision: {prompt}");
    let out = walk.agent(
        &place,
        json!([{ "merge_named": { "work.rs": "the place's own and the base's own\n" } }]),
    );
    assert!(
        out.contains(&format!("git merge --no-edit {base}")),
        "{out}"
    );
    until_landed(&walk, &place);
    assert_eq!(
        walk.on_remote("work.rs").as_deref(),
        Some("the place's own and the base's own\n")
    );
    assert!(!walk.has_branch("fix/three"));
}

#[test]
fn a_change_runs_construct_code_and_verify() {
    // S2, S35: construct writes the change, code ticks its tasks, status
    // showing them ticked along the way, and verify commits its report and
    // archives the change; the place lands with the report in the archive.
    let walk = Walk::new("walk-change");
    let place = walk.launch("change", "add-parser", &["--brief", "add a parser"]);
    walk.agent(&place, construct("add-parser"));
    let status = walk.until(&place, "its code stage", |status| {
        status["current"] == "code" && !status["agent"].is_null()
    });
    assert_eq!(stage(&status, "construct")["state"], "delivered");
    let ticked = |status: &Value| -> Vec<bool> {
        status["tasks"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|task| task["ticked"] == true)
            .collect()
    };
    assert_eq!(ticked(&status), vec![false, false], "{status}");

    let midway = walk.signal("midway");
    let tasks = "openspec/changes/add-parser/tasks.md";
    let agent = walk.start_agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "tick": { "path": tasks, "count": 1 } },
            { "bash": "git add -A && git commit --quiet -m 'write the parser'" },
            { "wait_for": midway },
            { "tick": { "path": tasks } },
            { "bash": "git add -A && git commit --quiet -m 'test the parser'" },
        ]),
    );
    let began = std::time::Instant::now();
    let status = loop {
        let status = walk.status(&place);
        if ticked(&status) == vec![true, false]
            || began.elapsed() > std::time::Duration::from_secs(60)
        {
            break status;
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    };
    assert_eq!(ticked(&status), vec![true, false], "midway: {status}");
    assert_eq!(status["current"], "code", "{status}");
    std::fs::write(&midway, "").expect("the signal");
    let ran = finish(agent);
    assert_eq!(ran.code, 0, "{}", ran.out);

    let status = walk.until(&place, "its verify stage", |status| {
        status["current"] == "verify" && !status["agent"].is_null()
    });
    assert_eq!(ticked(&status), vec![true, true], "{status}");
    walk.agent(&place, verify_holds("add-parser"));
    until_landed(&walk, &place);

    let files = walk.remote_files();
    let report = files
        .iter()
        .find(|file| {
            file.starts_with("openspec/changes/archive/") && file.ends_with("-add-parser/verify.md")
        })
        .unwrap_or_else(|| panic!("no report in the archive: {files:?}"));
    assert!(walk.on_remote(report).is_some());
    assert!(
        !files
            .iter()
            .any(|file| file.starts_with("openspec/changes/add-parser/")),
        "the change is left unarchived: {files:?}"
    );
    assert!(
        files.contains(&"openspec/specs/parser/spec.md".to_string()),
        "{files:?}"
    );
    assert!(!walk.has_branch("change/add-parser"));
}

#[test]
fn verify_finding_work_sends_the_place_back_to_code() {
    // 131: verify stops short having found work; the code stage starts
    // again with its words, records the fix as a task ticked, and verify
    // then archives the change and the place lands.
    let walk = Walk::new("walk-found");
    let place = walk.launch("change", "add-parser", &["--brief", "add a parser"]);
    walk.agent(&place, construct("add-parser"));
    walk.until(&place, "its code stage", |status| {
        status["current"] == "code" && !status["agent"].is_null()
    });
    let tasks = "openspec/changes/add-parser/tasks.md";
    walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "tick": { "path": tasks } },
            { "bash": "git add -A && git commit --quiet -m 'the parser'" },
        ]),
    );
    walk.until(&place, "its verify stage", |status| {
        status["current"] == "verify" && !status["agent"].is_null()
    });
    let found = "the parser fails on empty input";
    walk.agent(
        &place,
        json!([{ "bash": format!("flume needs --found '{found}'") }]),
    );
    let status = walk.until(&place, "its code stage again", |status| {
        status["current"] == "code" && !status["agent"].is_null()
    });
    assert_eq!(stage(&status, "code")["starts"], 2, "{status}");
    let prompt = walk.prompt(&place);
    assert!(prompt.contains(found), "{prompt}");
    walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() { /* and empty input */ }\n" } },
            { "bash": format!("printf -- '- [x] 1.3 Parse empty input\\n' >> {tasks}") },
            { "bash": "git add -A && git commit --quiet -m 'parse empty input'" },
        ]),
    );
    walk.until(&place, "its verify stage again", |status| {
        status["current"] == "verify" && !status["agent"].is_null()
    });
    walk.agent(&place, verify_holds("add-parser"));
    until_landed(&walk, &place);
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() { /* and empty input */ }\n")
    );
}

#[test]
fn a_bound_host_queues_launches_and_starts_them_by_priority() {
    // S37: with room for two agents, the first two launches start, the
    // third and fourth wait for room, and as each agent ends the waiting
    // launch of the higher priority starts first.
    let walk = Walk::new("walk-room");
    walk.configure("[host]\nagents = 2\n");
    let first = walk.launch("fix", "first", &["--brief", "one"]);
    let second = walk.launch("fix", "second", &["--brief", "two"]);
    let low = walk.launch("fix", "low", &["--brief", "three"]);
    let high = walk.launch("fix", "high", &["--brief", "four", "--priority", "5"]);
    let host = walk.host();
    assert_eq!(
        (host["running"].clone(), host["room"].clone()),
        (json!(2), json!(0)),
        "{host}"
    );
    let queue: Vec<Value> = host["queue"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|queued| queued["place"].clone())
        .collect();
    assert_eq!(queue, vec![json!(high), json!(low)], "{host}");
    for waiting in [&low, &high] {
        let status = walk.status(waiting);
        assert!(status["agent"].is_null(), "{status}");
        assert!(
            status["waiting_on"].is_null(),
            "waiting for room is no caller's: {status}"
        );
    }

    walk.agent(&first, commits("first.rs", "first\n"));
    walk.until(&high, "its agent started", |status| {
        !status["agent"].is_null()
    });
    assert!(
        walk.status(&low)["agent"].is_null(),
        "the lower priority started first"
    );
    let host = walk.host();
    assert_eq!(host["queue"].as_array().map(Vec::len), Some(1), "{host}");
    assert_eq!(host["queue"][0]["place"], low.as_str(), "{host}");

    walk.agent(&second, commits("second.rs", "second\n"));
    walk.until(&low, "its agent started", |status| {
        !status["agent"].is_null()
    });
    assert_eq!(walk.host()["queue"], json!([]));
}

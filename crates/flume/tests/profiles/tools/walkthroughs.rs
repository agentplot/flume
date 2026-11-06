//! The walkthroughs (design 18): flume's built binary under the printed
//! command, on a scratch repository bound to `git-worktree`, `openspec`
//! and `sluice`, landing on a local bare remote, each agent the stand-in
//! for Claude Code run as a person runs the printed command, and the test
//! settling as a caller would between its commands. A kind runs through
//! on its own: a fix lands through flume's own submission (S1); a failed
//! submission and a conflicting one each go back to code with sluice's
//! words and land on the second (112), a conflict the stand-in for sluice
//! answers at the submit itself among them (133); a pull request the
//! stand-in for sluice answers with no entry is followed until it is
//! taken in, then to its landing, and a failure of the entry taken in
//! from it goes back to code once and lands on the next (124, C.6); a change runs construct, code and
//! verify, its report committed in the archived change (S2, S35); verify
//! finding work sends the place back to code (131); a fix whose prompt
//! Claude Code submits inside paste tags lands (89); a host's bound
//! queues a launch and starts the waiting ones by priority (S37), and a
//! stage waiting for room starts at the next command of flume's once the
//! host's configuration gives it room, status saying no watch is kept
//! (70, 119); a fix launched on a stream whose pool holds only `codex`
//! accounts is refused with nothing made, and a change whose pool the
//! host's configuration leaves a `codex` account alone while construct
//! works has code refused, no command printed for it, until a retry once
//! the configuration is mended (S81; 4, 32, 145); and
//! change@2's code stage works a change of three task groups one fresh
//! agent at a time, and a failed submission sends it back for one
//! repetition with no group (S50, S52; 112, 140 to 142); and a fix whose
//! code agent stops at its account's usage limit moves to the pool's
//! second account and lands (S55; 146, 151), and, where the second's
//! `projects` is a link to the first's, goes on there in its own
//! conversation and lands, flume writing nothing in either home (S78;
//! 179); and a fix whose first code
//! agent comes up without its account's variable is ended and its stage
//! started afresh under the account, and lands (S65; 161); a fix whose
//! host's table gives `edits` over its repository's `full` starts its
//! agent in the mode the host gave and lands, as does a fix no layer gives
//! a mode, passing none (S66; 162); a fix whose agent begins in another
//! mode than its start asked at both its starts is never begun, naming
//! both modes (S67; 161, 162); and an agent run only after its start was
//! read never begun acks its start and its prompt, its stage is read
//! begun, and a retry leaves it be (35, 38, 50); and a host's source's
//! kind, locked by `flume kinds lock --host --hash`, runs and lands, a
//! second version added and its source's file renamed over, the next
//! launch running it and the place launched before landing on its own
//! version (design 13; 21, 154, 155, 159, 191).

use super::walk::*;
use flume::profiles::agent::claude::carry::UNREACHED;
use flume::profiles::kinds::weir::Weir;
use flume_seams::{Kinds, PackageRead};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

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
    construct_tasks(
        name,
        "## 1. Parser\n\n- [ ] 1.1 Write the parser\n- [ ] 1.2 Test the parser\n",
    )
}

/// The stand-in writing the change `name` with `tasks` as its tasks
/// file, and committing it.
fn construct_tasks(name: &str, tasks: &str) -> Value {
    let at = format!("openspec/changes/{name}");
    json!([
        { "bash": format!("openspec new change {name}") },
        { "write": { "path": format!("{at}/proposal.md"), "text": "## Why\n\nThe tool cannot read its input.\n\n## What Changes\n\n- Add a parser.\n" } },
        { "write": { "path": format!("{at}/specs/parser/spec.md"), "text": "## ADDED Requirements\n\n### Requirement: Parsing\nThe tool SHALL parse its input.\n\n#### Scenario: Plain input\n- **WHEN** the input is a word\n- **THEN** the parse yields the word\n" } },
        { "write": { "path": format!("{at}/design.md"), "text": "## Context\n\nA parser of words.\n" } },
        { "write": { "path": format!("{at}/tasks.md"), "text": tasks } },
        { "bash": format!("git add -A && git commit --quiet -m 'the change {name}'") },
    ])
}

/// The stand-in's verify: the report written, the change archived with
/// it, and committed.
fn verify_holds(name: &str) -> Value {
    verify_archives(name, "")
}

/// The stand-in's verify, archiving with `archive`'s further words.
fn verify_archives(name: &str, archive: &str) -> Value {
    json!([
        { "write": { "path": format!("openspec/changes/{name}/verify.md"), "text": "# Verify\n\nEvery requirement met, every task ticked.\n" } },
        { "bash": format!("openspec archive {name} --yes{archive}") },
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
fn a_fix_lands_when_its_prompt_is_submitted_inside_paste_tags() {
    // A terminal sends the prompt as a paste, and the pinned Claude Code,
    // where the account's features turn paste tags on, submits it inside
    // `<pasted_content id="....">` tags: the stage still reads its prompt
    // received and begun, and the fix lands, where a signed-in agent's
    // stage read never begun while it did the work (38, 89; record 0016).
    let walk = Walk::new("walk-paste");
    let place = walk.launch("fix", "one", &["--brief", "fix the parser"]);
    walk.agent_with(
        &place,
        commits("parser.rs", "fn parse() {}\n"),
        &[("STAND_IN_PASTE_TAG", "fad0")],
    );
    walk.until(&place, "landed and tidied", |status| {
        assert_ne!(stage(status, "code")["state"], "never_begun", "{status}");
        status.is_null()
    });
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n")
    );
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
fn a_conflict_sluice_answers_at_the_submit_goes_back_to_code_and_lands() {
    // 112, 133: the stand-in for sluice answers the first submission with
    // its entry's record, found in conflict as it was taken in, exiting
    // as a conflict does and delivered, as sluice answers a no-wait submit
    // on a base that takes pull requests. The place goes back to code with
    // the files contested and the base's revision in its words, never
    // waiting on its caller as if refused, and lands on the second.
    let walk = Walk::new("walk-conflict-at-submit");
    walk.sluice_decides_the_first_submit();
    let place = walk.launch("fix", "four", &["--brief", "rework the work"]);
    let base = walk.landed_elsewhere("work.rs", "the base's own\n");
    walk.agent(&place, commits("work.rs", "the place's own\n"));
    let status = walk.until(&place, "its code stage started again", |status| {
        stage(status, "code")["starts"] == 2 && !status["agent"].is_null()
    });
    assert!(status["submission"]["waiting"].is_null(), "{status}");
    assert_eq!(
        status["submission"]["verdict"]["conflict"]["files"],
        json!(["work.rs"]),
        "{status}"
    );
    let first = walk.submissions();
    assert_eq!(first.len(), 1, "{first:?}");
    assert_eq!(first[0]["state"], "conflict", "{first:?}");
    assert!(
        first[0]["delivered"].is_string(),
        "the outcome sluice's answer delivered: {first:?}"
    );
    let prompt = walk.prompt(&place);
    for named in [
        "conflict on main: work.rs",
        "The files contested: work.rs.",
        &base,
    ] {
        assert!(prompt.contains(named), "{named} is not in {prompt}");
    }
    walk.agent(
        &place,
        json!([{ "merge_named": { "work.rs": "the place's own and the base's own\n" } }]),
    );
    until_landed(&walk, &place);
    assert_eq!(
        walk.on_remote("work.rs").as_deref(),
        Some("the place's own and the base's own\n")
    );
    // sluice withdraws the conflicting entry once the worktree submits
    // again.
    let states: Vec<Value> = walk
        .submissions()
        .iter()
        .map(|entry| entry["state"].clone())
        .collect();
    assert_eq!(states.len(), 2, "{states:?}");
    assert_eq!(states[1], "landed", "{states:?}");
    assert!(!walk.has_branch("fix/four"));
}

#[test]
fn a_pull_request_is_followed_from_no_entry_to_its_entry_and_its_landing() {
    // 124, C.6: the stand-in for sluice answers the submission as sluice
    // answers one on a base that takes pull requests whose request no
    // intake has taken in yet: the pull request and no entry. The place's
    // submission is open and undecided, never refused nor waiting on its
    // caller, until the host that takes the base's requests in takes it
    // in; flume follows the entry, reads its landing from sluice's
    // record, the inbox giving nothing, and tidies the place.
    let walk = Walk::new("walk-pull-request");
    walk.sluice_takes_pull_requests();
    let place = walk.launch("fix", "five", &["--brief", "fix the parser"]);
    walk.agent(&place, commits("parser.rs", "fn parse() {}\n"));
    walk.until(&place, "its submission held as a pull request", |status| {
        status["submission"]["request"] == PULL_REQUEST
    });
    for _ in 0..3 {
        walk.settle(&place);
    }
    let status = walk.status(&place);
    assert_eq!(status["phase"], "live", "{status}");
    assert_eq!(status["submission"]["submits"], 1, "{status}");
    assert_eq!(status["submission"]["read"], "none", "{status}");
    for unset in ["verdict", "waiting"] {
        assert!(status["submission"][unset].is_null(), "{status}");
    }
    assert!(status["waiting_on"].is_null(), "{status}");
    assert_eq!(status["stalled"], false, "{status}");
    assert!(walk.submissions().is_empty(), "an entry before intake");
    let person = walk.person(&["status", &place]);
    assert!(
        person.contains(&format!(
            "pull request #{PULL_REQUEST}, not yet taken in by the merge queue"
        )),
        "{person}"
    );

    let entry = walk.take_in_request();
    let status = walk.status(&place);
    let read = &status["submission"]["read"]["found"]["submission"];
    assert_eq!(read["id"], entry["id"].to_string(), "{status}");
    assert_eq!(read["request"], PULL_REQUEST, "{status}");
    assert_eq!(read["state"], "waiting", "{status}");
    until_landed(&walk, &place);

    let submissions = walk.submissions();
    assert_eq!(submissions.len(), 1, "{submissions:?}");
    let landed = &submissions[0];
    assert_eq!(landed["state"], "landed", "{landed}");
    assert_eq!(
        landed["from"]["worktree"],
        format!("pull/{PULL_REQUEST}"),
        "{landed}"
    );
    assert_eq!(landed["from"]["head"], "fix/five", "{landed}");
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n")
    );
    assert!(!Path::new(&place).exists(), "the place is left");
    assert!(!walk.has_branch("fix/five"), "the place's branch is left");
}

#[test]
fn a_pull_requests_failure_goes_back_to_code_once_and_its_next_request_lands() {
    // 112, 124, 133: the gate fails the entry taken in from the place's
    // pull request, its outcome read from sluice's record; the code stage
    // starts again once with sluice's words, its agent fixes it, and the
    // request flume submits again waits to be taken in, the failed entry
    // never read as it, then lands.
    let walk = Walk::new("walk-pull-request-failed");
    walk.sluice_takes_pull_requests();
    let place = walk.launch("fix", "six", &["--brief", "fix the parser"]);
    walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "write": { "path": "broken", "text": "not yet\n" } },
            { "bash": "git add -A && git commit --quiet -m 'fix the parser'" },
        ]),
    );
    walk.until(&place, "its submission held as a pull request", |status| {
        status["submission"]["request"] == PULL_REQUEST
    });
    walk.take_in_request();
    let status = walk.until(&place, "its code stage started again", |status| {
        stage(status, "code")["starts"] == 2 && !status["agent"].is_null()
    });
    assert_eq!(status["submission"]["send_backs"], 1, "{status}");
    assert!(
        status["submission"]["verdict"]["failed"].is_object(),
        "{status}"
    );
    let prompt = walk.prompt(&place);
    assert!(prompt.contains("failed on main"), "{prompt}");
    for _ in 0..3 {
        walk.settle(&place);
    }
    assert_eq!(
        stage(&walk.status(&place), "code")["starts"],
        2,
        "the failure acted on again"
    );

    walk.agent(
        &place,
        json!([{ "bash": "git rm --quiet broken && git commit --quiet -m 'the parser test passes'" }]),
    );
    let status = walk.until(&place, "its second submission held", |status| {
        status["submission"]["submits"] == 2
    });
    assert_eq!(status["submission"]["read"], "none", "{status}");
    assert_eq!(status["submission"]["request"], PULL_REQUEST, "{status}");
    assert_eq!(status["submission"]["send_backs"], 1, "{status}");
    walk.take_in_request();
    until_landed(&walk, &place);
    // sluice withdraws the failed entry once the request is taken in
    // again, and the place never reads that withdrawal as its own.
    let states: Vec<Value> = walk
        .submissions()
        .iter()
        .map(|entry| entry["state"].clone())
        .collect();
    assert_eq!(states, vec![json!("withdrawn"), json!("landed")]);
    assert_eq!(walk.on_remote("broken"), None);
    assert!(!walk.has_branch("fix/six"));
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

#[test]
fn a_stage_waiting_for_room_starts_at_the_next_command_once_the_host_gives_room() {
    // On deneb a stage waiting for room still waited once the host's
    // configuration gave it room, until a settle by hand: the printed
    // command keeps no watch, so a place moves on only when a command of
    // flume's acts or an agent goes quiet (119). Status says so and
    // settles nothing; the next command that acts, a launch here, reads
    // the configuration as it stands and starts the waiting stage before
    // its own (46, 57, 70).
    let walk = Walk::new("walk-room-given");
    walk.configure("[host]\nagents = 1\n");
    let first = walk.launch("fix", "first", &["--brief", "one"]);
    let waiting = walk.launch("fix", "waiting", &["--brief", "two"]);
    assert!(
        !walk.status(&first)["agent"].is_null(),
        "the first did not start"
    );
    assert!(
        walk.status(&waiting)["agent"].is_null(),
        "the second launch started past the bound"
    );
    assert_eq!(walk.host()["watch"], json!("none"));
    let said = walk.person(&["status"]);
    assert!(
        said.contains(
            "the terminal keeps no watch: places move on when a command of flume's acts or an agent goes quiet"
        ),
        "{said}"
    );

    walk.configure("[host]\nagents = 2\n");
    let host = walk.host();
    assert_eq!(
        (host["running"].clone(), host["room"].clone()),
        (json!(1), json!(1)),
        "{host}"
    );
    assert!(
        walk.status(&waiting)["agent"].is_null(),
        "status started the waiting stage"
    );

    let third = walk.launch("fix", "third", &["--brief", "three"]);
    let started = walk.status(&waiting);
    assert!(!started["agent"]["printed"].is_null(), "{started}");
    assert!(
        walk.status(&third)["agent"].is_null(),
        "the launch's own stage took the room"
    );
    let host = walk.host();
    let queue: Vec<Value> = host["queue"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|queued| queued["place"].clone())
        .collect();
    assert_eq!(queue, vec![json!(third)], "{host}");
}

/// The host's configuration with the `default` stream's pool, the
/// stand-in's, holding a `codex` account alone.
const NO_CLAUDE: &str = "[streams.default]\npool = \"stand-in\"\n\n[pools.stand-in]\nstrategy = \"fill-first\"\naccounts = [{ name = \"codex-1\", program = \"codex\" }]\n";

#[test]
fn a_pool_with_no_account_of_a_stages_program_refuses_it_at_its_launch_and_at_a_later_start() {
    // S81, 4, 32, 145, under the printed command, every stage running
    // `claude`: a fix launched on a stream whose pool holds only `codex`
    // accounts is refused naming code, the pool and `claude`, with no
    // branch and no place made. A change on the stand-in's pool runs
    // construct, and while construct works the host's configuration leaves
    // that pool a `codex` account alone; once construct delivers, code is
    // refused naming the pool and `claude`, with no command printed for it
    // and the place waiting on its caller's retry. Once the configuration
    // is mended, a retry starts code under the stand-in's account, the
    // intent's for `claude`.
    let walk = Walk::new("walk-no-account");
    walk.configure("[streams.elsewhere]\npool = \"elsewhere\"\n\n[pools.elsewhere]\nstrategy = \"fill-first\"\naccounts = [\n  { name = \"codex-1\", program = \"codex\" },\n  { name = \"codex-2\", program = \"codex\" },\n]\n");
    let (answer, refused) = walk.flume_ran(&[
        "launch",
        "--kind",
        "fix",
        "--name",
        "one",
        "--brief",
        "fix the parser",
        "--stream",
        "elsewhere",
    ]);
    assert!(refused, "the launch was taken: {answer}");
    let reason = answer.to_string();
    assert!(
        reason.contains("stage code's")
            && reason.contains("pool elsewhere")
            && reason.contains("account of claude"),
        "the refusal names not code, the pool and claude: {reason}"
    );
    assert!(!walk.has_branch("fix/one"), "the fix's branch was made");
    assert_eq!(walk.everything()["places"], json!([]), "a place was made");
    assert_eq!(
        walk.repository.git(&["worktree", "list"]).lines().count(),
        1,
        "a worktree was made"
    );

    let place = walk.launch("change", "add-parser", &["--brief", "add a parser"]);
    assert_eq!(
        walk.status(&place)["agent"]["account"]["name"],
        "stand-in-1",
        "construct's account"
    );
    let dropped = walk.signal("dropped");
    let mut acts = vec![json!({ "wait_for": dropped })];
    acts.extend(
        construct("add-parser")
            .as_array()
            .cloned()
            .unwrap_or_default(),
    );
    let agent = walk.start_agent(&place, Value::Array(acts));
    walk.until(&place, "construct begun", |status| {
        stage(status, "construct")["state"] == "begun"
    });
    walk.configure_whole(NO_CLAUDE);
    std::fs::write(&dropped, "").expect("the signal");
    let ran = finish(agent);
    assert_eq!(ran.code, 0, "{}", ran.out);

    let status = walk.until(&place, "code refused", |status| {
        stage(status, "code")["state"] == "refused"
    });
    assert_eq!(stage(&status, "construct")["state"], "delivered");
    let code = stage(&status, "code");
    let words = code["refused_setting"]
        .as_str()
        .unwrap_or_else(|| panic!("no refusal given: {status}"));
    assert!(
        words.contains("stage code's")
            && words.contains("pool stand-in")
            && words.contains("account of claude"),
        "the refusal names not code, the pool and claude: {words}"
    );
    assert_eq!(code["starts"], 0, "{status}");
    assert!(status["agent"].is_null(), "a command was printed: {status}");
    assert_eq!(
        (
            &status["waiting_on"]["act"],
            &status["waiting_on"]["stage"],
            &status["waiting_on"]["why"]
        ),
        (&json!("retry"), &json!("code"), &json!("refused")),
        "{status}"
    );
    assert_eq!(walk.host()["queue"], json!([]), "code waits for room");

    walk.configure("");
    walk.flume(&["retry", &place]);
    let status = walk.status(&place);
    assert_eq!(
        status["agent"]["account"],
        json!({ "pool": "stand-in", "name": "stand-in-1", "from": "affinity" }),
        "{status}"
    );
    let code = stage(&status, "code");
    assert_eq!(code["starts"], 1, "{status}");
    assert!(code["refused_setting"].is_null(), "{status}");
    assert!(
        walk.printed(&place).contains("claude"),
        "code's printed command"
    );
}

/// The three task groups of the repeating walkthrough's change, two
/// tasks, one and two, none ticked.
const THREE_GROUPS: &str = "## 1. Parser\n\n- [ ] 1.1 Write the parser\n- [ ] 1.2 Test the parser\n\n## 2. Reader\n\n- [ ] 2.1 Read the input\n\n## 3. Docs\n\n- [ ] 3.1 Document the parser\n- [ ] 3.2 Note the change\n";

/// A code repetition's acts: its prompt kept in `prompt`, a line added to
/// the notes at the path its prompt names, `count` tasks ticked, and
/// committed, the notes left out of the place.
fn repetition(prompt: &Path, tasks: &str, count: usize, line: &str) -> Value {
    json!([
        { "bash": format!("printf '%s' \"$STAND_IN_PROMPT\" > '{}'", prompt.display()) },
        { "bash": format!("notes=$(printf '%s\\n' \"$STAND_IN_PROMPT\" | sed -n 's/^The notes: //p') && mkdir -p \"$(dirname \"$notes\")\" && printf '%s\\n' '{line}' >> \"$notes\"") },
        { "tick": { "path": tasks, "count": count } },
        { "bash": format!("git add -A && git commit --quiet -m '{line}'") },
    ])
}

/// The code stage's group in status: its place and how many there are.
fn group(status: &Value) -> (Value, Value) {
    let group = &stage(status, "code")["group"];
    (group["place"].clone(), group["of"].clone())
}

#[test]
fn a_change_of_three_task_groups_takes_three_agents_in_turn() {
    // S50, 140 to 142: change@2's code stage on a change of three groups
    // starts a fresh agent for each in turn; each ticks its group, adds to
    // the notes its prompt names and commits; status between them gives
    // code 2/3; the three prompts are byte for byte the same up to the
    // group; verify archives and the place lands.
    let walk = Walk::new("walk-groups");
    let place = walk.launch("change", "add-parser", &["--brief", "add a parser"]);
    assert_eq!(walk.status(&place)["version"], 2, "the change's version");
    walk.agent(&place, construct_tasks("add-parser", THREE_GROUPS));
    let tasks = "openspec/changes/add-parser/tasks.md";
    let mut prompts = vec![];
    for (n, count) in [(1, 2), (2, 1), (3, 2)] {
        let status = walk.until(&place, &format!("code on group {n}"), |status| {
            status["current"] == "code"
                && !status["agent"].is_null()
                && group(status) == (json!(n), json!(3))
        });
        assert_eq!(stage(&status, "code")["starts"], n, "{status}");
        if n == 2 {
            let output = walk
                .command(env!("CARGO_BIN_EXE_flume"), &walk.repository.dir)
                .args(["status", &place])
                .output()
                .expect("flume runs");
            let text = String::from_utf8_lossy(&output.stdout);
            assert!(text.contains("stage code 2/3:"), "{text}");
        }
        let prompt = walk.signal(&format!("prompt-{n}"));
        walk.agent(
            &place,
            repetition(&prompt, tasks, count, &format!("group {n} built")),
        );
        prompts.push(std::fs::read_to_string(&prompt).expect("the prompt the stand-in was sent"));
    }
    let status = walk.until(&place, "its verify stage", |status| {
        status["current"] == "verify" && !status["agent"].is_null()
    });
    assert_eq!(stage(&status, "code")["starts"], 3, "{status}");

    let at = |prompt: &str, heading: &str| {
        prompt
            .find(heading)
            .unwrap_or_else(|| panic!("{heading} is not in {prompt}"))
    };
    let first = at(&prompts[0], "Group 1. Parser");
    for (prompt, heading) in [
        (&prompts[1], "Group 2. Reader"),
        (&prompts[2], "Group 3. Docs"),
    ] {
        let from = at(prompt, heading);
        assert_eq!(from, first, "where the group begins");
        assert_eq!(
            &prompt[..from],
            &prompts[0][..first],
            "the prompts up to the group"
        );
    }
    let notes = prompts[0]
        .lines()
        .find_map(|line| line.strip_prefix("The notes: "))
        .expect("the notes' path");
    assert_eq!(
        std::fs::read_to_string(notes).expect("the notes"),
        "group 1 built\ngroup 2 built\ngroup 3 built\n",
        "the notes as the three repetitions left them"
    );

    walk.agent(&place, verify_holds("add-parser"));
    until_landed(&walk, &place);
    assert!(!walk.has_branch("change/add-parser"));
    assert!(!Path::new(notes).exists(), "the notes outlive the place");
    let files = walk.remote_files();
    assert!(
        files.iter().all(|file| !file.contains("notes/")),
        "the notes were committed: {files:?}"
    );
}

#[test]
fn a_failed_submission_sends_a_repeating_stage_back_for_one_repetition_with_no_group() {
    // S52, 112, 140: the gate fails the change's first submission with
    // every task ticked; code starts one repetition with no group and
    // sluice's words, which moves the change out of the archive, fixes
    // the tree and records the fix as a task ticked; verify archives
    // again and the second submission lands.
    let walk = Walk::new("walk-groups-failed");
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
            { "write": { "path": "broken", "text": "not yet\n" } },
            { "tick": { "path": tasks } },
            { "bash": "git add -A && git commit --quiet -m 'the parser'" },
        ]),
    );
    walk.until(&place, "its verify stage", |status| {
        status["current"] == "verify" && !status["agent"].is_null()
    });
    walk.agent(&place, verify_holds("add-parser"));
    let status = walk.until(&place, "its code stage started again", |status| {
        status["current"] == "code"
            && stage(status, "code")["starts"] == 2
            && !status["agent"].is_null()
    });
    assert_eq!(status["submission"]["send_backs"], 1, "{status}");
    assert!(stage(&status, "code")["group"].is_null(), "{status}");
    let prompt = walk.prompt(&place);
    assert!(
        prompt.contains("There is no task group for you"),
        "{prompt}"
    );
    assert!(prompt.contains("[sluice] #1 failed on main"), "{prompt}");
    walk.agent(
        &place,
        json!([
            { "bash": "git mv openspec/changes/archive/*-add-parser openspec/changes/add-parser" },
            { "bash": format!("git rm --quiet broken && printf -- '- [x] 1.3 Pass the gate\\n' >> {tasks}") },
            { "bash": "git add -A && git commit --quiet -m 'the parser test passes'" },
        ]),
    );
    let status = walk.until(&place, "its verify stage again", |status| {
        status["current"] == "verify" && !status["agent"].is_null()
    });
    assert_eq!(stage(&status, "code")["starts"], 2, "{status}");
    walk.agent(&place, verify_archives("add-parser", " --skip-specs"));
    until_landed(&walk, &place);
    let states: Vec<Value> = walk
        .submissions()
        .iter()
        .map(|entry| entry["state"].clone())
        .collect();
    assert_eq!(states.len(), 2, "{states:?}");
    assert!(states.contains(&json!("landed")), "{states:?}");
    assert_eq!(walk.on_remote("broken"), None);
    assert!(walk.on_remote("parser.rs").is_some());
}

/// The words a carried start's prompt opens with, flume's own (146, 179).
const CARRIED: &str = "Your last turn stopped at your account's usage limit. This session now goes on under an account of the pool, with its conversation as it stood: carry on from where you stopped.";

/// The way to stop short the Claude Code profile gives every start (28).
const STOP_SHORT: &str = "run `flume needs \"<words>\"` when you cannot go on without an answer, saying what you need, or `flume needs --found \"<words>\"` when you found work this stage cannot deliver past, saying what you found; then stop.";

/// The host's fill-first pool of two accounts, `stand-in-1` and
/// `stand-in-2`, whose `CLAUDE_CONFIG_DIR`s are `first` and `second`.
fn two_accounts(walk: &Walk, first: &Path, second: &Path) {
    walk.configure_whole(&format!(
        "[host]\npool = \"stand-in\"\n\n[pools.stand-in]\nstrategy = \"fill-first\"\naccounts = [\n  {{ name = \"stand-in-1\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?} }} }},\n  {{ name = \"stand-in-2\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?} }} }},\n]\n",
        first.display().to_string(),
        second.display().to_string(),
    ));
}

/// One thing a configuration home holds.
#[derive(Debug, PartialEq)]
enum Holding {
    Folder,
    File(String),
    Link(PathBuf),
}

/// What a configuration home holds, links not followed: each path below
/// it, with a file's text and a link's target.
fn holdings(home: &Path) -> BTreeMap<PathBuf, Holding> {
    let mut held = BTreeMap::new();
    let mut folders = vec![home.to_path_buf()];
    while let Some(folder) = folders.pop() {
        for entry in std::fs::read_dir(&folder)
            .expect("a home's folder")
            .flatten()
        {
            let path = entry.path();
            let below = path
                .strip_prefix(home)
                .expect("below the home")
                .to_path_buf();
            let kind = std::fs::symlink_metadata(&path)
                .expect("a home's entry")
                .file_type();
            let holding = if kind.is_symlink() {
                Holding::Link(std::fs::read_link(&path).expect("a link's target"))
            } else if kind.is_dir() {
                folders.push(path);
                Holding::Folder
            } else {
                Holding::File(std::fs::read_to_string(&path).expect("a home's file"))
            };
            held.insert(below, holding);
        }
    }
    held
}

/// A home as an account's holds it before any agent ran under it: its
/// credentials, a file of the walk's own, which no agent and no command
/// of flume's may touch; the holdings it gives are read after.
fn signed_in(home: &Path, account: &str) -> BTreeMap<PathBuf, Holding> {
    std::fs::write(
        home.join(".credentials.json"),
        format!("{{\"stand-in\": \"the credentials of {account}\"}}\n"),
    )
    .expect("the account's credentials");
    holdings(home)
}

/// The sessions the stand-in said it ran, in the order it ran them.
fn sessions(out: &str) -> Vec<String> {
    out.lines()
        .filter_map(|line| line.strip_prefix("stand-in: session "))
        .filter_map(|rest| rest.split_once(" in "))
        .map(|(session, _)| session.to_string())
        .collect()
}

/// `home` holds what it held, `held`, as it held it, and besides only the
/// transcripts the stand-in wrote of `sessions`, under `projects`, with
/// their folders: flume copied, linked and wrote nothing there (179, C.4;
/// core.32).
fn holds_only(home: &Path, held: &BTreeMap<PathBuf, Holding>, sessions: &[String]) {
    let now = holdings(home);
    for (path, holding) in held {
        assert_eq!(
            now.get(path),
            Some(holding),
            "{} changed in {}",
            path.display(),
            home.display()
        );
    }
    for (path, holding) in now.iter().filter(|(path, _)| !held.contains_key(*path)) {
        let names: Vec<String> = path
            .components()
            .map(|name| name.as_os_str().to_string_lossy().into_owned())
            .collect();
        let transcript = match names.as_slice() {
            [projects, _, file] if projects == "projects" => {
                file.strip_suffix(".jsonl")
                    .is_some_and(|session| sessions.iter().any(|ran| ran == session))
                    && matches!(holding, Holding::File(_))
            }
            [projects] | [projects, _] if projects == "projects" => holding == &Holding::Folder,
            _ => false,
        };
        assert!(
            transcript,
            "{} holds {} ({holding:?}), which no session of {sessions:?} wrote",
            home.display(),
            path.display()
        );
    }
}

#[test]
fn a_spent_account_moves_the_stage_to_the_pools_next_and_the_fix_lands() {
    // S55, 146, 151, 179: on a fill-first pool of two accounts, each a
    // scratch `CLAUDE_CONFIG_DIR` of its own, the fix's code agent starts
    // under the first and its turn fails at the session limit through
    // StopFailure. The first account reads spent until the hour the words
    // name, code starts afresh under the second with flume's words, the
    // move recorded with its cache lost and its conversation stayed
    // behind, since the second's home does not reach it, said naming
    // `CLAUDE_CONFIG_DIR` and no path; the second's agent commits what the
    // first left, and the place lands. Each home holds what it held and the
    // transcript its agent wrote, and nothing else (C.4; core.32).
    let walk = Walk::new("walk-spent");
    let first = walk.account().to_path_buf();
    let second = walk.account_folder("account-2");
    let held = [
        signed_in(&first, "stand-in-1"),
        signed_in(&second, "stand-in-2"),
    ];
    two_accounts(&walk, &first, &second);

    let place = walk.launch("fix", "one", &["--brief", "fix the parser"]);
    let status = walk.status(&place);
    assert_eq!(status["agent"]["account"]["name"], "stand-in-1", "{status}");
    let (reset, limit) = session_limit_after(now());
    let out = walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "fail": { "error": "rate_limit", "words": limit } },
        ]),
    );
    assert!(out.contains("the turn failed"), "{out}");
    assert!(!out.contains("stop blocked"), "{out}");
    let mut ran = sessions(&out);

    let status = walk.until(&place, "code started afresh", |status| {
        stage(status, "code")["starts"] == 2 && !status["agent"].is_null()
    });
    assert_eq!(
        status["agent"]["account"],
        json!({ "pool": "stand-in", "name": "stand-in-2", "from": "strategy" }),
        "{status}"
    );
    assert_eq!(
        status["agent"]["conversation"],
        json!({ "from": 1, "carried": false, "why": UNREACHED }),
        "{status}"
    );
    assert!(UNREACHED.contains("`CLAUDE_CONFIG_DIR`"), "{UNREACHED}");
    for home in [&first, &second] {
        assert!(
            !status.to_string().contains(&*home.to_string_lossy()),
            "status holds a configuration home's path: {status}"
        );
    }
    let moved = status["moves"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|moved| moved["on"] == "stage")
        .unwrap_or_else(|| panic!("no move of code: {status}"));
    assert_eq!(
        (
            &moved["stage"],
            &moved["from"],
            &moved["to"],
            &moved["why"],
            &moved["cache"],
            &moved["carried"]
        ),
        (
            &json!("code"),
            &json!("stand-in-1"),
            &json!("stand-in-2"),
            &json!("spent"),
            &json!("lost"),
            &json!(false)
        ),
        "{status}"
    );
    assert!(stage(&status, "code")["end"].is_null(), "{status}");
    assert_eq!(walk.resumes(&place), None, "{}", walk.printed(&place));
    let prompt = walk.prompt(&place);
    assert!(
        prompt.contains("stopped at its account's usage limit"),
        "flume's words: {prompt}"
    );
    assert!(
        prompt.contains("fix the parser"),
        "the stage's own: {prompt}"
    );
    let host = walk.host();
    let accounts = &host["pools"][0]["accounts"];
    assert_eq!(accounts[0]["name"], "stand-in-1", "{host}");
    assert_eq!(accounts[0]["spent_until"], reset, "{host}");
    assert_eq!(accounts[0]["running"], 0, "{host}");
    assert_eq!(accounts[1]["name"], "stand-in-2", "{host}");
    assert!(accounts[1]["spent_until"].is_null(), "{host}");
    assert_eq!(accounts[1]["running"], 1, "{host}");

    let out = walk.agent(
        &place,
        json!([{ "bash": "git add -A && git commit --quiet -m 'fix the parser'" }]),
    );
    ran.extend(sessions(&out));
    assert_eq!(ran.len(), 2, "a session each: {ran:?}");
    assert_ne!(ran[0], ran[1], "the second agent's session is its own");
    until_landed(&walk, &place);
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n"),
        "what the first agent left, committed by the second"
    );
    assert!(!walk.has_branch("fix/one"));
    for (account, held) in [&first, &second].into_iter().zip(&held) {
        assert!(
            account.join("projects").exists(),
            "an agent under {}",
            account.display()
        );
        holds_only(account, held, &ran);
    }
}

#[test]
fn a_spent_agents_conversation_goes_on_under_the_pools_next_and_the_fix_lands() {
    // S78, 146, 179: on a fill-first pool of two accounts, each a scratch
    // `CLAUDE_CONFIG_DIR`, the second's `projects` a link to the first's,
    // the fix's code agent starts under the first and its turn fails at
    // the session limit through StopFailure. Code starts again under the
    // second, its printed command resuming the first's session, and its
    // prompt flume's carry words and the way to stop short, nothing of the
    // stage's own; status gives the conversation carried from start 1 and
    // the move with it. The resumed session goes on in the first's
    // transcript, commits what the first left, and the place lands. Each
    // home holds what it held and the transcript the stand-in wrote, the
    // link as the host made it, and nothing else (C.4; core.32).
    let walk = Walk::new("walk-carried");
    let first = walk.account().to_path_buf();
    let second = walk.account_folder("account-2");
    std::fs::create_dir_all(first.join("projects")).expect("the first's projects");
    std::os::unix::fs::symlink(first.join("projects"), second.join("projects"))
        .expect("the second's projects, a link to the first's");
    let held = [
        signed_in(&first, "stand-in-1"),
        signed_in(&second, "stand-in-2"),
    ];
    two_accounts(&walk, &first, &second);

    let place = walk.launch("fix", "one", &["--brief", "fix the parser"]);
    let status = walk.status(&place);
    assert_eq!(status["agent"]["account"]["name"], "stand-in-1", "{status}");
    assert!(status["agent"]["conversation"].is_null(), "{status}");
    let (_, limit) = session_limit_after(now());
    let out = walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "fail": { "error": "rate_limit", "words": limit } },
        ]),
    );
    assert!(out.contains("the turn failed"), "{out}");
    let ran = sessions(&out);
    assert_eq!(ran.len(), 1, "{out}");

    let status = walk.until(&place, "code started again", |status| {
        stage(status, "code")["starts"] == 2 && !status["agent"].is_null()
    });
    assert_eq!(
        status["agent"]["account"],
        json!({ "pool": "stand-in", "name": "stand-in-2", "from": "strategy" }),
        "{status}"
    );
    assert_eq!(
        status["agent"]["conversation"],
        json!({ "from": 1, "carried": true }),
        "{status}"
    );
    let moved = status["moves"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|moved| moved["on"] == "stage")
        .unwrap_or_else(|| panic!("no move of code: {status}"));
    assert_eq!(
        (
            &moved["stage"],
            &moved["from"],
            &moved["to"],
            &moved["why"],
            &moved["cache"],
            &moved["carried"]
        ),
        (
            &json!("code"),
            &json!("stand-in-1"),
            &json!("stand-in-2"),
            &json!("spent"),
            &json!("lost"),
            &json!(true)
        ),
        "{status}"
    );
    for home in [&first, &second] {
        assert!(
            !status.to_string().contains(&*home.to_string_lossy()),
            "status holds a configuration home's path: {status}"
        );
    }
    let person = walk.person(&["status"]);
    assert!(
        person.contains("carries the conversation of start 1")
            && person.contains("its conversation with it"),
        "{person}"
    );
    assert_eq!(
        walk.resumes(&place).as_deref(),
        Some(ran[0].as_str()),
        "{}",
        walk.printed(&place)
    );
    let prompt = walk.prompt(&place);
    assert_eq!(prompt, format!("{CARRIED}\n\n{STOP_SHORT}"));

    let out = walk.agent(
        &place,
        json!([{ "bash": "git add -A && git commit --quiet -m 'fix the parser'" }]),
    );
    assert!(out.contains(", resumed"), "{out}");
    assert_eq!(sessions(&out), ran, "the first's session resumed: {out}");
    let transcripts: Vec<PathBuf> = std::fs::read_dir(first.join("projects"))
        .expect("the transcripts' folder")
        .flatten()
        .flat_map(|folder| {
            std::fs::read_dir(folder.path())
                .into_iter()
                .flatten()
                .flatten()
        })
        .map(|file| file.path())
        .collect();
    assert_eq!(transcripts.len(), 1, "{transcripts:?}");
    let text = std::fs::read_to_string(&transcripts[0]).expect("the transcript");
    let turns: Vec<Value> = text
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("a JSON record"))
        .filter(|record| record["type"] == "user" && record["message"]["content"].is_string())
        .map(|record| record["message"]["content"].clone())
        .collect();
    assert_eq!(turns.len(), 2, "the stage's prompt, then flume's: {text}");
    assert!(
        turns[0]
            .as_str()
            .is_some_and(|turn| turn.contains("fix the parser")),
        "{text}"
    );
    assert_eq!(turns[1], json!(prompt), "{text}");

    until_landed(&walk, &place);
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n"),
        "what the first agent left, committed in its own conversation"
    );
    assert!(!walk.has_branch("fix/one"));
    for (account, held) in [&first, &second].into_iter().zip(&held) {
        holds_only(account, held, &ran);
    }
}

#[test]
fn a_start_without_its_account_is_ended_and_its_stage_started_afresh() {
    // S65, 161: the fix's code stage starts under the pool's account, its
    // `CLAUDE_CONFIG_DIR` a scratch folder, and its program comes up
    // without the variable, as a pane brought back without its
    // environment does. Its SessionStart hook acks nothing and records the
    // variable missing; flume ends that agent, under the printed command
    // once its session ended, and starts code afresh under the account,
    // whose agent runs with the variable, delivers, and the place lands.
    // Status names `CLAUDE_CONFIG_DIR` and no value.
    let walk = Walk::new("walk-unsettled");
    let account = walk.account().to_path_buf();
    let place = walk.launch("fix", "one", &["--brief", "fix the parser"]);
    let out = walk.agent(&place, json!([{ "unset": "CLAUDE_CONFIG_DIR" }]));
    assert!(!out.contains("stop blocked"), "{out}");

    let status = walk.until(&place, "code started afresh", |status| {
        stage(status, "code")["starts"] == 2 && !status["agent"].is_null()
    });
    let unsettled = status["unsettled"].as_array().cloned().unwrap_or_default();
    assert_eq!(unsettled.len(), 1, "{status}");
    assert_eq!(
        (
            &unsettled[0]["stage"],
            &unsettled[0]["start"],
            &unsettled[0]["missing"],
            &unsettled[0]["different"],
            &unsettled[0]["program"]
        ),
        (
            &json!("code"),
            &json!(1),
            &json!(["CLAUDE_CONFIG_DIR"]),
            &json!([]),
            &json!(false)
        ),
        "{status}"
    );
    assert!(
        !status.to_string().contains(&*account.to_string_lossy()),
        "status holds the variable's value: {status}"
    );
    assert_eq!(status["agent"]["account"]["name"], "stand-in-1", "{status}");
    assert!(stage(&status, "code")["end"].is_null(), "{status}");
    assert!(status["waiting_on"].is_null(), "{status}");
    let prompt = walk.prompt(&place);
    assert!(
        prompt.contains("began without the settings its start recorded"),
        "flume's words: {prompt}"
    );

    walk.agent(&place, commits("parser.rs", "fn parse() {}\n"));
    until_landed(&walk, &place);
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n")
    );
    assert!(
        account.join("projects").exists(),
        "the fresh agent ran under the account's CLAUDE_CONFIG_DIR"
    );
}

/// The repository's binding with `more` beside it, committed and pushed
/// to the remote, where the base lives.
fn the_repository_gives(walk: &Walk, more: &str) {
    let file = walk.repository.dir.join(".config/flume.toml");
    let binding = std::fs::read_to_string(&file).expect("the binding");
    walk.repository
        .write(".config/flume.toml", &format!("{binding}\n{more}"));
    walk.repository.commit("the repository's settings");
    walk.repository.git(&["push", "--quiet", "origin", "main"]);
}

#[test]
fn a_hosts_mode_over_the_repositorys_and_none_where_no_layer_gives_one() {
    // S66, 162: the host's `[kinds.fix.agents]` gives `edits`, while the
    // repository's `.config/flume.toml` gives `full`; the host's table
    // comes first, so the printed command passes `--permission-mode
    // acceptEdits`, the started ack is taken on the program and the
    // variables, the stand-in's first prompt names `acceptEdits` and is
    // taken as received, so code begins, status names `edits` from
    // `host-kind` passed as `acceptEdits`, and the fix lands.
    let walk = Walk::new("walk-mode");
    walk.configure("[kinds.fix.agents]\nmode = \"edits\"\n");
    the_repository_gives(&walk, "[kinds.fix.agents]\nmode = \"full\"\n");
    let place = walk.launch("fix", "one", &["--brief", "fix the parser"]);
    let printed = walk.printed(&place);
    assert!(
        printed.contains("--permission-mode acceptEdits"),
        "the printed command: {printed}"
    );
    let status = walk.status(&place);
    assert_eq!(
        (
            &status["agent"]["settings"]["from"]["mode"],
            &status["agent"]["mode"]
        ),
        (
            &json!({ "layer": "host-kind", "value": "edits" }),
            &json!({ "passed": "acceptEdits", "entry": null, "read_back": true })
        ),
        "{status}"
    );
    let signal = walk.signal("mode-acked");
    let mut acts = vec![json!({ "wait_for": signal })];
    acts.extend(
        commits("parser.rs", "fn parse() {}\n")
            .as_array()
            .cloned()
            .unwrap_or_default(),
    );
    let agent = walk.start_agent(&place, Value::Array(acts));
    let status = walk.until(&place, "code begun", |status| {
        stage(status, "code")["state"] == "begun"
    });
    assert_eq!(status["agent"]["started"], "yes", "{status}");
    assert!(
        status["unsettled"].as_array().is_none_or(Vec::is_empty),
        "{status}"
    );
    std::fs::write(&signal, "").expect("the signal");
    let ran = finish(agent);
    assert_eq!(ran.code, 0, "the stand-in in {place}: {}", ran.out);
    until_landed(&walk, &place);
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n")
    );

    // 162: no layer gives a mode, so none is passed, the start records
    // none, the account's own configuration decides, and the fix lands.
    let walk = Walk::new("walk-no-mode");
    let place = walk.launch("fix", "one", &["--brief", "fix the parser"]);
    let printed = walk.printed(&place);
    assert!(
        !printed.contains("--permission-mode"),
        "the printed command: {printed}"
    );
    let status = walk.status(&place);
    assert!(status["agent"]["mode"].is_null(), "{status}");
    assert!(
        status["agent"]["settings"]["from"]["mode"].is_null(),
        "{status}"
    );
    walk.agent(&place, commits("parser.rs", "fn parse() {}\n"));
    until_landed(&walk, &place);
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n")
    );
}

#[test]
fn a_fix_begun_in_another_mode_twice_is_never_begun() {
    // S67, 161, 162: a fix launched with `--agent mode=full`, which the
    // start passes as `bypassPermissions`, whose session begins in
    // `default`, as Claude Code begins where the account's configuration
    // disables the mode asked. Its SessionStart acks it started, and its
    // first prompt, naming `default`, is blocked, so it takes no turn;
    // flume ends that agent once its session ended and starts code
    // afresh, whose session begins in `default` again: code is never
    // begun, status and its end naming `bypassPermissions` asked and
    // `manual` begun.
    let walk = Walk::new("walk-other-mode");
    let place = walk.launch(
        "fix",
        "one",
        &["--brief", "fix the parser", "--agent", "mode=full"],
    );
    let printed = walk.printed(&place);
    assert!(
        printed.contains("--permission-mode bypassPermissions"),
        "the printed command: {printed}"
    );
    let out = walk.agent(&place, json!([{ "begin_in": "default" }]));
    assert!(out.contains("prompt blocked"), "{out}");
    assert!(!out.contains("stop blocked"), "{out}");
    let both = json!({ "asked": "bypassPermissions", "began": "manual" });

    let status = walk.until(&place, "code started afresh", |status| {
        stage(status, "code")["starts"] == 2 && !status["agent"].is_null()
    });
    let unsettled = status["unsettled"].as_array().cloned().unwrap_or_default();
    assert_eq!(
        unsettled
            .iter()
            .map(|read| (&read["start"], &read["mode"], &read["missing"]))
            .collect::<Vec<_>>(),
        vec![(&json!(1), &both, &json!([]))],
        "{status}"
    );
    assert!(stage(&status, "code")["end"].is_null(), "{status}");
    assert!(
        walk.printed(&place)
            .contains("--permission-mode bypassPermissions"),
        "the fresh start's command"
    );

    walk.agent(&place, json!([{ "begin_in": "default" }]));
    let status = walk.until(&place, "code never begun", |status| {
        stage(status, "code")["state"] == "never_begun"
    });
    let code = stage(&status, "code");
    assert_eq!(code["starts"], 2, "{status}");
    assert_eq!(code["end"]["unsettled"]["mode"], both, "{status}");
    let unsettled = status["unsettled"].as_array().cloned().unwrap_or_default();
    assert_eq!(
        unsettled
            .iter()
            .map(|read| (&read["start"], &read["mode"]))
            .collect::<Vec<_>>(),
        vec![(&json!(1), &both), (&json!(2), &both)],
        "{status}"
    );
    assert!(
        walk.on_remote("parser.rs").is_none(),
        "a fix never begun landed"
    );
}

/// A kind of one stage that does not land, its start read never begun a
/// second after it starts with no word that it started.
const QUICK: &str = r#"[kinds.quick]
version = 1
branch = "quick"
starts_unready = true
lands = false

[[kinds.quick.stages]]
name = "code"
prompt = "prompts/work.md"
deliverables = ["commits"]
agents = [{}]
quiet = "1s"
"#;

#[test]
fn an_agent_run_after_its_start_was_read_never_begun_is_begun_and_a_retry_leaves_it() {
    // On deneb a stage's start was read never begun while its agent sat at
    // a dialog; a person answered it and the agent began, but flume still
    // held the stage never begun, and the operator's retry ended the
    // working session. Here the printed command is run only once the
    // start was read never begun at the stage's limit: the stand-in acks
    // its start and its prompt through the plugin's hooks, flume reads the
    // stage begun, and a retry, forced or not, is refused naming the agent
    // and the stop that ends it first. The agent then delivers (35, 38,
    // 43, 50).
    let walk = Walk::new("walk-began-late");
    let kinds = walk.repository.scratch.path("kinds/kinds.toml");
    std::fs::create_dir_all(kinds.parent().expect("a folder").join("prompts"))
        .expect("the kinds' folder");
    std::fs::write(&kinds, QUICK).expect("the kinds");
    std::fs::write(
        walk.repository.scratch.path("kinds/prompts/work.md"),
        "Do the work of {{name}}: {{brief}}. To stop short: {{needs}}\n",
    )
    .expect("the prompt");
    let place = walk.launch(
        "quick",
        "one",
        &[
            "--brief",
            "fix the parser",
            "--kinds",
            &kinds.to_string_lossy(),
        ],
    );
    let status = walk.until(&place, "never begun", |status| {
        stage(status, "code")["state"] == "never_begun"
    });
    assert_eq!(status["waiting_on"]["why"], "never_begun", "{status}");
    let identity = status["agent"]["identity"]
        .as_str()
        .unwrap_or_else(|| panic!("the never-begun start's agent: {status}"))
        .to_string();

    let midway = walk.signal("midway");
    let started = walk.signal("started");
    let agent = walk.start_agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "bash": "git add parser.rs && git commit --quiet -m 'the parser'" },
            { "bash": format!("touch '{}'", started.display()) },
            { "wait_for": midway },
        ]),
    );
    let began = std::time::Instant::now();
    while !started.exists() {
        assert!(
            began.elapsed() < std::time::Duration::from_secs(60),
            "the stand-in never came to its commit"
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let status = walk.until(&place, "begun", |status| {
        stage(status, "code")["state"] == "begun"
    });
    assert!(status["waiting_on"].is_null(), "{status}");
    assert!(stage(&status, "code")["end"].is_null(), "{status}");
    assert_eq!(status["agent"]["identity"], identity.as_str(), "{status}");
    for words in [&["retry", &place][..], &["retry", &place, "--force"][..]] {
        let (answer, refused) = walk.flume_ran(words);
        assert!(refused, "flume {} was taken: {answer}", words.join(" "));
        let reason = answer.to_string();
        assert!(
            reason.contains(&identity) && reason.contains("flume stop"),
            "the refusal names not the agent and its stop: {reason}"
        );
    }
    assert_eq!(stage(&walk.status(&place), "code")["starts"], 1);

    std::fs::write(&midway, "").expect("the signal");
    let ran = finish(agent);
    assert_eq!(ran.code, 0, "{}", ran.out);
    let status = walk.until(&place, "delivered", |status| {
        stage(status, "code")["state"] == "delivered"
    });
    assert_eq!(stage(&status, "code")["starts"], 1, "{status}");
}

/// Writes acme's `review@<version>` in `kinds_dir`: one code stage whose
/// agent commits, in a kind that lands, its prompt naming its version;
/// gives its hash as the kinds profile reads it (156, 157, 159).
fn review_package(kinds_dir: &Path, version: u32) -> String {
    let dir = kinds_dir.join(format!("review@{version}"));
    std::fs::create_dir_all(dir.join("prompts")).expect("the package's folder");
    std::fs::write(
        dir.join("kind.toml"),
        format!(
            "[kinds.review]\nversion = {version}\nbranch = \"review\"\nstarts_unready = true\nlands = true\n\n\
             [[kinds.review.stages]]\nname = \"code\"\nprompt = \"prompts/code.md\"\n\
             deliverables = [\"commits\"]\nagents = [{{}}]\n"
        ),
    )
    .expect("the package's declaration");
    std::fs::write(
        dir.join("prompts/code.md"),
        format!(
            "You are acme's review@{version}, the {{{{kind}}}} {{{{name}}}} in {{{{place}}}} on {{{{branch}}}}.\n\n{{{{brief}}}}\n\nCommit the work on {{{{branch}}}}.\n"
        ),
    )
    .expect("the package's prompt");
    match Weir.read_package("acme", &kinds_dir.to_string_lossy(), "review", version) {
        PackageRead::Read { package } => package.hash,
        PackageRead::CouldNotRead { reason } => panic!("review@{version} not read: {reason}"),
    }
}

/// Declares the host's source acme at `kinds_dir`, as a program that
/// manages the host does: its file written beside under a name flume
/// does not read, then renamed over `acme.toml` (191, C.7; design 12).
fn declare_acme(walk: &Walk, kinds_dir: &Path) -> PathBuf {
    let sources = walk.home().join("config/flume/sources");
    std::fs::create_dir_all(&sources).expect("the sources directory");
    let written = sources.join(".acme.toml.tmp");
    std::fs::write(
        &written,
        format!("kinds_dir = \"{}\"\n", kinds_dir.display()),
    )
    .expect("the source's file");
    let file = sources.join("acme.toml");
    std::fs::rename(&written, &file).expect("the source's file renamed over");
    file
}

/// Launches `kind` as the place `review--<name>` and gives its location,
/// as the launch answers it.
fn launch_review(walk: &Walk, kind: &str, name: &str) -> String {
    let launched = walk.flume(&[
        "launch",
        "--kind",
        kind,
        "--name",
        name,
        "--brief",
        &format!("Review the parser for {name}."),
    ]);
    launched["launched"]["place"]["place"]
        .as_str()
        .unwrap_or_else(|| panic!("a place launched: {launched}"))
        .to_string()
}

#[test]
fn a_hosts_source_runs_its_locked_kind_and_a_place_keeps_its_version() {
    // Design 13; 21, 154, 155, 159, 191: the host's source acme holds
    // review@1, locked by `flume kinds lock --host --hash`; a launch of
    // `acme/review` runs it, status naming acme/review@1 from the
    // source's file and the kind's directory with its hash, locked in the
    // host's lock. review@2 is then added and locked, and `acme.toml`
    // renamed over by one naming a directory holding review@2 alone: the
    // next launch of `acme/review` runs review@2, one of `acme/review@1`
    // is refused, and the place of review@1 launched before runs on with
    // its own prompt to its landing, as does the place of review@2.
    let walk = Walk::new("walk-source");
    let first = walk.repository.scratch.path("packages/acme-1");
    let one_hash = review_package(&first, 1);
    let acme = declare_acme(&walk, &first);
    let lock = walk.home().join("config/flume/flume.lock");
    let locked = walk.flume(&[
        "kinds",
        "lock",
        "--host",
        "--hash",
        &one_hash,
        "acme/review@1",
    ]);
    assert_eq!(
        locked["locked"]["lock_file"],
        lock.to_string_lossy().as_ref(),
        "{locked}"
    );

    let one = launch_review(&walk, "acme/review", "one");
    let status = walk.status(&one);
    assert_eq!(status["identity"], "acme/review@1", "{status}");
    assert_eq!(status["declared"], "host", "{status}");
    assert_eq!(
        status["kind_from"],
        json!({
            "identity": {"source": "acme", "name": "review", "version": 1, "spelled": "acme/review@1"},
            "source": "acme",
            "source_file": acme.to_string_lossy(),
            "kind_dir": first.join("review@1").to_string_lossy(),
            "hash": one_hash,
            "lock_file": lock.to_string_lossy(),
        }),
        "{status}"
    );
    let person = walk.person(&["status", &one]);
    assert!(
        person.contains(&format!(
            "  kind acme/review@1 from the source acme ({}) at {}, its hash {one_hash}, locked in {}",
            acme.display(),
            first.join("review@1").display(),
            lock.display()
        )),
        "{person}"
    );
    assert!(
        walk.prompt(&one).contains("You are acme's review@1"),
        "{}",
        walk.prompt(&one)
    );

    let second = walk.repository.scratch.path("packages/acme-2");
    let two_hash = review_package(&second, 2);
    assert_eq!(declare_acme(&walk, &second), acme);
    walk.flume(&[
        "kinds",
        "lock",
        "--host",
        "--hash",
        &two_hash,
        "acme/review@2",
    ]);
    let sources = &walk.host()["sources"];
    assert_eq!(
        sources,
        &json!([{
            "name": "acme",
            "source_file": acme.to_string_lossy(),
            "kinds_dir": second.to_string_lossy(),
            "kinds": ["acme/review@2"],
        }]),
        "{sources}"
    );
    let two = launch_review(&walk, "acme/review", "two");
    let status = walk.status(&two);
    assert_eq!(status["identity"], "acme/review@2", "{status}");
    assert_eq!(status["kind_from"]["hash"], two_hash.as_str(), "{status}");
    assert_eq!(
        status["kind_from"]["kind_dir"],
        second.join("review@2").to_string_lossy().as_ref(),
        "{status}"
    );
    assert!(
        walk.prompt(&two).contains("You are acme's review@2"),
        "{}",
        walk.prompt(&two)
    );
    let (refused, was) = walk.flume_ran(&[
        "launch",
        "--kind",
        "acme/review@1",
        "--name",
        "three",
        "--brief",
        "Review it again.",
    ]);
    assert!(was, "{refused}");
    assert!(
        refused["refused"]["refusal"]["reason"]
            .as_str()
            .is_some_and(|reason| reason.contains("the source acme holds no acme/review@1")),
        "{refused}"
    );

    let status = walk.status(&one);
    assert_eq!(status["identity"], "acme/review@1", "{status}");
    assert_eq!(
        status["kind_from"]["source_file"],
        acme.to_string_lossy().as_ref()
    );
    assert_eq!(
        status["kind_from"]["kind_dir"],
        first.join("review@1").to_string_lossy().as_ref(),
        "the directory as the launch read it"
    );
    assert!(walk.prompt(&one).contains("You are acme's review@1"));
    walk.agent(&one, commits("one.rs", "fn one() {}\n"));
    until_landed(&walk, &one);
    assert_eq!(walk.on_remote("one.rs").as_deref(), Some("fn one() {}\n"));
    walk.agent(&two, commits("two.rs", "fn two() {}\n"));
    until_landed(&walk, &two);
    assert_eq!(walk.on_remote("two.rs").as_deref(), Some("fn two() {}\n"));
    assert!(!Path::new(&one).exists() && !Path::new(&two).exists());
}

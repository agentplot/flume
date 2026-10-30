//! The walkthroughs (design 18): flume's built binary under the printed
//! command, on a scratch repository bound to `git-worktree`, `openspec`
//! and `sluice`, landing on a local bare remote, each agent the stand-in
//! for Claude Code run as a person runs the printed command, and the test
//! settling as a caller would between its commands. A kind runs through
//! on its own: a fix lands through flume's own submission (S1); a failed
//! submission and a conflicting one each go back to code with sluice's
//! words and land on the second (112); a change runs construct, code and
//! verify, its report committed in the archived change (S2, S35); verify
//! finding work sends the place back to code (131); a fix whose prompt
//! Claude Code submits inside paste tags lands (89); a host's bound
//! queues a launch and starts the waiting ones by priority (S37); and
//! change@2's code stage works a change of three task groups one fresh
//! agent at a time, and a failed submission sends it back for one
//! repetition with no group (S50, S52; 112, 140 to 142); and a fix whose
//! code agent stops at its account's usage limit moves to the pool's
//! second account and lands (S55; 146, 151).

use super::walk::*;
use serde_json::{json, Value};
use std::path::Path;

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

/// The usage limit the spent walkthrough's first code agent stops at, in
/// the words the pinned Claude Code writes it (151, C.4).
const SESSION_LIMIT: &str = "You've hit your session limit · resets 3pm (UTC)";

/// The seconds since the epoch now.
fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("a clock past the epoch")
        .as_secs()
}

/// The reset `SESSION_LIMIT` names for a hook run at `at`: the first
/// 15:00 UTC no more than a minute before it (151).
fn three_pm_after(at: u64) -> u64 {
    let today = at - at % 86_400 + 15 * 3600;
    if today + 60 >= at {
        today
    } else {
        today + 86_400
    }
}

#[test]
fn a_spent_account_moves_the_stage_to_the_pools_next_and_the_fix_lands() {
    // S55, 146, 151: on a fill-first pool of two accounts, each a scratch
    // `CLAUDE_CONFIG_DIR`, the fix's code agent starts under the first and
    // its turn fails at the session limit through StopFailure. The first
    // account reads spent until 15:00 UTC, code starts afresh under the
    // second with flume's words, the move recorded with its cache lost;
    // the second's agent commits what the first left, and the place lands.
    let walk = Walk::new("walk-spent");
    let first = walk.account().to_path_buf();
    let second = walk.account_folder("account-2");
    walk.configure_whole(&format!(
        "[host]\npool = \"stand-in\"\n\n[pools.stand-in]\nstrategy = \"fill-first\"\naccounts = [\n  {{ name = \"stand-in-1\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?} }} }},\n  {{ name = \"stand-in-2\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?} }} }},\n]\n",
        first.display().to_string(),
        second.display().to_string(),
    ));
    // The walkthrough runs clear of the minutes around the reset, where a
    // limit read just before 15:00 has already passed when code starts
    // afresh.
    while (14 * 3600 + 55 * 60..15 * 3600 + 2 * 60).contains(&(now() % 86_400)) {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    let place = walk.launch("fix", "one", &["--brief", "fix the parser"]);
    let status = walk.status(&place);
    assert_eq!(status["agent"]["account"]["name"], "stand-in-1", "{status}");
    let before = now();
    let out = walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "fail": { "error": "rate_limit", "words": SESSION_LIMIT } },
        ]),
    );
    assert!(out.contains("the turn failed"), "{out}");
    assert!(!out.contains("stop blocked"), "{out}");
    let after = now();

    let status = walk.until(&place, "code started afresh", |status| {
        stage(status, "code")["starts"] == 2 && !status["agent"].is_null()
    });
    assert_eq!(
        status["agent"]["account"],
        json!({ "pool": "stand-in", "name": "stand-in-2", "from": "strategy" }),
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
            &moved["cache"]
        ),
        (
            &json!("code"),
            &json!("stand-in-1"),
            &json!("stand-in-2"),
            &json!("spent"),
            &json!("lost")
        ),
        "{status}"
    );
    assert!(stage(&status, "code")["end"].is_null(), "{status}");
    let prompt = walk.prompt(&place);
    assert!(
        prompt.contains("stopped at its account's usage limit"),
        "flume's words: {prompt}"
    );
    let host = walk.host();
    let accounts = &host["pools"][0]["accounts"];
    let reset = three_pm_after(before);
    assert_eq!(reset, three_pm_after(after), "a reset read on either side");
    assert_eq!(accounts[0]["name"], "stand-in-1", "{host}");
    assert_eq!(accounts[0]["spent_until"], reset, "{host}");
    assert_eq!(accounts[0]["running"], 0, "{host}");
    assert_eq!(accounts[1]["name"], "stand-in-2", "{host}");
    assert!(accounts[1]["spent_until"].is_null(), "{host}");
    assert_eq!(accounts[1]["running"], 1, "{host}");

    walk.agent(
        &place,
        json!([{ "bash": "git add -A && git commit --quiet -m 'fix the parser'" }]),
    );
    until_landed(&walk, &place);
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n"),
        "what the first agent left, committed by the second"
    );
    assert!(!walk.has_branch("fix/one"));
    for account in [&first, &second] {
        assert!(
            account.join("projects").exists(),
            "an agent under {}",
            account.display()
        );
    }
}

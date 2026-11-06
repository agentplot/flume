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
//! 179); a most-left pool starts a fix on the account whose reading
//! leaves more, and, that reading written past ninety percent, the
//! intent's next fix on the other, status giving both readings and the
//! first account past its threshold with no reading's path, and the fix
//! running on it lands (145, 147, 184, 185); and a fix whose first code
//! agent comes up without its account's variable is ended and its stage
//! started afresh under the account, and lands (S65; 161); a fix whose
//! host's table gives `edits` over its repository's `full` starts its
//! agent in the mode the host gave and lands, as does a fix no layer gives
//! a mode, passing none (S66; 162); a fix whose agent begins in another
//! mode than its start asked at both its starts is never begun, naming
//! both modes (S67; 161, 162); an agent run only after its start was
//! read never begun acks its start and its prompt, its stage is read
//! begun, and a retry leaves it be (35, 38, 50); a fix's code agent's
//! line, asked by `flume hook status-line` in its environment as the
//! plugin's module asks it, is status's line asked alone and in full, and
//! the fix lands, while a change@2 code agent's names its group, its run
//! and the change's tasks (56, 60, 142; S14); a host's people's file, the
//! host's own status line over it, a stream's and a repository's file reach
//! a fix's agent in `work` and in `default` by a `--settings` file of its
//! start's own, which the stand-in reads over its account's settings, the
//! keys another setting owns left out and named in status, and a fix no
//! layer gives own settings passes none, each landing (S100; 32, 143, 195,
//! 196); seven launches whose own settings give a reserved key, hold a
//! credential or name a file that cannot be read are refused naming the
//! key, the layer and the file, never the value, making nothing (S101; 4,
//! 157, 195); a fork's attribution is the host's for every stage of a
//! fix and a chore, a repository's its own `[agents]`' and a third's its
//! account's, each landing (S102; 32, 195, C.2, C.7); a host's source's
//! kind, locked by `flume kinds lock --host --hash`, runs and lands, a
//! second version added and its source's file renamed over, the next
//! launch running it and the place launched before landing on its own
//! version (design 13; 21, 154, 155, 159, 191); and a talk read lost,
//! whose session a person resumed by hand under another home, waits at its
//! retry while that session runs, flume ending nothing, and goes on in
//! the same session and transcript once it ends, and a lost talk whose
//! transcript is gone starts afresh with flume's words (S113, S115, S117;
//! 49, 77, 179, 207).

use super::walk::*;
use flume::profiles::agent::claude::carry::{UNREACHED, UNREADABLE};
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

/// The variables the place's printed command gives its agent, from the
/// file the command names, as the agent program's process holds them.
fn agents_environment(walk: &Walk, place: &str) -> Vec<(String, String)> {
    let printed = walk.printed(place);
    let file = printed
        .split(' ')
        .map(|word| word.trim_matches('\''))
        .find(|word| word.ends_with(".env"))
        .unwrap_or_else(|| panic!("no environment file in {printed}"));
    std::fs::read_to_string(file)
        .expect("the command's environment")
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(name, value)| {
            let value = value.strip_prefix('\'').unwrap_or(value);
            let value = value.strip_suffix('\'').unwrap_or(value);
            (name.to_string(), value.replace("'\\''", "'"))
        })
        .collect()
}

/// `flume hook status-line` run in the place in its agent's environment,
/// as the plugin's module runs it in the agent's session (design 4, 5).
fn status_line(walk: &Walk, place: &str) -> String {
    let output = walk
        .command(env!("CARGO_BIN_EXE_flume"), Path::new(place))
        .envs(agents_environment(walk, place))
        .args(["hook", "status-line"])
        .output()
        .expect("flume runs");
    assert!(output.status.success(), "status-line exits 0");
    let line = String::from_utf8_lossy(&output.stdout).into_owned();
    assert_eq!(line.lines().count(), 1, "one line: {line}");
    line.trim_end().to_string()
}

#[test]
fn a_fix_agents_line_is_status_line_and_the_fix_lands() {
    // While the fix's code agent waits on a file, the line the hook
    // prints in its environment, as the plugin's module runs it, is the
    // place's line status --line gives and status's JSON gives in full;
    // then the agent commits and the fix lands (56, 60; S14).
    let walk = Walk::new("walk-line");
    let place = walk.launch("fix", "one", &["--brief", "fix the parser"]);
    let go = walk.signal("go");
    let mut acts = vec![json!({ "wait_for": go })];
    acts.extend(
        commits("parser.rs", "fn parse() {}\n")
            .as_array()
            .cloned()
            .expect("the acts"),
    );
    let agent = walk.start_agent(&place, Value::Array(acts));
    walk.until(&place, "its code agent working", |status| {
        status["line"]["state"] == "working"
    });

    let line = status_line(&walk, &place);
    assert_eq!(line, "fix--one · fix@1 · code · run 1 · working");
    assert_eq!(walk.person(&["status", &place, "--line"]).trim_end(), line);
    assert_eq!(walk.status(&place)["line"]["full"], line.as_str());

    std::fs::write(&go, "").expect("the signal");
    let ran = finish(agent);
    assert_eq!(ran.code, 0, "{}", ran.out);
    until_landed(&walk, &place);
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n")
    );
}

/// A change of two task groups, two tasks and one, none ticked.
const TWO_GROUPS: &str = "## 1. Parser\n\n- [ ] 1.1 Write the parser\n- [ ] 1.2 Test the parser\n\n## 2. Reader\n\n- [ ] 2.1 Read the input\n";

#[test]
fn a_change_code_agents_line_names_its_group_its_run_and_its_tasks() {
    // change@2's code agent on the first of two groups, waiting on a
    // file: its line names the stage with its group, its run and the
    // change's tasks ticked at the head (56, 60, 140, 142; S14).
    let walk = Walk::new("walk-line-groups");
    let place = walk.launch("change", "add-parser", &["--brief", "add a parser"]);
    walk.agent(&place, construct_tasks("add-parser", TWO_GROUPS));
    walk.until(&place, "code on group 1", |status| {
        status["current"] == "code" && !status["agent"].is_null()
    });
    let go = walk.signal("go");
    let agent = walk.start_agent(
        &place,
        json!([
            { "wait_for": go },
            { "tick": { "path": "openspec/changes/add-parser/tasks.md", "count": 2 } },
            { "bash": "git add -A && git commit --quiet -m 'group 1 built'" },
        ]),
    );
    let status = walk.until(&place, "its code agent working", |status| {
        status["line"]["state"] == "working"
    });
    assert_eq!(
        status["line"]["short"], "code 1/2 · run 1 · tasks 0/3 · working",
        "{status}"
    );
    let line = status_line(&walk, &place);
    assert_eq!(
        line,
        "change--add-parser · change@2 · code 1/2 · run 1 · tasks 0/3 · working"
    );
    assert_eq!(walk.person(&["status", &place, "--line"]).trim_end(), line);
    assert_eq!(walk.status(&place)["line"]["full"], line.as_str());

    std::fs::write(&go, "").expect("the signal");
    let ran = finish(agent);
    assert_eq!(ran.code, 0, "{}", ran.out);
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
    // configuration gave it room, until a settle by hand: no caller keeps
    // watch, so a place moves on only when a command of flume's acts or an
    // agent goes quiet (119, 208). Status says so and
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
            "no caller keeps watch: every place settles at each command of flume's that acts"
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
/// their folders, and the folder it names its running sessions in, empty
/// once they ended: flume copied, linked and wrote nothing there (179,
/// C.4, 207; core.32).
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
            // The folder 2.1.296 names its running sessions in, each
            // record gone with its session.
            [sessions] if sessions == "sessions" => holding == &Holding::Folder,
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

/// `at`, seconds by the host's clock, as an RFC 3339 time with its
/// offset, as a reading writes its times (C.7).
fn rfc3339(at: u64) -> String {
    chrono::DateTime::from_timestamp(i64::try_from(at).expect("a time"), 0)
        .expect("a time")
        .to_rfc3339()
}

/// An account's reading read at `read_at` with `windows`, written whole at
/// `path` as a host's reader writes one: a new file beside it renamed over
/// the old (184, C.7).
fn reads(path: &Path, read_at: u64, windows: Value) {
    let new = path.with_extension("json.new");
    std::fs::write(
        &new,
        json!({ "read_at": rfc3339(read_at), "reader": "the walk's", "windows": windows })
            .to_string(),
    )
    .expect("the reading");
    std::fs::rename(&new, path).expect("the reading written whole");
}

#[test]
fn a_most_left_pool_starts_on_the_account_with_most_left_and_none_past_its_threshold() {
    // 145, 147, 184, 185, under the printed command: on a most-left pool of
    // two accounts, each a scratch `CLAUDE_CONFIG_DIR` whose `usage` names a
    // scratch file under the home, the second's reading leaving more, a fix
    // starts on the second. Its reading is then written past ninety percent
    // in its 5-hour window, and the intent's next fix starts on the first,
    // the intent moving off the second for `claude` with the window and its
    // reset. Status gives the pool's threshold, both readings and the
    // second past its threshold, in the JSON form and the person's, with no
    // reading's path; the fix already running on the second goes on and
    // lands under its account, and flume wrote neither reading.
    let walk = Walk::new("walk-most-left");
    let first = walk.account().to_path_buf();
    let second = walk.account_folder("account-2");
    let readings = walk.home().join("readings");
    std::fs::create_dir_all(&readings).expect("the readings' folder");
    let usage = [
        readings.join("stand-in-1.json"),
        readings.join("stand-in-2.json"),
    ];
    walk.configure_whole(&format!(
        "[streams.default]\npool = \"stand-in\"\n\n[pools.stand-in]\nstrategy = \"most-left\"\naccounts = [\n  {{ name = \"stand-in-1\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?} }}, usage = \"~/readings/stand-in-1.json\" }},\n  {{ name = \"stand-in-2\", program = \"claude\", env = {{ CLAUDE_CONFIG_DIR = {:?} }}, usage = \"~/readings/stand-in-2.json\" }},\n]\n",
        first.display().to_string(),
        second.display().to_string(),
    ));
    let read = now();
    let reset = read - read % 3600 + 2 * 3600;
    reads(
        &usage[0],
        read,
        json!([
            { "name": "5-hour", "used": 40, "resets_at": rfc3339(reset) },
            { "name": "7-day", "used": 55 },
        ]),
    );
    reads(
        &usage[1],
        read,
        json!([
            { "name": "5-hour", "used": 12.5, "resets_at": rfc3339(reset) },
            { "name": "7-day", "used": 30 },
        ]),
    );

    let one = walk.launch("fix", "one", &["--brief", "fix the parser"]);
    assert_eq!(
        walk.status(&one)["agent"]["account"],
        json!({ "pool": "stand-in", "name": "stand-in-2", "from": "strategy" }),
        "the account whose reading leaves more"
    );

    let past = now();
    reads(
        &usage[1],
        past,
        json!([
            { "name": "5-hour", "used": 93, "resets_at": rfc3339(reset) },
            { "name": "7-day", "used": 30 },
        ]),
    );
    let written = usage.each_ref().map(|path| {
        let file = std::fs::read_to_string(path).expect("the reading");
        let at = std::fs::metadata(path)
            .and_then(|meta| meta.modified())
            .expect("the reading's write time");
        (file, at)
    });
    let two = walk.launch("fix", "two", &["--brief", "fix the reader"]);
    let status = walk.status(&two);
    assert_eq!(
        status["agent"]["account"],
        json!({ "pool": "stand-in", "name": "stand-in-1", "from": "strategy" }),
        "the intent's affinity past its threshold: {status}"
    );
    let moved = status["moves"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|moved| moved["on"] == "intent")
        .unwrap_or_else(|| panic!("no move of the intent: {status}"));
    assert_eq!(
        (
            &moved["program"],
            &moved["from"],
            &moved["to"],
            &moved["why"],
            &moved["window"],
            &moved["until"]
        ),
        (
            &json!("claude"),
            &json!("stand-in-2"),
            &json!("stand-in-1"),
            &json!("threshold"),
            &json!("5-hour"),
            &json!(reset)
        ),
        "{status}"
    );
    assert_eq!(
        walk.status(&one)["agent"]["account"]["name"],
        "stand-in-2",
        "the running fix moved"
    );

    let answer = walk.flume(&["status"]);
    let pool = &answer["status"]["host"]["pools"][0];
    assert_eq!(
        (&pool["name"], &pool["strategy"], &pool["threshold"]),
        (&json!("stand-in"), &json!("most-left"), &json!(90)),
        "{pool}"
    );
    let accounts = &pool["accounts"];
    assert_eq!(
        (&accounts[0]["name"], &accounts[0]["running"]),
        (&json!("stand-in-1"), &json!(1)),
        "{pool}"
    );
    assert_eq!(
        accounts[0]["reading"],
        json!({
            "read_at": read,
            "fresh": true,
            "windows": [
                { "name": "5-hour", "used": 40, "resets_at": reset, "past": false },
                { "name": "7-day", "used": 55, "past": false },
            ],
        }),
        "{pool}"
    );
    assert!(accounts[0].get("past").is_none(), "{pool}");
    assert_eq!(
        (&accounts[1]["name"], &accounts[1]["running"]),
        (&json!("stand-in-2"), &json!(1)),
        "{pool}"
    );
    assert_eq!(
        accounts[1]["reading"],
        json!({
            "read_at": past,
            "fresh": true,
            "windows": [
                { "name": "5-hour", "used": 93, "resets_at": reset, "past": true },
                { "name": "7-day", "used": 30, "past": false },
            ],
        }),
        "{pool}"
    );
    assert_eq!(
        accounts[1]["past"],
        json!({ "window": "5-hour", "until": reset }),
        "{pool}"
    );
    let said = walk.person(&["status"]);
    for lines in [
        format!("  pool stand-in, most-left, threshold 90%:\n    stand-in-1 (claude): 1 agents running, no bound of its own\n      5-hour 40% (resets {reset}) · 7-day 55% · read at {read}\n"),
        format!("    stand-in-2 (claude): 1 agents running, no bound of its own\n      5-hour 93% (resets {reset}) · 7-day 30% · read at {past}\n      past its threshold in 5-hour until {reset}\n"),
        format!("stand-in-2 past its threshold in 5-hour until {reset}"),
    ] {
        assert!(said.contains(&lines), "{lines} in {said}");
    }
    for words in [answer.to_string(), said] {
        for path in [readings.to_string_lossy().into_owned(), "readings/".into()] {
            assert!(!words.contains(&path), "a reading's path in {words}");
        }
    }

    walk.agent(&one, commits("parser.rs", "fn parse() {}\n"));
    until_landed(&walk, &one);
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n"),
        "the fix on the account past its threshold"
    );
    assert!(
        second.join("projects").exists() && !first.join("projects").exists(),
        "the fix's agent under stand-in-2's home alone"
    );
    for (path, (file, at)) in usage.iter().zip(written) {
        let now = std::fs::read_to_string(path).expect("the reading");
        let now_at = std::fs::metadata(path)
            .and_then(|meta| meta.modified())
            .expect("the reading's write time");
        assert_eq!((now, now_at), (file, at), "{} written", path.display());
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

/// The printed command run to its end, the stand-in committing `file`:
/// the settings the stand-in read, as it merged them, its account's, the
/// place's and those `--settings` passed (195; design 14).
fn read_settings(walk: &Walk, place: &str, file: &str) -> Value {
    let read = walk.signal(&format!("read-{file}.json"));
    walk.agent_with(
        place,
        commits(file, "fixed\n"),
        &[("STAND_IN_SETTINGS", &read.to_string_lossy())],
    );
    let text = std::fs::read_to_string(&read).expect("the settings the stand-in read");
    serde_json::from_str(&text).expect("the settings as JSON")
}

/// The own settings' file the place's printed command names, as JSON.
fn passed(walk: &Walk, place: &str) -> Value {
    let file = walk
        .own_settings(place)
        .unwrap_or_else(|| panic!("no --settings in {}", walk.printed(place)));
    let text = std::fs::read_to_string(&file).expect("the own settings' file");
    serde_json::from_str(&text).expect("the own settings as JSON")
}

/// Every file named `own-settings.json` under `dir`.
fn own_files(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.extend(own_files(&path));
        } else if path
            .file_name()
            .is_some_and(|name| name == "own-settings.json")
        {
            found.push(path);
        }
    }
    found
}

/// The keys of a JSON object, in order.
fn keys(value: &Value) -> Vec<&str> {
    value
        .as_object()
        .into_iter()
        .flatten()
        .map(|(key, _)| key.as_str())
        .collect()
}

/// The settings file a host's launcher gives its people's sessions: what
/// they see, and the keys flume's other settings, the account and a person
/// own.
const PEOPLE: &str = r#"{
  "tui": "fullscreen",
  "statusLine": { "type": "command", "command": "people-line" },
  "env": { "PEOPLE": "1" },
  "permissions": { "allow": ["Bash(git log:*)"], "defaultMode": "bypassPermissions" },
  "skipDangerousModePermissionPrompt": true
}
"#;

#[test]
fn a_hosts_people_file_its_lines_and_a_repositorys_file_reach_each_streams_agents() {
    // S100, 32, 143, 195, 196: the host names its people's file for every
    // agent with a status line of its own over it, and another for
    // `work`; the repository names a file of its own for fix. A fix in
    // `work` is started with `--settings` naming a file of its start's own
    // under the place's folder, never the people's file, and the stand-in
    // read work's line, the file's `tui` and `permissions` without
    // `defaultMode` joined with its account's own, the repository's
    // `alwaysThinkingEnabled` and its account's `cleanupPeriodDays`, with
    // no `env` and no acceptance; status names each key with its layer and
    // file and the three keys left out with their owners. A fix in
    // `default` read the host's line.
    // A host and a repository that give no entry start a fix with nothing
    // passed and no file written. Each lands.
    let walk = Walk::new("walk-own");
    let people = walk.repository.scratch.path("people-settings.json");
    std::fs::write(&people, PEOPLE).expect("the people's file");
    std::fs::write(
        walk.account().join("settings.json"),
        "{ \"cleanupPeriodDays\": 30, \"permissions\": { \"allow\": [\"Bash(git log:*)\", \"Read\"] } }\n",
    )
    .expect("the account's settings");
    walk.configure(&format!(
        "[streams.work]\npool = \"stand-in\"\n\n[agents.programs.claude]\nfile = {:?}\nsettings = {{ statusLine = {{ type = \"command\", command = \"host-line\" }} }}\n\n[agents.streams.work.programs.claude.settings]\nstatusLine = {{ type = \"command\", command = \"work-line\" }}\n",
        people.display().to_string()
    ));
    walk.repository.write(
        ".config/claude-fix.json",
        "{ \"alwaysThinkingEnabled\": false }\n",
    );
    the_repository_gives(
        &walk,
        "[kinds.fix.agents.programs.claude]\nfile = \"claude-fix.json\"\n",
    );

    let place = walk.launch(
        "fix",
        "work",
        &["--brief", "fix the parser", "--stream", "work"],
    );
    let printed = walk.printed(&place);
    assert!(!printed.contains("people-settings"), "{printed}");
    let file = walk.own_settings(&place).expect("--settings");
    assert!(
        file.starts_with(walk.home().join("state")) && file.ends_with("own-settings.json"),
        "the file is not the start's own: {printed}"
    );
    let given = passed(&walk, &place);
    assert_eq!(
        keys(&given),
        vec!["alwaysThinkingEnabled", "permissions", "statusLine", "tui"],
        "{given}"
    );
    let status = walk.status(&place);
    let from = &status["agent"]["settings"]["from"];
    let people = people.to_string_lossy().into_owned();
    assert_eq!(
        (
            &from["programs.claude.settings.statusLine"],
            &from["programs.claude.settings.tui"],
            &from["programs.claude.settings.permissions"],
            &from["programs.claude.settings.alwaysThinkingEnabled"],
        ),
        (
            &json!({ "layer": "host-stream", "value": "{\"command\":\"work-line\",\"type\":\"command\"}" }),
            &json!({ "layer": "host", "value": "\"fullscreen\"", "file": people }),
            &json!({ "layer": "host", "value": "{\"allow\":[\"Bash(git log:*)\"]}", "file": people }),
            &json!({ "layer": "repository", "value": "false", "file": ".config/claude-fix.json" }),
        ),
        "{status}"
    );
    let own = &status["agent"]["own"];
    assert_eq!(
        own["passed"],
        format!("--settings {}", file.display()),
        "{status}"
    );
    assert_eq!(own["read_back"], false, "{status}");
    let host_file = own["files"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|file| file["layer"] == "host")
        .unwrap_or_else(|| panic!("no host file in {status}"));
    assert_eq!(
        host_file["left_out"],
        json!([
            { "key": "env", "owner": "the account's `env`" },
            { "key": "permissions.defaultMode", "owner": "flume's `mode`" },
            { "key": "skipDangerousModePermissionPrompt", "owner": "a person, who alone accepts the warning it records" },
        ]),
        "{status}"
    );

    let read = read_settings(&walk, &place, "in-work.rs");
    assert_eq!(read["statusLine"]["command"], "work-line", "{read}");
    assert_eq!(read["tui"], "fullscreen", "{read}");
    // The account's list joined with the file's, the rule both give kept
    // once, as the release joins two sources' lists.
    assert_eq!(
        read["permissions"],
        json!({ "allow": ["Bash(git log:*)", "Read"] }),
        "{read}"
    );
    assert_eq!(read["alwaysThinkingEnabled"], false, "{read}");
    assert_eq!(read["cleanupPeriodDays"], 30, "{read}");
    assert!(
        read.get("env").is_none() && read.get("skipDangerousModePermissionPrompt").is_none(),
        "{read}"
    );
    until_landed(&walk, &place);
    assert_eq!(walk.on_remote("in-work.rs").as_deref(), Some("fixed\n"));

    // A launch names `default`, since the intent's stream is `work` now.
    let place = walk.launch(
        "fix",
        "default",
        &["--brief", "fix the reader", "--stream", "default"],
    );
    assert_eq!(
        walk.status(&place)["agent"]["settings"]["from"]["programs.claude.settings.statusLine"]
            ["layer"],
        "host"
    );
    let read = read_settings(&walk, &place, "default.rs");
    assert_eq!(read["statusLine"]["command"], "host-line", "{read}");
    assert_eq!(read["tui"], "fullscreen", "{read}");
    until_landed(&walk, &place);
    assert_eq!(walk.on_remote("default.rs").as_deref(), Some("fixed\n"));

    let walk = Walk::new("walk-own-none");
    std::fs::write(
        walk.account().join("settings.json"),
        "{ \"cleanupPeriodDays\": 30 }\n",
    )
    .expect("the account's settings");
    let place = walk.launch("fix", "one", &["--brief", "fix the parser"]);
    let printed = walk.printed(&place);
    assert!(!printed.contains("--settings"), "{printed}");
    let status = walk.status(&place);
    assert!(status["agent"]["own"].is_null(), "{status}");
    assert_eq!(own_files(walk.home()), Vec::<PathBuf>::new());
    let read = read_settings(&walk, &place, "parser.rs");
    assert_eq!(read, json!({ "cleanupPeriodDays": 30 }));
    until_landed(&walk, &place);
    assert_eq!(walk.on_remote("parser.rs").as_deref(), Some("fixed\n"));
}

/// A launch of a fix `name` with `words` beside, which flume refuses
/// having made nothing: its words.
fn refused_whole(walk: &Walk, name: &str, words: &[&str]) -> String {
    let mut launch = vec![
        "launch",
        "--kind",
        "fix",
        "--name",
        name,
        "--brief",
        "fix the parser",
    ];
    launch.extend(words);
    let (answer, refused) = walk.flume_ran(&launch);
    assert!(refused, "the launch was taken: {answer}");
    assert!(
        !walk.has_branch(&format!("fix/{name}")),
        "the fix's branch was made"
    );
    assert_eq!(
        walk.repository.git(&["worktree", "list"]).lines().count(),
        1,
        "a worktree was made"
    );
    answer.to_string()
}

/// The repository's binding with `more` beside it in place of whatever it
/// gave before, committed and pushed to the remote.
fn the_repository_gives_only(walk: &Walk, binding: &str, more: &str) {
    walk.repository
        .write(".config/flume.toml", &format!("{binding}\n{more}"));
    walk.repository.commit("the repository's settings");
    walk.repository.git(&["push", "--quiet", "origin", "main"]);
}

/// A value shaped as a token an issuer gives, which no layer may hold
/// (design 7).
const TOKEN: &str = "ghp_StandInWordShapedAsAToken0123456789";

#[test]
fn own_settings_that_are_reserved_or_hold_a_credential_or_cannot_be_read_refuse_the_launch() {
    // S101, 4, 157, 195: seven launches, each refused naming its key, its
    // layer and its file where it came from one, never a value, and each
    // making nothing.
    let walk = Walk::new("walk-own-refused");
    let binding = std::fs::read_to_string(walk.repository.dir.join(".config/flume.toml"))
        .expect("the binding");
    let scratch = |name: &str| walk.repository.scratch.path(name);
    let says = |words: &str, parts: &[&str], value: &str| {
        for part in parts {
            assert!(
                words.contains(part),
                "the refusal names not {part}: {words}"
            );
        }
        assert!(
            !words.contains(value),
            "the refusal gives the value: {words}"
        );
        let status = walk.everything().to_string();
        assert!(!status.contains(value), "status gives the value: {status}");
        assert_eq!(walk.everything()["places"], json!([]), "a place was made");
    };

    // The launch gives Claude Code an `env`, which is the account's.
    let words = refused_whole(
        &walk,
        "env",
        &[
            "--agent",
            "programs.claude.settings.env={\"STAND_IN_WORD\":\"never-given\"}",
        ],
    );
    says(
        &words,
        &[
            "programs.claude.settings.env",
            "launch",
            "the account's `env`",
        ],
        "never-given",
    );

    // A token in a status line the launch gives.
    let line = format!(
        "programs.claude.settings.statusLine={{\"type\":\"command\",\"command\":\"line {TOKEN}\"}}"
    );
    let words = refused_whole(&walk, "launch-token", &["--agent", &line]);
    says(
        &words,
        &["programs.claude.settings.statusLine.command", "launch"],
        TOKEN,
    );

    // The host's own settings give `apiKeyHelper`, the account's sign-in.
    // The refusal gives no value; status's configuration values give the
    // host's own words as written, a script's name being no credential
    // (core.45; design 12), so only the refusal is read for it.
    walk.configure("[agents.programs.claude.settings]\napiKeyHelper = \"key-helper-script\"\n");
    let words = refused_whole(&walk, "helper", &[]);
    assert!(
        [
            "programs.claude.settings.apiKeyHelper",
            "host",
            "the account's sign-in"
        ]
        .iter()
        .all(|part| words.contains(part))
            && !words.contains("key-helper-script"),
        "{words}"
    );

    // A token in the status line of the host's named file.
    let tokened = scratch("tokened.json");
    std::fs::write(
        &tokened,
        format!(
            "{{ \"statusLine\": {{ \"type\": \"command\", \"command\": \"line {TOKEN}\" }} }}\n"
        ),
    )
    .expect("the host's file");
    walk.configure(&format!(
        "[agents.programs.claude]\nfile = {:?}\n",
        tokened.display().to_string()
    ));
    let words = refused_whole(&walk, "host-token", &[]);
    says(
        &words,
        &[
            "programs.claude.settings.statusLine.command",
            "host",
            &tokened.to_string_lossy(),
        ],
        TOKEN,
    );

    // The host names a file that does not exist.
    let missing = scratch("missing.json");
    walk.configure(&format!(
        "[agents.programs.claude]\nfile = {:?}\n",
        missing.display().to_string()
    ));
    let words = refused_whole(&walk, "missing", &[]);
    assert!(
        [
            "config.toml",
            "agents.programs.claude.file",
            "could not be read"
        ]
        .iter()
        .all(|part| words.contains(part))
            && words.contains(&*missing.to_string_lossy()),
        "{words}"
    );
    walk.configure("");

    // The repository's kind table gives `permissions.defaultMode`, flume's
    // mode's.
    the_repository_gives_only(
        &walk,
        &binding,
        "[kinds.fix.agents.programs.claude.settings]\npermissions = { defaultMode = \"bypassPermissions\" }\n",
    );
    let words = refused_whole(&walk, "mode", &[]);
    says(
        &words,
        &[
            "programs.claude.settings.permissions.defaultMode",
            "repository",
            "flume's `mode`",
        ],
        "bypassPermissions",
    );

    // The repository names a file past its root, which stands.
    std::fs::write(
        walk.repository.scratch.path("claude.json"),
        "{ \"tui\": \"fullscreen\" }\n",
    )
    .expect("a file outside the repository");
    the_repository_gives_only(
        &walk,
        &binding,
        "[kinds.fix.agents.programs.claude]\nfile = \"../../claude.json\"\n",
    );
    let words = refused_whole(&walk, "outside", &[]);
    says(
        &words,
        &[
            "kinds.fix.agents.programs.claude.file",
            "../../claude.json",
            "lies outside the repository",
        ],
        "fullscreen",
    );
}

/// Claude Code's attribution with no text and no session link, as a host
/// writes it for a fork whose upstream refuses a tagline (design 16).
const NO_TAGLINE: &str = "attribution = { commit = \"\", pr = \"\", sessionUrl = false }\n";

#[test]
fn a_forks_attribution_is_the_hosts_a_repositorys_its_own_and_another_its_accounts() {
    // S102, 32, 143, 195, C.2, C.7: three repositories on one host. The
    // host's `[repos."<fork>".agents]` gives Claude Code's attribution with
    // no text and no session link; `ours` commits its own words in its
    // `.config/flume.toml`'s `[agents]`; `plain` gives none, and the
    // account's own settings give one. Every stage of a fix and a chore in
    // the fork is started with the attribution as the host wrote it, from
    // `host-repository`, and the stand-in read it over its account's; a fix
    // in `ours` with its own words, from `repository`; a fix in `plain`
    // with nothing passed, the stand-in reading its account's. Each lands.
    let fork = Walk::new("walk-fork");
    let ours = Walk::new("walk-ours").with_home(fork.home().to_path_buf());
    let plain = Walk::new("walk-plain").with_home(fork.home().to_path_buf());
    std::fs::write(
        fork.account().join("settings.json"),
        "{ \"attribution\": { \"commit\": \"Co-Authored-By: the account\" } }\n",
    )
    .expect("the account's settings");
    fork.configure(&format!(
        "[repos.{:?}.agents.programs.claude.settings]\n{NO_TAGLINE}",
        fork.repository.name()
    ));
    the_repository_gives(
        &ours,
        "[agents.programs.claude.settings]\nattribution = { commit = \"Assisted-by: Claude Code\", pr = \"\" }\n",
    );

    let none = json!({ "commit": "", "pr": "", "sessionUrl": false });
    for kind in ["fix", "chore"] {
        let place = fork.launch(kind, "fork", &["--brief", "mend the parser"]);
        let given = passed(&fork, &place);
        assert_eq!(given, json!({ "attribution": none }), "{kind}");
        let status = fork.status(&place);
        assert_eq!(
            status["agent"]["settings"]["from"]["programs.claude.settings.attribution"],
            json!({ "layer": "host-repository", "value": "{\"commit\":\"\",\"pr\":\"\",\"sessionUrl\":false}" }),
            "{status}"
        );
        let read = read_settings(&fork, &place, &format!("{kind}.rs"));
        assert_eq!(read["attribution"], none, "{kind}: {read}");
        until_landed(&fork, &place);
        assert_eq!(
            fork.on_remote(&format!("{kind}.rs")).as_deref(),
            Some("fixed\n")
        );
    }

    let place = ours.launch("fix", "ours", &["--brief", "mend the parser"]);
    let words = json!({ "commit": "Assisted-by: Claude Code", "pr": "" });
    assert_eq!(passed(&ours, &place), json!({ "attribution": words }));
    let status = ours.status(&place);
    assert_eq!(
        status["agent"]["settings"]["from"]["programs.claude.settings.attribution"]["layer"],
        "repository",
        "{status}"
    );
    let read = read_settings(&ours, &place, "ours.rs");
    assert_eq!(read["attribution"], words, "{read}");
    until_landed(&ours, &place);
    assert_eq!(ours.on_remote("ours.rs").as_deref(), Some("fixed\n"));

    let place = plain.launch("fix", "plain", &["--brief", "mend the parser"]);
    let printed = plain.printed(&place);
    assert!(!printed.contains("--settings"), "{printed}");
    let read = read_settings(&plain, &place, "plain.rs");
    assert_eq!(
        read["attribution"],
        json!({ "commit": "Co-Authored-By: the account" }),
        "{read}"
    );
    until_landed(&plain, &place);
    assert_eq!(plain.on_remote("plain.rs").as_deref(), Some("fixed\n"));
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

/// A kind of the walkthrough's own whose one stage is a conversation, and
/// which does not land.
const TALK_KIND: &str = r#"[kinds.talk]
version = 1
branch = "talk"
starts_unready = true
lands = false

[[kinds.talk.stages]]
name = "talk"
prompt = "prompts/talk.md"
conversation = true
agents = [{}]
"#;

/// The words of a lost conversation's carried start (49, 179).
const RESUMED: &str = "Your session was lost and goes on here, with its conversation as it stood. Anything you had running then has stopped: read the place again before you carry on.";

/// The words before a lost conversation's fresh start's own (49).
const LOST: &str = "The stage's earlier agent was lost, and its conversation could not be carried, so this agent starts the stage afresh. What it committed is on the place's branch and what it left is in the place: read them and carry the stage on from there.";

/// Launches `talk--<name>` from the walkthrough's own kinds and gives the
/// place's location.
fn launch_talk(walk: &Walk, name: &str) -> String {
    let kinds = walk.repository.scratch.path("talk-kinds");
    std::fs::create_dir_all(kinds.join("prompts")).expect("the kinds' folder");
    std::fs::write(kinds.join("kinds.toml"), TALK_KIND).expect("the kinds");
    std::fs::write(
        kinds.join("prompts/talk.md"),
        "Talk with the person about {{brief}}.\n\n{{#words}}{{words}}\n\n{{/words}}To stop short: {{needs}}\n",
    )
    .expect("the prompt");
    walk.launch(
        "talk",
        name,
        &[
            "--brief",
            "the parser",
            "--kinds",
            &kinds.join("kinds.toml").to_string_lossy(),
        ],
    )
}

/// The talk's first agent run to its end, as a person leaves the printed
/// command's session, and the stage read lost: the session it ran and the
/// transcript it wrote.
fn talk_lost(walk: &Walk, place: &str) -> (String, PathBuf) {
    let out = walk.agent(place, json!([]));
    let ran = sessions(&out);
    assert_eq!(ran.len(), 1, "{out}");
    walk.until(place, "talk lost", |status| {
        stage(status, "talk")["state"] == "lost"
    });
    let transcripts = transcripts(walk.account());
    assert_eq!(transcripts.len(), 1, "{transcripts:?}");
    (ran[0].clone(), transcripts[0].clone())
}

/// Every transcript under `home`'s `projects`, links followed.
fn transcripts(home: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(home.join("projects"))
        .into_iter()
        .flatten()
        .flatten()
        .flat_map(|folder| {
            std::fs::read_dir(folder.path())
                .into_iter()
                .flatten()
                .flatten()
        })
        .map(|file| file.path())
        .collect()
}

/// The transcript each start of the place acked, by its started ack in
/// the place's own folder, in the order of the starts.
fn acked_transcripts(walk: &Walk) -> Vec<(String, PathBuf)> {
    let mut acked = vec![];
    let mut folders = vec![walk.home().join("state/flume")];
    while let Some(folder) = folders.pop() {
        for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.is_dir() {
                folders.push(path);
            } else if path.file_name().is_some_and(|name| name == "started.json") {
                let ack: Value =
                    serde_json::from_str(&std::fs::read_to_string(&path).expect("the started ack"))
                        .expect("the started ack as JSON");
                acked.push((
                    path.to_string_lossy().into_owned(),
                    PathBuf::from(ack["transcript_path"].as_str().unwrap_or_default()),
                ));
            }
        }
    }
    acked.sort();
    acked
}

/// The device and inode of the file at `path`, links followed.
fn file_id(path: &Path) -> (u64, u64) {
    use std::os::unix::fs::MetadataExt;
    let meta =
        std::fs::metadata(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    (meta.dev(), meta.ino())
}

#[test]
fn a_talk_resumed_by_hand_waits_at_its_retry_and_goes_on_in_its_session_once_it_ends() {
    // S113, S117, 49, 77, 179, 207: a talk's agent ends and the stage is
    // read lost. A person resumes its session by hand, `claude --resume
    // <session>`, under a second configuration home whose `projects` is a
    // link to the account's, with no flume of its own, and says something
    // in it. The caller's retry is taken and the stage waits on its
    // session, status naming the process and the line saying so, while
    // flume starts, ends and sends nothing to that process. Once the
    // person ends it, the next settle starts the stage on the account its
    // lost start ran under, its command resuming the lost session, its
    // prompt flume's words for a lost session and the way to stop short;
    // it goes on in the same transcript, by device and inode, which holds
    // what the person said, and the place is done and tidied (core.10,
    // core.18).
    let walk = Walk::new("walk-lost-talk");
    let place = launch_talk(&walk, "one");
    let (session, transcript) = talk_lost(&walk, &place);

    let by_hand_home = walk.account_folder("by-hand");
    std::os::unix::fs::symlink(
        walk.account().join("projects"),
        by_hand_home.join("projects"),
    )
    .expect("the by-hand home's projects, a link to the account's");
    let ended = walk.signal("by-hand-ends");
    let script = walk.repository.scratch.path("by-hand.json");
    std::fs::write(
        &script,
        json!([{ "wait_for": ended.to_string_lossy() }]).to_string(),
    )
    .expect("the by-hand script");
    let mut by_hand = walk
        .command("claude", Path::new(&place))
        .args(["--resume", &session, "a person's word in the talk"])
        .env("CLAUDE_CONFIG_DIR", &by_hand_home)
        .env("STAND_IN_SCRIPT", &script)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("the stand-in resumed by hand");
    let record = by_hand_home.join(format!("sessions/{}.json", by_hand.id()));
    let began = std::time::Instant::now();
    while !record.is_file() {
        assert!(
            began.elapsed() < std::time::Duration::from_secs(30),
            "the by-hand session never named itself"
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    walk.flume(&["retry", &place, "--stage", "talk"]);
    for _ in 0..3 {
        walk.settle(&place);
    }
    let status = walk.status(&place);
    let wait = &status["waits_on_session"];
    assert_eq!(wait["stage"], "talk", "{status}");
    assert_eq!(wait["processes"][0]["pid"], json!(by_hand.id()), "{status}");
    assert!(
        status["line"]["full"]
            .as_str()
            .is_some_and(|line| line.contains("waiting on its session")),
        "{status}"
    );
    assert_eq!(stage(&status, "talk")["starts"], 1, "{status}");
    assert!(!status["stalled"].as_bool().unwrap_or(false), "{status}");
    let person = walk.person(&["status", &place]);
    assert!(
        person.contains("talk waits for its conversation to end since")
            && person.contains(&format!("process {}", by_hand.id())),
        "{person}"
    );
    assert!(
        by_hand.try_wait().expect("the by-hand process").is_none(),
        "flume ended the session run by hand"
    );
    assert!(record.is_file(), "the by-hand session's record");

    std::fs::write(&ended, "").expect("the person ends the session");
    let ran = finish(by_hand);
    assert_eq!(ran.code, 0, "the by-hand session: {}", ran.out);
    assert!(ran.out.contains(", resumed"), "{}", ran.out);
    assert!(!record.exists(), "the by-hand session's record left behind");

    let status = walk.until(&place, "talk started again", |status| {
        stage(status, "talk")["starts"] == 2 && !status["agent"].is_null()
    });
    assert!(status["waits_on_session"].is_null(), "{status}");
    assert_eq!(
        status["agent"]["conversation"],
        json!({ "from": 1, "cause": "lost", "carried": true }),
        "{status}"
    );
    assert_eq!(
        status["agent"]["account"],
        json!({ "pool": "stand-in", "name": "stand-in-1", "from": "conversation" }),
        "{status}"
    );
    assert_eq!(walk.resumes(&place).as_deref(), Some(session.as_str()));
    let prompt = walk.prompt(&place);
    assert_eq!(prompt, format!("{RESUMED}\n\n{STOP_SHORT}"));
    let person = walk.person(&["status", &place]);
    assert!(
        person.contains("carries the conversation of start 1, lost")
            && person.contains("(its conversation's)"),
        "{person}"
    );

    let out = walk.agent(&place, json!([]));
    assert!(out.contains(", resumed"), "{out}");
    assert_eq!(sessions(&out), std::slice::from_ref(&session), "{out}");
    let acked = acked_transcripts(&walk);
    assert_eq!(acked.len(), 2, "{acked:?}");
    for (ack, path) in &acked {
        assert_eq!(
            file_id(path),
            file_id(&transcript),
            "{ack} names another file"
        );
    }
    assert_eq!(
        transcripts(walk.account()),
        std::slice::from_ref(&transcript)
    );
    let text = std::fs::read_to_string(&transcript).expect("the transcript");
    let turns: Vec<Value> = text
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("a JSON record"))
        .filter(|record| record["type"] == "user" && record["message"]["content"].is_string())
        .map(|record| record["message"]["content"].clone())
        .collect();
    assert_eq!(
        turns.len(),
        3,
        "the stage's prompt, the person's, then flume's: {text}"
    );
    assert_eq!(turns[1], json!("a person's word in the talk"), "{text}");
    assert_eq!(turns[2], json!(prompt), "{text}");

    walk.until(&place, "talk lost again", |status| {
        stage(status, "talk")["state"] == "lost"
    });
    walk.flume(&["done", &place]);
    walk.until(&place, "done and tidied", Value::is_null);
}

#[test]
fn a_lost_talk_whose_transcript_is_gone_starts_afresh_with_flumes_words() {
    // S115, 49, 179: a talk's agent ends and the stage is read lost; its
    // transcript is then removed. The caller's retry starts the stage at
    // once, no process running its session, afresh: its command resumes
    // nothing, its prompt holds flume's words for a lost stage, and status
    // says why the conversation could not be carried, naming
    // `CLAUDE_CONFIG_DIR` and no path (core.30).
    let walk = Walk::new("walk-lost-talk-gone");
    let place = launch_talk(&walk, "two");
    let (_, transcript) = talk_lost(&walk, &place);
    std::fs::remove_file(&transcript).expect("the transcript removed");

    walk.flume(&["retry", &place, "--stage", "talk"]);
    let status = walk.until(&place, "talk started again", |status| {
        stage(status, "talk")["starts"] == 2 && !status["agent"].is_null()
    });
    assert!(status["waits_on_session"].is_null(), "{status}");
    assert_eq!(
        status["agent"]["conversation"],
        json!({ "from": 1, "cause": "lost", "carried": false, "why": UNREADABLE }),
        "{status}"
    );
    assert!(
        !status
            .to_string()
            .contains(&*walk.account().to_string_lossy()),
        "status holds a configuration home's path: {status}"
    );
    assert_eq!(walk.resumes(&place), None, "{}", walk.printed(&place));
    let prompt = walk.prompt(&place);
    assert!(
        prompt.starts_with("Talk with the person about the parser.") && prompt.contains(LOST),
        "{prompt}"
    );
    let person = walk.person(&["status", &place]);
    assert!(
        person.contains(&format!("starts afresh after start 1, lost: {UNREADABLE}")),
        "{person}"
    );

    let out = walk.agent(&place, json!([]));
    assert!(!out.contains(", resumed"), "{out}");
    walk.until(&place, "talk lost again", |status| {
        stage(status, "talk")["state"] == "lost"
    });
    walk.flume(&["done", &place]);
    walk.until(&place, "done and tidied", Value::is_null);
}

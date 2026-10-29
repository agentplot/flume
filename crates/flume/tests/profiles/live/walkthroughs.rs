//! The fix and change walkthroughs of the check-tools layer with the
//! pinned Claude Code under herdr in place of the stand-in, and a code
//! stage waiting on its suite with its pane quiet (39, S3). Each brief
//! asks for what the stand-in's script acted; the kinds' own prompts say
//! the rest.

use super::walk::stage;
use super::Live;
use std::path::Path;
use std::time::{Duration, Instant};

/// The label of the workspace of the walkthrough's repository and main.
fn workspace(live: &Live) -> String {
    let name = live.walk.repository.name();
    let repository = Path::new(&name)
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or(name.clone());
    format!("{repository} main")
}

#[test]
fn a_fix_lands_under_herdr_with_claude_code() {
    // S1: the agent commits; flume submits the place, sluice lands it on
    // the remote, and flume tidies the place, its branch, and its tab and
    // workspace in herdr.
    let live = Live::new("live-fix");
    let place = live.walk.launch(
        "fix",
        "one",
        &[
            "--brief",
            "Create the file parser.rs at the repository's root holding exactly the line `fn parse() {}`, and commit it with git.",
        ],
    );
    live.until(&place, "its code stage begun", 5, |status| {
        status["agent"]["started"] == "yes"
    });
    assert!(
        live.workspaces().contains(&workspace(&live)),
        "{:?}",
        live.workspaces()
    );
    live.until_landed(&place, 20);

    let submissions = live.walk.submissions();
    assert_eq!(submissions.len(), 1, "{submissions:?}");
    assert_eq!(submissions[0]["state"], "landed", "{}", submissions[0]);
    let parser = live.walk.on_remote("parser.rs").unwrap_or_default();
    assert!(parser.contains("fn parse()"), "parser.rs: {parser:?}");
    assert!(!Path::new(&place).exists(), "the place is left");
    assert!(
        !live.walk.has_branch("fix/one"),
        "the place's branch is left"
    );
    assert!(
        !live.workspaces().contains(&workspace(&live)),
        "the base's workspace is left: {:?}",
        live.workspaces()
    );
}

#[test]
fn a_change_runs_construct_code_and_verify_under_herdr_with_claude_code() {
    // S2: construct writes the change, code ticks its tasks, and verify
    // commits its report and archives the change; the place lands with
    // the report in the archive.
    let live = Live::new("live-change");
    let place = live.walk.launch(
        "change",
        "add-parser",
        &[
            "--brief",
            "Add a parser: a file parser.rs at the repository's root holding `pub fn parse(input: &str) -> &str { input }`. Keep the change small: one requirement, one or two tasks.",
        ],
    );
    let status = live.until(&place, "its code stage", 30, |status| {
        status["current"] == "code"
    });
    assert_eq!(
        stage(&status, "construct")["state"],
        "delivered",
        "{status}"
    );
    let status = live.until(&place, "its verify stage", 30, |status| {
        status["current"] == "verify"
    });
    let ticked = status["tasks"]
        .as_array()
        .into_iter()
        .flatten()
        .all(|task| task["ticked"] == true);
    assert!(ticked, "verify began with a task unticked: {status}");
    live.until_landed(&place, 30);

    let files = live.walk.remote_files();
    assert!(
        files.iter().any(|file| {
            file.starts_with("openspec/changes/archive/") && file.ends_with("-add-parser/verify.md")
        }),
        "no report in the archive: {files:?}"
    );
    assert!(
        !files
            .iter()
            .any(|file| file.starts_with("openspec/changes/add-parser/")),
        "the change is left unarchived: {files:?}"
    );
    assert!(files.contains(&"parser.rs".to_string()), "{files:?}");
    assert!(!live.walk.has_branch("change/add-parser"));
}

#[test]
fn s3_a_code_stage_waiting_on_its_suite_is_not_ended_while_its_pane_is_quiet() {
    // 39, S3: the agent starts a two-minute suite in the background and
    // ends its turn; its pane is quiet with no deliverable, and no settle
    // ends the stage, sends the agent back or reports it stuck. Once the
    // suite passes the agent commits, and the place lands.
    let live = Live::new("live-quiet");
    let place = live.walk.launch(
        "fix",
        "suite",
        &[
            "--brief",
            "Run `sh suite.sh` as a background task: it takes about two minutes. Do not wait for it in the foreground or poll it: end your turn while it runs. Once it reports that the suite passed, create the file parser.rs at the repository's root holding exactly the line `fn parse() {}`, and commit it with git.",
        ],
    );
    let quiet = live.until(&place, "its agent quiet on its suite", 5, |status| {
        status["agent"]["started"] == "yes"
            && status["agent"]["state"] == "waiting"
            && status["agent"]["own_work"] == "yes"
    });
    assert_eq!(quiet["delivered"], false, "{quiet}");

    let began = Instant::now();
    while began.elapsed() < Duration::from_secs(60) {
        live.walk.settle(&place);
        let status = live.walk.status(&place);
        assert_eq!(status["current"], "code", "{status}");
        let code = stage(&status, "code");
        assert!(
            code["state"] == "begun" || code["state"] == "started",
            "the code stage ended or changed while its suite ran: {status}"
        );
        assert_eq!(code["starts"], 1, "the agent was sent back: {status}");
        assert!(!status["agent"].is_null(), "the agent was ended: {status}");
        if status["delivered"] == true {
            break;
        }
        std::thread::sleep(Duration::from_secs(5));
    }
    live.until_landed(&place, 15);
    let parser = live.walk.on_remote("parser.rs").unwrap_or_default();
    assert!(parser.contains("fn parse()"), "parser.rs: {parser:?}");
}

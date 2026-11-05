//! The fix and change walkthroughs of the check-tools layer with the
//! pinned Claude Code under herdr in place of the stand-in, a code stage
//! waiting on its suite with its pane quiet (39, S3), a fix begun in
//! the mode its host gives (162, S66), a fix whose code agent stops at
//! its account's usage limit going on in its own conversation under the
//! pool's other account (S78), a fix whose code agent's pane holds the
//! place's line under its prompt (56, 60, C.4), and a fix whose host's own
//! settings for the program reach its code agent's session (195, S100,
//! S102). Each brief asks for what the stand-in's script acted; the kinds'
//! own prompts say the rest.

use super::common::git;
use super::walk::{now, session_limit_after, stage};
use super::Live;
use flume::profiles::agent::claude::record::Record;
use flume::profiles::terminal::herdr::herdr_name;
use serde_json::{json, Value};
use std::path::Path;
use std::time::{Duration, Instant};

/// A fix's brief whose agent, in its first turn, waits a minute in one
/// foreground call on the repository's `step.sh`, since the pinned release
/// refuses a bare `sleep` there, before it commits.
const STEP_FIRST: &str = "Do this in two steps, in order. First run `sh step.sh` with the Bash tool in the foreground, never in the background: it takes about a minute, so wait for it to end. Only then create the file parser.rs at the repository's root holding exactly the line `fn parse() {}`, and commit it with git.";

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
    // The agent's Stop hook settles the place the moment it is quiet, so
    // a trivial fix can be submitted, landed and tidied between two of
    // these reads: begun or already tidied both answer.
    let begun = live.until(&place, "its code stage begun or tidied", 5, |status| {
        status.is_null() || status["agent"]["started"] == "yes"
    });
    if !begun.is_null() {
        // The workspace stands while the place does; read the place again
        // after the workspaces, so a tidy between the two is no failure.
        let workspaces = live.workspaces();
        assert!(
            workspaces.contains(&workspace(&live)) || live.walk.status(&place).is_null(),
            "the base's workspace is missing while the place lives: {workspaces:?}"
        );
    }
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

#[test]
fn s66_a_fix_begins_in_the_mode_its_host_gives_over_the_repositorys_own() {
    // S66, 161, 162, 169, C.4: the host's `[kinds.fix.agents]` gives
    // `ask`, which the start passes as `--permission-mode manual` over the
    // repository's own `defaultMode` of `acceptEdits`. The session begins
    // in `default`, the pinned release's word for `manual`, only where the
    // flag reached it. The started ack is taken on the program and the
    // variables, and the UserPromptSubmit hook takes the start's first
    // prompt only once the `permission_mode` it reads back is the mode
    // recorded: code begins at its first start, nothing unsettled. Every
    // tool the fix uses is allowed, so the agent asks nothing in `ask`,
    // and the place lands.
    let live = Live::with_fix_agents("live-mode", "mode = \"ask\"\n");
    let launched = live.walk.flume(&[
        "launch",
        "--kind",
        "fix",
        "--name",
        "one",
        "--brief",
        "Create the file parser.rs at the repository's root holding exactly the line `fn parse() {}`, and commit it with git.",
    ]);
    let place = launched["launched"]["place"]["place"]
        .as_str()
        .expect("the place's location")
        .to_string();
    // Read from the launch's own answer, which no landing can outrun.
    let mode = json!({ "passed": "manual", "entry": null, "read_back": true });
    let agent = &launched["launched"]["place"]["agent"];
    assert_eq!(
        (&agent["settings"]["from"]["mode"], &agent["mode"]),
        (&json!({ "layer": "host-kind", "value": "ask" }), &mode),
        "{launched}"
    );
    // As in the fix's walkthrough, the place may be tidied between two
    // reads. A start begun in another mode would be ended and code started
    // afresh in the same mode, never begun, which `until` refuses at once,
    // so a landing too says the first start's prompt was taken.
    let begun = live.until(&place, "its code stage begun or tidied", 5, |status| {
        status.is_null() || stage(status, "code")["state"] == "begun"
    });
    if !begun.is_null() {
        assert_eq!(begun["agent"]["started"], "yes", "{begun}");
        assert_eq!(begun["agent"]["mode"], mode, "{begun}");
        assert_eq!(stage(&begun, "code")["starts"], 1, "{begun}");
        assert!(
            begun["unsettled"].as_array().is_none_or(Vec::is_empty),
            "{begun}"
        );
    }
    live.until_landed(&place, 20);

    let submissions = live.walk.submissions();
    assert_eq!(submissions.len(), 1, "{submissions:?}");
    assert_eq!(submissions[0]["state"], "landed", "{}", submissions[0]);
    let parser = live.walk.on_remote("parser.rs").unwrap_or_default();
    assert!(parser.contains("fn parse()"), "parser.rs: {parser:?}");
}

/// The records of the transcript at `path`, none where it is not written.
fn records(path: &str) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect()
}

/// A turn's text: its content's string, or its text blocks joined.
fn text(content: &Value) -> Option<String> {
    match content {
        Value::String(text) => Some(text.clone()),
        Value::Array(blocks) => Some(
            blocks
                .iter()
                .filter_map(|block| block["text"].as_str())
                .collect::<Vec<_>>()
                .join("\n"),
        ),
        _ => None,
    }
}

/// The id of a foreground Bash call running `step.sh` whose result the
/// transcript at `path` does not hold yet: the agent waits in its step.
fn stepping(path: &str) -> Option<String> {
    let records = records(path);
    let blocks = || {
        records
            .iter()
            .filter_map(|record| record["message"]["content"].as_array())
            .flatten()
    };
    blocks()
        .filter(|block| block["type"] == "tool_use" && block["name"] == "Bash")
        .filter(|call| call["input"]["run_in_background"] != true)
        .filter(|call| {
            call["input"]["command"]
                .as_str()
                .is_some_and(|line| line.contains("step.sh"))
        })
        .find(|call| {
            !blocks()
                .any(|block| block["type"] == "tool_result" && block["tool_use_id"] == call["id"])
        })
        .and_then(|call| call["id"].as_str().map(str::to_string))
}

#[test]
fn s78_a_spent_agents_conversation_goes_on_under_the_pools_other_account() {
    // S78, 146, 161, 179, C.4: on a fill-first pool of two accounts signed
    // in by the test account's token, each with a scratch configuration
    // home, the second's `projects` a link to the first's, the fix's code
    // agent starts under the first and, in its first turn, waits a minute
    // in one foreground call on the repository's `step.sh`, since the
    // pinned release refuses a bare `sleep` there. No test account reaches
    // its limit on demand, so the test ends that turn while the call runs
    // as a usage limit ends one: it runs the plugin's StopFailure hook for
    // that start with the input 2.1.296 builds for a session limit, and
    // interrupts the turn, which runs no Stop hook, as a turn the API
    // failed runs none. Once the agent is quiet flume ends it and starts
    // code under the second account, resuming the first's session: the
    // carried agent says it started in that session, its prompt is flume's
    // carry words alone, status gives the conversation carried and the
    // move with it, and the agent goes on in the first's transcript from
    // what the brief asked, which its prompt does not hold, and the place
    // lands.
    let live = Live::sharing_projects("live-carry");
    let place = live.walk.launch("fix", "one", &["--brief", STEP_FIRST]);
    // Settled until code has begun, its prompt received: the launch sends
    // the prompt only where the agent said it started by the time herdr
    // read it ready, and a settle that read no started ack leaves it to
    // the next, which no watch makes here (35, 38, 119).
    let status = live.until(&place, "its code stage begun", 5, |status| {
        status["agent"]["started"] == "yes" && stage(status, "code")["state"] == "begun"
    });
    assert_eq!(
        status["agent"]["account"]["name"], "test-account",
        "{status}"
    );
    let folder = live.folder(&place);
    let first = live.live_start(&place);
    let started = Record::at(&folder, &first.identity)
        .started()
        .expect("the first start's started ack");
    // The agent is in its step once one call to it still runs ten seconds
    // after it was first read: a call the program refused, or put in the
    // background, is answered at once.
    let began = Instant::now();
    let mut seen: Option<(String, Instant)> = None;
    loop {
        let call = stepping(&started.transcript_path);
        match &seen {
            Some((was, at)) if call.as_ref() == Some(was) => {
                if at.elapsed() >= Duration::from_secs(10) {
                    break;
                }
            }
            _ => seen = call.map(|call| (call, Instant::now())),
        }
        assert!(
            began.elapsed() < Duration::from_secs(300),
            "the code agent never came to its step: {}",
            live.walk.status(&place)
        );
        std::thread::sleep(Duration::from_secs(1));
    }

    // The turn ends at the session limit: StopFailure, as 2.1.296 builds
    // its input, and the turn interrupted, so its agent waits.
    let (_, limit) = session_limit_after(now());
    live.hook(
        &place,
        &first,
        "stop-failure",
        &json!({
            "session_id": started.session_id,
            "transcript_path": started.transcript_path,
            "cwd": place,
            "hook_event_name": "StopFailure",
            "error": "rate_limit",
            "last_assistant_message": limit,
        }),
    );
    live.herdr(&["agent", "send-keys", &herdr_name(&first.identity), "esc"]);

    // Read each second, as the carried agent may deliver within seconds
    // of its start.
    let status = live.until_every(
        &place,
        "code started again and begun",
        10,
        Duration::from_secs(1),
        |status| stage(status, "code")["starts"] == 2 && status["agent"]["started"] == "yes",
    );
    assert_eq!(
        status["agent"]["account"]["name"], "test-account-2",
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
            &moved["from"],
            &moved["to"],
            &moved["why"],
            &moved["carried"]
        ),
        (
            &json!("test-account"),
            &json!("test-account-2"),
            &json!("spent"),
            &json!(true)
        ),
        "{status}"
    );
    let carried = live.live_start(&place);
    assert_ne!(carried.identity, first.identity);
    let resumed = Record::at(&folder, &carried.identity)
        .started()
        .expect("the carried start's started ack");
    assert_eq!(
        (resumed.session_id.as_str(), resumed.source.as_str()),
        (started.session_id.as_str(), "resume"),
        "the carried agent began in the first's session"
    );
    for home in &live.homes {
        assert!(
            !status.to_string().contains(&*home.to_string_lossy()),
            "status holds a configuration home's path: {status}"
        );
    }
    live.until_landed(&place, 20);

    let parser = live.walk.on_remote("parser.rs").unwrap_or_default();
    assert!(parser.contains("fn parse()"), "parser.rs: {parser:?}");
    let turns: Vec<String> = records(&started.transcript_path)
        .iter()
        .filter(|record| record["type"] == "user")
        .filter_map(|record| text(&record["message"]["content"]))
        .collect();
    assert!(
        turns
            .iter()
            .any(|turn| turn.contains("Your last turn stopped at your account's usage limit")),
        "the carried prompt is not in the first's transcript: {turns:?}"
    );
}

/// The rows of a pane's screen beneath the last that holds the prompt's
/// input, `❯`: what the agent program pins under its prompt.
fn under_the_prompt(screen: &str) -> Vec<&str> {
    let rows: Vec<&str> = screen.lines().collect();
    rows.iter()
        .rposition(|row| row.trim_start().starts_with('❯'))
        .map(|at| rows[at + 1..].to_vec())
        .unwrap_or_default()
}

#[test]
fn a_fix_agents_pane_holds_its_place_line_under_the_prompt() {
    // Design 4, 5; 56, 60, C.4: the fix's code agent, in its first turn,
    // waits a minute in one foreground call on the repository's
    // `step.sh`. While it works, its pane, read by herdr as the screen
    // shows it, holds the place's line beneath the prompt's input, pinned
    // there by flume's plugin's module from `flume hook status-line`,
    // which the module runs at the session's start, after each tool call
    // and every ten seconds. Then the agent commits and the place lands.
    let live = Live::new("live-line");
    let place = live.walk.launch("fix", "one", &["--brief", STEP_FIRST]);
    live.until(&place, "its code stage begun", 5, |status| {
        status["agent"]["started"] == "yes" && stage(status, "code")["state"] == "begun"
    });
    let start = live.live_start(&place);
    let agent = live.herdr(&["agent", "get", &herdr_name(&start.identity)]);
    let pane = agent["agent"]["pane_id"]
        .as_str()
        .unwrap_or_else(|| panic!("herdr lists no pane for the code agent: {agent}"))
        .to_string();

    let line = "fix--one · fix@1 · code · run 1 ·";
    let began = Instant::now();
    loop {
        let screen = live.screen(&pane);
        if under_the_prompt(&screen)
            .iter()
            .any(|row| row.contains(line))
        {
            break;
        }
        assert!(
            began.elapsed() < Duration::from_secs(60),
            "the code agent's pane holds no `{line}` under its prompt:\n{screen}"
        );
        std::thread::sleep(Duration::from_secs(1));
    }
    // Read while code works on: its agent live, nothing delivered.
    let status = live.walk.status(&place);
    assert_eq!(status["current"], "code", "{status}");
    assert_eq!(stage(&status, "code")["state"], "begun", "{status}");
    assert!(!status["agent"].is_null(), "{status}");
    live.until_landed(&place, 20);

    let submissions = live.walk.submissions();
    assert_eq!(submissions.len(), 1, "{submissions:?}");
    assert_eq!(submissions[0]["state"], "landed", "{}", submissions[0]);
    let parser = live.walk.on_remote("parser.rs").unwrap_or_default();
    assert!(parser.contains("fn parse()"), "parser.rs: {parser:?}");
}

#[test]
fn a_hosts_own_settings_reach_its_fix_agents_session_and_commit() {
    // Design 15, 16; S100, S102, 195, C.4: the host names, for every
    // agent, a settings file of the kind a host's launcher gives its
    // people, whose `tui` is `fullscreen` and whose SessionStart hook
    // writes what the session gives it into a scratch file, and gives
    // Claude Code's attribution with no text and no session link for the
    // walkthrough's repository, as for a fork whose upstream refuses a
    // tagline. The launch names each key with its layer, the file's with
    // the file, and the start passes flume's own file, never the host's.
    // The fix's code agent, in its first turn, waits a minute on the
    // repository's `step.sh`, so its start is read while it works: its
    // started ack, written by flume's plugin's SessionStart hook, was
    // taken, and the host's hook ran in the same session, which only the
    // file's keys reaching the release by `--settings` could make it do.
    // The commit the agent made carries no `Co-Authored-By` trailer, which
    // the release's own words ask for where no attribution says otherwise,
    // and the place lands.
    let live = Live::with_host("live-own", |walk| {
        let ran = walk.repository.scratch.path("session-start.json");
        let file = walk.repository.scratch.path("people-settings.json");
        let settings = json!({
            "tui": "fullscreen",
            "hooks": {
                "SessionStart": [{
                    "hooks": [{ "type": "command", "command": format!("cat >> '{}'", ran.display()) }]
                }]
            }
        });
        std::fs::write(&file, format!("{settings:#}\n")).expect("the people's file");
        format!(
            "\n[agents.programs.claude]\nfile = {:?}\n\n[repos.{:?}.agents.programs.claude.settings]\nattribution = {{ commit = \"\", pr = \"\", sessionUrl = false }}\n",
            file.display().to_string(),
            walk.repository.name()
        )
    });
    let ran = live.walk.repository.scratch.path("session-start.json");
    let file = live
        .walk
        .repository
        .scratch
        .path("people-settings.json")
        .to_string_lossy()
        .into_owned();
    let launched = live.walk.flume(&[
        "launch", "--kind", "fix", "--name", "one", "--brief", STEP_FIRST,
    ]);
    let place = launched["launched"]["place"]["place"]
        .as_str()
        .expect("the place's location")
        .to_string();
    // Read from the launch's own answer, which no landing can outrun.
    let agent = &launched["launched"]["place"]["agent"];
    let from = &agent["settings"]["from"];
    assert_eq!(
        (
            &from["programs.claude.settings.tui"],
            &from["programs.claude.settings.attribution"],
        ),
        (
            &json!({ "layer": "host", "value": "\"fullscreen\"", "file": file }),
            &json!({ "layer": "host-repository", "value": "{\"commit\":\"\",\"pr\":\"\",\"sessionUrl\":false}" }),
        ),
        "{launched}"
    );
    let hooks = &from["programs.claude.settings.hooks"];
    assert_eq!(
        (&hooks["layer"], &hooks["file"]),
        (&json!("host"), &json!(file)),
        "{launched}"
    );
    let given: Value = serde_json::from_str(hooks["value"].as_str().unwrap_or_default())
        .unwrap_or_else(|_| panic!("the hooks' value as JSON: {launched}"));
    assert_eq!(
        given["SessionStart"][0]["hooks"][0]["command"],
        format!("cat >> '{}'", ran.display()),
        "{launched}"
    );
    let passed = agent["own"]["passed"].as_str().unwrap_or_default();
    assert!(
        passed.starts_with("--settings ") && !passed.contains(&file),
        "the start passes not its own file: {launched}"
    );

    live.until(&place, "its code stage begun", 5, |status| {
        status["agent"]["started"] == "yes" && stage(status, "code")["state"] == "begun"
    });
    let start = live.live_start(&place);
    let started = Record::at(&live.folder(&place), &start.identity)
        .started()
        .expect("the code agent's started ack");
    let began = Instant::now();
    loop {
        // Each run of the hook appends its input as it was given, which
        // need not end its line.
        let text = std::fs::read_to_string(&ran).unwrap_or_default();
        let sessions: Vec<Value> = serde_json::Deserializer::from_str(&text)
            .into_iter::<Value>()
            .map_while(Result::ok)
            .collect();
        if sessions.iter().any(|session| {
            session["hook_event_name"] == "SessionStart"
                && session["session_id"] == started.session_id
        }) {
            break;
        }
        assert!(
            began.elapsed() < Duration::from_secs(30),
            "the host's SessionStart hook never ran in the code agent's session {}: {sessions:?}",
            started.session_id
        );
        std::thread::sleep(Duration::from_secs(1));
    }
    live.until_landed(&place, 20);

    let submissions = live.walk.submissions();
    assert_eq!(submissions.len(), 1, "{submissions:?}");
    assert_eq!(submissions[0]["state"], "landed", "{}", submissions[0]);
    let parser = live.walk.on_remote("parser.rs").unwrap_or_default();
    assert!(parser.contains("fn parse()"), "parser.rs: {parser:?}");
    // The agent's own commits: sluice's merge, if any, is not one.
    let messages = git(
        &live.walk.remote,
        &[
            "log",
            "--no-merges",
            "--format=%B%x00",
            "main",
            "--",
            "parser.rs",
        ],
    );
    let messages: Vec<&str> = messages
        .split('\0')
        .map(str::trim)
        .filter(|message| !message.is_empty())
        .collect();
    assert!(!messages.is_empty(), "no commit of the agent's on main");
    for message in messages {
        assert!(
            !message.to_lowercase().contains("co-authored-by"),
            "the code agent's commit carries a trailer: {message:?}"
        );
    }
}

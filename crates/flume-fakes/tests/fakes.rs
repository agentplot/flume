//! The fakes' own behaviour: each answer settable, refusals and unknowns
//! among them, every call logged in order, and a clock that moves only
//! when told. Their contracts are the conformance suite's seam scenarios.

use flume_fakes::*;
use flume_seams::*;
use std::collections::BTreeMap;
use std::sync::Arc;

fn identity(stage: &str, start: u32) -> Identity {
    Identity {
        repository: "/r".into(),
        place: "fix--one".into(),
        stage: stage.into(),
        start,
    }
}

fn place_ref() -> PlaceRef {
    PlaceRef {
        repository: "/r".into(),
        location: "/r/places/fix--one".into(),
        branch: "fix/one".into(),
        base: "main".into(),
    }
}

#[test]
fn the_clock_moves_only_when_advanced_or_slept_on() {
    let host = FakeHost::default();
    let at = host.now();
    assert_eq!(host.now(), at);
    host.advance(30);
    assert_eq!(host.now(), at + 30);
    host.sleep(60);
    assert_eq!(host.now(), at + 90);
    assert_eq!(host.log.calls(), vec!["sleep 60"]);
}

#[test]
fn the_host_names_its_commands_and_forgets_them_on_a_restart() {
    let host = FakeHost::default();
    let first = host.command();
    assert!(host.alive(&first));
    host.restart();
    assert!(!host.alive(&first));
    assert!(host.alive(&host.command()));
    assert_ne!(host.secret(), host.secret());
    host.write_file("/f");
    assert_eq!(host.write_time("/f"), Some(host.now()));
    assert_eq!(host.write_time("/g"), None);
}

#[test]
fn the_hosts_bound_is_set() {
    let host = FakeHost::default();
    assert_eq!(host.bound(), None);
    host.set_bound(Some(2));
    assert_eq!(host.bound(), Some(2));
    host.set_bound(None);
    assert_eq!(host.bound(), None);
}

#[test]
fn the_callers_recipient_is_set() {
    let host = FakeHost::default();
    assert_eq!(host.recipient(), None);
    host.set_recipient(Some("operator"));
    assert_eq!(host.recipient(), Some("operator".into()));
    assert_eq!(host.log.count("recipient"), 2);
}

#[test]
fn the_room_lock_is_exclusive() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;
    let host = Arc::new(FakeHost::default());
    let guard = host.room();
    assert!(host.room_taken());
    let entered = Arc::new(AtomicBool::new(false));
    let other = {
        let host = host.clone();
        let entered = entered.clone();
        std::thread::spawn(move || {
            let _guard = host.room();
            entered.store(true, Ordering::SeqCst);
        })
    };
    std::thread::sleep(Duration::from_millis(50));
    assert!(
        !entered.load(Ordering::SeqCst),
        "a second taker entered a held room"
    );
    drop(guard);
    other.join().expect("the second taker");
    assert!(entered.load(Ordering::SeqCst));
    assert!(!host.room_taken());
    assert_eq!(host.log.count("room"), 2);
}

#[test]
fn every_call_is_logged_in_order() {
    let git = FakeGit::new();
    let base = git.init("/r", "main");
    let place = FakePlace::new(git.clone());
    let at = place_ref();
    place.make("/r", &base, &at.branch, &at.location);
    place.ready(&at, &base);
    place.read(&at);
    place.remove(&at);
    place.remove_branch("/r", &at.branch, "main");
    let calls: Vec<String> = place
        .log
        .calls()
        .iter()
        .map(|line| line.split(' ').next().unwrap().to_string())
        .collect();
    assert_eq!(
        calls,
        vec!["make", "ready", "read", "remove", "remove_branch"]
    );
}

#[test]
fn a_scripted_teardown_refusal_and_an_unreadable_place() {
    let git = FakeGit::new();
    let base = git.init("/r", "main");
    let place = FakePlace::new(git.clone());
    let at = place_ref();
    place.make("/r", &base, &at.branch, &at.location);
    place.fail_teardown(&at.location, "pre-remove", "busy");
    assert_eq!(
        place.remove(&at),
        Removed::Refused {
            step: "pre-remove".into(),
            words: "busy".into()
        }
    );
    place.unreadable(&at.location, "no git");
    assert!(matches!(place.read(&at), PlaceRead::CouldNotRead { .. }));
}

#[test]
fn the_agents_answers_are_settable_unknowns_among_them() {
    let host = Arc::new(FakeHost::default());
    let terminal = Arc::new(FakeTerminal::new(host.clone()));
    let agent = FakeAgent::new(terminal.clone());
    let who = identity("code", 1);
    let folder = "/state/fix--one";
    assert_eq!(agent.received(folder, &who, "go"), Received::Unknown);
    terminal.open(
        &place_ref(),
        "/state/fix--one",
        &AgentCommand::default(),
        &who,
        &OpenIn::Own,
        None,
    );
    agent.set_own_work(&who, Reading::Unknown);
    agent.set_asking(&who, Reading::Yes);
    assert_eq!(agent.own_work(folder, &who), Reading::Unknown);
    assert_eq!(agent.asking(folder, &who), Reading::Yes);
    assert!(matches!(
        agent.received(folder, &who, "go"),
        Received::NotReceived { .. }
    ));
    terminal.send(&who, "go");
    assert!(matches!(
        agent.received(folder, &who, "go"),
        Received::Received { .. }
    ));
    agent.received_unknown(&who);
    assert_eq!(agent.received(folder, &who, "go"), Received::Unknown);
    agent.refuse_act(&who, "submit");
    assert_eq!(
        agent.guard(folder, &who),
        Guarded::Refused {
            acts: vec!["submit".into()]
        }
    );
    assert_eq!(agent.spent(folder, &who), Spent::No);
    let limit = Spent::Yes {
        until: Some(host.now() + 3600),
        words: "a usage limit this test writes".into(),
    };
    agent.set_spent(&who, limit.clone());
    assert_eq!(agent.spent(folder, &who), limit);
    assert_eq!(agent.started(folder, &who), Reading::Yes);
    assert_eq!(agent.started_without(folder, &who), None);
    let unsettled = Unsettled {
        missing: vec!["AGENT_HOME".into()],
        ..Unsettled::default()
    };
    agent.set_started_without(&who, unsettled.clone());
    assert_eq!(agent.started_without(folder, &who), Some(unsettled));
    assert_eq!(
        agent.started(folder, &who),
        Reading::No,
        "an agent begun without its settings says it started nothing"
    );
    assert_eq!(
        agent.log.of("started_without").len(),
        2,
        "each reading logged"
    );
    assert_eq!(agent.ended(folder, &who), Reading::No);
    agent.set_ended(&who, Reading::Yes);
    assert_eq!(agent.ended(folder, &who), Reading::Yes);
    agent.no_send_back();
    agent.no_on_quiet();
    agent.no_guard();
    assert_eq!(agent.send_back(), Offered::NotOffered);
    assert_eq!(agent.on_quiet(), Offered::NotOffered);
    assert_eq!(agent.guard(folder, &who), Guarded::NotOffered);
    assert_eq!(
        agent.log.of("asking"),
        vec![format!("asking {who} in {folder}")]
    );
}

#[test]
fn the_agents_record_is_read_in_its_folder_alone() {
    let host = Arc::new(FakeHost::default());
    let terminal = Arc::new(FakeTerminal::new(host));
    let agent = FakeAgent::new(terminal.clone());
    let who = identity("code", 1);
    terminal.open(
        &place_ref(),
        "/state/fix--one",
        &AgentCommand::default(),
        &who,
        &OpenIn::Own,
        None,
    );
    agent.keep_record_in(&who, "/state/fix--one");
    assert_eq!(agent.asking("/state/fix--one", &who), Reading::No);
    assert_eq!(agent.asking("/state/fix--two", &who), Reading::Unknown);
    assert_eq!(agent.own_work("/state/fix--two", &who), Reading::Unknown);
    assert_eq!(agent.ended("/state/fix--two", &who), Reading::Unknown);
    assert_eq!(
        agent.received("/state/fix--two", &who, "go"),
        Received::Unknown
    );
}

#[test]
fn the_agents_modes_are_scripted() {
    // Unscripted, the fake maps no mode and reads none back, as the
    // seam's defaults do; scripted, it maps the four and its own values,
    // its command passing the value mapped and its log naming each
    // mapping (161, 169).
    let host = Arc::new(FakeHost::default());
    let terminal = Arc::new(FakeTerminal::new(host));
    let agent = FakeAgent::new(terminal.clone());
    let full = ModeEntry::Flume(Mode::Full);
    assert!(agent.mode(&full).is_err(), "an unscripted fake mapped full");
    assert_eq!(agent.mode_read_back(), Offered::NotOffered);
    agent.map_mode(Mode::Full, "all");
    agent.map_mode(Mode::Ask, "ask-each");
    agent.own_mode("careful");
    assert_eq!(agent.mode(&full), Ok("all".to_string()));
    assert_eq!(
        agent.mode(&ModeEntry::Own("careful".into())),
        Ok("careful".to_string())
    );
    let words = agent
        .mode(&ModeEntry::Flume(Mode::Edits))
        .expect_err("a mode left unscripted");
    assert!(words.contains("`edits`"), "{words}");
    let words = agent
        .mode(&ModeEntry::Own("yolo".into()))
        .expect_err("an own value it does not name");
    assert!(
        words.contains("`yolo`") && words.contains("careful"),
        "{words}"
    );
    assert_eq!(
        agent.log.of("mode"),
        vec![
            "mode full",
            "mode full",
            "mode careful",
            "mode edits",
            "mode yolo"
        ]
    );

    let folder = "/state/fix--one";
    let opened = |who: &Identity, mode: Option<&str>| {
        let command = agent.command(
            "/state/fix--one",
            &Settings::default(),
            mode,
            None,
            &BTreeMap::new(),
            &place_ref(),
            who,
            &Secret("s".into()),
            "go",
        );
        terminal.open(&place_ref(), folder, &command, who, &OpenIn::Own, None);
        command
    };
    let who = identity("code", 1);
    let command = opened(&who, Some("all"));
    assert!(
        command
            .args
            .windows(2)
            .any(|pair| pair == ["--mode", "all"]),
        "{:?}",
        command.args
    );

    // Begun in another mode, read only once the fake reads the mode back.
    agent.set_began_in(&who, "ask-each");
    assert_eq!(
        agent.started_without(folder, &who),
        None,
        "a fake that reads no mode back compares none"
    );
    assert_eq!(agent.started(folder, &who), Reading::Yes);
    agent.read_mode_back();
    assert_eq!(agent.mode_read_back(), Offered::Offered);
    let began = ModeBegan {
        asked: "all".into(),
        began: Some("ask-each".into()),
    };
    assert_eq!(
        agent.started_without(folder, &who),
        Some(Unsettled {
            mode: Some(began.clone()),
            ..Unsettled::default()
        })
    );
    assert_eq!(
        agent.started(folder, &who),
        Reading::No,
        "an agent begun in another mode says it started nothing"
    );
    agent.set_started_without(
        &who,
        Unsettled {
            missing: vec!["AGENT_HOME".into()],
            ..Unsettled::default()
        },
    );
    assert_eq!(
        agent.started_without(folder, &who),
        Some(Unsettled {
            missing: vec!["AGENT_HOME".into()],
            mode: Some(began),
            ..Unsettled::default()
        }),
        "a variable and the mode named together"
    );

    // Begun in the mode asked, or where its command passed none.
    let same = identity("code", 2);
    opened(&same, Some("all"));
    agent.set_began_in(&same, "all");
    assert_eq!(agent.started_without(folder, &same), None);
    let bare = identity("code", 3);
    let none = opened(&bare, None);
    assert!(!none.args.iter().any(|arg| arg == "--mode"));
    agent.set_began_in(&bare, "ask-each");
    assert_eq!(
        agent.started_without(folder, &bare),
        None,
        "a start that passed no mode compares none"
    );
}

#[test]
fn the_agent_names_its_mode_at_its_first_prompt() {
    // Scripted to name its mode only at its start's first prompt: its word
    // that it started is taken whatever mode it began in, and once a prompt
    // reached it, it names the mode and does not receive the prompt; begun
    // in the mode asked, it receives it (161).
    let host = Arc::new(FakeHost::default());
    let terminal = Arc::new(FakeTerminal::new(host));
    let agent = FakeAgent::new(terminal.clone());
    agent.map_mode(Mode::Full, "all");
    agent.read_mode_at_prompt();
    assert_eq!(agent.mode_read_back(), Offered::Offered);
    let folder = "/state/fix--one";
    let opened = |who: &Identity| {
        let command = agent.command(
            "/state/fix--one",
            &Settings::default(),
            Some("all"),
            None,
            &BTreeMap::new(),
            &place_ref(),
            who,
            &Secret("s".into()),
            "go",
        );
        terminal.open(&place_ref(), folder, &command, who, &OpenIn::Own, None);
    };
    let who = identity("code", 1);
    opened(&who);
    agent.set_began_in(&who, "ask-each");
    assert_eq!(agent.started(folder, &who), Reading::Yes);
    assert_eq!(
        agent.started_without(folder, &who),
        None,
        "the mode named before any prompt reached it"
    );
    assert_eq!(terminal.send(&who, "go"), Sent::Sent);
    assert_eq!(
        agent.started_without(folder, &who),
        Some(Unsettled {
            mode: Some(ModeBegan {
                asked: "all".into(),
                began: Some("ask-each".into()),
            }),
            ..Unsettled::default()
        })
    );
    assert_eq!(
        agent.started(folder, &who),
        Reading::Yes,
        "its word that it started still stands"
    );
    assert!(
        matches!(
            agent.received(folder, &who, "go"),
            Received::NotReceived { .. }
        ),
        "the prompt held back was received"
    );

    let same = identity("code", 2);
    opened(&same);
    agent.set_began_in(&same, "all");
    assert_eq!(terminal.send(&same, "go"), Sent::Sent);
    assert_eq!(agent.started_without(folder, &same), None);
    assert!(matches!(
        agent.received(folder, &same, "go"),
        Received::Received { .. }
    ));
}

#[test]
fn the_agents_carry_is_scripted_by_each_accounts_home() {
    // Unscripted, the fake carries nothing, as the seam's default; scripted,
    // it answers yes with the session to one account and no with the words
    // to another, no naming the variable to an account none is scripted
    // for and for a start kept in another folder, and logs each answer
    // (128, 179).
    let host = Arc::new(FakeHost::default());
    let terminal = Arc::new(FakeTerminal::new(host));
    let agent = FakeAgent::new(terminal);
    let folder = "/state/fix--one";
    let stopped = identity("code", 1);
    let home = |home: &str| BTreeMap::from([(FakeAgent::HOME.to_string(), home.to_string())]);
    assert_eq!(
        agent.carry(folder, &stopped, &home("/scratch/work-2")),
        Carry::NotOffered
    );
    agent.carry_to(
        "/scratch/work-2",
        Carry::Yes {
            session: "s-1".into(),
        },
    );
    agent.carry_to(
        "/scratch/work-3",
        Carry::No {
            why: "work-3 shares nothing".into(),
        },
    );
    assert_eq!(
        agent.carry(folder, &stopped, &home("/scratch/work-2")),
        Carry::Yes {
            session: "s-1".into()
        }
    );
    assert_eq!(
        agent.carry(folder, &stopped, &home("/scratch/work-3")),
        Carry::No {
            why: "work-3 shares nothing".into()
        }
    );
    match agent.carry(folder, &stopped, &home("/scratch/work-4")) {
        Carry::No { why } => {
            assert!(why.contains(FakeAgent::HOME), "{why}");
            assert!(!why.contains("/scratch"), "{why}");
        }
        other => panic!("an account none is scripted for: {other:?}"),
    }
    assert!(matches!(
        agent.carry(folder, &stopped, &BTreeMap::new()),
        Carry::No { .. }
    ));
    agent.keep_record_in(&stopped, folder);
    assert!(matches!(
        agent.carry("/state/fix--two", &stopped, &home("/scratch/work-2")),
        Carry::No { .. }
    ));
    assert_eq!(
        agent.log.of("carry"),
        vec![
            format!("carry {stopped} in {folder} to /scratch/work-2: not offered"),
            format!("carry {stopped} in {folder} to /scratch/work-2: yes s-1"),
            format!("carry {stopped} in {folder} to /scratch/work-3: no"),
            format!("carry {stopped} in {folder} to /scratch/work-4: no"),
            format!("carry {stopped} in {folder} to no home: no"),
            format!("carry {stopped} in /state/fix--two to /scratch/work-2: no"),
        ]
    );
}

#[test]
fn the_agents_command_resumes_the_session_it_is_given() {
    // The command passes `--resume <session>` where it is given one, and
    // its log names it (88, 179).
    let host = Arc::new(FakeHost::default());
    let agent = FakeAgent::new(Arc::new(FakeTerminal::new(host)));
    let command = |who: &Identity, resume: Option<&str>| {
        agent.command(
            "/state/fix--one",
            &Settings::default(),
            None,
            resume,
            &BTreeMap::new(),
            &place_ref(),
            who,
            &Secret("s".into()),
            "go",
        )
    };
    let carried = identity("code", 2);
    let args = command(&carried, Some("s-1")).args;
    assert!(
        args.windows(2).any(|pair| pair == ["--resume", "s-1"]),
        "{args:?}"
    );
    let fresh = identity("code", 3);
    let args = command(&fresh, None).args;
    assert!(!args.iter().any(|arg| arg == "--resume"), "{args:?}");
    assert_eq!(
        agent.log.of("command"),
        vec![
            format!("command {carried} --resume s-1"),
            format!("command {fresh}")
        ]
    );
}

#[test]
fn the_agents_word_names_the_session_it_resumes() {
    // A start that resumes a session says it started in that session
    // unless scripted otherwise; in another, it says it started nothing
    // and began in another conversation; a start that resumes none
    // compares none (161, 179).
    let host = Arc::new(FakeHost::default());
    let terminal = Arc::new(FakeTerminal::new(host));
    let agent = FakeAgent::new(terminal.clone());
    let folder = "/state/fix--one";
    let opened = |who: &Identity, resume: Option<&str>| {
        let command = agent.command(
            "/state/fix--one",
            &Settings::default(),
            None,
            resume,
            &BTreeMap::new(),
            &place_ref(),
            who,
            &Secret("s".into()),
            "go",
        );
        terminal.open(&place_ref(), folder, &command, who, &OpenIn::Own, None);
    };
    let carried = identity("code", 2);
    opened(&carried, Some("s-1"));
    assert_eq!(agent.started(folder, &carried), Reading::Yes);
    assert_eq!(agent.started_without(folder, &carried), None);
    agent.set_session(&carried, "s-1");
    assert_eq!(agent.started(folder, &carried), Reading::Yes);
    agent.set_session(&carried, "s-2");
    assert_eq!(
        agent.started_without(folder, &carried),
        Some(Unsettled {
            session: true,
            ..Unsettled::default()
        })
    );
    assert_eq!(
        agent.started(folder, &carried),
        Reading::No,
        "an agent begun in another session says it started nothing"
    );

    let fresh = identity("code", 3);
    opened(&fresh, None);
    agent.set_session(&fresh, "s-2");
    assert_eq!(agent.started_without(folder, &fresh), None);
    assert_eq!(agent.started(folder, &fresh), Reading::Yes);
}

#[test]
fn a_carried_start_scripted_to_end_before_its_word() {
    // Scripted so, a start that resumes a session ends before its word
    // that it started: it reads not started and ended, and the terminal
    // reads it stopped; a start that resumes none is not touched (146,
    // S79).
    let host = Arc::new(FakeHost::default());
    let terminal = Arc::new(FakeTerminal::new(host));
    let agent = FakeAgent::new(terminal.clone());
    agent.carried_starts_end();
    let folder = "/state/fix--one";
    let opened = |who: &Identity, resume: Option<&str>| {
        let command = agent.command(
            "/state/fix--one",
            &Settings::default(),
            None,
            resume,
            &BTreeMap::new(),
            &place_ref(),
            who,
            &Secret("s".into()),
            "go",
        );
        terminal.open(&place_ref(), folder, &command, who, &OpenIn::Own, None);
    };
    let carried = identity("code", 2);
    opened(&carried, Some("s-1"));
    assert_ne!(terminal.state(&carried), AgentState::Stopped);
    assert_eq!(agent.started(folder, &carried), Reading::No);
    assert_eq!(terminal.state(&carried), AgentState::Stopped);
    assert_eq!(agent.ended(folder, &carried), Reading::Yes);

    let fresh = identity("code", 3);
    opened(&fresh, None);
    assert_eq!(agent.started(folder, &fresh), Reading::Yes);
    assert_eq!(agent.ended(folder, &fresh), Reading::No);
    assert_ne!(terminal.state(&fresh), AgentState::Stopped);
}

#[test]
fn the_agent_gives_both_stop_shorts() {
    let host = Arc::new(FakeHost::default());
    let agent = FakeAgent::new(Arc::new(FakeTerminal::new(host)));
    let way = agent.stop_short(&identity("code", 1));
    assert!(way.contains("flume needs \""), "{way}");
    assert!(way.contains("flume needs --found"), "{way}");
}

#[test]
fn the_terminals_states_follow_the_fake_clock() {
    let host = Arc::new(FakeHost::default());
    let terminal = FakeTerminal::new(host.clone());
    let who = identity("code", 1);
    terminal.comes_up(AgentState::Unknown);
    terminal.open(
        &place_ref(),
        "/state/fix--one",
        &AgentCommand::default(),
        &who,
        &OpenIn::Own,
        None,
    );
    terminal.schedule(&who, host.now() + 240, AgentState::Waiting);
    assert_eq!(terminal.state(&who), AgentState::Unknown);
    host.advance(240);
    assert_eq!(terminal.state(&who), AgentState::Waiting);
    terminal.refuse_close(&who);
    assert!(matches!(terminal.close(&who), Closed::Refused { .. }));
    terminal.allow_close(&who);
    assert_eq!(terminal.close(&who), Closed::Closed);
    assert_eq!(terminal.state(&who), AgentState::Stopped);
    terminal.cannot_send();
    assert!(matches!(terminal.send(&who, "x"), Sent::Refused { .. }));
    assert_eq!(terminal.session(), None);
    terminal.set_session(Some("pane-3"));
    assert_eq!(terminal.session().as_deref(), Some("pane-3"));
}

fn submitted(answer: Submitted) -> Submission {
    match answer {
        Submitted::Submitted { submission } => submission,
        other => panic!("submit answered {other:?}"),
    }
}

fn outcomes(answer: Inbox) -> Vec<SubmissionOutcome> {
    match answer {
        Inbox::Outcomes { outcomes, .. } => outcomes,
        other => panic!("the inbox answered {other:?}"),
    }
}

#[test]
fn submit_holds_no_wait_and_a_step_decides() {
    let git = FakeGit::new();
    let base = git.init("/r", "main");
    let queue = FakeMergeQueue::new(git.clone());
    let at = place_ref();
    git.set_branch("/r", "fix/one", &base);
    let head = git.commit("/r", "fix/one", &["a"]);
    let submission = submitted(queue.submit(&at, &head));
    assert!(submission.state.open());
    assert_eq!(outcomes(queue.inbox(&at)), vec![]);
    assert_eq!(queue.step("/r", "main"), Stepped::Stepped);
    let landed = outcomes(queue.inbox(&at));
    assert!(
        matches!(landed.as_slice(), [SubmissionOutcome::Landed { .. }]),
        "{landed:?}"
    );
    assert_eq!(outcomes(queue.inbox(&at)), vec![], "delivered twice");
    assert_eq!(
        queue
            .log
            .calls()
            .iter()
            .map(|line| line.split(' ').next().unwrap())
            .collect::<Vec<_>>(),
        vec!["submit", "inbox", "step", "inbox", "inbox"]
    );
}

#[test]
fn the_merge_queues_outcomes_are_planned() {
    let git = FakeGit::new();
    let base = git.init("/r", "main");
    let queue = FakeMergeQueue::new(git.clone());
    let at = place_ref();
    git.set_branch("/r", "fix/one", &base);
    let head = git.commit("/r", "fix/one", &["a"]);
    queue.plan_submits(vec![
        Plan::Fail("test x".into()),
        Plan::Conflict(vec!["a".into()], "conflicts with main".into()),
        Plan::Withdraw,
        Plan::Refuse("no such base".into()),
        Plan::Hold,
    ]);
    for _ in 0..3 {
        submitted(queue.submit(&at, &head));
        queue.step("/r", "main");
    }
    assert_eq!(
        outcomes(queue.inbox(&at)),
        vec![
            SubmissionOutcome::Failed {
                words: "test x".into()
            },
            SubmissionOutcome::Conflict {
                files: vec!["a".into()],
                base: base.clone(),
                words: "conflicts with main".into()
            },
        ],
        "a withdrawal is never in the inbox"
    );
    assert!(matches!(
        queue.read_submission("/r", "fix/one").read,
        SubmissionRead::Found {
            submission: Submission {
                state: SubmissionState::Withdrawn,
                ..
            }
        }
    ));
    assert_eq!(
        queue.submit(&at, &head),
        Submitted::Refused {
            reason: "no such base".into()
        }
    );
    submitted(queue.submit(&at, &head));
    queue.step("/r", "main");
    assert_eq!(outcomes(queue.inbox(&at)), vec![], "a held submission");
    queue.decide("/r", "fix/one", Plan::Land);
    assert!(matches!(
        outcomes(queue.inbox(&at)).as_slice(),
        [SubmissionOutcome::Landed { .. }]
    ));
    queue.moves_on_its_own();
    assert_eq!(queue.step("/r", "main"), Stepped::MovesOnItsOwn);
    queue.unreadable("down");
    assert!(matches!(queue.inbox(&at), Inbox::CouldNotRead { .. }));
    assert!(matches!(
        queue.step("/r", "main"),
        Stepped::CouldNotRead { .. }
    ));
    assert!(matches!(
        queue.read_submission("/r", "fix/one").read,
        SubmissionRead::CouldNotRead { .. }
    ));
    queue.set_base(
        "/r",
        "main",
        BaseRevision::Refused {
            reason: "fetch failed".into(),
        },
    );
    assert!(matches!(
        queue.base_revision("/r", "main"),
        BaseRevision::Refused { .. }
    ));
}

#[test]
fn a_conflict_at_submit_is_answered_and_never_in_the_inbox() {
    let git = FakeGit::new();
    let base = git.init("/r", "main");
    let queue = FakeMergeQueue::new(git.clone());
    let at = place_ref();
    git.set_branch("/r", "fix/one", &base);
    let head = git.commit("/r", "fix/one", &["a"]);
    queue.plan_submits(vec![Plan::ConflictAtSubmit(
        vec!["a".into()],
        "conflict on main: a with #1".into(),
    )]);
    let Submitted::Decided {
        submission,
        outcome,
    } = queue.submit(&at, &head)
    else {
        panic!("a conflict at submit was not answered decided");
    };
    assert_eq!(
        outcome,
        SubmissionOutcome::Conflict {
            files: vec!["a".into()],
            base,
            words: "conflict on main: a with #1".into()
        }
    );
    assert!(matches!(
        queue.read_submission("/r", "fix/one").read,
        SubmissionRead::Found { submission: found } if found == submission
    ));
    queue.step("/r", "main");
    assert_eq!(outcomes(queue.inbox(&at)), vec![], "an outcome delivered");
}

#[test]
fn a_request_is_an_entry_only_once_taken_in_and_its_outcome_goes_to_the_request() {
    let git = FakeGit::new();
    let base = git.init("/r", "main");
    let queue = FakeMergeQueue::new(git.clone());
    let at = place_ref();
    git.set_branch("/r", "fix/one", &base);
    let head = git.commit("/r", "fix/one", &["a"]);
    let request = |then, to_place| Plan::Request {
        then: Box::new(then),
        to_place,
    };
    queue.plan_submits(vec![
        request(Plan::Fail("check x failed".into()), true),
        request(Plan::Land, false),
    ]);
    assert_eq!(
        queue.submit(&at, &head),
        Submitted::Requested {
            request: "1".into()
        }
    );
    assert_eq!(
        queue.read_submission("/r", "fix/one"),
        SubmissionRead::None.into()
    );
    queue.step("/r", "main");
    assert_eq!(
        queue.read_submission("/r", "fix/one"),
        SubmissionRead::None.into(),
        "a round takes no request in"
    );
    let entry = queue.take_in("/r", "fix/one").expect("the request");
    assert_eq!(entry.request.as_deref(), Some("1"));
    assert_eq!(queue.take_in("/r", "fix/one"), None, "taken in once");
    queue.step("/r", "main");
    assert_eq!(
        outcomes(queue.inbox(&at)),
        vec![SubmissionOutcome::Failed {
            words: "check x failed".into()
        }],
        "an outcome left for the place as well"
    );
    assert_eq!(
        queue.submit(&at, &head),
        Submitted::Requested {
            request: "1".into()
        },
        "the open request, updated"
    );
    queue.take_in("/r", "fix/one").expect("the request again");
    queue.step("/r", "main");
    assert!(matches!(
        queue.read_submission("/r", "fix/one").read,
        SubmissionRead::Found { submission }
            if matches!(submission.state, SubmissionState::Landed { .. })
                && submission.request.as_deref() == Some("1")
    ));
    assert_eq!(
        outcomes(queue.inbox(&at)),
        vec![],
        "delivered to the request"
    );
}

#[test]
fn the_change_is_scripted_as_groups() {
    let git = FakeGit::new();
    let base = git.init("/r", "main");
    let change = FakeChange::new(git.clone());
    change.set(
        "/r",
        &base,
        "one",
        FakeChange::groups(&[("1", "Parser", 2), ("2", "Tests", 1)], 2),
    );
    let ChangeRead::Read { facts } = change.read_change("/r", &base, "one") else {
        panic!("the change read");
    };
    assert_eq!(
        facts.groups,
        vec![
            TaskGroup {
                number: Some("1".into()),
                title: Some("Parser".into()),
            },
            TaskGroup {
                number: Some("2".into()),
                title: Some("Tests".into()),
            },
        ]
    );
    assert_eq!(
        facts
            .tasks
            .iter()
            .map(|task| (task.text.as_str(), task.ticked, task.group))
            .collect::<Vec<_>>(),
        vec![
            ("1.1 task", true, Some(0)),
            ("1.2 task", true, Some(0)),
            ("2.1 task", false, Some(1)),
        ]
    );
    assert_eq!(facts.first_unticked_group(), Some(1));
}

#[test]
fn the_kinds_and_change_are_set() {
    let git = FakeGit::new();
    let base = git.init("/r", "main");
    let kinds = FakeKinds::default();
    kinds.unreadable_binding("/r", "fetch failed");
    assert!(matches!(
        kinds.read_binding("/r"),
        BindingRead::CouldNotRead { .. }
    ));
    kinds.unreadable_kinds("/r", "bad toml");
    assert!(matches!(
        kinds.read_kinds("/r", &base),
        KindsRead::CouldNotRead { .. }
    ));
    let change = FakeChange::new(git.clone());
    change.set(
        "/r",
        &base,
        "one",
        ChangeFacts {
            exists: true,
            ..ChangeFacts::default()
        },
    );
    let later = git.commit("/r", "main", &["x"]);
    assert!(matches!(
        change.read_change("/r", &later, "one"),
        ChangeRead::Read { facts } if facts.exists
    ));
    change.unreadable("/r");
    assert!(matches!(
        change.read_change("/r", &later, "one"),
        ChangeRead::CouldNotRead { .. }
    ));
}

#[test]
fn the_store_runs_an_interleaved_act_before_the_next_write() {
    let store = Arc::new(FakeStore::default());
    let id = PlaceId::new("/r/places/fix--one");
    let other = store.clone();
    let moved = id.clone();
    store.before_next_write(move || {
        assert!(other.read(&moved).is_none());
    });
    assert_eq!(store.read_every().len(), 0);
    assert!(store.folder(&id).starts_with("/host/state/flume/"));
}

#[test]
fn one_resolver_from_profile_names_to_fakes() {
    let git = FakeGit::new();
    let kinds = Arc::new(FakeKinds::default());
    let resolver = FakeResolver::new(kinds);
    let place = Arc::new(FakePlace::new(git.clone()));
    resolver.bind_no_tool_place(place.clone());
    resolver.bind_place("trees", place.clone());
    resolver.bind_change("plans", Arc::new(FakeChange::new(git.clone())));
    resolver.bind_merge_queue("queue", Arc::new(FakeMergeQueue::new(git.clone())));
    let host = Arc::new(FakeHost::default());
    resolver.bind_agent(
        "agent",
        Arc::new(FakeAgent::new(Arc::new(FakeTerminal::new(host)))),
    );
    let binding = |place: Option<&str>| Binding {
        place: place.map(str::to_string),
        change: None,
        merge_queue: Some("queue".into()),
    };
    assert!(resolver.place("/r", &binding(None)).is_some());
    assert!(resolver.place("/r", &binding(Some("trees"))).is_some());
    assert!(resolver.place("/r", &binding(Some("other"))).is_none());
    assert_eq!(resolver.place_profiles(), vec!["trees".to_string()]);
    assert_eq!(resolver.log.of("place")[1], "place /r trees queue");
    assert!(resolver.change("plans").is_some());
    assert!(resolver.merge_queue("queue").is_some());
    assert!(resolver.merge_queue("other").is_none());
    assert!(resolver.agent("agent").is_some());
    assert!(resolver.agent("other").is_none());
}

#[test]
fn the_settings_of_the_host_and_of_a_repository_are_set() {
    let given = |model: &str| KindSettings {
        every_stage: Settings {
            model: Some(model.into()),
            ..Settings::default()
        },
        stages: Default::default(),
    };
    let host = FakeHost::default();
    host.set_settings(None, "change", given("opus"));
    host.set_settings(Some("/r"), "change", given("sonnet"));
    let in_r = host.settings("/r", "change");
    assert_eq!(in_r.kind, given("opus"));
    assert_eq!(in_r.repository, given("sonnet"));
    assert_eq!(
        host.settings("/s", "change").repository,
        KindSettings::default()
    );
    host.set_settings(None, "change", KindSettings::default());
    assert_eq!(host.settings("/r", "change").kind, KindSettings::default());
    assert_eq!(host.login("HOME").as_deref(), Some("/home/user"));
    host.set_login("ACCOUNT", Some("work"));
    assert_eq!(host.login("ACCOUNT").as_deref(), Some("work"));
    assert_eq!(host.login("NOT_SET"), None);

    let git = FakeGit::new();
    let base = git.init("/r", "main");
    let next = git.commit("/r", "main", &["next.rs"]);
    let kinds = FakeKinds::default();
    kinds.set_settings("/r", "change", given("opus"));
    kinds.set_settings_at("/r", &next, "change", given("sonnet"));
    let settings = |revision: &Revision| match kinds.read_kinds("/r", revision) {
        KindsRead::Read { settings, .. } => settings,
        other => panic!("read-kinds gave {other:?}"),
    };
    assert_eq!(settings(&base)["change"], given("opus"));
    assert_eq!(settings(&next)["change"], given("sonnet"));
}

#[test]
fn the_terminal_holds_recipients_sessions_by_name() {
    let host = Arc::new(FakeHost::default());
    let terminal = FakeTerminal::new(host);
    assert_eq!(terminal.session_state("ops"), None);
    assert_eq!(terminal.send_to("ops", "one"), ToSession::NotFound);
    terminal.add_session("ops", AgentState::Asking);
    assert_eq!(terminal.session_state("ops"), Some(AgentState::Asking));
    terminal.set_session_state("ops", AgentState::Waiting);
    assert_eq!(terminal.send_to("ops", "two"), ToSession::Sent);
    terminal.remove_session("ops");
    assert_eq!(terminal.send_to("ops", "three"), ToSession::NotFound);
    terminal.add_session("ops", AgentState::Working);
    assert_eq!(terminal.send_to("ops", "four"), ToSession::Sent);
    assert_eq!(terminal.notices("ops"), vec!["two", "four"]);

    let who = identity("code", 1);
    let opened = terminal.open(
        &place_ref(),
        "/state/fix--one",
        &AgentCommand::default(),
        &who,
        &OpenIn::Own,
        Some("madswan/planner"),
    );
    assert!(matches!(opened, Opened::Opened { .. }));
    assert_eq!(
        terminal.session_state("madswan/planner"),
        Some(AgentState::Waiting)
    );
    assert_eq!(terminal.send_to("madswan/planner", "five"), ToSession::Sent);
    assert_eq!(terminal.sent_to(&who), vec!["five"]);
    let second = terminal.open(
        &place_ref(),
        "/state/fix--one",
        &AgentCommand::default(),
        &identity("code", 2),
        &OpenIn::Own,
        Some("madswan/planner"),
    );
    assert!(matches!(second, Opened::Refused { .. }), "{second:?}");
    terminal.kill(&who);
    assert_eq!(terminal.session_state("madswan/planner"), None);
    assert_eq!(
        terminal.send_to("madswan/planner", "six"),
        ToSession::NotFound
    );

    terminal.cannot_send();
    assert!(matches!(
        terminal.send_to("ops", "seven"),
        ToSession::Refused { .. }
    ));
    assert_eq!(terminal.notices("ops"), vec!["two", "four"]);
}

/// The history-and-host spec's host's streams and pools: a stream's pool
/// and the host's, each with where it came from, and the streams
/// declared, `default` among them, as the configuration stands when
/// asked (143, 144, C.7).
#[test]
fn a_streams_pool_and_the_hosts() {
    let host = FakeHost::default();
    assert_eq!(
        host.streams().keys().collect::<Vec<_>>(),
        vec!["default"],
        "default stands with nothing declared"
    );
    let pool = Pool {
        strategy: Strategy::FillFirst,
        accounts: vec![Account {
            name: "work-1".into(),
            program: "agent".into(),
            env: BTreeMap::from([("AGENT_HOME".into(), "/scratch/work-1".into())]),
            agents: Some(4),
        }],
    };
    host.set_pool("work", Some(pool.clone()));
    host.set_stream(
        "work",
        Some(Stream {
            agents: Some(5),
            priority: 10,
            pool: Some("work".into()),
            ..Stream::default()
        }),
    );
    host.set_stream("personal", Some(Stream::default()));
    host.set_host_pool(Some("personal"));
    assert_eq!(
        host.streams().keys().collect::<Vec<_>>(),
        vec!["default", "personal", "work"]
    );
    assert_eq!(
        host.pool_for("work"),
        Some(PoolNamed {
            name: "work".into(),
            from: PoolFrom::Stream,
        })
    );
    for stream in ["personal", "default"] {
        assert_eq!(
            host.pool_for(stream),
            Some(PoolNamed {
                name: "personal".into(),
                from: PoolFrom::Host,
            })
        );
    }
    assert_eq!(host.pool("work"), Some(pool));
    host.set_host_pool(None);
    assert_eq!(host.pool_for("default"), None, "read as it stands now");
}

/// The stream the host names for a repository and kind, else the
/// repository, else the kind, with where it came from (143).
#[test]
fn a_stream_named_for_a_place() {
    let host = FakeHost::default();
    host.name_stream(None, Some("chore"), Some("personal"));
    host.name_stream(Some("/r"), None, Some("work"));
    host.name_stream(Some("/r"), Some("fix"), Some("nightly"));
    let named = |repository, kind| host.stream_for(repository, kind);
    assert_eq!(
        named("/r", "fix"),
        Some(StreamHeld {
            name: "nightly".into(),
            from: StreamFrom::HostRepositoryKind,
        })
    );
    assert_eq!(
        named("/r", "chore"),
        Some(StreamHeld {
            name: "work".into(),
            from: StreamFrom::HostRepository,
        })
    );
    assert_eq!(
        named("/s", "chore"),
        Some(StreamHeld {
            name: "personal".into(),
            from: StreamFrom::HostKind,
        })
    );
    assert_eq!(named("/s", "fix"), None);
}

/// A one-stage kind of the name given, which declares no label.
fn kind_named(name: &str) -> Kind {
    Kind {
        name: name.into(),
        version: 1,
        branch_prefix: name.into(),
        starts_unready: false,
        lands: true,
        agents: KindSettings::default(),
        stages: vec![],
        label: None,
    }
}

/// The seam-contracts spec's scenario of where an agent opens: the fake
/// terminal asked to open one agent beside the pane `operator`, one in
/// the session `work` and one in its own logs each where it was asked,
/// and answers each the same (94, 107).
#[test]
fn the_terminal_logs_where_each_agent_opens() {
    let terminal = FakeTerminal::new(Arc::new(FakeHost::default()));
    let at = [
        OpenIn::Beside {
            pane: "operator".into(),
        },
        OpenIn::Session {
            name: "work".into(),
        },
        OpenIn::Own,
    ];
    let mut answers = vec![];
    for (n, at) in at.iter().enumerate() {
        let who = identity("code", n as u32 + 1);
        answers.push(terminal.open(
            &place_ref(),
            "/state/fix--one",
            &AgentCommand::default(),
            &who,
            at,
            None,
        ));
        assert_eq!(terminal.opened_at(&who).as_ref(), Some(at));
    }
    assert_eq!(
        terminal.log.of("at"),
        vec![
            format!("at {} beside operator", identity("code", 1)),
            format!("at {} in session work", identity("code", 2)),
            format!("at {} in own", identity("code", 3)),
        ],
        "where each open was asked"
    );
    assert_eq!(
        terminal.log.of("open"),
        (1..=3)
            .map(|n| format!("open {} /r/places/fix--one", identity("code", n)))
            .collect::<Vec<_>>(),
        "each open logged as before"
    );
    assert!(
        answers.iter().all(|answer| *answer == answers[0]),
        "{answers:?}"
    );
    assert_eq!(
        terminal.beside(&identity("code", 1)).as_deref(),
        Some("operator")
    );
    assert_eq!(terminal.beside(&identity("code", 2)), None);
}

/// A workspace per session and base: an agent opened in a session makes
/// its place's tab in its base's workspace there; `shown_at` names the
/// tab and workspace of the live agent's session and each of the base's
/// workspaces in `bases`; a tidy closes the place's tab in each session,
/// and each workspace it leaves empty, keeping one a person's pane holds
/// (107, 137, 138, 139).
#[test]
fn a_workspace_per_session_and_base() {
    let terminal = FakeTerminal::new(Arc::new(FakeHost::default()));
    let open = |start, at: OpenIn| {
        terminal.open(
            &place_ref(),
            "/state/fix--one",
            &AgentCommand::default(),
            &identity("code", start),
            &at,
            None,
        )
    };
    open(1, OpenIn::Own);
    terminal.kill(&identity("code", 1));
    open(
        2,
        OpenIn::Session {
            name: "work".into(),
        },
    );
    assert_eq!(
        terminal.made(),
        (
            vec!["/r/places/fix--one".to_string(); 2],
            vec!["r main".to_string(), "r main in work".to_string()],
        )
    );
    assert_eq!(
        terminal.tabs_of(&place_ref()),
        vec![None, Some("work".to_string())]
    );
    let live = identity("code", 2);
    assert_eq!(
        terminal.shown_at(&place_ref(), Some(&live)),
        ShownAt {
            place: Some("tab fix--one in work".into()),
            agent: Some(format!("pane {live}")),
            base: Some("workspace r main in work".into()),
            bases: vec!["workspace r main".into(), "workspace r main in work".into()],
        }
    );
    assert_eq!(
        terminal.shown_at(&place_ref(), None).place.as_deref(),
        Some("tab fix--one in work"),
        "the newest tab where no agent is named"
    );
    terminal.person_pane_in(Some("work"), &place_ref(), "w4:p1");
    let tidy = terminal.tidy(&place_ref());
    assert_eq!(
        tidy,
        TerminalTidy {
            closed: vec![
                "the tab fix--one".into(),
                "the tab fix--one in work".into(),
                "the workspace r main".into(),
            ],
            kept: vec!["the workspace r main in work, held by the pane w4:p1".into()],
        }
    );
    assert_eq!(
        terminal.kept(),
        vec!["the workspace r main in work, held by the pane w4:p1".to_string()]
    );
    assert_eq!(
        terminal.made(),
        (vec![], vec!["r main in work".to_string()])
    );
}

/// The seam-fakes spec's scripted session: the fake host gives `work`
/// the session `office`, and a fix's agent of `work` opened in the
/// session its stream names is logged opened in `office` (107).
#[test]
fn a_scripted_session() {
    let host = Arc::new(FakeHost::default());
    host.set_stream_session("work", Some("office"));
    let terminal = FakeTerminal::new(host.clone());
    let session = host
        .streams()
        .get("work")
        .and_then(|stream| stream.session.clone())
        .expect("work's session");
    let who = identity("code", 1);
    terminal.open(
        &place_ref(),
        "/state/fix--one",
        &AgentCommand::default(),
        &who,
        &OpenIn::Session { name: session },
        None,
    );
    assert_eq!(
        terminal.log.of("at"),
        vec![format!("at {who} in session office")]
    );
    assert_eq!(terminal.tabs_of(&place_ref()), vec![Some("office".into())]);
    host.set_stream_session("work", None);
    assert_eq!(
        host.streams()["work"].session,
        None,
        "read as it stands now"
    );
}

/// The seam-fakes spec's scripted label: the fake kinds give `deploy` the
/// label `operation` wherever it is read, and with the fake host giving
/// none, a deploy place's label is `operation` from `kind` (180).
#[test]
fn a_scripted_label() {
    let git = FakeGit::new();
    let base = git.init("/r", "main");
    let kinds = FakeKinds::default();
    kinds.set_kinds("/r", vec![kind_named("deploy"), kind_named("fix")]);
    kinds.set_own(vec![kind_named("deploy")]);
    kinds.set_label("deploy", Some("operation"));
    let KindsRead::Read { kinds: read, .. } = kinds.read_kinds("/r", &base) else {
        panic!("the kinds were not read");
    };
    let labels: Vec<Option<&str>> = read.iter().map(|kind| kind.label.as_deref()).collect();
    assert_eq!(labels, vec![Some("operation"), None]);
    assert_eq!(kinds.flumes_own()[0].label.as_deref(), Some("operation"));
    let host = FakeHost::default();
    assert_eq!(host.label_for("/r", "deploy"), None);
    let label = host.label_for("/r", "deploy").or_else(|| {
        read[0].label.clone().map(|name| LabelHeld {
            name,
            from: LabelFrom::Kind,
        })
    });
    assert_eq!(
        label,
        Some(LabelHeld {
            name: "operation".into(),
            from: LabelFrom::Kind,
        })
    );
    kinds.set_label("deploy", None);
    assert_eq!(kinds.flumes_own()[0].label, None, "as it was set");
}

/// The fake host's labels for a repository and kind, else for a kind,
/// each with where it came from; its streams' shares; and its
/// configuration's values with their layers, its local layer's state and
/// the keys passed over, each as scripted when asked (143, 180, 181,
/// C.7).
#[test]
fn the_hosts_labels_shares_and_layers_are_scripted() {
    let host = FakeHost::default();
    host.name_label(None, "research", Some("inception"));
    host.name_label(Some("/r"), "research", Some("operation"));
    assert_eq!(
        host.label_for("/r", "research"),
        Some(LabelHeld {
            name: "operation".into(),
            from: LabelFrom::HostRepositoryKind,
        })
    );
    assert_eq!(
        host.label_for("/s", "research"),
        Some(LabelHeld {
            name: "inception".into(),
            from: LabelFrom::HostKind,
        })
    );
    assert_eq!(host.label_for("/s", "fix"), None);
    host.name_label(Some("/r"), "research", None);
    assert_eq!(
        host.label_for("/r", "research").map(|label| label.from),
        Some(LabelFrom::HostKind)
    );

    let share = Share {
        reserve: 2,
        keep_free: 1,
        unknown: vec![],
    };
    host.set_share("work", "operation", Some(share.clone()));
    host.set_share("work", "construction", Some(Share::default()));
    host.set_share("work", "construction", None);
    assert_eq!(
        host.streams()["work"].labels,
        BTreeMap::from([("operation".to_string(), share)])
    );

    assert_eq!(host.configuration(), HostConfiguration::default());
    host.set_value("streams.work.agents", "6", ConfigLayer::Managed);
    host.set_value("streams.work.agents", "3", ConfigLayer::Local);
    host.set_value("streams.work.pool", "\"work\"", ConfigLayer::Managed);
    let values = host.configuration().values;
    assert_eq!(
        values
            .iter()
            .map(|value| (value.key.as_str(), value.value.as_str(), value.layer))
            .collect::<Vec<_>>(),
        vec![
            ("streams.work.agents", "3", ConfigLayer::Local),
            ("streams.work.pool", "\"work\"", ConfigLayer::Managed),
        ]
    );
    let configuration = HostConfiguration {
        local: LocalLayer::NotTaken {
            keys: vec!["pools.work".into()],
            fault: None,
        },
        values,
        passed_over: vec![PinnedKey {
            key: "streams.work.agents".into(),
            pin: "streams.work.agents".into(),
        }],
    };
    host.set_configuration(configuration.clone());
    assert_eq!(host.configuration(), configuration);
    assert_eq!(host.log.count("configuration"), 3);
}

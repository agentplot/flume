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
            folder,
            &Settings::default(),
            mode,
            &BTreeMap::new(),
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
            folder,
            &Settings::default(),
            Some("all"),
            &BTreeMap::new(),
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
            &BTreeMap::new(),
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
            folder,
            &Settings::default(),
            None,
            &BTreeMap::new(),
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

/// The seam-fakes spec's scenario "A bare agent brought back", as the
/// fakes give it: the terminal holds the start's agent again, waiting,
/// its word that it started reads no, the log names the bringing back,
/// and the terminal reads it as the test scripts (77, 106).
#[test]
fn a_bare_agent_brought_back() {
    let host = Arc::new(FakeHost::default());
    let terminal = Arc::new(FakeTerminal::new(host.clone()));
    let agent = FakeAgent::new(terminal.clone());
    let talk = identity("talk", 1);
    let folder = "/state/fix--one";
    terminal.open(
        &place_ref(),
        folder,
        &AgentCommand::default(),
        &talk,
        &OpenIn::Own,
        None,
    );
    agent.keep_record_in(&talk, folder);
    assert_eq!(agent.started(folder, &talk), Reading::Yes);
    host.restart();
    terminal.forget_all();
    assert_eq!(terminal.state(&talk), AgentState::Stopped);
    agent.bring_back_bare(&talk);
    assert_eq!(terminal.state(&talk), AgentState::Waiting);
    assert_eq!(agent.started(folder, &talk), Reading::No);
    assert_eq!(
        agent.log.of("bring"),
        vec![format!("bring back bare {talk}")]
    );
    for state in [
        AgentState::Working,
        AgentState::Waiting,
        AgentState::Stopped,
    ] {
        terminal.set_state(&talk, state);
        assert_eq!(terminal.state(&talk), state);
    }
}

/// The seam-fakes spec's scenario "A scripted live session that ends":
/// live answers the processes scripted for the session a start's word
/// names, the words scripted where it cannot be read, and none where
/// nothing is scripted or the session ended, each reading and each
/// scripting logged (207).
#[test]
fn a_scripted_live_session_that_ends() {
    let host = Arc::new(FakeHost::default());
    let terminal = Arc::new(FakeTerminal::new(host.clone()));
    let agent = FakeAgent::new(terminal.clone());
    let folder = "/state/fix--one";
    let talk = identity("talk", 1);
    let other = identity("talk", 2);
    let unset = identity("code", 1);
    agent.keep_record_in(&talk, folder);
    agent.set_session(&talk, "s-1");
    agent.set_session(&other, "s-2");
    let home = BTreeMap::from([(FakeAgent::HOME.to_string(), "/scratch/work-1".to_string())]);
    let envs = [BTreeMap::new(), home];
    assert_eq!(agent.live(folder, &talk, &envs), Live::None);
    let process = LiveProcess {
        pid: 4242,
        started: host.now(),
    };
    agent.live_runs("s-1", std::slice::from_ref(&process));
    agent.live_unknown("s-2", "the sessions under AGENT_HOME cannot be read");
    assert_eq!(
        agent.live(folder, &talk, &envs),
        Live::Runs(vec![process.clone()])
    );
    assert_eq!(
        agent.live(folder, &other, &envs),
        Live::Unknown {
            what: "the sessions under AGENT_HOME cannot be read".into()
        }
    );
    assert_eq!(agent.live(folder, &unset, &envs), Live::None);
    // A start whose record is kept in another place's folder is not read
    // there, so it names no session.
    assert_eq!(agent.live("/state/fix--two", &talk, &envs), Live::None);
    agent.live_ended("s-1");
    assert_eq!(agent.live(folder, &talk, &[]), Live::None);
    let talk_lives: Vec<String> = agent
        .log
        .of("live")
        .into_iter()
        .filter(|line| line.starts_with(&format!("live {talk} in {folder} ")))
        .collect();
    assert_eq!(
        talk_lives,
        vec![
            format!("live {talk} in {folder} under no home, /scratch/work-1: none"),
            format!("live {talk} in {folder} under no home, /scratch/work-1: runs 4242"),
            format!("live {talk} in {folder} under no account: none"),
        ]
    );
    assert_eq!(
        (
            agent.log.of("live_runs"),
            agent.log.count("live_unknown"),
            agent.log.of("live_ended")
        ),
        (
            vec!["live_runs s-1: 4242".to_string()],
            1,
            vec!["live_ended s-1".to_string()]
        )
    );
    // Set to read none, it answers the seam's default whatever is scripted.
    agent.live_runs("s-1", &[process]);
    agent.live_not_read();
    assert_eq!(
        agent.live(folder, &talk, &envs),
        Live::Unknown {
            what: READS_NO_SESSION.into()
        }
    );
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
            folder,
            &Settings::default(),
            None,
            &BTreeMap::new(),
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
fn the_agents_efforts_are_scripted() {
    // Unscripted, the fake takes no effort, refusing every word in the
    // seam's default words, and names no variable read over it; scripted
    // to take `low` and `high`, it takes each and refuses any other naming
    // it and both, names the variable scripted, its log names each effort
    // asked, as at a launch and a start, and its command passes the
    // effort its settings give as `--effort <word>`, and none where they
    // give none (88, 106, 197).
    let host = Arc::new(FakeHost::default());
    let agent = FakeAgent::new(Arc::new(FakeTerminal::new(host)));
    assert_eq!(agent.effort("high"), Err(no_effort("high")));
    assert_eq!(agent.effort_variable(), None);

    agent.take_efforts(&["low", "high"]);
    agent.read_effort_over("FAKE_EFFORT");
    assert_eq!(agent.effort("high"), Ok(()));
    assert_eq!(agent.effort("high"), Ok(()));
    assert_eq!(agent.effort("low"), Ok(()));
    let words = agent.effort("med").expect_err("a word it does not take");
    assert!(
        words.contains("`med`") && words.contains("low") && words.contains("high"),
        "{words}"
    );
    assert_eq!(agent.effort_variable(), Some("FAKE_EFFORT"));
    assert_eq!(
        agent.log.of("effort"),
        vec![
            "effort high",
            "effort high",
            "effort high",
            "effort low",
            "effort med"
        ]
    );

    let who = identity("code", 1);
    let command = |effort: Option<&str>| {
        agent.command(
            "/state/fix--one",
            &Settings {
                effort: effort.map(str::to_string),
                ..Settings::default()
            },
            None,
            &BTreeMap::new(),
            None,
            &BTreeMap::new(),
            &place_ref(),
            &who,
            &Secret("s".into()),
            "go",
        )
    };
    let high = command(Some("high"));
    assert!(
        high.args
            .windows(2)
            .any(|pair| pair == ["--effort", "high"]),
        "{:?}",
        high.args
    );
    let none = command(None);
    assert!(
        !none.args.iter().any(|arg| arg == "--effort"),
        "{:?}",
        none.args
    );
}

#[test]
fn the_agents_own_settings_are_scripted() {
    // Unscripted, the fake reserves no key and refuses own settings in the
    // seam's default words, taking an empty entry, and its command lists
    // no file; scripted, it reserves each key with its owner, a nested one
    // by its path, takes them, and its command lists them as one JSON
    // object in a file under the place's own folder named by `--own
    // <file>`; every command given own settings logs their keys, and one
    // given none lists nothing (195, 196).
    let host = Arc::new(FakeHost::default());
    let agent = FakeAgent::new(Arc::new(FakeTerminal::new(host)));
    let given = BTreeMap::from([
        (
            "line".to_string(),
            serde_json::json!({"type": "command", "command": "line"}),
        ),
        ("tips".to_string(), serde_json::json!(false)),
    ]);
    assert_eq!(agent.reserved(), vec![]);
    assert_eq!(agent.own(&given), Err(NO_OWN.to_string()));
    assert_eq!(agent.own(&BTreeMap::new()), Ok(()));
    let folder = "/state/fix--one";
    let who = identity("code", 1);
    let command = |own: &BTreeMap<String, serde_json::Value>| {
        agent.command(
            folder,
            &Settings::default(),
            None,
            own,
            None,
            &BTreeMap::new(),
            &place_ref(),
            &who,
            &Secret("s".into()),
            "go",
        )
    };
    let lists_none = |command: &AgentCommand| {
        command.files.is_empty() && !command.args.iter().any(|arg| arg == "--own")
    };
    assert!(lists_none(&command(&given)), "a fake that takes none");

    agent.take_own();
    agent.reserve("env", "the account");
    agent.reserve("permissions.default", "flume's mode");
    assert_eq!(
        agent.reserved(),
        vec![
            Reserved {
                key: "env".into(),
                owner: "the account".into(),
            },
            Reserved {
                key: "permissions.default".into(),
                owner: "flume's mode".into(),
            },
        ]
    );
    assert_eq!(agent.own(&given), Ok(()));
    let taken = command(&given);
    let file = FakeAgent::own_file(folder, &who);
    assert!(file.starts_with("/state/fix--one/"), "{file}");
    assert!(
        taken
            .args
            .windows(2)
            .any(|pair| pair == ["--own", file.as_str()]),
        "{:?}",
        taken.args
    );
    assert_eq!(taken.files.keys().collect::<Vec<_>>(), vec![&file]);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&taken.files[&file]).expect("JSON"),
        serde_json::json!({"line": {"type": "command", "command": "line"}, "tips": false})
    );
    assert!(
        lists_none(&command(&BTreeMap::new())),
        "a command given none"
    );
    assert_eq!(
        agent.log.of("own"),
        vec![format!("own {who}: line, tips"); 2]
    );
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
    let model = |model: &str| Settings {
        model: Some(model.into()),
        ..Settings::default()
    };
    host.set_every_kind("/r", model("haiku"));
    host.set_stream_settings("work", model("fast"));
    host.set_every_agent(model("any"));
    let in_r = host.settings("/r", "change", "work");
    assert_eq!(in_r.kind, given("opus"));
    assert_eq!(in_r.repository, given("sonnet"));
    assert_eq!(in_r.every_kind, model("haiku"));
    assert_eq!(in_r.stream, model("fast"));
    assert_eq!(in_r.every_agent, model("any"));
    assert_eq!(
        host.log.of("settings"),
        vec!["settings /r change work".to_string()]
    );
    let in_s = host.settings("/s", "change", "default");
    assert_eq!(in_s.repository, KindSettings::default());
    assert_eq!(in_s.every_kind, Settings::default());
    assert_eq!(in_s.stream, Settings::default());
    assert_eq!(in_s.every_agent, model("any"));
    host.set_settings(None, "change", KindSettings::default());
    host.set_every_kind("/r", Settings::default());
    host.set_stream_settings("work", Settings::default());
    host.set_every_agent(Settings::default());
    assert_eq!(
        host.settings("/r", "change", "work"),
        HostSettings {
            repository: given("sonnet"),
            ..HostSettings::default()
        }
    );
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

/// The seam-fakes spec's "An entry set and taken away": a standing agent
/// given `madswan/planner` is among the host's standing entries until a
/// test takes it away, a run set beside it left as it is, and each read
/// logged (132, C.7).
#[test]
fn an_entry_set_and_taken_away() {
    let host = FakeHost::default();
    assert!(host.standing().is_empty());
    let planner = StandingAgent {
        agent: "madswan-planner".into(),
        session: Some("madswan".into()),
        stream: None,
    };
    host.set_standing("madswan/planner", Some(planner.clone()));
    assert_eq!(
        host.standing(),
        BTreeMap::from([("madswan/planner".to_string(), planner)])
    );
    assert_eq!(host.role("madswan/planner"), None, "no run for it");
    host.set_standing("madswan/planner", None);
    assert!(host.standing().is_empty());
    assert_eq!(
        host.log.calls(),
        vec!["standing", "standing", "role madswan/planner", "standing"]
    );
}

/// The seam-fakes spec's "An agent added, removed and refused":
/// `w-planner` added waiting in `w` is listed there by its name under its
/// key, which a send and a state reach and whose notices are kept; gone
/// once removed; and `w` refused answers not read in the test's words,
/// each asking logged and nothing opened (132, C.5).
#[test]
fn an_agent_added_removed_and_refused() {
    let terminal = FakeTerminal::new(Arc::new(FakeHost::default()));
    assert_eq!(
        terminal.agents_in(Some("w")),
        AgentsIn::Read { agents: vec![] },
        "a part holding none"
    );
    let at = terminal.add_agent(Some("w"), "w-planner", AgentState::Waiting);
    assert_eq!(
        terminal.agents_in(Some("w")),
        AgentsIn::Read {
            agents: vec![NamedAgent {
                name: "w-planner".into(),
                at: at.clone(),
                state: AgentState::Waiting,
            }]
        }
    );
    assert_eq!(
        terminal.agents_in(Some("office")),
        AgentsIn::Read { agents: vec![] },
        "another part's agents are its own"
    );
    assert_eq!(terminal.agents_in(None), AgentsIn::Read { agents: vec![] });
    assert_eq!(terminal.session_state(&at), Some(AgentState::Waiting));
    assert_eq!(terminal.send_to(&at, "an end of fix--one"), ToSession::Sent);
    assert_eq!(terminal.notices(&at), vec!["an end of fix--one"]);
    terminal.set_session_state(&at, AgentState::Asking);
    let AgentsIn::Read { agents } = terminal.agents_in(Some("w")) else {
        panic!("w is read");
    };
    assert_eq!(agents[0].state, AgentState::Asking, "its state as set");

    terminal.remove_agent(&at);
    assert_eq!(
        terminal.agents_in(Some("w")),
        AgentsIn::Read { agents: vec![] }
    );
    assert_eq!(terminal.session_state(&at), None);
    assert_eq!(terminal.send_to(&at, "another"), ToSession::NotFound);

    terminal.refuse_agents_in(Some("w"), "the session w did not answer");
    assert_eq!(
        terminal.agents_in(Some("w")),
        AgentsIn::Unread {
            reason: "the session w did not answer".into()
        }
    );
    let own = terminal.add_agent(None, "planner", AgentState::Working);
    assert!(own.ends_with("@own"), "{own}");
    assert_eq!(
        terminal.agents_in(None),
        AgentsIn::Read {
            agents: vec![NamedAgent {
                name: "planner".into(),
                at: own,
                state: AgentState::Working,
            }]
        },
        "the terminal's own part, read while w is refused"
    );
    assert_eq!(
        terminal.log.of("agents_in"),
        vec![
            "agents_in w",
            "agents_in w",
            "agents_in office",
            "agents_in own",
            "agents_in w",
            "agents_in w",
            "agents_in w",
            "agents_in own",
        ]
    );
    assert_eq!(terminal.log.count("open"), 0, "nothing started");
}

/// The seam-fakes spec's "Two of one name": `w-planner` added twice in
/// `w` is listed twice, each under its own key, which the core reads
/// as no one agent (132; core.28).
#[test]
fn two_of_one_name() {
    let terminal = FakeTerminal::new(Arc::new(FakeHost::default()));
    let first = terminal.add_agent(Some("w"), "w-planner", AgentState::Waiting);
    let second = terminal.add_agent(Some("w"), "w-planner", AgentState::Working);
    assert_ne!(first, second);
    assert_eq!(
        terminal.agents_in(Some("w")),
        AgentsIn::Read {
            agents: vec![
                NamedAgent {
                    name: "w-planner".into(),
                    at: first.clone(),
                    state: AgentState::Waiting,
                },
                NamedAgent {
                    name: "w-planner".into(),
                    at: second.clone(),
                    state: AgentState::Working,
                },
            ]
        }
    );
    assert_eq!(terminal.send_to(&second, "one"), ToSession::Sent);
    assert_eq!(terminal.notices(&second), vec!["one"]);
    assert!(
        terminal.notices(&first).is_empty(),
        "each reached by its key"
    );
}

fn attach_command(part: &str) -> Attached {
    Attached::Command {
        command: ViewCommand {
            program: "fake-terminal".into(),
            arguments: vec!["attach".into(), part.into()],
            environment: BTreeMap::new(),
        },
    }
}

/// The seam-fakes spec's scripted attach: the app's view scripted to
/// refuse, a window's command for `work` and then the refusal, each
/// asking logged with its view; and a command of the fake's own for its
/// own session, shown for its app once the script is gone, and either
/// view scripted apart from the other (187).
#[test]
fn a_scripted_attach() {
    let terminal = FakeTerminal::new(Arc::new(FakeHost::default()));
    let no_app = Attached::Refused {
        reason: "no app".into(),
    };
    terminal.attaches_as(View::App, Some(no_app.clone()));
    assert_eq!(
        terminal.attach(Some("work"), View::Terminal),
        attach_command("work")
    );
    assert_eq!(terminal.attach(Some("work"), View::App), no_app);
    assert_eq!(
        terminal.log.of("attach"),
        vec!["attach work for terminal", "attach work for app"]
    );

    assert_eq!(terminal.attach(None, View::Terminal), attach_command("own"));
    terminal.attaches_as(View::App, None);
    assert_eq!(terminal.attach(Some("work"), View::App), Attached::Shown);
    let no_session = Attached::Refused {
        reason: "the terminal holds no session".into(),
    };
    terminal.attaches_as(View::Terminal, Some(no_session.clone()));
    assert_eq!(terminal.attach(Some("work"), View::Terminal), no_session);
    assert_eq!(terminal.attach(None, View::App), Attached::Shown);
    assert_eq!(
        terminal.log.of("attach")[2..],
        [
            "attach own for terminal",
            "attach work for app",
            "attach work for terminal",
            "attach own for app",
        ]
    );
}

/// The seam-fakes spec's scripted runs: the fake terminal scripted to
/// read `work` running answers it so, and `office`, never scripted, not
/// running, each asking logged and nothing opened; its own part not
/// running as `own` (106, 201).
#[test]
fn a_scripted_runs() {
    let terminal = FakeTerminal::new(Arc::new(FakeHost::default()));
    terminal.runs_as(
        "work",
        SessionRuns::Runs {
            name: "work".into(),
        },
    );
    assert_eq!(
        terminal.runs(&[Some("work")]),
        vec![SessionRuns::Runs {
            name: "work".into()
        }]
    );
    assert_eq!(
        terminal.runs(&[Some("office")]),
        vec![SessionRuns::NotRunning {
            name: "office".into()
        }]
    );
    assert_eq!(terminal.log.of("runs"), vec!["runs work", "runs office"]);
    assert_eq!(terminal.log.count("open"), 0);
    assert_eq!(
        terminal.runs(&[None]),
        vec![SessionRuns::NotRunning { name: "own".into() }]
    );
    assert_eq!(
        terminal.log.calls(),
        vec!["runs work", "runs office", "runs own"]
    );
}

/// The seam-fakes spec's several parts in one runs: `work`, `office` and
/// its own part asked in one call answer as each does alone, in that
/// order, and the call is logged once (106, 201).
#[test]
fn several_parts_in_one_runs() {
    let terminal = FakeTerminal::new(Arc::new(FakeHost::default()));
    terminal.runs_as(
        "work",
        SessionRuns::Runs {
            name: "work".into(),
        },
    );
    assert_eq!(
        terminal.runs(&[Some("work"), Some("office"), None]),
        vec![
            SessionRuns::Runs {
                name: "work".into()
            },
            SessionRuns::NotRunning {
                name: "office".into()
            },
            SessionRuns::NotRunning { name: "own".into() },
        ]
    );
    assert_eq!(terminal.log.calls(), vec!["runs work office own"]);
}

/// The seam-fakes spec's scripted window, the fakes' part of it: the
/// window scripted to print `attach work` answers that line for the
/// terminal's command under the title the host declares for `work`, its
/// log naming one show under that title and no window opened; then it
/// opens one and refuses as scripted. The scenario through the core's
/// open is conformance's `open: the fake window scripted to print attach
/// work …` (188).
#[test]
fn a_scripted_window() {
    let host = Arc::new(FakeHost::default());
    host.set_title("work", Some("Work Agents"));
    let title = host.streams()["work"].title.clone().expect("work's title");
    let terminal = FakeTerminal::new(host);
    let Attached::Command { command } = terminal.attach(Some("work"), View::Terminal) else {
        panic!("the fake terminal answers a command");
    };
    let window = FakeWindow::default();
    let printed = WindowShown::Printed {
        line: "attach work".into(),
    };
    window.answers(Some(printed.clone()));
    assert_eq!(window.show(&command, &title), printed);
    assert_eq!(
        window.log.calls(),
        vec!["show fake-terminal attach work titled Work Agents"]
    );
    assert!(window.windows().is_empty());

    window.answers(None);
    assert_eq!(window.show(&command, &title), WindowShown::Opened);
    assert_eq!(window.windows(), vec![(command.clone(), title.clone())]);
    let refused = WindowShown::Refused {
        reason: "no display".into(),
    };
    window.answers(Some(refused.clone()));
    assert_eq!(window.show(&command, &title), refused);
    assert_eq!(window.windows().len(), 1);
    assert_eq!(window.log.count("show"), 3);
}

/// The seam-fakes spec's scripted title, the fakes' part of it: the fake
/// host's `work` titled as scripted and every other stream with none.
/// The scenario through status is conformance's `streams: the fake
/// host's work scripted with the title Work Agents …` (143).
#[test]
fn a_scripted_title() {
    let host = FakeHost::default();
    host.set_stream(
        "personal",
        Some(Stream {
            agents: Some(3),
            ..Stream::default()
        }),
    );
    host.set_title("work", Some("Work Agents"));
    host.set_title("personal", Some("Personal"));
    host.set_title("personal", None);
    let streams = host.streams();
    assert_eq!(streams["work"].title.as_deref(), Some("Work Agents"));
    assert_eq!(streams["personal"].title, None);
    assert_eq!(streams["personal"].agents, Some(3));
    assert_eq!(streams[DEFAULT_STREAM].title, None);
}

/// The seam-fakes spec's scripted display: the host's own view `app` and
/// `trial`'s multiplexer `other`, `trial` giving `other` from `stream`,
/// every stream the view `app` from `host`, and every other key the
/// fakes' own from `neither`, each asking logged; then a stream's key
/// over the host's, and the host's over the fakes' own (143, 190).
#[test]
fn a_scripted_display() {
    let host = FakeHost::default();
    host.set_host_display(None, Some(View::App), None);
    host.set_display("trial", Some("other"), None, None);
    host.set_stream("work", Some(Stream::default()));
    let held = |profile: &str, from| ProfileHeld {
        profile: profile.into(),
        from,
    };
    for name in host.streams().keys() {
        let multiplexer = match name.as_str() {
            "trial" => held("other", NamedBy::Stream),
            _ => held("fake", NamedBy::Neither),
        };
        assert_eq!(
            host.display_for(name),
            StreamDisplay {
                multiplexer,
                view: ViewHeld {
                    view: View::App,
                    from: NamedBy::Host,
                },
                terminal_app: held("fake-window", NamedBy::Neither),
            },
            "{name}'s display"
        );
    }
    assert_eq!(
        host.log.of("display_for"),
        vec![
            "display_for default",
            "display_for trial",
            "display_for work"
        ]
    );

    host.set_host_display(Some("host-terminal"), None, Some("host-window"));
    host.set_display("trial", None, Some(View::Terminal), Some("trial-window"));
    assert_eq!(
        host.display_for("trial"),
        StreamDisplay {
            multiplexer: held("host-terminal", NamedBy::Host),
            view: ViewHeld {
                view: View::Terminal,
                from: NamedBy::Stream,
            },
            terminal_app: held("trial-window", NamedBy::Stream),
        }
    );
    assert_eq!(
        host.display_for("work").view,
        ViewHeld {
            view: View::Terminal,
            from: NamedBy::Neither,
        }
    );
    assert_eq!(
        host.display_for("work").terminal_app,
        held("host-window", NamedBy::Host)
    );
}

/// The fake resolver's terminals and windows by name: none until a test
/// names one, each the very fake it was given, with a log of its own,
/// and a name it holds none for none (105, 188, 190).
#[test]
fn the_resolver_holds_terminals_and_windows_by_name() {
    let resolver = FakeResolver::new(Arc::new(FakeKinds::default()));
    assert!(resolver.terminal("fake").is_none());
    assert!(resolver.window("fake-window").is_none());
    let host = Arc::new(FakeHost::default());
    let own = Arc::new(FakeTerminal::new(host.clone()));
    let other = Arc::new(FakeTerminal::new(host));
    let window = Arc::new(FakeWindow::default());
    resolver.with_terminal("fake", own.clone());
    resolver.with_terminal("other", other.clone());
    resolver.with_window("fake-window", window.clone());

    let Attached::Command { command } = resolver
        .terminal("other")
        .expect("the terminal other")
        .attach(Some("work"), View::Terminal)
    else {
        panic!("the fake terminal answers a command");
    };
    let shown = resolver
        .window("fake-window")
        .expect("the window fake-window")
        .show(&command, "Work Agents");
    assert_eq!(shown, WindowShown::Opened);
    assert_eq!(other.log.of("attach"), vec!["attach work for terminal"]);
    assert!(own.log.calls().is_empty());
    assert_eq!(window.windows(), vec![(command, "Work Agents".to_string())]);
    assert_eq!(resolver.terminal_profiles(), vec!["fake", "other"]);
    assert_eq!(resolver.window_profiles(), vec!["fake-window"]);
    assert!(resolver.terminal("rex").is_none());
    assert!(resolver.window("kitty").is_none());
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
        threshold: None,
        accounts: vec![Account {
            name: "work-1".into(),
            program: "agent".into(),
            env: BTreeMap::from([("AGENT_HOME".into(), "/scratch/work-1".into())]),
            agents: Some(4),
            usage: None,
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

/// A one-stage kind `name` at `version`.
fn kind_at(name: &str, version: u32) -> Kind {
    Kind {
        name: name.into(),
        version,
        branch_prefix: name.into(),
        starts_unready: false,
        lands: true,
        agents: KindSettings::default(),
        stages: vec![],
        label: None,
    }
}

fn hash_of(digit: char) -> String {
    format!("sha256:{}", digit.to_string().repeat(64))
}

fn lock_entry(name: &str, version: u32, hash: &str) -> LockEntry {
    LockEntry {
        kind: kind_identity("acme", name, version),
        hash: hash.into(),
    }
}

#[test]
fn a_kinds_name_is_read_as_written_or_refused_as_scripted() {
    let kinds = FakeKinds::default();
    assert_eq!(
        kinds.read_named("acme/review@2"),
        Ok(KindNamed {
            source: Some("acme".into()),
            name: "review".into(),
            version: Some(2),
            spelled: "acme/review@2".into(),
        })
    );
    assert_eq!(
        kinds.read_named("review"),
        Ok(KindNamed {
            source: None,
            name: "review".into(),
            version: None,
            spelled: "review".into(),
        })
    );
    for text in ["acme/", "review@0", "review@x", "a/b/c"] {
        assert!(kinds.read_named(text).is_err(), "`{text}` read");
    }
    kinds.unreadable_named("review", "scripted");
    assert_eq!(kinds.read_named("review"), Err("scripted".into()));
    assert_eq!(
        kinds.log.of("read_named"),
        [
            "read_named acme/review@2",
            "read_named review",
            "read_named acme/",
            "read_named review@0",
            "read_named review@x",
            "read_named a/b/c",
            "read_named review",
        ]
    );
}

#[test]
fn flumes_own_are_packages_at_a_stand_in_hash_or_as_scripted() {
    let kinds = FakeKinds::default();
    kinds.set_own(vec![kind_at("fix", 1), kind_at("change", 2)]);
    let own = kinds.flumes_own();
    assert_eq!(
        own.iter()
            .map(|package| package.identity.spelled.as_str())
            .collect::<Vec<_>>(),
        ["flume/fix@1", "flume/change@2"]
    );
    assert!(own
        .iter()
        .all(|package| package.hash.starts_with("sha256:")
            && package.hash.len() == "sha256:".len() + 64));
    assert_ne!(own[0].hash, own[1].hash, "two kinds at one stand-in hash");
    kinds.set_own(vec![kind_at("fix", 1)]);
    assert_eq!(
        kinds.flumes_own()[0].hash,
        own[0].hash,
        "one kind's hash moved"
    );
    kinds.set_own_packages(vec![packaged(
        "flume",
        kind_at("fix", 1),
        Some(&hash_of('f')),
    )]);
    assert_eq!(kinds.flumes_own()[0].hash, hash_of('f'));
    assert_eq!(kinds.log.count("flumes_own"), 3);
}

#[test]
fn a_sources_packages_are_scripted_by_its_directory() {
    let kinds = FakeKinds::default();
    kinds.set_package("/kinds/acme", kind_at("review", 2), &hash_of('2'));
    kinds.set_package("/kinds/acme", kind_at("review", 1), &hash_of('1'));
    kinds.refuse_package(
        "/kinds/acme",
        "lint",
        1,
        "kinds.lint.env: no declaration gives one",
    );
    assert_eq!(
        kinds.read_source("acme", "/kinds/acme"),
        SourceRead::Read {
            kinds: vec![
                kind_identity("acme", "lint", 1),
                kind_identity("acme", "review", 1),
                kind_identity("acme", "review", 2),
            ],
        },
        "every package by identity, the refused one among them"
    );
    assert_eq!(
        kinds.read_package("acme", "/kinds/acme", "review", 2),
        PackageRead::Read {
            package: Box::new(Packaged {
                dir: Some(package_dir("/kinds/acme", "review", 2)),
                ..packaged("acme", kind_at("review", 2), Some(&hash_of('2')))
            }),
        },
        "the package with its directory as read"
    );
    assert_eq!(
        kinds.read_package("acme", "/kinds/acme", "lint", 1),
        PackageRead::CouldNotRead {
            reason: "kinds.lint.env: no declaration gives one".into(),
        }
    );
    assert!(matches!(
        kinds.read_package("acme", "/kinds/acme", "review", 3),
        PackageRead::CouldNotRead { reason } if reason.contains("review@3")
    ));
    kinds.set_package("/kinds/acme", kind_at("review", 2), &hash_of('3'));
    assert!(
        matches!(
            kinds.read_package("acme", "/kinds/acme", "review", 2),
            PackageRead::Read { package } if package.hash == hash_of('3')
        ),
        "a package scripted again at another hash"
    );
    kinds.remove_package("/kinds/acme", "review", 1);
    assert!(matches!(
        kinds.read_source("acme", "/kinds/acme"),
        SourceRead::Read { kinds } if kinds.len() == 2
    ));
    assert!(matches!(
        kinds.read_source("beta", "/kinds/beta"),
        SourceRead::CouldNotRead { reason } if reason.contains("/kinds/beta")
    ));
    kinds.unreadable_source("/kinds/acme", "permission denied");
    assert_eq!(
        kinds.read_source("acme", "/kinds/acme"),
        SourceRead::CouldNotRead {
            reason: "permission denied".into(),
        }
    );
    assert_eq!(
        kinds.read_package("acme", "/kinds/acme", "review", 2),
        PackageRead::CouldNotRead {
            reason: "permission denied".into(),
        }
    );
    assert_eq!(
        kinds.log.of("read_source")[0],
        "read_source acme /kinds/acme"
    );
    assert_eq!(
        kinds.log.of("read_package")[0],
        "read_package acme /kinds/acme review@2"
    );
}

#[test]
fn the_repositorys_lock_is_scripted_at_a_revision_or_every_revision() {
    let git = FakeGit::new();
    let first = git.init("/r", "main");
    let second = git.commit("/r", "main", &["x"]);
    let third = git.commit("/r", "main", &["y"]);
    let kinds = FakeKinds::default();
    assert_eq!(kinds.read_lock_at("/r", &first), LockRead::None);
    kinds.set_lock("/r", vec![lock_entry("review", 1, &hash_of('1'))]);
    kinds.set_lock_at("/r", &second, vec![]);
    kinds.unreadable_lock_at("/r", &third, "kinds: must be an array of tables");
    assert_eq!(
        kinds.read_lock_at("/r", &first),
        LockRead::Read {
            entries: vec![lock_entry("review", 1, &hash_of('1'))],
        }
    );
    assert_eq!(
        kinds.read_lock_at("/r", &second),
        LockRead::Read { entries: vec![] }
    );
    assert_eq!(
        kinds.read_lock_at("/r", &third),
        LockRead::CouldNotRead {
            reason: "kinds: must be an array of tables".into(),
        }
    );
    assert_eq!(
        kinds.log.of("read_lock_at")[0],
        format!("read_lock_at /r {first}")
    );
}

#[test]
fn a_lock_on_disk_is_scripted_and_a_lock_written_replaces_it() {
    let kinds = FakeKinds::default();
    let host_lock = "/config/flume/flume.lock";
    assert_eq!(kinds.read_lock_file(host_lock), LockRead::None);
    kinds.set_lock_file(host_lock, vec![lock_entry("review", 1, &hash_of('1'))]);
    assert_eq!(
        kinds.read_lock_file(host_lock),
        LockRead::Read {
            entries: vec![lock_entry("review", 1, &hash_of('1'))],
        }
    );
    let written = [
        lock_entry("review", 2, &hash_of('2')),
        lock_entry("lint", 1, &hash_of('3')),
    ];
    kinds.write_lock(host_lock, &written).expect("written");
    assert_eq!(
        kinds.read_lock_file(host_lock),
        LockRead::Read {
            entries: vec![written[1].clone(), written[0].clone()],
        },
        "written whole, in identity order"
    );
    assert_eq!(
        kinds.log.of("write_lock"),
        [format!(
            "write_lock {host_lock} acme/review@2={},acme/lint@1={}",
            hash_of('2'),
            hash_of('3')
        )]
    );
    kinds.unwritable_lock(host_lock, "read-only");
    assert_eq!(kinds.write_lock(host_lock, &[]), Err("read-only".into()));
    assert!(
        matches!(
            kinds.read_lock_file(host_lock),
            LockRead::Read { entries } if entries.len() == 2
        ),
        "a write that failed left the lock as it was"
    );
    kinds.unreadable_lock_file("/bad.lock", "not TOML");
    assert_eq!(
        kinds.read_lock_file("/bad.lock"),
        LockRead::CouldNotRead {
            reason: "not TOML".into(),
        }
    );
}

#[test]
fn the_hosts_sources_are_scripted() {
    let host = FakeHost::default();
    assert!(
        host.sources().is_empty(),
        "a host with no sources directory"
    );
    host.set_source("acme", "/kinds/acme");
    host.faulty_source("bad", "it gives no kinds_dir");
    let sources = host.sources();
    assert_eq!(
        sources.get("acme"),
        Some(&HostSource {
            file: source_file("acme"),
            read: HostSourceRead::Read {
                kinds_dir: "/kinds/acme".into(),
            },
        })
    );
    assert_eq!(
        sources.get("bad"),
        Some(&HostSource {
            file: format!("{SOURCES}/bad.toml"),
            read: HostSourceRead::CouldNotRead {
                reason: "it gives no kinds_dir".into(),
            },
        })
    );
    host.set_source("acme", "/kinds/acme-2");
    assert!(
        matches!(
            &host.sources()["acme"].read,
            HostSourceRead::Read { kinds_dir } if kinds_dir == "/kinds/acme-2"
        ),
        "a file renamed over another"
    );
    host.remove_source("bad");
    assert_eq!(host.sources().keys().collect::<Vec<_>>(), ["acme"]);
    assert_eq!(host.log.count("sources"), 4);
    assert_eq!(
        (host.sources_dir(), host.kinds_lock()),
        (Some(SOURCES.to_string()), Some(HOST_LOCK.to_string())),
        "where the sources' files and the host's lock lie"
    );
}

#[test]
fn a_declarations_identity_and_the_repositorys_files() {
    let kinds = FakeKinds::default();
    assert_eq!(
        kinds.identity_of("repo", "bump", 2),
        Ok(kind_identity("repo", "bump", 2))
    );
    assert_eq!(
        kinds.log.of("identity_of"),
        vec!["identity_of repo bump@2".to_string()]
    );
    assert_eq!(
        (kinds.declaration_path(), kinds.lock_path()),
        (
            Some(".config/flume.toml".to_string()),
            Some(".config/flume.lock".to_string())
        )
    );
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

/// The fake host's sessions for a stream's kinds and their stages, each
/// table's other keys kept, as the test scripts them and as they stand
/// when asked (199).
#[test]
fn a_streams_sessions_for_its_kinds_and_stages_are_scripted() {
    let host = FakeHost::default();
    let research = KindSessions {
        session: Some("work-design".into()),
        ..KindSessions::default()
    };
    let release = KindSessions {
        stages: BTreeMap::from([(
            "review".to_string(),
            StageSession {
                session: Some("work-design".into()),
                unknown: vec![],
            },
        )]),
        ..KindSessions::default()
    };
    let plan = KindSessions {
        session: Some("work-plan".into()),
        unknown: vec!["label".into()],
        ..KindSessions::default()
    };
    host.set_kind_sessions("work", "research", Some(research.clone()));
    host.set_kind_sessions("work", "release", Some(release.clone()));
    host.set_kind_sessions("work", "plan", Some(plan.clone()));
    assert_eq!(
        host.streams()["work"].kinds,
        BTreeMap::from([
            ("plan".to_string(), plan),
            ("release".to_string(), release),
            ("research".to_string(), research),
        ])
    );
    assert_eq!(host.streams()["work"].session, None, "work's own unnamed");
    host.set_kind_sessions("work", "plan", None);
    assert!(
        !host.streams()["work"].kinds.contains_key("plan"),
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
    assert_eq!(
        kinds.flumes_own()[0].kind.label.as_deref(),
        Some("operation")
    );
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
    assert_eq!(kinds.flumes_own()[0].kind.label, None, "as it was set");
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

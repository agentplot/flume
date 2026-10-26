//! The fakes' own behaviour: each answer settable, refusals and unknowns
//! among them, every call logged in order, and a clock that moves only
//! when told. Their contracts are the conformance suite's seam scenarios.

use flume_fakes::*;
use flume_seams::*;
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
    assert_eq!(agent.received(&who, "go"), Received::Unknown);
    terminal.open(&place_ref(), &AgentCommand::default(), &who, None);
    agent.set_own_work(&who, Reading::Unknown);
    agent.set_asking(&who, Reading::Yes);
    assert_eq!(agent.own_work(&who), Reading::Unknown);
    assert_eq!(agent.asking(&who), Reading::Yes);
    assert!(matches!(
        agent.received(&who, "go"),
        Received::NotReceived { .. }
    ));
    terminal.send(&who, "go");
    assert!(matches!(
        agent.received(&who, "go"),
        Received::Received { .. }
    ));
    agent.received_unknown(&who);
    assert_eq!(agent.received(&who, "go"), Received::Unknown);
    agent.refuse_act(&who, "submit");
    assert_eq!(
        agent.guard(&who),
        Guarded::Refused {
            acts: vec!["submit".into()]
        }
    );
    agent.no_send_back();
    agent.no_on_quiet();
    agent.no_guard();
    assert_eq!(agent.send_back(), Offered::NotOffered);
    assert_eq!(agent.on_quiet(), Offered::NotOffered);
    assert_eq!(agent.guard(&who), Guarded::NotOffered);
}

#[test]
fn the_terminals_states_follow_the_fake_clock() {
    let host = Arc::new(FakeHost::default());
    let terminal = FakeTerminal::new(host.clone());
    let who = identity("code", 1);
    terminal.comes_up(AgentState::Unknown);
    terminal.open(&place_ref(), &AgentCommand::default(), &who, None);
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
}

#[test]
fn the_merge_queues_outcomes_are_planned() {
    let git = FakeGit::new();
    git.init("/r", "main");
    let queue = FakeMergeQueue::new(git.clone(), None);
    let at = place_ref();
    let head = git.commit("/r", "fix/one", &["a"]);
    queue.plan_submits(vec![Plan::Fail("test x".into()), Plan::Withdraw]);
    assert_eq!(
        queue.submit(&at, &head),
        Verdict::Failed {
            words: "test x".into()
        }
    );
    assert_eq!(queue.submit(&at, &head), Verdict::Withdrawn);
    assert!(matches!(queue.submit(&at, &head), Verdict::Landed { .. }));
    queue.unreadable("down");
    assert!(matches!(
        queue.read_submission("/r", "fix/one"),
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
    resolver.bind_merge_queue("queue", Arc::new(FakeMergeQueue::new(git.clone(), None)));
    let host = Arc::new(FakeHost::default());
    resolver.bind_agent(
        "agent",
        Arc::new(FakeAgent::new(Arc::new(FakeTerminal::new(host)))),
    );
    assert!(resolver.place(None).is_some());
    assert!(resolver.place(Some("trees")).is_some());
    assert!(resolver.place(Some("other")).is_none());
    assert!(resolver.change("plans").is_some());
    assert!(resolver.merge_queue("queue").is_some());
    assert!(resolver.merge_queue("other").is_none());
    assert!(resolver.agent("agent").is_some());
    assert!(resolver.agent("other").is_none());
}

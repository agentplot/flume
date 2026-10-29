//! Each type the seams pass, written to JSON and read back unchanged.

use crate::*;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;

fn round_trips<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: T) {
    let json = serde_json::to_string(&value).expect("written");
    let back: T = serde_json::from_str(&json).expect("read back");
    assert_eq!(back, value, "{json}");
}

fn settings() -> Settings {
    Settings {
        program: Some("agent".into()),
        model: Some("large".into()),
        instructions: Some("be brief".into()),
        env: BTreeMap::from([("A".into(), "1".into())]),
        beside: Some("operator".into()),
    }
}

fn stage() -> Stage {
    Stage {
        name: "code".into(),
        prompt: "{{brief}}".into(),
        conversation: false,
        deliverables: vec![Deliverable::Commits, Deliverable::TasksTicked],
        agents: vec![settings(), Settings::default()],
        waits: true,
        quiet_limit: Some(45 * 60),
        send_back_to: Some("code".into()),
        send_back_limit: Some(3),
        makes: vec!["preview".into(), "spike".into()],
        repeats: true,
    }
}

fn kind() -> Kind {
    Kind {
        name: "bump".into(),
        version: 2,
        branch_prefix: "bump".into(),
        starts_unready: true,
        lands: true,
        agents: kind_settings(),
        stages: vec![stage()],
    }
}

/// What a run made: a spike by its address, a preview by the branch
/// and revision that bring it up, and a line that is no object.
fn made() -> Vec<Made> {
    [
        r#"{"name":"spike","what":"spike","address":"https://forge.example/o/r/tree/spike/x?ref=a%2Fb"}"#,
        r#"{"name":"preview","what":"preview","branch":"fix/one","revision":"c3"}"#,
        "a preview at port 4000",
    ]
    .into_iter()
    .map(Made::read)
    .collect()
}

fn kind_settings() -> KindSettings {
    KindSettings {
        every_stage: Settings {
            model: Some("sonnet".into()),
            ..Settings::default()
        },
        stages: BTreeMap::from([("code".into(), settings())]),
    }
}

fn identity() -> Identity {
    Identity {
        repository: "/r".into(),
        place: "bump--one".into(),
        stage: "code".into(),
        start: 2,
    }
}

fn holds() -> Holds {
    Holds {
        uncommitted: vec!["a.rs".into()],
        untracked: vec!["b.txt".into()],
        lock: Some("locked by hand".into()),
    }
}

fn readiness() -> Readiness {
    Readiness {
        steps: vec![
            ReadyStep {
                step: "toolchain".into(),
                passed: true,
                words: "ok".into(),
            },
            ReadyStep {
                step: "setup".into(),
                passed: false,
                words: "no network".into(),
            },
        ],
    }
}

fn prompt() -> Prompt {
    Prompt {
        text: "do it".into(),
        carried: false,
        sendings: vec![10, 70],
        received_at: Some(75),
    }
}

fn held() -> HeldState {
    HeldState {
        place: PlaceRecord {
            id: PlaceId::new("/r/places/bump--one"),
            repository: "/r".into(),
            kind: kind(),
            declared: Declared::Repository,
            name: "one".into(),
            branch: "bump/one".into(),
            base: "main".into(),
            base_revision: Revision::new("c1"),
            location: "/r/places/bump--one".into(),
            binding: Binding {
                place: Some("place-tool".into()),
                change: Some("change-tool".into()),
                merge_queue: Some("queue-tool".into()),
            },
            brief: Some("bump it".into()),
            mark: Some("team-a".into()),
            settings: BTreeMap::from([("code".into(), settings())]),
            every_stage: settings(),
            repository_settings: kind_settings(),
            hold: vec!["review".into()],
            hold_submission: true,
            hold_words: BTreeMap::from([("review".into(), "until the design is ruled".into())]),
            recipients: vec!["operator".into(), "role:planner".into()],
        },
        phase: Phase::Over {
            over: Over::Landed {
                commit: Revision::new("c9"),
            },
            kept: Some(Kept::Holds { holds: holds() }),
        },
        readiness: readiness(),
        stages: vec![StageHeld {
            name: "code".into(),
            state: StageState::Begun,
            starts: 2,
            refused_part: Some("words".into()),
            refused_setting: Some("the program nothing-here, from host-kind".into()),
            end: Some(StageEnd {
                head: Some(Revision::new("c3")),
                report: Some("/s/conclusion.md".into()),
                words: Some("which design?".into()),
                needs: Some(StopKind::FoundWork),
                last_words: Some("I could not sign in".into()),
                commits: vec![Revision::new("c3")],
                screen: Some("Bypass Permissions mode".into()),
                made: made(),
                group: Some(Box::new(GroupHeld {
                    index: 1,
                    number: Some("2".into()),
                    title: Some("Two".into()),
                    tasks: vec!["2.1 do it".into()],
                })),
            }),
            room: Some(RoomWait {
                order: 3,
                words: Some("the merge queue's words".into()),
            }),
            made_from: Some(1),
            start: Some(StartHeld {
                n: 2,
                started_at: 5,
                start_head: Revision::new("c2"),
                made_from: 2,
                words: Some("the second".into()),
                identity: identity(),
                secret: Secret("s3cret".into()),
                program: "agent".into(),
                settings: settings(),
                from: BTreeMap::from([
                    (
                        "model".into(),
                        Given {
                            layer: Layer::Repository,
                            value: "opus".into(),
                        },
                    ),
                    (
                        "env.AGENT_HOME".into(),
                        Given {
                            layer: Layer::HostKind,
                            value: "~/.agent-work".into(),
                        },
                    ),
                ]),
                record: Some("record-1".into()),
                prompt: prompt(),
                live: true,
                begun_at: Some(75),
                first_quiet: Some(100),
                last_head: Revision::new("c3"),
                last_head_move: 90,
                past_limit: true,
                printed: Some("cd /r/places/bump--one && agent 'bump it'".into()),
                group: Some(GroupHeld {
                    index: 1,
                    number: Some("2".into()),
                    title: Some("Task groups".into()),
                    tasks: vec!["2.1 add the groups".into(), "2.2 read them".into()],
                }),
            }),
            since: Some(120),
        }],
        current: Some(0),
        submission: SubmissionHeld {
            due: true,
            submits: 2,
            send_backs: 1,
            id: Some("7".into()),
            before: Some("3".into()),
            verdict: Some(Verdict::Conflict {
                files: vec!["src/lib.rs".into()],
                words: "conflicts with main".into(),
            }),
            waiting: Some("withdrawn".into()),
            waiting_since: Some(130),
            unread: Some("the merge queue is down".into()),
        },
        failures: vec!["the agent did not end".into()],
        priority: -3,
        shown: Some("code · working; 1 running".into()),
        ends: ends(),
        role: Some("madswan/planner".into()),
        unwoken: BTreeMap::from([(
            "madswan/ops".to_string(),
            "the host's configuration names no role for madswan/ops".to_string(),
        )]),
        holds: holds_put(),
        waiting: Some(Waiting {
            on: WaitingOn {
                act: Act::Retry,
                stage: Some("review".into()),
                why: Waits::OnHold,
                since: Some(140),
                words: Some("until the design is ruled".into()),
            },
            stalled: true,
        }),
        stall_after: 30 * 60,
    }
}

fn holds_put() -> Vec<Hold> {
    vec![
        Hold {
            on: HoldOn::Stage {
                stage: "review".into(),
            },
            words: Some("until the design is ruled".into()),
            since: 10,
        },
        Hold {
            on: HoldOn::Submission,
            words: None,
            since: 10,
        },
    ]
}

fn ends() -> Vec<EndHeld> {
    vec![
        EndHeld {
            of: EndOf::Stage {
                stage: "code".into(),
                start: 2,
                state: StageState::Delivered,
                quiet_from: None,
                end: StageEnd {
                    head: Some(Revision::new("c3")),
                    ..StageEnd::default()
                },
            },
            read_at: 140,
            deliveries: vec![
                Delivery {
                    recipient: "operator".into(),
                    at: 141,
                    by: DeliveredBy::Notice,
                },
                Delivery {
                    recipient: "role:planner".into(),
                    at: 150,
                    by: DeliveredBy::Inbox,
                },
            ],
        },
        EndHeld {
            of: EndOf::Stage {
                stage: "code".into(),
                start: 3,
                state: StageState::Stuck,
                quiet_from: Some(160),
                end: StageEnd {
                    last_words: Some("I am waiting".into()),
                    commits: vec![Revision::new("c4")],
                    screen: Some("the agent's screen".into()),
                    ..StageEnd::default()
                },
            },
            read_at: 170,
            deliveries: vec![],
        },
        EndHeld {
            of: EndOf::Submission {
                submit: 2,
                verdict: Verdict::Landed {
                    commit: Revision::new("c9"),
                    place_removed: false,
                    branch_removed: false,
                },
            },
            read_at: 180,
            deliveries: vec![Delivery {
                recipient: "operator".into(),
                at: 181,
                by: DeliveredBy::Wait,
            }],
        },
        EndHeld {
            of: EndOf::Stall {
                since: 190,
                waiting: "a retry of code".into(),
            },
            read_at: 490,
            deliveries: vec![],
        },
    ]
}

#[test]
fn a_launch() {
    round_trips(Launch {
        repository: "/r".into(),
        kind: "bump".into(),
        name: "one".into(),
        base: "main".into(),
        brief: Some("bump it".into()),
        declaration: Some("kinds.toml".into()),
        settings: BTreeMap::from([("code".into(), settings())]),
        every_stage: settings(),
        hold: vec!["review".into()],
        hold_submission: true,
        mark: Some("team-a".into()),
        hold_words: BTreeMap::from([("review".into(), "until the design is ruled".into())]),
        priority: 5,
        recipients: vec!["operator".into()],
        stall_after: Some(30 * 60),
    });
}

#[test]
fn holds_the_waiting_move_and_the_stall_bound() {
    let held = held();
    for hold in held.holds.clone() {
        round_trips(hold.on.clone());
        round_trips(hold);
    }
    round_trips(held.waiting.clone().unwrap());
    for act in [
        Act::Retry,
        Act::Stop,
        Act::Drop,
        Act::Done,
        Act::Finish,
        Act::Tell,
        Act::Needs,
        Act::Priority,
        Act::Hold,
    ] {
        round_trips(act);
    }
    for why in [
        Waits::OnHold,
        Waits::Answer,
        Waits::Lost,
        Waits::NeverBegun,
        Waits::Unready,
        Waits::Refused,
        Waits::Stopped,
        Waits::PastBound,
        Waits::Submission,
        Waits::Delivered,
    ] {
        round_trips(why);
    }
    let value = serde_json::to_value(&held.holds).unwrap();
    assert_eq!(
        value,
        serde_json::json!([
            {"on": "stage", "stage": "review", "words": "until the design is ruled", "since": 10},
            {"on": "submission", "since": 10},
        ])
    );
    assert_eq!(held.hold_on("review"), Some(&held.holds[0]));
    assert_eq!(held.hold_on("code"), None);
    assert_eq!(held.submission_hold(), Some(&held.holds[1]));
    let mut value = serde_json::to_value(&held).unwrap();
    for field in ["holds", "waiting", "stall_after"] {
        value.as_object_mut().unwrap().remove(field);
    }
    let before: HeldState = serde_json::from_value(value).unwrap();
    assert!(before.holds.is_empty() && before.waiting.is_none());
    assert_eq!(
        before.stall_after, STALL_AFTER,
        "five minutes where none is set"
    );
    round_trips(HeldState {
        waiting: None,
        ..held
    });
    let mut launch = serde_json::to_value(Launch::default()).unwrap();
    assert!(launch.get("stall_after").is_none(), "no bound written");
    launch.as_object_mut().unwrap().remove("stall_after");
    let launch: Launch = serde_json::from_value(launch).unwrap();
    assert_eq!(launch.stall_after, None);
}

#[test]
fn a_moves_words() {
    let on = |act, stage: Option<&str>| WaitingOn {
        act,
        stage: stage.map(str::to_string),
        why: Waits::Answer,
        since: Some(1),
        words: Some("which?".into()),
    };
    assert_eq!(
        on(Act::Retry, Some("construct")).move_words(),
        "a retry of construct"
    );
    assert_eq!(
        on(Act::Retry, None).move_words(),
        "a retry of the submission"
    );
    assert_eq!(on(Act::Done, None).move_words(), "done");
    let mut other = on(Act::Retry, Some("construct"));
    other.words = None;
    other.since = Some(9);
    assert!(other.same_move(&on(Act::Retry, Some("construct"))));
    assert!(!other.same_move(&on(Act::Retry, Some("code"))));
}

#[test]
fn a_held_state() {
    round_trips(held());
    for phase in [
        Phase::Making { by: "c1".into() },
        Phase::Live,
        Phase::Gone {
            over: Over::Dropped,
            at: None,
        },
        Phase::Gone {
            over: Over::Done,
            at: Some(200),
        },
        Phase::Tidied {
            over: Over::Landed {
                commit: Revision::new("c9"),
            },
            at: 200,
            pending_for: vec!["operator".into()],
        },
        Phase::Over {
            over: Over::Done,
            kept: Some(Kept::Teardown {
                step: "release".into(),
                words: "busy".into(),
            }),
        },
        Phase::Over {
            over: Over::Dropped,
            kept: Some(Kept::AgentLive {
                identity: identity().to_string(),
            }),
        },
        Phase::Over {
            over: Over::Dropped,
            kept: Some(Kept::Unreadable {
                reason: "down".into(),
            }),
        },
    ] {
        round_trips(phase);
    }
}

#[test]
fn an_end_and_its_deliveries() {
    for end in ends() {
        round_trips(end.of.clone());
        round_trips(end.deliveries.clone());
        round_trips(end);
    }
    for by in [DeliveredBy::Notice, DeliveredBy::Wait, DeliveredBy::Inbox] {
        round_trips(by);
    }
    let before: HeldState = serde_json::from_value({
        let mut value = serde_json::to_value(held()).unwrap();
        value.as_object_mut().unwrap().remove("ends");
        value
    })
    .unwrap();
    assert!(before.ends.is_empty(), "a document from before ends");
}

#[test]
fn a_role_runs_address() {
    let held = held();
    assert_eq!(held.role.as_deref(), Some("madswan/planner"));
    let mut value = serde_json::to_value(&held).unwrap();
    assert_eq!(value["role"], "madswan/planner");
    value.as_object_mut().unwrap().remove("role");
    let before: HeldState = serde_json::from_value(value).unwrap();
    assert_eq!(before.role, None, "a document from before roles");
    round_trips(HeldState { role: None, ..held });
}

#[test]
fn a_wake_not_made_and_the_submission_before_the_launch() {
    let held = held();
    let mut value = serde_json::to_value(&held).unwrap();
    assert_eq!(
        value["unwoken"]["madswan/ops"],
        "the host's configuration names no role for madswan/ops"
    );
    assert_eq!(value["submission"]["before"], "3");
    value.as_object_mut().unwrap().remove("unwoken");
    value["submission"]
        .as_object_mut()
        .unwrap()
        .remove("before");
    let before: HeldState = serde_json::from_value(value).unwrap();
    assert!(before.unwoken.is_empty(), "a document from before wakes");
    assert_eq!(before.submission.before, None);
    round_trips(HeldState {
        unwoken: BTreeMap::new(),
        ..held
    });
}

#[test]
fn a_role_run_as_the_configuration_names_it() {
    let run = RoleRun {
        repository: "/r".into(),
        kind: "planner".into(),
        priority: 5,
        base: Some("next".into()),
        brief: Some("read the records".into()),
    };
    round_trips(run.clone());
    round_trips(RoleRun {
        base: None,
        brief: None,
        ..run
    });
}

#[test]
fn an_address_is_a_partition_and_a_role() {
    assert!(is_address("madswan/planner"));
    assert!(is_address("slc-2/ops.1"));
    assert_eq!(role_of("madswan/planner"), "planner");
    for session in [
        "ops",
        "term:p1@/tmp/term.sock",
        "a/b/c",
        "/planner",
        "madswan/",
        "role:planner",
    ] {
        assert!(!is_address(session), "{session} is a session's name");
    }
}

#[test]
fn an_ends_key_and_its_pending_recipients() {
    let ends = ends();
    let mut read_again = ends[0].of.clone();
    if let EndOf::Stage { end, .. } = &mut read_again {
        end.report = Some("read again".into());
    }
    assert!(
        ends[0].same(&read_again),
        "an end read again is the same end"
    );
    assert!(!ends[0].same(&ends[1].of), "a later start is another end");
    let recipients = vec!["operator".to_string(), "ops".to_string()];
    assert_eq!(ends[0].pending(&recipients), vec!["ops".to_string()]);
    assert_eq!(ends[1].pending(&recipients), recipients);
    assert_eq!(
        held().pending_for(),
        vec!["operator".to_string(), "role:planner".to_string()]
    );
}

#[test]
fn a_versioned_held_state() {
    round_trips(Versioned {
        version: 4,
        state: held(),
    });
    round_trips(Written::Written { version: 5 });
    round_trips(Written::Moved);
}

#[test]
fn a_kind_with_its_stages() {
    round_trips(kind());
    round_trips(KindsRead::Read {
        kinds: vec![kind()],
        settings: BTreeMap::from([("change".into(), kind_settings())]),
    });
    round_trips(HostSettings {
        repository: kind_settings(),
        kind: KindSettings::default(),
    });
    for layer in [
        Layer::Launch,
        Layer::HostRepository,
        Layer::HostKind,
        Layer::Repository,
        Layer::Kind,
        Layer::Default,
    ] {
        round_trips(layer);
        assert_eq!(
            serde_json::to_value(layer).unwrap(),
            serde_json::Value::String(layer.name().into())
        );
    }
    round_trips(KindsRead::CouldNotRead {
        reason: "no file".into(),
    });
    for declared in [Declared::Launch, Declared::Repository, Declared::Flume] {
        round_trips(declared);
    }
}

#[test]
fn a_stage_with_two_agents_keeps_both() {
    let json = serde_json::to_string(&stage()).unwrap();
    let back: Stage = serde_json::from_str(&json).unwrap();
    assert_eq!(back.agents.len(), 2);
    assert_eq!(back.deliverables.len(), 2);
}

#[test]
fn a_binding() {
    round_trips(BindingRead::Read {
        binding: held().place.binding,
    });
    round_trips(BindingRead::CouldNotRead {
        reason: "fetch failed".into(),
    });
}

#[test]
fn the_six_deliverable_forms() {
    for deliverable in [
        Deliverable::Commits,
        Deliverable::ChangeWritten,
        Deliverable::TasksTicked,
        Deliverable::ChangeArchived,
        Deliverable::Report {
            path: "{{state}}/conclusion.md".into(),
        },
        Deliverable::CommittedReport {
            path: "notes/report.md".into(),
        },
        Deliverable::Unreadable {
            spelled: "passes".into(),
        },
    ] {
        round_trips(deliverable);
    }
}

#[test]
fn a_prompt_and_its_parts() {
    round_trips(prompt());
    for part in PromptPart::ALL {
        round_trips(part);
        assert_eq!(PromptPart::named(part.name()), Some(part));
    }
}

#[test]
fn an_identity_a_secret_and_a_stop_short() {
    round_trips(identity());
    assert_eq!(identity().to_string(), "flume:/r:bump--one:code:2");
    round_trips(Secret("s".into()));
    for kind in [StopKind::NeedsAnswer, StopKind::FoundWork] {
        round_trips(kind);
        round_trips(StopShort {
            identity: identity().to_string(),
            secret: Secret("s".into()),
            words: "which of two designs?".into(),
            kind,
        });
    }
    round_trips(StageEnd {
        head: None,
        report: None,
        words: Some("the tests fail on main".into()),
        needs: Some(StopKind::FoundWork),
        ..StageEnd::default()
    });
    round_trips(StageEnd {
        head: Some(Revision::new("c4")),
        last_words: Some("Usage limit reached".into()),
        commits: vec![Revision::new("c4"), Revision::new("c3")],
        screen: Some("Do you trust the files in this folder?".into()),
        ..StageEnd::default()
    });
    let before: StageEnd =
        serde_json::from_str(r#"{"head":"c3","report":null,"words":null}"#).unwrap();
    assert_eq!(before.commits, Vec::<Revision>::new());
    assert_eq!(before.screen, None);
}

#[test]
fn what_a_stage_made() {
    let made = made();
    assert_eq!(
        made.iter().map(Made::name).collect::<Vec<_>>(),
        vec![Some("spike"), Some("preview"), None]
    );
    for line in &made {
        round_trips(line.clone());
    }
    round_trips(StageEnd {
        head: Some(Revision::new("c3")),
        made: made.clone(),
        ..StageEnd::default()
    });
    let Made::Thing(spike) = &made[0] else {
        panic!("the spike read as text");
    };
    assert_eq!(
        spike["address"], "https://forge.example/o/r/tree/spike/x?ref=a%2Fb",
        "the address as written"
    );
    assert_eq!(
        Made::read(r#"{"what":"spike"}"#),
        Made::Text(r#"{"what":"spike"}"#.into()),
        "a line naming nothing"
    );
    assert_eq!(
        Made::read(r#"["spike"]"#),
        Made::Text(r#"["spike"]"#.into()),
        "a line that is no object"
    );
    let before: StageEnd = serde_json::from_str(r#"{"head":"c3"}"#).unwrap();
    assert!(before.made.is_empty(), "an end before made");
    let start: StartHeld = {
        let mut json = serde_json::to_value(held().stages[0].start.clone().unwrap()).unwrap();
        json.as_object_mut().unwrap().remove("made_from");
        serde_json::from_value(json).unwrap()
    };
    assert_eq!(start.made_from, 0, "a start before made");
    let stage: Stage = {
        let mut json = serde_json::to_value(stage()).unwrap();
        json.as_object_mut().unwrap().remove("makes");
        serde_json::from_value(json).unwrap()
    };
    assert!(stage.makes.is_empty(), "a stage declaring nothing");
}

#[test]
fn a_stage_held_before_it_could_repeat() {
    let mut json = serde_json::to_value(kind()).unwrap();
    json["stages"][0].as_object_mut().unwrap().remove("repeats");
    let before: Kind = serde_json::from_value(json).unwrap();
    assert!(!before.stages[0].repeats, "a stage held before repeats not");
    assert_eq!(
        Kind {
            stages: vec![Stage {
                repeats: true,
                ..before.stages[0].clone()
            }],
            ..before.clone()
        },
        kind(),
        "every other field as held"
    );
    let mut state = serde_json::to_value(held()).unwrap();
    state["place"]["kind"]["stages"][0]
        .as_object_mut()
        .unwrap()
        .remove("repeats");
    let state: HeldState = serde_json::from_value(state).unwrap();
    assert!(!state.place.kind.stages[0].repeats, "a place held before");
}

#[test]
fn a_start_held_before_it_had_a_group() {
    let mut json = serde_json::to_value(held().stages[0].start.clone().unwrap()).unwrap();
    json.as_object_mut().unwrap().remove("group");
    let before: StartHeld = serde_json::from_value(json).unwrap();
    assert_eq!(before.group, None, "a start held before groups");
    let none = serde_json::to_value(&before).unwrap();
    assert!(
        none.get("group").is_none(),
        "a start with no group writes none"
    );
    let group: GroupHeld = serde_json::from_str(r#"{"index":0}"#).unwrap();
    assert_eq!(group, GroupHeld::default(), "the group above every heading");
    round_trips(group);
    round_trips(held().stages[0].start.clone().unwrap().group.unwrap());
}

#[test]
fn a_stage_and_its_end_held_before_repetitions_carried_theirs() {
    let mut json = serde_json::to_value(held().stages[0].clone()).unwrap();
    json.as_object_mut().unwrap().remove("made_from");
    json["end"].as_object_mut().unwrap().remove("group");
    let before: StageHeld = serde_json::from_value(json).unwrap();
    assert_eq!(before.made_from, None, "a stage held before");
    assert_eq!(
        before.end.as_ref().and_then(|end| end.group.clone()),
        None,
        "an end held before"
    );
    let none = serde_json::to_value(StageHeld {
        end: Some(StageEnd::default()),
        ..before
    })
    .unwrap();
    assert!(none.get("made_from").is_none(), "no count written");
    assert!(none["end"].get("group").is_none(), "no group written");
}

#[test]
fn a_submission_and_its_outcomes() {
    for state in [
        SubmissionState::Waiting,
        SubmissionState::UnderTest,
        SubmissionState::Landed {
            commit: Revision::new("c9"),
        },
        SubmissionState::Failed { words: "x".into() },
        SubmissionState::Conflict {
            files: vec!["src/lib.rs".into()],
            words: "conflicts".into(),
        },
        SubmissionState::Withdrawn,
    ] {
        round_trips(SubmissionRead::Found {
            submission: Submission {
                id: "7".into(),
                branch: "fix/one".into(),
                target: "main".into(),
                state,
            },
        });
    }
    round_trips(SubmissionRead::None);
    round_trips(SubmissionRead::CouldNotRead {
        reason: "down".into(),
    });
    for verdict in [
        Verdict::Landed {
            commit: Revision::new("c9"),
            place_removed: true,
            branch_removed: false,
        },
        Verdict::Failed { words: "x".into() },
        Verdict::Conflict {
            files: vec!["src/lib.rs".into()],
            words: "conflicts".into(),
        },
        Verdict::Withdrawn,
        Verdict::Refused {
            reason: "conflict".into(),
        },
    ] {
        round_trips(verdict);
    }
}

fn a_submission() -> Submission {
    Submission {
        id: "7".into(),
        branch: "fix/one".into(),
        target: "main".into(),
        state: SubmissionState::UnderTest,
    }
}

#[test]
fn submit_holds_no_wait() {
    round_trips(Submitted::Submitted {
        submission: a_submission(),
    });
    round_trips(Submitted::Refused {
        reason: "no such base".into(),
    });
}

#[test]
fn the_inbox_and_its_outcomes() {
    let outcomes = vec![
        SubmissionOutcome::Landed {
            commit: Revision::new("c9"),
        },
        SubmissionOutcome::Failed {
            words: "check test-x failed".into(),
        },
        SubmissionOutcome::Conflict {
            files: vec!["src/lib.rs".into(), "Cargo.lock".into()],
            base: Revision::new("c8"),
            words: "conflicts with main at c8".into(),
        },
    ];
    for outcome in &outcomes {
        round_trips(outcome.clone());
    }
    round_trips(Inbox::Outcomes { outcomes });
    round_trips(Inbox::Outcomes { outcomes: vec![] });
    round_trips(Inbox::CouldNotRead {
        reason: "down".into(),
    });
}

#[test]
fn a_step() {
    round_trips(Stepped::Stepped);
    round_trips(Stepped::MovesOnItsOwn);
    round_trips(Stepped::CouldNotRead {
        reason: "down".into(),
    });
}

#[test]
fn where_the_base_lives() {
    round_trips(BaseRevision::At {
        revision: Revision::new("c1"),
    });
    round_trips(BaseRevision::Refused {
        reason: "fetch failed".into(),
    });
}

#[test]
fn agent_and_stage_states() {
    for state in [
        AgentState::Working,
        AgentState::Waiting,
        AgentState::Asking,
        AgentState::Stopped,
        AgentState::Unknown,
    ] {
        round_trips(state);
    }
    for state in [
        StageState::NotReached,
        StageState::OnHold,
        StageState::WaitingOnReadiness,
        StageState::WaitingForRoom,
        StageState::Refused,
        StageState::Starting,
        StageState::Started,
        StageState::Begun,
        StageState::NeverBegun,
        StageState::Stuck,
        StageState::Lost,
        StageState::Stopped,
        StageState::Delivered,
        StageState::Repeating,
        StageState::EndedShort,
        StageState::Finished,
    ] {
        round_trips(state);
    }
}

#[test]
fn the_place_seams_answers() {
    round_trips(held().place.place_ref());
    for made in [
        MakeOutcome::Made,
        MakeOutcome::BranchExists,
        MakeOutcome::LocationExists,
        MakeOutcome::Refused {
            reason: "disk full".into(),
        },
    ] {
        round_trips(made);
    }
    round_trips(readiness());
    round_trips(PlaceRead::Live {
        branch: "bump/one".into(),
        head: Revision::new("c2"),
        holds: holds(),
    });
    round_trips(PlaceRead::Gone);
    round_trips(PlaceRead::CouldNotRead {
        reason: "no git".into(),
    });
    round_trips(Removed::Removed);
    round_trips(Removed::Refused {
        step: "pre-remove".into(),
        words: "busy".into(),
    });
    round_trips(BranchRemoved::Removed);
    round_trips(BranchRemoved::Gone);
    round_trips(BranchRemoved::Kept {
        commits: vec![Revision::new("c4")],
    });
}

fn change_facts() -> ChangeFacts {
    ChangeFacts {
        exists: true,
        planning_complete: true,
        tasks: vec![
            Task {
                text: "0.1 above every heading".into(),
                ticked: true,
                group: Some(0),
            },
            Task {
                text: "1.1 do it".into(),
                ticked: true,
                group: Some(1),
            },
            Task {
                text: "2.1 then this".into(),
                ticked: false,
                group: Some(2),
            },
        ],
        archived: false,
        groups: vec![
            TaskGroup::default(),
            TaskGroup {
                number: Some("1".into()),
                title: Some("Parser".into()),
            },
            TaskGroup {
                number: Some("2".into()),
                title: Some("Tests".into()),
            },
        ],
    }
}

#[test]
fn the_change_seams_answers() {
    round_trips(ChangeRead::Read {
        facts: change_facts(),
    });
    round_trips(ChangeRead::CouldNotRead {
        reason: "bad schema".into(),
    });
}

#[test]
fn a_change_read_before_its_groups() {
    let mut json = serde_json::to_value(change_facts()).unwrap();
    json.as_object_mut().unwrap().remove("groups");
    for task in json["tasks"].as_array_mut().unwrap() {
        task.as_object_mut().unwrap().remove("group");
    }
    let before: ChangeFacts = serde_json::from_value(json).unwrap();
    assert!(before.groups.is_empty(), "no groups");
    assert!(
        before.tasks.iter().all(|task| task.group.is_none()),
        "every task in no group"
    );
    assert_eq!(before.first_unticked_group(), None, "no group to take");
    assert_eq!(before.ticked(), 2, "the tasks as held");
}

#[test]
fn the_first_unticked_group_and_a_groups_tasks() {
    let mut facts = change_facts();
    assert_eq!(facts.first_unticked_group(), Some(2));
    assert_eq!(
        facts
            .tasks_of(1)
            .map(|task| task.text.as_str())
            .collect::<Vec<_>>(),
        vec!["1.1 do it"]
    );
    assert_eq!(facts.tasks_of(3).count(), 0, "a group the change lacks");
    facts.tasks[0].ticked = false;
    assert_eq!(facts.first_unticked_group(), Some(0), "the group above");
    facts.tasks.iter_mut().for_each(|task| task.ticked = true);
    assert_eq!(facts.first_unticked_group(), None, "every task ticked");
    facts.tasks.push(Task {
        text: "9.9 in no group".into(),
        ticked: false,
        group: None,
    });
    assert_eq!(facts.first_unticked_group(), None, "a task in no group");
}

#[test]
fn the_agent_seams_answers() {
    round_trips(AgentCommand {
        program: "agent".into(),
        args: vec!["--model".into(), "large".into()],
        env: BTreeMap::from([("ID".into(), identity().to_string())]),
        prompt_args: vec!["do it".into()],
        folder_trust: Some(FolderTrust {
            question: "Yes, I trust this folder".into(),
            focus: ">".into(),
            next: "down".into(),
            confirm: "enter".into(),
        }),
        kind: Some("agent".into()),
    });
    let before: AgentCommand =
        serde_json::from_str(r#"{"program":"agent","args":[],"env":{},"prompt_args":[]}"#).unwrap();
    assert_eq!(
        before.kind, None,
        "a command written before it named a kind"
    );
    for reading in [Reading::Yes, Reading::No, Reading::Unknown] {
        round_trips(reading);
    }
    round_trips(Received::Received {
        record: "r1".into(),
    });
    round_trips(Received::NotReceived {
        record: "r1".into(),
    });
    round_trips(Received::Unknown);
    round_trips(Offered::Offered);
    round_trips(Offered::NotOffered);
    round_trips(Guarded::Refused {
        acts: vec!["submit".into()],
    });
    round_trips(Guarded::NotOffered);
}

#[test]
fn the_terminal_seams_answers() {
    round_trips(Opened::Opened {
        carried: true,
        printed: Some("cd /r/places/fix--one && agent 'do it'".into()),
    });
    round_trips(Opened::Opened {
        carried: false,
        printed: None,
    });
    let before: Opened = serde_json::from_str(r#"{"opened":{"carried":true}}"#).unwrap();
    assert_eq!(
        before,
        Opened::Opened {
            carried: true,
            printed: None
        }
    );
    round_trips(Opened::Refused {
        reason: "identity live".into(),
    });
    round_trips(Opened::Held {
        screen: "WARNING: Bypass Permissions mode".into(),
    });
    round_trips(Opened::NotComeUp {
        screen: "agent: command not found".into(),
    });
    round_trips(TerminalTidy {
        closed: vec!["the tab fix--one".into()],
        kept: vec!["the workspace r main, held by the pane w1:p3".into()],
    });
    round_trips(Clues {
        stage: "code".into(),
        state: "working".into(),
        agent: Some(identity()),
        base: BaseCount {
            running: 3,
            asking: 1,
            waiting_on_caller: 1,
            stalled: 0,
        },
    });
    assert_eq!(
        BaseCount {
            running: 3,
            asking: 1,
            waiting_on_caller: 1,
            stalled: 0
        }
        .to_string(),
        "3 running · 1 asking · 1 waiting on caller · 0 stalled"
    );
    round_trips(ShownAt {
        place: Some("tab w1:t2".into()),
        agent: Some("pane w1:p3".into()),
        base: Some("workspace w1".into()),
    });
    round_trips(ShownAt::default());
    round_trips(Shown::Shown);
    round_trips(Shown::ShowsNone);
    round_trips(Shown::Failed {
        reason: "the terminal refused".into(),
    });
    round_trips(Sent::Sent);
    round_trips(Sent::Refused {
        reason: "cannot send".into(),
    });
    round_trips(ToSession::Sent);
    round_trips(ToSession::NotFound);
    round_trips(ToSession::Refused {
        reason: "the session asks a question".into(),
    });
    round_trips(Some(AgentState::Waiting));
    round_trips(None::<AgentState>);
    round_trips(Closed::Closed);
    round_trips(Closed::Refused {
        reason: "still running".into(),
    });
    round_trips(Watch::Bound { seconds: 30 });
    round_trips(Watch::None);
    round_trips(Some("pane-3".to_string()));
    round_trips(Some("Do you trust the files in this folder?".to_string()));
}

#[test]
fn the_room_guard_holds_until_dropped() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    struct Release(Arc<AtomicBool>);
    impl Drop for Release {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }
    let released = Arc::new(AtomicBool::new(false));
    let guard = RoomGuard::new(Release(released.clone()));
    assert!(!released.load(Ordering::SeqCst));
    drop(guard);
    assert!(released.load(Ordering::SeqCst));
}

#[test]
fn the_historys_answers() {
    round_trips(Touch {
        commit: Revision::new("c3"),
        since: true,
    });
}

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
        beside: Some("operator".into()),
        mode: None,
        session: Some("office".into()),
        programs: BTreeMap::new(),
    }
}

/// Own settings for two programs: one's keys given inline over a named
/// file whose `env` and nested `permissions.default` its profile
/// reserved and left out, the other's given inline alone (195).
fn own_programs() -> BTreeMap<String, ProgramSettings> {
    BTreeMap::from([
        (
            "agent".into(),
            ProgramSettings {
                file: Some(NamedFile {
                    written: "~/.config/agent/settings.json".into(),
                    read: "/home/operator/.config/agent/settings.json".into(),
                    keys: BTreeMap::from([
                        ("tips".into(), serde_json::json!(false)),
                        (
                            "permissions".into(),
                            serde_json::json!({"allow": ["Read", "Bash(git log:*)"]}),
                        ),
                    ]),
                    left_out: vec!["env".into(), "permissions.default".into()],
                }),
                settings: BTreeMap::from([(
                    "line".into(),
                    serde_json::json!({"type": "command", "command": "line", "padding": 1}),
                )]),
            },
        ),
        (
            "other".into(),
            ProgramSettings {
                file: None,
                settings: BTreeMap::from([("theme".into(), serde_json::json!("dark"))]),
            },
        ),
    ])
}

/// A repository's settings for every kind run in it: what its agents'
/// commits and pull requests say of who made them (195).
fn every_kind() -> Settings {
    Settings {
        programs: BTreeMap::from([(
            "agent".into(),
            ProgramSettings {
                file: None,
                settings: BTreeMap::from([(
                    "attribution".into(),
                    serde_json::json!({"commit": "", "pr": "", "sessionUrl": false}),
                )]),
            },
        )]),
        ..Settings::default()
    }
}

/// A mode as a table of programs: an own value under one program's name
/// and one of flume's four under another's (169).
fn programs() -> ModeSetting {
    ModeSetting::Programs(BTreeMap::from([
        ("agent".into(), "careful".into()),
        ("other".into(), "edits".into()),
    ]))
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
        label: Some("operation".into()),
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
            mode: Some(ModeSetting::Flume(Mode::Edits)),
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

/// A kind's identity of the host's source `acme`, as a reader spells it.
fn kind_identity() -> KindIdentity {
    KindIdentity {
        source: "acme".into(),
        name: "review".into(),
        version: 2,
        spelled: "acme/review@2".into(),
    }
}

fn hash() -> String {
    format!("sha256:{}", "ab".repeat(32))
}

/// A kind of the host's source `acme`, locked in the host's lock (19, 21,
/// 155, 191).
fn kind_from() -> KindFrom {
    KindFrom {
        identity: kind_identity(),
        source: "acme".into(),
        source_file: Some("/config/flume/sources/acme.toml".into()),
        kind_dir: Some("/data/flume/kinds/acme/review@2".into()),
        declaration_file: None,
        revision: None,
        hash: Some(hash()),
        lock_file: Some("/config/flume/flume.lock".into()),
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
            declared: Declared::Host,
            kind_from: Some(kind_from()),
            name: "one".into(),
            branch: "bump/one".into(),
            base: "main".into(),
            base_revision: Revision::new("c1"),
            base_from: Some(BaseFrom::OriginLacks),
            location: "/r/places/bump--one".into(),
            binding: Binding {
                place: Some("place-tool".into()),
                change: Some("change-tool".into()),
                merge_queue: Some("queue-tool".into()),
            },
            brief: Some("bump it".into()),
            mark: Some("team-a".into()),
            settings: BTreeMap::from([(
                "code".into(),
                Settings {
                    mode: Some(programs()),
                    programs: own_programs(),
                    ..settings()
                },
            )]),
            every_stage: settings(),
            repository_settings: kind_settings(),
            repository_agents: every_kind(),
            hold: vec!["review".into()],
            hold_submission: true,
            hold_words: BTreeMap::from([("review".into(), "until the design is ruled".into())]),
            recipients: vec!["operator".into(), "role:planner".into()],
            recipients_from: Some(RecipientsFrom::Launch),
            stream: StreamHeld {
                name: "work".into(),
                from: StreamFrom::HostRepository,
            },
            intent: IntentKey::Launch {
                name: "the-surfaces".into(),
                digest: "d1".into(),
            },
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
                left_in_pane: false,
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
                unsettled: Some(Box::new(unsettled())),
            }),
            room: Some(RoomWait {
                live_unread: None,
                live: vec![],
                order: 3,
                words: Some("the merge queue's words".into()),
                carry: Some(CarryFrom {
                    start: Identity {
                        start: 1,
                        ..identity()
                    },
                    cause: CarryCause::Spent,
                    carry: true,
                    why: None,
                }),
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
                settings: Settings {
                    mode: Some(programs()),
                    ..settings()
                },
                from: BTreeMap::from([
                    (
                        "model".into(),
                        Given {
                            layer: Layer::Repository,
                            value: "opus".into(),
                            file: None,
                        },
                    ),
                    (
                        "mode".into(),
                        Given {
                            layer: Layer::Launch,
                            value: programs().written(),
                            file: None,
                        },
                    ),
                    (
                        "env.AGENT_HOME".into(),
                        Given {
                            layer: Layer::Account,
                            value: "~/.agent-work".into(),
                            file: None,
                        },
                    ),
                    (
                        "programs.agent.settings.line".into(),
                        Given {
                            layer: Layer::HostStream,
                            value: r#"{"type":"command","command":"work-line"}"#.into(),
                            file: None,
                        },
                    ),
                    (
                        "programs.agent.settings.tips".into(),
                        Given {
                            layer: Layer::Host,
                            value: "false".into(),
                            file: Some("/home/operator/.config/agent/settings.json".into()),
                        },
                    ),
                ]),
                account: Some(AccountHeld {
                    pool: "work".into(),
                    name: "work-2".into(),
                    from: AccountFrom::Strategy,
                }),
                start_record: start_record(),
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
                opening: Some("4242@Thu Oct 30 15:45:50 2026".into()),
                dialog: Some(Dialog {
                    words: "New MCP server found in .mcp.json: docs".into(),
                    since: 6,
                }),
                conversation: Some(Conversation {
                    from: 1,
                    cause: CarryCause::Spent,
                    carried: true,
                    why: None,
                }),
                session: Some(SessionHeld {
                    name: Some("office".into()),
                    from: SessionFrom::Launch,
                }),
                label: Some(LabelHeld {
                    name: "operation".into(),
                    from: LabelFrom::Kind,
                }),
                terminal: Some(ProfileHeld {
                    profile: "fake".into(),
                    from: NamedBy::Stream,
                }),
                own: Some(OwnHeld {
                    files: vec![OwnFile {
                        layer: Layer::Host,
                        written: "~/.config/agent/settings.json".into(),
                        read: "/home/operator/.config/agent/settings.json".into(),
                        left_out: vec!["env".into()],
                    }],
                    passed: Some("--own /state/places/x/agent/own.json".into()),
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
            request: Some("56".into()),
            before: Some("3".into()),
            verdict: Some(Verdict::Conflict {
                files: vec!["src/lib.rs".into()],
                words: "conflicts with main".into(),
            }),
            waiting: Some("withdrawn".into()),
            waiting_since: Some(130),
            unread: Some("the merge queue is down".into()),
            passed_over: vec![PassedOver {
                branch: "bolt/x".into(),
                words: "bolt/x's policy cannot be read".into(),
            }],
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
        moves: vec![
            AccountMove {
                on: MoveOn::Stage {
                    stage: "code".into(),
                },
                from: "work-1".into(),
                to: "work-2".into(),
                why: MoveWhy::Spent,
                at: 110,
                carried: Some(true),
                window: None,
                until: None,
            },
            AccountMove {
                on: MoveOn::Intent {
                    program: Some("agent".into()),
                },
                from: "work-1".into(),
                to: "work-2".into(),
                why: MoveWhy::Full,
                at: 110,
                carried: None,
                window: None,
                until: None,
            },
        ],
        unsettled: vec![UnsettledStart {
            stage: "code".into(),
            start: 1,
            at: 4,
            unsettled: unsettled(),
        }],
        uncarried: vec![UncarriedStart {
            stage: "code".into(),
            start: 3,
            at: 8,
            words: "the start that carried its conversation ended before it said it started".into(),
        }],
        terminals: vec![ProfileHeld {
            profile: "printed".into(),
            from: NamedBy::Host,
        }],
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

/// A start's record: the program, the account, two digests, no value,
/// the mode its profile passed and the session it resumes (161, 162,
/// 179).
fn start_record() -> StartRecord {
    StartRecord {
        program: "agent".into(),
        account: Some("work-2".into()),
        env: BTreeMap::from([
            ("AGENT_HOME".into(), "9f2c".into()),
            ("FLUME_AGENT".into(), "41ab".into()),
        ]),
        mode: Some("careful".into()),
        session: Some("4e3c05c8-51c4-4449-8577-4f7038e394bf".into()),
    }
}

/// An agent begun without one variable, with another value of a second,
/// under another program, in another conversation than its start resumes
/// and in another mode than its start asked (161, 162, 179).
fn unsettled() -> Unsettled {
    Unsettled {
        missing: vec!["AGENT_HOME".into()],
        different: vec!["FLUME_STAGE".into()],
        program: true,
        session: true,
        mode: Some(ModeBegan {
            asked: "careful".into(),
            began: Some("ask-each".into()),
        }),
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
        stream: Some("work".into()),
        intent: Some("the-surfaces".into()),
    });
}

#[test]
fn an_intent_by_its_name_or_its_repository_and_base() {
    let named = IntentKey::Launch {
        name: "the-surfaces".into(),
        digest: "d1".into(),
    };
    let based = IntentKey::Base {
        repository: "/r".into(),
        base: "main".into(),
        digest: "d2".into(),
    };
    round_trips(named.clone());
    round_trips(based.clone());
    assert_eq!((named.digest(), based.digest()), ("d1", "d2"));
    assert_eq!(
        serde_json::to_value(&named).unwrap()["from"],
        serde_json::json!("launch")
    );
    assert_eq!(
        serde_json::to_value(&based).unwrap()["from"],
        serde_json::json!("base")
    );
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
        Act::Answer,
    ] {
        round_trips(act);
    }
    for why in [
        Waits::OnHold,
        Waits::Answer,
        Waits::Lost,
        Waits::NeverBegun,
        Waits::Dialog,
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
    round_trips(start_record());
    round_trips(StartRecord {
        account: None,
        ..start_record()
    });
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
fn a_held_state_with_a_stream_an_account_and_a_record() {
    // A place in a stream with where it came from, an intent, and a start
    // under an account with its start's record, which holds each
    // variable's digest by its name and no value (143 to 147, 161).
    let state = held();
    round_trips(state.clone());
    let json = serde_json::to_value(&state).expect("held state as JSON");
    assert_eq!(json["place"]["stream"]["name"], "work");
    assert_eq!(json["place"]["intent"]["name"], "the-surfaces");
    let start = &json["stages"][0]["start"];
    assert_eq!(start["account"]["name"], "work-2");
    assert_eq!(start["start_record"]["account"], "work-2");
    assert_eq!(start["start_record"]["env"]["AGENT_HOME"], "9f2c");
    assert_eq!(start["from"]["env.AGENT_HOME"]["layer"], "account");
}

#[test]
fn a_held_state_with_where_its_recipients_came_from() {
    // Each of the four a launch takes its recipients from reads back
    // unchanged, the answerer with its source; a state written before
    // reads with none, and writes none (7, 193).
    for from in [
        RecipientsFrom::Launch,
        RecipientsFrom::Environment,
        RecipientsFrom::Session,
        RecipientsFrom::Answerer {
            source: "acme".into(),
        },
    ] {
        let mut state = held();
        state.place.recipients = vec!["east/operator".into()];
        state.place.recipients_from = Some(from);
        round_trips(state);
    }
    let mut state = held();
    state.place.recipients_from = Some(RecipientsFrom::Answerer {
        source: "acme".into(),
    });
    let json = serde_json::to_value(&state).expect("held state as JSON");
    assert_eq!(json["place"]["recipients_from"]["from"], "answerer");
    assert_eq!(json["place"]["recipients_from"]["source"], "acme");
    let mut before = json;
    before["place"]
        .as_object_mut()
        .expect("the place")
        .remove("recipients_from");
    let read: HeldState = serde_json::from_value(before).expect("a state written before");
    assert_eq!(read.place.recipients_from, None);
    let written = serde_json::to_value(&read).expect("held state as JSON");
    assert!(written["place"].get("recipients_from").is_none());
}

/// `value` with every `mode` key taken out, at any depth.
fn without_modes(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(object) => {
            object.remove("mode");
            object.values_mut().for_each(without_modes);
        }
        serde_json::Value::Array(items) => items.iter_mut().for_each(without_modes),
        _ => {}
    }
}

#[test]
fn a_held_state_with_a_mode() {
    // The launch gives code a table of programs, the kind gives every
    // stage `edits`, and the start records the value passed and began in
    // another (161, 162, 169).
    let state = held();
    round_trips(state.clone());
    let json = serde_json::to_value(&state).expect("held state as JSON");
    assert_eq!(
        json["place"]["settings"]["code"]["mode"],
        serde_json::json!({"agent": "careful", "other": "edits"})
    );
    assert_eq!(
        json["place"]["kind"]["agents"]["every_stage"]["mode"],
        "edits"
    );
    let start = &json["stages"][0]["start"];
    assert_eq!(start["start_record"]["mode"], "careful");
    assert_eq!(
        start["from"]["mode"]["value"],
        r#"{ agent = "careful", other = "edits" }"#
    );
    assert_eq!(
        json["unsettled"][0]["mode"],
        serde_json::json!({"asked": "careful", "began": "ask-each"})
    );

    // The same state written before the mode reads with none, and writes
    // none back.
    let mut before = json.clone();
    without_modes(&mut before);
    let read: HeldState = serde_json::from_value(before.clone()).expect("read with no mode");
    assert_eq!(read.place.settings["code"].mode, None);
    assert_eq!(read.place.kind.agents.every_stage.mode, None);
    let start = read.stages[0].start.as_ref().expect("a start");
    assert_eq!(start.start_record.mode, None);
    assert_eq!(start.settings.mode, None);
    assert_eq!(read.unsettled[0].unsettled.mode, None);
    assert_eq!(
        serde_json::to_value(&read).expect("written again"),
        before,
        "no mode written where none is held"
    );
}

#[test]
fn a_bare_word_of_a_programs_own() {
    // A program's own value goes under its program's name, never bare,
    // wherever a mode is read (169).
    let refused = serde_json::from_str::<ModeSetting>(r#""auto""#)
        .expect_err("a bare word of a program's own");
    let words = refused.to_string();
    assert!(words.contains("`auto`"), "{words}");
    assert!(
        words.contains("a program's own value goes under its program's name"),
        "{words}"
    );
    assert!(
        serde_json::from_str::<Settings>(r#"{"mode":"auto"}"#).is_err(),
        "settings with a bare word of a program's own"
    );
    assert!(
        serde_json::from_str::<ModeSetting>("{}").is_err(),
        "a table naming no program"
    );
    for mode in Mode::ALL {
        round_trips(mode);
        round_trips(ModeSetting::Flume(mode));
        round_trips(ModeEntry::Flume(mode));
    }
    round_trips(programs());
    round_trips(ModeEntry::Own("careful".into()));
    round_trips(ModeBegan {
        asked: "careful".into(),
        began: None,
    });
    assert_eq!(
        serde_json::to_value(ModeSetting::Flume(Mode::Full)).unwrap(),
        "full"
    );
}

#[test]
fn a_mode_read_from_its_word() {
    for (word, mode) in [
        ("plan", Mode::Plan),
        ("ask", Mode::Ask),
        ("edits", Mode::Edits),
        ("full", Mode::Full),
    ] {
        assert_eq!(ModeSetting::from_word(word), Ok(ModeSetting::Flume(mode)));
        assert_eq!(mode.to_string(), word);
    }
    let words = ModeSetting::from_word("auto").expect_err("an own value given bare");
    assert_eq!(
        words,
        "the mode `auto` is none of flume's four, plan, ask, edits and full: a program's own \
         value goes under its program's name, as mode = { <program> = \"auto\" }"
    );
    assert!(
        ModeSetting::from_word("Edits").is_err(),
        "a mode is spelled as written"
    );
    assert!(ModeSetting::from_programs(BTreeMap::new()).is_err());
    assert_eq!(
        ModeSetting::from_programs(BTreeMap::from([("agent".into(), "careful".into())])),
        Ok(ModeSetting::Programs(BTreeMap::from([(
            "agent".into(),
            "careful".into()
        )])))
    );
}

#[test]
fn a_mode_as_its_layer_wrote_it() {
    assert_eq!(ModeSetting::Flume(Mode::Edits).written(), "edits");
    assert_eq!(
        programs().written(),
        r#"{ agent = "careful", other = "edits" }"#,
        "each program in order"
    );
}

#[test]
fn an_agents_entry_of_a_mode() {
    // One of the four is every program's; a table's entry is its
    // program's alone, one of the four or an own value, and a program it
    // names no entry for has none (169).
    let full = ModeSetting::Flume(Mode::Full);
    assert_eq!(full.entry("agent"), Some(ModeEntry::Flume(Mode::Full)));
    assert_eq!(full.entry("other"), Some(ModeEntry::Flume(Mode::Full)));
    assert_eq!(
        programs().entry("agent"),
        Some(ModeEntry::Own("careful".into()))
    );
    assert_eq!(
        programs().entry("other"),
        Some(ModeEntry::Flume(Mode::Edits))
    );
    assert_eq!(programs().entry("third"), None, "no entry for the program");
    assert_eq!(ModeEntry::Own("careful".into()).to_string(), "careful");
    assert_eq!(ModeEntry::Flume(Mode::Plan).to_string(), "plan");
}

#[test]
fn a_start_begun_in_another_mode_in_words() {
    let began = |began: Option<&str>| Unsettled {
        mode: Some(ModeBegan {
            asked: "all".into(),
            began: began.map(String::from),
        }),
        ..Unsettled::default()
    };
    assert_eq!(
        began(Some("ask-each")).words(),
        "in the mode `ask-each`, where its start asked `all`",
        "both modes named"
    );
    assert_eq!(
        began(None).words(),
        "in a mode its word did not name, where its start asked `all`",
        "a mode with none begun"
    );
    assert_eq!(
        Unsettled {
            missing: vec!["AGENT_HOME".into()],
            ..began(Some("ask-each"))
        }
        .words(),
        "without AGENT_HOME, in the mode `ask-each`, where its start asked `all`"
    );
}

#[test]
fn a_start_begun_in_another_conversation_in_words() {
    // The words say another conversation and name no session, which the
    // start's record holds and status never gives (56, 161, 179).
    let session = start_record().session.expect("a session");
    let other = Unsettled {
        session: true,
        ..Unsettled::default()
    };
    assert_eq!(
        other.words(),
        "in another conversation than the one its start resumes"
    );
    assert!(!other.words().contains(&session), "{}", other.words());
    assert_eq!(
        Unsettled {
            missing: vec!["AGENT_HOME".into()],
            ..other
        }
        .words(),
        "without AGENT_HOME, in another conversation than the one its start resumes"
    );
}

#[test]
fn a_held_state_with_a_carried_start() {
    // A stage waits after a spent agent, its start carries a conversation
    // with its record's session, its move says carried, and the place
    // lists a start carried that did not begin (146, 161, 179).
    let state = held();
    round_trips(state.clone());
    let json = serde_json::to_value(&state).expect("held state as JSON");
    let stage = &json["stages"][0];
    assert_eq!(stage["room"]["carry"]["start"]["start"], 1);
    assert_eq!(stage["room"]["carry"]["carry"], true);
    assert!(
        stage["room"]["carry"].get("why").is_none(),
        "a why where it may carry"
    );
    assert_eq!(
        stage["start"]["conversation"],
        serde_json::json!({"from": 1, "carried": true})
    );
    assert_eq!(
        stage["start"]["start_record"]["session"],
        "4e3c05c8-51c4-4449-8577-4f7038e394bf"
    );
    assert_eq!(json["moves"][0]["carried"], true);
    assert!(
        json["moves"][1].get("carried").is_none(),
        "an intent's move"
    );
    assert_eq!(json["unsettled"][0]["session"], true);
    assert_eq!(
        json["uncarried"][0],
        serde_json::json!({
            "stage": "code",
            "start": 3,
            "at": 8,
            "words": "the start that carried its conversation ended before it said it started",
        })
    );
    round_trips(CarryFrom {
        start: identity(),
        cause: CarryCause::Spent,
        carry: false,
        why: Some("the start that carried its conversation ended before it said it started".into()),
    });
    round_trips(Conversation {
        from: 1,
        cause: CarryCause::Spent,
        carried: false,
        why: Some("its agent profile carries no conversation".into()),
    });
}

#[test]
fn a_held_state_written_before_carried_starts() {
    // The same state written before this change reads with no wait after
    // a spent agent, no conversation, no session, no move carried and no
    // start carried that did not begin, and writes none back (146).
    let mut before = serde_json::to_value(held()).expect("held state as JSON");
    let stage = &mut before["stages"][0];
    stage["room"].as_object_mut().unwrap().remove("carry");
    stage["start"]
        .as_object_mut()
        .unwrap()
        .remove("conversation");
    stage["start"]["start_record"]
        .as_object_mut()
        .unwrap()
        .remove("session");
    for moved in before["moves"].as_array_mut().unwrap() {
        moved.as_object_mut().unwrap().remove("carried");
    }
    before["unsettled"][0]
        .as_object_mut()
        .unwrap()
        .remove("session");
    before.as_object_mut().unwrap().remove("uncarried");
    before.as_object_mut().unwrap().remove("terminals");
    let read: HeldState = serde_json::from_value(before.clone()).expect("read from before");
    let stage = &read.stages[0];
    assert_eq!(stage.room.as_ref().expect("a room").carry, None);
    let start = stage.start.as_ref().expect("a start");
    assert_eq!(start.conversation, None);
    assert_eq!(start.start_record.session, None);
    assert!(read.moves.iter().all(|moved| moved.carried.is_none()));
    assert!(!read.unsettled[0].unsettled.session);
    assert!(read.uncarried.is_empty());
    assert!(read.terminals.is_empty());
    assert_eq!(
        serde_json::to_value(&read).expect("written again"),
        before,
        "nothing of a carried start written where none is held"
    );
}

/// A held state whose talk was reported lost and its caller retried it:
/// its wait carries the conversation of the lost start 1, as live last
/// read it run by process 4242, and its start 2 carries that conversation
/// under work-2, the account start 1 ran under, chosen from its
/// conversation; its end before says flume left the agent the terminal
/// brought back in its pane (43, 49, 145, 179, 207).
fn talk() -> HeldState {
    let mut state = held();
    let talk = Identity {
        stage: "talk".into(),
        ..identity()
    };
    let stage = &mut state.stages[0];
    stage.name = "talk".into();
    stage.state = StageState::WaitingForRoom;
    stage.end = Some(StageEnd {
        left_in_pane: true,
        ..StageEnd::default()
    });
    stage.room = Some(RoomWait {
        order: 4,
        words: Some("we were on the second option".into()),
        carry: Some(CarryFrom {
            start: Identity {
                start: 1,
                ..talk.clone()
            },
            cause: CarryCause::Lost,
            carry: true,
            why: None,
        }),
        live: vec![LiveProcess {
            pid: 4242,
            started: 100,
        }],
        live_unread: None,
    });
    let start = stage.start.as_mut().expect("a start");
    start.identity = talk;
    start.account = Some(AccountHeld {
        pool: "work".into(),
        name: "work-2".into(),
        from: AccountFrom::Conversation,
    });
    start.conversation = Some(Conversation {
        from: 1,
        cause: CarryCause::Lost,
        carried: true,
        why: None,
    });
    state
}

/// The seam-contracts spec's scenario "A lost conversation's wait and
/// conversation" (49, 179).
#[test]
fn a_lost_conversations_wait_and_conversation() {
    let state = talk();
    round_trips(state.clone());
    let json = serde_json::to_value(&state).expect("held state as JSON");
    let stage = &json["stages"][0];
    assert_eq!(stage["room"]["carry"]["start"]["start"], 1);
    assert_eq!(stage["room"]["carry"]["cause"], "lost");
    assert_eq!(
        stage["start"]["conversation"],
        serde_json::json!({"from": 1, "cause": "lost", "carried": true})
    );
    // A spent start's wait and conversation write no cause, so what a
    // reader of either wrote before stands (core.14).
    let spent = serde_json::to_value(held()).expect("held state as JSON");
    assert!(spent["stages"][0]["room"]["carry"].get("cause").is_none());
    assert!(spent["stages"][0]["start"]["conversation"]
        .get("cause")
        .is_none());
    // One whose wait and conversation name no cause reads as spent.
    let mut before = json;
    let stage = &mut before["stages"][0];
    stage["room"]["carry"]
        .as_object_mut()
        .unwrap()
        .remove("cause");
    stage["start"]["conversation"]
        .as_object_mut()
        .unwrap()
        .remove("cause");
    let read: HeldState = serde_json::from_value(before.clone()).expect("read from before");
    let stage = &read.stages[0];
    let carry = stage.room.as_ref().and_then(|room| room.carry.as_ref());
    assert_eq!(carry.map(|carry| carry.cause), Some(CarryCause::Spent));
    let start = stage.start.as_ref().expect("a start");
    assert_eq!(
        start
            .conversation
            .as_ref()
            .map(|conversation| conversation.cause),
        Some(CarryCause::Spent)
    );
    assert_eq!(
        serde_json::to_value(&read).expect("written again"),
        before,
        "no cause written where a spent one is held"
    );
    assert_eq!(
        (CarryCause::Spent.name(), CarryCause::Lost.name()),
        ("spent", "lost")
    );
}

/// The seam-contracts spec's scenario "An account from the conversation
/// and an agent left" (43, 145).
#[test]
fn an_account_from_the_conversation_and_an_agent_left() {
    let state = talk();
    round_trips(state.clone());
    round_trips(AccountFrom::Conversation);
    let json = serde_json::to_value(&state).expect("held state as JSON");
    let stage = &json["stages"][0];
    assert_eq!(
        stage["start"]["account"],
        serde_json::json!({"pool": "work", "name": "work-2", "from": "conversation"})
    );
    assert_eq!(stage["end"]["left_in_pane"], true);
    // An end written before reads with no agent left, and writes none.
    let held = serde_json::to_value(held()).expect("held state as JSON");
    assert!(held["stages"][0]["end"].get("left_in_pane").is_none());
    for end in held["ends"].as_array().unwrap() {
        assert!(end["end"].get("left_in_pane").is_none(), "{end}");
    }
    let read: HeldState = serde_json::from_value(held).expect("read from before");
    assert!(!read.stages[0].end.as_ref().expect("an end").left_in_pane);
}

/// The seam-contracts spec's scenario "A wait on a live session" (49, 64,
/// 207).
#[test]
fn a_wait_on_a_live_session() {
    let state = talk();
    round_trips(state.clone());
    let json = serde_json::to_value(&state).expect("held state as JSON");
    assert_eq!(
        json["stages"][0]["room"]["live"],
        serde_json::json!([{"pid": 4242, "started": 100}])
    );
    assert!(json["stages"][0]["room"].get("live_unread").is_none());
    // A wait on a session live could not read, naming a variable and no
    // path (core.30).
    let unread = RoomWait {
        order: 5,
        words: None,
        carry: None,
        live: vec![],
        live_unread: Some("the sessions under CLAUDE_CONFIG_DIR cannot be read".into()),
    };
    round_trips(unread.clone());
    assert!(serde_json::to_value(&unread)
        .expect("a wait as JSON")
        .get("live")
        .is_none());
    // A place kept while its session runs, by the process that runs it,
    // or by what could not be read.
    let kept = Phase::Over {
        over: Over::Dropped,
        kept: Some(Kept::SessionRuns {
            processes: vec![LiveProcess {
                pid: 4242,
                started: 100,
            }],
            unread: None,
        }),
    };
    round_trips(kept.clone());
    assert_eq!(
        serde_json::to_value(&kept).expect("a phase as JSON")["over"]["kept"],
        serde_json::json!({"session_runs": {"processes": [{"pid": 4242, "started": 100}]}})
    );
    round_trips(Phase::Over {
        over: Over::Done,
        kept: Some(Kept::SessionRuns {
            processes: vec![],
            unread: Some("the host's processes cannot be listed".into()),
        }),
    });
    // A wait written before reads with no process and nothing unread, and
    // writes none.
    let before = serde_json::to_value(held()).expect("held state as JSON");
    let room = &before["stages"][0]["room"];
    assert!(room.get("live").is_none() && room.get("live_unread").is_none());
    let read: HeldState = serde_json::from_value(before).expect("read from before");
    let room = read.stages[0].room.as_ref().expect("a room");
    assert_eq!((&room.live, &room.live_unread), (&vec![], &None));
}

#[test]
fn a_held_state_with_a_session_and_a_label() {
    // A held state whose kind declares a label, whose launch's settings
    // name a session, and whose live start holds its session and label,
    // reads back unchanged; one written before reads with no label and no
    // session, and writes none back (107, 180).
    let state = held();
    round_trips(state.clone());
    let json = serde_json::to_value(&state).expect("held state as JSON");
    assert_eq!(json["place"]["kind"]["label"], "operation");
    assert_eq!(json["place"]["every_stage"]["session"], "office");
    let start = &json["stages"][0]["start"];
    assert_eq!(
        start["session"],
        serde_json::json!({"name": "office", "from": "launch"})
    );
    assert_eq!(
        start["label"],
        serde_json::json!({"name": "operation", "from": "kind"})
    );
    let mut before = json;
    let without_sessions = |value: &mut serde_json::Value| {
        value.as_object_mut().unwrap().remove("session");
    };
    before["place"]["kind"]
        .as_object_mut()
        .unwrap()
        .remove("label");
    without_sessions(&mut before["place"]["every_stage"]);
    without_sessions(&mut before["place"]["settings"]["code"]);
    without_sessions(&mut before["place"]["kind"]["stages"][0]["agents"][0]);
    without_sessions(&mut before["place"]["kind"]["agents"]["stages"]["code"]);
    without_sessions(&mut before["place"]["repository_settings"]["stages"]["code"]);
    let start = &mut before["stages"][0]["start"];
    without_sessions(&mut start["settings"]);
    without_sessions(start);
    start.as_object_mut().unwrap().remove("label");
    let read: HeldState = serde_json::from_value(before.clone()).expect("read from before");
    assert_eq!(read.place.kind.label, None);
    assert_eq!(read.place.every_stage.session, None);
    assert_eq!(read.place.settings["code"].session, None);
    let start = read.stages[0].start.as_ref().expect("a start");
    assert_eq!((&start.session, &start.label), (&None, &None));
    assert_eq!(start.settings.session, None);
    assert_eq!(
        serde_json::to_value(&read).expect("written again"),
        before,
        "no session and no label written where none is held"
    );
}

/// `value` with every `programs` of a settings' table, a start's `own`
/// and a setting's `file` taken out, at any depth, as a held state
/// written before own settings holds none.
fn without_programs(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            map.remove("programs");
            map.remove("own");
            map.remove("file");
            map.values_mut().for_each(without_programs);
        }
        serde_json::Value::Array(values) => values.iter_mut().for_each(without_programs),
        _ => {}
    }
}

#[test]
fn a_held_state_with_own_settings() {
    // A held state whose launch gives own settings for two programs, one
    // by a named file whose `env` its profile left out, and whose start
    // took a key from the host's table for its stream, reads back
    // unchanged, holding `env` by name alone; one written before reads
    // with none, and writes none back (195).
    let state = held();
    round_trips(state.clone());
    let json = serde_json::to_value(&state).expect("held state as JSON");
    let agent = &json["place"]["settings"]["code"]["programs"]["agent"];
    assert_eq!(
        agent["file"]["left_out"],
        serde_json::json!(["env", "permissions.default"])
    );
    assert_eq!(
        agent["file"]["keys"].get("env"),
        None,
        "env held by name alone"
    );
    assert_eq!(
        agent["file"]["keys"]["permissions"],
        serde_json::json!({"allow": ["Read", "Bash(git log:*)"]})
    );
    assert_eq!(agent["settings"]["line"]["padding"], 1);
    assert_eq!(
        json["place"]["settings"]["code"]["programs"]["other"],
        serde_json::json!({"settings": {"theme": "dark"}}),
        "a program with no file writes none"
    );
    assert_eq!(
        json["stages"][0]["start"]["from"]["programs.agent.settings.line"]["layer"],
        "host-stream"
    );
    assert_eq!(
        json["stages"][0]["start"]["from"]["programs.agent.settings.line"].get("file"),
        None,
        "a key given inline names no file"
    );
    assert_eq!(
        json["stages"][0]["start"]["own"]["files"][0]["left_out"],
        serde_json::json!(["env"])
    );
    let mut before = json;
    without_programs(&mut before);
    let read: HeldState = serde_json::from_value(before.clone()).expect("read from before");
    assert!(read.place.settings["code"].programs.is_empty());
    assert!(read.place.repository_agents.programs.is_empty());
    let start = read.stages[0].start.as_ref().expect("a start");
    assert_eq!(start.own, None);
    assert_eq!(start.from["programs.agent.settings.tips"].file, None);
    assert_eq!(
        serde_json::to_value(&read).expect("written again"),
        before,
        "no programs written where none are held"
    );
}

#[test]
fn a_held_state_with_a_repositorys_settings_for_every_kind() {
    // A kinds read and a held state whose repository gave its agents'
    // attribution for every kind read back unchanged; each written before
    // reads with none (21, 32, 195, C.2).
    let read = KindsRead::Read {
        kinds: vec![kind()],
        settings: BTreeMap::from([("change".into(), kind_settings())]),
        agents: every_kind(),
    };
    round_trips(read.clone());
    let state = held();
    round_trips(state.clone());
    let attribution = serde_json::json!({"commit": "", "pr": "", "sessionUrl": false});
    assert_eq!(
        serde_json::to_value(&read).unwrap()["read"]["agents"]["programs"]["agent"]["settings"]
            ["attribution"],
        attribution
    );
    let mut before = serde_json::to_value(&state).unwrap();
    assert_eq!(
        before["place"]["repository_agents"]["programs"]["agent"]["settings"]["attribution"],
        attribution
    );
    before["place"]
        .as_object_mut()
        .unwrap()
        .remove("repository_agents");
    let state: HeldState = serde_json::from_value(before).expect("held state from before");
    assert_eq!(state.place.repository_agents, Settings::default());
    let mut before = serde_json::to_value(&read).unwrap();
    before["read"].as_object_mut().unwrap().remove("agents");
    let KindsRead::Read { agents, .. } =
        serde_json::from_value(before).expect("kinds read from before")
    else {
        panic!("a kinds read read back as another");
    };
    assert_eq!(agents, Settings::default());
}

#[test]
fn where_an_agent_opens_and_its_session() {
    for at in [
        OpenIn::Beside {
            pane: "operator".into(),
        },
        OpenIn::Session {
            name: "work".into(),
        },
        OpenIn::Own,
    ] {
        round_trips(at);
    }
    assert_eq!(
        serde_json::to_value(OpenIn::Session {
            name: "work".into()
        })
        .unwrap(),
        serde_json::json!({"in": "session", "name": "work"})
    );
    for (from, spelled) in [
        (SessionFrom::Launch, "launch"),
        (SessionFrom::StreamStage, "stream-stage"),
        (SessionFrom::StreamKind, "stream-kind"),
        (SessionFrom::Stream, "stream"),
        (SessionFrom::StreamName, "stream-name"),
        (SessionFrom::Own, "own"),
    ] {
        round_trips(from);
        assert_eq!(serde_json::to_value(from).unwrap(), spelled);
    }
    round_trips(SessionHeld {
        name: Some("work".into()),
        from: SessionFrom::StreamName,
    });
    let own = SessionHeld {
        name: None,
        from: SessionFrom::Own,
    };
    round_trips(own.clone());
    assert_eq!(
        serde_json::to_value(own).unwrap(),
        serde_json::json!({"from": "own"}),
        "no name for the terminal's own"
    );
}

#[test]
fn a_starts_session_from_its_streams_word_for_its_stage() {
    // A held state whose live start holds `work-review` from
    // `stream-stage`, and a stream naming `work-design` for research and
    // `work-review` for release's review, each read back unchanged; a
    // stream written before reads with no session for any kind or stage,
    // and writes none back (199).
    let mut state = held();
    state.stages[0].start.as_mut().expect("a start").session = Some(SessionHeld {
        name: Some("work-review".into()),
        from: SessionFrom::StreamStage,
    });
    round_trips(state.clone());
    assert_eq!(
        serde_json::to_value(&state).expect("held state as JSON")["stages"][0]["start"]["session"],
        serde_json::json!({"name": "work-review", "from": "stream-stage"})
    );
    let stream = Stream {
        kinds: BTreeMap::from([
            (
                "research".into(),
                KindSessions {
                    session: Some("work-design".into()),
                    ..KindSessions::default()
                },
            ),
            (
                "release".into(),
                KindSessions {
                    stages: BTreeMap::from([(
                        "review".into(),
                        StageSession {
                            session: Some("work-review".into()),
                            unknown: vec![],
                        },
                    )]),
                    ..KindSessions::default()
                },
            ),
        ]),
        ..Stream::default()
    };
    round_trips(stream.clone());
    assert_eq!(
        serde_json::to_value(&stream).unwrap(),
        serde_json::json!({
            "priority": 0,
            "kinds": {
                "release": {"stages": {"review": {"session": "work-review"}}},
                "research": {"session": "work-design"},
            },
        })
    );
    let before: Stream = serde_json::from_str(r#"{"agents":4,"priority":10,"session":"office"}"#)
        .expect("a stream written before");
    assert!(before.kinds.is_empty(), "no session for any kind or stage");
    let written = serde_json::to_string(&before).unwrap();
    assert!(!written.contains("kinds"), "{written} gives no kinds");

    // A table's other keys are kept, by the kind's table and the stage's
    // (199, C.7).
    round_trips(KindSessions {
        session: Some("work-plan".into()),
        stages: BTreeMap::from([(
            "review".into(),
            StageSession {
                session: None,
                unknown: vec!["pool".into()],
            },
        )]),
        unknown: vec!["label".into()],
    });
    round_trips(KindSessions::default());
    assert_eq!(
        serde_json::to_value(KindSessions::default()).unwrap(),
        serde_json::json!({})
    );

    // Status gives each session a stream names, with its kinds and
    // stages and each one's layer, and each fault of its tables (56, 199,
    // C.7).
    let named = NamedSession {
        name: "work-design".into(),
        kinds: vec![NamedFor {
            name: "research".into(),
            layer: ConfigLayer::Managed,
        }],
        stages: vec![NamedFor {
            name: "release.review".into(),
            layer: ConfigLayer::Local,
        }],
    };
    round_trips(named.clone());
    assert_eq!(
        serde_json::to_value(&named).unwrap(),
        serde_json::json!({
            "name": "work-design",
            "kinds": [{"name": "research", "layer": "managed"}],
            "stages": [{"name": "release.review", "layer": "local"}],
        })
    );
    round_trips(NamedSession {
        name: "work-review".into(),
        kinds: vec![],
        stages: vec![],
    });
    round_trips(TableFault {
        table: "streams.work.kinds.research".into(),
        refused: "streams.work.kinds.research holds label".into(),
    });
}

#[test]
fn a_label_and_its_share() {
    for (from, spelled) in [
        (LabelFrom::HostRepositoryKind, "host-repository-kind"),
        (LabelFrom::HostKind, "host-kind"),
        (LabelFrom::Kind, "kind"),
    ] {
        round_trips(LabelHeld {
            name: "inception".into(),
            from,
        });
        assert_eq!(serde_json::to_value(from).unwrap(), spelled);
    }
    round_trips(Share::default());
    let read: Share = serde_json::from_str(r#"{"reserve":2}"#).unwrap();
    assert_eq!(
        read,
        Share {
            reserve: 2,
            keep_free: 0,
            unknown: vec![],
        },
        "zero kept free and nothing unknown where its table gives none"
    );
    let before: Stream = serde_json::from_str(r#"{"agents":5,"priority":10}"#).unwrap();
    assert_eq!((before.session, before.labels.len()), (None, 0));
    let kind: Kind = serde_json::from_value({
        let mut written = serde_json::to_value(kind()).unwrap();
        written.as_object_mut().unwrap().remove("label");
        written
    })
    .unwrap();
    assert_eq!(kind.label, None, "a kind read before has no label");
}

#[test]
fn the_hosts_configuration_and_its_layers() {
    let configuration = HostConfiguration {
        local: LocalLayer::Taken,
        values: vec![
            ConfigValue {
                key: "streams.work.agents".into(),
                value: "3".into(),
                layer: ConfigLayer::Local,
            },
            ConfigValue {
                key: "pools.work.accounts[0].env.AGENT_HOME".into(),
                value: SET.into(),
                layer: ConfigLayer::Managed,
            },
        ],
        passed_over: vec![PinnedKey {
            key: "streams.work.priority".into(),
            pin: "streams.work".into(),
        }],
    };
    round_trips(configuration.clone());
    let json = serde_json::to_value(&configuration).unwrap();
    assert_eq!(json["local"], serde_json::json!({"state": "taken"}));
    assert_eq!(json["values"][0]["layer"], "local");
    for local in [
        LocalLayer::NoFile,
        LocalLayer::Taken,
        LocalLayer::NotTaken {
            keys: vec!["pools.work".into(), "host.agents".into()],
            fault: None,
        },
        LocalLayer::NotTaken {
            keys: vec![],
            fault: Some("expected `=` at line 2".into()),
        },
    ] {
        round_trips(local);
    }
    assert_eq!(
        serde_json::to_value(LocalLayer::NotTaken {
            keys: vec!["pools.work".into()],
            fault: None,
        })
        .unwrap(),
        serde_json::json!({"state": "not-taken", "keys": ["pools.work"]})
    );
    round_trips(HostConfiguration::default());
    assert_eq!(HostConfiguration::default().local, LocalLayer::NoFile);
}

/// An agent profile that keeps every default of the agent seam.
struct Bare;

impl Agent for Bare {
    fn command(
        &self,
        _folder: &str,
        _settings: &Settings,
        _mode: Option<&str>,
        _own: &BTreeMap<String, serde_json::Value>,
        _resume: Option<&str>,
        _env: &BTreeMap<String, String>,
        _place: &PlaceRef,
        _identity: &Identity,
        _secret: &Secret,
        _prompt: &str,
    ) -> AgentCommand {
        AgentCommand::default()
    }

    fn started(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::Unknown
    }

    fn received(&self, _folder: &str, _agent: &Identity, _text: &str) -> Received {
        Received::Unknown
    }

    fn asking(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::Unknown
    }

    fn own_work(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::Unknown
    }

    fn ended(&self, _folder: &str, _agent: &Identity) -> Reading {
        Reading::Unknown
    }

    fn last_words(&self, _folder: &str, _agent: &Identity) -> Option<String> {
        None
    }

    fn stop_short(&self, _agent: &Identity) -> String {
        String::new()
    }

    fn send_back(&self) -> Offered {
        Offered::NotOffered
    }

    fn guard(&self, _folder: &str, _agent: &Identity) -> Guarded {
        Guarded::NotOffered
    }

    fn on_quiet(&self) -> Offered {
        Offered::NotOffered
    }
}

#[test]
fn a_profile_that_maps_no_mode() {
    // A profile that keeps the seam's defaults refuses every entry, one
    // of the four or its program's own, naming it, and reads no mode back
    // (169, 170).
    let words = Bare
        .mode(&ModeEntry::Flume(Mode::Edits))
        .expect_err("a profile that maps no mode mapped edits");
    assert!(words.contains("`edits`"), "{words}");
    let words = Bare
        .mode(&ModeEntry::Own("careful".into()))
        .expect_err("a profile that maps no mode mapped an own value");
    assert!(words.contains("`careful`"), "{words}");
    assert_eq!(Bare.mode_read_back(), Offered::NotOffered);
}

#[test]
fn a_profile_that_passes_no_own_settings() {
    // A profile that keeps the seam's defaults reserves no key, refuses
    // its program's keys, saying it passes none, and takes an empty entry
    // (170, 195, 196).
    assert_eq!(Bare.reserved(), vec![]);
    let given = BTreeMap::from([("line".to_string(), serde_json::json!("a"))]);
    let words = Bare
        .own(&given)
        .expect_err("a profile that passes no own settings took one");
    assert!(
        words.contains("passes no settings of its program's own"),
        "{words}"
    );
    assert_eq!(Bare.own(&BTreeMap::new()), Ok(()));
}

#[test]
fn a_profile_that_carries_no_conversation() {
    // A profile that keeps the seam's default offers no carry, for any
    // start and account (179).
    let env = BTreeMap::from([("AGENT_HOME".to_string(), "/scratch/work-2".to_string())]);
    assert_eq!(Bare.carry("/folder", &identity(), &env), Carry::NotOffered);
    assert_eq!(
        Bare.carry("/folder", &identity(), &BTreeMap::new()),
        Carry::NotOffered
    );
}

/// The seam-contracts spec's scenario "A profile that reads no running
/// session" (207).
#[test]
fn a_profile_that_reads_no_running_session() {
    let env = BTreeMap::from([("AGENT_HOME".to_string(), "/scratch/work-2".to_string())]);
    for envs in [vec![], vec![BTreeMap::new(), env]] {
        assert_eq!(
            Bare.live("/folder", &identity(), &envs),
            Live::Unknown {
                what: READS_NO_SESSION.to_string()
            }
        );
    }
    assert_eq!(
        READS_NO_SESSION,
        "its agent profile reads no running session"
    );
    for live in [
        Live::Runs(vec![LiveProcess {
            pid: 4242,
            started: 100,
        }]),
        Live::None,
        Live::Unknown {
            what: "the sessions under CLAUDE_CONFIG_DIR cannot be read".into(),
        },
    ] {
        round_trips(live);
    }
}

#[test]
fn settings_under_others_take_the_mode_whole() {
    // A table over a table is taken whole, never merged; a mode under
    // settings that give none stands (32, 169).
    let under = Settings {
        mode: Some(programs()),
        ..Settings::default()
    };
    let over = Settings {
        mode: Some(ModeSetting::Programs(BTreeMap::from([(
            "third".into(),
            "full".into(),
        )]))),
        ..Settings::default()
    };
    assert_eq!(under.under(&over).mode, over.mode);
    assert_eq!(under.under(&Settings::default()).mode, Some(programs()));
    assert_eq!(Settings::default().under(&under).mode, Some(programs()));
}

#[test]
fn settings_under_others_take_own_settings_whole() {
    // Own settings over own settings are taken whole, the more specific
    // table's where it gives any; their fold key by key is the core's
    // (195).
    let under = Settings {
        programs: own_programs(),
        ..Settings::default()
    };
    let over = every_kind();
    assert_eq!(under.under(&over).programs, over.programs);
    assert_eq!(under.under(&Settings::default()).programs, own_programs());
    assert_eq!(Settings::default().under(&under).programs, own_programs());
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
fn a_role_run_as_its_source_declares_it() {
    // A run's entry with every key, and with none it may leave out; its
    // stream is its address's, so it holds none (132, 193, C.7).
    let run = RoleRun {
        source: "acme".into(),
        repository: "/r".into(),
        kind: "turn".into(),
        priority: 5,
        base: Some("next".into()),
        brief: Some("read the records".into()),
        name: Some("east".into()),
        intent: Some("east-planner".into()),
        agents: RoleAgents {
            every_stage: Settings {
                model: Some("opus".into()),
                ..Settings::default()
            },
            stages: BTreeMap::from([(
                "turn".into(),
                Settings {
                    instructions: Some("be brief".into()),
                    ..Settings::default()
                },
            )]),
        },
        answers: Some("The work's scope.".into()),
        answerer: true,
    };
    round_trips(run.clone());
    let bare = RoleRun {
        base: None,
        brief: None,
        name: None,
        intent: None,
        agents: RoleAgents::default(),
        answers: None,
        answerer: false,
        ..run
    };
    round_trips(bare.clone());
    assert_eq!(
        serde_json::to_value(&bare).unwrap(),
        serde_json::json!({"source": "acme", "repository": "/r", "kind": "turn", "priority": 5}),
        "only the keys the entry gives"
    );
    let before: RoleRun = serde_json::from_value(
        serde_json::json!({"repository": "/r", "kind": "planner", "priority": 5}),
    )
    .unwrap();
    assert_eq!(before.source, "", "a run status gave before sources");
}

#[test]
fn what_a_source_declares_for_an_address() {
    // A run, a standing agent and a carried address, an entry that
    // declares nothing, and a roles file not read, as the host gives them
    // (132, 191, 193, C.7).
    let standing = StandingAgent {
        source: "acme".into(),
        agent: "east-design".into(),
        session: None,
        answers: Some("A decision the design has not made.".into()),
        answerer: false,
    };
    let roles = HostRoles {
        declared: BTreeMap::from([
            (
                "east/design".to_string(),
                RoleEntry::Standing(standing.clone()),
            ),
            (
                "north/planner".to_string(),
                RoleEntry::Carried(CarriedRole {
                    source: "acme".into(),
                    answers: Some("The work's scope.".into()),
                }),
            ),
            (
                "east/operator".to_string(),
                RoleEntry::Run(Box::new(RoleRun {
                    source: "beta".into(),
                    repository: "/r".into(),
                    kind: "turn".into(),
                    priority: 0,
                    base: None,
                    brief: None,
                    name: None,
                    intent: None,
                    agents: RoleAgents::default(),
                    answers: None,
                    answerer: true,
                })),
            ),
        ]),
        refused: vec![RoleRefused {
            address: "east/ops".into(),
            source: "acme".into(),
            reason: "its kind beta/turn is not acme's own".into(),
        }],
        unread: BTreeMap::from([(
            "gamma".to_string(),
            RolesUnread {
                file: "/state/gamma/roles.toml".into(),
                reason: "is not TOML".into(),
            },
        )]),
    };
    round_trips(roles.clone());
    round_trips(HostRoles::default());
    assert_eq!(
        serde_json::to_value(HostRoles::default()).unwrap(),
        serde_json::json!({})
    );
    assert_eq!(
        serde_json::to_value(RoleEntry::Standing(standing.clone())).unwrap(),
        serde_json::json!({"standing": {
            "source": "acme",
            "agent": "east-design",
            "answers": "A decision the design has not made."
        }})
    );
    assert_eq!(roles.run("east/design"), None, "a standing address's run");
    assert_eq!(
        roles.standing(),
        BTreeMap::from([("east/design".to_string(), standing)])
    );
    assert_eq!(
        roles.declared("east/ops"),
        Err("its kind beta/turn is not acme's own".into())
    );
    assert!(roles
        .declared("west/ops")
        .is_err_and(|why| why.contains("no source") && why.contains("west/ops")));
    let door = StreamAnswerer::Stands(DeclaredBy {
        address: "east/operator".into(),
        source: "beta".into(),
    });
    assert_eq!(roles.answerer("east"), Some(door.clone()));
    assert_eq!(
        roles.answerer("north"),
        None,
        "a carried address stands behind none"
    );
    round_trips(door);
    round_trips(StreamAnswerer::Contested {
        roles: vec![
            DeclaredBy {
                address: "east/operator".into(),
                source: "acme".into(),
            },
            DeclaredBy {
                address: "east/ops".into(),
                source: "beta".into(),
            },
        ],
    });
    let answering = roles.answering();
    round_trips(answering[0].clone());
    assert_eq!(
        answering
            .iter()
            .map(|role| (role.address.as_str(), role.stream.as_str()))
            .collect::<Vec<_>>(),
        [("east/design", "east"), ("north/planner", "north")],
        "each address declared with its words, and its stream"
    );
}

#[test]
fn a_standing_address_round_trips() {
    // The seam-contracts spec's "A standing address round-trips", for
    // the seams' types: a standing entry with and without its session,
    // its words and its standing behind its stream, a part's agents read,
    // two of one name among them, and not read, and a session from the
    // entry's own `session` (56, 132, 193, C.5, C.7; core.14).
    let entry = StandingAgent {
        source: "acme".into(),
        agent: "madswan-planner".into(),
        session: Some("madswan".into()),
        answers: Some("The work's scope.".into()),
        answerer: true,
    };
    round_trips(entry.clone());
    let bare = StandingAgent {
        session: None,
        answers: None,
        answerer: false,
        ..entry
    };
    round_trips(bare.clone());
    assert_eq!(
        serde_json::to_value(&bare).unwrap(),
        serde_json::json!({"source": "acme", "agent": "madswan-planner"}),
        "no session, words or standing behind where the entry gives none"
    );
    let named = |name: &str, pane: &str, state| NamedAgent {
        name: name.into(),
        at: format!("terminal:{pane}@/run/madswan.sock"),
        state,
    };
    let read = AgentsIn::Read {
        agents: vec![
            named("madswan-planner", "w1-1", AgentState::Waiting),
            named("madswan-ops", "w1-2", AgentState::Working),
            named("madswan-ops", "w1-3", AgentState::Asking),
        ],
    };
    round_trips(read.clone());
    let json = serde_json::to_value(&read).unwrap();
    assert_eq!(
        json["read"]["agents"][0],
        serde_json::json!({
            "name": "madswan-planner",
            "at": "terminal:w1-1@/run/madswan.sock",
            "state": "waiting"
        })
    );
    round_trips(AgentsIn::Read { agents: vec![] });
    let unread = AgentsIn::Unread {
        reason: NO_NAMED_AGENTS.into(),
    };
    round_trips(unread.clone());
    assert_eq!(
        serde_json::to_value(&unread).unwrap(),
        serde_json::json!({"unread": {"reason": "the terminal holds no agent a name finds"}})
    );
    round_trips(SessionFrom::Role);
    assert_eq!(serde_json::to_value(SessionFrom::Role).unwrap(), "role");
    let session = SessionHeld {
        name: Some("madswan".into()),
        from: SessionFrom::Role,
    };
    round_trips(session.clone());
    assert_eq!(
        serde_json::to_value(session).unwrap(),
        serde_json::json!({"name": "madswan", "from": "role"})
    );
}

#[test]
fn streams_and_pools_as_the_configuration_names_them() {
    round_trips(Stream {
        agents: Some(5),
        priority: 10,
        pool: Some("work".into()),
        session: Some("office".into()),
        labels: BTreeMap::from([
            (
                "operation".into(),
                Share {
                    reserve: 2,
                    keep_free: 1,
                    unknown: vec![],
                },
            ),
            (
                "construction".into(),
                Share {
                    reserve: 0,
                    keep_free: 0,
                    unknown: vec!["pool".into()],
                },
            ),
        ]),
        ..Stream::default()
    });
    round_trips(Stream::default());
    let pool = Pool {
        strategy: Strategy::FillFirst,
        threshold: None,
        accounts: vec![
            Account {
                name: "work-1".into(),
                program: "agent".into(),
                env: BTreeMap::from([("AGENT_HOME".into(), "~/.agent-work-1".into())]),
                agents: Some(4),
                usage: None,
            },
            Account {
                name: "work-2".into(),
                program: "agent".into(),
                env: BTreeMap::new(),
                agents: None,
                usage: None,
            },
        ],
    };
    round_trips(pool.clone());
    round_trips(Pool {
        strategy: Strategy::LeastBusy,
        ..pool
    });
    for (strategy, spelled) in [
        (Strategy::FillFirst, "fill-first"),
        (Strategy::LeastBusy, "least-busy"),
    ] {
        assert_eq!(strategy.name(), spelled);
        assert_eq!(
            serde_json::to_string(&strategy).unwrap(),
            format!("{spelled:?}")
        );
    }
    for from in [
        StreamFrom::Launch,
        StreamFrom::Intent,
        StreamFrom::HostRepositoryKind,
        StreamFrom::HostRepository,
        StreamFrom::HostKind,
        StreamFrom::Default,
    ] {
        round_trips(StreamHeld {
            name: DEFAULT_STREAM.into(),
            from,
        });
    }
    assert_eq!(
        serde_json::to_string(&StreamFrom::HostRepositoryKind).unwrap(),
        "\"host-repository-kind\""
    );
    for from in [PoolFrom::Stream, PoolFrom::Host] {
        round_trips(PoolNamed {
            name: "work".into(),
            from,
        });
    }
}

/// The seam-contracts spec's scenario "A pool and a reading" (184, 185).
#[test]
fn a_pool_and_a_reading() {
    let pool = Pool {
        strategy: Strategy::MostLeft,
        threshold: Some(85),
        accounts: vec![Account {
            name: "work-1".into(),
            program: "agent".into(),
            env: BTreeMap::new(),
            agents: None,
            usage: Some("~/.local/state/usage/work-1.json".into()),
        }],
    };
    round_trips(pool.clone());
    let json = serde_json::to_value(&pool).unwrap();
    assert_eq!(json["strategy"], serde_json::json!("most-left"));
    assert_eq!(json["threshold"], serde_json::json!(85));
    assert_eq!(
        json["accounts"][0]["usage"],
        serde_json::json!("~/.local/state/usage/work-1.json")
    );
    assert_eq!(Strategy::MostLeft.name(), "most-left");
    let usage = Usage {
        read_at: 1_000,
        windows: vec![
            UsageWindow {
                name: "5-hour".into(),
                used: 4_200,
                resets_at: Some(12_000),
            },
            UsageWindow {
                name: "7-day".into(),
                used: 7_150,
                resets_at: None,
            },
        ],
    };
    round_trips(usage.clone());
    let json = serde_json::to_value(&usage).unwrap();
    assert!(json["windows"][1].get("resets_at").is_none());
    let unstated = serde_json::to_value(Pool {
        threshold: None,
        accounts: vec![Account {
            usage: None,
            ..pool.accounts[0].clone()
        }],
        ..pool
    })
    .unwrap();
    assert!(unstated.get("threshold").is_none());
    assert!(unstated["accounts"][0].get("usage").is_none());
}

/// The seam-contracts spec's scenario "A move past a threshold" (147,
/// 185).
#[test]
fn a_move_past_a_threshold() {
    let moved = AccountMove {
        on: MoveOn::Intent {
            program: Some("agent".into()),
        },
        from: "work-1".into(),
        to: "work-2".into(),
        why: MoveWhy::Threshold,
        at: 50_400,
        carried: None,
        window: Some("5-hour".into()),
        until: Some(61_200),
    };
    round_trips(moved.clone());
    let json = serde_json::to_value(&moved).unwrap();
    assert_eq!(json["why"], serde_json::json!("threshold"));
    assert_eq!(json["window"], serde_json::json!("5-hour"));
    assert_eq!(json["until"], serde_json::json!(61_200));
    assert_eq!(MoveWhy::Threshold.name(), "threshold");
    let before: AccountMove = serde_json::from_value(serde_json::json!({
        "on": "intent",
        "program": "agent",
        "from": "work-1",
        "to": "work-2",
        "why": "spent",
        "at": 47_400,
    }))
    .expect("read");
    assert_eq!(before.window, None);
    assert_eq!(before.until, None);
    let written = serde_json::to_value(&before).unwrap();
    assert!(written.get("window").is_none() && written.get("until").is_none());
}

#[test]
fn an_address_is_a_stream_and_a_role() {
    assert!(is_address("madswan/planner"));
    assert!(is_address("slc-2/ops.1"));
    assert_eq!(role_of("madswan/planner"), "planner");
    assert_eq!(stream_of("madswan/planner"), "madswan");
    assert_eq!(stream_of("slc-2/ops.1"), "slc-2");
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
fn the_hosts_own_documents_and_their_keys() {
    for key in [
        HostKey::Account {
            pool: "work".into(),
            account: "work-1".into(),
        },
        HostKey::Intent {
            digest: "d1".into(),
        },
    ] {
        round_trips(key);
    }
    round_trips(HostVersioned {
        version: 3,
        document: HostDocument::Account(AccountSpent {
            until: 54_000,
            words: Some("You've hit your session limit".into()),
            recorded_at: 47_400,
            unread: false,
        }),
    });
    let unread = AccountSpent {
        until: 51_000,
        words: Some("You've hit your weekly limit \u{b7} resets soon (UTC)".into()),
        recorded_at: 47_400,
        unread: true,
    };
    round_trips(unread.clone());
    // A document written before reads as a reset read.
    let before: AccountSpent = serde_json::from_value(serde_json::json!({
        "until": 54_000,
        "words": "You've hit your session limit",
        "recorded_at": 47_400,
    }))
    .expect("a spent document written before");
    assert!(!before.unread);
    assert!(serde_json::to_value(&before)
        .expect("json")
        .get("unread")
        .is_none());
    assert_eq!(serde_json::to_value(&unread).expect("json")["unread"], true);
    round_trips(HostVersioned {
        version: 4,
        document: HostDocument::Intent(two_programs()),
    });
    assert_eq!(
        HostKey::Account {
            pool: "work".into(),
            account: "work-1".into()
        }
        .to_string(),
        "account work/work-1"
    );
}

/// An affinity holding an account for `agent` and one for `other`, and
/// an intent's move for `agent` (147).
fn two_programs() -> Affinity {
    Affinity {
        stream: "work".into(),
        accounts: BTreeMap::from([
            (
                "agent".into(),
                AffinityAccount {
                    pool: "work".into(),
                    account: "agent-2".into(),
                },
            ),
            (
                "other".into(),
                AffinityAccount {
                    pool: "work".into(),
                    account: "other-1".into(),
                },
            ),
        ]),
        moves: vec![AccountMove {
            on: MoveOn::Intent {
                program: Some("agent".into()),
            },
            from: "agent-1".into(),
            to: "agent-2".into(),
            why: MoveWhy::Spent,
            at: 47_400,
            carried: None,
            window: None,
            until: None,
        }],
        gone: Some(50_000),
        pool: None,
        account: None,
    }
}

#[test]
fn an_affinity_of_two_programs() {
    let affinity = two_programs();
    round_trips(affinity.clone());
    let json = serde_json::to_value(&affinity).unwrap();
    assert_eq!(json["moves"][0]["program"], serde_json::json!("agent"));
    assert!(json.get("pool").is_none() && json.get("account").is_none());
    assert!(affinity.left("agent", "agent-1"));
    assert!(!affinity.left("other", "agent-1"));
    assert!(!affinity.left("agent", "agent-2"));
}

#[test]
fn an_affinity_written_before() {
    let json = serde_json::json!({
        "stream": "work",
        "pool": "work",
        "account": "work-2",
        "moves": [{
            "on": "intent",
            "from": "work-1",
            "to": "work-2",
            "why": "spent",
            "at": 47_400,
        }],
        "gone": null,
    });
    let before: Affinity = serde_json::from_value(json).expect("read");
    assert_eq!(before.pool.as_deref(), Some("work"));
    assert_eq!(before.account.as_deref(), Some("work-2"));
    assert!(before.accounts.is_empty());
    assert_eq!(before.moves[0].on, MoveOn::Intent { program: None });
    assert!(!before.left("agent", "work-1"));
    round_trips(before.clone());
    let taken = Affinity {
        pool: None,
        account: None,
        ..before
    };
    let written = serde_json::to_value(&taken).unwrap();
    assert!(written.get("pool").is_none() && written.get("account").is_none());
    assert!(written["moves"][0].get("program").is_none());
    assert_eq!(written["accounts"], serde_json::json!({}));
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
        agents: every_kind(),
    });
    round_trips(HostSettings {
        repository: kind_settings(),
        kind: KindSettings::default(),
        ..HostSettings::default()
    });
    round_trips(HostSettings {
        every_kind: every_kind(),
        stream: Settings {
            model: Some("opus".into()),
            ..every_kind()
        },
        every_agent: Settings {
            model: Some("sonnet".into()),
            ..Settings::default()
        },
        ..HostSettings::default()
    });
    let written_before: HostSettings = serde_json::from_value(serde_json::json!({
        "repository": KindSettings::default(),
        "kind": KindSettings::default(),
    }))
    .unwrap();
    assert_eq!(written_before, HostSettings::default());
    for layer in [
        Layer::Launch,
        Layer::HostRepository,
        Layer::HostKind,
        Layer::HostStream,
        Layer::Host,
        Layer::Repository,
        Layer::Kind,
        Layer::Default,
        Layer::Account,
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
    for declared in [
        Declared::Launch,
        Declared::Repository,
        Declared::Flume,
        Declared::Host,
    ] {
        round_trips(declared);
    }
    assert_eq!(serde_json::to_value(Declared::Host).unwrap(), "host");
}

#[test]
fn a_held_state_with_where_its_kind_came_from() {
    // A held state whose kind came from the host's source acme, with its
    // file, its kind's directory, its hash and the host's lock, reads back
    // as written; one written before a launch kept where its kind came
    // from reads with none, and writes none back (19, 21; core.14).
    let state = held();
    round_trips(state.clone());
    let json = serde_json::to_value(&state).expect("held state as JSON");
    let from = &json["place"]["kind_from"];
    assert_eq!(from["identity"]["spelled"], "acme/review@2");
    assert_eq!(from["source"], "acme");
    assert_eq!(from["source_file"], "/config/flume/sources/acme.toml");
    assert_eq!(from["kind_dir"], "/data/flume/kinds/acme/review@2");
    assert_eq!(from["hash"], hash());
    assert_eq!(from["lock_file"], "/config/flume/flume.lock");
    assert!(
        from.get("declaration_file").is_none() && from.get("revision").is_none(),
        "a field that does not apply is written: {from}"
    );
    let mut before = json;
    before["place"].as_object_mut().unwrap().remove("kind_from");
    let read: HeldState = serde_json::from_value(before.clone()).expect("read from before");
    assert_eq!(read.place.kind_from, None);
    assert_eq!(
        serde_json::to_value(&read).expect("written again"),
        before,
        "no kind_from written where none is held"
    );
}

#[test]
fn a_kinds_identity_name_package_source_and_lock() {
    // Each type the kinds seam passes for a name, a package, a source's
    // listing and a lock (154 to 157, 160, 192).
    round_trips(kind_identity());
    assert_eq!(kind_identity().to_string(), "acme/review@2");
    round_trips(KindNamed {
        source: Some("acme".into()),
        name: "review".into(),
        version: Some(2),
        spelled: "acme/review@2".into(),
    });
    let bare = KindNamed {
        source: None,
        name: "review".into(),
        version: None,
        spelled: "review".into(),
    };
    round_trips(bare.clone());
    assert_eq!(
        serde_json::to_value(&bare).unwrap(),
        serde_json::json!({"name": "review", "spelled": "review"}),
        "a part not given is not written"
    );
    let package = Packaged {
        identity: kind_identity(),
        kind: kind(),
        hash: hash(),
        dir: Some("/kinds/acme/review@2".into()),
    };
    round_trips(package.clone());
    round_trips(Packaged {
        dir: None,
        ..package.clone()
    });
    round_trips(PackageRead::Read {
        package: Box::new(package),
    });
    round_trips(PackageRead::CouldNotRead {
        reason: "kinds.review.version: makes the package acme/review@3".into(),
    });
    round_trips(SourceRead::Read {
        kinds: vec![kind_identity()],
    });
    round_trips(SourceRead::CouldNotRead {
        reason: "/data/flume/kinds/acme does not exist".into(),
    });
    let entry = LockEntry {
        kind: kind_identity(),
        hash: hash(),
    };
    round_trips(entry.clone());
    round_trips(LockRead::None);
    round_trips(LockRead::Read {
        entries: vec![entry],
    });
    round_trips(LockRead::CouldNotRead {
        reason: "kinds[0].hash: is no content hash".into(),
    });
}

#[test]
fn where_a_kind_came_from() {
    // A host's source's, the repository's at the base's revision and a
    // launch's, each with only the fields that apply (19, 21, 56).
    round_trips(kind_from());
    let repository = KindFrom {
        identity: KindIdentity {
            source: "repo".into(),
            name: "bump".into(),
            version: 2,
            spelled: "repo/bump@2".into(),
        },
        source: "repo".into(),
        source_file: None,
        kind_dir: None,
        declaration_file: Some(".config/flume.toml".into()),
        revision: Some(Revision::new("c1")),
        hash: None,
        lock_file: None,
    };
    round_trips(repository.clone());
    let launch = KindFrom {
        identity: KindIdentity {
            source: "launch".into(),
            name: "x".into(),
            version: 1,
            spelled: "launch/x@1".into(),
        },
        source: "launch".into(),
        declaration_file: Some("/tmp/kinds.toml".into()),
        revision: None,
        ..repository
    };
    round_trips(launch.clone());
    let json = serde_json::to_value(&launch).unwrap();
    assert_eq!(
        json.as_object().unwrap().keys().collect::<Vec<_>>(),
        ["declaration_file", "identity", "source"],
        "only the fields that apply"
    );
}

#[test]
fn a_source_the_host_declares() {
    // A source's file read, its kinds_dir, or its fault (159, 191, C.7).
    round_trips(HostSource {
        file: "/config/flume/sources/acme.toml".into(),
        read: HostSourceRead::Read {
            kinds_dir: "/data/flume/kinds/acme".into(),
            roles: Some("/state/crew/flume/roles.toml".into()),
        },
    });
    let bare = HostSourceRead::Read {
        kinds_dir: "/data/flume/kinds/acme".into(),
        roles: None,
    };
    round_trips(bare.clone());
    assert_eq!(
        serde_json::to_value(&bare).unwrap(),
        serde_json::json!({"read": {"kinds_dir": "/data/flume/kinds/acme"}}),
        "no roles where the source names none, as a source read before roles"
    );
    round_trips(HostSource {
        file: "/config/flume/sources/bad.toml".into(),
        read: HostSourceRead::CouldNotRead {
            reason: "it gives no kinds_dir".into(),
        },
    });
}

/// A kinds profile that keeps every default of the kinds seam.
struct BareKinds;

impl Kinds for BareKinds {
    fn read_kinds(&self, _repository: &str, _revision: &Revision) -> KindsRead {
        KindsRead::CouldNotRead {
            reason: "bare".into(),
        }
    }

    fn read_declaration(&self, _declaration: &str) -> KindsRead {
        KindsRead::CouldNotRead {
            reason: "bare".into(),
        }
    }

    fn flumes_own(&self) -> Vec<Packaged> {
        vec![]
    }

    fn read_binding(&self, _repository: &str) -> BindingRead {
        BindingRead::Read {
            binding: Binding::default(),
        }
    }
}

#[test]
fn a_kinds_profile_that_reads_no_name_source_package_or_lock() {
    // A profile that keeps the seam's defaults answers that it reads none
    // of them and writes no lock, naming what it was asked where it was
    // given a name (154, 160, 192).
    let words = BareKinds
        .read_named("acme/review@2")
        .expect_err("a bare profile read a name");
    assert!(words.contains("`acme/review@2`"), "{words}");
    assert!(matches!(
        BareKinds.read_source("acme", "/kinds/acme"),
        SourceRead::CouldNotRead { .. }
    ));
    match BareKinds.read_package("acme", "/kinds/acme", "review", 2) {
        PackageRead::CouldNotRead { reason } => assert!(reason.contains("review@2"), "{reason}"),
        read => panic!("a bare profile read a package: {read:?}"),
    }
    assert!(matches!(
        BareKinds.read_lock_at("/r", &Revision::new("c1")),
        LockRead::CouldNotRead { .. }
    ));
    assert!(matches!(
        BareKinds.read_lock_file("/config/flume/flume.lock"),
        LockRead::CouldNotRead { .. }
    ));
    assert!(BareKinds
        .write_lock("/config/flume/flume.lock", &[])
        .is_err());
    let words = BareKinds
        .identity_of("repo", "bump", 2)
        .expect_err("a bare profile spelled an identity");
    assert!(words.contains("bump@2"), "{words}");
    assert_eq!(BareKinds.declaration_path(), None);
    assert_eq!(BareKinds.lock_path(), None);
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
            base: None,
            words: "conflicts".into(),
        },
        SubmissionState::Conflict {
            files: vec!["src/lib.rs".into()],
            base: Some(Revision::new("c8")),
            words: "conflicts".into(),
        },
        SubmissionState::Withdrawn,
    ] {
        for request in [None, Some("56".to_string())] {
            round_trips(SubmissionRead::Found {
                submission: Submission {
                    id: "7".into(),
                    branch: "fix/one".into(),
                    target: "main".into(),
                    state: state.clone(),
                    request,
                },
            });
        }
    }
    round_trips(SubmissionRead::None);
    round_trips(SubmissionRead::CouldNotRead {
        reason: "down".into(),
    });
    round_trips(ReadAcross::from(SubmissionRead::None));
    round_trips(ReadAcross {
        read: SubmissionRead::None,
        passed_over: vec![PassedOver {
            branch: "bolt/x".into(),
            words: "bolt/x's policy cannot be read".into(),
        }],
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
        request: None,
    }
}

#[test]
fn submit_holds_no_wait() {
    round_trips(Submitted::Submitted {
        submission: a_submission(),
    });
    round_trips(Submitted::Requested {
        request: "56".into(),
    });
    round_trips(Submitted::Decided {
        submission: a_submission(),
        outcome: SubmissionOutcome::Conflict {
            files: vec!["src/lib.rs".into()],
            base: Revision::new("c8"),
            words: "conflict on main: src/lib.rs with #6".into(),
        },
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
    round_trips(Inbox::Outcomes {
        outcomes: outcomes.clone(),
        passed_over: vec![],
    });
    round_trips(Inbox::Outcomes {
        outcomes,
        passed_over: vec![PassedOver {
            branch: "bolt/x".into(),
            words: "bolt/x's records cannot be read".into(),
        }],
    });
    round_trips(Inbox::Outcomes {
        outcomes: vec![],
        passed_over: vec![],
    });
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
    round_trips(BaseRevision::Unlanded);
    for branch in [
        OriginBranch::At {
            revision: Revision::new("c2"),
        },
        OriginBranch::NoOrigin,
        OriginBranch::Lacks,
        OriginBranch::Refused {
            reason: "origin is out of reach".into(),
        },
    ] {
        round_trips(branch);
    }
    for from in [
        BaseFrom::MergeQueue,
        BaseFrom::Origin,
        BaseFrom::NoOrigin,
        BaseFrom::OriginLacks,
    ] {
        round_trips(from);
    }
}

#[test]
fn a_place_held_before_it_said_where_its_base_was_read() {
    // A place launched before flume said where it read the base reads
    // with none, and writes none back (56, C.1).
    let mut json = serde_json::to_value(held()).expect("held state as JSON");
    json["place"]
        .as_object_mut()
        .expect("the place")
        .remove("base_from");
    let before: HeldState = serde_json::from_value(json).expect("read back");
    assert_eq!(before.place.base_from, None);
    let written = serde_json::to_value(&before).expect("written");
    assert!(written["place"].get("base_from").is_none(), "{written}");
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
        files: BTreeMap::from([(
            "/state/fix--one/agent/own.json".into(),
            r#"{"line":"a"}"#.into(),
        )]),
    });
    let before: AgentCommand =
        serde_json::from_str(r#"{"program":"agent","args":[],"env":{},"prompt_args":[]}"#).unwrap();
    assert_eq!(
        before.kind, None,
        "a command written before it named a kind"
    );
    assert!(
        before.files.is_empty(),
        "a command written before it listed files"
    );
    round_trips(Reserved {
        key: "permissions.default".into(),
        owner: "flume's mode".into(),
    });
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
    round_trips(Spent::Yes {
        until: Some(1_800_000_000),
        words: "You've hit your session limit · resets 3pm (UTC)".into(),
    });
    round_trips(Spent::Yes {
        until: None,
        words: "You've hit your limit".into(),
    });
    round_trips(Spent::No);
    round_trips(Spent::Unknown);
    round_trips(unsettled());
    round_trips(Unsettled::default());
    assert_eq!(
        unsettled().words(),
        "without AGENT_HOME, with another FLUME_STAGE, under another program, \
         in another conversation than the one its start resumes, \
         in the mode `ask-each`, where its start asked `careful`"
    );
    round_trips(Carry::Yes {
        session: "4e3c05c8-51c4-4449-8577-4f7038e394bf".into(),
    });
    round_trips(Carry::No {
        why: "the chosen account's configuration home (`CLAUDE_CONFIG_DIR`) does not reach \
              its conversation"
            .into(),
    });
    round_trips(Carry::NotOffered);
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
        bases: vec!["workspace w1".into(), "workspace w4".into()],
    });
    round_trips(ShownAt::default());
    let before: ShownAt =
        serde_json::from_str(r#"{"place":null,"agent":null,"base":"workspace w1"}"#).unwrap();
    assert!(before.bases.is_empty(), "no workspaces where none written");
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
fn a_view_a_window_and_a_stream() {
    // The seam-contracts spec's view, window and stream's terminal: a
    // command needing a variable, attach's and show's answers, each view,
    // a stream's display from each source, a stream with its four keys
    // and a start with its terminal read back unchanged, and a stream and
    // a start written before reading with none (143, 187, 188, 190).
    let command = ViewCommand {
        program: "/bin/terminal".into(),
        arguments: vec!["session".into(), "attach".into(), "work".into()],
        environment: BTreeMap::from([("XDG_CONFIG_HOME".into(), "/scratch/sessions".into())]),
    };
    round_trips(command.clone());
    round_trips(ViewCommand {
        environment: BTreeMap::new(),
        ..command.clone()
    });
    let bare: ViewCommand =
        serde_json::from_str(r#"{"program":"/bin/terminal","arguments":["attach"]}"#).unwrap();
    assert!(bare.environment.is_empty());
    assert!(!serde_json::to_string(&bare)
        .unwrap()
        .contains("environment"));
    round_trips(Attached::Command { command });
    round_trips(Attached::Shown);
    round_trips(Attached::Refused {
        reason: "the terminal holds no session a person can be shown".into(),
    });
    round_trips(WindowShown::Opened);
    round_trips(WindowShown::Printed {
        line: "XDG_CONFIG_HOME=<set> /bin/terminal session attach work".into(),
    });
    round_trips(WindowShown::Refused {
        reason: "Unable to find application named 'Window.app'".into(),
    });
    for (view, spelled) in [(View::Terminal, "terminal"), (View::App, "app")] {
        round_trips(view);
        assert_eq!(serde_json::to_value(view).unwrap(), spelled);
        assert_eq!(view.name(), spelled);
    }
    for (from, spelled) in [
        (NamedBy::Stream, "stream"),
        (NamedBy::Host, "host"),
        (NamedBy::Neither, "neither"),
    ] {
        round_trips(from);
        assert_eq!(serde_json::to_value(from).unwrap(), spelled);
        let display = StreamDisplay {
            multiplexer: ProfileHeld {
                profile: "terminal".into(),
                from,
            },
            view: ViewHeld {
                view: View::App,
                from,
            },
            terminal_app: ProfileHeld {
                profile: "window".into(),
                from,
            },
        };
        round_trips(display.clone());
        assert_eq!(
            serde_json::to_value(&display).unwrap()["view"],
            serde_json::json!({"view": "app", "from": spelled})
        );
    }
    round_trips(Stream {
        title: Some("Work Agents".into()),
        ..Stream::default()
    });
    let stream = Stream {
        multiplexer: Some("terminal".into()),
        view: Some(View::Terminal),
        terminal_app: Some("window".into()),
        title: Some("Work Agents".into()),
        ..Stream::default()
    };
    round_trips(stream.clone());
    assert_eq!(
        serde_json::to_value(&stream).unwrap(),
        serde_json::json!({
            "priority": 0,
            "multiplexer": "terminal",
            "view": "terminal",
            "terminal_app": "window",
            "title": "Work Agents",
        })
    );
    let before: Stream = serde_json::from_str(r#"{"agents":4,"priority":10}"#).unwrap();
    assert_eq!(before.title, None);
    assert_eq!(
        (&before.multiplexer, before.view, &before.terminal_app),
        (&None, None, &None)
    );
    let written = serde_json::to_string(&before).unwrap();
    for key in ["title", "multiplexer", "view", "terminal_app"] {
        assert!(!written.contains(key), "{written} gives no {key}");
    }

    // A start holds the terminal its agent opened in; one written before
    // reads with none, the host's, and writes none back (56, 190).
    let state = held();
    let json = serde_json::to_value(&state).expect("held state as JSON");
    assert_eq!(
        json["stages"][0]["start"]["terminal"],
        serde_json::json!({"profile": "fake", "from": "stream"})
    );
    let mut before = json;
    before["stages"][0]["start"]
        .as_object_mut()
        .unwrap()
        .remove("terminal");
    let read: HeldState = serde_json::from_value(before.clone()).expect("read from before");
    assert_eq!(
        read.stages[0].start.as_ref().expect("a start").terminal,
        None
    );
    assert_eq!(
        serde_json::to_value(&read).expect("written again"),
        before,
        "no terminal written where none is held"
    );
}

#[test]
fn each_answer_of_runs() {
    // The seam-contracts spec's runs: each of its four answers, with its
    // name or its reason, written and read back unchanged (61, 201).
    for (runs, spelled) in [
        (SessionRuns::Runs { name: "w".into() }, "runs"),
        (SessionRuns::NotRunning { name: "w".into() }, "not_running"),
        (
            SessionRuns::NoPart {
                reason: "the terminal holds no session a person can be shown".into(),
            },
            "no_part",
        ),
        (
            SessionRuns::Unread {
                reason: "the terminal did not answer within its bound".into(),
            },
            "unread",
        ),
    ] {
        round_trips(runs.clone());
        let json = serde_json::to_value(&runs).unwrap();
        assert!(json.get(spelled).is_some(), "{json} is tagged {spelled}");
    }
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

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
    }
}

fn kind() -> Kind {
    Kind {
        name: "bump".into(),
        version: 2,
        branch_prefix: "bump".into(),
        starts_unready: true,
        lands: true,
        stages: vec![stage()],
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
            end: Some(StageEnd {
                head: Some(Revision::new("c3")),
                report: Some("/s/conclusion.md".into()),
                words: Some("which design?".into()),
                needs: Some(StopKind::FoundWork),
            }),
            start: Some(StartHeld {
                n: 2,
                started_at: 5,
                start_head: Revision::new("c2"),
                words: Some("the second".into()),
                identity: identity(),
                secret: Secret("s3cret".into()),
                program: "agent".into(),
                settings: settings(),
                record: Some("record-1".into()),
                prompt: prompt(),
                live: true,
                begun_at: Some(75),
                first_quiet: Some(100),
                last_head: Revision::new("c3"),
                last_head_move: 90,
                past_limit: true,
            }),
            waiting_since: Some(120),
        }],
        current: Some(0),
        submission: SubmissionHeld {
            due: true,
            submits: 2,
            send_backs: 1,
            id: Some("7".into()),
            verdict: Some(Verdict::Conflict {
                files: vec!["src/lib.rs".into()],
                words: "conflicts with main".into(),
            }),
            waiting: Some("withdrawn".into()),
            unread: Some("the merge queue is down".into()),
        },
        failures: vec!["the agent did not end".into()],
        priority: -3,
    }
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
        hold: vec!["review".into()],
        hold_submission: true,
        mark: Some("team-a".into()),
        hold_words: BTreeMap::from([("review".into(), "until the design is ruled".into())]),
        priority: 5,
        recipients: vec!["operator".into()],
    });
}

#[test]
fn a_held_state() {
    round_trips(held());
    for phase in [
        Phase::Making { by: "c1".into() },
        Phase::Live,
        Phase::Gone {
            over: Over::Dropped,
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
    });
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
    });
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
        StageState::Started,
        StageState::Begun,
        StageState::NeverBegun,
        StageState::Stuck,
        StageState::Lost,
        StageState::Stopped,
        StageState::Delivered,
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

#[test]
fn the_change_seams_answers() {
    round_trips(ChangeRead::Read {
        facts: ChangeFacts {
            exists: true,
            planning_complete: true,
            tasks: vec![Task {
                text: "1.1 do it".into(),
                ticked: true,
            }],
            archived: false,
        },
    });
    round_trips(ChangeRead::CouldNotRead {
        reason: "bad schema".into(),
    });
}

#[test]
fn the_agent_seams_answers() {
    round_trips(AgentCommand {
        program: "agent".into(),
        args: vec!["--model".into(), "large".into()],
        env: BTreeMap::from([("ID".into(), identity().to_string())]),
        prompt_args: vec!["do it".into()],
        folder_trust: Some(FolderTrust {
            question: "Do you trust the files in this folder?".into(),
            answer: "1".into(),
        }),
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
    round_trips(Offered::Offered);
    round_trips(Offered::NotOffered);
    round_trips(Guarded::Refused {
        acts: vec!["submit".into()],
    });
    round_trips(Guarded::NotOffered);
}

#[test]
fn the_terminal_seams_answers() {
    round_trips(Opened::Opened { carried: true });
    round_trips(Opened::Refused {
        reason: "identity live".into(),
    });
    round_trips(Sent::Sent);
    round_trips(Sent::Refused {
        reason: "cannot send".into(),
    });
    round_trips(Closed::Closed);
    round_trips(Closed::Refused {
        reason: "still running".into(),
    });
    round_trips(Watch::Bound { seconds: 30 });
    round_trips(Watch::None);
    round_trips(Some("pane-3".to_string()));
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

//! Every act's answer in one JSON form that only ever gains fields (61):
//! the suite holds each answer's field paths, read from a full instance
//! of it, and fails naming the answer and the field when one is gone. A
//! new field is added to its list in the commit that adds it.

use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::*;
use flume_seams::*;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "answers: every answer holds every field its list names",
            cites: &["61", "60"],
            run: every_field_listed,
        },
        Scenario {
            name:
                "answers: an answer that drops a listed field fails naming the answer and the field",
            cites: &["61"],
            run: a_field_removed,
        },
        Scenario {
            name: "answers: a refusal is an answer of the same form",
            cites: &["2", "61"],
            run: a_refusal_is_an_answer,
        },
    ]
}

/// Every field path of a JSON value: an object's keys joined by `.`, an
/// array's items under `[]`.
pub fn paths(value: &Value) -> BTreeSet<String> {
    fn walk(value: &Value, at: &str, into: &mut BTreeSet<String>) {
        match value {
            Value::Object(map) => {
                for (key, inner) in map {
                    let path = if at.is_empty() {
                        key.clone()
                    } else {
                        format!("{at}.{key}")
                    };
                    into.insert(path.clone());
                    walk(inner, &path, into);
                }
            }
            Value::Array(items) => {
                for item in items {
                    walk(item, &format!("{at}[]"), into);
                }
            }
            _ => {}
        }
    }
    let mut into = BTreeSet::new();
    walk(value, "", &mut into);
    into
}

/// Each field `answer`'s list names that `json` lacks.
pub fn missing(answer: &str, json: &Value) -> Vec<String> {
    let have = paths(json);
    FIELDS
        .iter()
        .filter(|(name, _)| *name == answer)
        .flat_map(|(_, listed)| listed.iter())
        .filter(|path| !have.contains(**path))
        .map(|path| format!("the answer {answer} lacks its field {path}"))
        .collect()
}

fn settings() -> Settings {
    Settings {
        program: Some("agent".into()),
        model: Some("large".into()),
        instructions: Some("brief".into()),
        env: BTreeMap::from([("KEY".into(), "value".into())]),
        beside: Some("operator".into()),
    }
}

fn place_status() -> PlaceStatus {
    PlaceStatus {
        place: PlaceId::new("/r/places/fix--one"),
        repository: "/r".into(),
        kind: "fix".into(),
        version: 1,
        declared: Declared::Flume,
        name: "one".into(),
        branch: "fix/one".into(),
        base: "main".into(),
        mark: Some("team-a".into()),
        lands: true,
        phase: Phase::Over {
            over: Over::Landed {
                commit: Revision::new("c9"),
            },
            kept: Some(Kept::Holds {
                holds: Holds {
                    uncommitted: vec!["a.rs".into()],
                    untracked: vec!["b.txt".into()],
                    lock: Some("locked".into()),
                },
            }),
        },
        readiness: Readiness {
            steps: vec![ReadyStep {
                step: "toolchain".into(),
                passed: false,
                words: "offline".into(),
            }],
        },
        stages: vec![StageStatus {
            name: "code".into(),
            state: StageState::EndedShort,
            starts: 2,
            end: Some(StageEnd {
                head: Some(Revision::new("c3")),
                report: Some("/s/conclusion.md".into()),
                words: Some("which design?".into()),
                needs: Some(StopKind::NeedsAnswer),
            }),
            refused_part: Some("words".into()),
            conversation: false,
        }],
        current: Some("code".into()),
        agent: Some(AgentStatus {
            identity: "flume:/r:fix--one:code:2".into(),
            stage: "code".into(),
            program: "agent".into(),
            settings: settings(),
            state: AgentState::Working,
            asking: Reading::No,
            own_work: Reading::Unknown,
            past_limit: true,
        }),
        head: Some(Revision::new("c3")),
        delivered: Some(false),
        tasks: Some(vec![Task {
            text: "1.1 do it".into(),
            ticked: true,
        }]),
        submission: Some(SubmissionStatus {
            read: SubmissionRead::Found {
                submission: Submission {
                    id: "7".into(),
                    branch: "fix/one".into(),
                    target: "main".into(),
                    state: SubmissionState::Failed {
                        words: "test x failed".into(),
                    },
                },
            },
            submits: 2,
            send_backs: 1,
            verdict: Some(Verdict::Failed {
                words: "test x failed".into(),
            }),
            waiting: Some("the count is reached".into()),
            held: true,
        }),
        refused_acts: vec!["a push".into()],
        cannot_read: vec!["the agent's state".into()],
        failures: vec!["did not end".into()],
    }
}

fn tidied() -> Tidied {
    let place = PlaceId::new("/r/places/fix--one");
    Tidied {
        removed: vec![place.clone()],
        kept: vec![KeptPlace {
            place: place.clone(),
            why: Kept::Teardown {
                step: "pre-remove".into(),
                words: "busy".into(),
            },
        }],
        branches_kept: vec![BranchKept {
            place: place.clone(),
            branch: "fix/one".into(),
            commits: vec![Revision::new("c4")],
        }],
        not_over: vec![place],
    }
}

/// A full instance of each answer, every field present.
pub fn full() -> Vec<(&'static str, Answer)> {
    let place = PlaceId::new("/r/places/fix--one");
    vec![
        (
            "status",
            Answer::Status {
                places: vec![place_status()],
            },
        ),
        (
            "launched",
            Answer::Launched {
                place: place_status(),
            },
        ),
        (
            "prompt",
            Answer::Prompt {
                place: place.clone(),
                stage: "code".into(),
                text: "do it".into(),
            },
        ),
        (
            "acted",
            Answer::Acted {
                act: Act::Drop,
                place: place.clone(),
                status: Some(place_status()),
                tidy: Some(tidied()),
            },
        ),
        (
            "settled",
            Answer::Settled {
                places: vec![place],
                tidy: tidied(),
            },
        ),
        ("tidied", Answer::Tidied { tidy: tidied() }),
        (
            "stop_check",
            Answer::StopCheck {
                words: Some("deliver commits".into()),
            },
        ),
        (
            "refused",
            Answer::refused("the kind bump resolves to no declaration"),
        ),
    ]
}

fn every_field_listed(_: &()) -> Outcome {
    let full = full();
    ensure_eq!(
        full.iter().map(|(name, _)| *name).collect::<Vec<_>>(),
        FIELDS.iter().map(|(name, _)| *name).collect::<Vec<_>>(),
        "the answers listed"
    );
    for (name, answer) in full {
        let json: Value =
            serde_json::from_str(&answer.to_json()).map_err(|error| error.to_string())?;
        let missing = missing(name, &json);
        ensure!(missing.is_empty(), "{}", missing.join("; "));
    }
    Ok(())
}

fn a_field_removed(_: &()) -> Outcome {
    let (name, answer) = full().remove(1);
    let mut json: Value =
        serde_json::from_str(&answer.to_json()).map_err(|error| error.to_string())?;
    json["launched"]["place"]
        .as_object_mut()
        .ok_or("no place")?
        .remove("branch");
    let missing = missing(name, &json);
    ensure_eq!(
        missing,
        vec!["the answer launched lacks its field launched.place.branch".to_string()],
        "the fields missed"
    );
    Ok(())
}

fn a_refusal_is_an_answer(_: &()) -> Outcome {
    let world = super::world::World::new();
    let refused = world
        .engine
        .launch(&super::world::launch("bump", "one", Some("x")));
    let json = refused.to_json();
    let back: Answer = serde_json::from_str(&json).map_err(|error| error.to_string())?;
    ensure_eq!(back, refused, "the refusal read back");
    let value: Value = serde_json::from_str(&json).map_err(|error| error.to_string())?;
    ensure!(
        missing("refused", &value).is_empty(),
        "the refusal lacks a field"
    );
    Ok(())
}

/// Each answer's field paths, read from its full instance.
pub const FIELDS: &[(&str, &[&str])] = &[
    (
        "status",
        &[
            "status",
            "status.places",
            "status.places[].agent",
            "status.places[].agent.asking",
            "status.places[].agent.identity",
            "status.places[].agent.own_work",
            "status.places[].agent.past_limit",
            "status.places[].agent.program",
            "status.places[].agent.settings",
            "status.places[].agent.settings.beside",
            "status.places[].agent.settings.env",
            "status.places[].agent.settings.env.KEY",
            "status.places[].agent.settings.instructions",
            "status.places[].agent.settings.model",
            "status.places[].agent.settings.program",
            "status.places[].agent.stage",
            "status.places[].agent.state",
            "status.places[].base",
            "status.places[].branch",
            "status.places[].cannot_read",
            "status.places[].current",
            "status.places[].declared",
            "status.places[].delivered",
            "status.places[].failures",
            "status.places[].head",
            "status.places[].kind",
            "status.places[].lands",
            "status.places[].mark",
            "status.places[].name",
            "status.places[].phase",
            "status.places[].phase.over",
            "status.places[].phase.over.kept",
            "status.places[].phase.over.kept.holds",
            "status.places[].phase.over.kept.holds.holds",
            "status.places[].phase.over.kept.holds.holds.lock",
            "status.places[].phase.over.kept.holds.holds.uncommitted",
            "status.places[].phase.over.kept.holds.holds.untracked",
            "status.places[].phase.over.over",
            "status.places[].phase.over.over.landed",
            "status.places[].phase.over.over.landed.commit",
            "status.places[].place",
            "status.places[].readiness",
            "status.places[].readiness.steps",
            "status.places[].readiness.steps[].passed",
            "status.places[].readiness.steps[].step",
            "status.places[].readiness.steps[].words",
            "status.places[].refused_acts",
            "status.places[].repository",
            "status.places[].stages",
            "status.places[].stages[].conversation",
            "status.places[].stages[].end",
            "status.places[].stages[].end.head",
            "status.places[].stages[].end.needs",
            "status.places[].stages[].end.report",
            "status.places[].stages[].end.words",
            "status.places[].stages[].name",
            "status.places[].stages[].refused_part",
            "status.places[].stages[].starts",
            "status.places[].stages[].state",
            "status.places[].submission",
            "status.places[].submission.held",
            "status.places[].submission.read",
            "status.places[].submission.read.found",
            "status.places[].submission.read.found.submission",
            "status.places[].submission.read.found.submission.branch",
            "status.places[].submission.read.found.submission.id",
            "status.places[].submission.read.found.submission.state",
            "status.places[].submission.read.found.submission.state.failed",
            "status.places[].submission.read.found.submission.state.failed.words",
            "status.places[].submission.read.found.submission.target",
            "status.places[].submission.send_backs",
            "status.places[].submission.submits",
            "status.places[].submission.verdict",
            "status.places[].submission.verdict.failed",
            "status.places[].submission.verdict.failed.words",
            "status.places[].submission.waiting",
            "status.places[].tasks",
            "status.places[].tasks[].text",
            "status.places[].tasks[].ticked",
            "status.places[].version",
        ],
    ),
    (
        "launched",
        &[
            "launched",
            "launched.place",
            "launched.place.agent",
            "launched.place.agent.asking",
            "launched.place.agent.identity",
            "launched.place.agent.own_work",
            "launched.place.agent.past_limit",
            "launched.place.agent.program",
            "launched.place.agent.settings",
            "launched.place.agent.settings.beside",
            "launched.place.agent.settings.env",
            "launched.place.agent.settings.env.KEY",
            "launched.place.agent.settings.instructions",
            "launched.place.agent.settings.model",
            "launched.place.agent.settings.program",
            "launched.place.agent.stage",
            "launched.place.agent.state",
            "launched.place.base",
            "launched.place.branch",
            "launched.place.cannot_read",
            "launched.place.current",
            "launched.place.declared",
            "launched.place.delivered",
            "launched.place.failures",
            "launched.place.head",
            "launched.place.kind",
            "launched.place.lands",
            "launched.place.mark",
            "launched.place.name",
            "launched.place.phase",
            "launched.place.phase.over",
            "launched.place.phase.over.kept",
            "launched.place.phase.over.kept.holds",
            "launched.place.phase.over.kept.holds.holds",
            "launched.place.phase.over.kept.holds.holds.lock",
            "launched.place.phase.over.kept.holds.holds.uncommitted",
            "launched.place.phase.over.kept.holds.holds.untracked",
            "launched.place.phase.over.over",
            "launched.place.phase.over.over.landed",
            "launched.place.phase.over.over.landed.commit",
            "launched.place.place",
            "launched.place.readiness",
            "launched.place.readiness.steps",
            "launched.place.readiness.steps[].passed",
            "launched.place.readiness.steps[].step",
            "launched.place.readiness.steps[].words",
            "launched.place.refused_acts",
            "launched.place.repository",
            "launched.place.stages",
            "launched.place.stages[].conversation",
            "launched.place.stages[].end",
            "launched.place.stages[].end.head",
            "launched.place.stages[].end.needs",
            "launched.place.stages[].end.report",
            "launched.place.stages[].end.words",
            "launched.place.stages[].name",
            "launched.place.stages[].refused_part",
            "launched.place.stages[].starts",
            "launched.place.stages[].state",
            "launched.place.submission",
            "launched.place.submission.held",
            "launched.place.submission.read",
            "launched.place.submission.read.found",
            "launched.place.submission.read.found.submission",
            "launched.place.submission.read.found.submission.branch",
            "launched.place.submission.read.found.submission.id",
            "launched.place.submission.read.found.submission.state",
            "launched.place.submission.read.found.submission.state.failed",
            "launched.place.submission.read.found.submission.state.failed.words",
            "launched.place.submission.read.found.submission.target",
            "launched.place.submission.send_backs",
            "launched.place.submission.submits",
            "launched.place.submission.verdict",
            "launched.place.submission.verdict.failed",
            "launched.place.submission.verdict.failed.words",
            "launched.place.submission.waiting",
            "launched.place.tasks",
            "launched.place.tasks[].text",
            "launched.place.tasks[].ticked",
            "launched.place.version",
        ],
    ),
    (
        "prompt",
        &["prompt", "prompt.place", "prompt.stage", "prompt.text"],
    ),
    (
        "acted",
        &[
            "acted",
            "acted.act",
            "acted.place",
            "acted.status",
            "acted.status.agent",
            "acted.status.agent.asking",
            "acted.status.agent.identity",
            "acted.status.agent.own_work",
            "acted.status.agent.past_limit",
            "acted.status.agent.program",
            "acted.status.agent.settings",
            "acted.status.agent.settings.beside",
            "acted.status.agent.settings.env",
            "acted.status.agent.settings.env.KEY",
            "acted.status.agent.settings.instructions",
            "acted.status.agent.settings.model",
            "acted.status.agent.settings.program",
            "acted.status.agent.stage",
            "acted.status.agent.state",
            "acted.status.base",
            "acted.status.branch",
            "acted.status.cannot_read",
            "acted.status.current",
            "acted.status.declared",
            "acted.status.delivered",
            "acted.status.failures",
            "acted.status.head",
            "acted.status.kind",
            "acted.status.lands",
            "acted.status.mark",
            "acted.status.name",
            "acted.status.phase",
            "acted.status.phase.over",
            "acted.status.phase.over.kept",
            "acted.status.phase.over.kept.holds",
            "acted.status.phase.over.kept.holds.holds",
            "acted.status.phase.over.kept.holds.holds.lock",
            "acted.status.phase.over.kept.holds.holds.uncommitted",
            "acted.status.phase.over.kept.holds.holds.untracked",
            "acted.status.phase.over.over",
            "acted.status.phase.over.over.landed",
            "acted.status.phase.over.over.landed.commit",
            "acted.status.place",
            "acted.status.readiness",
            "acted.status.readiness.steps",
            "acted.status.readiness.steps[].passed",
            "acted.status.readiness.steps[].step",
            "acted.status.readiness.steps[].words",
            "acted.status.refused_acts",
            "acted.status.repository",
            "acted.status.stages",
            "acted.status.stages[].conversation",
            "acted.status.stages[].end",
            "acted.status.stages[].end.head",
            "acted.status.stages[].end.needs",
            "acted.status.stages[].end.report",
            "acted.status.stages[].end.words",
            "acted.status.stages[].name",
            "acted.status.stages[].refused_part",
            "acted.status.stages[].starts",
            "acted.status.stages[].state",
            "acted.status.submission",
            "acted.status.submission.held",
            "acted.status.submission.read",
            "acted.status.submission.read.found",
            "acted.status.submission.read.found.submission",
            "acted.status.submission.read.found.submission.branch",
            "acted.status.submission.read.found.submission.id",
            "acted.status.submission.read.found.submission.state",
            "acted.status.submission.read.found.submission.state.failed",
            "acted.status.submission.read.found.submission.state.failed.words",
            "acted.status.submission.read.found.submission.target",
            "acted.status.submission.send_backs",
            "acted.status.submission.submits",
            "acted.status.submission.verdict",
            "acted.status.submission.verdict.failed",
            "acted.status.submission.verdict.failed.words",
            "acted.status.submission.waiting",
            "acted.status.tasks",
            "acted.status.tasks[].text",
            "acted.status.tasks[].ticked",
            "acted.status.version",
            "acted.tidy",
            "acted.tidy.branches_kept",
            "acted.tidy.branches_kept[].branch",
            "acted.tidy.branches_kept[].commits",
            "acted.tidy.branches_kept[].place",
            "acted.tidy.kept",
            "acted.tidy.kept[].place",
            "acted.tidy.kept[].why",
            "acted.tidy.kept[].why.teardown",
            "acted.tidy.kept[].why.teardown.step",
            "acted.tidy.kept[].why.teardown.words",
            "acted.tidy.not_over",
            "acted.tidy.removed",
        ],
    ),
    (
        "settled",
        &[
            "settled",
            "settled.places",
            "settled.tidy",
            "settled.tidy.branches_kept",
            "settled.tidy.branches_kept[].branch",
            "settled.tidy.branches_kept[].commits",
            "settled.tidy.branches_kept[].place",
            "settled.tidy.kept",
            "settled.tidy.kept[].place",
            "settled.tidy.kept[].why",
            "settled.tidy.kept[].why.teardown",
            "settled.tidy.kept[].why.teardown.step",
            "settled.tidy.kept[].why.teardown.words",
            "settled.tidy.not_over",
            "settled.tidy.removed",
        ],
    ),
    (
        "tidied",
        &[
            "tidied",
            "tidied.tidy",
            "tidied.tidy.branches_kept",
            "tidied.tidy.branches_kept[].branch",
            "tidied.tidy.branches_kept[].commits",
            "tidied.tidy.branches_kept[].place",
            "tidied.tidy.kept",
            "tidied.tidy.kept[].place",
            "tidied.tidy.kept[].why",
            "tidied.tidy.kept[].why.teardown",
            "tidied.tidy.kept[].why.teardown.step",
            "tidied.tidy.kept[].why.teardown.words",
            "tidied.tidy.not_over",
            "tidied.tidy.removed",
        ],
    ),
    ("stop_check", &["stop_check", "stop_check.words"]),
    (
        "refused",
        &["refused", "refused.refusal", "refused.refusal.reason"],
    ),
];

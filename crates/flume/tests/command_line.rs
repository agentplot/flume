//! A caller's hold and keyed answers through the command line over the
//! fakes (command-line spec, "needs --to, hold --waiting, retry --answers
//! and tell --answers"): each command's words parsed as flume's binary
//! parses them and asked of one core built over the conformance world,
//! whose change `plan` in `east` waits on construct's stop-short naming
//! `ops`. The fakes keep construct's agent open for its answer, which the
//! printed command cannot (41, 42, 49, 53, 61, 130, 198).

use flume::commands;
use flume_conformance::engine::caller_acts::asks_ops;
use flume_conformance::engine::world::World;
use flume_core::{Answer, PlaceStatus};
use flume_seams::*;
use serde_json::Value;

/// The key of construct's first stop-short, as the inbox and status give
/// it (42, 43).
const KEY: &str = "stage/construct/1/ended-short";

/// What flume printed for `words`, asked of a core built over `world`,
/// and its exit status.
fn run(world: &World, words: &[&str]) -> (String, u8) {
    let outcome = commands::run(
        std::iter::once("flume").chain(words.iter().copied()),
        &|| Ok(world.engine()),
    );
    (outcome.text, outcome.code)
}

/// The answer `words` printed as its JSON, exiting 0.
fn answered(world: &World, words: &[&str]) -> Answer {
    let (text, code) = run(world, words);
    assert_eq!(code, 0, "{words:?}: {text}");
    serde_json::from_str(text.trim()).unwrap_or_else(|error| panic!("{error}: {text}"))
}

/// The refusal's reason `words` printed in the JSON form, exiting 1.
fn refused(world: &World, words: &[&str]) -> String {
    let (text, code) = run(world, words);
    assert_eq!(code, 1, "{words:?}: {text}");
    let answer: Value = serde_json::from_str(text.trim()).expect("the refusal's JSON");
    answer["refused"]["refusal"]["reason"]
        .as_str()
        .unwrap_or_else(|| panic!("no reason: {text}"))
        .to_string()
}

/// The place's status an act answered with, the act checked.
fn acted(answer: Answer, act: Act) -> PlaceStatus {
    match answer {
        Answer::Acted {
            act: done,
            status: Some(status),
            ..
        } if done == act => status,
        other => panic!("{act:?} answered {other:?}"),
    }
}

/// The answer the end keyed construct's first stop-short holds, as status
/// gives it.
fn answer_on(status: &PlaceStatus) -> Option<AnswerHeld> {
    status
        .ends
        .iter()
        .find(|end| end.key.to_string() == KEY)
        .unwrap_or_else(|| panic!("no end keyed {KEY}: {:?}", status.ends))
        .end
        .answer
        .clone()
}

/// The fake terminal's closes of the change `plan`'s agents.
fn closes(world: &World) -> usize {
    world
        .terminal
        .log
        .calls()
        .into_iter()
        .filter(|call| call.starts_with("close") && call.contains("change--plan"))
        .count()
}

#[test]
fn a_hold_on_the_waiting_move_then_a_retry_naming_its_key() {
    // The spec's "The forms" (41, 42, 49, 130): the hold answers with its
    // words and bound, and the retry with the place's status, its answer
    // recorded on the stop-short with its key and the hold ended.
    let world = World::new();
    let (id, _) = asks_ops(&world, None).expect("construct asks ops");
    let place = &id.to_string();
    let held = acted(
        answered(
            &world,
            &[
                "hold",
                place,
                "on the user's card",
                "--waiting",
                "--for",
                "1h",
                "--json",
            ],
        ),
        Act::Hold,
    );
    let hold = held
        .holds
        .iter()
        .find(|hold| hold.on == HoldOn::Waiting)
        .unwrap_or_else(|| panic!("no hold on the waiting move: {:?}", held.holds));
    assert_eq!(
        (hold.words.as_deref(), hold.bound, hold.lapsed),
        (Some("on the user's card"), Some(60 * 60), None)
    );
    assert!(!held.stalled, "stalled under the hold");

    let retried = acted(
        answered(
            &world,
            &[
                "retry",
                place,
                "--stage",
                "construct",
                "--answers",
                KEY,
                "--words",
                "yes",
                "--json",
            ],
        ),
        Act::Retry,
    );
    let answer = answer_on(&retried).expect("no answer recorded on the stop-short");
    assert_eq!(
        (
            answer.words.as_str(),
            answer.act,
            answer.key.map(|key| key.to_string())
        ),
        ("yes", Act::Retry, Some(KEY.to_string()))
    );
    assert!(
        !retried.holds.iter().any(|hold| hold.on == HoldOn::Waiting),
        "the hold outlived the move: {:?}",
        retried.holds
    );
}

#[test]
fn a_tell_naming_its_key() {
    // The spec's "A tell's key" (42, 53, 198): construct's agent kept open
    // takes the tell as its answer, recorded with the key; a second naming
    // the key is refused, naming the answer taken.
    let world = World::new();
    let (id, _) = asks_ops(&world, Some("1h")).expect("construct asks ops");
    let place = &id.to_string();
    let told = acted(
        answered(&world, &["tell", place, "--answers", KEY, "yes", "--json"]),
        Act::Message,
    );
    let answer = answer_on(&told).expect("no answer recorded on the stop-short");
    assert_eq!(
        (
            answer.words.as_str(),
            answer.act,
            answer.key.map(|key| key.to_string()),
            answer.session.is_some()
        ),
        ("yes", Act::Message, Some(KEY.to_string()), true)
    );
    let again = refused(&world, &["tell", place, "--answers", KEY, "no", "--json"]);
    assert!(
        again.contains(KEY) && again.contains("yes") && again.contains("taken once"),
        "{again}"
    );
}

#[test]
fn a_bound_and_a_key_in_a_wrong_form() {
    // The spec's "A hold's bound in a wrong form" and "A key in a wrong
    // form" (42, 130): each refused naming the forms, nothing held and no
    // agent ended.
    let world = World::new();
    let (id, _) = asks_ops(&world, None).expect("construct asks ops");
    let place = &id.to_string();
    let reason = refused(
        &world,
        &[
            "hold",
            place,
            "words",
            "--waiting",
            "--for",
            "soon",
            "--json",
        ],
    );
    assert_eq!(reason, "soon is no time: it is <n>, <n>s, <n>m or <n>h");
    let held = world.held(&id).expect("the place");
    assert!(
        !held.holds.iter().any(|hold| hold.on == HoldOn::Waiting),
        "a hold was put: {:?}",
        held.holds
    );

    let closed = closes(&world);
    let reason = refused(
        &world,
        &["retry", place, "--answers", "construct", "--json"],
    );
    assert_eq!(
        reason,
        format!("`construct` is no end's key: a key is {END_KEY_FORMS}")
    );
    assert_eq!(closes(&world), closed, "an agent was ended");
    assert_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::EndedShort)
    );
}

#[test]
fn a_bound_on_a_stage_hold() {
    // 130: a bound is the waiting move's alone, refused on a stage's hold
    // by the core, nothing held.
    let world = World::new();
    let (id, _) = asks_ops(&world, None).expect("construct asks ops");
    let reason = refused(
        &world,
        &[
            "hold",
            &id.to_string(),
            "after review",
            "--stage",
            "verify",
            "--for",
            "1h",
            "--json",
        ],
    );
    assert!(reason.contains("a bound is given only"), "{reason}");
    assert!(world.held(&id).expect("the place").holds.is_empty());
}

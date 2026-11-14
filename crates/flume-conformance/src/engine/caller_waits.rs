//! What a place waits on its caller for, recorded in flume and never in
//! a caller's memory (8, 47, 49, 56, 130; core.24): a hold put at the
//! launch or after it, with its words and since when, ended only by a
//! retry naming it; the move a waiting place waits on and since when,
//! read by the settle from the held state alone; and a place waiting
//! under no hold's words past its bound read stalled, which each
//! recipient is delivered once. A hold's event is the caller's to see,
//! and a stage waiting for room waits on the host, never on its caller
//! (70, 72).

use super::role_waking::answerer_behind;
use super::settling::{construct_begun, deliver_fix, fix_begun, write_change};
use super::stage_runs::east_and_west;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Answer, Collected, PlaceStatus, Retry, Waits};
use flume_fakes::Plan;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "caller waits: a hold on code put while construct works, for a card's answer, starts no code at construct's delivery until a retry names code",
            cites: &["130", "47", "49", "8"],
            run: a_hold_for_a_cards_answer,
        },
        Scenario {
            name: "caller waits: a hold on a fix's submission put after the launch submits nothing until a retry names the submission",
            cites: &["130", "47", "49", "111"],
            run: a_hold_on_the_submission,
        },
        Scenario {
            name: "caller waits: a fix of its base's failed checks, launched on the base with the failure's words as its brief and its submission held, is cut from the base's current revision, its code delivers and the place waits on the hold, and a retry naming the submission submits it and it lands",
            cites: &["S64", "7", "22", "47", "49", "111", "131"],
            run: s64_a_fix_of_its_bases_failed_checks,
        },
        Scenario {
            name: "caller waits: a hold on the stage whose agent works is refused naming the stage, and nothing changes",
            cites: &["130", "47", "8"],
            run: a_hold_on_a_running_stage,
        },
        Scenario {
            name: "caller waits: a hold on a repeating code stage while its repetition on group 2 works lets that repetition go on, starts no next one at its delivery, status giving code on hold with the words and naming group 3, and a retry of code starts group 3",
            cites: &["142", "130", "47", "49", "S51"],
            run: a_hold_on_a_running_repeating_stage,
        },
        Scenario {
            name: "caller waits: a hold on a place whose work is over, a stage it has not, a second hold and a submission undecided are each refused naming why",
            cites: &["130", "47", "63", "111"],
            run: holds_refused,
        },
        Scenario {
            name: "caller waits: a hold on a stage waiting for room takes it out of the queue, and its retry queues it again",
            cites: &["130", "70", "47", "49"],
            run: a_hold_on_a_stage_waiting_for_room,
        },
        Scenario {
            name: "caller waits: a change launched with code held with words leaves code on hold with them at construct's delivery, and is never stalled",
            cites: &["130", "7", "47"],
            run: held_at_the_launch_with_words,
        },
        Scenario {
            name: "caller waits: a hold whose words name another place stays when that place lands, until a retry names it",
            cites: &["130", "70", "72", "47"],
            run: another_place_lands,
        },
        Scenario {
            name: "caller waits: a lost stage waits on a retry since that reading, and five minutes on it is stalled",
            cites: &["130", "45", "43"],
            run: a_lost_stage_waits_on_its_caller,
        },
        Scenario {
            name: "caller waits: a construct ended short needing an answer under a watch is stalled at five minutes, each recipient handed that once by notice and the stall pending for each until it confirms it",
            cites: &["130", "43", "42", "122"],
            run: stalled_after_five_minutes,
        },
        Scenario {
            name: "caller waits: a launch's bound of thirty minutes leaves its place unstalled at five, and stalled at thirty",
            cites: &["130", "7"],
            run: the_launchs_bound,
        },
        Scenario {
            name: "caller waits: a place held with its caller's words for a day is never stalled and no stall is delivered",
            cites: &["130", "47", "43"],
            run: a_held_place_is_not_stalled,
        },
        Scenario {
            name: "caller waits: a place launched from the pane ops stalls in east, behind which acme declares east/operator with a run, and its stall is handed once to ops by notice and stands pending for ops and east/operator, whose run the settle wakes and whose inbox gives the stall with the stop-short's words and key",
            cites: &["193", "130", "43", "132", "S94"],
            run: the_stall_reaches_the_answerer,
        },
        Scenario {
            name: "caller waits: a construct in east whose recipient is east/planner, stopped short naming ops and neither held nor answered for five minutes, stalls to east/planner, east/ops and east/operator, once each",
            cites: &["130", "41", "43", "193"],
            run: the_role_a_question_named,
        },
        Scenario {
            name: "caller waits: the same construct stopped short naming west/design stalls to east/planner, west/design and east/operator, the answerer behind its own stream, once each, and not to west/operator",
            cites: &["130", "41", "143", "193", "S98"],
            run: another_streams_role_named,
        },
        Scenario {
            name: "caller waits: a place whose one recipient is its stream's answerer stalls to the answerer once",
            cites: &["130", "43", "193"],
            run: the_answerer_is_the_recipient,
        },
        Scenario {
            name: "caller waits: a stall an earlier flume recorded, delivered to nobody, is delivered once to east/operator, now standing behind the place's stream, at the next settle, with the stop-short it waits on, and to no one again after",
            cites: &["130", "43", "193"],
            run: a_stall_from_the_earlier_release,
        },
        Scenario {
            name: "caller waits: a role's run holds the waiting move of a construct ended short needing an answer with east-planner has it for an hour; status gives the hold with its words, since when and its bound, and the place is not stalled while it holds",
            cites: &["130", "47", "56"],
            run: a_role_takes_the_question,
        },
        Scenario {
            name: "caller waits: a later hold on the waiting move, passed to east/design for an hour, gives its words, since when and bound in place of the first",
            cites: &["130", "47"],
            run: the_question_passed_on,
        },
        Scenario {
            name: "caller waits: a retry naming construct while a hold on the waiting move stands ends the hold, and a drop ends another with its place",
            cites: &["130", "49", "52"],
            run: the_move_made,
        },
        Scenario {
            name: "caller waits: a hold on the waiting move of a place whose construct works is refused naming the place and that it waits on no move of its caller's, a bound on a stage's hold is refused, and nothing changes",
            cites: &["130", "47"],
            run: nothing_waits,
        },
        Scenario {
            name: "caller waits: a place in east whose recipient is east/planner stops short naming west/design; west/design's run holds the waiting move on the user's card for a day; the place is not stalled that day; at the day the watch's settle lapses the hold, status says it lapsed and when, and the stall naming its words is delivered to east/planner, west/design and east/operator once each; the operator's run holds again and nothing more is delivered",
            cites: &["S98", "130", "41", "132", "193"],
            run: s98_the_users_question_held_and_lapsed,
        },
        Scenario {
            name: "caller waits: a stage waiting for room an hour waits on the host, never on its caller, and is never stalled",
            cites: &["130", "70", "56"],
            run: a_long_queue,
        },
    ]
}

/// The place an answer acted on, or what the answer was.
fn acted(answer: &Answer) -> Result<PlaceStatus, String> {
    match answer {
        Answer::Acted {
            status: Some(status),
            ..
        } => Ok(status.clone()),
        other => Err(format!("the act answered {other:?}")),
    }
}

/// The hold a place's status gives on `on`.
fn hold_of(status: &PlaceStatus, on: &HoldOn) -> Option<Hold> {
    status.holds.iter().find(|hold| &hold.on == on).cloned()
}

fn on_stage(stage: &str) -> HoldOn {
    HoldOn::Stage {
        stage: stage.into(),
    }
}

/// The stall ends the place records.
fn stalls(world: &World, id: &PlaceId) -> Vec<EndHeld> {
    world
        .held(id)
        .map(|held| held.ends)
        .unwrap_or_default()
        .into_iter()
        .filter(|end| matches!(end.of, EndOf::Stall { .. }))
        .collect()
}

/// The notices sent to `recipient` that say a place stalled.
fn stall_notices(world: &World, recipient: &str) -> Vec<String> {
    world
        .terminal
        .notices(recipient)
        .into_iter()
        .filter(|text| text.contains(" stalled: "))
        .collect()
}

/// The key of the end of construct's first start, ended short.
fn construct_short_key() -> EndKey {
    EndKey::Stage {
        stage: "construct".into(),
        start: 1,
        state: StageState::EndedShort,
        answered: 0,
    }
}

/// The secret of the live start of the place's stage at `index`.
fn secret(world: &World, id: &PlaceId, index: usize) -> Result<Secret, String> {
    Ok(world
        .held(id)
        .and_then(|held| held.stages[index].start.clone())
        .ok_or("no start")?
        .secret)
}

/// A change `name` launched for `recipients`, its construct ended short
/// needing an answer; gives the place and when it was read so.
pub fn construct_asks(
    world: &World,
    name: &str,
    recipients: &[&str],
    stall_after: Option<Seconds>,
) -> Result<(PlaceId, Seconds), String> {
    let mut launch = launch("change", name, Some("plan it"));
    launch.recipients = recipients.iter().map(|name| name.to_string()).collect();
    launch.stall_after = stall_after;
    launched(&world.engine.launch(&launch))?;
    let id = World::id("change", name);
    world.settle(&id);
    let construct = World::identity("change", name, "construct", 1);
    let asked = world.host.now();
    let answer = world.engine.needs(
        &construct.to_string(),
        &secret(world, &id, 0)?.0,
        "Which design does the brief mean?",
        StopKind::NeedsAnswer,
    );
    ensure!(answer.refusal().is_none(), "the stop-short was refused");
    world.settle(&id);
    Ok((id, asked))
}

fn a_hold_for_a_cards_answer(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    world.host.advance(30);
    let held_at = world.host.now();
    let words = "waiting on the user's card about the schema";
    let status = acted(&world.engine.hold(&id, on_stage("code"), words, None))?;
    ensure_eq!(
        hold_of(&status, &on_stage("code")),
        Some(Hold {
            on: on_stage("code"),
            words: Some(words.into()),
            since: held_at,
            bound: None,
            lapsed: None,
        }),
        "the hold the act answers"
    );
    write_change(&world, "plan");
    world.host.advance(10);
    let delivered_at = world.host.now();
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::OnHold),
        "code at construct's delivery"
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    let status = world.status(&id)?;
    ensure_eq!(
        hold_of(&status, &on_stage("code")).and_then(|hold| hold.words),
        Some(words.to_string()),
        "the hold's words status gives"
    );
    let waiting = status.waiting_on.ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage.as_deref(), waiting.why),
        (Act::Retry, Some("code"), Waits::OnHold),
        "the move"
    );
    ensure_eq!(waiting.words.as_deref(), Some(words), "the move's words");
    ensure_eq!(waiting.since, Some(delivered_at), "since when");
    let retried = world
        .engine
        .retry(&id, Retry::Stage("code".into()), None, false);
    ensure!(retried.refusal().is_none(), "the retry: {retried:?}");
    world.settle(&id);
    ensure!(
        world
            .stage_state(&id, "code")
            .is_some_and(StageState::running),
        "code did not start on the retry"
    );
    let status = world.status(&id)?;
    ensure!(status.holds.is_empty(), "the retry left {:?}", status.holds);
    ensure_eq!(status.waiting_on, None, "the move once code runs");
    Ok(())
}

fn a_hold_on_the_submission(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_begun(&world, "one")?;
    let words = "until the release window opens";
    acted(&world.engine.hold(&id, HoldOn::Submission, words, None))?;
    deliver_fix(&world, &id);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions");
    let status = world.status(&id)?;
    ensure!(
        status.submission.as_ref().is_some_and(|s| s.held),
        "status gives the submission not held"
    );
    let waiting = status.waiting_on.ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage, waiting.why, waiting.words),
        (Act::Retry, None, Waits::OnHold, Some(words.to_string())),
        "the move"
    );
    let retried = world.engine.retry(&id, Retry::Submission, None, false);
    ensure!(retried.refusal().is_none(), "the retry: {retried:?}");
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    ensure!(
        world.status(&id)?.holds.is_empty(),
        "the submission still held"
    );
    Ok(())
}

fn s64_a_fix_of_its_bases_failed_checks(_: &()) -> Outcome {
    let world = World::new();
    let failure = "main's checks failed at the landed revision: the test parser::reads_tabs panicked at src/parser.rs:41";
    let base = world
        .git
        .branch_head(REPOSITORY, "main")
        .ok_or("main has no head")?;
    let mut fix = launch("fix", "main-checks", Some(failure));
    fix.hold_submission = true;
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "main-checks");
    world.settle(&id);
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "fix/main-checks"),
        Some(base),
        "the fix's branch, cut from the base's current revision"
    );
    let code = World::identity("fix", "main-checks", "code", 1);
    let prompt = world.prompt(&id, &code).ok_or("no prompt sent")?;
    ensure!(
        prompt.contains(failure),
        "the prompt lacks the failure's words: {prompt}"
    );
    deliver_fix(&world, &id);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "code"
    );
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions held");
    let status = world.status(&id)?;
    ensure!(
        status.submission.as_ref().is_some_and(|s| s.held),
        "status gives the submission not held"
    );
    let waiting = status.waiting_on.ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage, waiting.why),
        (Act::Retry, None, Waits::OnHold),
        "the move the place waits on"
    );
    let retried = world.engine.retry(&id, Retry::Submission, None, false);
    ensure!(retried.refusal().is_none(), "the retry: {retried:?}");
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the fix did not land");
    Ok(())
}

fn a_hold_on_a_running_stage(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_begun(&world, "one")?;
    let before = world.held(&id).ok_or("no place")?;
    let reason = refused(&world.engine.hold(&id, on_stage("code"), "wait", None))?;
    ensure!(reason.contains("code"), "the refusal: {reason}");
    ensure_eq!(world.held(&id), Some(before), "the held state");
    ensure!(
        world
            .stage_state(&id, "code")
            .is_some_and(StageState::running),
        "code no longer runs"
    );
    Ok(())
}

fn a_hold_on_a_running_repeating_stage(_: &()) -> Outcome {
    use super::repeating_stages::{branch, code_begun, group_of, GROUPS};
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    world.tick_groups(&branch("plan"), "plan", GROUPS, 2);
    world.settle(&id);
    let second = World::identity("change", "plan", "code", 2);
    ensure_eq!(
        group_of(&world, &id)?.and_then(|group| group.number),
        Some("2".to_string()),
        "the repetition working"
    );
    let words = "waiting on the user's card about group 3's schema";
    let status = acted(&world.engine.hold(&id, on_stage("code"), words, None))?;
    ensure_eq!(
        hold_of(&status, &on_stage("code")).and_then(|hold| hold.words),
        Some(words.to_string()),
        "the hold the act answers"
    );
    ensure_eq!(
        status.agent.as_ref().map(|agent| agent.identity.clone()),
        Some(second.to_string()),
        "the repetition going on"
    );
    ensure!(
        world
            .stage_state(&id, "code")
            .is_some_and(StageState::running),
        "the repetition no longer runs"
    );
    world.host.advance(10 * 60);
    world.tick_groups(&branch("plan"), "plan", GROUPS, 4);
    let delivered_at = world.host.now();
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::OnHold),
        "code once the repetition on group 2 delivered"
    );
    ensure_eq!(
        world.terminal.log.count("open"),
        3,
        "agents opened: construct's and two repetitions'"
    );
    let status = world.status(&id)?;
    let waiting = status.waiting_on.ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage.as_deref(), waiting.why),
        (Act::Retry, Some("code"), Waits::OnHold),
        "the move"
    );
    ensure_eq!(waiting.words.as_deref(), Some(words), "the move's words");
    ensure_eq!(
        waiting.since,
        Some(delivered_at),
        "since the repetition's end, not code's first start"
    );
    ensure_eq!(
        status.stages[1]
            .group
            .as_ref()
            .map(|group| (group.place, group.of, group.number.clone())),
        Some((3, 3, Some("3".to_string()))),
        "the group the held stage would take"
    );
    let retried = world
        .engine
        .retry(&id, Retry::Stage("code".into()), None, false);
    ensure!(retried.refusal().is_none(), "the retry: {retried:?}");
    ensure!(
        world
            .stage_state(&id, "code")
            .is_some_and(StageState::running),
        "code did not start on the retry"
    );
    let held = world.held(&id).ok_or("the place is gone")?;
    let start = held.stages[1].start.clone().ok_or("no start")?;
    ensure_eq!(
        (start.identity, start.group.and_then(|group| group.number)),
        (
            World::identity("change", "plan", "code", 3),
            Some("3".to_string())
        ),
        "the retry's repetition"
    );
    ensure!(held.holds.is_empty(), "the retry left {:?}", held.holds);
    Ok(())
}

fn holds_refused(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = construct_begun(&world, "plan")?;
    let reason = refused(&world.engine.hold(&id, on_stage("review"), "x", None))?;
    ensure!(reason.contains("review"), "no stage: {reason}");
    acted(&world.engine.hold(&id, on_stage("code"), "first", None))?;
    let reason = refused(&world.engine.hold(&id, on_stage("code"), "second", None))?;
    ensure!(
        reason.contains("code") && reason.contains("already"),
        "a second hold: {reason}"
    );
    let fix = fix_begun(&world, "one")?;
    deliver_fix(&world, &fix);
    world.settle(&fix);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    let reason = refused(&world.engine.hold(&fix, HoldOn::Submission, "x", None))?;
    ensure!(
        reason.contains("submission"),
        "a submission undecided: {reason}"
    );
    let dropped = world.engine.drop_place(&fix, true);
    ensure!(dropped.refusal().is_none(), "the drop: {dropped:?}");
    let reason = refused(&world.engine.hold(&fix, on_stage("code"), "x", None))?;
    ensure!(
        reason.contains("over") || reason.contains("no place"),
        "a place whose work is over: {reason}"
    );
    Ok(())
}

fn a_hold_on_a_stage_waiting_for_room(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(1));
    let one = fix_begun(&world, "one")?;
    let two = fix_begun(&world, "two")?;
    ensure_eq!(
        world.stage_state(&two, "code"),
        Some(StageState::WaitingForRoom),
        "two's code"
    );
    acted(
        &world
            .engine
            .hold(&two, on_stage("code"), "after one lands", None),
    )?;
    ensure_eq!(
        world.stage_state(&two, "code"),
        Some(StageState::OnHold),
        "two's code held"
    );
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&two, "code"),
        Some(StageState::OnHold),
        "two's code once the host has room"
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    let retried = world
        .engine
        .retry(&two, Retry::Stage("code".into()), None, false);
    ensure!(retried.refusal().is_none(), "the retry: {retried:?}");
    ensure!(
        world
            .stage_state(&two, "code")
            .is_some_and(StageState::running),
        "two's code did not start on the retry"
    );
    ensure_eq!(
        world.stage_state(&one, "code"),
        Some(StageState::Lost),
        "one's code"
    );
    Ok(())
}

fn held_at_the_launch_with_words(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let mut launch = launch("change", "plan", Some("plan it"));
    launch.recipients = vec!["ops".into()];
    launch.hold = vec!["code".into()];
    launch
        .hold_words
        .insert("code".into(), "waiting on the schema card".into());
    launched(&world.engine.launch(&launch))?;
    let id = World::id("change", "plan");
    world.settle(&id);
    write_change(&world, "plan");
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::OnHold),
        "code at construct's delivery"
    );
    let waiting = world
        .status(&id)?
        .waiting_on
        .ok_or("the place waits on nobody")?;
    ensure_eq!(
        (
            waiting.stage.as_deref(),
            waiting.why,
            waiting.words.as_deref()
        ),
        (
            Some("code"),
            Waits::OnHold,
            Some("waiting on the schema card")
        ),
        "the move"
    );
    world.host.advance(24 * 60 * 60);
    world.settle(&id);
    ensure!(!world.status(&id)?.stalled, "a held place stalled");
    ensure!(stalls(&world, &id).is_empty(), "a stall recorded");
    ensure!(stall_notices(&world, "ops").is_empty(), "a stall delivered");
    Ok(())
}

fn another_place_lands(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    acted(
        &world
            .engine
            .hold(&id, on_stage("code"), "until fix--one lands", None),
    )?;
    write_change(&world, "plan");
    world.settle(&id);
    let fix = fix_begun(&world, "one")?;
    deliver_fix(&world, &fix);
    world.engine().settle(None);
    world.engine().settle(None);
    ensure!(world.held(&fix).is_none(), "fix--one did not land");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::OnHold),
        "code once fix--one landed"
    );
    ensure!(
        hold_of(&world.status(&id)?, &on_stage("code")).is_some(),
        "the hold ended"
    );
    Ok(())
}

fn a_lost_stage_waits_on_its_caller(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.host.set_recipient(Some("ops"));
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    world.host.advance(20);
    world
        .terminal
        .kill(&World::identity("build", "one", "work", 1));
    let lost_at = world.host.now();
    world.settle(&id);
    let status = world.status(&id)?;
    let waiting = status.waiting_on.ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.stage.as_deref(), waiting.why, waiting.since),
        (Some("work"), Waits::Lost, Some(lost_at)),
        "the move and since when"
    );
    ensure!(!status.stalled, "stalled at once");
    world.host.advance(STALL_AFTER - 1);
    world.settle(&id);
    ensure!(!world.status(&id)?.stalled, "stalled before five minutes");
    world.host.advance(1);
    world.settle(&id);
    ensure!(world.status(&id)?.stalled, "not stalled at five minutes");
    ensure_eq!(
        stall_notices(&world, "ops"),
        vec![world.confirming(&format!(
            "flume: build--one (do the work) stalled: a retry of work [stall/{lost_at}]; read: flume inbox"
        ), "ops")],
        "the stall delivered"
    );
    Ok(())
}

fn stalled_after_five_minutes(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.terminal.add_session("lead", AgentState::Waiting);
    world.engine.settle_every(30);
    let (id, asked) = construct_asks(&world, "plan", &["ops", "lead"], None)?;
    for _ in 0..9 {
        world.host.advance(30);
        world.settle(&id);
    }
    ensure!(
        !world.status(&id)?.stalled,
        "stalled at four and a half minutes"
    );
    world.host.advance(30);
    world.settle(&id);
    let status = world.status(&id)?;
    ensure!(status.stalled, "not stalled at five minutes");
    let waiting = status.waiting_on.ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.stage.as_deref(), waiting.why, waiting.since),
        (Some("construct"), Waits::Answer, Some(asked)),
        "the move stalled on"
    );
    let notice = format!(
        "flume: change--plan (plan it) stalled: a retry of construct [stall/{asked}]; read: flume inbox"
    );
    for _ in 0..3 {
        world.host.advance(30);
        world.settle(&id);
    }
    for recipient in ["ops", "lead"] {
        ensure_eq!(
            stall_notices(&world, recipient),
            vec![world.confirming(&notice, recipient)],
            "the stall notices to {recipient}"
        );
    }
    let stalls = stalls(&world, &id);
    ensure_eq!(stalls.len(), 1, "stalls recorded");
    ensure_eq!(
        stalls[0].of,
        EndOf::Stall {
            since: asked,
            waiting: "a retry of construct".into(),
            answerer: None,
            stop_short: Some(Box::new(StopShortWaited {
                words: Some("Which design does the brief mean?".into()),
                needs: Some(StopKind::NeedsAnswer),
                key: construct_short_key(),
                addressee: None,
            })),
            answerer_read: true,
            lapsed: None,
        },
        "the stall"
    );
    ensure_eq!(
        stalls[0].pending(&world.held(&id).ok_or("no place")?.place, &world.owing()),
        vec!["ops".to_string(), "lead".to_string()],
        "the stall pending for, handed and not confirmed"
    );
    Ok(())
}

fn the_launchs_bound(_: &()) -> Outcome {
    let world = World::new();
    let (id, _) = construct_asks(&world, "plan", &[], Some(30 * 60))?;
    world.host.advance(STALL_AFTER);
    world.settle(&id);
    ensure!(!world.status(&id)?.stalled, "stalled at five minutes");
    world.host.advance(30 * 60 - STALL_AFTER - 1);
    world.settle(&id);
    ensure!(!world.status(&id)?.stalled, "stalled before thirty minutes");
    world.host.advance(1);
    world.settle(&id);
    ensure!(world.status(&id)?.stalled, "not stalled at thirty minutes");
    ensure_eq!(stalls(&world, &id).len(), 1, "stalls recorded");
    Ok(())
}

fn a_held_place_is_not_stalled(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.host.set_recipient(Some("ops"));
    let id = construct_begun(&world, "plan")?;
    let words = "until the user rules on the schema";
    let held_at = world.host.now();
    acted(&world.engine.hold(&id, on_stage("code"), words, None))?;
    write_change(&world, "plan");
    world.settle(&id);
    for _ in 0..24 {
        world.host.advance(60 * 60);
        world.settle(&id);
    }
    let status = world.status(&id)?;
    ensure_eq!(
        hold_of(&status, &on_stage("code")),
        Some(Hold {
            on: on_stage("code"),
            words: Some(words.into()),
            since: held_at,
            bound: None,
            lapsed: None,
        }),
        "the hold and since when"
    );
    ensure!(!status.stalled, "a held place stalled");
    ensure!(stalls(&world, &id).is_empty(), "a stall recorded");
    ensure!(stall_notices(&world, "ops").is_empty(), "a stall delivered");
    Ok(())
}

fn a_long_queue(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.host.set_recipient(Some("ops"));
    world.host.set_bound(Some(1));
    fix_begun(&world, "one")?;
    let two = fix_begun(&world, "two")?;
    for _ in 0..12 {
        world.host.advance(5 * 60);
        world.engine().settle(None);
    }
    ensure_eq!(
        world.stage_state(&two, "code"),
        Some(StageState::WaitingForRoom),
        "two's code"
    );
    let status = match world.engine().status(None) {
        Answer::Status { places, host } => {
            ensure!(
                host.queue.iter().any(|queued| queued.place == two),
                "two is not in the host's queue"
            );
            places
                .into_iter()
                .find(|place| place.place == two)
                .ok_or("no status of two")?
        }
        other => return Err(format!("status answered {other:?}")),
    };
    ensure_eq!(
        status.waiting_on,
        None,
        "the move two waits on its caller for"
    );
    ensure!(!status.stalled, "a place waiting for room stalled");
    ensure!(stalls(&world, &two).is_empty(), "a stall recorded");
    ensure!(stall_notices(&world, "ops").is_empty(), "a stall delivered");
    Ok(())
}

/// A change `plan` launched in `east` for `recipients`, its construct
/// ended short needing an answer, naming the role `to` where given;
/// gives the place and when it asked. The streams `east` and `west` and
/// acme's runs are as [`east_and_west`] declares them (41, 132, 193).
pub fn asks_in_east(
    world: &World,
    recipients: &[&str],
    to: Option<&str>,
) -> Result<(PlaceId, Seconds), String> {
    let mut change = launch("change", "plan", Some("one of two designs"));
    change.stream = Some("east".into());
    change.recipients = recipients.iter().map(|name| name.to_string()).collect();
    launched(&world.engine.launch(&change))?;
    let id = World::id("change", "plan");
    world.settle(&id);
    let construct = World::identity("change", "plan", "construct", 1);
    let asked = world.host.now();
    let answer = world.engine.needs_to(
        &construct.to_string(),
        &secret(world, &id, 0)?.0,
        "Which design does the brief mean?",
        StopKind::NeedsAnswer,
        to,
    );
    ensure!(
        answer.refusal().is_none(),
        "the stop-short was refused: {answer:?}"
    );
    world.settle(&id);
    Ok((id, asked))
}

/// The place's stall bound passed, the place settled, then every place of
/// the host settled twice, so a run the stall woke is up and its notice
/// lands (130, 132).
pub fn stall_and_settle(world: &World, id: &PlaceId) {
    world.host.advance(STALL_AFTER);
    world.settle(id);
    world.engine().settle(None);
    world.engine().settle(None);
}

/// The recipients the place's newest stall was delivered to, in order,
/// each as often as it was.
fn stall_reached(world: &World, id: &PlaceId) -> Result<Vec<String>, String> {
    let stall = stalls(world, id).pop().ok_or("no stall")?;
    let held = world.held(id).ok_or("no place")?;
    let mut reached: Vec<String> = world
        .end_status(&held, &stall)
        .end
        .deliveries
        .into_iter()
        .map(|delivery| delivery.recipient)
        .collect();
    reached.sort();
    Ok(reached)
}

/// The stop-short the place's newest stall carries.
fn stall_asks(world: &World, id: &PlaceId) -> Result<StopShortWaited, String> {
    match stalls(world, id).pop().map(|end| end.of) {
        Some(EndOf::Stall {
            stop_short: Some(stop_short),
            ..
        }) => Ok(*stop_short),
        other => Err(format!("the stall carries no stop-short: {other:?}")),
    }
}

fn the_stall_reaches_the_answerer(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    answerer_behind(&world, "east");
    world.terminal.add_session("ops", AgentState::Waiting);
    world.keep_watch();
    world.host.set_recipient(None);
    world.terminal.set_session(Some("ops"));
    let (id, _) = asks_in_east(&world, &[], None)?;
    world.host.advance(STALL_AFTER);
    world.settle(&id);
    let held = world.held(&id).ok_or("no place")?;
    ensure_eq!(held.place.recipients, vec!["ops".to_string()], "recipients");
    let stall = stalls(&world, &id).pop().ok_or("no stall")?;
    ensure_eq!(
        world
            .end_status(&held, &stall)
            .end
            .deliveries
            .iter()
            .map(|delivery| delivery.recipient.clone())
            .collect::<Vec<_>>(),
        vec!["ops".to_string()],
        "the stall's deliveries"
    );
    ensure_eq!(
        stall.pending(&held.place, &world.owing()),
        vec!["ops".to_string(), "east/operator".to_string()],
        "the stall pending for"
    );
    ensure!(
        world.held(&World::id("turn", "east-operator")).is_some(),
        "no run of east/operator woken"
    );
    let ends = match world.engine.inbox("east/operator", None, true) {
        Answer::Inbox { ends, .. } => ends,
        other => return Err(format!("the inbox answered {other:?}")),
    };
    let collected = ends
        .iter()
        .find(|Collected { place, .. }| place == &id)
        .ok_or("the inbox gives nothing of the place")?;
    ensure_eq!(
        collected.end.end.of.key(),
        stall.of.key(),
        "the end the inbox gives"
    );
    let EndOf::Stall {
        stop_short: Some(stop_short),
        ..
    } = &collected.end.end.of
    else {
        return Err(format!("the stall collected: {:?}", collected.end.end.of));
    };
    ensure_eq!(
        (stop_short.words.as_deref(), stop_short.key.clone()),
        (
            Some("Which design does the brief mean?"),
            construct_short_key()
        ),
        "the stop-short's words and key"
    );
    ensure_eq!(
        stall_notices(&world, "ops").len(),
        1,
        "the stall's notices to ops"
    );
    Ok(())
}

fn the_role_a_question_named(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    answerer_behind(&world, "east");
    let (id, _) = asks_in_east(&world, &["east/planner"], Some("ops"))?;
    stall_and_settle(&world, &id);
    world.engine().settle(None);
    ensure_eq!(
        stall_reached(&world, &id)?,
        vec![
            "east/operator".to_string(),
            "east/ops".to_string(),
            "east/planner".to_string()
        ],
        "the stall's deliveries"
    );
    ensure_eq!(
        stall_asks(&world, &id)?.addressee,
        Some("east/ops".to_string()),
        "the role the stall says was asked"
    );
    Ok(())
}

/// The caller-waits spec's "Another stream's role named", and the
/// end-delivery spec's "A stall of a question for another stream": the
/// stall reaches the place's recipient, the role its stop-short named in
/// another stream and the answerer behind the place's own stream, once
/// each, and not the answerer behind the named role's stream (130, 193).
fn another_streams_role_named(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    answerer_behind(&world, "east");
    answerer_behind(&world, "west");
    world.host.set_role(
        "west/design",
        Some(RoleRun {
            answers: Some("a decision west's design has not made".into()),
            ..super::role_waking::planner_run(None, None)
        }),
    );
    let (id, _) = asks_in_east(&world, &["east/planner"], Some("west/design"))?;
    stall_and_settle(&world, &id);
    world.engine().settle(None);
    ensure_eq!(
        stall_reached(&world, &id)?,
        vec![
            "east/operator".to_string(),
            "east/planner".to_string(),
            "west/design".to_string()
        ],
        "the stall's deliveries"
    );
    ensure!(
        world.held(&World::id("turn", "west-operator")).is_none(),
        "a run of west/operator was woken"
    );
    Ok(())
}

fn the_answerer_is_the_recipient(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    answerer_behind(&world, "east");
    let (id, _) = asks_in_east(&world, &["east/operator"], None)?;
    stall_and_settle(&world, &id);
    ensure_eq!(
        stall_reached(&world, &id)?,
        vec!["east/operator".to_string()],
        "the stall's deliveries"
    );
    ensure_eq!(
        world.terminal.notices("east/operator"),
        vec![
            world.confirming(
                "flume: change--plan (one of two designs) construct needs an answer, its agent kept open for the answer until 1003600 [stage/construct/1/ended-short]; read: flume inbox", "east/operator"
            ),
            world.confirming(
                "flume: change--plan (one of two designs) stalled: a retry of construct [stall/1000000]; read: flume inbox", "east/operator"
            ),
        ],
        "the notices into the answerer's run's pane"
    );
    Ok(())
}

fn a_stall_from_the_earlier_release(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    world.terminal.add_session("ops", AgentState::Waiting);
    let (id, asked) = asks_in_east(&world, &["ops"], None)?;
    world.host.advance(STALL_AFTER);
    world.settle(&id);
    // The held state as an earlier flume wrote it: a place with no
    // recipient, its stall delivered to nobody, naming no answerer and no
    // stop-short.
    let read = world.store.read(&id).ok_or("no place")?;
    let mut json = serde_json::to_value(&read.state).map_err(|error| error.to_string())?;
    json["place"]["recipients"] = serde_json::json!([]);
    json["place"]
        .as_object_mut()
        .ok_or("a place")?
        .remove("token");
    for end in json["ends"].as_array_mut().ok_or("no ends")? {
        if end["of"] == "stall" {
            let end = end.as_object_mut().ok_or("an end")?;
            for field in ["answerer", "stop_short", "answerer_read"] {
                end.remove(field);
            }
            end.insert("deliveries".into(), serde_json::json!([]));
        }
    }
    let earlier: HeldState = serde_json::from_value(json).map_err(|error| error.to_string())?;
    ensure!(
        matches!(
            world.store.write(&id, Some(read.version), Some(&earlier)),
            Written::Written { .. }
        ),
        "the earlier held state was not written"
    );
    answerer_behind(&world, "east");
    world.settle(&id);
    ensure!(
        world.held(&World::id("turn", "east-operator")).is_some(),
        "no run of east/operator woken at the next settle"
    );
    for _ in 0..3 {
        world.host.advance(30);
        world.engine().settle(None);
    }
    let stall = stalls(&world, &id).pop().ok_or("no stall")?;
    let held = world.held(&id).ok_or("no place")?;
    ensure_eq!(
        world
            .end_status(&held, &stall)
            .end
            .deliveries
            .iter()
            .map(|delivery| delivery.recipient.clone())
            .collect::<Vec<_>>(),
        vec!["east/operator".to_string()],
        "the stall's deliveries"
    );
    ensure_eq!(
        stall.of,
        EndOf::Stall {
            since: asked,
            waiting: "a retry of construct".into(),
            answerer: Some("east/operator".into()),
            stop_short: Some(Box::new(StopShortWaited {
                words: Some("Which design does the brief mean?".into()),
                needs: Some(StopKind::NeedsAnswer),
                key: construct_short_key(),
                addressee: None,
            })),
            answerer_read: true,
            lapsed: None,
        },
        "the stall read for its answerer"
    );
    ensure_eq!(stalls(&world, &id).len(), 1, "stalls recorded");
    Ok(())
}

/// The hold on the waiting move a place's status gives.
fn waiting_hold(status: &PlaceStatus) -> Option<Hold> {
    hold_of(status, &HoldOn::Waiting)
}

fn a_role_takes_the_question(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    answerer_behind(&world, "east");
    let (id, _) = asks_in_east(&world, &["east/planner"], None)?;
    world.host.advance(60);
    let held_at = world.host.now();
    let words = "east-planner has it";
    let status = acted(
        &world
            .engine
            .hold(&id, HoldOn::Waiting, words, Some(60 * 60)),
    )?;
    let due = Hold {
        on: HoldOn::Waiting,
        words: Some(words.into()),
        since: held_at,
        bound: Some(60 * 60),
        lapsed: None,
    };
    ensure_eq!(waiting_hold(&status), Some(due.clone()), "the hold acted");
    for _ in 0..10 {
        world.host.advance(5 * 60);
        world.settle(&id);
    }
    let status = world.status(&id)?;
    ensure_eq!(waiting_hold(&status), Some(due), "the hold status gives");
    ensure!(!status.stalled, "the place stalled while it holds");
    ensure!(stalls(&world, &id).is_empty(), "a stall recorded");
    ensure_eq!(
        status
            .waiting_on
            .map(|waiting| (waiting.stage, waiting.why)),
        Some((Some("construct".to_string()), Waits::Answer)),
        "the move held"
    );
    Ok(())
}

fn the_question_passed_on(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    let (id, _) = asks_in_east(&world, &["east/planner"], None)?;
    acted(
        &world
            .engine
            .hold(&id, HoldOn::Waiting, "east-planner has it", Some(60 * 60)),
    )?;
    world.host.advance(20 * 60);
    world.settle(&id);
    let passed_at = world.host.now();
    let words = "passed to east/design";
    acted(
        &world
            .engine
            .hold(&id, HoldOn::Waiting, words, Some(60 * 60)),
    )?;
    let status = world.status(&id)?;
    ensure_eq!(
        status
            .holds
            .iter()
            .filter(|hold| hold.on == HoldOn::Waiting)
            .count(),
        1,
        "holds on the waiting move"
    );
    ensure_eq!(
        waiting_hold(&status),
        Some(Hold {
            on: HoldOn::Waiting,
            words: Some(words.into()),
            since: passed_at,
            bound: Some(60 * 60),
            lapsed: None,
        }),
        "the hold in place of the first"
    );
    world.host.advance(50 * 60);
    world.settle(&id);
    ensure!(
        stalls(&world, &id).is_empty(),
        "stalled at the first hold's bound"
    );
    Ok(())
}

fn the_move_made(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    let (id, _) = asks_in_east(&world, &["east/planner"], None)?;
    acted(
        &world
            .engine
            .hold(&id, HoldOn::Waiting, "east-planner has it", Some(60 * 60)),
    )?;
    let retried = world.engine.retry(
        &id,
        Retry::Stage("construct".into()),
        Some("The first design.".into()),
        false,
    );
    ensure!(retried.refusal().is_none(), "the retry: {retried:?}");
    let status = world.status(&id)?;
    ensure_eq!(waiting_hold(&status), None, "the hold once the retry ran");
    world.settle(&id);
    ensure_eq!(
        waiting_hold(&world.status(&id)?),
        None,
        "the hold once the place settled"
    );
    let other = World::new();
    east_and_west(&other);
    let (dropped, _) = asks_in_east(&other, &["east/planner"], None)?;
    acted(&other.engine.hold(&dropped, HoldOn::Waiting, "mine", None))?;
    other.engine.drop_place(&dropped, true);
    ensure!(
        other
            .held(&dropped)
            .is_none_or(|held| held.holds.iter().all(|hold| hold.on != HoldOn::Waiting)),
        "the hold outlived its dropped place"
    );
    Ok(())
}

fn nothing_waits(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    let before = world.held(&id).ok_or("no place")?;
    let reason = refused(&world.engine.hold(&id, HoldOn::Waiting, "mine", Some(60)))?;
    ensure!(
        reason.contains(&id.0) && reason.contains("waits on no move of its caller's"),
        "the refusal: {reason}"
    );
    let reason = refused(&world.engine.hold(&id, on_stage("code"), "later", Some(60)))?;
    ensure!(
        reason.contains("bound"),
        "the refusal of a stage's bound: {reason}"
    );
    ensure_eq!(world.held(&id), Some(before), "the held state");
    Ok(())
}

fn s98_the_users_question_held_and_lapsed(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    answerer_behind(&world, "east");
    answerer_behind(&world, "west");
    world.host.set_role(
        "west/design",
        Some(RoleRun {
            answers: Some("a decision west's design has not made".into()),
            ..super::role_waking::planner_run(None, None)
        }),
    );
    world.keep_watch();
    let (id, _) = asks_in_east(&world, &["east/planner"], Some("west/design"))?;
    world.engine().settle(None);
    ensure!(
        world.held(&World::id("turn", "west-design")).is_some(),
        "no run of west/design woken"
    );
    world.host.advance(60);
    let held_at = world.host.now();
    let words = "on the user's card req_01K";
    acted(
        &world
            .engine
            .hold(&id, HoldOn::Waiting, words, Some(24 * 60 * 60)),
    )?;
    for _ in 0..23 {
        world.host.advance(60 * 60);
        world.engine().settle(None);
    }
    ensure!(!world.status(&id)?.stalled, "stalled under the hold");
    ensure!(stalls(&world, &id).is_empty(), "a stall under the hold");
    world.host.advance(60 * 60);
    let lapsed_at = world.host.now();
    world.engine().settle(None);
    world.engine().settle(None);
    let status = world.status(&id)?;
    ensure_eq!(
        waiting_hold(&status),
        Some(Hold {
            on: HoldOn::Waiting,
            words: Some(words.into()),
            since: held_at,
            bound: Some(24 * 60 * 60),
            lapsed: Some(lapsed_at),
        }),
        "the hold lapsed, and when, as status gives it"
    );
    ensure!(status.stalled, "status does not say stalled");
    let stall = stalls(&world, &id).pop().ok_or("no stall")?;
    ensure!(
        matches!(&stall.of, EndOf::Stall { since, lapsed: Some(lapsed), .. }
            if *since == lapsed_at && lapsed == words),
        "the stall: {:?}",
        stall.of
    );
    ensure_eq!(
        stall_reached(&world, &id)?,
        vec![
            "east/operator".to_string(),
            "east/planner".to_string(),
            "west/design".to_string()
        ],
        "the stall's deliveries"
    );
    world.host.advance(60);
    acted(&world.engine.hold(
        &id,
        HoldOn::Waiting,
        "east-operator: the user was asked on req_01K",
        Some(24 * 60 * 60),
    ))?;
    for _ in 0..3 {
        world.host.advance(60 * 60);
        world.engine().settle(None);
    }
    ensure_eq!(stalls(&world, &id).len(), 1, "stalls recorded");
    ensure_eq!(
        stall_reached(&world, &id)?.len(),
        3,
        "the stall's deliveries after the second hold"
    );
    ensure!(!world.status(&id)?.stalled, "stalled under the second hold");
    Ok(())
}

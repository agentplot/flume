//! What a place waits on its caller for, recorded in flume and never in
//! a caller's memory (8, 47, 49, 56, 130; core.24): a hold put at the
//! launch or after it, with its words and since when, ended only by a
//! retry naming it; the move a waiting place waits on and since when,
//! read by the settle from the held state alone; and a place waiting
//! under no hold's words past its bound read stalled, which each
//! recipient is delivered once. A hold's event is the caller's to see,
//! and a stage waiting for room waits on the host, never on its caller
//! (70, 72).

use super::settling::{construct_begun, deliver_fix, fix_begun, write_change};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Answer, PlaceStatus, Retry, Waits};
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
            name: "caller waits: a construct ended short needing an answer under a watch is stalled at five minutes, each recipient delivered that once",
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
    let status = acted(&world.engine.hold(&id, on_stage("code"), words))?;
    ensure_eq!(
        hold_of(&status, &on_stage("code")),
        Some(Hold {
            on: on_stage("code"),
            words: Some(words.into()),
            since: held_at,
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
    acted(&world.engine.hold(&id, HoldOn::Submission, words))?;
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
    let reason = refused(&world.engine.hold(&id, on_stage("code"), "wait"))?;
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
    let status = acted(&world.engine.hold(&id, on_stage("code"), words))?;
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
    let reason = refused(&world.engine.hold(&id, on_stage("review"), "x"))?;
    ensure!(reason.contains("review"), "no stage: {reason}");
    acted(&world.engine.hold(&id, on_stage("code"), "first"))?;
    let reason = refused(&world.engine.hold(&id, on_stage("code"), "second"))?;
    ensure!(
        reason.contains("code") && reason.contains("already"),
        "a second hold: {reason}"
    );
    let fix = fix_begun(&world, "one")?;
    deliver_fix(&world, &fix);
    world.settle(&fix);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    let reason = refused(&world.engine.hold(&fix, HoldOn::Submission, "x"))?;
    ensure!(
        reason.contains("submission"),
        "a submission undecided: {reason}"
    );
    let dropped = world.engine.drop_place(&fix, true);
    ensure!(dropped.refusal().is_none(), "the drop: {dropped:?}");
    let reason = refused(&world.engine.hold(&fix, on_stage("code"), "x"))?;
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
    acted(&world.engine.hold(&two, on_stage("code"), "after one lands"))?;
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
            .hold(&id, on_stage("code"), "until fix--one lands"),
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
        vec![
            "flume: build--one stalled: a retry of work; read: flume status build--one".to_string()
        ],
        "the stall delivered"
    );
    Ok(())
}

fn stalled_after_five_minutes(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Working);
    world.terminal.add_session("lead", AgentState::Waiting);
    world.terminal.set_watch(Watch::Bound { seconds: 30 });
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
    let notice =
        "flume: change--plan stalled: a retry of construct; read: flume status change--plan";
    for _ in 0..3 {
        world.host.advance(30);
        world.settle(&id);
    }
    for recipient in ["ops", "lead"] {
        ensure_eq!(
            stall_notices(&world, recipient),
            vec![notice.to_string()],
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
        },
        "the stall"
    );
    ensure!(
        stalls[0].pending(&["ops".into(), "lead".into()]).is_empty(),
        "the stall still pending"
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
    acted(&world.engine.hold(&id, on_stage("code"), words))?;
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

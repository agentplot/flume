//! Each end delivered once to each recipient (7, 38, 43 to 46, 95, 133;
//! core.11, core.25): the settle that reads a stage's end, a stage lost,
//! a submission's outcome records it on the place and sends it as a
//! notice into each recipient's session that asks no question, recorded
//! with to whom and when; a recipient asking, gone or under the printed
//! command leaves it pending; and a tidied place's document stays while
//! an end of it is pending, until it is delivered or the place dropped,
//! a launch of the same name going on beside it (3, 62). A wait by place
//! answers an end pending for its recipient and delivers it, reads one
//! already delivered, and otherwise follows the host's state under its
//! timeout, settling nothing under a watch (119, 122); the inbox collects
//! every end pending for a recipient, each delivered once (75).

use super::settling::deliver_fix;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, EndStatus, WaitFor};
use flume_fakes::Plan;
use flume_seams::*;
use std::sync::{Arc, Mutex};

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "end delivery: a stage's end is delivered as one notice to its recipient and recorded with to whom and when",
            cites: &["43", "46", "95", "7"],
            run: a_delivery_and_its_record,
        },
        Scenario {
            name: "end delivery: a place settled again sends no second notice",
            cites: &["43", "74", "46"],
            run: settled_twice,
        },
        Scenario {
            name: "end delivery: an agent killed with nothing delivered is a lost end, delivered once with what flume read",
            cites: &["43", "45", "S28"],
            run: an_agent_killed,
        },
        Scenario {
            name: "end delivery: a failure and a later landing are each an end delivered once, with the merge queue's words and the commit",
            cites: &["43", "133", "44"],
            run: a_landing_and_a_failure,
        },
        Scenario {
            name: "end delivery: no notice is sent into a recipient asking a question, and the first settle that reads it not asking delivers it",
            cites: &["43", "95", "38"],
            run: a_recipient_asking,
        },
        Scenario {
            name: "end delivery: a recipient no session holds leaves the end pending, and the settle goes on",
            cites: &["43", "95", "46"],
            run: a_recipient_gone,
        },
        Scenario {
            name: "end delivery: a recipient scripted away and back is delivered to once it is back",
            cites: &["43", "95"],
            run: a_recipient_away_and_back,
        },
        Scenario {
            name: "end delivery: under a terminal that cannot send, nothing is sent and status gives every end pending",
            cites: &["43", "38", "119"],
            run: under_the_printed_command,
        },
        Scenario {
            name: "end delivery: a place landing with its recipient away is tidied, its document kept pending until delivered, then removed",
            cites: &["43", "62", "66", "67"],
            run: a_landing_with_its_recipient_away,
        },
        Scenario {
            name: "end delivery: a tidied place dropped while an end is pending is gone with its ends",
            cites: &["43", "62", "52"],
            run: dropped_while_pending,
        },
        Scenario {
            name: "end delivery: a place launched again while a tidied document of its name stays is made, the document's ends pending as they were",
            cites: &["3", "43", "62"],
            run: launched_again,
        },
        Scenario {
            name: "end delivery: a place landing with its recipient away is tidied, and once the recipient collects by the inbox the next settle removes its document",
            cites: &["43", "62", "66"],
            run: collected_after_the_tidy,
        },
        Scenario {
            name: "end delivery: a recipient restarted while waiting on a submission waits again and is answered at once with the landing, delivered by the wait",
            cites: &["43", "77", "133"],
            run: a_restarted_recipient_waits_again,
        },
        Scenario {
            name: "end delivery: a wait that finds nothing pending until its timeout exits still waiting and records nothing delivered",
            cites: &["43"],
            run: the_timeout,
        },
        Scenario {
            name: "end delivery: an end already delivered is answered to a wait as a read, delivering nothing again",
            cites: &["43", "74"],
            run: a_wait_reads_an_end_delivered,
        },
        Scenario {
            name: "end delivery: under a terminal that keeps watch a wait calls no seam but the store, and the place moves on only by the watch's settles",
            cites: &["43", "119", "122", "120"],
            run: a_wait_under_a_watch,
        },
        Scenario {
            name: "end delivery: under a terminal that keeps no watch a wait settles every place before each reading",
            cites: &["43", "119"],
            run: a_wait_attended,
        },
        Scenario {
            name: "end delivery: a wait on what the place has no end of and never will is refused naming why",
            cites: &["43", "2"],
            run: a_wait_refused,
        },
        Scenario {
            name: "end delivery: a restarted recipient collects by the inbox every end pending for it, each delivered once, and status or another's inbox marks nothing",
            cites: &["43", "57"],
            run: collected_after_a_restart,
        },
        Scenario {
            name: "end delivery: two inboxes at once deliver each end once",
            cites: &["43", "75"],
            run: two_inboxes_at_once,
        },
    ]
}

/// A fix named `name` launched for the recipient `ops` and begun.
fn fix_for_ops(world: &World, name: &str) -> Result<PlaceId, String> {
    let mut launch = launch("fix", name, Some("mend it"));
    launch.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&launch))?;
    let id = World::id("fix", name);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    Ok(id)
}

/// The place's ends as status gives them.
fn ends(world: &World, id: &PlaceId) -> Result<Vec<EndStatus>, String> {
    Ok(world.status(id)?.ends)
}

/// The notice a place's end is sent as.
fn notice(name: &str, what: &str) -> String {
    format!("flume: fix--{name} {what}; read: flume status fix--{name}")
}

/// The tidied documents standing on the host.
fn tidied(world: &World) -> Vec<HeldState> {
    world
        .store
        .read_every()
        .into_iter()
        .map(|read| read.state)
        .filter(|held| matches!(held.phase, Phase::Tidied { .. }))
        .collect()
}

fn a_delivery_and_its_record(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![notice("one", "code delivered")],
        "the notices to ops"
    );
    let ends = ends(&world, &id)?;
    let end = ends.first().ok_or("no end recorded")?;
    ensure!(
        matches!(&end.end.of, EndOf::Stage { stage, state: StageState::Delivered, end, .. }
            if stage == "code" && end.head.is_some()),
        "the end: {:?}",
        end.end.of
    );
    ensure_eq!(
        end.end
            .deliveries
            .iter()
            .map(|delivery| (delivery.recipient.as_str(), delivery.at, delivery.by))
            .collect::<Vec<_>>(),
        vec![("ops", world.host.now(), DeliveredBy::Notice)],
        "the delivery recorded"
    );
    ensure!(
        end.pending_for.is_empty(),
        "pending for {:?}",
        end.pending_for
    );
    Ok(())
}

fn settled_twice(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    world.settle(&id);
    world.engine().settle(None);
    ensure_eq!(world.terminal.notices("ops").len(), 1, "notices to ops");
    ensure_eq!(ends(&world, &id)?.len(), 1, "ends recorded");
    Ok(())
}

fn an_agent_killed(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Working);
    let id = fix_for_ops(&world, "one")?;
    let code = World::identity("fix", "one", "code", 1);
    world.agent.set_last_words(&code, "I was cut off");
    world.terminal.kill(&code);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![notice("one", "code lost")],
        "the notices to ops"
    );
    let ends = ends(&world, &id)?;
    ensure_eq!(ends.len(), 1, "ends recorded");
    ensure!(
        matches!(&ends[0].end.of, EndOf::Stage { state: StageState::Lost, end, .. }
            if end.last_words.as_deref() == Some("I was cut off")),
        "the lost end carries no reading: {:?}",
        ends[0].end.of
    );
    Ok(())
}

fn a_landing_and_a_failure(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world
        .queue
        .plan_submits(vec![Plan::Fail("test x failed".into()), Plan::Land]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    let failed = ends(&world, &id)?
        .into_iter()
        .find(|end| matches!(end.end.of, EndOf::Submission { .. }))
        .ok_or("no submission end")?;
    ensure!(
        matches!(&failed.end.of, EndOf::Submission { submit: 1, verdict: Verdict::Failed { words } }
            if words.contains("test x failed")),
        "the failure: {:?}",
        failed.end.of
    );
    ensure_eq!(
        world.stage_state(&id, "code").map(StageState::running),
        Some(true),
        "code running again"
    );
    world.settle(&id);
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place stands");
    let landed = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    let notices = world.terminal.notices("ops");
    for what in [
        "code delivered".to_string(),
        "submission failed".to_string(),
        format!("submission landed {landed}"),
    ] {
        let sent = notices
            .iter()
            .filter(|text| **text == notice("one", &what))
            .count();
        let once = if what == "code delivered" { 2 } else { 1 };
        ensure_eq!(sent, once, "notices of {what}");
    }
    ensure!(
        tidied(&world).is_empty(),
        "a document kept with nothing pending"
    );
    Ok(())
}

fn a_recipient_asking(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Asking);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(
        world.terminal.notices("ops").is_empty(),
        "a notice sent into a question"
    );
    ensure_eq!(
        ends(&world, &id)?
            .first()
            .map(|end| end.pending_for.clone()),
        Some(vec!["ops".to_string()]),
        "pending"
    );
    world.terminal.set_session_state("ops", AgentState::Unknown);
    world.settle(&id);
    ensure!(
        world.terminal.notices("ops").is_empty(),
        "a notice sent into a session read unknown"
    );
    world.terminal.set_session_state("ops", AgentState::Working);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![notice("one", "code delivered")],
        "the notices once ops asks nothing"
    );
    Ok(())
}

fn a_recipient_gone(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent to nobody"
    );
    ensure_eq!(world.queue.log.count("submit"), 1, "the place submitted");
    ensure_eq!(
        ends(&world, &id)?
            .first()
            .map(|end| end.pending_for.clone()),
        Some(vec!["ops".to_string()]),
        "pending"
    );
    Ok(())
}

fn a_recipient_away_and_back(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    world.terminal.remove_session("ops");
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(
        world.terminal.notices("ops").is_empty(),
        "a notice to ops while away"
    );
    world.terminal.add_session("ops", AgentState::Waiting);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![notice("one", "code delivered")],
        "the notices once ops is back"
    );
    Ok(())
}

fn under_the_printed_command(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.cannot_send();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    world.settle(&id);
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent: {:?}",
        world.terminal.every_notice()
    );
    let ends = ends(&world, &id)?;
    ensure!(!ends.is_empty(), "no end recorded");
    for end in &ends {
        ensure_eq!(end.pending_for, vec!["ops".to_string()], "pending");
        ensure!(end.end.deliveries.is_empty(), "a delivery recorded");
    }
    Ok(())
}

/// A fix for `ops` landed while no session is named `ops`, tidied.
fn landed_away(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = fix_for_ops(world, name)?;
    deliver_fix(world, &id);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place's own document");
    ensure!(!world.place.exists(&id.0), "the landed place's worktree");
    Ok(id)
}

fn a_landing_with_its_recipient_away(_: &()) -> Outcome {
    let world = World::new();
    let id = landed_away(&world, "one")?;
    ensure!(
        world.git.branch_head(REPOSITORY, "fix/one").is_none(),
        "the landed place's branch"
    );
    let kept = tidied(&world);
    ensure_eq!(kept.len(), 1, "tidied documents");
    let kept_id = kept[0].place.id.clone();
    let status = world.status(&id)?;
    ensure!(
        matches!(&status.phase, Phase::Tidied { pending_for, .. } if pending_for == &vec!["ops".to_string()]),
        "the tidied place's phase: {:?}",
        status.phase
    );
    ensure!(
        status
            .ends
            .iter()
            .all(|end| end.pending_for == vec!["ops".to_string()]),
        "ends: {:?}",
        status.ends
    );
    world.terminal.add_session("ops", AgentState::Waiting);
    world.engine().settle(None);
    ensure_eq!(
        world.terminal.notices("ops").len(),
        2,
        "the notices once ops is back"
    );
    ensure!(
        world.held(&kept_id).is_none(),
        "the document kept once every end was delivered"
    );
    Ok(())
}

fn dropped_while_pending(_: &()) -> Outcome {
    let world = World::new();
    let id = landed_away(&world, "one")?;
    ensure_eq!(tidied(&world).len(), 1, "tidied documents");
    let answer = world.engine().drop_place(&id, false);
    ensure!(answer.refusal().is_none(), "the drop refused: {answer:?}");
    ensure!(tidied(&world).is_empty(), "the document after its drop");
    ensure!(
        collected(&world.engine().inbox("ops"))?.is_empty(),
        "the inbox gave an end of a dropped place"
    );
    world.terminal.add_session("ops", AgentState::Waiting);
    world.engine().settle(None);
    ensure!(
        world.terminal.notices("ops").is_empty(),
        "an end of a dropped place delivered"
    );
    Ok(())
}

fn launched_again(_: &()) -> Outcome {
    let world = World::new();
    let id = landed_away(&world, "one")?;
    let before = tidied(&world);
    let mut again = launch("fix", "one", Some("mend it again"));
    again.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&again))?;
    ensure_eq!(
        world.held(&id).map(|held| held.phase),
        Some(Phase::Live),
        "the place launched again"
    );
    ensure!(world.place.exists(&id.0), "the place's worktree made again");
    let after = tidied(&world);
    ensure_eq!(after.len(), 1, "tidied documents");
    ensure_eq!(after[0].ends, before[0].ends, "the tidied document's ends");
    ensure!(
        after[0]
            .ends
            .iter()
            .all(|end| end.pending(&after[0].place.recipients) == vec!["ops".to_string()]),
        "the tidied document's ends are not all pending for ops"
    );
    Ok(())
}

/// The ends an inbox's answer collected, by what ended.
fn collected(answer: &Answer) -> Result<Vec<String>, String> {
    match answer {
        Answer::Inbox { ends, .. } => Ok(ends
            .iter()
            .map(|collected| {
                format!(
                    "{}--{} {}",
                    collected.kind,
                    collected.name,
                    flume_core::what_ended(&collected.end.end.of)
                )
            })
            .collect()),
        other => Err(format!("the inbox answered {other:?}")),
    }
}

/// The end a wait answered and whether the wait delivered it; none while
/// still waiting.
fn waited(answer: &Answer) -> Result<Option<(EndStatus, bool)>, String> {
    match answer {
        Answer::Waited { end, delivered, .. } => Ok(end.clone().map(|end| (end, *delivered))),
        other => Err(format!("the wait answered {other:?}")),
    }
}

/// The recipients and the means each delivery of an end went by.
fn deliveries(end: &EndHeld) -> Vec<(String, DeliveredBy)> {
    end.deliveries
        .iter()
        .map(|delivery| (delivery.recipient.clone(), delivery.by))
        .collect()
}

fn collected_after_the_tidy(_: &()) -> Outcome {
    let world = World::new();
    landed_away(&world, "one")?;
    let kept = tidied(&world);
    ensure_eq!(kept.len(), 1, "tidied documents");
    let landed = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    ensure_eq!(
        collected(&world.engine().inbox("ops"))?,
        vec![
            "fix--one code delivered".to_string(),
            format!("fix--one submission landed {landed}"),
        ],
        "what ops collected"
    );
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent to nobody"
    );
    world.engine().settle(None);
    ensure!(
        tidied(&world).is_empty(),
        "the document kept once every end was collected"
    );
    ensure!(
        collected(&world.engine().inbox("ops"))?.is_empty(),
        "a second inbox gave something"
    );
    Ok(())
}

fn a_restarted_recipient_waits_again(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Working);
    let id = fix_for_ops(&world, "one")?;
    let first = world
        .engine()
        .wait(&id, &WaitFor::Submission, "ops", Some(10));
    ensure!(waited(&first)?.is_none(), "the first wait: {first:?}");
    ensure_eq!(first.exit(), 3, "the first wait's exit");
    world.terminal.remove_session("ops");
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place's own document");
    world.terminal.add_session("ops", AgentState::Working);
    world.host.log.clear();
    let again = world
        .engine
        .wait(&id, &WaitFor::Submission, "ops", Some(10));
    ensure_eq!(world.host.log.count("sleep"), 0, "the wait slept");
    let (end, delivered) = waited(&again)?.ok_or("still waiting")?;
    let landed = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    ensure!(
        matches!(&end.end.of, EndOf::Submission { verdict: Verdict::Landed { commit, .. }, .. } if *commit == landed),
        "the end answered: {:?}",
        end.end.of
    );
    ensure!(delivered, "the wait did not deliver it");
    ensure_eq!(again.exit(), 0, "the wait's exit");
    ensure_eq!(
        deliveries(&end.end),
        vec![("ops".to_string(), DeliveredBy::Wait)],
        "the landing's deliveries"
    );
    let status = world.status(&id)?;
    let landing = status
        .ends
        .iter()
        .find(|end| matches!(end.end.of, EndOf::Submission { .. }))
        .ok_or("no landing in status")?;
    ensure!(
        landing.end.delivered_to("ops") && landing.pending_for.is_empty(),
        "the landing as status gives it: {landing:?}"
    );
    Ok(())
}

fn the_timeout(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_for_ops(&world, "one")?;
    let began = world.host.now();
    let answer = world
        .engine()
        .wait(&id, &WaitFor::Stage("code".into()), "ops", Some(60));
    ensure!(waited(&answer)?.is_none(), "the wait: {answer:?}");
    ensure_eq!(answer.exit(), 3, "the wait's exit");
    ensure_eq!(world.host.now() - began, 60, "how long the wait followed");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    let held = world.held(&id).ok_or("no place")?;
    ensure!(
        held.ends.iter().all(|end| end.deliveries.is_empty()),
        "a delivery recorded"
    );
    Ok(())
}

fn a_wait_reads_an_end_delivered(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure_eq!(world.terminal.notices("ops").len(), 1, "notices to ops");
    let answer = world
        .engine()
        .wait(&id, &WaitFor::Stage("code".into()), "ops", Some(10));
    let (end, delivered) = waited(&answer)?.ok_or("still waiting")?;
    ensure!(!delivered, "the wait delivered an end delivered before");
    ensure_eq!(answer.exit(), 0, "the wait's exit");
    ensure_eq!(
        deliveries(&end.end),
        vec![("ops".to_string(), DeliveredBy::Notice)],
        "the deliveries"
    );
    let other = world
        .engine()
        .wait(&id, &WaitFor::Stage("code".into()), "planner", Some(10));
    let (end, delivered) = waited(&other)?.ok_or("still waiting")?;
    ensure!(!delivered, "a wait delivered to a recipient not named");
    ensure_eq!(
        end.end.deliveries.len(),
        1,
        "deliveries after another's wait"
    );
    ensure_eq!(world.terminal.notices("ops").len(), 1, "notices to ops");
    Ok(())
}

/// Every call a seam's fake logged, but the store's.
fn seam_calls(world: &World) -> Vec<String> {
    [
        &world.history.log,
        &world.place.log,
        &world.kinds.log,
        &world.change.log,
        &world.terminal.log,
        &world.agent.log,
        &world.queue.log,
        &world.resolver.log,
    ]
    .iter()
    .flat_map(|log| log.calls())
    .collect()
}

/// Clears every seam's fake's log, the store's among them.
fn clear_calls(world: &World) {
    for log in [
        &world.history.log,
        &world.place.log,
        &world.kinds.log,
        &world.change.log,
        &world.terminal.log,
        &world.agent.log,
        &world.queue.log,
        &world.resolver.log,
        &world.store.log,
    ] {
        log.clear();
    }
}

fn a_wait_under_a_watch(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    world.terminal.set_watch(Watch::Bound { seconds: 30 });
    deliver_fix(&world, &id);
    clear_calls(&world);
    let first = world
        .engine()
        .wait(&id, &WaitFor::Stage("code".into()), "ops", Some(20));
    ensure!(waited(&first)?.is_none(), "the first wait: {first:?}");
    ensure_eq!(
        seam_calls(&world),
        Vec::<String>::new(),
        "calls but the store's"
    );
    ensure!(world.store.log.count("read") > 0, "the store was not read");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code moved without the watch"
    );
    world.settle(&id);
    clear_calls(&world);
    let second = world
        .engine()
        .wait(&id, &WaitFor::Stage("code".into()), "ops", Some(20));
    ensure_eq!(
        seam_calls(&world),
        Vec::<String>::new(),
        "calls but the store's"
    );
    let (end, delivered) = waited(&second)?.ok_or("still waiting")?;
    ensure!(
        delivered
            && matches!(
                end.end.of,
                EndOf::Stage {
                    state: StageState::Delivered,
                    ..
                }
            ),
        "the wait after the watch's settle: {second:?}"
    );
    Ok(())
}

fn a_wait_attended(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    let answer = world
        .engine()
        .wait(&id, &WaitFor::Stage("code".into()), "ops", Some(20));
    let (end, delivered) = waited(&answer)?.ok_or("still waiting")?;
    ensure!(
        delivered
            && matches!(
                end.end.of,
                EndOf::Stage {
                    state: StageState::Delivered,
                    ..
                }
            ),
        "the wait: {answer:?}"
    );
    ensure_eq!(world.queue.log.count("submit"), 1, "the place submitted");
    Ok(())
}

fn a_wait_refused(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_for_ops(&world, "one")?;
    let engine = world.engine();
    let none = engine.wait(
        &World::id("fix", "none"),
        &WaitFor::Submission,
        "ops",
        Some(10),
    );
    ensure!(refused(&none)?.contains("no place"), "{none:?}");
    ensure_eq!(none.exit(), 1, "the refusal's exit");
    let stage = engine.wait(&id, &WaitFor::Stage("review".into()), "ops", Some(10));
    ensure!(refused(&stage)?.contains("no stage review"), "{stage:?}");
    world.begun("notes", None, "two")?;
    let two = World::id("notes", "two");
    let submission = engine.wait(&two, &WaitFor::Submission, "ops", Some(10));
    ensure!(
        refused(&submission)?.contains("does not land"),
        "{submission:?}"
    );
    Ok(())
}

/// Two fixes for `ops`, each with its code delivered and its submission
/// held, while no session is named `ops`.
fn two_pending(world: &World) -> Result<(), String> {
    world.queue.plan_submits(vec![Plan::Hold, Plan::Hold]);
    for name in ["one", "two"] {
        let id = fix_for_ops(world, name)?;
        deliver_fix(world, &id);
        world.settle(&id);
    }
    Ok(())
}

fn collected_after_a_restart(_: &()) -> Outcome {
    let world = World::new();
    two_pending(&world)?;
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent to nobody"
    );
    world.engine().status(None);
    ensure!(
        collected(&world.engine().inbox("planner"))?.is_empty(),
        "another recipient's inbox collected"
    );
    world.terminal.set_watch(Watch::Bound { seconds: 30 });
    world.terminal.add_session("ops", AgentState::Working);
    let answer = world.engine().inbox("ops");
    ensure_eq!(
        collected(&answer)?,
        vec![
            "fix--one code delivered".to_string(),
            "fix--two code delivered".to_string(),
        ],
        "what ops collected"
    );
    let Answer::Inbox { ends, .. } = &answer else {
        return Err("no inbox".into());
    };
    for collected in ends {
        ensure!(
            matches!(&collected.end.end.of, EndOf::Stage { end, .. } if end.head.is_some()),
            "an end not whole: {collected:?}"
        );
        ensure_eq!(
            deliveries(&collected.end.end),
            vec![("ops".to_string(), DeliveredBy::Inbox)],
            "the deliveries"
        );
    }
    for name in ["one", "two"] {
        let status = world.status(&World::id("fix", name))?;
        ensure!(
            status.ends.iter().all(|end| end.pending_for.is_empty()),
            "pending after the inbox: {:?}",
            status.ends
        );
    }
    ensure!(
        collected(&world.engine().inbox("ops"))?.is_empty(),
        "a second inbox gave something"
    );
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent besides the inbox"
    );
    Ok(())
}

fn two_inboxes_at_once(_: &()) -> Outcome {
    let world = World::new();
    two_pending(&world)?;
    world.terminal.set_watch(Watch::Bound { seconds: 30 });
    let other = Arc::new(Mutex::new(None));
    let engine = world.engine();
    let into = other.clone();
    world.store.before_next_write(move || {
        let answer = engine.inbox("ops");
        *into.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(answer);
    });
    let first = collected(&world.engine().inbox("ops"))?;
    let second = other
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .take()
        .ok_or("the second inbox did not run")?;
    let mut both = first.clone();
    both.extend(collected(&second)?);
    both.sort();
    ensure_eq!(
        both,
        vec![
            "fix--one code delivered".to_string(),
            "fix--two code delivered".to_string(),
        ],
        "each end collected once across both inboxes"
    );
    ensure!(
        world
            .store
            .log
            .calls()
            .iter()
            .any(|call| call.ends_with("moved")),
        "the two inboxes did not meet at one write"
    );
    for name in ["one", "two"] {
        let held = world.held(&World::id("fix", name)).ok_or("no place")?;
        for end in &held.ends {
            ensure_eq!(end.deliveries.len(), 1, "deliveries of {name}'s end");
        }
    }
    Ok(())
}

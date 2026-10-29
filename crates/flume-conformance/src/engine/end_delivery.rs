//! Each end delivered once to each recipient (7, 38, 43 to 46, 95, 133;
//! core.11, core.25): the settle that reads a stage's end, a stage lost,
//! a submission's outcome records it on the place and sends it as a
//! notice into each recipient's session that asks no question, recorded
//! with to whom and when; a recipient asking, gone or under the printed
//! command leaves it pending; and a tidied place's document stays while
//! an end of it is pending, until it is delivered or the place dropped,
//! a launch of the same name going on beside it (3, 62).

use super::settling::deliver_fix;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::EndStatus;
use flume_fakes::Plan;
use flume_seams::*;

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
            cites: &["43", "C.5", "38"],
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

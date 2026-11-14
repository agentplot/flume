//! What a recipient takes at its next wait (43, 54, 75, 95, 121, 132;
//! every delivery pending for a recipient handed as one prompt at its
//! wait, record 0069): a recipient's session is typed nothing while it
//! works; at its wait it takes every delivery owed to it and handed to it
//! by nothing, ends and messages alike, in the order recorded, as one
//! prompt, each item with how to confirm it; each is claimed beside the
//! host's deliveries before the send and recorded handed on its answer,
//! so two settles hand it once; a send refused leaves nothing claimed or
//! recorded; and a claim left sending holds its item out of later batches
//! while the inbox still gives it.

use super::messages::from;
use super::role_waking::{configure_at, fix_for, planner_run};
use super::settling::deliver_fix;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::deliveries::{self, rewrite_whole};
use flume_core::{notice_to, Answer};
use flume_fakes::Plan;
use flume_seams::*;
use std::sync::{Arc, Mutex};

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "batch: an end read while its recipient's session works is typed into it never while it works and recorded handed to no one, and the first settle that reads it waiting hands it once, a recipient taking what is pending for it at its wait",
            cites: &["43", "54", "95"],
            run: a_recipient_at_work,
        },
        Scenario {
            name: "batch: two messages and a fix's end for east/ops come while its run's agent works; nothing is typed while it works, and once it goes quiet and its place settles its session takes one prompt holding the two messages and the end in the order recorded, each with its confirm, each recorded handed to the address, a role's deliveries taken as one prompt at its wait",
            cites: &["43", "54", "121", "132"],
            run: two_messages_and_an_end_while_a_role_works,
        },
        Scenario {
            name: "batch: an end handed to madswan/ops by its inbox, then a message for it, gives the next batch to the address the message alone",
            cites: &["43", "53", "132"],
            run: something_handed_before,
        },
        Scenario {
            name: "batch: two settles reading ops's batch at once, the second while the first's send is in flight, hand it once, ops taking one prompt and one hand-over recorded",
            cites: &["43", "75"],
            run: two_settles_at_once,
        },
        Scenario {
            name: "batch: a send of ops's batch the terminal refuses records no hand-over and leaves no claim, and the next settle that reads ops waiting hands the batch",
            cites: &["43", "75", "95"],
            run: a_send_refused,
        },
        Scenario {
            name: "batch: a claim of ops's end left sending, as by a settle ended mid-send, keeps that end out of the next batch, which holds the new end alone, and ops's inbox gives both",
            cites: &["43", "75"],
            run: a_settle_ended_mid_send,
        },
        Scenario {
            name: "batch: a message to madswan/planner, whose standing agent works, is typed into it never while it works, and the host's watch's settle once it waits hands it the message once, recorded handed by message, a role's messages taken at its wait",
            cites: &["43", "53", "132"],
            run: a_message_to_a_standing_agent_at_the_watchs_settle,
        },
    ]
}

/// The items `recipient` was handed in its one prompt, as `owing`
/// records each key's message: each the message and its confirm, in
/// `keys`' order, joined as a batch joins them.
fn batch_of(world: &World, keys: &[String], recipient: &str) -> Result<String, String> {
    let owing = world.owing();
    keys.iter()
        .map(|key| {
            owing
                .deliveries
                .get(key)
                .map(|delivery| notice_to(&delivery.message, key, recipient))
                .ok_or_else(|| format!("no delivery under {key}"))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|items| items.join("\n\n"))
}

/// Each hand-over of the delivery under `key`: to whom and by what.
fn handovers(world: &World, key: &str) -> Vec<(String, DeliveredBy)> {
    world
        .owing()
        .deliveries
        .get(key)
        .map(|delivery| {
            delivery
                .handovers
                .iter()
                .map(|handover| (handover.recipient.clone(), handover.by))
                .collect()
        })
        .unwrap_or_default()
}

/// The claims beside the host's deliveries.
fn claims(world: &World) -> Result<Vec<DeliveryClaim>, String> {
    Ok(deliveries::read_whole(&*world.store)?.claims)
}

/// A message's key, as the message to an address answered it.
fn message_key(answer: &Answer) -> Result<String, String> {
    match answer {
        Answer::Messaged { key, .. } => Ok(key.clone()),
        other => Err(format!("the message answered {other:?}")),
    }
}

/// A fix named `name` for `recipient`, begun and delivered, its
/// submission held, and settled, and the key of its code end's delivery.
fn delivered_for(world: &World, name: &str, recipient: &str) -> Result<(PlaceId, String), String> {
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for(world, name, &[recipient])?;
    deliver_fix(world, &id);
    world.settle(&id);
    let key = world
        .delivery_key(&format!("fix--{name}"), "stage/code/1/delivered")
        .ok_or("no delivery of code's end")?;
    Ok((id, key))
}

/// acme declares the standing agent `agent` for `address`, in the
/// session `madswan` of the stream `madswan` the host declares (132).
fn stands(world: &World, address: &str, agent: &str) {
    world.host.set_stream("madswan", Some(Stream::default()));
    world.host.set_standing(
        address,
        Some(StandingAgent {
            source: "acme".into(),
            agent: agent.into(),
            session: Some("madswan".into()),
            answers: None,
            answerer: false,
        }),
    );
}

fn a_recipient_at_work(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Working);
    let (id, key) = delivered_for(&world, "one", "ops")?;
    world.settle(&id);
    ensure!(
        world.terminal.notices("ops").is_empty(),
        "a notice typed into a session at work"
    );
    ensure_eq!(handovers(&world, &key), vec![], "the hand-overs at work");
    world.terminal.set_session_state("ops", AgentState::Waiting);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![batch_of(&world, std::slice::from_ref(&key), "ops")?],
        "the notices once ops waits"
    );
    ensure_eq!(
        handovers(&world, &key),
        vec![("ops".to_string(), DeliveredBy::Notice)],
        "the hand-overs"
    );
    Ok(())
}

fn two_messages_and_an_end_while_a_role_works(_: &()) -> Outcome {
    let world = World::new();
    let ops = "east/ops";
    configure_at(
        &world,
        ops,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    ensure!(world.engine.wake(ops).refusal().is_none(), "the wake");
    let run = World::id("turn", "east-ops");
    world.settle(&run);
    ensure_eq!(
        world.stage_state(&run, "plan"),
        Some(StageState::Begun),
        "the run's stage"
    );
    let agent = World::identity("turn", "east-ops", "plan", 1);
    world.terminal.set_state(&agent, AgentState::Working);
    let first = message_key(&world.engine.message_address(
        ops,
        "the nightly job failed",
        None,
        &from(Some("ops")),
    ))?;
    let second = message_key(&world.engine.message_address(
        ops,
        "and the weekly one",
        None,
        &from(Some("operator")),
    ))?;
    let (_, end) = delivered_for(&world, "one", ops)?;
    ensure!(
        world.terminal.notices(ops).is_empty(),
        "something typed while the run works: {:?}",
        world.terminal.notices(ops)
    );
    world.terminal.set_state(&agent, AgentState::Waiting);
    world.settle(&run);
    ensure_eq!(
        world.terminal.notices(ops),
        vec![batch_of(
            &world,
            &[first.clone(), second.clone(), end.clone()],
            ops
        )?],
        "the one prompt the run's session took"
    );
    for (key, by) in [
        (&first, DeliveredBy::Message),
        (&second, DeliveredBy::Message),
        (&end, DeliveredBy::Notice),
    ] {
        ensure_eq!(
            handovers(&world, key),
            vec![(ops.to_string(), by)],
            format!("the hand-overs of {key}")
        );
    }
    world.settle(&run);
    ensure_eq!(
        world.terminal.notices(ops).len(),
        1,
        "prompts after a settle"
    );
    Ok(())
}

fn something_handed_before(_: &()) -> Outcome {
    let world = World::new();
    let ops = "madswan/ops";
    stands(&world, ops, "madswan-ops");
    let at = world
        .terminal
        .add_agent(Some("madswan"), "madswan-ops", AgentState::Asking);
    let (id, end) = delivered_for(&world, "one", ops)?;
    world.engine.inbox(ops, None, false);
    ensure_eq!(
        handovers(&world, &end),
        vec![(ops.to_string(), DeliveredBy::Inbox)],
        "the end's hand-overs"
    );
    let message = message_key(&world.engine.message_address(
        ops,
        "look at the queue",
        None,
        &from(Some("operator")),
    ))?;
    world.terminal.set_session_state(&at, AgentState::Waiting);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices(&at),
        vec![batch_of(&world, std::slice::from_ref(&message), ops)?],
        "the next batch to the address"
    );
    Ok(())
}

fn two_settles_at_once(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Asking);
    let (id, key) = delivered_for(&world, "one", "ops")?;
    world.terminal.set_session_state("ops", AgentState::Waiting);
    let engine = world.engine();
    let other = id.clone();
    let ran = Arc::new(Mutex::new(false));
    let other_ran = ran.clone();
    world.terminal.during_next_send(move || {
        drop(engine.settle(Some(&other)));
        *other_ran
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = true;
    });
    world.settle(&id);
    ensure!(
        *ran.lock().unwrap_or_else(|poisoned| poisoned.into_inner()),
        "the other settle did not run while the send was in flight"
    );
    ensure_eq!(world.terminal.notices("ops").len(), 1, "prompts to ops");
    ensure_eq!(
        handovers(&world, &key),
        vec![("ops".to_string(), DeliveredBy::Notice)],
        "the hand-overs"
    );
    ensure_eq!(claims(&world)?, vec![], "the claims left");
    Ok(())
}

fn a_send_refused(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Asking);
    let (id, key) = delivered_for(&world, "one", "ops")?;
    world.terminal.set_session_state("ops", AgentState::Waiting);
    // The send is refused, and ops then asks, so the rest of this settle
    // hands it nothing.
    let terminal = world.terminal.clone();
    world
        .terminal
        .refuse_next_send("the terminal did not answer within its bound", move || {
            terminal.set_session_state("ops", AgentState::Asking)
        });
    world.settle(&id);
    ensure!(world.terminal.notices("ops").is_empty(), "a notice typed");
    ensure_eq!(handovers(&world, &key), vec![], "the hand-overs");
    ensure_eq!(claims(&world)?, vec![], "the claims left");
    world.terminal.set_session_state("ops", AgentState::Waiting);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![batch_of(&world, std::slice::from_ref(&key), "ops")?],
        "the notices at the next settle"
    );
    Ok(())
}

fn a_settle_ended_mid_send(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Asking);
    let (_, first) = delivered_for(&world, "one", "ops")?;
    let at = world.host.now();
    rewrite_whole(&*world.store, |records, claims| {
        let mut next = claims.to_vec();
        next.push(DeliveryClaim {
            key: first.clone(),
            recipient: "ops".into(),
            at,
            sending: true,
            start: None,
        });
        Some((records.to_vec(), next))
    })?;
    let (two, second) = delivered_for(&world, "two", "ops")?;
    world.terminal.set_session_state("ops", AgentState::Waiting);
    world.settle(&two);
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![batch_of(&world, std::slice::from_ref(&second), "ops")?],
        "the next batch"
    );
    ensure_eq!(
        handovers(&world, &first),
        vec![],
        "the first end's hand-overs"
    );
    let collected = match world.engine.inbox("ops", None, false) {
        Answer::Inbox { ends, .. } => ends.len(),
        other => return Err(format!("the inbox answered {other:?}")),
    };
    ensure_eq!(collected, 2, "the ends ops's inbox gave");
    Ok(())
}

fn a_message_to_a_standing_agent_at_the_watchs_settle(_: &()) -> Outcome {
    let world = World::new();
    let planner = "madswan/planner";
    stands(&world, planner, "madswan-planner");
    let at = world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Working);
    let key = message_key(&world.engine.message_address(
        planner,
        "the nightly job failed",
        None,
        &from(Some("ops")),
    ))?;
    world.engine.settle_every(30);
    ensure!(
        world.terminal.notices(&at).is_empty(),
        "something typed while the standing agent works"
    );
    world.terminal.set_session_state(&at, AgentState::Waiting);
    world.engine.settle_every(30);
    world.engine.settle_every(30);
    ensure_eq!(
        world.terminal.notices(&at),
        vec![batch_of(&world, std::slice::from_ref(&key), planner)?],
        "the message handed at the watch's settle"
    );
    ensure_eq!(
        handovers(&world, &key),
        vec![(planner.to_string(), DeliveredBy::Message)],
        "the hand-overs"
    );
    Ok(())
}

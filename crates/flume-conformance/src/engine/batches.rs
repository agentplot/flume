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
use super::open_sessions::stops_short;
use super::role_waking::{configure_at, fix_for, planner_run};
use super::settling::deliver_fix;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::deliveries::{self, rewrite_whole};
use flume_core::message::message_words;
use flume_core::{notice_to, way_to_stop_short, Answer, Engine, Retry};
use flume_fakes::Plan;
use flume_fakes::{FakeAgent, FakeHost};
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
        Scenario {
            name: "batch: a message for east/ops while its run's plan ended short needing an answer, its agent kept open, is typed into its session never and handed to none; once a retry answers the stop-short and the agent goes quiet again, the next settle hands it the message, a delivery never taken for the answer",
            cites: &["42", "53", "132", "198"],
            run: a_message_while_the_run_waits_on_its_answer,
        },
        Scenario {
            name: "batch: a message and a fix's end pending for east/planner when its run's start fills its prompt go in that prompt after the kind's template and before the way to stop short, each with its confirm, a role's run's first prompt carrying its batch",
            cites: &["28", "43", "132"],
            run: the_first_prompt_carries_the_batch,
        },
        Scenario {
            name: "batch: a run's start carrying a batch, read never begun, lets its claims go and records nothing handed, and the next start's prompt carries the same batch",
            cites: &["37", "38", "43", "132"],
            run: a_start_that_never_began,
        },
        Scenario {
            name: "batch: an end for east/planner that comes after its run's prompt was filled and before it was received is handed to none until the run's agent next waits, and then goes in one prompt alone",
            cites: &["37", "38", "43", "132"],
            run: what_came_after_the_prompt,
        },
        Scenario {
            name: "batch: the fake agent of east/ops's run offers deliver; an end for the address comes while it works, and once it goes quiet a settle sends nothing through the terminal and its take answers the end's notice with its confirm, recorded handed to the address by notice, its deliver part taking the place of the terminal",
            cites: &["43", "54", "121", "132"],
            run: a_run_whose_agent_takes_its_batch,
        },
        Scenario {
            name: "message: the fake agent's profile offers deliver and a caller messages its place while it works: nothing goes through the fake terminal, the message waits on the start untaken, and once the agent waits and takes it, it is read received and the message answers acted, the deliver part taking the place of the terminal",
            cites: &["53", "54", "38"],
            run: a_message_taken_by_the_deliver_part,
        },
        Scenario {
            name: "message: the fake agent's profile offers deliver, a caller messages its place with a bound of a minute and the agent works past it: the message is refused with nothing typed, the start holds no message waiting, and a take after the bound gets nothing",
            cites: &["53", "54"],
            run: a_message_the_deliver_part_never_takes,
        },
        Scenario {
            name: "message: the fake agent's profile offers deliver, takes a message and reads it not received within 60 seconds, then takes it again and reads it not received again: the message is refused naming that the agent did not receive the words, taken twice",
            cites: &["38", "53", "54"],
            run: a_message_taken_and_not_received,
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

fn a_message_while_the_run_waits_on_its_answer(_: &()) -> Outcome {
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
    let agent = World::identity("turn", "east-ops", "plan", 1);
    stops_short(
        &world,
        &run,
        0,
        &agent,
        "Which queue?",
        StopKind::NeedsAnswer,
    )?;
    world.settle(&run);
    ensure_eq!(
        world.stage_state(&run, "plan"),
        Some(StageState::EndedShort),
        "the run's plan"
    );
    world.terminal.set_state(&agent, AgentState::Waiting);
    let key = message_key(&world.engine.message_address(
        ops,
        "the nightly job failed",
        None,
        &from(Some("operator")),
    ))?;
    world.settle(&run);
    ensure!(
        world.terminal.notices(ops).is_empty(),
        "something typed while the run waits on its answer"
    );
    ensure_eq!(handovers(&world, &key), vec![], "the message's hand-overs");
    let retried = world.engine.retry(
        &run,
        Retry::Stage("plan".into()),
        Some("the main queue".into()),
        false,
    );
    ensure!(retried.refusal().is_none(), "the retry: {retried:?}");
    ensure_eq!(
        world.stage_state(&run, "plan"),
        Some(StageState::Begun),
        "the run's plan once answered"
    );
    world.terminal.set_state(&agent, AgentState::Waiting);
    world.settle(&run);
    ensure_eq!(
        world.terminal.notices(ops),
        vec![batch_of(&world, std::slice::from_ref(&key), ops)?],
        "the message handed once the run took its answer"
    );
    Ok(())
}

/// The planner's address, which acme declares with a run.
const PLANNER: &str = "east/planner";

/// The prompt the start at `index` of the place `id` was given.
fn prompt_of(world: &World, id: &PlaceId, start: u32) -> String {
    world
        .held(id)
        .and_then(|held| {
            held.stages
                .iter()
                .filter_map(|stage| stage.start.clone())
                .find(|held| held.n == start)
        })
        .map(|start| start.prompt.text)
        .unwrap_or_default()
}

/// A message to `address` recorded beside the host's deliveries, as a
/// message's act records it, waking nothing; its key (214, C.7).
fn recorded_message(world: &World, address: &str, words: &str) -> Result<String, String> {
    let key = format!("message/{address}/0a1b2c3d");
    let at = deliveries::time(world.host.now());
    let message = message_words(address, Some("ops"), words);
    deliveries::update(&*world.store, |records| {
        match weir_delivery::record(records, &key, &message, &[address], &at, 604_800) {
            Ok(weir_delivery::Recorded::Add(added)) => added,
            _ => vec![],
        }
    })?;
    Ok(key)
}

fn the_first_prompt_carries_the_batch(_: &()) -> Outcome {
    let world = World::new();
    // A caller keeps watch, so no act settles every place and wakes the
    // run for the message before the end comes.
    world.keep_watch();
    let template = "Read what is pending for you.";
    configure_at(&world, PLANNER, template, planner_run(None, None));
    let message = recorded_message(&world, PLANNER, "the nightly job failed")?;
    let (_, end) = delivered_for(&world, "one", PLANNER)?;
    let run = World::id("turn", "east-planner");
    let agent = World::identity("turn", "east-planner", "plan", 1);
    let prompt = prompt_of(&world, &run, 1);
    let batch = batch_of(&world, &[message.clone(), end.clone()], PLANNER)?;
    let way = way_to_stop_short(&*world.agent, &agent);
    ensure!(
        prompt.starts_with(&format!("{template}\n\n{batch}\n\n{way}")),
        "the run's first prompt: {prompt}"
    );
    world.settle(&run);
    for (key, by) in [
        (&message, DeliveredBy::Message),
        (&end, DeliveredBy::Notice),
    ] {
        ensure_eq!(
            handovers(&world, key),
            vec![(PLANNER.to_string(), by)],
            format!("the hand-overs of {key}")
        );
    }
    ensure_eq!(claims(&world)?, vec![], "the claims left");
    ensure!(
        world.terminal.notices(PLANNER).is_empty(),
        "a notice into the run's pane"
    );
    Ok(())
}

fn a_start_that_never_began(_: &()) -> Outcome {
    let world = World::new();
    configure_at(
        &world,
        PLANNER,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let first = World::identity("turn", "east-planner", "plan", 1);
    world.agent.receives_at(&first, None);
    let (_, end) = delivered_for(&world, "one", PLANNER)?;
    let run = World::id("turn", "east-planner");
    let item = batch_of(&world, std::slice::from_ref(&end), PLANNER)?;
    ensure!(
        prompt_of(&world, &run, 1).contains(&item),
        "the first start's prompt carries not the end"
    );
    ensure_eq!(claims(&world)?.len(), 1, "the claims of the first start");
    for _ in 0..3 {
        world.host.advance(60);
        world.settle(&run);
    }
    ensure_eq!(
        world.stage_state(&run, "plan"),
        Some(StageState::NeverBegun),
        "the run's first start"
    );
    ensure_eq!(claims(&world)?, vec![], "the claims once it never began");
    ensure_eq!(handovers(&world, &end), vec![], "the end's hand-overs");
    let retried = world.engine.retry(&run, Retry::Current, None, true);
    ensure!(retried.refusal().is_none(), "the retry: {retried:?}");
    ensure!(
        prompt_of(&world, &run, 2).contains(&item),
        "the next start's prompt carries not the end: {}",
        prompt_of(&world, &run, 2)
    );
    Ok(())
}

fn what_came_after_the_prompt(_: &()) -> Outcome {
    let world = World::new();
    configure_at(
        &world,
        PLANNER,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let agent = World::identity("turn", "east-planner", "plan", 1);
    world.agent.set_started(&agent, Reading::No);
    ensure!(world.engine.wake(PLANNER).refusal().is_none(), "the wake");
    let run = World::id("turn", "east-planner");
    world.settle(&run);
    let (one, end) = delivered_for(&world, "one", PLANNER)?;
    world.settle(&one);
    ensure!(
        world.terminal.notices(PLANNER).is_empty(),
        "a notice before the run's prompt was received"
    );
    ensure_eq!(handovers(&world, &end), vec![], "the end's hand-overs");
    world.agent.set_started(&agent, Reading::Yes);
    world.settle(&run);
    ensure_eq!(
        world.stage_state(&run, "plan"),
        Some(StageState::Begun),
        "the run's stage"
    );
    world.settle(&run);
    ensure!(
        !prompt_of(&world, &run, 1).contains(&end),
        "the run's prompt carries the end that came after it"
    );
    ensure_eq!(
        world.terminal.notices(PLANNER),
        vec![batch_of(&world, std::slice::from_ref(&end), PLANNER)?],
        "the end handed at the run's wait"
    );
    Ok(())
}

fn a_run_whose_agent_takes_its_batch(_: &()) -> Outcome {
    let world = World::new();
    world.agent.offer_deliver();
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
    let agent = World::identity("turn", "east-ops", "plan", 1);
    let secret = world
        .held(&run)
        .and_then(|held| held.stages[0].start.clone())
        .map(|start| start.secret.0)
        .ok_or("no start")?;
    world.terminal.set_state(&agent, AgentState::Working);
    let (_, end) = delivered_for(&world, "one", ops)?;
    world.terminal.set_state(&agent, AgentState::Waiting);
    world.settle(&run);
    ensure!(
        world.terminal.notices(ops).is_empty(),
        "a notice through the terminal to a run that takes its own"
    );
    ensure_eq!(
        handovers(&world, &end),
        vec![],
        "the end's hand-overs before the take"
    );
    let refused = world.engine.take(&run, &agent.to_string(), "another");
    ensure!(
        refused.refusal().is_some(),
        "a take under another secret: {refused:?}"
    );
    let taken = world.engine.take(&run, &agent.to_string(), &secret);
    ensure_eq!(
        taken,
        Answer::Taken {
            words: Some(batch_of(&world, std::slice::from_ref(&end), ops)?)
        },
        "the take"
    );
    ensure_eq!(
        handovers(&world, &end),
        vec![(ops.to_string(), DeliveredBy::Notice)],
        "the end's hand-overs"
    );
    ensure_eq!(
        world.engine.take(&run, &agent.to_string(), &secret),
        Answer::Taken { words: None },
        "a second take"
    );
    Ok(())
}

/// The fix `one` of a one-stage kind begun, its agent's profile offering
/// deliver: the place, its agent and its start's secret.
fn begun_delivering(world: &World) -> Result<(PlaceId, Identity, String), String> {
    world.agent.offer_deliver();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    let secret = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .map(|start| start.secret.0)
        .ok_or("no start")?;
    Ok((id, work, secret))
}

/// The messages waiting on the place's first start.
fn waiting_on(world: &World, id: &PlaceId) -> Vec<MessageHeld> {
    world
        .held(id)
        .and_then(|held| held.stages[0].start.clone())
        .map(|start| start.messages)
        .unwrap_or_default()
}

/// A take at each of the host's sleeps from now, as a deliver part asks
/// at each turn's end and idle tick, each words it took kept in `took`
/// and, where `submits`, submitted by the agent's program (215).
fn take_at_each_sleep(
    host: Arc<FakeHost>,
    engine: Arc<Engine>,
    taker: (PlaceId, Identity, String),
    took: Arc<Mutex<Vec<String>>>,
    submits: Option<Arc<FakeAgent>>,
) {
    let next = host.clone();
    host.at_next_sleep(move || {
        let (place, agent, secret) = &taker;
        if let Answer::Taken { words: Some(words) } = engine.take(place, &agent.to_string(), secret)
        {
            if let Some(fake) = &submits {
                fake.submits(agent, &words);
            }
            took.lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .push(words);
        }
        take_at_each_sleep(next, engine, taker, took, submits);
    });
}

/// What the take read while the agent worked: the messages waiting on
/// its start, and what went through the terminal.
type Seen = (Vec<MessageHeld>, Vec<String>);

fn a_message_taken_by_the_deliver_part(_: &()) -> Outcome {
    let world = World::new();
    let (id, work, secret) = begun_delivering(&world)?;
    world.terminal.set_state(&work, AgentState::Working);
    let seen: Arc<Mutex<Option<Seen>>> = Arc::default();
    let (store, terminal, agent, into) = (
        world.store.clone(),
        world.terminal.clone(),
        world.agent.clone(),
        seen.clone(),
    );
    let (place, who, engine) = (id.clone(), work.clone(), world.engine());
    world.host.at_next_sleep(move || {
        let waiting = store
            .held(&place)
            .and_then(|held| held.stages[0].start.clone())
            .map(|start| start.messages)
            .unwrap_or_default();
        *into.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) =
            Some((waiting, terminal.sent_to(&who)));
        terminal.set_state(&who, AgentState::Waiting);
        if let Answer::Taken { words: Some(words) } = engine.take(&place, &who.to_string(), &secret)
        {
            agent.submits(&who, &words);
        }
    });
    let answer = world.engine.message(&id, "use the second", None);
    ensure!(answer.refusal().is_none(), "the message: {answer:?}");
    let (waiting, sent) = seen
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .take()
        .ok_or("the take did not run")?;
    ensure_eq!(
        waiting
            .iter()
            .map(|message| (message.words.as_str(), message.taken_at))
            .collect::<Vec<_>>(),
        vec![("use the second", None)],
        "the messages waiting on the start while the agent worked"
    );
    ensure!(
        !sent.iter().any(|text| text == "use the second"),
        "the words went through the terminal"
    );
    ensure!(
        !world
            .terminal
            .sent_to(&work)
            .iter()
            .any(|text| text == "use the second"),
        "the words went through the terminal"
    );
    ensure_eq!(
        waiting_on(&world, &id),
        vec![],
        "the messages once received"
    );
    Ok(())
}

fn a_message_the_deliver_part_never_takes(_: &()) -> Outcome {
    let world = World::new();
    let (id, work, secret) = begun_delivering(&world)?;
    world.terminal.set_state(&work, AgentState::Working);
    let at = world.host.now();
    let answer = world.engine.message(&id, "later", Some(60));
    let reason = answer.refusal().ok_or("the message was taken")?;
    ensure!(
        reason.contains("within the bound") && reason.contains("nothing was typed"),
        "the refusal: {reason}"
    );
    ensure!(
        world.host.now() - at >= 60,
        "the message gave up before its bound"
    );
    ensure_eq!(waiting_on(&world, &id), vec![], "the messages waiting");
    ensure!(
        world
            .terminal
            .sent_to(&work)
            .iter()
            .all(|text| text != "later"),
        "the words went through the terminal"
    );
    world.terminal.set_state(&work, AgentState::Waiting);
    ensure_eq!(
        world.engine.take(&id, &work.to_string(), &secret),
        Answer::Taken { words: None },
        "a take after the bound"
    );
    Ok(())
}

fn a_message_taken_and_not_received(_: &()) -> Outcome {
    let world = World::new();
    let (id, work, secret) = begun_delivering(&world)?;
    let took: Arc<Mutex<Vec<String>>> = Arc::default();
    take_at_each_sleep(
        world.host.clone(),
        Arc::new(world.engine()),
        (id.clone(), work.clone(), secret),
        took.clone(),
        None,
    );
    let answer = world.engine.message(&id, "use the second", None);
    let reason = answer.refusal().ok_or("the message was taken")?;
    ensure!(
        reason.contains("did not receive the words"),
        "the refusal: {reason}"
    );
    ensure_eq!(
        took.lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone(),
        vec!["use the second".to_string(), "use the second".to_string()],
        "the takes"
    );
    ensure_eq!(waiting_on(&world, &id), vec![], "the messages waiting");
    Ok(())
}

//! A message to a role's address (53, 132, 193; a role's messages wait at
//! its address as deliveries, record 0069): recorded at once in the
//! host's deliveries under `message/<stream>/<role>/<token>`, its one
//! reader the address, its words after a first line naming who sent it,
//! expiring after `[host] ends_expire`; its run woken where its source
//! gives it one and none stands; refused, recording nothing, for an
//! address no source declares and with a bound or an answer's key; and
//! answered at once with its key, the address and what it found (43, 53,
//! 132, 193).

use super::role_waking::{
    address_of, asks, configure, configure_at, fix_for, planner_id, planner_run, runs, PLANNER,
};
use super::settling::deliver_fix;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::deliveries;
use flume_core::{Answer, Answering, Found};
use flume_seams::*;
use weir_delivery::Record;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "message: the pane ops messaging east/planner, whose source gives it a run and none is live, is answered with a key message/east/planner/<token>, the address and its run woken; the host's deliveries hold one delivery under that key, its one reader the address and its message ops messages east/planner: with the words after it, a role's messages waiting at its address; and one run of the address is launched",
            cites: &["53", "132", "193", "43"],
            run: a_message_to_a_role_with_no_run_live,
        },
        Scenario {
            name: "message: a message to east/nobody, which no source declares, is refused naming that no source declares it, and the host's deliveries are unchanged",
            cites: &["53", "132"],
            run: a_message_to_an_address_no_source_declares,
        },
        Scenario {
            name: "message: a message to east/planner with a bound of a minute is refused naming that a bound is how long a message waits for a place's agent, and so is one naming an answer's key, each recording nothing",
            cites: &["42", "53"],
            run: a_bound_with_an_address,
        },
        Scenario {
            name: "message: on a host whose ends_expire is 48h, the pane ops messaging east/planner records one weir-delivery/1 keyed message/east/planner/ and eight hex characters, its readers east/planner alone, its message ops messages east/planner: and the words on the next line, expiring 48 hours after it was recorded, a role's messages waiting at its address",
            cites: &["43", "53", "132"],
            run: a_message_recorded,
        },
        Scenario {
            name: "message: a command that reads no recipient of its own messaging east/planner records a delivery whose message begins A message to east/planner:",
            cites: &["43", "53"],
            run: a_message_from_no_one_the_command_names,
        },
        Scenario {
            name: "message: a message to madswan/planner while no planner run is live or waiting launches one planner run by its own act, and a settle of every place that follows launches none; and a message recorded at another address with no run, handed to nothing, is woken by the next settle of every place, a role's messages waking it as its ends do",
            cites: &["132", "43", "53"],
            run: a_message_wakes_the_role,
        },
        Scenario {
            name: "message: a message pending at east/planner with an end of a fix, flume inbox --recipient east/planner gives the end under ends and the message under messages, with its key, its sender ops and its words, each handed to east/planner by the inbox and pending for it; flume confirm takes the message's key as east/planner, and refuses it as ops, which it is not owed to, and a message key no delivery stands under",
            cites: &["43", "53", "132"],
            run: a_message_collected,
        },
        Scenario {
            name: "message: a message at east/planner handed to the address, confirmed by it, and the host settled is let go of with its records, its weir-known/1 kept, and the log names it let go of with east/planner",
            cites: &["43", "53"],
            run: confirmed_then_let_go_of,
        },
        Scenario {
            name: "message: a message at east/planner past its expiry and never confirmed stays pending once the host is settled, owed to east/planner expired, and status gives it expired under east/planner",
            cites: &["43", "53"],
            run: expired_and_kept,
        },
        Scenario {
            name: "message: status gives east/ops, whose run two working fixes report to and a message from ops is pending at, handed to it by nothing, with its run, messages holding the message's key, ops, when and its first line, handed to none, and reporting naming both fixes; once one lands and is tidied, reporting names the other alone, a role's messages waiting at its address",
            cites: &["56", "7", "43", "132"],
            run: a_roles_messages_and_the_places_reporting_to_it,
        },
        Scenario {
            name: "message: the run of east/ops launches a fix naming no recipient, its environment naming the run's place: the fix's recipients are east/ops from role-run, and once it delivers its end is pending at east/ops with the address's other deliveries, a place a role launches reporting to the role's address",
            cites: &["7", "132", "43"],
            run: a_roles_run_launches_its_project,
        },
    ]
}

/// What a message to an address answered: its key, address, what it
/// found and its run.
fn messaged(answer: &Answer) -> Result<(String, String, Found, Option<PlaceId>), String> {
    match answer {
        Answer::Messaged {
            key,
            address,
            found,
            run,
            ..
        } => Ok((key.clone(), address.clone(), *found, run.clone())),
        other => Err(format!("the message answered {other:?}")),
    }
}

/// A message from the pane `from`, as the sending command's recipient
/// names it; none where it names none.
pub(crate) fn from(by: Option<&str>) -> Answering {
    Answering {
        key: None,
        by: by.map(str::to_string),
    }
}

/// The host's deliveries' records.
pub(crate) fn records(world: &World) -> Result<Vec<Record>, String> {
    deliveries::read(&*world.store).map(|(_, records)| records)
}

/// A message's delivery as the host's deliveries record it: its key,
/// readers, message and expiry.
pub(crate) struct Recorded {
    pub key: String,
    pub readers: Vec<String>,
    pub message: String,
    pub expires: String,
}

/// The deliveries of the host's records keyed as a message's.
pub(crate) fn messages(world: &World) -> Result<Vec<Recorded>, String> {
    Ok(records(world)?
        .into_iter()
        .filter_map(|record| match record {
            Record::Delivery(delivery) if delivery.key().as_str().starts_with("message/") => {
                Some(Recorded {
                    key: delivery.key().to_string(),
                    readers: delivery.readers().iter().map(ToString::to_string).collect(),
                    message: delivery.message().to_string(),
                    expires: delivery.expires_at().as_str().to_string(),
                })
            }
            _ => None,
        })
        .collect())
}

/// Whether `key` is `message/<address>/` and eight hex characters (C.7).
pub(crate) fn keyed_for(key: &str, address: &str) -> bool {
    key.strip_prefix(&format!("message/{address}/"))
        .is_some_and(|token| token.len() == 8 && token.chars().all(|c| c.is_ascii_hexdigit()))
}

fn a_message_to_a_role_with_no_run_live(_: &()) -> Outcome {
    let world = World::new();
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let answer =
        world
            .engine
            .message_address(PLANNER, "the nightly job failed", None, &from(Some("ops")));
    let (key, address, found, run) = messaged(&answer)?;
    ensure!(keyed_for(&key, PLANNER), "the key {key}");
    ensure_eq!(
        (address.as_str(), found, run),
        (PLANNER, Found::Woken, Some(planner_id())),
        "the address and what the message found"
    );
    let messages = messages(&world)?;
    ensure_eq!(messages.len(), 1, "the messages recorded");
    let recorded = &messages[0];
    ensure_eq!(&recorded.key, &key, "the delivery's key");
    ensure_eq!(
        &recorded.readers,
        &vec![PLANNER.to_string()],
        "the delivery's readers"
    );
    ensure_eq!(
        recorded.message.as_str(),
        "ops messages east/planner:\nthe nightly job failed",
        "the delivery's message"
    );
    ensure_eq!(runs(&world), vec![planner_id()], "the runs launched");
    Ok(())
}

fn a_message_to_an_address_no_source_declares(_: &()) -> Outcome {
    let world = World::new();
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let before = records(&world)?;
    let answer = world
        .engine
        .message_address("east/nobody", "words", None, &from(Some("ops")));
    let reason = answer.refusal().ok_or("the message was taken")?;
    ensure!(
        reason.contains("no source of the host declares east/nobody"),
        "the refusal: {reason}"
    );
    ensure_eq!(records(&world)?, before, "the host's deliveries");
    ensure!(runs(&world).is_empty(), "a run was launched");
    Ok(())
}

fn a_bound_with_an_address(_: &()) -> Outcome {
    let world = World::new();
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let answer = world
        .engine
        .message_address(PLANNER, "words", Some(60), &from(Some("ops")));
    let reason = answer
        .refusal()
        .ok_or("the message with a bound was taken")?;
    ensure!(
        reason.contains("a bound is how long a message waits for a place's agent"),
        "the refusal: {reason}"
    );
    let keyed = Answering {
        key: Some(
            "stage/construct/1/ended-short"
                .parse()
                .map_err(|_| "a key")?,
        ),
        by: Some("ops".into()),
    };
    let answer = world.engine.message_address(PLANNER, "words", None, &keyed);
    let reason = answer.refusal().ok_or("the message with a key was taken")?;
    ensure!(
        reason.contains("goes to the place that asked"),
        "the refusal: {reason}"
    );
    ensure_eq!(records(&world)?, vec![], "the host's deliveries");
    ensure!(runs(&world).is_empty(), "a run was launched");
    Ok(())
}

fn a_message_recorded(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_ends_expire(48 * 60 * 60);
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let at = world.host.now();
    let answer =
        world
            .engine
            .message_address(PLANNER, "the nightly job failed", None, &from(Some("ops")));
    messaged(&answer)?;
    let records = records(&world)?;
    let formats: Vec<&str> = records.iter().map(Record::format).collect();
    ensure_eq!(formats, vec![weir_delivery::DELIVERY], "the records");
    let messages = messages(&world)?;
    ensure_eq!(messages.len(), 1, "the messages recorded");
    let recorded = &messages[0];
    ensure!(
        keyed_for(&recorded.key, PLANNER),
        "the key {}",
        recorded.key
    );
    ensure_eq!(&recorded.readers, &vec![PLANNER.to_string()], "its readers");
    ensure_eq!(
        recorded.message.as_str(),
        "ops messages east/planner:\nthe nightly job failed",
        "its message"
    );
    ensure_eq!(
        deliveries::seconds(&recorded.expires),
        Some(at + 48 * 60 * 60),
        "its expiry"
    );
    Ok(())
}

fn a_message_from_no_one_the_command_names(_: &()) -> Outcome {
    let world = World::new();
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    messaged(
        &world
            .engine
            .message_address(PLANNER, "look", None, &from(None)),
    )?;
    let messages = messages(&world)?;
    ensure_eq!(messages.len(), 1, "the messages recorded");
    ensure!(
        messages[0]
            .message
            .starts_with("A message to east/planner:\n"),
        "its message: {}",
        messages[0].message
    );
    Ok(())
}

fn a_message_wakes_the_role(_: &()) -> Outcome {
    let world = World::new();
    let address = "madswan/planner";
    configure_at(
        &world,
        address,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let run = World::id("turn", "madswan-planner");
    let (_, _, found, woken) = messaged(&world.engine.message_address(
        address,
        "look at the nightly job",
        None,
        &from(Some("ops")),
    ))?;
    ensure_eq!(
        (found, woken),
        (Found::Woken, Some(run.clone())),
        "what the message found"
    );
    ensure_eq!(runs(&world), vec![run.clone()], "the runs the message woke");
    world.engine.settle(None);
    ensure_eq!(runs(&world), vec![run.clone()], "the runs after a settle");
    // A message to east/planner recorded with no act's wake after it, as
    // by an act whose wake was refused, is woken by the next settle.
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let at = deliveries::time(world.host.now());
    deliveries::update(&*world.store, |records| {
        match weir_delivery::record(
            records,
            "message/east/planner/0a1b2c3d",
            "A message to east/planner:\nlook",
            &[PLANNER],
            &at,
            604_800,
        ) {
            Ok(weir_delivery::Recorded::Add(added)) => added,
            _ => vec![],
        }
    })?;
    ensure_eq!(
        runs(&world),
        vec![run.clone()],
        "the runs before the settle"
    );
    world.engine.settle(None);
    let mut woken = runs(&world);
    woken.sort();
    let mut due = vec![run, planner_id()];
    due.sort();
    ensure_eq!(woken, due, "the runs the settle woke for the message");
    Ok(())
}

fn a_message_collected(_: &()) -> Outcome {
    let world = World::new();
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let one = fix_for(&world, "one", &[PLANNER])?;
    asks(&world, &one)?;
    let (key, ..) = messaged(&world.engine.message_address(
        PLANNER,
        "the nightly job failed",
        None,
        &from(Some("ops")),
    ))?;
    let (ends, messages) = match world.engine.inbox(PLANNER, None, false) {
        Answer::Inbox { ends, messages, .. } => (ends, messages),
        other => return Err(format!("the inbox answered {other:?}")),
    };
    ensure_eq!(ends.len(), 1, "the ends collected");
    ensure_eq!(
        messages
            .iter()
            .map(|message| (
                message.key.as_str(),
                message.address.as_str(),
                message.from.as_deref(),
                message.words.as_str(),
                message.confirmed
            ))
            .collect::<Vec<_>>(),
        vec![(
            key.as_str(),
            PLANNER,
            Some("ops"),
            "the nightly job failed",
            false
        )],
        "the messages collected"
    );
    let owing = world.owing();
    let delivery = owing.deliveries.get(&key).ok_or("no delivery")?;
    ensure!(
        delivery
            .handovers
            .iter()
            .any(|handover| handover.recipient == PLANNER && handover.by == DeliveredBy::Inbox),
        "the message not handed by the inbox: {:?}",
        delivery.handovers
    );
    ensure!(
        delivery.owed.iter().any(|owed| owed.recipient == PLANNER),
        "the message not pending for the address"
    );
    let as_ops = world.engine.confirm("ops", std::slice::from_ref(&key));
    ensure!(as_ops.refusal().is_some(), "confirmed as ops: {as_ops:?}");
    let none = world
        .engine
        .confirm(PLANNER, &["message/east/planner/ffffffff".to_string()]);
    ensure!(
        none.refusal().is_some(),
        "an unknown key confirmed: {none:?}"
    );
    match world.engine.confirm(PLANNER, std::slice::from_ref(&key)) {
        Answer::Confirmed { confirmed, .. } if confirmed.len() == 1 && !confirmed[0].before => {}
        other => return Err(format!("the confirm answered {other:?}")),
    }
    ensure!(
        world
            .owing()
            .deliveries
            .get(&key)
            .is_some_and(|delivery| delivery.owed.is_empty()),
        "the message still owed once confirmed"
    );
    Ok(())
}

/// acme declares a standing agent for east/planner, so a message to it
/// wakes no run (132).
fn planner_stands(world: &World) {
    world.host.set_stream("east", Some(Stream::default()));
    world.host.set_standing(
        PLANNER,
        Some(StandingAgent {
            source: "acme".into(),
            agent: "east-planner".into(),
            session: Some("east".into()),
            answers: None,
            answerer: false,
        }),
    );
}

fn confirmed_then_let_go_of(_: &()) -> Outcome {
    let world = World::new();
    planner_stands(&world);
    let (key, ..) = messaged(&world.engine.message_address(
        PLANNER,
        "the nightly job failed",
        None,
        &from(Some("ops")),
    ))?;
    world.engine.inbox(PLANNER, None, true);
    world.engine.settle(None);
    ensure!(
        !world.owing().deliveries.contains_key(&key),
        "the confirmed message kept"
    );
    let records = records(&world)?;
    ensure_eq!(
        records.iter().map(Record::format).collect::<Vec<_>>(),
        vec![weir_delivery::KNOWN],
        "the records left"
    );
    ensure!(
        world.host.logged.calls().iter().any(|line| line
            == &format!(
                "let go of the delivery {key}, confirmed by east/planner: its reader confirmed it"
            )),
        "the log: {:?}",
        world.host.logged.calls()
    );
    Ok(())
}

fn expired_and_kept(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_ends_expire(60 * 60);
    planner_stands(&world);
    let (key, ..) = messaged(&world.engine.message_address(
        PLANNER,
        "the nightly job failed",
        None,
        &from(Some("ops")),
    ))?;
    world.host.advance(2 * 60 * 60);
    world.engine.settle(None);
    let owing = world.owing();
    let delivery = owing
        .deliveries
        .get(&key)
        .ok_or("the expired message let go of")?;
    ensure_eq!(
        delivery
            .owed
            .iter()
            .map(|owed| (owed.recipient.as_str(), owed.expired))
            .collect::<Vec<_>>(),
        vec![(PLANNER, true)],
        "whom it is owed to"
    );
    let status = address_of(&world, PLANNER)?;
    ensure_eq!(
        status
            .messages
            .iter()
            .map(|message| (message.key.as_str(), message.expired))
            .collect::<Vec<_>>(),
        vec![(key.as_str(), true)],
        "the message status gives expired under its reader"
    );
    Ok(())
}

fn a_roles_messages_and_the_places_reporting_to_it(_: &()) -> Outcome {
    let world = World::new();
    let ops = "east/ops";
    configure_at(
        &world,
        ops,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let one = fix_for(&world, "one", &[ops])?;
    let two = fix_for(&world, "two", &[ops])?;
    let at = world.host.now();
    let (key, _, _, run) = messaged(&world.engine.message_address(
        ops,
        "the nightly job failed\nsince Tuesday",
        None,
        &from(Some("ops")),
    ))?;
    let status = address_of(&world, ops)?;
    ensure_eq!(status.run, run, "the address's run");
    ensure_eq!(
        status
            .messages
            .iter()
            .map(|message| (
                message.key.as_str(),
                message.from.as_deref(),
                message.at,
                message.line.as_str(),
                message.handovers.len(),
                message.expired
            ))
            .collect::<Vec<_>>(),
        vec![(
            key.as_str(),
            Some("ops"),
            at,
            "the nightly job failed",
            0,
            false
        )],
        "the messages pending at the address"
    );
    let mut reporting = status.reporting.clone();
    reporting.sort();
    ensure_eq!(
        reporting,
        vec![one.clone(), two.clone()],
        "the places reporting"
    );
    deliver_fix(&world, &one);
    world.settle(&one);
    ensure!(
        world
            .held(&one)
            .is_none_or(|held| held.phase != Phase::Live),
        "one still live"
    );
    ensure_eq!(
        address_of(&world, ops)?.reporting,
        vec![two],
        "the places reporting once one landed"
    );
    Ok(())
}

fn a_roles_run_launches_its_project(_: &()) -> Outcome {
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
    // The run's command, whose environment names no recipient and names
    // the run's place.
    world.host.set_recipient(None);
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.from_place = Some(run);
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    let held = world.held(&id).ok_or("no fix")?;
    ensure_eq!(
        (
            held.place.recipients.clone(),
            held.place.recipients_from.clone()
        ),
        (vec![ops.to_string()], Some(RecipientsFrom::RoleRun)),
        "the fix's recipients"
    );
    world.settle(&id);
    deliver_fix(&world, &id);
    world.settle(&id);
    let key = world
        .delivery_key("fix--one", "stage/code/1/delivered")
        .ok_or("no delivery of code's end")?;
    ensure!(
        world
            .owing()
            .deliveries
            .get(&key)
            .is_some_and(|delivery| delivery.owed.iter().any(|owed| owed.recipient == ops)),
        "the fix's end is not pending at east/ops"
    );
    Ok(())
}

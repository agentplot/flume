//! A message to a role's address (53, 132, 193; a role's messages wait at
//! its address as deliveries, record 0069): recorded at once in the
//! host's deliveries under `message/<stream>/<role>/<token>`, its one
//! reader the address, its words after a first line naming who sent it,
//! expiring after `[host] ends_expire`; its run woken where its source
//! gives it one and none stands; refused, recording nothing, for an
//! address no source declares and with a bound or an answer's key; and
//! answered at once with its key, the address and what it found (43, 53,
//! 132, 193).

use super::role_waking::{configure, planner_id, planner_run, runs, PLANNER};
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

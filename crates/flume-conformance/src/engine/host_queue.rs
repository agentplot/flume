//! The host's queue (70, 75): a stage starts only under the host's bound,
//! every caller's agents counted together, and under its stream's (143),
//! and otherwise waits for room; every settle starts what waits by its
//! stream's priority, then its place's, then the order it came to wait,
//! passing over a stage whose stream is full and ending no agent for one
//! (31, 46; core.26); priority is the caller's (7); status gives the
//! host's agents running, bound, room and queue, and each stream's (56,
//! 130); a host with no bound of its own and no stream's bound has no
//! queue, and a stream's bound holds on a host that states none of its
//! own.

use super::account_pools::{account, account_of, pool, spent, stream_on};
use super::settling::{construct_begun, deliver_fix, fix_begun, write_change};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, HostStatus, RoomWanted};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "host queue: a third launch on a host of two is taken and its stage waits for room",
            cites: &["70", "75", "2", "6"],
            run: a_third_launch_on_a_host_of_two,
        },
        Scenario {
            name: "host queue: two settles at once with two stages waiting on a host of two start exactly one",
            cites: &["70", "75", "46", "45"],
            run: two_settles_at_once,
        },
        Scenario {
            name: "host queue: nine changes on a host of four, one raised while it waits, start by priority as each agent ends",
            cites: &["S37", "70", "46", "31", "7"],
            run: s37_nine_units_on_a_host_of_four,
        },
        Scenario {
            name: "host queue: the room a landing frees is taken by the stage waiting, at that settle",
            cites: &["70", "46", "31", "113"],
            run: room_freed_by_a_settle,
        },
        Scenario {
            name: "host queue: a priority raised while its stage runs leaves the agent and orders the place's next start",
            cites: &["7", "70", "56"],
            run: priority_raised_while_running,
        },
        Scenario {
            name: "host queue: a priority given a place whose work is over is refused naming it",
            cites: &["7", "70", "63"],
            run: priority_once_over,
        },
        Scenario {
            name: "host queue: status of a full host gives two running, a bound of two, no room, and the place waiting on the host",
            cites: &["56", "70", "130"],
            run: status_of_a_full_host,
        },
        Scenario {
            name: "host queue: a host with no bound starts every first stage and judges no room",
            cites: &["70", "56"],
            run: a_host_with_no_bound,
        },
        Scenario {
            name: "host queue: a stream's bound holds its stages on a host that states no bound of its own",
            cites: &["70", "143", "46"],
            run: a_streams_bound_on_a_host_with_no_bound,
        },
        Scenario {
            name: "host queue: two streams under a host of six start four work and two personal agents, and the waiting work place starts first when a work agent ends",
            cites: &["S53", "70", "143", "46"],
            run: s53_two_streams_under_the_hosts_bound,
        },
        Scenario {
            name: "host queue: a stream at its bound holds no other stream's room, and its waiting place keeps its turn for its stream's next room",
            cites: &["70", "143", "46", "31"],
            run: a_full_stream_holds_no_others_room,
        },
        Scenario {
            name: "host queue: a stage whose pool has every account spent is passed over for a stage of another pool behind it, and keeps its turn until an account of its pool is usable",
            cites: &["70", "145", "46"],
            run: a_pool_with_nothing_usable,
        },
        Scenario {
            name: "host queue: a stream's priority orders what waits before a place's",
            cites: &["70", "143", "46"],
            run: the_streams_priority_before_the_places,
        },
        Scenario {
            name: "host queue: status of a full host gives each place waiting with its stream, its priority and since when, and each stream's part",
            cites: &["56", "70", "130", "143"],
            run: status_of_a_full_host_with_streams,
        },
        Scenario {
            name: "host queue: a place waiting on its stream's bound while the host has room waits on the host, naming the stream's bound, never on its caller and never stalled",
            cites: &["70", "130", "143", "56"],
            run: waiting_on_a_streams_room,
        },
    ]
}

/// The host's part of status.
fn host(world: &World) -> Result<HostStatus, String> {
    match world.engine().status(None) {
        Answer::Status { host, .. } => Ok(host),
        other => Err(format!("status answered {other:?}")),
    }
}

/// Fixes launched as `names` at `priority`, each settled once.
fn fixes(world: &World, names: &[&str], priority: i64) -> Result<Vec<PlaceId>, String> {
    names
        .iter()
        .map(|name| {
            launched(&world.engine.launch(&Launch {
                priority,
                ..launch("fix", name, Some("x"))
            }))?;
            let id = World::id("fix", name);
            world.settle(&id);
            Ok(id)
        })
        .collect()
}

/// Whether the place's stage runs: its agent started and its end not
/// yet read.
fn runs(world: &World, id: &PlaceId, stage: &str) -> bool {
    world
        .stage_state(id, stage)
        .is_some_and(StageState::running)
}

fn waits(world: &World, id: &PlaceId, stage: &str) -> bool {
    world.stage_state(id, stage) == Some(StageState::WaitingForRoom)
}

fn a_third_launch_on_a_host_of_two(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(2));
    let places = fixes(&world, &["one", "two"], 0)?;
    let third = world.engine.launch(&launch("fix", "three", Some("x")));
    let status = launched(&third)?;
    ensure_eq!(
        status.stages[0].state,
        StageState::WaitingForRoom,
        "the third launch's code stage"
    );
    ensure!(status.agent.is_none(), "the third launch has an agent");
    for id in &places {
        ensure!(runs(&world, id, "code"), "{id}'s code does not run");
    }
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn two_settles_at_once(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(2));
    let places = fixes(&world, &["one", "two", "three", "four"], 0)?;
    ensure!(
        waits(&world, &places[2], "code") && waits(&world, &places[3], "code"),
        "two stages do not wait for room"
    );
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    std::thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| world.engine().settle(None));
        }
    });
    ensure_eq!(
        world.stage_state(&places[0], "code"),
        Some(StageState::Lost),
        "the ended agent's stage"
    );
    ensure_eq!(world.terminal.log.count("open"), 3, "agents opened");
    ensure!(
        runs(&world, &places[2], "code"),
        "the first waiting stage did not start"
    );
    ensure!(
        waits(&world, &places[3], "code"),
        "the second waiting stage does not wait"
    );
    ensure_eq!(host(&world)?.running, 2, "agents running");
    Ok(())
}

fn s37_nine_units_on_a_host_of_four(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(4));
    let names = ["u1", "u2", "u3", "u4", "u5", "u6", "u7", "u8", "u9"];
    let mut places = vec![];
    for name in &names[..4] {
        places.push(construct_begun(&world, name)?);
    }
    for name in &names[4..] {
        launched(
            &world
                .engine
                .launch(&launch("change", name, Some("plan it"))),
        )?;
        places.push(World::id("change", name));
    }
    let queue = host(&world)?;
    ensure_eq!(queue.running, 4, "agents running");
    ensure_eq!(
        queue
            .queue
            .iter()
            .map(|queued| &queued.place)
            .collect::<Vec<_>>(),
        places[4..].iter().collect::<Vec<_>>(),
        "the queue's order"
    );
    let raised = world.engine.priority(&places[8], 1);
    ensure!(
        raised.refusal().is_none(),
        "the priority was refused: {raised:?}"
    );
    ensure_eq!(
        host(&world)?
            .queue
            .first()
            .map(|queued| queued.place.clone()),
        Some(places[8].clone()),
        "the queue's head once raised"
    );
    write_change(&world, "u1");
    world.settle(&places[0]);
    ensure!(
        runs(&world, &places[8], "construct"),
        "the raised unit's construct did not start at the next room"
    );
    ensure!(
        waits(&world, &places[0], "code"),
        "the ended unit's code did not wait behind the units that came first"
    );
    ensure!(
        world.terminal.log.of("close").len() == 1,
        "an agent was ended for room: {:?}",
        world.terminal.log.of("close")
    );
    let order: Vec<(PlaceId, String)> = host(&world)?
        .queue
        .into_iter()
        .map(|queued| (queued.place, queued.stage))
        .collect();
    let due: Vec<(PlaceId, String)> = places[4..8]
        .iter()
        .map(|id| (id.clone(), "construct".to_string()))
        .chain([(places[0].clone(), "code".to_string())])
        .collect();
    ensure_eq!(order, due, "the queue after the room was taken");
    Ok(())
}

fn room_freed_by_a_settle(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(1));
    let places = fixes(&world, &["one", "two"], 0)?;
    ensure!(
        waits(&world, &places[1], "code"),
        "the second fix does not wait"
    );
    deliver_fix(&world, &places[0]);
    world.settle(&places[0]);
    ensure!(world.held(&places[0]).is_none(), "the landed place is kept");
    ensure!(
        runs(&world, &places[1], "code"),
        "the waiting fix did not start"
    );
    ensure_eq!(
        world.terminal.log.of("close"),
        vec![format!(
            "close {}",
            World::identity("fix", "one", "code", 1)
        )],
        "agents ended"
    );
    Ok(())
}

fn priority_raised_while_running(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(1));
    let first = construct_begun(&world, "first")?;
    for name in ["second", "third"] {
        launched(
            &world
                .engine
                .launch(&launch("change", name, Some("plan it"))),
        )?;
    }
    let opened = world.terminal.log.count("open");
    let raised = world.engine.priority(&first, 5);
    ensure!(
        raised.refusal().is_none(),
        "the priority was refused: {raised:?}"
    );
    ensure!(
        runs(&world, &first, "construct"),
        "the running agent changed"
    );
    ensure_eq!(world.terminal.log.count("close"), 0, "agents ended");
    ensure_eq!(world.terminal.log.count("open"), opened, "agents opened");
    ensure_eq!(
        world.status(&first)?.priority,
        5,
        "the priority status gives"
    );
    write_change(&world, "first");
    world.settle(&first);
    ensure!(
        runs(&world, &first, "code"),
        "the raised place's code did not start ahead of the places waiting"
    );
    for name in ["second", "third"] {
        ensure!(
            waits(&world, &World::id("change", name), "construct"),
            "{name} does not wait"
        );
    }
    Ok(())
}

fn priority_once_over(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_begun(&world, "one")?;
    let held = world.held(&id).ok_or("no place")?;
    world.place.set_holds(
        &held.place.location,
        Holds {
            untracked: vec!["notes.txt".into()],
            ..Holds::default()
        },
    );
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(
        matches!(
            world.held(&id).map(|held| held.phase),
            Some(Phase::Over { .. })
        ),
        "the place's work is not over"
    );
    let reason = refused(&world.engine.priority(&id, 3))?;
    ensure!(
        reason.contains(&id.0) && reason.contains("over"),
        "the refusal does not name the place over: {reason}"
    );
    ensure_eq!(
        world.held(&id).map(|held| held.priority),
        Some(0),
        "the priority"
    );
    Ok(())
}

fn status_of_a_full_host(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(2));
    fixes(&world, &["one", "two"], 0)?;
    let since = world.host.now();
    let waiting = fixes(&world, &["three"], 3)?.remove(0);
    world.host.advance(30);
    let host = host(&world)?;
    ensure_eq!(host.running, 2, "agents running");
    ensure_eq!(host.bound, Some(2), "the bound");
    ensure_eq!(host.room, Some(0), "the room");
    ensure_eq!(host.queue.len(), 1, "places waiting");
    let queued = &host.queue[0];
    ensure_eq!(queued.place, waiting, "the place waiting");
    ensure_eq!(queued.stage, "code", "its stage");
    ensure_eq!(queued.priority, 3, "its priority");
    ensure_eq!(queued.since, Some(since), "since when it waits");
    let status = world.status(&waiting)?;
    ensure_eq!(status.priority, 3, "the place's priority");
    ensure!(
        status.waiting_on.is_none(),
        "the place waiting for room waits on its caller: {:?}",
        status.waiting_on
    );
    Ok(())
}

fn a_host_with_no_bound(_: &()) -> Outcome {
    let world = World::new();
    let places = fixes(&world, &["a", "b", "c", "d", "e"], 0)?;
    for id in &places {
        ensure!(runs(&world, id, "code"), "{id}'s code does not run");
    }
    let host = host(&world)?;
    ensure_eq!(host.running, 5, "agents running");
    ensure_eq!(host.bound, None, "the bound");
    ensure_eq!(host.room, None, "the room");
    ensure!(host.queue.is_empty(), "a host with no bound has a queue");
    Ok(())
}

/// The host declares `name` with `agents` as its bound and `priority`.
fn declare(world: &World, name: &str, agents: Option<u32>, priority: i64) {
    world.host.set_stream(
        name,
        Some(Stream {
            agents,
            priority,
            pool: None,
        }),
    );
}

/// Fixes launched as `names` in `stream` at `priority`, each settled
/// once.
fn fixes_in(
    world: &World,
    stream: &str,
    names: &[&str],
    priority: i64,
) -> Result<Vec<PlaceId>, String> {
    names
        .iter()
        .map(|name| {
            launched(&world.engine.launch(&Launch {
                priority,
                stream: Some(stream.into()),
                ..launch("fix", name, Some("x"))
            }))?;
            let id = World::id("fix", name);
            world.settle(&id);
            Ok(id)
        })
        .collect()
}

/// Ends the fix `name`'s code agent and settles the host once.
fn end_code(world: &World, name: &str) {
    world
        .terminal
        .kill(&World::identity("fix", name, "code", 1));
    world.engine().settle(None);
}

/// The places of the host's queue, in the order they would start.
fn queue_order(world: &World) -> Result<Vec<PlaceId>, String> {
    Ok(host(world)?
        .queue
        .into_iter()
        .map(|queued| queued.place)
        .collect())
}

fn s53_two_streams_under_the_hosts_bound(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(6));
    declare(&world, "work", Some(4), 10);
    declare(&world, "personal", Some(3), 0);
    let work = fixes_in(&world, "work", &["w1", "w2", "w3", "w4", "w5"], 0)?;
    let personal = fixes_in(&world, "personal", &["p1", "p2", "p3"], 0)?;
    for id in work[..4].iter().chain(&personal[..2]) {
        ensure!(runs(&world, id, "code"), "{id}'s code does not run");
    }
    ensure!(waits(&world, &work[4], "code"), "w5 does not wait");
    ensure!(waits(&world, &personal[2], "code"), "p3 does not wait");
    let wants: Vec<(PlaceId, RoomWanted)> = host(&world)?
        .queue
        .into_iter()
        .map(|queued| (queued.place, queued.wants))
        .collect();
    ensure_eq!(
        wants,
        vec![
            (
                work[4].clone(),
                RoomWanted::Stream {
                    stream: "work".into(),
                    bound: 4
                }
            ),
            (personal[2].clone(), RoomWanted::Host),
        ],
        "what each waiting place is short of, in the order they would start"
    );
    end_code(&world, "w1");
    ensure!(
        runs(&world, &work[4], "code"),
        "the waiting work place did not start first"
    );
    ensure!(
        waits(&world, &personal[2], "code"),
        "the personal place does not wait"
    );
    ensure_eq!(host(&world)?.running, 6, "agents running");
    Ok(())
}

fn a_full_stream_holds_no_others_room(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(2));
    declare(&world, "work", Some(1), 10);
    declare(&world, "personal", None, 0);
    fixes_in(&world, "work", &["w1"], 0)?;
    let w2 = fixes_in(&world, "work", &["w2"], 0)?.remove(0);
    let p1 = fixes_in(&world, "personal", &["p1"], 0)?.remove(0);
    ensure!(
        waits(&world, &w2, "code"),
        "w2 does not wait for its stream"
    );
    ensure!(
        runs(&world, &p1, "code"),
        "the personal place was held behind the full stream"
    );
    let p2 = fixes_in(&world, "personal", &["p2"], 0)?.remove(0);
    ensure!(waits(&world, &p2, "code"), "p2 does not wait for the host");
    end_code(&world, "p1");
    ensure!(
        runs(&world, &p2, "code"),
        "the room p1 left was held for the full stream"
    );
    ensure!(waits(&world, &w2, "code"), "w2 does not wait");
    ensure_eq!(
        queue_order(&world)?,
        vec![w2.clone()],
        "the work place keeps its turn"
    );
    end_code(&world, "w1");
    ensure!(
        runs(&world, &w2, "code"),
        "the work place did not start at its stream's next room"
    );
    ensure_eq!(
        world.terminal.log.count("close"),
        0,
        "agents ended for room"
    );
    ensure!(runs(&world, &p2, "code"), "p2's agent was ended");
    Ok(())
}

fn the_streams_priority_before_the_places(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(1));
    declare(&world, "work", None, 10);
    declare(&world, "personal", None, 0);
    fixes_in(&world, "personal", &["p0"], 0)?;
    let p1 = fixes_in(&world, "personal", &["p1"], 50)?.remove(0);
    let w1 = fixes_in(&world, "work", &["w1"], 0)?.remove(0);
    ensure_eq!(
        queue_order(&world)?,
        vec![w1.clone(), p1.clone()],
        "the queue's order"
    );
    end_code(&world, "p0");
    ensure!(
        runs(&world, &w1, "code"),
        "the work place did not start ahead of the personal place"
    );
    ensure!(
        waits(&world, &p1, "code"),
        "the personal place does not wait"
    );
    Ok(())
}

fn status_of_a_full_host_with_streams(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(2));
    declare(&world, "work", None, 10);
    fixes(&world, &["one", "two"], 0)?;
    let since = world.host.now();
    let waiting = fixes_in(&world, "work", &["three"], 3)?.remove(0);
    world.host.advance(30);
    let host = host(&world)?;
    ensure_eq!(
        (host.running, host.bound, host.room),
        (2, Some(2), Some(0)),
        "the host's agents running, bound and room"
    );
    ensure_eq!(host.queue.len(), 1, "places waiting");
    let queued = &host.queue[0];
    ensure_eq!(
        (
            &queued.place,
            queued.stage.as_str(),
            queued.stream.as_str(),
            queued.priority,
            queued.since,
            &queued.wants
        ),
        (&waiting, "code", "work", 3, Some(since), &RoomWanted::Host),
        "the place waiting"
    );
    /// A stream's name, agents running, bound, room and stages waiting.
    type Part = (String, u32, Option<u32>, Option<u32>, usize);
    let streams: Vec<Part> = host
        .streams
        .iter()
        .map(|stream| {
            (
                stream.name.clone(),
                stream.running,
                stream.bound,
                stream.room,
                stream.queue.len(),
            )
        })
        .collect();
    ensure_eq!(
        streams,
        vec![
            ("work".to_string(), 0, None, None, 1),
            (DEFAULT_STREAM.to_string(), 2, None, None, 0),
        ],
        "each stream's part, the higher priority first"
    );
    ensure!(
        world.status(&waiting)?.waiting_on.is_none(),
        "the place waiting for room waits on its caller"
    );
    Ok(())
}

fn waiting_on_a_streams_room(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(4));
    declare(&world, "work", Some(1), 0);
    fixes_in(&world, "work", &["w1"], 0)?;
    let w2 = fixes_in(&world, "work", &["w2"], 0)?.remove(0);
    ensure!(waits(&world, &w2, "code"), "w2 does not wait");
    world.host.advance(600);
    world.engine().settle(None);
    ensure!(
        waits(&world, &w2, "code"),
        "w2 started past its stream's bound"
    );
    let host = host(&world)?;
    ensure_eq!(host.room, Some(3), "the host's room");
    ensure_eq!(
        host.queue.first().map(|queued| queued.wants.clone()),
        Some(RoomWanted::Stream {
            stream: "work".into(),
            bound: 1
        }),
        "what the place waits on"
    );
    let status = world.status(&w2)?;
    ensure!(
        status.waiting_on.is_none(),
        "the place waits on its caller: {:?}",
        status.waiting_on
    );
    ensure!(!status.stalled, "the place waiting for room is stalled");
    Ok(())
}

fn a_streams_bound_on_a_host_with_no_bound(_: &()) -> Outcome {
    let world = World::new();
    declare(&world, "work", Some(1), 0);
    let w1 = fixes_in(&world, "work", &["w1"], 0)?.remove(0);
    let w2 = fixes_in(&world, "work", &["w2"], 0)?.remove(0);
    let d1 = fixes(&world, &["d1"], 0)?.remove(0);
    ensure!(runs(&world, &w1, "code"), "w1's code does not run");
    ensure!(
        runs(&world, &d1, "code"),
        "the default place's code does not run"
    );
    ensure!(
        waits(&world, &w2, "code"),
        "w2 started past its stream's bound on a host with none of its own"
    );
    let host = host(&world)?;
    ensure_eq!(
        (host.bound, host.room),
        (None, None),
        "the host's bound and room"
    );
    ensure_eq!(
        host.queue
            .iter()
            .map(|queued| (queued.place.clone(), queued.wants.clone()))
            .collect::<Vec<_>>(),
        vec![(
            w2.clone(),
            RoomWanted::Stream {
                stream: "work".into(),
                bound: 1
            }
        )],
        "the queue"
    );
    end_code(&world, "w1");
    ensure!(
        runs(&world, &w2, "code"),
        "w2 did not start at its stream's next room"
    );
    Ok(())
}

fn a_pool_with_nothing_usable(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(2));
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("work-1", None)],
    );
    pool(
        &world,
        "personal",
        Strategy::FillFirst,
        vec![account("personal-1", None)],
    );
    stream_on(&world, "work", Some("work"), 10);
    stream_on(&world, "personal", Some("personal"), 0);
    let until = world.host.now() + 600;
    spent(&world, "work", "work-1", until);
    let w1 = fixes_in(&world, "work", &["w1"], 0)?.remove(0);
    let p1 = fixes_in(&world, "personal", &["p1"], 0)?.remove(0);
    ensure!(
        waits(&world, &w1, "code"),
        "w1 does not wait for an account"
    );
    ensure!(
        runs(&world, &p1, "code"),
        "the stage behind was held behind the pool with nothing usable"
    );
    let p2 = fixes_in(&world, "personal", &["p2"], 0)?.remove(0);
    ensure!(runs(&world, &p2, "code"), "p2 was held for w1");
    ensure_eq!(queue_order(&world)?, vec![w1.clone()], "the queue");
    world.host.advance(601);
    end_code(&world, "p2");
    ensure!(
        runs(&world, &w1, "code"),
        "w1 did not take its turn once its pool's account was usable"
    );
    ensure_eq!(
        account_of(&world, &w1, "code"),
        Some(("work".to_string(), "work-1".to_string())),
        "w1's account"
    );
    Ok(())
}

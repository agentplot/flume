//! A place's label (180; core.46): the one the host's configuration
//! gives its repository's kind, else its kind, else its kind's own, else
//! none, read at each start and held on it, so an agent counts toward the
//! label it started under until it ends (32). Status gives each place's
//! label as it would resolve now, with where it came from (56). Within a
//! stream that states a bound, a stage of a label running fewer than its
//! reserve starts first, whatever the priorities, and a stage starts only
//! while its stream's room passes what the stream's other labels keep
//! free and do not use; no agent is ended for a reserve (31, 70, 181). A
//! stream whose reserves pass its bound, whose label keeps more free than
//! its reserve, or whose label's table holds another key starts no stage,
//! each refused naming why (4, 181, 182).

use super::settling::{construct_begun, write_change};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, HostStatus, LabelStatus, Reserves, Retry, RoomWanted};
use flume_fakes::place_location;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "labels: a place's label is the host's for its repository's kind, else for its kind, else its kind's own, each with where it came from",
            cites: &["S85", "180", "56"],
            run: each_source_of_a_label,
        },
        Scenario {
            name: "labels: a label changed between starts leaves the start that ran under the label it started with and the next start under the new one",
            cites: &["32", "180"],
            run: a_label_changed_between_starts,
        },
        Scenario {
            name: "labels: a reserve is lent while its label runs none and taken back by the next start, and a slot it keeps free holds a place of no label",
            cites: &["S84", "70", "180", "181", "31"],
            run: s84_a_reserve_lent_and_a_slot_kept_free,
        },
        Scenario {
            name: "labels: one settle orders what waits again after each start, a label at its reserve going back among its stream's others",
            cites: &["70", "181", "46"],
            run: one_settle_ranks_again,
        },
        Scenario {
            name: "labels: a stage below its label's reserve waits on the host's room where the host has none",
            cites: &["70", "181"],
            run: a_reserve_where_the_host_has_no_room,
        },
        Scenario {
            name: "labels: a stream that states no bound holds no reserve, and its stages start in today's order",
            cites: &["70", "181"],
            run: a_stream_with_no_bound,
        },
        Scenario {
            name: "labels: a reserve the local layer sets is held from the next start",
            cites: &["181", "S85"],
            run: a_reserve_set_locally,
        },
        Scenario {
            name: "labels: a key but reserve and keep_free in a label's table refuses each start of its stream, naming the key",
            cites: &["182", "4", "S85"],
            run: a_pool_in_a_labels_table,
        },
        Scenario {
            name: "labels: reserves past their stream's bound refuse each start of the stream, naming the reserves and the bound",
            cites: &["181", "4"],
            run: reserves_past_the_bound,
        },
        Scenario {
            name: "labels: a label keeping more free than its reserve refuses each start of its stream, naming both",
            cites: &["181", "4"],
            run: more_kept_free_than_owed,
        },
        Scenario {
            name: "labels: a start refused for its stream's labels starts once the configuration is mended and its stage retried",
            cites: &["181", "4"],
            run: the_configuration_mended,
        },
        Scenario {
            name: "labels: status gives each place's label with where it came from, and each stream's labels with their reserves, what they keep free, their agents running and their stages waiting, in the JSON form",
            cites: &["S85", "56", "180", "181"],
            run: the_labels_in_status,
        },
        Scenario {
            name: "labels: a place waiting on a slot another label keeps free wants that room, in the JSON form and in words, and waits on the host, never on its caller",
            cites: &["56", "70", "181"],
            run: waiting_on_a_slot_kept_free,
        },
    ]
}

/// The repository's kinds `build`, labelled `construction`, `deploy`,
/// labelled `operation`, and `research`, of no label, each one stage,
/// `work`, landing nothing.
pub fn labelled_kinds(world: &World) {
    world.kinds.set_kinds(
        REPOSITORY,
        vec![
            work_kind("build"),
            work_kind("deploy"),
            work_kind("research"),
        ],
    );
    world.host.name_label(None, "build", Some("construction"));
    world.host.name_label(None, "deploy", Some("operation"));
}

/// The host declares `stream` with `agents`, and each label's share of
/// it, `(label, reserve, keep_free)`.
pub fn shared(world: &World, stream: &str, agents: Option<u32>, shares: &[(&str, u32, u32)]) {
    world.host.set_stream(
        stream,
        Some(Stream {
            agents,
            labels: shares
                .iter()
                .map(|(label, reserve, keep_free)| {
                    (
                        (*label).to_string(),
                        Share {
                            reserve: *reserve,
                            keep_free: *keep_free,
                            unknown: vec![],
                        },
                    )
                })
                .collect(),
            ..Stream::default()
        }),
    );
}

/// A place of `kind` launched as `name` in `stream` at `priority`, and
/// settled once.
pub fn placed(
    world: &World,
    stream: &str,
    kind: &str,
    name: &str,
    priority: i64,
) -> Result<PlaceId, String> {
    launched(&world.engine.launch(&Launch {
        priority,
        stream: Some(stream.into()),
        ..launch(kind, name, Some("x"))
    }))?;
    let id = World::id(kind, name);
    world.settle(&id);
    Ok(id)
}

/// Whether the place's one stage runs.
pub fn running(world: &World, id: &PlaceId) -> bool {
    world
        .stage_state(id, "work")
        .is_some_and(StageState::running)
}

/// Whether the place's one stage waits for room.
pub fn waiting(world: &World, id: &PlaceId) -> bool {
    world.stage_state(id, "work") == Some(StageState::WaitingForRoom)
}

/// Ends the agent of the place `kind--name` and settles the host once.
pub fn end(world: &World, kind: &str, name: &str) {
    world.terminal.kill(&World::identity(kind, name, "work", 1));
    world.engine().settle(None);
}

/// The host's part of status.
fn host(world: &World) -> Result<HostStatus, String> {
    match world.engine().status(None) {
        Answer::Status { host, .. } => Ok(host),
        other => Err(format!("status answered {other:?}")),
    }
}

/// What the place waits for in the host's queue, none where it is not
/// there.
fn wants(world: &World, id: &PlaceId) -> Result<Option<RoomWanted>, String> {
    Ok(host(world)?
        .queue
        .into_iter()
        .find(|queued| &queued.place == id)
        .map(|queued| queued.wants))
}

/// The words the place's one stage was refused with.
fn refused_for(world: &World, id: &PlaceId) -> Result<String, String> {
    let status = world.status(id)?;
    ensure_eq!(
        status.stages[0].state,
        StageState::Refused,
        "the stage refused"
    );
    status.stages[0]
        .refused_setting
        .clone()
        .ok_or_else(|| "the refusal gives no words".to_string())
}

/// The second repository, which declares `deploy` as the first does.
const SECOND: &str = "/s";

/// A one-stage kind `name` that does not land, so it needs no merge queue.
fn work_kind(name: &str) -> Kind {
    kind(
        name,
        false,
        vec![stage("work", Some(Deliverable::Commits), "{{brief}}")],
    )
}

fn held(name: &str, from: LabelFrom) -> Option<LabelHeld> {
    Some(LabelHeld {
        name: name.into(),
        from,
    })
}

/// The label the start under `identity` holds that it started under.
fn held_label(
    world: &World,
    id: &PlaceId,
    identity: &Identity,
) -> Result<Option<LabelHeld>, String> {
    let state = world.held(id).ok_or("no held state")?;
    let start = state
        .stages
        .iter()
        .filter_map(|stage| stage.start.as_ref())
        .find(|start| &start.identity == identity)
        .ok_or_else(|| format!("no start {identity}"))?;
    Ok(start.label.clone())
}

fn each_source_of_a_label(_: &()) -> Outcome {
    let world = World::new();
    world.git.init(SECOND, "main");
    world
        .kinds
        .set_kinds(REPOSITORY, vec![work_kind("research"), work_kind("deploy")]);
    world.kinds.set_kinds(SECOND, vec![work_kind("deploy")]);
    world.kinds.set_label("deploy", Some("operation"));
    world.host.name_label(None, "research", Some("inception"));
    world
        .host
        .name_label(Some(REPOSITORY), "deploy", Some("construction"));
    let mut in_second = launch("deploy", "two", Some("x"));
    in_second.repository = SECOND.into();
    let cases = [
        (
            launch("research", "one", Some("x")),
            World::id("research", "one"),
            held("inception", LabelFrom::HostKind),
            "a research place",
        ),
        (
            launch("deploy", "one", Some("x")),
            World::id("deploy", "one"),
            held("construction", LabelFrom::HostRepositoryKind),
            "a deploy place in the repository",
        ),
        (
            in_second,
            PlaceId::new(place_location(SECOND, "deploy--two")),
            held("operation", LabelFrom::Kind),
            "a deploy place in /s",
        ),
    ];
    for (launch, id, label, what) in cases {
        let place = launched(&world.engine.launch(&launch))?;
        ensure_eq!(
            place.label,
            label,
            format!("{what}'s label in the launch's answer")
        );
        ensure_eq!(
            world.status(&id)?.label,
            label,
            format!("{what}'s label in status")
        );
        let start = world
            .held(&id)
            .and_then(|held| held.stages[0].start.clone())
            .ok_or_else(|| format!("{what} did not start"))?;
        ensure_eq!(start.label, label, format!("{what}'s start's label"));
    }
    Ok(())
}

fn a_label_changed_between_starts(_: &()) -> Outcome {
    let world = World::new();
    world.host.name_label(None, "change", Some("construction"));
    let id = construct_begun(&world, "one")?;
    let construct = World::identity("change", "one", "construct", 1);
    world.host.name_label(None, "change", Some("operation"));
    world.settle(&id);
    ensure_eq!(
        held_label(&world, &id, &construct)?,
        held("construction", LabelFrom::HostKind),
        "the construct start's label once the kind is operation's"
    );
    ensure_eq!(
        world.status(&id)?.label,
        held("operation", LabelFrom::HostKind),
        "the place's label as it would resolve now"
    );
    write_change(&world, "one");
    world.settle(&id);
    world.settle(&id);
    let code = World::identity("change", "one", "code", 1);
    ensure_eq!(
        held_label(&world, &id, &code)?,
        held("operation", LabelFrom::HostKind),
        "the code start's label"
    );
    Ok(())
}

fn s84_a_reserve_lent_and_a_slot_kept_free(_: &()) -> Outcome {
    let world = World::new();
    labelled_kinds(&world);
    shared(
        &world,
        "work",
        Some(5),
        &[("operation", 2, 1), ("construction", 2, 0)],
    );
    let mut builds = vec![];
    for name in ["b1", "b2", "b3", "b4"] {
        builds.push(placed(&world, "work", "build", name, 0)?);
    }
    for id in &builds {
        ensure!(running(&world, id), "{id} does not run");
    }
    let research = placed(&world, "work", "research", "r1", 50)?;
    ensure!(
        waiting(&world, &research),
        "the research place did not wait"
    );
    ensure_eq!(
        wants(&world, &research)?,
        Some(RoomWanted::KeptFree {
            stream: "work".into(),
            labels: vec!["operation".into()],
        }),
        "what the research place waits for"
    );
    let first = placed(&world, "work", "deploy", "d1", 0)?;
    ensure!(
        running(&world, &first),
        "the first operation place did not start in the slot kept free"
    );
    let second = placed(&world, "work", "deploy", "d2", 0)?;
    ensure!(
        waiting(&world, &second),
        "the second operation place did not wait"
    );
    ensure!(
        builds.iter().all(|id| running(&world, id)),
        "an agent was ended for the reserve"
    );
    end(&world, "build", "b1");
    ensure!(
        running(&world, &second),
        "the second operation place did not start first"
    );
    ensure!(
        waiting(&world, &research),
        "the research place did not keep waiting"
    );
    ensure_eq!(world.terminal.log.count("open"), 6, "agents opened");
    Ok(())
}

fn one_settle_ranks_again(_: &()) -> Outcome {
    let world = World::new();
    labelled_kinds(&world);
    shared(
        &world,
        "work",
        Some(6),
        &[("operation", 2, 0), ("construction", 2, 0)],
    );
    placed(&world, "work", "deploy", "d1", 0)?;
    for name in ["b1", "b2", "b3"] {
        placed(&world, "work", "build", name, 0)?;
    }
    world.host.set_bound(Some(4));
    let d2 = placed(&world, "work", "deploy", "d2", 0)?;
    let d3 = placed(&world, "work", "deploy", "d3", 0)?;
    let b4 = placed(&world, "work", "build", "b4", 10)?;
    for id in [&d2, &d3, &b4] {
        ensure!(waiting(&world, id), "{id} does not wait");
    }
    world.host.set_bound(None);
    world.terminal.log.clear();
    world.engine().settle(None);
    let opened: Vec<String> = world
        .terminal
        .log
        .of("open")
        .into_iter()
        .filter_map(|line| line.split(' ').nth(1).map(str::to_string))
        .collect();
    ensure_eq!(
        opened,
        vec![
            World::identity("deploy", "d2", "work", 1).to_string(),
            World::identity("build", "b4", "work", 1).to_string(),
        ],
        "the agents the settle opened, in order"
    );
    ensure!(
        waiting(&world, &d3),
        "the second operation place did not wait"
    );
    Ok(())
}

fn a_reserve_where_the_host_has_no_room(_: &()) -> Outcome {
    let world = World::new();
    labelled_kinds(&world);
    world.host.set_bound(Some(2));
    shared(&world, "work", Some(5), &[("operation", 1, 0)]);
    placed(&world, "work", "build", "b1", 0)?;
    placed(&world, "work", "build", "b2", 0)?;
    let deploy = placed(&world, "work", "deploy", "d1", 0)?;
    ensure!(waiting(&world, &deploy), "the operation place did not wait");
    ensure_eq!(
        wants(&world, &deploy)?,
        Some(RoomWanted::Host),
        "what the operation place waits for"
    );
    Ok(())
}

fn a_stream_with_no_bound(_: &()) -> Outcome {
    let world = World::new();
    labelled_kinds(&world);
    world.host.set_bound(Some(1));
    shared(&world, "personal", None, &[("operation", 1, 0)]);
    placed(&world, "personal", "build", "b1", 0)?;
    let build = placed(&world, "personal", "build", "b2", 10)?;
    let deploy = placed(&world, "personal", "deploy", "d1", 0)?;
    end(&world, "build", "b1");
    ensure!(
        running(&world, &build),
        "the construction place at priority 10 did not start"
    );
    ensure!(
        waiting(&world, &deploy),
        "the operation place was put first in a stream with no bound"
    );
    Ok(())
}

fn a_reserve_set_locally(_: &()) -> Outcome {
    let world = World::new();
    labelled_kinds(&world);
    shared(
        &world,
        "work",
        Some(4),
        &[("operation", 1, 0), ("construction", 1, 0)],
    );
    placed(&world, "work", "deploy", "d1", 0)?;
    placed(&world, "work", "deploy", "d2", 0)?;
    placed(&world, "work", "build", "b1", 0)?;
    placed(&world, "work", "build", "b2", 0)?;
    let build = placed(&world, "work", "build", "b3", 50)?;
    let deploy = placed(&world, "work", "deploy", "d3", 0)?;
    shared(
        &world,
        "work",
        Some(4),
        &[("operation", 3, 0), ("construction", 1, 0)],
    );
    world.host.set_value(
        "streams.work.labels.operation.reserve",
        "3",
        ConfigLayer::Local,
    );
    end(&world, "build", "b1");
    ensure!(
        running(&world, &deploy),
        "operation's reserve of three was not held at the next start"
    );
    ensure!(
        waiting(&world, &build),
        "the construction place at priority 50 did not wait"
    );
    let host: &dyn Host = world.host.as_ref();
    ensure_eq!(
        host.configuration()
            .values
            .iter()
            .find(|value| value.key == "streams.work.labels.operation.reserve")
            .map(|value| value.layer),
        Some(ConfigLayer::Local),
        "the reserve's layer"
    );
    Ok(())
}

/// The fix `one` launched on `work` and its start's refusal.
fn fix_refused(world: &World) -> Result<String, String> {
    launched(&world.engine.launch(&Launch {
        stream: Some("work".into()),
        ..launch("fix", "one", Some("x"))
    }))?;
    let id = World::id("fix", "one");
    let status = world.status(&id)?;
    ensure_eq!(
        status.stages[0].state,
        StageState::Refused,
        "the fix's code stage"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    status.stages[0]
        .refused_setting
        .clone()
        .ok_or_else(|| "the refusal gives no words".to_string())
}

fn a_pool_in_a_labels_table(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_share(
        "work",
        "operation",
        Some(Share {
            reserve: 1,
            keep_free: 0,
            unknown: vec!["pool".into()],
        }),
    );
    let words = fix_refused(&world)?;
    ensure!(
        words.contains("streams.work.labels.operation.pool"),
        "the refusal names not the key: {words}"
    );
    Ok(())
}

fn reserves_past_the_bound(_: &()) -> Outcome {
    let world = World::new();
    shared(
        &world,
        "work",
        Some(5),
        &[("operation", 3, 0), ("construction", 3, 0)],
    );
    let words = fix_refused(&world)?;
    ensure!(
        words.contains("work") && words.contains("6 in all") && words.contains("bound of 5"),
        "the refusal names not the stream, its reserves and its bound: {words}"
    );
    Ok(())
}

fn more_kept_free_than_owed(_: &()) -> Outcome {
    let world = World::new();
    shared(&world, "work", Some(5), &[("operation", 1, 2)]);
    let words = fix_refused(&world)?;
    ensure!(
        words.contains("operation") && words.contains("keeps 2 free of a reserve of 1"),
        "the refusal names not what operation keeps free and its reserve: {words}"
    );
    Ok(())
}

fn the_configuration_mended(_: &()) -> Outcome {
    let world = World::new();
    shared(&world, "work", Some(5), &[("operation", 1, 2)]);
    fix_refused(&world)?;
    let id = World::id("fix", "one");
    ensure!(
        refused_for(&world, &id)?.contains("operation"),
        "the refusal names not the label"
    );
    shared(&world, "work", Some(5), &[("operation", 2, 2)]);
    let retried = world
        .engine
        .retry(&id, Retry::Stage("code".into()), None, false);
    ensure!(
        matches!(retried, Answer::Acted { .. }),
        "the retry answered {retried:?}"
    );
    ensure!(
        world
            .stage_state(&id, "code")
            .is_some_and(StageState::running),
        "the fix's code did not start once mended"
    );
    Ok(())
}

fn the_labels_in_status(_: &()) -> Outcome {
    let world = World::new();
    labelled_kinds(&world);
    world.host.name_label(None, "research", Some("inception"));
    world
        .host
        .name_label(Some(REPOSITORY), "deploy", Some("construction"));
    shared(
        &world,
        "work",
        Some(5),
        &[("operation", 2, 1), ("construction", 1, 0)],
    );
    let research = placed(&world, "work", "research", "r1", 0)?;
    let deploy = placed(&world, "work", "deploy", "d1", 0)?;
    ensure_eq!(
        world.status(&research)?.label,
        held("inception", LabelFrom::HostKind),
        "the research place's label"
    );
    ensure_eq!(
        world.status(&deploy)?.label,
        held("construction", LabelFrom::HostRepositoryKind),
        "the deploy place's label"
    );
    let host = host(&world)?;
    let work = host
        .streams
        .iter()
        .find(|stream| stream.name == "work")
        .ok_or("status gives no stream work")?;
    ensure_eq!(
        work.labels,
        vec![
            LabelStatus {
                name: "construction".into(),
                reserve: 1,
                keep_free: 0,
                running: 1,
                waiting: 0,
            },
            LabelStatus {
                name: "operation".into(),
                reserve: 2,
                keep_free: 1,
                running: 0,
                waiting: 0,
            },
        ],
        "work's labels"
    );
    ensure_eq!(work.reserves, Reserves::Held, "work's reserves");
    ensure_eq!(work.refused, None, "what work refuses");
    let json = serde_json::to_value(&work.labels).map_err(|error| error.to_string())?;
    ensure_eq!(
        json[1],
        serde_json::json!({
            "name": "operation",
            "reserve": 2,
            "keep_free": 1,
            "running": 0,
            "waiting": 0,
        }),
        "operation in the JSON form"
    );
    Ok(())
}

fn waiting_on_a_slot_kept_free(_: &()) -> Outcome {
    let world = World::new();
    labelled_kinds(&world);
    shared(
        &world,
        "work",
        Some(5),
        &[("operation", 2, 1), ("construction", 2, 0)],
    );
    for name in ["b1", "b2", "b3", "b4"] {
        placed(&world, "work", "build", name, 0)?;
    }
    let research = placed(&world, "work", "research", "r1", 0)?;
    let queued = host(&world)?
        .queue
        .into_iter()
        .find(|queued| queued.place == research)
        .ok_or("the research place is not in the queue")?;
    ensure_eq!(
        serde_json::to_value(&queued.wants).map_err(|error| error.to_string())?,
        serde_json::json!({
            "room": "kept-free",
            "stream": "work",
            "labels": ["operation"],
        }),
        "what it wants in the JSON form"
    );
    ensure_eq!(
        queued.wants.words(),
        "the stream work's room, the rest kept free for operation".to_string(),
        "what it wants in words"
    );
    let status = world.status(&research)?;
    ensure_eq!(status.waiting_on, None, "what it waits on its caller for");
    ensure!(!status.stalled, "the research place is said stalled");
    world.host.advance(600);
    world.settle(&research);
    let status = world.status(&research)?;
    ensure!(
        status.waiting_on.is_none() && !status.stalled,
        "the research place waits on its caller once ten minutes pass"
    );
    Ok(())
}

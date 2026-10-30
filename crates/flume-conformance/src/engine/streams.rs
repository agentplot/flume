//! A place's stream (143; core.26): chosen once, at its launch, the
//! launch's, else the one the host's configuration names for its
//! repository and kind, for its repository, for its kind, else
//! `default`; kept with where it came from, and given by status (56). A
//! launch naming a stream the host does not declare is refused whole (4),
//! and a place keeps its stream when the host's configuration moves its
//! repository to another (8). A place's intent is its launch's name, else
//! its repository and base. Status gives each stream's agents running,
//! bound, room and queue.

use super::settling::{construct_begun, write_change};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "streams: a place's stream is the host's for its repository and kind, else for its repository, else for its kind, else default, each with where it came from",
            cites: &["143", "56"],
            run: each_source_in_its_order,
        },
        Scenario {
            name: "streams: a launch naming a stream runs its place in it, from the launch, over the host's word for the repository",
            cites: &["143", "7"],
            run: the_launchs_stream,
        },
        Scenario {
            name: "streams: a launch naming a stream the host does not declare is refused naming it, and nothing is made",
            cites: &["143", "4"],
            run: an_undeclared_stream,
        },
        Scenario {
            name: "streams: a stream the host's configuration names for a repository and does not declare refuses the launch naming it, and nothing is made",
            cites: &["143", "4"],
            run: an_undeclared_stream_the_host_names,
        },
        Scenario {
            name: "streams: the host's configuration moving a repository to another stream leaves a running place's later stages in its own, and a place launched after runs in the new one",
            cites: &["143", "8"],
            run: the_stream_fixed_at_the_launch,
        },
        Scenario {
            name: "streams: a place's status gives its stream with where it came from, and its intent, the launch's name or its repository and base",
            cites: &["56", "143", "7"],
            run: status_gives_the_stream_and_the_intent,
        },
        Scenario {
            name: "streams: the host's part of status gives each stream's agents running, bound, room and stages waiting, and the place's own status its stream",
            cites: &["56", "143", "70"],
            run: a_streams_part_of_status,
        },
    ]
}

/// The host declares `work` and `personal`.
fn two_streams(world: &World) {
    for name in ["work", "personal"] {
        world.host.set_stream(name, Some(Stream::default()));
    }
}

fn stream(name: &str, from: StreamFrom) -> StreamHeld {
    StreamHeld {
        name: name.into(),
        from,
    }
}

/// The stream the held state of the place `id` keeps.
fn held_stream(world: &World, id: &PlaceId) -> Result<StreamHeld, String> {
    Ok(world.held(id).ok_or("no held state")?.place.stream)
}

fn each_source_in_its_order(_: &()) -> Outcome {
    let world = World::new();
    two_streams(&world);
    world.host.set_stream("nightly", Some(Stream::default()));
    let launch_one = |kind: &str, name: &str| -> Result<StreamHeld, String> {
        Ok(launched(&world.engine.launch(&launch(kind, name, Some("x"))))?.stream)
    };
    ensure_eq!(
        launch_one("fix", "one")?,
        stream(DEFAULT_STREAM, StreamFrom::Default),
        "a place nothing names a stream for"
    );
    world
        .host
        .name_stream(None, Some("chore"), Some("personal"));
    world.host.name_stream(None, Some("fix"), Some("personal"));
    ensure_eq!(
        launch_one("fix", "two")?,
        stream("personal", StreamFrom::HostKind),
        "a fix with the kind's word alone"
    );
    world.host.name_stream(Some(REPOSITORY), None, Some("work"));
    ensure_eq!(
        launch_one("chore", "three")?,
        stream("work", StreamFrom::HostRepository),
        "a chore in the repository, its word before the kind's"
    );
    world
        .host
        .name_stream(Some(REPOSITORY), Some("fix"), Some("nightly"));
    ensure_eq!(
        launch_one("fix", "four")?,
        stream("nightly", StreamFrom::HostRepositoryKind),
        "a fix in the repository, the word for both before the repository's"
    );
    ensure_eq!(
        held_stream(&world, &World::id("fix", "four"))?,
        stream("nightly", StreamFrom::HostRepositoryKind),
        "the held state's stream"
    );
    Ok(())
}

fn the_launchs_stream(_: &()) -> Outcome {
    let world = World::new();
    two_streams(&world);
    world.host.name_stream(Some(REPOSITORY), None, Some("work"));
    let place = launched(&world.engine.launch(&Launch {
        stream: Some("personal".into()),
        ..launch("chore", "one", Some("x"))
    }))?;
    ensure_eq!(
        place.stream,
        stream("personal", StreamFrom::Launch),
        "the launch's stream"
    );
    Ok(())
}

/// Nothing of a refused launch stands: no held state, no place, no
/// branch and no agent.
fn nothing_made(world: &World, kind: &str, name: &str) -> Outcome {
    ensure!(world.store.places().is_empty(), "held state was left");
    ensure_eq!(world.place.log.count("make"), 0, "places made");
    ensure!(
        !world
            .git
            .branch_exists(REPOSITORY, &format!("{kind}/{name}")),
        "the branch was made"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    Ok(())
}

fn an_undeclared_stream(_: &()) -> Outcome {
    let world = World::new();
    two_streams(&world);
    let reason = refused(&world.engine.launch(&Launch {
        stream: Some("nightly".into()),
        ..launch("fix", "one", Some("x"))
    }))?;
    ensure!(
        reason.contains("nightly") && reason.contains("not declared"),
        "the refusal does not name the stream: {reason}"
    );
    nothing_made(&world, "fix", "one")
}

fn an_undeclared_stream_the_host_names(_: &()) -> Outcome {
    let world = World::new();
    world
        .host
        .name_stream(Some(REPOSITORY), None, Some("nightly"));
    let reason = refused(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure!(
        reason.contains("nightly") && reason.contains(REPOSITORY),
        "the refusal does not name the stream and where it is named: {reason}"
    );
    nothing_made(&world, "fix", "one")
}

fn the_stream_fixed_at_the_launch(_: &()) -> Outcome {
    let world = World::new();
    two_streams(&world);
    world.host.name_stream(Some(REPOSITORY), None, Some("work"));
    let id = construct_begun(&world, "one")?;
    world
        .host
        .name_stream(Some(REPOSITORY), None, Some("personal"));
    write_change(&world, "one");
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the later stage"
    );
    ensure_eq!(
        world.status(&id)?.stream,
        stream("work", StreamFrom::HostRepository),
        "the running place's stream once its later stage started"
    );
    let after = launched(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    ensure_eq!(
        after.stream,
        stream("personal", StreamFrom::HostRepository),
        "a place launched after the move"
    );
    Ok(())
}

fn status_gives_the_stream_and_the_intent(_: &()) -> Outcome {
    let world = World::new();
    two_streams(&world);
    launched(&world.engine.launch(&Launch {
        stream: Some("work".into()),
        intent: Some("the-surfaces".into()),
        ..launch("fix", "one", Some("x"))
    }))?;
    launched(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    launched(&world.engine.launch(&launch("chore", "three", Some("x"))))?;
    let one = world.status(&World::id("fix", "one"))?;
    ensure_eq!(
        one.stream,
        stream("work", StreamFrom::Launch),
        "the named place's stream"
    );
    let IntentKey::Launch { name, digest } = &one.intent else {
        return Err(format!("the named place's intent: {:?}", one.intent));
    };
    ensure_eq!(name.as_str(), "the-surfaces", "the intent's name");
    let json = serde_json::to_value(&one).map_err(|error| error.to_string())?;
    ensure_eq!(
        (
            json["stream"]["name"].as_str(),
            json["stream"]["from"].as_str(),
            json["intent"]["from"].as_str(),
            json["intent"]["name"].as_str(),
        ),
        (
            Some("work"),
            Some("launch"),
            Some("launch"),
            Some("the-surfaces")
        ),
        "the JSON form's stream and intent"
    );
    let two = world.status(&World::id("fix", "two"))?;
    ensure_eq!(
        two.stream,
        stream(DEFAULT_STREAM, StreamFrom::Default),
        "the other place's stream"
    );
    let IntentKey::Base {
        repository,
        base,
        digest: base_digest,
    } = &two.intent
    else {
        return Err(format!("the other place's intent: {:?}", two.intent));
    };
    ensure_eq!(
        (repository.as_str(), base.as_str()),
        (REPOSITORY, "main"),
        "the intent of a place naming none"
    );
    ensure!(digest != base_digest, "two intents share a digest");
    let three = world.status(&World::id("chore", "three"))?;
    ensure_eq!(
        three.intent.digest(),
        base_digest.as_str(),
        "a third place on the same repository and base"
    );
    Ok(())
}

fn a_streams_part_of_status(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(3));
    world.host.set_stream(
        "work",
        Some(Stream {
            agents: Some(4),
            priority: 10,
            pool: None,
        }),
    );
    let mut places = vec![];
    for name in ["one", "two", "three", "four"] {
        launched(&world.engine.launch(&Launch {
            stream: Some("work".into()),
            ..launch("fix", name, Some("x"))
        }))?;
        let id = World::id("fix", name);
        world.settle(&id);
        places.push(id);
    }
    let flume_core::Answer::Status { host, .. } = world.engine().status(None) else {
        return Err("status answered no status".into());
    };
    let work = host
        .streams
        .iter()
        .find(|stream| stream.name == "work")
        .ok_or("the host's streams give no work")?;
    ensure_eq!(
        (work.running, work.bound, work.room),
        (3, Some(4), Some(1)),
        "work's agents running, bound and room"
    );
    ensure_eq!(
        work.queue
            .iter()
            .map(|queued| queued.place.clone())
            .collect::<Vec<_>>(),
        vec![places[3].clone()],
        "the stage waiting in work"
    );
    ensure_eq!(
        world.status(&places[3])?.stream,
        stream("work", StreamFrom::Launch),
        "the waiting place's own stream"
    );
    Ok(())
}

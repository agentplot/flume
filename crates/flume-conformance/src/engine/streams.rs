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
use flume_core::{Answer, TitleFrom, TitleHeld};
use flume_fakes::FakeWindow;
use flume_seams::*;
use std::sync::Arc;

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
        Scenario {
            name: "streams: a later place of an intent naming no stream runs in its affinity's stream, from the intent, over the host's word for its repository",
            cites: &["143", "147"],
            run: a_later_place_of_an_intent,
        },
        Scenario {
            name: "streams: the host's facts give a stream's session and its labels' shares, a kind's label in any repository with where it came from, and each value of the configuration with its layer, as the core reads them",
            cites: &["143", "180", "181"],
            run: the_hosts_sessions_labels_and_layers,
        },
        Scenario {
            name: "streams: the host's facts give the sessions a stream names for a kind and for a kind's stage, and each other key of their tables, as the core reads them",
            cites: &["143", "199"],
            run: the_hosts_sessions_for_its_kinds_and_stages,
        },
        Scenario {
            name: "streams: status gives each stream's multiplexer, view, terminal app and title with where each came from, in the JSON form",
            cites: &["56", "143", "186", "190"],
            run: the_display_in_status,
        },
        Scenario {
            name: "streams: the fake host's work scripted with the title Work Agents has status give that title from stream",
            cites: &["106", "143"],
            run: a_scripted_title,
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
            ..Stream::default()
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

fn a_later_place_of_an_intent(_: &()) -> Outcome {
    let world = World::new();
    two_streams(&world);
    super::account_pools::pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![super::account_pools::account("work-1", None)],
    );
    world.host.set_host_pool(Some("work"));
    world.host.name_stream(Some(REPOSITORY), None, Some("work"));
    super::account_pools::fix_of(&world, Some("personal"), "one", Some("the-surfaces"))?;
    let later = launched(&world.engine.launch(&Launch {
        intent: Some("the-surfaces".into()),
        ..launch("fix", "two", Some("x"))
    }))?;
    ensure_eq!(
        later.stream,
        stream("personal", StreamFrom::Intent),
        "the later place's stream"
    );
    let other = launched(&world.engine.launch(&Launch {
        intent: Some("another".into()),
        ..launch("fix", "three", Some("x"))
    }))?;
    ensure_eq!(
        other.stream,
        stream("work", StreamFrom::HostRepository),
        "another intent's place's stream"
    );
    Ok(())
}

fn the_hosts_sessions_labels_and_layers(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream_session("work", Some("office"));
    let operation = Share {
        reserve: 2,
        keep_free: 1,
        unknown: vec![],
    };
    world
        .host
        .set_share("work", "operation", Some(operation.clone()));
    world.host.name_label(None, "research", Some("inception"));
    world
        .host
        .set_value("streams.work.agents", "3", ConfigLayer::Local);
    let host: &dyn Host = world.host.as_ref();
    let streams = host.streams();
    let work = streams
        .get("work")
        .ok_or("the host's streams give no work")?;
    ensure_eq!(work.session.as_deref(), Some("office"), "work's session");
    ensure_eq!(
        work.labels.get("operation"),
        Some(&operation),
        "operation's share of work"
    );
    for repository in [REPOSITORY, "/another"] {
        ensure_eq!(
            host.label_for(repository, "research"),
            Some(LabelHeld {
                name: "inception".into(),
                from: LabelFrom::HostKind,
            }),
            "research's label in {repository}"
        );
    }
    let configuration = host.configuration();
    ensure_eq!(
        configuration
            .values
            .iter()
            .find(|value| value.key == "streams.work.agents")
            .map(|value| (value.value.as_str(), value.layer)),
        Some(("3", ConfigLayer::Local)),
        "streams.work.agents and its layer"
    );
    Ok(())
}

/// The seam-contracts spec's fake host's sessions for its kinds and
/// stages: `work` naming `work-design` for research and for release's
/// review, and `work-plan` for plan with the other key `label` (143,
/// 199).
fn the_hosts_sessions_for_its_kinds_and_stages(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_kind_sessions(
        "work",
        "research",
        Some(KindSessions {
            session: Some("work-design".into()),
            ..KindSessions::default()
        }),
    );
    world.host.set_kind_sessions(
        "work",
        "release",
        Some(KindSessions {
            stages: [(
                "review".to_string(),
                StageSession {
                    session: Some("work-design".into()),
                    unknown: vec![],
                },
            )]
            .into(),
            ..KindSessions::default()
        }),
    );
    world.host.set_kind_sessions(
        "work",
        "plan",
        Some(KindSessions {
            session: Some("work-plan".into()),
            unknown: vec!["label".into()],
            ..KindSessions::default()
        }),
    );
    let host: &dyn Host = world.host.as_ref();
    let streams = host.streams();
    let work = streams
        .get("work")
        .ok_or("the host's streams give no work")?;
    let research = work.kinds.get("research").ok_or("work names no research")?;
    ensure_eq!(
        research.session.as_deref(),
        Some("work-design"),
        "work's session for research"
    );
    let review = work
        .kinds
        .get("release")
        .and_then(|release| release.stages.get("review"))
        .ok_or("work names nothing for release's review")?;
    ensure_eq!(
        review.session.as_deref(),
        Some("work-design"),
        "work's session for release's review"
    );
    let plan = work.kinds.get("plan").ok_or("work names no plan")?;
    ensure_eq!(
        (plan.session.as_deref(), plan.unknown.clone()),
        (Some("work-plan"), vec!["label".to_string()]),
        "work's session for plan and its table's other key"
    );
    ensure_eq!(work.session, None, "work's own session unnamed");
    Ok(())
}

/// The seam-fakes spec's scripted title: the fake host's `work` titled
/// `Work Agents`, and status asked (106, 143).
fn a_scripted_title(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_title("work", Some("Work Agents"));
    let Answer::Status { host, .. } = world.engine().status(None) else {
        return Err("status answered no status".into());
    };
    let work = host
        .streams
        .iter()
        .find(|stream| stream.name == "work")
        .ok_or("the host's streams give no work")?;
    ensure_eq!(
        work.title,
        TitleHeld {
            title: "Work Agents".into(),
            from: TitleFrom::Stream,
        },
        "work's title"
    );
    Ok(())
}

fn the_display_in_status(_: &()) -> Outcome {
    let world = World::new();
    world
        .resolver
        .with_window("office-window", Arc::new(FakeWindow::default()));
    world
        .host
        .set_host_display(Some("fake"), None, Some("fake-window"));
    world
        .host
        .set_display("office", None, None, Some("office-window"));
    world.host.set_title("office", Some("Office Agents"));
    world.host.set_stream("personal", Some(Stream::default()));
    let Answer::Status { host, .. } = world.engine().status(None) else {
        return Err("status answered no status".into());
    };
    let stream = |name: &str| {
        host.streams
            .iter()
            .find(|stream| stream.name == name)
            .cloned()
            .ok_or_else(|| format!("the host's streams give no {name}"))
    };
    let (office, personal) = (stream("office")?, stream("personal")?);
    let profile = |name: &str, from| ProfileHeld {
        profile: name.into(),
        from,
    };
    ensure_eq!(
        (office.terminal_app.clone(), office.title.clone()),
        (
            profile("office-window", NamedBy::Stream),
            TitleHeld {
                title: "Office Agents".into(),
                from: TitleFrom::Stream,
            }
        ),
        "office's terminal app and title"
    );
    ensure_eq!(
        (
            personal.multiplexer.clone(),
            personal.terminal_app.clone(),
            personal.view.clone(),
            personal.title.clone()
        ),
        (
            profile("fake", NamedBy::Host),
            profile("fake-window", NamedBy::Host),
            ViewHeld {
                view: View::Terminal,
                from: NamedBy::Neither,
            },
            TitleHeld {
                title: "personal".into(),
                from: TitleFrom::StreamName,
            }
        ),
        "personal's display and title"
    );
    ensure!(
        office.refused.is_none() && personal.refused.is_none(),
        "a stream refused: {:?}, {:?}",
        office.refused,
        personal.refused
    );
    let json = serde_json::to_value(&personal).map_err(|error| error.to_string())?;
    ensure_eq!(
        (
            json["multiplexer"]["from"].as_str(),
            json["view"]["view"].as_str(),
            json["view"]["from"].as_str(),
            json["terminal_app"]["profile"].as_str(),
            json["title"]["title"].as_str(),
            json["title"]["from"].as_str()
        ),
        (
            Some("host"),
            Some("terminal"),
            Some("neither"),
            Some("fake-window"),
            Some("personal"),
            Some("stream-name")
        ),
        "personal's display in the JSON form"
    );
    Ok(())
}

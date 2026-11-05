//! Where a stream's agents open in the terminal (107, 143; core.44):
//! where neither the launch's settings nor the stage's declaration name a
//! pane, a stage's agent opens in its place's stream's session, the one
//! the host's configuration names for the stream, else the one named by
//! the stream's name, and, for the default stream naming none, the
//! terminal's own; read at each start, an agent already started staying
//! where it opened (32). A launch names a session for every stage or one,
//! before its stream's, and no other layer gives one (158). Status gives
//! each agent's session and each stream's with where it came from (56).

use super::settling::{construct_begun, write_change};
use super::world::*;
use crate::{ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "sessions: two streams naming no session open their agents each in the session of its name, and the default stream's in the terminal's own, each start holding where it opened",
            cites: &["S82", "107", "143"],
            run: s82_each_stream_in_its_own_session,
        },
        Scenario {
            name: "sessions: a stream that names its session opens its agents in it",
            cites: &["107", "143"],
            run: a_stream_that_names_its_session,
        },
        Scenario {
            name: "sessions: the default stream that names a session opens a place naming no stream in it",
            cites: &["107", "143"],
            run: the_default_stream_that_names_one,
        },
        Scenario {
            name: "sessions: a stream's session changed between starts leaves the agent started where it opened and opens the next start in the new one",
            cites: &["32", "107"],
            run: a_session_changed_between_starts,
        },
        Scenario {
            name: "sessions: a launch's session comes before its stream's",
            cites: &["S82", "107", "7"],
            run: a_launchs_session_over_its_streams,
        },
        Scenario {
            name: "sessions: a launch's session for one stage opens that stage there and the others in the stream's",
            cites: &["32", "107"],
            run: one_stages_session,
        },
        Scenario {
            name: "sessions: a pane the launch names opens the conversation beside it, whatever session it names",
            cites: &["107", "32"],
            run: a_pane_before_a_session,
        },
        Scenario {
            name: "sessions: a session the kind's settings or the host's agents tables give is no setting, and the agent opens in its stream's",
            cites: &["32", "107"],
            run: a_session_no_other_layer_gives,
        },
        Scenario {
            name: "sessions: status gives each agent's session and each stream's with where it came from, in the JSON form",
            cites: &["S82", "56", "107", "143"],
            run: the_sessions_in_status,
        },
    ]
}

/// The host declares `work` and `personal`, naming no session.
fn two_streams(world: &World) {
    for name in ["work", "personal"] {
        world.host.set_stream(name, Some(Stream::default()));
    }
}

/// A fix launched as `name` on `stream`, none for the default.
fn fix_on(world: &World, stream: Option<&str>, name: &str) -> Result<Identity, String> {
    launched(&world.engine.launch(&Launch {
        stream: stream.map(str::to_string),
        ..launch("fix", name, Some("x"))
    }))?;
    Ok(World::identity("fix", name, "code", 1))
}

fn session(name: &str) -> Option<OpenIn> {
    Some(OpenIn::Session { name: name.into() })
}

fn held(name: Option<&str>, from: SessionFrom) -> Option<SessionHeld> {
    Some(SessionHeld {
        name: name.map(str::to_string),
        from,
    })
}

/// The session the start under `identity` holds that it opened in.
fn held_session(
    world: &World,
    id: &PlaceId,
    identity: &Identity,
) -> Result<Option<SessionHeld>, String> {
    let state = world.held(id).ok_or("no held state")?;
    let start = state
        .stages
        .iter()
        .filter_map(|stage| stage.start.as_ref())
        .find(|start| &start.identity == identity)
        .ok_or_else(|| format!("no start {identity}"))?;
    Ok(start.session.clone())
}

fn s82_each_stream_in_its_own_session(_: &()) -> Outcome {
    let world = World::new();
    two_streams(&world);
    let work = fix_on(&world, Some("work"), "one")?;
    let personal = fix_on(&world, Some("personal"), "two")?;
    let default = fix_on(&world, None, "three")?;
    ensure_eq!(
        world.terminal.opened_at(&work),
        session("work"),
        "the work fix's agent"
    );
    ensure_eq!(
        world.terminal.opened_at(&personal),
        session("personal"),
        "the personal fix's agent"
    );
    ensure_eq!(
        world.terminal.opened_at(&default),
        Some(OpenIn::Own),
        "the default fix's agent"
    );
    ensure_eq!(
        held_session(&world, &World::id("fix", "one"), &work)?,
        held(Some("work"), SessionFrom::StreamName),
        "the work start's session"
    );
    ensure_eq!(
        held_session(&world, &World::id("fix", "three"), &default)?,
        held(None, SessionFrom::Own),
        "the default start's session"
    );
    let place = world
        .held(&World::id("fix", "one"))
        .ok_or("no held state")?
        .place
        .place_ref();
    ensure_eq!(
        world.terminal.tabs_of(&place),
        vec![Some("work".to_string())],
        "the work fix's tab"
    );
    Ok(())
}

fn a_stream_that_names_its_session(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream_session("work", Some("office"));
    let work = fix_on(&world, Some("work"), "one")?;
    ensure_eq!(
        world.terminal.opened_at(&work),
        session("office"),
        "the work fix's agent"
    );
    ensure_eq!(
        held_session(&world, &World::id("fix", "one"), &work)?,
        held(Some("office"), SessionFrom::Stream),
        "the start's session"
    );
    Ok(())
}

fn the_default_stream_that_names_one(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream_session(DEFAULT_STREAM, Some("misc"));
    let fix = fix_on(&world, None, "one")?;
    ensure_eq!(
        world.terminal.opened_at(&fix),
        session("misc"),
        "the fix's agent"
    );
    ensure_eq!(
        held_session(&world, &World::id("fix", "one"), &fix)?,
        held(Some("misc"), SessionFrom::Stream),
        "the start's session"
    );
    Ok(())
}

fn a_session_changed_between_starts(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    world.host.name_stream(Some(REPOSITORY), None, Some("work"));
    let id = construct_begun(&world, "one")?;
    let construct = World::identity("change", "one", "construct", 1);
    ensure_eq!(
        world.terminal.opened_at(&construct),
        session("work"),
        "the construct agent"
    );
    world.host.set_stream_session("work", Some("office"));
    world.settle(&id);
    ensure_eq!(
        world.terminal.opened_at(&construct),
        session("work"),
        "the construct agent once its stream names office"
    );
    write_change(&world, "one");
    world.settle(&id);
    world.settle(&id);
    let code = World::identity("change", "one", "code", 1);
    ensure_eq!(
        world.terminal.opened_at(&code),
        session("office"),
        "the code agent"
    );
    ensure_eq!(
        held_session(&world, &id, &construct)?,
        held(Some("work"), SessionFrom::StreamName),
        "the construct start's session"
    );
    ensure_eq!(
        held_session(&world, &id, &code)?,
        held(Some("office"), SessionFrom::Stream),
        "the code start's session"
    );
    Ok(())
}

fn a_launchs_session_over_its_streams(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream_session("work", Some("office"));
    launched(&world.engine.launch(&Launch {
        stream: Some("work".into()),
        every_stage: Settings {
            session: Some("review".into()),
            ..Settings::default()
        },
        ..launch("fix", "one", Some("x"))
    }))?;
    let fix = World::identity("fix", "one", "code", 1);
    ensure_eq!(
        world.terminal.opened_at(&fix),
        session("review"),
        "the fix's agent"
    );
    ensure_eq!(
        held_session(&world, &World::id("fix", "one"), &fix)?,
        held(Some("review"), SessionFrom::Launch),
        "the start's session"
    );
    Ok(())
}

fn one_stages_session(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    world.host.name_stream(Some(REPOSITORY), None, Some("work"));
    let mut change = launch("change", "one", Some("plan it"));
    change.settings.insert(
        "verify".into(),
        Settings {
            session: Some("review".into()),
            ..Settings::default()
        },
    );
    launched(&world.engine.launch(&change))?;
    let id = World::id("change", "one");
    world.settle(&id);
    write_change(&world, "one");
    world.settle(&id);
    world.settle(&id);
    world.tick("change/one", "one", 1, 1);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::Begun),
        "verify"
    );
    for (stage, opened) in [
        ("construct", session("work")),
        ("code", session("work")),
        ("verify", session("review")),
    ] {
        ensure_eq!(
            world
                .terminal
                .opened_at(&World::identity("change", "one", stage, 1)),
            opened,
            "the {stage} agent"
        );
    }
    ensure_eq!(
        held_session(&world, &id, &World::identity("change", "one", "verify", 1))?,
        held(Some("review"), SessionFrom::Launch),
        "the verify start's session"
    );
    Ok(())
}

fn a_pane_before_a_session(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind("talk", false, vec![stage("talk", None, "{{brief}}")])],
    );
    let mut talk = launch("talk", "one", Some("x"));
    talk.every_stage.session = Some("review".into());
    talk.settings.insert(
        "talk".into(),
        Settings {
            beside: Some("operator".into()),
            ..Settings::default()
        },
    );
    launched(&world.engine.launch(&talk))?;
    let agent = World::identity("talk", "one", "talk", 1);
    ensure_eq!(
        world.terminal.opened_at(&agent),
        Some(OpenIn::Beside {
            pane: "operator".into()
        }),
        "the conversation's agent"
    );
    ensure_eq!(
        held_session(&world, &World::id("talk", "one"), &agent)?,
        None,
        "the start's session"
    );
    Ok(())
}

fn a_session_no_other_layer_gives(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    let mut code = stage("code", Some(Deliverable::Commits), "{{brief}}");
    code.agents = vec![Settings {
        session: Some("x".into()),
        ..Settings::default()
    }];
    let mut mend = kind("mend", true, vec![code]);
    mend.agents.every_stage.session = Some("x".into());
    world.kinds.set_kinds(REPOSITORY, vec![mend]);
    let named = |session: &str| KindSettings {
        every_stage: Settings {
            session: Some(session.into()),
            ..Settings::default()
        },
        ..KindSettings::default()
    };
    world.host.set_settings(None, "mend", named("y"));
    world
        .host
        .set_settings(Some(REPOSITORY), "mend", named("z"));
    let place = launched(&world.engine.launch(&Launch {
        stream: Some("work".into()),
        ..launch("mend", "one", Some("x"))
    }))?;
    let agent = World::identity("mend", "one", "code", 1);
    ensure_eq!(
        world.terminal.opened_at(&agent),
        session("work"),
        "the mend's agent"
    );
    ensure_eq!(
        place.agent.ok_or("no agent")?.settings.settings.session,
        None,
        "the agent's session setting"
    );
    Ok(())
}

fn the_sessions_in_status(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    fix_on(&world, Some("work"), "one")?;
    launched(&world.engine.launch(&Launch {
        every_stage: Settings {
            session: Some("review".into()),
            ..Settings::default()
        },
        ..launch("fix", "two", Some("x"))
    }))?;
    fix_on(&world, None, "three")?;
    for (name, expected, from) in [
        (
            "one",
            held(Some("work"), SessionFrom::StreamName),
            "stream-name",
        ),
        ("two", held(Some("review"), SessionFrom::Launch), "launch"),
        ("three", held(None, SessionFrom::Own), "own"),
    ] {
        let status = world.status(&World::id("fix", name))?;
        let agent = status.agent.ok_or_else(|| format!("{name} has no agent"))?;
        ensure_eq!(agent.session, expected, "{name}'s agent's session");
        let json = serde_json::to_value(&agent).map_err(|error| error.to_string())?;
        ensure_eq!(
            json["session"]["from"].as_str(),
            Some(from),
            "{name}'s agent's session's from in the JSON form"
        );
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
        Some(work.session.clone()),
        held(Some("work"), SessionFrom::StreamName),
        "the stream work's session"
    );
    let json = serde_json::to_value(work).map_err(|error| error.to_string())?;
    ensure_eq!(
        (
            json["session"]["name"].as_str(),
            json["session"]["from"].as_str()
        ),
        (Some("work"), Some("stream-name")),
        "the stream work's session in the JSON form"
    );
    let default = host
        .streams
        .iter()
        .find(|stream| stream.name == DEFAULT_STREAM)
        .ok_or("the host's streams give no default")?;
    ensure_eq!(
        Some(default.session.clone()),
        held(None, SessionFrom::Own),
        "the default stream's session"
    );
    Ok(())
}

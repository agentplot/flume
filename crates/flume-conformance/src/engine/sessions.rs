//! Where a stream's agents open in the terminal (107, 143, 199; core.44):
//! where neither the launch's settings nor the stage's declaration name a
//! pane, a stage's agent opens in its place's stream's session, the one
//! the stream names for the place's kind's stage, else for its kind, else
//! the one the host's configuration names for the stream, else the one
//! named by the stream's name, and, for the default stream naming none,
//! the terminal's own; read at each start, an agent already started
//! staying where it opened (32, 190). A launch names a session for every
//! stage or one, before its stream's, and no other layer gives one (158).
//! A stream's table for a kind or a stage holding another key, or naming
//! a session another stream's agents open in by that stream's word,
//! refuses the starts that read it, every other start running; the place
//! keeps its stream's room and shares (4, 70, 143, 182, 199). Status
//! gives each agent's session and each stream's with where it came from
//! (56).

use super::account_pools::{account, pool};
use super::settling::{construct_begun, write_change};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_seams::*;
use std::collections::BTreeMap;

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
        Scenario {
            name: "sessions: a stream's session for a kind opens every agent of the kind's places there, and its other places' agents in its own",
            cites: &["S106", "107", "199"],
            run: a_kinds_session,
        },
        Scenario {
            name: "sessions: a stream's session for a kind's stage opens that stage's agent there, and the kind's other stages' in its own",
            cites: &["S106", "107", "199"],
            run: a_stages_session,
        },
        Scenario {
            name: "sessions: a stream's session for a kind's stage comes before its session for the kind",
            cites: &["107", "199"],
            run: a_stages_session_before_its_kinds,
        },
        Scenario {
            name: "sessions: one stream's session for a kind opens nothing of a place in another stream",
            cites: &["143", "199"],
            run: another_streams_word,
        },
        Scenario {
            name: "sessions: a kind's session changed between starts leaves the agent started where it opened and opens the next start in the new one",
            cites: &["S107", "32", "190", "199"],
            run: a_kinds_session_changed_between_starts,
        },
        Scenario {
            name: "sessions: a launch's session comes before its stream's session for the kind",
            cites: &["S106", "107", "199"],
            run: a_launchs_session_over_the_streams_word_for_a_kind,
        },
        Scenario {
            name: "sessions: a launch's session comes before its stream's session for a stage",
            cites: &["S106", "107", "199"],
            run: a_launchs_session_over_the_streams_word_for_a_stage,
        },
        Scenario {
            name: "sessions: a pane the launch names opens the conversation beside it, and the kind's other stages in the stream's session for the kind",
            cites: &["S106", "107", "199"],
            run: a_pane_before_the_streams_word_for_a_kind,
        },
        Scenario {
            name: "sessions: a stream's table for a kind no place runs is ignored, and no start is refused",
            cites: &["199"],
            run: a_kind_no_place_runs,
        },
        Scenario {
            name: "sessions: a stream's table for a stage the kind does not hold is ignored, and no start is refused",
            cites: &["199"],
            run: a_stage_the_kind_does_not_hold,
        },
        Scenario {
            name: "sessions: a stream's table for a kind holding another key refuses that kind's starts on that stream alone, naming the table and the key, until it is mended",
            cites: &["S107", "4", "199"],
            run: a_tables_other_key,
        },
        Scenario {
            name: "sessions: a stream's table for a stage holding another key refuses that stage's start, naming the table and the key, and the stages before it start",
            cites: &["4", "182", "199"],
            run: a_stages_tables_other_key,
        },
        Scenario {
            name: "sessions: a stream's session for a kind that is another stream's own refuses the kind's starts naming both streams and the session, and the stream's other starts run",
            cites: &["S107", "4", "199"],
            run: another_streams_own_session,
        },
        Scenario {
            name: "sessions: two streams naming one session for a kind each refuse that kind's starts naming both streams and the session",
            cites: &["4", "199"],
            run: two_streams_naming_one_session_for_a_kind,
        },
        Scenario {
            name: "sessions: a stream naming its own session for a kind opens the kind's agents there from its word for the kind",
            cites: &["107", "199"],
            run: a_streams_own_session_named_for_a_kind,
        },
        Scenario {
            name: "sessions: a place whose agents open in its stream's session for its kind keeps its stream's room and pool",
            cites: &["S106", "70", "143", "199"],
            run: room_and_shares_unchanged,
        },
        Scenario {
            name: "sessions: a stage waiting for room whose stream's table for its kind comes to hold another key is refused at the next settle, whatever room its stream has",
            cites: &["4", "70", "199"],
            run: a_waiting_stage_whose_table_comes_to_hold_another_key,
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

/// The repository's `research`, whose two stages deliver commits; its
/// `release`, whose three do; and its `inquiry`, a stage delivering
/// commits and then a conversation; none of them landing.
fn kinds_of_the_repository(world: &World) {
    let commits = |name: &str| stage(name, Some(Deliverable::Commits), "{{brief}}");
    world.kinds.set_kinds(
        REPOSITORY,
        vec![
            kind(
                "research",
                false,
                vec![commits("explore"), commits("write")],
            ),
            kind(
                "release",
                false,
                vec![commits("build"), commits("review"), commits("publish")],
            ),
            kind(
                "inquiry",
                false,
                vec![commits("draft"), stage("talk", None, "{{brief}}")],
            ),
        ],
    );
}

/// The stream `stream`'s table for `kind`: `session`, each stage's
/// session by its name, and the table's other keys.
fn named(
    world: &World,
    stream: &str,
    kind: &str,
    session: Option<&str>,
    stages: &[(&str, &str)],
    unknown: &[&str],
) {
    world.host.set_kind_sessions(
        stream,
        kind,
        Some(KindSessions {
            session: session.map(str::to_string),
            stages: stages
                .iter()
                .map(|(stage, session)| {
                    (
                        (*stage).to_string(),
                        StageSession {
                            session: Some((*session).to_string()),
                            unknown: vec![],
                        },
                    )
                })
                .collect(),
            unknown: unknown.iter().map(|key| (*key).to_string()).collect(),
        }),
    );
}

/// A place of `kind` launched as `name` on `stream` with `every_stage`.
fn placed_on(
    world: &World,
    stream: &str,
    kind: &str,
    name: &str,
    every_stage: Settings,
) -> Result<PlaceId, String> {
    launched(&world.engine.launch(&Launch {
        stream: Some(stream.into()),
        every_stage,
        ..launch(kind, name, Some("x"))
    }))?;
    Ok(World::id(kind, name))
}

/// The stage running in the place `<kind>--<name>` delivered by a commit
/// on its branch, and the place settled, which starts its next stage.
fn delivered(world: &World, kind: &str, name: &str, stage: &str) {
    world.commit(&format!("{kind}/{name}"), &[&format!("{stage}.md")]);
    world.settle(&World::id(kind, name));
}

/// Where the first start of `stage` of the place `<kind>--<name>` opened.
fn opened(world: &World, kind: &str, name: &str, stage: &str) -> Option<OpenIn> {
    world
        .terminal
        .opened_at(&World::identity(kind, name, stage, 1))
}

/// The session the first start of `stage` of the place `<kind>--<name>`
/// holds that it opened in.
fn held_by(
    world: &World,
    kind: &str,
    name: &str,
    stage: &str,
) -> Result<Option<SessionHeld>, String> {
    held_session(
        world,
        &World::id(kind, name),
        &World::identity(kind, name, stage, 1),
    )
}

/// The words that refused `stage` of the place `id`, which opened no
/// agent.
fn refused_with(world: &World, id: &PlaceId, stage: &str) -> Result<String, String> {
    let status = world.status(id)?;
    let held = status
        .stages
        .iter()
        .find(|held| held.name == stage)
        .ok_or_else(|| format!("no stage {stage}"))?;
    ensure_eq!(
        held.state,
        StageState::Refused,
        format!("the stage {stage}")
    );
    held.refused_setting
        .clone()
        .ok_or_else(|| format!("the refusal of {stage} gives no words"))
}

fn a_kinds_session(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    named(&world, "work", "research", Some("work-design"), &[], &[]);
    placed_on(&world, "work", "research", "one", Settings::default())?;
    placed_on(&world, "work", "change", "two", Settings::default())?;
    let fix = fix_on(&world, Some("work"), "three")?;
    delivered(&world, "research", "one", "explore");
    for stage in ["explore", "write"] {
        ensure_eq!(
            opened(&world, "research", "one", stage),
            session("work-design"),
            format!("the research place's {stage} agent")
        );
        ensure_eq!(
            held_by(&world, "research", "one", stage)?,
            held(Some("work-design"), SessionFrom::StreamKind),
            format!("the research {stage} start's session")
        );
    }
    ensure_eq!(
        opened(&world, "change", "two", "construct"),
        session("work"),
        "the change's construct agent"
    );
    ensure_eq!(
        world.terminal.opened_at(&fix),
        session("work"),
        "the fix's agent"
    );
    ensure_eq!(
        held_session(&world, &World::id("fix", "three"), &fix)?,
        held(Some("work"), SessionFrom::StreamName),
        "the fix start's session"
    );
    Ok(())
}

fn a_stages_session(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    named(
        &world,
        "work",
        "release",
        None,
        &[("review", "work-review")],
        &[],
    );
    placed_on(&world, "work", "release", "one", Settings::default())?;
    delivered(&world, "release", "one", "build");
    delivered(&world, "release", "one", "review");
    for (stage, opens, from) in [
        ("build", "work", SessionFrom::StreamName),
        ("review", "work-review", SessionFrom::StreamStage),
        ("publish", "work", SessionFrom::StreamName),
    ] {
        ensure_eq!(
            opened(&world, "release", "one", stage),
            session(opens),
            format!("the release's {stage} agent")
        );
        ensure_eq!(
            held_by(&world, "release", "one", stage)?,
            held(Some(opens), from),
            format!("the {stage} start's session")
        );
    }
    Ok(())
}

fn a_stages_session_before_its_kinds(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    named(
        &world,
        "work",
        "release",
        Some("work-release"),
        &[("review", "work-review")],
        &[],
    );
    placed_on(&world, "work", "release", "one", Settings::default())?;
    delivered(&world, "release", "one", "build");
    delivered(&world, "release", "one", "review");
    for (stage, opens, from) in [
        ("build", "work-release", SessionFrom::StreamKind),
        ("review", "work-review", SessionFrom::StreamStage),
        ("publish", "work-release", SessionFrom::StreamKind),
    ] {
        ensure_eq!(
            opened(&world, "release", "one", stage),
            session(opens),
            format!("the release's {stage} agent")
        );
        ensure_eq!(
            held_by(&world, "release", "one", stage)?,
            held(Some(opens), from),
            format!("the {stage} start's session")
        );
    }
    Ok(())
}

fn another_streams_word(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    two_streams(&world);
    named(&world, "work", "research", Some("work-design"), &[], &[]);
    placed_on(&world, "personal", "research", "one", Settings::default())?;
    ensure_eq!(
        opened(&world, "research", "one", "explore"),
        session("personal"),
        "the research place's agent on personal"
    );
    ensure_eq!(
        held_by(&world, "research", "one", "explore")?,
        held(Some("personal"), SessionFrom::StreamName),
        "its start's session"
    );
    Ok(())
}

fn a_kinds_session_changed_between_starts(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    named(&world, "work", "research", Some("work-design"), &[], &[]);
    let id = placed_on(&world, "work", "research", "one", Settings::default())?;
    world.settle(&id);
    named(&world, "work", "research", Some("work-research"), &[], &[]);
    world.settle(&id);
    ensure_eq!(
        opened(&world, "research", "one", "explore"),
        session("work-design"),
        "the explore agent once its stream names work-research"
    );
    ensure_eq!(
        world.terminal.log.count("open"),
        1,
        "agents opened before explore delivered"
    );
    delivered(&world, "research", "one", "explore");
    ensure_eq!(
        opened(&world, "research", "one", "write"),
        session("work-research"),
        "the write agent"
    );
    ensure_eq!(
        held_by(&world, "research", "one", "explore")?,
        held(Some("work-design"), SessionFrom::StreamKind),
        "the explore start's session"
    );
    ensure_eq!(
        held_by(&world, "research", "one", "write")?,
        held(Some("work-research"), SessionFrom::StreamKind),
        "the write start's session"
    );
    Ok(())
}

/// Settings naming `session` for every stage.
fn in_session(session: &str) -> Settings {
    Settings {
        session: Some(session.into()),
        ..Settings::default()
    }
}

fn a_launchs_session_over_the_streams_word_for_a_kind(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    named(&world, "work", "research", Some("work-design"), &[], &[]);
    placed_on(&world, "work", "research", "one", in_session("review"))?;
    delivered(&world, "research", "one", "explore");
    for stage in ["explore", "write"] {
        ensure_eq!(
            opened(&world, "research", "one", stage),
            session("review"),
            format!("the research place's {stage} agent")
        );
        ensure_eq!(
            held_by(&world, "research", "one", stage)?,
            held(Some("review"), SessionFrom::Launch),
            format!("the {stage} start's session")
        );
    }
    Ok(())
}

fn a_launchs_session_over_the_streams_word_for_a_stage(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    named(
        &world,
        "work",
        "release",
        None,
        &[("review", "work-review")],
        &[],
    );
    placed_on(&world, "work", "release", "one", in_session("elsewhere"))?;
    delivered(&world, "release", "one", "build");
    delivered(&world, "release", "one", "review");
    for stage in ["build", "review", "publish"] {
        ensure_eq!(
            opened(&world, "release", "one", stage),
            session("elsewhere"),
            format!("the release's {stage} agent")
        );
        ensure_eq!(
            held_by(&world, "release", "one", stage)?,
            held(Some("elsewhere"), SessionFrom::Launch),
            format!("the {stage} start's session")
        );
    }
    Ok(())
}

fn a_pane_before_the_streams_word_for_a_kind(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    named(&world, "work", "inquiry", Some("work-design"), &[], &[]);
    let mut inquiry = Launch {
        stream: Some("work".into()),
        ..launch("inquiry", "one", Some("x"))
    };
    inquiry.settings.insert(
        "talk".into(),
        Settings {
            beside: Some("operator".into()),
            ..Settings::default()
        },
    );
    launched(&world.engine.launch(&inquiry))?;
    delivered(&world, "inquiry", "one", "draft");
    ensure_eq!(
        opened(&world, "inquiry", "one", "draft"),
        session("work-design"),
        "the inquiry's draft agent"
    );
    ensure_eq!(
        opened(&world, "inquiry", "one", "talk"),
        Some(OpenIn::Beside {
            pane: "operator".into()
        }),
        "the conversation's agent"
    );
    ensure_eq!(
        held_by(&world, "inquiry", "one", "talk")?,
        None,
        "the conversation start's session"
    );
    Ok(())
}

fn a_kind_no_place_runs(_: &()) -> Outcome {
    let world = World::new();
    named(&world, "work", "writeback", Some("x"), &[], &[]);
    let id = placed_on(&world, "work", "change", "one", Settings::default())?;
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::Begun),
        "construct"
    );
    ensure_eq!(
        held_by(&world, "change", "one", "construct")?,
        held(Some("work"), SessionFrom::StreamName),
        "the construct start's session"
    );
    Ok(())
}

fn a_stage_the_kind_does_not_hold(_: &()) -> Outcome {
    let world = World::new();
    named(&world, "work", "change", None, &[("review", "x")], &[]);
    let id = placed_on(&world, "work", "change", "one", Settings::default())?;
    world.settle(&id);
    write_change(&world, "one");
    world.settle(&id);
    world.settle(&id);
    for stage in ["construct", "code"] {
        ensure!(
            world.stage_state(&id, stage) != Some(StageState::Refused),
            "the change's {stage} was refused"
        );
        ensure_eq!(
            held_by(&world, "change", "one", stage)?,
            held(Some("work"), SessionFrom::StreamName),
            format!("the {stage} start's session")
        );
    }
    Ok(())
}

fn a_tables_other_key(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    two_streams(&world);
    named(
        &world,
        "work",
        "research",
        Some("work-design"),
        &[],
        &["label"],
    );
    let research = placed_on(&world, "work", "research", "one", Settings::default())?;
    let change = placed_on(&world, "work", "change", "two", Settings::default())?;
    placed_on(&world, "personal", "research", "three", Settings::default())?;
    let words = refused_with(&world, &research, "explore")?;
    ensure!(
        words.contains("streams.work.kinds.research.label"),
        "the refusal names not the table and the key: {words}"
    );
    ensure_eq!(
        opened(&world, "research", "one", "explore"),
        None,
        "the refused research place's agent"
    );
    ensure_eq!(
        opened(&world, "change", "two", "construct"),
        session("work"),
        "the change's construct agent on work"
    );
    world.settle(&change);
    ensure!(
        world
            .stage_state(&change, "construct")
            .is_some_and(StageState::running),
        "the change on work did not start"
    );
    ensure_eq!(
        opened(&world, "research", "three", "explore"),
        session("personal"),
        "the research place's agent on personal"
    );
    named(&world, "work", "research", Some("work-design"), &[], &[]);
    let retried = world
        .engine
        .retry(&research, Retry::Stage("explore".into()), None, false);
    ensure!(
        matches!(retried, flume_core::Answer::Acted { .. }),
        "the retry answered {retried:?}"
    );
    ensure!(
        world
            .stage_state(&research, "explore")
            .is_some_and(StageState::running),
        "the research stage did not start once mended"
    );
    ensure_eq!(
        opened(&world, "research", "one", "explore"),
        session("work-design"),
        "the research place's agent once mended"
    );
    Ok(())
}

fn a_stages_tables_other_key(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    world.host.set_kind_sessions(
        "work",
        "release",
        Some(KindSessions {
            stages: BTreeMap::from([(
                "review".to_string(),
                StageSession {
                    session: Some("work-review".into()),
                    unknown: vec!["pool".into()],
                },
            )]),
            ..KindSessions::default()
        }),
    );
    let id = placed_on(&world, "work", "release", "one", Settings::default())?;
    ensure_eq!(
        opened(&world, "release", "one", "build"),
        session("work"),
        "the release's build agent"
    );
    delivered(&world, "release", "one", "build");
    let words = refused_with(&world, &id, "review")?;
    ensure!(
        words.contains("streams.work.kinds.release.stages.review.pool"),
        "the refusal names not the table and the key: {words}"
    );
    ensure_eq!(
        opened(&world, "release", "one", "review"),
        None,
        "the refused review's agent"
    );
    Ok(())
}

/// Whether `words` name the streams `work` and `personal` and the
/// session `session`.
fn names_both(words: &str, session: &str) -> bool {
    words.contains("the stream work")
        && words.contains("the stream personal")
        && words.contains(&format!("the session {session} "))
}

fn another_streams_own_session(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    two_streams(&world);
    named(&world, "work", "research", Some("personal"), &[], &[]);
    let research = placed_on(&world, "work", "research", "one", Settings::default())?;
    let fix = fix_on(&world, Some("work"), "two")?;
    let words = refused_with(&world, &research, "explore")?;
    ensure!(
        names_both(&words, "personal"),
        "the refusal names not both streams and the session: {words}"
    );
    ensure_eq!(
        opened(&world, "research", "one", "explore"),
        None,
        "the refused research place's agent"
    );
    ensure_eq!(
        world.terminal.opened_at(&fix),
        session("work"),
        "the fix's agent"
    );
    Ok(())
}

fn two_streams_naming_one_session_for_a_kind(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    for stream in ["work", "personal"] {
        named(&world, stream, "research", Some("design"), &[], &[]);
    }
    for (stream, name) in [("work", "one"), ("personal", "two")] {
        let id = placed_on(&world, stream, "research", name, Settings::default())?;
        let words = refused_with(&world, &id, "explore")?;
        ensure!(
            names_both(&words, "design"),
            "the refusal on {stream} names not both streams and the session: {words}"
        );
        ensure!(
            words.starts_with(&format!("streams.{stream}.kinds.research.session")),
            "the refusal on {stream} names not its own table: {words}"
        );
    }
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    Ok(())
}

fn a_streams_own_session_named_for_a_kind(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    world.host.set_stream_session("work", Some("office"));
    named(&world, "work", "research", Some("office"), &[], &[]);
    placed_on(&world, "work", "research", "one", Settings::default())?;
    ensure_eq!(
        opened(&world, "research", "one", "explore"),
        session("office"),
        "the research place's agent"
    );
    ensure_eq!(
        held_by(&world, "research", "one", "explore")?,
        held(Some("office"), SessionFrom::StreamKind),
        "its start's session"
    );
    Ok(())
}

fn room_and_shares_unchanged(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    pool(
        &world,
        "work",
        Strategy::LeastBusy,
        vec![account("a", None), account("b", None)],
    );
    world.host.set_stream(
        "work",
        Some(Stream {
            agents: Some(2),
            pool: Some("work".into()),
            ..Stream::default()
        }),
    );
    named(&world, "work", "research", Some("work-design"), &[], &[]);
    let research = placed_on(&world, "work", "research", "one", Settings::default())?;
    let fix = placed_on(&world, "work", "fix", "two", Settings::default())?;
    let chore = placed_on(&world, "work", "chore", "three", Settings::default())?;
    for (id, stage) in [(&research, "explore"), (&fix, "code")] {
        ensure!(
            world
                .stage_state(id, stage)
                .is_some_and(StageState::running),
            "{id} did not run"
        );
        let state = world.held(id).ok_or("no held state")?;
        let start = state
            .stages
            .iter()
            .find(|held| held.name == stage)
            .and_then(|held| held.start.as_ref())
            .ok_or_else(|| format!("{id} holds no start"))?;
        ensure_eq!(
            start.account.as_ref().map(|account| account.pool.as_str()),
            Some("work"),
            format!("the pool {id}'s account came from")
        );
    }
    ensure_eq!(
        world.stage_state(&chore, "code"),
        Some(StageState::WaitingForRoom),
        "the chore"
    );
    ensure_eq!(
        opened(&world, "research", "one", "explore"),
        session("work-design"),
        "the research place's agent"
    );
    ensure_eq!(
        opened(&world, "fix", "two", "code"),
        session("work"),
        "the fix's agent"
    );
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
        (2, Some(2), Some(0)),
        "work's agents running, bound and room"
    );
    Ok(())
}

fn a_waiting_stage_whose_table_comes_to_hold_another_key(_: &()) -> Outcome {
    let world = World::new();
    kinds_of_the_repository(&world);
    world.host.set_stream(
        "work",
        Some(Stream {
            agents: Some(1),
            ..Stream::default()
        }),
    );
    named(&world, "work", "research", Some("work-design"), &[], &[]);
    let fix = fix_on(&world, Some("work"), "one")?;
    let research = placed_on(&world, "work", "research", "two", Settings::default())?;
    ensure_eq!(
        world.stage_state(&research, "explore"),
        Some(StageState::WaitingForRoom),
        "the research place behind the fix"
    );
    named(
        &world,
        "work",
        "research",
        Some("work-design"),
        &[],
        &["label"],
    );
    world.engine().settle(None);
    let words = refused_with(&world, &research, "explore")?;
    ensure!(
        words.contains("streams.work.kinds.research.label"),
        "the refusal names not the table and the key: {words}"
    );
    ensure_eq!(
        world.terminal.opened_at(&fix),
        session("work"),
        "the fix's agent, still running"
    );
    ensure_eq!(
        opened(&world, "research", "two", "explore"),
        None,
        "the refused research place's agent"
    );
    Ok(())
}

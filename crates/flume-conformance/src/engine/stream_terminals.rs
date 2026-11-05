//! A stream's terminal and the window it is viewed in, given by name
//! (105, 190; core.49): the resolver gives a terminal profile and a
//! window profile by name, none for a name it carries none for, and
//! lists the names it carries, the one the host's multiplexer names
//! being the very terminal the core is built with. A stage's agent opens
//! in the terminal its stream's multiplexer names as the host's
//! configuration stands at its start, else the host's, and records it;
//! every later call on the agent goes to the terminal it recorded, the
//! host's for a start written before that records none; and a stream
//! whose multiplexer names a terminal profile the resolver does not
//! carry refuses its launches and its starts, naming it, and nothing
//! else (4, 32, 107, 190).

use super::settling::{construct_begun, write_change};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_fakes::{FakeTerminal, FakeWindow};
use flume_seams::*;
use std::sync::Arc;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "stream terminals: the resolver gives terminals and windows by name, the core's own among them, and none for a name it carries none for",
            cites: &["105", "190"],
            run: named_terminals_and_windows,
        },
        Scenario {
            name: "stream terminals: two streams under two terminals open their agents each in its own, each start recording its terminal with where it came from, and each terminal hears only of its own",
            cites: &["190", "S91"],
            run: two_streams_under_two_terminals,
        },
        Scenario {
            name: "stream terminals: a stream naming no multiplexer on a host naming none opens its agent in the terminal the host's reader names for neither",
            cites: &["105", "190"],
            run: a_stream_naming_none_on_a_host_naming_none,
        },
        Scenario {
            name: "stream terminals: a stream's multiplexer changed while a change runs leaves its agent read, told and ended where it opened, and opens the next stage in the new one",
            cites: &["32", "107", "190", "S91"],
            run: a_multiplexer_changed_while_a_change_runs,
        },
        Scenario {
            name: "stream terminals: a start written before, recording no terminal, has its agent read and closed through the host's terminal",
            cites: &["190"],
            run: a_start_written_before,
        },
        Scenario {
            name: "stream terminals: a stream naming rex refuses its launch naming the stream, rex and the terminal profiles carried, nothing made, while a launch on another stream opens its agent",
            cites: &["4", "190", "S91"],
            run: a_stream_naming_rex,
        },
        Scenario {
            name: "stream terminals: a stream set to rex while its place waits for room refuses the start as a setting is refused, whatever room the host has, and nothing opens",
            cites: &["4", "190"],
            run: a_stream_changed_to_rex_while_it_waits,
        },
    ]
}

/// Whether `given` is the very fake `fake`.
fn is<T: ?Sized, F>(given: &Arc<T>, fake: &Arc<F>) -> bool {
    std::ptr::addr_eq(Arc::as_ptr(given), Arc::as_ptr(fake))
}

fn named_terminals_and_windows(_: &()) -> Outcome {
    let world = World::new();
    let other = Arc::new(FakeTerminal::new(world.host.clone()));
    let other_window = Arc::new(FakeWindow::default());
    world.resolver.with_terminal("other", other.clone());
    world
        .resolver
        .with_window("other-window", other_window.clone());
    let resolver = &world.resolver;
    ensure!(
        resolver
            .terminal("fake")
            .is_some_and(|terminal| is(&terminal, &world.terminal)),
        "fake is not the terminal the core is built with"
    );
    ensure!(
        resolver
            .terminal("other")
            .is_some_and(|terminal| is(&terminal, &other)),
        "other is not the terminal given"
    );
    ensure!(resolver.terminal("rex").is_none(), "a terminal named rex");
    ensure!(
        resolver
            .window("fake-window")
            .is_some_and(|window| is(&window, &world.window)),
        "fake-window is not the fake window"
    );
    ensure!(resolver.window("kitty").is_none(), "a window named kitty");
    ensure_eq!(
        resolver.terminal_profiles(),
        vec!["fake".to_string(), "other".to_string()],
        "the terminal profiles"
    );
    ensure_eq!(
        resolver.window_profiles(),
        vec!["fake-window".to_string(), "other-window".to_string()],
        "the window profiles"
    );
    Ok(())
}

/// A fix launched as `name` on `stream`, and its code stage's agent.
fn fix_on(world: &World, stream: &str, name: &str) -> Result<Identity, String> {
    launched(&world.engine.launch(&Launch {
        stream: Some(stream.into()),
        ..launch("fix", name, Some("x"))
    }))?;
    Ok(World::identity("fix", name, "code", 1))
}

/// The terminal the start under `identity` records it opened in.
fn held_terminal(
    world: &World,
    id: &PlaceId,
    identity: &Identity,
) -> Result<Option<ProfileHeld>, String> {
    let state = world.held(id).ok_or("no held state")?;
    let start = state
        .stages
        .iter()
        .filter_map(|stage| stage.start.as_ref())
        .find(|start| &start.identity == identity)
        .ok_or_else(|| format!("no start {identity}"))?;
    Ok(start.terminal.clone())
}

fn profile(name: &str, from: NamedBy) -> Option<ProfileHeld> {
    Some(ProfileHeld {
        profile: name.into(),
        from,
    })
}

/// Whether a line of `terminal`'s log names `agent`.
fn heard_of(terminal: &FakeTerminal, agent: &Identity) -> bool {
    let agent = agent.to_string();
    terminal
        .log
        .calls()
        .iter()
        .any(|call| call.split(' ').any(|word| word == agent))
}

/// The host's own multiplexer `fake`, the stream `work` naming none and
/// `trial` naming `other`, a fake terminal of its own.
fn work_and_trial(world: &World) -> Arc<FakeTerminal> {
    let other = world.other_terminal("other");
    world.host.set_host_display(Some("fake"), None, None);
    world.host.set_stream("work", Some(Stream::default()));
    world.host.set_display("trial", Some("other"), None, None);
    other
}

fn two_streams_under_two_terminals(_: &()) -> Outcome {
    let world = World::new();
    let other = work_and_trial(&world);
    let work = fix_on(&world, "work", "one")?;
    let trial = fix_on(&world, "trial", "two")?;
    ensure!(
        world.terminal.opened(&work).is_some(),
        "the work fix's agent did not open in fake"
    );
    ensure!(
        other.opened(&trial).is_some(),
        "the trial fix's agent did not open in other"
    );
    ensure_eq!(
        held_terminal(&world, &World::id("fix", "one"), &work)?,
        profile("fake", NamedBy::Host),
        "the work start's terminal"
    );
    ensure_eq!(
        held_terminal(&world, &World::id("fix", "two"), &trial)?,
        profile("other", NamedBy::Stream),
        "the trial start's terminal"
    );
    for (id, stage) in [
        (World::id("fix", "one"), "code"),
        (World::id("fix", "two"), "code"),
    ] {
        ensure_eq!(
            world.stage_state(&id, stage),
            Some(StageState::Begun),
            "{id}'s code stage"
        );
    }
    ensure!(
        other.sent_to(&trial).len() == 1 && world.terminal.sent_to(&trial).is_empty(),
        "the trial fix's prompt was not sent through other alone"
    );
    ensure!(
        !heard_of(&world.terminal, &trial),
        "fake heard of the trial fix's agent: {:?}",
        world.terminal.log.calls()
    );
    ensure!(
        !heard_of(&other, &work),
        "other heard of the work fix's agent: {:?}",
        other.log.calls()
    );
    Ok(())
}

fn a_stream_naming_none_on_a_host_naming_none(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    let work = fix_on(&world, "work", "one")?;
    ensure!(
        world.terminal.opened(&work).is_some(),
        "the fix's agent did not open in the fake host's default"
    );
    ensure_eq!(
        held_terminal(&world, &World::id("fix", "one"), &work)?,
        profile("fake", NamedBy::Neither),
        "the start's terminal"
    );
    Ok(())
}

fn a_multiplexer_changed_while_a_change_runs(_: &()) -> Outcome {
    let world = World::new();
    let other = work_and_trial(&world);
    world
        .host
        .name_stream(Some(REPOSITORY), None, Some("trial"));
    let id = construct_begun(&world, "one")?;
    let construct = World::identity("change", "one", "construct", 1);
    ensure!(
        other.opened(&construct).is_some(),
        "the construct agent did not open in other"
    );
    world.host.set_display("trial", Some("fake"), None, None);
    other.log.clear();
    world.terminal.log.clear();
    if let Some(reason) = world.engine().tell(&id, "keep it short", None).refusal() {
        return Err(format!("the tell was refused: {reason}"));
    }
    ensure_eq!(
        other.sent_to(&construct).last().cloned(),
        Some("keep it short".to_string()),
        "the tell's words through other"
    );
    write_change(&world, "one");
    world.settle(&id);
    world.settle(&id);
    ensure!(
        other.log.count("state") > 0 && other.log.count("close") == 1,
        "the construct agent was not read and closed through other: {:?}",
        other.log.calls()
    );
    ensure!(
        !heard_of(&world.terminal, &construct),
        "fake heard of the construct agent: {:?}",
        world.terminal.log.calls()
    );
    let code = World::identity("change", "one", "code", 1);
    ensure!(
        world.terminal.opened(&code).is_some() && other.opened(&code).is_none(),
        "the code stage did not open in fake"
    );
    ensure_eq!(
        held_terminal(&world, &id, &construct)?,
        profile("other", NamedBy::Stream),
        "the construct start's terminal"
    );
    ensure_eq!(
        held_terminal(&world, &id, &code)?,
        profile("fake", NamedBy::Stream),
        "the code start's terminal"
    );
    Ok(())
}

fn a_start_written_before(_: &()) -> Outcome {
    let world = World::new();
    let other = world.other_terminal("other");
    let fix = fix_on(&world, DEFAULT_STREAM, "one")?;
    let id = World::id("fix", "one");
    let mut held = world.held(&id).ok_or("no held state")?;
    for stage in &mut held.stages {
        if let Some(start) = stage.start.as_mut() {
            start.terminal = None;
        }
    }
    world.store.put(&id, held);
    world
        .host
        .set_display(DEFAULT_STREAM, Some("other"), None, None);
    world.terminal.log.clear();
    world.commit("fix/one", &["src/lib.rs"]);
    world.settle(&id);
    ensure!(
        world.terminal.log.count("state") > 0
            && world.terminal.log.of("close") == vec![format!("close {fix}")],
        "the agent was not read and closed through the host's terminal: {:?}",
        world.terminal.log.calls()
    );
    ensure!(
        !heard_of(&other, &fix),
        "the stream's terminal heard of the agent: {:?}",
        other.log.calls()
    );
    Ok(())
}

fn a_stream_naming_rex(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    world.host.set_display("trial", Some("rex"), None, None);
    let reason = refused(&world.engine.launch(&Launch {
        stream: Some("trial".into()),
        ..launch("fix", "one", Some("x"))
    }))?;
    ensure!(
        ["trial", "rex", "terminal profiles there are: fake"]
            .iter()
            .all(|word| reason.contains(word)),
        "the refusal: {reason}"
    );
    ensure!(
        world.held(&World::id("fix", "one")).is_none() && world.place.log.count("make") == 0,
        "the trial fix made something"
    );
    let work = fix_on(&world, "work", "two")?;
    ensure!(
        world.terminal.opened(&work).is_some(),
        "the work fix's agent did not open"
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

fn a_stream_changed_to_rex_while_it_waits(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(1));
    world.host.set_stream("work", Some(Stream::default()));
    world.host.set_stream("trial", Some(Stream::default()));
    fix_on(&world, "work", "one")?;
    fix_on(&world, "trial", "two")?;
    let trial = World::id("fix", "two");
    ensure_eq!(
        world.stage_state(&trial, "code"),
        Some(StageState::WaitingForRoom),
        "the trial fix's code stage"
    );
    world.host.set_display("trial", Some("rex"), None, None);
    world.settle(&trial);
    ensure_eq!(
        world.stage_state(&trial, "code"),
        Some(StageState::Refused),
        "the trial fix's code stage, the host still full"
    );
    let setting = world.status(&trial)?.stages[0].refused_setting.clone();
    ensure!(
        setting
            .as_deref()
            .is_some_and(|words| words.contains("trial") && words.contains("rex")),
        "the refused setting: {setting:?}"
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

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
        Scenario {
            name: "stream terminals: a place whose stages ran in two terminals is tidied in each once, and the tidy's answer names each terminal with what it closed",
            cites: &["137", "190"],
            run: a_place_in_two_terminals,
        },
        Scenario {
            name: "stream terminals: a stage retried after its stream's multiplexer changed is tidied in the terminal of each of its starts",
            cites: &["137", "190"],
            run: a_stage_retried_in_another_terminal,
        },
        Scenario {
            name: "stream terminals: a caller under a stream's terminal is a launch's recipient, and its end is sent to it through that terminal",
            cites: &["7", "43", "190"],
            run: a_caller_under_a_streams_terminal,
        },
        Scenario {
            name: "stream terminals: under a caller's watch, the host's terminal fake and a stream's other, a caller's act on another stream settles nothing first, and the stream's places move at the caller's settles",
            cites: &["119", "122", "190", "208", "S91"],
            run: one_stream_under_another_terminal,
        },
        Scenario {
            name: "stream terminals: with no caller keeping watch, the host's terminal fake and a stream's other, a caller's act settles every place first under either, and a wait drives its settles",
            cites: &["119", "122", "190", "208"],
            run: two_terminals_and_no_watch,
        },
        Scenario {
            name: "stream terminals: S91, a stream under a terminal of its own beside the host's, which decides nothing of whether flume runs attended, its next stage in the terminal it is set to while its agent stays, and a stream naming rex refused while every other runs",
            cites: &["S91", "4", "119", "143", "190", "208"],
            run: s91_a_stream_under_its_own_terminal,
        },
        Scenario {
            name: "stream terminals: status gives each stream's multiplexer with where it came from, each agent's terminal, and each terminal the host binds with what it keeps and its watch none, in the JSON form",
            cites: &["56", "122", "190", "S91"],
            run: the_terminals_in_status,
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
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    let json = serde_json::to_value(&agent).map_err(|error| error.to_string())?;
    ensure!(
        agent.terminal.is_none() && json["terminal"].is_null(),
        "status's terminal of a start written before: {:?}",
        agent.terminal
    );
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
    let refused = host_stream(&world, "trial")?.refused;
    ensure!(
        refused
            .as_deref()
            .is_some_and(|words| words.contains("trial") && words.contains("rex")),
        "status's refusal of trial: {refused:?}"
    );
    ensure_eq!(
        host_stream(&world, "work")?.refused,
        None,
        "status's refusal of work"
    );
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

/// The repository's kind `pair`, which lands, of two stages each
/// delivering commits.
fn pair(world: &World) {
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "pair",
            true,
            vec![
                stage("one", Some(Deliverable::Commits), "{{brief}}"),
                stage("two", Some(Deliverable::Commits), "{{brief}}"),
            ],
        )],
    );
}

fn a_place_in_two_terminals(_: &()) -> Outcome {
    let world = World::new();
    let other = work_and_trial(&world);
    pair(&world);
    launched(&world.engine.launch(&Launch {
        stream: Some("trial".into()),
        ..launch("pair", "one", Some("x"))
    }))?;
    let id = World::id("pair", "one");
    let one = World::identity("pair", "one", "one", 1);
    let two = World::identity("pair", "one", "two", 1);
    ensure!(
        other.opened(&one).is_some(),
        "stage one did not open in other"
    );
    world.host.set_display("trial", Some("fake"), None, None);
    world.commit("pair/one", &["one.rs"]);
    world.settle(&id);
    ensure!(
        world.terminal.opened(&two).is_some(),
        "stage two did not open in fake"
    );
    world.commit("pair/one", &["two.rs"]);
    let mut tidied = vec![];
    for _ in 0..3 {
        if let flume_core::Answer::Settled { tidy, .. } = world.engine().settle(Some(&id)) {
            tidied.extend(tidy.terminal);
        }
    }
    ensure!(world.held(&id).is_none(), "the landed place still stands");
    ensure_eq!(
        (other.log.count("tidy"), world.terminal.log.count("tidy")),
        (1, 1),
        "the tidies asked of other and of fake"
    );
    let closed = vec![
        "the tab pair--one in trial".to_string(),
        "the workspace repository main in trial".to_string(),
    ];
    ensure_eq!(
        tidied
            .iter()
            .map(|tidied| (tidied.profile.clone(), tidied.closed.clone()))
            .collect::<Vec<_>>(),
        vec![
            (Some("other".to_string()), closed.clone()),
            (Some("fake".to_string()), closed)
        ],
        "which terminal closed what, in the tidy's answer"
    );
    Ok(())
}

fn a_caller_under_a_streams_terminal(_: &()) -> Outcome {
    let world = World::new();
    let other = work_and_trial(&world);
    other.set_session(Some("caller-1"));
    other.add_session("caller-1", AgentState::Waiting);
    let fix = fix_on(&world, "work", "one")?;
    let id = World::id("fix", "one");
    ensure_eq!(
        world.held(&id).ok_or("no held state")?.place.recipients,
        vec!["caller-1".to_string()],
        "the fix's recipients"
    );
    ensure!(
        world.terminal.opened(&fix).is_some(),
        "the work fix's agent did not open in fake"
    );
    world.commit("fix/one", &["fix.rs"]);
    world.settle(&id);
    let notices = other.notices("caller-1");
    ensure!(
        notices
            .first()
            .is_some_and(|notice| notice.contains("code delivered")),
        "the notices through other: {notices:?}"
    );
    ensure!(
        world.terminal.every_notice().is_empty(),
        "fake was sent notices: {:?}",
        world.terminal.every_notice()
    );
    Ok(())
}

/// The stream-terminals spec's "Two terminals and no watch": no caller
/// keeps watch, the host's terminal is `fake` and `trial`'s `other`; a
/// caller's act settles every place first under either, and a wait drives
/// its settles (119, 122, 190, 208).
fn two_terminals_and_no_watch(_: &()) -> Outcome {
    let world = World::new();
    let other = work_and_trial(&world);
    world.terminal.add_session("ops", AgentState::Working);
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&Launch {
        stream: Some("work".into()),
        recipients: vec!["ops".into()],
        ..launch("fix", "one", Some("x"))
    }))?;
    let trial = fix_on(&world, "trial", "two")?;
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    let waited = world.engine().wait(
        &id,
        &flume_core::WaitFor::Stage("code".into()),
        "ops",
        Some(10),
    );
    ensure!(
        matches!(&waited, flume_core::Answer::Waited { end: Some(_), .. }),
        "the wait answered no end: {waited:?}"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "the stage once the wait settled it"
    );
    other.kill(&trial);
    let two = World::id("fix", "two");
    ensure_eq!(
        world.status(&two)?.stages[0].state,
        StageState::Begun,
        "the trial stage, under other, before a command acts"
    );
    let dropped = world.engine.drop_place(&id, true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.status(&two)?.stages[0].state,
        StageState::Lost,
        "the trial stage, under other, once a command acted"
    );
    Ok(())
}

fn s91_a_stream_under_its_own_terminal(_: &()) -> Outcome {
    let world = World::new();
    world.engine.settle_every(30);
    let other = work_and_trial(&world);
    let work = fix_on(&world, "work", "one")?;
    ensure_eq!(
        world.terminal.opened_at(&work),
        Some(OpenIn::Session {
            name: "work".into()
        }),
        "the work fix's agent"
    );
    launched(&world.engine.launch(&Launch {
        stream: Some("trial".into()),
        ..launch("change", "two", Some("plan it"))
    }))?;
    let change = World::id("change", "two");
    world.settle(&change);
    let construct = World::identity("change", "two", "construct", 1);
    ensure_eq!(
        other.opened_at(&construct),
        Some(OpenIn::Session {
            name: "trial".into()
        }),
        "the trial change's construct agent"
    );
    world.terminal.kill(&work);
    let moved = world.engine.priority(&change, 1);
    ensure!(
        moved.refusal().is_none(),
        "the priority was refused: {moved:?}"
    );
    ensure_eq!(
        world.stage_state(&World::id("fix", "one"), "code"),
        Some(StageState::Begun),
        "the work fix's stage once a caller acted under the watch, trial nudging nothing"
    );
    world.engine.settle_every(30);
    ensure_eq!(
        world.stage_state(&World::id("fix", "one"), "code"),
        Some(StageState::Lost),
        "the work fix's stage at the caller's settle"
    );
    world.host.set_display("trial", Some("fake"), None, None);
    write_change(&world, "two");
    world.settle(&change);
    world.settle(&change);
    ensure!(
        other.log.count("close") == 1 && !heard_of(&world.terminal, &construct),
        "the construct agent was not ended where it opened: {:?}",
        other.log.calls()
    );
    let code = World::identity("change", "two", "code", 1);
    ensure_eq!(
        world.terminal.opened_at(&code),
        Some(OpenIn::Session {
            name: "trial".into()
        }),
        "the trial change's code agent"
    );
    ensure_eq!(
        world
            .status(&change)?
            .agent
            .and_then(|agent| agent.terminal),
        profile("fake", NamedBy::Stream),
        "status's terminal of the trial change's code agent"
    );
    world.host.set_display("lab", Some("rex"), None, None);
    let reason = refused(&world.engine.launch(&Launch {
        stream: Some("lab".into()),
        ..launch("fix", "three", Some("x"))
    }))?;
    ensure!(reason.contains("rex"), "the refusal: {reason}");
    fix_on(&world, "work", "four")?;
    fix_on(&world, "trial", "five")?;
    for name in ["four", "five"] {
        ensure_eq!(
            world.stage_state(&World::id("fix", name), "code"),
            Some(StageState::Begun),
            "the {name} fix's code stage"
        );
    }
    Ok(())
}

/// The stream `name` of status's host part.
fn host_stream(world: &World, name: &str) -> Result<flume_core::StreamStatus, String> {
    let flume_core::Answer::Status { host, .. } = world.engine().status(None) else {
        return Err("status answered no status".into());
    };
    host.streams
        .into_iter()
        .find(|stream| stream.name == name)
        .ok_or_else(|| format!("the host's streams give no {name}"))
}

fn the_terminals_in_status(_: &()) -> Outcome {
    let world = World::new();
    work_and_trial(&world);
    fix_on(&world, "work", "one")?;
    fix_on(&world, "trial", "two")?;
    ensure_eq!(
        (
            Some(host_stream(&world, "work")?.multiplexer),
            Some(host_stream(&world, "trial")?.multiplexer)
        ),
        (
            profile("fake", NamedBy::Host),
            profile("other", NamedBy::Stream)
        ),
        "each stream's multiplexer"
    );
    for (name, expected) in [
        ("one", profile("fake", NamedBy::Host)),
        ("two", profile("other", NamedBy::Stream)),
    ] {
        let agent = world
            .status(&World::id("fix", name))?
            .agent
            .ok_or_else(|| format!("the {name} fix has no agent"))?;
        ensure_eq!(
            agent.terminal,
            expected,
            "the {name} fix's agent's terminal"
        );
    }
    let flume_core::Answer::Status { host, .. } = world.engine().status(None) else {
        return Err("status answered no status".into());
    };
    ensure_eq!(
        host.terminals,
        vec![
            flume_core::TerminalStatus {
                profile: Some("fake".into()),
                kept: vec![],
                watch: Watch::None,
            },
            flume_core::TerminalStatus {
                profile: Some("other".into()),
                kept: vec![],
                watch: Watch::None,
            },
        ],
        "the host's terminals"
    );
    let json = serde_json::to_value(&host).map_err(|error| error.to_string())?;
    ensure_eq!(
        (
            json["terminals"][1]["profile"].as_str(),
            json["terminals"][0]["watch"].as_str(),
            json["terminals"][0].as_object().map(|answer| answer.len())
        ),
        (Some("other"), Some("none"), Some(3)),
        "the host's terminals in the JSON form"
    );
    Ok(())
}

fn a_stage_retried_in_another_terminal(_: &()) -> Outcome {
    let world = World::new();
    let other = work_and_trial(&world);
    let first = fix_on(&world, "trial", "one")?;
    let id = World::id("fix", "one");
    ensure!(
        other.opened(&first).is_some(),
        "the first start did not open in other"
    );
    other.kill(&first);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "the first start's stage"
    );
    world.host.set_display("trial", Some("fake"), None, None);
    let retried = world
        .engine()
        .retry(&id, flume_core::Retry::Current, None, false);
    ensure!(
        retried.refusal().is_none(),
        "the retry was refused: {retried:?}"
    );
    let second = World::identity("fix", "one", "code", 2);
    ensure!(
        world.terminal.opened(&second).is_some(),
        "the retry did not open in fake"
    );
    world.commit("fix/one", &["fix.rs"]);
    for _ in 0..3 {
        world.settle(&id);
    }
    ensure!(world.held(&id).is_none(), "the landed place still stands");
    ensure_eq!(
        (other.log.count("tidy"), world.terminal.log.count("tidy")),
        (1, 1),
        "the tidies asked of other and of fake"
    );
    ensure_eq!(
        other.made(),
        (vec![], vec![]),
        "what other still holds of the place"
    );
    Ok(())
}

/// The stream-terminals spec's "One stream under another terminal": no
/// terminal decides whether flume runs attended, so under a caller's
/// watch a drop on `work` settles nothing first, and `trial`'s place,
/// under `other`, moves at the caller's next settle (119, 122, 190, 208,
/// S91).
fn one_stream_under_another_terminal(_: &()) -> Outcome {
    let world = World::new();
    let other = work_and_trial(&world);
    let trial = fix_on(&world, "trial", "one")?;
    fix_on(&world, "work", "two")?;
    world.engine.settle_every(30);
    other.kill(&trial);
    let one = World::id("fix", "one");
    let dropped = world.engine.drop_place(&World::id("fix", "two"), true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.stage_state(&one, "code"),
        Some(StageState::Begun),
        "the trial stage once a caller dropped a place on work under the watch"
    );
    world.host.advance(30);
    world.engine.settle_every(30);
    ensure_eq!(
        world.stage_state(&one, "code"),
        Some(StageState::Lost),
        "the trial stage at the caller's settle"
    );
    Ok(())
}

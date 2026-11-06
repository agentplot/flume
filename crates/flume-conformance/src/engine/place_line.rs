//! A place's one line (56, 57, 58, 139, 142): its parts, its state in
//! words, read from its held state, the change at its head and its live
//! agent's asking mark alone, never the terminal or the merge queue, and
//! the same line in status (60; S14).

use super::account_pools::begun_fix;
use super::caller_waits::construct_asks;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, PlaceLine, Retry, NOT_ITS_AGENT};
use flume_fakes::Plan;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "place line: a change@2 place at its code stage's first of two groups gives `change--parser · change@2 · code 1/2 · run 1 · tasks 0/3 · working`, and its short form less its name and kind",
            cites: &["56", "142", "48"],
            run: a_change_at_its_first_group,
        },
        Scenario {
            name: "place line: a fix at its code stage's second start gives `fix--parser · fix@1 · code · run 2 · working`, with no tasks part",
            cites: &["56", "48"],
            run: a_fix_at_its_second_start,
        },
        Scenario {
            name: "place line: a fix landed and tidied, its document kept for an end pending, gives `fix--parser · fix@1 · landed`, with no stage, run or tasks part",
            cites: &["63", "43", "62"],
            run: a_landed_place,
        },
        Scenario {
            name: "place line: a code stage on hold under no words is held, and stalled five minutes later",
            cites: &["47", "130"],
            run: held_then_stalled,
        },
        Scenario {
            name: "place line: a construct stage stopped short needing an answer is waiting on an answer",
            cites: &["42", "130"],
            run: waiting_on_an_answer,
        },
        Scenario {
            name: "place line: a started code agent whose profile's asking mark stands is asking, and working once it is cleared",
            cites: &["90", "56"],
            run: asking_then_working,
        },
        Scenario {
            name: "place line: an agent opened that has not said it started is starting, and another place's first stage waiting for room is waiting for room",
            cites: &["35", "70"],
            run: starting_and_waiting_for_room,
        },
        Scenario {
            name: "place line: a fix whose last stage delivered and was submitted is submitted, and landed after the settle that reads its landing",
            cites: &["48", "63"],
            run: submitted_then_landed,
        },
        Scenario {
            name: "place line: asked alone while its code agent works, the line calls no terminal, merge-queue or host operation and writes nothing",
            cites: &["57", "58"],
            run: nothing_called_and_nothing_written,
        },
        Scenario {
            name: "place line: a change@2 place whose change cannot be read gives its stage's name alone and no tasks part",
            cites: &["58", "142"],
            run: a_change_that_cannot_be_read,
        },
        Scenario {
            name: "place line: asked for a repetition's earlier agent once its code stage moved to the second group, the line names `code 2/2` and ends saying the session is not its agent, its short form not",
            cites: &["140", "35", "142"],
            run: a_repetitions_earlier_agent,
        },
        Scenario {
            name: "place line: status's line is the line asked alone, of a fix and of a change at its code stage",
            cites: &["S14", "60", "56"],
            run: asked_two_ways,
        },
    ]
}

/// The task groups of a change of two groups and three tasks.
pub const TWO_GROUPS: &[(&str, &str, usize)] = &[("1", "One", 2), ("2", "Two", 1)];

/// The change place `name` of flume's own change@2, its change written
/// as `groups` none ticked, and code's first repetition begun.
pub fn code_of_change(
    world: &World,
    name: &str,
    groups: &[(&str, &str, usize)],
) -> Result<PlaceId, String> {
    launched(
        &world
            .engine
            .launch(&launch("change", name, Some("plan it"))),
    )?;
    let id = World::id("change", name);
    world.settle(&id);
    world.tick_groups(&format!("change/{name}"), name, groups, 0);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code's first repetition"
    );
    Ok(id)
}

/// The place's line asked alone, with no session's identity.
fn line(world: &World, id: &PlaceId) -> Result<PlaceLine, String> {
    world
        .engine()
        .line(id, None)
        .ok_or_else(|| format!("no line of {id}"))
}

/// A fix named `name` whose ends are pending for a role's address that
/// no run collects, so its document stays once it is tidied, its code
/// stage begun.
pub fn fix_for_a_role(world: &World, name: &str) -> Result<PlaceId, String> {
    launched(&world.engine.launch(&Launch {
        recipients: vec!["madswan/planner".into()],
        ..launch("fix", name, Some("x"))
    }))?;
    let id = World::id("fix", name);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "{name}'s code"
    );
    Ok(id)
}

fn a_change_at_its_first_group(_: &()) -> Outcome {
    let world = World::new();
    let id = code_of_change(&world, "parser", TWO_GROUPS)?;
    let line = line(&world, &id)?;
    ensure_eq!(
        line.full.as_str(),
        "change--parser · change@2 · code 1/2 · run 1 · tasks 0/3 · working",
        "the line"
    );
    ensure_eq!(
        line.short.as_str(),
        "code 1/2 · run 1 · tasks 0/3 · working",
        "the short form"
    );
    ensure_eq!(
        (
            line.name.as_str(),
            line.kind.as_str(),
            line.stage.as_deref(),
            line.run,
            line.tasks.as_deref(),
            line.state.as_str()
        ),
        (
            "change--parser",
            "change@2",
            Some("code 1/2"),
            Some(1),
            Some("0/3"),
            "working"
        ),
        "the parts"
    );
    Ok(())
}

fn a_fix_at_its_second_start(_: &()) -> Outcome {
    let world = World::new();
    let id = begun_fix(&world, "parser")?;
    let stopped = world.engine.stop(&id, true);
    ensure!(
        stopped.refusal().is_none(),
        "the stop was refused: {stopped:?}"
    );
    let retried = world.engine.retry(&id, Retry::Current, None, false);
    ensure!(
        retried.refusal().is_none(),
        "the retry was refused: {retried:?}"
    );
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code's second start"
    );
    ensure_eq!(
        line(&world, &id)?.full,
        "fix--parser · fix@1 · code · run 2 · working",
        "the line"
    );
    Ok(())
}

fn a_landed_place(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_for_a_role(&world, "parser")?;
    world.commit("fix/parser", &["fix.rs"]);
    world.settle(&id);
    world.settle(&id);
    let line = line(&world, &id)?;
    ensure!(
        world
            .store
            .read_every()
            .iter()
            .any(|read| matches!(read.state.phase, Phase::Tidied { .. })),
        "no tidied document kept for the end pending"
    );
    ensure_eq!(
        line.full.as_str(),
        "fix--parser · fix@1 · landed",
        "the line"
    );
    ensure_eq!(
        (line.stage, line.run, line.tasks),
        (None, None, None),
        "the stage, run and tasks parts"
    );
    Ok(())
}

fn held_then_stalled(_: &()) -> Outcome {
    let world = World::new();
    launched(&world.engine.launch(&Launch {
        hold: vec!["code".into()],
        ..launch("fix", "parser", Some("x"))
    }))?;
    let id = World::id("fix", "parser");
    world.settle(&id);
    let held = line(&world, &id)?;
    ensure_eq!(held.state.as_str(), "held", "the state while held");
    ensure_eq!(
        held.full.as_str(),
        "fix--parser · fix@1 · code · held",
        "the line while held"
    );
    world.host.advance(STALL_AFTER);
    world.settle(&id);
    ensure_eq!(
        line(&world, &id)?.state,
        "stalled",
        "the state five minutes later"
    );
    Ok(())
}

fn waiting_on_an_answer(_: &()) -> Outcome {
    let world = World::new();
    let (id, _) = construct_asks(&world, "plan", &[], None)?;
    ensure_eq!(
        line(&world, &id)?.state,
        "waiting on an answer",
        "the state"
    );
    Ok(())
}

fn asking_then_working(_: &()) -> Outcome {
    let world = World::new();
    let id = begun_fix(&world, "parser")?;
    let code = World::identity("fix", "parser", "code", 1);
    world.agent.set_asking(&code, Reading::Yes);
    ensure_eq!(
        line(&world, &id)?.state,
        "asking",
        "the state while the mark stands"
    );
    world.agent.set_asking(&code, Reading::No);
    ensure_eq!(
        line(&world, &id)?.state,
        "working",
        "the state once it is cleared"
    );
    Ok(())
}

fn starting_and_waiting_for_room(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(1));
    world
        .agent
        .set_started(&World::identity("fix", "one", "code", 1), Reading::No);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let one = World::id("fix", "one");
    world.settle(&one);
    launched(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    let two = World::id("fix", "two");
    world.settle(&two);
    ensure_eq!(
        (
            world.stage_state(&one, "code"),
            world.stage_state(&two, "code")
        ),
        (Some(StageState::Starting), Some(StageState::WaitingForRoom)),
        "the two code stages"
    );
    ensure_eq!(
        (line(&world, &one)?.state, line(&world, &two)?.state),
        ("starting".to_string(), "waiting for room".to_string()),
        "the two states"
    );
    Ok(())
}

fn submitted_then_landed(_: &()) -> Outcome {
    let world = World::new();
    world.queue.moves_on_its_own();
    let id = fix_for_a_role(&world, "parser")?;
    world.commit("fix/parser", &["fix.rs"]);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "the submits");
    ensure_eq!(
        line(&world, &id)?.full,
        "fix--parser · fix@1 · code · run 1 · submitted",
        "the line once submitted"
    );
    world.queue.decide(REPOSITORY, "fix/parser", Plan::Land);
    ensure_eq!(
        line(&world, &id)?.state,
        "submitted",
        "the state before a settle reads the landing"
    );
    world.settle(&id);
    ensure_eq!(
        line(&world, &id)?.state,
        "landed",
        "the state after the settle"
    );
    Ok(())
}

fn nothing_called_and_nothing_written(_: &()) -> Outcome {
    let world = World::new();
    let id = begun_fix(&world, "parser")?;
    let version = |world: &World| world.store.read(&id).map(|read| read.version);
    let before = version(&world);
    world.terminal.log.clear();
    world.queue.log.clear();
    world.host.log.clear();
    world.store.log.clear();
    let line = line(&world, &id)?;
    ensure_eq!(line.state.as_str(), "working", "the state");
    ensure_eq!(
        world.terminal.log.calls(),
        Vec::<String>::new(),
        "the terminal's calls"
    );
    ensure_eq!(
        world.queue.log.calls(),
        Vec::<String>::new(),
        "the merge queue's calls"
    );
    ensure_eq!(
        world.host.log.calls(),
        Vec::<String>::new(),
        "the host's calls"
    );
    ensure!(
        world
            .store
            .log
            .calls()
            .iter()
            .all(|call| call.starts_with("read ")),
        "the store was asked more than a read: {:?}",
        world.store.log.calls()
    );
    ensure_eq!(version(&world), before, "the place's version");
    Ok(())
}

fn a_change_that_cannot_be_read(_: &()) -> Outcome {
    let world = World::new();
    let id = code_of_change(&world, "parser", TWO_GROUPS)?;
    world.change.unreadable(REPOSITORY);
    let line = line(&world, &id)?;
    ensure_eq!(
        line.full.as_str(),
        "change--parser · change@2 · code · run 1 · working",
        "the line"
    );
    ensure_eq!(line.tasks, None, "the tasks part");
    Ok(())
}

fn a_repetitions_earlier_agent(_: &()) -> Outcome {
    let world = World::new();
    let groups: &[(&str, &str, usize)] = &[("1", "One", 1), ("2", "Two", 1)];
    let id = code_of_change(&world, "parser", groups)?;
    let first = World::identity("change", "parser", "code", 1);
    ensure!(
        !world
            .engine()
            .line(&id, Some(&first))
            .ok_or("no line")?
            .full
            .ends_with(NOT_ITS_AGENT),
        "the first repetition's own agent is told it is not its agent"
    );
    world.tick_groups("change/parser", "parser", groups, 1);
    world.settle(&id);
    world.settle(&id);
    let line = world.engine().line(&id, Some(&first)).ok_or("no line")?;
    ensure_eq!(line.stage.as_deref(), Some("code 2/2"), "the stage named");
    ensure!(
        line.full.ends_with(&format!(" · {NOT_ITS_AGENT}")),
        "the line: {}",
        line.full
    );
    ensure!(
        !line.short.contains(NOT_ITS_AGENT),
        "the short form: {}",
        line.short
    );
    Ok(())
}

fn asked_two_ways(_: &()) -> Outcome {
    let world = World::new();
    let fix = begun_fix(&world, "parser")?;
    let change = code_of_change(&world, "lexer", TWO_GROUPS)?;
    for id in [&fix, &change] {
        ensure_eq!(
            world.status(id)?.line,
            line(&world, id)?,
            "{id}'s line in status and alone"
        );
    }
    let Answer::Lines { lines } = world.engine().lines(None) else {
        return Err("the lines were refused".into());
    };
    let Answer::Status { places, .. } = world.engine().status(None) else {
        return Err("status was refused".into());
    };
    ensure_eq!(
        lines,
        places
            .into_iter()
            .map(|place| place.line)
            .collect::<Vec<_>>(),
        "every place's line"
    );
    Ok(())
}

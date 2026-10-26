//! Settling (40, 46, 111 to 113, 119 to 122, 124): what moves a place on
//! with no process of flume's own, and a landing kind's submission.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name:
                "settle: a commit and a quiet agent end the stage and start the next in one settle",
            cites: &["S29", "40", "46", "121", "30"],
            run: s29_a_commit_and_a_quiet_agent,
        },
        Scenario {
            name: "settle: a delivered stage whose next waits on its caller starts nothing more",
            cites: &["S7", "47", "70", "I6"],
            run: s7_two_held_places,
        },
        Scenario {
            name: "settle: with nobody watching, a killed agent is read only when a command acts",
            cites: &["S27", "119", "I8", "57"],
            run: s27_killed_with_nobody_watching,
        },
        Scenario {
            name: "settle: under a watch, a crashed agent reads lost within its bound",
            cites: &["S28", "122", "46", "120"],
            run: s28_crashed_under_a_watch,
        },
        Scenario {
            name: "settle: a settle repeated changes nothing",
            cites: &["74", "46"],
            run: a_settle_repeated,
        },
        Scenario {
            name: "settle: a place whose launching command died is undone",
            cites: &["2", "76", "17"],
            run: a_launch_that_died,
        },
    ]
}

/// A change's construct stage begun, its agent waiting.
fn construct_begun(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = World::id("change", name);
    launched(
        &world
            .engine
            .launch(&launch("change", name, Some("plan it"))),
    )?;
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::Begun),
        "construct"
    );
    Ok(id)
}

/// The change written on the place's branch, its planning complete.
fn write_change(world: &World, name: &str) {
    let revision = world.commit(
        &format!("change/{name}"),
        &[&format!("plans/{name}/design.md")],
    );
    world.change.set(
        REPOSITORY,
        &revision,
        name,
        ChangeFacts {
            exists: true,
            planning_complete: true,
            tasks: vec![Task {
                text: "1.1 do it".into(),
                ticked: false,
            }],
            archived: false,
        },
    );
}

fn s29_a_commit_and_a_quiet_agent(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    write_change(&world, "plan");
    world.terminal.log.clear();
    world.settle(&id);
    let calls: Vec<String> = world
        .terminal
        .log
        .calls()
        .into_iter()
        .filter(|call| call.starts_with("close") || call.starts_with("open"))
        .collect();
    ensure_eq!(
        calls,
        vec![
            format!(
                "close {}",
                World::identity("change", "plan", "construct", 1)
            ),
            format!(
                "open {} {}",
                World::identity("change", "plan", "code", 1),
                id
            ),
        ],
        "the settle's closes and opens"
    );
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::Delivered),
        "construct"
    );
    ensure!(
        world
            .stage_state(&id, "code")
            .is_some_and(StageState::running),
        "code did not start"
    );
    Ok(())
}

fn s7_two_held_places(_: &()) -> Outcome {
    let world = World::new();
    let mut ids = vec![];
    for name in ["one", "two"] {
        let mut held = launch("change", name, Some("plan it"));
        held.hold = vec!["code".into(), "verify".into()];
        held.hold_submission = true;
        held.mark = Some("team-a".into());
        launched(&world.engine.launch(&held))?;
        let id = World::id("change", name);
        world.settle(&id);
        write_change(&world, name);
        world.settle(&id);
        ensure_eq!(
            world.stage_state(&id, "construct"),
            Some(StageState::Delivered),
            "construct"
        );
        ensure_eq!(
            world.stage_state(&id, "code"),
            Some(StageState::OnHold),
            "code"
        );
        ids.push(id);
    }
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    let answer = world
        .engine
        .retry(&ids[0], Retry::Stage("code".into()), None, false);
    ensure!(
        answer.refusal().is_none(),
        "the retry was refused: {answer:?}"
    );
    ensure!(
        world
            .stage_state(&ids[0], "code")
            .is_some_and(StageState::running),
        "the first place's code stage"
    );
    ensure_eq!(
        world.stage_state(&ids[1], "code"),
        Some(StageState::OnHold),
        "the second's"
    );
    ensure_eq!(world.place.log.count("make"), 2, "places made");
    Ok(())
}

fn s27_killed_with_nobody_watching(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    world.host.advance(3600);
    ensure_eq!(
        world.status(&id)?.stages[0].state,
        StageState::Begun,
        "the stage before a command acts"
    );
    launched(&world.engine.launch(&launch("fix", "two", Some("y"))))?;
    ensure_eq!(
        world.status(&id)?.stages[0].state,
        StageState::Lost,
        "the stage once a command acted"
    );
    Ok(())
}

fn s28_crashed_under_a_watch(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_watch(Watch::Bound { seconds: 30 });
    world.agent.no_on_quiet();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    launched(&world.engine.launch(&launch("fix", "two", Some("y"))))?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "a launch under a watch settles not"
    );
    world.host.advance(30);
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "the stage the watch read"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn a_settle_repeated(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    write_change(&world, "plan");
    world.settle(&id);
    world.settle(&id);
    let held = world.held(&id).ok_or("no place")?;
    let writes = world.store.log.count("write");
    world.settle(&id);
    ensure_eq!(world.held(&id), Some(held), "the held state a repeat left");
    ensure_eq!(
        world.store.log.count("write"),
        writes,
        "writes a repeat made"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn a_launch_that_died(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let mut held = world.held(&id).ok_or("no place")?;
    held.phase = Phase::Making {
        by: world.host.another_command(),
    };
    world.store.put(&id, held);
    world.settle(&id);
    ensure!(
        world.held(&id).is_some(),
        "a place its live launch makes was undone"
    );
    if let Some(Phase::Making { by }) = world.held(&id).map(|held| held.phase) {
        world.host.end_command(&by);
    }
    world.engine().settle(None);
    ensure!(
        world.held(&id).is_none(),
        "the dead launch's held state is left"
    );
    ensure!(
        !world.place.exists(&id.0),
        "the dead launch's worktree is left"
    );
    Ok(())
}

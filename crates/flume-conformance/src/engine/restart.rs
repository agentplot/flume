//! A restart (77): a core built anew over the same store and seams reads
//! every place again, and loses nothing flume held.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "restart: while code works, the held state is whole and the stage reads lost",
            cites: &["S12", "77", "45", "48", "73", "I8"],
            run: s12_the_host_restarts,
        },
        Scenario {
            name: "restart: an agent resumed under the stage's identity with another record reads lost",
            cites: &["77", "35"],
            run: a_resumed_agent_with_another_record,
        },
        Scenario {
            name: "restart: an agent resumed under the stage's identity whose record cannot be read is neither taken nor lost",
            cites: &["77", "93", "31"],
            run: a_resumed_agent_whose_record_cannot_be_read,
        },
    ]
}

fn s12_the_host_restarts(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    let code = World::identity("fix", "one", "code", 1);
    world.terminal.set_state(&code, AgentState::Working);
    world.settle(&id);
    let before = world.held(&id).ok_or("no place")?;
    world.host.restart();
    world.terminal.forget_all();
    let rebuilt = world.engine();
    let after = world.held(&id).ok_or("the place's held state was lost")?;
    ensure_eq!(after, before, "the held state after a restart");
    rebuilt.settle(None);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "the code stage"
    );
    world.engine().settle(None);
    ensure_eq!(
        world.terminal.log.count("open"),
        1,
        "agents opened before a retry"
    );
    let answer = world.engine().retry(&id, Retry::Current, None, false);
    ensure!(
        answer.refusal().is_none(),
        "the retry was refused: {answer:?}"
    );
    ensure_eq!(
        world.terminal.log.count("open"),
        2,
        "agents opened after a retry"
    );
    Ok(())
}

fn a_resumed_agent_with_another_record(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    let code = World::identity("fix", "one", "code", 1);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage"
    );
    world.host.restart();
    world.terminal.forget_all();
    world.terminal.resume(&code);
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "a resumed agent with its own record"
    );
    world.host.restart();
    world.terminal.forget_all();
    world.terminal.resume(&code);
    world.agent.set_record(&code, "a session begun afresh");
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "a resumed agent with another record"
    );
    Ok(())
}

fn a_resumed_agent_whose_record_cannot_be_read(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    let code = World::identity("fix", "one", "code", 1);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage"
    );
    world.host.restart();
    world.terminal.forget_all();
    world.terminal.resume(&code);
    world.agent.received_unknown(&code);
    world.commit("fix/one", &["fix.rs"]);
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "a resumed agent whose record cannot be read, its deliverable made"
    );
    ensure_eq!(world.terminal.log.count("close"), 0, "agents ended");
    let status = world.status(&id)?;
    ensure!(
        status
            .cannot_read
            .iter()
            .any(|unread| unread == "the agent's own record"),
        "status names not the agent's record among what it cannot read: {:?}",
        status.cannot_read
    );
    Ok(())
}

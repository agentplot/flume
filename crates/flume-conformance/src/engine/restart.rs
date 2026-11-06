//! A restart (77): a core built anew over the same store and seams reads
//! every place again, and loses nothing flume held.

use super::carried_conversations::{talk, talking};
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
        Scenario {
            name: "restart: an agent the terminal brings back bare and quiet in a talk's pane is left there, closed and sent nothing over three settles, its start live until the terminal reads it stopped",
            cites: &["S113", "31", "77"],
            run: a_bare_agent_in_a_conversations_pane,
        },
        Scenario {
            name: "restart: an agent the terminal brings back bare and quiet in a code stage's pane is ended by the settle that reads code lost",
            cites: &["S116", "S41", "31", "77"],
            run: a_bare_agent_in_a_code_stages_pane,
        },
        Scenario {
            name: "restart: the caller's stop closes the agent the terminal brought back bare in a lost talk's pane",
            cites: &["51", "77"],
            run: a_stop_ends_the_agent_left,
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

/// The research place `one`'s talk, its host restarted and its agent
/// brought back bare and quiet by the fake terminal, settled once.
fn brought_back_bare(world: &World) -> Result<PlaceId, String> {
    let id = talking(world)?;
    world.host.restart();
    world.terminal.forget_all();
    world.agent.bring_back_bare(&talk(1));
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Lost),
        "talk after the first settle"
    );
    Ok(id)
}

/// Whether talk's start 1 is live, as held state gives it.
fn talk_live(world: &World, id: &PlaceId) -> Result<bool, String> {
    world
        .held(id)
        .ok_or("no held state")?
        .stages
        .into_iter()
        .find(|stage| stage.name == "talk")
        .and_then(|stage| stage.start)
        .map(|start| start.live)
        .ok_or_else(|| "talk has no start".to_string())
}

fn a_bare_agent_in_a_conversations_pane(_: &()) -> Outcome {
    let world = World::new();
    let id = brought_back_bare(&world)?;
    let sent = world.terminal.sent_to(&talk(1)).len();
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        (
            world.terminal.log.of("close"),
            world.terminal.sent_to(&talk(1)).len(),
            talk_live(&world, &id)?
        ),
        (vec![], sent, true),
        "the agents closed, what talk's agent was sent and whether its start is live"
    );
    world.terminal.kill(&talk(1));
    world.settle(&id);
    ensure_eq!(
        (talk_live(&world, &id)?, world.terminal.log.count("close")),
        (false, 0),
        "talk's start once the terminal read its agent stopped, and the agents closed"
    );
    Ok(())
}

fn a_bare_agent_in_a_code_stages_pane(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    let code = World::identity("fix", "one", "code", 1);
    world.host.restart();
    world.terminal.forget_all();
    world.agent.bring_back_bare(&code);
    world.engine().settle(None);
    ensure_eq!(
        (
            world.stage_state(&id, "code"),
            world.terminal.log.of("close")
        ),
        (Some(StageState::Lost), vec![format!("close {code}")]),
        "code and the agents closed"
    );
    let end = world
        .held(&id)
        .and_then(|held| held.stages[0].end.clone())
        .ok_or("code holds no end")?;
    ensure!(!end.left_in_pane, "code's end says an agent was left");
    Ok(())
}

fn a_stop_ends_the_agent_left(_: &()) -> Outcome {
    let world = World::new();
    let id = brought_back_bare(&world)?;
    let stopped = world.engine().stop(&id, false);
    ensure!(
        stopped.refusal().is_none(),
        "the stop was refused: {stopped:?}"
    );
    ensure_eq!(
        (
            world.terminal.log.of("close"),
            talk_live(&world, &id)?,
            world.stage_state(&id, "talk")
        ),
        (
            vec![format!("close {}", talk(1))],
            false,
            Some(StageState::Lost)
        ),
        "the agents closed, talk's start and talk once stopped"
    );
    Ok(())
}

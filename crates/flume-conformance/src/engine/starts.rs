//! A start (34, 35, 37, 38, 43, 77, 90, 134 to 136; record 0016): it is
//! starting until its agent says it started under its start's identity,
//! no prompt is sent before that word, a start that never says it is
//! reported never begun with what the terminal shows, and what a terminal
//! reads of an agent is taken only as far as the agent's own record bears
//! it out.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, QUIET_LIMIT};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "start: the folder-trust question answered, then the prompt sent only once the agent said it started",
            cites: &["S39", "34", "35", "37", "38", "135"],
            run: s39_started_then_prompted,
        },
        Scenario {
            name: "start: held at a dialog, nothing answered, never begun at the stage's limit with the screen",
            cites: &["S40", "34", "35", "43", "44", "136"],
            run: s40_held_at_a_dialog,
        },
        Scenario {
            name: "start: held at the bypass warning by a terminal that says so, never begun at once with the warning, no prompt sent",
            cites: &["S40", "34", "35", "43", "94"],
            run: s40_held_never_begun_at_once,
        },
        Scenario {
            name: "start: a program that never comes up within the terminal's bound is never begun with the screen, no prompt sent",
            cites: &["S45", "35", "38", "43", "94"],
            run: s45_never_came_up,
        },
        Scenario {
            name: "clues: the pane's title follows the stage's state at each settle that changes it, a failed clue logged, nothing read back",
            cites: &["S49", "139", "46", "56"],
            run: s49_clues_follow_the_stage,
        },
        Scenario {
            name: "identity: an agent's identity names its repository by the id of its common git directory",
            cites: &["33", "29"],
            run: the_identity_names_the_repository_by_its_id,
        },
        Scenario {
            name: "start: an agent that stops before it says it started is never begun at once",
            cites: &["35", "43", "96"],
            run: stopped_before_it_started,
        },
        Scenario {
            name: "start: a caller's stop while the start is starting reports it never begun",
            cites: &["35", "51"],
            run: stopped_while_starting,
        },
        Scenario {
            name: "restart: a session brought back bare says it started nothing, reads lost with its last words and commits, and is ended once quiet",
            cites: &["S41", "31", "43", "45", "77", "135"],
            run: s41_brought_back_bare,
        },
        Scenario {
            name: "state: a finished turn nobody looked at reads waiting: the stage delivers, its agent is ended and the next starts",
            cites: &["S42", "40", "96"],
            run: s42_a_finished_turn,
        },
        Scenario {
            name: "prompt: left typed and never submitted, cleared before the second sending, which is received",
            cites: &["S43", "38", "95"],
            run: s43_a_prompt_left_typed,
        },
        Scenario {
            name: "state: a terminal's blocked with no asking mark reads unknown, and nothing is typed",
            cites: &["90", "37", "96"],
            run: blocked_without_a_mark,
        },
        Scenario {
            name: "start: no prompt is sent while another command still opens the agent, and a start whose opener is gone reads opened",
            cites: &["34", "35", "37", "75"],
            run: opened_by_another_command,
        },
        Scenario {
            name: "state: an agent the terminal cannot read reads stopped once its record says its session ended",
            cites: &["134", "96", "45"],
            run: ended_where_the_terminal_reads_nothing,
        },
    ]
}

fn agent_status(world: &World, place: &PlaceId) -> Result<flume_core::AgentStatus, String> {
    world
        .status(place)?
        .agent
        .ok_or("no agent in status".into())
}

fn s39_started_then_prompted(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.agent.set_started(&code, Reading::No);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let (command, _) = world.terminal.opened(&code).ok_or("not opened")?;
    ensure!(
        command.folder_trust.is_some(),
        "the command names no folder-trust question for the terminal to answer"
    );
    for _ in 0..3 {
        world.settle(&id);
        world.host.advance(60);
    }
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Starting),
        "the code stage before the agent said it started"
    );
    ensure_eq!(world.terminal.log.count("send"), 0, "prompts sent");
    let agent = agent_status(&world, &id)?;
    ensure_eq!(agent.started, Reading::No, "started, as status gives it");
    ensure!(
        agent.screen.is_some(),
        "status gives no screen while the start is starting"
    );
    world.agent.set_started(&code, Reading::Yes);
    world.settle(&id);
    ensure_eq!(world.terminal.log.count("send"), 1, "prompts sent");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage"
    );
    ensure_eq!(
        agent_status(&world, &id)?.screen,
        None,
        "the screen once the stage has begun"
    );
    Ok(())
}

fn s40_held_at_a_dialog(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    let warning = "WARNING: running in Bypass Permissions mode";
    world.agent.set_started(&code, Reading::No);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.terminal.set_state(&code, AgentState::Asking);
    world.terminal.set_screen(&code, Some(warning));
    world.settle(&id);
    let agent = agent_status(&world, &id)?;
    ensure_eq!(
        agent.state,
        AgentState::Unknown,
        "a dialog no mark names, as status reads it"
    );
    ensure_eq!(
        agent.screen.as_deref(),
        Some(warning),
        "the screen status gives"
    );
    world.host.advance(QUIET_LIMIT - 1);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Starting),
        "the start before the stage's limit"
    );
    world.host.advance(1);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::NeverBegun),
        "the start at the stage's limit"
    );
    let end = world.status(&id)?.stages[0].end.clone().ok_or("no end")?;
    ensure_eq!(
        end.screen.as_deref(),
        Some(warning),
        "the screen the end carries"
    );
    ensure!(
        world.terminal.sent_to(&code).is_empty(),
        "flume typed into the dialog"
    );
    Ok(())
}

fn stopped_before_it_started(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.agent.set_started(&code, Reading::No);
    world.agent.set_last_words(&code, "Invalid API key");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world
        .terminal
        .set_screen(&code, Some("Invalid API key · Please run /login"));
    world.terminal.kill(&code);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::NeverBegun),
        "the start whose program ended with no word"
    );
    let end = world.status(&id)?.stages[0].end.clone().ok_or("no end")?;
    ensure_eq!(
        end.last_words.as_deref(),
        Some("Invalid API key"),
        "the last words the end carries"
    );
    ensure!(end.screen.is_some(), "the end carries no screen");
    ensure_eq!(world.terminal.log.count("send"), 0, "prompts sent");
    Ok(())
}

fn stopped_while_starting(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.agent.set_started(&code, Reading::No);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let answer = world.engine().stop(&id, false);
    ensure!(
        answer.refusal().is_none(),
        "the stop was refused: {answer:?}"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::NeverBegun),
        "the start a caller stopped while it was starting"
    );
    let end = world.status(&id)?.stages[0].end.clone().ok_or("no end")?;
    ensure!(end.screen.is_some(), "the end carries no screen");
    Ok(())
}

fn s41_brought_back_bare(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::TasksTicked), "one")?;
    let code = World::identity("build", "one", "work", 1);
    let commit = world.tick("build/one", "one", 2, 5);
    world.terminal.set_state(&code, AgentState::Working);
    world.settle(&id);
    world.host.restart();
    world.terminal.forget_all();
    world.terminal.resume(&code);
    world.terminal.set_state(&code, AgentState::Working);
    world.agent.set_started(&code, Reading::No);
    world.agent.set_last_words(&code, "Halfway through the fix");
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Lost),
        "a stage whose agent came back saying nothing of its start"
    );
    let end = world.status(&id)?.stages[0].end.clone().ok_or("no end")?;
    ensure_eq!(
        end.last_words.as_deref(),
        Some("Halfway through the fix"),
        "the last words the end carries"
    );
    ensure_eq!(end.commits, vec![commit], "the commits the end carries");
    ensure_eq!(
        world.terminal.log.count("close"),
        0,
        "agents ended while working"
    );
    world.terminal.set_state(&code, AgentState::Waiting);
    world.engine().settle(None);
    ensure_eq!(
        world.terminal.log.count("close"),
        1,
        "the resumed agent, ended once quiet"
    );
    ensure!(
        world.held(&id).is_some_and(|held| held
            .stages
            .iter()
            .all(|stage| stage.start.as_ref().is_none_or(|start| !start.live))),
        "a start stays live after its agent was ended"
    );
    Ok(())
}

fn s42_a_finished_turn(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "two",
            false,
            vec![
                stage("code", Some(Deliverable::Commits), "{{brief}}"),
                stage("review", Some(Deliverable::Commits), "{{brief}}"),
            ],
        )],
    );
    let id = World::id("two", "one");
    let code = World::identity("two", "one", "code", 1);
    launched(&world.engine.launch(&launch("two", "one", Some("x"))))?;
    world.settle(&id);
    world.terminal.set_state(&code, AgentState::Working);
    world.commit("two/one", &["fix.rs"]);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "a working agent's stage"
    );
    world.terminal.set_state(&code, AgentState::Waiting);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "the code stage"
    );
    ensure_eq!(world.terminal.log.count("close"), 1, "agents ended");
    ensure!(
        matches!(
            world.stage_state(&id, "review"),
            Some(StageState::Starting | StageState::Started | StageState::Begun)
        ),
        "the review stage did not start: {:?}",
        world.stage_state(&id, "review")
    );
    Ok(())
}

fn s43_a_prompt_left_typed(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.agent.receives_at(&code, Some(2));
    world.terminal.comes_up(AgentState::Unknown);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.terminal.set_state(&code, AgentState::Waiting);
    world.terminal.leaves_next_typed(&code);
    world.settle(&id);
    let prompt = world.prompt(&id, &code).ok_or("no prompt")?;
    ensure_eq!(
        world.terminal.sent_to(&code),
        vec![prompt.clone(), prompt],
        "the sendings"
    );
    ensure_eq!(
        world.terminal.typed(&code),
        Some((String::new(), 1)),
        "what is left typed, and how often a send cleared it"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage"
    );
    Ok(())
}

fn blocked_without_a_mark(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.terminal.comes_up(AgentState::Asking);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    ensure_eq!(world.terminal.log.count("send"), 0, "prompts sent");
    let agent = agent_status(&world, &id)?;
    ensure_eq!(agent.state, AgentState::Unknown, "the agent's state");
    ensure_eq!(agent.asking, Reading::No, "the agent's mark");
    let told = world.engine().tell(&id, "go on", Some(30));
    ensure!(told.refusal().is_some(), "a tell into a dialog was taken");
    ensure!(
        world.terminal.sent_to(&code).is_empty(),
        "flume typed into a dialog it cannot name"
    );
    world.agent.set_asking(&code, Reading::Yes);
    ensure_eq!(
        agent_status(&world, &id)?.state,
        AgentState::Asking,
        "the agent's state, with the mark"
    );
    Ok(())
}

fn opened_by_another_command(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.terminal.comes_up(AgentState::Unknown);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let mut held = world.held(&id).ok_or("no place")?;
    let start = held.stages[0].start.as_mut().ok_or("no start")?;
    ensure_eq!(
        start.opening,
        None,
        "the opener, once the launch's open answered"
    );
    // As a retry's start stands while its command's open still brings
    // the program up: the agent said it started and the terminal reads
    // it waiting before it has drawn.
    let opener = world.host.another_command();
    start.opening = Some(opener.clone());
    world.store.put(&id, held);
    world.terminal.set_state(&code, AgentState::Waiting);
    world.settle(&id);
    ensure_eq!(
        world.terminal.log.count("send"),
        0,
        "prompts sent while another command opens the agent"
    );
    world.host.end_command(&opener);
    world.settle(&id);
    ensure_eq!(
        world.terminal.log.count("send"),
        1,
        "prompts sent once its opener is gone"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage"
    );
    Ok(())
}

fn ended_where_the_terminal_reads_nothing(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    world.terminal.set_state(&code, AgentState::Unknown);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "a stage whose agent the terminal cannot read"
    );
    world.agent.set_ended(&code, Reading::Yes);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "a stage whose agent's record says its session ended"
    );
    match world.engine.status(Some(&id)) {
        Answer::Status { places, .. } => ensure!(
            places[0].agent.is_none(),
            "a lost stage's agent still reads live"
        ),
        other => return Err(format!("status answered {other:?}")),
    }
    Ok(())
}

/// A start the terminal reads held or never come up: never begun at once,
/// with the screen it gave, no prompt sent, then or at a later settle.
fn never_begun_at_open(world: &World, screen: &str) -> Outcome {
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.agent.set_started(&code, Reading::No);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::NeverBegun),
        "the code stage once the terminal answered"
    );
    for _ in 0..2 {
        world.settle(&id);
        world.host.advance(60);
    }
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::NeverBegun),
        "the code stage after two settles"
    );
    ensure_eq!(world.terminal.log.count("send"), 0, "prompts sent");
    let status = world.status(&id)?;
    let end = status.stages[0].end.clone().ok_or("no end in status")?;
    ensure_eq!(end.screen.as_deref(), Some(screen), "the end's screen");
    Ok(())
}

fn s40_held_never_begun_at_once(_: &()) -> Outcome {
    let world = World::new();
    let warning = "WARNING: the agent runs in Bypass Permissions mode";
    world.terminal.hold_opens(warning);
    never_begun_at_open(&world, warning)
}

fn s45_never_came_up(_: &()) -> Outcome {
    let world = World::new();
    let screen = "zsh: command not found: agent";
    world.terminal.opens_never_come_up(screen);
    never_begun_at_open(&world, screen)
}

fn s49_clues_follow_the_stage(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.agent.set_started(&code, Reading::No);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    world.agent.set_started(&code, Reading::Yes);
    world.terminal.set_state(&code, AgentState::Waiting);
    world.settle(&id);
    world.terminal.set_state(&code, AgentState::Working);
    world.settle(&id);
    world.settle(&id);
    world.agent.set_asking(&code, Reading::Yes);
    world.terminal.set_state(&code, AgentState::Asking);
    world.settle(&id);
    let titles: Vec<String> = world
        .terminal
        .shown()
        .into_iter()
        .map(|(_, clues)| format!("{} · {}", clues.stage, clues.state))
        .collect();
    ensure_eq!(
        titles,
        vec![
            "code · starting".to_string(),
            "code · waiting".to_string(),
            "code · working".to_string(),
            "code · asking".to_string(),
        ],
        "the titles shown, once per change"
    );
    let (_, last) = world.terminal.shown().pop().ok_or("nothing shown")?;
    ensure_eq!(
        last.agent,
        Some(code.clone()),
        "the agent the clues are beside"
    );
    ensure_eq!(
        last.base.to_string(),
        "0 running · 1 asking · 0 waiting on caller · 0 stalled",
        "the base's count"
    );
    world.terminal.shows_fail(true);
    world.terminal.set_state(&code, AgentState::Working);
    world.agent.set_asking(&code, Reading::No);
    world.settle(&id);
    ensure_eq!(
        world.host.logged.count("the"),
        1,
        "the failed clue logged once"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the stage after a clue failed"
    );
    Ok(())
}

fn the_identity_names_the_repository_by_its_id(_: &()) -> Outcome {
    let world = World::new();
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let (command, _) = world
        .terminal
        .opened(&World::identity("fix", "one", "code", 1))
        .ok_or("not opened under the identity named by the repository's id")?;
    let identity = command
        .env
        .get("AGENT_IDENTITY")
        .ok_or("no identity in the command")?;
    let id = flume_core::repository_id("/repository/.git");
    ensure_eq!(
        identity.split(':').nth(1),
        Some(id.as_str()),
        "the repository's part of the identity"
    );
    ensure_eq!(
        identity.split(':').nth(1).map(str::len),
        Some(12),
        "the id's length"
    );
    ensure!(
        !identity.contains(REPOSITORY),
        "the identity names the path"
    );
    Ok(())
}

//! A stage's run (24 to 45, 107 to 110, 118): one fresh agent, its prompt
//! filled and delivered, and its end read from its deliverable, its
//! agent's stop-short or a conversation's finish.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, Retry};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name:
                "prompt: the brief whole and the way to stop short, rendered with nothing started",
            cites: &["5", "28", "29", "119"],
            run: the_prompt_rendered,
        },
        Scenario {
            name: "prompt: a part with nothing to fill it leaves the stage refused naming it",
            cites: &["27"],
            run: a_part_with_nothing_to_fill_it,
        },
        Scenario {
            name: "prompt: a retry's words asked for and missing, then given",
            cites: &["S32", "27", "49"],
            run: s32_words_missing_then_given,
        },
        Scenario {
            name: "start: the agent's settings resolve launch over stage over flume's default",
            cites: &["32", "29", "107"],
            run: settings_resolved,
        },
        Scenario {
            name: "prompt: a slow start reads unknown, then a missed sending is sent once more",
            cites: &["S5", "35", "38", "I5"],
            run: s5_a_slow_start,
        },
        Scenario {
            name: "prompt: under a terminal that cannot send, the start carries the one sending",
            cites: &["38", "95"],
            run: a_terminal_that_cannot_send,
        },
        Scenario {
            name: "prompt: never sent into a question, nor sent once received",
            cites: &["37", "38", "90"],
            run: never_into_a_question,
        },
    ]
}

fn the_prompt_rendered(_: &()) -> Outcome {
    let world = World::new();
    let brief = "Bump the pin to 2.4.1.\n\nKeep the lockfile — exactly as it stands.";
    let mut held = launch("fix", "one", Some(brief));
    held.hold = vec!["code".into()];
    launched(&world.engine.launch(&held))?;
    let calls = (
        world.terminal.log.calls().len(),
        world.store.log.count("write"),
        world.agent.log.count("command"),
    );
    let text = match world.engine.prompt(&World::id("fix", "one"), "code") {
        Answer::Prompt { text, .. } => text,
        other => return Err(format!("prompt answered {other:?}")),
    };
    ensure!(text.contains(brief), "the brief is not whole in {text}");
    let way = format!("{}", World::identity("fix", "one", "code", 1));
    ensure!(
        text.contains("flume needs") && text.contains(&way),
        "the way to stop short is not in {text}"
    );
    ensure_eq!(
        (
            world.terminal.log.calls().len(),
            world.store.log.count("write"),
            world.agent.log.count("command"),
        ),
        calls,
        "calls a rendering made"
    );
    Ok(())
}

fn a_part_with_nothing_to_fill_it(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "notes",
            false,
            vec![stage(
                "write",
                Some(Deliverable::Commits),
                "write {{report}}",
            )],
        )],
    );
    let place = launched(&world.engine.launch(&launch("notes", "one", Some("x"))))?;
    ensure_eq!(place.stages[0].state, StageState::Refused, "the stage");
    ensure_eq!(
        place.stages[0].refused_part.as_deref(),
        Some("report"),
        "the part named"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    Ok(())
}

fn s32_words_missing_then_given(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "review",
            false,
            vec![stage(
                "review",
                Some(Deliverable::Commits),
                "Review against what the operator said: {{words}}",
            )],
        )],
    );
    let id = World::id("review", "one");
    let place = launched(&world.engine.launch(&launch("review", "one", Some("x"))))?;
    ensure_eq!(
        place.stages[0].state,
        StageState::Refused,
        "the review stage"
    );
    ensure_eq!(
        place.stages[0].refused_part.as_deref(),
        Some("words"),
        "the part"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    let answer = world.engine.retry(
        &id,
        Retry::Stage("review".into()),
        Some("check the error paths".into()),
        false,
    );
    ensure!(
        answer.refusal().is_none(),
        "the retry was refused: {answer:?}"
    );
    ensure_eq!(
        world.stage_state(&id, "review"),
        Some(StageState::Started),
        "the review stage"
    );
    let prompt = world
        .prompt(&id, &World::identity("review", "one", "review", 1))
        .ok_or("no prompt")?;
    ensure!(
        prompt.contains("check the error paths"),
        "the words are not in {prompt}"
    );
    Ok(())
}

fn settings_resolved(_: &()) -> Outcome {
    let world = World::new();
    let mut code = stage("code", Some(Deliverable::Commits), "{{brief}}");
    code.agents = vec![Settings {
        model: Some("small".into()),
        instructions: Some("keep it short".into()),
        ..Settings::default()
    }];
    world
        .kinds
        .set_kinds(REPOSITORY, vec![kind("bump", false, vec![code])]);
    let mut bump = launch("bump", "one", Some("x"));
    bump.settings.insert(
        "code".into(),
        Settings {
            model: Some("large".into()),
            beside: Some("operator".into()),
            ..Settings::default()
        },
    );
    let place = launched(&world.engine.launch(&bump))?;
    let agent = place.agent.ok_or("no agent")?;
    ensure_eq!(agent.program, "agent", "the program");
    ensure_eq!(agent.settings.model.as_deref(), Some("large"), "the model");
    ensure_eq!(
        agent.settings.instructions.as_deref(),
        Some("keep it short"),
        "the instructions"
    );
    let identity = World::identity("bump", "one", "code", 1);
    ensure_eq!(
        world.terminal.beside(&identity),
        Some("operator".to_string()),
        "where the agent runs"
    );
    let (command, _) = world.terminal.opened(&identity).ok_or("not opened")?;
    ensure!(
        command.args.contains(&"large".to_string()),
        "the command carries not the model: {command:?}"
    );
    Ok(())
}

fn s5_a_slow_start(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.comes_up(AgentState::Unknown);
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world
        .terminal
        .schedule(&code, world.host.now() + 240, AgentState::Waiting);
    world.agent.receives_at(&code, Some(2));
    for _ in 0..7 {
        world.engine().settle(Some(&id));
        ensure_eq!(
            world.stage_state(&id, "code"),
            Some(StageState::Started),
            "the code stage"
        );
        let status = match world.engine.status(Some(&id)) {
            Answer::Status { places } => places,
            other => return Err(format!("status answered {other:?}")),
        };
        let agent = status[0].agent.clone().ok_or("no agent")?;
        ensure_eq!(agent.state, AgentState::Unknown, "the start");
        ensure_eq!(
            world.terminal.log.count("send"),
            0,
            "prompts sent before it waits"
        );
        world.host.advance(30);
    }
    world.host.advance(30);
    world.engine().settle(Some(&id));
    let prompt = world.prompt(&id, &code).ok_or("no prompt")?;
    ensure_eq!(
        world.terminal.sent_to(&code),
        vec![prompt.clone(), prompt],
        "the sendings"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage"
    );
    Ok(())
}

fn a_terminal_that_cannot_send(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.cannot_send();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.agent.receives_at(&code, None);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let (_, carried) = world.terminal.opened(&code).ok_or("not opened")?;
    ensure!(carried, "the start did not carry the prompt");
    world.engine.settle(Some(&id));
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::NeverBegun),
        "the code stage"
    );
    ensure_eq!(world.terminal.log.count("send"), 0, "sends");
    let other = World::id("fix", "two");
    launched(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    world.engine.settle(Some(&other));
    ensure_eq!(
        world.stage_state(&other, "code"),
        Some(StageState::Begun),
        "a carried prompt received"
    );
    Ok(())
}

fn never_into_a_question(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.agent.set_asking(&code, Reading::Yes);
    world.engine.settle(Some(&id));
    ensure_eq!(world.terminal.log.count("send"), 0, "sends into a question");
    world.agent.set_asking(&code, Reading::No);
    world.engine.settle(Some(&id));
    ensure_eq!(world.terminal.log.count("send"), 1, "sends");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage"
    );
    world.engine.settle(Some(&id));
    let other = World::id("fix", "two");
    let two = World::identity("fix", "two", "code", 1);
    world.agent.receives_at(&two, Some(0));
    launched(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    world.engine.settle(Some(&other));
    ensure!(
        world.terminal.sent_to(&two).is_empty(),
        "a received prompt was sent"
    );
    ensure_eq!(world.terminal.log.count("send"), 1, "sends in all");
    Ok(())
}

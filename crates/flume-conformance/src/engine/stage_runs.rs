//! A stage's run (24 to 45, 107 to 110, 118): one fresh agent, its prompt
//! filled and delivered, and its end read from its deliverable, its
//! agent's stop-short or a conversation's finish.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, Retry};
use flume_fakes::Log;
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
        Scenario {
            name:
                "end: tasks ticked over four commits deliver only at the head that ticks the last",
            cites: &["S2", "24", "40", "43", "56"],
            run: s2_tasks_ticked_over_four_commits,
        },
        Scenario {
            name: "end: facts are never paired across heads",
            cites: &["25", "S2"],
            run: facts_never_paired_across_heads,
        },
        Scenario {
            name: "end: an agent waiting on own work is neither ended nor stuck",
            cites: &["S3", "39", "40", "44", "91"],
            run: s3_waiting_on_own_work,
        },
        Scenario {
            name: "end: quiet past its limit with nothing delivered reads stuck, and is not ended",
            cites: &["S17", "44", "110", "I1"],
            run: s17_stuck,
        },
        Scenario {
            name: "end: a conversation is never stuck, and never ended by flume",
            cites: &["S23", "109", "18"],
            run: s23_a_conversation_over_an_hour,
        },
        Scenario {
            name:
                "end: an agent that stopped with nothing delivered reads lost, and no agent starts",
            cites: &["45", "48", "39"],
            run: lost,
        },
        Scenario {
            name: "end: an agent working past its limit is reported so, and not ended",
            cites: &["118", "31"],
            run: working_past_its_limit,
        },
        Scenario {
            name: "end: an agent whose own work cannot be read is not quiet, and status names it",
            cites: &["31", "93"],
            run: unknown_is_not_quiet,
        },
        Scenario {
            name:
                "end: each of the six forms delivers where it lives, made since the stage started",
            cites: &["24", "25", "123"],
            run: the_six_forms,
        },
        Scenario {
            name: "stop-short: a construct stops short, and no code stage starts",
            cites: &["S4", "41", "42", "36", "92"],
            run: s4_a_construct_stops_short,
        },
        Scenario {
            name: "stop-short: one carrying another start's secret or identity is refused",
            cites: &["41", "33"],
            run: another_starts_secret,
        },
        Scenario {
            name: "stop check: an agent that would stop with nothing made is sent back",
            cites: &["S24", "110", "115", "28"],
            run: s24_sent_back,
        },
        Scenario {
            name: "stop check: own work, a question or a conversation are not going quiet",
            cites: &["110", "S3", "S23"],
            run: not_going_quiet,
        },
        Scenario {
            name: "guard: an agent's submission refused is reported, and the stage goes on",
            cites: &["S25", "108", "116", "56", "71"],
            run: s25_an_agent_tries_to_submit,
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

fn closes(log: &Log) -> usize {
    log.count("close")
}

fn s2_tasks_ticked_over_four_commits(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::TasksTicked), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.terminal.set_state(&work, AgentState::Working);
    world.tick("build/one", "one", 1, 5);
    world.tick("build/one", "one", 2, 5);
    world.settle(&id);
    let status = world.status(&id)?;
    let ticked = |status: &flume_core::PlaceStatus| {
        status
            .tasks
            .as_ref()
            .map(|tasks| (tasks.iter().filter(|task| task.ticked).count(), tasks.len()))
    };
    ensure_eq!(ticked(&status), Some((2, 5)), "the tasks ticked");
    ensure_eq!(status.delivered, Some(false), "delivered");
    world.tick("build/one", "one", 4, 5);
    let last = world.tick("build/one", "one", 5, 5);
    world.settle(&id);
    let status = world.status(&id)?;
    ensure_eq!(ticked(&status), Some((5, 5)), "the tasks ticked");
    ensure_eq!(status.delivered, Some(true), "delivered");
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Begun),
        "a working agent's stage"
    );
    world.terminal.set_state(&work, AgentState::Waiting);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Delivered),
        "the stage"
    );
    let end = world.status(&id)?.stages[0].end.clone().ok_or("no end")?;
    ensure_eq!(end.head, Some(last), "the head it delivered at");
    ensure_eq!(closes(&world.terminal.log), 1, "agents ended");
    Ok(())
}

fn facts_never_paired_across_heads(_: &()) -> Outcome {
    let world = World::new();
    let base = world.git.branch_head(REPOSITORY, "main").ok_or("main")?;
    let done = ChangeFacts {
        exists: true,
        planning_complete: true,
        tasks: vec![Task {
            text: "1.1".into(),
            ticked: true,
        }],
        archived: false,
    };
    world.change.set(REPOSITORY, &base, "one", done);
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "build",
            false,
            vec![stage("work", Some(Deliverable::TasksTicked), "{{brief}}")],
        )],
    );
    let mut build = launch("build", "one", Some("x"));
    build.declaration = None;
    let id = World::id("build", "one");
    let answer = world.engine.launch(&build);
    if answer.refusal().is_some() {
        world
            .change
            .set(REPOSITORY, &base, "one", ChangeFacts::default());
        launched(&world.engine.launch(&build))?;
    }
    world.settle(&id);
    let work = World::identity("build", "one", "work", 1);
    world.terminal.set_state(&work, AgentState::Working);
    let unticked = world.tick("build/one", "one", 0, 1);
    world.git.set_branch(REPOSITORY, "build/one", &base);
    world.change.set(
        REPOSITORY,
        &base,
        "one",
        ChangeFacts {
            exists: true,
            planning_complete: true,
            tasks: vec![Task {
                text: "1.1".into(),
                ticked: true,
            }],
            archived: false,
        },
    );
    world
        .git
        .move_after_reads(REPOSITORY, "build/one", 1, &unticked);
    world.history.log.clear();
    world.change.log.clear();
    let status = world.status(&id)?;
    ensure_eq!(
        status.head,
        Some(unticked.clone()),
        "the head the facts were read at"
    );
    ensure_eq!(status.delivered, Some(false), "delivered");
    ensure!(
        world
            .change
            .log
            .calls()
            .iter()
            .any(|call| call.contains(&unticked.0)),
        "the change was not read again at the new head: {:?}",
        world.change.log.calls()
    );
    Ok(())
}

fn s3_waiting_on_own_work(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::TasksTicked), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.tick("build/one", "one", 3, 3);
    world.agent.set_own_work(&work, Reading::Yes);
    for _ in 0..12 {
        world.host.advance(60);
        world.settle(&id);
        ensure_eq!(
            world.stage_state(&id, "work"),
            Some(StageState::Begun),
            "the stage"
        );
    }
    ensure_eq!(closes(&world.terminal.log), 0, "agents ended");
    world.agent.set_own_work(&work, Reading::No);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Delivered),
        "the stage"
    );
    Ok(())
}

fn s17_stuck(_: &()) -> Outcome {
    let world = World::new();
    world.agent.no_send_back();
    let id = world.begun("build", Some(Deliverable::TasksTicked), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.tick("build/one", "one", 2, 3);
    world.settle(&id);
    world.host.advance(29 * 60);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Begun),
        "the stage short of its limit"
    );
    world.host.advance(2 * 60);
    world.settle(&id);
    let status = world.status(&id)?;
    ensure_eq!(status.stages[0].state, StageState::Stuck, "the stage");
    ensure_eq!(
        status.agent.map(|agent| agent.identity),
        Some(work.to_string()),
        "the agent status names"
    );
    ensure_eq!(closes(&world.terminal.log), 0, "agents ended");
    world.terminal.set_state(&work, AgentState::Working);
    world.settle(&id);
    world.tick("build/one", "one", 3, 3);
    world.terminal.set_state(&work, AgentState::Waiting);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Delivered),
        "the stage"
    );
    Ok(())
}

fn s23_a_conversation_over_an_hour(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("round", None, "one")?;
    let work = World::identity("round", "one", "work", 1);
    for minute in 0..60 {
        let state = if minute % 2 == 0 {
            AgentState::Waiting
        } else {
            AgentState::Working
        };
        world.terminal.set_state(&work, state);
        world.host.advance(60);
        world.settle(&id);
        ensure_eq!(
            world.stage_state(&id, "work"),
            Some(StageState::Begun),
            "the conversation"
        );
    }
    ensure_eq!(closes(&world.terminal.log), 0, "agents ended");
    Ok(())
}

fn lost(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.terminal.kill(&work);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Lost),
        "the stage"
    );
    world.settle(&id);
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

fn working_past_its_limit(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.terminal.set_state(&work, AgentState::Working);
    world.host.advance(31 * 60);
    world.settle(&id);
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    ensure!(agent.past_limit, "the agent is not reported past its limit");
    ensure_eq!(closes(&world.terminal.log), 0, "agents ended");
    world.commit("build/one", &["work.rs"]);
    world.settle(&id);
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    ensure!(!agent.past_limit, "a head move left it past its limit");
    Ok(())
}

fn unknown_is_not_quiet(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.commit("build/one", &["work.rs"]);
    world.agent.set_own_work(&work, Reading::Unknown);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Begun),
        "the stage"
    );
    ensure_eq!(closes(&world.terminal.log), 0, "agents ended");
    let status = world.status(&id)?;
    ensure!(
        status
            .cannot_read
            .iter()
            .any(|what| what.contains("own work")),
        "status names not what it cannot read: {:?}",
        status.cannot_read
    );
    Ok(())
}

fn the_six_forms(_: &()) -> Outcome {
    let forms = [
        ("commits", Deliverable::Commits),
        ("written", Deliverable::ChangeWritten),
        ("ticked", Deliverable::TasksTicked),
        ("archived", Deliverable::ChangeArchived),
        (
            "report",
            Deliverable::Report {
                path: "{{state}}/conclusion.md".into(),
            },
        ),
        (
            "committed",
            Deliverable::CommittedReport {
                path: "notes/report.md".into(),
            },
        ),
    ];
    for (name, form) in forms {
        let world = World::new();
        let id = world.begun(name, Some(form.clone()), "one")?;
        let branch = format!("{name}/one");
        world.settle(&id);
        ensure_eq!(
            world.stage_state(&id, "work"),
            Some(StageState::Begun),
            name
        );
        match form {
            Deliverable::Commits => {
                world.commit(&branch, &["work.rs"]);
            }
            Deliverable::ChangeWritten => {
                let revision = world.commit(&branch, &["plans/one/design.md"]);
                world.change.set(
                    REPOSITORY,
                    &revision,
                    "one",
                    ChangeFacts {
                        exists: true,
                        planning_complete: true,
                        ..ChangeFacts::default()
                    },
                );
            }
            Deliverable::TasksTicked => {
                world.tick(&branch, "one", 1, 1);
            }
            Deliverable::ChangeArchived => {
                let revision = world.commit(&branch, &["plans/archive/one/tasks.md"]);
                world.change.set(
                    REPOSITORY,
                    &revision,
                    "one",
                    ChangeFacts {
                        archived: true,
                        ..ChangeFacts::default()
                    },
                );
            }
            Deliverable::Report { .. } => {
                world.host.advance(1);
                world
                    .host
                    .write_file(&format!("{}/conclusion.md", world.store.folder(&id)));
            }
            Deliverable::CommittedReport { .. } => {
                world.commit(&branch, &["notes/report.md"]);
            }
            Deliverable::Unreadable { .. } => {}
        }
        world.settle(&id);
        ensure_eq!(
            world.stage_state(&id, "work"),
            Some(StageState::Delivered),
            name
        );
    }
    Ok(())
}

fn s4_a_construct_stops_short(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("one of two designs"))),
    )?;
    world.settle(&id);
    let construct = World::identity("change", "plan", "construct", 1);
    let secret = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?
        .secret;
    let words = "Which design does the brief mean: the queue,\nor the log? Both fit.";
    let answer = world.engine.needs(&construct.to_string(), &secret.0, words);
    ensure!(
        answer.refusal().is_none(),
        "the stop-short was refused: {answer:?}"
    );
    world.settle(&id);
    let status = world.status(&id)?;
    ensure_eq!(status.stages[0].state, StageState::EndedShort, "construct");
    ensure_eq!(status.stages[1].state, StageState::NotReached, "code");
    ensure_eq!(
        status.stages[0].end.clone().and_then(|end| end.words),
        Some(words.to_string()),
        "the words status gives"
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

fn another_starts_secret(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let first = World::identity("build", "one", "work", 1);
    let old = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?
        .secret;
    world.terminal.kill(&first);
    world.settle(&id);
    ensure!(
        world
            .engine
            .retry(&id, Retry::Current, None, false)
            .refusal()
            .is_none(),
        "the retry was refused"
    );
    let second = World::identity("build", "one", "work", 2);
    let new = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?
        .secret;
    refused(&world.engine.needs(&second.to_string(), &old.0, "stop"))?;
    refused(&world.engine.needs(&first.to_string(), &old.0, "stop"))?;
    ensure!(
        world
            .stage_state(&id, "work")
            .is_some_and(StageState::running),
        "a refused stop-short ended the stage"
    );
    ensure!(
        world
            .engine
            .needs(&second.to_string(), &new.0, "stop")
            .refusal()
            .is_none(),
        "the current start's stop-short was refused"
    );
    Ok(())
}

fn stop_words(world: &World, identity: &Identity) -> Option<String> {
    match world.engine.stop_check(&identity.to_string()) {
        Answer::StopCheck { words } => words,
        _ => None,
    }
}

fn s24_sent_back(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    let code = World::identity("fix", "one", "code", 1);
    let words = stop_words(&world, &code).ok_or("the agent was not sent back")?;
    ensure!(
        words.contains("commits"),
        "the words name not commits: {words}"
    );
    ensure!(
        words.contains("flume needs"),
        "the words name not the way to stop short: {words}"
    );
    world.commit("fix/one", &["fix.rs"]);
    ensure_eq!(
        stop_words(&world, &code),
        None,
        "the stop check once delivered"
    );
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "the code stage"
    );
    Ok(())
}

fn not_going_quiet(_: &()) -> Outcome {
    let world = World::new();
    world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.agent.set_own_work(&work, Reading::Yes);
    ensure_eq!(
        stop_words(&world, &work),
        None,
        "the stop check with own work"
    );
    world.agent.set_own_work(&work, Reading::No);
    world.agent.set_asking(&work, Reading::Yes);
    ensure_eq!(
        stop_words(&world, &work),
        None,
        "the stop check with a question open"
    );
    world.agent.set_asking(&work, Reading::No);
    ensure!(
        stop_words(&world, &work).is_some(),
        "an agent with nothing made was not sent back"
    );
    let other = World::new();
    other.begun("round", None, "one")?;
    let round = World::identity("round", "one", "work", 1);
    ensure_eq!(
        stop_words(&other, &round),
        None,
        "the stop check of a conversation"
    );
    Ok(())
}

fn s25_an_agent_tries_to_submit(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    let code = World::identity("fix", "one", "code", 1);
    world
        .agent
        .refuse_act(&code, "a submission of fix/one to the merge queue");
    world.settle(&id);
    let status = world.status(&id)?;
    ensure!(
        status
            .refused_acts
            .iter()
            .any(|act| act.contains("submission")),
        "status reports no refusal: {:?}",
        status.refused_acts
    );
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions");
    ensure_eq!(status.stages[0].state, StageState::Begun, "the code stage");
    Ok(())
}

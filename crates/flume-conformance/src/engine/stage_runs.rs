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
            name: "stop-short: a place's prompt lists its own stream's roles by name, then every other stream's whole, each with its words, after every other part",
            cites: &["28", "41", "132", "193"],
            run: the_roles_a_place_may_name,
        },
        Scenario {
            name: "stop-short: a role's own run lists every role a source declares with words but its own",
            cites: &["28", "41", "132", "193"],
            run: a_roles_own_run_lists_the_others,
        },
        Scenario {
            name: "stop-short: where no source declares a role with words, the prompt lists none",
            cites: &["28", "41", "193"],
            run: no_roles_listed,
        },
        Scenario {
            name: "stop-short: a construct's prompt says stopping short is its one way to ask, and that it writes no card and no message to a person",
            cites: &["28", "41", "69", "72"],
            run: the_one_way_to_ask,
        },
        Scenario {
            name: "stop-short: a question naming its own stream's ops by the role alone stands pending for east/ops alone, which the settle wakes, and wakes not the place's recipient",
            cites: &["41", "43", "132", "193", "S93"],
            run: a_question_for_its_own_streams_ops,
        },
        Scenario {
            name: "stop-short: a question naming another stream's role by its address stands pending for it alone, its run woken in its own stream",
            cites: &["41", "43", "132", "143", "193"],
            run: a_question_for_another_streams_role,
        },
        Scenario {
            name: "stop-short: a name its prompt did not list is refused naming those it did, and the stage goes on",
            cites: &["41", "132"],
            run: a_name_outside_the_list,
        },
        Scenario {
            name: "stop-short: one naming no role goes to the place's recipients, as before",
            cites: &["41", "43"],
            run: no_address_named,
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
    let way = world
        .agent
        .stop_short(&World::identity("fix", "one", "code", 1));
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
    let review = World::identity("review", "one", "review", 1);
    ensure_eq!(
        world.stage_state(&id, "review"),
        Some(StageState::Begun),
        "the review stage, its agent sent its prompt by the retry"
    );
    ensure_eq!(world.terminal.sent_to(&review).len(), 1, "sendings");
    let prompt = world.prompt(&id, &review).ok_or("no prompt")?;
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
    ensure_eq!(
        agent.settings.settings.model.as_deref(),
        Some("large"),
        "the model"
    );
    ensure_eq!(
        agent.settings.settings.instructions.as_deref(),
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
            Answer::Status { places, .. } => places,
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
    world.agent.set_asking(&code, Reading::Yes);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure_eq!(
        world.terminal.log.count("send"),
        0,
        "sends into a question at the launch"
    );
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
            group: None,
        }],
        archived: false,
        groups: vec![],
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
                group: None,
            }],
            archived: false,
            groups: vec![],
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
    let told = world.engine.tell(&id, "the last task is the docs", None);
    ensure!(told.refusal().is_none(), "the tell was refused: {told:?}");
    world.terminal.set_state(&work, AgentState::Working);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Begun),
        "the woken stage"
    );
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
    world.terminal.set_state(&work, AgentState::Waiting);
    let finished = world.engine.finish(&id, false);
    ensure!(
        finished.refusal().is_none(),
        "the finish was refused: {finished:?}"
    );
    ensure_eq!(closes(&world.terminal.log), 1, "agents ended");
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Finished),
        "the conversation"
    );
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
    let answer = world.engine.needs(
        &construct.to_string(),
        &secret.0,
        words,
        StopKind::NeedsAnswer,
    );
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
    ensure_eq!(
        status.stages[0].end.clone().and_then(|end| end.needs),
        Some(StopKind::NeedsAnswer),
        "the kind of stop-short status gives"
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
    refused(
        &world
            .engine
            .needs(&second.to_string(), &old.0, "stop", StopKind::NeedsAnswer),
    )?;
    refused(
        &world
            .engine
            .needs(&first.to_string(), &old.0, "stop", StopKind::FoundWork),
    )?;
    ensure!(
        world
            .stage_state(&id, "work")
            .is_some_and(StageState::running),
        "a refused stop-short ended the stage"
    );
    ensure!(
        world
            .engine
            .needs(&second.to_string(), &new.0, "stop", StopKind::NeedsAnswer)
            .refusal()
            .is_none(),
        "the current start's stop-short was refused"
    );
    Ok(())
}

/// The roles acme declares, each with its words of what it answers (28,
/// 193).
const ROLES: [(&str, &str); 4] = [
    ("east/design", "a decision the design has not made"),
    ("east/ops", "a reading of east's live systems"),
    ("east/planner", "the work's scope and order"),
    ("west/ops", "a reading of west's live systems"),
];

/// The words that open the list of roles a stop-short may name (28).
const LISTED: &str = "Where your words are for one of these roles";

/// The scratch host declares the streams `east` and `west`, and acme,
/// its kind `turn` locked, declares a run of it for each of [`ROLES`]
/// with its words (132, 193).
fn east_and_west(world: &World) {
    let run = |answers: &str| RoleRun {
        answers: Some(answers.into()),
        ..super::role_waking::planner_run(None, None)
    };
    super::role_waking::configure_at(
        world,
        "west/ops",
        "Answer what is pending for you.",
        run(ROLES[3].1),
    );
    world.host.set_stream("east", Some(Stream::default()));
    for (address, answers) in ROLES {
        world.host.set_role(address, Some(run(answers)));
    }
}

/// The prompt the place's first stage started with, and the addresses
/// its start recorded a stop-short may name.
fn first_start(world: &World, id: &PlaceId) -> Result<(String, Vec<String>), String> {
    let start = world
        .held(id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?;
    Ok((start.prompt.text, start.may_name))
}

/// Where each of `names`, as a list's line gives it with its words,
/// stands in `text`, in order, or the first that is not there.
fn listed_in_order(text: &str, names: &[(&str, &str)]) -> Outcome {
    let mut from = text.find(LISTED).ok_or("no roles are listed")?;
    for (name, words) in names {
        let line = format!("- `{name}`: {words}");
        let at = text[from..]
            .find(&line)
            .ok_or_else(|| format!("{line} is not listed after the last in {text}"))?;
        from += at + line.len();
    }
    Ok(())
}

/// The stage-runs spec's "A place of the stream's work": a fix in `east`
/// naming `east/planner` lists `design`, `ops` and `planner` by the role
/// alone, then `west/ops` whole, each with its words, last in its prompt,
/// after the way to stop short; its start records the four addresses
/// whole (28, 41, 132, 193; core.31).
fn the_roles_a_place_may_name(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.stream = Some("east".into());
    fix.recipients = vec!["east/planner".into()];
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    let (text, may_name) = first_start(&world, &id)?;
    listed_in_order(
        &text,
        &[
            ("design", ROLES[0].1),
            ("ops", ROLES[1].1),
            ("planner", ROLES[2].1),
            ("west/ops", ROLES[3].1),
        ],
    )?;
    let way = flume_core::way_to_stop_short(
        world.agent.as_ref(),
        &World::identity("fix", "one", "code", 1),
    );
    let (at_way, at_list) = (
        text.find(&way).ok_or("no way to stop short")?,
        text.find(LISTED).ok_or("no list")?,
    );
    ensure!(at_way < at_list, "the list stands before the way: {text}");
    ensure!(
        text.trim_end().ends_with(ROLES[3].1),
        "the list is not the prompt's last part: {text}"
    );
    ensure!(
        text.contains(&world.agent.stop_short_to()),
        "the list does not say how to name a role: {text}"
    );
    ensure_eq!(
        may_name,
        vec![
            "east/design".to_string(),
            "east/ops".into(),
            "east/planner".into(),
            "west/ops".into()
        ],
        "the addresses the start may name"
    );
    let rendered = match world.engine.prompt(&id, "code") {
        Answer::Prompt { text, .. } => text,
        other => return Err(format!("prompt answered {other:?}")),
    };
    listed_in_order(
        &rendered,
        &[("design", ROLES[0].1), ("west/ops", ROLES[3].1)],
    )
}

/// The stage-runs spec's "A role's own run": the run of `east/ops`,
/// woken, lists `design` and `planner`, then `west/ops`, and not `ops`
/// (28, 41, 132, 193).
fn a_roles_own_run_lists_the_others(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    ensure!(
        world.engine.wake("east/ops").refusal().is_none(),
        "the wake of east/ops was refused"
    );
    let id = World::id("turn", "east-ops");
    world.settle(&id);
    let (text, may_name) = first_start(&world, &id)?;
    listed_in_order(
        &text,
        &[
            ("design", ROLES[0].1),
            ("planner", ROLES[2].1),
            ("west/ops", ROLES[3].1),
        ],
    )?;
    ensure!(
        !text.contains("- `ops`"),
        "the run lists its own role: {text}"
    );
    ensure_eq!(
        may_name,
        vec![
            "east/design".to_string(),
            "east/planner".into(),
            "west/ops".into()
        ],
        "the addresses the run may name"
    );
    Ok(())
}

/// The stage-runs spec's "No roles": a fix whose recipient is the
/// caller's pane, where the one role a source declares gives no words of
/// what it answers, lists no role, and its start records none (28, 41,
/// 193).
fn no_roles_listed(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_role(
        "default/reviewer",
        Some(super::role_waking::planner_run(None, None)),
    );
    launched(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    let (text, may_name) = first_start(&world, &id)?;
    ensure!(!text.contains(LISTED), "roles are listed: {text}");
    ensure!(
        !text.contains("reviewer"),
        "a role with no words is named: {text}"
    );
    ensure_eq!(may_name, Vec::<String>::new(), "the addresses it may name");
    Ok(())
}

/// The stage-runs spec's "A change's construct": its way to stop short
/// says the agent asks by it alone and writes no card and no message to a
/// person, with roles listed or none (28, 41, 69, 72).
fn the_one_way_to_ask(_: &()) -> Outcome {
    for listed in [false, true] {
        let world = World::new();
        let mut change = launch("change", "plan", Some("one of two designs"));
        if listed {
            east_and_west(&world);
            change.stream = Some("east".into());
            change.recipients = vec!["east/planner".into()];
        }
        launched(&world.engine.launch(&change))?;
        let id = World::id("change", "plan");
        world.settle(&id);
        let (text, _) = first_start(&world, &id)?;
        ensure!(
            text.contains(flume_core::ONE_WAY_TO_ASK),
            "the construct is not told it asks by stopping short alone ({}): {text}",
            if listed { "roles listed" } else { "no roles" }
        );
        ensure!(
            flume_core::ONE_WAY_TO_ASK.contains("no card and no message to a person"),
            "the words do not say it writes no card and no message"
        );
        ensure_eq!(text.contains(LISTED), listed, "the roles listed");
    }
    Ok(())
}

/// A change `plan` launched in `east` for `east/operator`, standing
/// behind the stream, under [`east_and_west`]'s roles, and begun: the
/// place, and its construct's identity and secret (7, 41, 193).
fn construct_in_east(world: &World) -> Result<(PlaceId, String, String), String> {
    east_and_west(world);
    world.host.set_role(
        "east/operator",
        Some(RoleRun {
            answerer: true,
            ..super::role_waking::planner_run(None, None)
        }),
    );
    let mut change = launch("change", "plan", Some("one of two designs"));
    change.stream = Some("east".into());
    change.recipients = vec!["east/operator".into()];
    launched(&world.engine.launch(&change))?;
    let id = World::id("change", "plan");
    world.settle(&id);
    let start = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?;
    Ok((id, start.identity.to_string(), start.secret.0))
}

/// The recipients the place's newest end stands pending for.
fn pending_for(world: &World, id: &PlaceId) -> Result<Vec<String>, String> {
    let held = world.held(id).ok_or("no place")?;
    let end = held.ends.last().ok_or("no end")?;
    Ok(end.pending(&held.place.recipients))
}

/// The places of every role's run on the host, each with its stream.
fn role_runs(world: &World) -> Vec<(String, String)> {
    world
        .store
        .places()
        .into_iter()
        .filter_map(|id| world.held(&id))
        .filter_map(|held| held.role.clone().map(|role| (role, held.place.stream.name)))
        .collect()
}

/// The stage-runs spec's "A question for its own stream's ops": the
/// construct of a place in `east`, whose recipient is `east/operator`,
/// stops short naming `ops`; the stage ends short, its end stands
/// pending for `east/ops` alone, status gives that address, the settle
/// wakes `east/ops`, and no run of `east/operator` is launched (41, 43,
/// 132, 193, S93).
fn a_question_for_its_own_streams_ops(_: &()) -> Outcome {
    let world = World::new();
    let (id, construct, secret) = construct_in_east(&world)?;
    let words = "Which queue does east run in production?";
    let answer = world.engine.needs_to(
        &construct,
        &secret,
        words,
        StopKind::NeedsAnswer,
        Some("ops"),
    );
    ensure!(
        answer.refusal().is_none(),
        "the stop-short was refused: {answer:?}"
    );
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::EndedShort),
        "construct"
    );
    ensure_eq!(
        pending_for(&world, &id)?,
        vec!["east/ops".to_string()],
        "the end's recipients"
    );
    let status = world.status(&id)?;
    ensure_eq!(
        status.stages[0].end.clone().and_then(|end| end.addressee),
        Some("east/ops".to_string()),
        "the address status gives"
    );
    world.settle(&id);
    ensure_eq!(
        role_runs(&world),
        vec![("east/ops".to_string(), "east".to_string())],
        "the runs woken"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::NotReached),
        "code"
    );
    Ok(())
}

/// The stage-runs spec's "A question for another stream's role": the
/// construct of a place in `east` stops short naming `west/ops`, which
/// its list gave; its end stands pending for `west/ops` alone, and the
/// settle wakes `west/ops`, whose run runs in `west` (41, 43, 132, 143,
/// 193).
fn a_question_for_another_streams_role(_: &()) -> Outcome {
    let world = World::new();
    let (id, construct, secret) = construct_in_east(&world)?;
    let answer = world.engine.needs_to(
        &construct,
        &secret,
        "Is west's queue on 2.4?",
        StopKind::NeedsAnswer,
        Some("west/ops"),
    );
    ensure!(
        answer.refusal().is_none(),
        "the stop-short was refused: {answer:?}"
    );
    ensure_eq!(
        pending_for(&world, &id)?,
        vec!["west/ops".to_string()],
        "the end's recipients"
    );
    world.settle(&id);
    ensure_eq!(
        role_runs(&world),
        vec![("west/ops".to_string(), "west".to_string())],
        "the runs woken"
    );
    Ok(())
}

/// The stage-runs spec's "A name outside the list": a stop-short naming
/// `south/ops` or `reviewer`, neither listed, is refused naming the roles
/// and addresses it may name, and the stage goes on; one naming `ops`
/// where its prompt listed no role is refused saying so (41, 132).
fn a_name_outside_the_list(_: &()) -> Outcome {
    let world = World::new();
    let (id, construct, secret) = construct_in_east(&world)?;
    for name in ["south/ops", "reviewer"] {
        let reason = refused(&world.engine.needs_to(
            &construct,
            &secret,
            "who?",
            StopKind::NeedsAnswer,
            Some(name),
        ))?;
        for listed in ["`design`", "`ops`", "`planner`", "`west/ops`", name] {
            ensure!(
                reason.contains(listed),
                "the refusal of {name} does not name {listed}: {reason}"
            );
        }
        ensure!(
            world
                .stage_state(&id, "construct")
                .is_some_and(StageState::running),
            "a refused stop-short naming {name} ended the stage"
        );
    }
    let bare = World::new();
    launched(&bare.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    let fix = World::id("fix", "one");
    bare.settle(&fix);
    let start = bare
        .held(&fix)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?;
    let reason = refused(&bare.engine.needs_to(
        &start.identity.to_string(),
        &start.secret.0,
        "who?",
        StopKind::NeedsAnswer,
        Some("ops"),
    ))?;
    ensure!(
        reason.contains("listed no role"),
        "the refusal where none was listed: {reason}"
    );
    ensure!(
        bare.stage_state(&fix, "code")
            .is_some_and(StageState::running),
        "a refused stop-short ended the fix's stage"
    );
    Ok(())
}

/// The stage-runs spec's "No address named": the construct stops short
/// naming none, and its end stands pending for the place's recipient,
/// `east/operator`, as before (41, 43).
fn no_address_named(_: &()) -> Outcome {
    let world = World::new();
    let (id, construct, secret) = construct_in_east(&world)?;
    let answer = world
        .engine
        .needs(&construct, &secret, "which design?", StopKind::NeedsAnswer);
    ensure!(
        answer.refusal().is_none(),
        "the stop-short was refused: {answer:?}"
    );
    ensure_eq!(
        pending_for(&world, &id)?,
        vec!["east/operator".to_string()],
        "the end's recipients"
    );
    ensure_eq!(
        world.status(&id)?.stages[0]
            .end
            .clone()
            .and_then(|end| end.addressee),
        None,
        "the address status gives"
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
    let mut fix = launch("fix", "one", Some("x"));
    fix.hold_submission = true;
    launched(&world.engine.launch(&fix))?;
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

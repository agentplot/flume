//! The caller's acts (47 to 55, 109, 114): retry, stop, drop, done,
//! finish and tell, none ending an agent that is not quiet unless forced.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Answer, Retry};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "retry: a construct that ended short, retried with words, starts afresh with the question and the answer",
            cites: &["S4", "49", "50", "42"],
            run: s4_retried_with_words,
        },
        Scenario {
            name: "stop: a working agent is stopped only forced, and the stage waits for a retry",
            cites: &["S20", "51", "31", "I2"],
            run: s20_a_working_agent_stopped,
        },
        Scenario {
            name: "retry: a retry of a place whose work landed is refused naming it",
            cites: &["S21", "50", "63"],
            run: s21_a_retry_of_a_landed_place,
        },
        Scenario {
            name: "done: a fix cannot be done, and is dropped instead",
            cites: &["S26", "114", "52"],
            run: s26_done_on_a_fix,
        },
        Scenario {
            name: "drop: a fix whose branch holds a commit its base lacks is removed, its branch kept and named",
            cites: &["S9", "52", "64", "66", "67", "16", "I4"],
            run: s9_a_drop_that_keeps_the_branch,
        },
        Scenario {
            name: "tell: refused into a question with nothing typed; given once the agent waits",
            cites: &["S6", "53", "54", "55", "I5"],
            run: s6_a_tell_into_a_question_then_into_work,
        },
        Scenario {
            name: "tell: one not delivered within its bound is refused",
            cites: &["54", "53"],
            run: a_tell_past_its_bound,
        },
        Scenario {
            name: "retry: naming the submission submits the head and starts no agent",
            cites: &["S7", "49", "47", "70"],
            run: s7_the_held_submission_released,
        },
        Scenario {
            name: "finish: refused unforced while the agent works, then ends it and counts delivered",
            cites: &["109", "S23", "31"],
            run: finish_while_the_agent_works,
        },
        Scenario {
            name: "finish: a conversation whose agent stopped reads lost, and a finish ends it delivered",
            cites: &["45", "109", "40"],
            run: a_lost_conversation_finished,
        },
        Scenario {
            name: "retry: the first stage of a place whose readiness failed runs readiness again",
            cites: &["12", "49"],
            run: a_retry_after_readiness_failed,
        },
        Scenario {
            name: "stop: an end the agent did not take is reported a failure, never done",
            cites: &["31", "97", "98"],
            run: an_end_not_taken,
        },
        Scenario {
            name: "held state: a stop and a retry at once leave one agent",
            cites: &["75", "30", "I2"],
            run: a_stop_and_a_retry_at_once,
        },
        Scenario {
            name: "held state: a drop repeated answers as the first and ends and removes nothing more",
            cites: &["74", "52"],
            run: a_drop_repeated,
        },
        Scenario {
            name: "start: a settle and a retry starting one stage at once open one agent",
            cites: &["30", "75", "I2"],
            run: two_starts_at_once,
        },
        Scenario {
            name: "start: each start's identity names its start and is shared by none",
            cites: &["33", "48"],
            run: identities_never_shared,
        },
    ]
}

fn taken(answer: &Answer) -> Outcome {
    match answer.refusal() {
        Some(reason) => Err(format!("the act was refused: {reason}")),
        None => Ok(()),
    }
}

fn s4_retried_with_words(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("change", "plan");
    launched(&world.engine.launch(&launch("change", "plan", Some("x"))))?;
    world.settle(&id);
    let first = World::identity("change", "plan", "construct", 1);
    let secret = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?
        .secret;
    taken(&world.engine.needs(
        &first.to_string(),
        &secret.0,
        "which design?",
        StopKind::NeedsAnswer,
    ))?;
    let words = "The design agent says: the queue.";
    taken(&world.engine.retry(
        &id,
        Retry::Stage("construct".into()),
        Some(words.into()),
        false,
    ))?;
    ensure_eq!(
        world.terminal.log.of("close"),
        vec![format!("close {first}")],
        "the agent ended first"
    );
    let second = World::identity("change", "plan", "construct", 2);
    let prompt = world.prompt(&id, &second).ok_or("no fresh agent")?;
    let asked = format!("The question: which design?\n\nThe answer: {words}");
    ensure!(
        prompt.contains(&asked),
        "the question then the answer are not in {prompt}"
    );
    Ok(())
}

fn s20_a_working_agent_stopped(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.terminal.set_state(&work, AgentState::Working);
    refused(&world.engine.stop(&id, false))?;
    ensure_eq!(world.terminal.log.count("close"), 0, "closes");
    taken(&world.engine.stop(&id, true))?;
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Stopped),
        "the stage"
    );
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Stopped),
        "the stage after a settle"
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    taken(&world.engine.retry(&id, Retry::Current, None, false))?;
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn s21_a_retry_of_a_landed_place(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    world.place.set_holds(
        &id.0,
        Holds {
            untracked: vec!["notes.txt".into()],
            ..Holds::default()
        },
    );
    world.commit("fix/one", &["fix.rs"]);
    world.settle(&id);
    ensure!(
        matches!(
            world.held(&id).map(|held| held.phase),
            Some(Phase::Over {
                over: Over::Landed { .. },
                ..
            })
        ),
        "the place did not land"
    );
    let reason = refused(
        &world
            .engine
            .retry(&id, Retry::Stage("code".into()), None, false),
    )?;
    ensure!(
        reason.contains(&id.0),
        "the refusal names not the place: {reason}"
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

fn s26_done_on_a_fix(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    let reason = refused(&world.engine.done(&id, false))?;
    ensure!(
        reason.contains(&id.0),
        "the refusal names not the place: {reason}"
    );
    taken(&world.engine.drop_place(&id, false))?;
    ensure!(world.held(&id).is_none(), "the dropped place stands");
    Ok(())
}

fn s9_a_drop_that_keeps_the_branch(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let mut fix = launch("fix", "one", Some("x"));
    fix.hold_submission = true;
    launched(&world.engine.launch(&fix))?;
    world.settle(&id);
    let commit = world.commit("fix/one", &["fix.rs"]);
    let answer = world.engine.drop_place(&id, false);
    let tidy = match &answer {
        Answer::Acted {
            act: Act::Drop,
            tidy: Some(tidy),
            ..
        } => tidy.clone(),
        other => return Err(format!("the drop answered {other:?}")),
    };
    ensure_eq!(
        world.terminal.log.count("close"),
        1,
        "the quiet agent ended"
    );
    ensure_eq!(world.place.log.count("remove"), 1, "removes");
    ensure_eq!(tidy.removed, vec![id.clone()], "the places removed");
    ensure_eq!(tidy.branches_kept.len(), 1, "branches kept");
    ensure_eq!(tidy.branches_kept[0].branch, "fix/one", "the branch named");
    ensure!(
        tidy.branches_kept[0].commits.contains(&commit),
        "the commit is not named"
    );
    ensure!(
        world.git.branch_head(REPOSITORY, "fix/one").is_some(),
        "the branch is gone"
    );
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions");
    Ok(())
}

fn s6_a_tell_into_a_question_then_into_work(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.agent.set_asking(&work, Reading::Yes);
    refused(&world.engine.tell(&id, "use the second", None))?;
    ensure!(
        world
            .terminal
            .sent_to(&work)
            .iter()
            .all(|text| text != "use the second"),
        "words were typed"
    );
    world.agent.set_asking(&work, Reading::No);
    world.terminal.set_state(&work, AgentState::Working);
    world
        .terminal
        .schedule(&work, world.host.now() + 120, AgentState::Waiting);
    let before = world.held(&id).ok_or("no place")?.stages;
    taken(&world.engine.tell(&id, "use the second", None))?;
    ensure_eq!(
        world.terminal.sent_to(&work).last().cloned(),
        Some("use the second".to_string()),
        "the words sent"
    );
    ensure_eq!(
        world.held(&id).ok_or("no place")?.stages,
        before,
        "the stages after a tell"
    );
    Ok(())
}

fn a_tell_past_its_bound(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.terminal.set_state(&work, AgentState::Working);
    let at = world.host.now();
    refused(&world.engine.tell(&id, "later", None))?;
    ensure!(
        world.host.now() - at >= 600,
        "the tell gave up before ten minutes"
    );
    refused(&world.engine.tell(&id, "sooner", Some(60)))?;
    ensure!(
        world
            .terminal
            .sent_to(&work)
            .iter()
            .all(|text| text != "later" && text != "sooner"),
        "words were typed"
    );
    let none = World::new();
    let other = none.begun("build", Some(Deliverable::Commits), "two")?;
    none.terminal
        .kill(&World::identity("build", "two", "work", 1));
    none.settle(&other);
    refused(&none.engine.tell(&other, "anyone?", None))?;
    Ok(())
}

fn s7_the_held_submission_released(_: &()) -> Outcome {
    let world = World::new();
    let mut fix = launch("fix", "one", Some("x"));
    fix.hold_submission = true;
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    world.commit("fix/one", &["fix.rs"]);
    world.settle(&id);
    ensure_eq!(
        world.queue.log.count("submit"),
        0,
        "a held submission was made"
    );
    taken(&world.engine.retry(&id, Retry::Submission, None, false))?;
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

fn finish_while_the_agent_works(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("round", None, "one")?;
    let work = World::identity("round", "one", "work", 1);
    world.terminal.set_state(&work, AgentState::Working);
    refused(&world.engine.finish(&id, false))?;
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Begun),
        "the conversation"
    );
    world.terminal.set_state(&work, AgentState::Waiting);
    taken(&world.engine.finish(&id, false))?;
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Finished),
        "the conversation"
    );
    ensure!(StageState::Finished.ended(), "a finish is no end");
    ensure_eq!(world.terminal.log.count("close"), 1, "closes");
    Ok(())
}

fn a_lost_conversation_finished(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "round",
            false,
            vec![
                stage("talk", None, "{{brief}}"),
                stage("work", Some(Deliverable::Commits), "{{brief}}"),
            ],
        )],
    );
    launched(
        &world
            .engine
            .launch(&launch("round", "one", Some("talk it over"))),
    )?;
    let id = World::id("round", "one");
    world.settle(&id);
    let talk = World::identity("round", "one", "talk", 1);
    world.terminal.kill(&talk);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Lost),
        "the conversation"
    );
    ensure_eq!(
        world.terminal.log.count("open"),
        1,
        "agents opened before the finish"
    );
    taken(&world.engine.finish(&id, false))?;
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Finished),
        "the conversation"
    );
    ensure_eq!(world.terminal.log.count("close"), 0, "closes");
    ensure_eq!(
        world.terminal.log.count("open"),
        2,
        "agents opened after the finish"
    );
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Begun),
        "the stage after the conversation"
    );
    Ok(())
}

fn a_retry_after_readiness_failed(_: &()) -> Outcome {
    let world = World::new();
    world
        .place
        .set_readiness(REPOSITORY, &[("toolchain", false, "offline")]);
    launched(&world.engine.launch(&launch("change", "plan", Some("x"))))?;
    let id = World::id("change", "plan");
    taken(
        &world
            .engine
            .retry(&id, Retry::Stage("construct".into()), None, false),
    )?;
    ensure_eq!(world.place.log.count("ready"), 2, "readiness runs");
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::WaitingOnReadiness),
        "construct"
    );
    world
        .place
        .set_readiness(REPOSITORY, &[("toolchain", true, "installed")]);
    taken(
        &world
            .engine
            .retry(&id, Retry::Stage("construct".into()), None, false),
    )?;
    ensure_eq!(world.place.log.count("ready"), 3, "readiness runs");
    ensure!(
        world
            .stage_state(&id, "construct")
            .is_some_and(StageState::running),
        "construct did not start"
    );
    taken(
        &world
            .engine
            .retry(&id, Retry::Stage("construct".into()), None, false),
    )?;
    ensure_eq!(
        world.place.log.count("ready"),
        3,
        "readiness runs on a later retry"
    );
    Ok(())
}

fn an_end_not_taken(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.terminal.refuse_close(&work);
    let reason = refused(&world.engine.stop(&id, true))?;
    ensure!(
        reason.contains("did not end"),
        "the refusal names no failure: {reason}"
    );
    let status = world.status(&id)?;
    ensure!(!status.failures.is_empty(), "status reports no failure");
    ensure!(status.agent.is_some(), "the agent reads ended");
    Ok(())
}

fn a_stop_and_a_retry_at_once(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let other = world.engine();
    let retried = id.clone();
    world.store.before_next_write(move || {
        let _ = other.retry(&retried, Retry::Current, None, false);
    });
    let _ = world.engine.stop(&id, false);
    let held = world.held(&id).ok_or("no place")?;
    let live: Vec<&StartHeld> = held
        .stages
        .iter()
        .filter_map(|stage| stage.start.as_ref())
        .filter(|start| start.live)
        .collect();
    ensure!(live.len() <= 1, "two agents are held live");
    let running = [1, 2]
        .iter()
        .filter(|start| {
            world
                .terminal
                .state(&World::identity("build", "one", "work", **start))
                != AgentState::Stopped
        })
        .count();
    ensure!(running <= 1, "two agents run");
    let alone = matches!(held.stages[0].state, StageState::Stopped) && live.is_empty()
        || held.stages[0].starts == 2 && live.len() == 1;
    ensure!(
        alone,
        "the place is left as neither act alone leaves it: {:?}",
        held.stages[0]
    );
    Ok(())
}

fn a_drop_repeated(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let first = world.engine.drop_place(&id, false);
    let (closes, removes) = (
        world.terminal.log.count("close"),
        world.place.log.count("remove"),
    );
    ensure_eq!(
        (closes, removes),
        (1, 1),
        "the first drop's closes and removes"
    );
    let second = world.engine.drop_place(&id, false);
    for answer in [&first, &second] {
        ensure!(
            matches!(answer, Answer::Acted { act: Act::Drop, place, .. } if place == &id),
            "a drop answered {answer:?}"
        );
    }
    ensure_eq!(
        (
            world.terminal.log.count("close"),
            world.place.log.count("remove")
        ),
        (closes, removes),
        "the repeat's closes and removes"
    );
    Ok(())
}

fn two_starts_at_once(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_watch(Watch::Bound { seconds: 30 });
    let id = World::id("change", "plan");
    launched(&world.engine.launch(&launch("change", "plan", Some("x"))))?;
    world.settle(&id);
    let mut held = world.held(&id).ok_or("no place")?;
    held.stages[0].state = StageState::Delivered;
    if let Some(start) = held.stages[0].start.as_mut() {
        start.live = false;
    }
    world
        .terminal
        .kill(&World::identity("change", "plan", "construct", 1));
    world.store.put(&id, held);
    let other = world.engine();
    let retried = id.clone();
    world.store.before_next_write(move || {
        let _ = other.retry(&retried, Retry::Stage("code".into()), None, false);
    });
    world.settle(&id);
    let code = World::identity("change", "plan", "code", 1);
    ensure_eq!(
        world
            .terminal
            .log
            .of("open")
            .iter()
            .filter(|open| open.contains(":code:"))
            .count(),
        1,
        "code agents opened"
    );
    ensure!(
        world.terminal.state(&code) != AgentState::Stopped,
        "no code agent runs"
    );
    Ok(())
}

fn identities_never_shared(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    for start in 1..=2 {
        world
            .terminal
            .kill(&World::identity("build", "one", "work", start));
        world.settle(&id);
        taken(&world.engine.retry(&id, Retry::Current, None, false))?;
    }
    let opened: Vec<String> = world.terminal.log.of("open");
    ensure_eq!(opened.len(), 3, "starts");
    for (start, open) in opened.iter().enumerate() {
        let identity = World::identity("build", "one", "work", start as u32 + 1);
        ensure!(
            open.contains(&format!("{identity} ")),
            "the start {} opened as {open}",
            start + 1
        );
    }
    let held = world.held(&id).ok_or("no place")?;
    ensure_eq!(held.stages[0].starts, 3, "the stage's starts");
    Ok(())
}

//! The caller's acts (47 to 55, 109, 114): retry, stop, drop, done,
//! finish and tell, none ending an agent that is not quiet unless forced.

use super::carried_conversations::{by_hand, retry_talk, talk, talking};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Answer, Answering, Retry, KEEP_OPEN};
use flume_seams::*;
use std::sync::{Arc, Mutex};

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "retry: a construct that ended short, its agent let go and ended, retried with words, starts afresh with the question and the answer",
            cites: &["S4", "49", "50", "42"],
            run: s4_retried_with_words,
        },
        Scenario {
            name: "stop: a working agent is stopped only forced, and the stage waits for a retry",
            cites: &["S20", "51", "31", "I2"],
            run: s20_a_working_agent_stopped,
        },
        Scenario {
            name: "retry: refused, forced or not, while the place's agent is live and its stage has begun, naming the agent and the stop that ends it first; once stopped, a retry starts the stage afresh",
            cites: &["49", "50", "51", "31", "35"],
            run: a_retry_of_a_begun_stage,
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
        Scenario {
            name: "retry: a lost talk's retry, unforced, while the agent the terminal brought back bare works in its pane and its session runs, is taken, says talk waits for its conversation to end naming the process, and closes and sends nothing",
            cites: &["49", "50", "77", "207"],
            run: a_bare_agent_at_work_in_a_lost_talks_pane,
        },
        Scenario {
            name: "retry: a lost talk's retry, forced, while its session runs in a pane of the caller's own, is taken and talk waits, flume closing nothing",
            cites: &["49", "207"],
            run: a_session_run_in_a_pane_flume_did_not_make,
        },
        Scenario {
            name: "answer: construct ended short needing an answer naming east/ops, its agent kept open for no time, and the run of east/ops retries construct naming the key stage/construct/1/ended-short with its words: a fresh agent starts with the question then the words, and status gives the end answered by east/ops, with when",
            cites: &["42", "49", "50", "56"],
            run: one_answer_taken_by_a_fresh_agent,
        },
        Scenario {
            name: "answer: the same stop-short's agent kept open, the run of east/ops retries construct naming its key within its time: the answer reaches that agent's session, no agent is ended or started, construct carries on under its first start, and status gives the end answered by east/ops, with when",
            cites: &["42", "49", "53", "198", "S93"],
            run: one_answer_taken_by_the_agent_kept_open,
        },
        Scenario {
            name: "answer: a retry from the pane ops naming the same key a minute later is refused naming east/ops, its words and when, and no agent is ended or started",
            cites: &["42", "49", "S93"],
            run: a_second_answer_to_the_same_question,
        },
        Scenario {
            name: "answer: a tell from the pane ops naming the same key while the agent is still kept open is refused naming east/ops, its words and when, and nothing is typed",
            cites: &["42", "53", "54"],
            run: a_tell_naming_an_answered_key,
        },
        Scenario {
            name: "answer: construct, retried with an answer, delivers and code starts; a late retry naming construct's first stop-short's key is refused naming the move the place waits on now, none, and code goes on",
            cites: &["42", "49", "50"],
            run: an_answer_after_the_place_moved_on,
        },
        Scenario {
            name: "answer: construct's agent kept open takes an answer in its session and stops short again: the second stop-short's key is stage/construct/1/ended-short/1, a retry naming it is taken, and one naming stage/construct/1/ended-short is refused as answered",
            cites: &["42", "43", "49", "198"],
            run: a_second_stop_short_of_one_start,
        },
        Scenario {
            name: "answer: a caller's retry of a stage that ended short, with words and no key, is taken as before, and the end records the answer with no key named",
            cites: &["42", "49"],
            run: an_answer_with_no_key,
        },
        Scenario {
            name: "S93: a change launched in east from no session, its recipient east/operator, asks ops; the run of east/ops is woken, collects the question with its key, holds the waiting move and answers by the key within the hour, construct carrying on in its session; a second answer by the key is refused naming ops's and when, and east/operator is woken for none of it",
            cites: &["S93", "7", "28", "41", "42", "43", "49", "130", "132", "193", "198"],
            run: s93_a_question_for_ops_answered_once,
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
    // Its agent kept open for the answer is let go by its time and ended
    // first, so the retry starts a fresh agent (198).
    world.host.advance(KEEP_OPEN);
    world.settle(&id);
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

fn a_retry_of_a_begun_stage(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    for (state, force) in [(AgentState::Waiting, false), (AgentState::Working, true)] {
        world.terminal.set_state(&work, state);
        let reason = refused(&world.engine.retry(&id, Retry::Current, None, force))?;
        ensure!(
            reason.contains(&work.to_string()) && reason.contains(&format!("flume stop {id}")),
            "the refusal names not the agent and its stop: {reason}"
        );
    }
    ensure_eq!(world.terminal.log.count("close"), 0, "agents closed");
    ensure_eq!(
        world.stage_state(&id, "work"),
        Some(StageState::Begun),
        "the stage"
    );
    world.terminal.set_state(&work, AgentState::Waiting);
    taken(&world.engine.stop(&id, false))?;
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
    taken(&world.engine.stop(&id, false))?;
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
    world.engine.settle_every(30);
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
    let host = world.host.clone();
    let retry = Arc::new(Mutex::new(None));
    let handle = retry.clone();
    world.store.before_next_write(move || {
        let rooms = host.log.count("room");
        let thread = std::thread::spawn(move || {
            let _ = other.retry(&retried, Retry::Stage("code".into()), None, false);
        });
        while host.log.count("room") == rooms {
            std::thread::yield_now();
        }
        *handle
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(thread);
    });
    world.settle(&id);
    let thread = retry
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .take();
    if let Some(thread) = thread {
        thread.join().map_err(|_| "the retry panicked")?;
    }
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

fn a_bare_agent_at_work_in_a_lost_talks_pane(_: &()) -> Outcome {
    let world = World::new();
    let id = talking(&world)?;
    world.host.restart();
    world.terminal.forget_all();
    world.agent.bring_back_bare(&talk(1));
    world.terminal.set_state(&talk(1), AgentState::Working);
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Lost),
        "talk with its agent brought back bare"
    );
    world.agent.live_runs("s-1", &[by_hand()]);
    let sent = world.terminal.log.count("send");
    let status = retry_talk(&world, &id, None, false)?;
    let waiting = status
        .waits_on_session
        .ok_or("the retry's answer says talk waits on nothing")?;
    ensure_eq!(
        (waiting.stage.as_str(), waiting.processes),
        ("talk", vec![by_hand()]),
        "what the retry's answer says talk waits on"
    );
    ensure_eq!(
        (
            world.terminal.log.count("close"),
            world.terminal.log.count("send"),
            world.stage_state(&id, "talk")
        ),
        (0, sent, Some(StageState::WaitingForRoom)),
        "the agents closed, the sendings and talk after the retry"
    );
    Ok(())
}

fn a_session_run_in_a_pane_flume_did_not_make(_: &()) -> Outcome {
    let world = World::new();
    let id = talking(&world)?;
    world.terminal.kill(&talk(1));
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Lost),
        "talk once its agent was killed"
    );
    let place = world.held(&id).ok_or("no held state")?.place.place_ref();
    world.terminal.person_pane(&place, "w1:p9");
    world.agent.live_runs("s-1", &[by_hand()]);
    retry_talk(&world, &id, None, true)?;
    world.settle(&id);
    ensure_eq!(
        (
            world.terminal.log.count("close"),
            world.stage_state(&id, "talk")
        ),
        (0, Some(StageState::WaitingForRoom)),
        "the agents closed and talk after a forced retry"
    );
    Ok(())
}

/// The question construct asks `east/ops` in the answer scenarios.
pub const ASKED: &str = "Which queue's settings are live in east?";

/// The answer `east/ops` gives it.
pub const READ: &str = "east runs the queue at 2.4, as deployed.";

/// The key of construct's first start ended short after `answered`
/// answers in its session (42, 43).
pub fn construct_key(answered: u32) -> EndKey {
    EndKey::Stage {
        stage: "construct".into(),
        start: 1,
        state: StageState::EndedShort,
        answered,
    }
}

/// construct's start `start` of the change `plan`.
fn construct(start: u32) -> Identity {
    World::identity("change", "plan", "construct", start)
}

/// An answer naming `key`, given by `by`.
fn answering(key: Option<EndKey>, by: &str) -> Answering {
    Answering {
        key,
        by: Some(by.into()),
    }
}

/// The question then the answer, as a retry's prompt and an answer handed
/// to a session give them (42, 49).
fn asked(answer: &str) -> String {
    format!("The question: {ASKED}\n\nThe answer: {answer}")
}

/// The change `plan` in `east` whose construct stopped short needing an
/// answer, naming `ops`, and was settled, its agent kept open for the
/// launch's `keep_open` where one is given; the place and when it stopped
/// short (41, 198).
fn asks_ops(world: &World, keep_open: Option<&str>) -> Result<(PlaceId, Seconds), String> {
    let (id, identity, secret) = super::stage_runs::construct_in_east_kept(world, keep_open)?;
    let at = world.host.now();
    taken(&world.engine.needs_to(
        &identity,
        &secret,
        ASKED,
        StopKind::NeedsAnswer,
        Some("ops"),
    ))?;
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::EndedShort),
        "construct once it stopped short"
    );
    Ok((id, at))
}

/// The answer the end keyed `key` holds, as status gives it, with the key
/// status gives that end (42, 56).
fn answer_on(world: &World, id: &PlaceId, key: &EndKey) -> Result<Option<AnswerHeld>, String> {
    let status = world.status(id)?;
    let end = status
        .ends
        .iter()
        .find(|end| end.key == *key)
        .ok_or_else(|| format!("status gives no end keyed {key}"))?;
    Ok(end.end.answer.clone())
}

/// The agents of the change `plan` opened and closed, as the terminal's
/// log gives them.
fn plans_agents(world: &World) -> Vec<String> {
    world
        .terminal
        .log
        .calls()
        .into_iter()
        .filter(|call| {
            (call.starts_with("open") || call.starts_with("close")) && call.contains("change--plan")
        })
        .collect()
}

/// The answer `east/ops` gave by `act` at `at`, naming the key `key`,
/// handed to a session at `at` where `session`, as held state records it.
fn by_ops(act: Act, at: Seconds, key: Option<EndKey>, session: bool) -> AnswerHeld {
    AnswerHeld {
        words: READ.into(),
        act,
        at,
        session: session.then_some(Handed {
            sent_at: at,
            received_at: at,
        }),
        by: Some("east/ops".into()),
        key,
    }
}

/// The run of `east/ops` answered construct's stop-short by a retry naming
/// its key at `at`, construct's agent kept open taking it in its session:
/// the place, and when.
pub fn answered_by_ops(world: &World) -> Result<(PlaceId, Seconds), String> {
    let (id, _) = asks_ops(world, None)?;
    world.host.advance(10 * 60);
    let at = world.host.now();
    taken(&world.engine.retry_answering(
        &id,
        Retry::Stage("construct".into()),
        Some(READ.into()),
        false,
        &answering(Some(construct_key(0)), "east/ops"),
    ))?;
    Ok((id, at))
}

/// A refusal of a second answer to construct's first stop-short: it names
/// `east/ops`, its words and when it was taken.
fn names_ops_answer(reason: &str, at: Seconds) -> Outcome {
    ensure!(
        reason.contains("east/ops") && reason.contains(READ) && reason.contains(&at.to_string()),
        "the refusal names not east/ops, its words and when: {reason}"
    );
    Ok(())
}

fn one_answer_taken_by_a_fresh_agent(_: &()) -> Outcome {
    let world = World::new();
    let (id, _) = asks_ops(&world, Some("0"))?;
    let at = world.host.now();
    taken(&world.engine.retry_answering(
        &id,
        Retry::Stage("construct".into()),
        Some(READ.into()),
        false,
        &answering(Some(construct_key(0)), "east/ops"),
    ))?;
    let prompt = world
        .prompt(&id, &construct(2))
        .ok_or("no fresh agent of construct")?;
    ensure!(
        prompt.contains(&asked(READ)),
        "the question then the answer are not in {prompt}"
    );
    ensure_eq!(
        answer_on(&world, &id, &construct_key(0))?,
        Some(by_ops(Act::Retry, at, Some(construct_key(0)), false)),
        "the answer the stop-short's end holds"
    );
    Ok(())
}

fn one_answer_taken_by_the_agent_kept_open(_: &()) -> Outcome {
    let world = World::new();
    let (id, at) = answered_by_ops(&world)?;
    ensure_eq!(
        world.terminal.sent_to(&construct(1)).last().cloned(),
        Some(asked(READ)),
        "the last words construct's agent was sent"
    );
    ensure_eq!(
        plans_agents(&world),
        vec![format!("open {} {id}", construct(1))],
        "the place's agents opened and closed"
    );
    let held = world.held(&id).ok_or("no held state")?;
    let stage = &held.stages[0];
    ensure_eq!(
        (stage.state, stage.start.as_ref().map(|start| start.n)),
        (StageState::Begun, Some(1)),
        "construct and its start"
    );
    ensure_eq!(
        answer_on(&world, &id, &construct_key(0))?,
        Some(by_ops(Act::Retry, at, Some(construct_key(0)), true)),
        "the answer the stop-short's end holds"
    );
    Ok(())
}

fn a_second_answer_to_the_same_question(_: &()) -> Outcome {
    let world = World::new();
    let (id, at) = answered_by_ops(&world)?;
    world.host.advance(60);
    let sent = world.terminal.sent_to(&construct(1)).len();
    let reason = refused(&world.engine.retry_answering(
        &id,
        Retry::Stage("construct".into()),
        Some("2.3".into()),
        false,
        &answering(Some(construct_key(0)), "ops"),
    ))?;
    names_ops_answer(&reason, at)?;
    ensure_eq!(
        (
            plans_agents(&world).len(),
            world.terminal.sent_to(&construct(1)).len()
        ),
        (1, sent),
        "the place's agents opened and closed, and the words sent to construct's agent"
    );
    ensure_eq!(
        answer_on(&world, &id, &construct_key(0))?,
        Some(by_ops(Act::Retry, at, Some(construct_key(0)), true)),
        "the answer the stop-short's end holds"
    );
    Ok(())
}

fn a_tell_naming_an_answered_key(_: &()) -> Outcome {
    let world = World::new();
    let (id, at) = answered_by_ops(&world)?;
    world.host.advance(60);
    let sent = world.terminal.sent_to(&construct(1)).len();
    let reason = refused(&world.engine.tell_answering(
        &id,
        "2.3",
        None,
        &answering(Some(construct_key(0)), "ops"),
    ))?;
    names_ops_answer(&reason, at)?;
    ensure_eq!(
        world.terminal.sent_to(&construct(1)).len(),
        sent,
        "the words sent to construct's agent"
    );
    Ok(())
}

fn an_answer_after_the_place_moved_on(_: &()) -> Outcome {
    let world = World::new();
    let (id, _) = answered_by_ops(&world)?;
    super::settling::write_change(&world, "plan");
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::Delivered),
        "construct"
    );
    let code = World::identity("change", "plan", "code", 1);
    ensure!(
        world
            .stage_state(&id, "code")
            .is_some_and(StageState::running),
        "code did not start"
    );
    let reason = refused(&world.engine.retry_answering(
        &id,
        Retry::Stage("construct".into()),
        Some("late".into()),
        false,
        &answering(Some(construct_key(0)), "ops"),
    ))?;
    ensure!(
        reason.contains("waits on no move of its caller's now"),
        "the refusal names not the move the place waits on now: {reason}"
    );
    ensure!(
        world
            .stage_state(&id, "code")
            .is_some_and(StageState::running),
        "code did not go on"
    );
    ensure!(
        !plans_agents(&world).contains(&format!("close {code}")),
        "code's agent was closed"
    );
    Ok(())
}

fn a_second_stop_short_of_one_start(_: &()) -> Outcome {
    let world = World::new();
    let (id, at) = answered_by_ops(&world)?;
    world.host.advance(5 * 60);
    let secret = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?
        .secret;
    taken(&world.engine.needs_to(
        &construct(1).to_string(),
        &secret.0,
        "And west's?",
        StopKind::NeedsAnswer,
        Some("ops"),
    ))?;
    world.settle(&id);
    ensure!(
        answer_on(&world, &id, &construct_key(1))?.is_none(),
        "the second stop-short is answered already"
    );
    ensure_eq!(
        construct_key(1).to_string(),
        "stage/construct/1/ended-short/1",
        "the second stop-short's key"
    );
    let reason = refused(&world.engine.retry_answering(
        &id,
        Retry::Stage("construct".into()),
        Some("2.3".into()),
        false,
        &answering(Some(construct_key(0)), "ops"),
    ))?;
    names_ops_answer(&reason, at)?;
    let second = world.host.now();
    taken(&world.engine.retry_answering(
        &id,
        Retry::Stage("construct".into()),
        Some(READ.into()),
        false,
        &answering(Some(construct_key(1)), "east/ops"),
    ))?;
    ensure_eq!(
        answer_on(&world, &id, &construct_key(1))?,
        Some(by_ops(Act::Retry, second, Some(construct_key(1)), true)),
        "the answer the second stop-short's end holds"
    );
    Ok(())
}

fn an_answer_with_no_key(_: &()) -> Outcome {
    let world = World::new();
    let (id, _) = asks_ops(&world, Some("0"))?;
    let at = world.host.now();
    taken(&world.engine.retry(
        &id,
        Retry::Stage("construct".into()),
        Some(READ.into()),
        false,
    ))?;
    let prompt = world
        .prompt(&id, &construct(2))
        .ok_or("no fresh agent of construct")?;
    ensure!(
        prompt.contains(&asked(READ)),
        "the question then the answer are not in {prompt}"
    );
    ensure_eq!(
        answer_on(&world, &id, &construct_key(0))?,
        Some(AnswerHeld {
            words: READ.into(),
            act: Act::Retry,
            at,
            session: None,
            by: None,
            key: None,
        }),
        "the answer the stop-short's end holds, no key named"
    );
    Ok(())
}

/// S93's engine part: no session and no recipient named, the place's
/// recipient is `east/operator`, standing behind `east`; construct asks
/// `ops`, whose run is woken in `east`, collects the question with its
/// key, holds the waiting move and answers by the key within the hour, as
/// the run's own command's recipient names it; construct carries on in its
/// session; a second answer by the key a minute later is refused naming
/// ops's and when; and no run of `east/operator` is launched (7, 28, 41,
/// 42, 43, 49, 130, 132, 193, 198).
fn s93_a_question_for_ops_answered_once(_: &()) -> Outcome {
    let world = World::new();
    super::launching::from_no_session(&world);
    let (id, _) = asks_ops(&world, None)?;
    ensure_eq!(
        world.held(&id).ok_or("no place")?.place.recipients,
        vec!["east/operator".to_string()],
        "the place's recipients"
    );
    let ops = World::id("turn", "east-ops");
    ensure_eq!(
        world.held(&ops).and_then(|held| held.role),
        Some("east/ops".to_string()),
        "the run woken for east/ops"
    );
    // The run collects before its notice lands, as a run whose start
    // collects does (43; core.25).
    world.keep_watch();
    let Answer::Inbox { ends, .. } = world.engine.inbox("east/ops") else {
        return Err("the inbox did not answer".into());
    };
    ensure_eq!(
        ends.iter()
            .map(|collected| (
                collected.place.clone(),
                collected.end.key.clone(),
                match &collected.end.end.of {
                    EndOf::Stage { end, .. } => end.words.clone(),
                    _ => None,
                }
            ))
            .collect::<Vec<_>>(),
        vec![(id.clone(), construct_key(0), Some(ASKED.to_string()))],
        "what the run of east/ops collected: the place, the key and the question"
    );
    taken(&world.engine.hold(
        &id,
        HoldOn::Waiting,
        "east/ops reads the deployed settings",
        Some(60 * 60),
    ))?;
    world.host.advance(20 * 60);
    let by = world
        .engine
        .recipient(Some(&ops))
        .ok_or("the run's command names no recipient")?;
    ensure_eq!(by.as_str(), "east/ops", "the run's command's recipient");
    let at = world.host.now();
    taken(&world.engine.retry_answering(
        &id,
        Retry::Stage("construct".into()),
        Some(READ.into()),
        false,
        &Answering {
            key: Some(construct_key(0)),
            by: Some(by),
        },
    ))?;
    ensure_eq!(
        world.terminal.sent_to(&construct(1)).last().cloned(),
        Some(asked(READ)),
        "the last words construct's agent was sent"
    );
    ensure_eq!(
        (
            world.stage_state(&id, "construct"),
            plans_agents(&world).len()
        ),
        (Some(StageState::Begun), 1),
        "construct, carrying on under its first start, and the place's agents opened and closed"
    );
    ensure_eq!(
        answer_on(&world, &id, &construct_key(0))?,
        Some(by_ops(Act::Retry, at, Some(construct_key(0)), true)),
        "the answer status gives on the stop-short"
    );
    world.host.advance(60);
    let reason = refused(&world.engine.retry_answering(
        &id,
        Retry::Stage("construct".into()),
        Some("2.3".into()),
        false,
        &answering(Some(construct_key(0)), "pane-7"),
    ))?;
    names_ops_answer(&reason, at)?;
    world.engine().settle(None);
    ensure!(
        world.held(&World::id("turn", "east-operator")).is_none(),
        "a run of east/operator was woken"
    );
    Ok(())
}

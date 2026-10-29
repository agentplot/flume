//! Status (43, 56 to 62, 69): every fact read where it lives when asked,
//! at one head per place, from one core, and nothing written, sent or
//! started.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Answer, PlaceStatus, WaitingOn, Waits};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "status: asked three times of one place, it answers the same and starts or tells nothing",
            cites: &["S14", "57", "60", "69", "72"],
            run: s14_asked_three_times,
        },
        Scenario {
            name: "status: a change ticked between two asks is read anew, and neither ask writes",
            cites: &["58", "57", "59"],
            run: a_change_ticked_between_asks,
        },
        Scenario {
            name: "status: every fact of clause 56 for every place, side by side",
            cites: &["56", "43", "7", "19", "48"],
            run: every_fact_of_every_place,
        },
        Scenario {
            name: "status: a place no longer held is not answered for",
            cites: &["62", "58"],
            run: a_place_gone,
        },
        Scenario {
            name: "status: a place whose construct ended short waits on a retry of construct, since then, with the question whole",
            cites: &["56", "130", "42", "41"],
            run: waiting_on_an_answer,
        },
        Scenario {
            name: "status: a lost stage, a stopped one and a done round each name the move they wait on",
            cites: &["56", "130", "45", "51", "114"],
            run: the_moves_waited_on,
        },
    ]
}

fn waiting_on_an_answer(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("change", "plan");
    launched(&world.engine.launch(&launch("change", "plan", Some("x"))))?;
    world.settle(&id);
    let construct = World::identity("change", "plan", "construct", 1);
    let secret = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?
        .secret;
    world.host.advance(300);
    let asked = world.host.now();
    let question = "Which design does the brief mean: the queue,\nor the log?";
    let answer = world.engine.needs(
        &construct.to_string(),
        &secret.0,
        question,
        StopKind::NeedsAnswer,
    );
    ensure!(answer.refusal().is_none(), "the stop-short was refused");
    world.host.advance(60);
    world.settle(&id);
    ensure_eq!(
        world.status(&id)?.waiting_on,
        Some(WaitingOn {
            act: Act::Retry,
            stage: Some("construct".into()),
            why: Waits::Answer,
            since: Some(asked),
            words: Some(question.into()),
        }),
        "the move the place waits on"
    );
    Ok(())
}

fn the_moves_waited_on(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    ensure_eq!(world.status(&id)?.waiting_on, None, "a working place");
    world.terminal.kill(&work);
    world.settle(&id);
    let lost = world
        .status(&id)?
        .waiting_on
        .ok_or("a lost stage waits on nobody")?;
    ensure_eq!(
        (lost.act, lost.stage.as_deref(), lost.why),
        (Act::Retry, Some("work"), Waits::Lost),
        "a lost stage"
    );
    let id = world.begun("tend", Some(Deliverable::Commits), "two")?;
    ensure!(
        world.engine.stop(&id, true).refusal().is_none(),
        "the stop was refused"
    );
    let stopped = world
        .status(&id)?
        .waiting_on
        .ok_or("a stopped stage waits on nobody")?;
    ensure_eq!(
        (stopped.act, stopped.why),
        (Act::Retry, Waits::Stopped),
        "a stopped stage"
    );
    let id = world.begun("sweep", Some(Deliverable::Commits), "three")?;
    world.commit("sweep/three", &["swept.rs"]);
    world.settle(&id);
    let done = world
        .status(&id)?
        .waiting_on
        .ok_or("a delivered round waits on nobody")?;
    ensure_eq!(
        (done.act, done.stage, done.why),
        (Act::Done, None, Waits::Delivered),
        "a round delivered whose kind does not land"
    );
    Ok(())
}

fn places(answer: Answer) -> Result<Vec<PlaceStatus>, String> {
    match answer {
        Answer::Status { places, .. } => Ok(places),
        other => Err(format!("status answered {other:?}")),
    }
}

/// The calls that would start, tell or write.
fn acting_calls(world: &World) -> (usize, usize, usize, usize) {
    (
        world.terminal.log.count("open"),
        world.terminal.log.count("send"),
        world.store.log.count("write") + world.store.log.count("remove"),
        world.queue.log.count("submit"),
    )
}

fn s14_asked_three_times(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::TasksTicked), "one")?;
    world.tick("build/one", "one", 1, 2);
    let before = acting_calls(&world);
    let line = places(world.engine().status(Some(&id)))?;
    let session = places(world.engine().status(Some(&id)))?;
    let sidebar = places(world.engine().status(Some(&id)))?;
    ensure_eq!(line, session, "two surfaces' answers");
    ensure_eq!(session, sidebar, "two surfaces' answers");
    ensure_eq!(
        acting_calls(&world),
        before,
        "calls that start, tell or write"
    );
    Ok(())
}

fn a_change_ticked_between_asks(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::TasksTicked), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.terminal.set_state(&work, AgentState::Working);
    world.tick("build/one", "one", 1, 3);
    let before = acting_calls(&world);
    let first = world.status(&id)?;
    let head = world.tick("build/one", "one", 2, 3);
    let second = world.status(&id)?;
    let ticked = |status: &PlaceStatus| {
        status
            .tasks
            .as_ref()
            .map(|tasks| tasks.iter().filter(|task| task.ticked).count())
    };
    ensure_eq!(
        (ticked(&first), ticked(&second)),
        (Some(1), Some(2)),
        "the tasks ticked"
    );
    ensure_eq!(second.head, Some(head), "the head read");
    ensure_eq!(
        acting_calls(&world),
        before,
        "calls that start, tell or write"
    );
    Ok(())
}

fn every_fact_of_every_place(_: &()) -> Outcome {
    let world = World::new();
    let mut fix = launch("fix", "one", Some("x"));
    fix.mark = Some("team-a slot-2".into());
    launched(&world.engine.launch(&fix))?;
    launched(&world.engine.launch(&launch("change", "plan", Some("y"))))?;
    world.engine().settle(None);
    let fix_id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.agent.refuse_act(&code, "a push of fix/one");
    let all = places(world.engine.status(None))?;
    ensure_eq!(all.len(), 2, "places");
    let one = all
        .iter()
        .find(|place| place.place == fix_id)
        .ok_or("no fix")?;
    ensure_eq!(
        (
            one.repository.as_str(),
            one.kind.as_str(),
            one.version,
            one.declared
        ),
        (REPOSITORY, "fix", 1, Declared::Flume),
        "the repository and kind"
    );
    ensure_eq!(
        (
            one.name.as_str(),
            one.branch.as_str(),
            one.base.as_str(),
            one.lands
        ),
        ("one", "fix/one", "main", true),
        "the name, branch, base and landing"
    );
    ensure_eq!(one.mark.as_deref(), Some("team-a slot-2"), "the mark");
    ensure_eq!(one.stages[0].starts, 1, "the code stage's starts");
    let agent = one.agent.clone().ok_or("no agent")?;
    ensure_eq!(agent.identity, code.to_string(), "the agent's identity");
    ensure_eq!(
        agent.settings.settings.program.as_deref(),
        Some("agent"),
        "the agent's settings"
    );
    ensure_eq!(agent.state, AgentState::Waiting, "the agent's state");
    ensure_eq!(one.delivered, Some(false), "whether the deliverable exists");
    let submission = one.submission.clone().ok_or("no submission")?;
    ensure_eq!(submission.read, SubmissionRead::None, "the submission");
    ensure_eq!(submission.submits, 0, "submissions");
    ensure!(!one.refused_acts.is_empty(), "no refused act");
    let plan = all
        .iter()
        .find(|place| place.kind == "change")
        .ok_or("no change")?;
    ensure!(plan.tasks.is_some(), "no tasks for a change's place");
    Ok(())
}

fn a_place_gone(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    world.engine.drop_place(&id, true);
    let reason = refused(&world.engine.status(Some(&id)))?;
    ensure!(
        reason.contains(&id.0),
        "the refusal names not the place: {reason}"
    );
    ensure!(
        places(world.engine.status(None))?.is_empty(),
        "a gone place is answered for"
    );
    Ok(())
}

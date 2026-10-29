//! A kind runs through on its own (40, 47, 111, 112, 130, 131): each
//! delivery starts the next stage and the last submits, with no caller's
//! word; a failed or conflicting submission, and found work from the
//! kind's last stage, start the kind's fix round with the merge queue's or
//! the agent's words, every going-back counted together toward the kind's
//! bound; found work from any other stage waits on an answer.

use super::settling::{
    archive, change_at_verify, construct_begun, deliver_fix, fix_begun, write_change,
};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Waits};
use flume_fakes::Plan;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "run through: under a terminal that keeps watch a fix runs from its launch to its landing on the watch's and its agent's settles alone, the landing delivered to its recipient and the place tidied",
            cites: &["119", "122", "131", "I8", "43"],
            run: a_fix_runs_unattended,
        },
        Scenario {
            name: "run through: a change launched with nothing on hold runs construct, code and verify to its landing on its agents' going quiet alone",
            cites: &["S35", "S2", "40", "111", "121", "131"],
            run: s35_a_change_runs_through,
        },
        Scenario {
            name: "run through: two fixes launched with notes as briefs each land with no further word",
            cites: &["S36", "40", "111", "131"],
            run: s36_two_fixes_land,
        },
        Scenario {
            name: "fix round: verify finding work starts code at once with the findings, and verify runs again after it",
            cites: &["S35", "131", "41", "22", "30"],
            run: verify_finds_work,
        },
        Scenario {
            name: "fix round: a conflict starts code with words naming the files and the base's revision, and flume merges nothing",
            cites: &["112", "14", "133"],
            run: a_conflict,
        },
        Scenario {
            name: "fix round: a finding then a failure past the kind's bound sends nothing more back, and status gives the move and the words",
            cites: &["112", "131", "23", "130", "56"],
            run: a_finding_then_a_failure_past_the_bound,
        },
        Scenario {
            name: "fix round: construct finding work starts nothing, and waits on an answer, status saying it found work",
            cites: &["131", "42", "41", "56"],
            run: construct_finds_work,
        },
    ]
}

/// The secret of the live start of the place's stage at `index`.
fn secret(world: &World, id: &PlaceId, index: usize) -> Result<Secret, String> {
    Ok(world
        .held(id)
        .and_then(|held| held.stages[index].start.clone())
        .ok_or("no start")?
        .secret)
}

fn s35_a_change_runs_through(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    write_change(&world, "plan");
    world.settle(&id);
    ensure!(
        world
            .stage_state(&id, "code")
            .is_some_and(StageState::running),
        "code did not start in construct's settle"
    );
    world.tick("change/plan", "plan", 0, 2);
    world.settle(&id);
    ensure_eq!(
        world.status(&id)?.tasks.map(|tasks| tasks.len()),
        Some(2),
        "the tasks status gives along the way"
    );
    world.tick("change/plan", "plan", 2, 2);
    world.settle(&id);
    ensure!(
        world
            .stage_state(&id, "verify")
            .is_some_and(StageState::running),
        "verify did not start in code's settle"
    );
    archive(&world, "plan");
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    ensure_eq!(world.terminal.log.count("open"), 3, "agents opened");
    Ok(())
}

fn s36_two_fixes_land(_: &()) -> Outcome {
    let world = World::new();
    let one = fix_begun(&world, "one")?;
    let two = fix_begun(&world, "two")?;
    deliver_fix(&world, &one);
    world.settle(&one);
    deliver_fix(&world, &two);
    world.settle(&two);
    ensure_eq!(world.queue.log.count("submit"), 2, "submissions");
    for id in [&one, &two] {
        ensure!(world.held(id).is_none(), "the landed place {id} is kept");
        ensure!(!world.place.exists(&id.0), "the landed place {id} stands");
    }
    Ok(())
}

fn verify_finds_work(_: &()) -> Outcome {
    let world = World::new();
    let id = change_at_verify(&world, "plan")?;
    let verify = World::identity("change", "plan", "verify", 1);
    let findings = "Task 2.1 is ticked and its test fails.";
    let answer = world.engine.needs(
        &verify.to_string(),
        &secret(&world, &id, 2)?.0,
        findings,
        StopKind::FoundWork,
    );
    ensure!(answer.refusal().is_none(), "the stop-short was refused");
    world.settle(&id);
    ensure!(
        world
            .terminal
            .log
            .of("close")
            .contains(&format!("close {verify}")),
        "verify's agent was not ended before code started"
    );
    let code = World::identity("change", "plan", "code", 2);
    let prompt = world.prompt(&id, &code).ok_or("code did not start")?;
    ensure!(
        prompt.contains(findings),
        "the findings are not in {prompt}"
    );
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.submission.send_backs, 1, "goings-back");
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::NotReached),
        "verify, before code delivers again"
    );
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions");
    world.tick("change/plan", "plan", 2, 2);
    world.settle(&id);
    ensure!(
        world
            .stage_state(&id, "verify")
            .is_some_and(StageState::running),
        "verify did not run again"
    );
    ensure_eq!(
        world.held(&id).map(|held| held.stages[2].starts),
        Some(2),
        "verify's starts"
    );
    Ok(())
}

fn a_conflict(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_begun(&world, "one")?;
    world.commit("main", &["src/lib.rs"]);
    let base = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    world.queue.plan_submits(vec![Plan::Conflict(
        vec!["src/lib.rs".into(), "Cargo.lock".into()],
        "src/lib.rs conflicts with main".into(),
    )]);
    deliver_fix(&world, &id);
    let head = world
        .git
        .branch_head(REPOSITORY, "fix/one")
        .ok_or("no branch")?;
    world.settle(&id);
    let code = World::identity("fix", "one", "code", 2);
    let prompt = world.prompt(&id, &code).ok_or("code did not start again")?;
    for named in [
        "src/lib.rs conflicts with main",
        "src/lib.rs, Cargo.lock",
        &base.to_string(),
    ] {
        ensure!(prompt.contains(named), "{named} is not in {prompt}");
    }
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "fix/one"),
        Some(head),
        "the place's branch, which flume moves not"
    );
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "main"),
        Some(base),
        "the base, which flume moves not"
    );
    match world.status(&id)?.submission.and_then(|s| s.verdict) {
        Some(Verdict::Conflict { files, .. }) => ensure_eq!(
            files,
            vec!["src/lib.rs".to_string(), "Cargo.lock".to_string()],
            "the files contested"
        ),
        other => return Err(format!("the verdict status gives: {other:?}")),
    }
    Ok(())
}

fn a_finding_then_a_failure_past_the_bound(_: &()) -> Outcome {
    let world = World::new();
    let code = stage(
        "code",
        Some(Deliverable::Commits),
        "{{brief}}{{#words}} {{words}}{{/words}}",
    );
    let mut verify = stage(
        "verify",
        Some(Deliverable::CommittedReport {
            path: "verify.md".into(),
        }),
        "Check it.{{#words}} {{words}}{{/words}}",
    );
    verify.send_back_to = Some("code".into());
    verify.send_back_limit = Some(1);
    world
        .kinds
        .set_kinds(REPOSITORY, vec![kind("mend", true, vec![code, verify])]);
    world
        .queue
        .plan_submits(vec![Plan::Fail("check test-y failed".into())]);
    launched(&world.engine.launch(&launch("mend", "one", Some("x"))))?;
    let id = World::id("mend", "one");
    world.settle(&id);
    world.commit("mend/one", &["a.rs"]);
    world.settle(&id);
    let first = World::identity("mend", "one", "verify", 1);
    let answer = world.engine.needs(
        &first.to_string(),
        &secret(&world, &id, 1)?.0,
        "a.rs leaves a case out",
        StopKind::FoundWork,
    );
    ensure!(answer.refusal().is_none(), "the stop-short was refused");
    world.settle(&id);
    ensure_eq!(
        world.held(&id).map(|held| held.submission.send_backs),
        Some(1),
        "goings-back after the finding"
    );
    world.commit("mend/one", &["b.rs"]);
    world.settle(&id);
    world.commit("mend/one", &["verify.md"]);
    let failed_at = world.host.now();
    world.settle(&id);
    let status = world.status(&id)?;
    let submission = status.submission.clone().ok_or("no submission")?;
    ensure_eq!(submission.submits, 1, "submissions");
    ensure_eq!(submission.send_backs, 1, "goings-back");
    ensure_eq!(world.terminal.log.count("open"), 4, "agents opened");
    ensure_eq!(status.phase, Phase::Live, "the place");
    let waiting = status.waiting_on.ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage.as_deref(), waiting.why),
        (Act::Retry, Some("code"), Waits::PastBound),
        "the move the place waits on"
    );
    ensure_eq!(waiting.since, Some(failed_at), "since when it waits");
    ensure!(
        waiting
            .words
            .as_deref()
            .is_some_and(|words| words.contains("check test-y failed")),
        "the merge queue's words are not given: {:?}",
        waiting.words
    );
    Ok(())
}

fn construct_finds_work(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    let construct = World::identity("change", "plan", "construct", 1);
    let findings = "The brief's log already exists on main.";
    let answer = world.engine.needs(
        &construct.to_string(),
        &secret(&world, &id, 0)?.0,
        findings,
        StopKind::FoundWork,
    );
    ensure!(answer.refusal().is_none(), "the stop-short was refused");
    world.settle(&id);
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    let status = world.status(&id)?;
    ensure_eq!(status.stages[0].state, StageState::EndedShort, "construct");
    ensure_eq!(status.stages[1].state, StageState::NotReached, "code");
    ensure_eq!(
        status.stages[0].end.clone().and_then(|end| end.needs),
        Some(StopKind::FoundWork),
        "the kind status gives"
    );
    let waiting = status.waiting_on.ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage.as_deref(), waiting.why),
        (Act::Retry, Some("construct"), Waits::Answer),
        "the move the place waits on"
    );
    ensure_eq!(waiting.words.as_deref(), Some(findings), "the words");
    ensure_eq!(
        world.held(&id).map(|held| held.submission.send_backs),
        Some(0),
        "goings-back"
    );
    Ok(())
}

fn a_fix_runs_unattended(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_watch(Watch::Bound { seconds: 30 });
    world.terminal.add_session("ops", AgentState::Waiting);
    let id = World::id("fix", "one");
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&fix))?;
    // From here only the watch settles every place on its bound, and the
    // agent's going quiet settles its own; no caller acts.
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, at the watch's first settle"
    );
    deliver_fix(&world, &id);
    world.settle(&id);
    for _ in 0..3 {
        world.host.advance(30);
        world.engine().settle(None);
    }
    ensure!(world.held(&id).is_none(), "the place's document stands");
    ensure!(!world.place.exists(&id.0), "the place's worktree stands");
    ensure!(
        world.queue.log.count("submit") == 1,
        "submissions: {:?}",
        world.queue.log.of("submit")
    );
    let notices = world.terminal.notices("ops");
    ensure!(
        notices
            .iter()
            .any(|notice| notice.contains("code delivered"))
            && notices.iter().any(|notice| notice.contains("landed")),
        "the notices to ops: {notices:?}"
    );
    ensure!(
        world
            .store
            .read_every()
            .iter()
            .all(|read| read.state.place.id != id),
        "a tidied document stands with every end delivered"
    );
    Ok(())
}

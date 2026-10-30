//! Settling (40, 46, 111 to 113, 119 to 122, 124, 125, 133): what moves a
//! place on with no process of flume's own, and a landing kind's
//! submission, made with no wait and its outcomes collected at the
//! settles that follow.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_fakes::Plan;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name:
                "settle: a commit and a quiet agent end the stage and start the next in one settle",
            cites: &["S29", "40", "46", "121", "30"],
            run: s29_a_commit_and_a_quiet_agent,
        },
        Scenario {
            name: "settle: a delivered stage whose next waits on its caller starts nothing more",
            cites: &["S7", "47", "70", "17", "I6"],
            run: s7_two_held_places,
        },
        Scenario {
            name: "settle: with nobody watching, a killed agent is read only when a command acts",
            cites: &["S27", "119", "I8", "57"],
            run: s27_killed_with_nobody_watching,
        },
        Scenario {
            name: "settle: under a watch, a crashed agent reads lost within its bound",
            cites: &["S28", "122", "46", "120"],
            run: s28_crashed_under_a_watch,
        },
        Scenario {
            name: "settle: under a watch, a launch and a retry each send the agent they opened its prompt, waiting on no receipt, which the watch's settles read",
            cites: &["6", "37", "38", "46", "49", "122"],
            run: opened_and_sent_under_a_watch,
        },
        Scenario {
            name: "settle: a settle repeated changes nothing",
            cites: &["74", "46"],
            run: a_settle_repeated,
        },
        Scenario {
            name: "settle: a place whose launching command died is undone",
            cites: &["2", "76"],
            run: a_launch_that_died,
        },
        Scenario {
            name: "settle: the agent's record is read in the place's own folder",
            cites: &["128", "77", "102"],
            run: the_record_read_in_the_places_folder,
        },
        Scenario {
            name: "submit: a fix lands, the merge queue removes nothing, and flume's tidy removes the place",
            cites: &["S1", "111", "113", "133", "66", "67", "62", "101"],
            run: s1_a_fix_lands,
        },
        Scenario {
            name: "submit: work landed inside another place's landing is not this place's",
            cites: &["S8", "63", "101"],
            run: s8_another_places_landing,
        },
        Scenario {
            name: "submit: a change's failed submission sends it back to code with the words, verify runs again, then it lands",
            cites: &["S19", "S35", "112", "131", "I7", "22", "48"],
            run: s19_a_failure_then_a_landing,
        },
        Scenario {
            name: "submit: a held submission's place landed by hand is over and never submitted",
            cites: &["S30", "63", "47", "111"],
            run: s30_a_landing_before_flume_submits,
        },
        Scenario {
            name: "submit: after a restart a landing is collected by the next settle, never submitted anew",
            cites: &["S34", "124", "133", "120", "77"],
            run: s34_a_restart_while_under_test,
        },
        Scenario {
            name: "submit: one settle of two places open on one base steps that base once",
            cites: &["125", "124", "111"],
            run: two_places_one_base,
        },
        Scenario {
            name: "submit: an inbox that cannot be read acts on nothing and is named in status",
            cites: &["133", "68", "57"],
            run: an_unreadable_inbox,
        },
        Scenario {
            name: "submit: a place gone back as often as its kind allows waits for its caller",
            cites: &["112", "23"],
            run: the_count_reached,
        },
        Scenario {
            name: "submit: a withdrawn or refused submission leaves the place waiting, status naming why",
            cites: &["112", "117"],
            run: withdrawn_and_refused,
        },
    ]
}

/// A change's construct stage begun, its agent waiting.
pub fn construct_begun(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = World::id("change", name);
    launched(
        &world
            .engine
            .launch(&launch("change", name, Some("plan it"))),
    )?;
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::Begun),
        "construct"
    );
    Ok(id)
}

/// The change written on the place's branch, its planning complete.
pub fn write_change(world: &World, name: &str) {
    let revision = world.commit(
        &format!("change/{name}"),
        &[&format!("plans/{name}/design.md")],
    );
    world.change.set(
        REPOSITORY,
        &revision,
        name,
        ChangeFacts {
            exists: true,
            planning_complete: true,
            tasks: vec![Task {
                text: "1.1 do it".into(),
                ticked: false,
                group: None,
            }],
            archived: false,
            groups: vec![],
        },
    );
}

fn s29_a_commit_and_a_quiet_agent(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    write_change(&world, "plan");
    world.terminal.log.clear();
    world.settle(&id);
    let calls: Vec<String> = world
        .terminal
        .log
        .calls()
        .into_iter()
        .filter(|call| call.starts_with("close") || call.starts_with("open"))
        .collect();
    ensure_eq!(
        calls,
        vec![
            format!(
                "close {}",
                World::identity("change", "plan", "construct", 1)
            ),
            format!(
                "open {} {}",
                World::identity("change", "plan", "code", 1),
                id
            ),
        ],
        "the settle's closes and opens"
    );
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::Delivered),
        "construct"
    );
    ensure!(
        world
            .stage_state(&id, "code")
            .is_some_and(StageState::running),
        "code did not start"
    );
    Ok(())
}

fn the_record_read_in_the_places_folder(_: &()) -> Outcome {
    let world = World::new();
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    let folder = world.store.folder(&id);
    world.agent.keep_record_in(&code, &folder);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, its record read in the place's folder"
    );
    let elsewhere: Vec<String> = world
        .agent
        .log
        .calls()
        .into_iter()
        .filter(|call| !call.starts_with("command") && !call.ends_with(&format!(" in {folder}")))
        .collect();
    ensure_eq!(
        elsewhere,
        Vec::<String>::new(),
        "readings in another folder"
    );
    world.commit("fix/one", &["fix.rs"]);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the place did not land");
    Ok(())
}

fn s7_two_held_places(_: &()) -> Outcome {
    let world = World::new();
    let mut ids = vec![];
    for name in ["one", "two"] {
        let mut held = launch("change", name, Some("plan it"));
        held.hold = vec!["code".into(), "verify".into()];
        held.hold_submission = true;
        held.mark = Some("team-a".into());
        launched(&world.engine.launch(&held))?;
        let id = World::id("change", name);
        world.settle(&id);
        write_change(&world, name);
        world.settle(&id);
        ensure_eq!(
            world.stage_state(&id, "construct"),
            Some(StageState::Delivered),
            "construct"
        );
        ensure_eq!(
            world.stage_state(&id, "code"),
            Some(StageState::OnHold),
            "code"
        );
        ids.push(id);
    }
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    let answer = world
        .engine
        .retry(&ids[0], Retry::Stage("code".into()), None, false);
    ensure!(
        answer.refusal().is_none(),
        "the retry was refused: {answer:?}"
    );
    ensure!(
        world
            .stage_state(&ids[0], "code")
            .is_some_and(StageState::running),
        "the first place's code stage"
    );
    ensure_eq!(
        world.stage_state(&ids[1], "code"),
        Some(StageState::OnHold),
        "the second's"
    );
    let opened = world.terminal.log.count("open");
    let answer = world.engine.retry(&ids[0], Retry::Submission, None, false);
    ensure!(
        answer.refusal().is_none(),
        "the submission's retry was refused: {answer:?}"
    );
    ensure_eq!(world.queue.log.of("submit").len(), 1, "submissions");
    ensure!(
        world.queue.log.of("submit")[0].contains("change/one"),
        "another place was submitted"
    );
    ensure_eq!(
        world.terminal.log.count("open"),
        opened,
        "agents a submission's retry opened"
    );
    ensure_eq!(world.place.log.count("make"), 2, "places made");
    Ok(())
}

fn s27_killed_with_nobody_watching(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let other = World::id("fix", "two");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    launched(&world.engine.launch(&launch("fix", "two", Some("y"))))?;
    world.settle(&id);
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    world.host.advance(3600);
    ensure_eq!(
        world.status(&id)?.stages[0].state,
        StageState::Begun,
        "the stage before a command acts"
    );
    let dropped = world.engine.drop_place(&other, true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.status(&id)?.stages[0].state,
        StageState::Lost,
        "the stage once a command acted"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn s28_crashed_under_a_watch(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_watch(Watch::Bound { seconds: 30 });
    world.agent.no_on_quiet();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    launched(&world.engine.launch(&launch("fix", "two", Some("y"))))?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "a launch under a watch settles not"
    );
    world.host.advance(30);
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "the stage the watch read"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn opened_and_sent_under_a_watch(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_watch(Watch::Bound { seconds: 30 });
    let id = World::id("fix", "one");
    let first = World::identity("fix", "one", "code", 1);
    let at = world.host.now();
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure_eq!(
        world.terminal.sent_to(&first).len(),
        1,
        "the launch's sendings"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage after the launch"
    );
    let second = World::identity("fix", "one", "code", 2);
    world.agent.receives_at(&second, Some(2));
    let retried = world
        .engine()
        .retry(&id, Retry::Stage("code".into()), None, false);
    ensure!(
        retried.refusal().is_none(),
        "the retry was refused: {retried:?}"
    );
    ensure_eq!(
        world.terminal.sent_to(&second).len(),
        1,
        "the retry's sendings"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Started),
        "the code stage after the retry, its sending not yet received"
    );
    ensure_eq!(
        world.host.now(),
        at,
        "the time the launch and the retry waited"
    );
    world.host.advance(30);
    world.engine().settle(None);
    ensure_eq!(
        world.terminal.sent_to(&second).len(),
        2,
        "sendings once the watch read the first not received within 60 seconds"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage"
    );
    Ok(())
}

fn a_settle_repeated(_: &()) -> Outcome {
    let world = World::new();
    let id = construct_begun(&world, "plan")?;
    write_change(&world, "plan");
    world.settle(&id);
    world.settle(&id);
    let held = world.held(&id).ok_or("no place")?;
    let writes = world.store.log.count("write");
    world.settle(&id);
    ensure_eq!(world.held(&id), Some(held), "the held state a repeat left");
    ensure_eq!(
        world.store.log.count("write"),
        writes,
        "writes a repeat made"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn a_launch_that_died(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let mut held = world.held(&id).ok_or("no place")?;
    held.phase = Phase::Making {
        by: world.host.another_command(),
    };
    world.store.put(&id, held);
    world.settle(&id);
    ensure!(
        world.held(&id).is_some(),
        "a place its live launch makes was undone"
    );
    if let Some(Phase::Making { by }) = world.held(&id).map(|held| held.phase) {
        world.host.end_command(&by);
    }
    world.engine().settle(None);
    ensure!(
        world.held(&id).is_none(),
        "the dead launch's held state is left"
    );
    ensure!(
        !world.place.exists(&id.0),
        "the dead launch's worktree is left"
    );
    Ok(())
}

/// A change's place driven to its verify stage, begun.
pub fn change_at_verify(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = construct_begun(world, name)?;
    write_change(world, name);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    world.tick(&format!("change/{name}"), name, 1, 1);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::Begun),
        "verify"
    );
    Ok(id)
}

/// The change archived on the place's branch, its report with it.
pub fn archive(world: &World, name: &str) {
    let revision = world.commit(
        &format!("change/{name}"),
        &[&format!("plans/archive/{name}/verify.md")],
    );
    world.change.set(
        REPOSITORY,
        &revision,
        name,
        ChangeFacts {
            archived: true,
            ..ChangeFacts::default()
        },
    );
}

fn s1_a_fix_lands(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    launched(
        &world
            .engine
            .launch(&launch("fix", "one", Some("the release's brief"))),
    )?;
    world.settle(&id);
    world.commit("fix/one", &["Cargo.toml"]);
    world.agent.set_own_work(&code, Reading::Yes);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code while its test runs"
    );
    world.agent.set_own_work(&code, Reading::No);
    let answer = world.engine().settle(Some(&id));
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    ensure!(
        world.held(&id).is_none(),
        "held state of a landed place is kept"
    );
    ensure!(!world.place.exists(&id.0), "the place stands");
    ensure!(
        world.git.branch_head(REPOSITORY, "fix/one").is_none(),
        "a branch holding nothing its base lacks is kept"
    );
    ensure_eq!(world.place.log.count("remove"), 1, "removes flume made");
    ensure_eq!(world.queue.log.count("inbox"), 2, "inbox reads");
    match answer {
        flume_core::Answer::Settled { tidy, .. } => {
            ensure_eq!(tidy.removed, vec![id.clone()], "the places removed")
        }
        other => return Err(format!("settle answered {other:?}")),
    }
    Ok(())
}

fn s8_another_places_landing(_: &()) -> Outcome {
    let world = World::new();
    let mut held = launch("change", "one", Some("plan it"));
    held.hold = vec!["code".into()];
    launched(&world.engine.launch(&held))?;
    let id = World::id("change", "one");
    world.settle(&id);
    write_change(&world, "one");
    world.settle(&id);
    let head = world
        .git
        .branch_head(REPOSITORY, "change/one")
        .ok_or("no branch")?;
    world.git.set_branch(REPOSITORY, "change/other", &head);
    world.commit("change/other", &["other.rs"]);
    world.queue.hand_landing(REPOSITORY, "change/other", "main");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.phase, Phase::Live, "the place");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::OnHold),
        "code"
    );
    Ok(())
}

fn s19_a_failure_then_a_landing(_: &()) -> Outcome {
    let world = World::new();
    let id = change_at_verify(&world, "plan")?;
    let words = "check test-x failed: 2 of 46";
    world.queue.plan_submits(vec![Plan::Fail(words.into())]);
    archive(&world, "plan");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.submission.send_backs, 1, "send-backs");
    ensure_eq!(held.stages[1].starts, 2, "code's starts");
    ensure_eq!(held.stages[2].starts, 1, "verify's starts");
    let code = World::identity("change", "plan", "code", 2);
    let prompt = world.prompt(&id, &code).ok_or("no second code")?;
    ensure!(prompt.contains(words), "the words are not in {prompt}");
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::Delivered),
        "construct"
    );
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::NotReached),
        "verify, before code delivers again"
    );
    world.tick("change/plan", "plan", 2, 2);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "code, delivered again"
    );
    ensure!(
        world
            .stage_state(&id, "verify")
            .is_some_and(StageState::running),
        "verify did not run again"
    );
    archive(&world, "plan");
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 2, "submissions");
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    Ok(())
}

fn s30_a_landing_before_flume_submits(_: &()) -> Outcome {
    let world = World::new();
    let mut fix = launch("fix", "one", Some("x"));
    fix.hold_submission = true;
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    world.commit("fix/one", &["fix.rs"]);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "code"
    );
    ensure_eq!(
        world.queue.log.count("submit"),
        0,
        "a held submission was made"
    );
    world.queue.hand_landing(REPOSITORY, "fix/one", "main");
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions flume made");
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    Ok(())
}

fn s34_a_restart_while_under_test(_: &()) -> Outcome {
    let world = World::new();
    let id = change_at_verify(&world, "plan")?;
    world.queue.plan_submits(vec![Plan::Hold]);
    archive(&world, "plan");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure!(
        held.submission.id.is_some(),
        "no submission of flume's is recorded"
    );
    ensure_eq!(held.phase, Phase::Live, "the place under test");
    world.host.restart();
    world.terminal.forget_all();
    world.queue.decide(REPOSITORY, "change/plan", Plan::Land);
    world.engine().settle(None);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    ensure!(!world.place.exists(&id.0), "the landed place stands");
    Ok(())
}

/// A fix launched as `name` with its code stage begun.
pub fn fix_begun(world: &World, name: &str) -> Result<PlaceId, String> {
    launched(&world.engine.launch(&launch("fix", name, Some("x"))))?;
    let id = World::id("fix", name);
    world.settle(&id);
    Ok(id)
}

/// The fix's code stage delivered, unsettled.
pub fn deliver_fix(world: &World, id: &PlaceId) {
    let name = id.0.rsplit("--").next().unwrap_or_default();
    world.commit(&format!("fix/{name}"), &["fix.rs"]);
}

fn two_places_one_base(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold, Plan::Hold]);
    let one = fix_begun(&world, "one")?;
    let two = fix_begun(&world, "two")?;
    deliver_fix(&world, &one);
    deliver_fix(&world, &two);
    world.queue.log.clear();
    world.engine().settle(None);
    ensure_eq!(world.queue.log.count("submit"), 2, "submissions");
    ensure_eq!(
        world.queue.log.of("step"),
        vec![format!("step {REPOSITORY} main")],
        "steps one settle made"
    );
    for id in [&one, &two] {
        ensure_eq!(
            world.held(id).map(|held| held.phase),
            Some(Phase::Live),
            "a place under test"
        );
    }
    world.engine().settle(None);
    ensure_eq!(world.queue.log.count("submit"), 2, "submissions made anew");
    ensure_eq!(world.queue.log.count("step"), 2, "steps two settles made");
    Ok(())
}

fn an_unreadable_inbox(_: &()) -> Outcome {
    let world = World::new();
    world
        .queue
        .plan_submits(vec![Plan::Fail("check x failed".into())]);
    let id = fix_begun(&world, "one")?;
    deliver_fix(&world, &id);
    world.queue.moves_on_its_own();
    world.settle(&id);
    world.queue.run_round(REPOSITORY, "main");
    world.queue.unreadable("the merge queue is down");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.submission.send_backs, 0, "send-backs");
    ensure_eq!(held.stages[0].starts, 1, "code's starts");
    ensure!(
        world
            .status(&id)?
            .cannot_read
            .iter()
            .any(|what| what.contains("inbox") && what.contains("down")),
        "status names not the unread inbox"
    );
    world.queue.readable();
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.submission.send_backs, 1, "send-backs once read");
    ensure_eq!(
        held.submission.unread,
        None,
        "the inbox's reason, once read"
    );
    Ok(())
}

fn the_count_reached(_: &()) -> Outcome {
    let world = World::new();
    let mut code = stage(
        "code",
        Some(Deliverable::Commits),
        "{{brief}}{{#words}} {{words}}{{/words}}",
    );
    code.send_back_limit = Some(1);
    code.send_back_to = Some("code".into());
    world
        .kinds
        .set_kinds(REPOSITORY, vec![kind("bump", true, vec![code])]);
    world.queue.plan_submits(vec![
        Plan::Fail("first".into()),
        Plan::Fail("second".into()),
    ]);
    launched(&world.engine.launch(&launch("bump", "one", Some("x"))))?;
    let id = World::id("bump", "one");
    world.settle(&id);
    world.commit("bump/one", &["a.rs"]);
    world.settle(&id);
    world.settle(&id);
    world.commit("bump/one", &["b.rs"]);
    world.settle(&id);
    let status = world.status(&id)?;
    let submission = status.submission.ok_or("no submission")?;
    ensure_eq!(submission.submits, 2, "submissions");
    ensure_eq!(submission.send_backs, 1, "send-backs");
    ensure!(
        submission
            .waiting
            .as_deref()
            .is_some_and(|why| why.contains("second")),
        "status names not why it waits: {:?}",
        submission.waiting
    );
    ensure_eq!(status.stages[0].starts, 2, "code's starts");
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn withdrawn_and_refused(_: &()) -> Outcome {
    for (plan, why) in [
        (Plan::Withdraw, "withdrew"),
        (
            Plan::Refuse("a conflict with main".into()),
            "a conflict with main",
        ),
    ] {
        let world = World::new();
        world.queue.plan_submits(vec![plan]);
        launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
        let id = World::id("fix", "one");
        world.settle(&id);
        world.commit("fix/one", &["fix.rs"]);
        world.settle(&id);
        let status = world.status(&id)?;
        let waiting = status.submission.and_then(|submission| submission.waiting);
        ensure!(
            waiting.as_deref().is_some_and(|text| text.contains(why)),
            "status names not why it waits: {waiting:?}"
        );
        ensure_eq!(status.phase, Phase::Live, "the place");
        ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    }
    Ok(())
}

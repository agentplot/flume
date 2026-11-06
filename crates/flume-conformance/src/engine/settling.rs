//! Settling (40, 46, 111 to 113, 119 to 122, 124, 125, 133, 208): what
//! moves a place on with no process of flume's own, a caller's watch over
//! the host among it, and a landing kind's submission, made with no wait
//! and its outcomes collected at the settles that follow.

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
            name: "settle: with no caller keeping watch and nothing nudging, a killed agent is read only when a command acts",
            cites: &["S27", "119", "I8", "57"],
            run: s27_killed_with_nobody_watching,
        },
        Scenario {
            name: "settle: under a caller's watch, with nothing nudging, a crashed agent reads lost at the caller's next settle, within its bound",
            cites: &["S28", "46", "120", "208"],
            run: s28_crashed_under_a_watch,
        },
        Scenario {
            name: "settle: under a caller's watch, the host's own terminal nudging and a stream under a terminal that nudges nothing, a caller's act settles nothing first, and the stream's places move at the caller's settles",
            cites: &["119", "122", "190", "208", "S91"],
            run: a_watched_host_with_one_stream_unwatched,
        },
        Scenario {
            name: "settle: under a caller's watch, the host's own terminal nudging, a crashed agent reads lost at the nudge's settle before the caller's next, and the nudge writes no watch",
            cites: &["S28", "122", "46", "208"],
            run: crashed_under_a_nudge,
        },
        Scenario {
            name: "settle: a caller's settle with a bound of two minutes holds the host's watch, flume unattended three minutes on and attended at five with no other settle between",
            cites: &["119", "208", "S118"],
            run: a_watch_kept_then_lapsed,
        },
        Scenario {
            name: "settle: a terminal's nudge keeps no watch: with no caller's settle with a bound, a caller's drop settles every place first",
            cites: &["119", "122", "208"],
            run: a_nudge_keeps_no_watch,
        },
        Scenario {
            name: "settle: S109, under a caller's watch a submitted chore with no agent is settled at each of the caller's settles, each stepping its base once, and once it lands the next delivers the landing and tidies it",
            cites: &["S109", "46", "124", "125", "208", "I8"],
            run: s109_a_submitted_place_with_no_agent_moves_under_a_watch,
        },
        Scenario {
            name: "settle: under a caller's watch, a chore submitted to a queue that moves on its own is stepped at each settle to no effect, and the settle after its landing tidies it",
            cites: &["124", "125", "208"],
            run: a_queue_that_moves_on_its_own,
        },
        Scenario {
            name: "settle: one settle of every place with a submission open on main and one open on a branch landing on main steps each base once in the command",
            cites: &["124", "125", "46"],
            run: two_depths_in_one_settle,
        },
        Scenario {
            name: "settle: S118, a host with no watch runs attended, a nudge and each act settling every place, until a caller keeps watch and lands a chore with no act, and four minutes after its last settle began it runs attended again",
            cites: &["S118", "46", "119", "122", "124", "208"],
            run: s118_a_host_with_no_watch_runs_attended,
        },
        Scenario {
            name: "settle: under a caller's watch, a launch and a retry each send the agent they opened its prompt, waiting on no receipt, which the watch's settles read",
            cites: &["6", "37", "38", "46", "49", "208"],
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
            name: "submit: a branch the merge queue cannot read is passed over, named in status with its words, and a place's submission released on its base is still found, followed and landed",
            cites: &["99", "133", "111", "124", "47", "49", "56", "58", "63"],
            run: a_branch_passed_over,
        },
        Scenario {
            name: "submit: a place whose base the merge queue cannot read reads its own submission unread and acts on nothing until the base reads again",
            cites: &["99", "133", "15", "68", "56", "124"],
            run: its_base_passed_over,
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
    world.tick("change/one", "one", 1, 1);
    world.settle(&ids[0]);
    ensure_eq!(
        world.stage_state(&ids[0], "verify"),
        Some(StageState::OnHold),
        "the first place's verify once its code delivered"
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
    world.engine.settle_every(30);
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
    world.engine().settle_every(30);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "the stage the watch read"
    );
    ensure_eq!(world.terminal.log.count("open"), 2, "agents opened");
    Ok(())
}

fn a_watched_host_with_one_stream_unwatched(_: &()) -> Outcome {
    let world = World::new();
    world.engine.settle_every(30);
    world.terminal.set_nudge(Nudge::OnChange);
    let trial = world.other_terminal("other");
    world.host.set_stream("work", Some(Stream::default()));
    world.host.set_display("trial", Some("other"), None, None);
    let id = World::id("fix", "one");
    let other = World::id("fix", "two");
    let tried = World::id("fix", "three");
    for (stream, name) in [("work", "one"), ("work", "two"), ("trial", "three")] {
        launched(&world.engine.launch(&Launch {
            stream: Some(stream.into()),
            ..launch("fix", name, Some("x"))
        }))?;
    }
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    trial.kill(&World::identity("fix", "three", "code", 1));
    let dropped = world.engine.drop_place(&other, true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    for (place, stream) in [(&id, "work"), (&tried, "trial")] {
        ensure_eq!(
            world.status(place)?.stages[0].state,
            StageState::Begun,
            format!("the {stream} stage once the drop acted, which settles nothing first")
        );
    }
    world.host.advance(30);
    world.engine().settle_every(30);
    for (place, stream) in [(&id, "work"), (&tried, "trial")] {
        ensure_eq!(
            world.status(place)?.stages[0].state,
            StageState::Lost,
            format!("the {stream} stage at the caller's settle")
        );
    }
    Ok(())
}

fn opened_and_sent_under_a_watch(_: &()) -> Outcome {
    let world = World::new();
    world.engine.settle_every(30);
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
    // The begun stage's agent is the caller's to end first (50).
    let stopped = world.engine().stop(&id, false);
    ensure!(
        stopped.refusal().is_none(),
        "the stop was refused: {stopped:?}"
    );
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
    world.engine().settle_every(30);
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

/// The words the merge queue refused a branch with on deneb, 2026-11-04.
const REFUSED: &str =
    "bolt/a-run-compiles-through-a-shared-cache's policy at 4f6fe55f: [checkout] declares a cache";

fn a_branch_passed_over(_: &()) -> Outcome {
    let world = World::new();
    let mut fix = launch("fix", "one", Some("x"));
    fix.hold_submission = true;
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    world.commit("fix/one", &["fix.rs"]);
    world.settle(&id);
    let other = "bolt/a-run-compiles-through-a-shared-cache";
    world.queue.refuse_branch(REPOSITORY, other, REFUSED);
    world.queue.plan_submits(vec![Plan::Hold]);
    let released = world
        .engine
        .retry(&id, flume_core::Retry::Submission, None, false);
    if let Some(reason) = released.refusal() {
        return Err(format!("the submission's retry was refused: {reason}"));
    }
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    let status = world.status(&id)?;
    let read = status.submission.map(|submission| submission.read);
    ensure!(
        matches!(&read, Some(SubmissionRead::Found { submission }) if submission.state.open()),
        "the place's submission past {other} read {read:?}"
    );
    ensure_eq!(
        status
            .cannot_read
            .iter()
            .filter(|what| what.contains(other) && what.contains(REFUSED))
            .count(),
        2,
        format!(
            "the branch passed over, by read-submission and the inbox, in {:?}",
            status.cannot_read
        )
    );
    world.queue.decide(REPOSITORY, "fix/one", Plan::Land);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    ensure!(!world.place.exists(&id.0), "the landed place stands");
    Ok(())
}

fn its_base_passed_over(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_begun(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    world.queue.decide(REPOSITORY, "fix/one", Plan::Land);
    world.queue.refuse_branch(REPOSITORY, "main", REFUSED);
    world.queue.log.clear();
    world.settle(&id);
    ensure_eq!(world.queue.log.count("step"), 0, "steps of an unread base");
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.phase, Phase::Live, "the place");
    let status = world.status(&id)?;
    let read = status.submission.map(|submission| submission.read);
    ensure!(
        matches!(&read, Some(SubmissionRead::CouldNotRead { reason }) if reason.contains(REFUSED)),
        "the place's submission on its unread base read {read:?}"
    );
    ensure!(
        status
            .cannot_read
            .iter()
            .any(|what| what.contains("main") && what.contains(REFUSED)),
        "status names not the base passed over: {:?}",
        status.cannot_read
    );
    world.queue.read_branch(REPOSITORY, "main");
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions made anew");
    ensure!(world.held(&id).is_none(), "the landed place is kept");
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

/// The host's watch as the store holds it.
fn host_watch(world: &World) -> Option<Watched> {
    match world.store.host_document(&HostKey::Watch)? {
        HostDocument::Watch(watched) => Some(watched),
        _ => None,
    }
}

/// The host's part of status.
fn host_status(world: &World) -> Result<flume_core::HostStatus, String> {
    match world.engine().status(None) {
        flume_core::Answer::Status { host, .. } => Ok(host),
        other => Err(format!("status answered {other:?}")),
    }
}

/// A chore named `name` for the recipient `ops`, its code stage delivered
/// and its agent ended, the place submitted at its agent's settle.
fn chore_submitted(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = World::id("chore", name);
    let mut chore = launch("chore", name, Some("bump it"));
    chore.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&chore))?;
    world.settle(&id);
    world.commit(&format!("chore/{name}"), &["chore.rs"]);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "the chore submitted");
    ensure!(no_agent_live(world, &id), "an agent of the chore is live");
    Ok(id)
}

/// Whether no agent of the place is live.
fn no_agent_live(world: &World, id: &PlaceId) -> bool {
    world.held(id).is_some_and(|held| {
        held.stages
            .iter()
            .filter_map(|stage| stage.start.as_ref())
            .all(|start| !start.live)
    })
}

fn crashed_under_a_nudge(_: &()) -> Outcome {
    let world = World::new();
    world.engine.settle_every(30);
    let watched = host_watch(&world).ok_or("no watch held")?;
    world.terminal.set_nudge(Nudge::OnChange);
    world.agent.no_on_quiet();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    world.host.advance(2);
    // The terminal's nudge at the agent's change: a settle naming no place.
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "the stage at the nudge's settle"
    );
    ensure_eq!(host_watch(&world), Some(watched), "the watch after a nudge");
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

fn a_watch_kept_then_lapsed(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    for name in ["one", "two", "three"] {
        launched(&world.engine.launch(&launch("fix", name, Some("x"))))?;
    }
    let began = world.host.now();
    world.engine.settle_every(120);
    ensure_eq!(
        host_watch(&world),
        Some(Watched { began, bound: 120 }),
        "the host's watch"
    );
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    world.host.advance(3 * 60);
    let dropped = world.engine.drop_place(&World::id("fix", "two"), true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the stage once a drop acted three minutes on, unattended"
    );
    world.host.advance(2 * 60);
    let dropped = world.engine.drop_place(&World::id("fix", "three"), true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "the stage once a drop acted five minutes on, attended"
    );
    ensure_eq!(
        host_watch(&world),
        Some(Watched { began, bound: 120 }),
        "the host's watch, which only a caller's settle with a bound writes"
    );
    Ok(())
}

fn a_nudge_keeps_no_watch(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_nudge(Nudge::OnChange);
    let id = World::id("fix", "one");
    for name in ["one", "two"] {
        launched(&world.engine.launch(&launch("fix", name, Some("x"))))?;
    }
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    let dropped = world.engine.drop_place(&World::id("fix", "two"), true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "the stage once the drop settled every place first"
    );
    ensure_eq!(host_watch(&world), None, "a watch held");
    Ok(())
}

fn s109_a_submitted_place_with_no_agent_moves_under_a_watch(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    world.engine.settle_every(120);
    let id = chore_submitted(&world, "one")?;
    for pass in 1..=3 {
        world.host.advance(120);
        world.queue.log.clear();
        world.engine().settle_every(120);
        ensure_eq!(
            world.queue.log.of("step"),
            vec![format!("step {REPOSITORY} main")],
            format!("the steps of the caller's settle {pass}")
        );
        ensure_eq!(
            world.held(&id).map(|held| held.phase),
            Some(Phase::Live),
            format!("the chore at the caller's settle {pass}")
        );
        ensure!(
            no_agent_live(&world, &id),
            "an agent of the chore is live at the caller's settle {pass}"
        );
    }
    // The third round lands it.
    world.queue.decide(REPOSITORY, "chore/one", Plan::Land);
    world.host.advance(120);
    world.engine().settle_every(120);
    ensure!(world.held(&id).is_none(), "the chore's document stands");
    ensure!(!world.place.exists(&id.0), "the chore's worktree stands");
    ensure!(
        world
            .terminal
            .notices("ops")
            .iter()
            .any(|notice| notice.contains("landed")),
        "the landing was not delivered: {:?}",
        world.terminal.notices("ops")
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

fn a_queue_that_moves_on_its_own(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.moves_on_its_own();
    world.queue.plan_submits(vec![Plan::Land]);
    world.engine.settle_every(120);
    let id = chore_submitted(&world, "one")?;
    for pass in 1..=2 {
        let before = world.held(&id).ok_or("no chore")?;
        world.host.advance(120);
        world.queue.log.clear();
        world.engine().settle_every(120);
        ensure_eq!(
            world.queue.log.of("step"),
            vec![format!("step {REPOSITORY} main")],
            format!("the steps of the caller's settle {pass}")
        );
        ensure_eq!(
            world.held(&id).map(|held| (held.phase, held.submission)),
            Some((before.phase, before.submission)),
            format!("the chore after the caller's settle {pass}")
        );
    }
    world.queue.run_round(REPOSITORY, "main");
    world.host.advance(120);
    world.engine().settle_every(120);
    ensure!(world.held(&id).is_none(), "the chore's document stands");
    ensure!(
        world
            .terminal
            .notices("ops")
            .iter()
            .any(|notice| notice.contains("landed")),
        "the landing was not delivered: {:?}",
        world.terminal.notices("ops")
    );
    Ok(())
}

fn s118_a_host_with_no_watch_runs_attended(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_nudge(Nudge::OnChange);
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let host = host_status(&world)?;
    ensure_eq!(
        (host.watch, host.watch_settled, host.watch_lapsed),
        (Watch::None, None, None),
        "status before any caller keeps watch"
    );
    launched(&world.engine.launch(&launch("fix", "other", Some("x"))))?;
    let id = World::id("chore", "one");
    let mut chore = launch("chore", "one", Some("bump it"));
    chore.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&chore))?;
    world.settle(&id);
    world.commit("chore/one", &["chore.rs"]);
    world.queue.log.clear();
    // The terminal's nudge at the agent's end settles every place.
    world.engine().settle(None);
    ensure_eq!(world.queue.log.count("submit"), 1, "the chore submitted");
    ensure_eq!(
        world.queue.log.of("step"),
        vec![format!("step {REPOSITORY} main")],
        "the nudge's steps"
    );
    world.queue.log.clear();
    world.host.advance(10 * 60);
    ensure_eq!(world.queue.log.count("step"), 0, "steps with no act");
    let dropped = world.engine.drop_place(&World::id("fix", "other"), true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.queue.log.of("step"),
        vec![format!("step {REPOSITORY} main")],
        "the drop's steps, settling every place first"
    );
    // The runner starts, each pass settling every place with a bound.
    let first = world.host.now();
    world.engine().settle_every(120);
    let host = host_status(&world)?;
    ensure_eq!(
        (host.watch, host.watch_settled, host.watch_lapsed),
        (Watch::Bound { seconds: 120 }, Some(first), None),
        "status under the runner's watch"
    );
    world.queue.decide(REPOSITORY, "chore/one", Plan::Land);
    world.host.advance(120);
    let last = world.host.now();
    world.engine().settle_every(120);
    ensure!(world.held(&id).is_none(), "the chore's document stands");
    ensure!(
        world
            .terminal
            .notices("ops")
            .iter()
            .any(|notice| notice.contains("landed")),
        "the landing was not delivered: {:?}",
        world.terminal.notices("ops")
    );
    // The runner stops; places launched meanwhile are not settled first.
    for name in ["three", "four"] {
        launched(&world.engine.launch(&launch("fix", name, Some("x"))))?;
    }
    let four = World::id("fix", "four");
    world
        .terminal
        .kill(&World::identity("fix", "four", "code", 1));
    world.host.advance(4 * 60);
    let host = host_status(&world)?;
    ensure_eq!(
        (host.watch, host.watch_settled, host.watch_lapsed),
        (Watch::None, Some(last), Some(last + 4 * 60)),
        "status four minutes after the last pass began"
    );
    ensure_eq!(
        world.stage_state(&four, "code"),
        Some(StageState::Begun),
        "the stage before the operator acts"
    );
    let dropped = world.engine.drop_place(&World::id("fix", "three"), true);
    ensure!(
        dropped.refusal().is_none(),
        "the drop was refused: {dropped:?}"
    );
    ensure_eq!(
        world.stage_state(&four, "code"),
        Some(StageState::Lost),
        "the stage once the operator's drop settled every place first"
    );
    Ok(())
}

/// The settling spec's "Two depths in one settle": a chore submitted to
/// `main` and another submitted to `train/x`, a branch that lands on
/// `main`, open together; one settle of every place steps each base once
/// (124, 125).
fn two_depths_in_one_settle(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold, Plan::Hold]);
    world.commit("train/x", &["train.rs"]);
    for (name, base) in [("one", "main"), ("two", "train/x")] {
        let id = World::id("chore", name);
        launched(&world.engine.launch(&Launch {
            base: base.into(),
            ..launch("chore", name, Some("bump it"))
        }))?;
        world.settle(&id);
        world.commit(&format!("chore/{name}"), &["chore.rs"]);
        world.settle(&id);
    }
    ensure_eq!(world.queue.log.count("submit"), 2, "the chores submitted");
    world.queue.log.clear();
    world.engine().settle(None);
    let mut steps = world.queue.log.of("step");
    steps.sort();
    ensure_eq!(
        steps,
        vec![
            format!("step {REPOSITORY} main"),
            format!("step {REPOSITORY} train/x"),
        ],
        "the steps of one settle of every place"
    );
    Ok(())
}

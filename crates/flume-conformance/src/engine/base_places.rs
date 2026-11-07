//! Base places (9, 10, 18, 202 to 206): a place whose kind lands and has
//! no stages, a branch the merge queue opens and closes and the state
//! flume holds, with no worktree, whose branch other places are cut from
//! and land on, at any depth; submitted by name on its caller's retry
//! once none of them is live, its outcome read from the merge queue's
//! record of its branch, and tidied by the merge queue's close. The kind
//! is the repository's own, `train`, and nothing of the core reads its
//! name or a depth to tell it.

use super::settling::{archive, write_change};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "base places: a base place launched on main has its branch opened after its held state is written, nothing made at its location, no readiness and no agent, and waits on its caller with no worktree",
            cites: &["202", "205", "S110", "9", "10", "11", "70", "165", "78"],
            run: a_base_place_launched_on_main,
        },
        Scenario {
            name: "base places: a base place's launch the merge queue refuses its branch is refused with its words, leaving nothing",
            cites: &["2", "8", "205"],
            run: the_merge_queue_refuses_the_branch,
        },
        Scenario {
            name: "base places: a base place whose launching command died has its branch closed by the next settle, and no held state stays",
            cites: &["17", "206", "2", "76"],
            run: a_launch_that_died_while_making,
        },
        Scenario {
            name: "base places: two changes launched on a base place's branch are cut where the merge queue says it lives, submitted to it and tidied once they land, and the base place still waits on its caller",
            cites: &["15", "202", "S110", "9", "126"],
            run: two_changes_on_a_base_places_branch,
        },
        Scenario {
            name: "base places: a base place two deep is opened from its base place's branch, and a fix on it lands there",
            cites: &["202", "205", "9", "15"],
            run: a_base_place_two_deep,
        },
        Scenario {
            name: "base places: a launch on a live change's branch is refused naming the change, and nothing is made",
            cites: &["10", "204", "S111"],
            run: on_a_changes_branch,
        },
        Scenario {
            name: "base places: a launch on a submitted base place's branch is refused naming the base place, and nothing is made",
            cites: &["204", "S111", "111"],
            run: on_a_submitted_base_places_branch,
        },
        Scenario {
            name: "base places: a launch on a branch no place holds is taken as before, cut where the merge queue says it lives",
            cites: &["9", "126", "204"],
            run: on_a_branch_no_place_holds,
        },
    ]
}

/// The repository's kind that lands with no stages, so its places are
/// base places.
fn train() -> Kind {
    kind("train", true, vec![])
}

/// The base place `train--<name>` launched on `base` for the recipient
/// `ops`.
fn train_on(world: &World, name: &str, base: &str) -> Result<PlaceId, String> {
    world.kinds.set_kinds(REPOSITORY, vec![train()]);
    let mut launch = Launch {
        base: base.into(),
        ..launch("train", name, None)
    };
    launch.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&launch))?;
    Ok(World::id("train", name))
}

/// The base place `train--spring` on `main`, its recipient `ops` a
/// session that asks nothing.
fn spring(world: &World) -> Result<PlaceId, String> {
    world.terminal.add_session("ops", AgentState::Waiting);
    train_on(world, "spring", "main")
}

/// A change named `name` launched on `base` and run to its verify stage,
/// begun: a place that works on the base.
fn change_working(world: &World, name: &str, base: &str) -> Result<PlaceId, String> {
    let id = World::id("change", name);
    launched(&world.engine.launch(&Launch {
        base: base.into(),
        ..launch("change", name, Some("plan it"))
    }))?;
    world.settle(&id);
    write_change(world, name);
    world.settle(&id);
    world.settle(&id);
    world.tick(&format!("change/{name}"), name, 1, 1);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::Begun),
        "{name}'s verify"
    );
    Ok(id)
}

/// A change working on `base` archived, so its next settle submits it,
/// the merge queue's round lands it and its tidy removes it.
fn change_lands(world: &World, name: &str) -> Result<(), String> {
    let id = World::id("change", name);
    archive(world, name);
    world.settle(&id);
    ensure!(
        world.held(&id).is_none(),
        "{name} did not land and go: {:?}",
        world.held(&id).map(|held| held.phase)
    );
    Ok(())
}

fn a_base_place_launched_on_main(_: &()) -> Outcome {
    let world = World::new();
    let opened_before = Arc::new(AtomicUsize::new(usize::MAX));
    {
        let queue = world.queue.clone();
        let opened_before = opened_before.clone();
        world.store.before_next_write(move || {
            opened_before.store(queue.log.count("open"), Ordering::SeqCst);
        });
    }
    let id = spring(&world)?;
    ensure_eq!(
        opened_before.load(Ordering::SeqCst),
        0,
        "the opens before the held state's first write"
    );
    ensure_eq!(
        world.queue.log.of("open"),
        vec!["open train/spring from main".to_string()],
        "the merge queue's opens"
    );
    ensure_eq!(world.place.log.count("make"), 0, "the place seam's makes");
    ensure_eq!(world.place.log.count("ready"), 0, "readiness run");
    ensure!(!world.place.exists(&id.0), "something stands at {id}");
    ensure_eq!(world.terminal.log.count("open"), 0, "the terminal's opens");
    let held = world.held(&id).ok_or("no held state")?;
    ensure_eq!(
        Some(held.place.base_revision.clone()),
        world.git.branch_head(REPOSITORY, "train/spring"),
        "the base revision held, the one open made the branch at"
    );
    ensure_eq!(held.phase, Phase::Live, "the phase");
    ensure!(held.stages.is_empty(), "a stage held: {:?}", held.stages);
    let status = world.status(&id)?;
    ensure_eq!(status.current, None, "the current stage");
    ensure_eq!(status.worktree, Some(false), "worktree");
    ensure_eq!(status.on_branch, Some(vec![]), "on_branch");
    ensure!(status.agent.is_none(), "an agent: {:?}", status.agent);
    let waiting = status.waiting_on.ok_or("waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage, waiting.why),
        (Act::Retry, None, Waits::Unsubmitted),
        "the move it waits on its caller for"
    );
    ensure_eq!(
        status.waits_on,
        Some(flume_core::WaitsOn::Caller),
        "whom it waits on"
    );
    ensure_eq!(status.line.short, "waiting on its caller", "its line");
    Ok(())
}

fn the_merge_queue_refuses_the_branch(_: &()) -> Outcome {
    let world = World::new();
    let words = "the policy names no table for train/spring";
    world.queue.answer_open(
        REPOSITORY,
        "train/spring",
        BranchOpened::Refused {
            reason: words.into(),
        },
    );
    world.kinds.set_kinds(REPOSITORY, vec![train()]);
    let reason = refused(&world.engine.launch(&launch("train", "spring", None)))?;
    ensure!(reason.contains(words), "the refusal: {reason}");
    ensure!(
        world.store.places().is_empty(),
        "the store holds {:?}",
        world.store.places()
    );
    ensure_eq!(world.place.log.count("make"), 0, "the place seam's makes");
    Ok(())
}

fn a_launch_that_died_while_making(_: &()) -> Outcome {
    let world = World::new();
    let id = spring(&world)?;
    let mut held = world.held(&id).ok_or("no place")?;
    let by = world.host.another_command();
    held.phase = Phase::Making { by: by.clone() };
    world.store.put(&id, held);
    world.engine().settle(None);
    ensure!(
        world.held(&id).is_some(),
        "a base place its live launch makes was undone"
    );
    ensure_eq!(world.queue.log.count("close"), 0, "closes while it lives");
    world.host.end_command(&by);
    world.engine().settle(None);
    ensure_eq!(
        world.queue.log.of("close"),
        vec!["close train/spring into main".to_string()],
        "the merge queue's closes"
    );
    ensure!(
        world.held(&id).is_none(),
        "the dead launch's held state stays"
    );
    ensure!(
        world.git.branch_head(REPOSITORY, "train/spring").is_none(),
        "the branch open made stays"
    );
    ensure_eq!(
        world.place.log.count("remove"),
        0,
        "the place seam's removes"
    );
    ensure_eq!(
        world.place.log.count("remove_branch"),
        0,
        "the place seam's branch removals"
    );
    Ok(())
}

fn two_changes_on_a_base_places_branch(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    let at = world
        .git
        .branch_head(REPOSITORY, "train/spring")
        .ok_or("no branch train/spring")?;
    for name in ["one", "two"] {
        let id = change_working(&world, name, "train/spring")?;
        let held = world.held(&id).ok_or("no change")?;
        ensure_eq!(held.place.base_revision, at, "{name}'s cut");
        ensure_eq!(
            held.place.base_from,
            Some(BaseFrom::MergeQueue),
            "where {name}'s base was read"
        );
    }
    for name in ["one", "two"] {
        change_lands(&world, name)?;
        let submitted = world
            .queue
            .submissions(REPOSITORY, &format!("change/{name}"));
        ensure_eq!(
            submitted
                .iter()
                .map(|submission| submission.target.as_str())
                .collect::<Vec<_>>(),
            vec!["train/spring"],
            "{name}'s targets"
        );
    }
    world.engine().settle(None);
    let status = world.status(&spring)?;
    ensure_eq!(status.phase, Phase::Live, "the base place's phase");
    ensure_eq!(
        status.waiting_on.map(|waiting| waiting.why),
        Some(Waits::Unsubmitted),
        "what the base place waits on its caller for"
    );
    ensure_eq!(
        world.queue.log.count("submit"),
        2,
        "the submissions, the two changes' alone"
    );
    Ok(())
}

fn a_base_place_two_deep(_: &()) -> Outcome {
    let world = World::new();
    spring(&world)?;
    let week = train_on(&world, "week", "train/spring")?;
    ensure_eq!(
        world.queue.log.of("open"),
        vec![
            "open train/spring from main".to_string(),
            "open train/week from train/spring".to_string(),
        ],
        "the merge queue's opens"
    );
    let at = world
        .git
        .branch_head(REPOSITORY, "train/week")
        .ok_or("no branch train/week")?;
    let fix = World::id("fix", "deep");
    launched(&world.engine.launch(&Launch {
        base: "train/week".into(),
        ..launch("fix", "deep", Some("mend it"))
    }))?;
    ensure_eq!(
        world.held(&fix).map(|held| held.place.base_revision),
        Some(at),
        "the fix's cut"
    );
    world.settle(&fix);
    world.commit("fix/deep", &["deep.rs"]);
    world
        .agent
        .set_own_work(&World::identity("fix", "deep", "code", 1), Reading::No);
    world.settle(&fix);
    ensure!(world.held(&fix).is_none(), "the fix did not land and go");
    let submitted = world.queue.submissions(REPOSITORY, "fix/deep");
    ensure!(
        matches!(
            submitted.as_slice(),
            [Submission { target, state: SubmissionState::Landed { .. }, .. }] if target == "train/week"
        ),
        "the fix's submissions: {submitted:?}"
    );
    ensure_eq!(
        world.held(&week).map(|held| held.phase),
        Some(Phase::Live),
        "the base place two deep"
    );
    Ok(())
}

fn on_a_changes_branch(_: &()) -> Outcome {
    let world = World::new();
    change_working(&world, "one", "main")?;
    world.place.log.clear();
    world.queue.log.clear();
    let before = world.store.places();
    let reason = refused(&world.engine.launch(&Launch {
        base: "change/one".into(),
        ..launch("fix", "under", Some("mend it"))
    }))?;
    ensure!(
        reason.contains(&World::id("change", "one").to_string()),
        "the refusal names no change: {reason}"
    );
    ensure_eq!(world.store.places(), before, "the places held");
    ensure_eq!(world.place.log.count("make"), 0, "the place seam's makes");
    ensure_eq!(world.queue.log.count("open"), 0, "the merge queue's opens");
    Ok(())
}

fn on_a_submitted_base_places_branch(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    world
        .engine()
        .retry(&spring, flume_core::Retry::Submission, None, false);
    ensure_eq!(
        world.queue.log.count("submit"),
        1,
        "the base place's submits"
    );
    world.place.log.clear();
    let before = world.store.places();
    let reason = refused(&world.engine.launch(&Launch {
        base: "train/spring".into(),
        ..launch("fix", "late", Some("mend it"))
    }))?;
    ensure!(
        reason.contains(&spring.to_string()),
        "the refusal names no base place: {reason}"
    );
    ensure_eq!(world.store.places(), before, "the places held");
    ensure_eq!(world.place.log.count("make"), 0, "the place seam's makes");
    Ok(())
}

fn on_a_branch_no_place_holds(_: &()) -> Outcome {
    let world = World::new();
    let old = world.commit("bolt/old", &["old.rs"]);
    let id = World::id("fix", "on-old");
    launched(&world.engine.launch(&Launch {
        base: "bolt/old".into(),
        ..launch("fix", "on-old", Some("mend it"))
    }))?;
    let held = world.held(&id).ok_or("no place")?;
    ensure_eq!(held.place.base_revision, old, "the cut");
    ensure_eq!(
        held.place.base_from,
        Some(BaseFrom::MergeQueue),
        "where the base was read"
    );
    ensure_eq!(world.place.log.count("make"), 1, "the place seam's makes");
    Ok(())
}

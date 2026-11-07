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
            cites: &["202", "205", "S110", "9", "10", "11", "70", "78"],
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
        Scenario {
            name: "base places: S110, a base place on main and two changes on its branch land under a caller's watch, the base place submitted by name on its caller's retry, its landing read from the merge queue's record and delivered, and its branch closed into main at its tidy",
            cites: &["S110", "63", "67", "202", "203", "204", "205", "206", "117", "124", "133", "208"],
            run: s110_a_base_place_and_two_changes_land,
        },
        Scenario {
            name: "base places: S111, a retry of a base place's submission is refused while a change on its branch works, submits once it lands, and then a launch on its branch is refused, as is a launch on a live change's branch",
            cites: &["S111", "10", "203", "204", "49"],
            run: s111_a_retry_and_a_launch_refused,
        },
        Scenario {
            name: "base places: twenty settles of a base place with no place on its branch submit nothing, and it waits on its caller",
            cites: &["111", "203"],
            run: no_settle_submits_it,
        },
        Scenario {
            name: "base places: a base place's landing is read from the merge queue's record with no inbox asked, delivered once to its recipient, and its work is over",
            cites: &["63", "133", "203", "43", "99"],
            run: landed_with_no_inbox,
        },
        Scenario {
            name: "base places: S112, a base place's failed submission starts nothing and waits on its caller with the merge queue's words, delivered once; a fix launched on its branch with them lands there, and the retry lands the base place",
            cites: &["S112", "112", "203", "204", "43", "36"],
            run: s112_a_failed_base_place_takes_a_fix,
        },
        Scenario {
            name: "base places: a base place's conflicting submission starts no stage, and waits on its caller with words naming the files and the base's revision it was tested against",
            cites: &["112", "203", "S112"],
            run: a_conflicting_submission,
        },
        Scenario {
            name: "base places: a drop of a base place while a change on its branch works is refused naming the change, and the base place stays live",
            cites: &["52", "204"],
            run: a_drop_while_a_change_works,
        },
        Scenario {
            name: "base places: a base place whose submission landed is tidied by the merge queue's close into main, the place seam asked no removal, and no held state stays once its ends are delivered",
            cites: &["62", "66", "67", "204", "206", "S110"],
            run: the_landed_base_place_tidied,
        },
        Scenario {
            name: "base places: a dropped base place whose branch the merge queue keeps is removed, the branch named once with the merge queue's words in the drop's answer, and a launch of it again refused while the branch stands",
            cites: &["3", "67", "206", "52"],
            run: the_merge_queue_keeps_the_branch,
        },
        Scenario {
            name: "base places: an hour after its launch, a base place with two changes on its branch gives worktree false, waits on its places, names them, its head where the merge queue says its branch lives, no stall, and a line ending 2 on its branch · open; a change gives neither worktree nor on_branch",
            cites: &["56", "61", "80", "126", "130", "203"],
            run: waiting_on_its_places,
        },
        Scenario {
            name: "base places: once the changes on its branch landed, a base place waits on its caller for its submission since the settle that read none live, is stalled after six minutes, and each recipient is delivered the stall once",
            cites: &["43", "130", "203"],
            run: waiting_on_its_caller,
        },
        Scenario {
            name: "base places: the list gives a base place with one change on its branch a line ending 1 on its branch · open",
            cites: &["139", "200", "203"],
            run: the_line_of_a_base_place_with_one_place_on_its_branch,
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

/// The notice a base place's end is sent to its recipient as, with its
/// key, pointing at status where the place waits on no move of its
/// caller's as it is sent (43; design 9).
fn notice(name: &str, what: &str, key: &str) -> String {
    format!("flume: train--{name} {what} [{key}]; read: flume status train--{name}")
}

/// The notice of an end sent while the place waits on its caller, which
/// points at the inbox (43, 130; design 9).
fn notice_waiting(name: &str, what: &str, key: &str) -> String {
    format!("flume: train--{name} {what} [{key}]; read: flume inbox")
}

/// A retry of the base place's submission, or its refusal's reason.
fn retry_submission(world: &World, place: &PlaceId) -> Result<(), String> {
    match world
        .engine()
        .retry(place, flume_core::Retry::Submission, None, false)
    {
        flume_core::Answer::Acted { .. } => Ok(()),
        other => Err(format!("the retry answered {other:?}")),
    }
}

fn s110_a_base_place_and_two_changes_land(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let watch = || world.engine().settle_every(30);
    watch();
    let spring = train_on(&world, "spring", "main")?;
    ensure_eq!(
        world.queue.log.of("open"),
        vec!["open train/spring from main".to_string()],
        "the merge queue's opens"
    );
    ensure_eq!(world.place.log.count("make"), 0, "the place seam's makes");
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    for name in ["one", "two"] {
        launched(&world.engine.launch(&Launch {
            base: "train/spring".into(),
            ..launch("change", name, Some("plan it"))
        }))?;
    }
    watch();
    for name in ["one", "two"] {
        write_change(&world, name);
    }
    watch();
    watch();
    for name in ["one", "two"] {
        world.tick(&format!("change/{name}"), name, 1, 1);
    }
    watch();
    watch();
    for name in ["one", "two"] {
        ensure_eq!(
            world.stage_state(&World::id("change", name), "verify"),
            Some(StageState::Begun),
            "{name}'s verify"
        );
        archive(&world, name);
    }
    // Each watch's settle steps the branch's queue once, so the second
    // change, submitted after the first's round, lands at the next (125).
    watch();
    watch();
    for name in ["one", "two"] {
        ensure!(
            world.held(&World::id("change", name)).is_none(),
            "{name} did not land and go"
        );
        let submitted = world
            .queue
            .submissions(REPOSITORY, &format!("change/{name}"));
        ensure!(
            matches!(
                submitted.as_slice(),
                [Submission { target, state: SubmissionState::Landed { .. }, .. }] if target == "train/spring"
            ),
            "{name}'s submissions: {submitted:?}"
        );
    }
    ensure!(
        world
            .queue
            .submissions(REPOSITORY, "train/spring")
            .is_empty(),
        "the base place was submitted by a settle"
    );
    ensure_eq!(
        world.status(&spring)?.waiting_on.map(|waiting| waiting.why),
        Some(Waits::Unsubmitted),
        "what the base place waits on its caller for"
    );
    retry_submission(&world, &spring)?;
    ensure_eq!(
        world.queue.log.of("submit").last().cloned(),
        Some("submit train/spring named".to_string()),
        "the base place's submit"
    );
    let landed = match world
        .queue
        .submissions(REPOSITORY, "train/spring")
        .as_slice()
    {
        [Submission {
            target,
            state: SubmissionState::Landed { commit },
            ..
        }] if target == "main" => commit.clone(),
        other => return Err(format!("the base place's submissions: {other:?}")),
    };
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![notice(
            "spring",
            &format!("submission landed {landed}"),
            "submission/1/landed"
        )],
        "the notices to ops"
    );
    ensure!(world.held(&spring).is_none(), "the base place stays");
    ensure_eq!(
        world.queue.log.of("close"),
        vec!["close train/spring into main".to_string()],
        "the merge queue's closes"
    );
    ensure!(
        world.git.branch_head(REPOSITORY, "train/spring").is_none(),
        "the base place's branch stays"
    );
    let location = spring.to_string();
    ensure!(
        !world
            .place
            .log
            .calls()
            .iter()
            .chain(world.queue.log.of("inbox").iter())
            .any(|call| call.split(' ').any(|word| word == location)
                || call.starts_with(&format!("remove_branch {REPOSITORY} train/spring "))),
        "the place seam or an inbox was asked of the base place: {:?}",
        world.place.log.calls()
    );
    for name in ["one", "two"] {
        ensure!(
            world
                .git
                .branch_head(REPOSITORY, &format!("change/{name}"))
                .is_none(),
            "{name}'s branch stays"
        );
    }
    Ok(())
}

fn s111_a_retry_and_a_launch_refused(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    change_working(&world, "one", "train/spring")?;
    let reason = match world
        .engine()
        .retry(&spring, flume_core::Retry::Submission, None, false)
    {
        answer @ flume_core::Answer::Refused { .. } => refused(&answer)?,
        other => return Err(format!("the retry answered {other:?}")),
    };
    ensure!(
        reason.contains(&World::id("change", "one").to_string()),
        "the refusal names no change: {reason}"
    );
    ensure_eq!(world.queue.log.count("submit"), 0, "submits");
    change_lands(&world, "one")?;
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    retry_submission(&world, &spring)?;
    ensure_eq!(
        world.queue.log.of("submit").last().cloned(),
        Some("submit train/spring named".to_string()),
        "the base place's submit"
    );
    let reason = refused(&world.engine.launch(&Launch {
        base: "train/spring".into(),
        ..launch("fix", "late", Some("mend it"))
    }))?;
    ensure!(
        reason.contains(&spring.to_string()),
        "the refusal names no base place: {reason}"
    );
    change_working(&world, "two", "main")?;
    let reason = refused(&world.engine.launch(&Launch {
        base: "change/two".into(),
        ..launch("fix", "under", Some("mend it"))
    }))?;
    ensure!(
        reason.contains(&World::id("change", "two").to_string()),
        "the refusal names no change: {reason}"
    );
    Ok(())
}

fn no_settle_submits_it(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    for _ in 0..20 {
        world.engine().settle(None);
    }
    ensure_eq!(world.queue.log.count("submit"), 0, "submits");
    let status = world.status(&spring)?;
    ensure_eq!(
        status.waits_on,
        Some(flume_core::WaitsOn::Caller),
        "whom it waits on"
    );
    ensure_eq!(
        status.waiting_on.map(|waiting| (waiting.act, waiting.why)),
        Some((Act::Retry, Waits::Unsubmitted)),
        "what it waits on its caller for"
    );
    Ok(())
}

fn landed_with_no_inbox(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    retry_submission(&world, &spring)?;
    ensure_eq!(
        world.status(&spring)?.line.short,
        "submitted",
        "the line while under test"
    );
    world
        .queue
        .decide(REPOSITORY, "train/spring", flume_fakes::Plan::Land);
    world.engine().settle(None);
    world.engine().settle(None);
    let landed = world
        .queue
        .submissions(REPOSITORY, "train/spring")
        .into_iter()
        .find_map(|submission| match submission.state {
            SubmissionState::Landed { commit } => Some(commit),
            _ => None,
        })
        .ok_or("the submission did not land")?;
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![notice(
            "spring",
            &format!("submission landed {landed}"),
            "submission/1/landed"
        )],
        "the notices to ops, once"
    );
    ensure!(world.held(&spring).is_none(), "the landed base place stays");
    ensure!(
        !world
            .queue
            .log
            .of("inbox")
            .iter()
            .any(|call| call.contains(&spring.to_string())),
        "an inbox was asked of the base place"
    );
    Ok(())
}

fn s112_a_failed_base_place_takes_a_fix(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    let words = "the check test failed on main: 2 of 46";
    world
        .queue
        .plan_submits(vec![flume_fakes::Plan::Fail(words.into())]);
    retry_submission(&world, &spring)?;
    world.engine().settle(None);
    let status = world.status(&spring)?;
    ensure_eq!(status.phase, Phase::Live, "the failed base place");
    let waiting = status.waiting_on.ok_or("waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.why),
        (Act::Retry, Waits::Submission),
        "the move it waits on"
    );
    ensure!(
        waiting
            .words
            .as_deref()
            .is_some_and(|said| said.contains(words)),
        "its words: {:?}",
        waiting.words
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    let fix = World::id("fix", "spring-test");
    launched(&world.engine.launch(&Launch {
        base: "train/spring".into(),
        ..launch("fix", "spring-test", Some(words))
    }))?;
    world.settle(&fix);
    world.commit("fix/spring-test", &["test.rs"]);
    world.agent.set_own_work(
        &World::identity("fix", "spring-test", "code", 1),
        Reading::No,
    );
    world.settle(&fix);
    ensure!(world.held(&fix).is_none(), "the fix did not land and go");
    retry_submission(&world, &spring)?;
    world.engine().settle(None);
    ensure!(
        world.held(&spring).is_none(),
        "the base place did not land and go"
    );
    let notices = world.terminal.notices("ops");
    ensure_eq!(
        notices
            .iter()
            .filter(|text| {
                **text == notice_waiting("spring", "submission failed", "submission/1/failed")
            })
            .count(),
        1,
        "notices of the failure: {notices:?}"
    );
    ensure!(
        notices
            .iter()
            .any(|text| text.starts_with("flume: train--spring submission landed")),
        "no notice of the landing: {notices:?}"
    );
    ensure!(
        world
            .terminal
            .log
            .of("open")
            .iter()
            .all(|call| !call.contains("train--spring")),
        "an agent opened in the base place"
    );
    Ok(())
}

fn a_conflicting_submission(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    world.commit("main", &["README.md"]);
    let tested = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    world.queue.plan_submits(vec![flume_fakes::Plan::Conflict(
        vec!["README.md".into()],
        "README.md conflicts with main".into(),
    )]);
    retry_submission(&world, &spring)?;
    world.engine().settle(None);
    let status = world.status(&spring)?;
    let waiting = status.waiting_on.ok_or("waits on nobody")?;
    ensure_eq!(waiting.why, Waits::Submission, "why it waits");
    let said = waiting.words.unwrap_or_default();
    ensure!(
        said.contains("README.md") && said.contains(&tested.to_string()),
        "its words name neither the file nor {tested}: {said}"
    );
    ensure!(status.stages.is_empty(), "a stage: {:?}", status.stages);
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    Ok(())
}

fn a_drop_while_a_change_works(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    change_working(&world, "one", "train/spring")?;
    let reason = refused(&world.engine().drop_place(&spring, false))?;
    ensure!(
        reason.contains(&World::id("change", "one").to_string()),
        "the refusal names no change: {reason}"
    );
    ensure_eq!(
        world.held(&spring).map(|held| held.phase),
        Some(Phase::Live),
        "the base place"
    );
    Ok(())
}

fn the_landed_base_place_tidied(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    retry_submission(&world, &spring)?;
    world.place.log.clear();
    world.queue.log.clear();
    world
        .queue
        .decide(REPOSITORY, "train/spring", flume_fakes::Plan::Land);
    let answer = world.engine().settle(None);
    ensure_eq!(
        world.queue.log.of("close"),
        vec!["close train/spring into main".to_string()],
        "the merge queue's closes"
    );
    ensure!(
        world.place.log.calls().is_empty(),
        "the place seam was asked: {:?}",
        world.place.log.calls()
    );
    ensure!(
        world.held(&spring).is_none(),
        "the base place's held state stays"
    );
    ensure!(
        world.store.places().is_empty(),
        "documents stay: {:?}",
        world.store.places()
    );
    match answer {
        flume_core::Answer::Settled { tidy, .. } => {
            ensure_eq!(tidy.removed, vec![spring.clone()], "the places removed");
            ensure!(
                tidy.branches_kept.is_empty(),
                "a branch kept: {:?}",
                tidy.branches_kept
            );
        }
        other => return Err(format!("settle answered {other:?}")),
    }
    Ok(())
}

fn the_merge_queue_keeps_the_branch(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    world.queue.answer_close(
        REPOSITORY,
        "train/spring",
        BranchClosed::Kept {
            reason: "work main lacks".into(),
        },
    );
    let tidy = match world.engine().drop_place(&spring, false) {
        flume_core::Answer::Acted {
            tidy: Some(tidy), ..
        } => tidy,
        other => return Err(format!("the drop answered {other:?}")),
    };
    ensure_eq!(tidy.removed, vec![spring.clone()], "the places removed");
    ensure_eq!(
        tidy.branches_kept
            .iter()
            .map(|kept| (kept.branch.as_str(), kept.reason.as_deref()))
            .collect::<Vec<_>>(),
        vec![("train/spring", Some("work main lacks"))],
        "the branches kept, named once"
    );
    ensure_eq!(
        world.place.log.count("remove_branch"),
        0,
        "the place seam's branch removals"
    );
    ensure!(
        world.held(&spring).is_none(),
        "the dropped base place stays"
    );
    world.engine().settle(None);
    ensure_eq!(
        world.queue.log.count("close"),
        1,
        "the merge queue's closes"
    );
    ensure!(
        world.git.branch_head(REPOSITORY, "train/spring").is_some(),
        "the kept branch is gone"
    );
    let reason = refused(&world.engine.launch(&launch("train", "spring", None)))?;
    ensure!(
        reason.contains("train/spring"),
        "the refusal names no branch: {reason}"
    );
    Ok(())
}

fn waiting_on_its_places(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    let one = change_working(&world, "one", "train/spring")?;
    let two = change_working(&world, "two", "train/spring")?;
    world.host.advance(3600);
    world.engine().settle(None);
    let status = world.status(&spring)?;
    ensure_eq!(status.worktree, Some(false), "worktree");
    ensure_eq!(
        status.waits_on,
        Some(flume_core::WaitsOn::Places),
        "whom it waits on"
    );
    let mut on_branch = status.on_branch.clone().ok_or("no on_branch")?;
    on_branch.sort();
    ensure_eq!(on_branch, vec![one.clone(), two.clone()], "on_branch");
    ensure_eq!(
        status.head,
        world.git.branch_head(REPOSITORY, "train/spring"),
        "the head"
    );
    ensure!(
        world
            .queue
            .log
            .of("base_revision")
            .contains(&format!("base_revision {REPOSITORY} train/spring")),
        "the head was not read where the merge queue says the branch lives"
    );
    ensure!(
        !status.stalled && status.waiting_on.is_none(),
        "it waits on its caller: {:?}, stalled {}",
        status.waiting_on,
        status.stalled
    );
    ensure!(
        !status
            .ends
            .iter()
            .any(|end| matches!(end.end.of, EndOf::Stall { .. })),
        "a stall was read"
    );
    ensure!(
        status.line.full.ends_with("2 on its branch · open"),
        "its line: {}",
        status.line.full
    );
    ensure!(
        !world
            .place
            .log
            .calls()
            .iter()
            .any(|call| call.split(' ').any(|word| word == spring.to_string())),
        "its location was read"
    );
    let change = world.status(&one)?;
    ensure_eq!(
        (change.worktree, change.on_branch),
        (None, None),
        "a change's worktree and on_branch"
    );
    let json: serde_json::Value =
        serde_json::from_str(&world.engine().status(Some(&one)).to_json())
            .map_err(|error| error.to_string())?;
    ensure!(
        json["status"]["places"][0].get("worktree").is_none()
            && json["status"]["places"][0].get("on_branch").is_none(),
        "a change's status gives worktree or on_branch"
    );
    Ok(())
}

fn waiting_on_its_caller(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    change_working(&world, "one", "train/spring")?;
    change_working(&world, "two", "train/spring")?;
    change_lands(&world, "one")?;
    change_lands(&world, "two")?;
    let read_none = world.host.now();
    world.engine().settle(None);
    world.host.advance(6 * 60);
    world.engine().settle(None);
    world.engine().settle(None);
    let status = world.status(&spring)?;
    let waiting = status.waiting_on.clone().ok_or("waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage, waiting.why, waiting.since),
        (Act::Retry, None, Waits::Unsubmitted, Some(read_none)),
        "the move it waits on its caller for"
    );
    ensure!(status.stalled, "not stalled");
    ensure_eq!(status.line.short, "stalled", "its line");
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![notice_waiting(
            "spring",
            "stalled: a retry of the submission",
            &format!("stall/{read_none}")
        )],
        "the notices to ops"
    );
    Ok(())
}

fn the_line_of_a_base_place_with_one_place_on_its_branch(_: &()) -> Outcome {
    let world = World::new();
    spring(&world)?;
    change_working(&world, "one", "train/spring")?;
    let places = match world.engine().list(false) {
        flume_core::Answer::List { places, .. } => places,
        other => return Err(format!("the list answered {other:?}")),
    };
    let listed = places
        .iter()
        .find(|listed| listed.name == "train--spring")
        .ok_or("no line of train--spring")?;
    ensure!(
        listed.line.ends_with("1 on its branch · open"),
        "its line: {}",
        listed.line
    );
    ensure_eq!(
        listed.waits_on,
        Some(flume_core::WaitsOn::Places),
        "whom it waits on"
    );
    Ok(())
}

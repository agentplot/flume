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
            name: "base places: S112, a base place whose kind names no fix: its failed submission starts nothing, launches no fix and is never forecast, and it waits on its caller with the merge queue's words, delivered once; a fix its caller launches on its branch with them lands there, and the retry lands the base place",
            cites: &["S112", "112", "203", "204", "43", "36"],
            run: s112_a_failed_base_place_takes_a_fix,
        },
        Scenario {
            name: "base places: a base place whose kind names no fix: its conflicting submission starts no stage and launches no fix, and it waits on its caller with words naming the files and the base's revision it was tested against",
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
        Scenario {
            name: "base places: a base place whose kind names mend the kind of its fixes is launched, holding repo/mend@1 as its launch resolved it from the repository's declaration",
            cites: &["21", "23", "154", "203"],
            run: a_base_place_whose_kind_names_mend,
        },
        Scenario {
            name: "base places: a base place's launch whose kind of its fixes resolves to no kind is refused naming it, the merge queue opening nothing and the store holding nothing",
            cites: &["4", "8", "154", "203"],
            run: a_fix_kind_that_resolves_to_nothing,
        },
        Scenario {
            name: "base places: a base place's launch whose kind names itself, a kind with no stages, the kind of its fixes is refused, nothing made",
            cites: &["4", "23", "203"],
            run: a_fix_kind_with_no_stages,
        },
        Scenario {
            name: "base places: a settle of every place forecasts no base place while a change works on its branch, and the base place waits on its places",
            cites: &["203", "210"],
            run: no_forecast_while_a_change_works,
        },
        Scenario {
            name: "base places: a settle forecasts no base place whose submission is open",
            cites: &["111", "203", "210"],
            run: no_forecast_while_submitted,
        },
        Scenario {
            name: "base places: a settle forecasts no base place whose kind names no fix",
            cites: &["203"],
            run: no_forecast_for_a_kind_naming_no_fix,
        },
        Scenario {
            name: "base places: three settles of a base place waiting on its caller forecast it once each, and it holds the forecast the first read, clear at main's revision, the two after writing none",
            cites: &["46", "203", "210"],
            run: forecast_while_it_waits_on_its_caller,
        },
        Scenario {
            name: "base places: a forecast that could not be read launches nothing and fails no settle, each settle asking again, and the base place holds it unread with the merge queue's words",
            cites: &["17", "203", "210"],
            run: a_forecast_that_could_not_be_read,
        },
        Scenario {
            name: "base places: a base place's failed submission, its kind naming mend, is delivered once to ops and launches mend--spring-1 inside the settle that reads it, its brief asking it to make the branch pass main's checks over the merge queue's words; no agent starts in the base place",
            cites: &["1", "43", "112", "203", "I6"],
            run: a_failed_submission_launches_mend,
        },
        Scenario {
            name: "base places: a base place's conflicting submission, its kind naming mend, launches mend--spring-1, its brief asking it to merge main's revision tested against in, resolving README.md, over the merge queue's words naming both",
            cites: &["112", "203"],
            run: a_conflicting_submission_launches_mend,
        },
        Scenario {
            name: "base places: a forecast in conflict only with an entry queued ahead launches nothing, and once the entry lands and the forecast conflicts with main, the next settle launches mend--spring-1",
            cites: &["203", "210", "46"],
            run: a_conflict_with_an_entry_queued_ahead,
        },
        Scenario {
            name: "base places: a forecast conflict of a base place waiting on its caller in east at priority 4 launches mend--spring-1 on its branch inside a settle of every place, in east at priority 4, delivering to ops from its base place, holding its base place, its number and the forecast as its cause",
            cites: &["9", "15", "43", "70", "143", "147", "202", "203", "S120"],
            run: a_forecast_conflict_launches_mend,
        },
        Scenario {
            name: "base places: a forecast conflict's fix is briefed to merge main at the revision forecast into the branch, resolving the files contested, before anything else, over the merge queue's words",
            cites: &["112", "203"],
            run: the_brief_of_a_forecast_conflicts_fix,
        },
        Scenario {
            name: "base places: S120, a base place whose changes landed is forecast in conflict at the host's watch once main lands a change to README.md, its fix launched for it with the merge queue's words, landed on its branch, the next forecast clear, and the caller's retry lands the base place",
            cites: &["S120", "23", "43", "130", "203", "208", "210"],
            run: s120_a_base_place_forecast_in_conflict_launches_its_fix,
        },
        Scenario {
            name: "base places: no forecast is asked while a base place's fix is live; the settle after it lands asks one, the base place waits on its caller once it is clear, and the caller's retry submits its branch by name, which lands",
            cites: &["111", "203", "S120"],
            run: the_fix_lands_and_the_forecast_is_clear,
        },
        Scenario {
            name: "base places: two settles of every place side by side reading one forecast conflict launch one fix, and the base place holds one",
            cites: &["17", "75", "203"],
            run: two_settles_at_once,
        },
        Scenario {
            name: "base places: a fix written onto its base place whose launching command died before it was made is launched by the next settle, the base place holding one fix",
            cites: &["17", "74", "203"],
            run: a_launch_that_died_between_its_writing_and_its_making,
        },
        Scenario {
            name: "base places: a dropped fix leaves its base place waiting on its caller with words naming it dropped, and three settles of the same forecast launch nothing more; once main moves and the forecast conflicts there, the next settle launches mend--spring-2",
            cites: &["63", "75", "130", "203"],
            run: a_fix_dropped,
        },
        Scenario {
            name: "base places: a forecast conflict read once its kind's bound of 1 fix is spent launches nothing, and the base place waits on its caller with the merge queue's words and the bound spent, stalled after six minutes and delivered to ops once",
            cites: &["23", "43", "130", "203"],
            run: the_bound_spent,
        },
        Scenario {
            name: "base places: a base place whose kind gives no bound runs flume's default for a stage's, two fixes, and a third conflict launches nothing",
            cites: &["23", "58", "203"],
            run: the_bound_left_out,
        },
        Scenario {
            name: "base places: a fix whose launch is refused, its branch standing, counts as a fix held refused with the refusal's words, the base place waiting on its caller with them, and the same forecast launches nothing more",
            cites: &["3", "4", "203"],
            run: a_refused_launch,
        },
        Scenario {
            name: "base places: a base place launched with mend at version 1, whose repository then declares mend at version 2, has its fix launched as repo/mend@1, refused naming it, and waits on its caller with the refusal's words",
            cites: &["21", "154", "203"],
            run: a_declaration_changed_after_the_launch,
        },
    ]
}

/// The repository's kind that lands with no stages, so its places are
/// base places, naming no kind for its fixes (203).
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

/// The repository's kind that lands with one code stage delivering
/// commits, the kind a base place's kind names for its fixes.
fn mend() -> Kind {
    kind(
        "mend",
        true,
        vec![stage("code", Some(Deliverable::Commits), "{{brief}}")],
    )
}

/// `train` naming `fix` the kind of its fixes, with `limit` its bound
/// where one is given (23).
fn train_naming(fix: &str, limit: Option<u32>) -> Kind {
    Kind {
        fix: Some(fix.into()),
        fix_limit: limit,
        ..train()
    }
}

/// The launch of `train--spring` on `main` for the recipient `ops`.
fn spring_launch() -> Launch {
    Launch {
        recipients: vec!["ops".into()],
        ..launch("train", "spring", None)
    }
}

/// The base place `train--spring` on `main`, its kind naming `mend` the
/// kind of its fixes with `limit` its bound, the repository declaring
/// both; its recipient `ops` a session that asks nothing (23, 203).
fn spring_mended(world: &World, limit: Option<u32>) -> Result<PlaceId, String> {
    world.terminal.add_session("ops", AgentState::Waiting);
    world
        .kinds
        .set_kinds(REPOSITORY, vec![train_naming("mend", limit), mend()]);
    launched(&world.engine.launch(&spring_launch()))?;
    Ok(World::id("train", "spring"))
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
    ensure!(
        world.store.places() == vec![spring.clone()],
        "a place was launched for the failure: {:?}",
        world.store.places()
    );
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
    ensure_eq!(forecasts(&world), Vec::<String>::new(), "the forecasts");
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
    ensure_eq!(forecasts(&world), Vec::<String>::new(), "the forecasts");
    ensure!(
        world.store.places() == vec![spring.clone()],
        "a place was launched for the conflict: {:?}",
        world.store.places()
    );
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

fn a_base_place_whose_kind_names_mend(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    let held = world.held(&spring).ok_or("no base place")?;
    let fixes_as = held.place.fixes_as.ok_or("no kind of its fixes held")?;
    ensure_eq!(
        fixes_as.identity.spelled,
        "repo/mend@1",
        "the kind its fixes run as"
    );
    ensure_eq!(fixes_as.from.source, "repo", "where it came from");
    ensure!(
        fixes_as.from.declaration_file.is_some() && fixes_as.from.revision.is_some(),
        "its declaration's file and revision: {:?}",
        fixes_as.from
    );
    ensure_eq!(held.phase, Phase::Live, "the base place");
    Ok(())
}

fn a_fix_kind_that_resolves_to_nothing(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world
        .kinds
        .set_kinds(REPOSITORY, vec![train_naming("mend", None)]);
    let reason = refused(&world.engine.launch(&spring_launch()))?;
    ensure!(
        reason.contains("names mend the kind of its fixes") && reason.contains("resolves to no"),
        "the refusal names neither the kind of its fixes nor that it resolves to none: {reason}"
    );
    ensure_eq!(world.queue.log.count("open"), 0, "the merge queue's opens");
    ensure!(
        world.store.places().is_empty(),
        "documents stand: {:?}",
        world.store.places()
    );
    Ok(())
}

fn a_fix_kind_with_no_stages(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world
        .kinds
        .set_kinds(REPOSITORY, vec![train_naming("train", None)]);
    let reason = refused(&world.engine.launch(&spring_launch()))?;
    ensure!(
        reason.contains("names train the kind of its fixes")
            && reason.contains("a kind with no stages runs no fix"),
        "the refusal names neither the kind of its fixes nor why: {reason}"
    );
    ensure_eq!(world.queue.log.count("open"), 0, "the merge queue's opens");
    ensure!(world.store.places().is_empty(), "documents stand");
    ensure!(
        world.git.branch_head(REPOSITORY, "train/spring").is_none(),
        "the branch was made"
    );
    Ok(())
}

/// The forecasts the merge queue was asked, as its log names them.
fn forecasts(world: &World) -> Vec<String> {
    world.queue.log.of("forecast")
}

fn no_forecast_while_a_change_works(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    change_working(&world, "one", "train/spring")?;
    world.queue.log.clear();
    world.engine().settle(None);
    ensure_eq!(forecasts(&world), Vec::<String>::new(), "the forecasts");
    ensure_eq!(
        world.status(&spring)?.waits_on,
        Some(flume_core::WaitsOn::Places),
        "whom it waits on"
    );
    Ok(())
}

fn no_forecast_while_submitted(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    retry_submission(&world, &spring)?;
    world.queue.log.clear();
    world.engine().settle(None);
    ensure_eq!(forecasts(&world), Vec::<String>::new(), "the forecasts");
    Ok(())
}

fn no_forecast_for_a_kind_naming_no_fix(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring(&world)?;
    for _ in 0..3 {
        world.engine().settle(None);
    }
    ensure_eq!(forecasts(&world), Vec::<String>::new(), "the forecasts");
    ensure_eq!(
        world.status(&spring)?.waits_on,
        Some(flume_core::WaitsOn::Caller),
        "whom it waits on"
    );
    Ok(())
}

fn forecast_while_it_waits_on_its_caller(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    world.queue.log.clear();
    let first = world.host.now();
    for _ in 0..3 {
        world.engine().settle(None);
        world.host.advance(60);
    }
    ensure_eq!(
        forecasts(&world),
        vec!["forecast train/spring onto main".to_string(); 3],
        "the forecasts"
    );
    let main = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    let head = world
        .git
        .branch_head(REPOSITORY, "train/spring")
        .ok_or("no train/spring")?;
    ensure_eq!(
        world.held(&spring).and_then(|held| held.forecast),
        Some(ForecastHeld {
            at: first,
            forecast: Forecast::Clear { head, onto: main },
        }),
        "the forecast held"
    );
    Ok(())
}

fn a_forecast_that_could_not_be_read(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    let torn = Forecast::CouldNotRead {
        reason: "the reader's document is torn".into(),
    };
    world
        .queue
        .answer_forecast(REPOSITORY, "train/spring", torn.clone());
    world.queue.log.clear();
    for _ in 0..2 {
        match world.engine().settle(None) {
            flume_core::Answer::Settled { .. } => {}
            other => return Err(format!("the settle answered {other:?}")),
        }
    }
    ensure_eq!(forecasts(&world).len(), 2, "the forecasts asked");
    ensure_eq!(
        world.store.places(),
        vec![spring.clone()],
        "the places standing"
    );
    ensure_eq!(
        world
            .held(&spring)
            .and_then(|held| held.forecast)
            .map(|held| held.forecast),
        Some(torn),
        "the forecast held"
    );
    Ok(())
}

/// The base place `train--spring`'s fix `number`, a place of `mend`.
fn mend_fix(number: u32) -> PlaceId {
    World::id("mend", &format!("spring-{number}"))
}

/// The merge queue's words for a conflict of `train/spring` on
/// `README.md` with `with`, at main's revision `onto`, in the lines a
/// conflicting submission's words carry (112, 210).
fn conflict_words(with: &str, onto: &Revision) -> String {
    format!(
        "would conflict: README.md with {with}\n\nThe files contested: README.md.\nThe base's revision the merge queue tested against: {onto}."
    )
}

/// The fake merge queue forecasting `train/spring` at its head in conflict
/// on `README.md` with main's revision, meeting `with` queued ahead, none
/// where it meets main itself: the head and the revision.
fn forecast_conflict(world: &World, with: &[&str]) -> Result<(Revision, Revision), String> {
    let head = world
        .git
        .branch_head(REPOSITORY, "train/spring")
        .ok_or("no train/spring")?;
    let onto = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    let met = with.first().copied().unwrap_or("main");
    world.queue.answer_forecast(
        REPOSITORY,
        "train/spring",
        Forecast::Conflict {
            head: head.clone(),
            onto: onto.clone(),
            files: vec!["README.md".into()],
            with: with.iter().map(|entry| entry.to_string()).collect(),
            words: conflict_words(met, &onto),
        },
    );
    Ok((head, onto))
}

/// The fake merge queue forecasting `train/spring` clear onto main.
fn forecast_clear(world: &World) -> Result<(), String> {
    let head = world
        .git
        .branch_head(REPOSITORY, "train/spring")
        .ok_or("no train/spring")?;
    let onto = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    world
        .queue
        .answer_forecast(REPOSITORY, "train/spring", Forecast::Clear { head, onto });
    Ok(())
}

/// The fix `number` of `train--spring` run through: its agent commits on
/// its branch and delivers, and it lands on `train/spring` and is tidied.
fn mend_lands(world: &World, number: u32) -> Result<(), String> {
    let fix = mend_fix(number);
    let name = format!("spring-{number}");
    world.settle(&fix);
    world.commit(&format!("mend/{name}"), &["README.md"]);
    world
        .agent
        .set_own_work(&World::identity("mend", &name, "code", 1), Reading::No);
    world.settle(&fix);
    ensure!(
        world.held(&fix).is_none(),
        "mend--{name} did not land and go: {:?}",
        world.held(&fix).map(|held| held.phase)
    );
    Ok(())
}

/// The brief a fix was launched with.
fn brief_of(world: &World, fix: &PlaceId) -> Result<String, String> {
    world
        .held(fix)
        .ok_or_else(|| format!("{fix} was not launched"))?
        .place
        .brief
        .ok_or_else(|| format!("{fix} has no brief"))
}

fn a_failed_submission_launches_mend(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    let words = "the check test failed on main: 2 of 46";
    world
        .queue
        .plan_submits(vec![flume_fakes::Plan::Fail(words.into())]);
    retry_submission(&world, &spring)?;
    world.engine().settle(None);
    let brief = brief_of(&world, &mend_fix(1))?;
    ensure_eq!(
        brief,
        format!(
            "Make train/spring pass the checks of main that failed on its submission.\n\n{words}"
        ),
        "the fix's brief"
    );
    let failures = world
        .terminal
        .notices("ops")
        .into_iter()
        .filter(|text| text.contains("submission failed"))
        .count();
    ensure_eq!(failures, 1, "the notices of the failure to ops");
    ensure!(
        world
            .terminal
            .log
            .of("open")
            .iter()
            .all(|call| !call.contains("train--spring")),
        "an agent opened in the base place"
    );
    ensure_eq!(
        world.status(&spring)?.waits_on,
        Some(flume_core::WaitsOn::Places),
        "whom the base place waits on"
    );
    Ok(())
}

fn a_conflicting_submission_launches_mend(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    world.commit("main", &["README.md"]);
    let tested = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    world.queue.plan_submits(vec![flume_fakes::Plan::Conflict(
        vec!["README.md".into()],
        "README.md conflicts with main".into(),
    )]);
    retry_submission(&world, &spring)?;
    world.engine().settle(None);
    let brief = brief_of(&world, &mend_fix(1))?;
    let (line, words) = brief.split_once("\n\n").ok_or("a brief of one line")?;
    ensure_eq!(
        line,
        format!("Take main at {tested} into train/spring, whose submission conflicted with it on README.md: merge that revision in and commit it, resolving the files contested, before anything else."),
        "the brief's first line"
    );
    ensure!(
        words.contains("README.md") && words.contains(&tested.to_string()),
        "the merge queue's words name neither README.md nor {tested}: {words}"
    );
    Ok(())
}

fn a_conflict_with_an_entry_queued_ahead(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    forecast_conflict(&world, &["#7"])?;
    world.engine().settle(None);
    ensure!(
        world.held(&mend_fix(1)).is_none(),
        "a fix was launched for a conflict with an entry ahead"
    );
    let held = world.held(&spring).ok_or("no base place")?;
    ensure!(
        matches!(
            held.forecast.map(|held| held.forecast),
            Some(Forecast::Conflict { with, .. }) if with == vec!["#7".to_string()]
        ),
        "the forecast held is no conflict with #7"
    );
    ensure!(held.fixes.is_empty(), "fixes: {:?}", held.fixes);
    world.commit("main", &["README.md"]);
    forecast_conflict(&world, &[])?;
    world.engine().settle(None);
    ensure!(
        world.held(&mend_fix(1)).is_some(),
        "mend--spring-1 was not launched"
    );
    Ok(())
}

fn a_forecast_conflict_launches_mend(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("east", Some(Stream::default()));
    world.terminal.add_session("ops", AgentState::Waiting);
    world
        .kinds
        .set_kinds(REPOSITORY, vec![train_naming("mend", None), mend()]);
    launched(&world.engine.launch(&Launch {
        priority: 4,
        stream: Some("east".into()),
        ..spring_launch()
    }))?;
    let spring = World::id("train", "spring");
    world.commit("main", &["README.md"]);
    let (head, onto) = forecast_conflict(&world, &[])?;
    world.engine().settle(None);
    let fix = world
        .held(&mend_fix(1))
        .ok_or("mend--spring-1 was not launched")?;
    let place = &fix.place;
    ensure_eq!(
        (place.base.as_str(), place.branch.as_str()),
        ("train/spring", "mend/spring-1"),
        "the fix's base and branch"
    );
    ensure_eq!(place.stream.name, "east", "the fix's stream");
    ensure_eq!(fix.priority, 4, "the fix's priority");
    ensure_eq!(
        (place.recipients.clone(), place.recipients_from.clone()),
        (vec!["ops".to_string()], Some(RecipientsFrom::BasePlace)),
        "the fix's recipients"
    );
    ensure_eq!(
        place.fix_of,
        Some(FixOf {
            place: spring.clone(),
            number: 1,
            cause: FixCause::Forecast { head, onto },
        }),
        "the base place the fix holds"
    );
    let base = world.held(&spring).ok_or("no base place")?;
    ensure!(
        matches!(
            base.fixes.as_slice(),
            [BaseFix {
                number: 1,
                ended: FixEnded::Live,
                ..
            }]
        ),
        "the base place's fixes: {:?}",
        base.fixes
    );
    ensure!(
        world.engine().list(false).refusal().is_none(),
        "the list refused"
    );
    Ok(())
}

fn the_brief_of_a_forecast_conflicts_fix(_: &()) -> Outcome {
    let world = World::new();
    spring_mended(&world, None)?;
    world.commit("main", &["README.md"]);
    let (_, onto) = forecast_conflict(&world, &[])?;
    world.engine().settle(None);
    let brief = brief_of(&world, &mend_fix(1))?;
    ensure_eq!(
        brief,
        format!(
            "Take main at {onto} into train/spring, which the merge queue forecast to conflict with it on README.md: merge that revision in and commit it, resolving the files contested, before anything else.\n\n{}",
            conflict_words("main", &onto)
        ),
        "the fix's brief"
    );
    Ok(())
}

fn s120_a_base_place_forecast_in_conflict_launches_its_fix(_: &()) -> Outcome {
    let world = World::new();
    let watch = || world.engine().settle_every(30);
    let spring = spring_mended(&world, Some(2))?;
    for name in ["one", "two"] {
        change_working(&world, name, "train/spring")?;
        world.commit(&format!("change/{name}"), &["README.md"]);
        change_lands(&world, name)?;
    }
    forecast_clear(&world)?;
    watch();
    ensure_eq!(
        world.status(&spring)?.waits_on,
        Some(flume_core::WaitsOn::Caller),
        "whom it waits on once its changes landed"
    );
    world.commit("main", &["README.md"]);
    let (_, onto) = forecast_conflict(&world, &[])?;
    watch();
    let fix = mend_fix(1);
    let held = world
        .held(&fix)
        .ok_or("its fix was not launched at the watch")?;
    ensure!(
        held.place
            .brief
            .as_deref()
            .is_some_and(|brief| brief.ends_with(&conflict_words("main", &onto))),
        "the fix's brief carries no words of the merge queue's: {:?}",
        held.place.brief
    );
    ensure_eq!(
        held.place
            .fix_of
            .as_ref()
            .map(|of| (of.place.clone(), of.number)),
        Some((spring.clone(), 1)),
        "the base place the fix names"
    );
    ensure_eq!(
        held.place.stream.name,
        world
            .held(&spring)
            .ok_or("no base place")?
            .place
            .stream
            .name,
        "the fix's stream"
    );
    mend_lands(&world, 1)?;
    ensure!(
        world
            .terminal
            .notices("ops")
            .iter()
            .any(|text| text.starts_with("flume: mend--spring-1 ")),
        "ops was delivered nothing of the fix"
    );
    forecast_clear(&world)?;
    watch();
    let status = world.status(&spring)?;
    ensure_eq!(
        status.waits_on,
        Some(flume_core::WaitsOn::Caller),
        "whom it waits on once its fix landed"
    );
    ensure!(
        matches!(
            world.held(&spring).ok_or("no base place")?.fixes.as_slice(),
            [BaseFix {
                number: 1,
                ended: FixEnded::Landed { .. },
                ..
            }]
        ),
        "the base place's fixes"
    );
    ensure!(
        world
            .queue
            .submissions(REPOSITORY, "train/spring")
            .is_empty(),
        "the base place was submitted by a settle"
    );
    retry_submission(&world, &spring)?;
    watch();
    ensure!(
        world.held(&spring).is_none(),
        "the base place did not land and go"
    );
    Ok(())
}

fn the_fix_lands_and_the_forecast_is_clear(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    world.commit("main", &["README.md"]);
    forecast_conflict(&world, &[])?;
    world.engine().settle(None);
    ensure!(
        world.held(&mend_fix(1)).is_some(),
        "mend--spring-1 was not launched"
    );
    world.queue.log.clear();
    world.engine().settle(None);
    mend_lands(&world, 1)?;
    ensure_eq!(
        forecasts(&world),
        Vec::<String>::new(),
        "the forecasts while the fix was live"
    );
    forecast_clear(&world)?;
    world.engine().settle(None);
    ensure_eq!(
        forecasts(&world),
        vec!["forecast train/spring onto main".to_string()],
        "the forecasts once it landed"
    );
    let status = world.status(&spring)?;
    ensure_eq!(
        status.waiting_on.map(|waiting| (waiting.act, waiting.why)),
        Some((Act::Retry, Waits::Unsubmitted)),
        "what the base place waits on its caller for"
    );
    retry_submission(&world, &spring)?;
    ensure_eq!(
        world.queue.log.of("submit").last().cloned(),
        Some("submit train/spring named".to_string()),
        "the base place's submit"
    );
    world.engine().settle(None);
    ensure!(
        world.held(&spring).is_none(),
        "the base place did not land and go"
    );
    Ok(())
}

fn two_settles_at_once(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    world.commit("main", &["README.md"]);
    forecast_conflict(&world, &[])?;
    let other = world.engine();
    world.store.before_next_write(move || {
        other.settle(None);
    });
    world.engine().settle(None);
    let fixes: Vec<HeldState> = world
        .store
        .read_every()
        .into_iter()
        .map(|read| read.state)
        .filter(|held| held.place.fix_of.is_some())
        .collect();
    ensure_eq!(fixes.len(), 1, "the fixes launched");
    ensure_eq!(fixes[0].place.id, mend_fix(1), "the fix launched");
    ensure_eq!(
        world.held(&spring).ok_or("no base place")?.fixes.len(),
        1,
        "the fixes the base place holds"
    );
    Ok(())
}

fn a_launch_that_died_between_its_writing_and_its_making(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    world.commit("main", &["README.md"]);
    let (head, onto) = forecast_conflict(&world, &[])?;
    let forecast = world.queue.forecast(REPOSITORY, "train/spring", "main");
    let now = world.host.now();
    world.engine.update(&spring, |held| {
        held.forecast = Some(ForecastHeld {
            at: now,
            forecast: forecast.clone(),
        });
        held.fixes.push(BaseFix {
            number: 1,
            name: "spring-1".into(),
            place: mend_fix(1),
            cause: FixCause::Forecast {
                head: head.clone(),
                onto: onto.clone(),
            },
            launched: now,
            ended: FixEnded::Live,
        });
        flume_core::Step::Write(())
    });
    ensure!(world.held(&mend_fix(1)).is_none(), "the fix stands");
    world.engine().settle(None);
    let brief = brief_of(&world, &mend_fix(1))?;
    ensure!(
        brief.ends_with(&conflict_words("main", &onto)),
        "the fix's brief carries no words of the merge queue's: {brief}"
    );
    let held = world.held(&spring).ok_or("no base place")?;
    ensure_eq!(held.fixes.len(), 1, "the fixes the base place holds");
    ensure_eq!(held.fixes[0].ended, FixEnded::Live, "the fix's end");
    Ok(())
}

fn a_fix_dropped(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    world.commit("main", &["README.md"]);
    forecast_conflict(&world, &[])?;
    world.engine().settle(None);
    let one = mend_fix(1);
    ensure!(
        world.held(&one).is_some(),
        "mend--spring-1 was not launched"
    );
    if let Some(reason) = world.engine().drop_place(&one, true).refusal() {
        return Err(format!("the drop of mend--spring-1 was refused: {reason}"));
    }
    ensure!(world.held(&one).is_none(), "the dropped fix stands");
    for _ in 0..3 {
        world.engine().settle(None);
    }
    ensure!(
        world.held(&mend_fix(2)).is_none(),
        "a second fix was launched for the same forecast"
    );
    let held = world.held(&spring).ok_or("no base place")?;
    ensure!(
        matches!(
            held.fixes.as_slice(),
            [BaseFix {
                number: 1,
                ended: FixEnded::Dropped,
                ..
            }]
        ),
        "the base place's fixes: {:?}",
        held.fixes
    );
    let status = world.status(&spring)?;
    ensure_eq!(
        status.waits_on,
        Some(flume_core::WaitsOn::Caller),
        "whom the base place waits on"
    );
    let words = status
        .waiting_on
        .and_then(|waiting| waiting.words)
        .unwrap_or_default();
    ensure!(
        words.contains("mend--spring-1") && words.contains("dropped"),
        "its words name no dropped mend--spring-1: {words}"
    );
    world.commit("main", &["README.md"]);
    forecast_conflict(&world, &[])?;
    world.engine().settle(None);
    ensure!(
        world.held(&mend_fix(2)).is_some(),
        "mend--spring-2 was not launched once main moved"
    );
    Ok(())
}

/// The words a base place waits on its caller with, none where it waits
/// on its caller with none or on nobody.
fn waiting_words(world: &World, place: &PlaceId) -> Result<String, String> {
    let status = world.status(place)?;
    ensure_eq!(
        status.waits_on,
        Some(flume_core::WaitsOn::Caller),
        "whom the base place waits on"
    );
    Ok(status
        .waiting_on
        .and_then(|waiting| waiting.words)
        .unwrap_or_default())
}

/// A conflict of `train/spring` with main forecast once main moves with
/// a change to `README.md`, read by a settle of every place.
fn main_moves_against(world: &World) -> Result<Revision, String> {
    world.commit("main", &["README.md"]);
    let (_, onto) = forecast_conflict(world, &[])?;
    world.engine().settle(None);
    Ok(onto)
}

fn the_bound_spent(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, Some(1))?;
    main_moves_against(&world)?;
    mend_lands(&world, 1)?;
    let onto = main_moves_against(&world)?;
    ensure!(
        world.held(&mend_fix(2)).is_none(),
        "a fix was launched past the bound"
    );
    let words = waiting_words(&world, &spring)?;
    ensure!(
        words.contains(&conflict_words("main", &onto))
            && words.contains("Its kind's 1 fix was launched, as many as it allows"),
        "its words give neither the merge queue's words nor the bound spent: {words}"
    );
    world.host.advance(6 * 60);
    world.engine().settle(None);
    world.engine().settle(None);
    ensure!(world.status(&spring)?.stalled, "not stalled");
    let stalls = world
        .terminal
        .notices("ops")
        .into_iter()
        .filter(|text| text.starts_with("flume: train--spring stalled"))
        .count();
    ensure_eq!(stalls, 1, "the stalls delivered to ops");
    Ok(())
}

fn the_bound_left_out(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    for number in 1..=2 {
        main_moves_against(&world)?;
        ensure!(
            world.held(&mend_fix(number)).is_some(),
            "mend--spring-{number} was not launched"
        );
        mend_lands(&world, number)?;
    }
    main_moves_against(&world)?;
    ensure!(
        world.held(&mend_fix(3)).is_none(),
        "a third fix was launched"
    );
    let words = waiting_words(&world, &spring)?;
    ensure!(
        words.contains("Its kind's 2 fixes were launched, as many as it allows"),
        "its words give no bound spent: {words}"
    );
    Ok(())
}

fn a_refused_launch(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    let main = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    world.git.set_branch(REPOSITORY, "mend/spring-1", &main);
    main_moves_against(&world)?;
    let held = world.held(&spring).ok_or("no base place")?;
    let reason = match held.fixes.as_slice() {
        [BaseFix {
            number: 1,
            ended: FixEnded::Refused { reason },
            ..
        }] => reason.clone(),
        other => return Err(format!("the base place's fixes: {other:?}")),
    };
    ensure!(
        reason.contains("mend/spring-1"),
        "the refusal names no branch: {reason}"
    );
    let words = waiting_words(&world, &spring)?;
    ensure!(
        words.contains("The fix 1, repo/mend@1, was not launched") && words.contains(&reason),
        "its words give no refusal: {words}"
    );
    world.queue.log.clear();
    world.engine().settle(None);
    ensure_eq!(forecasts(&world).len(), 1, "the forecasts asked again");
    ensure_eq!(
        world.held(&spring).ok_or("no base place")?.fixes.len(),
        1,
        "the fixes the base place holds"
    );
    ensure!(
        world.held(&mend_fix(1)).is_none() && world.held(&mend_fix(2)).is_none(),
        "a fix was launched"
    );
    Ok(())
}

fn a_declaration_changed_after_the_launch(_: &()) -> Outcome {
    let world = World::new();
    let spring = spring_mended(&world, None)?;
    world.kinds.set_kinds(
        REPOSITORY,
        vec![
            train_naming("mend", None),
            Kind {
                version: 2,
                ..mend()
            },
        ],
    );
    main_moves_against(&world)?;
    let held = world.held(&spring).ok_or("no base place")?;
    let reason = match held.fixes.as_slice() {
        [BaseFix {
            ended: FixEnded::Refused { reason },
            ..
        }] => reason.clone(),
        other => return Err(format!("the base place's fixes: {other:?}")),
    };
    ensure!(
        reason.contains("repo/mend@1"),
        "the refusal names no repo/mend@1: {reason}"
    );
    let words = waiting_words(&world, &spring)?;
    ensure!(
        words.contains(&reason),
        "its words give no refusal: {words}"
    );
    Ok(())
}

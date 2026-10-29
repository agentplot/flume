//! Tidying (16, 62 to 68, 137, 138): a place whose work is over removed
//! through its repository's teardown, or kept naming what it holds; a
//! branch kept while it holds work its base lacks; what the terminal
//! made for a place closed with it, and for its base once nothing live
//! is in it, never what a person put there.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, Retry, Tidied};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "tidy: a place whose work is not over is named so and kept",
            cites: &["64", "63"],
            run: not_over,
        },
        Scenario {
            name: "tidy: an untracked file and a failed teardown each keep their place, named",
            cites: &["S10", "65", "66"],
            run: s10_kept_places,
        },
        Scenario {
            name: "tidy: a done round removes its place and folder, keeps its branch, and never submits",
            cites: &["S18", "123", "114", "67", "24", "25", "I7"],
            run: s18_a_done_round,
        },
        Scenario {
            name: "tidy: nothing is removed while the merge queue cannot be read",
            cites: &["68", "99"],
            run: the_merge_queue_unreadable,
        },
        Scenario {
            name: "tidy: a teardown the fake place refuses keeps the place, after its one remove",
            cites: &["66", "81"],
            run: a_scripted_refusal,
        },
        Scenario {
            name: "tidy: the last place on a base tidied closes its tab and the base's workspace",
            cites: &["S47", "137", "138", "64"],
            run: s47_the_last_place_on_a_base,
        },
        Scenario {
            name: "tidy: a person's pane in the base's workspace keeps it open, named in status, its place's tab closed",
            cites: &["S48", "137", "138", "56"],
            run: s48_a_persons_pane,
        },
        Scenario {
            name: "tidy: a place whose agent works is kept until the agent stops",
            cites: &["64", "16", "31"],
            run: an_agent_still_working,
        },
    ]
}

fn tidied(answer: Answer) -> Result<Tidied, String> {
    match answer {
        Answer::Tidied { tidy } => Ok(tidy),
        other => Err(format!("the tidy answered {other:?}")),
    }
}

fn not_over(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let tidy = tidied(world.engine.tidy(Some(&id)))?;
    ensure_eq!(tidy.not_over, vec![id.clone()], "the places named not over");
    ensure_eq!(world.place.log.count("remove"), 0, "removes");
    Ok(())
}

fn s10_kept_places(_: &()) -> Outcome {
    let world = World::new();
    let one = world.begun("notes", Some(Deliverable::Commits), "one")?;
    launched(&world.engine.launch(&launch("notes", "two", Some("x"))))?;
    let two = World::id("notes", "two");
    world.place.set_holds(
        &one.0,
        Holds {
            untracked: vec!["scratch.txt".into()],
            ..Holds::default()
        },
    );
    world
        .place
        .fail_teardown(&two.0, "pre-remove", "the dev site would not stop");
    world.engine.done(&one, true);
    world.engine.done(&two, true);
    let tidy = tidied(world.engine.tidy(None))?;
    ensure_eq!(tidy.kept.len(), 2, "places kept");
    let first = world.status(&one)?;
    ensure!(
        matches!(&first.phase, Phase::Over { kept: Some(Kept::Holds { holds }), .. } if holds.untracked == vec!["scratch.txt".to_string()]),
        "status names not the file: {:?}",
        first.phase
    );
    let second = world.status(&two)?;
    ensure!(
        matches!(&second.phase, Phase::Over { kept: Some(Kept::Teardown { step, .. }), .. } if step == "pre-remove"),
        "status names not the step: {:?}",
        second.phase
    );
    world.place.set_holds(&one.0, Holds::default());
    world.place.mend_teardown(&two.0);
    let tidy = tidied(world.engine.tidy(None))?;
    ensure_eq!(tidy.removed.len(), 2, "places removed once mended");
    Ok(())
}

fn s18_a_done_round(_: &()) -> Outcome {
    let world = World::new();
    let mut round = stage(
        "round",
        Some(Deliverable::Report {
            path: "{{state}}/conclusion.md".into(),
        }),
        "Elaborate {{brief}}; write {{report}}.{{#words}} The operator says: {{words}}{{/words}}",
    );
    round.agents = vec![Settings::default()];
    let mut elaboration = kind("elaboration", false, vec![round]);
    elaboration.branch_prefix = "sounding".into();
    world
        .kinds
        .set_declaration("elaboration.toml", vec![elaboration]);
    let mut first = launch("elaboration", "cache", Some("a cache for the index"));
    first.declaration = Some("elaboration.toml".into());
    let place = launched(&world.engine.launch(&first))?;
    ensure_eq!(place.declared, Declared::Launch, "the declaration");
    ensure_eq!(place.branch, "sounding/cache", "the branch");
    let id = World::id("elaboration", "cache");
    world.settle(&id);
    let prototype = world.commit("sounding/cache", &["prototype.rs"]);
    let conclusion = format!("{}/conclusion.md", world.store.folder(&id));
    world.host.advance(60);
    world.host.write_file(&conclusion);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "round"),
        Some(StageState::Delivered),
        "the first round"
    );
    let end = world.status(&id)?.stages[0].end.clone().ok_or("no end")?;
    ensure_eq!(end.report, Some(conclusion.clone()), "the report delivered");
    let answer = world.engine.retry(
        &id,
        Retry::Current,
        Some("try the second layout".into()),
        false,
    );
    ensure!(
        answer.refusal().is_none(),
        "the go-on was refused: {answer:?}"
    );
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "round"),
        Some(StageState::Begun),
        "the second round before its conclusion"
    );
    world.host.advance(60);
    world.host.write_file(&conclusion);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "round"),
        Some(StageState::Delivered),
        "the second round"
    );
    let done = world.engine.done(&id, false);
    let tidy = match done {
        Answer::Acted {
            tidy: Some(tidy), ..
        } => tidy,
        other => return Err(format!("the done answered {other:?}")),
    };
    ensure_eq!(tidy.removed, vec![id.clone()], "the places removed");
    ensure!(
        world.held(&id).is_none(),
        "the place's held state and folder are kept"
    );
    ensure_eq!(tidy.branches_kept.len(), 1, "branches kept");
    ensure!(
        tidy.branches_kept[0].commits.contains(&prototype),
        "the prototype commit is not named"
    );
    ensure_eq!(
        (
            world.queue.log.count("submit"),
            world.queue.log.count("read_submission"),
            world.queue.log.count("wait"),
        ),
        (0, 0, 0),
        "submissions made, read or waited on"
    );
    Ok(())
}

fn the_merge_queue_unreadable(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    world
        .queue
        .unreadable("the merge queue's state could not be read");
    world.engine.drop_place(&id, true);
    ensure_eq!(world.place.log.count("remove"), 0, "removes");
    let status = world.status(&id)?;
    ensure!(
        status
            .cannot_read
            .iter()
            .any(|what| what.contains("merge queue")),
        "status names not what it could not read: {:?}",
        status.cannot_read
    );
    world.queue.readable();
    let tidy = tidied(world.engine.tidy(Some(&id)))?;
    ensure_eq!(
        tidy.removed,
        vec![id.clone()],
        "the places removed once readable"
    );
    Ok(())
}

fn an_agent_still_working(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_watch(Watch::Bound { seconds: 30 });
    let id = world.begun("notes", Some(Deliverable::Commits), "one")?;
    let work = World::identity("notes", "one", "work", 1);
    world.terminal.set_state(&work, AgentState::Working);
    refused(&world.engine.done(&id, false))?;
    let mut held = world.held(&id).ok_or("no place")?;
    held.phase = Phase::Over {
        over: Over::Done,
        kept: None,
    };
    world.store.put(&id, held);
    let tidy = tidied(world.engine.tidy(Some(&id)))?;
    ensure!(
        matches!(
            tidy.kept.first().map(|kept| &kept.why),
            Some(Kept::AgentLive { .. })
        ),
        "a place with a working agent was not kept: {tidy:?}"
    );
    ensure_eq!(world.place.log.count("remove"), 0, "removes");
    world.terminal.set_state(&work, AgentState::Waiting);
    let tidy = tidied(world.engine.tidy(Some(&id)))?;
    ensure_eq!(
        tidy.removed,
        vec![id.clone()],
        "the place once its agent is quiet"
    );
    ensure_eq!(world.terminal.log.count("close"), 1, "agents ended");
    Ok(())
}

fn a_scripted_refusal(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("notes", Some(Deliverable::Commits), "one")?;
    world
        .place
        .fail_teardown(&id.0, "post-remove", "the database would not drop");
    let answer = world.engine.done(&id, false);
    let tidy = match answer {
        Answer::Acted {
            tidy: Some(tidy), ..
        } => tidy,
        other => return Err(format!("the done answered {other:?}")),
    };
    ensure!(
        matches!(tidy.kept.first().map(|kept| &kept.why), Some(Kept::Teardown { step, .. }) if step == "post-remove"),
        "the place was not kept naming the step: {tidy:?}"
    );
    ensure_eq!(
        world.place.log.of("remove"),
        vec![format!("remove {}", id.0)],
        "the removes"
    );
    Ok(())
}

/// Two places of the repository on `main`, each begun.
fn two_on_main(world: &World) -> Result<(PlaceId, PlaceId), String> {
    let one = world.begun("notes", Some(Deliverable::Commits), "one")?;
    launched(&world.engine.launch(&launch("notes", "two", Some("x"))))?;
    let two = World::id("notes", "two");
    world.settle(&two);
    Ok((one, two))
}

fn s47_the_last_place_on_a_base(_: &()) -> Outcome {
    let world = World::new();
    let (one, two) = two_on_main(&world)?;
    ensure_eq!(
        world.terminal.made(),
        (
            vec![one.0.clone(), two.0.clone()],
            vec!["repository main".to_string()]
        ),
        "the tabs and workspace made"
    );
    world.engine.done(&one, true);
    ensure_eq!(
        world.terminal.made(),
        (vec![two.0.clone()], vec!["repository main".to_string()]),
        "what stands once the first place went"
    );
    let answer = world.engine.done(&two, true);
    let tidy = match answer {
        Answer::Acted {
            tidy: Some(tidy), ..
        } => tidy,
        other => return Err(format!("done answered {other:?}")),
    };
    ensure_eq!(
        tidy.terminal.first().map(|tidied| tidied.closed.clone()),
        Some(vec![
            "the tab notes--two".to_string(),
            "the workspace repository main".to_string()
        ]),
        "what the terminal closed with the last place"
    );
    ensure_eq!(
        world.terminal.made(),
        (vec![], vec![]),
        "what stands once the last place went"
    );
    Ok(())
}

fn s48_a_persons_pane(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("notes", Some(Deliverable::Commits), "one")?;
    let place = world.held(&id).ok_or("no place")?.place.place_ref();
    world.terminal.person_pane(&place, "p9");
    world.engine.drop_place(&id, true);
    let held_by = "the workspace repository main, held by the pane p9".to_string();
    ensure_eq!(
        world.terminal.made(),
        (vec![], vec!["repository main".to_string()]),
        "what stands once the place was dropped"
    );
    match world.engine.status(None) {
        Answer::Status { host, .. } => {
            ensure_eq!(host.terminal_kept, vec![held_by], "status's host part")
        }
        other => return Err(format!("status answered {other:?}")),
    }
    ensure_eq!(world.terminal.log.count("close"), 1, "agents ended");
    Ok(())
}

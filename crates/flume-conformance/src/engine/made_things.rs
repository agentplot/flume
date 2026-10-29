//! What a stage made besides its deliverable (129; record 0007;
//! core.23): `{{made}}` renders the path of `made.jsonl` in the place's
//! own folder for a stage that declares what it makes, and a stage that
//! declares nothing asking for it is refused naming the part (27); a
//! declared thing with no line holds the stage undelivered, the stop
//! check naming it (44, 110); the lines written since the stage's start
//! are carried in its end, status, a wait and the inbox as the run wrote
//! them, flume building, checking or starting nothing for them (43, 56);
//! and they go with the place's folder at the tidy (62, 123).

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, WaitFor};
use flume_fakes::Plan;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "made things: a stage that declares what it makes has {{made}} rendered as the path of made.jsonl in the place's folder",
            cites: &["129", "27", "128"],
            run: the_path_rendered,
        },
        Scenario {
            name: "made things: a stage that declares nothing it makes and asks for {{made}} stands refused naming it, and starts no agent",
            cites: &["129", "27"],
            run: asked_for_and_not_declared,
        },
        Scenario {
            name: "made things: a stage that commits and goes quiet with no line for a thing it declares is not delivered, and the stop check names the thing",
            cites: &["129", "44", "110"],
            run: commits_and_no_line,
        },
        Scenario {
            name: "made things: the line written, the stage is delivered at its head with the line in its end",
            cites: &["129", "43", "44"],
            run: the_line_written,
        },
        Scenario {
            name: "made things: an address and a thing not declared are carried whole in the end, status, a wait and the inbox",
            cites: &["129", "43", "56", "119"],
            run: an_address_carried_whole,
        },
        Scenario {
            name: "made things: a thing that must run is carried as the branch and revision that bring it up, and flume starts nothing for it",
            cites: &["129", "43"],
            run: a_thing_that_must_run,
        },
        Scenario {
            name: "made things: a place whose stage recorded made lines lands and is tidied, and the store keeps no line of it",
            cites: &["129", "62", "123"],
            run: after_the_tidy,
        },
    ]
}

/// A landing kind `spike` whose code stage delivers commits and makes
/// `makes`, its prompt `prompt`.
fn spike(world: &World, makes: &[&str], prompt: &str) {
    let mut code = stage("code", Some(Deliverable::Commits), prompt);
    code.makes = makes.iter().map(|name| (*name).to_string()).collect();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![kind("spike", true, vec![code])]);
}

/// A spike named `name` launched for `recipients` and settled once.
fn spiked(world: &World, name: &str, recipients: &[&str]) -> Result<PlaceId, String> {
    let mut launch = launch("spike", name, Some("try it"));
    launch.recipients = recipients.iter().map(|name| (*name).to_string()).collect();
    launched(&world.engine.launch(&launch))?;
    let id = World::id("spike", name);
    world.settle(&id);
    Ok(id)
}

/// A spike named `name` making `makes`, begun.
fn begun(
    world: &World,
    name: &str,
    makes: &[&str],
    recipients: &[&str],
) -> Result<PlaceId, String> {
    spike(
        world,
        makes,
        "{{brief}}\n\nRecord what you made at {{made}}.",
    );
    let id = spiked(world, name, recipients)?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage"
    );
    Ok(id)
}

fn stop_words(world: &World, identity: &Identity) -> Option<String> {
    match world.engine().stop_check(&identity.to_string()) {
        Answer::StopCheck { words } => words,
        _ => None,
    }
}

/// The code stage's end as held state gives it.
fn code_end(world: &World, id: &PlaceId) -> Result<StageEnd, String> {
    world
        .held(id)
        .and_then(|held| held.stages.into_iter().find(|stage| stage.name == "code"))
        .and_then(|stage| stage.end)
        .ok_or_else(|| "the code stage has no end".into())
}

fn the_path_rendered(_: &()) -> Outcome {
    let world = World::new();
    let id = begun(&world, "one", &["preview"], &[])?;
    let prompt = world
        .prompt(&id, &World::identity("spike", "one", "code", 1))
        .ok_or("no prompt")?;
    let path = format!("{}/made.jsonl", world.store.folder(&id));
    ensure!(
        prompt.contains(&format!("Record what you made at {path}.")),
        "the prompt carries not {path}: {prompt}"
    );
    Ok(())
}

fn asked_for_and_not_declared(_: &()) -> Outcome {
    let world = World::new();
    spike(
        &world,
        &[],
        "{{brief}}\n\nRecord what you made at {{made}}.",
    );
    let id = spiked(&world, "one", &[])?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Refused),
        "the code stage"
    );
    let status = world.status(&id)?;
    ensure_eq!(
        status.stages[0].refused_part.as_deref(),
        Some("made"),
        "the part named"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    Ok(())
}

fn commits_and_no_line(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = begun(&world, "one", &["preview"], &[])?;
    world.commit("spike/one", &["spike.rs"]);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the code stage with no line for preview"
    );
    ensure_eq!(
        world.status(&id)?.delivered,
        Some(false),
        "status's delivered"
    );
    let words = stop_words(&world, &World::identity("spike", "one", "code", 1))
        .ok_or("the agent was not sent back")?;
    ensure!(
        words.contains("preview"),
        "the words name not preview: {words}"
    );
    ensure!(
        words.contains(&format!("{}/made.jsonl", world.store.folder(&id))),
        "the words name not the file: {words}"
    );
    ensure!(
        words.contains("flume needs"),
        "the words name not the way to stop short: {words}"
    );
    Ok(())
}

fn the_line_written(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = begun(&world, "one", &["preview"], &[])?;
    let head = world.commit("spike/one", &["spike.rs"]);
    world.settle(&id);
    let line = format!(
        r#"{{"name":"preview","what":"preview","branch":"spike/one","revision":"{}"}}"#,
        head.0
    );
    world.store.record_made(&id, &line);
    let code = World::identity("spike", "one", "code", 1);
    ensure_eq!(stop_words(&world, &code), None, "the stop check once made");
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "the code stage"
    );
    let end = code_end(&world, &id)?;
    ensure_eq!(end.head, Some(head), "the head delivered at");
    ensure_eq!(end.made, vec![Made::read(&line)], "the made lines");
    Ok(())
}

fn an_address_carried_whole(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    world.store.record_made(
        &World::id("spike", "one"),
        r#"{"name":"spike","what":"spike","address":"https://forge.example/o/r/tree/spike/old"}"#,
    );
    let id = begun(&world, "one", &["spike"], &["ops", "planner"])?;
    let address = "https://forge.example/o/r/tree/spike/x?ref=a%2Fb";
    let lines = [
        format!(r#"{{"name":"spike","what":"spike","address":"{address}","note":"kept"}}"#),
        r#"{"name":"notes","what":"page","address":"https://pages.example/notes#Top"}"#.to_string(),
    ];
    for line in &lines {
        world.store.record_made(&id, line);
    }
    world.commit("spike/one", &["spike.rs"]);
    world.settle(&id);
    let made: Vec<Made> = lines.iter().map(|line| Made::read(line)).collect();
    let Made::Thing(spike) = &made[0] else {
        return Err("the spike's line read as text".into());
    };
    ensure_eq!(spike["address"], address, "the address read");
    ensure_eq!(code_end(&world, &id)?.made, made, "the end's lines");
    let status = world.status(&id)?;
    ensure_eq!(
        status.stages[0].end.as_ref().map(|end| end.made.clone()),
        Some(made.clone()),
        "status's stage"
    );
    let carried = |of: &EndOf| matches!(of, EndOf::Stage { end, .. } if end.made == made);
    ensure!(
        status.ends.iter().any(|end| carried(&end.end.of)),
        "status's ends: {:?}",
        status.ends
    );
    match world
        .engine()
        .wait(&id, &WaitFor::Stage("code".into()), "planner", Some(0))
    {
        Answer::Waited { end: Some(end), .. } if carried(&end.end.of) => {}
        other => return Err(format!("the wait answered {other:?}")),
    }
    match world.engine().inbox("ops") {
        Answer::Inbox { ends, .. } if ends.len() == 1 && carried(&ends[0].end.end.of) => {}
        other => return Err(format!("the inbox answered {other:?}")),
    }
    Ok(())
}

fn a_thing_that_must_run(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = begun(&world, "one", &["preview"], &[])?;
    let head = world.commit("spike/one", &["spike.rs"]);
    world.store.record_made(
        &id,
        &format!(
            r#"{{"name":"preview","what":"preview","branch":"spike/one","revision":"{}"}}"#,
            head.0
        ),
    );
    world.settle(&id);
    let end = code_end(&world, &id)?;
    let [Made::Thing(preview)] = end.made.as_slice() else {
        return Err(format!("the end's lines: {:?}", end.made));
    };
    ensure_eq!(preview["branch"], "spike/one", "the branch");
    ensure_eq!(preview["revision"], head.0.as_str(), "the revision");
    ensure!(
        !preview.contains_key("address"),
        "an address flume made: {preview:?}"
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

fn after_the_tidy(_: &()) -> Outcome {
    let world = World::new();
    let id = begun(&world, "one", &["preview"], &[])?;
    let head = world.commit("spike/one", &["spike.rs"]);
    world.store.record_made(
        &id,
        &format!(
            r#"{{"name":"preview","what":"preview","branch":"spike/one","revision":"{}"}}"#,
            head.0
        ),
    );
    ensure!(world.store.holds_made(&id), "no line recorded");
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place's document");
    ensure!(!world.store.holds_made(&id), "the store keeps the lines");
    ensure!(
        world.store.read_every().iter().all(|read| read
            .state
            .stages
            .iter()
            .all(|stage| stage.end.as_ref().is_none_or(|end| end.made.is_empty()))),
        "a document keeps the lines"
    );
    Ok(())
}

//! Launching (1 to 12, 19 to 23): one place made and its first stage
//! started, held or waiting; or the launch refused whole, naming why.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, WaitingOn, Waits};
use flume_fakes::Log;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "launch: a fix makes one place at the merge queue's base and starts its code stage",
            cites: &["S1", "1", "2", "6", "9", "35"],
            run: a_fix_launched,
        },
        Scenario {
            name: "launch: the first stage on hold starts no agent, and reads on hold with its words",
            cites: &["7", "47", "130", "I6"],
            run: the_first_stage_on_hold,
        },
        Scenario {
            name: "launch: recipients named, from the caller's environment, or the launching session, fixed at the launch",
            cites: &["7", "8", "43"],
            run: recipients_at_the_launch,
        },
        Scenario {
            name: "launch: readiness fails; a change's place waits, a fix's starts with the step in its prompt",
            cites: &["S16", "11", "12"],
            run: readiness_fails,
        },
        Scenario {
            name: "launch: a kind no declaration holds is refused, nothing made",
            cites: &["S15", "4", "19"],
            run: a_kind_no_declaration_holds,
        },
        Scenario {
            name: "launch: a kind with a stage of two agents is refused, nothing made",
            cites: &["S31", "4", "23"],
            run: a_stage_with_two_agents,
        },
        Scenario {
            name: "launch: a brief a later stage asks for and the launch lacks refuses it",
            cites: &["S22", "4", "27"],
            run: a_brief_a_later_stage_asks_for,
        },
        Scenario {
            name: "launch: a repository binding nothing refuses a landing kind and runs one that does not land",
            cites: &["S33", "105", "4"],
            run: no_binding,
        },
        Scenario {
            name: "launch: two launches of one place at once make one place",
            cites: &["S11", "3", "75"],
            run: two_launches_at_once,
        },
        Scenario {
            name: "launch: a place keeps the kind its launch resolved",
            cites: &["S13", "8", "19", "20", "21"],
            run: a_kinds_version_changes,
        },
        Scenario {
            name: "launch: a refused make leaves no held state",
            cites: &["2", "3", "76"],
            run: a_refused_make,
        },
        Scenario {
            name: "launch: a base whose revision cannot be read refuses the launch",
            cites: &["9", "2"],
            run: no_base_revision,
        },
        Scenario {
            name: "launch: a change name the base or a live place holds refuses the launch",
            cites: &["3", "26"],
            run: a_change_name_taken,
        },
    ]
}

fn makes(log: &Log) -> usize {
    log.count("make")
}

fn a_fix_launched(_: &()) -> Outcome {
    let world = World::new();
    let remote = world.git.commit(REPOSITORY, "main", &["remote.rs"]);
    let local = world.git.commit(REPOSITORY, "main-local", &[]);
    world.git.set_branch(REPOSITORY, "main", &local);
    world.queue.set_base(
        REPOSITORY,
        "main",
        BaseRevision::At {
            revision: remote.clone(),
        },
    );
    world.terminal.comes_up(AgentState::Unknown);
    let answer = world
        .engine
        .launch(&launch("fix", "one", Some("bump the pin")));
    let place = launched(&answer)?;
    ensure_eq!(place.branch, "fix/one", "the branch");
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "fix/one"),
        Some(remote),
        "the place's revision"
    );
    ensure_eq!(world.place.log.count("ready"), 1, "readiness runs");
    ensure_eq!(place.stages[0].state, StageState::Started, "the code stage");
    let agent = place.agent.ok_or("no agent in the answer")?;
    ensure_eq!(agent.state, AgentState::Unknown, "the agent's state");
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    ensure_eq!(
        world.terminal.log.count("send"),
        0,
        "prompts sent before the agent waits"
    );
    Ok(())
}

fn the_first_stage_on_hold(_: &()) -> Outcome {
    let world = World::new();
    let mut held = launch("fix", "held", Some("x"));
    let words = "until the release branch is cut";
    held.hold = vec!["code".into()];
    held.hold_words = [("code".to_string(), words.to_string())].into();
    let now = world.host.now();
    let place = launched(&world.engine.launch(&held))?;
    ensure_eq!(place.stages[0].state, StageState::OnHold, "the code stage");
    ensure_eq!(world.place.log.count("ready"), 1, "readiness");
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    ensure_eq!(
        place.hold_words.get("code").map(String::as_str),
        Some(words),
        "the hold's words status gives"
    );
    ensure_eq!(
        place.waiting_on,
        Some(WaitingOn {
            act: Act::Retry,
            stage: Some("code".into()),
            why: Waits::OnHold,
            since: Some(now),
            words: Some(words.into()),
        }),
        "the move the place waits on"
    );
    Ok(())
}

fn recipients_at_the_launch(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_session(Some("pane-7"));
    let mut named = launch("fix", "named", Some("x"));
    named.recipients = vec!["operator".into(), "role:planner".into()];
    launched(&world.engine.launch(&named))?;
    world.host.set_recipient(Some("conductor"));
    launched(&world.engine().launch(&launch("fix", "variable", Some("x"))))?;
    world.host.set_recipient(None);
    launched(&world.engine().launch(&launch("fix", "session", Some("x"))))?;
    world.terminal.set_session(None);
    world.host.set_recipient(Some("someone else"));
    world.settle(&World::id("fix", "named"));
    for (name, recipients) in [
        ("named", vec!["operator", "role:planner"]),
        ("variable", vec!["conductor"]),
        ("session", vec!["pane-7"]),
    ] {
        ensure_eq!(
            world.status(&World::id("fix", name))?.recipients,
            recipients
                .iter()
                .map(|recipient| recipient.to_string())
                .collect::<Vec<_>>(),
            format!("the recipients of {name}")
        );
    }
    Ok(())
}

fn readiness_fails(_: &()) -> Outcome {
    let world = World::new();
    world.place.set_readiness(
        REPOSITORY,
        &[("toolchain", false, "the toolchain did not install")],
    );
    let change = launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    ensure_eq!(
        change.stages[0].state,
        StageState::WaitingOnReadiness,
        "the change's construct stage"
    );
    let failed = change
        .readiness
        .failed()
        .ok_or("status names no failed step")?;
    ensure_eq!(failed.step, "toolchain", "the failed step");
    ensure_eq!(
        failed.words,
        "the toolchain did not install",
        "the step's words"
    );
    let fix = launched(&world.engine.launch(&launch("fix", "mend", Some("mend it"))))?;
    ensure_eq!(
        fix.stages[0].state,
        StageState::Started,
        "the fix's code stage"
    );
    let prompt = world
        .prompt(
            &World::id("fix", "mend"),
            &World::identity("fix", "mend", "code", 1),
        )
        .ok_or("no prompt")?;
    ensure!(
        prompt.contains("toolchain") && prompt.contains("the toolchain did not install"),
        "the fix's prompt lacks the step and its words: {prompt}"
    );
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

fn nothing_made(world: &World) -> Outcome {
    ensure_eq!(makes(&world.place.log), 0, "makes");
    ensure!(world.store.places().is_empty(), "held state was left");
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    ensure_eq!(world.terminal.log.count("send"), 0, "prompts sent");
    Ok(())
}

fn a_kind_no_declaration_holds(_: &()) -> Outcome {
    let world = World::new();
    let reason = refused(&world.engine.launch(&launch("bump", "one", Some("x"))))?;
    ensure!(
        reason.contains("bump"),
        "the refusal names no kind: {reason}"
    );
    nothing_made(&world)
}

fn a_stage_with_two_agents(_: &()) -> Outcome {
    let world = World::new();
    let mut code = stage("code", Some(Deliverable::Commits), "{{brief}}");
    code.agents.push(Settings::default());
    world
        .kinds
        .set_kinds(REPOSITORY, vec![kind("pair", true, vec![code])]);
    let reason = refused(&world.engine.launch(&launch("pair", "one", Some("x"))))?;
    ensure!(
        reason.contains("pair"),
        "the refusal names no kind: {reason}"
    );
    nothing_made(&world)
}

fn a_brief_a_later_stage_asks_for(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "review",
            false,
            vec![
                stage("code", Some(Deliverable::Commits), "code {{name}}"),
                stage("check", Some(Deliverable::Commits), "check {{brief}}"),
            ],
        )],
    );
    let reason = refused(&world.engine.launch(&launch("review", "one", None)))?;
    ensure!(
        reason.contains("check") && reason.contains("brief"),
        "the refusal names not the stage and the brief: {reason}"
    );
    nothing_made(&world)
}

fn no_binding(_: &()) -> Outcome {
    let world = World::new();
    let plain = "/plain";
    world.git.init(plain, "main");
    world.kinds.set_kinds(
        plain,
        vec![kind(
            "notes",
            false,
            vec![stage(
                "write",
                Some(Deliverable::Commits),
                "write {{brief}}",
            )],
        )],
    );
    let mut fix = launch("fix", "one", Some("x"));
    fix.repository = plain.into();
    let reason = refused(&world.engine.launch(&fix))?;
    ensure!(
        reason.contains("fix") && reason.contains("merge queue"),
        "the refusal names not the kind and the merge queue: {reason}"
    );
    nothing_made(&world)?;
    let mut notes = launch("notes", "one", Some("x"));
    notes.repository = plain.into();
    let place = launched(&world.engine.launch(&notes))?;
    ensure_eq!(place.stages[0].state, StageState::Started, "the stage");
    ensure_eq!(
        makes(&world.place.log),
        1,
        "makes through the no-tool place"
    );
    ensure_eq!(world.queue.log.calls().len(), 0, "merge queue calls");
    Ok(())
}

fn two_launches_at_once(_: &()) -> Outcome {
    let world = World::new();
    let other = world.engine();
    world.store.before_next_write(move || {
        let _ = other.launch(&launch("fix", "one", Some("from a send")));
    });
    let reason = refused(
        &world
            .engine
            .launch(&launch("fix", "one", Some("from a key"))),
    )?;
    ensure!(
        reason.contains(&World::id("fix", "one").0),
        "the refusal names not the place: {reason}"
    );
    ensure_eq!(makes(&world.place.log), 1, "makes");
    ensure_eq!(world.store.places().len(), 1, "places held");
    Ok(())
}

fn a_kinds_version_changes(_: &()) -> Outcome {
    let world = World::new();
    let first = world.git.branch_head(REPOSITORY, "main").ok_or("main")?;
    let mut version_one = kind(
        "bump",
        false,
        vec![stage("code", Some(Deliverable::Commits), "{{brief}}")],
    );
    world
        .kinds
        .set_kinds_at(REPOSITORY, &first, vec![version_one.clone()]);
    let one = launched(&world.engine.launch(&launch("bump", "one", Some("x"))))?;
    let second = world.git.commit(REPOSITORY, "main", &[".config/kinds"]);
    version_one.version = 2;
    version_one
        .stages
        .push(stage("check", Some(Deliverable::Commits), "{{brief}}"));
    world
        .kinds
        .set_kinds_at(REPOSITORY, &second, vec![version_one]);
    let two = launched(&world.engine.launch(&launch("bump", "two", Some("x"))))?;
    ensure_eq!((one.version, two.version), (1, 2), "the places' versions");
    ensure_eq!(one.declared, Declared::Repository, "the declaration");
    let held = world
        .held(&World::id("bump", "one"))
        .ok_or("no place one")?;
    ensure_eq!(held.place.kind.version, 1, "the first place's kind");
    ensure_eq!(held.stages.len(), 1, "the first place's stages");
    let status = match world.engine.status(None) {
        flume_core::Answer::Status { places, .. } => places,
        other => return Err(format!("status answered {other:?}")),
    };
    let versions: Vec<(String, u32)> = status
        .iter()
        .map(|place| (place.name.clone(), place.version))
        .collect();
    ensure_eq!(
        versions,
        vec![("one".to_string(), 1), ("two".to_string(), 2)],
        "the versions status names"
    );
    Ok(())
}

fn a_refused_make(_: &()) -> Outcome {
    let world = World::new();
    let location = World::id("fix", "one").0;
    world.place.occupy(&location);
    let reason = refused(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure!(
        reason.contains(&location),
        "the refusal names not the location: {reason}"
    );
    ensure!(world.store.places().is_empty(), "held state was left");
    ensure!(
        world.git.branch_head(REPOSITORY, "fix/one").is_none(),
        "a branch was left"
    );
    Ok(())
}

fn no_base_revision(_: &()) -> Outcome {
    let world = World::new();
    world.queue.set_base(
        REPOSITORY,
        "main",
        BaseRevision::Refused {
            reason: "the fetch of origin failed".into(),
        },
    );
    let reason = refused(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure!(
        reason.contains("fetch"),
        "the refusal names no fetch: {reason}"
    );
    nothing_made(&world)
}

fn a_change_name_taken(_: &()) -> Outcome {
    let world = World::new();
    let base = world.git.branch_head(REPOSITORY, "main").ok_or("main")?;
    world.change.set(
        REPOSITORY,
        &base,
        "filed",
        ChangeFacts {
            archived: true,
            ..ChangeFacts::default()
        },
    );
    let reason = refused(&world.engine.launch(&launch("change", "filed", Some("x"))))?;
    ensure!(
        reason.contains("filed"),
        "the refusal names no change: {reason}"
    );
    launched(&world.engine.launch(&launch("change", "plan", Some("x"))))?;
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "spec",
            false,
            vec![stage(
                "write",
                Some(Deliverable::ChangeWritten),
                "{{brief}}",
            )],
        )],
    );
    let reason = refused(&world.engine.launch(&launch("spec", "plan", Some("x"))))?;
    ensure!(
        reason.contains("plan") && reason.contains(&World::id("change", "plan").0),
        "the refusal names not the live place: {reason}"
    );
    Ok(())
}

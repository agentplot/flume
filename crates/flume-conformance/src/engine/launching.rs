//! Launching (1 to 12, 19 to 23, 154): one place made, made ready at the
//! revision its launch read, and its first stage started, held or
//! waiting, its kind named with its version or with none, or by its
//! identity; or the launch refused whole, naming why.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Retry, WaitingOn, Waits};
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
            name: "launch: a retry of a first stage whose readiness failed readies the place at the revision its launch read, not the base's as it moved on",
            cites: &["12", "20"],
            run: a_retry_readies_at_the_launchs_revision,
        },
        Scenario {
            name: "launch: each step of the merge queue's equipping is named in status as it passed, in order, and the first stage starts",
            cites: &["S89", "12"],
            run: each_step_named_as_it_passed,
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
            name: "launch: a binding naming a place profile flume does not hold is refused, naming the place profiles there are, nothing made",
            cites: &["105", "2", "20"],
            run: a_place_profile_flume_does_not_hold,
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
            name: "launch: a place the place seam gives no location refuses the launch, nothing made",
            cites: &["2", "78"],
            run: no_location,
        },
        Scenario {
            name: "launch: a base whose revision cannot be read refuses the launch",
            cites: &["9", "2"],
            run: no_base_revision,
        },
        Scenario {
            name: "launch: a base no merge queue lands is cut at origin's after flume's fetch, the host's own lagging it, bound to a merge queue or to none, and status says so",
            cites: &["9", "56", "126"],
            run: a_base_no_queue_lands,
        },
        Scenario {
            name: "launch: a base origin lacks, or of a repository with no origin, is cut at the host's own, and status says which",
            cites: &["9", "56", "126"],
            run: a_base_origin_lacks,
        },
        Scenario {
            name: "launch: a fetch of a base from origin that fails refuses the launch, nothing made",
            cites: &["9", "2", "126"],
            run: a_fetch_from_origin_fails,
        },
        Scenario {
            name: "launch: a change name the base or a live place holds refuses the launch",
            cites: &["3", "26"],
            run: a_change_name_taken,
        },
        Scenario {
            name: "launch: a change launched naming no version runs change@2, the newest of flume's own, and status names change@2",
            cites: &["22", "154", "19"],
            run: no_version_named,
        },
        Scenario {
            name: "launch: a change launched as change@1 runs change@1, its code stage one agent for the whole change, and status names change@1",
            cites: &["22", "154", "19"],
            run: change_1_named,
        },
        Scenario {
            name: "launch: a launch naming change@3 is refused naming it and the versions flume holds, 1 and 2, nothing made",
            cites: &["4", "22", "154"],
            run: a_version_flume_does_not_hold,
        },
        Scenario {
            name: "launch: a launch naming change@2 where the repository declares change at version 1 is refused naming the version, nothing made",
            cites: &["4", "19", "154"],
            run: a_version_the_repository_does_not_hold,
        },
        Scenario {
            name: "launch: a change named with no version, in a repository that locks nothing, is named flume/change@2 by status, and one named change@1 flume/change@1",
            cites: &["22", "154", "19"],
            run: the_identity_status_names,
        },
        Scenario {
            name: "launch: a kind of a host's source named whole, locked by the host, runs that source's kind, status naming its identity",
            cites: &["154", "155", "191"],
            run: a_kind_of_a_hosts_source_named_whole,
        },
        Scenario {
            name: "launch: a stream whose multiplexer names a terminal profile flume does not carry is refused, naming the stream, the name and the terminal profiles carried, nothing made",
            cites: &["4", "190"],
            run: a_streams_multiplexer_flume_does_not_carry,
        },
    ]
}

fn the_identity_status_names(_: &()) -> Outcome {
    let world = World::new();
    let two = launched_as(&world, "change", "two")?;
    ensure_eq!(
        two.identity.as_deref(),
        Some("flume/change@2"),
        "the identity of a change named with no version"
    );
    let one = launched_as(&world, "change@1", "one")?;
    ensure_eq!(
        one.identity.as_deref(),
        Some("flume/change@1"),
        "the identity of change@1"
    );
    let held = world.held(&World::id("change", "one")).ok_or("no place")?;
    ensure!(
        !held.place.kind.stages[1].repeats,
        "change@1's code repeats"
    );
    Ok(())
}

fn a_kind_of_a_hosts_source_named_whole(_: &()) -> Outcome {
    use crate::seams::kinds::{entry, hash_of};
    let world = World::new();
    super::kind_sources::holds(&world, "acme", "/kinds/acme", &[("review", 2, '2')]);
    super::kind_sources::host_locks(&world, vec![entry("acme", "review", 2, &hash_of('2'))]);
    let place = launched(
        &world
            .engine
            .launch(&launch("acme/review@2", "one", Some("x"))),
    )?;
    ensure_eq!(
        (place.declared, place.identity.as_deref()),
        (Declared::Host, Some("acme/review@2")),
        "the kind run"
    );
    let held = world.held(&World::id("review", "one")).ok_or("no place")?;
    ensure_eq!(
        held.place.kind,
        super::kind_sources::packaged("review", 2),
        "the place's kind"
    );
    Ok(())
}

/// The change place `name` launched as `kind`, and its status.
fn launched_as(world: &World, kind: &str, name: &str) -> Result<flume_core::PlaceStatus, String> {
    launched(&world.engine.launch(&launch(kind, name, Some("plan it"))))?;
    world.status(&World::id("change", name))
}

fn no_version_named(_: &()) -> Outcome {
    let world = World::new();
    let status = launched_as(&world, "change", "plan")?;
    ensure_eq!(
        (status.kind.as_str(), status.version),
        ("change", 2),
        "the kind status names"
    );
    let held = world.held(&World::id("change", "plan")).ok_or("no place")?;
    ensure!(
        held.place.kind.stages[1].repeats,
        "change@2's code does not repeat"
    );
    Ok(())
}

fn change_1_named(_: &()) -> Outcome {
    let world = World::new();
    let status = launched_as(&world, "change@1", "plan")?;
    ensure_eq!(
        (status.kind.as_str(), status.version),
        ("change", 1),
        "the kind status names"
    );
    let held = world.held(&World::id("change", "plan")).ok_or("no place")?;
    ensure_eq!(held.place.branch, "change/plan", "the place's branch");
    ensure!(
        !held.place.kind.stages[1].repeats,
        "change@1's code repeats"
    );
    Ok(())
}

fn a_version_flume_does_not_hold(_: &()) -> Outcome {
    let world = World::new();
    let reason = refused(&world.engine.launch(&launch("change@3", "plan", Some("x"))))?;
    ensure!(
        reason.contains("change@3") && reason.contains("versions 1, 2"),
        "the refusal names not the version and the versions held: {reason}"
    );
    nothing_made(&world)
}

fn a_version_the_repository_does_not_hold(_: &()) -> Outcome {
    let world = World::new();
    let declared = crate::fake_harness::flumes_own()
        .into_iter()
        .find(|kind| kind.name == "change" && kind.version == 1)
        .ok_or("no change@1")?;
    world.kinds.set_kinds(REPOSITORY, vec![declared]);
    let reason = refused(&world.engine.launch(&launch("change@2", "plan", Some("x"))))?;
    ensure!(
        reason.contains("change@2") && reason.contains("version 1"),
        "the refusal names not the version and the one declared: {reason}"
    );
    nothing_made(&world)
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
    // origin's main, elsewhere again, is not read where the merge queue
    // lands the base (126, C.1).
    let origins = world.git.commit(REPOSITORY, "origin-main", &["origin.rs"]);
    world.git.set_origin(REPOSITORY, "main", &origins);
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
    ensure_eq!(
        place.base_from,
        Some(BaseFrom::MergeQueue),
        "where the base was read"
    );
    ensure_eq!(
        world.history.log.count("origin_branch"),
        0,
        "origin asked where the merge queue lands the base"
    );
    ensure_eq!(world.place.log.count("ready"), 1, "readiness runs");
    ensure_eq!(
        place.stages[0].state,
        StageState::Started,
        "the code stage, its agent come up saying it started"
    );
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
            open_until: None,
            closed: None,
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
        StageState::Begun,
        "the fix's code stage, its prompt sent at the launch"
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

fn a_retry_readies_at_the_launchs_revision(_: &()) -> Outcome {
    let world = World::new();
    let launched_at = world
        .git
        .branch_head(REPOSITORY, "main")
        .ok_or("main has no revision")?;
    world
        .place
        .set_readiness(REPOSITORY, &[("toolchain", false, "offline")]);
    let change = launched(&world.engine.launch(&launch("change", "plan", Some("x"))))?;
    ensure_eq!(
        change.stages[0].state,
        StageState::WaitingOnReadiness,
        "construct at the launch"
    );
    let moved = world.git.commit(REPOSITORY, "main", &["moved.rs"]);
    world
        .place
        .set_readiness(REPOSITORY, &[("toolchain", true, "installed")]);
    let id = World::id("change", "plan");
    if let Some(reason) = world
        .engine
        .retry(&id, Retry::Stage("construct".into()), None, false)
        .refusal()
    {
        return Err(format!("the retry was refused: {reason}"));
    }
    let readied = world.place.log.of("ready");
    ensure_eq!(readied.len(), 2, "readiness runs");
    ensure!(
        readied
            .iter()
            .all(|line| line.ends_with(&format!(" {launched_at}"))),
        "the place was not readied at the launch's revision {launched_at}, main now {moved}: {readied:?}"
    );
    ensure!(
        world
            .stage_state(&id, "construct")
            .is_some_and(StageState::running),
        "construct did not start"
    );
    Ok(())
}

fn each_step_named_as_it_passed(_: &()) -> Outcome {
    let world = World::new();
    let equipped = [
        ("toolchain", true, "activated"),
        ("proto install", true, "installed moon 2.6.1"),
        ("moon --version", true, "moon 2.6.1"),
        ("moon setup", true, "set up"),
    ];
    world.place.set_readiness(REPOSITORY, &equipped);
    let change = launched(&world.engine.launch(&launch("change", "plan", Some("x"))))?;
    let named: Vec<(&str, bool, &str)> = change
        .readiness
        .steps
        .iter()
        .map(|step| (step.step.as_str(), step.passed, step.words.as_str()))
        .collect();
    ensure_eq!(named, equipped.to_vec(), "the steps status names");
    ensure!(
        change.stages[0].state.running(),
        "construct did not start: {:?}",
        change.stages[0].state
    );
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
    ensure_eq!(
        place.stages[0].state,
        StageState::Begun,
        "the stage, its prompt sent at the launch"
    );
    ensure_eq!(
        makes(&world.place.log),
        1,
        "makes through the no-tool place"
    );
    ensure_eq!(world.queue.log.calls().len(), 0, "merge queue calls");
    Ok(())
}

fn a_place_profile_flume_does_not_hold(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_binding(
        REPOSITORY,
        Binding {
            place: Some("groves".into()),
            change: Some("plans".into()),
            merge_queue: Some("queue".into()),
        },
    );
    let reason = refused(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure!(
        reason.contains("groves") && reason.contains("trees"),
        "the refusal names not the profile asked for and the ones there are: {reason}"
    );
    nothing_made(&world)?;
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

fn no_location(_: &()) -> Outcome {
    let world = World::new();
    world
        .place
        .refuse_locations("neither $XDG_DATA_HOME nor $HOME is set");
    let reason = refused(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure!(
        reason.contains("location") && reason.contains("$XDG_DATA_HOME"),
        "the refusal names not why: {reason}"
    );
    ensure_eq!(makes(&world.place.log), 0, "makes");
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

/// The repository's main at a first commit, and origin's two commits past
/// it, which the host's own main lacks (S92).
fn origin_ahead(world: &World) -> Result<(Revision, Revision), String> {
    let local = world.git.branch_head(REPOSITORY, "main").ok_or("main")?;
    world.git.set_branch(REPOSITORY, "origin-main", &local);
    world.git.commit(REPOSITORY, "origin-main", &["one.rs"]);
    let origins = world.git.commit(REPOSITORY, "origin-main", &["two.rs"]);
    world.git.set_origin(REPOSITORY, "main", &origins);
    Ok((local, origins))
}

fn a_base_no_queue_lands(_: &()) -> Outcome {
    // A merge queue bound that lands no such base.
    let world = World::new();
    let (local, origins) = origin_ahead(&world)?;
    world
        .queue
        .set_base(REPOSITORY, "main", BaseRevision::Unlanded);
    let place = launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "fix/one"),
        Some(origins.clone()),
        "the place's revision, under a merge queue that lands no main"
    );
    ensure_eq!(
        place.base_from,
        Some(BaseFrom::Origin),
        "where the base was read"
    );
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "main"),
        Some(local.clone()),
        "the host's own main, moved"
    );
    let status = world.engine.status(Some(&World::id("fix", "one")));
    let flume_core::Answer::Status { places, .. } = status else {
        return Err(format!("status answered {status:?}"));
    };
    ensure_eq!(
        places.first().and_then(|place| place.base_from),
        Some(BaseFrom::Origin),
        "where status says the base was read"
    );
    // No merge queue bound at all, a kind that does not land.
    let world = World::new();
    let (local, origins) = origin_ahead(&world)?;
    world.kinds.set_binding(
        REPOSITORY,
        Binding {
            place: Some("trees".into()),
            change: None,
            merge_queue: None,
        },
    );
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "tweak",
            false,
            vec![stage("code", Some(Deliverable::Commits), "{{brief}}")],
        )],
    );
    let place = launched(&world.engine.launch(&launch("tweak", "one", Some("x"))))?;
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "tweak/one"),
        Some(origins),
        "the place's revision, under no merge queue"
    );
    ensure_eq!(
        place.base_from,
        Some(BaseFrom::Origin),
        "where the base was read"
    );
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "main"),
        Some(local),
        "the host's own main, moved"
    );
    ensure_eq!(world.queue.log.calls().len(), 0, "merge queue calls");
    Ok(())
}

fn a_base_origin_lacks(_: &()) -> Outcome {
    let world = World::new();
    let main = world.git.branch_head(REPOSITORY, "main").ok_or("main")?;
    world.git.set_branch(REPOSITORY, "trial", &main);
    let trial = world.git.commit(REPOSITORY, "trial", &["trial.rs"]);
    world.git.set_origin(REPOSITORY, "main", &main);
    world
        .queue
        .set_base(REPOSITORY, "trial", BaseRevision::Unlanded);
    let place = launched(&world.engine.launch(&Launch {
        base: "trial".into(),
        ..launch("fix", "one", Some("x"))
    }))?;
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "fix/one"),
        Some(trial),
        "the place's revision, origin holding no trial"
    );
    ensure_eq!(
        place.base_from,
        Some(BaseFrom::OriginLacks),
        "where the base was read"
    );
    let world = World::new();
    let main = world.git.branch_head(REPOSITORY, "main").ok_or("main")?;
    world
        .queue
        .set_base(REPOSITORY, "main", BaseRevision::Unlanded);
    let place = launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "fix/one"),
        Some(main),
        "the place's revision, the repository having no origin"
    );
    ensure_eq!(
        place.base_from,
        Some(BaseFrom::NoOrigin),
        "where the base was read"
    );
    Ok(())
}

fn a_fetch_from_origin_fails(_: &()) -> Outcome {
    let world = World::new();
    world
        .queue
        .set_base(REPOSITORY, "main", BaseRevision::Unlanded);
    world.git.origin_fails(
        REPOSITORY,
        "the fetch of main from origin failed: unreachable",
    );
    let reason = refused(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure!(
        reason.contains("fetch") && reason.contains("main"),
        "the refusal names not the fetch and the base: {reason}"
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

fn a_streams_multiplexer_flume_does_not_carry(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_display("trial", Some("rex"), None, None);
    let reason = refused(&world.engine.launch(&Launch {
        stream: Some("trial".into()),
        ..launch("fix", "one", Some("x"))
    }))?;
    ensure!(
        reason.contains("trial")
            && reason.contains("rex")
            && reason.contains("the terminal profiles there are: fake"),
        "the refusal: {reason}"
    );
    nothing_made(&world)
}

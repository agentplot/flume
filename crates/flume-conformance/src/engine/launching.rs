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
            name: "launch: a change launched naming no version runs change@3, the newest of flume's own, its code stage one agent for the whole change, and status names change@3",
            cites: &["22", "154", "19", "S119"],
            run: no_version_named,
        },
        Scenario {
            name: "launch: a change launched as change@2 runs change@2, its code stage repeating per task group, and status names change@2",
            cites: &["22", "154", "19", "140"],
            run: change_2_named,
        },
        Scenario {
            name: "launch: a change launched as change@1 runs change@1, its code stage one agent for the whole change, and status names change@1",
            cites: &["22", "154", "19"],
            run: change_1_named,
        },
        Scenario {
            name: "launch: a launch naming change@4 is refused naming it and the versions flume holds, 1, 2 and 3, nothing made",
            cites: &["4", "22", "154"],
            run: a_version_flume_does_not_hold,
        },
        Scenario {
            name: "launch: a launch naming change@2 where the repository declares change at version 1 is refused naming the version, nothing made",
            cites: &["4", "19", "154"],
            run: a_version_the_repository_does_not_hold,
        },
        Scenario {
            name: "launch: a change named with no version, in a repository that locks nothing, is named flume/change@3 by status, one named change@2 flume/change@2, and one named change@1 flume/change@1",
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
        Scenario {
            name: "launch: a launch from no session naming no recipient takes the answerer behind its stream, east/operator, which status gives as from the stream's answerer declared by acme",
            cites: &["7", "193", "S93"],
            run: a_launch_from_no_session_takes_its_streams_answerer,
        },
        Scenario {
            name: "launch: each stream its own answerer, a like launch in west taking west/operator",
            cites: &["143", "193"],
            run: each_stream_its_own_answerer,
        },
        Scenario {
            name: "launch: a launch in east naming west/planner, another stream's role acme declares, takes it from the launch",
            cites: &["7", "132", "193"],
            run: another_streams_role_named,
        },
        Scenario {
            name: "launch: a launch from a shell into a stream no role stands behind is refused naming the four ways to give a recipient, nothing made",
            cites: &["4", "7", "193"],
            run: a_launch_from_a_shell_into_a_stream_no_role_stands_behind,
        },
        Scenario {
            name: "launch: a launch from no session into a stream two roles stand behind is refused naming both roles and their sources, nothing made",
            cites: &["4", "7", "193"],
            run: two_roles_behind_one_stream,
        },
        Scenario {
            name: "launch: a launch naming an address no source declares, into a stream no role stands behind, is refused naming the address, nothing made",
            cites: &["4", "132", "193"],
            run: an_address_that_wakes_nothing_with_no_answerer,
        },
        Scenario {
            name: "launch: a launch naming an address no source declares, into a stream a role stands behind, is taken, and status says the address wakes nothing",
            cites: &["4", "132", "193"],
            run: an_address_that_wakes_nothing_behind_an_answerer,
        },
        Scenario {
            name: "launch: a launch naming an address a source declares carried, into a stream no role stands behind, is taken",
            cites: &["4", "132", "193"],
            run: a_carried_address,
        },
        Scenario {
            name: "launch: a chore in a repository whose origin carries a user and a token runs in work, the stream the host names for its address, status giving the address acme/acme beside its repository and nothing holding the token",
            cites: &["56", "143"],
            run: the_address_kept_and_given,
        },
        Scenario {
            name: "launch: a place in a repository whose address is acme/acme takes the model and the label the host gives that address's tables at its start",
            cites: &["32", "180"],
            run: its_starts_take_the_addresss_tables,
        },
        Scenario {
            name: "launch: a place held with no address takes its folder's tables at its start, its repository's origin read as acme/acme only after its launch",
            cites: &["32"],
            run: a_place_held_with_no_address,
        },
        Scenario {
            name: "launch: a caller named is kept on the place apart from its recipients, given by status, and left as it was by a hold and a retry after the launch",
            cites: &["7", "8", "56"],
            run: a_caller_named,
        },
        Scenario {
            name: "launch: a caller written empty refuses the launch whole, naming the caller, nothing made",
            cites: &["4", "7"],
            run: an_empty_caller,
        },
        Scenario {
            name: "launch: a launch naming no caller has none, whatever FLUME_RECIPIENT gives its recipients",
            cites: &["7"],
            run: no_caller_named,
        },
        Scenario {
            name: "launch: a role's run an end pending for its address wakes has no caller, whatever the caller of the place whose end woke it",
            cites: &["132"],
            run: a_woken_run_has_no_caller,
        },
    ]
}

/// The host's settings giving every stage of a kind's agents `model`.
fn every_stage_model(model: &str) -> KindSettings {
    KindSettings {
        every_stage: Settings {
            model: Some(model.into()),
            ..Settings::default()
        },
        stages: Default::default(),
    }
}

/// Where the current agent of `place` took its model.
fn model_given(world: &World, place: &PlaceId) -> Result<Option<Given>, String> {
    Ok(world
        .status(place)?
        .agent
        .ok_or("no agent")?
        .settings
        .from
        .get("model")
        .cloned())
}

fn the_address_kept_and_given(_: &()) -> Outcome {
    // 56, 143; core.30: the address read from the origin and kept, the URL
    // dropped.
    let world = World::new();
    world
        .git
        .set_origin_url(REPOSITORY, "https://user:token@github.com/acme/acme.git");
    world.host.set_stream("work", Some(Stream::default()));
    world
        .host
        .name_stream(Some("acme/acme"), None, Some("work"));
    let place = launched(&world.engine.launch(&launch("chore", "one", Some("x"))))?;
    ensure_eq!(
        place.stream,
        StreamHeld {
            name: "work".into(),
            from: StreamFrom::HostRepository,
        },
        "the place's stream"
    );
    let id = World::id("chore", "one");
    let status = world.status(&id)?;
    ensure_eq!(
        (status.repository.as_str(), status.address.as_deref()),
        (REPOSITORY, Some("acme/acme")),
        "the repository and its address in status"
    );
    let held = world.held(&id).ok_or("no place")?;
    ensure_eq!(
        held.place.address.as_deref(),
        Some("acme/acme"),
        "the held address"
    );
    let held = serde_json::to_string(&held).map_err(|error| error.to_string())?;
    ensure!(!held.contains("token"), "the held state holds the token");
    let status = serde_json::to_string(&status).map_err(|error| error.to_string())?;
    ensure!(!status.contains("token"), "status holds the token");
    Ok(())
}

fn its_starts_take_the_addresss_tables(_: &()) -> Outcome {
    // 32, 180: the address's tables read at the start, for the model and
    // the label.
    let world = World::new();
    world
        .git
        .set_origin_url(REPOSITORY, "git@github.com:acme/acme.git");
    world
        .host
        .set_settings(Some("acme/acme"), "chore", every_stage_model("haiku"));
    world
        .host
        .name_label(Some("acme/acme"), "chore", Some("construction"));
    let place = launched(&world.engine.launch(&launch("chore", "one", Some("x"))))?;
    let id = World::id("chore", "one");
    ensure_eq!(
        model_given(&world, &id)?,
        Some(Given {
            layer: Layer::HostRepository,
            value: "haiku".into(),
            file: None,
        }),
        "the agent's model"
    );
    ensure_eq!(
        place.label,
        Some(LabelHeld {
            name: "construction".into(),
            from: LabelFrom::HostRepositoryKind,
        }),
        "the place's label"
    );
    Ok(())
}

fn a_place_held_with_no_address(_: &()) -> Outcome {
    // 32: a place launched with no address held takes its folder's tables
    // at a later start, though its origin now gives an address.
    let world = World::new();
    world
        .host
        .set_settings(Some(REPOSITORY), "chore", every_stage_model("opus"));
    world
        .host
        .set_settings(Some("acme/acme"), "chore", every_stage_model("haiku"));
    let mut held = launch("chore", "one", Some("x"));
    held.hold = vec!["code".into()];
    launched(&world.engine.launch(&held))?;
    let id = World::id("chore", "one");
    ensure_eq!(
        world.held(&id).ok_or("no place")?.place.address,
        None,
        "the held address"
    );
    world
        .git
        .set_origin_url(REPOSITORY, "git@github.com:acme/acme.git");
    if let Some(reason) = world
        .engine
        .retry(&id, Retry::Stage("code".into()), None, false)
        .refusal()
    {
        return Err(format!("the retry was refused: {reason}"));
    }
    ensure_eq!(
        model_given(&world, &id)?,
        Some(Given {
            layer: Layer::HostRepository,
            value: "opus".into(),
            file: None,
        }),
        "the agent's model"
    );
    Ok(())
}

fn the_identity_status_names(_: &()) -> Outcome {
    let world = World::new();
    let three = launched_as(&world, "change", "three")?;
    ensure_eq!(
        three.identity.as_deref(),
        Some("flume/change@3"),
        "the identity of a change named with no version"
    );
    let two = launched_as(&world, "change@2", "two")?;
    ensure_eq!(
        two.identity.as_deref(),
        Some("flume/change@2"),
        "the identity of change@2"
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
        (
            status.kind.as_str(),
            status.version,
            status.identity.as_deref()
        ),
        ("change", 3, Some("flume/change@3")),
        "the kind status names"
    );
    let held = world.held(&World::id("change", "plan")).ok_or("no place")?;
    ensure!(
        !held.place.kind.stages[1].repeats,
        "change@3's code repeats"
    );
    Ok(())
}

fn change_2_named(_: &()) -> Outcome {
    let world = World::new();
    let status = launched_as(&world, "change@2", "plan")?;
    ensure_eq!(
        (
            status.kind.as_str(),
            status.version,
            status.identity.as_deref()
        ),
        ("change", 2, Some("flume/change@2")),
        "the kind status names"
    );
    let held = world.held(&World::id("change", "plan")).ok_or("no place")?;
    ensure_eq!(held.place.branch, "change/plan", "the place's branch");
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
    let reason = refused(&world.engine.launch(&launch("change@4", "plan", Some("x"))))?;
    ensure!(
        reason.contains("change@4") && reason.contains("versions 1, 2, 3"),
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
    for (name, recipients, from) in [
        (
            "named",
            vec!["operator", "role:planner"],
            RecipientsFrom::Launch,
        ),
        ("variable", vec!["conductor"], RecipientsFrom::Environment),
        ("session", vec!["pane-7"], RecipientsFrom::Session),
    ] {
        let status = world.status(&World::id("fix", name))?;
        ensure_eq!(
            status.recipients,
            recipients
                .iter()
                .map(|recipient| recipient.to_string())
                .collect::<Vec<_>>(),
            format!("the recipients of {name}")
        );
        ensure_eq!(
            status.recipients_from,
            Some(from),
            format!("where {name}'s recipients came from")
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

/// A run the scratch source `source` declares for a role of its own kind
/// `turn`, standing behind its address's stream where `answerer` holds
/// (132, 193).
fn role_run(source: &str, answerer: bool) -> RoleRun {
    RoleRun {
        source: source.into(),
        repository: REPOSITORY.into(),
        kind: "turn".into(),
        priority: 0,
        base: None,
        brief: None,
        name: None,
        intent: None,
        agents: RoleAgents::default(),
        answers: None,
        answerer,
    }
}

/// The scratch host declares `stream`, and `source` declares
/// `<stream>/<role>` with a run, standing behind the stream where
/// `answerer` holds (143, 193).
pub fn declares(world: &World, source: &str, address: &str, answerer: bool) {
    world
        .host
        .set_stream(stream_of(address), Some(Stream::default()));
    world
        .host
        .set_role(address, Some(role_run(source, answerer)));
}

/// The launching command runs in no session the terminal names, and its
/// environment names no recipient.
pub fn from_no_session(world: &World) {
    world.host.set_recipient(None);
    world.terminal.set_session(None);
}

/// A fix named `name` launched in `stream` naming `recipients`.
fn fix_in(stream: &str, name: &str, recipients: &[&str]) -> Launch {
    Launch {
        stream: Some(stream.into()),
        recipients: recipients.iter().map(|name| name.to_string()).collect(),
        ..launch("fix", name, Some("mend it"))
    }
}

/// The place's recipients and where they came from, as status gives
/// them.
fn recipients_of(
    world: &World,
    name: &str,
) -> Result<(Vec<String>, Option<RecipientsFrom>), String> {
    let status = world.status(&World::id("fix", name))?;
    Ok((status.recipients, status.recipients_from))
}

/// The launching spec's "A launch from no session takes its stream's
/// answerer" (7, 193, S93).
fn a_launch_from_no_session_takes_its_streams_answerer(_: &()) -> Outcome {
    let world = World::new();
    from_no_session(&world);
    declares(&world, "acme", "east/operator", true);
    launched(&world.engine.launch(&fix_in("east", "one", &[])))?;
    ensure_eq!(
        recipients_of(&world, "one")?,
        (
            vec!["east/operator".to_string()],
            Some(RecipientsFrom::Answerer {
                source: "acme".into()
            })
        ),
        "the place's recipients and where they came from"
    );
    Ok(())
}

/// The launching spec's "Each stream its own answerer" (143, 193).
fn each_stream_its_own_answerer(_: &()) -> Outcome {
    let world = World::new();
    from_no_session(&world);
    declares(&world, "acme", "east/operator", true);
    declares(&world, "acme", "west/operator", true);
    launched(&world.engine.launch(&fix_in("east", "one", &[])))?;
    launched(&world.engine().launch(&fix_in("west", "two", &[])))?;
    ensure_eq!(
        recipients_of(&world, "one")?.0,
        vec!["east/operator".to_string()],
        "east's place's recipients"
    );
    ensure_eq!(
        recipients_of(&world, "two")?.0,
        vec!["west/operator".to_string()],
        "west's place's recipients"
    );
    Ok(())
}

/// The launching spec's "Another stream's role named" (7, 132, 193).
fn another_streams_role_named(_: &()) -> Outcome {
    let world = World::new();
    from_no_session(&world);
    declares(&world, "acme", "east/operator", true);
    declares(&world, "acme", "west/planner", false);
    launched(
        &world
            .engine
            .launch(&fix_in("east", "one", &["west/planner"])),
    )?;
    ensure_eq!(
        recipients_of(&world, "one")?,
        (
            vec!["west/planner".to_string()],
            Some(RecipientsFrom::Launch)
        ),
        "the place's recipients and where they came from"
    );
    Ok(())
}

/// The launching spec's "A launch from a shell into a stream no role
/// stands behind" (4, 7, 193; core.8).
fn a_launch_from_a_shell_into_a_stream_no_role_stands_behind(_: &()) -> Outcome {
    let world = World::new();
    from_no_session(&world);
    declares(&world, "acme", "east/operator", true);
    let reason = refused(&world.engine.launch(&fix_in(DEFAULT_STREAM, "one", &[])))?;
    for way in [
        "--recipient",
        "$FLUME_RECIPIENT",
        "session",
        "stream default",
    ] {
        ensure!(reason.contains(way), "the refusal names no {way}: {reason}");
    }
    nothing_made(&world)
}

/// The launching spec's "Two roles behind one stream" (4, 193; core.8).
fn two_roles_behind_one_stream(_: &()) -> Outcome {
    let world = World::new();
    from_no_session(&world);
    declares(&world, "acme", "east/operator", true);
    declares(&world, "beta", "east/door", true);
    let reason = refused(&world.engine.launch(&fix_in("east", "one", &[])))?;
    ensure!(
        reason.contains("east/operator, declared by acme")
            && reason.contains("east/door, declared by beta"),
        "the refusal names not both roles and their sources: {reason}"
    );
    nothing_made(&world)
}

/// The launching spec's "An address that wakes nothing, with no
/// answerer" (4, 132, 193; core.8).
fn an_address_that_wakes_nothing_with_no_answerer(_: &()) -> Outcome {
    let world = World::new();
    declares(&world, "acme", "east/operator", true);
    let reason = refused(
        &world
            .engine
            .launch(&fix_in(DEFAULT_STREAM, "one", &["east/planner"])),
    )?;
    ensure!(
        reason.contains("east/planner") && reason.contains("stream default"),
        "the refusal names not the address and the stream: {reason}"
    );
    nothing_made(&world)
}

/// The launching spec's "An address that wakes nothing, behind an
/// answerer": taken, and once an end stands pending for it, status says
/// the address wakes nothing (4, 56, 132, 193).
fn an_address_that_wakes_nothing_behind_an_answerer(_: &()) -> Outcome {
    let world = World::new();
    declares(&world, "acme", "east/operator", true);
    launched(
        &world
            .engine
            .launch(&fix_in("east", "one", &["east/planner"])),
    )?;
    ensure_eq!(
        recipients_of(&world, "one")?,
        (
            vec!["east/planner".to_string()],
            Some(RecipientsFrom::Launch)
        ),
        "the place's recipients and where they came from"
    );
    let id = World::id("fix", "one");
    world.settle(&id);
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    super::settling::deliver_fix(&world, &id);
    world.settle(&id);
    let host = match world.engine().status(None) {
        flume_core::Answer::Status { host, .. } => host,
        other => return Err(format!("status answered {other:?}")),
    };
    let planner = host
        .addresses
        .iter()
        .find(|address| address.address == "east/planner")
        .ok_or("status names no east/planner")?;
    ensure!(
        !planner.pending.is_empty(),
        "no end pending for east/planner"
    );
    ensure_eq!(
        planner.wakes_nothing.as_deref(),
        Some("no source of the host declares east/planner"),
        "why the address wakes nothing"
    );
    Ok(())
}

/// The launching spec's "A carried address" (4, 132, 193).
fn a_carried_address(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_carried(
        "north/planner",
        Some(CarriedRole {
            source: "acme".into(),
            answers: None,
        }),
    );
    launched(
        &world
            .engine
            .launch(&fix_in(DEFAULT_STREAM, "one", &["north/planner"])),
    )?;
    ensure_eq!(
        recipients_of(&world, "one")?.0,
        vec!["north/planner".to_string()],
        "the place's recipients"
    );
    Ok(())
}

/// The launching spec's "A caller named": the recipient
/// `madswan/planner` and the caller `boomdrive`, each given by status
/// apart, and the caller as it was after a hold and a retry (7, 8, 56).
fn a_caller_named(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    declares(&world, "acme", "madswan/planner", false);
    let mut named = fix_in("madswan", "one", &["madswan/planner"]);
    named.caller = Some("boomdrive".into());
    let place = launched(&world.engine.launch(&named))?;
    ensure_eq!(
        place.caller.as_deref(),
        Some("boomdrive"),
        "the caller the launch gives"
    );
    let id = World::id("fix", "one");
    world.settle(&id);
    let held = world
        .engine
        .hold(&id, HoldOn::Submission, "until the review", None);
    ensure!(held.refusal().is_none(), "the hold: {held:?}");
    super::settling::deliver_fix(&world, &id);
    world.settle(&id);
    let retried = world.engine.retry(&id, Retry::Submission, None, false);
    ensure!(retried.refusal().is_none(), "the retry: {retried:?}");
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    ensure_eq!(
        recipients_of(&world, "one")?,
        (
            vec!["madswan/planner".to_string()],
            Some(RecipientsFrom::Launch)
        ),
        "the place's recipients and where they came from"
    );
    ensure_eq!(
        world.status(&id)?.caller,
        Some("boomdrive".to_string()),
        "the caller after a hold and a retry"
    );
    ensure_eq!(
        world.held(&id).and_then(|held| held.place.caller),
        Some("boomdrive".to_string()),
        "the caller held"
    );
    Ok(())
}

/// The launching spec's "An empty caller" (4, 7; core.8).
fn an_empty_caller(_: &()) -> Outcome {
    let world = World::new();
    let mut empty = launch("fix", "one", Some("x"));
    empty.caller = Some(String::new());
    let reason = refused(&world.engine.launch(&empty))?;
    ensure!(
        reason.contains("caller"),
        "the refusal names no caller: {reason}"
    );
    nothing_made(&world)
}

/// The launching spec's "No caller named": `FLUME_RECIPIENT` gives the
/// place its recipient `ops`, and no caller (7).
fn no_caller_named(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_session(None);
    world.host.set_recipient(Some("ops"));
    let place = launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure_eq!(place.caller, None, "the caller the launch gives");
    ensure_eq!(
        recipients_of(&world, "one")?,
        (vec!["ops".to_string()], Some(RecipientsFrom::Environment)),
        "the place's recipients and where they came from"
    );
    ensure_eq!(
        world.status(&World::id("fix", "one"))?.caller,
        None,
        "the place's caller"
    );
    Ok(())
}

/// The launching spec's "A woken run has no caller": a fix whose
/// recipient is `east/ops` and whose caller is `boomdrive` delivers its
/// code, and the end pending for `east/ops` wakes acme's run of it,
/// which has no caller (132).
fn a_woken_run_has_no_caller(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    from_no_session(&world);
    super::role_waking::configure_at(
        &world,
        "east/ops",
        "Read what is pending for you.",
        super::role_waking::planner_run(None, None),
    );
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.recipients = vec!["east/ops".into()];
    fix.caller = Some("boomdrive".into());
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    super::settling::deliver_fix(&world, &id);
    world.settle(&id);
    let run = World::id("turn", "east-ops");
    let status = world.status(&run)?;
    ensure_eq!(
        status.role.as_deref(),
        Some("east/ops"),
        "the run the end woke"
    );
    ensure_eq!(status.caller, None, "the run's caller");
    Ok(())
}

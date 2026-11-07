//! Role addresses and the wake (3, 43, 132, S38; core.28): a role's run
//! is a place of the kind the host's configuration names for the
//! address, named by the role, on its base or the repository's default
//! branch, with its brief, its agents' session named by the address and
//! no recipient of its own; a settle that finds an end pending for an
//! address with no run standing launches it, a carrier's wake does the
//! same, and a wake of an address whose run stands makes nothing; an
//! address the configuration names nothing for is woken by nothing and
//! status names its pending ends; and a run's inbox collects for its
//! address. A place launched again under a name whose earlier place
//! landed never reads that landing as its own (3, 99). The host gives an
//! address a standing agent holds apart from a run (132, C.7): a settle
//! wakes no run for it and records nothing, a carrier's wake launches
//! nothing and answers held, and the configuration at each delivery
//! decides between a standing agent and a run (132, S38; core.28).

use super::settling::deliver_fix;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{
    AddressStatus, Answer, Engine, PendingEnd, PlaceStatus, StandingFound, StandingStatus, Woke,
};
use flume_fakes::{FakeHistory, FakeStore};
use flume_seams::*;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "role waking: a wake of a configured role launches its kind in its repository at its priority, cut from the default branch, with no brief and no recipient",
            cites: &["132", "70", "7"],
            run: a_role_configured,
        },
        Scenario {
            name: "role waking: a role's base and brief are its run's, the brief given whole to a stage that asks for it",
            cites: &["132", "27"],
            run: a_base_and_a_brief,
        },
        Scenario {
            name: "role waking: an end pending for an address the configuration names nothing for launches nothing, and status names it pending with no role",
            cites: &["132", "43", "56"],
            run: an_address_with_no_kind,
        },
        Scenario {
            name: "role waking: a run's agent is opened under the address as its session's name, and status names the place's role",
            cites: &["132", "94", "56"],
            run: the_runs_session,
        },
        Scenario {
            name: "role waking: two settles that each find an end pending for an address with no run make one run, the other's launch refused naming it and neither settle failing",
            cites: &["132", "3", "75"],
            run: two_settles_at_once,
        },
        Scenario {
            name: "role waking: three ends for an address with no live run are one run, whose inbox gives all three (S38)",
            cites: &["S38", "43", "132"],
            run: three_ends_one_run,
        },
        Scenario {
            name: "role waking: a carrier's wake while the address's run waits for room answers that it waits and makes no second place (S38)",
            cites: &["S38", "132", "70"],
            run: a_wake_while_the_run_waits_for_room,
        },
        Scenario {
            name: "role waking: a run's inbox with no recipient named collects every end pending for its address, each then delivered to it",
            cites: &["132", "43"],
            run: the_run_starts,
        },
        Scenario {
            name: "role waking: a place launched again under a name whose earlier place landed is not read landed by its first settle",
            cites: &["3", "99", "132"],
            run: launched_again_after_a_landing,
        },
        Scenario {
            name: "role waking: a role's run whose table names a stream runs in it, from the launch, and one naming a stream the host does not declare is not woken",
            cites: &["132", "143"],
            run: a_role_in_its_stream,
        },
        Scenario {
            name: "role waking: a role whose kind names a host's source with no version is woken by an end pending for it to the newest the locks record, status naming its identity and the source's file",
            cites: &["132", "154", "155", "191"],
            run: a_role_woken_to_a_hosts_source,
        },
        Scenario {
            name: "role waking: a role whose kind no lock records is refused at its wake naming it, no place made",
            cites: &["4", "132", "155"],
            run: a_roles_kind_unlocked,
        },
        Scenario {
            name: "role waking: the host gives the standing agent it names for one address apart from the run it names for another, and no run for the standing address",
            cites: &["132", "3"],
            run: standing_and_run_apart,
        },
        Scenario {
            name: "role waking: three ends pending for a standing address whose agent is nowhere, settled twice, make no place, open nothing and leave the address unwoken by nothing (S38)",
            cites: &["132", "S38"],
            run: ends_pending_while_the_agent_is_away,
        },
        Scenario {
            name: "role waking: a carrier's wake of a standing address launches nothing and answers held, naming the agent, its session, what was found and the ends pending",
            cites: &["132"],
            run: a_carrier_wakes_a_standing_address,
        },
        Scenario {
            name: "role waking: an entry giving a standing agent for an address whose run of an earlier entry is live sends the next end to the standing agent and none to the run",
            cites: &["132", "32"],
            run: a_standing_agent_over_a_run_still_live,
        },
        Scenario {
            name: "role waking: an entry that gave a standing agent and then gives a kind wakes a run for the next end, and the standing agent takes no second notice",
            cites: &["132", "32"],
            run: a_standing_entry_becomes_a_runs,
        },
    ]
}

/// The seam-contracts spec's "Standing and run apart": the fake host
/// given a standing agent for the planner and a run for `madswan/ops`
/// holds the planner alone among its standing entries, gives no run for
/// it, and gives ops's run (3, 132, C.7).
fn standing_and_run_apart(_: &()) -> Outcome {
    let world = World::new();
    let planner = StandingAgent {
        agent: "madswan-planner".into(),
        session: Some("madswan".into()),
        stream: None,
    };
    world.host.set_standing(PLANNER, Some(planner.clone()));
    world
        .host
        .set_role("madswan/ops", Some(planner_run(None, None)));
    let host: &dyn Host = world.host.as_ref();
    ensure_eq!(
        host.standing(),
        BTreeMap::from([(PLANNER.to_string(), planner)]),
        "the standing entries"
    );
    ensure_eq!(host.role(PLANNER), None, "the planner's run");
    ensure_eq!(
        host.role("madswan/ops"),
        Some(planner_run(None, None)),
        "ops's run"
    );
    Ok(())
}

const PLANNER: &str = "madswan/planner";

/// The planner's run as the configuration names it in the repository.
fn planner_run(base: Option<&str>, brief: Option<&str>) -> RoleRun {
    RoleRun {
        repository: REPOSITORY.into(),
        kind: "planner".into(),
        priority: 5,
        base: base.map(str::to_string),
        brief: brief.map(str::to_string),
        stream: None,
    }
}

/// The repository declares the kind `planner`, one conversation stage
/// whose prompt is `prompt`, and the host names `run` for the planner.
fn configure(world: &World, prompt: &str, run: RoleRun) {
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind("planner", false, vec![stage("plan", None, prompt)])],
    );
    world.host.set_role(PLANNER, Some(run));
}

fn planner_id() -> PlaceId {
    World::id("planner", "planner")
}

/// What a wake answered: how it woke and the run's status.
fn woken(answer: &Answer) -> Result<(Woke, PlaceStatus), String> {
    match answer {
        Answer::Woken { woke, place, .. } => Ok((*woke, place.clone())),
        other => Err(format!("the wake answered {other:?}")),
    }
}

/// The places of every role's run on the host.
fn runs(world: &World) -> Vec<PlaceId> {
    world
        .store
        .places()
        .into_iter()
        .filter(|id| world.held(id).is_some_and(|held| held.role.is_some()))
        .collect()
}

/// A fix named `name` launched for `recipients` and begun.
fn fix_for(world: &World, name: &str, recipients: &[&str]) -> Result<PlaceId, String> {
    let mut launch = launch("fix", name, Some("mend it"));
    launch.recipients = recipients.iter().map(|name| name.to_string()).collect();
    launched(&world.engine.launch(&launch))?;
    let id = World::id("fix", name);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "{name}'s code"
    );
    Ok(id)
}

/// The fix's code agent stops short needing an answer, and the place is
/// settled.
fn asks(world: &World, id: &PlaceId) -> Outcome {
    let start = world
        .held(id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?;
    let answer = world.engine.needs(
        &start.identity.to_string(),
        &start.secret.0,
        "Which file holds the bug?",
        StopKind::NeedsAnswer,
    );
    ensure!(answer.refusal().is_none(), "the stop-short was refused");
    world.settle(id);
    Ok(())
}

/// The host's role addresses as status gives them.
fn addresses(world: &World) -> Result<Vec<AddressStatus>, String> {
    match world.engine().status(None) {
        Answer::Status { host, .. } => Ok(host.addresses),
        other => Err(format!("status answered {other:?}")),
    }
}

fn a_role_configured(_: &()) -> Outcome {
    let world = World::new();
    world.git.init(REPOSITORY, "trunk");
    world.git.set_default_branch(REPOSITORY, "trunk");
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let (woke, place) = woken(&world.engine.wake(PLANNER))?;
    ensure_eq!(woke, Woke::Launched, "the wake");
    ensure_eq!(place.place, planner_id(), "the run's place");
    ensure_eq!(
        (
            place.kind.as_str(),
            place.name.as_str(),
            place.base.as_str()
        ),
        ("planner", "planner", "trunk"),
        "the run's kind, name and base"
    );
    ensure_eq!(place.repository, REPOSITORY, "the run's repository");
    ensure_eq!(place.priority, 5, "the run's priority");
    ensure_eq!(place.role.as_deref(), Some(PLANNER), "the run's role");
    ensure_eq!(
        place.recipients,
        Vec::<String>::new(),
        "the run's recipients"
    );
    let held = world.held(&planner_id()).ok_or("no run")?;
    ensure_eq!(held.place.brief, None, "the run's brief");
    let refused = world.engine.wake("planner");
    ensure!(
        refused
            .refusal()
            .is_some_and(|reason| reason.contains("address")),
        "a wake of no address: {refused:?}"
    );
    Ok(())
}

fn a_base_and_a_brief(_: &()) -> Outcome {
    let world = World::new();
    world.git.init(REPOSITORY, "next");
    let brief = "Plan the next unit.\n\nRead the records first.";
    configure(&world, "{{brief}}", planner_run(Some("next"), Some(brief)));
    let (_, place) = woken(&world.engine.wake(PLANNER))?;
    ensure_eq!(place.base, "next", "the run's base");
    let identity = World::identity("planner", "planner", "plan", 1);
    let prompt = world
        .prompt(&planner_id(), &identity)
        .ok_or("no prompt sent")?;
    ensure!(
        prompt.contains(brief),
        "the prompt lacks the brief: {prompt}"
    );
    Ok(())
}

fn an_address_with_no_kind(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_for(&world, "one", &["madswan/ops"])?;
    deliver_fix(&world, &id);
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    world.settle(&id);
    ensure_eq!(runs(&world), Vec::<PlaceId>::new(), "runs launched");
    ensure_eq!(world.store.places(), vec![id.clone()], "places on the host");
    let addresses = addresses(&world)?;
    ensure_eq!(addresses.len(), 1, "addresses status names");
    let ops = &addresses[0];
    ensure_eq!(ops.address, "madswan/ops", "the address");
    ensure_eq!(ops.role, None, "its role");
    ensure_eq!(ops.run, None, "its run");
    ensure_eq!(ops.pending.len(), 1, "the ends pending for it");
    ensure_eq!(ops.pending[0].what, "code delivered", "what ended");
    ensure!(
        ops.unwoken
            .as_deref()
            .is_some_and(|why| why.contains("names no role for madswan/ops")),
        "why it was not woken: {:?}",
        ops.unwoken
    );
    Ok(())
}

fn the_runs_session(_: &()) -> Outcome {
    let world = World::new();
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    woken(&world.engine.wake(PLANNER))?;
    world.settle(&planner_id());
    ensure_eq!(
        world.stage_state(&planner_id(), "plan"),
        Some(StageState::Begun),
        "the run's stage"
    );
    ensure!(
        world.terminal.session_state(PLANNER).is_some(),
        "no session is named {PLANNER}"
    );
    let status = world.status(&planner_id())?;
    ensure_eq!(
        status.role.as_deref(),
        Some(PLANNER),
        "the role status names"
    );
    Ok(())
}

/// Runs `then` at the first write after the history has been asked
/// whether a branch is valid, as a launch asks just before it creates its
/// place's document; re-armed at each write before that.
fn at_the_create(store: Arc<FakeStore>, history: Arc<FakeHistory>, then: Box<dyn FnOnce() + Send>) {
    let again = store.clone();
    store.before_next_write(move || {
        if history.log.count("valid_branch") > 0 {
            then();
        } else {
            at_the_create(again, history, then);
        }
    });
}

fn two_settles_at_once(_: &()) -> Outcome {
    let world = World::new();
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let id = fix_for(&world, "one", &[PLANNER])?;
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    deliver_fix(&world, &id);
    world.history.log.clear();
    let other: Engine = world.engine();
    let place = id.clone();
    let ran = Arc::new(Mutex::new(false));
    let into = ran.clone();
    at_the_create(
        world.store.clone(),
        world.history.clone(),
        Box::new(move || {
            other.settle(Some(&place));
            *into.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = true;
        }),
    );
    let answer = world.engine().settle(Some(&id));
    ensure!(answer.refusal().is_none(), "the settle failed: {answer:?}");
    ensure!(
        *ran.lock().unwrap_or_else(|poisoned| poisoned.into_inner()),
        "the other settle did not run"
    );
    ensure!(
        world
            .store
            .log
            .calls()
            .iter()
            .any(|call| call == &format!("write {} at new moved", planner_id())),
        "the second launch was not refused at its create"
    );
    ensure_eq!(runs(&world), vec![planner_id()], "the runs made");
    let opened = World::identity("planner", "planner", "plan", 1);
    ensure!(
        world.terminal.opened(&opened).is_some(),
        "the run's agent was not opened"
    );
    ensure_eq!(
        world.terminal.log.count("open"),
        2,
        "agents opened: the fix's and the run's"
    );
    let held = world.held(&id).ok_or("no place")?;
    ensure!(
        held.unwoken.is_empty(),
        "a wake was named failed: {:?}",
        held.unwoken
    );
    Ok(())
}

fn three_ends_one_run(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_watch(Watch::Bound { seconds: 30 });
    world.host.set_bound(Some(3));
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let mut busy = launch("fix", "busy", Some("hold the room"));
    busy.recipients = vec!["operator".into()];
    launched(&world.engine.launch(&busy))?;
    let busy = World::id("fix", "busy");
    world.settle(&busy);
    let one = fix_for(&world, "one", &[PLANNER])?;
    let two = fix_for(&world, "two", &[PLANNER])?;
    asks(&world, &one)?;
    ensure_eq!(
        runs(&world),
        vec![planner_id()],
        "the run the first end woke"
    );
    ensure_eq!(
        world.stage_state(&planner_id(), "plan"),
        Some(StageState::WaitingForRoom),
        "the run's stage"
    );
    asks(&world, &two)?;
    world.host.advance(5 * 60);
    world.settle(&one);
    let held = world.held(&one).ok_or("no place")?;
    ensure!(
        held.ends
            .iter()
            .any(|end| matches!(end.of, EndOf::Stall { .. })),
        "one did not stall: {:?}",
        held.ends
    );
    ensure_eq!(runs(&world), vec![planner_id()], "runs after three ends");
    ensure_eq!(
        world.stage_state(&planner_id(), "plan"),
        Some(StageState::WaitingForRoom),
        "the run's stage before room"
    );
    let busy_start = World::identity("fix", "busy", "code", 1);
    world.terminal.kill(&busy_start);
    world.settle(&busy);
    let identity = World::identity("planner", "planner", "plan", 1);
    ensure!(
        world.terminal.opened(&identity).is_some(),
        "the run's agent did not start once there was room"
    );
    let run = world.engine();
    let recipient = run
        .recipient(Some(&planner_id()))
        .ok_or("the run names no recipient")?;
    ensure_eq!(recipient, PLANNER, "the run's recipient");
    let collected = match run.inbox(&recipient) {
        Answer::Inbox { ends, .. } => ends
            .iter()
            .map(|collected| {
                format!(
                    "{}--{} {}",
                    collected.kind,
                    collected.name,
                    flume_core::what_ended(&collected.end.end.of)
                )
            })
            .collect::<Vec<_>>(),
        other => return Err(format!("the inbox answered {other:?}")),
    };
    let mut collected = collected;
    collected.sort();
    ensure_eq!(
        collected,
        vec![
            "fix--one code needs an answer".to_string(),
            "fix--one stalled: a retry of code".to_string(),
            "fix--two code needs an answer".to_string(),
        ],
        "the ends the run's inbox gives"
    );
    ensure_eq!(
        world.terminal.notices(PLANNER),
        Vec::<String>::new(),
        "notices"
    );
    ensure_eq!(runs(&world), vec![planner_id()], "runs at the end");
    Ok(())
}

fn a_wake_while_the_run_waits_for_room(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_bound(Some(1));
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    fix_for(&world, "busy", &["operator"])?;
    let (woke, place) = woken(&world.engine().wake(PLANNER))?;
    ensure_eq!(woke, Woke::Launched, "the first wake");
    ensure_eq!(
        place.stages[0].state,
        StageState::WaitingForRoom,
        "the run's stage"
    );
    let opens = world.terminal.log.count("open");
    let places = world.store.places();
    let (woke, place) = woken(&world.engine().wake(PLANNER))?;
    ensure_eq!(woke, Woke::Waiting, "the carrier's wake");
    ensure_eq!(place.place, planner_id(), "the run it names");
    ensure_eq!(world.store.places(), places, "the places on the host");
    ensure_eq!(world.terminal.log.count("open"), opens, "agents opened");
    world.settle(&planner_id());
    Ok(())
}

fn the_run_starts(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_watch(Watch::Bound { seconds: 30 });
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    let one = fix_for(&world, "one", &[PLANNER, "operator"])?;
    deliver_fix(&world, &one);
    world.settle(&one);
    ensure_eq!(runs(&world), vec![planner_id()], "the run woken");
    let run = world.engine();
    let recipient = run
        .recipient(Some(&planner_id()))
        .ok_or("the run names no recipient")?;
    let ends = match run.inbox(&recipient) {
        Answer::Inbox { ends, .. } => ends,
        other => return Err(format!("the inbox answered {other:?}")),
    };
    ensure_eq!(ends.len(), 1, "the ends collected");
    let held = world.held(&one).ok_or("no place")?;
    ensure!(
        held.ends
            .iter()
            .all(|end| end.deliveries.iter().any(|delivery| {
                delivery.recipient == PLANNER && delivery.by == DeliveredBy::Inbox
            })),
        "an end not delivered to the address: {:?}",
        held.ends
    );
    ensure_eq!(
        held.pending_for(),
        vec!["operator".to_string()],
        "the ends still pending"
    );
    Ok(())
}

fn launched_again_after_a_landing(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_for(&world, "one", &["operator"])?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place's own document");
    launched(
        &world
            .engine
            .launch(&launch("fix", "one", Some("mend it again"))),
    )?;
    world.settle(&id);
    let held = world.held(&id).ok_or("the place launched again is gone")?;
    ensure_eq!(held.phase, Phase::Live, "the place launched again");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "its code"
    );
    ensure!(
        world
            .status(&id)?
            .submission
            .is_some_and(|submission| submission.read == SubmissionRead::None),
        "status reads the earlier place's submission as its own"
    );
    Ok(())
}

fn a_role_in_its_stream(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    configure(
        &world,
        "Read what is pending for you.",
        RoleRun {
            stream: Some("work".into()),
            ..planner_run(None, None)
        },
    );
    let (woke, place) = woken(&world.engine.wake(PLANNER))?;
    ensure_eq!(woke, Woke::Launched, "the wake");
    ensure_eq!(
        place.stream,
        StreamHeld {
            name: "work".into(),
            from: StreamFrom::Launch,
        },
        "the run's stream"
    );
    world.host.set_role(
        "madswan/ops",
        Some(RoleRun {
            kind: "planner".into(),
            stream: Some("nightly".into()),
            ..planner_run(None, None)
        }),
    );
    let refused = world.engine.wake("madswan/ops");
    ensure!(
        refused
            .refusal()
            .is_some_and(|reason| reason.contains("nightly")),
        "a wake into an undeclared stream: {refused:?}"
    );
    ensure_eq!(runs(&world), vec![planner_id()], "the runs on the host");
    Ok(())
}

/// The planner's run, its kind `kind` as the host's configuration names
/// it, with acme holding turn@1 and turn@2 (132, 154, 191).
fn turn_planner(world: &World, kind: &str) {
    super::kind_sources::holds(
        world,
        "acme",
        "/kinds/acme",
        &[("turn", 1, '1'), ("turn", 2, '2')],
    );
    world.host.set_role(
        PLANNER,
        Some(RoleRun {
            kind: kind.into(),
            ..planner_run(None, Some("take your turn"))
        }),
    );
}

fn a_role_woken_to_a_hosts_source(_: &()) -> Outcome {
    use crate::seams::kinds::{entry, hash_of};
    let world = World::new();
    turn_planner(&world, "acme/turn");
    super::kind_sources::host_locks(
        &world,
        vec![
            entry("acme", "turn", 1, &hash_of('1')),
            entry("acme", "turn", 2, &hash_of('2')),
        ],
    );
    let id = fix_for(&world, "one", &[PLANNER])?;
    deliver_fix(&world, &id);
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    world.settle(&id);
    let run = World::id("turn", "planner");
    ensure_eq!(runs(&world), vec![run.clone()], "the run woken");
    let status = world.status(&run)?;
    ensure_eq!(
        status.identity.as_deref(),
        Some("acme/turn@2"),
        "the run's kind"
    );
    ensure_eq!(
        status.kind_from.and_then(|from| from.source_file),
        Some(flume_fakes::source_file("acme")),
        "the source's file"
    );
    ensure!(
        woken(&world.engine.wake(PLANNER))?.0 != Woke::Launched,
        "a second wake of the run that stands launched another"
    );
    ensure_eq!(runs(&world), vec![run], "the runs after a second wake");
    Ok(())
}

fn a_roles_kind_unlocked(_: &()) -> Outcome {
    let world = World::new();
    turn_planner(&world, "acme/turn@1");
    let refused = world.engine.wake(PLANNER);
    let reason = refused.refusal().ok_or("the wake was taken")?;
    ensure!(
        reason.contains("acme/turn@1") && reason.contains("recorded in no lock"),
        "the refusal names not the kind as unlocked: {reason}"
    );
    ensure_eq!(world.store.places(), Vec::<PlaceId>::new(), "places made");
    ensure_eq!(world.place.log.count("make"), 0, "makes");
    Ok(())
}

/// The host's configuration gives the planner the standing agent
/// `madswan-planner` in the session `madswan`.
fn planner_stands(world: &World) {
    world.host.set_standing(
        PLANNER,
        Some(StandingAgent {
            agent: "madswan-planner".into(),
            session: Some("madswan".into()),
            stream: None,
        }),
    );
}

/// The notice a fix's code delivered is sent as.
fn code_delivered(name: &str) -> String {
    format!("flume: fix--{name} code delivered; read: flume status fix--{name}")
}

/// A fix named `name` for the planner, delivered with its submission
/// held, and settled.
fn delivered_for_planner(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = fix_for(world, name, &[PLANNER])?;
    deliver_fix(world, &id);
    world.settle(&id);
    Ok(id)
}

fn ends_pending_while_the_agent_is_away(_: &()) -> Outcome {
    let world = World::new();
    planner_stands(&world);
    world.queue.plan_submits(vec![
        flume_fakes::Plan::Hold,
        flume_fakes::Plan::Hold,
        flume_fakes::Plan::Hold,
    ]);
    let places: Vec<PlaceId> = ["one", "two", "three"]
        .into_iter()
        .map(|name| fix_for(&world, name, &[PLANNER]))
        .collect::<Result<_, _>>()?;
    let opens = world.terminal.log.count("open");
    for id in &places {
        deliver_fix(&world, id);
    }
    for _ in 0..2 {
        for id in &places {
            world.settle(id);
        }
    }
    world.engine().settle(None);
    let mut sorted = places.clone();
    sorted.sort();
    ensure_eq!(world.store.places(), sorted, "the places on the host");
    ensure_eq!(runs(&world), Vec::<PlaceId>::new(), "runs launched");
    ensure_eq!(world.terminal.log.count("open"), opens, "agents opened");
    for id in &places {
        let held = world.held(id).ok_or("no place")?;
        ensure_eq!(
            held.pending_for(),
            vec![PLANNER.to_string()],
            format!("pending for {id}")
        );
        ensure!(
            held.unwoken.is_empty(),
            "{id}'s unwoken names {:?}",
            held.unwoken
        );
    }
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent to nobody"
    );
    Ok(())
}

/// What a wake answered held: the address, where it lives and its ends.
fn held(answer: &Answer) -> Result<(String, StandingStatus, Vec<PendingEnd>), String> {
    match answer {
        Answer::Held {
            address,
            standing,
            pending,
        } => Ok((address.clone(), standing.clone(), pending.clone())),
        other => Err(format!("the wake answered {other:?}")),
    }
}

fn a_carrier_wakes_a_standing_address(_: &()) -> Outcome {
    let world = World::new();
    planner_stands(&world);
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    let id = delivered_for_planner(&world, "one")?;
    let opens = world.terminal.log.count("open");
    let (address, standing, pending) = held(&world.engine().wake(PLANNER))?;
    ensure_eq!(address, PLANNER, "the address");
    ensure_eq!(standing.agent, "madswan-planner", "the agent");
    ensure_eq!(
        standing.session,
        SessionHeld {
            name: Some("madswan".into()),
            from: SessionFrom::Role,
        },
        "the session"
    );
    ensure_eq!(standing.found, StandingFound::None, "found while away");
    ensure_eq!(
        pending
            .iter()
            .map(|end| (end.place.clone(), end.what.clone()))
            .collect::<Vec<_>>(),
        vec![(id.clone(), "code delivered".to_string())],
        "the ends pending"
    );
    let at = world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Waiting);
    let (_, standing, pending) = held(&world.engine().wake(PLANNER))?;
    ensure_eq!(
        standing.found,
        StandingFound::One {
            at: at.clone(),
            state: AgentState::Waiting,
        },
        "found once it waits"
    );
    ensure_eq!(
        pending,
        Vec::<PendingEnd>::new(),
        "pending after the wake's settle"
    );
    ensure_eq!(
        world.terminal.notices(&at),
        vec![code_delivered("one")],
        "the notice the wake's settle sent"
    );
    ensure_eq!(world.store.places(), vec![id], "the places on the host");
    ensure_eq!(runs(&world), Vec::<PlaceId>::new(), "runs launched");
    ensure_eq!(world.terminal.log.count("open"), opens, "agents opened");
    Ok(())
}

fn a_standing_agent_over_a_run_still_live(_: &()) -> Outcome {
    let world = World::new();
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let (woke, _) = woken(&world.engine.wake(PLANNER))?;
    ensure_eq!(woke, Woke::Launched, "the run woken");
    world.settle(&planner_id());
    ensure!(
        world.terminal.session_state(PLANNER).is_some(),
        "the run's session is not named {PLANNER}"
    );
    world.host.set_role(PLANNER, None);
    planner_stands(&world);
    let at = world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Waiting);
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    delivered_for_planner(&world, "one")?;
    ensure_eq!(
        world.terminal.notices(&at),
        vec![code_delivered("one")],
        "the standing agent's notices"
    );
    ensure_eq!(
        world.terminal.notices(PLANNER),
        Vec::<String>::new(),
        "the run's notices"
    );
    ensure_eq!(runs(&world), vec![planner_id()], "the runs on the host");
    Ok(())
}

fn a_standing_entry_becomes_a_runs(_: &()) -> Outcome {
    let world = World::new();
    planner_stands(&world);
    let at = world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Waiting);
    world
        .queue
        .plan_submits(vec![flume_fakes::Plan::Hold, flume_fakes::Plan::Hold]);
    delivered_for_planner(&world, "one")?;
    ensure_eq!(
        world.terminal.notices(&at),
        vec![code_delivered("one")],
        "the standing agent's first notice"
    );
    ensure_eq!(runs(&world), Vec::<PlaceId>::new(), "runs while it stands");
    world.host.set_standing(PLANNER, None);
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let two = delivered_for_planner(&world, "two")?;
    ensure_eq!(runs(&world), vec![planner_id()], "the run woken");
    ensure_eq!(
        world.held(&two).map(|held| held.unwoken),
        Some(BTreeMap::new()),
        "two's unwoken"
    );
    world.settle(&planner_id());
    world.settle(&two);
    ensure_eq!(
        world.terminal.notices(&at).len(),
        1,
        "the standing agent's notices"
    );
    Ok(())
}

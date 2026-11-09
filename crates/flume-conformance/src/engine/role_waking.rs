//! Role addresses and the wake (3, 43, 114, 130, 132, 193, S38, S99;
//! core.28): an address is `<stream>/<role>`; a role's run is a place of
//! the kind of its own a source declares for the address,
//! `<source>/<kind>` at the version the locks record, in the address's
//! stream under its kind's label, named by its entry's name, else
//! `<stream>-<role>`, on its base or the repository's default branch,
//! with its brief, intent and agents' settings, its agents' session named
//! by the address and no recipient of its own; a settle that finds an end
//! pending for an address with no run standing launches it, a carrier's
//! wake does the same, and a wake of an address whose run stands, found
//! by its role first, makes nothing; an address no source declares, two
//! declare, whose stream the host does not declare or whose entry is at
//! fault is woken by nothing, its wake and status saying why; a carried
//! address wakes nothing and its ends stay pending; and a run's inbox
//! collects for its address. A run is done and tidied once its last
//! stage delivers, the next end waking another, and its stall reaches
//! the answerer behind its stream, never its own address. A place
//! launched again under a name whose earlier place landed never reads
//! that landing as its own (3, 99). The host gives an address a standing
//! agent holds apart from a run (132, 193): a settle wakes no run for it
//! and records nothing, its ends go into the agent's pane, a carrier's
//! wake launches nothing and answers held, and the source's entry at
//! each delivery decides between a standing agent and a run (132, S38;
//! core.28; record 0039's amendment).

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
            name: "role waking: a wake of a role a source declares launches its source's own kind at the version locked, in its repository at its priority, cut from the default branch, with no brief and no recipient",
            cites: &["132", "193", "154", "155", "70", "7"],
            run: a_role_configured,
        },
        Scenario {
            name: "role waking: a role's base and brief are its run's, the brief given whole to a stage that asks for it",
            cites: &["132", "27"],
            run: a_base_and_a_brief,
        },
        Scenario {
            name: "role waking: an end pending for an address no source declares launches nothing, and status names it pending with no role",
            cites: &["132", "43", "56"],
            run: an_address_with_no_kind,
        },
        Scenario {
            name: "role waking: a run whose entry names no name is the place turn--east-planner, its agent opened under the address as its session's name, and status names the place's role",
            cites: &["132", "94", "56", "3", "193"],
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
            name: "role waking: a role's run runs in its address's stream, from the launch, under its kind's label, and one whose address names a stream the host does not declare is not woken",
            cites: &["132", "143", "193", "180", "S99"],
            run: a_role_in_its_stream,
        },
        Scenario {
            name: "role waking: a role whose kind is its source's own is woken by an end pending for it to the newest the locks record, status naming its identity and the source's file",
            cites: &["132", "154", "155", "191", "193"],
            run: a_role_woken_to_a_hosts_source,
        },
        Scenario {
            name: "role waking: a role whose kind no lock records is refused at its wake naming it as a launch's kind is, no place made",
            cites: &["4", "132", "155", "193"],
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
        Scenario {
            name: "role waking: a wake of south/planner, whose entry names a stream the host does not declare, is refused naming south, no place made, and status names the entry and the stream under its source",
            cites: &["4", "143", "132", "193"],
            run: a_stream_the_host_does_not_declare,
        },
        Scenario {
            name: "role waking: a run whose entry gives the intent east-planner and the model opus serves that intent, its agent starting with opus, which status gives as from the launch",
            cites: &["32", "147", "7", "193"],
            run: the_runs_intent_and_settings,
        },
        Scenario {
            name: "role waking: ends at once for east/planner and west/planner, declared in one repository and kind, named east and west, launch turn--east and turn--west, each run's inbox giving its own address's ends (S99)",
            cites: &["S99", "132", "193", "3", "143"],
            run: s99_one_declaration_two_streams,
        },
        Scenario {
            name: "role waking: a run of east/planner an earlier flume made as turn--planner is found by its role, no run launched for the next end, which is delivered into its pane",
            cites: &["132", "3", "193"],
            run: a_run_an_earlier_flume_named,
        },
        Scenario {
            name: "role waking: an end for east/planner whose run's agent waits in its session before its start took its prompt stays pending, and is delivered into its pane once by the settle after its start is begun",
            cites: &["37", "38", "43", "132"],
            run: a_run_not_yet_given_its_prompt,
        },
        Scenario {
            name: "role waking: an end pending for north/planner, which acme declares carried and whose stream the host does not declare, launches nothing and stays pending, status naming it carried and its wake refused so",
            cites: &["132", "193", "56"],
            run: a_carried_address,
        },
        Scenario {
            name: "role waking: an end pending for east/design, which acme gives the standing agent east-design, is delivered into that agent's pane in east's session as a notice, nothing launched",
            cites: &["132", "193", "190"],
            run: a_standing_design_agent,
        },
        Scenario {
            name: "role waking: acme's file trading east/design's agent for the kind turn and a repository, the host's configuration unchanged, wakes a run of acme/turn@1 in east for the next end, and nothing more goes to east-design",
            cites: &["132", "193", "154"],
            run: the_crossing_to_its_kind,
        },
        Scenario {
            name: "role waking: a place launched from no session into east, behind which acme gives the standing agent east-operator, stalls into that agent's pane, and status names east/operator as east's answerer from acme",
            cites: &["193", "130", "132", "7"],
            run: a_standing_door,
        },
        Scenario {
            name: "role waking: an entry at fault and an address two sources declare wake nothing, their wakes and status saying why, while another address of the source wakes its run",
            cites: &["3", "4", "56", "132", "191", "193"],
            run: a_faults_address,
        },
        Scenario {
            name: "role waking: a role's run whose one stage delivered, its agent quiet, is declared done by the settle that reads it with no caller's word and tidied, no run of the address standing (S99)",
            cites: &["114", "132", "193", "S99"],
            run: a_turn_delivers,
        },
        Scenario {
            name: "role waking: an end for east/planner after its run was tidied launches a new run at the settle that reads it (S99)",
            cites: &["S99", "132", "114"],
            run: the_next_end,
        },
        Scenario {
            name: "role waking: a role's run whose kind lands, its one stage delivered and its agent quiet, is submitted through the merge queue and declared nothing done, and is tidied once its submission lands",
            cites: &["63", "114", "132"],
            run: a_run_whose_kind_lands,
        },
        Scenario {
            name: "role waking: the run of east/planner lost and left five minutes stalls to east/operator, the answerer behind east, whose run is woken, and never to its own address",
            cites: &["130", "132", "193"],
            run: a_planners_run_is_lost,
        },
        Scenario {
            name: "role waking: the run of east/operator, its own stream's answerer, lost and stalled, reaches no address, and status names its stall under the host's addresses",
            cites: &["130", "132", "193", "56"],
            run: the_doors_own_run,
        },
    ]
}

/// The seam-contracts spec's "Standing and run apart": the fake host
/// given a standing agent for the planner and a run for `east/ops`
/// holds the planner alone among its standing entries, gives no run for
/// it, and gives ops's run (3, 132, C.7).
fn standing_and_run_apart(_: &()) -> Outcome {
    let world = World::new();
    let planner = StandingAgent {
        source: ACME.into(),
        agent: "east-planner".into(),
        session: Some("east".into()),
        answers: None,
        answerer: false,
    };
    world.host.set_standing(PLANNER, Some(planner.clone()));
    world
        .host
        .set_role("east/ops", Some(planner_run(None, None)));
    let host: &dyn Host = world.host.as_ref();
    ensure_eq!(
        host.standing(),
        BTreeMap::from([(PLANNER.to_string(), planner)]),
        "the standing entries"
    );
    ensure_eq!(host.role(PLANNER), None, "the planner's run");
    ensure_eq!(
        host.role("east/ops"),
        Some(planner_run(None, None)),
        "ops's run"
    );
    Ok(())
}

const PLANNER: &str = "east/planner";

/// The source that declares the roles, and where its packages lie.
const ACME: &str = "acme";
const ACME_KINDS: &str = "/kinds/acme";

/// The planner's run as acme declares it in the repository, of its own
/// kind `turn`.
pub fn planner_run(base: Option<&str>, brief: Option<&str>) -> RoleRun {
    RoleRun {
        source: ACME.into(),
        repository: REPOSITORY.into(),
        kind: "turn".into(),
        priority: 5,
        base: base.map(str::to_string),
        brief: brief.map(str::to_string),
        name: None,
        intent: None,
        agents: RoleAgents::default(),
        answers: None,
        answerer: false,
    }
}

/// acme declares a run of `<stream>/operator` standing behind `stream`,
/// so a launch into it may name an address that wakes nothing (193).
pub fn answerer_behind(world: &World, stream: &str) {
    world.host.set_role(
        &format!("{stream}/operator"),
        Some(RoleRun {
            answerer: true,
            ..planner_run(None, None)
        }),
    );
}

/// acme holds `turn@1`, one conversation stage `plan` whose prompt is
/// `prompt`, which the host's lock records; the host declares the stream
/// `address` names; and acme declares `run` for `address` (132, 155,
/// 193).
pub fn configure_at(world: &World, address: &str, prompt: &str, run: RoleRun) {
    declare_turn(world, address, stage("plan", None, prompt), run);
}

/// As `configure_at`, `turn`'s one stage delivering the report
/// `{{state}}/turn.md` in place of a conversation, as a turn that ends
/// once it has acted does (114, 132).
fn configure_turn(world: &World, address: &str, run: RoleRun) {
    let plan = stage(
        "plan",
        Some(Deliverable::Report {
            path: "{{state}}/turn.md".into(),
        }),
        "Read what is pending for you; write {{report}}.",
    );
    declare_turn(world, address, plan, run);
}

/// The run's turn written: its report, a minute on, and the run settled.
fn deliver_turn(world: &World, run: &PlaceId) {
    world.host.advance(60);
    world
        .host
        .write_file(&format!("{}/turn.md", world.store.folder(run)));
    world.settle(run);
}

/// acme holds `turn@1` of the one stage `plan`, declaring the label
/// `planning` as a role's kind does (180), which the host's lock
/// records; the host declares the stream `address` names; and acme
/// declares `run` for `address` (132, 155, 193).
fn declare_turn(world: &World, address: &str, plan: Stage, run: RoleRun) {
    declare_turn_landing(world, address, plan, run, false);
}

/// As `declare_turn`, `turn` landing where `lands` (63, 114).
fn declare_turn_landing(world: &World, address: &str, plan: Stage, run: RoleRun, lands: bool) {
    use crate::seams::kinds::{entry, hash_of};
    let mut turn = kind("turn", lands, vec![plan]);
    turn.version = 1;
    turn.label = Some("planning".into());
    world.host.set_source(ACME, ACME_KINDS);
    world.kinds.set_package(ACME_KINDS, turn, &hash_of('a'));
    super::kind_sources::host_locks(world, vec![entry(ACME, "turn", 1, &hash_of('a'))]);
    world
        .host
        .set_stream(stream_of(address), Some(Stream::default()));
    world.host.set_role(address, Some(run));
}

/// acme declares `run` for the planner, as `configure_at` has it.
fn configure(world: &World, prompt: &str, run: RoleRun) {
    configure_at(world, PLANNER, prompt, run);
}

/// The planner's run, named `<stream>-<role>` as its entry names none.
fn planner_id() -> PlaceId {
    World::id("turn", "east-planner")
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

/// The role-waking spec's "A role configured": the scratch host declares
/// the stream `east`, acme declares `east/planner` in the repository of
/// its own kind `turn` at priority 5, `acme/turn@1` is locked, and a wake
/// launches a place of `acme/turn@1` there at priority 5, cut from the
/// repository's default branch, with no brief and no recipient; a name
/// of no address is refused (132, 154, 155, 193).
fn a_role_configured(_: &()) -> Outcome {
    let world = World::new();
    world.git.init(REPOSITORY, "trunk");
    world.git.set_default_branch(REPOSITORY, "trunk");
    configure_at(
        &world,
        "east/planner",
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let (woke, place) = woken(&world.engine.wake("east/planner"))?;
    ensure_eq!(woke, Woke::Launched, "the wake");
    ensure_eq!(runs(&world), vec![place.place.clone()], "the run's place");
    ensure_eq!(
        (place.kind.as_str(), place.base.as_str()),
        ("turn", "trunk"),
        "the run's kind and base"
    );
    let status = world.status(&place.place)?;
    ensure_eq!(
        status.identity.as_deref(),
        Some("acme/turn@1"),
        "the run's kind, its source's own at the version locked"
    );
    ensure_eq!(place.repository, REPOSITORY, "the run's repository");
    ensure_eq!(place.priority, 5, "the run's priority");
    ensure_eq!(
        place.role.as_deref(),
        Some("east/planner"),
        "the run's role"
    );
    ensure_eq!(
        place.recipients,
        Vec::<String>::new(),
        "the run's recipients"
    );
    let held = world.held(&place.place).ok_or("no run")?;
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
    let identity = World::identity("turn", "east-planner", "plan", 1);
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
    answerer_behind(&world, DEFAULT_STREAM);
    let id = fix_for(&world, "one", &["east/ops"])?;
    deliver_fix(&world, &id);
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    world.settle(&id);
    ensure_eq!(runs(&world), Vec::<PlaceId>::new(), "runs launched");
    ensure_eq!(world.store.places(), vec![id.clone()], "places on the host");
    let addresses = addresses(&world)?;
    ensure_eq!(addresses.len(), 1, "addresses status names");
    let ops = &addresses[0];
    ensure_eq!(ops.address, "east/ops", "the address");
    ensure_eq!(ops.role, None, "its role");
    ensure_eq!(ops.run, None, "its run");
    ensure_eq!(ops.pending.len(), 1, "the ends pending for it");
    ensure_eq!(ops.pending[0].what, "code delivered", "what ended");
    ensure!(
        ops.unwoken
            .as_deref()
            .is_some_and(|why| why.contains("no source of the host declares east/ops")),
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
    let (_, place) = woken(&world.engine.wake(PLANNER))?;
    ensure_eq!(
        (place.place.clone(), place.name.as_str()),
        (World::id("turn", "east-planner"), "east-planner"),
        "the run's place, named <stream>-<role>"
    );
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
    let opened = World::identity("turn", "east-planner", "plan", 1);
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
    // A watch whose bound holds it past the five minutes the stall takes.
    world.engine.settle_every(300);
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
    let identity = World::identity("turn", "east-planner", "plan", 1);
    ensure!(
        world.terminal.opened(&identity).is_some(),
        "the run's agent did not start once there was room"
    );
    // The run's command, whose environment names no recipient.
    world.host.set_recipient(None);
    let run = world.engine();
    let recipient = run
        .recipient(Some(&planner_id()))
        .ok_or("the run names no recipient")?;
    ensure_eq!(recipient, PLANNER, "the run's recipient");
    let collected = match run.inbox(&recipient, None) {
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
    world.engine.settle_every(30);
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
    // The run's command, whose environment names no recipient.
    world.host.set_recipient(None);
    let run = world.engine();
    let recipient = run
        .recipient(Some(&planner_id()))
        .ok_or("the run names no recipient")?;
    let ends = match run.inbox(&recipient, None) {
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
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let (woke, place) = woken(&world.engine.wake(PLANNER))?;
    ensure_eq!(woke, Woke::Launched, "the wake");
    ensure_eq!(
        place.stream,
        StreamHeld {
            name: "east".into(),
            from: StreamFrom::Launch,
        },
        "the run's stream, its address's"
    );
    ensure_eq!(
        place.label,
        Some(LabelHeld {
            name: "planning".into(),
            from: LabelFrom::Kind,
        }),
        "the run's label, its kind's"
    );
    world
        .host
        .set_role("nightly/ops", Some(planner_run(None, None)));
    let refused = world.engine.wake("nightly/ops");
    ensure!(
        refused
            .refusal()
            .is_some_and(|reason| reason.contains("nightly")),
        "a wake into an undeclared stream: {refused:?}"
    );
    ensure_eq!(runs(&world), vec![planner_id()], "the runs on the host");
    Ok(())
}

/// The planner's run of acme's own kind `turn`, with acme holding turn@1
/// and turn@2, in the stream `east` the host declares (132, 154, 191,
/// 193).
fn turn_planner(world: &World) {
    super::kind_sources::holds(
        world,
        ACME,
        ACME_KINDS,
        &[("turn", 1, '1'), ("turn", 2, '2')],
    );
    world.host.set_stream("east", Some(Stream::default()));
    world
        .host
        .set_role(PLANNER, Some(planner_run(None, Some("take your turn"))));
}

fn a_role_woken_to_a_hosts_source(_: &()) -> Outcome {
    use crate::seams::kinds::{entry, hash_of};
    let world = World::new();
    turn_planner(&world);
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
    let run = planner_id();
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
    turn_planner(&world);
    let refused = world.engine.wake(PLANNER);
    let reason = refused.refusal().ok_or("the wake was taken")?;
    let launched_as_a_launch =
        world
            .engine
            .launch(&launch("acme/turn", "planner", Some("take your turn")));
    let launch_refused = launched_as_a_launch
        .refusal()
        .ok_or("the launch was taken")?;
    ensure!(
        reason.contains("acme/turn") && reason.contains(launch_refused),
        "the refusal names not the kind as unlocked: {reason}"
    );
    ensure_eq!(world.store.places(), Vec::<PlaceId>::new(), "places made");
    ensure_eq!(world.place.log.count("make"), 0, "makes");
    Ok(())
}

/// acme gives the planner the standing agent `east-planner` in the
/// session `east`, of the stream `east` the host declares.
fn planner_stands(world: &World) {
    world.host.set_stream("east", Some(Stream::default()));
    world.host.set_standing(
        PLANNER,
        Some(StandingAgent {
            source: ACME.into(),
            agent: "east-planner".into(),
            session: Some("east".into()),
            answers: None,
            answerer: false,
        }),
    );
}

/// The notice a fix's code delivered is sent as.
fn code_delivered(name: &str) -> String {
    format!(
        "flume: fix--{name} (mend it) code delivered [stage/code/1/delivered]; read: flume status fix--{name}"
    )
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
    ensure_eq!(standing.agent, "east-planner", "the agent");
    ensure_eq!(
        standing.session,
        SessionHeld {
            name: Some("east".into()),
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
        .add_agent(Some("east"), "east-planner", AgentState::Waiting);
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
        vec![world.confirming(&code_delivered("one"))],
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
        .add_agent(Some("east"), "east-planner", AgentState::Waiting);
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    delivered_for_planner(&world, "one")?;
    ensure_eq!(
        world.terminal.notices(&at),
        vec![world.confirming(&code_delivered("one"))],
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
        .add_agent(Some("east"), "east-planner", AgentState::Waiting);
    world
        .queue
        .plan_submits(vec![flume_fakes::Plan::Hold, flume_fakes::Plan::Hold]);
    delivered_for_planner(&world, "one")?;
    ensure_eq!(
        world.terminal.notices(&at),
        vec![world.confirming(&code_delivered("one"))],
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

/// The host's sources as status gives them.
fn sources(world: &World) -> Result<Vec<flume_core::SourceStatus>, String> {
    match world.engine().status(None) {
        Answer::Status { host, .. } => Ok(host.sources),
        other => Err(format!("status answered {other:?}")),
    }
}

/// What status gives of `address` among the host's addresses.
fn address_of(world: &World, address: &str) -> Result<AddressStatus, String> {
    addresses(world)?
        .into_iter()
        .find(|status| status.address == address)
        .ok_or_else(|| format!("status names no address {address}"))
}

/// The role-waking spec's "A stream the host does not declare": acme's
/// entry for `south/planner` declares nothing, as the host's reader
/// words it, and its wake is refused saying so, no place made, while
/// status names the entry and the stream under acme (4, 143, 193).
fn a_stream_the_host_does_not_declare(_: &()) -> Outcome {
    let world = World::new();
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let why =
        "south/planner names the stream south, which the host does not declare, and is not carried";
    world.host.refuse_role("south/planner", ACME, why);
    let reason = refused(&world.engine.wake("south/planner"))?;
    ensure_eq!(
        reason,
        format!("south/planner wakes nothing: {why}"),
        "the wake's refusal"
    );
    ensure_eq!(world.store.places(), Vec::<PlaceId>::new(), "places made");
    let acme = sources(&world)?
        .into_iter()
        .find(|source| source.name == ACME)
        .ok_or("status names no source acme")?;
    ensure!(
        acme.roles_refused
            .iter()
            .any(|refused| refused.address == "south/planner" && refused.reason == why),
        "acme's entries that declare nothing: {:?}",
        acme.roles_refused
    );
    Ok(())
}

/// The role-waking spec's "The run's intent and settings" (32, 147, 193).
fn the_runs_intent_and_settings(_: &()) -> Outcome {
    let world = World::new();
    configure(
        &world,
        "Read what is pending for you.",
        RoleRun {
            intent: Some("east-planner".into()),
            agents: RoleAgents {
                every_stage: Settings {
                    model: Some("opus".into()),
                    ..Settings::default()
                },
                stages: BTreeMap::new(),
            },
            ..planner_run(None, None)
        },
    );
    woken(&world.engine.wake(PLANNER))?;
    world.settle(&planner_id());
    let status = world.status(&planner_id())?;
    ensure!(
        matches!(&status.intent, IntentKey::Launch { name, .. } if name == "east-planner"),
        "the run's intent: {:?}",
        status.intent
    );
    let agent = status.agent.ok_or("no agent")?;
    ensure_eq!(
        agent.settings.settings.model.as_deref(),
        Some("opus"),
        "the agent's model"
    );
    ensure_eq!(
        agent.settings.from.get("model").map(|given| given.layer),
        Some(Layer::Launch),
        "the model's layer"
    );
    Ok(())
}

/// The role-waking spec's "Two streams' planners from one declaration"
/// (S99): one roles file declares the planner of `east` and of `west`,
/// one repository and kind, named by their streams, and ends at once for
/// each wake a place each, whose inbox gives its own address's ends (3,
/// 132, 143, 193).
fn s99_one_declaration_two_streams(_: &()) -> Outcome {
    let world = World::new();
    world.keep_watch();
    let named = |name: &str| RoleRun {
        name: Some(name.into()),
        ..planner_run(None, None)
    };
    configure_at(
        &world,
        PLANNER,
        "Read what is pending for you.",
        named("east"),
    );
    world.host.set_stream("west", Some(Stream::default()));
    world.host.set_role("west/planner", Some(named("west")));
    world
        .queue
        .plan_submits(vec![flume_fakes::Plan::Hold, flume_fakes::Plan::Hold]);
    let one = fix_for(&world, "one", &[PLANNER])?;
    let two = fix_for(&world, "two", &["west/planner"])?;
    deliver_fix(&world, &one);
    deliver_fix(&world, &two);
    world.settle(&one);
    world.settle(&two);
    let east = World::id("turn", "east");
    let west = World::id("turn", "west");
    let mut made = runs(&world);
    made.sort();
    let mut due = vec![east.clone(), west.clone()];
    due.sort();
    ensure_eq!(made, due, "the runs launched");
    for (run, stream) in [(&east, "east"), (&west, "west")] {
        ensure_eq!(
            world.status(run)?.stream.name,
            stream,
            format!("{run}'s stream")
        );
    }
    // Each run's command, whose environment names no recipient.
    world.host.set_recipient(None);
    for (run, address, fix) in [(&east, PLANNER, &one), (&west, "west/planner", &two)] {
        let engine = world.engine();
        let recipient = engine
            .recipient(Some(run))
            .ok_or_else(|| format!("{run} names no recipient"))?;
        ensure_eq!(recipient, address, format!("{run}'s recipient"));
        let places = match engine.inbox(&recipient, None) {
            Answer::Inbox { ends, .. } => ends
                .into_iter()
                .map(|collected| collected.place)
                .collect::<Vec<_>>(),
            other => return Err(format!("the inbox answered {other:?}")),
        };
        ensure_eq!(
            places,
            vec![fix.clone()],
            format!("the ends {address}'s inbox gives")
        );
    }
    Ok(())
}

/// The role-waking spec's "A run an earlier flume named": a run of the
/// planner made as `turn--planner`, as an earlier flume named every run
/// by its role alone, its role the address, is found by its role once
/// the entry names no name, and the next end for the address goes into
/// its pane with no run launched (3, 132, 193).
fn a_run_an_earlier_flume_named(_: &()) -> Outcome {
    let world = World::new();
    world.keep_watch();
    configure(
        &world,
        "Read what is pending for you.",
        RoleRun {
            name: Some("planner".into()),
            ..planner_run(None, None)
        },
    );
    let (woke, _) = woken(&world.engine.wake(PLANNER))?;
    ensure_eq!(woke, Woke::Launched, "the earlier run");
    let earlier = World::id("turn", "planner");
    world.settle(&earlier);
    ensure_eq!(
        world.stage_state(&earlier, "plan"),
        Some(StageState::Begun),
        "the earlier run's stage"
    );
    world.host.set_role(PLANNER, Some(planner_run(None, None)));
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    let one = delivered_for_planner(&world, "one")?;
    ensure_eq!(runs(&world), vec![earlier.clone()], "the runs on the host");
    ensure_eq!(
        world.terminal.notices(PLANNER),
        vec![world.confirming(&code_delivered("one"))],
        "the notices into the earlier run's pane"
    );
    ensure_eq!(
        world.held(&one).map(|held| held.pending_for()),
        Some(vec![]),
        "the ends still pending"
    );
    let (woke, place) = woken(&world.engine.wake(PLANNER))?;
    ensure!(woke != Woke::Launched, "a wake launched a second run");
    ensure_eq!(place.place, earlier, "the run the wake names");
    Ok(())
}

/// The role-waking spec's "A run not yet given its prompt": the
/// planner's run, woken for a fix's end, its agent waiting in the
/// address's session before it says it started, takes no notice, which it
/// would take as its prompt, and the end stays pending; once it says it
/// started and its prompt is received, the next settle of the fix
/// delivers the end into its pane once (37, 38, 43, 132).
fn a_run_not_yet_given_its_prompt(_: &()) -> Outcome {
    let world = World::new();
    world.keep_watch();
    configure(
        &world,
        "Read what is pending for you.",
        planner_run(None, None),
    );
    let agent = World::identity("turn", "east-planner", "plan", 1);
    world.agent.set_started(&agent, Reading::No);
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    let one = delivered_for_planner(&world, "one")?;
    ensure_eq!(runs(&world), vec![planner_id()], "the run woken");
    world.settle(&planner_id());
    world.settle(&one);
    ensure_eq!(
        world.stage_state(&planner_id(), "plan"),
        Some(StageState::Starting),
        "the run's stage before its agent said it started"
    );
    ensure_eq!(
        world.terminal.notices(PLANNER),
        Vec::<String>::new(),
        "the notices into the run's pane before its prompt"
    );
    ensure_eq!(
        world.held(&one).map(|held| held.pending_for()),
        Some(vec![PLANNER.to_string()]),
        "the ends pending before the run's prompt"
    );
    world.agent.set_started(&agent, Reading::Yes);
    world.settle(&planner_id());
    ensure_eq!(
        world.stage_state(&planner_id(), "plan"),
        Some(StageState::Begun),
        "the run's stage once its prompt was received"
    );
    world.settle(&one);
    world.settle(&one);
    ensure_eq!(
        world.terminal.notices(PLANNER),
        vec![world.confirming(&code_delivered("one"))],
        "the notices into the run's pane"
    );
    ensure_eq!(
        world.held(&one).map(|held| held.pending_for()),
        Some(vec![]),
        "the ends pending once the run is begun"
    );
    Ok(())
}

/// The role-waking spec's "A carried address" (132, 193).
fn a_carried_address(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_carried(
        "north/planner",
        Some(CarriedRole {
            source: ACME.into(),
            answers: Some("The work's scope.".into()),
        }),
    );
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    let id = fix_for(&world, "one", &["north/planner"])?;
    deliver_fix(&world, &id);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(runs(&world), Vec::<PlaceId>::new(), "runs launched");
    let held = world.held(&id).ok_or("no place")?;
    ensure_eq!(
        held.pending_for(),
        vec!["north/planner".to_string()],
        "the ends pending"
    );
    ensure!(
        held.unwoken.is_empty(),
        "the place's unwoken: {:?}",
        held.unwoken
    );
    let north = address_of(&world, "north/planner")?;
    ensure_eq!(
        north.carried.map(|carried| carried.source),
        Some(ACME.to_string()),
        "the address carried"
    );
    ensure_eq!(north.pending.len(), 1, "its ends pending");
    let reason = refused(&world.engine.wake("north/planner"))?;
    ensure!(
        reason.contains("carried") && reason.contains(ACME),
        "the wake's refusal: {reason}"
    );
    ensure_eq!(world.store.places(), vec![id], "the places on the host");
    Ok(())
}

/// acme gives `east/design` the standing agent `east-design`, sought in
/// the stream `east`'s own session, and the fake terminal holds it there
/// waiting: where it is.
fn design_stands(world: &World) -> String {
    world.host.set_stream("east", Some(Stream::default()));
    world.host.set_standing(
        "east/design",
        Some(StandingAgent {
            source: ACME.into(),
            agent: "east-design".into(),
            session: None,
            answers: Some("A decision the design has not made.".into()),
            answerer: false,
        }),
    );
    world
        .terminal
        .add_agent(Some("east"), "east-design", AgentState::Waiting)
}

/// A fix named `name` for `east/design`, delivered with its submission
/// held, and settled.
fn delivered_for_design(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = fix_for(world, name, &["east/design"])?;
    deliver_fix(world, &id);
    world.settle(&id);
    Ok(id)
}

/// The role-waking spec's "A standing design agent" (132, 190, 193).
fn a_standing_design_agent(_: &()) -> Outcome {
    let world = World::new();
    let at = design_stands(&world);
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    let opens = world.terminal.log.count("open");
    let id = delivered_for_design(&world, "one")?;
    ensure_eq!(
        world.terminal.notices(&at),
        vec![world.confirming(&code_delivered("one"))],
        "the notices into east-design's pane"
    );
    ensure_eq!(runs(&world), Vec::<PlaceId>::new(), "runs launched");
    ensure_eq!(
        world.terminal.log.count("open"),
        opens + 1,
        "agents opened: the fix's alone"
    );
    ensure_eq!(world.store.places(), vec![id], "the places on the host");
    Ok(())
}

/// The role-waking spec's "The crossing to its kind": acme's file trades
/// `east/design`'s agent for its own kind `turn` and a repository, the
/// host's configuration unchanged, and the next end wakes a run (132,
/// 193; record 0039's amendment).
fn the_crossing_to_its_kind(_: &()) -> Outcome {
    let world = World::new();
    let at = design_stands(&world);
    world
        .queue
        .plan_submits(vec![flume_fakes::Plan::Hold, flume_fakes::Plan::Hold]);
    delivered_for_design(&world, "one")?;
    ensure_eq!(
        world.terminal.notices(&at),
        vec![world.confirming(&code_delivered("one"))],
        "east-design's notice before the crossing"
    );
    let streams = world.host.streams();
    world.host.set_standing("east/design", None);
    configure_at(
        &world,
        "east/design",
        "Read what is pending for you.",
        planner_run(None, None),
    );
    ensure_eq!(
        world.host.streams(),
        streams,
        "the host's streams, unchanged"
    );
    delivered_for_design(&world, "two")?;
    let run = World::id("turn", "east-design");
    ensure_eq!(runs(&world), vec![run.clone()], "the run woken");
    let status = world.status(&run)?;
    ensure_eq!(
        status.identity.as_deref(),
        Some("acme/turn@1"),
        "the run's kind"
    );
    ensure_eq!(status.stream.name, "east", "the run's stream");
    ensure_eq!(
        world.terminal.notices(&at).len(),
        1,
        "east-design's notices after the crossing"
    );
    Ok(())
}

/// The role-waking spec's "A standing door" (7, 130, 132, 193).
fn a_standing_door(_: &()) -> Outcome {
    let world = World::new();
    super::launching::from_no_session(&world);
    world.host.set_stream("east", Some(Stream::default()));
    world.host.set_standing(
        "east/operator",
        Some(StandingAgent {
            source: ACME.into(),
            agent: "east-operator".into(),
            session: None,
            answers: None,
            answerer: true,
        }),
    );
    let at = world
        .terminal
        .add_agent(Some("east"), "east-operator", AgentState::Waiting);
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.stream = Some("east".into());
    let place = launched(&world.engine.launch(&fix))?;
    ensure_eq!(
        place.recipients,
        vec!["east/operator".to_string()],
        "the place's recipients"
    );
    let id = World::id("fix", "one");
    world.settle(&id);
    asks(&world, &id)?;
    world.host.advance(5 * 60);
    world.settle(&id);
    ensure!(
        world
            .terminal
            .notices(&at)
            .iter()
            .any(|text| text.contains("fix--one (mend it) stalled: ")),
        "the stall in east-operator's pane: {:?}",
        world.terminal.notices(&at)
    );
    ensure_eq!(runs(&world), Vec::<PlaceId>::new(), "runs launched");
    let streams = match world.engine().status(None) {
        Answer::Status { host, .. } => host.streams,
        other => return Err(format!("status answered {other:?}")),
    };
    let east = streams
        .into_iter()
        .find(|stream| stream.name == "east")
        .ok_or("status names no stream east")?;
    ensure_eq!(
        east.answerer,
        Some(StreamAnswerer::Stands(DeclaredBy {
            address: "east/operator".into(),
            source: ACME.into(),
        })),
        "east's answerer"
    );
    Ok(())
}

/// The role-waking spec's "A fault in a source's roles stays its own" and
/// "What two entries claim is neither's": acme's entry for `east/ops`
/// names beta's kind and `east/planner` is declared by acme and by beta,
/// each declaring nothing as the host's reader words it; each wake is
/// refused saying why, an end pending for `east/ops` wakes nothing and
/// status says why, and acme's `west/ops` wakes its run (3, 4, 56, 132,
/// 191, 193).
fn a_faults_address(_: &()) -> Outcome {
    let world = World::new();
    configure_at(
        &world,
        "west/ops",
        "Read what is pending for you.",
        planner_run(None, None),
    );
    world.host.set_stream("east", Some(Stream::default()));
    answerer_behind(&world, DEFAULT_STREAM);
    let kind_fault = "[roles.\"east/ops\"] names the kind beta/turn, which is not acme's own: a role runs as one of its source's own kinds";
    world.host.refuse_role("east/ops", ACME, kind_fault);
    let both = "east/planner is declared by acme and by beta, so by neither";
    world.host.refuse_role(PLANNER, ACME, both);
    world.host.refuse_role(PLANNER, "beta", both);
    ensure_eq!(
        refused(&world.engine.wake("east/ops"))?,
        format!("east/ops wakes nothing: {kind_fault}"),
        "east/ops's wake"
    );
    ensure_eq!(
        refused(&world.engine.wake(PLANNER))?,
        format!("{PLANNER} wakes nothing: {both}"),
        "the planner's wake"
    );
    ensure_eq!(world.store.places(), Vec::<PlaceId>::new(), "places made");
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    let id = fix_for(&world, "one", &["east/ops"])?;
    deliver_fix(&world, &id);
    world.settle(&id);
    let ops = address_of(&world, "east/ops")?;
    ensure_eq!(
        ops.wakes_nothing.as_deref(),
        Some(kind_fault),
        "why east/ops wakes nothing"
    );
    ensure_eq!(
        ops.unwoken,
        Some(format!("east/ops wakes nothing: {kind_fault}")),
        "why the settle did not wake it"
    );
    let (woke, _) = woken(&world.engine.wake("west/ops"))?;
    ensure_eq!(woke, Woke::Launched, "west/ops's wake");
    ensure_eq!(
        runs(&world),
        vec![World::id("turn", "west-ops")],
        "the runs on the host"
    );
    Ok(())
}

/// The planner's run woken, begun, and its one stage's report written,
/// its agent quiet, and settled.
fn run_delivered(world: &World) -> Outcome {
    configure_turn(world, PLANNER, planner_run(None, None));
    let (woke, _) = woken(&world.engine.wake(PLANNER))?;
    ensure_eq!(woke, Woke::Launched, "the wake");
    world.settle(&planner_id());
    ensure_eq!(
        world.stage_state(&planner_id(), "plan"),
        Some(StageState::Begun),
        "the run's stage"
    );
    deliver_turn(world, &planner_id());
    Ok(())
}

/// The role-waking spec's "A turn delivers" (S99; 114, 132).
fn a_turn_delivers(_: &()) -> Outcome {
    let world = World::new();
    world.keep_watch();
    run_delivered(&world)?;
    ensure!(
        world.held(&planner_id()).is_none(),
        "the run's held state stands: {:?}",
        world.held(&planner_id()).map(|held| held.phase)
    );
    ensure_eq!(world.place.log.count("remove"), 1, "places removed");
    ensure_eq!(runs(&world), Vec::<PlaceId>::new(), "runs standing");
    ensure!(
        addresses(&world)?
            .iter()
            .all(|address| address.address != PLANNER),
        "status names a run of {PLANNER} standing"
    );
    Ok(())
}

/// The role-waking spec's "The next end" (S99; 114, 132).
fn the_next_end(_: &()) -> Outcome {
    let world = World::new();
    world.keep_watch();
    run_delivered(&world)?;
    ensure_eq!(runs(&world), Vec::<PlaceId>::new(), "runs once tidied");
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    let one = delivered_for_planner(&world, "one")?;
    ensure_eq!(runs(&world), vec![planner_id()], "the new run");
    ensure!(
        world.held(&one).is_some_and(|held| held.unwoken.is_empty()),
        "the end's wake was named failed"
    );
    ensure_eq!(
        world.stage_state(&planner_id(), "plan"),
        Some(StageState::Begun),
        "the new run's stage"
    );
    Ok(())
}

/// The role-waking spec's "A run whose kind lands" (63, 114, 132): a
/// done is refused a place whose kind lands (114), so the run is
/// submitted as any place's at its delivery, and tidied at its landing.
fn a_run_whose_kind_lands(_: &()) -> Outcome {
    let world = World::new();
    world.keep_watch();
    let plan = stage(
        "plan",
        Some(Deliverable::Report {
            path: "{{state}}/turn.md".into(),
        }),
        "Read what is pending for you; write {{report}}.",
    );
    declare_turn_landing(&world, PLANNER, plan, planner_run(None, None), true);
    world.queue.plan_submits(vec![flume_fakes::Plan::Hold]);
    let (woke, _) = woken(&world.engine.wake(PLANNER))?;
    ensure_eq!(woke, Woke::Launched, "the wake");
    world.settle(&planner_id());
    deliver_turn(&world, &planner_id());
    let held = world
        .held(&planner_id())
        .ok_or("the run is gone before its landing")?;
    ensure_eq!(held.phase, Phase::Live, "the run once submitted");
    let branch = held.place.branch.clone();
    ensure_eq!(
        world.queue.submissions(REPOSITORY, &branch).len(),
        1,
        "the run's submissions"
    );
    ensure_eq!(runs(&world), vec![planner_id()], "runs while it is open");
    world
        .queue
        .decide(REPOSITORY, &branch, flume_fakes::Plan::Land);
    world.settle(&planner_id());
    ensure!(
        world.held(&planner_id()).is_none(),
        "the landed run's held state stands: {:?}",
        world.held(&planner_id()).map(|held| held.phase)
    );
    ensure_eq!(runs(&world), Vec::<PlaceId>::new(), "runs once it landed");
    Ok(())
}

/// The run of `address`, of the turn delivering a report, woken and
/// begun, then lost and left past the stall's bound, and settled: its
/// place.
fn run_lost_and_stalled(world: &World, address: &str, run: RoleRun) -> Result<PlaceId, String> {
    configure_turn(world, address, run);
    let (woke, place) = woken(&world.engine.wake(address))?;
    ensure_eq!(woke, Woke::Launched, "the wake");
    let id = place.place;
    world.settle(&id);
    let name = id.0.rsplit("--").next().unwrap_or_default().to_string();
    world
        .terminal
        .kill(&World::identity("turn", &name, "plan", 1));
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "plan"),
        Some(StageState::Lost),
        "the run's stage"
    );
    world.host.advance(5 * 60);
    world.settle(&id);
    Ok(id)
}

/// The stall `id` records, the newest.
fn stall_of(world: &World, id: &PlaceId) -> Result<EndHeld, String> {
    world
        .held(id)
        .ok_or("no run")?
        .ends
        .into_iter()
        .rev()
        .find(|end| matches!(end.of, EndOf::Stall { .. }))
        .ok_or_else(|| format!("{id} records no stall"))
}

/// The role-waking spec's "A planner's run is lost" (130, 132, 193).
fn a_planners_run_is_lost(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_role(
        "east/operator",
        Some(RoleRun {
            answerer: true,
            ..planner_run(None, None)
        }),
    );
    let id = run_lost_and_stalled(&world, PLANNER, planner_run(None, None))?;
    let stall = stall_of(&world, &id)?;
    ensure!(
        matches!(&stall.of, EndOf::Stall { answerer: Some(answerer), .. } if answerer == "east/operator"),
        "the stall's answerer: {:?}",
        stall.of
    );
    let held = world.held(&id).ok_or("no run")?;
    ensure!(
        !held.pending_for().contains(&PLANNER.to_string())
            && stall.recipients(&held.place.recipients) == vec!["east/operator".to_string()],
        "the stall's recipients: {:?}",
        stall.recipients(&held.place.recipients)
    );
    let operator = World::id("turn", "east-operator");
    let mut made = runs(&world);
    made.sort();
    let mut due = vec![operator.clone(), id.clone()];
    due.sort();
    ensure_eq!(
        made,
        due,
        "the runs: the planner's and the operator's woken"
    );
    ensure!(
        world.terminal.notices(PLANNER).is_empty(),
        "a notice into the planner's own pane"
    );
    ensure_eq!(
        held.pending_for(),
        vec!["east/operator".to_string()],
        "the ends pending once the stall is read"
    );
    // The host's places settled, the operator's run's agent up.
    world.engine().settle(None);
    ensure_eq!(
        world.terminal.notices("east/operator"),
        vec![world.confirming(
            "flume: turn--east-planner stalled: a retry of plan [stall/1000000]; read: flume inbox"
        )],
        "the notices into the operator's run's pane"
    );
    ensure_eq!(
        stall_of(&world, &id)?
            .deliveries
            .into_iter()
            .map(|delivery| (delivery.recipient, delivery.by))
            .collect::<Vec<_>>(),
        vec![("east/operator".to_string(), DeliveredBy::Notice)],
        "the stall's deliveries"
    );
    Ok(())
}

/// The role-waking spec's "The door's own run" (56, 130, 132, 193).
fn the_doors_own_run(_: &()) -> Outcome {
    let world = World::new();
    let door = run_lost_and_stalled(
        &world,
        "east/operator",
        RoleRun {
            answerer: true,
            ..planner_run(None, None)
        },
    )?;
    let stall = stall_of(&world, &door)?;
    ensure!(
        matches!(&stall.of, EndOf::Stall { answerer: None, .. }),
        "the stall's answerer: {:?}",
        stall.of
    );
    let held = world.held(&door).ok_or("no run")?;
    ensure_eq!(
        stall.recipients(&held.place.recipients),
        Vec::<String>::new(),
        "the stall's recipients"
    );
    ensure_eq!(runs(&world), vec![door.clone()], "the runs on the host");
    ensure!(
        world
            .terminal
            .every_notice()
            .iter()
            .all(|(_, text)| !text.contains(" stalled: ")),
        "a stall notice sent: {:?}",
        world.terminal.every_notice()
    );
    let operator = address_of(&world, "east/operator")?;
    ensure_eq!(operator.run, Some(door.clone()), "the address's run");
    ensure_eq!(
        operator.stalled.map(|stall| (stall.place, stall.key)),
        Some((door, stall.of.end_key())),
        "the stall status names under the address"
    );
    Ok(())
}

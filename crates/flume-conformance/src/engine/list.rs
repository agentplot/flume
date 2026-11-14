//! flume's list (200; core.52): a line for each place on the host under
//! its repository, a place that landed or was dropped only where every
//! place is asked, a tidied place whose document stays among them then,
//! in the words its own line gives it, with whom a waiting place waits on
//! and what keeps a place whose work is over; and a line for each stream
//! the host declares with whether each of its sessions runs, its own and
//! each it names for a kind or a stage. It reads each place's held
//! state, each live agent's asking mark and each stream's terminal once
//! by runs, and nothing else; it settles, starts, sends and writes
//! nothing, and gives none of a place's long words (56, 57, 62, 63, 64
//! to 66, 68, 70, 90, 107, 111, 130, 139, 186, 190, 199, 201).

use super::account_pools::begun_fix;
use super::answers::missing;
use super::caller_waits::construct_asks;
use super::carried_conversations::{by_hand, retry_talk, talk, talking};
use super::place_line::{code_of_change, fix_for_a_role, TWO_GROUPS};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, Listed, SessionListed, StreamListed, WaitsOn};
use flume_fakes::place_location;
use flume_seams::*;
use std::collections::BTreeMap;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "list: a fix whose code agent works and a change whose construct is on hold give `change--two · construct · held · waits on its caller` and `fix--one · code · working` under their repository",
            cites: &["47", "56", "130", "200"],
            run: a_working_fix_and_a_construct_on_hold,
        },
        Scenario {
            name: "list: a change@2 place at its code stage's second group names its stage with no group, run or tasks, and a place being made names no stage",
            cites: &["140", "200"],
            run: a_repeating_stage_and_a_place_being_made,
        },
        Scenario {
            name: "list: places of the repositories /b and /a come /a's first, each repository's by kind and name",
            cites: &["200"],
            run: places_of_two_repositories,
        },
        Scenario {
            name: "list: a place stalled on its caller, one waiting for room, one submitted and one whose agent asks say whom they wait on, the asking one nobody",
            cites: &["42", "70", "90", "111", "130", "200"],
            run: the_caller_the_host_and_the_merge_queue,
        },
        Scenario {
            name: "list: places that landed or were dropped, kept, being tidied or tidied, are left out and a done place and a live one listed; asked for every place, the list gives each",
            cites: &["62", "63", "200"],
            run: landed_and_dropped_places_left_out_unless_every_place_is_asked,
        },
        Scenario {
            name: "list: a landed chore whose teardown failed at release with forty lines of words is left out, and asked for every place reads `kept by release`, neither its line nor the JSON holding a word of the teardown's",
            cites: &["63", "66", "200"],
            run: a_teardown_that_failed,
        },
        Scenario {
            name: "list: a dropped place holding a file, a done place whose agent runs and a landed place's tidied document say in a word what keeps them, the done one alone listed unless every place is asked",
            cites: &["62", "63", "64", "65", "200"],
            run: holds_a_live_agent_and_a_tidied_document,
        },
        Scenario {
            name: "list: work, office naming desk, and default over the fake terminal read runs, not running and not read, one runs each",
            cites: &["107", "186", "201"],
            run: three_streams_over_the_fake_terminal,
        },
        Scenario {
            name: "list: work naming work-design for research and release's review, and work for fix, gives `work · session work · runs · session work-design · not running`, read by the one runs `runs work work-design`",
            cites: &["199", "200", "201"],
            run: a_streams_sessions_for_its_kinds_and_stages,
        },
        Scenario {
            name: "list: a stream whose multiplexer is cmux reads `no profile for cmux`, no terminal asked of it, the other streams' lines standing and the list not refused",
            cites: &["190", "200"],
            run: a_multiplexer_flume_carries_no_profile_for,
        },
        Scenario {
            name: "list: S108, fifty places in three repositories and three streams, the two landed left out unless every place is asked, read from what flume keeps alone, nothing settled, started or written",
            cites: &["S108", "57", "62", "63", "66", "70", "130", "200", "201"],
            run: s108_the_list_of_fifty_places,
        },
        Scenario {
            name: "list: no stop-short's, hold's, readiness step's, failed submission's or agent's last words are in the list's JSON, and status gives each",
            cites: &["12", "42", "47", "112", "200"],
            run: every_long_word_left_to_status,
        },
        Scenario {
            name: "list: a list of a place waiting on the host, a place kept by its teardown, a stream that runs and one uncarried reads back unchanged, waits_on reading host, and the field list holds every field, failing naming the answer and the field where one is dropped",
            cites: &["60", "61", "200"],
            run: the_list_round_trips_and_holds_its_fields,
        },
        Scenario {
            name: "list: a place whose lost talk waits on its session ends `waiting on its session · waits on its session`, the list asking no agent program whether the session runs",
            cites: &["49", "200", "207"],
            run: a_talk_waiting_on_its_session,
        },
    ]
}

/// The list as a fresh engine gives it, as the next command asks it, a
/// place that landed or was dropped left out.
fn list(world: &World) -> Result<(Vec<Listed>, Vec<StreamListed>), String> {
    listed(world, false)
}

/// The list of every place, as `flume list --all` asks it.
fn every(world: &World) -> Result<(Vec<Listed>, Vec<StreamListed>), String> {
    listed(world, true)
}

fn listed(world: &World, every: bool) -> Result<(Vec<Listed>, Vec<StreamListed>), String> {
    match world.engine().list(every) {
        Answer::List { places, streams } => Ok((places, streams)),
        other => Err(format!("the list answered {other:?}")),
    }
}

fn lines(places: &[Listed]) -> Vec<String> {
    places.iter().map(|place| place.line.clone()).collect()
}

/// The line of the place named `<kind>--<name>` in `repository`.
fn line_of(places: &[Listed], repository: &str, name: &str) -> Result<String, String> {
    places
        .iter()
        .find(|place| place.repository == repository && place.name == name)
        .map(|place| place.line.clone())
        .ok_or_else(|| format!("no line of {name} in {repository}: {:?}", lines(places)))
}

/// A copy of `from`'s held state written as the place `<kind>--<name>`
/// of `repository`, where the fake place would make it, as a store holds
/// a host's documents.
fn copied(world: &World, from: &PlaceId, repository: &str, name: &str) -> Result<PlaceId, String> {
    let mut held = world.held(from).ok_or("no place to copy")?;
    held.place.repository = repository.into();
    held.place.name = name.into();
    held.place.branch = format!("{}/{name}", held.place.kind.branch_prefix);
    held.place.location = place_location(repository, &held.place.short());
    held.place.id = PlaceId::new(held.place.location.clone());
    let id = held.place.id.clone();
    world.store.put(&id, held);
    Ok(id)
}

/// The place's held state set to `phase`, as the core would leave it.
fn phased(world: &World, id: &PlaceId, phase: Phase) -> Outcome {
    let mut held = world.held(id).ok_or("no place")?;
    held.phase = phase;
    world.store.put(id, held);
    Ok(())
}

/// A landed chore named `name` whose teardown fails at `release` with
/// forty lines of words, its document kept by its teardown.
fn chore_kept_by_its_teardown(world: &World, name: &str) -> Result<(PlaceId, String), String> {
    launched(&world.engine.launch(&launch("chore", name, Some("x"))))?;
    let id = World::id("chore", name);
    world.settle(&id);
    let words = (1..=40)
        .map(|line| format!("the configuration file could not be read at line {line}"))
        .collect::<Vec<_>>()
        .join("\n");
    world.place.fail_teardown(&id.0, "release", &words);
    world.commit(&format!("chore/{name}"), &["chore.rs"]);
    world.settle(&id);
    world.settle(&id);
    let held = world.held(&id).ok_or("no chore")?;
    ensure!(
        matches!(&held.phase, Phase::Over { over: Over::Landed { .. }, kept: Some(Kept::Teardown { step, .. }) } if step == "release"),
        "the chore was not kept by its teardown: {:?}",
        held.phase
    );
    Ok((id, words))
}

/// A fix named `name` landed and tidied, its document kept for an end
/// pending for a role's address no run collects.
fn landed_and_tidied(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = fix_for_a_role(world, name)?;
    world.commit(&format!("fix/{name}"), &["fix.rs"]);
    world.settle(&id);
    world.settle(&id);
    ensure!(
        world
            .store
            .read_every()
            .iter()
            .any(|read| read.state.place.location == id.0
                && matches!(read.state.phase, Phase::Tidied { .. })),
        "no tidied document of {name} kept for its end pending"
    );
    Ok(id)
}

/// A fix named `name` whose code stage waits for room on the host.
fn waiting_for_room(world: &World, name: &str) -> Result<PlaceId, String> {
    launched(&world.engine.launch(&launch("fix", name, Some("x"))))?;
    let id = World::id("fix", name);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::WaitingForRoom),
        "the code stage of the fix waiting for room"
    );
    Ok(id)
}

/// A fix named `name` whose code delivered and was submitted, the merge
/// queue yet to decide it.
fn submitted(world: &World, name: &str) -> Result<PlaceId, String> {
    world.queue.moves_on_its_own();
    let id = begun_fix(world, name)?;
    let before = world.queue.log.count("submit");
    world.commit(&format!("fix/{name}"), &["fix.rs"]);
    world.settle(&id);
    ensure_eq!(
        world.queue.log.count("submit"),
        before + 1,
        "the submits of the fix"
    );
    Ok(id)
}

fn a_working_fix_and_a_construct_on_hold(_: &()) -> Outcome {
    let world = World::new();
    begun_fix(&world, "one")?;
    launched(&world.engine.launch(&Launch {
        hold: vec!["construct".into()],
        ..launch("change", "two", Some("plan it"))
    }))?;
    world.settle(&World::id("change", "two"));
    let (places, _) = list(&world)?;
    ensure_eq!(
        lines(&places),
        vec![
            "change--two · construct · held · waits on its caller".to_string(),
            "fix--one · code · working".to_string(),
        ],
        "the places' lines"
    );
    ensure!(
        places.iter().all(|place| place.repository == REPOSITORY),
        "a place not under {REPOSITORY}: {places:?}"
    );
    ensure_eq!(
        places
            .iter()
            .map(|place| place.kind.as_str())
            .collect::<Vec<_>>(),
        vec!["change@3", "fix@1"],
        "the kinds"
    );
    ensure_eq!(
        places[0].waits_on,
        Some(WaitsOn::Caller),
        "whom the construct waits on"
    );
    Ok(())
}

fn a_repeating_stage_and_a_place_being_made(_: &()) -> Outcome {
    let world = World::new();
    let id = code_of_change(&world, "parser", TWO_GROUPS)?;
    world.tick_groups("change/parser", "parser", TWO_GROUPS, 2);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.status(&id)?.line.stage,
        Some("code 2/2".to_string()),
        "the stage its own line names"
    );
    launched(&world.engine.launch(&launch("fix", "made", Some("x"))))?;
    let made = World::id("fix", "made");
    phased(
        &world,
        &made,
        Phase::Making {
            by: "flume launch".into(),
        },
    )?;
    let (places, _) = list(&world)?;
    ensure_eq!(
        line_of(&places, REPOSITORY, "change--parser")?,
        "change--parser · code · working",
        "the repeating stage's line"
    );
    let made = places
        .iter()
        .find(|place| place.name == "fix--made")
        .ok_or("no line of the place being made")?;
    ensure_eq!(
        (made.stage.as_deref(), made.line.as_str()),
        (None, "fix--made · being made"),
        "the place being made"
    );
    Ok(())
}

fn places_of_two_repositories(_: &()) -> Outcome {
    let world = World::new();
    let one = begun_fix(&world, "one")?;
    copied(&world, &one, "/b", "one")?;
    copied(&world, &one, "/a", "two")?;
    copied(&world, &one, "/a", "one")?;
    let version = world.store.read(&one).map(|read| read.version);
    world.store.write(&one, version, None);
    let store = world.store.read_every();
    let (places, _) = list(&world)?;
    ensure_eq!(
        places
            .iter()
            .map(|place| (place.repository.as_str(), place.name.as_str()))
            .collect::<Vec<_>>(),
        vec![("/a", "fix--one"), ("/a", "fix--two"), ("/b", "fix--one")],
        "the places' order"
    );
    ensure_eq!(store.len(), 3, "the documents the store holds");
    Ok(())
}

fn the_caller_the_host_and_the_merge_queue(_: &()) -> Outcome {
    let world = World::new();
    let (plan, _) = construct_asks(&world, "plan", &[], None)?;
    world.host.advance(STALL_AFTER);
    world.settle(&plan);
    submitted(&world, "queued")?;
    begun_fix(&world, "ask")?;
    world
        .agent
        .set_asking(&World::identity("fix", "ask", "code", 1), Reading::Yes);
    world.host.set_bound(Some(1));
    waiting_for_room(&world, "room")?;
    let (places, _) = list(&world)?;
    ensure_eq!(
        lines(&places),
        vec![
            "change--plan · construct · stalled · waits on its caller".to_string(),
            "fix--ask · code · asking".to_string(),
            "fix--queued · code · submitted · waits on the merge queue".to_string(),
            "fix--room · code · waiting for room · waits on the host".to_string(),
        ],
        "the places' lines"
    );
    ensure_eq!(
        places
            .iter()
            .map(|place| place.waits_on)
            .collect::<Vec<_>>(),
        vec![
            Some(WaitsOn::Caller),
            None,
            Some(WaitsOn::MergeQueue),
            Some(WaitsOn::Host)
        ],
        "whom each waits on"
    );
    Ok(())
}

fn a_teardown_that_failed(_: &()) -> Outcome {
    let world = World::new();
    let (_, words) = chore_kept_by_its_teardown(&world, "one")?;
    let (places, _) = list(&world)?;
    ensure_eq!(lines(&places), Vec::<String>::new(), "the places listed");
    let answer = world.engine().list(true);
    let Answer::List { places, .. } = &answer else {
        return Err(format!("the list answered {answer:?}"));
    };
    ensure_eq!(
        lines(places),
        vec!["chore--one · landed · kept by release".to_string()],
        "the places' lines"
    );
    ensure_eq!(
        places[0].kept.as_deref(),
        Some("kept by release"),
        "what keeps it"
    );
    let json = answer.to_json();
    for line in words.lines() {
        ensure!(!json.contains(line), "the JSON holds {line:?}");
    }
    ensure!(
        !json.contains("could not be read"),
        "the JSON holds the teardown's words: {json}"
    );
    Ok(())
}

fn holds_a_live_agent_and_a_tidied_document(_: &()) -> Outcome {
    let world = World::new();
    world.engine.settle_every(30);
    let dropped = begun_fix(&world, "drop")?;
    world.place.set_holds(
        &dropped.0,
        Holds {
            untracked: vec!["scratch.txt".into()],
            ..Holds::default()
        },
    );
    let drop = world.engine.drop_place(&dropped, true);
    ensure!(drop.refusal().is_none(), "the drop was refused: {drop:?}");
    world.engine.tidy(Some(&dropped));
    let done = world.begun("notes", Some(Deliverable::Commits), "done")?;
    world.terminal.set_state(
        &World::identity("notes", "done", "work", 1),
        AgentState::Working,
    );
    phased(
        &world,
        &done,
        Phase::Over {
            over: Over::Done,
            kept: None,
        },
    )?;
    world.engine.tidy(Some(&done));
    landed_and_tidied(&world, "landed")?;
    let (places, _) = list(&world)?;
    ensure_eq!(
        lines(&places),
        vec!["notes--done · done · kept while its agent runs".to_string()],
        "the places' lines"
    );
    let (places, _) = every(&world)?;
    ensure_eq!(
        lines(&places),
        vec![
            "fix--drop · dropped · kept holding work".to_string(),
            "fix--landed · landed · tidied".to_string(),
            "notes--done · done · kept while its agent runs".to_string(),
        ],
        "every place's line"
    );
    Ok(())
}

fn landed_and_dropped_places_left_out_unless_every_place_is_asked(_: &()) -> Outcome {
    let world = World::new();
    let live = begun_fix(&world, "live")?;
    let landed = || Over::Landed {
        commit: Revision::new("c1"),
    };
    for (name, phase) in [
        (
            "landed",
            Phase::Over {
                over: landed(),
                kept: None,
            },
        ),
        (
            "dropped",
            Phase::Over {
                over: Over::Dropped,
                kept: Some(Kept::Holds {
                    holds: Holds {
                        untracked: vec!["scratch.txt".into()],
                        ..Holds::default()
                    },
                }),
            },
        ),
        (
            "going",
            Phase::Gone {
                over: landed(),
                at: None,
            },
        ),
        (
            "tidied",
            Phase::Tidied {
                over: Over::Dropped,
                at: 1,
                pending_for: vec!["operator".into()],
            },
        ),
        (
            "done",
            Phase::Over {
                over: Over::Done,
                kept: None,
            },
        ),
    ] {
        let id = copied(&world, &live, REPOSITORY, name)?;
        phased(&world, &id, phase)?;
    }
    let (places, _) = list(&world)?;
    ensure_eq!(
        lines(&places),
        vec![
            "fix--done · done".to_string(),
            "fix--live · code · working".to_string(),
        ],
        "the places' lines"
    );
    let (places, _) = every(&world)?;
    ensure_eq!(
        lines(&places),
        vec![
            "fix--done · done".to_string(),
            "fix--dropped · dropped · kept holding work".to_string(),
            "fix--going · landed · being tidied".to_string(),
            "fix--landed · landed".to_string(),
            "fix--live · code · working".to_string(),
            "fix--tidied · dropped · tidied".to_string(),
        ],
        "every place's line"
    );
    Ok(())
}

fn three_streams_over_the_fake_terminal(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    world.host.set_stream_session("office", Some("desk"));
    world.terminal.runs_as(
        "work",
        SessionRuns::Runs {
            name: "work".into(),
        },
    );
    world.terminal.runs_as(
        "desk",
        SessionRuns::NotRunning {
            name: "desk".into(),
        },
    );
    world.terminal.runs_as(
        "own",
        SessionRuns::Unread {
            reason: "the terminal did not answer".into(),
        },
    );
    let (_, streams) = list(&world)?;
    ensure_eq!(
        streams
            .iter()
            .map(|stream| stream.line.as_str())
            .collect::<Vec<_>>(),
        vec![
            "default · its terminal's own session · not read",
            "office · session desk · not running",
            "work · session work · runs",
        ],
        "the streams' lines, by priority and then by name"
    );
    ensure_eq!(
        world.terminal.log.of("runs"),
        vec![
            "runs own".to_string(),
            "runs desk".to_string(),
            "runs work".to_string()
        ],
        "the terminal's runs"
    );
    ensure_eq!(
        streams
            .iter()
            .map(|stream| stream.session.clone())
            .collect::<Vec<_>>(),
        vec![
            SessionHeld {
                name: None,
                from: SessionFrom::Own,
            },
            SessionHeld {
                name: Some("desk".into()),
                from: SessionFrom::Stream,
            },
            SessionHeld {
                name: Some("work".into()),
                from: SessionFrom::StreamName,
            },
        ],
        "the sessions resolved as an open resolves them"
    );
    ensure_eq!(
        streams[0].runs,
        Some(SessionRuns::Unread {
            reason: "the terminal did not answer".into(),
        }),
        "the own session's runs, in the terminal's words"
    );
    Ok(())
}

fn a_streams_sessions_for_its_kinds_and_stages(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    let design = || Some("work-design".to_string());
    world.host.set_kind_sessions(
        "work",
        "research",
        Some(KindSessions {
            session: design(),
            ..KindSessions::default()
        }),
    );
    world.host.set_kind_sessions(
        "work",
        "release",
        Some(KindSessions {
            stages: BTreeMap::from([(
                "review".to_string(),
                StageSession {
                    session: design(),
                    ..StageSession::default()
                },
            )]),
            ..KindSessions::default()
        }),
    );
    world.host.set_kind_sessions(
        "work",
        "fix",
        Some(KindSessions {
            session: Some("work".into()),
            ..KindSessions::default()
        }),
    );
    world.terminal.runs_as(
        "work",
        SessionRuns::Runs {
            name: "work".into(),
        },
    );
    let (_, streams) = list(&world)?;
    let work = streams
        .iter()
        .find(|stream| stream.stream == "work")
        .ok_or("no line for work")?;
    ensure_eq!(
        work.line.as_str(),
        "work · session work · runs · session work-design · not running",
        "work's line"
    );
    ensure_eq!(
        work.sessions.clone(),
        vec![SessionListed {
            name: "work-design".into(),
            runs: Some(SessionRuns::NotRunning {
                name: "work-design".into(),
            }),
        }],
        "work's sessions"
    );
    let json = serde_json::to_value(work).map_err(|error| error.to_string())?;
    ensure_eq!(
        json["sessions"][0]["runs"]["not_running"]["name"].as_str(),
        Some("work-design"),
        "work-design's runs in the JSON"
    );
    let logged = world.terminal.log.of("runs");
    ensure!(
        logged.contains(&"runs work work-design".to_string()),
        "the terminal's runs: {logged:?}"
    );
    ensure_eq!(logged.len(), 2, "the terminal's runs, one for each stream");
    Ok(())
}

fn a_multiplexer_flume_carries_no_profile_for(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    world.host.set_display("trial", Some("cmux"), None, None);
    let answer = world.engine().list(false);
    ensure!(
        answer.refusal().is_none(),
        "the list was refused: {answer:?}"
    );
    let (_, streams) = list(&world)?;
    ensure_eq!(
        streams
            .iter()
            .map(|stream| stream.line.as_str())
            .collect::<Vec<_>>(),
        vec![
            "default · session own · not running",
            "trial · session trial · no profile for cmux",
            "work · session work · not running",
        ],
        "the streams' lines"
    );
    let trial = &streams[1];
    ensure_eq!(trial.runs, None, "trial's runs");
    let uncarried = trial.uncarried.as_deref().ok_or("trial names no reason")?;
    ensure!(
        uncarried.contains("cmux") && uncarried.contains("fake"),
        "trial's reason names not cmux and the profiles carried: {uncarried}"
    );
    ensure!(
        !world
            .terminal
            .log
            .of("runs")
            .contains(&"runs trial".to_string()),
        "the terminal was asked of trial"
    );
    Ok(())
}

fn s108_the_list_of_fifty_places(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    world.host.set_stream("office", Some(Stream::default()));
    world.terminal.runs_as(
        "work",
        SessionRuns::Runs {
            name: "work".into(),
        },
    );
    chore_kept_by_its_teardown(&world, "kept")?;
    landed_and_tidied(&world, "tidied")?;
    submitted(&world, "queued")?;
    let working = begun_fix(&world, "work")?;
    begun_fix(&world, "ask")?;
    world
        .agent
        .set_asking(&World::identity("fix", "ask", "code", 1), Reading::Yes);
    launched(&world.engine.launch(&Launch {
        hold: vec!["code".into()],
        ..launch("fix", "held", Some("x"))
    }))?;
    world.settle(&World::id("fix", "held"));
    world.host.set_bound(Some(1));
    waiting_for_room(&world, "room")?;
    let repositories = [REPOSITORY, "/r/boats", "/r/docks"];
    for copy in 1..=43 {
        let repository = repositories[copy % 3];
        copied(&world, &working, repository, &format!("work-{copy:02}"))?;
    }
    let versions = |world: &World| -> BTreeMap<PlaceId, u64> {
        world
            .store
            .read_every()
            .into_iter()
            .map(|read| (read.state.place.id, read.version))
            .collect()
    };
    let before = versions(&world);
    for log in [
        &world.change.log,
        &world.queue.log,
        &world.history.log,
        &world.place.log,
        &world.terminal.log,
        &world.store.log,
    ] {
        log.clear();
    }
    let (places, streams) = list(&world)?;
    ensure_eq!(places.len(), 48, "the places listed");
    for name in ["chore--kept", "fix--tidied"] {
        ensure!(
            !places.iter().any(|place| place.name == name),
            "the landed {name} listed: {:?}",
            lines(&places)
        );
    }
    let mut at: Vec<&str> = places
        .iter()
        .map(|place| place.repository.as_str())
        .collect();
    let order = at.clone();
    at.sort();
    ensure_eq!(order, at, "the places under their repositories, in order");
    at.dedup();
    ensure_eq!(at.len(), 3, "the repositories");
    for (name, line) in [
        (
            "fix--queued",
            "fix--queued · code · submitted · waits on the merge queue",
        ),
        ("fix--work", "fix--work · code · working"),
        ("fix--ask", "fix--ask · code · asking"),
        ("fix--held", "fix--held · code · held · waits on its caller"),
        (
            "fix--room",
            "fix--room · code · waiting for room · waits on the host",
        ),
    ] {
        ensure_eq!(line_of(&places, REPOSITORY, name)?, line, "{name}'s line");
    }
    ensure_eq!(
        line_of(&places, "/r/docks", "fix--work-02")?,
        "fix--work-02 · code · working",
        "a copy's line"
    );
    ensure_eq!(
        streams
            .iter()
            .map(|stream| stream.line.as_str())
            .collect::<Vec<_>>(),
        vec![
            "default · session own · not running",
            "office · session office · not running",
            "work · session work · runs",
        ],
        "the streams' lines"
    );
    ensure_eq!(
        world.change.log.calls(),
        Vec::<String>::new(),
        "the change's reads"
    );
    ensure_eq!(
        world.queue.log.calls(),
        Vec::<String>::new(),
        "the merge queue's reads"
    );
    ensure_eq!(
        world.history.log.calls(),
        Vec::<String>::new(),
        "the history's reads"
    );
    ensure_eq!(
        world.place.log.calls(),
        Vec::<String>::new(),
        "the place seam's reads"
    );
    ensure_eq!(
        world.terminal.log.calls(),
        vec![
            "runs own".to_string(),
            "runs office".to_string(),
            "runs work".to_string()
        ],
        "the terminal's calls: three runs and nothing else, no session started"
    );
    ensure_eq!(
        world.store.log.calls(),
        vec!["read_every".to_string()],
        "the store's calls: one read and no write"
    );
    ensure_eq!(versions(&world), before, "every document's version");
    world.store.log.clear();
    let (places, _) = every(&world)?;
    ensure_eq!(places.len(), 50, "every place listed");
    for (name, line) in [
        ("chore--kept", "chore--kept · landed · kept by release"),
        ("fix--tidied", "fix--tidied · landed · tidied"),
    ] {
        ensure_eq!(line_of(&places, REPOSITORY, name)?, line, "{name}'s line");
    }
    ensure_eq!(
        world.store.log.calls(),
        vec!["read_every".to_string()],
        "the store's calls for every place: one read and no write"
    );
    ensure_eq!(versions(&world), before, "every document's version");
    Ok(())
}

fn every_long_word_left_to_status(_: &()) -> Outcome {
    let world = World::new();
    let (asked, _) = construct_asks(&world, "plan", &[], None)?;
    launched(&world.engine.launch(&Launch {
        hold: vec!["code".into()],
        hold_words: [("code".to_string(), "after review".to_string())].into(),
        ..launch("fix", "held", Some("x"))
    }))?;
    let held = World::id("fix", "held");
    world.settle(&held);
    let unready = world.begun("notes", Some(Deliverable::Commits), "unready")?;
    let mut state = world.held(&unready).ok_or("no notes place")?;
    state.readiness.steps.push(ReadyStep {
        step: "setup".into(),
        passed: false,
        words: "no toolchain".into(),
    });
    world.store.put(&unready, state);
    let failed = begun_fix(&world, "failed")?;
    let mut state = world.held(&failed).ok_or("no fix")?;
    state.submission.verdict = Some(Verdict::Failed {
        words: "check failed".into(),
    });
    world.store.put(&failed, state);
    let worded = begun_fix(&world, "worded")?;
    world
        .agent
        .set_last_words(&World::identity("fix", "worded", "code", 1), "done for now");
    let json = world.engine().list(false).to_json();
    let words = [
        (&asked, "Which design does the brief mean?"),
        (&held, "after review"),
        (&unready, "no toolchain"),
        (&failed, "check failed"),
        (&worded, "done for now"),
    ];
    for (place, word) in words {
        ensure!(
            !json.contains(word),
            "the list's JSON holds {word:?}: {json}"
        );
        let status = serde_json::to_string(&world.status(place)?).map_err(|e| e.to_string())?;
        ensure!(
            status.contains(word),
            "status of {place} gives not {word:?}"
        );
    }
    Ok(())
}

fn the_list_round_trips_and_holds_its_fields(_: &()) -> Outcome {
    let written = super::answers::list();
    let json = written.to_json();
    let back: Answer = serde_json::from_str(&json).map_err(|error| error.to_string())?;
    ensure_eq!(back, written, "the list read back");
    let mut value: serde_json::Value =
        serde_json::from_str(&json).map_err(|error| error.to_string())?;
    ensure_eq!(
        value["list"]["places"][0]["waits_on"].as_str(),
        Some("host"),
        "whom the first place waits on"
    );
    let missed = missing("list", &value);
    ensure!(missed.is_empty(), "{}", missed.join("; "));
    value["list"]["streams"][1]
        .as_object_mut()
        .ok_or("no second stream")?
        .remove("uncarried")
        .ok_or("no uncarried")?;
    ensure_eq!(
        missing("list", &value),
        vec!["the answer list lacks its field list.streams[].uncarried".to_string()],
        "the fields missed"
    );
    let world = World::new();
    begun_fix(&world, "one")?;
    world.host.set_display("trial", Some("cmux"), None, None);
    let listed = world.engine().list(false);
    let back: Answer =
        serde_json::from_str(&listed.to_json()).map_err(|error| error.to_string())?;
    ensure_eq!(back, listed, "the engine's list read back");
    Ok(())
}

fn a_talk_waiting_on_its_session(_: &()) -> Outcome {
    let world = World::new();
    let id = talking(&world)?;
    world.terminal.kill(&talk(1));
    world.settle(&id);
    world.agent.live_runs("s-1", &[by_hand()]);
    retry_talk(&world, &id, None, false)?;
    let asked = world.agent.log.count("live");
    let (places, _) = list(&world)?;
    let listed = places
        .iter()
        .find(|place| place.place == id)
        .ok_or("the list gives no line of the research place")?;
    ensure_eq!(
        (listed.line.as_str(), listed.waits_on),
        (
            "research--one · talk · waiting on its session · waits on its session",
            Some(WaitsOn::Session)
        ),
        "the research place's line and whom it waits on"
    );
    ensure_eq!(
        world.agent.log.count("live"),
        asked,
        "the live readings the list asked"
    );
    Ok(())
}

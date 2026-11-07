//! Status (43, 56 to 62, 69): every fact read where it lives when asked,
//! at one head per place, from one core, and nothing written, sent or
//! started. The host's part gives every role's address a standing agent
//! holds, pending or not, with what its terminal found at the ask, each
//! terminal and part read once (132, 190).

use super::account_pools::{
    account, account_of, begun_fix, code_delivered, fill_first_at_homes, fix_in, fix_of, home,
    host, of_the_surfaces, on, pool, pool_at, reading_at, reads, spent, stops_at_limit,
    two_programs, window, UNREACHED,
};
use super::agent_modes::{in_mode, in_programs, the_fakes_map};
use super::agent_settings::for_every_stage;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{
    AccountStatus, Act, AddressStatus, AgentStatus, Answer, Cache, ModeStatus, MoveStatus,
    PlaceStatus, PoolStatus, Retry, StandingFound, WaitingOn, Waits,
};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "status: asked three times of one place, it answers the same and starts or tells nothing",
            cites: &["S14", "57", "60", "69", "72"],
            run: s14_asked_three_times,
        },
        Scenario {
            name: "status: a change ticked between two asks is read anew, and neither ask writes",
            cites: &["58", "57", "59"],
            run: a_change_ticked_between_asks,
        },
        Scenario {
            name: "status: every fact of clause 56 for every place, side by side",
            cites: &["56", "43", "7", "19", "48"],
            run: every_fact_of_every_place,
        },
        Scenario {
            name: "status: three places of one repository each give their own submission from one read of the merge queue naming all three branches, and each live agent's state is read once for its place and its base's count",
            cites: &["58", "99", "139"],
            run: one_read_for_every_place,
        },
        Scenario {
            name: "status: a fake terminal scripted to lack one entry for flume's plugin gives it as the host's terminal setup, and status opens, sends to and closes nothing",
            cites: &["122", "57", "72"],
            run: the_terminals_setup,
        },
        Scenario {
            name: "status: a fix whose code agent works gives its line with every part in the JSON form, `fix--<name> · fix@1 · code · run 1 · working` in full",
            cites: &["56", "61"],
            run: a_line_in_status,
        },
        Scenario {
            name: "status: the place's answer dropping its line or one of the line's fields fails naming the answer and the field",
            cites: &["61"],
            run: a_line_field_dropped,
        },
        Scenario {
            name: "status: a terminal whose configuration binds no key to flume's places gives those words as the host's terminal setup when one place is asked of, and nothing is sent to any pane",
            cites: &["122", "72", "57"],
            run: a_popup_with_no_key,
        },
        Scenario {
            name: "status: a place no longer held is not answered for",
            cites: &["62", "58"],
            run: a_place_gone,
        },
        Scenario {
            name: "status: a place whose construct ended short waits on a retry of construct, since then, with the question whole",
            cites: &["56", "130", "42", "41"],
            run: waiting_on_an_answer,
        },
        Scenario {
            name: "status: a lost stage, a stopped one and a done round each name the move they wait on",
            cites: &["56", "130", "45", "51", "114"],
            run: the_moves_waited_on,
        },
        Scenario {
            name: "status: a place stalled on a retry of construct gives the move, since when, stalled, and its end delivered to one recipient and pending for another",
            cites: &["56", "130", "43"],
            run: a_stalled_place_with_a_pending_end,
        },
        Scenario {
            name: "status: a live place's clues and its base's count are the ones show was given, with where the terminal shows them",
            cites: &["139", "56", "60"],
            run: clues_in_status,
        },
        Scenario {
            name: "status: a repeating code stage's repetition on the second of three groups gives the group's place 2 of 3 with its number and title, `code 2/3` in the person's form and the clue's title",
            cites: &["56", "58", "139", "142"],
            run: status_mid_stage,
        },
        Scenario {
            name: "status: a place whose code start 2 carries start 1's conversation under work-2 gives the agent's conversation from start 1, carried, and code's move from work-1 to work-2 carried, in the JSON and the person's form",
            cites: &["56", "146", "S78"],
            run: a_carried_agent,
        },
        Scenario {
            name: "status: a place whose code start 2 started afresh because work-2's CLAUDE_CONFIG_DIR does not reach start 1's conversation gives the agent's conversation from start 1, not carried, and why by the variable's name and no path, and the move not carried",
            cites: &["56", "S55", "179"],
            run: a_conversation_that_stayed,
        },
        Scenario {
            name: "status: a place whose carried start 2 ended before it said it started and whose start 3 started afresh lists start 2 carried and not begun with why, and start 3's agent from start 1, not carried, and why",
            cites: &["56", "S79", "146"],
            run: a_carried_start_that_did_not_begin,
        },
        Scenario {
            name: "status: a repetition adding a fourth group to a change of three gives the count as 4, read at the head",
            cites: &["58", "142"],
            run: a_group_added_mid_stage,
        },
        Scenario {
            name: "status: a stage that moved from a spent account gives its agent's account from the strategy, and the move with why, when and the cache lost",
            cites: &["56", "145", "147"],
            run: a_stage_that_moved,
        },
        Scenario {
            name: "status: a place whose code started after the intent moved for one program, and whose review started on another program's account from the affinity, gives the intent's move with its program and the review's account from the affinity for its program",
            cites: &["56", "147", "S80"],
            run: an_intent_that_moved_for_one_program,
        },
        Scenario {
            name: "status: the host's pools give each pool's strategy and each account's agents running, bound and spent time, and no value of an account's environment",
            cites: &["56", "144", "145"],
            run: a_pool_with_a_spent_account,
        },
        Scenario {
            name: "status: a most-left pool stating no threshold gives most-left and 90, an account's fresh reading with its window's use as a percentage and its reset, another's stale reading, and a third's missing file with why, in the JSON form, and no reading's path",
            cites: &["56", "184", "185"],
            run: a_pool_with_readings,
        },
        Scenario {
            name: "status: an account whose 7-day window reads 95% used gives what holds it past its threshold, 7-day until its reset, and that window past, in the JSON form",
            cites: &["56", "185"],
            run: an_account_past_its_threshold,
        },
        Scenario {
            name: "status: an intent whose affinity for agent moved off work-1, past its threshold in 5-hour until 17:00, to work-2 gives the move with why threshold, the window and its end, in the JSON form and the person's",
            cites: &["56", "147", "185"],
            run: an_intents_move_past_a_threshold,
        },
        Scenario {
            name: "status: a move a held state holds as an earlier release wrote it, spent with no window or end, is given as written, in the JSON form and the person's",
            cites: &["56", "61"],
            run: a_move_recorded_before,
        },
        Scenario {
            name: "status: an agent that took edits from the host's table gives its mode as written from host-kind, the value its profile passed and that it was read back, in the JSON form",
            cites: &["S66", "56", "162", "169"],
            run: a_mode_from_the_hosts_table,
        },
        Scenario {
            name: "status: an agent that took its program's entry of the launch's table gives the table as written from launch, the entry and the value passed",
            cites: &["56", "169"],
            run: a_tables_entry,
        },
        Scenario {
            name: "status: an agent whose profile reads no mode back gives its mode as not read back",
            cites: &["169", "161"],
            run: a_mode_not_read_back,
        },
        Scenario {
            name: "status: a code start begun in ask-each where its record named all is named with the mode asked and the mode begun, and no variable, in the JSON form",
            cites: &["56", "161", "162", "S67"],
            run: a_start_in_another_mode,
        },
        Scenario {
            name: "status: an agent's own settings give each key with its layer and its file, how its profile passed them and that they were not read back, in the JSON form",
            cites: &["S100", "56", "32", "143"],
            run: own_settings_from_three_layers,
        },
        Scenario {
            name: "status: an agent's own settings name each file with the keys left out of it and their owners, and no value of either",
            cites: &["S100", "195", "56", "29"],
            run: own_settings_left_out_of_a_file,
        },
        Scenario {
            name: "status: an agent's own settings name nothing of another program's entry",
            cites: &["195", "56", "32"],
            run: own_settings_of_another_program,
        },
        Scenario {
            name: "status: a place of a host's source's kind gives its identity, declared host, and where it came from: the source, its file, the kind's directory, its hash and the host's lock",
            cites: &["19", "56", "191"],
            run: a_kind_of_a_hosts_source,
        },
        Scenario {
            name: "status: places of the repository's, a launch's and flume's own kinds give each identity and where it came from, the repository's file and revision, the launch's file, and flume's own hash with no lock",
            cites: &["19", "56"],
            run: the_repositorys_a_launchs_and_flumes_own,
        },
        Scenario {
            name: "status: a place whose held state keeps no kind_from gives no identity or hash, its declared the source, and settles as before",
            cites: &["21", "56"],
            run: a_place_launched_before,
        },
        Scenario {
            name: "status: the host's part gives each source with its file, its kinds_dir and every identity it holds, and a faulty one with its fault, read when asked",
            cites: &["S96", "56", "159", "191"],
            run: a_source_and_a_fault,
        },
        Scenario {
            name: "status: a standing address whose agent waits in its session gives the agent, the session from the entry, the agent found waiting and its two pending ends",
            cites: &["56", "132"],
            run: a_standing_agent_found,
        },
        Scenario {
            name: "status: a standing address whose session holds no agent of its name says none found, its pending ends beneath",
            cites: &["132", "56"],
            run: none_of_the_name,
        },
        Scenario {
            name: "status: a standing address whose session holds two agents of its name says two found",
            cites: &["132"],
            run: more_than_one_of_the_name,
        },
        Scenario {
            name: "status: a standing address whose part the terminal cannot read says so in the terminal's words",
            cites: &["132"],
            run: a_part_not_read,
        },
        Scenario {
            name: "status: a standing address with nothing pending is still given with its standing",
            cites: &["56", "132"],
            run: a_standing_address_with_nothing_pending,
        },
        Scenario {
            name: "status: two standing addresses whose agents live in one session ask the terminal for that session's agents once",
            cites: &["58", "132"],
            run: two_addresses_in_one_session,
        },
    ]
}

/// A fix in the stream `work`, its agent given own settings from the
/// host's table for `work`, the host's file for every agent and the
/// repository, the fake taking them by a file and reserving `env` and
/// `perms.mode`; its agent's status and the same in the JSON form.
fn own_settings_in_work(world: &World) -> Result<(AgentStatus, serde_json::Value), String> {
    use super::own_settings::{by_file, inline, HOST_FILE};
    use serde_json::json;
    world.agent.take_own();
    world.agent.reserve("env", "the account");
    world.agent.reserve("perms.mode", "flume's mode");
    world.host.set_stream("work", Some(Stream::default()));
    let mut every = by_file(
        "agent",
        HOST_FILE,
        json!({
            "tips": true,
            "env": { "PEOPLE": "people-only" },
            "perms": { "mode": "full-for-people", "allow": ["a"] },
        }),
        json!({}),
    );
    every
        .programs
        .extend(inline("other", json!({ "line": "b" })).programs);
    world.host.set_every_agent(every);
    world
        .host
        .set_stream_settings("work", inline("agent", json!({ "line": "work" })));
    world.kinds.set_settings(
        REPOSITORY,
        "fix",
        for_every_stage(inline("agent", json!({ "style": "plain" }))),
    );
    launched(&world.engine.launch(&Launch {
        stream: Some("work".into()),
        ..launch("fix", "one", Some("mend it"))
    }))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    let status = world.status(&id)?;
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    Ok((status.agent.ok_or("no agent")?, json["agent"].clone()))
}

fn own_settings_from_three_layers(_: &()) -> Outcome {
    use super::own_settings::HOST_FILE;
    use serde_json::json;
    let world = World::new();
    let (agent, json) = own_settings_in_work(&world)?;
    let from =
        |key: &str| json["settings"]["from"][format!("programs.agent.settings.{key}")].clone();
    ensure_eq!(
        from("line"),
        json!({ "layer": "host-stream", "value": r#""work""# }),
        "line"
    );
    ensure_eq!(
        from("tips"),
        json!({ "layer": "host", "value": "true", "file": HOST_FILE }),
        "tips"
    );
    ensure_eq!(
        from("style"),
        json!({ "layer": "repository", "value": r#""plain""# }),
        "style"
    );
    let own = agent.own.ok_or("no own settings")?;
    let file = flume_fakes::FakeAgent::own_file(
        &world.store.folder(&World::id("fix", "one")),
        &World::identity("fix", "one", "code", 1),
    );
    ensure_eq!(
        own.passed,
        Some(format!("--own {file}")),
        "how they were passed"
    );
    ensure!(!own.read_back, "own settings read back");
    ensure_eq!(json["own"]["read_back"], json!(false), "read_back as JSON");
    ensure_eq!(
        json["settings"]["programs"]["agent"]["settings"],
        json!({ "line": "work", "tips": true, "style": "plain", "perms": { "allow": ["a"] } }),
        "the keys the agent's settings give"
    );
    Ok(())
}

fn own_settings_left_out_of_a_file(_: &()) -> Outcome {
    use super::own_settings::HOST_FILE;
    use serde_json::json;
    let world = World::new();
    let (agent, json) = own_settings_in_work(&world)?;
    let own = agent.own.ok_or("no own settings")?;
    ensure_eq!(
        own.files
            .iter()
            .map(|file| (file.layer, file.read.as_str(), file.left_out.clone()))
            .collect::<Vec<_>>(),
        vec![(
            Layer::Host,
            HOST_FILE,
            vec![
                Reserved {
                    key: "env".into(),
                    owner: "the account".into(),
                },
                Reserved {
                    key: "perms.mode".into(),
                    owner: "flume's mode".into(),
                },
            ],
        )],
        "the file and the keys left out of it"
    );
    ensure_eq!(
        json["own"]["files"][0]["left_out"][1],
        json!({ "key": "perms.mode", "owner": "flume's mode" }),
        "a key left out as JSON"
    );
    let text = json.to_string();
    ensure!(
        !text.contains("people-only") && !text.contains("full-for-people"),
        "a value left out in status: {text}"
    );
    Ok(())
}

fn own_settings_of_another_program(_: &()) -> Outcome {
    let world = World::new();
    let (_, json) = own_settings_in_work(&world)?;
    let text = json.to_string();
    ensure!(
        !text.contains("programs.other") && json["settings"]["programs"].get("other").is_none(),
        "another program's entry in status: {text}"
    );
    Ok(())
}

fn a_kind_of_a_hosts_source(_: &()) -> Outcome {
    use super::kind_sources::{holds, host_locks};
    use crate::seams::kinds::{entry, hash_of};
    let world = World::new();
    holds(&world, "acme", "/kinds/acme", &[("review", 1, '1')]);
    host_locks(&world, vec![entry("acme", "review", 1, &hash_of('1'))]);
    launched(
        &world
            .engine
            .launch(&launch("acme/review@1", "one", Some("x"))),
    )?;
    let status = world.status(&World::id("review", "one"))?;
    ensure_eq!(
        (status.identity.as_deref(), status.declared),
        (Some("acme/review@1"), Declared::Host),
        "the identity and declared"
    );
    let from = status.kind_from.ok_or("no kind_from")?;
    ensure_eq!(
        (
            from.source.as_str(),
            from.source_file,
            from.kind_dir,
            from.hash,
            from.lock_file.as_deref()
        ),
        (
            "acme",
            Some(flume_fakes::source_file("acme")),
            Some(flume_fakes::package_dir("/kinds/acme", "review", 1)),
            Some(hash_of('1')),
            Some(flume_fakes::HOST_LOCK)
        ),
        "where the kind came from"
    );
    ensure!(
        from.declaration_file.is_none() && from.revision.is_none(),
        "a host's source's kind names a declaration: {:?}",
        (from.declaration_file, from.revision)
    );
    Ok(())
}

fn the_repositorys_a_launchs_and_flumes_own(_: &()) -> Outcome {
    let world = World::new();
    let mut bump = kind(
        "bump",
        false,
        vec![stage("code", Some(Deliverable::Commits), "{{brief}}")],
    );
    bump.version = 2;
    world.kinds.set_kinds(REPOSITORY, vec![bump]);
    let declaration = "/caller/kinds.toml";
    world.kinds.set_declaration(
        declaration,
        vec![kind(
            "spike",
            false,
            vec![stage("try", Some(Deliverable::Commits), "{{brief}}")],
        )],
    );
    let base = world.git.branch_head(REPOSITORY, "main").ok_or("main")?;
    launched(&world.engine.launch(&launch("bump", "one", Some("x"))))?;
    let mut carried = launch("spike", "one", Some("x"));
    carried.declaration = Some(declaration.into());
    launched(&world.engine.launch(&carried))?;
    launched(&world.engine.launch(&launch("change", "plan", Some("x"))))?;
    let of = |kind: &str, name: &str| -> Result<(Option<String>, KindFrom), String> {
        let status = world.status(&World::id(kind, name))?;
        Ok((status.identity, status.kind_from.ok_or("no kind_from")?))
    };
    let (identity, from) = of("bump", "one")?;
    ensure_eq!(identity.as_deref(), Some("repo/bump@2"), "the repository's");
    ensure_eq!(
        (
            from.source.as_str(),
            from.declaration_file.as_deref(),
            from.revision
        ),
        ("repo", Some(".config/flume.toml"), Some(base)),
        "where the repository's came from"
    );
    let (identity, from) = of("spike", "one")?;
    ensure_eq!(identity.as_deref(), Some("launch/spike@1"), "the launch's");
    ensure_eq!(
        (from.source.as_str(), from.declaration_file.as_deref()),
        ("launch", Some(declaration)),
        "where the launch's came from"
    );
    ensure!(
        from.hash.is_none() && from.lock_file.is_none(),
        "a declaration's kind names a hash or a lock: {from:?}"
    );
    let (identity, from) = of("change", "plan")?;
    ensure_eq!(identity.as_deref(), Some("flume/change@2"), "flume's own");
    let own = world
        .kinds
        .flumes_own()
        .into_iter()
        .find(|own| own.identity.spelled == "flume/change@2")
        .ok_or("flume holds no change@2")?;
    ensure_eq!(
        (from.source.as_str(), from.hash, from.lock_file),
        ("flume", Some(own.hash), None),
        "where flume's own came from"
    );
    Ok(())
}

fn a_place_launched_before(_: &()) -> Outcome {
    let world = World::new();
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let id = World::id("fix", "one");
    let mut held = world.held(&id).ok_or("no place")?;
    held.place.kind_from = None;
    world.store.put(&id, held);
    let status = world.status(&id)?;
    ensure_eq!(
        (status.identity, status.kind_from, status.declared),
        (None, None, Declared::Flume),
        "the kind's identity, where it came from, and declared"
    );
    world.settle(&id);
    super::settling::deliver_fix(&world, &id);
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    Ok(())
}

fn a_source_and_a_fault(_: &()) -> Outcome {
    use super::kind_sources::holds;
    let world = World::new();
    holds(
        &world,
        "acme",
        "/kinds/acme",
        &[("review", 1, '1'), ("review", 2, '2')],
    );
    world.host.faulty_source(
        "bad",
        "it gives no kinds_dir, the directory of its packages",
    );
    let sources = match world.engine.status(None) {
        Answer::Status { host, .. } => host.sources,
        other => return Err(format!("status answered {other:?}")),
    };
    ensure_eq!(
        sources,
        vec![
            flume_core::SourceStatus {
                name: "acme".into(),
                source_file: flume_fakes::source_file("acme"),
                kinds_dir: Some("/kinds/acme".into()),
                kinds: vec!["acme/review@1".into(), "acme/review@2".into()],
                cannot_read: None,
            },
            flume_core::SourceStatus {
                name: "bad".into(),
                source_file: flume_fakes::source_file("bad"),
                kinds_dir: None,
                kinds: vec![],
                cannot_read: Some("it gives no kinds_dir, the directory of its packages".into()),
            },
        ],
        "the host's sources"
    );
    world.kinds.set_package(
        "/kinds/acme",
        super::kind_sources::packaged("lint", 1),
        "sha256:0",
    );
    let again = match world.engine.status(None) {
        Answer::Status { host, .. } => host.sources,
        other => return Err(format!("status answered {other:?}")),
    };
    ensure_eq!(
        again[0].kinds.len(),
        3,
        "the source's identities, read again when asked"
    );
    Ok(())
}

/// The fix `given` launches, settled once: its agent's status, and the
/// same in the JSON form.
fn fix_with_a_mode(
    world: &World,
    given: &Launch,
) -> Result<(flume_core::AgentStatus, serde_json::Value), String> {
    launched(&world.engine.launch(given))?;
    let id = World::id("fix", &given.name);
    world.settle(&id);
    let status = world.status(&id)?;
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    Ok((status.agent.ok_or("no agent")?, json["agent"].clone()))
}

fn a_mode_from_the_hosts_table(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    world.agent.read_mode_back();
    world
        .host
        .set_settings(None, "fix", for_every_stage(in_mode(Mode::Edits)));
    let (agent, json) = fix_with_a_mode(&world, &launch("fix", "one", Some("mend it")))?;
    ensure_eq!(
        agent.mode,
        Some(ModeStatus {
            passed: "edit-files".into(),
            entry: None,
            read_back: true,
        }),
        "the agent's mode"
    );
    ensure_eq!(
        (
            json["settings"]["mode"].clone(),
            json["settings"]["from"]["mode"].clone(),
            json["mode"].clone()
        ),
        (
            serde_json::json!("edits"),
            serde_json::json!({"layer": "host-kind", "value": "edits"}),
            serde_json::json!({"passed": "edit-files", "entry": null, "read_back": true})
        ),
        "the mode in the JSON form"
    );
    Ok(())
}

fn a_tables_entry(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    world.agent.read_mode_back();
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = in_programs(&[("agent", "careful"), ("other", "edits")]);
    let (agent, json) = fix_with_a_mode(&world, &fix)?;
    ensure_eq!(
        agent.settings.from.get("mode"),
        Some(&Given {
            layer: Layer::Launch,
            value: r#"{ agent = "careful", other = "edits" }"#.into(),
            file: None,
        }),
        "the mode as written and its layer"
    );
    ensure_eq!(
        agent.mode,
        Some(ModeStatus {
            passed: "careful".into(),
            entry: Some("agent".into()),
            read_back: true,
        }),
        "the agent's mode"
    );
    ensure_eq!(
        (
            json["settings"]["mode"].clone(),
            json["mode"]["entry"].clone()
        ),
        (
            serde_json::json!({"agent": "careful", "other": "edits"}),
            serde_json::json!("agent")
        ),
        "the table and the entry in the JSON form"
    );
    Ok(())
}

fn a_mode_not_read_back(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = in_mode(Mode::Edits);
    let (agent, json) = fix_with_a_mode(&world, &fix)?;
    ensure_eq!(
        agent.mode.as_ref().map(|mode| mode.read_back),
        Some(false),
        "the mode read back"
    );
    ensure_eq!(
        json["mode"]["read_back"].clone(),
        serde_json::json!(false),
        "read_back in the JSON form"
    );
    Ok(())
}

fn a_start_in_another_mode(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    world.agent.read_mode_back();
    let who = World::identity("fix", "one", "code", 1);
    world.agent.set_began_in(&who, "ask-each");
    world.agent.set_own_work(&who, Reading::Yes);
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = in_mode(Mode::Full);
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    let status = world.status(&id)?;
    ensure_eq!(
        status
            .unsettled
            .iter()
            .map(|read| (read.stage.as_str(), read.start, read.unsettled.clone()))
            .collect::<Vec<_>>(),
        vec![(
            "code",
            1,
            Unsettled {
                mode: Some(ModeBegan {
                    asked: "all".into(),
                    began: Some("ask-each".into()),
                }),
                ..Unsettled::default()
            }
        )],
        "the start named with both modes and no variable"
    );
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    let read = &json["unsettled"][0];
    ensure_eq!(
        (
            read["mode"].clone(),
            read["missing"].clone(),
            read["different"].clone()
        ),
        (
            serde_json::json!({"asked": "all", "began": "ask-each"}),
            serde_json::json!([]),
            serde_json::json!([])
        ),
        "the start in the JSON form"
    );
    Ok(())
}

fn clues_in_status(_: &()) -> Outcome {
    let world = World::new();
    let one = world.begun("build", Some(Deliverable::TasksTicked), "one")?;
    let two = World::id("build", "two");
    launched(&world.engine.launch(&launch("build", "two", Some("x"))))?;
    world.settle(&two);
    let asking = World::identity("build", "two", "work", 1);
    world.terminal.set_state(&asking, AgentState::Working);
    world.settle(&one);
    world.settle(&two);
    let shown = world.terminal.shown();
    let (_, last) = shown.last().ok_or("nothing shown")?;
    let Answer::Status { places, host } = world.engine().status(None) else {
        return Err("status refused".into());
    };
    let base = host.bases.first().ok_or("no base")?;
    ensure_eq!(host.bases.len(), 1, "the bases");
    ensure_eq!(base.base.as_str(), "main", "the base");
    ensure_eq!(base.count, last.base, "the count show was given");
    ensure_eq!(base.line, last.base.to_string(), "the count's line");
    ensure!(base.shown_at.is_some(), "the base's workspace unnamed");
    let two = places
        .iter()
        .find(|place| place.place == two)
        .ok_or("no place two")?;
    let clues = two.clues.as_ref().ok_or("no clues")?;
    ensure_eq!(
        (clues.title.as_str(), clues.display_agent.as_str()),
        ("work · working", "build--two"),
        "the title and the agent's name"
    );
    ensure_eq!(
        format!("{} · {}", last.stage, last.state),
        clues.title,
        "the title show was given"
    );
    ensure!(
        clues.shown_at.agent.is_some() && clues.shown_at.place.is_some(),
        "the place's tab or its agent's pane unnamed: {:?}",
        clues.shown_at
    );
    Ok(())
}

fn waiting_on_an_answer(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("change", "plan");
    launched(&world.engine.launch(&launch("change", "plan", Some("x"))))?;
    world.settle(&id);
    let construct = World::identity("change", "plan", "construct", 1);
    let secret = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?
        .secret;
    world.host.advance(300);
    let asked = world.host.now();
    let question = "Which design does the brief mean: the queue,\nor the log?";
    let answer = world.engine.needs(
        &construct.to_string(),
        &secret.0,
        question,
        StopKind::NeedsAnswer,
    );
    ensure!(answer.refusal().is_none(), "the stop-short was refused");
    world.host.advance(60);
    world.settle(&id);
    ensure_eq!(
        world.status(&id)?.waiting_on,
        Some(WaitingOn {
            act: Act::Retry,
            stage: Some("construct".into()),
            why: Waits::Answer,
            since: Some(asked),
            words: Some(question.into()),
        }),
        "the move the place waits on"
    );
    Ok(())
}

fn the_moves_waited_on(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    ensure_eq!(world.status(&id)?.waiting_on, None, "a working place");
    world.terminal.kill(&work);
    world.settle(&id);
    let lost = world
        .status(&id)?
        .waiting_on
        .ok_or("a lost stage waits on nobody")?;
    ensure_eq!(
        (lost.act, lost.stage.as_deref(), lost.why),
        (Act::Retry, Some("work"), Waits::Lost),
        "a lost stage"
    );
    let id = world.begun("tend", Some(Deliverable::Commits), "two")?;
    ensure!(
        world.engine.stop(&id, true).refusal().is_none(),
        "the stop was refused"
    );
    let stopped = world
        .status(&id)?
        .waiting_on
        .ok_or("a stopped stage waits on nobody")?;
    ensure_eq!(
        (stopped.act, stopped.why),
        (Act::Retry, Waits::Stopped),
        "a stopped stage"
    );
    let id = world.begun("sweep", Some(Deliverable::Commits), "three")?;
    world.commit("sweep/three", &["swept.rs"]);
    world.settle(&id);
    let done = world
        .status(&id)?
        .waiting_on
        .ok_or("a delivered round waits on nobody")?;
    ensure_eq!(
        (done.act, done.stage, done.why),
        (Act::Done, None, Waits::Delivered),
        "a round delivered whose kind does not land"
    );
    Ok(())
}

fn places(answer: Answer) -> Result<Vec<PlaceStatus>, String> {
    match answer {
        Answer::Status { places, .. } => Ok(places),
        other => Err(format!("status answered {other:?}")),
    }
}

/// The calls that would start, tell or write.
fn acting_calls(world: &World) -> (usize, usize, usize, usize) {
    (
        world.terminal.log.count("open"),
        world.terminal.log.count("send"),
        world.store.log.count("write") + world.store.log.count("remove"),
        world.queue.log.count("submit"),
    )
}

fn s14_asked_three_times(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::TasksTicked), "one")?;
    world.tick("build/one", "one", 1, 2);
    let before = acting_calls(&world);
    let line = places(world.engine().status(Some(&id)))?;
    let session = places(world.engine().status(Some(&id)))?;
    let sidebar = places(world.engine().status(Some(&id)))?;
    ensure_eq!(line, session, "two surfaces' answers");
    ensure_eq!(session, sidebar, "two surfaces' answers");
    ensure_eq!(
        acting_calls(&world),
        before,
        "calls that start, tell or write"
    );
    Ok(())
}

fn a_change_ticked_between_asks(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::TasksTicked), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.terminal.set_state(&work, AgentState::Working);
    world.tick("build/one", "one", 1, 3);
    let before = acting_calls(&world);
    let first = world.status(&id)?;
    let head = world.tick("build/one", "one", 2, 3);
    let second = world.status(&id)?;
    let ticked = |status: &PlaceStatus| {
        status
            .tasks
            .as_ref()
            .map(|tasks| tasks.iter().filter(|task| task.ticked).count())
    };
    ensure_eq!(
        (ticked(&first), ticked(&second)),
        (Some(1), Some(2)),
        "the tasks ticked"
    );
    ensure_eq!(second.head, Some(head), "the head read");
    ensure_eq!(
        acting_calls(&world),
        before,
        "calls that start, tell or write"
    );
    Ok(())
}

fn every_fact_of_every_place(_: &()) -> Outcome {
    let world = World::new();
    let mut fix = launch("fix", "one", Some("x"));
    fix.mark = Some("team-a slot-2".into());
    launched(&world.engine.launch(&fix))?;
    launched(&world.engine.launch(&launch("change", "plan", Some("y"))))?;
    world.engine().settle(None);
    let fix_id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.agent.refuse_act(&code, "a push of fix/one");
    let all = places(world.engine.status(None))?;
    ensure_eq!(all.len(), 2, "places");
    let one = all
        .iter()
        .find(|place| place.place == fix_id)
        .ok_or("no fix")?;
    ensure_eq!(
        (
            one.repository.as_str(),
            one.kind.as_str(),
            one.version,
            one.declared
        ),
        (REPOSITORY, "fix", 1, Declared::Flume),
        "the repository and kind"
    );
    ensure_eq!(
        (
            one.name.as_str(),
            one.branch.as_str(),
            one.base.as_str(),
            one.lands
        ),
        ("one", "fix/one", "main", true),
        "the name, branch, base and landing"
    );
    ensure_eq!(one.mark.as_deref(), Some("team-a slot-2"), "the mark");
    ensure_eq!(one.stages[0].starts, 1, "the code stage's starts");
    let agent = one.agent.clone().ok_or("no agent")?;
    ensure_eq!(agent.identity, code.to_string(), "the agent's identity");
    ensure_eq!(
        agent.settings.settings.program.as_deref(),
        Some("agent"),
        "the agent's settings"
    );
    ensure_eq!(agent.state, AgentState::Waiting, "the agent's state");
    ensure_eq!(one.delivered, Some(false), "whether the deliverable exists");
    let submission = one.submission.clone().ok_or("no submission")?;
    ensure_eq!(submission.read, SubmissionRead::None, "the submission");
    ensure_eq!(submission.submits, 0, "submissions");
    ensure!(!one.refused_acts.is_empty(), "no refused act");
    let plan = all
        .iter()
        .find(|place| place.kind == "change")
        .ok_or("no change")?;
    ensure!(plan.tasks.is_some(), "no tasks for a change's place");
    Ok(())
}

fn one_read_for_every_place(_: &()) -> Outcome {
    let world = World::new();
    for name in ["one", "two", "three"] {
        launched(&world.engine.launch(&launch("fix", name, Some("x"))))?;
    }
    world.engine().settle(None);
    let handed =
        world
            .queue
            .hand_submission(REPOSITORY, "fix/two", "main", SubmissionState::Waiting);
    world.queue.log.clear();
    world.terminal.log.clear();
    let all = places(world.engine.status(None))?;
    ensure_eq!(
        world.queue.log.of("read_submission"),
        vec![format!(
            "read_submission {REPOSITORY} fix/one fix/three fix/two"
        )],
        "the merge queue's reads"
    );
    for place in &all {
        let submission = place.submission.as_ref().ok_or("no submission")?;
        let expected = match place.name.as_str() {
            "two" => SubmissionRead::Found {
                submission: handed.clone(),
            },
            _ => SubmissionRead::None,
        };
        ensure_eq!(
            submission.read,
            expected,
            format!("{}'s submission", place.name)
        );
        ensure!(place.agent.is_some(), "{} has no agent", place.name);
    }
    ensure_eq!(
        world.terminal.log.count("state"),
        3,
        "the agents' states read"
    );
    Ok(())
}

fn a_line_in_status(_: &()) -> Outcome {
    let world = World::new();
    let id = super::account_pools::begun_fix(&world, "parser")?;
    let status = world.status(&id)?;
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    let line = json["line"].as_object().ok_or("no line in the JSON form")?;
    ensure_eq!(
        line.keys().map(String::as_str).collect::<Vec<_>>(),
        vec!["full", "kind", "name", "run", "short", "stage", "state", "tasks"],
        "the line's fields"
    );
    ensure_eq!(
        status.line.full.as_str(),
        "fix--parser · fix@1 · code · run 1 · working",
        "the line in full"
    );
    Ok(())
}

fn a_line_field_dropped(_: &()) -> Outcome {
    use super::answers::{full, missing};
    let (name, answer) = full().remove(0);
    for (field, path) in [
        ("line", "status.places[].line"),
        ("short", "status.places[].line.short"),
    ] {
        let mut json: serde_json::Value =
            serde_json::from_str(&answer.to_json()).map_err(|error| error.to_string())?;
        let place = &mut json["status"]["places"][0];
        let holder = if field == "line" {
            place
        } else {
            &mut place["line"]
        };
        holder
            .as_object_mut()
            .ok_or("no object")?
            .remove(field)
            .ok_or("no such field")?;
        let missed = missing(name, &json);
        ensure!(
            missed.contains(&format!("the answer status lacks its field {path}")),
            "the fields missed: {missed:?}"
        );
    }
    Ok(())
}

fn a_popup_with_no_key(_: &()) -> Outcome {
    use crate::fake_harness::SETUP;
    let world = World::new();
    let id = super::account_pools::begun_fix(&world, "parser")?;
    world.terminal.set_setup(&[SETUP]);
    let sends = |world: &World| {
        (
            world.terminal.log.count("send"),
            world.terminal.log.count("send_to"),
            world.terminal.every_notice().len(),
        )
    };
    let before = sends(&world);
    let Answer::Status { host, .. } = world.engine().status(Some(&id)) else {
        return Err("status refused".into());
    };
    ensure_eq!(
        host.terminal_setup,
        vec![SETUP.to_string()],
        "the host's terminal setup"
    );
    ensure_eq!(sends(&world), before, "what was sent to a pane");
    Ok(())
}

fn the_terminals_setup(_: &()) -> Outcome {
    use crate::fake_harness::SETUP;
    let world = World::new();
    world.begun("build", Some(Deliverable::Commits), "one")?;
    world.terminal.set_setup(&[SETUP]);
    let calls = |world: &World| {
        ["open", "send", "close"].map(|operation| world.terminal.log.count(operation))
    };
    let before = calls(&world);
    let Answer::Status { host, .. } = world.engine().status(None) else {
        return Err("status refused".into());
    };
    ensure_eq!(
        host.terminal_setup,
        vec![SETUP.to_string()],
        "the host's terminal setup"
    );
    ensure_eq!(calls(&world), before, "the opens, sends and closes");
    Ok(())
}

fn a_place_gone(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    world.engine.drop_place(&id, true);
    let reason = refused(&world.engine.status(Some(&id)))?;
    ensure!(
        reason.contains(&id.0),
        "the refusal names not the place: {reason}"
    );
    ensure!(
        places(world.engine.status(None))?.is_empty(),
        "a gone place is answered for"
    );
    Ok(())
}

fn a_stalled_place_with_a_pending_end(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Working);
    let (id, asked) =
        super::caller_waits::construct_asks(&world, "plan", &["ops", "madswan/planner"], None)?;
    world.host.advance(STALL_AFTER);
    let stalled_at = world.host.now();
    world.settle(&id);
    let status = world.status(&id)?;
    let waiting = status.waiting_on.ok_or("the place waits on nobody")?;
    ensure_eq!(
        (
            waiting.act,
            waiting.stage.as_deref(),
            waiting.why,
            waiting.since
        ),
        (Act::Retry, Some("construct"), Waits::Answer, Some(asked)),
        "the move and since when"
    );
    ensure!(status.stalled, "status does not say stalled");
    let stall = status
        .ends
        .iter()
        .find(|end| matches!(end.end.of, EndOf::Stall { .. }))
        .ok_or("no stall among the ends")?;
    ensure_eq!(
        stall
            .end
            .deliveries
            .iter()
            .map(|delivery| (delivery.recipient.as_str(), delivery.at))
            .collect::<Vec<_>>(),
        vec![("ops", stalled_at)],
        "the stall's deliveries"
    );
    ensure_eq!(
        stall.pending_for,
        vec!["madswan/planner".to_string()],
        "pending for"
    );
    Ok(())
}

fn status_mid_stage(_: &()) -> Outcome {
    use super::repeating_stages::{branch, code_begun, GROUPS};
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    world.tick_groups(&branch("plan"), "plan", GROUPS, 2);
    world.settle(&id);
    let status = world.status(&id)?;
    let code = status
        .stages
        .iter()
        .find(|stage| stage.name == "code")
        .ok_or("no code stage")?;
    let json = serde_json::to_value(code).map_err(|error| error.to_string())?;
    ensure_eq!(
        json["group"],
        serde_json::json!({"place": 2, "of": 3, "number": "2", "title": "Two"}),
        "the group in the JSON form"
    );
    ensure_eq!(code.label(), "code 2/3", "the stage in the person's form");
    ensure!(
        status
            .stages
            .iter()
            .filter(|stage| stage.name != "code")
            .all(|stage| stage.group.is_none()),
        "a stage that does not repeat names a group"
    );
    let clues = status.clues.ok_or("no clues")?;
    ensure!(
        clues.title.starts_with("code 2/3 · "),
        "the clue's title: {}",
        clues.title
    );
    let shown = world.terminal.shown();
    let (_, last) = shown.last().ok_or("nothing shown")?;
    ensure_eq!(last.stage.as_str(), "code 2/3", "the stage show was given");
    Ok(())
}

fn a_group_added_mid_stage(_: &()) -> Outcome {
    use super::repeating_stages::{branch, code_begun, GROUPS};
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    world.tick_groups(&branch("plan"), "plan", GROUPS, 2);
    world.settle(&id);
    let mut four = GROUPS.to_vec();
    four.push(("4", "Four", 1));
    world.tick_groups(&branch("plan"), "plan", &four, 3);
    let group = world.status(&id)?.stages[1]
        .group
        .clone()
        .ok_or("code names no group")?;
    ensure_eq!(
        (group.place, group.of),
        (2, 4),
        "the group's place and the count, read at the head"
    );
    Ok(())
}

fn a_stage_that_moved(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("work-1", None), account("work-2", None)],
    );
    world.host.set_host_pool(Some("work"));
    let id = fix_in(&world, None, "one")?;
    spent(&world, "work", "work-1", world.host.now() + 3600);
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "code once its agent ended"
    );
    world.host.advance(30);
    let answer = world
        .engine()
        .retry(&id, Retry::Stage("code".into()), None, false);
    ensure!(answer.refusal().is_none(), "the retry: {answer:?}");
    let status = world.status(&id)?;
    ensure_eq!(
        status.agent.and_then(|agent| agent.account),
        Some(AccountHeld {
            pool: "work".into(),
            name: "work-2".into(),
            from: AccountFrom::Strategy
        }),
        "the agent's account"
    );
    let at = world.host.now();
    let moved = |on| MoveStatus {
        moved: AccountMove {
            on,
            from: "work-1".into(),
            to: "work-2".into(),
            why: MoveWhy::Spent,
            at,
            carried: None,
            window: None,
            until: None,
        },
        cache: Cache::Lost,
    };
    ensure_eq!(
        status.moves,
        vec![
            moved(MoveOn::Stage {
                stage: "code".into()
            }),
            moved(MoveOn::Intent {
                program: Some("agent".into())
            })
        ],
        "the place's moves"
    );
    Ok(())
}

fn an_intent_that_moved_for_one_program(_: &()) -> Outcome {
    let world = World::new();
    two_programs(&world, None);
    let first = of_the_surfaces(&world, "two", "a", None)?;
    code_delivered(&world, &first, "a")?;
    spent(&world, "work", "agent-1", world.host.now() + 3600);
    let id = of_the_surfaces(&world, "two", "b", None)?;
    code_delivered(&world, &id, "b")?;
    let status = world.status(&id)?;
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    ensure_eq!(
        json["moves"],
        serde_json::json!([{
            "on": "intent",
            "program": "agent",
            "from": "agent-1",
            "to": "agent-2",
            "why": "spent",
            "at": world.host.now(),
            "cache": "lost",
        }]),
        "the place's moves in the JSON form"
    );
    ensure_eq!(
        (
            json["agent"]["stage"].as_str(),
            json["agent"]["program"].as_str(),
            json["agent"]["account"]["name"].as_str(),
            json["agent"]["account"]["from"].as_str(),
        ),
        (
            Some("review"),
            Some("other"),
            Some("other-1"),
            Some("affinity")
        ),
        "the review's agent and its account in the JSON form"
    );
    let words = status.moves[0].words();
    ensure!(
        words.starts_with("the intent moved for agent from agent-1 to agent-2"),
        "the move's words: {words}"
    );
    Ok(())
}

fn a_pool_with_a_spent_account(_: &()) -> Outcome {
    let world = World::new();
    let mut work_1 = account("work-1", Some(4));
    work_1
        .env
        .insert("AGENT_TOKEN".into(), "a-token-of-the-account".into());
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![work_1, account("work-2", None)],
    );
    world.host.set_host_pool(Some("work"));
    let until = world.host.now() + 2 * 3600;
    spent(&world, "work", "work-2", until);
    fix_in(&world, None, "one")?;
    fix_in(&world, None, "two")?;
    let host = host(&world)?;
    ensure_eq!(
        host.pools,
        vec![PoolStatus {
            name: "work".into(),
            strategy: Strategy::FillFirst,
            threshold: 90,
            accounts: vec![
                AccountStatus {
                    name: "work-1".into(),
                    program: "agent".into(),
                    running: 2,
                    bound: Some(4),
                    spent_until: None,
                    spent_unread: None,
                    reading: None,
                    past: None,
                },
                AccountStatus {
                    name: "work-2".into(),
                    program: "agent".into(),
                    running: 0,
                    bound: None,
                    spent_until: Some(until),
                    spent_unread: None,
                    reading: None,
                    past: None,
                },
            ],
        }],
        "the host's pools"
    );
    let answer = world.engine().status(None).to_json();
    ensure!(
        !answer.contains("a-token-of-the-account"),
        "status gives a value of an account's environment"
    );
    Ok(())
}

fn a_pool_with_readings(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    let seventeen = now + 2 * 3600;
    pool_at(
        &world,
        "work",
        Strategy::MostLeft,
        None,
        ["work-1", "work-2", "work-3"]
            .into_iter()
            .map(|name| reading_at(account(name, None)))
            .collect(),
    );
    reads(
        &world,
        "work-1",
        now - 120,
        vec![window("5-hour", 4_200, Some(seventeen))],
    );
    reads(
        &world,
        "work-2",
        now - 20 * 60,
        vec![window("5-hour", 1_250, Some(seventeen))],
    );
    let json = serde_json::to_value(host(&world)?).map_err(|error| error.to_string())?;
    let unbound = |name: &str, reading| {
        serde_json::json!({
            "name": name,
            "program": "agent",
            "running": 0,
            "bound": null,
            "spent_until": null,
            "reading": reading,
        })
    };
    ensure_eq!(
        json["pools"],
        serde_json::json!([{
            "name": "work",
            "strategy": "most-left",
            "threshold": 90,
            "accounts": [
                unbound("work-1", serde_json::json!({
                    "read_at": now - 120,
                    "fresh": true,
                    "windows": [{"name": "5-hour", "used": 42, "resets_at": seventeen, "past": false}],
                })),
                unbound("work-2", serde_json::json!({
                    "read_at": now - 20 * 60,
                    "fresh": false,
                    "windows": [{"name": "5-hour", "used": 12.5, "resets_at": seventeen, "past": false}],
                })),
                unbound("work-3", serde_json::json!({"unread": "the file is missing"})),
            ],
        }]),
        "the host's pools in the JSON form"
    );
    let answer = world.engine().status(None).to_json();
    ensure!(
        !answer.contains(".local/state/usage"),
        "status gives a reading's path: {answer}"
    );
    Ok(())
}

fn an_account_past_its_threshold(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    let monday = now + 3 * 24 * 3600;
    pool_at(
        &world,
        "work",
        Strategy::FillFirst,
        None,
        vec![reading_at(account("work-1", None)), account("work-2", None)],
    );
    reads(
        &world,
        "work-1",
        now,
        vec![
            window("5-hour", 3_000, Some(now + 3600)),
            window("7-day", 9_500, Some(monday)),
        ],
    );
    let json = serde_json::to_value(host(&world)?).map_err(|error| error.to_string())?;
    let accounts = &json["pools"][0]["accounts"];
    ensure_eq!(
        accounts[0]["past"],
        serde_json::json!({"window": "7-day", "until": monday}),
        "what holds work-1 past its threshold"
    );
    ensure_eq!(
        accounts[0]["reading"]["windows"],
        serde_json::json!([
            {"name": "5-hour", "used": 30, "resets_at": now + 3600, "past": false},
            {"name": "7-day", "used": 95, "resets_at": monday, "past": true},
        ]),
        "work-1's windows"
    );
    ensure_eq!(
        (accounts[1].get("past"), accounts[1].get("reading")),
        (None, None),
        "work-2, which names no reading"
    );
    Ok(())
}

fn an_intents_move_past_a_threshold(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    let seventeen = now + 2 * 3600;
    pool_at(
        &world,
        "work",
        Strategy::FillFirst,
        None,
        vec![reading_at(account("work-1", None)), account("work-2", None)],
    );
    let first = fix_of(&world, None, "a", Some("the-surfaces"))?;
    ensure_eq!(
        account_of(&world, &first, "code"),
        on("work", "work-1"),
        "the intent's first place's account"
    );
    reads(
        &world,
        "work-1",
        now,
        vec![window("5-hour", 9_200, Some(seventeen))],
    );
    let id = fix_of(&world, None, "b", Some("the-surfaces"))?;
    let status = world.status(&id)?;
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    ensure_eq!(
        json["moves"],
        serde_json::json!([{
            "on": "intent",
            "program": "agent",
            "from": "work-1",
            "to": "work-2",
            "why": "threshold",
            "at": now,
            "window": "5-hour",
            "until": seventeen,
            "cache": "lost",
        }]),
        "the place's moves in the JSON form"
    );
    let words = status.moves[0].words();
    ensure!(
        words.starts_with(&format!(
            "the intent moved for agent from work-1 to work-2 at {now}, work-1 past its threshold in 5-hour until {seventeen},"
        )),
        "the move in the person's form: {words}"
    );
    Ok(())
}

fn a_move_recorded_before(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_in(&world, None, "one")?;
    let before = serde_json::json!({
        "on": "stage",
        "stage": "code",
        "from": "work-1",
        "to": "work-2",
        "why": "spent",
        "at": world.host.now(),
    });
    let mut held = world.held(&id).ok_or("no held state")?;
    held.moves
        .push(serde_json::from_value(before.clone()).map_err(|error| error.to_string())?);
    world.store.put(&id, held);
    let status = world.status(&id)?;
    ensure_eq!(
        status
            .moves
            .iter()
            .map(|moved| (moved.moved.window.clone(), moved.moved.until))
            .collect::<Vec<_>>(),
        vec![(None, None)],
        "the move's window and end"
    );
    let words = status.moves[0].words();
    ensure!(
        words.contains("work-1 spent,") && !words.contains("threshold"),
        "the move in the person's form: {words}"
    );
    let mut given = before;
    given["cache"] = serde_json::json!("lost");
    ensure_eq!(
        serde_json::to_value(&status).map_err(|error| error.to_string())?["moves"],
        serde_json::json!([given]),
        "the move in the JSON form"
    );
    Ok(())
}

/// The fix `one`'s code under work-1 stopped at its usage limit and
/// started again on work-2, where the fake agent answers `carry`; its
/// status and that status in the JSON form.
fn moved_to_work_2(
    world: &World,
    carry: Carry,
) -> Result<(PlaceStatus, serde_json::Value), String> {
    fill_first_at_homes(world);
    world.agent.carry_to(&home("work-2"), carry);
    let id = begun_fix(world, "one")?;
    stops_at_limit(world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    let status = world.status(&id)?;
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    Ok((status, json))
}

/// The move of code in a place's status.
fn code_moved(status: &PlaceStatus) -> Result<&MoveStatus, String> {
    status
        .moves
        .iter()
        .find(|moved| {
            moved.moved.on
                == MoveOn::Stage {
                    stage: "code".into(),
                }
        })
        .ok_or_else(|| "status gives no move of code".to_string())
}

fn a_carried_agent(_: &()) -> Outcome {
    let world = World::new();
    let (status, json) = moved_to_work_2(
        &world,
        Carry::Yes {
            session: "s-1".into(),
        },
    )?;
    ensure_eq!(
        json["agent"]["conversation"],
        serde_json::json!({"from": 1, "carried": true}),
        "the agent's conversation in the JSON form"
    );
    ensure_eq!(
        json["moves"][0]["carried"],
        serde_json::json!(true),
        "code's move in the JSON form"
    );
    let words = code_moved(&status)?.words();
    ensure!(
        words.starts_with("code moved from work-1 to work-2")
            && words.contains("its conversation with it"),
        "code's move in the person's form: {words}"
    );
    ensure!(
        !json.to_string().contains("/homes"),
        "status gives a configuration home's path"
    );
    Ok(())
}

fn a_conversation_that_stayed(_: &()) -> Outcome {
    let world = World::new();
    let (status, json) = moved_to_work_2(
        &world,
        Carry::No {
            why: UNREACHED.into(),
        },
    )?;
    ensure_eq!(
        json["agent"]["conversation"],
        serde_json::json!({"from": 1, "carried": false, "why": UNREACHED}),
        "the agent's conversation in the JSON form"
    );
    ensure!(
        UNREACHED.contains("CLAUDE_CONFIG_DIR") && !UNREACHED.contains('/'),
        "the why names no variable or names a path"
    );
    ensure_eq!(
        json["moves"][0]["carried"],
        serde_json::json!(false),
        "code's move in the JSON form"
    );
    let words = code_moved(&status)?.words();
    ensure!(
        words.contains("its conversation stayed behind"),
        "code's move in the person's form: {words}"
    );
    ensure!(
        !json.to_string().contains("/homes"),
        "status gives a configuration home's path"
    );
    Ok(())
}

fn a_carried_start_that_did_not_begin(_: &()) -> Outcome {
    let world = World::new();
    world.agent.carried_starts_end();
    let (status, json) = moved_to_work_2(
        &world,
        Carry::Yes {
            session: "s-1".into(),
        },
    )?;
    let ended = "the start that carried its conversation ended before it said it started";
    ensure_eq!(
        status
            .uncarried
            .iter()
            .map(|read| (read.stage.as_str(), read.start, read.words.as_str()))
            .collect::<Vec<_>>(),
        vec![("code", 2, ended)],
        "the place's carried starts that did not begin"
    );
    ensure_eq!(
        (
            json["agent"]["identity"].clone(),
            json["agent"]["conversation"].clone()
        ),
        (
            serde_json::json!(World::identity("fix", "one", "code", 3).to_string()),
            serde_json::json!({"from": 1, "carried": false, "why": ended})
        ),
        "the agent of start 3 and its conversation in the JSON form"
    );
    ensure_eq!(
        json["uncarried"][0]["words"],
        serde_json::json!(ended),
        "the place's carried start that did not begin in the JSON form"
    );
    Ok(())
}

const PLANNER: &str = "madswan/planner";

/// The host's configuration gives `address` the standing agent `agent`
/// in the session `madswan`.
fn stands_in_madswan(world: &World, address: &str, agent: &str) {
    world.host.set_standing(
        address,
        Some(StandingAgent {
            agent: agent.into(),
            session: Some("madswan".into()),
            stream: None,
        }),
    );
}

/// Two fixes for the planner delivered, their submissions held, while no
/// agent of its name stands, so both ends stand pending for it.
fn two_pending_for_the_planner(world: &World) -> Result<Vec<PlaceId>, String> {
    stands_in_madswan(world, PLANNER, "madswan-planner");
    world
        .queue
        .plan_submits(vec![flume_fakes::Plan::Hold, flume_fakes::Plan::Hold]);
    let mut places = vec![];
    for name in ["one", "two"] {
        let mut launch = launch("fix", name, Some("mend it"));
        launch.recipients = vec![PLANNER.into()];
        launched(&world.engine.launch(&launch))?;
        let id = World::id("fix", name);
        world.settle(&id);
        super::settling::deliver_fix(world, &id);
        world.settle(&id);
        places.push(id);
    }
    Ok(places)
}

/// The host's addresses as status gives them, by the address.
fn host_addresses(world: &World) -> Result<Vec<AddressStatus>, String> {
    match world.engine().status(None) {
        Answer::Status { host, .. } => Ok(host.addresses),
        other => Err(format!("status answered {other:?}")),
    }
}

/// The address `address` as status gives it.
fn address_status(world: &World, address: &str) -> Result<AddressStatus, String> {
    host_addresses(world)?
        .into_iter()
        .find(|status| status.address == address)
        .ok_or_else(|| format!("status gives no {address}"))
}

/// What the terminal found of the address's standing agent.
fn found(status: &AddressStatus) -> Result<StandingFound, String> {
    status
        .standing
        .as_ref()
        .map(|standing| standing.found.clone())
        .ok_or_else(|| format!("{} has no standing", status.address))
}

fn a_standing_agent_found(_: &()) -> Outcome {
    let world = World::new();
    let places = two_pending_for_the_planner(&world)?;
    let at = world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Waiting);
    let status = address_status(&world, PLANNER)?;
    let standing = status.standing.clone().ok_or("no standing")?;
    ensure_eq!(standing.agent, "madswan-planner", "the agent");
    ensure_eq!(standing.stream, DEFAULT_STREAM, "the stream");
    ensure_eq!(
        standing.session,
        SessionHeld {
            name: Some("madswan".into()),
            from: SessionFrom::Role,
        },
        "the session"
    );
    ensure_eq!(standing.terminal.as_deref(), Some("fake"), "the terminal");
    ensure_eq!(
        standing.found,
        StandingFound::One {
            at,
            state: AgentState::Waiting,
        },
        "found"
    );
    ensure_eq!(
        status
            .pending
            .iter()
            .map(|end| (end.place.clone(), end.what.clone()))
            .collect::<Vec<_>>(),
        places
            .into_iter()
            .map(|id| (id, "code delivered".to_string()))
            .collect::<Vec<_>>(),
        "the ends pending"
    );
    ensure_eq!(status.role, None, "its run's entry");
    ensure_eq!(status.unwoken, None, "why it was not woken");
    ensure!(
        world.terminal.every_notice().is_empty(),
        "status sent a notice"
    );
    Ok(())
}

fn none_of_the_name(_: &()) -> Outcome {
    let world = World::new();
    two_pending_for_the_planner(&world)?;
    world
        .terminal
        .add_agent(Some("madswan"), "madswan-ops", AgentState::Waiting);
    let status = address_status(&world, PLANNER)?;
    ensure_eq!(found(&status)?, StandingFound::None, "found");
    ensure_eq!(status.pending.len(), 2, "the ends pending");
    Ok(())
}

fn more_than_one_of_the_name(_: &()) -> Outcome {
    let world = World::new();
    two_pending_for_the_planner(&world)?;
    for _ in 0..2 {
        world
            .terminal
            .add_agent(Some("madswan"), "madswan-planner", AgentState::Waiting);
    }
    let status = address_status(&world, PLANNER)?;
    ensure_eq!(
        found(&status)?,
        StandingFound::Several { count: 2 },
        "found"
    );
    Ok(())
}

fn a_part_not_read(_: &()) -> Outcome {
    let world = World::new();
    stands_in_madswan(&world, PLANNER, "madswan-planner");
    world
        .terminal
        .refuse_agents_in(Some("madswan"), "the part madswan answered nothing");
    let status = address_status(&world, PLANNER)?;
    ensure_eq!(
        found(&status)?,
        StandingFound::Unread {
            reason: "the part madswan answered nothing".into(),
        },
        "found"
    );
    Ok(())
}

fn a_standing_address_with_nothing_pending(_: &()) -> Outcome {
    let world = World::new();
    stands_in_madswan(&world, "madswan/ops", "madswan-ops");
    world
        .terminal
        .add_agent(Some("madswan"), "madswan-ops", AgentState::Working);
    let status = address_status(&world, "madswan/ops")?;
    ensure_eq!(status.pending.len(), 0, "the ends pending");
    let standing = status.standing.ok_or("no standing")?;
    ensure_eq!(standing.agent, "madswan-ops", "the agent");
    ensure!(
        matches!(
            standing.found,
            StandingFound::One {
                state: AgentState::Working,
                ..
            }
        ),
        "found: {:?}",
        standing.found
    );
    Ok(())
}

fn two_addresses_in_one_session(_: &()) -> Outcome {
    let world = World::new();
    stands_in_madswan(&world, PLANNER, "madswan-planner");
    stands_in_madswan(&world, "madswan/ops", "madswan-ops");
    world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Waiting);
    world
        .terminal
        .add_agent(Some("madswan"), "madswan-ops", AgentState::Asking);
    world.terminal.log.clear();
    let addresses = host_addresses(&world)?;
    ensure_eq!(
        addresses
            .iter()
            .map(|status| status.address.as_str())
            .collect::<Vec<_>>(),
        vec!["madswan/ops", PLANNER],
        "the addresses"
    );
    for status in &addresses {
        ensure!(
            matches!(found(status)?, StandingFound::One { .. }),
            "{} found: {:?}",
            status.address,
            status.standing
        );
    }
    ensure_eq!(
        world.terminal.log.of("agents_in"),
        vec!["agents_in madswan".to_string()],
        "the parts the terminal was asked for"
    );
    Ok(())
}

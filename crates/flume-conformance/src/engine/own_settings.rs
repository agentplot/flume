//! A program's own settings (record 0043; core.50): a table of programs
//! in every layer, each entry a named file and keys in its program's own
//! words, only the agent's program's given its profile; each top-level
//! key taken whole from the first of the eight layers that gives it, a
//! stage's table before every stage's and that before a repository's for
//! every kind, a table's keys before its file's; the host's layers read
//! at each start and every other as the launch read it; nothing passed
//! where nothing is given; a key the profile reserves refused where a
//! table writes it and left out of a named file unread; a value shaped as
//! a credential refused before anything holds it, naming its key's path,
//! its layer and its file and never the value; and keys the profile gives
//! no way to take refused naming each with its layer (4, 8, 21, 29, 32,
//! 56, 88, 143, 157). The readers' own refusals, of an entry's key beside
//! its file, a date and a host's file that cannot be read, are the host's
//! configuration's and its profile's tests; a declaration's file outside
//! its repository is read here as the kinds seam refuses it.

use super::agent_settings::{for_every_stage, opened, to_code, to_verify};
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_fakes::FakeAgent;
use flume_seams::*;
use serde_json::{json, Value};
use std::collections::BTreeMap;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "own settings: only the agent's program's entry reaches its profile, status naming its keys with their layer and nothing of another program's",
            cites: &["32", "56", "88"],
            run: two_programs_entries,
        },
        Scenario {
            name: "own settings: a table's file lies beneath its own keys, status naming the file's key with the file and the table's key with none",
            cites: &["32", "56"],
            run: a_file_beneath_its_layers_own_keys,
        },
        Scenario {
            name: "own settings: a stage's table, its file among it, comes before the kind's table for every stage",
            cites: &["32"],
            run: a_stages_file_before_every_stages_own_keys,
        },
        Scenario {
            name: "own settings: the host's file changing while code works leaves code's agent its keys and gives verify the new ones",
            cites: &["8", "32"],
            run: a_hosts_file_read_at_each_start,
        },
        Scenario {
            name: "own settings: a launch's file is kept as the launch read it, every stage given its keys and status naming it by its absolute path",
            cites: &["21", "32"],
            run: a_launchs_file_read_at_the_launch,
        },
        Scenario {
            name: "own settings: a repository's file is kept at the revision the launch read, every stage given its keys from repository with the file",
            cites: &["21", "157"],
            run: a_repositorys_file_at_the_launchs_revision,
        },
        Scenario {
            name: "own settings: a declaration whose file lies outside its repository cannot be read, and nothing is launched",
            cites: &["4", "157"],
            run: a_file_outside_its_repository,
        },
        Scenario {
            name: "own settings: keys from the host's table for the place's stream, for every agent and the repository's, each with its layer, and the host's own key in another stream",
            cites: &["32", "143", "56"],
            run: keys_from_three_layers,
        },
        Scenario {
            name: "own settings: a key is taken whole from the first layer that gives it, never merged with another's value of it",
            cites: &["32"],
            run: a_key_taken_whole,
        },
        Scenario {
            name: "own settings: a table's empty table replaces its file's whole",
            cites: &["32"],
            run: a_files_table_replaced_by_an_empty_one,
        },
        Scenario {
            name: "own settings: the host's table changing while code works leaves code's agent its keys and gives verify the new ones",
            cites: &["8", "32"],
            run: the_hosts_table_changes_while_a_stage_works,
        },
        Scenario {
            name: "own settings: the host's table for every kind of a repository gives a fix and a chore there its keys from host-repository, and a repository it does not name the host's for the kind",
            cites: &["32", "56"],
            run: the_hosts_word_for_every_kind_of_a_repository,
        },
        Scenario {
            name: "own settings: the host's table for one kind of a repository comes before its table for every kind there",
            cites: &["32"],
            run: the_hosts_word_for_one_kind_before_every_kind,
        },
        Scenario {
            name: "own settings: a repository's [agents] gives every kind run in it its keys, beneath its table for the kind, each from repository",
            cites: &["32", "56"],
            run: the_repositorys_own_file_for_every_kind,
        },
        Scenario {
            name: "own settings: the host's table for every agent comes before a repository's [agents]",
            cites: &["32"],
            run: a_host_table_before_the_repositorys_file,
        },
        Scenario {
            name: "own settings: a repository's [agents] changed on the base after the launch leaves every stage the keys the launch read",
            cites: &["21", "32"],
            run: the_repositorys_file_changes_after_the_launch,
        },
        Scenario {
            name: "own settings: where no layer gives an entry the command passes none and lists no file, and the start holds none",
            cites: &["32", "88", "162"],
            run: nothing_where_no_layer_gives_own_settings,
        },
        Scenario {
            name: "own settings: a named file that gives no key passes nothing and lists no file, the start naming the file",
            cites: &["32", "88"],
            run: a_file_that_gives_no_key,
        },
        Scenario {
            name: "own settings: a reserved key the launch gives refuses it naming the key, the program, every stage, the launch and its owner, and nothing is made",
            cites: &["4", "32"],
            run: a_reserved_key_at_the_launch,
        },
        Scenario {
            name: "own settings: a reserved key the host's table gives after the launch refuses the next stage naming it and host, no agent starting, until a retry after the table is mended",
            cites: &["27", "32"],
            run: a_reserved_key_after_the_launch,
        },
        Scenario {
            name: "own settings: a people's file's reserved keys are left out, a nested one out of the table holding it, the rest kept and the launch going on",
            cites: &["29", "32", "162"],
            run: a_peoples_file_with_keys_another_setting_owns,
        },
        Scenario {
            name: "own settings: a credential under a key left out of a file is never read, and nothing of it is in status, a log line or held state",
            cites: &["29", "56"],
            run: a_credential_under_a_key_left_out,
        },
        Scenario {
            name: "own settings: keys for a profile that keeps the seam's default refuse the launch naming the program, the key and the layer",
            cites: &["4", "88"],
            run: a_profile_that_keeps_the_seams_default,
        },
        Scenario {
            name: "own settings: an entry for a program no stage runs reaches nothing, and the place launches",
            cites: &["32", "88"],
            run: an_entry_for_a_program_no_stage_runs,
        },
        Scenario {
            name: "own settings: a token in a launch refuses it naming the key and launch, saying a credential goes in the account's env, its words holding nothing of it, and nothing is made",
            cites: &["4", "29"],
            run: a_token_in_a_launch,
        },
        Scenario {
            name: "own settings: a token in the host's file refuses every launch and every later start, naming its key's path, host and the file, and never the token",
            cites: &["4", "27", "29"],
            run: a_token_in_a_hosts_file,
        },
        Scenario {
            name: "own settings: a secret-named key's value in the host's table refuses the launch naming its path and host, and never the value",
            cites: &["4", "29"],
            run: a_secret_named_key_in_the_hosts_configuration,
        },
        Scenario {
            name: "own settings: a secret-named key naming a variable rather than a value is taken as written",
            cites: &["29", "32"],
            run: a_variable_named_rather_than_a_value,
        },
        Scenario {
            name: "own settings: a credential in a launch's entry for a program no stage runs refuses the launch, and nothing is made or held",
            cites: &["4", "29"],
            run: a_credential_in_an_entry_no_stage_runs,
        },
        Scenario {
            name: "own settings: a home and a variable in a value reach the profile as written",
            cites: &["29", "32"],
            run: a_home_and_a_variable_in_a_command,
        },
        Scenario {
            name: "own settings: a named file is never passed as it stands, the command listing its own file under the start's folder",
            cites: &["32", "88", "128"],
            run: a_named_file_not_passed,
        },
        Scenario {
            name: "own settings: the fake scripted to take them by a file and reserve env is given the inline keys and not a file's env, its file written before the open, and env given inline refused naming its owner",
            cites: &["88", "106", "128"],
            run: a_scripted_way_and_a_reserved_key,
        },
    ]
}

/// `program`'s keys given inline.
pub(super) fn inline(program: &str, keys: Value) -> Settings {
    Settings {
        programs: BTreeMap::from([(
            program.to_string(),
            ProgramSettings {
                file: None,
                settings: table(keys),
            },
        )]),
        ..Settings::default()
    }
}

/// `program`'s keys given by the named file at `read` holding `keys`,
/// with `settings` inline over it.
pub(super) fn by_file(program: &str, read: &str, keys: Value, settings: Value) -> Settings {
    Settings {
        programs: BTreeMap::from([(
            program.to_string(),
            ProgramSettings {
                file: Some(NamedFile {
                    written: read.rsplit('/').next().unwrap_or(read).to_string(),
                    read: read.to_string(),
                    keys: table(keys),
                    left_out: vec![],
                }),
                settings: table(settings),
            },
        )]),
        ..Settings::default()
    }
}

/// A JSON object as a table of keys.
fn table(keys: Value) -> BTreeMap<String, Value> {
    serde_json::from_value(keys).expect("a JSON object")
}

/// The host's file every scenario names, as its reader read it.
pub(super) const HOST_FILE: &str = "/home/user/.config/flume/agent.json";

/// The own settings the fake's command passed the start under
/// `identity`: the JSON object of the file it names by `--own`, none
/// where it names none.
pub(super) fn own_passed(world: &World, identity: &Identity) -> Result<Option<Value>, String> {
    let command = opened(world, identity)?;
    let Some(file) = command.args.iter().skip_while(|arg| *arg != "--own").nth(1) else {
        return Ok(None);
    };
    let text = command
        .files
        .get(file)
        .ok_or_else(|| format!("{identity}'s command names {file} and lists no such file"))?;
    serde_json::from_str(text)
        .map(Some)
        .map_err(|error| format!("{identity}'s own file is no JSON: {error}"))
}

/// Where the current agent's own setting `key` of `agent`'s came from.
pub(super) fn key_from(world: &World, id: &PlaceId, key: &str) -> Result<Option<Given>, String> {
    Ok(world
        .status(id)?
        .agent
        .ok_or("no agent")?
        .settings
        .from
        .get(&format!("programs.agent.settings.{key}"))
        .cloned())
}

/// A key's layer and its value as JSON, with its file where it came from
/// one.
pub(super) fn given(layer: Layer, value: Value, file: Option<&str>) -> Option<Given> {
    Some(Given {
        layer,
        value: value.to_string(),
        file: file.map(str::to_string),
    })
}

/// The start of the stage `stage` as held.
fn start_of(world: &World, id: &PlaceId, stage: &str) -> Result<StartHeld, String> {
    world
        .held(id)
        .ok_or("no held state")?
        .stages
        .into_iter()
        .find(|held| held.name == stage)
        .and_then(|held| held.start)
        .ok_or_else(|| format!("no start of {stage}"))
}

/// Nothing made for a refused launch: no place, and no held state for
/// `id`.
fn nothing_made(world: &World, id: &PlaceId) -> Outcome {
    ensure_eq!(world.place.log.count("make"), 0, "places made");
    ensure!(world.held(id).is_none(), "held state written");
    Ok(())
}

/// A fix `name` launched and its code begun.
fn fix_begun(world: &World, launch: &Launch) -> Result<PlaceId, String> {
    launched(&world.engine.launch(launch))?;
    let id = World::id(&launch.kind, &launch.name);
    world.settle(&id);
    Ok(id)
}

/// A twenty-character run of letters and digits, which follows a token's
/// prefix in every scenario of a credential.
const TWENTY: &str = "a1B2c3D4e5F6g7H8i9J0";

fn two_programs_entries(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    let mut every = inline("agent", json!({ "line": "a" }));
    every
        .programs
        .extend(inline("other", json!({ "line": "b" })).programs);
    world.host.set_every_agent(every);
    let id = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    let code = World::identity("fix", "one", "code", 1);
    ensure_eq!(
        own_passed(&world, &code)?,
        Some(json!({ "line": "a" })),
        "the keys passed"
    );
    ensure_eq!(
        world.agent.log.of("own"),
        vec![format!("own {code}: line")],
        "the keys the fake was given"
    );
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    ensure_eq!(
        agent
            .settings
            .from
            .get("programs.agent.settings.line")
            .cloned(),
        given(Layer::Host, json!("a"), None),
        "where line came from"
    );
    ensure!(
        !agent.settings.from.keys().any(|key| key.contains("other")),
        "status names the other program: {:?}",
        agent.settings.from.keys()
    );
    ensure_eq!(
        agent.settings.settings.programs.keys().collect::<Vec<_>>(),
        vec!["agent"],
        "the programs status gives"
    );
    Ok(())
}

fn a_file_beneath_its_layers_own_keys(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world.host.set_every_agent(by_file(
        "agent",
        HOST_FILE,
        json!({ "tui": "fullscreen", "line": "file" }),
        json!({ "line": "inline" }),
    ));
    let id = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    ensure_eq!(
        own_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some(json!({ "tui": "fullscreen", "line": "inline" })),
        "the keys passed"
    );
    ensure_eq!(
        key_from(&world, &id, "tui")?,
        given(Layer::Host, json!("fullscreen"), Some(HOST_FILE)),
        "where tui came from"
    );
    ensure_eq!(
        key_from(&world, &id, "line")?,
        given(Layer::Host, json!("inline"), None),
        "where line came from"
    );
    Ok(())
}

/// flume's own change@1 as the repository declares it, its agents given
/// `agents`.
fn change_with(world: &World, agents: KindSettings) -> Result<(), String> {
    let mut change = crate::fake_harness::flumes_own()
        .into_iter()
        .find(|kind| kind.name == "change" && kind.version == 1)
        .ok_or("flume's own kinds hold no change@1")?;
    change.agents = agents;
    world.kinds.set_kinds(REPOSITORY, vec![change]);
    Ok(())
}

fn a_stages_file_before_every_stages_own_keys(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    change_with(
        &world,
        KindSettings {
            every_stage: inline("agent", json!({ "tui": "every" })),
            stages: BTreeMap::from([(
                "code".to_string(),
                by_file(
                    "agent",
                    "/repository/kinds/agent-code.json",
                    json!({ "tui": "code" }),
                    json!({}),
                ),
            )]),
        },
    )?;
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    to_code(&world, &id, "plan")?;
    ensure_eq!(
        own_passed(&world, &World::identity("change", "plan", "code", 1))?,
        Some(json!({ "tui": "code" })),
        "code's keys"
    );
    ensure_eq!(
        key_from(&world, &id, "tui")?,
        given(
            Layer::Kind,
            json!("code"),
            Some("/repository/kinds/agent-code.json")
        ),
        "where code's tui came from"
    );
    to_verify(&world, &id, "plan")?;
    ensure_eq!(
        own_passed(&world, &World::identity("change", "plan", "verify", 1))?,
        Some(json!({ "tui": "every" })),
        "verify's keys"
    );
    ensure_eq!(
        key_from(&world, &id, "tui")?,
        given(Layer::Kind, json!("every"), None),
        "where verify's tui came from"
    );
    Ok(())
}

fn a_hosts_file_read_at_each_start(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world.host.set_every_agent(by_file(
        "agent",
        HOST_FILE,
        json!({ "tui": "fullscreen" }),
        json!({}),
    ));
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    to_code(&world, &id, "plan")?;
    world.host.set_every_agent(by_file(
        "agent",
        HOST_FILE,
        json!({ "tui": "default" }),
        json!({}),
    ));
    world.settle(&id);
    let code = start_of(&world, &id, "code")?;
    ensure_eq!(
        code.settings.programs.get("agent").map(|own| &own.settings),
        Some(&table(json!({ "tui": "fullscreen" }))),
        "code's keys after the file changed"
    );
    to_verify(&world, &id, "plan")?;
    ensure_eq!(
        own_passed(&world, &World::identity("change", "plan", "verify", 1))?,
        Some(json!({ "tui": "default" })),
        "verify's keys"
    );
    Ok(())
}

fn a_launchs_file_read_at_the_launch(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    let file = "/work/caller/agent.json";
    let mut change = launch("change", "plan", Some("plan it"));
    change.every_stage = by_file("agent", file, json!({ "tui": "fullscreen" }), json!({}));
    launched(&world.engine.launch(&change))?;
    let id = World::id("change", "plan");
    world.settle(&id);
    for stage in ["construct", "code"] {
        if stage == "code" {
            to_code(&world, &id, "plan")?;
        }
        ensure_eq!(
            own_passed(&world, &World::identity("change", "plan", stage, 1))?,
            Some(json!({ "tui": "fullscreen" })),
            format!("{stage}'s keys")
        );
        ensure_eq!(
            key_from(&world, &id, "tui")?,
            given(Layer::Launch, json!("fullscreen"), Some(file)),
            format!("where {stage}'s tui came from")
        );
    }
    let held = world.held(&id).ok_or("no held state")?;
    ensure_eq!(
        held.place.every_stage.programs["agent"]
            .file
            .as_ref()
            .map(|file| file.keys.clone()),
        Some(table(json!({ "tui": "fullscreen" }))),
        "the launch's file as held"
    );
    Ok(())
}

fn a_repositorys_file_at_the_launchs_revision(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    let file = ".config/agent-change.json";
    let base = world.history.head(REPOSITORY, "main").ok_or("no main")?;
    world.kinds.set_settings_at(
        REPOSITORY,
        &base,
        "change",
        for_every_stage(by_file("agent", file, json!({ "tui": "first" }), json!({}))),
    );
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    let moved = world.commit("main", &[file]);
    world.kinds.set_settings_at(
        REPOSITORY,
        &moved,
        "change",
        for_every_stage(by_file("agent", file, json!({ "tui": "moved" }), json!({}))),
    );
    world.settle(&id);
    for stage in ["construct", "code"] {
        if stage == "code" {
            to_code(&world, &id, "plan")?;
        }
        ensure_eq!(
            own_passed(&world, &World::identity("change", "plan", stage, 1))?,
            Some(json!({ "tui": "first" })),
            format!("{stage}'s keys")
        );
        ensure_eq!(
            key_from(&world, &id, "tui")?,
            given(Layer::Repository, json!("first"), Some(file)),
            format!("where {stage}'s tui came from")
        );
    }
    Ok(())
}

fn a_file_outside_its_repository(_: &()) -> Outcome {
    let world = World::new();
    let words =
        "kinds.fix.agents.programs.agent.file: ../../agent.json lies outside the repository";
    world.kinds.unreadable_kinds(REPOSITORY, words);
    let reason = refused(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    ensure!(
        reason.contains("kinds.fix.agents.programs.agent.file")
            && reason.contains("outside the repository"),
        "the refusal: {reason}"
    );
    nothing_made(&world, &World::id("fix", "one"))
}

/// The host's stream `work`, declared.
fn work(world: &World) {
    world.host.set_stream("work", Some(Stream::default()));
}

/// A launch of a fix `name` in the stream `stream`.
fn fix_in(name: &str, stream: Option<&str>) -> Launch {
    Launch {
        stream: stream.map(str::to_string),
        ..launch("fix", name, Some("mend it"))
    }
}

fn keys_from_three_layers(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    work(&world);
    world
        .host
        .set_every_agent(inline("agent", json!({ "line": "host", "tips": true })));
    world
        .host
        .set_stream_settings("work", inline("agent", json!({ "line": "work" })));
    world.kinds.set_settings(
        REPOSITORY,
        "fix",
        for_every_stage(inline("agent", json!({ "style": "plain" }))),
    );
    let in_work = fix_begun(&world, &fix_in("one", Some("work")))?;
    ensure_eq!(
        own_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some(json!({ "line": "work", "tips": true, "style": "plain" })),
        "the keys in work"
    );
    for (key, from) in [
        ("line", given(Layer::HostStream, json!("work"), None)),
        ("tips", given(Layer::Host, json!(true), None)),
        ("style", given(Layer::Repository, json!("plain"), None)),
    ] {
        ensure_eq!(
            key_from(&world, &in_work, key)?,
            from,
            format!("{key} in work")
        );
    }
    let in_default = fix_begun(&world, &fix_in("two", None))?;
    ensure_eq!(
        key_from(&world, &in_default, "line")?,
        given(Layer::Host, json!("host"), None),
        "line in default"
    );
    Ok(())
}

fn a_key_taken_whole(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    work(&world);
    world.host.set_every_agent(by_file(
        "agent",
        HOST_FILE,
        json!({ "status": { "command": "a", "padding": 1 } }),
        json!({}),
    ));
    world.host.set_stream_settings(
        "work",
        inline("agent", json!({ "status": { "command": "b" } })),
    );
    let id = fix_begun(&world, &fix_in("one", Some("work")))?;
    ensure_eq!(
        own_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some(json!({ "status": { "command": "b" } })),
        "the keys passed"
    );
    ensure_eq!(
        key_from(&world, &id, "status")?,
        given(Layer::HostStream, json!({ "command": "b" }), None),
        "where status came from"
    );
    Ok(())
}

fn a_files_table_replaced_by_an_empty_one(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world.host.set_every_agent(by_file(
        "agent",
        HOST_FILE,
        json!({ "hooks": { "Start": [{ "command": "a" }], "Stop": [{ "command": "b" }] } }),
        json!({ "hooks": {} }),
    ));
    let id = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    ensure_eq!(
        own_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some(json!({ "hooks": {} })),
        "the keys passed"
    );
    ensure_eq!(
        key_from(&world, &id, "hooks")?,
        given(Layer::Host, json!({}), None),
        "where hooks came from"
    );
    Ok(())
}

fn the_hosts_table_changes_while_a_stage_works(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world
        .host
        .set_every_agent(inline("agent", json!({ "line": "before" })));
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    to_code(&world, &id, "plan")?;
    let closes = world.terminal.log.count("close");
    world
        .host
        .set_every_agent(inline("agent", json!({ "line": "after" })));
    world.settle(&id);
    ensure_eq!(
        (
            world.terminal.log.count("close"),
            world.stage_state(&id, "code")
        ),
        (closes, Some(StageState::Begun)),
        "agents ended and code's state after the host's table changed"
    );
    ensure_eq!(
        start_of(&world, &id, "code")?.from["programs.agent.settings.line"].value,
        json!("before").to_string(),
        "code's line as its start took it"
    );
    to_verify(&world, &id, "plan")?;
    ensure_eq!(
        own_passed(&world, &World::identity("change", "plan", "verify", 1))?,
        Some(json!({ "line": "after" })),
        "verify's keys"
    );
    Ok(())
}

fn the_hosts_word_for_every_kind_of_a_repository(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world.host.set_settings(
        None,
        "fix",
        for_every_stage(inline("agent", json!({ "line": "kind" }))),
    );
    world
        .host
        .set_every_kind("/elsewhere", inline("agent", json!({ "line": "repo" })));
    let elsewhere = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    ensure_eq!(
        key_from(&world, &elsewhere, "line")?,
        given(Layer::HostKind, json!("kind"), None),
        "a fix where the host names another repository"
    );
    world
        .host
        .set_every_kind(REPOSITORY, inline("agent", json!({ "line": "repo" })));
    for kind in ["fix", "chore"] {
        let id = fix_begun(&world, &launch(kind, "two", Some("mend it")))?;
        ensure_eq!(
            key_from(&world, &id, "line")?,
            given(Layer::HostRepository, json!("repo"), None),
            format!("a {kind} in the repository the host names")
        );
    }
    Ok(())
}

fn the_hosts_word_for_one_kind_before_every_kind(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world.host.set_settings(
        Some(REPOSITORY),
        "fix",
        for_every_stage(inline("agent", json!({ "line": "fix" }))),
    );
    world
        .host
        .set_every_kind(REPOSITORY, inline("agent", json!({ "line": "repo" })));
    for (kind, line) in [("fix", "fix"), ("chore", "repo")] {
        let id = fix_begun(&world, &launch(kind, "one", Some("mend it")))?;
        ensure_eq!(
            key_from(&world, &id, "line")?,
            given(Layer::HostRepository, json!(line), None),
            format!("a {kind}'s line")
        );
    }
    Ok(())
}

fn the_repositorys_own_file_for_every_kind(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world
        .kinds
        .set_every_kind(REPOSITORY, inline("agent", json!({ "line": "ours" })));
    world.kinds.set_settings(
        REPOSITORY,
        "fix",
        for_every_stage(inline("agent", json!({ "tips": true }))),
    );
    let fix = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    ensure_eq!(
        own_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some(json!({ "line": "ours", "tips": true })),
        "the fix's keys"
    );
    for (key, value) in [("line", json!("ours")), ("tips", json!(true))] {
        ensure_eq!(
            key_from(&world, &fix, key)?,
            given(Layer::Repository, value, None),
            format!("where the fix's {key} came from")
        );
    }
    let chore = fix_begun(&world, &launch("chore", "one", Some("tidy it")))?;
    ensure_eq!(
        own_passed(&world, &World::identity("chore", "one", "code", 1))?,
        Some(json!({ "line": "ours" })),
        "the chore's keys"
    );
    ensure_eq!(
        key_from(&world, &chore, "line")?,
        given(Layer::Repository, json!("ours"), None),
        "where the chore's line came from"
    );
    Ok(())
}

fn a_host_table_before_the_repositorys_file(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world
        .host
        .set_every_agent(inline("agent", json!({ "line": "host" })));
    world
        .kinds
        .set_every_kind(REPOSITORY, inline("agent", json!({ "line": "ours" })));
    let id = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    ensure_eq!(
        key_from(&world, &id, "line")?,
        given(Layer::Host, json!("host"), None),
        "where line came from"
    );
    Ok(())
}

fn the_repositorys_file_changes_after_the_launch(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    let base = world.history.head(REPOSITORY, "main").ok_or("no main")?;
    world.kinds.set_every_kind_at(
        REPOSITORY,
        &base,
        inline("agent", json!({ "line": "first" })),
    );
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    let moved = world.commit("main", &[".config/flume.toml"]);
    world.kinds.set_every_kind_at(
        REPOSITORY,
        &moved,
        inline("agent", json!({ "line": "moved" })),
    );
    world.settle(&id);
    to_code(&world, &id, "plan")?;
    ensure_eq!(
        own_passed(&world, &World::identity("change", "plan", "code", 1))?,
        Some(json!({ "line": "first" })),
        "code's keys"
    );
    ensure_eq!(
        key_from(&world, &id, "line")?,
        given(Layer::Repository, json!("first"), None),
        "where code's line came from"
    );
    Ok(())
}

fn nothing_where_no_layer_gives_own_settings(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    let id = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    let command = opened(&world, &World::identity("fix", "one", "code", 1))?;
    ensure!(
        !command.args.iter().any(|arg| arg == "--own"),
        "the command passes own settings: {:?}",
        command.args
    );
    ensure!(
        command.files.is_empty(),
        "files listed: {:?}",
        command.files
    );
    ensure_eq!(world.agent.log.count("own"), 0, "keys the fake was given");
    let start = start_of(&world, &id, "code")?;
    ensure_eq!(start.own, None, "the start's own settings");
    ensure!(
        start.settings.programs.is_empty(),
        "the start's programs: {:?}",
        start.settings.programs
    );
    Ok(())
}

fn a_file_that_gives_no_key(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world
        .host
        .set_every_agent(by_file("agent", HOST_FILE, json!({}), json!({})));
    let id = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    let command = opened(&world, &World::identity("fix", "one", "code", 1))?;
    ensure!(
        command.files.is_empty(),
        "files listed: {:?}",
        command.files
    );
    ensure_eq!(world.agent.log.count("own"), 0, "keys the fake was given");
    let own = start_of(&world, &id, "code")?
        .own
        .ok_or("no own settings held")?;
    ensure_eq!(
        own.files
            .iter()
            .map(|file| (file.layer, file.read.as_str()))
            .collect::<Vec<_>>(),
        vec![(Layer::Host, HOST_FILE)],
        "the files the start names"
    );
    ensure_eq!(own.passed, None, "how they were passed");
    Ok(())
}

/// The fake reserving `env` for the account and `perms.mode` for flume's
/// mode.
fn reserving(world: &World) {
    world.agent.take_own();
    world.agent.reserve("env", "the account");
    world.agent.reserve("perms.mode", "flume's mode");
}

fn a_reserved_key_at_the_launch(_: &()) -> Outcome {
    let world = World::new();
    reserving(&world);
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = inline("agent", json!({ "env": { "X": "1" } }));
    let reason = refused(&world.engine.launch(&fix))?;
    for named in [
        "`programs.agent.settings.env`",
        "`agent`",
        "for every stage",
        "`launch`",
        "the account",
    ] {
        ensure!(reason.contains(named), "{named} in the refusal: {reason}");
    }
    nothing_made(&world, &World::id("fix", "one"))
}

fn a_reserved_key_after_the_launch(_: &()) -> Outcome {
    let world = World::new();
    reserving(&world);
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    to_code(&world, &id, "plan")?;
    world
        .host
        .set_every_agent(inline("agent", json!({ "env": { "X": "1" } })));
    world.terminal.log.clear();
    world.tick("change/plan", "plan", 1, 1);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::Refused),
        "verify"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    let refusal = world.status(&id)?.stages[2]
        .refused_setting
        .clone()
        .ok_or("no setting named")?;
    ensure!(
        refusal.contains("`programs.agent.settings.env`") && refusal.contains("`host`"),
        "the refusal: {refusal}"
    );
    world.host.set_every_agent(Settings::default());
    let answer = world
        .engine()
        .retry(&id, Retry::Stage("verify".into()), None, false);
    ensure!(answer.refusal().is_none(), "the retry: {answer:?}");
    opened(&world, &World::identity("change", "plan", "verify", 1))?;
    Ok(())
}

fn a_peoples_file_with_keys_another_setting_owns(_: &()) -> Outcome {
    let world = World::new();
    reserving(&world);
    world.host.set_every_agent(by_file(
        "agent",
        HOST_FILE,
        json!({
            "env": { "PEOPLE": "1" },
            "perms": { "mode": "full", "allow": ["a"] },
            "line": "people",
        }),
        json!({}),
    ));
    let id = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    ensure_eq!(
        own_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some(json!({ "perms": { "allow": ["a"] }, "line": "people" })),
        "the keys passed"
    );
    let own = start_of(&world, &id, "code")?
        .own
        .ok_or("no own settings held")?;
    ensure_eq!(
        own.files
            .iter()
            .map(|file| (file.layer, file.left_out.clone()))
            .collect::<Vec<_>>(),
        vec![(Layer::Host, vec!["env".to_string(), "perms.mode".into()])],
        "the keys left out of the file"
    );
    ensure!(
        key_from(&world, &id, "env")?.is_none(),
        "status names env as given"
    );
    Ok(())
}

/// Every line every fake of `world` logged.
fn every_log_line(world: &World) -> Vec<String> {
    [
        world.agent.log.calls(),
        world.terminal.log.calls(),
        world.store.log.calls(),
        world.host.log.calls(),
        world.kinds.log.calls(),
        world.place.log.calls(),
        world.queue.log.calls(),
    ]
    .concat()
}

/// Whether `text` holds any part of the twenty characters a token's
/// prefix is followed by in these scenarios.
fn holds_the_token(text: &str) -> bool {
    text.contains(TWENTY) || text.contains(&TWENTY[..10]) || text.contains(&TWENTY[10..])
}

fn a_credential_under_a_key_left_out(_: &()) -> Outcome {
    let world = World::new();
    reserving(&world);
    world.host.set_every_agent(by_file(
        "agent",
        HOST_FILE,
        json!({ "env": { "TOKEN": format!("ghp_{TWENTY}") }, "line": "people" }),
        json!({}),
    ));
    let id = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    let status = serde_json::to_string(&world.status(&id)?).map_err(|error| error.to_string())?;
    ensure!(!holds_the_token(&status), "the token in status: {status}");
    let held = serde_json::to_string(&world.held(&id).ok_or("no held state")?)
        .map_err(|error| error.to_string())?;
    ensure!(!holds_the_token(&held), "the token in held state: {held}");
    let logged = every_log_line(&world);
    ensure!(
        !logged.iter().any(|line| holds_the_token(line)),
        "the token in a log line: {logged:?}"
    );
    Ok(())
}

fn a_profile_that_keeps_the_seams_default(_: &()) -> Outcome {
    let world = World::new();
    world
        .host
        .set_every_agent(inline("agent", json!({ "line": "a" })));
    let reason = refused(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    for named in ["`agent`", "`line`", "`host`", NO_OWN] {
        ensure!(reason.contains(named), "{named} in the refusal: {reason}");
    }
    nothing_made(&world, &World::id("fix", "one"))
}

fn an_entry_for_a_program_no_stage_runs(_: &()) -> Outcome {
    let world = World::new();
    world
        .host
        .set_every_agent(inline("other", json!({ "line": "b" })));
    let id = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    let command = opened(&world, &World::identity("fix", "one", "code", 1))?;
    ensure!(
        command.files.is_empty(),
        "files listed: {:?}",
        command.files
    );
    ensure_eq!(world.agent.log.count("own"), 0, "keys the fake was given");
    ensure_eq!(start_of(&world, &id, "code")?.own, None, "the start's own");
    Ok(())
}

fn a_token_in_a_launch(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = inline("agent", json!({ "line": format!("ghp_{TWENTY}") }));
    let reason = refused(&world.engine.launch(&fix))?;
    ensure!(
        reason.contains("`programs.agent.settings.line`")
            && reason.contains("`launch`")
            && reason.contains("account's `env`"),
        "the refusal: {reason}"
    );
    ensure!(
        !holds_the_token(&reason),
        "the token in the refusal: {reason}"
    );
    nothing_made(&world, &World::id("fix", "one"))
}

fn a_token_in_a_hosts_file(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    let tokened = by_file(
        "agent",
        HOST_FILE,
        json!({ "line": { "command": format!("run --key sk-{TWENTY}") } }),
        json!({}),
    );
    world.host.set_every_agent(tokened.clone());
    let reason = refused(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    for named in [
        "`programs.agent.settings.line.command`",
        "`host`",
        HOST_FILE,
    ] {
        ensure!(reason.contains(named), "{named} in the refusal: {reason}");
    }
    ensure!(
        !holds_the_token(&reason),
        "the token in the refusal: {reason}"
    );
    nothing_made(&world, &World::id("fix", "one"))?;
    world.host.set_every_agent(Settings::default());
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    to_code(&world, &id, "plan")?;
    world.host.set_every_agent(tokened);
    world.tick("change/plan", "plan", 1, 1);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::Refused),
        "verify"
    );
    let refusal = world.status(&id)?.stages[2]
        .refused_setting
        .clone()
        .ok_or("no setting named")?;
    ensure!(
        refusal.contains("`programs.agent.settings.line.command`") && refusal.contains(HOST_FILE),
        "the later refusal: {refusal}"
    );
    let held = serde_json::to_string(&world.held(&id).ok_or("no held state")?)
        .map_err(|error| error.to_string())?;
    ensure!(!holds_the_token(&held), "the token in held state: {held}");
    Ok(())
}

fn a_secret_named_key_in_the_hosts_configuration(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world.host.set_every_agent(inline(
        "agent",
        json!({ "headers": { "Authorization": "Bearer abc" } }),
    ));
    let reason = refused(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    ensure!(
        reason.contains("`programs.agent.settings.headers.Authorization`")
            && reason.contains("`host`"),
        "the refusal: {reason}"
    );
    ensure!(
        !reason.contains("abc"),
        "the value in the refusal: {reason}"
    );
    nothing_made(&world, &World::id("fix", "one"))
}

fn a_variable_named_rather_than_a_value(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    let headers = json!({ "Authorization": "Bearer ${API_TOKEN}" });
    world
        .host
        .set_every_agent(inline("agent", json!({ "headers": headers })));
    fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    ensure_eq!(
        own_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some(json!({ "headers": headers })),
        "the keys passed"
    );
    Ok(())
}

fn a_credential_in_an_entry_no_stage_runs(_: &()) -> Outcome {
    let world = World::new();
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = inline("other", json!({ "line": format!("ghp_{TWENTY}") }));
    let reason = refused(&world.engine.launch(&fix))?;
    ensure!(
        reason.contains("`programs.other.settings.line`") && reason.contains("`launch`"),
        "the refusal: {reason}"
    );
    nothing_made(&world, &World::id("fix", "one"))?;
    ensure!(world.store.places().is_empty(), "a place held");
    Ok(())
}

fn a_home_and_a_variable_in_a_command(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world
        .host
        .set_every_agent(inline("agent", json!({ "line": "~/bin/line $HOME" })));
    fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    ensure_eq!(
        own_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some(json!({ "line": "~/bin/line $HOME" })),
        "the keys passed"
    );
    Ok(())
}

fn a_named_file_not_passed(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world.host.set_every_agent(by_file(
        "agent",
        HOST_FILE,
        json!({ "tui": "fullscreen" }),
        json!({}),
    ));
    let id = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    let code = World::identity("fix", "one", "code", 1);
    let command = opened(&world, &code)?;
    let own = FakeAgent::own_file(&world.store.folder(&id), &code);
    ensure_eq!(
        command.files.keys().collect::<Vec<_>>(),
        vec![&own],
        "the files the command lists"
    );
    ensure!(
        !command.args.iter().any(|arg| arg.contains(HOST_FILE))
            && !command.env.values().any(|value| value.contains(HOST_FILE)),
        "the command names the host's file: {command:?}"
    );
    Ok(())
}

fn a_scripted_way_and_a_reserved_key(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_own();
    world.agent.reserve("env", "the account");
    world.host.set_every_agent(by_file(
        "agent",
        HOST_FILE,
        json!({ "env": { "PEOPLE": "1" } }),
        json!({ "line": "a", "tips": true }),
    ));
    let id = fix_begun(&world, &launch("fix", "one", Some("mend it")))?;
    let code = World::identity("fix", "one", "code", 1);
    let file = FakeAgent::own_file(&world.store.folder(&id), &code);
    ensure_eq!(
        world
            .store
            .file(&file)
            .map(|(text, _)| serde_json::from_str::<Value>(&text).ok()),
        Some(Some(json!({ "line": "a", "tips": true }))),
        "the file the core wrote"
    );
    ensure_eq!(
        world.agent.log.of("own"),
        vec![format!("own {code}: line, tips")],
        "the keys the fake was given"
    );
    let mut env = launch("fix", "two", Some("mend it"));
    env.every_stage = inline("agent", json!({ "env": { "X": "1" } }));
    let reason = refused(&world.engine.launch(&env))?;
    ensure!(
        reason.contains("`programs.agent.settings.env`") && reason.contains("the account"),
        "the refusal: {reason}"
    );
    Ok(())
}

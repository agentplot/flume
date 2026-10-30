//! An agent's mode (162, 169; record 0024): one of flume's four, plan,
//! ask, edits and full, or a table of programs each giving its program's
//! own value; given by any of the six layers and taken whole from the
//! first, a stage's own before every stage's; mapped by the agent's
//! profile to its program's setting that means it, or the launch or the
//! stage refused naming the mode, the program, the stage and the layer;
//! `plan` only on a conversation; none passed where no layer gives one;
//! and nothing answered for an agent in any mode (4, 8, 21, 27, 32, 34,
//! 90, 109).

use super::agent_settings::{for_every_stage, for_stage, opened, to_code, to_verify};
use super::world::*;
use crate::fake_harness::flumes_own;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_seams::*;
use std::collections::BTreeMap;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "agent modes: a program's own value given bare in a launch's declaration refuses the launch, naming the field and the value and saying it goes under its program's name, and nothing is made",
            cites: &["4", "169", "C.2"],
            run: a_programs_own_value_given_bare,
        },
        Scenario {
            name: "agent modes: a table of programs in a launch's declaration starts the agent with its program's entry passed, status giving the table as written",
            cites: &["162", "169", "56"],
            run: a_table_of_programs,
        },
        Scenario {
            name: "agent modes: the host's table over the repository's starts the agent with the value its profile maps the host's mode to, from host-kind",
            cites: &["S66", "32", "162"],
            run: s66_the_hosts_table_over_the_repositorys,
        },
        Scenario {
            name: "agent modes: the launch's mode for every stage comes before the kind's for a stage, and the kind's for a stage before its own for every stage",
            cites: &["32", "162"],
            run: a_stages_own_before_every_stages,
        },
        Scenario {
            name: "agent modes: the host's table changing while code works leaves code's agent in its mode and starts verify in the new one",
            cites: &["8", "32", "162"],
            run: the_hosts_table_changes_while_a_stage_works,
        },
        Scenario {
            name: "agent modes: with no mode in any layer the agent's command carries none, its start's record names none, nothing is mapped, and its started ack is taken whatever mode it reports",
            cites: &["S66", "161", "162"],
            run: no_mode_in_any_layer,
        },
        Scenario {
            name: "agent modes: each of flume's four is passed as the value the profile maps it to and no other, plan on a conversation",
            cites: &["88", "169", "109"],
            run: each_mode_mapped_as_the_profile_says,
        },
        Scenario {
            name: "agent modes: a mode the profile maps no setting for refuses the launch naming the mode, the program, the stage and the layer, and nothing is made",
            cites: &["4", "169"],
            run: a_mode_the_profile_cannot_honour,
        },
        Scenario {
            name: "agent modes: a later start whose host table gives a value the profile does not name is refused naming the mode as written, the program, the stage and host-kind, until a retry after the table is mended",
            cites: &["27", "32", "169"],
            run: a_later_start_refused,
        },
        Scenario {
            name: "agent modes: a table naming no entry for the agent's program refuses the launch naming the table as written, and no later layer's mode is taken in its place",
            cites: &["4", "169"],
            run: no_entry_for_the_agents_program,
        },
        Scenario {
            name: "agent modes: plan for every stage of a change refuses the launch naming construct, its deliverable and the layer, and nothing is made",
            cites: &["4", "169"],
            run: plan_for_every_stage_of_a_kind,
        },
        Scenario {
            name: "agent modes: plan for a conversation in a launch's declaration is taken, its agent passed the value its profile maps plan to",
            cites: &["109", "169"],
            run: plan_for_a_conversation,
        },
        Scenario {
            name: "agent modes: an agent started in ask that asks a person past its stage's quiet limit is read asking, sent nothing, and neither ended nor stuck",
            cites: &["34", "44", "90", "162"],
            run: an_agent_in_ask_asks_a_person,
        },
    ]
}

/// The fake agent's map of flume's four, `plan`, `ask`, `edits` and
/// `full` to `look`, `ask-each`, `edit-files` and `all`, and `careful` a
/// value of its own (169).
pub(super) fn the_fakes_map(world: &World) {
    for (mode, value) in [
        (Mode::Plan, "look"),
        (Mode::Ask, "ask-each"),
        (Mode::Edits, "edit-files"),
        (Mode::Full, "all"),
    ] {
        world.agent.map_mode(mode, value);
    }
    world.agent.own_mode("careful");
}

/// Settings giving one of flume's four.
pub(super) fn in_mode(mode: Mode) -> Settings {
    Settings {
        mode: Some(ModeSetting::Flume(mode)),
        ..Settings::default()
    }
}

/// Settings giving a table of programs.
pub(super) fn in_programs(entries: &[(&str, &str)]) -> Settings {
    Settings {
        mode: Some(ModeSetting::Programs(
            entries
                .iter()
                .map(|(program, value)| (program.to_string(), value.to_string()))
                .collect(),
        )),
        ..Settings::default()
    }
}

/// The value the command of the start under `identity` passed for its
/// mode, none where it passed none.
pub(super) fn passed(world: &World, identity: &Identity) -> Result<Option<String>, String> {
    let command = opened(world, identity)?;
    let values: Vec<&String> = command
        .args
        .windows(2)
        .filter(|pair| pair[0] == "--mode")
        .map(|pair| &pair[1])
        .collect();
    match values.as_slice() {
        [] => Ok(None),
        [value] => Ok(Some(value.to_string())),
        _ => Err(format!(
            "{identity}'s command passed more than one mode: {:?}",
            command.args
        )),
    }
}

/// Where the current agent's mode came from, and as it was written.
pub(super) fn mode_from(world: &World, id: &PlaceId) -> Result<Option<Given>, String> {
    Ok(world
        .status(id)?
        .agent
        .ok_or("no agent")?
        .settings
        .from
        .get("mode")
        .cloned())
}

fn given(layer: Layer, value: &str) -> Option<Given> {
    Some(Given {
        layer,
        value: value.into(),
    })
}

/// flume's own fix@1, its agents given `agents`.
fn fix_kind(agents: KindSettings) -> Result<Kind, String> {
    let mut fix = flumes_own()
        .into_iter()
        .find(|kind| kind.name == "fix")
        .ok_or("flume's own kinds hold no fix")?;
    fix.agents = agents;
    Ok(fix)
}

/// flume's own change@1, its agents given `agents`.
fn change_kind(agents: KindSettings) -> Result<Kind, String> {
    let mut change = flumes_own()
        .into_iter()
        .find(|kind| kind.name == "change" && kind.version == 1)
        .ok_or("flume's own kinds hold no change@1")?;
    change.agents = agents;
    Ok(change)
}

/// A repository's one-stage kind `talk`, its stage a conversation.
fn talk_kind() -> Kind {
    kind("talk", false, vec![stage("talk", None, "{{brief}}")])
}

/// Nothing made for a refused launch: no place, and no held state for
/// `id`.
fn nothing_made(world: &World, id: &PlaceId) -> Outcome {
    ensure_eq!(world.place.log.count("make"), 0, "places made");
    ensure!(world.held(id).is_none(), "held state written");
    Ok(())
}

fn a_programs_own_value_given_bare(_: &()) -> Outcome {
    let world = World::new();
    let words = ModeSetting::from_word("auto")
        .err()
        .ok_or("`auto` was read as one of flume's four")?;
    world
        .kinds
        .unreadable_declaration("modes.toml", &format!("kinds.fix.agents.mode: {words}"));
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.declaration = Some("modes.toml".into());
    let reason = refused(&world.engine.launch(&fix))?;
    ensure!(
        reason.contains("kinds.fix.agents.mode")
            && reason.contains("`auto`")
            && reason.contains("under its program's name"),
        "the refusal: {reason}"
    );
    nothing_made(&world, &World::id("fix", "one"))
}

fn a_table_of_programs(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    let table = in_programs(&[("agent", "careful"), ("other", "edits")]);
    world.kinds.set_declaration(
        "modes.toml",
        vec![fix_kind(for_every_stage(table.clone()))?],
    );
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.declaration = Some("modes.toml".into());
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    ensure_eq!(
        passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some("careful".to_string()),
        "the value passed"
    );
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    ensure_eq!(agent.settings.settings.mode, table.mode, "the agent's mode");
    ensure_eq!(
        agent.settings.from.get("mode").cloned(),
        given(Layer::Kind, r#"{ agent = "careful", other = "edits" }"#),
        "the mode as the table was written"
    );
    ensure_eq!(
        agent.mode.and_then(|mode| mode.entry),
        Some("agent".to_string()),
        "the entry the agent took"
    );
    Ok(())
}

fn s66_the_hosts_table_over_the_repositorys(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    world
        .host
        .set_settings(None, "fix", for_every_stage(in_mode(Mode::Edits)));
    world
        .kinds
        .set_settings(REPOSITORY, "fix", for_every_stage(in_mode(Mode::Full)));
    launched(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    ensure_eq!(
        passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some("edit-files".to_string()),
        "the value passed"
    );
    ensure_eq!(
        mode_from(&world, &id)?,
        given(Layer::HostKind, "edits"),
        "where the mode came from"
    );
    Ok(())
}

/// A stage's value passed for its mode and where its mode came from.
type StageMode = (Option<String>, Option<Given>);

/// The change `name` through construct, code and verify, each stage's
/// value passed and where its mode came from.
fn each_stage(world: &World, name: &str) -> Result<Vec<StageMode>, String> {
    let id = World::id("change", name);
    let read = |stage: &str| -> Result<StageMode, String> {
        Ok((
            passed(world, &World::identity("change", name, stage, 1))?,
            mode_from(world, &id)?,
        ))
    };
    let construct = read("construct")?;
    to_code(world, &id, name)?;
    let code = read("code")?;
    to_verify(world, &id, name)?;
    Ok(vec![construct, code, read("verify")?])
}

fn a_stages_own_before_every_stages(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    world.kinds.set_kinds(
        REPOSITORY,
        vec![change_kind(KindSettings {
            every_stage: in_mode(Mode::Edits),
            stages: BTreeMap::from([("verify".to_string(), in_mode(Mode::Ask))]),
        })?],
    );
    let mut full = launch("change", "one", Some("plan it"));
    full.every_stage = in_mode(Mode::Full);
    launched(&world.engine.launch(&full))?;
    let launched_full = (Some("all".to_string()), given(Layer::Launch, "full"));
    ensure_eq!(
        each_stage(&world, "one")?,
        vec![launched_full.clone(), launched_full.clone(), launched_full],
        "each stage under the launch's mode"
    );
    launched(
        &world
            .engine
            .launch(&launch("change", "two", Some("plan it"))),
    )?;
    let edits = (Some("edit-files".to_string()), given(Layer::Kind, "edits"));
    ensure_eq!(
        each_stage(&world, "two")?,
        vec![
            edits.clone(),
            edits,
            (Some("ask-each".to_string()), given(Layer::Kind, "ask"))
        ],
        "each stage under the kind's modes"
    );
    Ok(())
}

fn the_hosts_table_changes_while_a_stage_works(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    world
        .host
        .set_settings(None, "change", for_every_stage(in_mode(Mode::Edits)));
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
        .set_settings(None, "change", for_every_stage(in_mode(Mode::Full)));
    world.settle(&id);
    ensure_eq!(
        (
            world.terminal.log.count("close"),
            world.stage_state(&id, "code")
        ),
        (closes, Some(StageState::Begun)),
        "agents ended and code's state after the host's table changed"
    );
    let held = world.held(&id).ok_or("no held state")?;
    let code = held.stages[1].start.as_ref().ok_or("no start of code")?;
    ensure_eq!(
        (code.settings.mode.clone(), code.start_record.mode.clone()),
        (
            Some(ModeSetting::Flume(Mode::Edits)),
            Some("edit-files".to_string())
        ),
        "code's start keeps its mode"
    );
    to_verify(&world, &id, "plan")?;
    ensure_eq!(
        passed(&world, &World::identity("change", "plan", "verify", 1))?,
        Some("all".to_string()),
        "verify's value passed"
    );
    ensure_eq!(
        mode_from(&world, &id)?,
        given(Layer::HostKind, "full"),
        "where verify's mode came from"
    );
    Ok(())
}

fn no_mode_in_any_layer(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    world.agent.read_mode_back();
    let code = World::identity("fix", "one", "code", 1);
    world.agent.set_began_in(&code, "ask-each");
    launched(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    ensure_eq!(passed(&world, &code)?, None, "the value passed");
    let held = world.held(&id).ok_or("no held state")?;
    let start = held.stages[0].start.as_ref().ok_or("no start of code")?;
    ensure_eq!(start.start_record.mode, None, "the record's mode");
    ensure_eq!(
        world.agent.log.of("mode"),
        Vec::<String>::new(),
        "modes mapped"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, its started ack taken in whatever mode"
    );
    ensure!(held.unsettled.is_empty(), "a start read begun otherwise");
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    ensure_eq!(
        (agent.settings.settings.mode, agent.mode),
        (None, None),
        "the agent's mode"
    );
    ensure!(
        !agent.settings.from.contains_key("mode"),
        "status names a layer of the mode"
    );
    Ok(())
}

fn each_mode_mapped_as_the_profile_says(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    for (mode, value) in [
        (Mode::Ask, "ask-each"),
        (Mode::Edits, "edit-files"),
        (Mode::Full, "all"),
    ] {
        let name = mode.word();
        let mut fix = launch("fix", name, Some("mend it"));
        fix.settings.insert("code".into(), in_mode(mode));
        launched(&world.engine.launch(&fix))?;
        ensure_eq!(
            passed(&world, &World::identity("fix", name, "code", 1))?,
            Some(value.to_string()),
            "the value passed for {name}"
        );
    }
    world.kinds.set_kinds(REPOSITORY, vec![talk_kind()]);
    let mut talk = launch("talk", "plan", Some("talk it over"));
    talk.every_stage = in_mode(Mode::Plan);
    launched(&world.engine.launch(&talk))?;
    ensure_eq!(
        passed(&world, &World::identity("talk", "plan", "talk", 1))?,
        Some("look".to_string()),
        "the value passed for plan"
    );
    Ok(())
}

fn a_mode_the_profile_cannot_honour(_: &()) -> Outcome {
    let world = World::new();
    for (mode, value) in [
        (Mode::Plan, "look"),
        (Mode::Ask, "ask-each"),
        (Mode::Full, "all"),
    ] {
        world.agent.map_mode(mode, value);
    }
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.settings.insert("code".into(), in_mode(Mode::Edits));
    let reason = refused(&world.engine.launch(&fix))?;
    ensure!(
        ["`edits`", "`agent`", "`code`", "`launch`"]
            .iter()
            .all(|named| reason.contains(named)),
        "the refusal: {reason}"
    );
    nothing_made(&world, &World::id("fix", "one"))
}

fn a_later_start_refused(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    to_code(&world, &id, "plan")?;
    world.host.set_settings(
        None,
        "change",
        for_stage("verify", in_programs(&[("agent", "unheard")])),
    );
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
        [
            r#"`{ agent = "unheard" }`"#,
            "`agent`",
            "`verify`",
            "`host-kind`"
        ]
        .iter()
        .all(|named| refusal.contains(named)),
        "the refusal: {refusal}"
    );
    world
        .host
        .set_settings(None, "change", KindSettings::default());
    let answer = world
        .engine()
        .retry(&id, Retry::Stage("verify".into()), None, false);
    ensure!(answer.refusal().is_none(), "the retry: {answer:?}");
    ensure_eq!(
        passed(&world, &World::identity("change", "plan", "verify", 1))?,
        None,
        "verify's value passed once the table is mended"
    );
    ensure_eq!(
        world.status(&id)?.stages[2].refused_setting,
        None,
        "the refusal after the start"
    );
    Ok(())
}

fn no_entry_for_the_agents_program(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    world.kinds.set_kinds(
        REPOSITORY,
        vec![fix_kind(for_every_stage(in_mode(Mode::Edits)))?],
    );
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = in_programs(&[("other", "edits")]);
    let reason = refused(&world.engine.launch(&fix))?;
    ensure!(
        [
            r#"`{ other = "edits" }`"#,
            "`agent`",
            "`code`",
            "`launch`",
            "names no entry"
        ]
        .iter()
        .all(|named| reason.contains(named)),
        "the refusal: {reason}"
    );
    ensure_eq!(
        world.agent.log.of("mode"),
        Vec::<String>::new(),
        "modes mapped, the kind's edits among them"
    );
    nothing_made(&world, &World::id("fix", "one"))
}

fn plan_for_every_stage_of_a_kind(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    let mut change = launch("change", "plan", Some("plan it"));
    change.every_stage = in_mode(Mode::Plan);
    let reason = refused(&world.engine.launch(&change))?;
    ensure!(
        ["`construct`", "a change written", "`launch`", "plan mode"]
            .iter()
            .all(|named| reason.contains(named)),
        "the refusal: {reason}"
    );
    nothing_made(&world, &World::id("change", "plan"))
}

fn plan_for_a_conversation(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    let mut talk = talk_kind();
    talk.agents = for_stage("talk", in_mode(Mode::Plan));
    world.kinds.set_declaration("talk.toml", vec![talk]);
    let mut given = launch("talk", "one", Some("talk it over"));
    given.declaration = Some("talk.toml".into());
    launched(&world.engine.launch(&given))?;
    ensure_eq!(
        passed(&world, &World::identity("talk", "one", "talk", 1))?,
        Some("look".to_string()),
        "the value passed"
    );
    Ok(())
}

fn an_agent_in_ask_asks_a_person(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.settings.insert("code".into(), in_mode(Mode::Ask));
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    let code = World::identity("fix", "one", "code", 1);
    ensure_eq!(
        (passed(&world, &code)?, world.stage_state(&id, "code")),
        (Some("ask-each".to_string()), Some(StageState::Begun)),
        "code begun in ask"
    );
    let sends = world.terminal.log.count("send");
    world.agent.set_asking(&code, Reading::Yes);
    world.terminal.set_state(&code, AgentState::Asking);
    world.host.advance(flume_core::QUIET_LIMIT + 60);
    world.settle(&id);
    world.settle(&id);
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    ensure_eq!(
        (agent.state, agent.asking),
        (AgentState::Asking, Reading::Yes),
        "the agent read"
    );
    ensure_eq!(
        (
            world.terminal.log.count("send"),
            world.terminal.log.count("close")
        ),
        (sends, 0),
        "sends and closes while it asks"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code while its agent asks"
    );
    Ok(())
}

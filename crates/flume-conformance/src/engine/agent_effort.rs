//! An agent's effort (197; record 0044): its program's own word for how
//! hard its model works on each turn, given by any of the eight layers
//! and taken whole from the first, a stage's own before every stage's, as
//! the model is; asked of the agent's profile, which takes its program's
//! words or refuses, the launch or the stage refused naming the effort,
//! the program, the stage and the layer; and nothing asked or passed
//! where no layer gives one (4, 8, 21, 27, 32, 88).

use super::agent_modes::{change_kind, fix_kind, nothing_made};
use super::agent_settings::{for_every_stage, for_stage, opened, to_code, to_verify};
use super::settling::write_change;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "agent effort: the kind's effort is kept as written, the agent's settings giving it from kind",
            cites: &["197", "32", "158"],
            run: a_word_kept_as_written,
        },
        Scenario {
            name: "agent effort: a declaration whose effort is a table of programs refuses the launch, naming the table's effort, and nothing is made",
            cites: &["4", "8", "197"],
            run: a_table_refused,
        },
        Scenario {
            name: "agent effort: the launch's effort for one stage comes before the kind's for every stage, each stage starting with its own from its layer",
            cites: &["32", "197", "S103"],
            run: the_launchs_over_the_kinds,
        },
        Scenario {
            name: "agent effort: the host's table for a kind comes before the repository's, the agent starting with the host's from host-kind",
            cites: &["32", "197"],
            run: the_hosts_over_the_repositorys,
        },
        Scenario {
            name: "agent effort: the host's effort for a repository, a stream and every agent each come before the repository's own for every kind, from host-repository, host-stream and host",
            cites: &["32", "143", "197"],
            run: for_a_repository_a_stream_and_every_agent,
        },
        Scenario {
            name: "agent effort: the host's table changing while code works leaves code's start with its effort and starts verify with the new one, each from host-kind",
            cites: &["21", "32", "197"],
            run: the_hosts_table_changed_after_the_launch,
        },
        Scenario {
            name: "agent effort: with no effort in any layer nothing is asked of the profile, the command passes none and status gives the agent none",
            cites: &["144", "162", "197"],
            run: no_effort_in_any_layer,
        },
        Scenario {
            name: "agent effort: a word the profile takes is asked at the launch and at the start and passed, and one it does not take refuses the launch naming the word, the program, the stage, the layer and the words it takes",
            cites: &["4", "88", "197"],
            run: a_word_taken_and_one_refused,
        },
        Scenario {
            name: "agent effort: an effort the profile refuses for a later stage refuses the launch whole, naming the effort, the program, the stage and the layer, and nothing is made",
            cites: &["4", "8", "197"],
            run: a_refusal_at_the_launch,
        },
        Scenario {
            name: "agent effort: a later start whose host table gives an effort the profile refuses is refused naming it, the program, the stage and host-kind, starting no agent, until a retry after the table is mended",
            cites: &["27", "32", "197"],
            run: a_refusal_at_a_later_start,
        },
    ]
}

/// Settings giving the effort `word`.
pub(super) fn effort(word: &str) -> Settings {
    Settings {
        effort: Some(word.into()),
        ..Settings::default()
    }
}

/// The effort the command of the start under `identity` passed, none
/// where it passed none.
fn effort_passed(world: &World, identity: &Identity) -> Result<Option<String>, String> {
    let command = opened(world, identity)?;
    let words: Vec<&String> = command
        .args
        .windows(2)
        .filter(|pair| pair[0] == "--effort")
        .map(|pair| &pair[1])
        .collect();
    match words.as_slice() {
        [] => Ok(None),
        [word] => Ok(Some(word.to_string())),
        _ => Err(format!(
            "{identity}'s command passed more than one effort: {:?}",
            command.args
        )),
    }
}

/// Where the current agent's effort came from, and as it was written.
fn effort_from(world: &World, id: &PlaceId) -> Result<Option<Given>, String> {
    Ok(world
        .status(id)?
        .agent
        .ok_or("no agent")?
        .settings
        .from
        .get("effort")
        .cloned())
}

fn given(layer: Layer, value: &str) -> Option<Given> {
    Some(Given {
        layer,
        value: value.into(),
        file: None,
    })
}

/// The effort passed to and the layer of the agent of `stage`'s first
/// start in the place `kind--name`.
fn started_with(
    world: &World,
    kind: &str,
    name: &str,
    stage: &str,
) -> Result<(Option<String>, Option<Given>), String> {
    Ok((
        effort_passed(world, &World::identity(kind, name, stage, 1))?,
        effort_from(world, &World::id(kind, name))?,
    ))
}

fn a_word_kept_as_written(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_efforts(&["xhigh"]);
    world.kinds.set_kinds(
        REPOSITORY,
        vec![fix_kind(for_every_stage(effort("xhigh")))?],
    );
    launched(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    ensure_eq!(
        agent.settings.settings.effort.as_deref(),
        Some("xhigh"),
        "the agent's effort"
    );
    ensure_eq!(
        agent.settings.from.get("effort").cloned(),
        given(Layer::Kind, "xhigh"),
        "where the effort came from"
    );
    ensure_eq!(
        effort_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some("xhigh".to_string()),
        "the effort passed"
    );
    Ok(())
}

fn a_table_refused(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .unreadable_declaration("efforts.toml", "kinds.fix.agents.effort: must be a string");
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.declaration = Some("efforts.toml".into());
    let reason = refused(&world.engine.launch(&fix))?;
    ensure!(
        reason.contains("kinds.fix.agents.effort"),
        "the refusal: {reason}"
    );
    nothing_made(&world, &World::id("fix", "one"))
}

fn the_launchs_over_the_kinds(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_efforts(&["high", "max"]);
    world.kinds.set_kinds(
        REPOSITORY,
        vec![change_kind(for_every_stage(effort("high")))?],
    );
    let mut change = launch("change", "one", Some("plan it"));
    change.settings.insert("code".into(), effort("max"));
    launched(&world.engine.launch(&change))?;
    let id = World::id("change", "one");
    let construct = started_with(&world, "change", "one", "construct")?;
    to_code(&world, &id, "one")?;
    let code = started_with(&world, "change", "one", "code")?;
    to_verify(&world, &id, "one")?;
    let verify = started_with(&world, "change", "one", "verify")?;
    let from_kind = (Some("high".to_string()), given(Layer::Kind, "high"));
    ensure_eq!(
        vec![construct, code, verify],
        vec![
            from_kind.clone(),
            (Some("max".to_string()), given(Layer::Launch, "max")),
            from_kind
        ],
        "each stage's effort passed and its layer"
    );
    Ok(())
}

fn the_hosts_over_the_repositorys(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_efforts(&["medium", "xhigh"]);
    world
        .host
        .set_settings(None, "fix", for_every_stage(effort("medium")));
    world
        .kinds
        .set_settings(REPOSITORY, "fix", for_every_stage(effort("xhigh")));
    launched(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    world.settle(&World::id("fix", "one"));
    ensure_eq!(
        started_with(&world, "fix", "one", "code")?,
        (Some("medium".to_string()), given(Layer::HostKind, "medium")),
        "the fix's effort passed and its layer"
    );
    Ok(())
}

fn for_a_repository_a_stream_and_every_agent(_: &()) -> Outcome {
    let world = World::new();
    world
        .agent
        .take_efforts(&["low", "medium", "high", "xhigh"]);
    world.host.set_stream("work", Some(Stream::default()));
    world.host.set_every_agent(effort("medium"));
    world.host.set_stream_settings("work", effort("high"));
    world.host.set_every_kind(REPOSITORY, effort("xhigh"));
    world.kinds.set_every_kind(REPOSITORY, effort("low"));
    let in_stream = |name: &str, stream: Option<&str>| Launch {
        stream: stream.map(str::to_string),
        ..launch("fix", name, Some("mend it"))
    };
    launched(&world.engine.launch(&in_stream("one", Some("work"))))?;
    world.settle(&World::id("fix", "one"));
    ensure_eq!(
        started_with(&world, "fix", "one", "code")?,
        (
            Some("xhigh".to_string()),
            given(Layer::HostRepository, "xhigh")
        ),
        "a fix's effort the host names for every kind of its repository"
    );
    // The host names nothing for the repository now, as for another one.
    world.host.set_every_kind(REPOSITORY, Settings::default());
    for (name, stream, layer, word) in [
        ("two", Some("work"), Layer::HostStream, "high"),
        ("three", None, Layer::Host, "medium"),
    ] {
        launched(&world.engine.launch(&in_stream(name, stream)))?;
        world.settle(&World::id("fix", name));
        ensure_eq!(
            started_with(&world, "fix", name, "code")?,
            (Some(word.to_string()), given(layer, word)),
            format!("a fix's effort in {}", stream.unwrap_or("default"))
        );
    }
    Ok(())
}

fn the_hosts_table_changed_after_the_launch(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_efforts(&["high", "xhigh"]);
    world
        .host
        .set_settings(None, "change", for_every_stage(effort("high")));
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    let construct = started_with(&world, "change", "plan", "construct")?;
    to_code(&world, &id, "plan")?;
    world
        .host
        .set_settings(None, "change", for_every_stage(effort("xhigh")));
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code after the host's table changed"
    );
    to_verify(&world, &id, "plan")?;
    let verify = started_with(&world, "change", "plan", "verify")?;
    ensure_eq!(
        (construct, verify),
        (
            (Some("high".to_string()), given(Layer::HostKind, "high")),
            (Some("xhigh".to_string()), given(Layer::HostKind, "xhigh"))
        ),
        "construct's and verify's efforts"
    );
    let held = world.held(&id).ok_or("no held state")?;
    for (index, stage) in [(0, "construct"), (1, "code")] {
        let start = held.stages[index]
            .start
            .as_ref()
            .ok_or(format!("no start of {stage}"))?;
        ensure_eq!(
            start.settings.effort.as_deref(),
            Some("high"),
            format!("{stage}'s start keeps its effort")
        );
    }
    Ok(())
}

fn no_effort_in_any_layer(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_efforts(&["low", "high"]);
    launched(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    let code = World::identity("fix", "one", "code", 1);
    ensure_eq!(effort_passed(&world, &code)?, None, "the effort passed");
    ensure_eq!(
        world.agent.log.of("effort"),
        Vec::<String>::new(),
        "efforts asked"
    );
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    ensure_eq!(agent.settings.settings.effort, None, "the agent's effort");
    ensure!(
        !agent.settings.from.contains_key("effort"),
        "status names a layer of the effort"
    );
    Ok(())
}

fn a_word_taken_and_one_refused(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_efforts(&["low", "high"]);
    let mut high = launch("fix", "one", Some("mend it"));
    high.every_stage = effort("high");
    launched(&world.engine.launch(&high))?;
    world.settle(&World::id("fix", "one"));
    ensure_eq!(
        effort_passed(&world, &World::identity("fix", "one", "code", 1))?,
        Some("high".to_string()),
        "the effort passed"
    );
    // Asked of the profile at the launch, before anything is made, and
    // again at the start, before its command is built (197).
    ensure_eq!(
        world.agent.log.of("effort"),
        vec!["effort high".to_string(); 2],
        "efforts asked at the launch and the start"
    );
    let mut med = launch("fix", "two", Some("mend it"));
    med.every_stage = effort("med");
    let reason = refused(&world.engine.launch(&med))?;
    ensure!(
        ["`med`", "`agent`", "`code`", "`launch`", "low", "high"]
            .iter()
            .all(|named| reason.contains(named)),
        "the refusal: {reason}"
    );
    nothing_made_for(&world, &World::id("fix", "two"))
}

/// Nothing made for the refused launch `id` beside the one place
/// launched before it: one place made, and no held state for `id`.
fn nothing_made_for(world: &World, id: &PlaceId) -> Outcome {
    ensure_eq!(world.place.log.count("make"), 1, "places made");
    ensure!(world.held(id).is_none(), "held state written for {id}");
    Ok(())
}

fn a_refusal_at_the_launch(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_efforts(&["low", "high"]);
    let mut change = launch("change", "plan", Some("plan it"));
    change.settings.insert("verify".into(), effort("ultracode"));
    let reason = refused(&world.engine.launch(&change))?;
    ensure!(
        ["`ultracode`", "`agent`", "`verify`", "`launch`"]
            .iter()
            .all(|named| reason.contains(named)),
        "the refusal: {reason}"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    nothing_made(&world, &World::id("change", "plan"))
}

fn a_refusal_at_a_later_start(_: &()) -> Outcome {
    let world = World::new();
    world.agent.take_efforts(&["low", "high"]);
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    world.settle(&id);
    world
        .host
        .set_settings(None, "change", for_stage("code", effort("turbo")));
    world.terminal.log.clear();
    write_change(&world, "plan");
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Refused),
        "code"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    let refusal = world.status(&id)?.stages[1]
        .refused_setting
        .clone()
        .ok_or("no setting named")?;
    ensure!(
        ["`turbo`", "`agent`", "`code`", "`host-kind`"]
            .iter()
            .all(|named| refusal.contains(named)),
        "the refusal: {refusal}"
    );
    world
        .host
        .set_settings(None, "change", KindSettings::default());
    let answer = world
        .engine()
        .retry(&id, Retry::Stage("code".into()), None, false);
    ensure!(answer.refusal().is_none(), "the retry: {answer:?}");
    ensure_eq!(
        effort_passed(&world, &World::identity("change", "plan", "code", 1))?,
        None,
        "code's effort passed once the table is mended"
    );
    ensure_eq!(
        world.status(&id)?.stages[1].refused_setting,
        None,
        "the refusal after the start"
    );
    Ok(())
}

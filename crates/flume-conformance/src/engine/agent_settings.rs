//! An agent's settings, resolved at each start from six layers (32;
//! record 0017): the launch's, the host's for the repository and kind,
//! the host's for the kind, the repository's for a kind it does not
//! declare, the kind's declaration and flume's default; the environment
//! merged by variable, the host's values read against the login
//! environment, the host read at every start; and the launch refused for
//! settings flume cannot run.

use super::settling::write_change;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_seams::*;
use std::collections::BTreeMap;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "settings: the host's environment, the repository's model and the launch's instructions, each with its layer, and a host change taken by the next start only",
            cites: &["S44", "8", "29", "32", "56"],
            run: s44_settings_from_four_layers,
        },
        Scenario {
            name: "settings: the environment merges variable by variable across the layers",
            cites: &["32", "29"],
            run: the_environment_merged_by_variable,
        },
        Scenario {
            name: "settings: a program no agent profile binds refuses the launch, naming the program, the stage and the layer",
            cites: &["4", "32"],
            run: an_unbound_program_refuses_the_launch,
        },
        Scenario {
            name: "settings: a later start the host's configuration leaves unresolved is refused until a retry",
            cites: &["27", "32", "8"],
            run: a_later_start_refused_until_a_retry,
        },
        Scenario {
            name: "settings: an environment in the repository's declaration refuses the launch, naming the field",
            cites: &["32", "4"],
            run: an_environment_in_the_repositorys_declaration,
        },
        Scenario {
            name: "settings: settings naming a stage the kind does not hold refuse the launch, the host's ignored",
            cites: &["4", "32"],
            run: a_stage_the_kind_lacks,
        },
        Scenario {
            name: "settings: the repository's settings are kept as the launch read them",
            cites: &["8", "21", "32"],
            run: the_repositorys_settings_kept,
        },
    ]
}

fn model(model: &str) -> Settings {
    Settings {
        model: Some(model.into()),
        ..Settings::default()
    }
}

fn env(variables: &[(&str, &str)]) -> Settings {
    Settings {
        env: variables
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_string()))
            .collect(),
        ..Settings::default()
    }
}

fn for_every_stage(settings: Settings) -> KindSettings {
    KindSettings {
        every_stage: settings,
        stages: BTreeMap::new(),
    }
}

fn for_stage(stage: &str, settings: Settings) -> KindSettings {
    KindSettings {
        every_stage: Settings::default(),
        stages: BTreeMap::from([(stage.to_string(), settings)]),
    }
}

/// The command the start under `identity` was opened with.
fn opened(world: &World, identity: &Identity) -> Result<AgentCommand, String> {
    world
        .terminal
        .opened(identity)
        .map(|(command, _)| command)
        .ok_or_else(|| format!("{identity} was not opened"))
}

/// Construct of the change `name` delivered and code begun.
fn to_code(world: &World, id: &PlaceId, name: &str) -> Outcome {
    world.settle(id);
    write_change(world, name);
    world.settle(id);
    world.settle(id);
    ensure_eq!(
        world.stage_state(id, "code"),
        Some(StageState::Begun),
        "code"
    );
    Ok(())
}

/// Code of the change `name` delivered and verify begun.
fn to_verify(world: &World, id: &PlaceId, name: &str) -> Outcome {
    world.tick(&format!("change/{name}"), name, 1, 1);
    world.settle(id);
    world.settle(id);
    ensure_eq!(
        world.stage_state(id, "verify"),
        Some(StageState::Begun),
        "verify"
    );
    Ok(())
}

fn s44_settings_from_four_layers(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_settings(
        None,
        "change",
        for_every_stage(env(&[("AGENT_HOME", "~/.agent-work")])),
    );
    world
        .kinds
        .set_settings(REPOSITORY, "change", for_stage("code", model("opus")));
    let mut change = launch("change", "plan", Some("plan it"));
    change.settings.insert(
        "verify".into(),
        Settings {
            instructions: Some("read every task".into()),
            ..Settings::default()
        },
    );
    let id = World::id("change", "plan");
    launched(&world.engine.launch(&change))?;
    let construct = opened(&world, &World::identity("change", "plan", "construct", 1))?;
    ensure_eq!(
        construct.env.get("AGENT_HOME").map(String::as_str),
        Some("/home/user/.agent-work"),
        "construct's variable, read against the home"
    );
    ensure!(
        !construct.args.contains(&"opus".to_string()),
        "construct took code's model"
    );
    to_code(&world, &id, "plan")?;
    let code = World::identity("change", "plan", "code", 1);
    let command = opened(&world, &code)?;
    ensure!(
        command
            .args
            .windows(2)
            .any(|pair| pair == ["--model", "opus"]),
        "code's model: {:?}",
        command.args
    );
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    let from = &agent.settings.from;
    ensure_eq!(
        from.get("model"),
        Some(&Given {
            layer: Layer::Repository,
            value: "opus".into()
        }),
        "the model's layer"
    );
    ensure_eq!(
        from.get("env.AGENT_HOME"),
        Some(&Given {
            layer: Layer::HostKind,
            value: SET.into()
        }),
        "the variable's layer, its value unsaid"
    );
    ensure_eq!(
        agent
            .settings
            .settings
            .env
            .get("AGENT_HOME")
            .map(String::as_str),
        Some(SET),
        "status names the variable and gives no value"
    );
    ensure_eq!(
        from.get("program").map(|given| given.layer),
        Some(Layer::Default),
        "the program's layer"
    );
    world.host.set_settings(
        None,
        "change",
        for_every_stage(env(&[("AGENT_HOME", "~/.other")])),
    );
    let held = world.held(&id).ok_or("no held state")?;
    let kept = held.stages[1].start.as_ref().ok_or("no start of code")?;
    ensure_eq!(
        kept.settings.env.get("AGENT_HOME").map(String::as_str),
        Some(SET),
        "code's start keeps the variable by its name alone"
    );
    to_verify(&world, &id, "plan")?;
    let verify = opened(&world, &World::identity("change", "plan", "verify", 1))?;
    ensure_eq!(
        verify.env.get("AGENT_HOME").map(String::as_str),
        Some("/home/user/.other"),
        "verify starts with the new value"
    );
    ensure!(
        verify
            .args
            .windows(2)
            .any(|pair| pair == ["--instructions", "read every task"]),
        "verify's instructions: {:?}",
        verify.args
    );
    let agent = world.status(&id)?.agent.ok_or("no agent")?;
    ensure_eq!(
        agent
            .settings
            .from
            .get("instructions")
            .map(|given| given.layer),
        Some(Layer::Launch),
        "the instructions' layer"
    );
    Ok(())
}

fn the_environment_merged_by_variable(_: &()) -> Outcome {
    let world = World::new();
    world
        .host
        .set_settings(None, "fix", for_every_stage(env(&[("A", "1"), ("B", "2")])));
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = env(&[("B", "3")]);
    launched(&world.engine.launch(&fix))?;
    let command = opened(&world, &World::identity("fix", "one", "code", 1))?;
    ensure_eq!(command.env.get("A").map(String::as_str), Some("1"), "A");
    ensure_eq!(command.env.get("B").map(String::as_str), Some("3"), "B");
    let agent = world
        .status(&World::id("fix", "one"))?
        .agent
        .ok_or("no agent")?;
    let layer = |key: &str| agent.settings.from.get(key).map(|given| given.layer);
    ensure_eq!(layer("env.A"), Some(Layer::HostKind), "A's layer");
    ensure_eq!(layer("env.B"), Some(Layer::Launch), "B's layer");
    Ok(())
}

fn an_unbound_program_refuses_the_launch(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_settings(
        None,
        "change",
        for_stage(
            "code",
            Settings {
                program: Some("nothing-here".into()),
                ..Settings::default()
            },
        ),
    );
    let reason = refused(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    ensure!(
        reason.contains("nothing-here") && reason.contains("code") && reason.contains("host-kind"),
        "the refusal: {reason}"
    );
    ensure_eq!(world.place.log.count("make"), 0, "places made");
    ensure!(
        world.held(&World::id("change", "plan")).is_none(),
        "held state written"
    );
    Ok(())
}

fn a_later_start_refused_until_a_retry(_: &()) -> Outcome {
    let world = World::new();
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
        for_stage("verify", env(&[("X", "$NOT_SET")])),
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
    let status = world.status(&id)?;
    let refusal = status.stages[2]
        .refused_setting
        .clone()
        .ok_or("no setting named")?;
    ensure!(
        refusal.contains("NOT_SET") && refusal.contains("host-kind"),
        "the refusal: {refusal}"
    );
    let retried = refused(
        &world
            .engine()
            .retry(&id, Retry::Stage("verify".into()), None, false),
    );
    ensure!(retried.is_ok(), "a retry with nothing mended was taken");
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    world.host.set_login("NOT_SET", Some("set"));
    let answer = world
        .engine()
        .retry(&id, Retry::Stage("verify".into()), None, false);
    ensure!(answer.refusal().is_none(), "the retry: {answer:?}");
    let verify = opened(&world, &World::identity("change", "plan", "verify", 1))?;
    ensure_eq!(verify.env.get("X").map(String::as_str), Some("set"), "X");
    ensure_eq!(
        world.status(&id)?.stages[2].refused_setting,
        None,
        "the refusal after the start"
    );
    Ok(())
}

fn an_environment_in_the_repositorys_declaration(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_settings(REPOSITORY, "change", for_every_stage(env(&[("A", "b")])));
    let reason = refused(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    ensure!(
        reason.contains("kinds.change.agents.env"),
        "the refusal: {reason}"
    );
    let mut own = stage("work", Some(Deliverable::Commits), "{{brief}}");
    own.agents = vec![env(&[("A", "b")])];
    world
        .kinds
        .set_kinds(REPOSITORY, vec![kind("bump", false, vec![own])]);
    let reason = refused(&world.engine.launch(&launch("bump", "one", Some("x"))))?;
    ensure!(
        reason.contains("kinds.bump.stages[0].agents[0].env"),
        "the refusal: {reason}"
    );
    ensure_eq!(world.place.log.count("make"), 0, "places made");
    Ok(())
}

fn a_stage_the_kind_lacks(_: &()) -> Outcome {
    let world = World::new();
    world
        .host
        .set_settings(None, "fix", for_stage("review", model("ignored")));
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.settings.insert("review".into(), model("large"));
    let reason = refused(&world.engine.launch(&fix))?;
    ensure!(
        reason.contains("review") && reason.contains("launch"),
        "the refusal: {reason}"
    );
    world
        .kinds
        .set_settings(REPOSITORY, "fix", for_stage("review", model("large")));
    let reason = refused(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    ensure!(
        reason.contains("review") && reason.contains("repository"),
        "the refusal: {reason}"
    );
    ensure_eq!(world.place.log.count("make"), 0, "places made");
    world
        .kinds
        .set_settings(REPOSITORY, "fix", KindSettings::default());
    launched(&world.engine.launch(&launch("fix", "three", Some("x"))))?;
    Ok(())
}

fn the_repositorys_settings_kept(_: &()) -> Outcome {
    let world = World::new();
    let base = world.git.branch_head(REPOSITORY, "main").ok_or("no base")?;
    world
        .kinds
        .set_settings_at(REPOSITORY, &base, "change", for_every_stage(model("opus")));
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    let next = world.git.commit(REPOSITORY, "main", &["next.rs"]);
    world.kinds.set_settings_at(
        REPOSITORY,
        &next,
        "change",
        for_every_stage(model("sonnet")),
    );
    to_code(&world, &id, "plan")?;
    let command = opened(&world, &World::identity("change", "plan", "code", 1))?;
    ensure!(
        command
            .args
            .windows(2)
            .any(|pair| pair == ["--model", "opus"]),
        "code's model: {:?}",
        command.args
    );
    Ok(())
}

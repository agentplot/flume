//! An agent's settings, resolved at each start from six layers (32;
//! records 0017, 0020): the launch's, the host's for the repository and
//! kind, the host's for the kind, the repository's for a kind it does not
//! declare, the kind's declaration and flume's default, the host's two
//! giving a model and instructions alone; the environment its account's
//! alone, each value read against the login environment (29, 144); the
//! host's configuration read at every start; and the launch refused for
//! settings flume cannot run.

use super::account_pools::{account, fix_in, pool};
use super::settling::write_change;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_seams::*;
use std::collections::BTreeMap;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "settings: the account's environment, the repository's model and the launch's instructions, each with its layer",
            cites: &["S44", "29", "32", "56", "144"],
            run: s44_settings_from_four_layers,
        },
        Scenario {
            name: "settings: the host gives each kind a model, and a repository's kind its own, from host-kind and host-repository",
            cites: &["32", "56"],
            run: a_model_for_each_kind,
        },
        Scenario {
            name: "settings: an agent starts with its account's variables read against the host user's home and login environment, status naming each from the account and no other",
            cites: &["29", "32", "144", "56"],
            run: the_accounts_variables,
        },
        Scenario {
            name: "settings: the host's configuration changing an account's variable while a stage works leaves the agent's and gives the next start the new one",
            cites: &["S44", "8", "32", "144"],
            run: the_hosts_configuration_changes_while_a_stage_works,
        },
        Scenario {
            name: "settings: an account added to a stage's pool while its agent works leaves the agent and may be taken by the next start, the place's own keeping to its intent's account",
            cites: &["32", "144", "145"],
            run: an_account_added_to_a_pool,
        },
        Scenario {
            name: "settings: a program no agent profile binds refuses the launch, naming the program, the stage and the layer",
            cites: &["4", "32"],
            run: an_unbound_program_refuses_the_launch,
        },
        Scenario {
            name: "settings: a later start whose account names a variable the login environment lacks is refused naming it and the account, until a retry",
            cites: &["27", "32", "8", "144"],
            run: a_later_start_refused_until_a_retry,
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

/// The host's own pool, `work`, of one account `work-1` giving
/// `variables`.
fn one_account(world: &World, variables: &[(&str, &str)]) {
    let mut work_1 = account("work-1", None);
    work_1.env = variables
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_string()))
        .collect();
    pool(world, "work", Strategy::FillFirst, vec![work_1]);
    world.host.set_host_pool(Some("work"));
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
    one_account(&world, &[("AGENT_HOME", "~/.agent-work")]);
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
    ensure_eq!(
        command.env.get("AGENT_HOME").map(String::as_str),
        Some("/home/user/.agent-work"),
        "code's variable"
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
            layer: Layer::Account,
            value: SET.into()
        }),
        "the variable's source, its value unsaid"
    );
    ensure_eq!(
        agent.settings.env.get("AGENT_HOME").map(String::as_str),
        Some(SET),
        "status names the variable and gives no value"
    );
    ensure_eq!(
        from.get("program").map(|given| given.layer),
        Some(Layer::Default),
        "the program's layer"
    );
    to_verify(&world, &id, "plan")?;
    let verify = opened(&world, &World::identity("change", "plan", "verify", 1))?;
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

fn a_model_for_each_kind(_: &()) -> Outcome {
    let world = World::new();
    world
        .host
        .set_settings(None, "change", for_every_stage(model("opus")));
    world
        .host
        .set_settings(None, "fix", for_every_stage(model("sonnet")));
    world
        .host
        .set_settings(Some("/elsewhere"), "fix", for_every_stage(model("haiku")));
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world
        .host
        .set_settings(Some(REPOSITORY), "fix", for_every_stage(model("haiku")));
    launched(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    let model_of = |kind: &str, name: &str| -> Result<Option<Given>, String> {
        Ok(world
            .status(&World::id(kind, name))?
            .agent
            .ok_or("no agent")?
            .settings
            .from
            .get("model")
            .cloned())
    };
    let given = |layer, value: &str| {
        Some(Given {
            layer,
            value: value.into(),
        })
    };
    ensure_eq!(
        model_of("change", "plan")?,
        given(Layer::HostKind, "opus"),
        "the change's model"
    );
    ensure_eq!(
        model_of("fix", "one")?,
        given(Layer::HostKind, "sonnet"),
        "the fix's model where the host names another repository's"
    );
    ensure_eq!(
        model_of("fix", "two")?,
        given(Layer::HostRepository, "haiku"),
        "the fix's model the host names for its repository"
    );
    Ok(())
}

fn the_accounts_variables(_: &()) -> Outcome {
    let world = World::new();
    one_account(
        &world,
        &[
            ("AGENT_HOME", "~/.agent-work-1"),
            ("TOOL_HOME", "$HOME/tool"),
        ],
    );
    launched(&world.engine.launch(&launch("fix", "one", Some("mend it"))))?;
    let command = opened(&world, &World::identity("fix", "one", "code", 1))?;
    ensure_eq!(
        command.env.get("AGENT_HOME").map(String::as_str),
        Some("/home/user/.agent-work-1"),
        "AGENT_HOME"
    );
    ensure_eq!(
        command.env.get("TOOL_HOME").map(String::as_str),
        Some("/home/user/tool"),
        "TOOL_HOME"
    );
    let agent = world
        .status(&World::id("fix", "one"))?
        .agent
        .ok_or("no agent")?;
    let variables: Vec<(&str, Layer)> = agent
        .settings
        .from
        .iter()
        .filter_map(|(key, given)| key.strip_prefix("env.").map(|name| (name, given.layer)))
        .collect();
    ensure_eq!(
        variables,
        vec![
            ("AGENT_HOME", Layer::Account),
            ("TOOL_HOME", Layer::Account)
        ],
        "the variables status names, each from the account"
    );
    ensure_eq!(
        agent.settings.env.keys().collect::<Vec<_>>(),
        vec!["AGENT_HOME", "TOOL_HOME"],
        "the variables of the agent's settings"
    );
    Ok(())
}

fn the_hosts_configuration_changes_while_a_stage_works(_: &()) -> Outcome {
    let world = World::new();
    one_account(&world, &[("AGENT_HOME", "~/.agent-work")]);
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    to_code(&world, &id, "plan")?;
    let closes = world.terminal.log.count("close");
    one_account(&world, &[("AGENT_HOME", "~/.other")]);
    world.settle(&id);
    let code = opened(&world, &World::identity("change", "plan", "code", 1))?;
    ensure_eq!(
        code.env.get("AGENT_HOME").map(String::as_str),
        Some("/home/user/.agent-work"),
        "code's agent keeps the value it started with"
    );
    ensure_eq!(
        (
            world.terminal.log.count("close"),
            world.stage_state(&id, "code")
        ),
        (closes, Some(StageState::Begun)),
        "agents ended and code's state after the configuration's change"
    );
    let held = world.held(&id).ok_or("no held state")?;
    let kept = held.stages[1].start.as_ref().ok_or("no start of code")?;
    ensure_eq!(
        kept.from
            .get("env.AGENT_HOME")
            .map(|given| given.value.as_str()),
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
    Ok(())
}

fn an_account_added_to_a_pool(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("work-1", None)],
    );
    world.host.set_host_pool(Some("work"));
    let id = World::id("change", "plan");
    launched(
        &world
            .engine
            .launch(&launch("change", "plan", Some("plan it"))),
    )?;
    to_code(&world, &id, "plan")?;
    let closes = world.terminal.log.count("close");
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("work-0", None), account("work-1", None)],
    );
    world.settle(&id);
    let account_of = |stage: usize| -> Result<Option<String>, String> {
        Ok(world.held(&id).ok_or("no held state")?.stages[stage]
            .start
            .as_ref()
            .and_then(|start| start.account.as_ref())
            .map(|account| account.name.clone()))
    };
    ensure_eq!(account_of(1)?, Some("work-1".to_string()), "code's account");
    ensure_eq!(world.terminal.log.count("close"), closes, "agents ended");
    to_verify(&world, &id, "plan")?;
    ensure_eq!(
        account_of(2)?,
        Some("work-1".to_string()),
        "verify's account, its intent's"
    );
    let other = fix_in(&world, None, "other")?;
    ensure_eq!(
        super::account_pools::account_of(&world, &other, "code"),
        Some(("work".to_string(), "work-0".to_string())),
        "another intent's start, on the account added first in order"
    );
    Ok(())
}

fn an_unbound_program_refuses_the_launch(_: &()) -> Outcome {
    let world = World::new();
    let mut change = launch("change", "plan", Some("plan it"));
    change.settings.insert(
        "code".into(),
        Settings {
            program: Some("nothing-here".into()),
            ..Settings::default()
        },
    );
    let reason = refused(&world.engine.launch(&change))?;
    ensure!(
        reason.contains("nothing-here") && reason.contains("code") && reason.contains("launch"),
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
    one_account(&world, &[("X", "$NOT_SET")]);
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
        refusal.contains("NOT_SET") && refusal.contains("work-1"),
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

//! A start checked against its settings (161; core.10, core.30): each
//! start records what its agent is to run under, its program, its mode as
//! its profile passes it, its account and each variable's digest keyed by
//! its secret, never a value, leaving
//! out what a terminal sets for itself and what the agent program keeps
//! from the processes it starts; an agent whose word shows it began
//! otherwise is not taken as its stage's, is sent no prompt, and once
//! quiet is ended and its stage started afresh under its settings,
//! counted toward no bound; a fresh start that begins so again, or one
//! that never goes quiet, is never begun, its end naming the same; status
//! names each variable alone, and a mode that differed by the mode asked
//! and the mode begun; a profile that reads no mode back compares none
//! (31, 35, 37, 43, 56, 112, 135, 161, 162, 169).

use super::account_pools::pool;
use super::agent_modes::{in_mode, passed, the_fakes_map};
use super::agent_settings::for_every_stage;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::variable_digest;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "start integrity: a start's record names each variable of its account and flume's own with a digest keyed by its secret, and its token's value is in no held state, answer or log",
            cites: &["161", "32", "56"],
            run: the_record_holds_no_value,
        },
        Scenario {
            name: "start integrity: a start's record names its mode as its profile passes it, and a start whose settings give no mode names none",
            cites: &["161", "162", "S66"],
            run: the_mode_as_passed,
        },
        Scenario {
            name: "start integrity: PATH and SHELL, which a terminal sets for its own sake, are in no start's record, even where an account gives them",
            cites: &["161"],
            run: what_the_terminal_sets_is_left_out,
        },
        Scenario {
            name: "start integrity: a variable the agent program keeps from every process it starts is in no start's record, and the account's others are",
            cites: &["161"],
            run: what_the_program_withholds_is_left_out,
        },
        Scenario {
            name: "start integrity: an agent whose word shows a variable its record names missing is not taken: started reads no and the start reads begun without its settings, naming the variable",
            cites: &["135", "161", "35"],
            run: a_variable_missing,
        },
        Scenario {
            name: "start integrity: an agent begun with its program and every variable as its record names them is taken and sent its prompt",
            cites: &["35", "37", "161"],
            run: everything_matches,
        },
        Scenario {
            name: "start integrity: an agent come up without its account's variable is ended once quiet and its stage started afresh under the account with its environment, status naming the variable alone and the kind's bound not counted",
            cites: &["S65", "161", "31", "112", "56"],
            run: s65_a_pane_brought_back_without_its_environment,
        },
        Scenario {
            name: "start integrity: a fresh start whose agent also comes up without the variable ends never begun, its end naming the variable, delivered to the place's recipient once",
            cites: &["161", "35", "43"],
            run: twice_in_a_row,
        },
        Scenario {
            name: "start integrity: a start whose agent began without its settings and never goes quiet ends never begun at the stage's limit, its end naming the variable",
            cites: &["161", "35", "43"],
            run: never_quiet_until_the_limit,
        },
        Scenario {
            name: "start integrity: no prompt is sent to an agent begun without its settings, before or after it is ended",
            cites: &["37", "161"],
            run: no_prompt_to_the_agent_ended,
        },
        Scenario {
            name: "place status: a start whose agent began without its settings is named with each variable by its name and no value",
            cites: &["56", "161"],
            run: status_of_a_start_without_its_settings,
        },
        Scenario {
            name: "start integrity: an agent whose word shows every variable as recorded but another mode than its record's all is not taken: started reads no and the start reads begun without its settings, naming the mode asked all and the mode begun ask-each",
            cites: &["135", "161", "162"],
            run: another_mode,
        },
        Scenario {
            name: "start integrity: a code stage given full whose agent begins in another mode, as its account's configuration disabling the mode leaves it, is ended once quiet and started afresh counting no bound, and the fresh start begun so again is never begun, its end and status naming the mode asked and the mode begun",
            cites: &["S67", "35", "161", "162", "31", "112", "56"],
            run: s67_a_mode_the_accounts_configuration_disables,
        },
        Scenario {
            name: "start integrity: where the agent's profile reads no mode back, a start given edits records the value passed, its word that it started is taken with no mode in it, and status gives the mode as not read back",
            cites: &["161", "169"],
            run: the_fake_reading_no_mode_back,
        },
        Scenario {
            name: "seam fakes: the fake agent scripted to map full to all and its code start to begin in ask-each: the core records the start's mode all, reads it begun without its settings naming both modes, and the command passed all",
            cites: &["161", "169", "106"],
            run: a_scripted_start_in_another_mode,
        },
    ]
}

/// The value of the account's token, which nothing of flume's keeps.
const TOKEN: &str = "token-value-of-work-1";

/// The host's pool `work` of the one account `work-1`, whose environment
/// is `env`.
fn work_pool(world: &World, env: &[(&str, &str)]) {
    pool(
        world,
        "work",
        Strategy::FillFirst,
        vec![Account {
            name: "work-1".into(),
            program: "agent".into(),
            env: env
                .iter()
                .map(|(name, value)| (name.to_string(), value.to_string()))
                .collect(),
            agents: None,
        }],
    );
    world.host.set_host_pool(Some("work"));
}

/// The account's environment: a home folder and a token.
const ENV: [(&str, &str); 2] = [("AGENT_HOME", "/s/work-1"), ("AGENT_TOKEN", TOKEN)];

/// The fix `name` launched for the recipient `ops`, an intent of its own,
/// settled once.
fn fix(world: &World, name: &str) -> Result<PlaceId, String> {
    let mut given = launch("fix", name, Some("x"));
    given.intent = Some(name.into());
    given.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&given))?;
    let id = World::id("fix", name);
    world.settle(&id);
    Ok(id)
}

/// The fake agent of `name`'s code start `start` begins without
/// `AGENT_HOME`, its word saying so.
fn begins_without_home(world: &World, name: &str, start: u32) -> Identity {
    let who = World::identity("fix", name, "code", start);
    world.agent.set_started_without(
        &who,
        Unsettled {
            missing: vec!["AGENT_HOME".into()],
            ..Unsettled::default()
        },
    );
    who
}

/// The code stage's current start.
fn code_start(world: &World, id: &PlaceId) -> Result<StartHeld, String> {
    world
        .held(id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or_else(|| "no start of code".to_string())
}

fn the_record_holds_no_value(_: &()) -> Outcome {
    let world = World::new();
    work_pool(&world, &ENV);
    let id = fix(&world, "one")?;
    let start = code_start(&world, &id)?;
    let record = &start.start_record;
    ensure_eq!(
        record.env.keys().map(String::as_str).collect::<Vec<_>>(),
        vec![
            "AGENT_HOME",
            "AGENT_TOKEN",
            "FLUME_AGENT",
            "FLUME_PLACE",
            "FLUME_STAGE"
        ],
        "the variables the record names"
    );
    ensure_eq!(record.program, "agent".to_string(), "the record's program");
    ensure_eq!(
        record.account,
        Some("work-1".to_string()),
        "the record's account"
    );
    let held = world.held(&id).ok_or("no held state")?;
    let location = &held.place.location;
    for (name, value) in [
        ("AGENT_HOME", "/s/work-1"),
        ("AGENT_TOKEN", TOKEN),
        ("FLUME_AGENT", start.identity.to_string().as_str()),
        ("FLUME_PLACE", location.as_str()),
        ("FLUME_STAGE", "code"),
    ] {
        ensure_eq!(
            record.env.get(name),
            Some(&variable_digest(&start.secret, name, value)),
            "{name}'s digest"
        );
        ensure!(
            record.env.get(name) != Some(&variable_digest(&Secret("another".into()), name, value)),
            "{name}'s digest under another secret is the same"
        );
    }
    let held_text = serde_json::to_string(&held).map_err(|error| error.to_string())?;
    let status_text =
        serde_json::to_string(&world.status(&id)?).map_err(|error| error.to_string())?;
    ensure!(
        !held_text.contains(TOKEN),
        "the token's value is in held state"
    );
    ensure!(
        !status_text.contains(TOKEN),
        "the token's value is in status"
    );
    ensure!(
        !world
            .host
            .logged
            .calls()
            .iter()
            .any(|line| line.contains(TOKEN)),
        "the token's value is in the host's log"
    );
    Ok(())
}

fn the_mode_as_passed(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    work_pool(&world, &ENV);
    world
        .host
        .set_settings(None, "fix", for_every_stage(in_mode(Mode::Edits)));
    let one = fix(&world, "one")?;
    ensure_eq!(
        code_start(&world, &one)?.start_record.mode,
        Some("edit-files".to_string()),
        "the record's mode where the settings give edits"
    );
    world
        .host
        .set_settings(None, "fix", KindSettings::default());
    let two = fix(&world, "two")?;
    ensure_eq!(
        code_start(&world, &two)?.start_record.mode,
        None,
        "the record's mode where the settings give none"
    );
    Ok(())
}

fn what_the_terminal_sets_is_left_out(_: &()) -> Outcome {
    let world = World::new();
    work_pool(
        &world,
        &[
            ("AGENT_HOME", "/s/work-1"),
            ("PATH", "/s/bin"),
            ("SHELL", "/bin/sh"),
        ],
    );
    let id = fix(&world, "one")?;
    let record = code_start(&world, &id)?.start_record;
    ensure!(
        !record.env.contains_key("PATH") && !record.env.contains_key("SHELL"),
        "the record names what the terminal sets: {:?}",
        record.env.keys().collect::<Vec<_>>()
    );
    ensure!(
        record.env.contains_key("AGENT_HOME"),
        "the record leaves out the account's own variable"
    );
    Ok(())
}

fn what_the_program_withholds_is_left_out(_: &()) -> Outcome {
    let world = World::new();
    world.agent.withhold(&["AGENT_TOKEN"]);
    work_pool(&world, &ENV);
    let id = fix(&world, "one")?;
    let record = code_start(&world, &id)?.start_record;
    ensure_eq!(
        record.env.keys().map(String::as_str).collect::<Vec<_>>(),
        vec!["AGENT_HOME", "FLUME_AGENT", "FLUME_PLACE", "FLUME_STAGE"],
        "the variables the record names"
    );
    let who = World::identity("fix", "one", "code", 1);
    let (command, _) = world
        .terminal
        .opened(&who)
        .ok_or("the start opened no agent")?;
    ensure_eq!(
        command.env.get("AGENT_TOKEN").map(String::as_str),
        Some(TOKEN),
        "the agent's AGENT_TOKEN"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, its agent begun as recorded"
    );
    Ok(())
}

fn a_variable_missing(_: &()) -> Outcome {
    let world = World::new();
    work_pool(&world, &ENV);
    let who = begins_without_home(&world, "one", 1);
    world.agent.set_own_work(&who, Reading::Yes);
    let id = fix(&world, "one")?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Starting),
        "code, its agent begun without its settings"
    );
    let status = world.status(&id)?;
    let agent = status.agent.ok_or("status gives no agent")?;
    ensure_eq!(agent.started, Reading::No, "started");
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
                missing: vec!["AGENT_HOME".into()],
                ..Unsettled::default()
            }
        )],
        "the start read begun without its settings"
    );
    ensure_eq!(
        world.terminal.log.count("close"),
        0,
        "an agent at work ended"
    );
    Ok(())
}

fn everything_matches(_: &()) -> Outcome {
    let world = World::new();
    work_pool(&world, &ENV);
    let id = fix(&world, "one")?;
    let who = World::identity("fix", "one", "code", 1);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, its agent begun as recorded"
    );
    ensure!(
        !world.terminal.sent_to(&who).is_empty(),
        "no prompt was sent"
    );
    ensure!(
        world
            .held(&id)
            .is_some_and(|held| held.unsettled.is_empty()),
        "a start read begun without its settings"
    );
    Ok(())
}

fn s65_a_pane_brought_back_without_its_environment(_: &()) -> Outcome {
    let world = World::new();
    work_pool(&world, &ENV);
    let first = begins_without_home(&world, "one", 1);
    world.agent.set_own_work(&first, Reading::Yes);
    let id = fix(&world, "one")?;
    world.agent.set_own_work(&first, Reading::No);
    world.settle(&id);
    ensure_eq!(
        world.terminal.log.of("close"),
        vec![format!("close {first}")],
        "the agents ended"
    );
    let held = world.held(&id).ok_or("no held state")?;
    let start = code_start(&world, &id)?;
    ensure_eq!(start.n, 2, "code's start");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code's fresh start"
    );
    ensure_eq!(
        start.account.map(|account| account.name),
        Some("work-1".to_string()),
        "the fresh start's account"
    );
    let second = World::identity("fix", "one", "code", 2);
    let (command, _) = world
        .terminal
        .opened(&second)
        .ok_or("the fresh start opened no agent")?;
    ensure_eq!(
        command.env.get("AGENT_HOME").map(String::as_str),
        Some("/s/work-1"),
        "the fresh agent's AGENT_HOME"
    );
    let words = start.words.ok_or("the fresh start carries no words")?;
    ensure!(
        words.contains("without the settings"),
        "flume's words do not say why the stage started afresh: {words}"
    );
    ensure_eq!(
        held.submission.send_backs,
        0,
        "the kind's bound, counted by the place's send-backs"
    );
    ensure!(
        held.ends.is_empty(),
        "an end recorded for the start begun without its settings"
    );
    let status = world.status(&id)?;
    ensure!(
        status.waiting_on.is_none(),
        "the place waits on its caller: {:?}",
        status.waiting_on
    );
    ensure_eq!(
        status
            .unsettled
            .iter()
            .map(|read| (read.start, read.unsettled.missing.clone()))
            .collect::<Vec<_>>(),
        vec![(1, vec!["AGENT_HOME".to_string()])],
        "status names the variable"
    );
    let text = serde_json::to_string(&status).map_err(|error| error.to_string())?;
    ensure!(!text.contains("/s/work-1"), "status holds the value");
    Ok(())
}

fn twice_in_a_row(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    work_pool(&world, &ENV);
    begins_without_home(&world, "one", 1);
    begins_without_home(&world, "one", 2);
    let id = fix(&world, "one")?;
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::NeverBegun),
        "code, its agent begun without its settings twice"
    );
    let held = world.held(&id).ok_or("no held state")?;
    ensure_eq!(held.stages[0].starts, 2, "code's starts, no third begun");
    let ends: Vec<&EndHeld> = held
        .ends
        .iter()
        .filter(|end| matches!(&end.of, EndOf::Stage { .. }))
        .collect();
    ensure_eq!(ends.len(), 1, "the stage's ends");
    let EndOf::Stage {
        start, state, end, ..
    } = &ends[0].of
    else {
        return Err("no stage end".into());
    };
    ensure_eq!(
        (*start, *state),
        (2, StageState::NeverBegun),
        "the end recorded"
    );
    ensure_eq!(
        end.unsettled
            .as_ref()
            .map(|unsettled| unsettled.missing.clone()),
        Some(vec!["AGENT_HOME".to_string()]),
        "the end names the variable"
    );
    ensure_eq!(
        ends[0]
            .deliveries
            .iter()
            .map(|delivery| delivery.recipient.as_str())
            .collect::<Vec<_>>(),
        vec!["ops"],
        "the end's deliveries"
    );
    world.settle(&id);
    let again = world.held(&id).ok_or("no held state")?;
    ensure_eq!(
        again
            .ends
            .iter()
            .map(|end| end.deliveries.len())
            .sum::<usize>(),
        1,
        "the end delivered once"
    );
    let notices = world.terminal.log.of("send_to");
    ensure_eq!(notices.len(), 1, "the notices sent");
    Ok(())
}

fn never_quiet_until_the_limit(_: &()) -> Outcome {
    let world = World::new();
    work_pool(&world, &ENV);
    let who = begins_without_home(&world, "one", 1);
    world.agent.set_own_work(&who, Reading::Yes);
    let id = fix(&world, "one")?;
    world.host.advance(flume_core::QUIET_LIMIT);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::NeverBegun),
        "code, its agent at work past the stage's limit"
    );
    let held = world.held(&id).ok_or("no held state")?;
    let end = held
        .ends
        .iter()
        .find_map(|end| match &end.of {
            EndOf::Stage { start: 1, end, .. } => Some(end.clone()),
            _ => None,
        })
        .ok_or("no end of code's start")?;
    ensure_eq!(
        end.unsettled
            .as_ref()
            .map(|unsettled| unsettled.missing.clone()),
        Some(vec!["AGENT_HOME".to_string()]),
        "the end names the variable"
    );
    ensure!(
        world.terminal.sent_to(&who).is_empty(),
        "a prompt sent to the agent begun without its settings"
    );
    Ok(())
}

fn no_prompt_to_the_agent_ended(_: &()) -> Outcome {
    let world = World::new();
    work_pool(&world, &ENV);
    let first = begins_without_home(&world, "one", 1);
    world.agent.set_own_work(&first, Reading::Yes);
    let id = fix(&world, "one")?;
    world.settle(&id);
    ensure!(
        world.terminal.sent_to(&first).is_empty(),
        "a prompt sent before the agent was ended"
    );
    world.agent.set_own_work(&first, Reading::No);
    world.settle(&id);
    ensure!(
        world.terminal.sent_to(&first).is_empty(),
        "a prompt sent to the agent ended"
    );
    ensure!(
        !world
            .terminal
            .sent_to(&World::identity("fix", "one", "code", 2))
            .is_empty(),
        "the fresh agent was sent no prompt"
    );
    Ok(())
}

fn status_of_a_start_without_its_settings(_: &()) -> Outcome {
    let world = World::new();
    work_pool(&world, &ENV);
    let who = begins_without_home(&world, "one", 1);
    world.agent.set_own_work(&who, Reading::Yes);
    let id = fix(&world, "one")?;
    let status = world.status(&id)?;
    let read = status
        .unsettled
        .first()
        .ok_or("status names no start begun without its settings")?;
    ensure_eq!(
        (read.stage.as_str(), read.start),
        ("code", 1),
        "the start named"
    );
    ensure_eq!(
        read.unsettled.missing,
        vec!["AGENT_HOME".to_string()],
        "the variable named"
    );
    let text = serde_json::to_string(&status).map_err(|error| error.to_string())?;
    ensure!(
        !text.contains("/s/work-1") && !text.contains(TOKEN),
        "status holds a value"
    );
    Ok(())
}

/// The fake maps the four and reads the mode back, and the host's table
/// gives every stage of fix the mode `mode`.
fn in_a_mode_read_back(world: &World, mode: Mode) {
    the_fakes_map(world);
    world.agent.read_mode_back();
    world
        .host
        .set_settings(None, "fix", for_every_stage(in_mode(mode)));
}

/// What an agent begun in `ask-each` where its start passed `all` read.
fn all_begun_in_ask_each() -> Unsettled {
    Unsettled {
        mode: Some(ModeBegan {
            asked: "all".into(),
            began: Some("ask-each".into()),
        }),
        ..Unsettled::default()
    }
}

fn another_mode(_: &()) -> Outcome {
    let world = World::new();
    in_a_mode_read_back(&world, Mode::Full);
    work_pool(&world, &ENV);
    let who = World::identity("fix", "one", "code", 1);
    world.agent.set_began_in(&who, "ask-each");
    world.agent.set_own_work(&who, Reading::Yes);
    let id = fix(&world, "one")?;
    ensure_eq!(
        code_start(&world, &id)?.start_record.mode,
        Some("all".to_string()),
        "the record's mode"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Starting),
        "code, its agent begun in another mode"
    );
    let status = world.status(&id)?;
    let agent = status.agent.ok_or("status gives no agent")?;
    ensure_eq!(agent.started, Reading::No, "started");
    ensure_eq!(
        status
            .unsettled
            .iter()
            .map(|read| (read.stage.as_str(), read.start, read.unsettled.clone()))
            .collect::<Vec<_>>(),
        vec![("code", 1, all_begun_in_ask_each())],
        "the start read begun without its settings"
    );
    ensure!(
        world.terminal.sent_to(&who).is_empty(),
        "a prompt sent to the agent begun in another mode"
    );
    Ok(())
}

fn s67_a_mode_the_accounts_configuration_disables(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    in_a_mode_read_back(&world, Mode::Full);
    let first = World::identity("fix", "one", "code", 1);
    let second = World::identity("fix", "one", "code", 2);
    world.agent.set_began_in(&first, "ask-each");
    world.agent.set_began_in(&second, "ask-each");
    world.agent.set_own_work(&first, Reading::Yes);
    let id = fix(&world, "one")?;
    ensure_eq!(
        world.terminal.log.count("close"),
        0,
        "an agent at work ended"
    );
    world.agent.set_own_work(&first, Reading::No);
    world.settle(&id);
    ensure_eq!(
        world.terminal.log.of("close").first().cloned(),
        Some(format!("close {first}")),
        "the first agent ended once quiet"
    );
    let start = code_start(&world, &id)?;
    ensure_eq!(start.n, 2, "code's fresh start");
    let words = start.words.ok_or("the fresh start carries no words")?;
    ensure!(
        words.contains("without the settings"),
        "flume's words do not say why the stage started afresh: {words}"
    );
    ensure_eq!(
        (passed(&world, &first)?, passed(&world, &second)?),
        (Some("all".to_string()), Some("all".to_string())),
        "the mode each start passed"
    );
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::NeverBegun),
        "code, its agent begun in another mode twice"
    );
    let held = world.held(&id).ok_or("no held state")?;
    ensure_eq!(held.stages[0].starts, 2, "code's starts, no third begun");
    ensure_eq!(
        held.submission.send_backs,
        0,
        "the kind's bound, counted by the place's send-backs"
    );
    let end = held
        .ends
        .iter()
        .find_map(|end| match &end.of {
            EndOf::Stage {
                start: 2,
                state: StageState::NeverBegun,
                end,
                ..
            } => Some(end.clone()),
            _ => None,
        })
        .ok_or("no never-begun end of code's fresh start")?;
    ensure_eq!(
        end.unsettled.map(|unsettled| *unsettled),
        Some(all_begun_in_ask_each()),
        "the end names both modes"
    );
    let status = world.status(&id)?;
    ensure_eq!(
        status
            .unsettled
            .iter()
            .map(|read| (read.start, read.unsettled.mode.clone()))
            .collect::<Vec<_>>(),
        vec![
            (1, all_begun_in_ask_each().mode),
            (2, all_begun_in_ask_each().mode)
        ],
        "status names both modes of each start"
    );
    ensure!(
        world.terminal.sent_to(&first).is_empty() && world.terminal.sent_to(&second).is_empty(),
        "a prompt sent to an agent begun in another mode"
    );
    Ok(())
}

fn the_fake_reading_no_mode_back(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    world
        .host
        .set_settings(None, "fix", for_every_stage(in_mode(Mode::Edits)));
    let who = World::identity("fix", "one", "code", 1);
    world.agent.set_began_in(&who, "ask-each");
    let id = fix(&world, "one")?;
    ensure_eq!(
        code_start(&world, &id)?.start_record.mode,
        Some("edit-files".to_string()),
        "the record's mode"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code, its word taken with no mode in it"
    );
    let status = world.status(&id)?;
    ensure!(
        status.unsettled.is_empty(),
        "a start read begun without its settings: {:?}",
        status.unsettled
    );
    let agent = status.agent.ok_or("status gives no agent")?;
    ensure_eq!(agent.started, Reading::Yes, "started");
    ensure_eq!(
        agent.mode.map(|mode| (mode.passed, mode.read_back)),
        Some(("edit-files".to_string(), false)),
        "the agent's mode, not read back"
    );
    Ok(())
}

fn a_scripted_start_in_another_mode(_: &()) -> Outcome {
    let world = World::new();
    world.agent.map_mode(Mode::Full, "all");
    world.agent.read_mode_back();
    world
        .host
        .set_settings(None, "fix", for_every_stage(in_mode(Mode::Full)));
    let who = World::identity("fix", "one", "code", 1);
    world.agent.set_began_in(&who, "ask-each");
    world.agent.set_own_work(&who, Reading::Yes);
    let id = fix(&world, "one")?;
    ensure_eq!(
        code_start(&world, &id)?.start_record.mode,
        Some("all".to_string()),
        "the record's mode"
    );
    ensure_eq!(
        world.held(&id).map(|held| held
            .unsettled
            .iter()
            .map(|read| read.unsettled.clone())
            .collect()),
        Some(vec![all_begun_in_ask_each()]),
        "the start read begun without its settings"
    );
    ensure!(
        world
            .agent
            .log
            .of("mode")
            .iter()
            .all(|line| line == "mode full")
            && !world.agent.log.of("mode").is_empty(),
        "the modes the fake mapped: {:?}",
        world.agent.log.of("mode")
    );
    ensure_eq!(
        passed(&world, &who)?,
        Some("all".to_string()),
        "the mode the command passed"
    );
    Ok(())
}

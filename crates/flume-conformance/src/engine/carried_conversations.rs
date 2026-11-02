//! Carried conversations (146, 175): where an agent stopped at its
//! account's usage limit, the stage's next start carries that agent's
//! own conversation where the agent profile answers, under the room lock
//! once an account is chosen, that its program can carry it there; its
//! prompt is flume's words and the way to stop short alone, and its
//! record names the session it resumes. Where the profile answers no, or
//! offers no carry, the stage starts afresh as before, saying why. No
//! other start carries a conversation: a retry's, a fix round's and
//! another stage's carry none (30).

use super::account_pools::*;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Retry;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "carried conversations: once quiet, an agent stopped at work-1's usage limit is ended and its stage starts under work-2 resuming its conversation, with no caller's word and no bound of the kind's counted",
            cites: &["S76", "146", "175", "112"],
            run: s76_carried_to_the_pools_other_account,
        },
        Scenario {
            name: "carried conversations: a stage whose agent stopped at its limit waits while no account of its pool is usable, and the start that takes an account once the reset passes carries the conversation",
            cites: &["145", "146", "175"],
            run: a_carried_start_waits_for_a_usable_account,
        },
        Scenario {
            name: "carried conversations: a pool's only account spent until its reset, the start after it takes the same account and carries the conversation",
            cites: &["146", "175"],
            run: a_pool_of_one_account,
        },
        Scenario {
            name: "carried conversations: a retry of a stage waiting after its agent stopped at its limit starts with the stage's prompt and the retry's words, resuming no conversation",
            cites: &["30", "49", "146"],
            run: a_retry_carries_nothing,
        },
        Scenario {
            name: "carried conversations: once a carried start delivers, the next stage's agent resumes no conversation",
            cites: &["30", "175"],
            run: the_next_stage_carries_nothing,
        },
        Scenario {
            name: "carried conversations: a carried start's prompt is flume's words and the way to stop short, holding nothing of the stage's template",
            cites: &["28", "146", "175"],
            run: the_prompt_of_a_carried_start,
        },
        Scenario {
            name: "carried conversations: where the profile answers that work-2 does not reach the conversation, the stage starts afresh under work-2 with flume's words, saying why by the variable's name alone",
            cites: &["S55", "146", "175"],
            run: accounts_that_share_nothing,
        },
        Scenario {
            name: "carried conversations: under a profile that offers no carry the stage starts afresh, saying its agent profile carries no conversation",
            cites: &["146", "171", "175"],
            run: a_profile_that_carries_nothing,
        },
    ]
}

/// The fix `name`'s code start `start`.
fn code(name: &str, start: u32) -> Identity {
    World::identity("fix", name, "code", start)
}

/// The start the stage `stage` of the place `id` last made.
fn start_of(world: &World, id: &PlaceId, stage: &str) -> Result<StartHeld, String> {
    world
        .held(id)
        .ok_or("no held state")?
        .stages
        .into_iter()
        .find(|held| held.name == stage)
        .and_then(|held| held.start)
        .ok_or_else(|| format!("{stage} has no start"))
}

/// The fake agent carries a stopped start's conversation to the account
/// at `name`'s home with the session `s-1`.
fn carries_to(world: &World, name: &str) {
    world.agent.carry_to(
        &home(name),
        Carry::Yes {
            session: "s-1".into(),
        },
    );
}

/// The words a carried start's agent is sent: flume's, then the way to
/// stop short as the fake agent gives it.
fn carried_prompt(world: &World, identity: &Identity) -> String {
    format!(
        "Your last turn stopped at your account's usage limit. This session now goes on under an account of the pool, with its conversation as it stood: carry on from where you stopped.\n\n{}",
        world.agent.stop_short(identity)
    )
}

/// The session the start's command resumes, as the fake terminal opened
/// it.
fn resumed(world: &World, identity: &Identity) -> Result<Option<String>, String> {
    let (command, _) = world
        .terminal
        .opened(identity)
        .ok_or_else(|| format!("{identity} was not opened"))?;
    Ok(command
        .args
        .iter()
        .skip_while(|arg| *arg != "--resume")
        .nth(1)
        .cloned())
}

/// The start carries the conversation of its stage's start 1, resuming
/// `s-1`, as the profile answered it may.
fn carried(world: &World, id: &PlaceId, start: &StartHeld) -> Outcome {
    ensure_eq!(
        start.conversation,
        Some(Conversation {
            from: 1,
            carried: true,
            why: None,
        }),
        format!("{}'s conversation", start.identity)
    );
    ensure_eq!(
        (
            start.start_record.session.as_deref(),
            resumed(world, &start.identity)?
        ),
        (Some("s-1"), Some("s-1".to_string())),
        format!(
            "the session {}'s record names and its command resumes",
            start.identity
        )
    );
    let earlier = Identity {
        start: 1,
        ..start.identity.clone()
    };
    let folder = world.store.folder(id);
    ensure!(
        world.agent.log.of("carry").iter().any(|line| line
            .starts_with(&format!("carry {earlier} in {folder} to "))
            && line.ends_with(": yes s-1")),
        "the fake's log shows no carry of {earlier}'s conversation: {:?}",
        world.agent.log.of("carry")
    );
    Ok(())
}

/// The start carries no conversation: its record names no session, its
/// command resumes none, and it is no start after a spent agent.
fn carries_none(world: &World, start: &StartHeld) -> Outcome {
    ensure_eq!(
        (
            start.conversation.clone(),
            start.start_record.session.clone(),
            resumed(world, &start.identity)?
        ),
        (None, None, None),
        format!("{}'s conversation and session", start.identity)
    );
    Ok(())
}

fn s76_carried_to_the_pools_other_account(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-2");
    let id = begun_fix(&world, "one")?;
    let spent_agent = stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.agent.set_own_work(&spent_agent, Reading::Yes);
    world.settle(&id);
    ensure_eq!(
        (
            world.terminal.log.count("close"),
            world.agent.log.count("carry")
        ),
        (0, 0),
        "agents ended and carries asked while the spent agent was not quiet"
    );
    world.agent.set_own_work(&spent_agent, Reading::No);
    world.settle(&id);
    ensure_eq!(
        world.terminal.log.of("close"),
        vec![format!("close {spent_agent}")],
        "the agents ended"
    );
    let start = start_of(&world, &id, "code")?;
    ensure_eq!(
        (
            start.n,
            start
                .account
                .clone()
                .map(|account| (account.pool, account.name))
        ),
        (2, on("work", "work-2")),
        "the code stage's start and its account"
    );
    carried(&world, &id, &start)?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the carried start's stage"
    );
    let held = world.held(&id).ok_or("no held state")?;
    ensure_eq!(
        (held.submission.send_backs, held.ends.len()),
        (0, 0),
        "the kind's bound, counted by the place's send-backs, and the ends recorded"
    );
    let status = world.status(&id)?;
    ensure!(
        status.waiting_on.is_none(),
        "the place waits on its caller: {:?}",
        status.waiting_on
    );
    Ok(())
}

fn a_carried_start_waits_for_a_usable_account(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-1");
    carries_to(&world, "work-2");
    let fifteen = world.host.now() + 2 * 3600;
    let id = begun_fix(&world, "one")?;
    spent(&world, "work", "work-2", fifteen);
    stops_at_limit(&world, "one", 1, Some(fifteen));
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::WaitingForRoom),
        "the stage while no account of its pool is usable"
    );
    let room = world
        .held(&id)
        .and_then(|held| held.stages[0].room.clone())
        .ok_or("the waiting stage holds no room")?;
    ensure_eq!(
        room.carry,
        Some(AfterSpent {
            start: code("one", 1),
            carry: true,
            why: None,
        }),
        "the waiting stage's carry"
    );
    ensure_eq!(
        world.agent.log.count("carry"),
        0,
        "carries asked with no account chosen"
    );
    world.host.advance(2 * 3600);
    world.engine().settle(None);
    let start = start_of(&world, &id, "code")?;
    ensure_eq!(
        (
            start.n,
            start
                .account
                .clone()
                .map(|account| (account.pool, account.name))
        ),
        (2, on("work", "work-1")),
        "the start once the reset passed and its account"
    );
    carried(&world, &id, &start)?;
    ensure_eq!(
        world.agent.log.count("carry"),
        1,
        "carries asked of the profile"
    );
    Ok(())
}

fn a_pool_of_one_account(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "solo",
        Strategy::FillFirst,
        vec![account_at_home("work-1")],
    );
    world.host.set_host_pool(Some("solo"));
    carries_to(&world, "work-1");
    let fifteen = world.host.now() + 2 * 3600;
    let id = begun_fix(&world, "one")?;
    stops_at_limit(&world, "one", 1, Some(fifteen));
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::WaitingForRoom),
        "the stage while its pool's only account is spent"
    );
    world.host.advance(2 * 3600);
    world.engine().settle(None);
    let start = start_of(&world, &id, "code")?;
    ensure_eq!(
        (
            start.n,
            start
                .account
                .clone()
                .map(|account| (account.pool, account.name))
        ),
        (2, on("solo", "work-1")),
        "the start once the reset passed and its account"
    );
    carried(&world, &id, &start)?;
    ensure!(
        world.held(&id).is_some_and(|held| held.moves.is_empty()),
        "the stage moved, staying on its pool's only account"
    );
    Ok(())
}

fn a_retry_carries_nothing(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "solo",
        Strategy::FillFirst,
        vec![account_at_home("work-1")],
    );
    world.host.set_host_pool(Some("solo"));
    carries_to(&world, "work-1");
    let fifteen = world.host.now() + 2 * 3600;
    let id = begun_fix(&world, "one")?;
    stops_at_limit(&world, "one", 1, Some(fifteen));
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::WaitingForRoom),
        "the stage before the retry"
    );
    let answer = world
        .engine()
        .retry(&id, Retry::Current, Some("try it this way".into()), false);
    if let Some(reason) = answer.refusal() {
        return Err(format!("the retry was refused: {reason}"));
    }
    world.host.advance(2 * 3600);
    world.engine().settle(None);
    let start = start_of(&world, &id, "code")?;
    ensure_eq!(start.n, 2, "the retry's start");
    carries_none(&world, &start)?;
    ensure!(
        start.prompt.text.starts_with("Fix one on fix/one.")
            && start.prompt.text.contains("try it this way"),
        "the retry's prompt is not the stage's with its words: {}",
        start.prompt.text
    );
    ensure_eq!(
        world.agent.log.count("carry"),
        0,
        "carries asked of the profile"
    );
    Ok(())
}

fn the_next_stage_carries_nothing(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-2");
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "fix",
            false,
            vec![
                stage("code", Some(Deliverable::Commits), "{{brief}}"),
                stage("verify", Some(Deliverable::Commits), "{{brief}}"),
            ],
        )],
    );
    let id = begun_fix(&world, "one")?;
    stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    let start = start_of(&world, &id, "code")?;
    carried(&world, &id, &start)?;
    world.commit("fix/one", &["fix.rs"]);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::Begun),
        "verify once the carried code start delivered"
    );
    carries_none(&world, &start_of(&world, &id, "verify")?)?;
    ensure_eq!(
        world.agent.log.count("carry"),
        1,
        "carries asked of the profile"
    );
    Ok(())
}

fn the_prompt_of_a_carried_start(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-2");
    let id = begun_fix(&world, "one")?;
    stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    let carried_start = code("one", 2);
    let due = carried_prompt(&world, &carried_start);
    ensure_eq!(
        world.prompt(&id, &carried_start),
        Some(due.clone()),
        "the carried start's prompt"
    );
    ensure_eq!(
        world.terminal.sent_to(&carried_start),
        vec![due],
        "what the carried start's agent was sent"
    );
    ensure!(
        world
            .terminal
            .sent_to(&carried_start)
            .iter()
            .all(|sent| !sent.contains("Fix one") && !sent.contains("fix/one")),
        "the carried start's agent was sent the code stage's template"
    );
    Ok(())
}

fn accounts_that_share_nothing(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    world.agent.carry_to(
        &home("work-2"),
        Carry::No {
            why: UNREACHED.into(),
        },
    );
    let id = begun_fix(&world, "one")?;
    stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    let start = start_of(&world, &id, "code")?;
    ensure_eq!(
        (
            start.n,
            start
                .account
                .clone()
                .map(|account| (account.pool, account.name))
        ),
        (2, on("work", "work-2")),
        "the fresh start and its account"
    );
    let conversation = start
        .conversation
        .clone()
        .ok_or("the fresh start gives no conversation")?;
    let why = conversation.why.clone().unwrap_or_default();
    ensure!(
        conversation.from == 1
            && !conversation.carried
            && why.contains("CLAUDE_CONFIG_DIR")
            && !why.contains('/'),
        "the fresh start's conversation does not say it stayed and why, by the variable's name alone: {conversation:?}"
    );
    ensure_eq!(
        (
            start.start_record.session.clone(),
            resumed(&world, &start.identity)?
        ),
        (None, None),
        "the session the fresh start resumes"
    );
    ensure!(
        start.prompt.text.starts_with("Fix one on fix/one.")
            && start
                .prompt
                .text
                .contains("The stage's earlier agent stopped at its account's usage limit"),
        "the fresh start's prompt is not the stage's with flume's words: {}",
        start.prompt.text
    );
    Ok(())
}

fn a_profile_that_carries_nothing(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    let id = begun_fix(&world, "one")?;
    stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    let start = start_of(&world, &id, "code")?;
    ensure_eq!(
        (start.n, start.conversation.clone()),
        (
            2,
            Some(Conversation {
                from: 1,
                carried: false,
                why: Some("its agent profile carries no conversation".into()),
            })
        ),
        "the fresh start and its conversation"
    );
    ensure_eq!(
        world.agent.log.of("carry"),
        vec![format!(
            "carry {} in {} to {}: not offered",
            code("one", 1),
            world.store.folder(&id),
            home("work-2")
        )],
        "the carries asked of the profile"
    );
    ensure_eq!(
        resumed(&world, &start.identity)?,
        None,
        "the session the fresh start resumes"
    );
    Ok(())
}

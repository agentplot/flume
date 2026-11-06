//! Carried conversations (146, 179): where an agent stopped at its
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
use flume_fakes::FakeAgent;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "carried conversations: once quiet, an agent stopped at work-1's usage limit is ended and its stage starts under work-2 resuming its conversation, with no caller's word and no bound of the kind's counted",
            cites: &["S78", "146", "179", "112"],
            run: s78_carried_to_the_pools_other_account,
        },
        Scenario {
            name: "carried conversations: a stage whose agent stopped at its limit waits while no account of its pool is usable, and the start that takes an account once the reset passes carries the conversation",
            cites: &["145", "146", "179"],
            run: a_carried_start_waits_for_a_usable_account,
        },
        Scenario {
            name: "carried conversations: a pool's only account spent until its reset, the start after it takes the same account and carries the conversation",
            cites: &["146", "179"],
            run: a_pool_of_one_account,
        },
        Scenario {
            name: "carried conversations: a retry of a stage waiting after its agent stopped at its limit starts with the stage's prompt and the retry's words, resuming no conversation",
            cites: &["30", "49", "146"],
            run: a_retry_carries_nothing,
        },
        Scenario {
            name: "carried conversations: once a carried start delivers, the next stage's agent resumes no conversation",
            cites: &["30", "179"],
            run: the_next_stage_carries_nothing,
        },
        Scenario {
            name: "carried conversations: a carried start's prompt is flume's words and the way to stop short, holding nothing of the stage's template",
            cites: &["28", "146", "179"],
            run: the_prompt_of_a_carried_start,
        },
        Scenario {
            name: "carried conversations: where the profile answers that work-2 does not reach the conversation, the stage starts afresh under work-2 with flume's words, saying why by the variable's name alone",
            cites: &["S55", "146", "179"],
            run: accounts_that_share_nothing,
        },
        Scenario {
            name: "carried conversations: under a profile that offers no carry the stage starts afresh, saying its agent profile carries no conversation",
            cites: &["146", "179"],
            run: a_profile_that_carries_nothing,
        },
        Scenario {
            name: "carried conversations: a carried start whose program finds no such conversation and ends before it says it started delivers no end, and code starts afresh at once with flume's words, saying why and counting no bound",
            cites: &["S79", "146", "179", "35", "112"],
            run: s79_no_such_conversation,
        },
        Scenario {
            name: "carried conversations: a carried start whose agent says it started in another session is not taken, is ended once quiet, and the stage starts afresh carrying nothing",
            cites: &["161", "179", "146"],
            run: another_session,
        },
        Scenario {
            name: "carried conversations: the fresh start after a carried start that ended first, come up without its account's variable, is ended once quiet and the stage starts afresh once more, and the start after it come up so is never begun",
            cites: &["161", "146", "35"],
            run: the_fresh_start_after_it_begins_otherwise,
        },
        Scenario {
            name: "carried conversations: a carried start that does not say it started by the stage's limit, or does not receive its prompt, is sent nothing more, is ended once quiet and the stage starts afresh, saying which",
            cites: &["35", "38", "146", "179"],
            run: by_the_limit_and_not_received,
        },
        Scenario {
            name: "seam fakes: the fake agent scripted to end its carried start before its word: the core starts the stage afresh carrying nothing, the fake's log showing the fresh start's command with no session",
            cites: &["S79", "146", "106"],
            run: a_scripted_carried_start_that_ends_first,
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
            cause: CarryCause::Spent,
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

fn s78_carried_to_the_pools_other_account(_: &()) -> Outcome {
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
        Some(CarryFrom {
            start: code("one", 1),
            cause: CarryCause::Spent,
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
                cause: CarryCause::Spent,
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

/// The fix `name` launched for the recipient `ops`, whose pane waits, an
/// intent of its own, its code begun.
fn begun_for_ops(world: &World, name: &str) -> Result<PlaceId, String> {
    world.terminal.add_session("ops", AgentState::Waiting);
    let mut given = launch("fix", name, Some("x"));
    given.intent = Some(name.into());
    given.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&given))?;
    let id = World::id("fix", name);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "{name}'s code"
    );
    Ok(id)
}

/// No end recorded or delivered, no notice sent, and the kind's bound
/// not counted.
fn nothing_delivered(world: &World, id: &PlaceId) -> Outcome {
    let held = world.held(id).ok_or("no held state")?;
    ensure_eq!(
        (
            held.ends.len(),
            world.terminal.notices("ops").len(),
            held.submission.send_backs
        ),
        (0, 0, 0),
        "the ends recorded, the notices sent and the kind's bound counted"
    );
    ensure!(
        world.status(id)?.waiting_on.is_none(),
        "the place waits on its caller"
    );
    Ok(())
}

/// The fresh start after the carried start `carried`, which did not
/// begin as `why` says: afresh under flume's words for a spent agent,
/// after code's start 1, resuming nothing.
fn afresh_after(world: &World, start: &StartHeld, why: &str) -> Outcome {
    ensure_eq!(
        start.conversation,
        Some(Conversation {
            from: 1,
            cause: CarryCause::Spent,
            carried: false,
            why: Some(why.to_string()),
        }),
        format!("{}'s conversation", start.identity)
    );
    ensure_eq!(
        (
            start.start_record.session.clone(),
            resumed(world, &start.identity)?
        ),
        (None, None),
        format!("the session {} resumes", start.identity)
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

/// The place's carried starts that did not begin, by start and words.
fn uncarried(world: &World, id: &PlaceId) -> Result<Vec<(u32, String)>, String> {
    Ok(world
        .held(id)
        .ok_or("no held state")?
        .uncarried
        .into_iter()
        .map(|read| (read.start, read.words))
        .collect())
}

const ENDED_FIRST: &str = "the start that carried its conversation ended before it said it started";

fn s79_no_such_conversation(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-2");
    world.agent.carried_starts_end();
    let id = begun_for_ops(&world, "one")?;
    stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    let start = start_of(&world, &id, "code")?;
    ensure_eq!(start.n, 3, "code's start once the carried one ended");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code's fresh start"
    );
    afresh_after(&world, &start, ENDED_FIRST)?;
    ensure_eq!(
        uncarried(&world, &id)?,
        vec![(2, ENDED_FIRST.to_string())],
        "the carried starts that did not begin"
    );
    ensure_eq!(
        world.terminal.sent_to(&code("one", 2)),
        Vec::<String>::new(),
        "what the carried start's agent was sent"
    );
    ensure_eq!(
        world.agent.log.count("carry"),
        1,
        "carries asked of the profile"
    );
    nothing_delivered(&world, &id)
}

fn another_session(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-2");
    world.agent.set_session(&code("one", 2), "s-2");
    let id = begun_for_ops(&world, "one")?;
    let carried_start = code("one", 2);
    world.agent.set_own_work(&carried_start, Reading::Yes);
    stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    ensure_eq!(
        (
            world.stage_state(&id, "code"),
            start_of(&world, &id, "code")?.n
        ),
        (Some(StageState::Starting), 2),
        "code while the carried start's agent works"
    );
    let status = world.status(&id)?;
    ensure_eq!(
        status.agent.map(|agent| agent.started),
        Some(Reading::No),
        "the carried start's started ack"
    );
    world.agent.set_own_work(&carried_start, Reading::No);
    world.settle(&id);
    ensure!(
        world
            .terminal
            .log
            .of("close")
            .contains(&format!("close {carried_start}")),
        "the carried start's agent was not ended"
    );
    let start = start_of(&world, &id, "code")?;
    ensure_eq!(start.n, 3, "code's start after the carried one");
    afresh_after(
        &world,
        &start,
        "the start that carried its conversation began in another conversation than the one its start resumes",
    )?;
    ensure_eq!(
        world.terminal.sent_to(&carried_start),
        Vec::<String>::new(),
        "what the carried start's agent was sent"
    );
    ensure_eq!(
        world
            .held(&id)
            .ok_or("no held state")?
            .unsettled
            .iter()
            .map(|read| (read.start, read.unsettled.session))
            .collect::<Vec<_>>(),
        vec![(2, true)],
        "the starts read begun without their settings"
    );
    nothing_delivered(&world, &id)
}

fn the_fresh_start_after_it_begins_otherwise(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-2");
    world.agent.carried_starts_end();
    for start in [3, 4] {
        world.agent.set_started_without(
            &code("one", start),
            Unsettled {
                missing: vec![FakeAgent::HOME.into()],
                ..Unsettled::default()
            },
        );
    }
    let id = begun_for_ops(&world, "one")?;
    stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        (
            world.stage_state(&id, "code"),
            start_of(&world, &id, "code")?.n
        ),
        (Some(StageState::NeverBegun), 4),
        "code once the start after the fresh one came up so too"
    );
    ensure!(
        world
            .terminal
            .log
            .of("close")
            .contains(&format!("close {}", code("one", 3))),
        "the fresh start's agent was not ended"
    );
    let held = world.held(&id).ok_or("no held state")?;
    ensure_eq!(
        held.unsettled
            .iter()
            .map(|read| read.start)
            .collect::<Vec<_>>(),
        vec![3, 4],
        "the starts read begun without their settings"
    );
    ensure_eq!(
        held.stages[0]
            .start
            .as_ref()
            .and_then(|start| start.conversation.clone()),
        None,
        "the conversation of the start after the fresh one"
    );
    ensure_eq!(
        uncarried(&world, &id)?,
        vec![(2, ENDED_FIRST.to_string())],
        "the carried starts that did not begin"
    );
    Ok(())
}

fn by_the_limit_and_not_received(_: &()) -> Outcome {
    for (scripted, why) in [
        (
            "unstarted",
            "the start that carried its conversation did not say it started by the stage's limit",
        ),
        (
            "unreceived",
            "the start that carried its conversation did not receive its prompt",
        ),
    ] {
        let world = World::new();
        fill_first_at_homes(&world);
        carries_to(&world, "work-2");
        let carried_start = code("one", 2);
        if scripted == "unstarted" {
            world.agent.set_started(&carried_start, Reading::No);
        } else {
            world.agent.receives_at(&carried_start, None);
        }
        let id = begun_for_ops(&world, "one")?;
        stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
        world.settle(&id);
        for _ in 0..4 {
            if start_of(&world, &id, "code")?.n > 2 {
                break;
            }
            world.host.advance(flume_core::QUIET_LIMIT);
            world.settle(&id);
        }
        let start = start_of(&world, &id, "code")?;
        ensure_eq!(
            start.n,
            3,
            format!("code's start after its carried start {scripted}")
        );
        afresh_after(&world, &start, why)?;
        ensure_eq!(
            uncarried(&world, &id)?,
            vec![(2, why.to_string())],
            format!("the carried starts that did not begin, {scripted}")
        );
        ensure!(
            world
                .terminal
                .log
                .of("close")
                .contains(&format!("close {carried_start}")),
            "the carried start's agent, {scripted}, was not ended"
        );
        ensure!(
            world.terminal.sent_to(&carried_start).len() <= 2,
            "the carried start's agent, {scripted}, was sent more than its two sendings: {:?}",
            world.terminal.sent_to(&carried_start)
        );
        nothing_delivered(&world, &id)?;
    }
    working_past_the_limit()
}

/// A carried start read not begun at the stage's limit while its agent
/// works: recorded once, and its agent, which then says it started and
/// goes quiet, sent nothing before it is ended (35, 146).
fn working_past_the_limit() -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-2");
    let carried_start = code("one", 2);
    world.agent.set_started(&carried_start, Reading::No);
    world.agent.set_own_work(&carried_start, Reading::Yes);
    let id = begun_for_ops(&world, "one")?;
    stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    world.host.advance(flume_core::QUIET_LIMIT);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        (start_of(&world, &id, "code")?.n, uncarried(&world, &id)?),
        (
            2,
            vec![(
                2,
                "the start that carried its conversation did not say it started by the stage's limit"
                    .to_string()
            )]
        ),
        "code's start and the carried starts that did not begin while its agent works"
    );
    world.agent.set_started(&carried_start, Reading::Yes);
    world.agent.set_own_work(&carried_start, Reading::No);
    world.settle(&id);
    ensure_eq!(
        (
            start_of(&world, &id, "code")?.n,
            world.terminal.sent_to(&carried_start)
        ),
        (3, vec![]),
        "code's start once the carried start's agent is quiet, and what that agent was sent"
    );
    nothing_delivered(&world, &id)
}

fn a_scripted_carried_start_that_ends_first(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-2");
    world.agent.carried_starts_end();
    let id = begun_fix(&world, "one")?;
    stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    ensure_eq!(
        world.agent.log.of("command"),
        vec![
            format!("command {}", code("one", 1)),
            format!("command {} --resume s-1", code("one", 2)),
            format!("command {}", code("one", 3)),
        ],
        "the commands the fake's log shows"
    );
    carries_none_but_after(&world, &start_of(&world, &id, "code")?)
}

/// The start after a carried one that did not begin: its record names no
/// session and its command resumes none.
fn carries_none_but_after(world: &World, start: &StartHeld) -> Outcome {
    ensure_eq!(
        (
            start
                .conversation
                .as_ref()
                .map(|conversation| conversation.carried),
            start.start_record.session.clone(),
            resumed(world, &start.identity)?
        ),
        (Some(false), None, None),
        format!("{}'s conversation and session", start.identity)
    );
    Ok(())
}

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
        Scenario {
            name: "carried conversations: a talk lost with its host, retried, starts under work-1, the account its lost start ran under, resuming s-1, its record naming s-1 and its started ack taken there, status giving its conversation from start 1, lost and carried",
            cites: &["S113", "45", "48", "49", "145", "161", "179"],
            run: s113_a_talk_lost_with_its_host_comes_back_on_its_own_account,
        },
        Scenario {
            name: "carried conversations: a lost talk retried while the host is at its bound waits for room, and the start that takes the room once another place's agent ends carries s-1",
            cites: &["49", "70"],
            run: a_lost_talk_waits_for_room,
        },
        Scenario {
            name: "carried conversations: a lost talk's carried start that ends before it says it started delivers no end, and talk starts afresh at once with flume's words for a lost conversation then the retry's, saying why",
            cites: &["S79", "35", "49", "146"],
            run: a_lost_conversations_carried_start_that_does_not_begin,
        },
        Scenario {
            name: "carried conversations: a lost talk whose transcript cannot be read starts afresh under work-1 with flume's words before the retry's, status giving its conversation from start 1, lost, not carried, with the profile's words",
            cites: &["S115", "49", "179"],
            run: s115_a_lost_talk_whose_transcript_is_gone,
        },
        Scenario {
            name: "carried conversations: a lost talk whose account work-1 is spent starts afresh under work-2, which does not reach it, saying why by the variable's name alone, and its move from work-1 to work-2 gives not carried",
            cites: &["S114", "49", "145", "179"],
            run: s114_a_lost_talk_whose_account_is_spent,
        },
        Scenario {
            name: "carried conversations: a lost talk whose start never said it started starts afresh at its retry, status saying its conversation is not known",
            cites: &["35", "49", "179"],
            run: a_lost_start_that_never_acked,
        },
        Scenario {
            name: "carried conversations: a lost talk on a host with no pool resumes its session under no account where the profile answers for the login environment that it reaches it",
            cites: &["49", "179"],
            run: a_host_with_no_pool,
        },
        Scenario {
            name: "carried conversations: a lost talk's carried start is sent flume's words that its session was lost and goes on here, then the retry's words, then the way to stop short, and nothing of talk's template",
            cites: &["28", "49"],
            run: the_prompt_of_a_lost_conversations_carried_start,
        },
        Scenario {
            name: "carried conversations: a code stage lost with its host, its agent brought back bare and ended, carries nothing at its retry: its fresh agent's prompt is code's with the retry's words, and the profile is asked no carry",
            cites: &["S116", "39", "45", "49"],
            run: s116_a_lost_code_stages_retry_carries_nothing,
        },
        Scenario {
            name: "carried conversations: a talk its caller stopped carries nothing at its retry",
            cites: &["49", "51"],
            run: a_stopped_conversations_retry_carries_nothing,
        },
        Scenario {
            name: "carried conversations: a lost talk whose session a person runs by hand, process 4242, waits for room at its retry naming it, never stalled, flume opening, closing and sending nothing for the place",
            cites: &["S117", "49", "207"],
            run: s117_talks_resumed_by_hand_wait_for_their_sessions,
        },
        Scenario {
            name: "carried conversations: once the fake agent reads the lost talk's session live no more, the first settle of any place starts talk carrying its conversation",
            cites: &["S113", "49", "70", "207"],
            run: the_session_ends,
        },
        Scenario {
            name: "carried conversations: a lost talk whose session live cannot read waits, status giving what could not be read by the variable's name and no path",
            cites: &["207"],
            run: a_session_that_cannot_be_read,
        },
        Scenario {
            name: "carried conversations: a code agent stopped at work-1's limit whose session a person resumes by hand: code's carried start does not open while the session is live, and starts at the first settle after it ends",
            cites: &["30", "146", "207"],
            run: a_spent_agents_session_resumed_by_hand,
        },
        Scenario {
            name: "carried conversations: a lost talk that cannot be carried waits while its session is live, and starts afresh once it ends",
            cites: &["49", "207"],
            run: a_lost_talk_that_cannot_be_carried_its_session_still_live,
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
/// stop short as the fake agent gives it, saying it is the agent's one way
/// to ask (28).
fn carried_prompt(world: &World, identity: &Identity) -> String {
    format!(
        "Your last turn stopped at your account's usage limit. This session now goes on under an account of the pool, with its conversation as it stood: carry on from where you stopped.\n\n{}",
        flume_core::way_to_stop_short(world.agent.as_ref(), identity)
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

/// A repository's kind `research`, its one stage `talk` a conversation
/// whose prompt asks for the words a retry gives.
pub fn research() -> Kind {
    kind(
        "research",
        false,
        vec![stage(
            "talk",
            None,
            "Talk {{name}} over.\n\n{{brief}}\n\n{{#words}}{{words}}\n\n{{/words}}{{needs}}",
        )],
    )
}

/// The research place `one`'s talk start `start`.
pub fn talk(start: u32) -> Identity {
    World::identity("research", "one", "talk", start)
}

/// The research place `one` launched for the recipient `ops`, whose pane
/// waits, serving an intent of its own, its talk begun, and its talk
/// start 1's word naming the session `s-1`.
pub fn talking(world: &World) -> Result<PlaceId, String> {
    world.kinds.set_kinds(REPOSITORY, vec![research()]);
    world.terminal.add_session("ops", AgentState::Waiting);
    world.agent.set_session(&talk(1), "s-1");
    let mut given = launch("research", "one", Some("weigh the options"));
    given.intent = Some("one".into());
    given.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&given))?;
    let id = World::id("research", "one");
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Begun),
        "talk"
    );
    Ok(id)
}

/// The host restarts: the core is rebuilt over the same fake store, the
/// fake terminal holds no agent, and a settle reads talk lost.
pub fn lost_with_host(world: &World, id: &PlaceId) -> Outcome {
    world.host.restart();
    world.terminal.forget_all();
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(id, "talk"),
        Some(StageState::Lost),
        "talk once the host restarted"
    );
    Ok(())
}

/// The caller's retry of talk, unforced unless `force`, with `words`:
/// the place's status as it answers, or why it was refused.
pub fn retry_talk(
    world: &World,
    id: &PlaceId,
    words: Option<&str>,
    force: bool,
) -> Result<flume_core::PlaceStatus, String> {
    match world.engine().retry(
        id,
        Retry::Stage("talk".into()),
        words.map(str::to_string),
        force,
    ) {
        flume_core::Answer::Acted {
            status: Some(status),
            ..
        } => Ok(status),
        other => Err(format!("the retry answered {other:?}")),
    }
}

/// The process `4242`, started at 9000, as the fake agent's live names it.
pub fn by_hand() -> LiveProcess {
    LiveProcess {
        pid: 4242,
        started: 9000,
    }
}

/// The words a lost conversation's carried start's agent is sent:
/// flume's, then `words` where the retry gave any, then the way to stop
/// short as the fake agent gives it (28, 49).
fn resumed_prompt(world: &World, identity: &Identity, words: Option<&str>) -> String {
    let resumed = "Your session was lost and goes on here, with its conversation as it stood. Anything you had running then has stopped: read the place again before you carry on.";
    match words {
        Some(words) => format!(
            "{resumed}\n\n{words}\n\n{}",
            flume_core::way_to_stop_short(world.agent.as_ref(), identity)
        ),
        None => format!(
            "{resumed}\n\n{}",
            flume_core::way_to_stop_short(world.agent.as_ref(), identity)
        ),
    }
}

/// flume's words for a lost conversation's fresh start (49).
pub const LOST_WORDS: &str = "The stage's earlier agent was lost, and its conversation could not be carried, so this agent starts the stage afresh. What it committed is on the place's branch and what it left is in the place: read them and carry the stage on from there.";

/// A profile's words where a lost start's transcript cannot be read,
/// naming the variable and no path (179, S115; core.30).
const UNREADABLE: &str =
    "the lost start's transcript cannot be read under its configuration home (`CLAUDE_CONFIG_DIR`)";

/// The talk start `start` carries the conversation of talk's start 1,
/// lost, resuming `s-1` under `account` of `work`.
fn carries_lost(world: &World, start: &StartHeld, account: &str) -> Outcome {
    ensure_eq!(
        (
            start.conversation.clone(),
            start
                .account
                .clone()
                .map(|account| (account.pool, account.name))
        ),
        (
            Some(Conversation {
                from: 1,
                cause: CarryCause::Lost,
                carried: true,
                why: None,
            }),
            on("work", account)
        ),
        format!("{}'s conversation and account", start.identity)
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
    Ok(())
}

/// The talk start `start` starts afresh after talk's start 1, lost, as
/// `why` says: its prompt talk's with flume's words for a lost
/// conversation, then `words`, resuming nothing.
fn afresh_after_lost(world: &World, start: &StartHeld, why: &str, words: Option<&str>) -> Outcome {
    ensure_eq!(
        start.conversation,
        Some(Conversation {
            from: 1,
            cause: CarryCause::Lost,
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
    let given = match words {
        Some(words) => format!("{LOST_WORDS}\n\n{words}"),
        None => LOST_WORDS.to_string(),
    };
    ensure!(
        start
            .prompt
            .text
            .starts_with("Talk one over.\n\nweigh the options\n\n")
            && start.prompt.text.contains(&given),
        "the fresh start's prompt is not talk's with flume's words before the retry's: {}",
        start.prompt.text
    );
    Ok(())
}

fn s113_a_talk_lost_with_its_host_comes_back_on_its_own_account(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-1");
    let id = talking(&world)?;
    ensure_eq!(
        account_of(&world, &id, "talk"),
        on("work", "work-1"),
        "talk's first account"
    );
    lost_with_host(&world, &id)?;
    retry_talk(&world, &id, None, false)?;
    let start = start_of(&world, &id, "talk")?;
    ensure_eq!(start.n, 2, "talk's start after the retry");
    carries_lost(&world, &start, "work-1")?;
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Begun),
        "talk once its carried start said it started in s-1"
    );
    let status = world.status(&id)?;
    let agent = status.agent.ok_or("status gives no agent")?;
    ensure_eq!(
        (agent.started, agent.conversation),
        (
            Reading::Yes,
            Some(Conversation {
                from: 1,
                cause: CarryCause::Lost,
                carried: true,
                why: None,
            })
        ),
        "the agent's started ack and conversation in status"
    );
    let starts = status
        .stages
        .iter()
        .find(|stage| stage.name == "talk")
        .map(|stage| stage.starts);
    ensure_eq!(starts, Some(2), "talk's starts");
    Ok(())
}

fn a_lost_talk_waits_for_room(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-1");
    let id = talking(&world)?;
    lost_with_host(&world, &id)?;
    world.host.set_bound(Some(1));
    let other = fix_in(&world, None, "other")?;
    ensure_eq!(
        world.stage_state(&other, "code"),
        Some(StageState::Begun),
        "the other place's code"
    );
    retry_talk(&world, &id, None, false)?;
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::WaitingForRoom),
        "talk while the host is at its bound"
    );
    world
        .terminal
        .kill(&World::identity("fix", "other", "code", 1));
    world.engine().settle(None);
    let start = start_of(&world, &id, "talk")?;
    ensure_eq!(start.n, 2, "talk's start once room was made");
    carries_lost(&world, &start, "work-1")
}

fn a_lost_conversations_carried_start_that_does_not_begin(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-1");
    world.agent.carried_starts_end();
    let id = talking(&world)?;
    lost_with_host(&world, &id)?;
    let ends = world.held(&id).ok_or("no held state")?.ends.len();
    retry_talk(&world, &id, Some("we were on the second option"), false)?;
    world.settle(&id);
    let start = start_of(&world, &id, "talk")?;
    ensure_eq!(start.n, 3, "talk's start once the carried one ended");
    afresh_after_lost(
        &world,
        &start,
        ENDED_FIRST,
        Some("we were on the second option"),
    )?;
    ensure_eq!(
        uncarried(&world, &id)?,
        vec![(2, ENDED_FIRST.to_string())],
        "the carried starts that did not begin"
    );
    ensure_eq!(
        world.held(&id).ok_or("no held state")?.ends.len(),
        ends,
        "the ends recorded after the retry"
    );
    Ok(())
}

fn s115_a_lost_talk_whose_transcript_is_gone(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    world.agent.carry_to(
        &home("work-1"),
        Carry::No {
            why: UNREADABLE.into(),
        },
    );
    let id = talking(&world)?;
    lost_with_host(&world, &id)?;
    retry_talk(&world, &id, Some("pick up the second option"), false)?;
    let start = start_of(&world, &id, "talk")?;
    ensure_eq!(
        (start.n, account_of(&world, &id, "talk")),
        (2, on("work", "work-1")),
        "talk's fresh start and its account"
    );
    afresh_after_lost(
        &world,
        &start,
        UNREADABLE,
        Some("pick up the second option"),
    )?;
    let agent = world.status(&id)?.agent.ok_or("status gives no agent")?;
    ensure_eq!(
        agent.conversation,
        Some(Conversation {
            from: 1,
            cause: CarryCause::Lost,
            carried: false,
            why: Some(UNREADABLE.into()),
        }),
        "the agent's conversation in status"
    );
    Ok(())
}

fn s114_a_lost_talk_whose_account_is_spent(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    world.agent.carry_to(
        &home("work-2"),
        Carry::No {
            why: UNREACHED.into(),
        },
    );
    let id = talking(&world)?;
    lost_with_host(&world, &id)?;
    spent(&world, "work", "work-1", world.host.now() + 2 * 3600);
    retry_talk(&world, &id, Some("go on"), false)?;
    let start = start_of(&world, &id, "talk")?;
    ensure_eq!(
        (start.n, account_of(&world, &id, "talk")),
        (2, on("work", "work-2")),
        "talk's fresh start and its account"
    );
    let why = start
        .conversation
        .as_ref()
        .and_then(|conversation| conversation.why.clone())
        .unwrap_or_default();
    ensure!(
        why.contains("CLAUDE_CONFIG_DIR") && !why.contains('/'),
        "the conversation does not say why it stayed by the variable's name alone: {why}"
    );
    afresh_after_lost(&world, &start, UNREACHED, Some("go on"))?;
    let moved: Vec<(String, String, MoveWhy, Option<bool>)> = world
        .held(&id)
        .ok_or("no held state")?
        .moves
        .into_iter()
        .filter(|moved| matches!(moved.on, MoveOn::Stage { .. }))
        .map(|moved| (moved.from, moved.to, moved.why, moved.carried))
        .collect();
    ensure_eq!(
        moved,
        vec![(
            "work-1".to_string(),
            "work-2".to_string(),
            MoveWhy::Spent,
            Some(false)
        )],
        "talk's moves"
    );
    Ok(())
}

fn a_lost_start_that_never_acked(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    let unknown = "the start that stopped never said it started, so its conversation is not known";
    world.agent.carry_to(
        &home("work-1"),
        Carry::No {
            why: unknown.into(),
        },
    );
    let id = talking(&world)?;
    lost_with_host(&world, &id)?;
    retry_talk(&world, &id, None, false)?;
    let start = start_of(&world, &id, "talk")?;
    ensure_eq!(start.n, 2, "talk's fresh start");
    afresh_after_lost(&world, &start, unknown, None)
}

fn a_host_with_no_pool(_: &()) -> Outcome {
    let world = World::new();
    world.agent.carry_to(
        FakeAgent::LOGIN,
        Carry::Yes {
            session: "s-1".into(),
        },
    );
    let id = talking(&world)?;
    lost_with_host(&world, &id)?;
    retry_talk(&world, &id, None, false)?;
    let start = start_of(&world, &id, "talk")?;
    ensure_eq!(
        (
            start.n,
            start.account.clone(),
            resumed(&world, &start.identity)?
        ),
        (2, None, Some("s-1".to_string())),
        "talk's start after the retry, its account and the session it resumes"
    );
    ensure_eq!(
        world.agent.log.of("carry"),
        vec![format!(
            "carry {} in {} to no home: yes s-1",
            talk(1),
            world.store.folder(&id)
        )],
        "the carries asked of the profile"
    );
    Ok(())
}

fn the_prompt_of_a_lost_conversations_carried_start(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-1");
    let id = talking(&world)?;
    lost_with_host(&world, &id)?;
    retry_talk(&world, &id, Some("we were on the second option"), false)?;
    let due = resumed_prompt(&world, &talk(2), Some("we were on the second option"));
    ensure_eq!(
        world.prompt(&id, &talk(2)),
        Some(due.clone()),
        "the carried start's prompt"
    );
    ensure_eq!(
        world.terminal.sent_to(&talk(2)),
        vec![due],
        "what the carried start's agent was sent"
    );
    ensure!(
        world
            .terminal
            .sent_to(&talk(2))
            .iter()
            .all(|sent| !sent.contains("Talk one over") && !sent.contains("weigh the options")),
        "the carried start's agent was sent talk's template"
    );
    Ok(())
}

fn s116_a_lost_code_stages_retry_carries_nothing(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-1");
    let id = begun_fix(&world, "one")?;
    let first = code("one", 1);
    world.agent.set_session(&first, "s-1");
    world.terminal.set_state(&first, AgentState::Working);
    world.settle(&id);
    world.host.restart();
    world.terminal.forget_all();
    world.agent.bring_back_bare(&first);
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "code with its agent brought back bare"
    );
    ensure!(
        world
            .terminal
            .log
            .of("close")
            .contains(&format!("close {first}")),
        "the agent brought back bare in code's pane was not ended once quiet"
    );
    let answer = world.engine().retry(
        &id,
        Retry::Current,
        Some("start from the test".into()),
        false,
    );
    if let Some(reason) = answer.refusal() {
        return Err(format!("the retry was refused: {reason}"));
    }
    let start = start_of(&world, &id, "code")?;
    ensure_eq!(start.n, 2, "code's start after the retry");
    carries_none(&world, &start)?;
    ensure!(
        start.prompt.text.starts_with("Fix one on fix/one.")
            && start.prompt.text.contains("start from the test"),
        "the retry's prompt is not code's with its words: {}",
        start.prompt.text
    );
    ensure_eq!(
        world.agent.log.count("carry"),
        0,
        "carries asked of the profile"
    );
    Ok(())
}

fn a_stopped_conversations_retry_carries_nothing(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-1");
    let id = talking(&world)?;
    let stopped = world.engine().stop(&id, false);
    if let Some(reason) = stopped.refusal() {
        return Err(format!("the stop was refused: {reason}"));
    }
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::Stopped),
        "talk once stopped"
    );
    retry_talk(&world, &id, None, false)?;
    let start = start_of(&world, &id, "talk")?;
    ensure_eq!(start.n, 2, "talk's start after the retry");
    carries_none(&world, &start)?;
    ensure_eq!(
        world.agent.log.count("carry"),
        0,
        "carries asked of the profile"
    );
    Ok(())
}

/// The research place `one`, its talk lost with the host under work-1,
/// its session `s-1` run by hand as process 4242, retried and settled
/// twice (49, 207, S117).
fn resumed_by_hand(world: &World) -> Result<PlaceId, String> {
    fill_first_at_homes(world);
    carries_to(world, "work-1");
    let id = talking(world)?;
    lost_with_host(world, &id)?;
    world.agent.live_runs("s-1", &[by_hand()]);
    let opened = world.terminal.log.count("open");
    let closed = world.terminal.log.count("close");
    let sent = world.terminal.log.count("send");
    let status = retry_talk(world, &id, None, false)?;
    let waiting = status
        .waits_on_session
        .ok_or("the retry's answer names no process running talk's session")?;
    ensure_eq!(
        (waiting.stage.as_str(), waiting.processes),
        ("talk", vec![by_hand()]),
        "what the retry says talk waits on"
    );
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        (
            world.terminal.log.count("open"),
            world.terminal.log.count("close"),
            world.terminal.log.count("send"),
            world.terminal.sent_to(&talk(2)),
        ),
        (opened, closed, sent, vec![]),
        "the agents opened, the agents closed and the sendings to agents"
    );
    Ok(id)
}

fn s117_talks_resumed_by_hand_wait_for_their_sessions(_: &()) -> Outcome {
    let world = World::new();
    let id = resumed_by_hand(&world)?;
    let held = world.held(&id).ok_or("no held state")?;
    ensure_eq!(
        (
            held.stages[0].state,
            held.stages[0].starts,
            held.stages[0].room.as_ref().map(|room| room.live.clone())
        ),
        (StageState::WaitingForRoom, 1, Some(vec![by_hand()])),
        "talk while its session runs by hand"
    );
    let status = world.status(&id)?;
    ensure!(
        !status.stalled && status.waiting_on.is_none(),
        "the place waits on its caller or is stalled: {:?}",
        status.waiting_on
    );
    world.host.advance(3 * 3600);
    world.settle(&id);
    ensure!(
        !world.status(&id)?.stalled,
        "the place reads stalled hours after its retry"
    );
    ensure!(
        world.agent.log.of("live").len() >= 3,
        "live was not asked again at each settle: {:?}",
        world.agent.log.of("live")
    );
    Ok(())
}

fn the_session_ends(_: &()) -> Outcome {
    let world = World::new();
    let id = resumed_by_hand(&world)?;
    world.agent.live_ended("s-1");
    world.engine().settle(None);
    let start = start_of(&world, &id, "talk")?;
    ensure_eq!(start.n, 2, "talk's start once its session ended");
    carries_lost(&world, &start, "work-1")?;
    ensure!(
        world
            .held(&id)
            .is_some_and(|held| held.stages[0].room.is_none()),
        "talk's wait outlived its start"
    );
    Ok(())
}

fn a_session_that_cannot_be_read(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-1");
    let id = talking(&world)?;
    lost_with_host(&world, &id)?;
    let what = "the sessions of the configuration home `CLAUDE_CONFIG_DIR` names could not be read";
    world.agent.live_unknown("s-1", what);
    retry_talk(&world, &id, None, false)?;
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::WaitingForRoom),
        "talk while its session cannot be read"
    );
    let waiting = world
        .status(&id)?
        .waits_on_session
        .ok_or("status gives talk waiting on nothing")?;
    ensure_eq!(
        (waiting.processes, waiting.unread.clone()),
        (vec![], Some(what.to_string())),
        "what status says talk waits on"
    );
    ensure!(
        !waiting.unread.unwrap_or_default().contains('/'),
        "status names a path"
    );
    Ok(())
}

fn a_spent_agents_session_resumed_by_hand(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-2");
    let id = begun_fix(&world, "one")?;
    world.agent.set_session(&code("one", 1), "s-1");
    world.agent.live_runs("s-1", &[by_hand()]);
    stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        (
            world.stage_state(&id, "code"),
            start_of(&world, &id, "code")?.n
        ),
        (Some(StageState::WaitingForRoom), 1),
        "code while its spent agent's session runs by hand"
    );
    world.agent.live_ended("s-1");
    world.engine().settle(None);
    let start = start_of(&world, &id, "code")?;
    ensure_eq!(start.n, 2, "code's start once the session ended");
    carried(&world, &id, &start)
}

fn a_lost_talk_that_cannot_be_carried_its_session_still_live(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    world.agent.carry_to(
        &home("work-1"),
        Carry::No {
            why: UNREADABLE.into(),
        },
    );
    let id = talking(&world)?;
    lost_with_host(&world, &id)?;
    world.agent.live_runs("s-1", &[by_hand()]);
    retry_talk(&world, &id, None, false)?;
    world.settle(&id);
    ensure_eq!(
        (
            world.stage_state(&id, "talk"),
            start_of(&world, &id, "talk")?.n
        ),
        (Some(StageState::WaitingForRoom), 1),
        "talk's fresh start while its session runs"
    );
    world.agent.live_ended("s-1");
    world.engine().settle(None);
    let start = start_of(&world, &id, "talk")?;
    ensure_eq!(start.n, 2, "talk's fresh start once its session ended");
    afresh_after_lost(&world, &start, UNREADABLE, None)
}

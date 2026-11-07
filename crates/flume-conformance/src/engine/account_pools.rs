//! Pools of accounts (144, 145; core.32): a stage's pool is its stream's,
//! else the host's own; at each start of an agent with a pool an account
//! is chosen under the room lock, the strategy's over the pool's usable
//! accounts in order, usable meaning not spent, not past its threshold
//! by the host's reading, under its own bound and running the stage's
//! program; with none usable the stage waits for room, its want naming
//! the pool and the soonest time an account is back, while the pool holds
//! an account of its program, and is refused at its launch or a later
//! start where the pool holds none (4, 32, 184, 185). An intent keeps an account for each
//! program its agents run while it is usable, its affinity for that
//! program alone moving where the strategy goes when that account is
//! spent or full and never back, a document written before read as its
//! account's program's, and gone an hour after the intent's last place
//! (147). An agent stopped at its account's usage
//! limit pauses the account for every flume process until the reset its
//! record names, or for an hour, and once quiet is ended and its stage
//! started again with flume's words, carrying its conversation where the
//! account chosen reaches it and afresh otherwise, counted toward no
//! bound (146, 151, 153, 179).

use super::carried_conversations::{lost_with_host, retry_talk, talk, talking};
use super::settling::deliver_fix;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Answer, Cache, HostStatus, Retry, RoomWanted, Waits};
use flume_fakes::FakeAgent;
use flume_seams::*;
use std::collections::BTreeMap;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "account pools: a place in a stream with a pool takes its accounts, and a place in a stream with none the host's pool's",
            cites: &["144", "145"],
            run: the_streams_pool_before_the_hosts,
        },
        Scenario {
            name: "account pools: with no pool named anywhere every agent starts under no account, in the login environment alone, no variable named in its settings",
            cites: &["144", "29", "32"],
            run: no_pool_anywhere,
        },
        Scenario {
            name: "account pools: fill-first takes the first usable account in order, least-busy the one running fewest, equals in order",
            cites: &["S54", "145"],
            run: s54_fill_first_and_least_busy,
        },
        Scenario {
            name: "account pools: two settles at once with two stages waiting on an account's last room start exactly one on it",
            cites: &["145", "70", "75"],
            run: two_starts_and_one_accounts_last_room,
        },
        Scenario {
            name: "account pools: a stage whose program its pool's first account does not run starts on the account of its program after it",
            cites: &["144", "145"],
            run: an_account_for_another_program,
        },
        Scenario {
            name: "account pools: a launch whose review runs a program no account of its stream's pool runs is refused naming review, the pool and the program, and nothing is made",
            cites: &["S81", "4", "145"],
            run: a_launch_on_a_pool_of_another_programs_accounts,
        },
        Scenario {
            name: "account pools: once the host's configuration drops a pool's only account of review's program while code works, review is refused once code delivers, no agent opened and the place waiting on its caller's retry, and a retry once the pool holds it again starts review on it",
            cites: &["S81", "32", "145", "27"],
            run: s81_a_pool_with_no_account_of_the_stages_program,
        },
        Scenario {
            name: "account pools: a stage waiting for room on its program's only account, spent, on a host at its bound, is refused at the next settle once the host's configuration drops that account, and no agent is opened",
            cites: &["32", "145", "70"],
            run: a_stage_waiting_when_its_account_is_dropped,
        },
        Scenario {
            name: "account pools: a stage whose pool's accounts are all spent waits for room, status giving the pool's soonest reset",
            cites: &["145", "56", "70"],
            run: nothing_usable,
        },
        Scenario {
            name: "account pools: three places of one intent on a least-busy pool start on the first one's account, and once it is spent the next start goes to another, the affinity moving",
            cites: &["S56", "147", "145"],
            run: s56_three_units_of_one_intent,
        },
        Scenario {
            name: "account pools: three places of one intent each running code on one program and review on another keep an account for each, the later starts from the affinity with no move, and once the first program's account is spent its next start moves the intent for that program alone",
            cites: &["S80", "147", "145"],
            run: s80_two_programs_of_one_intent,
        },
        Scenario {
            name: "account pools: a start on another program while the intent's account for its first program is at its bound records no move and leaves that account as it was",
            cites: &["147", "145"],
            run: a_start_on_another_program,
        },
        Scenario {
            name: "account pools: an intent's document written before, holding one account and a move with no program, reads as that account's program's affinity, and its next change writes an account for each program and the move naming its program",
            cites: &["147", "153"],
            run: a_document_written_before,
        },
        Scenario {
            name: "account pools: an intent's document written before whose account its pool no longer declares reads with its stream alone, and the intent's next start takes the strategy's choice",
            cites: &["147", "153"],
            run: a_document_written_before_whose_account_is_gone,
        },
        Scenario {
            name: "account pools: an affinity that moved stays on its new account once the old one's reset passes, and a start the strategy sends back to the old one leaves the affinity where it is",
            cites: &["147", "145"],
            run: never_back,
        },
        Scenario {
            name: "account pools: an intent's affinity is gone an hour after its last place is tidied, and a place of the intent launched after is placed by the strategy, its stream chosen without the intent",
            cites: &["147", "143"],
            run: dropped_an_hour_after_the_last_place,
        },
        Scenario {
            name: "account pools: two places on one repository and base naming no intent share that intent and its affinity",
            cites: &["147", "145"],
            run: no_intent_named,
        },
        Scenario {
            name: "account pools: an agent's usage limit resetting at 15:00 pauses its account for another flume process's start until then, and a start after may take it again",
            cites: &["146", "151", "153", "S55"],
            run: a_reset_at_fifteen,
        },
        Scenario {
            name: "account pools: a usage limit naming no reset its profile reads pauses its account for an hour from the reading, status giving the words while it stands and none for a reset read, and a later time already recorded is kept",
            cites: &["146", "151", "153", "56"],
            run: no_reset_named,
        },
        Scenario {
            name: "account pools: once quiet, an agent stopped at its usage limit is ended and its stage starts afresh on the pool's next usable account with flume's words where that account does not reach its conversation, the move in status with the conversation that stayed and why, and the kind's bound unchanged",
            cites: &["S55", "146", "179", "31", "43", "112"],
            run: the_stage_moves,
        },
        Scenario {
            name: "account pools: once quiet, an agent stopped at its usage limit is ended and its stage starts on the pool's next usable account resuming its conversation, the move in status with its conversation and the kind's bound unchanged",
            cites: &["S78", "146", "179", "31", "112"],
            run: the_stage_moves_with_its_conversation,
        },
        Scenario {
            name: "account pools: another agent working on an account read spent works on, and moves only once its own turn fails at the limit",
            cites: &["146", "31"],
            run: another_agent_on_the_spent_account,
        },
        Scenario {
            name: "account pools: an agent under no account stopped at a usage limit is read as before, stuck with its last words, and no account is recorded spent",
            cites: &["43", "146", "151"],
            run: a_stage_under_no_account,
        },
        Scenario {
            name: "account pools: the host gives the reading set for work-1's path, the refusal's words for work-2's, and that the file is missing for work-3's",
            cites: &["184", "106"],
            run: the_hosts_reading,
        },
        Scenario {
            name: "account pools: a reading set again with a later read_at is the one the host gives when next asked, and the fake host's log shows each reading asked",
            cites: &["184", "106"],
            run: a_scripted_reading,
        },
        Scenario {
            name: "account pools: fill-first and least-busy pools whose accounts give readings under their threshold choose as with none",
            cites: &["145", "184", "S54"],
            run: readings_under_the_threshold,
        },
        Scenario {
            name: "account pools: a window past its reset reads as nothing used, and an account it would hold past its threshold takes the start",
            cites: &["184", "185"],
            run: a_window_past_its_reset,
        },
        Scenario {
            name: "account pools: an account whose reading's file is missing, is not a reading, or names a variable the login environment lacks has no reading, and a start on its pool is not refused",
            cites: &["184", "29"],
            run: a_reading_that_cannot_be_read,
        },
        Scenario {
            name: "account pools: an account's reading is read at each choice and when status is asked, and flume writes nothing of it, its file as the host wrote it and no document of the host's holding it",
            cites: &["184", "153"],
            run: nothing_written,
        },
        Scenario {
            name: "account pools: a fill-first pool's account whose reading gives its 5-hour window 92% used takes no start until 17:00, its two agents working on, the intent's affinity moving off it for the next stage, and after 17:00 a place of another intent starts on it while the intent stays where it moved",
            cites: &["S87", "31", "145", "147", "185"],
            run: s87_past_ninety_percent,
        },
        Scenario {
            name: "account pools: a reading three hours old whose 7-day window is 95% used holds its account until that window resets, the stage waiting with that time as its pool's soonest",
            cites: &["185", "145"],
            run: an_old_reading_holds,
        },
        Scenario {
            name: "account pools: a window 95% used with no reset holds its account while the reading is fresh, and no longer once it is fifteen minutes old with none newer",
            cites: &["185", "184"],
            run: a_window_with_no_reset,
        },
        Scenario {
            name: "account pools: a least-busy pool passes over an account past its threshold that runs no agent for one running two",
            cites: &["145", "185"],
            run: under_least_busy,
        },
        Scenario {
            name: "account pools: a pool's threshold of 100 holds an account at 100% and not at 99%, and one of 50 at 50% and not below",
            cites: &["185"],
            run: a_threshold_the_pool_states,
        },
        Scenario {
            name: "account pools: a most-left pool whose one account is past its threshold until Monday and whose other is spent until 15:00 though its reading gives 50% holds the stage waiting for room, not on its caller, 15:00 its pool's soonest time, a reading of 10% at 14:00 leaving the pause, and the first settle after 15:00 starts it on the spent one",
            cites: &["S88", "70", "130", "145", "146", "185"],
            run: s88_every_account_past_its_threshold_or_spent,
        },
        Scenario {
            name: "account pools: a spent account whose reading written before its reset gives it 10% used takes no start until its reset",
            cites: &["146", "185"],
            run: a_reading_under_the_threshold_while_spent,
        },
        Scenario {
            name: "account pools: a reading at 100% with no agent's record of the limit writes no spent time, and its account takes no start by its threshold",
            cites: &["146", "153", "185"],
            run: a_reading_at_its_limit,
        },
        Scenario {
            name: "account pools: a stage started again once its account is past its threshold moves to another, its move and its intent's saying the window and when it resets",
            cites: &["185", "145", "147", "56"],
            run: a_stage_moved_off_a_threshold,
        },
        Scenario {
            name: "account pools: a most-left pool's account reading 40% and 80% used two minutes ago has 20 left, taking a start before one with 19 left and after one with 21",
            cites: &["184", "145"],
            run: a_fresh_reading,
        },
        Scenario {
            name: "account pools: a most-left pool ranks an account whose reading is twenty minutes old with the accounts that have no reading, after every account with a fresh one",
            cites: &["184", "145"],
            run: a_stale_reading,
        },
        Scenario {
            name: "account pools: four places of four intents on a most-left pool start on the account with most left, twice while its reading stands, then on the one fresh reading once that one's is twenty minutes old, then on the account with no reading running fewest once the fresh one is spent",
            cites: &["S86", "145", "147", "184"],
            run: s86_four_places_of_four_intents,
        },
        Scenario {
            name: "account pools: a most-left pool whose accounts give no reading chooses as least-busy does",
            cites: &["145", "S54"],
            run: most_left_with_no_reading_anywhere,
        },
        Scenario {
            name: "account pools: an intent's next start on a most-left pool stays on its affinity's account while it is usable, though another's reading leaves more",
            cites: &["147", "145"],
            run: most_left_after_the_affinity,
        },
        Scenario {
            name: "account pools: a lost talk whose start ran under work-2 starts under work-2 from its conversation at its retry, though the intent's affinity keeps work-1, which stays, no move recorded",
            cites: &["S113", "145", "147"],
            run: the_lost_starts_account_while_the_intent_keeps_another,
        },
        Scenario {
            name: "account pools: a lost talk whose start's account work-1 is spent at its retry starts under work-2 by the strategy, its move from work-1 to work-2 saying spent and that its conversation went with it",
            cites: &["S114", "145", "146"],
            run: the_lost_starts_account_spent,
        },
        Scenario {
            name: "account pools: a lost talk whose start's account the pool no longer declares starts on the account the strategy chooses, of which the carry is asked",
            cites: &["145", "179"],
            run: the_lost_starts_account_gone_from_the_pool,
        },
        Scenario {
            name: "account pools: a lost talk retried while every account of its pool is spent waits for room, and the start that takes work-1 once its reset passes carries the conversation from work-1",
            cites: &["145", "179"],
            run: every_account_spent_at_a_lost_talks_retry,
        },
    ]
}

/// An account running the fake's program `agent`, bounded at `agents`.
pub fn account(name: &str, agents: Option<u32>) -> Account {
    Account {
        name: name.into(),
        program: "agent".into(),
        env: BTreeMap::new(),
        agents,
        usage: None,
    }
}

/// The host declares `name` with `accounts` under `strategy`.
pub fn pool(world: &World, name: &str, strategy: Strategy, accounts: Vec<Account>) {
    world.host.set_pool(
        name,
        Some(Pool {
            strategy,
            threshold: None,
            accounts,
        }),
    );
}

/// The host declares the stream `name` taking its accounts from `pool`.
pub fn stream_on(world: &World, name: &str, pool: Option<&str>, priority: i64) {
    world.host.set_stream(
        name,
        Some(Stream {
            agents: None,
            priority,
            pool: pool.map(str::to_string),
            ..Stream::default()
        }),
    );
}

/// The account `account` of `pool` recorded spent until `until`.
pub fn spent(world: &World, pool: &str, account: &str, until: Seconds) {
    world.store.put_host(
        &HostKey::Account {
            pool: pool.into(),
            account: account.into(),
        },
        HostDocument::Account(AccountSpent {
            until,
            words: Some("You've hit your session limit".into()),
            recorded_at: world.host.now(),
            unread: false,
        }),
    );
}

/// A fix named `name` launched in `stream` serving an intent of its own
/// name, settled once.
pub fn fix_in(world: &World, stream: Option<&str>, name: &str) -> Result<PlaceId, String> {
    fix_of(world, stream, name, Some(name))
}

/// A fix named `name` launched in `stream` serving `intent`, or its
/// repository and base's where none is named, settled once.
pub fn fix_of(
    world: &World,
    stream: Option<&str>,
    name: &str,
    intent: Option<&str>,
) -> Result<PlaceId, String> {
    launched(&world.engine.launch(&Launch {
        stream: stream.map(str::to_string),
        intent: intent.map(str::to_string),
        ..launch("fix", name, Some("x"))
    }))?;
    let id = World::id("fix", name);
    world.settle(&id);
    Ok(id)
}

/// The account the stage `stage` of the place `id` last started under,
/// as its pool and name.
pub fn account_of(world: &World, id: &PlaceId, stage: &str) -> Option<(String, String)> {
    world
        .held(id)?
        .stages
        .into_iter()
        .find(|held| held.name == stage)?
        .start?
        .account
        .map(|account| (account.pool, account.name))
}

pub fn on(pool: &str, account: &str) -> Option<(String, String)> {
    Some((pool.to_string(), account.to_string()))
}

/// The host's part of status.
pub fn host(world: &World) -> Result<HostStatus, String> {
    match world.engine().status(None) {
        Answer::Status { host, .. } => Ok(host),
        other => Err(format!("status answered {other:?}")),
    }
}

fn the_streams_pool_before_the_hosts(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("work-1", None)],
    );
    pool(
        &world,
        "personal",
        Strategy::FillFirst,
        vec![account("personal-1", None)],
    );
    stream_on(&world, "work", Some("work"), 0);
    world.host.set_host_pool(Some("personal"));
    let work = fix_in(&world, Some("work"), "w")?;
    let default = fix_in(&world, None, "d")?;
    ensure_eq!(
        account_of(&world, &work, "code"),
        on("work", "work-1"),
        "the work place's account"
    );
    ensure_eq!(
        account_of(&world, &default, "code"),
        on("personal", "personal-1"),
        "the default place's account"
    );
    let start = world
        .held(&work)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?;
    ensure_eq!(
        start.account.map(|account| account.from),
        Some(AccountFrom::Strategy),
        "what chose the account"
    );
    Ok(())
}

/// Three fixes started on a pool of `strategy` over two accounts, the
/// first bounded at two, and the accounts their code stages took.
fn three_starts(strategy: Strategy) -> Result<Vec<Option<(String, String)>>, String> {
    let world = World::new();
    pool(
        &world,
        "work",
        strategy,
        vec![account("work-1", Some(2)), account("work-2", None)],
    );
    world.host.set_host_pool(Some("work"));
    ["one", "two", "three"]
        .iter()
        .map(|name| {
            let id = fix_in(&world, None, name)?;
            Ok(account_of(&world, &id, "code"))
        })
        .collect()
}

fn s54_fill_first_and_least_busy(_: &()) -> Outcome {
    ensure_eq!(
        three_starts(Strategy::FillFirst)?,
        vec![
            on("work", "work-1"),
            on("work", "work-1"),
            on("work", "work-2")
        ],
        "fill-first's accounts"
    );
    ensure_eq!(
        three_starts(Strategy::LeastBusy)?,
        vec![
            on("work", "work-1"),
            on("work", "work-2"),
            on("work", "work-1")
        ],
        "least-busy's accounts"
    );
    Ok(())
}

fn two_starts_and_one_accounts_last_room(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("work-1", Some(1))],
    );
    world.host.set_host_pool(Some("work"));
    let places: Vec<PlaceId> = ["one", "two", "three"]
        .iter()
        .map(|name| fix_in(&world, None, name))
        .collect::<Result<_, _>>()?;
    for id in &places[1..] {
        ensure_eq!(
            world.stage_state(id, "code"),
            Some(StageState::WaitingForRoom),
            "{id}'s code"
        );
    }
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    std::thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| world.engine().settle(None));
        }
    });
    let running: Vec<&PlaceId> = places[1..]
        .iter()
        .filter(|id| {
            world
                .stage_state(id, "code")
                .is_some_and(StageState::running)
        })
        .collect();
    ensure_eq!(
        running.len(),
        1,
        "stages started on the account's last room"
    );
    ensure_eq!(
        account_of(&world, running[0], "code"),
        on("work", "work-1"),
        "the started stage's account"
    );
    ensure_eq!(
        world.terminal.log.count("open"),
        2,
        "agents opened, the first's among them"
    );
    Ok(())
}

fn an_account_for_another_program(_: &()) -> Outcome {
    let world = World::new();
    world.resolver.bind_agent("other", world.agent.clone());
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("work-1", None), of_other("work-2")],
    );
    world.host.set_host_pool(Some("work"));
    let mut settings = BTreeMap::new();
    settings.insert(
        "code".to_string(),
        Settings {
            program: Some("other".into()),
            ..Settings::default()
        },
    );
    launched(&world.engine.launch(&Launch {
        settings,
        ..launch("fix", "other", Some("x"))
    }))?;
    let id = World::id("fix", "other");
    world.settle(&id);
    ensure!(
        world
            .stage_state(&id, "code")
            .is_some_and(StageState::running),
        "the stage of another program did not start: {:?}",
        world.stage_state(&id, "code")
    );
    ensure_eq!(
        account_of(&world, &id, "code"),
        on("work", "work-2"),
        "the account of its program"
    );
    Ok(())
}

/// The refusal names the stage `stage`, the pool `work` and the program
/// `program`.
fn names_the_pool(refusal: &str, stage: &str, program: &str) -> Outcome {
    ensure!(
        refusal.contains(&format!("stage {stage}'s"))
            && refusal.contains("pool work")
            && refusal.contains(&format!("account of {program}")),
        "the refusal does not name {stage}, the pool and {program}: {refusal}"
    );
    Ok(())
}

fn a_launch_on_a_pool_of_another_programs_accounts(_: &()) -> Outcome {
    let world = World::new();
    two_programs(&world, None);
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("agent-1", None), account("agent-2", None)],
    );
    let reason = refused(&world.engine.launch(&Launch {
        intent: Some("the-surfaces".into()),
        ..launch("two", "a", Some("x"))
    }))?;
    names_the_pool(&reason, "review", "other")?;
    ensure_eq!(world.place.log.count("make"), 0, "places made");
    ensure!(
        world.held(&World::id("two", "a")).is_none(),
        "held state written"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    Ok(())
}

fn s81_a_pool_with_no_account_of_the_stages_program(_: &()) -> Outcome {
    let world = World::new();
    two_programs(&world, None);
    let with_other = vec![account("agent-1", None), of_other("other-1")];
    pool(&world, "work", Strategy::FillFirst, with_other.clone());
    let id = of_the_surfaces(&world, "two", "a", None)?;
    ensure_eq!(
        chosen(&world, &id, "code"),
        by("agent-1", AccountFrom::Strategy),
        "the code"
    );
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("agent-1", None)],
    );
    world.terminal.log.clear();
    world.commit("two/a", &["fix.rs"]);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "review"),
        Some(StageState::Refused),
        "the review once code delivered"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    let status = world.status(&id)?;
    let refusal = status.stages[1]
        .refused_setting
        .clone()
        .ok_or("no refusal given")?;
    names_the_pool(&refusal, "review", "other")?;
    let waiting = status.waiting_on.ok_or("the place waits on nobody")?;
    ensure_eq!(
        (waiting.act, waiting.stage.as_deref(), waiting.why),
        (Act::Retry, Some("review"), Waits::Refused),
        "the move the place waits on"
    );
    ensure_eq!(
        waiting.words.as_deref(),
        Some(refusal.as_str()),
        "the words"
    );
    ensure!(
        host(&world)?.queue.is_empty(),
        "the refused review is in the queue"
    );
    let retried = refused(
        &world
            .engine()
            .retry(&id, Retry::Stage("review".into()), None, false),
    )?;
    names_the_pool(&retried, "review", "other")?;
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    pool(&world, "work", Strategy::FillFirst, with_other);
    let answer = world
        .engine()
        .retry(&id, Retry::Stage("review".into()), None, false);
    ensure!(answer.refusal().is_none(), "the retry: {answer:?}");
    ensure_eq!(
        chosen(&world, &id, "review"),
        by("other-1", AccountFrom::Strategy),
        "the review once the pool holds other-1 again"
    );
    ensure_eq!(
        world.status(&id)?.stages[1].refused_setting,
        None,
        "the refusal after the start"
    );
    Ok(())
}

fn a_stage_waiting_when_its_account_is_dropped(_: &()) -> Outcome {
    let world = World::new();
    two_programs(&world, None);
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("agent-1", None), of_other("other-1")],
    );
    world.host.set_bound(Some(1));
    spent(&world, "work", "other-1", world.host.now() + 3600);
    let id = of_the_surfaces(&world, "two", "a", None)?;
    world.commit("two/a", &["fix.rs"]);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "review"),
        Some(StageState::WaitingForRoom),
        "the review on a spent other-1"
    );
    let busy = fix_in(&world, None, "busy")?;
    ensure!(
        world
            .stage_state(&busy, "code")
            .is_some_and(StageState::running),
        "the fix holding the host's room did not start"
    );
    let host_now = host(&world)?;
    ensure_eq!(
        (host_now.running, host_now.room),
        (1, Some(0)),
        "the host at its bound"
    );
    world.terminal.log.clear();
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("agent-1", None)],
    );
    world.engine().settle(None);
    ensure_eq!(
        world.stage_state(&id, "review"),
        Some(StageState::Refused),
        "the review once other-1 is dropped"
    );
    let refusal = world.status(&id)?.stages[1]
        .refused_setting
        .clone()
        .ok_or("no refusal given")?;
    names_the_pool(&refusal, "review", "other")?;
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    Ok(())
}

fn nothing_usable(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("work-1", None), account("work-2", None)],
    );
    world.host.set_host_pool(Some("work"));
    let now = world.host.now();
    let fifteen = now + 2 * 3600;
    spent(&world, "work", "work-1", now + 3 * 3600);
    spent(&world, "work", "work-2", fifteen);
    let id = fix_in(&world, None, "one")?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::WaitingForRoom),
        "the stage on a pool with nothing usable"
    );
    let host = host(&world)?;
    ensure_eq!(
        host.queue.first().map(|queued| queued.wants.clone()),
        Some(RoomWanted::Pool {
            pool: "work".into(),
            until: Some(fifteen)
        }),
        "what the stage waits on"
    );
    ensure!(
        world.status(&id)?.waiting_on.is_none(),
        "the stage waiting for an account waits on its caller"
    );
    world.host.advance(2 * 3600 + 1);
    world.engine().settle(None);
    ensure_eq!(
        account_of(&world, &id, "code"),
        on("work", "work-2"),
        "the account once its reset passed"
    );
    Ok(())
}

fn no_pool_anywhere(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_stream("work", Some(Stream::default()));
    let id = fix_in(&world, Some("work"), "one")?;
    let other = fix_in(&world, None, "two")?;
    for (place, name) in [(&id, "one"), (&other, "two")] {
        ensure_eq!(account_of(&world, place, "code"), None, "{name}'s account");
        let (command, _) = world
            .terminal
            .opened(&World::identity("fix", name, "code", 1))
            .ok_or("code was not opened")?;
        let given: Vec<&String> = command
            .env
            .keys()
            .filter(|variable| !variable.starts_with("AGENT_"))
            .collect();
        ensure!(given.is_empty(), "{name}'s agent was given {given:?}");
        let agent = world.status(place)?.agent.ok_or("no agent")?;
        ensure!(
            agent.settings.env.is_empty()
                && agent
                    .settings
                    .from
                    .keys()
                    .all(|key| !key.starts_with("env.")),
            "{name}'s settings name a variable: {:?}",
            agent.settings
        );
    }
    Ok(())
}

/// The intent `name`'s affinity as the host's document holds it.
pub fn affinity(world: &World, intent: &IntentKey) -> Option<Affinity> {
    match world.store.host_document(&HostKey::Intent {
        digest: intent.digest().to_string(),
    })? {
        HostDocument::Intent(affinity) => Some(affinity),
        HostDocument::Account(_) | HostDocument::Watch(_) => None,
    }
}

/// The account the intent's affinity keeps for `program`, as its pool
/// and name.
pub fn kept(world: &World, intent: &IntentKey, program: &str) -> Option<(String, String)> {
    affinity(world, intent)?
        .accounts
        .remove(program)
        .map(|kept| (kept.pool, kept.account))
}

/// The intent the place `id` serves.
fn intent_of(world: &World, id: &PlaceId) -> Result<IntentKey, String> {
    Ok(world.held(id).ok_or("no held state")?.place.intent)
}

/// A least-busy pool `work` of `work-1` and `work-2`, the host's own.
fn least_busy(world: &World) {
    pool(
        world,
        "work",
        Strategy::LeastBusy,
        vec![account("work-1", None), account("work-2", None)],
    );
    world.host.set_host_pool(Some("work"));
}

fn s56_three_units_of_one_intent(_: &()) -> Outcome {
    let world = World::new();
    least_busy(&world);
    let units: Vec<PlaceId> = ["a", "b", "c"]
        .iter()
        .map(|name| fix_of(&world, None, name, Some("the-surfaces")))
        .collect::<Result<_, _>>()?;
    for id in &units {
        ensure_eq!(
            account_of(&world, id, "code"),
            on("work", "work-1"),
            "{id}'s account, though work-2 runs fewer"
        );
    }
    let start = world
        .held(&units[1])
        .and_then(|held| held.stages[0].start.clone());
    ensure_eq!(
        start
            .and_then(|start| start.account)
            .map(|account| account.from),
        Some(AccountFrom::Affinity),
        "what chose the second unit's account"
    );
    let intent = intent_of(&world, &units[0])?;
    spent(&world, "work", "work-1", world.host.now() + 3600);
    let next = fix_of(&world, None, "d", Some("the-surfaces"))?;
    ensure_eq!(
        account_of(&world, &next, "code"),
        on("work", "work-2"),
        "the next start's account once work-1 is spent"
    );
    let moved = AccountMove {
        on: MoveOn::Intent {
            program: Some("agent".into()),
        },
        from: "work-1".into(),
        to: "work-2".into(),
        why: MoveWhy::Spent,
        at: world.host.now(),
        carried: None,
        window: None,
        until: None,
    };
    let held = affinity(&world, &intent).ok_or("no affinity")?;
    ensure_eq!(
        (kept(&world, &intent, "agent"), held.moves.clone()),
        (on("work", "work-2"), vec![moved.clone()]),
        "the affinity's account for agent and its moves"
    );
    ensure_eq!(
        world.held(&next).map(|held| held.moves),
        Some(vec![moved]),
        "the moves the place records"
    );
    Ok(())
}

fn never_back(_: &()) -> Outcome {
    let world = World::new();
    least_busy(&world);
    let first = fix_of(&world, None, "a", Some("the-surfaces"))?;
    let intent = intent_of(&world, &first)?;
    let reset = world.host.now() + 600;
    spent(&world, "work", "work-1", reset);
    fix_of(&world, None, "b", Some("the-surfaces"))?;
    world.host.advance(601);
    let after = fix_of(&world, None, "c", Some("the-surfaces"))?;
    ensure_eq!(
        account_of(&world, &after, "code"),
        on("work", "work-2"),
        "the start after work-1's reset"
    );
    spent(&world, "work", "work-2", world.host.now() + 600);
    let back = fix_of(&world, None, "d", Some("the-surfaces"))?;
    ensure_eq!(
        account_of(&world, &back, "code"),
        on("work", "work-1"),
        "the start the strategy places once work-2 is spent"
    );
    let held = affinity(&world, &intent).ok_or("no affinity")?;
    ensure_eq!(
        (kept(&world, &intent, "agent"), held.moves.len()),
        (on("work", "work-2"), 1),
        "the affinity for agent, which never moves back"
    );
    ensure_eq!(
        world.held(&back).map(|held| held.moves.len()),
        Some(0),
        "moves recorded for a start that moved no affinity"
    );
    Ok(())
}

fn dropped_an_hour_after_the_last_place(_: &()) -> Outcome {
    let world = World::new();
    least_busy(&world);
    world.host.set_stream("personal", Some(Stream::default()));
    let first = fix_of(&world, Some("personal"), "a", Some("the-surfaces"))?;
    let intent = intent_of(&world, &first)?;
    ensure_eq!(
        affinity(&world, &intent).map(|held| held.stream),
        Some("personal".to_string()),
        "the affinity's stream"
    );
    deliver_fix(&world, &first);
    world.settle(&first);
    ensure!(world.held(&first).is_none(), "the landed place is kept");
    let tidied_at = world.host.now();
    ensure_eq!(
        affinity(&world, &intent).and_then(|held| held.gone),
        Some(tidied_at),
        "when the affinity's last place went"
    );
    world.host.advance(61 * 60);
    let later = fix_of(&world, None, "b", Some("the-surfaces"))?;
    let place = world.held(&later).ok_or("no held state")?;
    ensure_eq!(
        place.place.stream,
        StreamHeld {
            name: DEFAULT_STREAM.into(),
            from: StreamFrom::Default
        },
        "the later place's stream"
    );
    let start = place.stages[0].start.clone().ok_or("no start")?;
    ensure_eq!(
        start.account.map(|account| account.from),
        Some(AccountFrom::Strategy),
        "what chose the later place's account"
    );
    let held = affinity(&world, &intent).ok_or("no affinity written again")?;
    ensure_eq!(
        (held.stream.as_str(), held.gone, held.moves.len()),
        (DEFAULT_STREAM, None, 0),
        "the affinity the later place wrote"
    );
    ensure_eq!(
        kept(&world, &intent, "agent"),
        account_of(&world, &later, "code"),
        "the affinity's account for agent the later place wrote"
    );
    Ok(())
}

fn no_intent_named(_: &()) -> Outcome {
    let world = World::new();
    least_busy(&world);
    let one = fix_of(&world, None, "one", None)?;
    let two = fix_of(&world, None, "two", None)?;
    let (first, second) = (intent_of(&world, &one)?, intent_of(&world, &two)?);
    ensure!(
        matches!(&first, IntentKey::Base { repository, base, .. } if repository == REPOSITORY && base == "main"),
        "the intent: {first:?}"
    );
    ensure_eq!(first, second, "the two places' intents");
    for id in [&one, &two] {
        ensure_eq!(
            account_of(&world, id, "code"),
            on("work", "work-1"),
            "{id}'s account"
        );
    }
    ensure_eq!(
        kept(&world, &first, "agent"),
        on("work", "work-1"),
        "the shared affinity's account for agent"
    );
    Ok(())
}

/// An account like [`account`], unbounded, running the fake's program
/// `other`.
fn of_other(name: &str) -> Account {
    Account {
        program: "other".into(),
        ..account(name, None)
    }
}

/// `other` bound to the fake agent; a fill-first pool `work` of agent-1,
/// bounded at `bound`, agent-2, other-1 and other-2, the host's own; and
/// the repository's kinds `two`, whose code runs `agent` and whose review
/// runs `other`, and `check`, whose one stage, review, runs `other`.
pub fn two_programs(world: &World, bound: Option<u32>) {
    world.resolver.bind_agent("other", world.agent.clone());
    pool(
        world,
        "work",
        Strategy::FillFirst,
        vec![
            account("agent-1", bound),
            account("agent-2", None),
            of_other("other-1"),
            of_other("other-2"),
        ],
    );
    world.host.set_host_pool(Some("work"));
    let mut review = stage("review", Some(Deliverable::Commits), "{{brief}}");
    review.agents = vec![Settings {
        program: Some("other".into()),
        ..Settings::default()
    }];
    world.kinds.set_kinds(
        REPOSITORY,
        vec![
            kind(
                "two",
                false,
                vec![
                    stage("code", Some(Deliverable::Commits), "{{brief}}"),
                    review.clone(),
                ],
            ),
            kind("check", false, vec![review]),
        ],
    );
}

/// The place `name` of the kind `kind` serving the intent the-surfaces,
/// launched with its first stage held where `held` names it, and settled
/// once.
pub fn of_the_surfaces(
    world: &World,
    kind: &str,
    name: &str,
    held: Option<&str>,
) -> Result<PlaceId, String> {
    launched(&world.engine.launch(&Launch {
        intent: Some("the-surfaces".into()),
        hold: held.into_iter().map(str::to_string).collect(),
        ..launch(kind, name, Some("x"))
    }))?;
    let id = World::id(kind, name);
    world.settle(&id);
    Ok(id)
}

/// The code of the place `name` of the kind `two` delivered, and its
/// review started.
pub fn code_delivered(world: &World, id: &PlaceId, name: &str) -> Outcome {
    world.commit(&format!("two/{name}"), &["fix.rs"]);
    world.settle(id);
    ensure!(
        world
            .stage_state(id, "review")
            .is_some_and(StageState::running),
        "{name}'s review did not start: {:?}",
        world.stage_state(id, "review")
    );
    Ok(())
}

/// The account the stage `stage` of the place `id` last started under,
/// as its name and what chose it.
fn chosen(world: &World, id: &PlaceId, stage: &str) -> Option<(String, AccountFrom)> {
    world
        .held(id)?
        .stages
        .into_iter()
        .find(|held| held.name == stage)?
        .start?
        .account
        .map(|account| (account.name, account.from))
}

fn by(account: &str, from: AccountFrom) -> Option<(String, AccountFrom)> {
    Some((account.to_string(), from))
}

fn s80_two_programs_of_one_intent(_: &()) -> Outcome {
    let world = World::new();
    two_programs(&world, None);
    let mut intent = None;
    for (n, name) in ["a", "b", "c"].into_iter().enumerate() {
        let id = of_the_surfaces(&world, "two", name, None)?;
        code_delivered(&world, &id, name)?;
        let from = match n {
            0 => AccountFrom::Strategy,
            _ => AccountFrom::Affinity,
        };
        ensure_eq!(
            (chosen(&world, &id, "code"), chosen(&world, &id, "review")),
            (by("agent-1", from), by("other-1", from)),
            "{name}'s code and review"
        );
        ensure_eq!(
            world.held(&id).map(|held| held.moves),
            Some(vec![]),
            "{name}'s moves"
        );
        intent = Some(intent_of(&world, &id)?);
    }
    let intent = intent.ok_or("no place")?;
    ensure_eq!(
        affinity(&world, &intent).map(|held| held.moves),
        Some(vec![]),
        "the affinity's moves before agent-1 is spent"
    );
    spent(&world, "work", "agent-1", world.host.now() + 3600);
    let next = of_the_surfaces(&world, "two", "d", None)?;
    ensure_eq!(
        chosen(&world, &next, "code"),
        by("agent-2", AccountFrom::Strategy),
        "the next code once agent-1 is spent"
    );
    code_delivered(&world, &next, "d")?;
    ensure_eq!(
        chosen(&world, &next, "review"),
        by("other-1", AccountFrom::Affinity),
        "the next review"
    );
    let moved = AccountMove {
        on: MoveOn::Intent {
            program: Some("agent".into()),
        },
        from: "agent-1".into(),
        to: "agent-2".into(),
        why: MoveWhy::Spent,
        at: world.host.now(),
        carried: None,
        window: None,
        until: None,
    };
    ensure_eq!(
        world.held(&next).map(|held| held.moves),
        Some(vec![moved.clone()]),
        "the moves the next place records"
    );
    let held = affinity(&world, &intent).ok_or("no affinity")?;
    ensure_eq!(
        (
            kept(&world, &intent, "agent"),
            kept(&world, &intent, "other"),
            held.moves
        ),
        (on("work", "agent-2"), on("work", "other-1"), vec![moved]),
        "the affinity's accounts and its moves"
    );
    Ok(())
}

fn a_start_on_another_program(_: &()) -> Outcome {
    let world = World::new();
    two_programs(&world, Some(1));
    let code = of_the_surfaces(&world, "two", "a", None)?;
    ensure_eq!(
        (
            chosen(&world, &code, "code"),
            world.stage_state(&code, "code")
        ),
        (
            by("agent-1", AccountFrom::Strategy),
            Some(StageState::Begun)
        ),
        "the code holding agent-1 at its bound"
    );
    let intent = intent_of(&world, &code)?;
    let review = of_the_surfaces(&world, "check", "b", None)?;
    ensure_eq!(
        chosen(&world, &review, "review"),
        by("other-1", AccountFrom::Strategy),
        "the review on other"
    );
    ensure_eq!(
        world.held(&review).map(|held| held.moves),
        Some(vec![]),
        "the moves the review's place records"
    );
    let held = affinity(&world, &intent).ok_or("no affinity")?;
    ensure_eq!(
        (
            kept(&world, &intent, "agent"),
            kept(&world, &intent, "other"),
            held.moves
        ),
        (on("work", "agent-1"), on("work", "other-1"), vec![]),
        "the affinity's accounts and its moves"
    );
    Ok(())
}

/// The intent's document as a release before an intent kept an account
/// per program wrote it: in `stream`, holding `account` of the pool
/// `work`, with `moves` (147, 153).
fn written_before(
    world: &World,
    intent: &IntentKey,
    stream: &str,
    account: &str,
    moves: Vec<AccountMove>,
) {
    world.store.put_host(
        &HostKey::Intent {
            digest: intent.digest().to_string(),
        },
        HostDocument::Intent(Affinity {
            stream: stream.into(),
            accounts: BTreeMap::new(),
            moves,
            gone: None,
            pool: Some("work".into()),
            account: Some(account.into()),
        }),
    );
}

fn a_document_written_before(_: &()) -> Outcome {
    let world = World::new();
    two_programs(&world, None);
    let id = of_the_surfaces(&world, "two", "a", Some("code"))?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::OnHold),
        "the held code"
    );
    let intent = intent_of(&world, &id)?;
    let before = AccountMove {
        on: MoveOn::Intent { program: None },
        from: "agent-1".into(),
        to: "agent-2".into(),
        why: MoveWhy::Spent,
        at: world.host.now(),
        carried: None,
        window: None,
        until: None,
    };
    written_before(
        &world,
        &intent,
        DEFAULT_STREAM,
        "agent-2",
        vec![before.clone()],
    );
    let answer = world
        .engine()
        .retry(&id, Retry::Stage("code".into()), None, false);
    ensure!(answer.refusal().is_none(), "the retry: {answer:?}");
    ensure_eq!(
        chosen(&world, &id, "code"),
        by("agent-2", AccountFrom::Affinity),
        "the code"
    );
    code_delivered(&world, &id, "a")?;
    ensure_eq!(
        chosen(&world, &id, "review"),
        by("other-1", AccountFrom::Strategy),
        "the review"
    );
    ensure_eq!(
        world.held(&id).map(|held| held.moves),
        Some(vec![]),
        "the moves the place records"
    );
    let held = affinity(&world, &intent).ok_or("no affinity")?;
    ensure_eq!(
        (
            kept(&world, &intent, "agent"),
            kept(&world, &intent, "other"),
            held.moves,
            held.pool,
            held.account
        ),
        (
            on("work", "agent-2"),
            on("work", "other-1"),
            vec![AccountMove {
                on: MoveOn::Intent {
                    program: Some("agent".into())
                },
                ..before
            }],
            None,
            None
        ),
        "the document once written again"
    );
    Ok(())
}

fn a_document_written_before_whose_account_is_gone(_: &()) -> Outcome {
    let world = World::new();
    two_programs(&world, None);
    stream_on(&world, "work", Some("work"), 0);
    let id = of_the_surfaces(&world, "two", "a", Some("code"))?;
    let intent = intent_of(&world, &id)?;
    written_before(&world, &intent, "work", "agent-9", vec![]);
    let later = of_the_surfaces(&world, "two", "b", None)?;
    let place = world.held(&later).ok_or("no held state")?;
    ensure_eq!(
        place.place.stream,
        StreamHeld {
            name: "work".into(),
            from: StreamFrom::Intent
        },
        "the later place's stream, from the affinity"
    );
    ensure_eq!(
        (chosen(&world, &later, "code"), place.moves),
        (by("agent-1", AccountFrom::Strategy), vec![]),
        "the later place's code and its moves"
    );
    let held = affinity(&world, &intent).ok_or("no affinity")?;
    ensure_eq!(
        (
            held.accounts.keys().cloned().collect::<Vec<_>>(),
            kept(&world, &intent, "agent"),
            held.pool,
            held.account
        ),
        (vec!["agent".to_string()], on("work", "agent-1"), None, None),
        "the document once written again"
    );
    Ok(())
}

/// The code agent of the fix `name`'s start `start` stops at its
/// account's usage limit, its record naming `until`, in words this test
/// writes.
pub fn stops_at_limit(world: &World, name: &str, start: u32, until: Option<Seconds>) -> Identity {
    let who = World::identity("fix", name, "code", start);
    world.agent.set_spent(
        &who,
        Spent::Yes {
            until,
            words: "a usage limit this test writes".into(),
        },
    );
    world
        .agent
        .set_last_words(&who, "a usage limit this test writes");
    who
}

/// The words status gives for the account `account` of the pool `work`,
/// where flume read no reset in them (56, 146).
fn spent_unread(world: &World, account: &str) -> Result<Option<String>, String> {
    host(world)?
        .pools
        .into_iter()
        .filter(|pool| pool.name == "work")
        .flat_map(|pool| pool.accounts)
        .find(|status| status.name == account)
        .map(|status| status.spent_unread)
        .ok_or_else(|| format!("status gives no account {account} of work"))
}

/// The account `account` of `pool`'s spent time as the host's document
/// records it.
fn spent_until(world: &World, pool: &str, account: &str) -> Option<AccountSpent> {
    match world.store.host_document(&HostKey::Account {
        pool: pool.into(),
        account: account.into(),
    })? {
        HostDocument::Account(spent) => Some(spent),
        HostDocument::Intent(_) | HostDocument::Watch(_) => None,
    }
}

/// The configuration home by which the fake agent knows the account
/// `name` (179).
pub fn home(name: &str) -> String {
    format!("/homes/{name}")
}

/// An account like [`account`], unbounded, whose environment names its
/// [`home`] (179).
pub fn account_at_home(name: &str) -> Account {
    Account {
        env: BTreeMap::from([(FakeAgent::HOME.to_string(), home(name))]),
        ..account(name, None)
    }
}

/// A fill-first pool `work` of `work-1` and `work-2`, each at its
/// [`home`], the host's own.
pub fn fill_first_at_homes(world: &World) {
    pool(
        world,
        "work",
        Strategy::FillFirst,
        vec![account_at_home("work-1"), account_at_home("work-2")],
    );
    world.host.set_host_pool(Some("work"));
}

/// The fake agent's words where an account's configuration home does not
/// reach a stopped start's conversation, as a profile gives them (179;
/// core.30).
pub const UNREACHED: &str = "the chosen account's configuration home (`CLAUDE_CONFIG_DIR`) does not reach its conversation: its `projects` folder is not the stopped account's";

/// A fill-first pool `work` of `work-1` and `work-2`, the host's own.
fn fill_first(world: &World) {
    pool(
        world,
        "work",
        Strategy::FillFirst,
        vec![account("work-1", None), account("work-2", None)],
    );
    world.host.set_host_pool(Some("work"));
}

/// The fix `name` launched with an intent of its own, its code begun.
pub fn begun_fix(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = fix_in(world, None, name)?;
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "{name}'s code"
    );
    Ok(id)
}

fn a_reset_at_fifteen(_: &()) -> Outcome {
    let world = World::new();
    fill_first(&world);
    let fifteen = world.host.now() + 2 * 3600;
    let first = begun_fix(&world, "one")?;
    stops_at_limit(&world, "one", 1, Some(fifteen));
    world.engine().settle(Some(&first));
    ensure!(
        !world.agent.log.of("spent").is_empty(),
        "the fake's log shows no spent reading"
    );
    ensure_eq!(
        spent_until(&world, "work", "work-1").map(|spent| (spent.until, spent.unread)),
        Some((fifteen, false)),
        "work-1's spent time, its reset read"
    );
    ensure_eq!(
        spent_unread(&world, "work-1")?,
        None,
        "status's words for a reset read"
    );
    // Another flume process, an engine of its own over the same store,
    // starts a stage of another place on the same pool before 15:00.
    let other = world.engine();
    launched(&other.launch(&Launch {
        intent: Some("two".into()),
        ..launch("fix", "two", Some("x"))
    }))?;
    let second = World::id("fix", "two");
    other.settle(Some(&second));
    ensure_eq!(
        account_of(&world, &second, "code"),
        on("work", "work-2"),
        "the other process's start before 15:00"
    );
    world.host.advance(2 * 3600);
    let third = fix_in(&world, None, "three")?;
    ensure_eq!(
        account_of(&world, &third, "code"),
        on("work", "work-1"),
        "a start once 15:00 passed"
    );
    Ok(())
}

fn no_reset_named(_: &()) -> Outcome {
    let world = World::new();
    fill_first(&world);
    let first = begun_fix(&world, "one")?;
    stops_at_limit(&world, "one", 1, None);
    let read_at = world.host.now();
    world.settle(&first);
    ensure_eq!(
        spent_until(&world, "work", "work-1").map(|spent| (
            spent.until,
            spent.recorded_at,
            spent.unread
        )),
        Some((read_at + 3600, read_at, true)),
        "work-1's spent time, when it was recorded, and that no reset was read"
    );
    ensure_eq!(
        spent_unread(&world, "work-1")?,
        Some("a usage limit this test writes".to_string()),
        "the words status gives for work-1's hour"
    );
    let later = world.host.now() + 5 * 3600;
    let second = World::id("fix", "one");
    let moved_to = account_of(&world, &second, "code");
    ensure_eq!(moved_to, on("work", "work-2"), "the fresh start's account");
    spent(&world, "work", "work-2", later);
    stops_at_limit(&world, "one", 2, Some(read_at + 1800));
    world.settle(&second);
    ensure_eq!(
        spent_until(&world, "work", "work-2").map(|spent| spent.until),
        Some(later),
        "work-2's later spent time, already recorded"
    );
    Ok(())
}

fn the_stage_moves(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    world.agent.carry_to(
        &home("work-2"),
        Carry::No {
            why: UNREACHED.into(),
        },
    );
    let id = begun_fix(&world, "one")?;
    let spent_agent = stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.agent.set_own_work(&spent_agent, Reading::Yes);
    world.settle(&id);
    ensure_eq!(
        world.terminal.log.count("close"),
        0,
        "agents ended while the spent one was not quiet"
    );
    ensure!(
        spent_until(&world, "work", "work-1").is_some(),
        "work-1 is not recorded spent while its agent works on"
    );
    world.agent.set_own_work(&spent_agent, Reading::No);
    let at = world.host.now();
    world.settle(&id);
    ensure_eq!(
        world.terminal.log.of("close"),
        vec![format!("close {spent_agent}")],
        "the agents ended"
    );
    let held = world.held(&id).ok_or("no held state")?;
    let start = held.stages[0].start.clone().ok_or("no start")?;
    ensure_eq!(start.n, 2, "the code stage's start");
    ensure_eq!(
        start.account.map(|account| (account.pool, account.name)),
        on("work", "work-2"),
        "the fresh start's account"
    );
    let words = start
        .words
        .clone()
        .ok_or("the fresh start carries no words")?;
    ensure!(
        words.contains("usage limit") && words.contains("place"),
        "flume's words do not say why the stage started afresh: {words}"
    );
    ensure_eq!(
        start.conversation,
        Some(Conversation {
            from: 1,
            cause: CarryCause::Spent,
            carried: false,
            why: Some(UNREACHED.into()),
        }),
        "the fresh start's conversation"
    );
    ensure_eq!(
        (
            start.start_record.session,
            world.agent.log.of("command").last().cloned()
        ),
        (None, Some(format!("command {}", start.identity))),
        "the session the fresh start resumes"
    );
    ensure_eq!(
        held.submission.send_backs,
        0,
        "the kind's bound, counted by the place's send-backs"
    );
    let status = world.status(&id)?;
    ensure!(
        status.waiting_on.is_none(),
        "the place waits on its caller: {:?}",
        status.waiting_on
    );
    let stage_move = status
        .moves
        .iter()
        .find(|moved| {
            moved.moved.on
                == MoveOn::Stage {
                    stage: "code".into(),
                }
        })
        .ok_or("status gives no move of code")?;
    ensure_eq!(
        (
            stage_move.moved.from.as_str(),
            stage_move.moved.to.as_str(),
            stage_move.moved.why,
            stage_move.moved.at,
            stage_move.cache
        ),
        ("work-1", "work-2", MoveWhy::Spent, at, Cache::Lost),
        "code's move"
    );
    ensure_eq!(
        (
            stage_move.moved.carried,
            status
                .agent
                .as_ref()
                .and_then(|agent| agent.conversation.clone())
        ),
        (
            Some(false),
            Some(Conversation {
                from: 1,
                cause: CarryCause::Spent,
                carried: false,
                why: Some(UNREACHED.into()),
            })
        ),
        "whether code's conversation went with it, and why not"
    );
    ensure!(
        world.held(&id).is_some_and(|held| held.ends.is_empty()),
        "an end recorded for the spent start, to be delivered"
    );
    Ok(())
}

fn the_stage_moves_with_its_conversation(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    world.agent.carry_to(
        &home("work-2"),
        Carry::Yes {
            session: "s-1".into(),
        },
    );
    let id = begun_fix(&world, "one")?;
    let spent_agent = stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    let at = world.host.now();
    world.settle(&id);
    ensure_eq!(
        world.terminal.log.of("close"),
        vec![format!("close {spent_agent}")],
        "the agents ended"
    );
    let held = world.held(&id).ok_or("no held state")?;
    let start = held.stages[0].start.clone().ok_or("no start")?;
    ensure_eq!(
        (
            start.n,
            start.account.map(|account| (account.pool, account.name))
        ),
        (2, on("work", "work-2")),
        "the code stage's start and its account"
    );
    ensure_eq!(
        (
            start.start_record.session.as_deref(),
            start.conversation.map(|conversation| conversation.carried)
        ),
        (Some("s-1"), Some(true)),
        "the session the start resumes and whether it carries the conversation"
    );
    ensure_eq!(
        world.agent.log.of("command").last().cloned(),
        Some(format!("command {} --resume s-1", start.identity)),
        "the start's command"
    );
    ensure_eq!(
        held.submission.send_backs,
        0,
        "the kind's bound, counted by the place's send-backs"
    );
    let status = world.status(&id)?;
    ensure!(
        status.waiting_on.is_none(),
        "the place waits on its caller: {:?}",
        status.waiting_on
    );
    let stage_move = status
        .moves
        .iter()
        .find(|moved| {
            moved.moved.on
                == MoveOn::Stage {
                    stage: "code".into(),
                }
        })
        .ok_or("status gives no move of code")?;
    ensure_eq!(
        (
            stage_move.moved.from.as_str(),
            stage_move.moved.to.as_str(),
            stage_move.moved.why,
            stage_move.moved.at,
            stage_move.moved.carried,
            stage_move.cache
        ),
        (
            "work-1",
            "work-2",
            MoveWhy::Spent,
            at,
            Some(true),
            Cache::Lost
        ),
        "code's move"
    );
    Ok(())
}

fn another_agent_on_the_spent_account(_: &()) -> Outcome {
    let world = World::new();
    fill_first(&world);
    let first = begun_fix(&world, "one")?;
    let second = begun_fix(&world, "two")?;
    ensure_eq!(
        account_of(&world, &second, "code"),
        on("work", "work-1"),
        "the second agent's account"
    );
    stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&first);
    world.settle(&second);
    let working = World::identity("fix", "two", "code", 1);
    ensure!(
        !world
            .terminal
            .log
            .of("close")
            .contains(&format!("close {working}")),
        "the second agent was ended for its account"
    );
    ensure_eq!(
        world.stage_state(&second, "code"),
        Some(StageState::Begun),
        "the second agent's stage"
    );
    stops_at_limit(&world, "two", 1, Some(world.host.now() + 3600));
    world.settle(&second);
    ensure_eq!(
        account_of(&world, &second, "code"),
        on("work", "work-2"),
        "the second agent's stage once its own turn failed at the limit"
    );
    Ok(())
}

fn a_stage_under_no_account(_: &()) -> Outcome {
    let world = World::new();
    let id = begun_fix(&world, "one")?;
    let who = stops_at_limit(&world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    world.host.advance(flume_core::QUIET_LIMIT);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Stuck),
        "the stage under no account"
    );
    let end = world
        .held(&id)
        .and_then(|held| held.stages[0].end.clone())
        .ok_or("no end")?;
    ensure_eq!(
        end.last_words.as_deref(),
        Some("a usage limit this test writes"),
        "the stuck end's last words"
    );
    ensure!(
        !world
            .terminal
            .log
            .of("close")
            .contains(&format!("close {who}")),
        "the agent under no account was ended"
    );
    ensure!(
        world.store.log.of("write_host").is_empty(),
        "an account recorded spent: {:?}",
        world.store.log.of("write_host")
    );
    Ok(())
}

/// The path of `account`'s reading under the fake host's `HOME`.
pub fn usage_path(account: &str) -> String {
    format!("/home/user/.local/state/usage/{account}.json")
}

/// A reading read at `read_at` of a 5-hour window `used` hundredths used,
/// resetting five hours after it, and a 7-day window with no reset.
fn reading(read_at: Seconds, used: u32) -> Usage {
    Usage {
        read_at,
        windows: vec![
            UsageWindow {
                name: "5-hour".into(),
                used,
                resets_at: Some(read_at + 5 * 3600),
            },
            UsageWindow {
                name: "7-day".into(),
                used: 3_000,
                resets_at: None,
            },
        ],
    }
}

/// The seam-contracts spec's fake host's reading: the reading set for a
/// path, the refusal set for another, and no reading for a path nothing
/// set, each as the core asks the host (184).
fn the_hosts_reading(_: &()) -> Outcome {
    let world = World::new();
    let read = reading(world.host.now(), 4_200);
    world
        .host
        .set_usage(&usage_path("work-1"), Ok(read.clone()));
    world.host.set_usage(
        &usage_path("work-2"),
        Err("the file is not JSON: expected value at line 1 column 1".into()),
    );
    let host: &dyn Host = world.host.as_ref();
    ensure_eq!(
        host.usage(&usage_path("work-1")),
        Ok(read),
        "work-1's reading"
    );
    ensure_eq!(
        host.usage(&usage_path("work-2")),
        Err("the file is not JSON: expected value at line 1 column 1".to_string()),
        "work-2's refusal"
    );
    ensure_eq!(
        host.usage(&usage_path("work-3")),
        Err("the file is missing".to_string()),
        "work-3, whose path nothing set"
    );
    Ok(())
}

/// The seam-fakes spec's scripted reading: a reading set again with a
/// later `read_at` is the one given at the next asking, and the fake's
/// log names each reading asked, in order (184).
fn a_scripted_reading(_: &()) -> Outcome {
    let world = World::new();
    let path = usage_path("work-1");
    let first = reading(world.host.now(), 4_200);
    world.host.set_usage(&path, Ok(first.clone()));
    let host: &dyn Host = world.host.as_ref();
    ensure_eq!(host.usage(&path), Ok(first), "the reading first set");
    world.host.advance(300);
    let later = reading(world.host.now(), 4_800);
    world.host.set_usage(&path, Ok(later.clone()));
    ensure_eq!(host.usage(&path), Ok(later), "the later reading");
    ensure_eq!(
        world.host.log.of("usage"),
        vec![format!("usage {path}"), format!("usage {path}")],
        "each reading asked"
    );
    Ok(())
}

/// An account like `account` whose reading is the file [`usage_path`]
/// names, under the fake host's `HOME` (184).
pub fn reading_at(account: Account) -> Account {
    Account {
        usage: Some(format!("~/.local/state/usage/{}.json", account.name)),
        ..account
    }
}

/// A window `name`, `used` hundredths used, resetting at `resets_at`.
pub fn window(name: &str, used: u32, resets_at: Option<Seconds>) -> UsageWindow {
    UsageWindow {
        name: name.into(),
        used,
        resets_at,
    }
}

/// The host writes `account`'s reading, read at `read_at`, of `windows`.
pub fn reads(world: &World, account: &str, read_at: Seconds, windows: Vec<UsageWindow>) {
    world
        .host
        .set_usage(&usage_path(account), Ok(Usage { read_at, windows }));
}

/// The host declares `name` with `accounts` under `strategy` and
/// `threshold`, the host's own pool.
pub fn pool_at(
    world: &World,
    name: &str,
    strategy: Strategy,
    threshold: Option<u32>,
    accounts: Vec<Account>,
) {
    world.host.set_pool(
        name,
        Some(Pool {
            strategy,
            threshold,
            accounts,
        }),
    );
    world.host.set_host_pool(Some(name));
}

/// A pool `work` of `strategy` whose accounts `names` each give a
/// reading, unbounded, the host's own.
fn read_pool(world: &World, strategy: Strategy, names: &[&str]) {
    pool_at(
        world,
        "work",
        strategy,
        None,
        names
            .iter()
            .map(|name| reading_at(account(name, None)))
            .collect(),
    );
}

/// The fix `name`'s code stage waits for room on the pool `work`, its
/// want naming `until` as the pool's soonest time, and not on its caller.
fn waits_on_the_pool(world: &World, id: &PlaceId, until: Option<Seconds>) -> Outcome {
    ensure_eq!(
        world.stage_state(id, "code"),
        Some(StageState::WaitingForRoom),
        format!("{id}'s code")
    );
    let queued = host(world)?
        .queue
        .into_iter()
        .find(|queued| &queued.place == id)
        .ok_or("the stage is not in the queue")?;
    ensure_eq!(
        queued.wants,
        RoomWanted::Pool {
            pool: "work".into(),
            until
        },
        format!("what {id}'s code waits on")
    );
    ensure!(
        world.status(id)?.waiting_on.is_none(),
        "{id} waits on its caller"
    );
    Ok(())
}

fn readings_under_the_threshold(_: &()) -> Outcome {
    for (strategy, expected) in [
        (Strategy::FillFirst, ["work-1", "work-1", "work-2"]),
        (Strategy::LeastBusy, ["work-1", "work-2", "work-1"]),
    ] {
        let world = World::new();
        let now = world.host.now();
        pool_at(
            &world,
            "work",
            strategy,
            None,
            vec![
                reading_at(account("work-1", Some(2))),
                reading_at(account("work-2", None)),
            ],
        );
        reads(&world, "work-1", now, vec![window("5-hour", 8_900, None)]);
        reads(
            &world,
            "work-2",
            now,
            vec![window("5-hour", 1_000, Some(now + 3600))],
        );
        for (name, expected) in ["one", "two", "three"].into_iter().zip(expected) {
            let id = fix_in(&world, None, name)?;
            ensure_eq!(
                account_of(&world, &id, "code"),
                on("work", expected),
                format!("{} {name}'s account", strategy.name())
            );
        }
    }
    Ok(())
}

fn a_window_past_its_reset(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    read_pool(&world, Strategy::FillFirst, &["work-1", "work-2"]);
    reads(
        &world,
        "work-1",
        now - 2 * 3600,
        vec![window("5-hour", 9_500, Some(now - 3600))],
    );
    let id = fix_in(&world, None, "one")?;
    ensure_eq!(
        account_of(&world, &id, "code"),
        on("work", "work-1"),
        "the start on work-1, its 5-hour window reset an hour ago"
    );
    Ok(())
}

fn a_reading_that_cannot_be_read(_: &()) -> Outcome {
    let not_json = "the file is not JSON: expected value at line 1 column 1";
    for (case, usage, set) in [
        ("missing", "~/.local/state/usage/work-1.json", None),
        (
            "not JSON",
            "~/.local/state/usage/work-1.json",
            Some(not_json),
        ),
        ("naming $GONE", "$GONE/work-1.json", None),
    ] {
        let world = World::new();
        pool_at(
            &world,
            "work",
            Strategy::FillFirst,
            None,
            vec![Account {
                usage: Some(usage.into()),
                ..account("work-1", None)
            }],
        );
        if let Some(words) = set {
            world
                .host
                .set_usage(&usage_path("work-1"), Err(words.into()));
        }
        let id = fix_in(&world, None, "one")?;
        ensure!(
            world
                .stage_state(&id, "code")
                .is_some_and(StageState::running),
            "the start on work-1, its reading {case}: {:?}",
            world.stage_state(&id, "code")
        );
        ensure_eq!(
            account_of(&world, &id, "code"),
            on("work", "work-1"),
            format!("the account, its reading {case}")
        );
        ensure_eq!(
            world.status(&id)?.stages[0].refused_setting,
            None,
            format!("the refusal, its reading {case}")
        );
        let asked = world.host.log.of("usage");
        match case {
            "naming $GONE" => ensure!(
                asked.is_empty(),
                "a reading asked of a path the login environment cannot give: {asked:?}"
            ),
            _ => ensure!(
                asked.contains(&format!("usage {}", usage_path("work-1"))),
                "the reading {case} was not asked: {asked:?}"
            ),
        }
    }
    Ok(())
}

fn nothing_written(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    read_pool(&world, Strategy::FillFirst, &["work-1", "work-2"]);
    let first = Usage {
        read_at: now,
        windows: vec![window("5-hour", 4_200, Some(now + 3600))],
    };
    let second = Usage {
        read_at: now,
        windows: vec![window("7-day", 9_700, Some(now + 3 * 24 * 3600))],
    };
    world
        .host
        .set_usage(&usage_path("work-1"), Ok(first.clone()));
    world
        .host
        .set_usage(&usage_path("work-2"), Ok(second.clone()));
    let id = fix_in(&world, None, "one")?;
    ensure_eq!(
        account_of(&world, &id, "code"),
        on("work", "work-1"),
        "the start"
    );
    let asked = |account: &str| {
        let line = format!("usage {}", usage_path(account));
        world
            .host
            .log
            .of("usage")
            .iter()
            .filter(|asked| **asked == line)
            .count()
    };
    let before = (asked("work-1"), asked("work-2"));
    ensure!(
        before.0 > 0 && before.1 > 0,
        "the readings were not read at the choice: {before:?}"
    );
    host(&world)?;
    ensure!(
        asked("work-1") > before.0 && asked("work-2") > before.1,
        "the readings were not read when status was asked"
    );
    let host_now: &dyn Host = world.host.as_ref();
    ensure_eq!(
        (
            host_now.usage(&usage_path("work-1")),
            host_now.usage(&usage_path("work-2"))
        ),
        (Ok(first), Ok(second)),
        "the readings as the host wrote them"
    );
    let written: Vec<String> = world
        .store
        .log
        .of("write_host")
        .into_iter()
        .filter(|line| !line.starts_with("write_host intent "))
        .collect();
    ensure!(
        written.is_empty(),
        "documents of the host's written: {written:?}"
    );
    for account in ["work-1", "work-2"] {
        ensure_eq!(
            spent_until(&world, "work", account),
            None,
            format!("{account}'s spent time")
        );
    }
    let affinity = affinity(&world, &intent_of(&world, &id)?).ok_or("no affinity")?;
    let document = serde_json::to_string(&affinity).map_err(|error| error.to_string())?;
    ensure!(
        !document.contains("read_at") && !document.contains("windows"),
        "the intent's document holds a reading: {document}"
    );
    Ok(())
}

fn s87_past_ninety_percent(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    let seventeen = now + 2 * 3600;
    pool_at(
        &world,
        "work",
        Strategy::FillFirst,
        None,
        vec![
            reading_at(account_at_home("work-1")),
            reading_at(account_at_home("work-2")),
        ],
    );
    let first = fix_of(&world, None, "a", Some("the-surfaces"))?;
    let other = fix_of(&world, None, "b", Some("another"))?;
    for id in [&first, &other] {
        ensure_eq!(
            account_of(&world, id, "code"),
            on("work", "work-1"),
            format!("{id}'s account")
        );
    }
    let intent = intent_of(&world, &first)?;
    reads(
        &world,
        "work-1",
        now,
        vec![
            window("5-hour", 9_200, Some(seventeen)),
            window("7-day", 3_000, Some(now + 3 * 24 * 3600)),
        ],
    );
    let next = fix_of(&world, None, "c", Some("the-surfaces"))?;
    ensure_eq!(
        chosen(&world, &next, "code"),
        by("work-2", AccountFrom::Strategy),
        "the intent's next stage, work-1 past its threshold"
    );
    for id in [&first, &other] {
        world.settle(id);
        ensure_eq!(
            (
                world.stage_state(id, "code"),
                account_of(&world, id, "code")
            ),
            (Some(StageState::Begun), on("work", "work-1")),
            format!("{id}'s agent on work-1")
        );
    }
    ensure_eq!(
        world.terminal.log.count("close"),
        0,
        "agents ended for work-1's threshold"
    );
    let moved = world
        .status(&next)?
        .moves
        .into_iter()
        .map(|status| status.moved)
        .find(|moved| {
            moved.on
                == MoveOn::Intent {
                    program: Some("agent".into()),
                }
        })
        .ok_or("status gives no move of the intent")?;
    ensure_eq!(
        (
            moved.from.as_str(),
            moved.to.as_str(),
            moved.why,
            moved.window.as_deref(),
            moved.until
        ),
        (
            "work-1",
            "work-2",
            MoveWhy::Threshold,
            Some("5-hour"),
            Some(seventeen)
        ),
        "the intent's move, work-1 past its threshold in 5-hour until 17:00"
    );
    ensure_eq!(
        kept(&world, &intent, "agent"),
        on("work", "work-2"),
        "the intent's affinity for agent"
    );
    world.host.advance(seventeen + 1 - world.host.now());
    let another = fix_of(&world, None, "d", Some("a-third"))?;
    ensure_eq!(
        account_of(&world, &another, "code"),
        on("work", "work-1"),
        "a place of another intent after 17:00, no newer reading written"
    );
    let stays = fix_of(&world, None, "e", Some("the-surfaces"))?;
    ensure_eq!(
        chosen(&world, &stays, "code"),
        by("work-2", AccountFrom::Affinity),
        "the intent that moved, after 17:00"
    );
    ensure_eq!(
        kept(&world, &intent, "agent"),
        on("work", "work-2"),
        "the intent's affinity for agent after 17:00"
    );
    let carried = world.agent.log.of("carry");
    ensure!(
        carried.is_empty(),
        "something was sent to an account's folders: {carried:?}"
    );
    Ok(())
}

fn an_old_reading_holds(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    let monday = now + 3 * 24 * 3600;
    read_pool(&world, Strategy::FillFirst, &["work-1"]);
    reads(
        &world,
        "work-1",
        now - 3 * 3600,
        vec![window("7-day", 9_500, Some(monday))],
    );
    let id = fix_in(&world, None, "one")?;
    waits_on_the_pool(&world, &id, Some(monday))?;
    world.host.advance(monday - 1 - world.host.now());
    world.engine().settle(None);
    waits_on_the_pool(&world, &id, Some(monday))?;
    world.host.advance(1);
    world.engine().settle(None);
    ensure_eq!(
        account_of(&world, &id, "code"),
        on("work", "work-1"),
        "the start once the 7-day window reset"
    );
    Ok(())
}

fn a_window_with_no_reset(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    read_pool(&world, Strategy::FillFirst, &["work-1"]);
    reads(&world, "work-1", now, vec![window("weekly", 9_500, None)]);
    let id = fix_in(&world, None, "one")?;
    let stale = now + flume_core::FRESH;
    waits_on_the_pool(&world, &id, Some(stale))?;
    world.host.advance(stale - 1 - world.host.now());
    world.engine().settle(None);
    waits_on_the_pool(&world, &id, Some(stale))?;
    world.host.advance(1);
    world.engine().settle(None);
    ensure_eq!(
        account_of(&world, &id, "code"),
        on("work", "work-1"),
        "the start once the reading is fifteen minutes old"
    );
    Ok(())
}

fn under_least_busy(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    read_pool(&world, Strategy::LeastBusy, &["work-1", "work-2"]);
    reads(
        &world,
        "work-1",
        now,
        vec![window("5-hour", 9_100, Some(now + 3600))],
    );
    for name in ["one", "two", "three"] {
        let id = fix_in(&world, None, name)?;
        ensure_eq!(
            account_of(&world, &id, "code"),
            on("work", "work-2"),
            format!("{name}'s account")
        );
    }
    let pool = host(&world)?
        .pools
        .into_iter()
        .find(|pool| pool.name == "work")
        .ok_or("status gives no pool work")?;
    let running: Vec<(String, u32)> = pool
        .accounts
        .into_iter()
        .map(|account| (account.name, account.running))
        .collect();
    ensure_eq!(
        running,
        vec![("work-1".to_string(), 0), ("work-2".to_string(), 3)],
        "the agents running on each account"
    );
    Ok(())
}

fn a_threshold_the_pool_states(_: &()) -> Outcome {
    for (threshold, under, at) in [(100, 9_900, 10_000), (50, 4_999, 5_000)] {
        let world = World::new();
        let now = world.host.now();
        pool_at(
            &world,
            "work",
            Strategy::FillFirst,
            Some(threshold),
            vec![
                reading_at(account("work-1", None)),
                reading_at(account("work-2", None)),
            ],
        );
        let resets = Some(now + 3600);
        reads(&world, "work-1", now, vec![window("5-hour", under, resets)]);
        let first = fix_in(&world, None, "one")?;
        ensure_eq!(
            account_of(&world, &first, "code"),
            on("work", "work-1"),
            format!(
                "the first start, work-1 {under} hundredths used under a threshold of {threshold}"
            )
        );
        reads(&world, "work-1", now, vec![window("5-hour", at, resets)]);
        let second = fix_in(&world, None, "two")?;
        ensure_eq!(
            account_of(&world, &second, "code"),
            on("work", "work-2"),
            format!(
                "the second start, work-1 {at} hundredths used under a threshold of {threshold}"
            )
        );
    }
    Ok(())
}

fn s88_every_account_past_its_threshold_or_spent(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    let fifteen = now + 2 * 3600;
    let monday = now + 3 * 24 * 3600;
    read_pool(&world, Strategy::MostLeft, &["work-1", "work-2"]);
    reads(
        &world,
        "work-1",
        now,
        vec![window("7-day", 9_500, Some(monday))],
    );
    reads(
        &world,
        "work-2",
        now,
        vec![window("5-hour", 5_000, Some(now + 4 * 3600))],
    );
    spent(&world, "work", "work-2", fifteen);
    let id = fix_in(&world, None, "one")?;
    waits_on_the_pool(&world, &id, Some(fifteen))?;
    world.host.advance(3600);
    reads(
        &world,
        "work-2",
        world.host.now(),
        vec![window("5-hour", 1_000, Some(now + 4 * 3600))],
    );
    world.engine().settle(None);
    waits_on_the_pool(&world, &id, Some(fifteen))?;
    world.host.advance(fifteen + 1 - world.host.now());
    world.engine().settle(None);
    ensure_eq!(
        account_of(&world, &id, "code"),
        on("work", "work-2"),
        "the start at the first settle after 15:00"
    );
    Ok(())
}

fn a_reading_under_the_threshold_while_spent(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    let fifteen = now + 2 * 3600;
    read_pool(&world, Strategy::FillFirst, &["work-2"]);
    spent(&world, "work", "work-2", fifteen);
    world.host.advance(3600);
    reads(
        &world,
        "work-2",
        world.host.now(),
        vec![window("5-hour", 1_000, Some(now + 4 * 3600))],
    );
    let id = fix_in(&world, None, "one")?;
    waits_on_the_pool(&world, &id, Some(fifteen))?;
    world.host.advance(fifteen - 1 - world.host.now());
    world.engine().settle(None);
    waits_on_the_pool(&world, &id, Some(fifteen))?;
    world.host.advance(1);
    world.engine().settle(None);
    ensure_eq!(
        account_of(&world, &id, "code"),
        on("work", "work-2"),
        "the start once 15:00 passed"
    );
    Ok(())
}

fn a_reading_at_its_limit(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    read_pool(&world, Strategy::FillFirst, &["work-1", "work-2"]);
    reads(
        &world,
        "work-1",
        now,
        vec![window("5-hour", 10_000, Some(now + 3600))],
    );
    let id = fix_in(&world, None, "one")?;
    ensure_eq!(
        account_of(&world, &id, "code"),
        on("work", "work-2"),
        "the start, work-1 at its limit by its reading"
    );
    ensure_eq!(
        spent_until(&world, "work", "work-1"),
        None,
        "work-1's spent time"
    );
    Ok(())
}

/// The window of a reading leaving `left` hundredths, resetting an hour
/// after `now`.
fn leaving(now: Seconds, left: u32) -> Vec<UsageWindow> {
    vec![window("5-hour", 10_000 - left, Some(now + 3600))]
}

fn a_fresh_reading(_: &()) -> Outcome {
    for (other, expected) in [(1_900, "work-1"), (2_100, "work-2")] {
        let world = World::new();
        let now = world.host.now();
        read_pool(&world, Strategy::MostLeft, &["work-2", "work-1"]);
        reads(
            &world,
            "work-1",
            now - 120,
            vec![
                window("5-hour", 4_000, Some(now + 3600)),
                window("7-day", 8_000, Some(now + 3 * 24 * 3600)),
            ],
        );
        reads(&world, "work-2", now - 120, leaving(now, other));
        let id = fix_in(&world, None, "one")?;
        ensure_eq!(
            account_of(&world, &id, "code"),
            on("work", expected),
            format!("the start, work-2 leaving {other} hundredths")
        );
    }
    Ok(())
}

fn a_stale_reading(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    pool_at(
        &world,
        "work",
        Strategy::MostLeft,
        None,
        vec![
            reading_at(account("work-1", None)),
            reading_at(account("work-2", None)),
            reading_at(account("work-3", Some(1))),
        ],
    );
    reads(&world, "work-1", now - 20 * 60, leaving(now, 9_000));
    reads(&world, "work-3", now - 120, leaving(now, 1_500));
    for (name, expected) in [("one", "work-3"), ("two", "work-1"), ("three", "work-2")] {
        let id = fix_in(&world, None, name)?;
        ensure_eq!(
            account_of(&world, &id, "code"),
            on("work", expected),
            format!("{name}'s account")
        );
    }
    Ok(())
}

fn s86_four_places_of_four_intents(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    pool_at(
        &world,
        "work",
        Strategy::MostLeft,
        None,
        vec![
            reading_at(account("work-1", None)),
            reading_at(account("work-2", None)),
            account("work-3", None),
        ],
    );
    let work_1 = |read_at| {
        reads(
            &world,
            "work-1",
            read_at,
            vec![
                window("5-hour", 4_000, Some(now + 5 * 3600)),
                window("7-day", 8_000, Some(now + 3 * 24 * 3600)),
            ],
        );
    };
    work_1(now - 120);
    reads(
        &world,
        "work-2",
        now - 120,
        vec![
            window("5-hour", 6_000, Some(now + 5 * 3600)),
            window("7-day", 3_000, Some(now + 3 * 24 * 3600)),
        ],
    );
    let first = fix_in(&world, None, "one")?;
    let second = fix_in(&world, None, "two")?;
    for (id, name) in [(&first, "the first"), (&second, "the second")] {
        ensure_eq!(
            chosen(&world, id, "code"),
            by("work-2", AccountFrom::Strategy),
            format!("{name} place's account")
        );
    }
    world.host.advance(18 * 60);
    work_1(world.host.now());
    let third = fix_in(&world, None, "three")?;
    ensure_eq!(
        account_of(&world, &third, "code"),
        on("work", "work-1"),
        "the third, work-2's reading twenty minutes old"
    );
    spent(&world, "work", "work-1", world.host.now() + 3600);
    let fourth = fix_in(&world, None, "four")?;
    ensure_eq!(
        account_of(&world, &fourth, "code"),
        on("work", "work-3"),
        "the fourth, work-1 spent"
    );
    for id in [&first, &second, &third, &fourth] {
        ensure_eq!(
            world.held(id).map(|held| held.moves),
            Some(vec![]),
            format!("{id}'s moves")
        );
    }
    Ok(())
}

fn most_left_with_no_reading_anywhere(_: &()) -> Outcome {
    let world = World::new();
    read_pool(&world, Strategy::MostLeft, &["work-1", "work-2"]);
    for (name, expected) in [("one", "work-1"), ("two", "work-2"), ("three", "work-1")] {
        let id = fix_in(&world, None, name)?;
        ensure_eq!(
            account_of(&world, &id, "code"),
            on("work", expected),
            format!("{name}'s account")
        );
    }
    Ok(())
}

fn most_left_after_the_affinity(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    read_pool(&world, Strategy::MostLeft, &["work-1", "work-2"]);
    reads(&world, "work-1", now, leaving(now, 9_000));
    reads(&world, "work-2", now, leaving(now, 5_000));
    let first = fix_of(&world, None, "a", Some("the-surfaces"))?;
    ensure_eq!(
        chosen(&world, &first, "code"),
        by("work-1", AccountFrom::Strategy),
        "the intent's first start"
    );
    reads(&world, "work-2", now, leaving(now, 9_500));
    let next = fix_of(&world, None, "b", Some("the-surfaces"))?;
    ensure_eq!(
        chosen(&world, &next, "code"),
        by("work-1", AccountFrom::Affinity),
        "the intent's next start, work-2 leaving more"
    );
    ensure_eq!(
        world.held(&next).map(|held| held.moves),
        Some(vec![]),
        "the next start's moves"
    );
    let other = fix_in(&world, None, "c")?;
    ensure_eq!(
        account_of(&world, &other, "code"),
        on("work", "work-2"),
        "a place of another intent"
    );
    Ok(())
}

fn a_stage_moved_off_a_threshold(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    let monday = now + 3 * 24 * 3600;
    read_pool(&world, Strategy::FillFirst, &["work-1", "work-2"]);
    let id = begun_fix(&world, "one")?;
    ensure_eq!(
        account_of(&world, &id, "code"),
        on("work", "work-1"),
        "the first start"
    );
    let first = World::identity("fix", "one", "code", 1);
    let secret = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?
        .secret;
    let asked = world.engine.needs(
        &first.to_string(),
        &secret.0,
        "which file?",
        StopKind::NeedsAnswer,
    );
    ensure!(asked.refusal().is_none(), "the stop-short: {asked:?}");
    reads(
        &world,
        "work-1",
        now,
        vec![
            window("5-hour", 9_300, Some(now + 3600)),
            window("7-day", 9_100, Some(monday)),
        ],
    );
    let at = world.host.now();
    // The asking agent stopped, so the retry starts a fresh agent rather
    // than answering its session (51, 198).
    ensure!(
        world.engine().stop(&id, false).refusal().is_none(),
        "the stop of the asking agent was refused"
    );
    let retried = world.engine().retry(
        &id,
        Retry::Stage("code".into()),
        Some("fix.rs".into()),
        false,
    );
    ensure!(retried.refusal().is_none(), "the retry: {retried:?}");
    ensure_eq!(
        account_of(&world, &id, "code"),
        on("work", "work-2"),
        "the start after work-1 passed its threshold"
    );
    let moves: Vec<AccountMove> = world
        .status(&id)?
        .moves
        .into_iter()
        .map(|status| status.moved)
        .collect();
    let off = |on: MoveOn| AccountMove {
        on,
        from: "work-1".into(),
        to: "work-2".into(),
        why: MoveWhy::Threshold,
        at,
        carried: None,
        window: Some("7-day".into()),
        until: Some(monday),
    };
    ensure_eq!(
        moves,
        vec![
            off(MoveOn::Stage {
                stage: "code".into()
            }),
            off(MoveOn::Intent {
                program: Some("agent".into())
            })
        ],
        "the moves, the window that holds work-1 latest named"
    );
    Ok(())
}

/// The account the talk stage of the place `id` last started under, and
/// what chose it.
fn talk_account(world: &World, id: &PlaceId) -> Option<(String, AccountFrom)> {
    chosen(world, id, "talk")
}

/// The fake agent carries a lost start's conversation to the account at
/// `name`'s home with the session `s-1`.
fn carries_to(world: &World, name: &str) {
    world.agent.carry_to(
        &home(name),
        Carry::Yes {
            session: "s-1".into(),
        },
    );
}

/// The session talk's start 2 resumes, from its record.
fn talk_resumes(world: &World, id: &PlaceId) -> Option<String> {
    world
        .held(id)?
        .stages
        .into_iter()
        .find(|stage| stage.name == "talk")?
        .start
        .filter(|start| start.n == 2)?
        .start_record
        .session
}

fn the_lost_starts_account_while_the_intent_keeps_another(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account_at_home("work-2"), account_at_home("work-1")],
    );
    world.host.set_host_pool(Some("work"));
    carries_to(&world, "work-2");
    let id = talking(&world)?;
    ensure_eq!(
        talk_account(&world, &id),
        by("work-2", AccountFrom::Strategy),
        "talk's first account"
    );
    let intent = intent_of(&world, &id)?;
    let mut kept_now = affinity(&world, &intent).ok_or("the intent keeps no affinity")?;
    kept_now.accounts.insert(
        "agent".into(),
        AffinityAccount {
            pool: "work".into(),
            account: "work-1".into(),
        },
    );
    world.store.put_host(
        &HostKey::Intent {
            digest: intent.digest().to_string(),
        },
        HostDocument::Intent(kept_now),
    );
    lost_with_host(&world, &id)?;
    retry_talk(&world, &id, None, false)?;
    ensure_eq!(
        (
            talk_account(&world, &id),
            talk_resumes(&world, &id),
            kept(&world, &intent, "agent")
        ),
        (
            by("work-2", AccountFrom::Conversation),
            Some("s-1".into()),
            on("work", "work-1")
        ),
        "talk's account after the retry, the session it resumes and the intent's affinity"
    );
    ensure_eq!(
        (
            world.held(&id).ok_or("no held state")?.moves,
            affinity(&world, &intent).map(|kept| kept.moves.len())
        ),
        (vec![], Some(0)),
        "the place's moves and the intent's"
    );
    Ok(())
}

fn the_lost_starts_account_spent(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-2");
    let id = talking(&world)?;
    lost_with_host(&world, &id)?;
    spent(&world, "work", "work-1", world.host.now() + 2 * 3600);
    retry_talk(&world, &id, None, false)?;
    ensure_eq!(
        (talk_account(&world, &id), talk_resumes(&world, &id)),
        (by("work-2", AccountFrom::Strategy), Some("s-1".into())),
        "talk's account after the retry and the session it resumes"
    );
    let moved: Vec<(String, String, MoveWhy, Option<bool>)> = world
        .held(&id)
        .ok_or("no held state")?
        .moves
        .into_iter()
        .filter(|moved| {
            moved.on
                == MoveOn::Stage {
                    stage: "talk".into(),
                }
        })
        .map(|moved| (moved.from, moved.to, moved.why, moved.carried))
        .collect();
    ensure_eq!(
        moved,
        vec![(
            "work-1".to_string(),
            "work-2".to_string(),
            MoveWhy::Spent,
            Some(true)
        )],
        "talk's moves"
    );
    Ok(())
}

fn the_lost_starts_account_gone_from_the_pool(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-2");
    let id = talking(&world)?;
    lost_with_host(&world, &id)?;
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account_at_home("work-2")],
    );
    retry_talk(&world, &id, None, false)?;
    ensure_eq!(
        (talk_account(&world, &id), talk_resumes(&world, &id)),
        (by("work-2", AccountFrom::Strategy), Some("s-1".into())),
        "talk's account after the retry and the session it resumes"
    );
    ensure_eq!(
        world.agent.log.of("carry"),
        vec![format!(
            "carry {} in {} to {}: yes s-1",
            talk(1),
            world.store.folder(&id),
            home("work-2")
        )],
        "the carries asked of the profile"
    );
    Ok(())
}

fn every_account_spent_at_a_lost_talks_retry(_: &()) -> Outcome {
    let world = World::new();
    fill_first_at_homes(&world);
    carries_to(&world, "work-1");
    let id = talking(&world)?;
    lost_with_host(&world, &id)?;
    let fifteen = world.host.now() + 2 * 3600;
    spent(&world, "work", "work-1", fifteen);
    spent(&world, "work", "work-2", fifteen + 3600);
    retry_talk(&world, &id, None, false)?;
    ensure_eq!(
        world.stage_state(&id, "talk"),
        Some(StageState::WaitingForRoom),
        "talk while every account of its pool is spent"
    );
    ensure_eq!(
        world.agent.log.count("carry"),
        0,
        "carries asked with no account chosen"
    );
    world.host.advance(2 * 3600);
    world.engine().settle(None);
    ensure_eq!(
        (talk_account(&world, &id), talk_resumes(&world, &id)),
        (by("work-1", AccountFrom::Conversation), Some("s-1".into())),
        "talk's account once work-1's reset passed and the session it resumes"
    );
    Ok(())
}

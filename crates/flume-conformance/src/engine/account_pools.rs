//! Pools of accounts (144, 145; core.32): a stage's pool is its stream's,
//! else the host's own; at each start of an agent with a pool an account
//! is chosen under the room lock, the strategy's over the pool's usable
//! accounts in order, usable meaning not spent, under its own bound and
//! running the stage's program; with none usable the stage waits for
//! room, its want naming the pool. An intent keeps to its account while
//! it is usable, its affinity moving where the strategy goes when that
//! account is spent or full and never back, and gone an hour after the
//! intent's last place (147). An agent stopped at its account's usage
//! limit pauses the account for every flume process until the reset its
//! record names, or for an hour, and once quiet is ended and its stage
//! started afresh with flume's words, counted toward no bound (146, 151,
//! 153).

use super::settling::deliver_fix;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, Cache, HostStatus, RoomWanted};
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
            name: "account pools: a stage whose program no account of its pool runs waits for room, status naming the pool and the program",
            cites: &["144", "145", "56"],
            run: an_account_for_another_program,
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
            name: "account pools: a usage limit naming no reset pauses its account for an hour from the reading, and a later time already recorded is kept",
            cites: &["146", "151", "153"],
            run: no_reset_named,
        },
        Scenario {
            name: "account pools: once quiet, an agent stopped at its usage limit is ended and its stage starts afresh on the pool's next usable account with flume's words, the move in status and the kind's bound unchanged",
            cites: &["S55", "146", "31", "43", "112"],
            run: the_stage_moves,
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
    ]
}

/// An account running the fake's program `agent`, bounded at `agents`.
pub fn account(name: &str, agents: Option<u32>) -> Account {
    Account {
        name: name.into(),
        program: "agent".into(),
        env: BTreeMap::new(),
        agents,
    }
}

/// The host declares `name` with `accounts` under `strategy`.
pub fn pool(world: &World, name: &str, strategy: Strategy, accounts: Vec<Account>) {
    world.host.set_pool(name, Some(Pool { strategy, accounts }));
}

/// The host declares the stream `name` taking its accounts from `pool`.
pub fn stream_on(world: &World, name: &str, pool: Option<&str>, priority: i64) {
    world.host.set_stream(
        name,
        Some(Stream {
            agents: None,
            priority,
            pool: pool.map(str::to_string),
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

fn on(pool: &str, account: &str) -> Option<(String, String)> {
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
        vec![account("work-1", None)],
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
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::WaitingForRoom),
        "the stage of another program"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    let host = host(&world)?;
    ensure_eq!(
        host.queue.first().map(|queued| queued.wants.clone()),
        Some(RoomWanted::Program {
            pool: "work".into(),
            program: "other".into()
        }),
        "what the stage waits on"
    );
    let words = host.queue[0].wants.words();
    ensure!(
        words.contains("work") && words.contains("other"),
        "the want's words do not name the pool and the program: {words}"
    );
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
        HostDocument::Account(_) => None,
    }
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
        on: MoveOn::Intent,
        from: "work-1".into(),
        to: "work-2".into(),
        why: MoveWhy::Spent,
        at: world.host.now(),
    };
    let held = affinity(&world, &intent).ok_or("no affinity")?;
    ensure_eq!(
        (held.account.as_str(), held.moves.clone()),
        ("work-2", vec![moved.clone()]),
        "the affinity's account and its moves"
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
        (held.account.as_str(), held.moves.len()),
        ("work-2", 1),
        "the affinity, which never moves back"
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
    Ok(())
}

/// The code agent of the fix `name`'s start `start` stops at its
/// account's usage limit, its record naming `until`, in words this test
/// writes.
fn stops_at_limit(world: &World, name: &str, start: u32, until: Option<Seconds>) -> Identity {
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

/// The account `account` of `pool`'s spent time as the host's document
/// records it.
fn spent_until(world: &World, pool: &str, account: &str) -> Option<AccountSpent> {
    match world.store.host_document(&HostKey::Account {
        pool: pool.into(),
        account: account.into(),
    })? {
        HostDocument::Account(spent) => Some(spent),
        HostDocument::Intent(_) => None,
    }
}

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
fn begun_fix(world: &World, name: &str) -> Result<PlaceId, String> {
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
        spent_until(&world, "work", "work-1").map(|spent| spent.until),
        Some(fifteen),
        "work-1's spent time"
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
        spent_until(&world, "work", "work-1").map(|spent| (spent.until, spent.recorded_at)),
        Some((read_at + 3600, read_at)),
        "work-1's spent time and when it was recorded"
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
    fill_first(&world);
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
    let words = start.words.ok_or("the fresh start carries no words")?;
    ensure!(
        words.contains("usage limit") && words.contains("place"),
        "flume's words do not say why the stage started afresh: {words}"
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
    ensure!(
        world.held(&id).is_some_and(|held| held.ends.is_empty()),
        "an end recorded for the spent start, to be delivered"
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

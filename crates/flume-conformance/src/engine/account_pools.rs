//! Pools of accounts (144, 145; core.32): a stage's pool is its stream's,
//! else the host's own; at each start of an agent with a pool an account
//! is chosen under the room lock, the strategy's over the pool's usable
//! accounts in order, usable meaning not spent, under its own bound and
//! running the stage's program; with none usable the stage waits for
//! room, its want naming the pool.

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Answer, HostStatus, RoomWanted};
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

/// A fix named `name` launched in `stream`, settled once.
pub fn fix_in(world: &World, stream: Option<&str>, name: &str) -> Result<PlaceId, String> {
    launched(&world.engine.launch(&Launch {
        stream: stream.map(str::to_string),
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

//! The host's queue (70, 75): a stage starts only while the agents
//! running on the host, every caller's and every stream's counted
//! together, are under the bound its configuration states, its stream's
//! are under the stream's (143), and an account of its pool is usable
//! (144, 145), and otherwise waits for room.
//! Every settle starts the stages waiting, the streams of higher priority
//! first, within them the places of higher priority, and equals in the
//! order they came to wait, as many as there is room for, passing over a
//! stage whose stream has no room, or whose pool has no usable account,
//! without holding the host's room for it,
//! and ending no agent for one (31, 46; core.26). A place's priority is
//! its caller's, given at the launch and changed after (7), and a
//! stream's the host's; a host that states no bound, in a stream that
//! states none, has no queue.

use crate::accounts::{AccountRoom, PoolRoom, Usable};
use crate::{Act, Answer, Engine, HostStatus, Queued, RoomWanted, Step, StreamStatus};
use flume_seams::*;
use std::collections::BTreeMap;

/// How a claim admits a start, with the queue as the room lock found it.
#[derive(Clone, Copy)]
pub(crate) enum Admit<'a> {
    /// A stage due to start: it waits unless the host, its stream and its
    /// pool have room and no stage waiting that ranks ahead of it could
    /// start now.
    Queue(&'a Queue),
    /// A stage the queue admits from waiting for room.
    Waiting(&'a Queue),
}

impl<'a> Admit<'a> {
    pub fn queue(self) -> &'a Queue {
        match self {
            Admit::Queue(queue) | Admit::Waiting(queue) => queue,
        }
    }
}

/// The host's agents running, its bound, each stream's, each stream's
/// pool and each pool's accounts, and the stages waiting for room, in the
/// order they would start, read under the room lock.
pub(crate) struct Queue {
    pub running: u32,
    pub bound: Option<u32>,
    pub streams: BTreeMap<String, StreamRoom>,
    /// Each stream's pool, the stream's own or the host's, none where
    /// neither names one (144).
    pub stream_pools: BTreeMap<String, Option<String>>,
    /// Each pool a stream takes its accounts from, by name, where the
    /// host's configuration declares it (144).
    pub pools: BTreeMap<String, PoolRoom>,
    pub waiting: Vec<Waiting>,
}

/// A stream's agents running, its bound and its priority among streams
/// (143); a stream a live place runs in that the host's configuration no
/// longer declares states no bound and ranks at zero.
#[derive(Clone, Copy, Default)]
pub(crate) struct StreamRoom {
    pub running: u32,
    pub bound: Option<u32>,
    pub priority: i64,
}

impl StreamRoom {
    /// The stream's agents that may yet start; none where it states no
    /// bound.
    pub fn room(&self) -> Option<u32> {
        self.bound.map(|bound| bound.saturating_sub(self.running))
    }
}

/// A stage waiting for room.
pub(crate) struct Waiting {
    pub place: PlaceId,
    pub index: usize,
    pub stage: String,
    pub stream: String,
    pub stream_priority: i64,
    /// The program its agent resolves to, by which its pool's accounts
    /// are usable (144).
    pub program: Option<String>,
    pub priority: i64,
    pub order: u64,
    pub since: Option<Seconds>,
    pub words: Option<String>,
}

impl Waiting {
    /// Its rank: its stream's priority, then its place's (70, 143).
    fn rank(&self) -> (i64, i64) {
        (self.stream_priority, self.priority)
    }
}

impl Queue {
    /// The agents that may yet start on the host; none where it states no
    /// bound.
    pub fn room(&self) -> Option<u32> {
        self.bound.map(|bound| bound.saturating_sub(self.running))
    }

    /// The stream `name` as the queue reads it.
    pub fn stream(&self, name: &str) -> StreamRoom {
        self.streams.get(name).copied().unwrap_or_default()
    }

    /// The pool a stage of `stream` takes its account from, none where
    /// neither the stream nor the host names one (144).
    pub fn pool_of(&self, stream: &str) -> Option<&str> {
        self.stream_pools.get(stream)?.as_deref()
    }

    /// The account `name` of the pool `pool`, as the queue reads it.
    pub fn account(&self, pool: &str, name: &str) -> Option<&AccountRoom> {
        self.pools.get(pool)?.account(name)
    }

    /// One more of flume's agents running on the account `name` of
    /// `pool`, as a start the queue admits takes it.
    pub fn take_account(&mut self, pool: &str, name: &str) {
        if let Some(room) = self
            .pools
            .get_mut(pool)
            .and_then(|room| room.account_mut(name))
        {
            room.running = room.running.saturating_add(1);
        }
    }

    /// The room a stage of `stream` whose agent runs `program` is short
    /// of now, none where the host and the stream have room and an
    /// account of its pool is usable: its stream's bound first, then its
    /// pool's accounts, then the host's (70, 143, 145).
    pub fn wants(&self, stream: &str, program: Option<&str>) -> Option<RoomWanted> {
        let of = self.stream(stream);
        if let (Some(0), Some(bound)) = (of.room(), of.bound) {
            return Some(RoomWanted::Stream {
                stream: stream.to_string(),
                bound,
            });
        }
        if let (Some(pool), Some(program)) = (self.pool_of(stream), program) {
            match self.pools.get(pool).map(|room| room.usable(program)) {
                Some(Usable::NoProgram) => {
                    return Some(RoomWanted::Program {
                        pool: pool.to_string(),
                        program: program.to_string(),
                    })
                }
                Some(Usable::None(until)) => {
                    return Some(RoomWanted::Pool {
                        pool: pool.to_string(),
                        until,
                    })
                }
                Some(Usable::Some) | None => {}
            }
        }
        (self.room() == Some(0)).then_some(RoomWanted::Host)
    }

    /// Whether a stage of `place` in `stream` whose agent runs `program`
    /// at `priority` due to start waits: the host or its stream is full,
    /// its pool has no usable account, or a stage of another place waits
    /// that ranks as high, its stream's priority then its place's, and
    /// could start now (70, 143, 145; core.26).
    pub fn must_wait(
        &self,
        place: &PlaceId,
        stream: &str,
        program: Option<&str>,
        priority: i64,
    ) -> bool {
        if self.wants(stream, program).is_some() {
            return true;
        }
        let rank = (self.stream(stream).priority, priority);
        self.waiting.iter().any(|waiting| {
            &waiting.place != place
                && waiting.rank() >= rank
                && self
                    .wants(&waiting.stream, waiting.program.as_deref())
                    .is_none()
        })
    }

    /// The order a stage that comes to wait now takes.
    pub fn next_order(&self) -> u64 {
        self.waiting
            .iter()
            .map(|waiting| waiting.order)
            .max()
            .map_or(1, |order| order + 1)
    }
}

impl Engine {
    /// The host's queue as the store holds it now.
    pub(crate) fn queue(&self) -> Queue {
        let every = self.store.read_every();
        let mut streams: BTreeMap<String, StreamRoom> = self
            .host
            .streams()
            .into_iter()
            .map(|(name, stream)| {
                (
                    name,
                    StreamRoom {
                        running: 0,
                        bound: stream.agents,
                        priority: stream.priority,
                    },
                )
            })
            .collect();
        let mut running = 0u32;
        let mut on_accounts: BTreeMap<(String, String), u32> = BTreeMap::new();
        for held in every.iter().map(|read| &read.state) {
            let live: Vec<&StartHeld> = held
                .stages
                .iter()
                .filter_map(|stage| stage.start.as_ref().filter(|start| start.live))
                .collect();
            let count = u32::try_from(live.len()).unwrap_or(u32::MAX);
            running = running.saturating_add(count);
            let stream = streams.entry(held.place.stream.name.clone()).or_default();
            stream.running = stream.running.saturating_add(count);
            for account in live.iter().filter_map(|start| start.account.as_ref()) {
                let on = on_accounts
                    .entry((account.pool.clone(), account.name.clone()))
                    .or_default();
                *on = on.saturating_add(1);
            }
        }
        let stream_pools: BTreeMap<String, Option<String>> = streams
            .keys()
            .map(|stream| {
                let pool = self.host.pool_for(stream).map(|named| named.name);
                (stream.clone(), pool)
            })
            .collect();
        let now = self.host.now();
        let mut pools = BTreeMap::new();
        for name in stream_pools.values().flatten() {
            if pools.contains_key(name) {
                continue;
            }
            let Some(pool) = self.host.pool(name) else {
                continue;
            };
            let accounts = pool
                .accounts
                .into_iter()
                .map(|account| {
                    let key = HostKey::Account {
                        pool: name.clone(),
                        account: account.name.clone(),
                    };
                    let spent_until = match self.store.read_host(&key) {
                        Some(HostVersioned {
                            document: HostDocument::Account(spent),
                            ..
                        }) if spent.until > now => Some(spent.until),
                        _ => None,
                    };
                    AccountRoom {
                        running: on_accounts
                            .get(&(name.clone(), account.name.clone()))
                            .copied()
                            .unwrap_or(0),
                        account,
                        spent_until,
                    }
                })
                .collect();
            pools.insert(
                name.clone(),
                PoolRoom {
                    strategy: pool.strategy,
                    accounts,
                },
            );
        }
        let mut waiting: Vec<Waiting> = every
            .iter()
            .map(|read| &read.state)
            .filter(|held| matches!(held.phase, Phase::Live | Phase::Making { .. }))
            .flat_map(|held| {
                let stream = held.place.stream.name.clone();
                let stream_priority = streams.get(&stream).map_or(0, |of| of.priority);
                held.stages
                    .iter()
                    .enumerate()
                    .filter(|(_, stage)| stage.state == StageState::WaitingForRoom)
                    .map(move |(index, stage)| {
                        let room = stage.room.clone().unwrap_or_default();
                        let program = self
                            .resolve(&held.place, &held.place.kind.stages[index])
                            .settings
                            .program;
                        Waiting {
                            place: held.place.id.clone(),
                            index,
                            stage: stage.name.clone(),
                            stream: stream.clone(),
                            stream_priority,
                            program,
                            priority: held.priority,
                            order: room.order,
                            since: stage.since,
                            words: room.words,
                        }
                    })
            })
            .collect();
        waiting.sort_by(|a, b| {
            b.rank()
                .cmp(&a.rank())
                .then(a.order.cmp(&b.order))
                .then(a.since.cmp(&b.since))
        });
        Queue {
            running,
            bound: self.host.bound(),
            streams,
            stream_pools,
            pools,
            waiting,
        }
    }

    /// Starts the stages waiting for room on the host, in the queue's
    /// order, as many as there is room for, passing over a stage whose
    /// stream has no room or whose pool has no usable account and holding
    /// none of the host's for it, each
    /// written live under the room lock and opened after it is released;
    /// ends no agent (31, 46, 70, 143; core.26).
    pub(crate) fn start_waiting(&self) {
        let any = self.store.read_every().iter().any(|read| {
            read.state
                .stages
                .iter()
                .any(|stage| stage.state == StageState::WaitingForRoom)
        });
        if !any {
            return;
        }
        let claims: Vec<_> = {
            let _room = self.host.room();
            let mut queue = self.queue();
            let waiting = std::mem::take(&mut queue.waiting);
            let mut claims = vec![];
            for waiting in waiting {
                if queue.room() == Some(0) {
                    break;
                }
                if queue
                    .wants(&waiting.stream, waiting.program.as_deref())
                    .is_some()
                {
                    continue;
                }
                if let Ok(claim) = self.claim(
                    &waiting.place,
                    waiting.index,
                    waiting.words,
                    Admit::Waiting(&queue),
                ) {
                    queue.running = queue.running.saturating_add(1);
                    let stream = queue.streams.entry(waiting.stream).or_default();
                    stream.running = stream.running.saturating_add(1);
                    if let Some(account) = claim.account() {
                        queue.take_account(&account.pool, &account.name);
                    }
                    claims.push((waiting.place, claim));
                }
            }
            claims
        };
        for (place, claim) in claims {
            self.open(&place, claim);
        }
    }

    /// The host's part of status: the agents running, the bound, the room
    /// and the queue in the order it would start, each stage waiting with
    /// the room it is short of, and the same for each stream (56, 70,
    /// 130, 143).
    pub(crate) fn host_status(&self) -> HostStatus {
        let queue = self.queue();
        let queued: Vec<Queued> = queue
            .waiting
            .iter()
            .map(|waiting| Queued {
                place: waiting.place.clone(),
                stage: waiting.stage.clone(),
                stream: waiting.stream.clone(),
                priority: waiting.priority,
                since: waiting.since,
                wants: queue
                    .wants(&waiting.stream, waiting.program.as_deref())
                    .unwrap_or(RoomWanted::Host),
            })
            .collect();
        let mut streams: Vec<StreamStatus> = queue
            .streams
            .iter()
            .map(|(name, stream)| StreamStatus {
                name: name.clone(),
                priority: stream.priority,
                running: stream.running,
                bound: stream.bound,
                room: stream.room(),
                queue: queued
                    .iter()
                    .filter(|queued| &queued.stream == name)
                    .cloned()
                    .collect(),
            })
            .collect();
        streams.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.name.cmp(&b.name)));
        HostStatus {
            running: queue.running,
            bound: queue.bound,
            room: queue.room(),
            queue: queued,
            streams,
            terminal_kept: self.terminal.kept(),
            addresses: self.addresses(),
            watch: self.terminal.watch(),
            bases: self.bases(),
        }
    }

    /// Sets the place's priority, which orders only what has not started;
    /// refused once its work is over (7, 70). flume assigns none itself.
    pub fn priority(&self, place: &PlaceId, priority: i64) -> Answer {
        self.settle_attended();
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        if read.state.phase != Phase::Live {
            return Answer::refused(format!("the work of the place {place} is over"));
        }
        let set = self.update(place, |held| {
            if held.phase != Phase::Live {
                return Step::Keep(false);
            }
            held.priority = priority;
            Step::Write(true)
        });
        if set != Some(true) {
            return Answer::refused(format!("the work of the place {place} is over"));
        }
        self.acted(Act::Priority, place)
    }
}

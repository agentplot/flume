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
//! and ending no agent for one (31, 46; core.26). Within a stream, a
//! stage of a label running fewer agents than its reserve starts first,
//! and a stage starts only while its stream's room passes what the
//! stream's other labels keep free and do not use; a stream whose labels
//! are at fault starts no stage (180 to 182; core.46). A place's priority is
//! its caller's, given at the launch and changed after (7), and a
//! stream's the host's; a host that states no bound, in a stream that
//! states none, has no queue.

use crate::accounts::{AccountRoom, PoolRoom, Usable, THRESHOLD};
use crate::answer::and_list;
use crate::live::waits_on_session;
use crate::{
    AccountStatus, Act, Answer, Engine, HostStatus, LabelStatus, PoolStatus, Queued, Reserves,
    RoomWanted, Step, StreamStatus, TitleFrom, TitleHeld,
};
use flume_seams::*;
use std::collections::{BTreeMap, BTreeSet};

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

/// A stream's agents running, its bound, its priority among streams, the
/// session it names, its labels' shares, whether their reserves are held,
/// and why its labels refuse its starts (107, 143, 181, 182); a stream a
/// live place runs in that the host's configuration no longer declares
/// states no bound, ranks at zero, names no session and has no labels.
#[derive(Clone, Default)]
pub(crate) struct StreamRoom {
    pub running: u32,
    pub bound: Option<u32>,
    pub priority: i64,
    pub session: Option<String>,
    pub labels: BTreeMap<String, LabelRoom>,
    /// Its labels' reserves and the slots they keep free are held: it
    /// states a bound and its labels are at no fault (70, 181).
    pub held: bool,
    /// Why its labels refuse every start of the stream, none where they
    /// do not (4, 181, 182).
    pub fault: Option<String>,
    /// Why its multiplexer runs no agent, a terminal profile the resolver
    /// does not carry, which refuses every start of the stream; none
    /// where it runs them (4, 190).
    pub uncarried: Option<String>,
}

/// A label's share of its stream as the queue reads it: its reserve, the
/// slots of the stream it keeps free while it runs fewer, and its agents
/// running, each counted toward the label it started under until it ends
/// (180, 181).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct LabelRoom {
    pub reserve: u32,
    pub keep_free: u32,
    pub running: u32,
}

impl LabelRoom {
    /// The stream's slots it keeps free and does not use.
    pub fn kept(&self) -> u32 {
        self.keep_free.saturating_sub(self.running)
    }
}

impl StreamRoom {
    /// The stream `name` as the host's configuration declares it, its
    /// labels running none yet (143, 181, 182).
    fn of(name: &str, stream: Stream) -> StreamRoom {
        let fault = fault_of(name, &stream);
        StreamRoom {
            running: 0,
            held: stream.agents.is_some() && fault.is_none(),
            bound: stream.agents,
            priority: stream.priority,
            session: stream.session,
            labels: stream
                .labels
                .into_iter()
                .map(|(label, share)| {
                    (
                        label,
                        LabelRoom {
                            reserve: share.reserve,
                            keep_free: share.keep_free,
                            running: 0,
                        },
                    )
                })
                .collect(),
            fault,
            uncarried: None,
        }
    }

    /// The stream's agents that may yet start; none where it states no
    /// bound.
    pub fn room(&self) -> Option<u32> {
        self.bound.map(|bound| bound.saturating_sub(self.running))
    }

    /// Whether a stage of `label` starts before the stream's others: its
    /// reserves are held and the label runs fewer than its reserve (181).
    pub fn below_reserve(&self, label: Option<&str>) -> bool {
        self.held
            && label
                .and_then(|label| self.labels.get(label))
                .is_some_and(|room| room.running < room.reserve)
    }

    /// The stream's slots its labels other than `label` keep free and do
    /// not use, and those labels, where its reserves are held; a stage of
    /// no label counts every label's (181).
    pub fn kept_for(&self, label: Option<&str>) -> (u32, Vec<String>) {
        if !self.held {
            return (0, vec![]);
        }
        let keeping: Vec<(&String, u32)> = self
            .labels
            .iter()
            .filter(|(name, _)| Some(name.as_str()) != label)
            .map(|(name, room)| (name, room.kept()))
            .filter(|(_, kept)| *kept > 0)
            .collect();
        (
            keeping.iter().map(|(_, kept)| kept).sum(),
            keeping.into_iter().map(|(name, _)| name.clone()).collect(),
        )
    }

    /// One more agent of the stream running, under `label`.
    fn take(&mut self, label: Option<&str>) {
        self.running = self.running.saturating_add(1);
        if let Some(room) = label.and_then(|label| self.labels.get_mut(label)) {
            room.running = room.running.saturating_add(1);
        }
    }

    /// The session the agents of the stream `name` open in where their
    /// launch names none: the one it names, else the one named by its
    /// name, and, for the host's default stream naming none, the
    /// terminal's own (107, 143).
    pub fn session(&self, name: &str) -> SessionHeld {
        match (&self.session, name) {
            (Some(session), _) => SessionHeld {
                name: Some(session.clone()),
                from: SessionFrom::Stream,
            },
            (None, DEFAULT_STREAM) => SessionHeld {
                name: None,
                from: SessionFrom::Own,
            },
            (None, _) => SessionHeld {
                name: Some(name.to_string()),
                from: SessionFrom::StreamName,
            },
        }
    }
}

/// Why the labels of the stream `name` refuse its starts: a label's table
/// holding another key, then a label keeping more free than its reserve,
/// then reserves past the stream's bound; none where none of them holds
/// (4, 181, 182).
fn fault_of(name: &str, stream: &Stream) -> Option<String> {
    let other = stream.labels.iter().find_map(|(label, share)| {
        share.unknown.first().map(|key| {
            format!(
                "streams.{name}.labels.{label}.{key}: a label's table holds its reserve and \
                 what it keeps free alone; its stream's pool, points and pace are its budget"
            )
        })
    });
    let more_free = || {
        stream.labels.iter().find_map(|(label, share)| {
            (share.keep_free > share.reserve).then(|| {
                format!(
                    "the label {label} of the stream {name} keeps {} free of a reserve of {}, \
                     and a reserve is at least what it keeps free",
                    share.keep_free, share.reserve
                )
            })
        })
    };
    let past_bound = || {
        let bound = stream.agents?;
        let total = stream
            .labels
            .values()
            .fold(0u32, |total, share| total.saturating_add(share.reserve));
        (total > bound).then(|| {
            let each: Vec<String> = stream
                .labels
                .iter()
                .filter(|(_, share)| share.reserve > 0)
                .map(|(label, share)| format!("{label} {}", share.reserve))
                .collect();
            format!(
                "the stream {name}'s reserves, {}, {total} in all, pass its bound of {bound}",
                and_list(&each)
            )
        })
    };
    other.or_else(more_free).or_else(past_bound)
}

/// What places a stage among those waiting: its stream, its stream's
/// priority then its place's, the order it came to wait, since when, and
/// whether its label runs fewer than its reserve (70, 143, 181).
struct Standing<'a> {
    stream: &'a str,
    rank: (i64, i64),
    order: u64,
    since: Option<Seconds>,
    below: bool,
}

/// The positions of `standings` in the order their stages start: by
/// their streams' priority, then their places', then the order they came
/// to wait; then, within each stream, its stages in the positions its
/// stages hold, a stage whose label runs fewer than its reserve first,
/// each part in that order (70, 143, 181).
fn start_order(standings: &[Standing<'_>]) -> Vec<usize> {
    let mut order: Vec<usize> = (0..standings.len()).collect();
    order.sort_by(|&a, &b| {
        let (a, b) = (&standings[a], &standings[b]);
        b.rank
            .cmp(&a.rank)
            .then(a.order.cmp(&b.order))
            .then(a.since.cmp(&b.since))
    });
    let streams: BTreeSet<&str> = standings.iter().map(|standing| standing.stream).collect();
    for stream in streams {
        let positions: Vec<usize> = (0..order.len())
            .filter(|&at| standings[order[at]].stream == stream)
            .collect();
        let (below, rest): (Vec<usize>, Vec<usize>) = positions
            .iter()
            .map(|&at| order[at])
            .partition(|&index| standings[index].below);
        for (at, index) in positions.into_iter().zip(below.into_iter().chain(rest)) {
            order[at] = index;
        }
    }
    order
}

/// A stage waiting for room.
pub(crate) struct Waiting {
    pub place: PlaceId,
    pub index: usize,
    pub stage: String,
    pub stream: String,
    pub stream_priority: i64,
    /// The label its place resolves to as the queue reads it (180).
    pub label: Option<String>,
    /// The program its agent resolves to, by which its pool's accounts
    /// are usable (144).
    pub program: Option<String>,
    pub priority: i64,
    pub order: u64,
    pub since: Option<Seconds>,
    pub words: Option<String>,
    /// It waits while a process runs the conversation its start would
    /// carry, or flume could not read whether one does, as the last settle
    /// read it: it starts only once a claim reads none (49, 207).
    pub session: bool,
}

impl Waiting {
    /// Its rank: its stream's priority, then its place's (70, 143).
    fn rank(&self) -> (i64, i64) {
        (self.stream_priority, self.priority)
    }

    /// Whether the room its claim would refuse it for has no bearing: its
    /// pool holds an account of its program, its stream's labels are at
    /// no fault and its stream's multiplexer runs agents (4, 145, 182,
    /// 190).
    fn runs(&self, queue: &Queue) -> bool {
        let stream = queue.stream(&self.stream);
        queue.runs(&self.stream, self.program.as_deref())
            && stream.fault.is_none()
            && stream.uncarried.is_none()
    }

    /// The room it is short of now, none where it could start (70).
    fn wants(&self, queue: &Queue) -> Option<RoomWanted> {
        queue.wants(&self.stream, self.label.as_deref(), self.program.as_deref())
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
        self.streams.get(name).cloned().unwrap_or_default()
    }

    /// Where a stage of `stream` with `settings` opens: none beside the
    /// pane its settings name, else the session its launch names, else
    /// its stream's as the host's configuration stands now (32, 107,
    /// 143).
    pub fn session_of(&self, stream: &str, settings: &Settings) -> Option<SessionHeld> {
        if settings.beside.is_some() {
            return None;
        }
        Some(match &settings.session {
            Some(name) => SessionHeld {
                name: Some(name.clone()),
                from: SessionFrom::Launch,
            },
            None => self.stream(stream).session(stream),
        })
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

    /// Whether a stage of `stream` whose agent runs `program` can start on
    /// its pool as the host's configuration declares it now: the pool
    /// holds an account of that program, spent, full or neither; a stream
    /// with no pool, a pool the host does not declare, or no program, can
    /// (145).
    pub fn runs(&self, stream: &str, program: Option<&str>) -> bool {
        let pool = self.pool_of(stream).and_then(|pool| self.pools.get(pool));
        match (pool, program) {
            (Some(pool), Some(program)) => pool.runs(program),
            _ => true,
        }
    }

    /// The room a stage of `stream` and `label` whose agent runs
    /// `program` is short of now, none where the host and the stream have
    /// room and an account of its pool is usable: its stream's bound
    /// first, then its stream's room past what its other labels keep free,
    /// then its pool's accounts, then the host's (70, 143, 145, 181). A
    /// pool that holds no account of `program` gives no want: no room
    /// could start the stage, and its claim refuses it (4, 32).
    pub fn wants(
        &self,
        stream: &str,
        label: Option<&str>,
        program: Option<&str>,
    ) -> Option<RoomWanted> {
        let of = self.stream(stream);
        if let (Some(0), Some(bound)) = (of.room(), of.bound) {
            return Some(RoomWanted::Stream {
                stream: stream.to_string(),
                bound,
            });
        }
        if let (Some(room), (kept, labels)) = (of.room(), of.kept_for(label)) {
            if kept > 0 && room <= kept {
                return Some(RoomWanted::KeptFree {
                    stream: stream.to_string(),
                    labels,
                });
            }
        }
        if let (Some(pool), Some(program)) = (self.pool_of(stream), program) {
            if let Some(Usable::None(until)) = self.pools.get(pool).map(|room| room.usable(program))
            {
                return Some(RoomWanted::Pool {
                    pool: pool.to_string(),
                    until,
                });
            }
        }
        (self.room() == Some(0)).then_some(RoomWanted::Host)
    }

    /// Whether a stage of `place` in `stream` and `label` whose agent runs
    /// `program` at `priority` due to start waits: the host or its stream
    /// is full, the room its stream's other labels keep free is all it has
    /// left, its pool has no usable account, or a stage of another place
    /// waits that stands before it in the order stages start and could
    /// start now, its pool holding an account of its program and no
    /// process last read running the conversation it would carry (70, 143,
    /// 145, 181, 207; core.26).
    pub fn must_wait(
        &self,
        place: &PlaceId,
        stream: &str,
        label: Option<&str>,
        program: Option<&str>,
        priority: i64,
    ) -> bool {
        if self.wants(stream, label, program).is_some() {
            return true;
        }
        let others: Vec<&Waiting> = self
            .waiting
            .iter()
            .filter(|waiting| &waiting.place != place)
            .collect();
        let mut standings: Vec<Standing<'_>> = others
            .iter()
            .map(|waiting| self.standing(waiting))
            .collect();
        standings.push(Standing {
            stream,
            rank: (self.stream(stream).priority, priority),
            order: self.next_order(),
            since: None,
            below: self.stream(stream).below_reserve(label),
        });
        start_order(&standings)
            .into_iter()
            .take_while(|&index| index < others.len())
            .any(|index| {
                let other = others[index];
                other.runs(self) && !other.session && other.wants(self).is_none()
            })
    }

    /// Where `waiting` stands among the stages waiting (70, 181).
    fn standing<'a>(&self, waiting: &'a Waiting) -> Standing<'a> {
        Standing {
            stream: &waiting.stream,
            rank: waiting.rank(),
            order: waiting.order,
            since: waiting.since,
            below: self
                .stream(&waiting.stream)
                .below_reserve(waiting.label.as_deref()),
        }
    }

    /// `waiting` in the order its stages start, as the queue's agents
    /// running stand now (70, 143, 181).
    pub fn ordered(&self, waiting: Vec<Waiting>) -> Vec<Waiting> {
        let order = start_order(
            &waiting
                .iter()
                .map(|waiting| self.standing(waiting))
                .collect::<Vec<_>>(),
        );
        let mut slots: Vec<Option<Waiting>> = waiting.into_iter().map(Some).collect();
        order
            .into_iter()
            .filter_map(|index| slots[index].take())
            .collect()
    }

    /// One more agent running on the host and in `stream`, under `label`,
    /// as a start the queue admits takes it (70, 180).
    pub fn take(&mut self, stream: &str, label: Option<&str>) {
        self.running = self.running.saturating_add(1);
        self.streams
            .entry(stream.to_string())
            .or_default()
            .take(label);
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
    /// A place's label as it resolves now: the one the host's
    /// configuration gives its repository's kind, else its kind, else
    /// the label its kind declares, as the launch read the kind, else
    /// none (21, 32, 180).
    pub(crate) fn label_of(&self, place: &PlaceRecord) -> Option<LabelHeld> {
        self.host
            .label_for(&place.repository, &place.kind.name)
            .or_else(|| {
                place.kind.label.as_ref().map(|name| LabelHeld {
                    name: name.clone(),
                    from: LabelFrom::Kind,
                })
            })
    }

    /// The host's queue as the store holds it now, each account of each
    /// pool with its reading as the host's file holds it now (184).
    pub(crate) fn queue(&self) -> Queue {
        let every = self.store.read_every();
        let mut streams: BTreeMap<String, StreamRoom> = self
            .host
            .streams()
            .into_iter()
            .map(|(name, stream)| {
                let room = StreamRoom {
                    uncarried: self.stream_terminal(&name).err(),
                    ..StreamRoom::of(&name, stream)
                };
                (name, room)
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
            for label in live.iter().filter_map(|start| start.label.as_ref()) {
                if let Some(room) = stream.labels.get_mut(&label.name) {
                    room.running = room.running.saturating_add(1);
                }
            }
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
            let threshold = pool.threshold.unwrap_or(THRESHOLD);
            let accounts = pool
                .accounts
                .into_iter()
                .map(|account| {
                    let key = HostKey::Account {
                        pool: name.clone(),
                        account: account.name.clone(),
                    };
                    let (spent_until, spent_unread) = match self.store.read_host(&key) {
                        Some(HostVersioned {
                            document: HostDocument::Account(spent),
                            ..
                        }) if spent.until > now => {
                            (Some(spent.until), spent.words.filter(|_| spent.unread))
                        }
                        _ => (None, None),
                    };
                    let running = on_accounts
                        .get(&(name.clone(), account.name.clone()))
                        .copied()
                        .unwrap_or(0);
                    AccountRoom {
                        reading: self.reading(&account),
                        account,
                        running,
                        spent_until,
                        spent_unread,
                        threshold,
                        now,
                    }
                })
                .collect();
            pools.insert(
                name.clone(),
                PoolRoom {
                    strategy: pool.strategy,
                    threshold,
                    accounts,
                },
            );
        }
        let waiting: Vec<Waiting> = every
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
                            label: self.label_of(&held.place).map(|label| label.name),
                            program,
                            priority: held.priority,
                            order: room.order,
                            since: stage.since,
                            session: waits_on_session(&room),
                            words: room.words,
                        }
                    })
            })
            .collect();
        let mut queue = Queue {
            running,
            bound: self.host.bound(),
            streams,
            stream_pools,
            pools,
            waiting: vec![],
        };
        queue.waiting = queue.ordered(waiting);
        queue
    }

    /// Starts the stages waiting for room on the host, in the queue's
    /// order, as many as there is room for, passing over a stage whose
    /// stream has no room or whose pool has no usable account and holding
    /// none of the host's for it, each written live under the room lock
    /// and opened after it is released; ends no agent (31, 46, 70, 143;
    /// core.26). After each start it counts that agent toward the host,
    /// its stream, its label and its account and orders what is left
    /// again, so a label that reaches its reserve goes back among its
    /// stream's others (181). Ahead of the host's room, each stage waiting
    /// whose pool holds no account of its program, whose stream's labels
    /// are at fault, or whose stream's multiplexer names a terminal
    /// profile the resolver does not carry, is claimed, which refuses it,
    /// whatever room the host has (4, 32, 145, 182, 190).
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
            let (mut runs, refused): (Vec<_>, Vec<_>) = std::mem::take(&mut queue.waiting)
                .into_iter()
                .partition(|waiting| waiting.runs(&queue));
            let mut claims = vec![];
            let mut claim = |queue: &mut Queue, waiting: Waiting| {
                if let Ok(claim) = self.claim(
                    &waiting.place,
                    waiting.index,
                    waiting.words,
                    Admit::Waiting(queue),
                ) {
                    queue.take(&waiting.stream, waiting.label.as_deref());
                    if let Some(account) = claim.account() {
                        queue.take_account(&account.pool, &account.name);
                    }
                    claims.push((waiting.place, claim));
                }
            };
            for waiting in refused {
                claim(&mut queue, waiting);
            }
            while queue.room() != Some(0) {
                runs = queue.ordered(runs);
                let Some(at) = runs
                    .iter()
                    .position(|waiting| waiting.wants(&queue).is_none())
                else {
                    break;
                };
                let waiting = runs.remove(at);
                claim(&mut queue, waiting);
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
    /// 130, 143); each base's count taking the state of each agent in
    /// `agents` as status read it for its place (139).
    pub(crate) fn host_status(&self, agents: &BTreeMap<PlaceId, AgentState>) -> HostStatus {
        let queue = self.queue();
        let declared = self.host.streams();
        let queued: Vec<Queued> = queue
            .waiting
            .iter()
            .map(|waiting| Queued {
                place: waiting.place.clone(),
                stage: waiting.stage.clone(),
                stream: waiting.stream.clone(),
                label: waiting.label.clone(),
                priority: waiting.priority,
                since: waiting.since,
                wants: waiting.wants(&queue).unwrap_or(RoomWanted::Host),
            })
            .collect();
        let mut streams: Vec<StreamStatus> = queue
            .streams
            .iter()
            .map(|(name, stream)| {
                let display = self.host.display_for(name);
                let title = match declared.get(name).and_then(|stream| stream.title.clone()) {
                    Some(title) => TitleHeld {
                        title,
                        from: TitleFrom::Stream,
                    },
                    None => TitleHeld {
                        title: name.clone(),
                        from: TitleFrom::StreamName,
                    },
                };
                (name, stream, display, title)
            })
            .map(|(name, stream, display, title)| StreamStatus {
                name: name.clone(),
                session: stream.session(name),
                refused: self.stream_refused(name, stream.fault.as_ref(), &display),
                view_refused: self.view_refused(name, &display),
                multiplexer: display.multiplexer,
                view: display.view,
                terminal_app: display.terminal_app,
                title,
                priority: stream.priority,
                running: stream.running,
                bound: stream.bound,
                room: stream.room(),
                queue: queued
                    .iter()
                    .filter(|queued| &queued.stream == name)
                    .cloned()
                    .collect(),
                labels: stream
                    .labels
                    .iter()
                    .map(|(label, room)| LabelStatus {
                        name: label.clone(),
                        reserve: room.reserve,
                        keep_free: room.keep_free,
                        running: room.running,
                        waiting: queued
                            .iter()
                            .filter(|queued| {
                                &queued.stream == name && queued.label.as_ref() == Some(label)
                            })
                            .count()
                            .try_into()
                            .unwrap_or(u32::MAX),
                    })
                    .collect(),
                reserves: match (stream.labels.is_empty(), stream.bound) {
                    (true, _) => Reserves::None,
                    (false, None) => Reserves::PassedOver,
                    (false, Some(_)) => Reserves::Held,
                },
            })
            .collect();
        streams.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.name.cmp(&b.name)));
        let pools = queue
            .pools
            .iter()
            .map(|(name, pool)| PoolStatus {
                name: name.clone(),
                strategy: pool.strategy,
                threshold: pool.threshold,
                accounts: pool
                    .accounts
                    .iter()
                    .map(|room| AccountStatus {
                        name: room.account.name.clone(),
                        program: room.account.program.clone(),
                        running: room.running,
                        bound: room.account.agents,
                        spent_until: room.spent_until,
                        spent_unread: room.spent_unread.clone(),
                        reading: room.reading_status(),
                        past: room.past(),
                    })
                    .collect(),
            })
            .collect();
        HostStatus {
            running: queue.running,
            bound: queue.bound,
            room: queue.room(),
            queue: queued,
            streams,
            pools,
            terminal_kept: self.terminal.kept(),
            addresses: self.addresses(),
            watch: self.terminal.watch(),
            terminals: self.terminals_status(),
            bases: self.bases(agents),
            terminal_setup: self.terminal.setup(),
            configuration: self.host.configuration(),
            sources: self.host_sources(),
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

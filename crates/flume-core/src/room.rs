//! The host's queue (70, 75): a stage starts only while the agents
//! running on the host, every caller's and every stream's counted
//! together, are under the bound its configuration states, and its
//! stream's are under the stream's (143), and otherwise waits for room.
//! Every settle starts the stages waiting, the streams of higher priority
//! first, within them the places of higher priority, and equals in the
//! order they came to wait, as many as there is room for, passing over a
//! stage whose stream has no room without holding the host's room for it,
//! and ending no agent for one (31, 46; core.26). A place's priority is
//! its caller's, given at the launch and changed after (7), and a
//! stream's the host's; a host that states no bound, in a stream that
//! states none, has no queue.

use crate::{Act, Answer, Engine, HostStatus, Queued, RoomWanted, Step, StreamStatus};
use flume_seams::*;
use std::collections::BTreeMap;

/// How a claim admits a start.
#[derive(Clone, Copy)]
pub(crate) enum Admit<'a> {
    /// A stage due to start: it waits unless the host and its stream have
    /// room and no stage waiting that ranks ahead of it could start now.
    Queue(&'a Queue),
    /// A stage the queue admits from waiting for room.
    Waiting,
}

/// The host's agents running, its bound, each stream's, and the stages
/// waiting for room, in the order they would start, read under the room
/// lock.
pub(crate) struct Queue {
    pub running: u32,
    pub bound: Option<u32>,
    pub streams: BTreeMap<String, StreamRoom>,
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

    /// The room a stage of `stream` is short of now, none where the host
    /// and the stream both have room (70, 143).
    pub fn wants(&self, stream: &str) -> Option<RoomWanted> {
        let of = self.stream(stream);
        if let (Some(0), Some(bound)) = (of.room(), of.bound) {
            return Some(RoomWanted::Stream {
                stream: stream.to_string(),
                bound,
            });
        }
        (self.room() == Some(0)).then_some(RoomWanted::Host)
    }

    /// Whether a stage of `place` in `stream` at `priority` due to start
    /// waits: the host or its stream is full, or a stage of another place
    /// waits that ranks as high, its stream's priority then its place's,
    /// and could start now (70, 143; core.26).
    pub fn must_wait(&self, place: &PlaceId, stream: &str, priority: i64) -> bool {
        if self.wants(stream).is_some() {
            return true;
        }
        let rank = (self.stream(stream).priority, priority);
        self.waiting.iter().any(|waiting| {
            &waiting.place != place
                && waiting.rank() >= rank
                && self.wants(&waiting.stream).is_none()
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
        for held in every.iter().map(|read| &read.state) {
            let live = held
                .stages
                .iter()
                .filter(|stage| stage.start.as_ref().is_some_and(|start| start.live))
                .count();
            let live = u32::try_from(live).unwrap_or(u32::MAX);
            running = running.saturating_add(live);
            let stream = streams.entry(held.place.stream.name.clone()).or_default();
            stream.running = stream.running.saturating_add(live);
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
                        Waiting {
                            place: held.place.id.clone(),
                            index,
                            stage: stage.name.clone(),
                            stream: stream.clone(),
                            stream_priority,
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
            waiting,
        }
    }

    /// Starts the stages waiting for room on the host, in the queue's
    /// order, as many as there is room for, passing over a stage whose
    /// stream has no room and holding none of the host's for it, each
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
                if queue.wants(&waiting.stream).is_some() {
                    continue;
                }
                if let Ok(claim) =
                    self.claim(&waiting.place, waiting.index, waiting.words, Admit::Waiting)
                {
                    queue.running = queue.running.saturating_add(1);
                    let stream = queue.streams.entry(waiting.stream).or_default();
                    stream.running = stream.running.saturating_add(1);
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
                wants: queue.wants(&waiting.stream).unwrap_or(RoomWanted::Host),
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

//! The host's queue (70, 75): a stage starts only while the agents
//! running on the host, every caller's counted together, are under the
//! bound its configuration states, and otherwise waits for room. Every
//! settle starts the stages waiting, the highest priority first and
//! equals in the order they came to wait, as many as there is room for,
//! ending no agent for one (31, 46). A place's priority is its caller's,
//! given at the launch and changed after (7); a host that states no bound
//! has no queue.

use crate::{Act, Answer, Engine, HostStatus, Queued, Step};
use flume_seams::*;

/// How a claim admits a start.
#[derive(Clone, Copy)]
pub(crate) enum Admit<'a> {
    /// A stage due to start: it waits unless the host has room and no
    /// stage waiting ranks ahead of it.
    Queue(&'a Queue),
    /// A stage the queue admits from waiting for room.
    Waiting,
}

/// The host's agents running, its bound and the stages waiting for room,
/// in the order they would start, read under the room lock.
pub(crate) struct Queue {
    pub running: u32,
    pub bound: Option<u32>,
    pub waiting: Vec<Waiting>,
}

/// A stage waiting for room.
pub(crate) struct Waiting {
    pub place: PlaceId,
    pub index: usize,
    pub stage: String,
    pub priority: i64,
    pub order: u64,
    pub since: Option<Seconds>,
    pub words: Option<String>,
}

impl Queue {
    /// The agents that may yet start; none where the host states no
    /// bound.
    pub fn room(&self) -> Option<u32> {
        self.bound.map(|bound| bound.saturating_sub(self.running))
    }

    /// Whether a stage of `place` at `priority` due to start waits: the
    /// host is full, or a stage of another place waits at a priority as
    /// high, which came to wait first.
    pub fn must_wait(&self, place: &PlaceId, priority: i64) -> bool {
        match self.room() {
            None => false,
            Some(0) => true,
            Some(_) => self
                .waiting
                .iter()
                .any(|waiting| &waiting.place != place && waiting.priority >= priority),
        }
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
        let running = every
            .iter()
            .flat_map(|read| &read.state.stages)
            .filter(|stage| stage.start.as_ref().is_some_and(|start| start.live))
            .count();
        let mut waiting: Vec<Waiting> = every
            .iter()
            .map(|read| &read.state)
            .filter(|held| matches!(held.phase, Phase::Live | Phase::Making { .. }))
            .flat_map(|held| {
                held.stages
                    .iter()
                    .enumerate()
                    .filter(|(_, stage)| stage.state == StageState::WaitingForRoom)
                    .map(|(index, stage)| {
                        let room = stage.room.clone().unwrap_or_default();
                        Waiting {
                            place: held.place.id.clone(),
                            index,
                            stage: stage.name.clone(),
                            priority: held.priority,
                            order: room.order,
                            since: stage.since,
                            words: room.words,
                        }
                    })
            })
            .collect();
        waiting.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then(a.order.cmp(&b.order))
                .then(a.since.cmp(&b.since))
        });
        Queue {
            running: u32::try_from(running).unwrap_or(u32::MAX),
            bound: self.host.bound(),
            waiting,
        }
    }

    /// Starts the stages waiting for room on the host, in the queue's
    /// order, as many as there is room for, each written live under the
    /// room lock and opened after it is released; ends no agent (31, 46,
    /// 70).
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
            let queue = self.queue();
            let room = queue.room().map_or(usize::MAX, |room| room as usize);
            let mut claims = vec![];
            for waiting in queue.waiting {
                if claims.len() >= room {
                    break;
                }
                if let Ok(claim) =
                    self.claim(&waiting.place, waiting.index, waiting.words, Admit::Waiting)
                {
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
    /// and the queue in the order it would start (56, 70).
    pub(crate) fn host_status(&self) -> HostStatus {
        let queue = self.queue();
        HostStatus {
            running: queue.running,
            bound: queue.bound,
            room: queue.room(),
            queue: queue
                .waiting
                .into_iter()
                .map(|waiting| Queued {
                    place: waiting.place,
                    stage: waiting.stage,
                    priority: waiting.priority,
                    since: waiting.since,
                })
                .collect(),
            terminal_kept: self.terminal.kept(),
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

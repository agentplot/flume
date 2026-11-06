//! A conversation a process runs (30, 49, 207; core.10, core.18): the
//! agent seam's live is asked, with the environments of the accounts the
//! host's configuration declares for the program and the login
//! environment's, whether a process of the host runs the conversation a
//! start would carry, or a lost conversation's fresh start would stand
//! beside, and whether one runs a place's last conversation before its
//! tidy. flume adopts, ends, signals and sends nothing to such a process:
//! it only waits, or keeps the place, while one runs or it cannot read
//! whether one does.

use crate::room::Queue;
use crate::Engine;
use flume_seams::*;
use std::collections::{BTreeMap, BTreeSet};

/// What holds a lost conversation's start where the agent the terminal
/// brought back in its last start's pane still stands, though live read
/// no process running its conversation: that agent may be a person's
/// session, so no start opens beside it (77; core.10).
pub(crate) const LEFT_STANDS: &str =
    "the agent the terminal brought back in its lost start's pane still stands";

/// What a start waits on while a process runs its conversation, or flume
/// cannot read whether one does: each process live read, or what it
/// could not read; none where it read none (207).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct LiveRead {
    pub processes: Vec<LiveProcess>,
    pub unread: Option<String>,
}

impl LiveRead {
    /// What `live` answered, none where it read no process running it.
    pub fn of(live: Live) -> Option<LiveRead> {
        match live {
            Live::None => None,
            Live::Runs(processes) => Some(LiveRead {
                processes,
                unread: None,
            }),
            Live::Unknown { what } => Some(LiveRead {
                processes: vec![],
                unread: Some(what),
            }),
        }
    }

    /// `room` holding it as last read.
    pub fn hold(&self, room: &mut RoomWait) {
        room.live.clone_from(&self.processes);
        room.live_unread.clone_from(&self.unread);
    }
}

/// Whether `room` waits on a process running the conversation its start
/// would carry, or on what flume could not read of one (49, 207).
pub(crate) fn waits_on_session(room: &RoomWait) -> bool {
    !room.live.is_empty() || room.live_unread.is_some()
}

impl Engine {
    /// The environments live reads a conversation of `program` under: each
    /// account the pools of the host's streams declare for `program`, its
    /// variables read against the login environment, and the login
    /// environment's own as an empty one, each once (29, 144, 207). An
    /// account whose variables name one the login environment lacks gives
    /// none, since no start runs under it.
    pub(crate) fn live_envs(&self, program: &str, queue: &Queue) -> Vec<BTreeMap<String, String>> {
        let mut envs: BTreeSet<BTreeMap<String, String>> = BTreeSet::new();
        let mut ordered = vec![];
        let accounts = queue
            .pools
            .values()
            .flat_map(|pool| pool.accounts.iter())
            .filter(|room| room.account.program == program);
        for room in accounts {
            if let Ok(env) = self.account_env(&room.account, "") {
                if envs.insert(env.clone()) {
                    ordered.push(env);
                }
            }
        }
        if envs.insert(BTreeMap::new()) {
            ordered.push(BTreeMap::new());
        }
        ordered
    }

    /// Whether a process runs the conversation of `start`, a start of
    /// `program`, as its agent profile's live reads it in the place's
    /// own folder: none where none does, and unknown, naming the program,
    /// where no agent profile runs it (171, 207).
    pub(crate) fn live_session(
        &self,
        id: &PlaceId,
        program: &str,
        start: &Identity,
        queue: &Queue,
    ) -> Option<LiveRead> {
        let live = match self.agent(program) {
            Some(agent) => agent.live(
                &self.store.folder(id),
                start,
                &self.live_envs(program, queue),
            ),
            None => Live::Unknown {
                what: format!("no agent profile runs the program {program}"),
            },
        };
        LiveRead::of(live)
    }
}

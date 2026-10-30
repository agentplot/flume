//! A spent account (146, 151, 153; core.32): where a settle reads a live
//! start's agent stopped at its account's usage limit, the account is
//! recorded spent in the host's own documents until the reset the
//! agent's record names, or for an hour, so no start of any flume process
//! on the host takes it before then; once the agent is quiet it is ended
//! and its stage started afresh with flume's own words, on another usable
//! account of the pool or waiting for one, counted toward no bound of the
//! kind's and waiting on no caller (31, 112). A start under no account is
//! read as clause 43 reads it, and no other agent on the account is
//! ended for it.

use crate::reading::AgentReading;
use crate::start::{StartReason, Starts};
use crate::{Engine, Step};
use flume_seams::*;

/// How long an account stays spent where its agent's record names no
/// reset (146).
pub const SPENT_FOR: Seconds = 60 * 60;

/// How often a write of an account's spent time reads it again when its
/// version moved under it (103).
const TRIES: usize = 8;

impl Engine {
    /// Records the account `account` spent until `until`, or for an hour
    /// from now where none is named, by compare-and-set: a later time
    /// already recorded is kept, and a pause standing with no reset named
    /// is not lengthened by reading the same limit again (146, 153).
    pub(crate) fn record_spent(&self, account: &AccountHeld, until: Option<Seconds>, words: &str) {
        let key = HostKey::Account {
            pool: account.pool.clone(),
            account: account.name.clone(),
        };
        for _ in 0..TRIES {
            let now = self.host.now();
            let read = self.store.read_host(&key);
            let standing = match &read {
                Some(HostVersioned {
                    document: HostDocument::Account(spent),
                    ..
                }) => Some(spent.until).filter(|until| *until > now),
                _ => None,
            };
            let until = match (until, standing) {
                (Some(until), Some(standing)) if standing >= until => return,
                (Some(until), _) => until,
                (None, Some(_)) => return,
                (None, None) => now + SPENT_FOR,
            };
            let next = HostDocument::Account(AccountSpent {
                until,
                words: Some(words.to_string()),
                recorded_at: now,
            });
            let at = read.map(|read| read.version);
            if let Written::Written { .. } = self.store.write_host(&key, at, Some(&next)) {
                return;
            }
        }
    }

    /// The live start `start` of the stage at `index` read spent: its
    /// account recorded spent, and, once its agent is quiet and the stage
    /// has not delivered, the agent ended and the stage started afresh
    /// with [`StartReason::Spent`]'s words; true when it was. A start
    /// under no account, or whose agent's record shows no usage limit,
    /// is left to the reading of its end (43, 146).
    pub(crate) fn read_spent(
        &self,
        id: &PlaceId,
        index: usize,
        start: &StartHeld,
        reading: &AgentReading,
        delivered: impl FnOnce() -> bool,
    ) -> bool {
        let Some(account) = &start.account else {
            return false;
        };
        let Some(agent) = self.agent(&start.program) else {
            return false;
        };
        let folder = self.store.folder(id);
        let Spent::Yes { until, words } = agent.spent(&folder, &start.identity) else {
            return false;
        };
        self.record_spent(account, until, &words);
        if !reading.quiet() || delivered() {
            return false;
        }
        let identity = &start.identity;
        if reading.state != AgentState::Stopped {
            if let Closed::Refused { reason } = self.terminal.close(identity) {
                self.failed_end(id, identity, &reason);
                return false;
            }
        }
        let words = StartReason::Spent.words();
        {
            let _room = self.host.room();
            let order = self.queue().next_order();
            self.update(id, |held| {
                let Some(at) = held.index_of(identity) else {
                    return Step::Keep(());
                };
                let stage = &mut held.stages[at];
                if let Some(start) = stage.start.as_mut() {
                    start.live = false;
                    start.first_quiet = None;
                    start.past_limit = false;
                }
                stage.state = StageState::WaitingForRoom;
                stage.end = None;
                stage.room = Some(RoomWait {
                    order,
                    words: Some(words.clone()),
                });
                Step::Write(())
            });
        }
        self.start_stage(id, index, Some(words));
        true
    }
}

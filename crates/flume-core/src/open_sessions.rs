//! An agent kept open for its answer (42, 198): a stage that ended short
//! needing an answer, its end giving the time its agent is kept open
//! until, keeps that agent live, holding its room and its account (70,
//! 145). A settle reads the agent let go once its caller stopped it, once
//! it is read stopped, its own record says its session ended or an agent
//! the terminal brought back bare stands in its pane, once its account is
//! spent, or once its time has passed, and records when and why in one
//! write (51, 77, 96, 134, 146). It then ends the agent once it is quiet,
//! never forced, the stage waiting on its answer still (31, 130): an
//! agent read stopped is read ended at once; one brought back bare is
//! left in a conversation's pane, sent nothing, and ended once quiet in
//! any other stage (77; core.10); one not quiet is left for a later
//! settle. No process of flume's waits for the time (120; core.13), and
//! every step is safe to repeat (74; core.17).
//!
//! The cause is read from the agent before the time: a caller's stop, an
//! agent ended or brought back bare, and a spent account each say more
//! than the time, and are what a later answer's retry acts on (49, 77),
//! so a settle that finds the time passed and the agent ended as well
//! records the agent ended.

use crate::reading::AgentReading;
use crate::start::Starts;
use crate::{Engine, Step};
use flume_seams::*;

impl Engine {
    /// The settle's step for a stage ended short needing an answer whose
    /// agent was kept open for it: the agent read let go and recorded so,
    /// then ended once quiet; true when it changed something (198).
    pub(crate) fn close_session(&self, id: &PlaceId) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let Some(kept) = Kept::of(&read.state) else {
            return false;
        };
        if kept.end.closed.is_none() && !self.let_go(id, &read.state, &kept) {
            return false;
        }
        self.end_let_go(id)
    }

    /// Records the agent `kept` holds open let go, where it is, in one
    /// write that takes it only while no other settle has; true when this
    /// one wrote it (74, 75, 198).
    fn let_go(&self, id: &PlaceId, held: &HeldState, kept: &Kept) -> bool {
        let start = &kept.start;
        let reading = start.live.then(|| self.read_agent(id, start));
        let Some(by) = self.let_go_by(id, kept, reading.as_ref()) else {
            return false;
        };
        let conversation = held.place.kind.stages[kept.index].conversation;
        let left = conversation && reading.as_ref().is_some_and(brought_back_bare);
        let now = self.host.now();
        self.update(id, |held| {
            let Some(end) = kept.end_in(held) else {
                return Step::Keep(false);
            };
            if end.closed.is_some() {
                return Step::Keep(false);
            }
            end.closed = Some(SessionClosed {
                at: now,
                by,
                ended_at: (!start.live).then_some(now),
                left_in_pane: left,
            });
            Step::Write(true)
        })
        .unwrap_or(false)
    }

    /// Why the agent `kept` holds open is let go now, as `reading` reads
    /// it where its start is live: its caller's stop, which leaves its
    /// start not live (51); its agent ended, read stopped, or not the
    /// start's, one the terminal brought back bare (77, 96, 134); its
    /// account spent, by its own record or the host's documents (146,
    /// core.32), an account past its pool's threshold not among them
    /// (185); or its time passed. None while it is still kept open.
    fn let_go_by(
        &self,
        id: &PlaceId,
        kept: &Kept,
        reading: Option<&AgentReading>,
    ) -> Option<ClosedBy> {
        let Some(reading) = reading else {
            return Some(ClosedBy::Stopped);
        };
        if reading.state == AgentState::Stopped || brought_back_bare(reading) {
            return Some(ClosedBy::AgentEnded);
        }
        if self.account_spent(id, &kept.start) {
            return Some(ClosedBy::AccountSpent);
        }
        let open_until = kept.end.open_until?;
        (self.host.now() >= open_until).then_some(ClosedBy::Time)
    }

    /// Whether the start's account is spent: its agent's own record shows
    /// it stopped at the account's usage limit, which is then recorded in
    /// the host's documents as a settle records it (146, 151), or those
    /// documents hold it spent now, another agent of the account having
    /// reached its limit (core.32). A start under no account is spent
    /// never.
    fn account_spent(&self, id: &PlaceId, start: &StartHeld) -> bool {
        let Some(account) = &start.account else {
            return false;
        };
        if let Some(agent) = self.agent(&start.program) {
            let folder = self.store.folder(id);
            if let Spent::Yes { until, words } = agent.spent(&folder, &start.identity) {
                self.record_spent(account, until, &words);
                return true;
            }
        }
        self.spent_now(account)
    }

    /// Ends the agent of a stage ended short whose letting go is recorded,
    /// once it is quiet, and records when; true when it changed something
    /// (31, 77; core.10).
    fn end_let_go(&self, id: &PlaceId) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let held = read.state;
        let Some(kept) = Kept::of(&held) else {
            return false;
        };
        let Some(closed) = kept.end.closed else {
            return false;
        };
        let start = &kept.start;
        if !start.live {
            return closed.ended_at.is_none() && self.ended_let_go(id, &kept, false);
        }
        let reading = self.read_agent(id, start);
        if reading.state == AgentState::Stopped {
            return self.ended_let_go(id, &kept, true);
        }
        if brought_back_bare(&reading) && held.place.kind.stages[kept.index].conversation {
            // An agent the terminal brought back in a conversation's pane
            // is left there, sent nothing and never ended by a settle: a
            // person may be at it (77; core.10).
            return !closed.left_in_pane && self.left_in_pane(id, &kept);
        }
        if !reading.quiet() {
            return false;
        }
        if let Closed::Refused { reason } = self.agent_on(start).close() {
            self.failed_end(id, &start.identity, &reason);
            return false;
        }
        self.ended_let_go(id, &kept, true)
    }

    /// The agent `kept` holds let go, ended now: its start no longer live
    /// where `live` says it was, and its close recording when; true when
    /// this write took it (31, 70).
    fn ended_let_go(&self, id: &PlaceId, kept: &Kept, live: bool) -> bool {
        let now = self.host.now();
        self.update(id, |held| {
            let Some(index) = held.index_of(&kept.start.identity) else {
                return Step::Keep(false);
            };
            if held.stages[index]
                .start
                .as_ref()
                .is_none_or(|start| start.live != live)
            {
                return Step::Keep(false);
            }
            let Some(closed) = kept
                .end_in(held)
                .and_then(|end| end.closed.as_mut())
                .filter(|closed| closed.ended_at.is_none())
            else {
                return Step::Keep(false);
            };
            closed.ended_at = Some(now);
            if let Some(start) = held.stages[index].start.as_mut() {
                start.live = false;
                start.first_quiet = None;
                start.past_limit = false;
            }
            Step::Write(true)
        })
        .unwrap_or(false)
    }

    /// The close of the agent `kept` holds records an agent the terminal
    /// brought back bare left in its conversation's pane; true when this
    /// write took it (77).
    fn left_in_pane(&self, id: &PlaceId, kept: &Kept) -> bool {
        self.update(id, |held| {
            match kept.end_in(held).and_then(|end| end.closed.as_mut()) {
                Some(closed) if !closed.left_in_pane && closed.ended_at.is_none() => {
                    closed.left_in_pane = true;
                    Step::Write(true)
                }
                _ => Step::Keep(false),
            }
        })
        .unwrap_or(false)
    }
}

/// Whether the agent `reading` reads in its start's pane is not the
/// start's, one a terminal brought back bare with no word that it started
/// under the start's identity, or with another record than its start's
/// (35, 77, 161).
fn brought_back_bare(reading: &AgentReading) -> bool {
    reading.state != AgentState::Stopped
        && (reading.started == Reading::No || reading.own_record == Reading::No)
}

/// A place's current stage ended short needing an answer whose end gives
/// the time its agent is kept open until, as one read of its held state
/// gives it.
struct Kept {
    index: usize,
    start: StartHeld,
    end: StageEnd,
}

impl Kept {
    fn of(held: &HeldState) -> Option<Kept> {
        let index = held.current?;
        let stage = &held.stages[index];
        if held.phase != Phase::Live || stage.state != StageState::EndedShort {
            return None;
        }
        let end = stage.end.clone().filter(|end| end.open_until.is_some())?;
        Some(Kept {
            index,
            start: stage.start.clone()?,
            end,
        })
    }

    /// The same stage's end in `held`, as a write decides it: none where
    /// the place moved on, its stage no longer ended short under the same
    /// start, or its end no longer the one kept open (75).
    fn end_in<'a>(&self, held: &'a mut HeldState) -> Option<&'a mut StageEnd> {
        if held.phase != Phase::Live || held.current != Some(self.index) {
            return None;
        }
        let stage = &mut held.stages[self.index];
        let same = stage.state == StageState::EndedShort
            && stage
                .start
                .as_ref()
                .is_some_and(|start| start.identity == self.start.identity);
        stage
            .end
            .as_mut()
            .filter(|end| same && end.open_until == self.end.open_until)
    }
}

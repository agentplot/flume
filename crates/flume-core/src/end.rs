//! Reading a begun stage's end (39 to 45, 109, 118): a stage that is not
//! a conversation ends only at its deliverable, read at one head, once
//! its agent is quiet; a quiet terminal, an ended turn or a closed one
//! never ends it. An agent quiet past the stage's limit with nothing
//! delivered is reported stuck, one working past it is reported so, and
//! one that stopped with nothing delivered is reported lost; flume ends
//! none of them. A conversation is read only for lost. After a restart,
//! an agent that did not survive reads lost, and one the terminal resumed
//! is taken only when its own record is its start's (77); one whose record
//! cannot be read is neither taken nor lost, and is not quiet (31, 93).

use crate::reading::{repeats_on, AgentReading, Facts};
use crate::start::Starts;
use crate::{Engine, Step, QUIET_LIMIT};
use flume_seams::*;

impl Engine {
    /// Reads the current begun stage; true when it ended or was lost.
    pub(crate) fn read_end(&self, id: &PlaceId) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let held = read.state;
        let Some(index) = held.current else {
            return false;
        };
        let declared = held.place.kind.stages[index].clone();
        let Some(start) = held.stages[index].start.clone().filter(|start| start.live) else {
            return false;
        };
        let Ok(bound) = self.bound(&held.place) else {
            return false;
        };
        let identity = start.identity.clone();
        let reading = self.read_agent(id, &start);
        if held.stages[index].state == StageState::Lost {
            return self.end_foreign(id, &identity, &reading);
        }
        let foreign = reading.started == Reading::No || reading.own_record == Reading::No;
        let gone = reading.state == AgentState::Stopped || foreign;
        if gone && !declared.conversation {
            let facts = self.facts(&held, &bound, Some(index));
            if facts.delivered {
                let state = delivered_state(&held, index, &facts);
                return self.ended(id, &identity, state, facts.head, facts.report);
            }
        }
        if gone {
            let end = self.read_out(&held, &start, false);
            let stopped = reading.state == AgentState::Stopped;
            self.end_start(
                id,
                &identity,
                |start, state| {
                    start.live = !stopped;
                    *state = StageState::Lost;
                },
                Some(end),
            );
            if !stopped {
                self.end_foreign(id, &identity, &reading);
            }
            return true;
        }
        if declared.conversation {
            return false;
        }
        let facts = self.facts(&held, &bound, Some(index));
        if facts.delivered && reading.quiet() {
            if let Closed::Refused { reason } = self.terminal.close(&identity) {
                self.failed_end(id, &identity, &reason);
                return false;
            }
            let state = delivered_state(&held, index, &facts);
            return self.ended(id, &identity, state, facts.head, facts.report);
        }
        let now = self.host.now();
        let limit = declared.quiet_limit.unwrap_or(QUIET_LIMIT);
        self.with_start(id, &identity, |start, state| {
            if start.last_head != facts.head {
                start.last_head = facts.head.clone();
                start.last_head_move = now;
                start.first_quiet = None;
            }
            if facts.delivered {
                start.first_quiet = None;
                start.past_limit = false;
                if *state == StageState::Stuck {
                    *state = StageState::Begun;
                }
                return;
            }
            if reading.quiet() {
                let since = *start.first_quiet.get_or_insert(now);
                if now.saturating_sub(since) >= limit {
                    *state = StageState::Stuck;
                }
            } else {
                start.first_quiet = None;
                if *state == StageState::Stuck {
                    *state = StageState::Begun;
                }
                if reading.state == AgentState::Working {
                    let from = start
                        .begun_at
                        .unwrap_or(start.started_at)
                        .max(start.last_head_move);
                    start.past_limit = now.saturating_sub(from) >= limit;
                }
            }
        });
        self.mark_stuck(id, &start);
        false
    }

    /// A stuck stage's end carries what was read of its agent while it
    /// stays stuck, and a stage that is stuck no more carries none (43).
    fn mark_stuck(&self, id: &PlaceId, start: &StartHeld) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let Some(index) = read.state.index_of(&start.identity) else {
            return;
        };
        let stage = &read.state.stages[index];
        let end = match (stage.state, stage.end.is_some()) {
            (StageState::Stuck, false) => Some(self.read_out(&read.state, start, false)),
            (StageState::Begun, true) => None,
            _ => return,
        };
        self.update(id, |held| {
            held.stages[index].end = end.clone();
            Step::Write(())
        });
    }

    /// The agent of a lost stage that is no agent of the stage's, a
    /// session the terminal brought back with no word that it started
    /// among them, ended once quiet (77, 31); true when it was ended.
    fn end_foreign(&self, id: &PlaceId, identity: &Identity, reading: &AgentReading) -> bool {
        if reading.state != AgentState::Stopped {
            if !reading.quiet() {
                return false;
            }
            if let Closed::Refused { reason } = self.terminal.close(identity) {
                self.failed_end(id, identity, &reason);
                return false;
            }
        }
        self.with_start(id, identity, |start, _| start.live = false);
        true
    }

    /// The stage under `identity` ended as `state`, with what it
    /// delivered and the lines its run wrote of what it made, its agent
    /// ended or lost (40, 43, 45, 129); a repetition that delivered its
    /// group with a task still unticked records no end, and its stage
    /// repeats (140); false when it had ended.
    pub(crate) fn ended(
        &self,
        id: &PlaceId,
        identity: &Identity,
        state: StageState,
        head: Revision,
        report: Option<String>,
    ) -> bool {
        let lines = self.store.made(id);
        self.update(id, |held| {
            let Some(index) = held.index_of(identity) else {
                return Step::Keep(false);
            };
            let stage = &mut held.stages[index];
            let lost = stage.state == StageState::Lost;
            let Some(start) = stage.start.as_mut().filter(|start| start.live || lost) else {
                return Step::Keep(false);
            };
            start.live = false;
            start.first_quiet = None;
            start.past_limit = false;
            stage.state = state;
            if state == StageState::Repeating {
                stage.end = None;
                return Step::Write(true);
            }
            let made = lines
                .iter()
                .skip(start.made_from)
                .map(|line| Made::read(line))
                .collect();
            stage.end = Some(StageEnd {
                head: Some(head.clone()),
                report: report.clone(),
                made,
                ..StageEnd::default()
            });
            if index + 1 == held.stages.len() && held.place.kind.lands {
                held.submission.due = true;
            }
            Step::Write(true)
        })
        .unwrap_or(false)
    }

    /// An end flume asked for that the agent did not take, reported as a
    /// failure (31).
    pub(crate) fn failed_end(&self, id: &PlaceId, identity: &Identity, reason: &str) {
        let failure = format!("{identity} did not end when closed: {reason}");
        self.update(id, |held| {
            if held.failures.last() != Some(&failure) {
                held.failures.push(failure.clone());
            }
            Step::Write(())
        });
    }
}

/// How a delivered start ends its stage: a repetition with a task of the
/// change still unticked at the head it delivered at repeats, recording
/// no end; any other start, the last repetition among them, delivers the
/// stage (40, 43, 140).
fn delivered_state(held: &HeldState, index: usize, facts: &Facts) -> StageState {
    if repeats_on(held, index, facts) {
        StageState::Repeating
    } else {
        StageState::Delivered
    }
}

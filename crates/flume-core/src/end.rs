//! Reading a begun stage's end (39 to 45, 109, 118): a stage that is not
//! a conversation ends only at its deliverable, read at one head, once
//! its agent is quiet; a quiet terminal, an ended turn or a closed one
//! never ends it. An agent quiet past the stage's limit with nothing
//! delivered is reported stuck, one working past it is reported so, and
//! one that stopped with nothing delivered is reported lost; flume ends
//! none of them. A conversation is read only for lost.

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
        let reading = self.read_agent(&start);
        if reading.state == AgentState::Stopped && !declared.conversation {
            let facts = self.facts(&held, &bound, Some(index));
            if facts.delivered {
                return self.ended(
                    id,
                    &identity,
                    StageState::Delivered,
                    facts.head,
                    facts.report,
                );
            }
        }
        if reading.state == AgentState::Stopped {
            self.with_start(id, &identity, |start, state| {
                start.live = false;
                *state = StageState::Lost;
            });
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
            return self.ended(
                id,
                &identity,
                StageState::Delivered,
                facts.head,
                facts.report,
            );
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
        false
    }

    /// The stage under `identity` ended as `state`, with what it
    /// delivered, its agent ended (40, 43); false when it had ended.
    pub(crate) fn ended(
        &self,
        id: &PlaceId,
        identity: &Identity,
        state: StageState,
        head: Revision,
        report: Option<String>,
    ) -> bool {
        self.update(id, |held| {
            let Some(index) = held.index_of(identity) else {
                return Step::Keep(false);
            };
            let stage = &mut held.stages[index];
            let Some(start) = stage.start.as_mut().filter(|start| start.live) else {
                return Step::Keep(false);
            };
            start.live = false;
            start.first_quiet = None;
            start.past_limit = false;
            stage.state = state;
            stage.end = Some(StageEnd {
                head: Some(head.clone()),
                report: report.clone(),
                words: None,
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

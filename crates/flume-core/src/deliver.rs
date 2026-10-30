//! Delivering a stage's prompt (35, 37, 38): a start is starting until
//! its agent says it started, read begun without its settings where its
//! agent's word shows so (161), and never begun at once when its agent
//! stops first, or at the stage's limit while it stays starting; the
//! prompt is sent only once the agent said it started and waits and is
//! not asking, read within 60 seconds of each sending whether it was
//! received, sent once more where the terminal can send, and reported
//! never begun when no sending is received. A start that carried its
//! prompt counts its one sending from the agent's word that it started.
//! A sending is written to held state before it is made, so two settles
//! at once send it once (75).

use crate::start::Starts;
use crate::{Engine, Step, POLL, QUIET_LIMIT, RECEIVED_WITHIN};
use flume_seams::*;

impl Engine {
    /// Moves the current start's prompt on as far as its agent allows;
    /// true when the stage's state changed.
    pub(crate) fn deliver(&self, id: &PlaceId) -> bool {
        let mut sent = false;
        loop {
            let Some(read) = self.store.read(id) else {
                return sent;
            };
            let held = read.state;
            let Some(index) = held.current else {
                return sent;
            };
            let stage = &held.stages[index];
            let Some(start) = stage.start.clone().filter(|start| start.live) else {
                return sent;
            };
            if !matches!(stage.state, StageState::Starting | StageState::Started) {
                return sent;
            }
            let Some(agent) = self.agent(&start.program) else {
                return sent;
            };
            let identity = &start.identity;
            let prompt = &start.prompt;
            let folder = self.store.folder(id);
            if stage.state == StageState::Starting {
                // An agent begun without its settings is sent no prompt:
                // it is ended once quiet and its stage started afresh
                // (161).
                if let Some(unsettled) = agent.started_without(&folder, identity) {
                    if self.read_unsettled(id, index, &start, unsettled) {
                        return true;
                    }
                } else if agent.started(&folder, identity) == Reading::Yes {
                    self.with_start(id, identity, |_, state| *state = StageState::Started);
                    sent = true;
                    continue;
                }
                // The program may ask whether it may work in the place
                // after the open returned: the terminal answers it where
                // it shows it now (34, 35).
                let _ = self.terminal.answer_trust(identity);
                let stopped =
                    self.state_of(Some(agent.as_ref()), &folder, identity) == AgentState::Stopped;
                let limit = held.place.kind.stages[index]
                    .quiet_limit
                    .unwrap_or(QUIET_LIMIT);
                let waited = self.host.now().saturating_sub(start.started_at);
                if stopped || waited >= limit {
                    self.never_begun(id, &held, &start, stopped);
                    return true;
                }
                return sent;
            }
            if let Received::Received { record } = agent.received(&folder, identity, &prompt.text) {
                let now = self.host.now();
                self.with_start(id, identity, |start, state| {
                    start.prompt.received_at = Some(now);
                    start.record = Some(record.clone());
                    start.begun_at = Some(now);
                    *state = StageState::Begun;
                });
                return true;
            }
            let state = self.state_of(Some(agent.as_ref()), &folder, identity);
            if state == AgentState::Stopped {
                let end = self.read_out(&held, &start, false);
                self.end_start(
                    id,
                    identity,
                    |start, state| {
                        start.live = false;
                        *state = StageState::Lost;
                    },
                    Some(end),
                );
                return true;
            }
            let now = self.host.now();
            if let Some(last) = prompt.sendings.last() {
                let waited = now.saturating_sub(*last);
                if waited < RECEIVED_WITHIN {
                    self.host.sleep(POLL.min(RECEIVED_WITHIN - waited));
                    continue;
                }
                if prompt.carried || prompt.sendings.len() >= 2 {
                    self.never_begun(id, &held, &start, false);
                    return true;
                }
            }
            if prompt.carried {
                self.claim_sending(id, identity, prompt.sendings.len(), now);
                continue;
            }
            if state != AgentState::Waiting || agent.asking(&folder, identity) != Reading::No {
                return sent;
            }
            if !self.claim_sending(id, identity, prompt.sendings.len(), now) {
                continue;
            }
            match self.terminal.send(identity, &prompt.text) {
                Sent::Sent => sent = true,
                Sent::Refused { reason } => {
                    let failure =
                        format!("the terminal did not send {identity} its prompt: {reason}");
                    self.never_begun(id, &held, &start, false);
                    self.update(id, |held| {
                        held.failures.push(failure.clone());
                        Step::Write(())
                    });
                    return true;
                }
            }
        }
    }

    /// The start's prompt reported never begun, with what the terminal
    /// shows of the agent, its last words and the commits since the start,
    /// and how its agent began without its settings where the place
    /// recorded so; its agent read ended where it `stopped` (35, 38, 43,
    /// 161).
    pub(crate) fn never_begun(
        &self,
        id: &PlaceId,
        held: &HeldState,
        start: &StartHeld,
        stopped: bool,
    ) {
        let mut end = self.read_out(held, start, true);
        end.unsettled = held
            .unsettled
            .iter()
            .find(|read| read.stage == start.identity.stage && read.start == start.n)
            .map(|read| Box::new(read.unsettled.clone()));
        self.end_start(
            id,
            &start.identity,
            |start, state| {
                if stopped {
                    start.live = false;
                }
                *state = StageState::NeverBegun;
            },
            Some(end),
        );
    }

    /// The start under `identity` reported never begun at once, the
    /// terminal having read it held at a question its program asks before
    /// it starts or its program not come up, with what the terminal
    /// showed then; no prompt is sent (35, 43).
    pub(crate) fn never_begun_with(&self, id: &PlaceId, identity: &Identity, screen: String) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let held = read.state;
        let Some(start) = held
            .index_of(identity)
            .and_then(|index| held.stages[index].start.clone())
        else {
            return;
        };
        let mut end = self.read_out(&held, &start, false);
        end.screen = Some(screen);
        self.end_start(
            id,
            identity,
            |_, state| *state = StageState::NeverBegun,
            Some(end),
        );
    }

    /// Records a sending at `now`, if the start has made `made` so far;
    /// false when another act made it first.
    fn claim_sending(&self, id: &PlaceId, identity: &Identity, made: usize, now: Seconds) -> bool {
        self.update(id, |held| {
            let Some(index) = held.index_of(identity) else {
                return Step::Keep(false);
            };
            let Some(start) = held.stages[index].start.as_mut() else {
                return Step::Keep(false);
            };
            if start.prompt.sendings.len() != made {
                return Step::Keep(false);
            }
            start.prompt.sendings.push(now);
            Step::Write(true)
        })
        .unwrap_or(false)
    }
}

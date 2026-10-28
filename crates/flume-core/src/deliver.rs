//! Delivering a stage's prompt (35, 37, 38): sent only once its agent is
//! waiting and not asking, read within 60 seconds of each sending
//! whether it was received, sent once more where the terminal can send,
//! and reported never begun when no sending is received. A sending is
//! written to held state before it is made, so two settles at once send
//! it once (75).

use crate::start::Starts;
use crate::{Engine, Step, POLL, RECEIVED_WITHIN};
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
            if stage.state != StageState::Started {
                return sent;
            }
            let Some(agent) = self.agent(&start.program) else {
                return sent;
            };
            let identity = &start.identity;
            let prompt = &start.prompt;
            let folder = self.store.folder(id);
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
            let state = self.terminal.state(identity);
            if state == AgentState::Stopped {
                self.with_start(id, identity, |start, state| {
                    start.live = false;
                    *state = StageState::Lost;
                });
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
                    self.with_start(id, identity, |_, state| *state = StageState::NeverBegun);
                    return true;
                }
            }
            if prompt.carried {
                if state == AgentState::Unknown {
                    return sent;
                }
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
                    self.update(id, |held| {
                        if let Some(index) = held.index_of(identity) {
                            held.stages[index].state = StageState::NeverBegun;
                            held.failures.push(failure.clone());
                        }
                        Step::Write(())
                    });
                    return true;
                }
            }
        }
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

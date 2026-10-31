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
//! at once send it once (75). While the command opening a start's agent
//! lives, the terminal may still be bringing its program up, so nothing
//! else is sent to it or answered in it; a start whose opener is gone
//! is read as opened (34, 35). A dialog the agent's program shows a
//! person before it takes its prompt holds the start, its words kept
//! with it: the place waits on a person, and the start is never read
//! never begun while the dialog shows (34, 35). A stage read never begun
//! whose agent is still live and then says it started, or received its
//! prompt, goes on as begun: its start is the stage's after all (35,
//! 38).

use crate::start::Starts;
use crate::{Engine, Step, POLL, QUIET_LIMIT, RECEIVED_WITHIN};
use flume_seams::*;

impl Engine {
    /// Moves the current start's prompt on as far as its agent allows,
    /// reading each sending back within 60 seconds; true when the stage's
    /// state changed.
    pub(crate) fn deliver(&self, id: &PlaceId) -> bool {
        self.deliver_reading(id, true)
    }

    /// Sends the current start's prompt where its agent allows it now,
    /// and leaves its receipt to the settles that follow; true when the
    /// stage's state changed (6, 38).
    pub(crate) fn send_prompt(&self, id: &PlaceId) -> bool {
        self.deliver_reading(id, false)
    }

    /// Moves the current start's prompt on, waiting on a sending's
    /// receipt only where `read_back`.
    fn deliver_reading(&self, id: &PlaceId, read_back: bool) -> bool {
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
            if start.opening.as_ref().is_some_and(|by| self.host.alive(by)) {
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
                    self.with_start(id, identity, |start, state| {
                        start.dialog = None;
                        *state = StageState::Started;
                    });
                    sent = true;
                    continue;
                }
                // The program may ask whether it may work in the place
                // after the open returned: the terminal answers it where
                // it shows it now (34, 35).
                let answered = self.terminal.answer_trust(identity);
                let stopped =
                    self.state_of(Some(agent.as_ref()), &folder, identity) == AgentState::Stopped;
                // Any other question it shows a person holds the start,
                // which waits on them however long (34, 35).
                let dialog = match (answered, stopped) {
                    (_, true) => false,
                    (true, false) => start.dialog.is_some(),
                    (false, false) => self.read_dialog(id, &start),
                };
                let limit = held.place.kind.stages[index]
                    .quiet_limit
                    .unwrap_or(QUIET_LIMIT);
                let waited = self.host.now().saturating_sub(start.started_at);
                if stopped || (!dialog && waited >= limit) {
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
                    start.dialog = None;
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
            // A dialog shown before the agent takes its prompt holds the
            // prompt: nothing is sent into it, and the prompt is never
            // read never begun while it shows (34, 35, 37).
            let dialog = self.read_dialog(id, &start);
            let now = self.host.now();
            if let Some(last) = prompt.sendings.last() {
                let waited = now.saturating_sub(*last);
                if waited < RECEIVED_WITHIN {
                    if !read_back {
                        return sent;
                    }
                    self.host.sleep(POLL.min(RECEIVED_WITHIN - waited));
                    continue;
                }
                if prompt.carried || prompt.sendings.len() >= 2 {
                    if dialog {
                        return sent;
                    }
                    self.never_begun(id, &held, &start, false);
                    return true;
                }
            }
            if prompt.carried {
                self.claim_sending(id, identity, prompt.sendings.len(), now);
                continue;
            }
            if dialog
                || state != AgentState::Waiting
                || agent.asking(&folder, identity) != Reading::No
            {
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
                start.dialog = None;
                *state = StageState::NeverBegun;
            },
            Some(end),
        );
    }

    /// Reads whether the agent's program shows a person a dialog before
    /// it takes its prompt: where the terminal reads it asking, its words
    /// as the terminal shows them now are kept with the start, since when
    /// it was first read so; where the terminal reads it otherwise, the
    /// dialog is gone; where it cannot read it, the start keeps what it
    /// held. Whether a dialog holds the start (34, 35).
    fn read_dialog(&self, id: &PlaceId, start: &StartHeld) -> bool {
        let identity = &start.identity;
        let dialog = match self.terminal.state(identity) {
            AgentState::Asking => Some(Dialog {
                words: self
                    .terminal
                    .screen(identity)
                    .map(|screen| shown(&screen))
                    .filter(|words| !words.is_empty())
                    .or_else(|| start.dialog.as_ref().map(|dialog| dialog.words.clone()))
                    .unwrap_or_default(),
                since: start
                    .dialog
                    .as_ref()
                    .map_or_else(|| self.host.now(), |dialog| dialog.since),
            }),
            AgentState::Unknown => start.dialog.clone(),
            _ => None,
        };
        if dialog != start.dialog {
            self.with_start(id, identity, |start, _| start.dialog.clone_from(&dialog));
        }
        dialog.is_some()
    }

    /// A stage read never begun whose agent is still live and has since
    /// said it started, before any sending, or received its prompt, goes
    /// on as its start's: started, its prompt sent as 37 allows, or begun;
    /// its never-begun end is read no more, and what was delivered of it
    /// stays delivered. True when the stage changed (35, 38, 43).
    pub(crate) fn begun_after(&self, id: &PlaceId) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let held = read.state;
        let Some(index) = held.current else {
            return false;
        };
        let stage = &held.stages[index];
        if stage.state != StageState::NeverBegun {
            return false;
        }
        let Some(start) = stage.start.clone().filter(|start| start.live) else {
            return false;
        };
        if start.opening.as_ref().is_some_and(|by| self.host.alive(by)) {
            return false;
        }
        let Some(agent) = self.agent(&start.program) else {
            return false;
        };
        let folder = self.store.folder(id);
        let identity = &start.identity;
        if agent.started_without(&folder, identity).is_some() {
            return false;
        }
        let received = match agent.received(&folder, identity, &start.prompt.text) {
            Received::Received { record } => Some(record),
            _ => None,
        };
        if received.is_none()
            && !(start.prompt.sendings.is_empty()
                && agent.started(&folder, identity) == Reading::Yes)
        {
            return false;
        }
        let now = self.host.now();
        self.update(id, |held| {
            let Some(index) = held.index_of(identity) else {
                return Step::Keep(false);
            };
            let stage = &mut held.stages[index];
            if stage.state != StageState::NeverBegun {
                return Step::Keep(false);
            }
            let Some(start) = stage.start.as_mut().filter(|start| start.live) else {
                return Step::Keep(false);
            };
            start.dialog = None;
            stage.end = None;
            stage.state = match &received {
                Some(record) => {
                    start.prompt.received_at = Some(now);
                    start.record = Some(record.clone());
                    start.begun_at = Some(now);
                    StageState::Begun
                }
                None => StageState::Started,
            };
            Step::Write(true)
        })
        .unwrap_or(false)
    }

    /// The start under `identity` reported never begun at once, the
    /// terminal having read its program not come up, with what the
    /// terminal showed then; no prompt is sent (35, 43).
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
            |start, state| {
                start.dialog = None;
                *state = StageState::NeverBegun;
            },
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

/// What a terminal shows, each line's trailing blanks and the blank lines
/// around it taken off, so a dialog's words read as it draws them.
fn shown(screen: &str) -> String {
    screen
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim_matches('\n')
        .to_string()
}

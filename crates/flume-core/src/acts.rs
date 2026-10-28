//! The caller's acts on a place after its launch (47 to 55, 109, 114),
//! and the prompt a stage would be sent (29). Each ends an agent only
//! through the terminal's close, only when it is quiet or the caller
//! forced it, and answers an end the agent did not take as a failure
//! (31).

use crate::start::{Start, Starts};
use crate::{Act, Answer, Engine, Step, Tidied, POLL, RECEIVED_WITHIN, TELL_BOUND};
use flume_seams::*;

/// What a retry names (49).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Retry {
    /// The current stage.
    Current,
    Stage(String),
    Submission,
}

impl Engine {
    /// The exact prompt the stage would be sent at its next start,
    /// rendered with nothing started, sent, settled or written (29,
    /// 119).
    pub fn prompt(&self, place: &PlaceId, stage: &str) -> Answer {
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        let held = read.state;
        let Some(index) = held.stages.iter().position(|held| held.name == stage) else {
            return Answer::refused(format!("the place {place} has no stage {stage}"));
        };
        let identity = self.next_identity(&held, index);
        match self.fill(&held, index, &identity, None) {
            Ok(text) => Answer::Prompt {
                place: place.clone(),
                stage: stage.into(),
                text,
            },
            Err(part) => Answer::refused(format!(
                "the stage {stage}'s prompt asks for {}, which has nothing to fill it",
                part.name()
            )),
        }
    }

    /// Ends the place's live agent, one that is not quiet only when
    /// forced; a stage it ran that had not ended reads stopped, and one
    /// still starting is reported never begun with what the terminal
    /// showed of it (31, 35, 51).
    fn end_agent(&self, id: &PlaceId, force: bool) -> Result<(), String> {
        let Some(read) = self.store.read(id) else {
            return Ok(());
        };
        let Some((index, start)) = read.state.live_start() else {
            return Ok(());
        };
        let identity = start.identity.clone();
        let reading = self.read_agent(id, start);
        let starting = (read.state.stages[index].state == StageState::Starting)
            .then(|| self.read_out(&read.state, start, true));
        if reading.state != AgentState::Stopped {
            if !reading.quiet() && !force {
                return Err(format!(
                    "the agent {identity} is not quiet; ask again forced to end it"
                ));
            }
            if let Closed::Refused { reason } = self.terminal.close(&identity) {
                self.failed_end(id, &identity, &reason);
                return Err(format!("the agent {identity} did not end: {reason}"));
            }
        }
        self.with_start(id, &identity, |start, state| {
            start.live = false;
            if *state == StageState::Starting {
                *state = StageState::NeverBegun;
            } else if state.running() {
                *state = StageState::Stopped;
            }
        });
        if let Some(end) = starting {
            self.end_with(id, &identity, end);
        }
        Ok(())
    }

    /// Starts the stage a retry names with a fresh agent and the retry's
    /// words, after the question of the stop-short the stage ended with,
    /// the stages after it then running again in order; or, naming
    /// the submission, submits the place's head again and starts no
    /// agent. The place's agent is ended first (49, 50, 12).
    pub fn retry(
        &self,
        place: &PlaceId,
        target: Retry,
        words: Option<String>,
        force: bool,
    ) -> Answer {
        self.settle_attended();
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        let held = read.state;
        if held.phase != Phase::Live {
            return Answer::refused(format!("the work of the place {place} is over"));
        }
        let index = match &target {
            Retry::Current => Some(held.current.unwrap_or(0)),
            Retry::Stage(name) => match held.stages.iter().position(|stage| &stage.name == name) {
                Some(index) => Some(index),
                None => return Answer::refused(format!("the place {place} has no stage {name}")),
            },
            Retry::Submission if !held.place.kind.lands => {
                return Answer::refused(format!(
                    "the place {place}'s kind does not land, so it has no submission"
                ))
            }
            Retry::Submission => None,
        };
        if let Err(reason) = self.end_agent(place, force) {
            return Answer::refused(reason);
        }
        let Some(index) = index else {
            self.submit(place, true);
            self.settle_place(place);
            return self.acted(Act::Retry, place);
        };
        if index == 0 && held.readiness.failed().is_some() {
            let Ok(bound) = self.bound(&held.place) else {
                return Answer::refused(format!("the place {place}'s seams are not bound"));
            };
            let readiness = bound
                .place
                .ready(&held.place.place_ref(), &held.place.base_revision);
            let failed = readiness.failed().is_some();
            let waits = failed && !held.place.kind.starts_unready;
            self.update(place, |held| {
                held.readiness = readiness.clone();
                if waits {
                    held.stages[0].state = StageState::WaitingOnReadiness;
                }
                Step::Write(())
            });
            if waits {
                return self.acted(Act::Retry, place);
            }
        }
        let words = answered(&held.stages[index], words);
        self.update(place, |held| {
            for stage in held.stages.iter_mut().skip(index + 1) {
                stage.state = StageState::NotReached;
            }
            held.submission.due = false;
            held.submission.waiting = None;
            Step::Write(())
        });
        match self.start_stage(place, index, words) {
            Start::Opened(_) | Start::WaitingForRoom => self.acted(Act::Retry, place),
            Start::Refused(part) => Answer::refused(format!(
                "the stage {}'s prompt asks for {}, which has nothing to fill it",
                held.stages[index].name,
                part.name()
            )),
            Start::AgentLive => Answer::refused(format!("an agent of the place {place} is live")),
            Start::NotOpened(reason) => {
                Answer::refused(format!("the terminal did not open the agent: {reason}"))
            }
            Start::NoAgent(program) => {
                Answer::refused(format!("no agent profile runs the program {program}"))
            }
            Start::NoPlace => Answer::refused(format!("no place {place} stands on this host")),
        }
    }

    /// Ends the place's agent and nothing else: the stage neither ends nor
    /// moves on, and waits for a retry or a drop (51).
    pub fn stop(&self, place: &PlaceId, force: bool) -> Answer {
        self.settle_attended();
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        if read.state.live_start().is_none() {
            return Answer::refused(format!("the place {place} has no live agent"));
        }
        match self.end_agent(place, force) {
            Ok(()) => self.acted(Act::Stop, place),
            Err(reason) => Answer::refused(reason),
        }
    }

    /// Gives up the place's work: its agent ended, nothing more started,
    /// and the place tidied (52, 63).
    pub fn drop_place(&self, place: &PlaceId, force: bool) -> Answer {
        self.give_up(place, force, Act::Drop, Over::Dropped)
    }

    /// Declares the work of a place whose kind does not land complete
    /// (114).
    pub fn done(&self, place: &PlaceId, force: bool) -> Answer {
        if let Some(read) = self.store.read(place) {
            if read.state.place.kind.lands {
                return Answer::refused(format!(
                    "the place {place}'s kind lands, so it is never done; drop it instead"
                ));
            }
        }
        self.give_up(place, force, Act::Done, Over::Done)
    }

    fn give_up(&self, place: &PlaceId, force: bool, act: Act, over: Over) -> Answer {
        let mut tidy = Tidied::default();
        tidy.add(self.settle_attended());
        let Some(read) = self.store.read(place) else {
            return Answer::Acted {
                act,
                place: place.clone(),
                status: None,
                tidy: Some(only(tidy, place)),
            };
        };
        if read.state.phase == Phase::Live {
            if let Err(reason) = self.end_agent(place, force) {
                return Answer::refused(reason);
            }
            self.over(place, over);
        }
        tidy.add(self.settle_place(place));
        Answer::Acted {
            act,
            place: place.clone(),
            status: self
                .store
                .read(place)
                .map(|read| self.place_status(&read.state)),
            tidy: Some(only(tidy, place)),
        }
    }

    /// Ends a conversation's agent once quiet, one not quiet only when
    /// forced, and the conversation then counts as delivered (109, 40).
    /// A conversation whose agent was lost has no agent to end, and waits
    /// on a retry or this finish (45).
    pub fn finish(&self, place: &PlaceId, force: bool) -> Answer {
        self.settle_attended();
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        let held = read.state;
        let current = held.current.filter(|index| {
            let state = held.stages[*index].state;
            held.place.kind.stages[*index].conversation
                && (state.running() || state == StageState::Lost)
        });
        let Some(index) = current else {
            return Answer::refused(format!("the place {place} runs no conversation"));
        };
        let Some(start) = held.stages[index].start.clone() else {
            return Answer::refused(format!("the place {place}'s conversation has no agent"));
        };
        let reading = start.live.then(|| self.read_agent(place, &start));
        if let Some(reading) = reading.filter(|reading| reading.state != AgentState::Stopped) {
            if !reading.quiet() && !force {
                return Answer::refused(format!(
                    "the agent {} is not quiet; ask again forced to finish it",
                    start.identity
                ));
            }
            if let Closed::Refused { reason } = self.terminal.close(&start.identity) {
                self.failed_end(place, &start.identity, &reason);
                return Answer::refused(format!(
                    "the agent {} did not end: {reason}",
                    start.identity
                ));
            }
        }
        let head = self
            .history
            .head(&held.place.repository, &held.place.branch)
            .unwrap_or_else(|| held.place.base_revision.clone());
        self.ended(place, &start.identity, StageState::Finished, head, None);
        self.settle_place(place);
        self.acted(Act::Finish, place)
    }

    /// Hands the agent of the place's current stage `words` as a prompt
    /// once it said it started and waits, received or sent once more; refused with nothing
    /// typed when the place has no live agent, its agent is asking, or the
    /// words are not delivered within the bound (53 to 55).
    pub fn tell(&self, place: &PlaceId, words: &str, bound: Option<Seconds>) -> Answer {
        self.settle_attended();
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        let Some((_, start)) = read.state.live_start() else {
            return Answer::refused(format!("the place {place} has no live agent"));
        };
        let identity = start.identity.clone();
        let Some(agent) = self.agent(&start.program) else {
            return Answer::refused(format!(
                "no agent profile runs the program {}",
                start.program
            ));
        };
        let folder = self.store.folder(place);
        let deadline = self.host.now() + bound.unwrap_or(TELL_BOUND);
        loop {
            let state = self.state_of(Some(agent.as_ref()), &folder, &identity);
            let asking = agent.asking(&folder, &identity);
            if state == AgentState::Asking || asking == Reading::Yes {
                return Answer::refused(format!(
                    "the agent {identity} is asking a person; nothing was typed"
                ));
            }
            if state == AgentState::Stopped {
                return Answer::refused(format!(
                    "the agent {identity} is not live; nothing was typed"
                ));
            }
            if state == AgentState::Waiting
                && asking == Reading::No
                && agent.started(&folder, &identity) == Reading::Yes
            {
                break;
            }
            if self.host.now() >= deadline {
                return Answer::refused(format!(
                    "the agent {identity} did not wait for input within the bound; nothing was typed"
                ));
            }
            self.host.sleep(POLL);
        }
        for _ in 0..2 {
            if let Sent::Refused { reason } = self.terminal.send(&identity, words) {
                return Answer::refused(format!("the terminal did not send the words: {reason}"));
            }
            let sent = self.host.now();
            loop {
                if let Received::Received { .. } = agent.received(&folder, &identity, words) {
                    return self.acted(Act::Tell, place);
                }
                let waited = self.host.now().saturating_sub(sent);
                if waited >= RECEIVED_WITHIN {
                    break;
                }
                self.host.sleep(POLL.min(RECEIVED_WITHIN - waited));
            }
        }
        Answer::refused(format!("the agent {identity} did not receive the words"))
    }

    /// The answer to an act taken: the place's status as it then stands.
    pub(crate) fn acted(&self, act: Act, place: &PlaceId) -> Answer {
        Answer::Acted {
            act,
            place: place.clone(),
            status: self
                .store
                .read(place)
                .map(|read| self.place_status(&read.state)),
            tidy: None,
        }
    }
}

/// A retry's words for a stage: where the stage ended short, the
/// question its agent stopped with, then the retry's words as the answer
/// (42, 49, core.27).
fn answered(stage: &StageHeld, words: Option<String>) -> Option<String> {
    let question = stage
        .end
        .as_ref()
        .filter(|_| stage.state == StageState::EndedShort)
        .and_then(|end| end.words.clone());
    match (question, words) {
        (Some(question), Some(answer)) => {
            Some(format!("The question: {question}\n\nThe answer: {answer}"))
        }
        (Some(question), None) => Some(format!("The question: {question}")),
        (None, words) => words,
    }
}

/// What a tidy did for one place.
fn only(tidy: Tidied, place: &PlaceId) -> Tidied {
    Tidied {
        removed: tidy.removed.into_iter().filter(|id| id == place).collect(),
        kept: tidy
            .kept
            .into_iter()
            .filter(|kept| &kept.place == place)
            .collect(),
        branches_kept: tidy
            .branches_kept
            .into_iter()
            .filter(|kept| &kept.place == place)
            .collect(),
        not_over: vec![],
    }
}

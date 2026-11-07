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
//!
//! While the agent is kept open, an answer, a retry with words naming its
//! stage or a tell to its place, is handed to its session as a tell is,
//! the question then the answer, and recorded on the stop-short it
//! answers in the one write that carries the stage on there under the
//! same start, begun again, starting and ending no agent; one the agent
//! does not take is refused with nothing recorded, the stage waiting on
//! its answer still (42, 47 to 50, 53 to 55).

use crate::acts::{asked, Answering};
use crate::reading::AgentReading;
use crate::start::Starts;
use crate::{Answer, Engine, Step, TELL_BOUND};
use flume_seams::*;

impl Engine {
    /// The agent of the place's current stage kept open for its answer, as
    /// `held` reads now: its stage ended short needing an answer, its time
    /// kept open not yet passed, its start live and no letting go recorded;
    /// none otherwise (42, 198).
    pub(crate) fn kept_open(&self, held: &HeldState) -> Option<Kept> {
        let kept = Kept::of(held)?;
        let open = kept.end.closed.is_none()
            && kept.start.live
            && kept
                .end
                .open_until
                .is_some_and(|until| self.host.now() < until);
        open.then_some(kept)
    }

    /// Hands `words`, the answer to the question the agent `kept` holds
    /// open stopped short with, to its session as a tell is handed, within
    /// `bound`, the question then the answer; once received, in one write,
    /// records it on the stop-short's end with `act`, when it was sent and
    /// received, and who gave it and the key it named as `answering` names
    /// them, raises its start's count of answers, carries the stage
    /// on begun under the same start, its quiet and its limit read afresh,
    /// and ends each hold on the stage, then settles the place, which reads
    /// the agent's end where a settle read its session ended once it took
    /// the answer. Refused with the tell's words, nothing recorded, where
    /// the agent does not take it, or where it was let go otherwise before
    /// the write (42, 47 to 50, 53 to 55, 96, 198; core.11).
    pub(crate) fn answer(
        &self,
        id: &PlaceId,
        kept: &Kept,
        words: &str,
        act: Act,
        bound: Option<Seconds>,
        answering: &Answering,
    ) -> Answer {
        let at = self.host.now();
        let question = kept.end.words.clone().unwrap_or_default();
        // The answer claimed on its stop-short before anything is typed, so
        // a second answer at once finds the claim and is refused with
        // nothing typed; let go when this returns where the answer was not
        // recorded, as where the agent did not take it (42, 53, 75).
        let key = EndKey::Stage {
            stage: kept.stage.clone(),
            start: kept.start.n,
            state: StageState::EndedShort,
            answered: kept.start.answered,
        };
        let _claimed = match self.claim_answer(id, &key, answering, bound.unwrap_or(TELL_BOUND)) {
            Ok(claimed) => claimed,
            Err(reason) => return Answer::refused(reason),
        };
        let handed = match self.hand(id, &kept.start, &asked(&question, words), bound) {
            Ok(handed) => handed,
            Err(reason) => return Answer::refused(reason),
        };
        let recorded = self
            .update(id, |held| {
                let Some(end) = kept.end_in(held) else {
                    return Step::Keep(false);
                };
                // The agent took the answer while it was kept open. A settle
                // that read it let go since, its session ended, as the
                // settle its own quiet starts reads it once it delivers on
                // the answer and ends, ended it after it took the answer:
                // the stage carries on under its start, live again, and the
                // settle that follows reads its end as any running stage's
                // (42, 96; design 4).
                let ended = end.closed.is_some_and(|closed| {
                    matches!(closed.by, ClosedBy::AgentEnded) && !closed.left_in_pane
                });
                let stage = &mut held.stages[kept.index];
                let name = stage.name.clone();
                let Some(start) = stage.start.as_ref().filter(|start| start.live || ended) else {
                    return Step::Keep(false);
                };
                let (n, answered) = (start.n, start.answered);
                let Some(end) = held.ends.iter_mut().rev().find(|end| {
                    end.answer.is_none()
                        && matches!(
                            &end.of,
                            EndOf::Stage { stage, start, state: StageState::EndedShort, answered: count, .. }
                                if *stage == name && *start == n && *count == answered
                        )
                }) else {
                    return Step::Keep(false);
                };
                end.answer = Some(AnswerHeld {
                    words: words.to_string(),
                    act,
                    at,
                    session: Some(handed),
                    by: answering.by.clone(),
                    key: answering.key.clone(),
                });
                end.claim = None;
                let stage = &mut held.stages[kept.index];
                if let Some(start) = stage.start.as_mut() {
                    start.live = true;
                    start.answered += 1;
                    start.first_quiet = None;
                    start.past_limit = false;
                }
                stage.state = StageState::Begun;
                stage.end = None;
                let on = HoldOn::Stage { stage: name };
                held.holds.retain(|hold| hold.on != on);
                Step::Write(true)
            })
            .unwrap_or(false);
        if !recorded {
            return Answer::refused(format!(
                "the stage {} of {id} moved on before its answer was recorded",
                kept.stage
            ));
        }
        self.settle_place(id);
        self.acted(act, id)
    }

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

/// The name of the place's current stage where it ended short waiting on
/// an answer: needing one, or having found work that sends the place back
/// nowhere, from a stage before the kind's last or from a last stage that
/// is its kind's own fix round (42, 131).
pub(crate) fn waits_on_answer(held: &HeldState) -> Option<&str> {
    let index = held.current?;
    let stage = &held.stages[index];
    if held.phase != Phase::Live || stage.state != StageState::EndedShort {
        return None;
    }
    let found = stage.end.as_ref().and_then(|end| end.needs) == Some(StopKind::FoundWork);
    (!found || !crate::submit::found_goes_back(held, index)).then_some(stage.name.as_str())
}

/// When the start `start` of the stage at `index` last took an answer in
/// its session, read received, as the ends `held` records give it; none
/// where it took none (42, 118).
pub(crate) fn last_answer(held: &HeldState, index: usize, start: &StartHeld) -> Option<Seconds> {
    let name = &held.stages[index].name;
    held.ends
        .iter()
        .filter(|end| matches!(&end.of, EndOf::Stage { stage, start: n, .. } if stage == name && *n == start.n))
        .filter_map(|end| end.answer.as_ref()?.session)
        .map(|handed| handed.received_at)
        .max()
}

/// The words the start `start` of the place's stage was given and each
/// answer it took in its session, the question then the answer, in order,
/// as a fresh start flume makes on its own after it carries them after
/// flume's own words; none where it was given none and took none (42, 146,
/// 161).
pub(crate) fn told(held: &HeldState, start: &StartHeld) -> Option<String> {
    let Some(index) = held.index_of(&start.identity) else {
        return start.words.clone();
    };
    let name = &held.stages[index].name;
    let answers = held.ends.iter().filter_map(|end| match &end.of {
        EndOf::Stage {
            stage,
            start: n,
            end: ended,
            ..
        } if stage == name && *n == start.n => {
            let answer = end
                .answer
                .as_ref()
                .filter(|answer| answer.session.is_some())?;
            Some(asked(
                ended.words.as_deref().unwrap_or_default(),
                &answer.words,
            ))
        }
        _ => None,
    });
    let parts: Vec<String> = start.words.clone().into_iter().chain(answers).collect();
    (!parts.is_empty()).then(|| parts.join("\n\n"))
}

/// A place's current stage ended short needing an answer whose end gives
/// the time its agent is kept open until, as one read of its held state
/// gives it.
pub(crate) struct Kept {
    index: usize,
    stage: String,
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
            stage: stage.name.clone(),
            start: stage.start.clone()?,
            end,
        })
    }

    /// The index of the stage whose agent is kept open.
    pub(crate) fn index(&self) -> usize {
        self.index
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

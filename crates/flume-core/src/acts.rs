//! The caller's acts on a place after its launch (47 to 55, 109, 114),
//! and the prompt a stage would be sent (29). Each ends an agent only
//! through the terminal's close, only when it is quiet or the caller
//! forced it, and answers an end the agent did not take as a failure
//! (31); a retry ends none whose stage it started or began under, so it
//! never ends a working session (35, 50).

use crate::base::base_place;
use crate::held::refresh_waiting;
use crate::launch::and_list;
use crate::open_sessions::waits_on_answer;
use crate::start::{Start, Starts};
use crate::status::waiting_on;
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

/// What an answer names beside its words: the key of the stop-short it
/// answers, taken only while the place waits on that stop-short, and who
/// gives it, as the answering command's recipient names it; each none
/// where the caller names none (42, 49, 53).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Answering {
    pub key: Option<EndKey>,
    pub by: Option<String>,
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
        let declared = &held.place.kind.stages[index];
        let agent = self
            .resolve(&held.place, declared)
            .and_then(|resolved| self.agent_for(&resolved, stage))
            .ok();
        let head = self
            .history
            .head(&held.place.repository, &held.place.branch)
            .unwrap_or_else(|| held.place.base_revision.clone());
        let group = self.group_at(&held, index, &head);
        let listed = self.may_name(&held);
        match self.fill(
            &held,
            index,
            &identity,
            None,
            agent.as_deref(),
            group.as_ref().map(Option::as_ref),
            &listed,
        ) {
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
            if let Closed::Refused { reason } = self.agent_on(start).close() {
                self.failed_end(id, &identity, &reason);
                return Err(format!("the agent {identity} did not end: {reason}"));
            }
        }
        self.end_start(
            id,
            &identity,
            |start, state| {
                start.live = false;
                if *state == StageState::Starting {
                    *state = StageState::NeverBegun;
                } else if state.running() {
                    *state = StageState::Stopped;
                }
            },
            starting,
        );
        Ok(())
    }

    /// Why a retry leaves the place's agent be: it is live, and the stage
    /// it runs started or began under it, a stage read never begun whose
    /// agent said so since among them, so ending it would end a working
    /// session. The refusal names the agent and the stop that ends it
    /// first; none where the agent is not so (31, 35, 50, 51).
    fn working(&self, id: &PlaceId) -> Option<String> {
        self.begun_after(id);
        let read = self.store.read(id)?;
        let (index, start) = read.state.live_start()?;
        let stage = &read.state.stages[index];
        let what = match stage.state {
            StageState::Started => "said it started",
            StageState::Begun => "has begun",
            _ => return None,
        };
        Some(format!(
            "the agent {} of the stage {} is live and {what}, and a retry never ends a working session; end it first with flume stop {id}, --force where it is not quiet, then retry",
            start.identity, stage.name
        ))
    }

    /// Starts the stage a retry names with a fresh agent and the retry's
    /// words, after the question of the stop-short the stage ended with,
    /// the stages after it then running again in order; or, naming
    /// the submission, submits the place's head again and starts no
    /// agent. The place's agent is ended first, but one whose stage
    /// started or began under it, which refuses the retry, forced or
    /// not, naming the agent and its stop (49, 50, 12, 35). A retry of a
    /// conversation reported lost ends no agent, forced or not: its stage
    /// waits to carry the lost start's conversation, and its start opens
    /// once no process runs it, its answer naming each that does (30, 49,
    /// 77, 207); so does a retry of a conversation whose agent kept open
    /// for its answer was let go ended, lost with its terminal or its
    /// host. A retry with words naming a stage whose agent is kept open
    /// for its answer is that answer, handed to its session, ending and
    /// starting no agent (42, 49, 50, 198). It names no key and no one
    /// who gives it: [`Engine::retry_answering`] with neither.
    pub fn retry(
        &self,
        place: &PlaceId,
        target: Retry,
        words: Option<String>,
        force: bool,
    ) -> Answer {
        self.retry_answering(place, target, words, force, &Answering::default())
    }

    /// A retry, as [`Engine::retry`] takes one, whose words answer the
    /// stop-short the place waits on as `answering` names it: one naming a
    /// key is taken only while the place waits on the stop-short the key
    /// names, with words, naming that stop-short's stage or the current
    /// one, and is otherwise refused before anything is ended, started or
    /// typed, naming the answer that stop-short took, who gave it and
    /// when, or the move the place waits on now. A retry with words of the
    /// stage whose stop-short the place waits on records on that
    /// stop-short's end its words, who gave them, the key it named, none
    /// where it named none, and when, whether they reach the agent kept
    /// open for them or a fresh agent (42, 49, 50, 53, 198).
    pub fn retry_answering(
        &self,
        place: &PlaceId,
        target: Retry,
        words: Option<String>,
        force: bool,
        answering: &Answering,
    ) -> Answer {
        self.settle_attended();
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        let held = read.state;
        if held.phase != Phase::Live {
            return Answer::refused(format!("the work of the place {place} is over"));
        }
        // A base place has no stage, and its submission waits on the
        // places on its branch, which land there first (49, 203).
        if base_place(&held.place.kind) {
            if target != Retry::Submission {
                return Answer::refused(format!(
                    "the base place {place} has no stage; a retry names its submission"
                ));
            }
            let on_branch = self.on_branch(&held.place);
            if !on_branch.is_empty() {
                return Answer::refused(format!(
                    "the base place {place} is submitted once no place on its branch {} is live, and {} {} (203)",
                    held.place.branch,
                    and_list(on_branch.iter().map(ToString::to_string).collect()),
                    if on_branch.len() == 1 { "is" } else { "are" }
                ));
            }
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
        // An answer naming its key is taken once, while the place waits on
        // the stop-short it names, before anything is ended, started or
        // typed (42, 49).
        if let Some(key) = &answering.key {
            let stage =
                Some(index.map_or("the submission", |index| held.stages[index].name.as_str()));
            let on_branch = self.on_branch_count(&held);
            if let Err(reason) = keyed(&held, on_branch, place, key, stage, words.is_some()) {
                return Answer::refused(reason);
            }
        }
        // An answer to the agent kept open for it: handed to its session,
        // which carries the stage on (42, 49, 198).
        if let (Some(index), Some(words)) = (index, words.as_deref()) {
            if let Some(kept) = self.kept_open(&held).filter(|kept| kept.index() == index) {
                return self.answer(place, &kept, words, Act::Retry, None, answering);
            }
        }
        // An answer naming its key, carried by a fresh agent: claimed on
        // its stop-short before any agent is ended or started, so a second
        // answer at once is refused, and let go where it is not recorded
        // (42, 49, 75).
        let _claimed = match &answering.key {
            Some(key) => match self.claim_answer(place, key, answering, TELL_BOUND) {
                Ok(claimed) => Some(claimed),
                Err(reason) => return Answer::refused(reason),
            },
            None => None,
        };
        if let Some(reason) = self.working(place) {
            return Answer::refused(reason);
        }
        // A conversation reported lost, or whose agent kept open for its
        // answer was lost with its terminal or its host: its start carries
        // the lost start's conversation, and no agent of the stage is
        // ended, an agent the terminal brought back in its pane, and any
        // process that runs its session, left as they are (30, 49, 77,
        // 198; core.10).
        let lost = index
            .filter(|&index| {
                held.place.kind.stages[index].conversation && carries_lost(&held.stages[index])
            })
            .and_then(|index| held.stages[index].start.as_ref())
            .map(|start| start.identity.clone());
        if lost.is_none() {
            if let Err(reason) = self.end_agent(place, force) {
                return Answer::refused(reason);
            }
        }
        let ends = match index {
            Some(index) => HoldOn::Stage {
                stage: held.stages[index].name.clone(),
            },
            None => HoldOn::Submission,
        };
        // The answer a fresh agent will carry, recorded on the stop-short
        // it answers (42, 49).
        let answer = words.as_ref().and_then(|words| {
            let key = waited_key(&held).filter(|_| index.is_some() && held.current == index)?;
            Some((
                key,
                AnswerHeld {
                    words: words.clone(),
                    act: Act::Retry,
                    at: self.host.now(),
                    session: None,
                    by: answering.by.clone(),
                    key: answering.key.clone(),
                },
            ))
        });
        self.update(place, |held| {
            held.holds
                .retain(|hold| hold.on != ends && hold.on != HoldOn::Waiting);
            if let Some((key, answer)) = &answer {
                if let Some(end) = held
                    .ends
                    .iter_mut()
                    .find(|end| end.answer.is_none() && end.of.end_key() == *key)
                {
                    end.answer = Some(answer.clone());
                    end.claim = None;
                }
            }
            Step::Write(())
        });
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
        let room = lost.as_ref().map(|_| self.host.room());
        let order = lost.as_ref().map(|_| self.queue().next_order());
        self.update(place, |held| {
            match (&lost, order) {
                // A lost conversation's start waits to carry its
                // conversation, as a spent agent's does (49, 179).
                (Some(lost), Some(order)) => {
                    let stage = &mut held.stages[index];
                    stage.state = StageState::WaitingForRoom;
                    stage.room = Some(RoomWait {
                        order,
                        words: words.clone(),
                        carry: Some(CarryFrom {
                            start: lost.clone(),
                            cause: CarryCause::Lost,
                            carry: true,
                            why: None,
                        }),
                        ..RoomWait::default()
                    });
                    held.current = Some(index);
                }
                // Any other retry's start carries no conversation, though
                // the stage waits after an agent stopped at its account's
                // usage limit (30, 146).
                _ => {
                    if let Some(room) = held.stages[index].room.as_mut() {
                        room.carry = None;
                    }
                }
            }
            for stage in held.stages.iter_mut().skip(index + 1) {
                stage.state = StageState::NotReached;
                stage.made_from = None;
            }
            held.submission.due = false;
            held.submission.waiting = None;
            Step::Write(())
        });
        drop(room);
        match self.start_stage(place, index, words) {
            Start::Opened(_) | Start::Held(_) | Start::NeverBegun(_) | Start::WaitingForRoom => {
                self.acted(Act::Retry, place)
            }
            Start::Refused(part) => Answer::refused(format!(
                "the stage {}'s prompt asks for {}, which has nothing to fill it",
                held.stages[index].name,
                part.name()
            )),
            Start::AgentLive => Answer::refused(format!("an agent of the place {place} is live")),
            Start::NotOpened(reason) => {
                Answer::refused(format!("the terminal did not open the agent: {reason}"))
            }
            Start::Unresolved(reason) => Answer::refused(reason),
            Start::NoPlace => Answer::refused(format!("no place {place} stands on this host")),
        }
    }

    /// Puts the place's stage, or its submission, on hold with the
    /// caller's words: taken for a stage not started since the place's
    /// current stage began, one waiting for room taken out of the queue,
    /// for a repeating stage whose repetition runs as a hold on its next
    /// repetition, the running one going on (142), or for a submission
    /// not made; refused naming why for any other stage started, a
    /// submission made and undecided, a hold already put, or a place
    /// whose work is over. Only a retry naming it ends it (8, 47, 49,
    /// 130; core.24).
    ///
    /// Or holds the move the place waits on its caller for, with the
    /// caller's words and the `bound` it holds for, none holding until the
    /// move is made: a later hold on the move replaces the words, the
    /// bound and since when, the place is not stalled while it holds, and
    /// the move made, a retry or a drop ends it; refused on a place that
    /// waits on no move of its caller's. A bound is the waiting move's
    /// alone (47, 49, 52, 130; core.24).
    pub fn hold(&self, place: &PlaceId, on: HoldOn, words: &str, bound: Option<Seconds>) -> Answer {
        self.settle_attended();
        let now = self.host.now();
        let refused = self.update(place, |held| {
            if held.phase != Phase::Live {
                return Step::Keep(Some(format!("the work of the place {place} is over")));
            }
            if on == HoldOn::Waiting {
                let on_branch = self.on_branch_count(held);
                return hold_waiting(held, on_branch, place, words, bound, now);
            }
            if bound.is_some() {
                return Step::Keep(Some(
                    "a bound is given only to a hold on the move the place waits on".into(),
                ));
            }
            let index = match &on {
                HoldOn::Stage { stage } => {
                    match held.stages.iter().position(|held| &held.name == stage) {
                        Some(index) => Some(index),
                        None => {
                            return Step::Keep(Some(format!(
                                "the place {place} has no stage {stage}"
                            )))
                        }
                    }
                }
                HoldOn::Submission if !held.place.kind.lands => {
                    return Step::Keep(Some(format!(
                        "the place {place}'s kind does not land, so it has no submission"
                    )))
                }
                HoldOn::Submission | HoldOn::Waiting => None,
            };
            if let Some(hold) = held.holds.iter().find(|hold| hold.on == on) {
                let what = match &on {
                    HoldOn::Stage { stage } => format!("the stage {stage}"),
                    HoldOn::Submission => "the submission".into(),
                    HoldOn::Waiting => "the move it waits on".into(),
                };
                return Step::Keep(Some(format!(
                    "{what} of the place {place} is on hold already, since {}",
                    hold.since
                )));
            }
            match index {
                Some(index) => {
                    let stage = &mut held.stages[index];
                    let unstarted = matches!(
                        stage.state,
                        StageState::NotReached
                            | StageState::OnHold
                            | StageState::WaitingForRoom
                            | StageState::WaitingOnReadiness
                    );
                    let repeats = held.place.kind.stages[index].repeats;
                    let next_repetition = repeats
                        && (stage.state.running() || stage.state == StageState::Repeating);
                    if held.current == Some(index) && !unstarted && !next_repetition {
                        return Step::Keep(Some(format!(
                            "the stage {} of the place {place} has started; a hold is put on a stage not started, or on a repeating stage's next repetition while one runs",
                            stage.name
                        )));
                    }
                    if stage.state == StageState::WaitingForRoom {
                        stage.state = StageState::OnHold;
                        stage.room = None;
                    }
                }
                None => {
                    let submission = &held.submission;
                    if submission.submits > 0 && submission.verdict.is_none() {
                        return Step::Keep(Some(format!(
                            "the submission of the place {place} is made and not yet decided"
                        )));
                    }
                }
            }
            held.holds.push(Hold {
                on: on.clone(),
                words: Some(words.to_string()),
                since: now,
                bound: None,
                lapsed: None,
            });
            Step::Write(None)
        });
        match refused {
            None => Answer::refused(format!("no place {place} stands on this host")),
            Some(Some(reason)) => Answer::refused(reason),
            Some(None) => self.acted(Act::Hold, place),
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
    /// and the place tidied (52, 63); a base place's refused while a
    /// place on its branch is live, since those land there and are
    /// dropped first (204).
    pub fn drop_place(&self, place: &PlaceId, force: bool) -> Answer {
        if let Some(read) = self.store.read(place) {
            let held = read.state;
            if base_place(&held.place.kind) && held.phase == Phase::Live {
                let on_branch = self.on_branch(&held.place);
                if !on_branch.is_empty() {
                    return Answer::refused(format!(
                        "the base place {place}'s branch {} is the base of {}, which {} live; drop {} first (52, 204)",
                        held.place.branch,
                        and_list(on_branch.iter().map(ToString::to_string).collect()),
                        if on_branch.len() == 1 { "is" } else { "are" },
                        if on_branch.len() == 1 { "it" } else { "them" }
                    ));
                }
            }
        }
        let live = self
            .store
            .read(place)
            .is_some_and(|read| !matches!(read.state.phase, Phase::Tidied { .. }));
        let tidied = if live { vec![] } else { self.tidied_of(place) };
        if !tidied.is_empty() {
            let documents: Vec<HeldState> = self.recorded(
                tidied
                    .iter()
                    .filter_map(|id| self.store.read(id))
                    .map(|read| read.state)
                    .collect(),
            );
            let owing = self.owing();
            let mut pending: Vec<String> = vec![];
            for held in &documents {
                for recipient in held.pending_for(&owing) {
                    if !pending.contains(&recipient) {
                        pending.push(recipient);
                    }
                }
            }
            if !pending.is_empty() {
                return Answer::refused(format!(
                    "the tidied place {place} stays while its ends stand pending for {}: each \
                     confirms them by flume confirm --recipient <recipient> <key>…, or by flume \
                     inbox --recipient <recipient> --confirm naming the place, and the next \
                     settle removes it (62; weir I9)",
                    and_list(pending)
                ));
            }
            let removed = documents
                .iter()
                .filter(|held| {
                    let removed = self.remove_held(&held.place.id);
                    if removed {
                        self.let_go_of(held);
                    }
                    removed
                })
                .map(|held| held.place.id.clone())
                .collect();
            return Answer::Acted {
                act: Act::Drop,
                place: place.clone(),
                status: None,
                tidy: Some(Tidied {
                    removed,
                    ..Tidied::default()
                }),
            };
        }
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
            if let Closed::Refused { reason } = self.agent_on(&start).close() {
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
    /// once it said it started and waits, received or sent once more;
    /// refused with nothing typed when the place has no live agent, its
    /// agent is asking, or the words are not delivered within the bound.
    /// A tell to a place whose stage ended short needing an answer is that
    /// answer while its agent is kept open for it, carrying the stage on
    /// in its session, and is refused, naming the retry that answers it,
    /// once its agent is not (42, 53 to 55, 198). It names no key and no
    /// one who gives it: [`Engine::tell_answering`] with neither.
    pub fn tell(&self, place: &PlaceId, words: &str, bound: Option<Seconds>) -> Answer {
        self.tell_answering(place, words, bound, &Answering::default())
    }

    /// A tell, as [`Engine::tell`] takes one, whose words, where they
    /// answer the agent kept open, record who gave them and the key they
    /// named, none where they named none, on the stop-short they answer.
    /// One naming a key is taken only while the place waits on the
    /// stop-short the key names, and is otherwise refused with nothing
    /// typed, naming the answer that stop-short took, who gave it and
    /// when, or the move the place waits on now (42, 53, 198).
    pub fn tell_answering(
        &self,
        place: &PlaceId,
        words: &str,
        bound: Option<Seconds>,
        answering: &Answering,
    ) -> Answer {
        self.settle_attended();
        let Some(read) = self.store.read(place) else {
            return Answer::refused(format!("no place {place} stands on this host"));
        };
        if let Some(key) = &answering.key {
            let on_branch = self.on_branch_count(&read.state);
            if let Err(reason) = keyed(&read.state, on_branch, place, key, None, true) {
                return Answer::refused(reason);
            }
        }
        if let Some(stage) = waits_on_answer(&read.state) {
            return match self.kept_open(&read.state) {
                Some(kept) => self.answer(place, &kept, words, Act::Message, bound, answering),
                None => Answer::refused(format!(
                    "the stage {stage} of {place} waits on an answer and its agent is no longer kept open; give the answer as a retry: flume retry {place} --words <answer>"
                )),
            };
        }
        let Some((_, start)) = read.state.live_start() else {
            return Answer::refused(format!("the place {place} has no live agent"));
        };
        match self.hand(place, start, words, bound) {
            Ok(_) => self.acted(Act::Message, place),
            Err(reason) => Answer::refused(reason),
        }
    }

    /// Hands the agent of `start` `words` as a prompt, as a tell hands
    /// them: once it said it started, waits and asks nothing, within the
    /// bound, ten minutes where none is given, its input cleared by the
    /// terminal's send, and read received within 60 seconds of each
    /// sending or sent once more. When it was last sent and read
    /// received, or the tell's words saying why nothing more was typed
    /// (37, 38, 53, 54; core.11).
    pub(crate) fn hand(
        &self,
        place: &PlaceId,
        start: &StartHeld,
        words: &str,
        bound: Option<Seconds>,
    ) -> Result<Handed, String> {
        let identity = &start.identity;
        let Some(agent) = self.agent(&start.program) else {
            return Err(format!(
                "no agent profile runs the program {}",
                start.program
            ));
        };
        let folder = self.store.folder(place);
        let deadline = self.host.now() + bound.unwrap_or(TELL_BOUND);
        loop {
            let state = self.state_of(Some(agent.as_ref()), &folder, start);
            let asking = agent.asking(&folder, identity);
            if state == AgentState::Asking || asking == Reading::Yes {
                return Err(format!(
                    "the agent {identity} is asking a person; nothing was typed"
                ));
            }
            if state == AgentState::Stopped {
                return Err(format!(
                    "the agent {identity} is not live; nothing was typed"
                ));
            }
            if state == AgentState::Waiting
                && asking == Reading::No
                && agent.started(&folder, identity) == Reading::Yes
            {
                break;
            }
            if self.host.now() >= deadline {
                return Err(format!(
                    "the agent {identity} did not wait for input within the bound; nothing was typed"
                ));
            }
            self.host.sleep(POLL);
        }
        for _ in 0..2 {
            if let Sent::Refused { reason } = self.agent_on(start).send(words) {
                return Err(format!("the terminal did not send the words: {reason}"));
            }
            let sent_at = self.host.now();
            loop {
                if let Received::Received { .. } = agent.received(&folder, identity, words) {
                    return Ok(Handed {
                        sent_at,
                        received_at: self.host.now(),
                    });
                }
                let waited = self.host.now().saturating_sub(sent_at);
                if waited >= RECEIVED_WITHIN {
                    break;
                }
                self.host.sleep(POLL.min(RECEIVED_WITHIN - waited));
            }
        }
        Err(format!("the agent {identity} did not receive the words"))
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
        (Some(question), Some(answer)) => Some(asked(&question, &answer)),
        (Some(question), None) => Some(format!("The question: {question}")),
        (None, words) => words,
    }
}

/// The key of the stop-short the place waits on its caller's answer to,
/// as its current stage's start records it; none where it waits on none
/// (42, 43, 131).
pub(crate) fn waited_key(held: &HeldState) -> Option<EndKey> {
    let name = waits_on_answer(held)?;
    let start = held.stages[held.current?].start.as_ref()?;
    Some(EndKey::Stage {
        stage: name.to_string(),
        start: start.n,
        state: StageState::EndedShort,
        answered: start.answered,
    })
}

/// Whether an answer naming `key` is taken: only while the place waits on
/// the stop-short `key` names and no answer was recorded on it, where the
/// answer names a stage or the submission, `stage`, that stop-short's, and
/// where it gives words; otherwise why not, naming the answer that
/// stop-short took, who gave it and when, or the move the place waits on
/// now (42, 49, 53).
fn keyed(
    held: &HeldState,
    on_branch: Option<usize>,
    place: &PlaceId,
    key: &EndKey,
    stage: Option<&str>,
    words: bool,
) -> Result<(), String> {
    let taken = held
        .ends
        .iter()
        .find(|end| end.of.end_key() == *key)
        .and_then(|end| end.answer.as_ref());
    if taken.is_some() || waited_key(held).as_ref() != Some(key) {
        let now = match waiting_on(held, on_branch) {
            Some(waiting) => format!("it waits on {} now", waiting.move_words()),
            None => "it waits on no move of its caller's now".to_string(),
        };
        return Err(match taken {
            Some(answer) => format!(
                "the stop-short {key} of the place {place} was answered by {} at {}: {}; an answer is taken once, and {now}",
                answer
                    .by
                    .as_deref()
                    .unwrap_or("a caller that named no recipient"),
                answer.at,
                answer.words
            ),
            None => format!("the place {place} waits on no answer to the stop-short {key}: {now}"),
        });
    }
    let EndKey::Stage { stage: asked, .. } = key else {
        unreachable!("the stop-short a place waits on is a stage's end");
    };
    if let Some(stage) = stage.filter(|stage| stage != asked) {
        return Err(format!(
            "the stop-short {key} of the place {place} is {asked}'s, and the retry names {stage}: retry {asked} with its answer"
        ));
    }
    if !words {
        return Err(format!(
            "an answer to the stop-short {key} of the place {place} gives its words: retry with --words <answer>"
        ));
    }
    Ok(())
}

/// How long past the hand's own bound a claim on a stop-short's answer
/// holds, the two sendings' reading of receipt among it, before a later
/// answer may take the stop-short (38, 53, 75).
const CLAIM_PAST_HAND: Seconds = 5 * 60;

/// A claim on the answer to a stop-short, taken by compare-and-set on its
/// end before the answer's words are typed or a fresh agent is started to
/// carry them; let go when dropped where the answer was not recorded,
/// which clears it (42, 53, 75).
pub(crate) struct Claimed<'a> {
    engine: &'a Engine,
    place: PlaceId,
    key: EndKey,
    claim: AnswerClaim,
}

impl Drop for Claimed<'_> {
    fn drop(&mut self) {
        let Claimed {
            engine,
            place,
            key,
            claim,
        } = self;
        engine.update(place, |held| {
            match held
                .ends
                .iter_mut()
                .find(|end| end.of.end_key() == *key && end.claim.as_ref() == Some(claim))
            {
                Some(end) => {
                    end.claim = None;
                    Step::Write(())
                }
                None => Step::Keep(()),
            }
        });
    }
}

impl Engine {
    /// Claims the answer to the stop-short `key`, the one the place waits
    /// on, for the answer `answering` gives, held for `bound` and
    /// [`CLAIM_PAST_HAND`] past it, in one write taken only while the
    /// stop-short is unanswered and no other answer's claim holds; refused
    /// otherwise, as [`keyed`] refuses, or naming the answer being taken,
    /// who gives it and since when, with nothing typed (42, 53, 75;
    /// core.54).
    pub(crate) fn claim_answer(
        &self,
        place: &PlaceId,
        key: &EndKey,
        answering: &Answering,
        bound: Seconds,
    ) -> Result<Claimed<'_>, String> {
        let now = self.host.now();
        let claim = AnswerClaim {
            by: answering.by.clone(),
            key: answering.key.clone(),
            at: now,
            until: now + bound + CLAIM_PAST_HAND,
        };
        let on_branch = self
            .store
            .read(place)
            .and_then(|read| self.on_branch_count(&read.state));
        let claimed = self.update(place, |held| {
            if let Err(reason) = keyed(held, on_branch, place, key, None, true) {
                return Step::Keep(Err(reason));
            }
            let Some(end) = held.ends.iter_mut().find(|end| end.of.end_key() == *key) else {
                return Step::Keep(Err(format!(
                    "the place {place} holds no stop-short {key}"
                )));
            };
            if let Some(other) = end.claim.as_ref().filter(|other| now < other.until) {
                return Step::Keep(Err(format!(
                    "an answer to the stop-short {key} of the place {place} is being taken, given by {} at {}: an answer is taken once, and nothing was typed",
                    other
                        .by
                        .as_deref()
                        .unwrap_or("a caller that named no recipient"),
                    other.at
                )));
            }
            end.claim = Some(claim.clone());
            Step::Write(Ok(()))
        });
        match claimed {
            Some(Ok(())) => Ok(Claimed {
                engine: self,
                place: place.clone(),
                key: key.clone(),
                claim,
            }),
            Some(Err(reason)) => Err(reason),
            None => Err(format!(
                "the place {place} moved under the answer to the stop-short {key}; nothing was typed"
            )),
        }
    }
}

/// A question a stop-short asked and its answer, as a retry's prompt and
/// an answer handed to a session give them (42, 49).
pub(crate) fn asked(question: &str, answer: &str) -> String {
    format!("The question: {question}\n\nThe answer: {answer}")
}

/// Whether a retry of a conversation's stage carries its last start's
/// conversation as a lost one's: the stage reported lost, or its agent
/// kept open for its answer let go ended, or an agent the terminal brought
/// back bare left in its pane, either lost with its terminal or its host
/// (49, 77, 179, 198).
fn carries_lost(stage: &StageHeld) -> bool {
    match stage.state {
        StageState::Lost => true,
        StageState::EndedShort => stage
            .end
            .as_ref()
            .and_then(|end| end.closed)
            .is_some_and(|closed| closed.by == ClosedBy::AgentEnded || closed.left_in_pane),
        _ => false,
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
        terminal: tidy
            .terminal
            .into_iter()
            .filter(|tidied| &tidied.place == place)
            .collect(),
    }
}

/// Holds the move the place waits on its caller for with `words`, for
/// `bound` where given, in place of any hold on it before, lapsed or not,
/// the place no longer stalled; refused, naming the place, where it waits
/// on no move of its caller's (130).
fn hold_waiting(
    held: &mut HeldState,
    on_branch: Option<usize>,
    place: &PlaceId,
    words: &str,
    bound: Option<Seconds>,
    now: Seconds,
) -> Step<Option<String>> {
    if waiting_on(held, on_branch).is_none() {
        return Step::Keep(Some(format!(
            "the place {place} waits on no move of its caller's"
        )));
    }
    refresh_waiting(held, now, on_branch);
    held.holds.retain(|hold| hold.on != HoldOn::Waiting);
    held.holds.push(Hold {
        on: HoldOn::Waiting,
        words: Some(words.to_string()),
        since: now,
        bound,
        lapsed: None,
    });
    if let Some(waiting) = &mut held.waiting {
        waiting.stalled = false;
    }
    Step::Write(None)
}

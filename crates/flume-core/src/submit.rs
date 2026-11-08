//! A landing kind's submission (63, 111 to 113, 124, 125, 133): made at
//! the place's head once its last stage delivered and its agent ended,
//! unless the launch holds it or the work is over, with no wait held on
//! it. Its outcomes are collected at the settles that follow through the
//! merge queue's inbox, or read from submit's answer where the merge
//! queue decided the submission as it took it in and delivered the
//! outcome with that answer, and acted on once each: a landing makes the
//! work over, a failure or a conflict starts the kind's fix round, as found
//! work from its last stage does where the fix round is a stage before it,
//! every going-back counted together up to the kind's bound. A withdrawal, and a landing before flume
//! submits, are read from the merge queue's record of the branch, never
//! one it held before the place's launch or before flume's submit (3,
//! 99); an open submission's base is driven one round per command, and
//! never submitted anew. Where the merge queue answers a submit with a
//! request it holds and no entry, the submission is open and undecided
//! until an intake takes the request in, its base driven as an open
//! submission's is; the entry taken in from it is followed in the
//! merge queue's record, which gives its outcome where the merge queue
//! delivered it to the request rather than the place, acted on once
//! whichever of the record and the inbox gives it first (124, 133, C.6).
//! A branch the merge queue could not read is passed over, named in
//! status, and decides nothing of the place unless it is the place's
//! base, where its submission is: the record and the inbox of every other
//! branch are read and acted on still (15, 68, 99, 133).
//!
//! A base place's submission is made only by its caller's retry, its
//! branch submitted by name at the revision where the merge queue says
//! it lives, from the repository with no worktree; its outcomes are read
//! from the merge queue's record of its branch alone, never an inbox, and
//! a failure or a conflict starts nothing: the place waits on its caller
//! with the merge queue's words (47, 111, 112, 117, 133, 203; core.6).

use crate::base::base_place;
use crate::fixes::{decide, Found};
use crate::start::Starts;
use crate::{Engine, Step};
use flume_seams::*;

impl Engine {
    /// The submission's step for a live place; true when it changed
    /// something.
    pub(crate) fn submission(&self, held: &HeldState) -> bool {
        let place = &held.place;
        if !place.kind.lands {
            return false;
        }
        let Some(queue) = self.bound(place).ok().and_then(|bound| bound.merge_queue) else {
            return false;
        };
        let id = &place.id;
        let ours = &held.submission;
        let base = base_place(&place.kind);
        if ours.submits > 0 && !base && self.collect(id, queue.as_ref(), &place.place_ref()) {
            return true;
        }
        match own_read(
            held,
            queue.read_submission(&place.repository, &place.branch),
        ) {
            SubmissionRead::CouldNotRead { .. } => false,
            SubmissionRead::Found { submission } => match &submission.state {
                SubmissionState::Landed { commit } if submission.target == place.base => {
                    self.over(
                        id,
                        Over::Landed {
                            commit: commit.clone(),
                        },
                    );
                    true
                }
                SubmissionState::Withdrawn
                    if is_ours(&held.submission, &submission) && ours.waiting.is_none() =>
                {
                    self.verdict(id, Verdict::Withdrawn);
                    true
                }
                // A base place's failure or conflict is read from the
                // record, since no inbox gives an outcome of a submission
                // by name (133, 203).
                SubmissionState::Failed { .. } | SubmissionState::Conflict { .. }
                    if base && is_ours(&held.submission, &submission) && ours.verdict.is_none() =>
                {
                    match recorded(queue.as_ref(), place, &submission.state) {
                        Some(outcome) => {
                            self.outcome(id, outcome);
                            true
                        }
                        None => false,
                    }
                }
                SubmissionState::Failed { .. } | SubmissionState::Conflict { .. }
                    if submission.request.is_some()
                        && is_ours(&held.submission, &submission)
                        && ours.verdict.is_none() =>
                {
                    self.taken_in(id, &submission);
                    match recorded(queue.as_ref(), place, &submission.state) {
                        Some(outcome) => {
                            self.outcome(id, outcome);
                            true
                        }
                        None => false,
                    }
                }
                state if state.open() => {
                    let taken = self.taken_in(id, &submission);
                    let stepped =
                        self.step_base(queue.as_ref(), &place.repository, &submission.target);
                    taken || stepped
                }
                _ => self.due(held, queue.as_ref()),
            },
            SubmissionRead::None => self.due(held, queue.as_ref()),
        }
    }

    /// Collects the place's pending outcomes and acts on each once, in
    /// the order given; an inbox that cannot be read is named in status
    /// and acts on nothing, and a branch it passed over is named in status
    /// and decides nothing of the outcomes it gave (133, 68). The outcome
    /// of a request's entry, which the merge queue's record gives as well,
    /// is acted on only where neither gave it since flume's submit (124,
    /// C.6). True when it changed something.
    fn collect(&self, id: &PlaceId, queue: &dyn MergeQueue, place: &PlaceRef) -> bool {
        match queue.inbox(place) {
            Inbox::CouldNotRead { reason } => {
                self.update(id, |held| {
                    held.submission.unread = Some(reason.clone());
                    Step::Write(())
                });
                false
            }
            Inbox::Outcomes {
                outcomes,
                passed_over,
            } => {
                let mut changed = self
                    .update(id, |held| {
                        let ours = &mut held.submission;
                        let was = ours.unread.take();
                        let moved = ours.passed_over != passed_over;
                        ours.passed_over = passed_over.clone();
                        Step::Write(was.is_some() || moved)
                    })
                    .unwrap_or(false);
                for outcome in outcomes {
                    let acted = self.store.read(id).is_some_and(|read| {
                        let ours = &read.state.submission;
                        ours.request.is_some() && ours.verdict.is_some()
                    });
                    if !acted {
                        self.outcome(id, outcome);
                    }
                    changed = true;
                }
                changed
            }
        }
    }

    /// The entry an intake took the place's request in as, recorded as
    /// the place's submission once read; true when it was recorded now
    /// (124).
    fn taken_in(&self, id: &PlaceId, submission: &Submission) -> bool {
        self.update(id, |held| {
            let ours = &mut held.submission;
            if ours.id.is_some() || ours.request.is_none() || submission.request != ours.request {
                return Step::Keep(false);
            }
            ours.id = Some(submission.id.clone());
            Step::Write(true)
        })
        .unwrap_or(false)
    }

    /// Drives `base`'s queue one round, once per repository and base in
    /// this command however many places wait on it (125). True when the
    /// round ran, so the place is read again.
    fn step_base(&self, queue: &dyn MergeQueue, repository: &str, base: &str) -> bool {
        let first = crate::lock(&self.stepped).insert((repository.to_string(), base.to_string()));
        if !first {
            return false;
        }
        matches!(queue.step(repository, base), Stepped::Stepped)
    }

    /// A due submission is made unless held, with no agent of the place
    /// live (47, 111). A request the merge queue holds for the place, not
    /// yet taken in, is open and undecided, its base driven one round as
    /// an open submission's is, so an intake there takes it in (124, 125).
    fn due(&self, held: &HeldState, queue: &dyn MergeQueue) -> bool {
        let ours = &held.submission;
        let place = &held.place;
        if ours.due && held.submission_hold().is_none() && held.live_start().is_none() {
            return self.submit(&place.id, false);
        }
        if ours.request.is_some() && ours.id.is_none() && ours.verdict.is_none() {
            return self.step_base(queue, &place.repository, &place.base);
        }
        false
    }

    /// Submits the place's branch at its head to its base, holding no
    /// wait on the verdict (111, 117): a base place's by its branch's
    /// name, at the revision where the merge queue says the branch
    /// lives, from the repository, and any other's from its worktree's
    /// head (203). A retry naming the submission submits whether or not
    /// one is due (49).
    pub(crate) fn submit(&self, id: &PlaceId, retried: bool) -> bool {
        let claimed = self.update(id, |held| {
            if held.phase != Phase::Live || !(retried || held.submission.due) {
                return Step::Keep(None);
            }
            held.submission.due = false;
            held.submission.submits += 1;
            if let Some(previous) = held.submission.id.take() {
                held.submission.before = Some(previous);
            }
            held.submission.request = None;
            held.submission.verdict = None;
            held.submission.waiting = None;
            Step::Write(Some(held.place.clone()))
        });
        let Some(Some(place)) = claimed else {
            return false;
        };
        let Some(queue) = self.bound(&place).ok().and_then(|bound| bound.merge_queue) else {
            return false;
        };
        // The branch's newest entry now is never this submission, whose
        // entry is the one the submit makes, or the one an intake takes
        // its request in as later (99, 124).
        let before = match on_base(
            &place.base,
            queue.read_submission(&place.repository, &place.branch),
        ) {
            SubmissionRead::Found { submission } => Some(submission.id),
            SubmissionRead::None | SubmissionRead::CouldNotRead { .. } => None,
        };
        let head = if base_place(&place.kind) {
            match self.base_place_head(&place, queue.as_ref()) {
                Ok(revision) => SubmitHead::Branch {
                    branch: place.branch.clone(),
                    revision,
                },
                Err(reason) => {
                    self.submitted(id, None, None, before);
                    self.verdict(
                        id,
                        Verdict::Refused {
                            reason: format!(
                                "the head of {} could not be read: {reason}",
                                place.branch
                            ),
                        },
                    );
                    return true;
                }
            }
        } else {
            SubmitHead::Worktree {
                head: self
                    .history
                    .head(&place.repository, &place.branch)
                    .unwrap_or_else(|| place.base_revision.clone()),
            }
        };
        match queue.submit(&place.place_ref(), &head) {
            Submitted::Submitted { submission } => {
                self.submitted(id, Some(&submission), None, before)
            }
            Submitted::Requested { request } => self.submitted(id, None, Some(request), before),
            Submitted::Decided {
                submission,
                outcome,
            } => {
                self.submitted(id, Some(&submission), None, before);
                self.outcome(id, outcome);
            }
            Submitted::Refused { reason } => {
                self.submitted(id, None, None, before);
                self.verdict(id, Verdict::Refused { reason });
            }
        }
        true
    }

    /// What the merge queue holds of flume's submit recorded as the
    /// place's own: the entry it made, or the request it opened, the
    /// branch's newest entry before it never read as the place's (99,
    /// 124).
    fn submitted(
        &self,
        id: &PlaceId,
        submission: Option<&Submission>,
        request: Option<String>,
        before: Option<String>,
    ) {
        self.update(id, |held| {
            held.submission.id = submission.map(|submission| submission.id.clone());
            held.submission.request = submission
                .and_then(|submission| submission.request.clone())
                .or_else(|| request.clone());
            if before.is_some() {
                held.submission.before = before.clone();
            }
            Step::Write(())
        });
    }

    /// Acts on an outcome of the place's submission, or its refusal at
    /// submit: a failure or a conflict starts the kind's fix round with
    /// the merge queue's words, or, for a base place, which has no stage
    /// to start, leaves it waiting on its caller with them, whose fix
    /// lands through a place on its branch (36, 112, 113, 131, 203). A failure or a conflict
    /// is acted on once for the submission flume made, decided in the
    /// write itself: two settles that read it at once, or one that
    /// reads it again from the record or the inbox, find the verdict
    /// held and change nothing, so it is counted once toward the kind's
    /// bound and never sends back over the fix round's agent (74, 75).
    pub(crate) fn verdict(&self, id: &PlaceId, verdict: Verdict) {
        self.verdict_at(id, verdict, None);
    }

    /// Acts on an outcome of the place's submission as [`Engine::verdict`]
    /// acts on its verdict, a conflict's at the base's revision the merge
    /// queue tested against (112).
    pub(crate) fn outcome(&self, id: &PlaceId, outcome: SubmissionOutcome) {
        let onto = match &outcome {
            SubmissionOutcome::Conflict { base, .. } => Some(base.clone()),
            _ => None,
        };
        self.verdict_at(id, verdict_of(outcome), onto);
    }

    /// Acts on a verdict, a conflict's at `onto` where it is known; a base
    /// place's failure or conflict, where its kind names a fix, launches
    /// that fix as a conflict with its base does, decided in the write that
    /// holds the verdict, so it is acted on once (112, 203).
    fn verdict_at(&self, id: &PlaceId, verdict: Verdict, onto: Option<Revision>) {
        let limit = self.defaults.send_backs;
        let now = self.host.now();
        let Some(read) = self.store.read(id) else {
            return;
        };
        let place_of = self.fix_place_of(&read.state);
        let back = self.update(id, |held| {
            if matches!(verdict, Verdict::Failed { .. } | Verdict::Conflict { .. })
                && held.submission.verdict.is_some()
            {
                return Step::Keep(None);
            }
            held.submission.verdict = Some(verdict.clone());
            let (cause, words) = match &verdict {
                Verdict::Landed { commit, .. } => {
                    if held.phase == Phase::Live {
                        held.phase = Phase::Over {
                            over: Over::Landed {
                                commit: commit.clone(),
                            },
                            kept: None,
                        };
                    }
                    held.submission.due = false;
                    return Step::Write(None);
                }
                Verdict::Withdrawn => {
                    held.submission.waiting =
                        Some("the merge queue withdrew the submission".into());
                    return Step::Write(None);
                }
                Verdict::Refused { reason } => {
                    held.submission.waiting =
                        Some(format!("the merge queue refused the submission: {reason}"));
                    return Step::Write(None);
                }
                Verdict::Failed { words } => ("the submission failed", words),
                Verdict::Conflict { words, .. } => {
                    ("the submission conflicted with its base", words)
                }
            };
            if held.phase != Phase::Live {
                return Step::Write(None);
            }
            if base_place(&held.place.kind) {
                let submission = held.submission.id.clone().unwrap_or_default();
                let found = Found::submission(&held.place, &submission, &verdict, onto.as_ref());
                if let (Some(_), Some(found)) = (&held.place.fixes_as, found) {
                    held.submission.waiting = Some(found.said.clone());
                    let fix = decide(held, &found, limit, now, &place_of);
                    return Step::Write(fix.map(|number| After::Fix(number, found)));
                }
                held.submission.waiting = Some(format!("{cause}: {words}"));
                return Step::Write(None);
            }
            Step::Write(
                fix_round(held, limit, cause, words)
                    .map(|(target, words)| After::Back(target, words)),
            )
        });
        match back.flatten() {
            Some(After::Back(target, words)) => {
                self.start_stage(id, target, Some(words));
            }
            Some(After::Fix(number, found)) => self.launch_fix(id, number, Some(&found)),
            None => {}
        }
    }

    /// Found work from the kind's last stage starts the kind's fix round
    /// with the findings as its words where the fix round is a stage
    /// before it, once the agent that stopped short is quiet and ended, so
    /// the place never has two agents (131, 30). A found from any other
    /// stage, and from a last stage that is its kind's own fix round, goes
    /// back nowhere: it changes nothing here, its agent kept as any
    /// stop-short's, and is taken as needing an answer, which a caller's
    /// retry gives (42, 49, 131). True when it changed something.
    pub(crate) fn found_work(&self, id: &PlaceId) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let held = read.state;
        let Some(index) = held.current.filter(|index| sent_back_by(&held, *index)) else {
            return false;
        };
        if let Some(start) = held.stages[index].start.clone().filter(|start| start.live) {
            let reading = self.read_agent(id, &start);
            if reading.state != AgentState::Stopped {
                if !reading.quiet() {
                    return false;
                }
                if let Closed::Refused { reason } = self.agent_on(&start).close() {
                    self.failed_end(id, &start.identity, &reason);
                    return false;
                }
            }
            self.with_start(id, &start.identity, |start, _| start.live = false);
        }
        let limit = self.defaults.send_backs;
        let back = self.update(id, |held| {
            if held.live_start().is_some() || held.current != Some(index) {
                return Step::Keep(None);
            }
            if !sent_back_by(held, index) {
                return Step::Keep(None);
            }
            let words = held.stages[index]
                .end
                .as_ref()
                .and_then(|end| end.words.clone())
                .unwrap_or_default();
            let cause = format!("the {} stage found work", held.stages[index].name);
            Step::Write(Some(fix_round(held, limit, &cause, &words)))
        });
        match back.flatten() {
            Some(Some((target, words))) => {
                self.start_stage(id, target, Some(words));
                true
            }
            Some(None) => true,
            None => false,
        }
    }

    /// The place's work is over (63).
    pub(crate) fn over(&self, id: &PlaceId, over: Over) {
        self.update(id, |held| {
            if !matches!(held.phase, Phase::Live) {
                return Step::Keep(());
            }
            held.phase = Phase::Over {
                over: over.clone(),
                kept: None,
            };
            held.submission.due = false;
            Step::Write(())
        });
    }
}

/// read-submission's answer as a place on `base` takes it: a branch
/// passed over decides that branch alone, so the read stands past every
/// other one, and where the branch passed over is the base, the one
/// branch flume submits the place to, the place's own submission could
/// not be read, whatever the other branches gave (15, 68, 99).
pub(crate) fn on_base(base: &str, across: ReadAcross) -> SubmissionRead {
    match across
        .passed_over
        .into_iter()
        .find(|over| over.branch == base)
    {
        Some(over) => SubmissionRead::CouldNotRead {
            reason: format!("{}: {}", over.branch, over.words),
        },
        None => across.read,
    }
}

/// The merge queue's read of the place's branch as the place's own, on
/// its base (15): a submission it held before the place was launched, an
/// earlier place's of the same name, reads as none (3, 99, 132).
pub(crate) fn own_read(held: &HeldState, across: ReadAcross) -> SubmissionRead {
    match on_base(&held.place.base, across) {
        SubmissionRead::Found { submission }
            if held.submission.before.as_ref() == Some(&submission.id) =>
        {
            SubmissionRead::None
        }
        read => read,
    }
}

/// Whether the entry read is the place's submission: the one flume's
/// submit made, or one an intake took in from the request it opened
/// (99, 124).
fn is_ours(ours: &SubmissionHeld, submission: &Submission) -> bool {
    ours.id.as_ref() == Some(&submission.id)
        || (ours.request.is_some() && submission.request == ours.request)
}

/// A decided entry's outcome as the merge queue's record gives it; a
/// conflict found before any test names the base's revision where it
/// lives now, and is read at a later settle where that cannot be read
/// (112, 126).
fn recorded(
    queue: &dyn MergeQueue,
    place: &PlaceRecord,
    state: &SubmissionState,
) -> Option<SubmissionOutcome> {
    match state {
        SubmissionState::Landed { commit } => Some(SubmissionOutcome::Landed {
            commit: commit.clone(),
        }),
        SubmissionState::Failed { words } => Some(SubmissionOutcome::Failed {
            words: words.clone(),
        }),
        SubmissionState::Conflict { files, base, words } => {
            let base = match base {
                Some(base) => base.clone(),
                None => match queue.base_revision(&place.repository, &place.base) {
                    BaseRevision::At { revision } => revision,
                    BaseRevision::Refused { .. } | BaseRevision::Unlanded => return None,
                },
            };
            Some(SubmissionOutcome::Conflict {
                files: files.clone(),
                base,
                words: words.clone(),
            })
        }
        SubmissionState::Waiting | SubmissionState::UnderTest | SubmissionState::Withdrawn => None,
    }
}

/// Whether the stage at `index` ended short having found work that sends
/// the place back: the kind's last stage, its fix round a stage before
/// it, not yet started nor refused past the kind's bound. A found from
/// any other stage, and from a last stage that is its kind's fix round,
/// is no send-back, since no stage before it can do that work again, and
/// is taken as needing an answer (42, 131). The settle and status read
/// it alike, so status never says a place waits on an answer that the
/// next settle sends back, nor the reverse.
pub(crate) fn sent_back_by(held: &HeldState, index: usize) -> bool {
    let stage = &held.stages[index];
    held.phase == Phase::Live
        && found_goes_back(held, index)
        && stage.state == StageState::EndedShort
        && stage.end.as_ref().and_then(|end| end.needs) == Some(StopKind::FoundWork)
        && held.submission.waiting.is_none()
}

/// Whether found work from the stage at `index` would send the place
/// back: the kind's last stage, its fix round a stage before it. A found
/// from any other stage is taken as needing an answer, and its agent kept
/// open for one as any such stop-short's, so the stop-short and the
/// settle read it alike (42, 53, 131).
pub(crate) fn found_goes_back(held: &HeldState, index: usize) -> bool {
    index + 1 == held.stages.len() && fix_round_of(held).is_some_and(|round| round < index)
}

/// The kind's fix round, the one reading of it the send-back, the move a
/// place past its bound waits on and a found all take: the position of
/// the stage the kind's last stage names for a failed submission, that
/// last stage where it names none, and none for a kind with no stages.
/// weir refuses a name that is no stage of the kind, so the last stage
/// stands only where it names none (23, 112, 131; core.4).
pub(crate) fn fix_round_of(held: &HeldState) -> Option<usize> {
    let last = held.stages.len().checked_sub(1)?;
    let named = held
        .place
        .kind
        .stages
        .last()
        .and_then(|stage| stage.send_back_to.as_ref());
    Some(
        named
            .and_then(|name| held.stages.iter().position(|stage| &stage.name == name))
            .unwrap_or(last),
    )
}

/// The kind's fix round decided in held state: the stage
/// [`fix_round_of`] reads, and every stage from it not reached again,
/// counted as one going-back against the last stage's bound, flume's
/// default where it sets none; past the bound, the place waits on its
/// caller with `cause` and the words (23, 112, 130, 131). The stage to
/// start and its words.
fn fix_round(
    held: &mut HeldState,
    limit: u32,
    cause: &str,
    words: &str,
) -> Option<(usize, String)> {
    let allowed = held
        .place
        .kind
        .stages
        .last()?
        .send_back_limit
        .unwrap_or(limit);
    let target = fix_round_of(held)?;
    if held.submission.send_backs >= allowed {
        held.submission.waiting = Some(format!(
            "{cause}: {words}. The place went back {} times, as many as its kind allows",
            held.submission.send_backs
        ));
        return None;
    }
    held.submission.send_backs += 1;
    held.submission.waiting = None;
    held.submission.due = false;
    for stage in held.stages.iter_mut().skip(target) {
        stage.state = StageState::NotReached;
        stage.made_from = None;
    }
    Some((target, words.to_string()))
}

/// An outcome as the answer reports it: the merge queue removes nothing
/// of a place (113). A conflict's words name the files contested and the
/// base's revision the merge queue tested against, which the fix round's
/// agent merges into the place's branch and commits before anything
/// else; flume merges nothing itself (14, 112, core.5).
fn verdict_of(outcome: SubmissionOutcome) -> Verdict {
    match outcome {
        SubmissionOutcome::Landed { commit } => Verdict::Landed {
            commit,
            place_removed: false,
            branch_removed: false,
        },
        SubmissionOutcome::Failed { words } => Verdict::Failed { words },
        SubmissionOutcome::Conflict { files, base, words } => {
            let words = format!(
                "{words}\n\nThe files contested: {}.\nThe base's revision the merge queue tested against: {base}.",
                files.join(", ")
            );
            Verdict::Conflict { files, words }
        }
    }
}

/// What follows a verdict's write: the place's fix round started at a
/// stage with the words, or a base place's fix launched by its number for
/// what found it (112, 203).
enum After {
    Back(usize, String),
    Fix(u32, Found),
}

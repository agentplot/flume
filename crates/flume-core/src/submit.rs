//! A landing kind's submission (63, 111 to 113, 124, 125, 133): made at
//! the place's head once its last stage delivered and its agent ended,
//! unless the launch holds it or the work is over, with no wait held on
//! it. Its outcomes are collected at the settles that follow through the
//! merge queue's inbox and acted on once each: a landing makes the work
//! over, a failure or a conflict starts the kind's fix round, as found
//! work from its last stage does, every going-back counted together up to
//! the kind's bound. A withdrawal, and a landing before flume
//! submits, are read from the merge queue's record of the branch; an
//! open submission's base is driven one round per command, and never
//! submitted anew.

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
        if ours.submits > 0 && self.collect(id, queue.as_ref(), &place.place_ref()) {
            return true;
        }
        match queue.read_submission(&place.repository, &place.branch) {
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
                    if ours.id.as_ref() == Some(&submission.id) && ours.waiting.is_none() =>
                {
                    self.verdict(id, Verdict::Withdrawn);
                    true
                }
                state if state.open() => {
                    self.step_base(queue.as_ref(), &place.repository, &submission.target)
                }
                _ => self.due(held),
            },
            SubmissionRead::None => self.due(held),
        }
    }

    /// Collects the place's pending outcomes and acts on each once, in
    /// the order given; an inbox that cannot be read is named in status
    /// and acts on nothing (133, 68). True when it changed something.
    fn collect(&self, id: &PlaceId, queue: &dyn MergeQueue, place: &PlaceRef) -> bool {
        match queue.inbox(place) {
            Inbox::CouldNotRead { reason } => {
                self.update(id, |held| {
                    held.submission.unread = Some(reason.clone());
                    Step::Write(())
                });
                false
            }
            Inbox::Outcomes { outcomes } => {
                let mut changed = self
                    .update(id, |held| {
                        let was = held.submission.unread.take();
                        Step::Write(was.is_some())
                    })
                    .unwrap_or(false);
                for outcome in outcomes {
                    self.verdict(id, verdict_of(outcome));
                    changed = true;
                }
                changed
            }
        }
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
    /// live (47, 111).
    fn due(&self, held: &HeldState) -> bool {
        let ours = &held.submission;
        if ours.due && !held.place.hold_submission && held.live_start().is_none() {
            return self.submit(&held.place.id, false);
        }
        false
    }

    /// Submits the place's branch at its head to its base, holding no
    /// wait on the verdict (111, 117). A retry naming the submission
    /// submits whether or not one is due (49).
    pub(crate) fn submit(&self, id: &PlaceId, retried: bool) -> bool {
        let claimed = self.update(id, |held| {
            if held.phase != Phase::Live || !(retried || held.submission.due) {
                return Step::Keep(None);
            }
            held.submission.due = false;
            held.submission.submits += 1;
            held.submission.id = None;
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
        let head = self
            .history
            .head(&place.repository, &place.branch)
            .unwrap_or_else(|| place.base_revision.clone());
        match queue.submit(&place.place_ref(), &head) {
            Submitted::Submitted { submission } => {
                self.update(id, |held| {
                    held.submission.id = Some(submission.id.clone());
                    Step::Write(())
                });
            }
            Submitted::Refused { reason } => self.verdict(id, Verdict::Refused { reason }),
        }
        true
    }

    /// Acts on an outcome of the place's submission, or its refusal at
    /// submit: a failure or a conflict starts the kind's fix round with
    /// the merge queue's words (112, 113, 131).
    pub(crate) fn verdict(&self, id: &PlaceId, verdict: Verdict) {
        let limit = self.defaults.send_backs;
        let back = self.update(id, |held| {
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
            Step::Write(fix_round(held, limit, cause, words))
        });
        if let Some(Some((target, words))) = back {
            self.start_stage(id, target, Some(words));
        }
    }

    /// Found work from the kind's last stage starts the kind's fix round
    /// with the findings as its words, once the agent that stopped short
    /// is quiet and ended, so the place never has two agents (131, 30).
    /// True when it changed something.
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
                if let Closed::Refused { reason } = self.terminal.close(&start.identity) {
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

/// Whether the stage at `index` ended short having found work, as the
/// kind's last stage, its fix round not yet started nor refused past the
/// kind's bound (131).
fn sent_back_by(held: &HeldState, index: usize) -> bool {
    let stage = &held.stages[index];
    held.phase == Phase::Live
        && index + 1 == held.stages.len()
        && stage.state == StageState::EndedShort
        && stage.end.as_ref().and_then(|end| end.needs) == Some(StopKind::FoundWork)
        && held.submission.waiting.is_none()
}

/// The kind's fix round decided in held state: the stage the kind's
/// last stage names for a failed submission, that last stage where it
/// names none, and every stage from it not reached again, counted as one
/// going-back against the last stage's bound, flume's default where it
/// sets none; past the bound, the place waits on its caller with `cause`
/// and the words (23, 112, 130, 131). The stage to start and its words.
fn fix_round(
    held: &mut HeldState,
    limit: u32,
    cause: &str,
    words: &str,
) -> Option<(usize, String)> {
    let last = held.place.kind.stages.last()?;
    let allowed = last.send_back_limit.unwrap_or(limit);
    let target = last
        .send_back_to
        .as_ref()
        .and_then(|name| held.stages.iter().position(|stage| &stage.name == name))
        .unwrap_or(held.stages.len() - 1);
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

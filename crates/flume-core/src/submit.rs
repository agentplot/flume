//! A landing kind's submission (63, 111 to 113, 124, 125): made at the
//! place's head once its last stage delivered and its agent ended, unless
//! the launch holds it or the work is over; the command that submits
//! records itself as the waiter and acts on the verdict. A failed
//! submission sends the place back to the stage its kind names, up to its
//! count; a landing read before flume submits is the landing; and an open
//! submission nobody waits on is waited on again, never made anew.

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
        let waiter_gone = ours
            .waiter
            .as_ref()
            .is_some_and(|waiter| !self.host.alive(waiter));
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
                state if state.open() => {
                    if ours.submits == 0 || (ours.waiter.is_some() && !waiter_gone) {
                        return false;
                    }
                    if !self.claim_waiter(id, ours.waiter.as_deref()) {
                        return true;
                    }
                    let verdict = queue.wait(&place.repository, &submission);
                    self.verdict(id, verdict);
                    true
                }
                SubmissionState::Failed { words } if waiter_gone => {
                    self.verdict(
                        id,
                        Verdict::Failed {
                            words: words.clone(),
                        },
                    );
                    true
                }
                SubmissionState::Withdrawn if waiter_gone => {
                    self.verdict(id, Verdict::Withdrawn);
                    true
                }
                _ => self.unwaited_or_due(held),
            },
            SubmissionRead::None => self.unwaited_or_due(held),
        }
    }

    /// A waiter gone with no submission made leaves it due again; a due
    /// submission is made unless held (47, 111).
    fn unwaited_or_due(&self, held: &HeldState) -> bool {
        let id = &held.place.id;
        let ours = &held.submission;
        if ours
            .waiter
            .as_ref()
            .is_some_and(|waiter| !self.host.alive(waiter))
        {
            self.update(id, |held| {
                held.submission.waiter = None;
                held.submission.due = true;
                Step::Write(())
            });
            return true;
        }
        if ours.due && !held.place.hold_submission && held.live_start().is_none() {
            return self.submit(id, false);
        }
        false
    }

    /// Records this command as the submission's waiter, if `was` still is.
    fn claim_waiter(&self, id: &PlaceId, was: Option<&str>) -> bool {
        let me = self.host.command();
        self.update(id, |held| {
            if held.submission.waiter.as_deref() != was {
                return Step::Keep(false);
            }
            held.submission.waiter = Some(me.clone());
            Step::Write(true)
        })
        .unwrap_or(false)
    }

    /// Submits the place's branch at its head to its base, this command
    /// waiting on the verdict, and acts on it (111, 117). A retry naming
    /// the submission submits whether or not one is due (49).
    pub(crate) fn submit(&self, id: &PlaceId, retried: bool) -> bool {
        let me = self.host.command();
        let claimed = self.update(id, |held| {
            if held.phase != Phase::Live || !(retried || held.submission.due) {
                return Step::Keep(None);
            }
            held.submission.due = false;
            held.submission.submits += 1;
            held.submission.waiter = Some(me.clone());
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
        let verdict = queue.submit(&place.place_ref(), &head);
        self.verdict(id, verdict);
        true
    }

    /// Acts on the merge queue's verdict (112, 113).
    pub(crate) fn verdict(&self, id: &PlaceId, verdict: Verdict) {
        let limit = self.defaults.send_backs;
        let back = self.update(id, |held| {
            held.submission.waiter = None;
            held.submission.verdict = Some(verdict.clone());
            match &verdict {
                Verdict::Landed { commit, .. } => {
                    held.phase = Phase::Over {
                        over: Over::Landed {
                            commit: commit.clone(),
                        },
                        kept: None,
                    };
                    Step::Write(None)
                }
                Verdict::Failed { words } => {
                    let last = held.place.kind.stages.last().expect("a kind has a stage");
                    let allowed = last.send_back_limit.unwrap_or(limit);
                    if held.submission.send_backs >= allowed {
                        held.submission.waiting = Some(format!(
                            "the submission failed: {words}. The place went back {} times, as many as its kind allows",
                            held.submission.send_backs
                        ));
                        return Step::Write(None);
                    }
                    let target = last
                        .send_back_to
                        .as_ref()
                        .and_then(|name| held.stages.iter().position(|stage| &stage.name == name))
                        .unwrap_or(held.stages.len() - 1);
                    held.submission.send_backs += 1;
                    for stage in held.stages.iter_mut().skip(target) {
                        stage.state = StageState::NotReached;
                    }
                    Step::Write(Some((target, words.clone())))
                }
                Verdict::Withdrawn => {
                    held.submission.waiting = Some("the merge queue withdrew the submission".into());
                    Step::Write(None)
                }
                Verdict::Refused { reason } => {
                    held.submission.waiting =
                        Some(format!("the merge queue refused the submission: {reason}"));
                    Step::Write(None)
                }
            }
        });
        if let Some(Some((target, words))) = back {
            self.start_stage(id, target, Some(words));
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

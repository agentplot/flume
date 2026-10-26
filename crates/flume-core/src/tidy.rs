//! The tidy (16, 62 to 68): a place whose work is over and whose agent
//! has stopped is removed through its repository's teardown, kept when
//! it holds work or its teardown fails; its branch goes only through
//! remove-branch's judgement, named once when kept; nothing is removed
//! while the merge queue cannot be read; its held state and folder go
//! last.

use crate::start::Starts;
use crate::{BranchKept, Engine, KeptPlace, Step, Tidied};
use flume_seams::*;

impl Engine {
    /// Tidies the place if its work is over; true when it changed
    /// something.
    pub(crate) fn tidy_place(&self, id: &PlaceId, tidied: &mut Tidied) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let held = read.state;
        let place = &held.place;
        let over = match &held.phase {
            Phase::Over { over, .. } | Phase::Gone { over } => over.clone(),
            _ => {
                tidied.not_over.push(id.clone());
                return false;
            }
        };
        let Ok(bound) = self.bound(place) else {
            return false;
        };
        let at = place.place_ref();
        if let Phase::Over { .. } = held.phase {
            if let (true, Some(queue)) = (place.kind.lands, &bound.merge_queue) {
                if let SubmissionRead::CouldNotRead { reason } =
                    queue.read_submission(&place.repository, &place.branch)
                {
                    return self.keep(id, Kept::Unreadable { reason }, tidied);
                }
            }
            if let Some((_, start)) = held.live_start() {
                let identity = start.identity.clone();
                let reading = self.read_agent(start);
                if reading.state != AgentState::Stopped {
                    if !reading.quiet() {
                        return self.keep(
                            id,
                            Kept::AgentLive {
                                identity: identity.to_string(),
                            },
                            tidied,
                        );
                    }
                    if let Closed::Refused { reason } = self.terminal.close(&identity) {
                        self.failed_end(id, &identity, &reason);
                        return self.keep(
                            id,
                            Kept::AgentLive {
                                identity: identity.to_string(),
                            },
                            tidied,
                        );
                    }
                }
                self.with_start(id, &identity, |start, _| start.live = false);
            }
            match bound.place.read(&at) {
                PlaceRead::Gone => {}
                PlaceRead::CouldNotRead { reason } => {
                    return self.keep(id, Kept::Unreadable { reason }, tidied)
                }
                PlaceRead::Live { holds, .. } if holds.any() => {
                    return self.keep(id, Kept::Holds { holds }, tidied)
                }
                PlaceRead::Live { .. } => {
                    if let Removed::Refused { step, words } = bound.place.remove(&at) {
                        return self.keep(id, Kept::Teardown { step, words }, tidied);
                    }
                }
            }
            self.update(id, |held| {
                held.phase = Phase::Gone { over: over.clone() };
                Step::Write(())
            });
        }
        if let BranchRemoved::Kept { commits } =
            bound
                .place
                .remove_branch(&place.repository, &place.branch, &place.base)
        {
            tidied.branches_kept.push(BranchKept {
                place: id.clone(),
                branch: place.branch.clone(),
                commits,
            });
        }
        if self.remove_held(id) {
            tidied.removed.push(id.clone());
        }
        true
    }

    /// Keeps a place whose work is over, naming why (64 to 66, 68).
    fn keep(&self, id: &PlaceId, why: Kept, tidied: &mut Tidied) -> bool {
        self.update(id, |held| {
            if let Phase::Over { kept, .. } = &mut held.phase {
                *kept = Some(why.clone());
            }
            Step::Write(())
        });
        tidied.kept.push(KeptPlace {
            place: id.clone(),
            why,
        });
        false
    }
}

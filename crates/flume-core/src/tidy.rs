//! The tidy (16, 62 to 68): a place whose work is over and whose agent
//! has stopped is removed through its repository's teardown, kept when
//! it holds work or its teardown fails; its branch goes only through
//! remove-branch's judgement, named once when kept; nothing is removed
//! while the merge queue cannot be read; what the terminal made for it
//! is tidied, kept where what flume did not make holds it (137, 138);
//! a place whose kind holds a conversation is kept while a process runs
//! its last conversation start's session (207; core.18); a place read
//! gone is removed all the same, so its location holds nothing of it
//! (62; record 0057);
//! its held state and folder go last, its document alone kept apart from
//! a live place's while an end of it stands pending (43, 62). A base
//! place has no worktree to remove and no teardown to run: its branch is
//! closed into its base by the merge queue, never by the place seam, and
//! one the merge queue keeps is named once with its words (3, 64 to 67,
//! 204, 206; core.18).

use crate::base::base_place;
use crate::start::Starts;
use crate::{Answer, BranchKept, Engine, KeptPlace, Step, TerminalTidied, Tidied};
use flume_seams::*;

impl Engine {
    /// Tidies the place, or every place on the host: each whose work is
    /// over is removed or kept naming why, and each whose work is not
    /// over is named so.
    pub fn tidy(&self, place: Option<&PlaceId>) -> Answer {
        let mut tidy = self.settle_attended();
        let targets: Vec<PlaceId> = match place {
            Some(id) => {
                if self.store.read(id).is_none() && !tidy.removed.contains(id) {
                    return Answer::refused(format!("no place {id} stands on this host"));
                }
                vec![id.clone()]
            }
            None => self
                .store
                .read_every()
                .into_iter()
                .map(|read| read.state.place.id)
                .collect(),
        };
        for id in &targets {
            match self.store.read(id).map(|read| read.state.phase) {
                Some(Phase::Over { .. } | Phase::Gone { .. } | Phase::Tidied { .. }) => {
                    tidy.add(self.settle_place(id))
                }
                Some(_) => tidy.not_over.push(id.clone()),
                None => {}
            }
        }
        let mut kept: Vec<KeptPlace> = vec![];
        for one in tidy.kept.into_iter().rev() {
            if !kept.iter().any(|other| other.place == one.place) {
                kept.insert(0, one);
            }
        }
        tidy.kept = kept;
        if let Some(id) = place {
            tidy.removed.retain(|removed| removed == id);
            tidy.kept.retain(|kept| &kept.place == id);
            tidy.branches_kept.retain(|kept| &kept.place == id);
        }
        Answer::Tidied { tidy }
    }

    /// Tidies the place if its work is over; true when it changed
    /// something.
    pub(crate) fn tidy_place(&self, id: &PlaceId, tidied: &mut Tidied) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let held = read.state;
        let place = &held.place;
        let over = match &held.phase {
            Phase::Over { over, .. } | Phase::Gone { over, .. } => over.clone(),
            _ => {
                tidied.not_over.push(id.clone());
                return false;
            }
        };
        // A base place's fix writes how it ended onto its base place before
        // anything of it is removed, so its end is never lost (203).
        if !self.fix_ended(&held, &over) {
            return false;
        }
        let Ok(bound) = self.bound(place) else {
            return false;
        };
        let at = place.place_ref();
        let base = base_place(&place.kind);
        if let Phase::Over { .. } = held.phase {
            if let (true, Some(queue)) = (place.kind.lands, &bound.merge_queue) {
                if let SubmissionRead::CouldNotRead { reason } = crate::submit::on_base(
                    &place.base,
                    queue.read_submission(&place.repository, &place.branch),
                ) {
                    return self.keep(id, Kept::Unreadable { reason }, tidied);
                }
            }
            if let Some((_, start)) = held.live_start() {
                let identity = start.identity.clone();
                let reading = self.read_agent(id, start);
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
                    if let Closed::Refused { reason } = self.agent_on(start).close() {
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
            // A place whose kind holds a conversation is kept while a
            // process runs its last conversation start's session, or flume
            // cannot read whether one does: a person may be at it, in a
            // pane flume did not make among them (64, 207; core.18).
            if let Some(start) = last_conversation(&held) {
                let queue = self.queue();
                if let Some(read) = self.live_session(id, &start.program, &start.identity, &queue) {
                    return self.keep(
                        id,
                        Kept::SessionRuns {
                            processes: read.processes,
                            unread: read.unread,
                        },
                        tidied,
                    );
                }
            }
            // A place read gone, its worktree gone to its repository, is
            // still removed through the place seam, so nothing of it stands
            // at its location (62; record 0057); a base place has none.
            match if base {
                None
            } else {
                Some(bound.place.read(&at))
            } {
                None => {}
                Some(PlaceRead::CouldNotRead { reason }) => {
                    return self.keep(id, Kept::Unreadable { reason }, tidied)
                }
                Some(PlaceRead::Live { holds, .. }) if holds.any() => {
                    return self.keep(id, Kept::Holds { holds }, tidied)
                }
                Some(PlaceRead::Live { .. } | PlaceRead::Gone) => {
                    if let Removed::Refused { step, words } = bound.place.remove(&at) {
                        return self.keep(id, Kept::Teardown { step, words }, tidied);
                    }
                }
            }
            let now = self.host.now();
            self.update(id, |held| {
                held.phase = Phase::Gone {
                    over: over.clone(),
                    at: Some(now),
                };
                Step::Write(())
            });
        }
        let removed = match (base, &bound.merge_queue) {
            (true, Some(queue)) => {
                match queue.close(&place.repository, &place.branch, &place.base) {
                    BranchClosed::Removed => BranchRemoved::Removed,
                    BranchClosed::Gone => BranchRemoved::Gone,
                    BranchClosed::Kept { reason } => BranchRemoved::Refused { reason },
                }
            }
            (true, None) => BranchRemoved::Refused {
                reason: format!("{} binds no merge queue to close it", place.repository),
            },
            (false, _) => bound
                .place
                .remove_branch(&place.repository, &place.branch, &place.base),
        };
        match removed {
            BranchRemoved::Kept { commits } => tidied.branches_kept.push(BranchKept {
                place: id.clone(),
                branch: place.branch.clone(),
                commits,
                reason: None,
            }),
            BranchRemoved::Refused { reason } => tidied.branches_kept.push(BranchKept {
                place: id.clone(),
                branch: place.branch.clone(),
                commits: vec![],
                reason: Some(reason),
            }),
            BranchRemoved::Removed | BranchRemoved::Gone => {}
        }
        // Each terminal its agents opened in tidies what it made for the
        // place, whatever the configuration names by now (137, 190).
        let (terminals, unreached) = self.tidy_terminals(&held);
        let answers: Vec<(Option<String>, TerminalTidy)> = terminals
            .iter()
            .map(|(profile, terminal)| (profile.clone(), terminal.tidy(&at)))
            .collect();
        for reason in unreached {
            self.host.log(&format!(
                "the place {id} was not tidied in a terminal: {reason}"
            ));
        }
        self.intent_left(&place.intent, id);
        self.hand_batches(Some(id));
        let read = self.store.read(id);
        // A place whose ends stand pending is removed only once the
        // document kept for them stands, so no pending end is left with
        // no document to read it from; the next settle tidies it again
        // (43, 62; weir I9).
        let kept = match read
            .as_ref()
            .map(|read| self.keep_for_ends(&read.state, &over))
        {
            Some(Err(why)) => {
                self.host
                    .log(&format!("the place {id} was not removed: {why}"));
                return false;
            }
            Some(Ok(kept)) => kept,
            None => None,
        };
        if self.remove_held(id) {
            // The tidy that keeps no document lets go of its ends'
            // deliveries, every one delivered (62; weir 107).
            if let (None, Some(read)) = (kept, &read) {
                self.let_go_of(&read.state);
            }
            tidied.removed.push(id.clone());
            for (profile, terminal) in answers {
                if !terminal.closed.is_empty() || !terminal.kept.is_empty() {
                    tidied.terminal.push(TerminalTidied {
                        place: id.clone(),
                        profile,
                        closed: terminal.closed,
                        kept: terminal.kept,
                    });
                }
            }
        }
        true
    }

    /// Keeps a place whose work is over, naming why (64 to 66, 68, 207).
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

/// The last start of the place's last stage declared a conversation that
/// started, none where its kind holds no conversation or none started
/// (64, 109, 207).
fn last_conversation(held: &HeldState) -> Option<&StartHeld> {
    held.stages
        .iter()
        .zip(&held.place.kind.stages)
        .filter(|(_, declared)| declared.conversation)
        .filter_map(|(stage, _)| stage.start.as_ref())
        .max_by_key(|start| start.started_at)
}

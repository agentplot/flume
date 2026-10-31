//! The settle, flume's one mover (46, 119 to 122): it reads every fact of
//! a place again and takes each step Part A then allows, collecting its
//! submission's outcomes first, safe to repeat (74, 133). It runs inside
//! a command of flume's, started by an agent going quiet, by the
//! terminal's watch, or by a caller, and waits on no merge queue (120,
//! core.13). Each end it or an act reads is recorded once as it is
//! written, and the settle delivers each end pending for a recipient,
//! then wakes each role's address one stands pending for with no run
//! (43, 132, core.25).

use crate::held::read_stall;
use crate::{Answer, Engine, Step, Tidied};
use flume_seams::*;

/// How many steps one settle of a place takes at most.
const STEPS: usize = 64;

impl Engine {
    /// Settles the place, or every place on the host.
    pub fn settle(&self, place: Option<&PlaceId>) -> Answer {
        let places: Vec<PlaceId> = match place {
            Some(id) => vec![id.clone()],
            None => self
                .store
                .read_every()
                .into_iter()
                .map(|read| read.state.place.id)
                .collect(),
        };
        let mut tidy = Tidied::default();
        for id in &places {
            tidy.add(self.settle_place(id));
        }
        Answer::Settled { places, tidy }
    }

    /// Settles every place on the host.
    pub(crate) fn settle_every(&self) -> Tidied {
        let mut tidy = Tidied::default();
        for read in self.store.read_every() {
            tidy.add(self.settle_place(&read.state.place.id));
        }
        tidy
    }

    /// Where the terminal keeps no watch, every command of flume's that
    /// acts on places settles every place first (119, I8); under a watch
    /// the watch settles them.
    pub(crate) fn settle_attended(&self) -> Tidied {
        if self.terminal.watch() == Watch::None {
            return self.settle_every();
        }
        Tidied::default()
    }

    /// Settles one place, then starts what waits for room on the host
    /// (46, 70), and gives what its tidy did.
    pub(crate) fn settle_place(&self, id: &PlaceId) -> Tidied {
        let mut tidied = Tidied::default();
        self.deliver_ends(id);
        for _ in 0..STEPS {
            if !self.step(id, &mut tidied) {
                break;
            }
        }
        self.stall(id);
        self.deliver_ends(id);
        self.wake_roles(id);
        self.show(id);
        self.start_waiting();
        tidied
    }

    /// Reads the move a live place waits on its caller for, and records
    /// it stalled once its bound has passed under no hold's words (130).
    fn stall(&self, id: &PlaceId) {
        let now = self.host.now();
        self.update(id, |held| {
            if held.phase != Phase::Live {
                return Step::Keep(());
            }
            read_stall(held, now);
            Step::Write(())
        });
    }

    /// Shows the place's clues where they changed since last shown: its
    /// current stage, a repeating one with its group's place, and that
    /// stage's state, and the count of the places
    /// of its repository on its base. A clue is display only: one that
    /// fails is logged and fails nothing, and nothing reads one back
    /// (139).
    fn show(&self, id: &PlaceId) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let held = read.state;
        let group = self.current_group(&held);
        let Some(mut clues) = self.clues(&held, None, group.as_ref()) else {
            return;
        };
        clues.base = self.base_count(&held.place);
        let place = &held.place;
        let key = format!("{} · {}; {}", clues.stage, clues.state, clues.base);
        if held.shown.as_ref() == Some(&key) {
            return;
        }
        match self.terminal.show(&place.place_ref(), &clues) {
            Shown::Failed { reason } => {
                self.host
                    .log(&format!("the clues of {id} did not show: {reason}"));
            }
            Shown::Shown | Shown::ShowsNone => {
                self.update(id, |held| {
                    held.shown = Some(key.clone());
                    Step::Write(())
                });
            }
        }
    }

    /// One step the place allows; true when it changed something.
    fn step(&self, id: &PlaceId, tidied: &mut Tidied) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let held = read.state;
        match &held.phase {
            Phase::Making { by } => return !self.host.alive(by) && self.undo(&held),
            Phase::Over { .. } | Phase::Gone { .. } => return self.tidy_place(id, tidied),
            Phase::Tidied { .. } => return self.settle_tidied(id),
            Phase::Live => {}
        }
        if self.submission(&held) {
            return true;
        }
        let Some(index) = held.current else {
            return false;
        };
        match held.stages[index].state {
            StageState::Starting | StageState::Started => self.deliver(id),
            StageState::NeverBegun => self.begun_after(id),
            StageState::Begun | StageState::Stuck | StageState::Lost => self.read_end(id),
            StageState::Delivered | StageState::Finished | StageState::Repeating => {
                self.advance(id)
            }
            StageState::EndedShort => self.found_work(id),
            _ => false,
        }
    }

    /// After a stage delivered and its agent ended, starts the next stage
    /// unless it waits on its caller's word (40, 47); after a repetition
    /// delivered its group with a task still unticked, starts the same
    /// stage again, waiting for room like any start, unless a hold names
    /// it (70, 140; core.26).
    fn advance(&self, id: &PlaceId) -> bool {
        let next = self.update(id, |held| {
            let Some(index) = held.current else {
                return Step::Keep(None);
            };
            let ended = &held.stages[index];
            if ended.start.as_ref().is_some_and(|start| start.live) {
                return Step::Keep(None);
            }
            if ended.state == StageState::Repeating {
                if held.hold_on(&ended.name).is_some() {
                    held.stages[index].state = StageState::OnHold;
                    return Step::Write(None);
                }
                return Step::Keep(Some(index));
            }
            let next = index + 1;
            if next >= held.stages.len() || held.stages[next].state != StageState::NotReached {
                return Step::Keep(None);
            }
            let declared = &held.place.kind.stages[next];
            if held.hold_on(&declared.name).is_some() || declared.waits {
                held.stages[next].state = StageState::OnHold;
                return Step::Write(None);
            }
            Step::Keep(Some(next))
        });
        match next.flatten() {
            Some(next) => {
                self.start_stage(id, next, None);
                true
            }
            None => false,
        }
    }

    /// A place whose launching command is gone while it was made: what
    /// was made is removed, so nothing of it is left (2, 76).
    fn undo(&self, held: &HeldState) -> bool {
        let place = &held.place;
        let Ok(bound) = self.bound(place) else {
            return false;
        };
        let at = place.place_ref();
        if let PlaceRead::Live { .. } = bound.place.read(&at) {
            if let Removed::Refused { .. } = bound.place.remove(&at) {
                return false;
            }
        }
        bound
            .place
            .remove_branch(&place.repository, &place.branch, &place.base);
        self.remove_held(&place.id)
    }
}

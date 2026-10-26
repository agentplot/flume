//! The settle, flume's one mover (46, 119 to 122): it reads every fact of
//! a place again and takes each step Part A then allows, safe to repeat
//! (74). It runs inside a command of flume's, started by an agent going
//! quiet, by the merge queue's verdict, by the terminal's watch, or by a
//! caller (120).

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

    /// Settles one place, and gives what its tidy did.
    pub(crate) fn settle_place(&self, id: &PlaceId) -> Tidied {
        let mut tidied = Tidied::default();
        for _ in 0..STEPS {
            if !self.step(id, &mut tidied) {
                break;
            }
        }
        tidied
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
            Phase::Live => {}
        }
        if self.submission(&held) {
            return true;
        }
        let Some(index) = held.current else {
            return false;
        };
        match held.stages[index].state {
            StageState::Started => self.deliver(id),
            StageState::Begun | StageState::Stuck => self.read_end(id),
            StageState::Delivered | StageState::Finished => self.advance(id),
            _ => false,
        }
    }

    /// After a stage delivered and its agent ended, starts the next stage
    /// unless it waits on its caller's word (40, 47).
    fn advance(&self, id: &PlaceId) -> bool {
        let next = self.update(id, |held| {
            let Some(index) = held.current else {
                return Step::Keep(None);
            };
            let ended = &held.stages[index];
            if ended.start.as_ref().is_some_and(|start| start.live) {
                return Step::Keep(None);
            }
            let next = index + 1;
            if next >= held.stages.len() || held.stages[next].state != StageState::NotReached {
                return Step::Keep(None);
            }
            let declared = &held.place.kind.stages[next];
            if held.place.hold.contains(&declared.name) || declared.waits {
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

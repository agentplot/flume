//! The settle, flume's one mover (46, 119 to 122): it reads every fact of
//! a place again and takes each step Part A then allows, safe to repeat
//! (74). It runs inside a command of flume's, started by an agent going
//! quiet, by the merge queue's verdict, by the terminal's watch, or by a
//! caller (120).

use crate::{Answer, Engine, Tidied};
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
        for id in &places {
            self.settle_place(id);
        }
        Answer::Settled {
            places,
            tidy: Tidied::default(),
        }
    }

    pub(crate) fn settle_place(&self, id: &PlaceId) {
        for _ in 0..STEPS {
            if !self.step(id) {
                return;
            }
        }
    }

    /// One step the place allows; true when it changed something.
    fn step(&self, id: &PlaceId) -> bool {
        let Some(read) = self.store.read(id) else {
            return false;
        };
        let held = read.state;
        if held.phase != Phase::Live {
            return false;
        }
        let Some(index) = held.current else {
            return false;
        };
        match held.stages[index].state {
            StageState::Started => self.deliver(id),
            _ => false,
        }
    }
}

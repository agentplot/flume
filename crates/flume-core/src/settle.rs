//! The settle, flume's one mover (46, 119 to 122): it reads every fact of
//! a place again and takes each step Part A then allows, collecting its
//! submission's outcomes first, safe to repeat (74, 133). It runs inside
//! a command of flume's, started by an agent going quiet, by the
//! terminal's watch, or by a caller, and waits on no merge queue (120,
//! core.13).

use crate::start::Starts;
use crate::status::waiting_on;
use crate::{Answer, Engine, Step, Tidied};
use flume_seams::*;

/// How many steps one settle of a place takes at most.
const STEPS: usize = 64;

/// How long a place waits on its caller under no hold's words before it
/// is stalled (core.24).
const STALLED_AFTER: Seconds = 5 * 60;

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
        for _ in 0..STEPS {
            if !self.step(id, &mut tidied) {
                break;
            }
        }
        self.show(id);
        self.start_waiting();
        tidied
    }

    /// Shows the place's clues where they changed since last shown: its
    /// current stage and that stage's state, and the count of the places
    /// of its repository on its base. A clue is display only: one that
    /// fails is logged and fails nothing, and nothing reads one back
    /// (139).
    fn show(&self, id: &PlaceId) {
        let Some(read) = self.store.read(id) else {
            return;
        };
        let held = read.state;
        if held.phase != Phase::Live {
            return;
        }
        let Some(index) = held.current else {
            return;
        };
        let stage = &held.stages[index];
        let agent = stage.start.as_ref().filter(|start| start.live);
        let state = match agent {
            Some(_) if stage.state == StageState::Starting => "starting".to_string(),
            Some(start) => word(&self.read_agent(id, start).state),
            None => word(&stage.state),
        };
        let place = &held.place;
        let mut base = BaseCount::default();
        let now = self.host.now();
        for other in self.store.read_every() {
            let other = other.state;
            if other.phase != Phase::Live
                || other.place.repository != place.repository
                || other.place.base != place.base
            {
                continue;
            }
            if let Some(waiting) = waiting_on(&other) {
                let since = waiting.since.unwrap_or(now);
                if waiting.words.is_none() && now.saturating_sub(since) >= STALLED_AFTER {
                    base.stalled += 1;
                } else {
                    base.waiting_on_caller += 1;
                }
            } else if let Some((_, start)) = other.live_start() {
                if self.read_agent(&other.place.id, start).state == AgentState::Asking {
                    base.asking += 1;
                } else {
                    base.running += 1;
                }
            }
        }
        let clues = Clues {
            stage: stage.name.clone(),
            state,
            agent: agent.map(|start| start.identity.clone()),
            base,
        };
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
            StageState::Begun | StageState::Stuck | StageState::Lost => self.read_end(id),
            StageState::Delivered | StageState::Finished => self.advance(id),
            StageState::EndedShort => self.found_work(id),
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

/// A state as its JSON spells it, in words.
fn word(state: &impl serde::Serialize) -> String {
    serde_json::to_value(state)
        .ok()
        .and_then(|value| value.as_str().map(|word| word.replace('_', " ")))
        .unwrap_or_else(|| "unknown".into())
}

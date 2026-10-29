//! The display clues show gives a terminal (139): a place's current
//! stage and that stage's state, and the count of the places of its
//! repository on its base. The settle shows them, and status gives them
//! with where the terminal shows them, so a plugin of the terminal's
//! refreshes them by copying status and computing nothing (core.14). A
//! clue is display only: nothing flume decides reads one back.

use crate::start::Starts;
use crate::{BaseClues, Engine, PlaceClues};
use flume_seams::*;

impl Engine {
    /// The clues of a live place with a current stage, its agent's state
    /// read where `state` gives none; none for any other place.
    pub(crate) fn clues(&self, held: &HeldState, state: Option<AgentState>) -> Option<Clues> {
        if held.phase != Phase::Live {
            return None;
        }
        let index = held.current?;
        let stage = &held.stages[index];
        let agent = stage.start.as_ref().filter(|start| start.live);
        let state = match agent {
            Some(_) if stage.state == StageState::Starting => "starting".to_string(),
            Some(start) => {
                word(&state.unwrap_or_else(|| self.read_agent(&held.place.id, start).state))
            }
            None => word(&stage.state),
        };
        Some(Clues {
            stage: stage.name.clone(),
            state,
            agent: agent.map(|start| start.identity.clone()),
            base: BaseCount::default(),
        })
    }

    /// The place's clues as status gives them, with where the terminal
    /// shows them; its agent's state as status read it.
    pub(crate) fn place_clues(
        &self,
        held: &HeldState,
        state: Option<AgentState>,
    ) -> Option<PlaceClues> {
        let clues = self.clues(held, state)?;
        let mut shown_at = self
            .terminal
            .shown_at(&held.place.place_ref(), clues.agent.as_ref());
        shown_at.base = None;
        Some(PlaceClues {
            title: format!("{} · {}", clues.stage, clues.state),
            display_agent: held.place.short(),
            stage: clues.stage,
            state: clues.state,
            shown_at,
        })
    }

    /// The count of the live places of `place`'s repository on its base,
    /// by what they do (139).
    pub(crate) fn base_count(&self, place: &PlaceRecord) -> BaseCount {
        let mut base = BaseCount::default();
        for other in self.store.read_every() {
            let other = other.state;
            if other.phase == Phase::Live
                && other.place.repository == place.repository
                && other.place.base == place.base
            {
                self.count(&mut base, &other);
            }
        }
        base
    }

    /// Counts one live place in its base's count.
    fn count(&self, base: &mut BaseCount, held: &HeldState) {
        if let Some(waiting) = &held.waiting {
            if waiting.stalled {
                base.stalled += 1;
            } else {
                base.waiting_on_caller += 1;
            }
        } else if let Some((_, start)) = held.live_start() {
            if self.read_agent(&held.place.id, start).state == AgentState::Asking {
                base.asking += 1;
            } else {
                base.running += 1;
            }
        }
    }

    /// Every repository's base a live place stands on, with its count, the
    /// count's one line as show gives it, and where the terminal shows it
    /// (139).
    pub(crate) fn bases(&self) -> Vec<BaseClues> {
        let mut bases: Vec<(PlaceRef, BaseCount)> = vec![];
        for read in self.store.read_every() {
            let held = read.state;
            if held.phase != Phase::Live {
                continue;
            }
            let at = held.place.place_ref();
            let index = match bases
                .iter()
                .position(|(place, _)| place.repository == at.repository && place.base == at.base)
            {
                Some(index) => index,
                None => {
                    bases.push((at, BaseCount::default()));
                    bases.len() - 1
                }
            };
            self.count(&mut bases[index].1, &held);
        }
        bases.sort_by(|a, b| (&a.0.repository, &a.0.base).cmp(&(&b.0.repository, &b.0.base)));
        bases
            .into_iter()
            .map(|(place, count)| BaseClues {
                shown_at: self.terminal.shown_at(&place, None).base,
                repository: place.repository,
                base: place.base,
                line: count.to_string(),
                count,
            })
            .collect()
    }
}

/// A state as its JSON spells it, in words.
pub(crate) fn word(state: &impl serde::Serialize) -> String {
    serde_json::to_value(state)
        .ok()
        .and_then(|value| value.as_str().map(|word| word.replace('_', " ")))
        .unwrap_or_else(|| "unknown".into())
}

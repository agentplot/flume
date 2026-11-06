//! The display clues show gives a terminal (139): a place's current
//! stage and that stage's state, and the count of the places of its
//! repository on its base. The settle shows them, and status gives them
//! with where the terminal shows them, so a plugin of the terminal's
//! refreshes them by copying status and computing nothing (core.14). A
//! clue is display only: nothing flume decides reads one back.

use crate::start::Starts;
use crate::terminals::{add, Terminals};
use crate::{label, side_by_side, BaseClues, Engine, GroupStatus, PlaceClues};
use flume_seams::*;
use std::collections::BTreeMap;

impl Engine {
    /// The clues of a live place with a current stage, its agent's state
    /// read where `state` gives none, a repeating stage named with
    /// `group`'s place among the change's groups, `code 3/7`; none for any
    /// other place.
    pub(crate) fn clues(
        &self,
        held: &HeldState,
        state: Option<AgentState>,
        group: Option<&GroupStatus>,
    ) -> Option<Clues> {
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
            stage: label(&stage.name, group),
            state,
            agent: agent.map(|start| start.identity.clone()),
            base: BaseCount::default(),
        })
    }

    /// The place's clues as status gives them, with where the terminal
    /// shows them; its agent's state and its current stage's group as
    /// status read them.
    pub(crate) fn place_clues(
        &self,
        held: &HeldState,
        state: Option<AgentState>,
        group: Option<&GroupStatus>,
    ) -> Option<PlaceClues> {
        let clues = self.clues(held, state, group)?;
        let mut shown_at = self
            .place_terminal(held)
            .map(|terminal| terminal.shown_at(&held.place.place_ref(), clues.agent.as_ref()))
            .unwrap_or_default();
        shown_at.base = None;
        shown_at.bases.clear();
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
                let state = self.counted_state(&other, &BTreeMap::new());
                count(&mut base, &other, state);
            }
        }
        base
    }

    /// The state a live place's agent is counted by: none where the place
    /// waits or has no live agent, else the state in `agents` read for
    /// the place, or read now where none was (139).
    fn counted_state(
        &self,
        held: &HeldState,
        agents: &BTreeMap<PlaceId, AgentState>,
    ) -> Option<AgentState> {
        if held.waiting.is_some() {
            return None;
        }
        let (_, start) = held.live_start()?;
        Some(match agents.get(&held.place.id) {
            Some(state) => *state,
            None => self.read_agent(&held.place.id, start).state,
        })
    }

    /// Every repository's base a live place stands on, with its count, the
    /// count's one line as show gives it, and where the terminal shows it:
    /// once for each of the base's workspaces each terminal its places
    /// run in names, each with the one count, and once with what a
    /// terminal names of it where it names no workspace of each session
    /// (139, 190; record 0031). Each agent is counted by its state in
    /// `agents`, and an agent status did not read is read once, the
    /// places side by side (58).
    pub(crate) fn bases(&self, agents: &BTreeMap<PlaceId, AgentState>) -> Vec<BaseClues> {
        let live: Vec<HeldState> = self
            .store
            .read_every()
            .into_iter()
            .map(|read| read.state)
            .filter(|held| held.phase == Phase::Live)
            .collect();
        let states = side_by_side(&live, |held| self.counted_state(held, agents));
        let mut bases: Vec<(PlaceRef, BaseCount, Terminals)> = vec![];
        for (held, state) in live.iter().zip(states) {
            let at = held.place.place_ref();
            let index = match bases.iter().position(|(place, _, _)| {
                place.repository == at.repository && place.base == at.base
            }) {
                Some(index) => index,
                None => {
                    bases.push((at, BaseCount::default(), vec![]));
                    bases.len() - 1
                }
            };
            count(&mut bases[index].1, held, state);
            if let Ok(terminal) = self.place_terminal(held) {
                add(&mut bases[index].2, terminal);
            }
        }
        bases.sort_by(|a, b| (&a.0.repository, &a.0.base).cmp(&(&b.0.repository, &b.0.base)));
        bases
            .into_iter()
            .flat_map(|(place, count, mut terminals)| {
                if terminals.is_empty() {
                    terminals.push(self.terminal.clone());
                }
                let at: Vec<Option<String>> = terminals
                    .iter()
                    .flat_map(|terminal| {
                        let shown = terminal.shown_at(&place, None);
                        if shown.bases.is_empty() {
                            vec![shown.base]
                        } else {
                            shown.bases.into_iter().map(Some).collect()
                        }
                    })
                    .collect();
                at.into_iter().map(move |shown_at| BaseClues {
                    shown_at,
                    repository: place.repository.clone(),
                    base: place.base.clone(),
                    line: count.to_string(),
                    count,
                })
            })
            .collect()
    }
}

/// Counts one live place in its base's count, its agent by `state`, the
/// state it is counted by.
fn count(base: &mut BaseCount, held: &HeldState, state: Option<AgentState>) {
    if let Some(waiting) = &held.waiting {
        if waiting.stalled {
            base.stalled += 1;
        } else {
            base.waiting_on_caller += 1;
        }
    } else if let Some(state) = state {
        if state == AgentState::Asking {
            base.asking += 1;
        } else {
            base.running += 1;
        }
    }
}

/// A state as its JSON spells it, in words.
pub(crate) fn word(state: &impl serde::Serialize) -> String {
    serde_json::to_value(state)
        .ok()
        .and_then(|value| value.as_str().map(|word| word.replace('_', " ")))
        .unwrap_or_else(|| "unknown".into())
}

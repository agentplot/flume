//! The display clues show gives a terminal (139): a place's current
//! stage and that stage's state, and the count of the places of its
//! repository on its base. The settle shows them, and status gives them
//! with where the terminal shows them, so a plugin of the terminal's
//! refreshes them by copying status and computing nothing (core.14). A
//! clue is display only: nothing flume decides reads one back.

use crate::start::Starts;
use crate::terminals::{add, Terminals};
use crate::{label, BaseClues, Engine, GroupStatus, PlaceClues};
use flume_seams::*;

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
    /// count's one line as show gives it, and where the terminal shows it:
    /// once for each of the base's workspaces each terminal its places
    /// run in names, each with the one count, and once with what a
    /// terminal names of it where it names no workspace of each session
    /// (139, 190; record 0031).
    pub(crate) fn bases(&self) -> Vec<BaseClues> {
        let mut bases: Vec<(PlaceRef, BaseCount, Terminals)> = vec![];
        for read in self.store.read_every() {
            let held = read.state;
            if held.phase != Phase::Live {
                continue;
            }
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
            self.count(&mut bases[index].1, &held);
            if let Ok(terminal) = self.place_terminal(&held) {
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

/// A state as its JSON spells it, in words.
pub(crate) fn word(state: &impl serde::Serialize) -> String {
    serde_json::to_value(state)
        .ok()
        .and_then(|value| value.as_str().map(|word| word.replace('_', " ")))
        .unwrap_or_else(|| "unknown".into())
}

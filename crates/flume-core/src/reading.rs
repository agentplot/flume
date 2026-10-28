//! Reading a stage's deliverable at one head of the place's branch (24,
//! 25), and an agent's state from the terminal and the agent seam (31,
//! 77).

use crate::{Bound, Engine};
use flume_seams::*;

/// The facts of a place read at one head.
pub(crate) struct Facts {
    pub head: Revision,
    /// Whether the stage's deliverable exists, made since it started.
    pub delivered: bool,
    /// The report it delivered, for a report form.
    pub report: Option<String>,
    pub change: Option<ChangeRead>,
}

/// An agent's state as flume reads it.
pub(crate) struct AgentReading {
    pub state: AgentState,
    pub asking: Reading,
    pub own_work: Reading,
    /// Whether the agent under the start's identity keeps the record its
    /// start began with: no where it keeps another (77).
    pub own_record: Reading,
}

impl AgentReading {
    /// Neither working, asking, nor waiting on own work; anything flume
    /// cannot read is not quiet (31).
    pub fn quiet(&self) -> bool {
        matches!(self.state, AgentState::Waiting | AgentState::Stopped)
            && self.asking == Reading::No
            && self.own_work == Reading::No
            && self.own_record != Reading::Unknown
    }

    /// What flume could not read of the agent (31).
    pub fn unread(&self) -> Vec<String> {
        let mut unread = vec![];
        if self.state == AgentState::Unknown {
            unread.push("the agent's state".to_string());
        }
        if self.asking == Reading::Unknown {
            unread.push("whether the agent is asking".to_string());
        }
        if self.own_work == Reading::Unknown {
            unread.push("whether the agent's own work runs".to_string());
        }
        if self.own_record == Reading::Unknown {
            unread.push("the agent's own record".to_string());
        }
        unread
    }
}

/// Whether a kind reads a change (3, 4).
pub(crate) fn reads_change(kind: &Kind) -> bool {
    kind.stages.iter().any(|stage| {
        stage.deliverables.iter().any(|deliverable| {
            matches!(
                deliverable,
                Deliverable::ChangeWritten | Deliverable::TasksTicked | Deliverable::ChangeArchived
            )
        })
    })
}

impl Engine {
    /// The place's head, the deliverable of the stage at `index` and the
    /// place's change, every fact read at one head: if the head moved by
    /// the end of the read, every fact is read again (25).
    pub(crate) fn facts(&self, held: &HeldState, bound: &Bound, index: Option<usize>) -> Facts {
        let place = &held.place;
        let head_now = || {
            self.history
                .head(&place.repository, &place.branch)
                .unwrap_or_else(|| place.base_revision.clone())
        };
        let mut head = head_now();
        loop {
            let change = match (&bound.change, reads_change(&place.kind)) {
                (Some(change), true) => {
                    Some(change.read_change(&place.repository, &head, &place.name))
                }
                _ => None,
            };
            let (delivered, report) = match index {
                Some(index) => self.delivered(held, bound, index, &head, change.as_ref()),
                None => (false, None),
            };
            let again = head_now();
            if again == head {
                return Facts {
                    head,
                    delivered,
                    report,
                    change,
                };
            }
            head = again;
        }
    }

    /// Whether the stage's deliverable exists at `head`, made since its
    /// start, and the report it is, for a report form (24, 25).
    fn delivered(
        &self,
        held: &HeldState,
        bound: &Bound,
        index: usize,
        head: &Revision,
        change: Option<&ChangeRead>,
    ) -> (bool, Option<String>) {
        let place = &held.place;
        let stage = &place.kind.stages[index];
        let Some(start) = held.stages[index].start.as_ref() else {
            return (false, None);
        };
        let since = &start.start_head;
        let facts = match change {
            Some(ChangeRead::Read { facts }) => Some(facts),
            _ => None,
        };
        let touched = || {
            bound.change.as_ref().is_some_and(|change| {
                self.history
                    .newest_touching(&place.repository, head, since, &change.path(&place.name))
                    .is_some_and(|touch| touch.since)
            })
        };
        match stage.deliverables.first() {
            None | Some(Deliverable::Unreadable { .. }) => (false, None),
            Some(Deliverable::Commits) => (
                self.history.holds_since(&place.repository, head, since),
                None,
            ),
            Some(Deliverable::ChangeWritten) => (
                facts.is_some_and(|facts| facts.exists && facts.planning_complete) && touched(),
                None,
            ),
            Some(Deliverable::TasksTicked) => (
                facts.is_some_and(|facts| {
                    !facts.tasks.is_empty() && facts.tasks.iter().all(|task| task.ticked)
                }) && touched(),
                None,
            ),
            Some(Deliverable::ChangeArchived) => (
                facts.is_some_and(|facts| facts.archived)
                    && self.history.holds_since(&place.repository, head, since),
                None,
            ),
            Some(Deliverable::Report { path }) => {
                let path = self.report_path(place, path);
                let written = self
                    .host
                    .write_time(&path)
                    .is_some_and(|written| written > start.started_at);
                (written, written.then_some(path))
            }
            Some(Deliverable::CommittedReport { path }) => {
                let committed = self
                    .history
                    .in_commit_since(&place.repository, head, since, path);
                (committed, committed.then(|| path.clone()))
            }
        }
    }

    /// The agent of a place's start, as the terminal and the agent seam
    /// read it, the agent's record read in the place's own folder (128).
    pub(crate) fn read_agent(&self, place: &PlaceId, start: &StartHeld) -> AgentReading {
        let state = self.terminal.state(&start.identity);
        let agent = self.agent(&start.program);
        let folder = self.store.folder(place);
        let own_record = own_record(agent.as_deref(), &folder, start);
        match agent {
            Some(agent) => AgentReading {
                state,
                asking: agent.asking(&folder, &start.identity),
                own_work: agent.own_work(&folder, &start.identity),
                own_record,
            },
            None => AgentReading {
                state,
                asking: Reading::Unknown,
                own_work: Reading::Unknown,
                own_record,
            },
        }
    }
}

/// Whether the agent under the start's identity keeps the record its
/// start began with: an agent the terminal resumed is the stage's only
/// when both its identity and its own record are its start's, and a
/// record the agent seam cannot read is unknown, never taken as the
/// start's (77, 93). A start whose prompt no reading has named received
/// keeps no record yet, and has none to differ from.
fn own_record(agent: Option<&dyn Agent>, folder: &str, start: &StartHeld) -> Reading {
    let Some(kept) = &start.record else {
        return Reading::Yes;
    };
    let Some(agent) = agent else {
        return Reading::Unknown;
    };
    match agent.received(folder, &start.identity, &start.prompt.text) {
        Received::Received { record } | Received::NotReceived { record } if &record == kept => {
            Reading::Yes
        }
        Received::Received { .. } | Received::NotReceived { .. } => Reading::No,
        Received::Unknown => Reading::Unknown,
    }
}

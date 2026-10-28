use crate::{lock, FakeGit, Log};
use flume_seams::{Change, ChangeFacts, ChangeRead, Revision};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

/// The change seam over the fake git: a change's facts as set at a
/// commit hold at every commit after it on its first-parent line, as a
/// committed change does, until set again.
pub struct FakeChange {
    git: Arc<FakeGit>,
    state: Mutex<ChangeState>,
    pub log: Log,
}

#[derive(Default)]
struct ChangeState {
    facts: BTreeMap<(String, Revision, String), ChangeFacts>,
    unreadable: BTreeSet<String>,
}

impl FakeChange {
    pub fn new(git: Arc<FakeGit>) -> FakeChange {
        FakeChange {
            git,
            state: Mutex::default(),
            log: Log::default(),
        }
    }

    pub fn set(&self, repository: &str, revision: &Revision, name: &str, facts: ChangeFacts) {
        lock(&self.state)
            .facts
            .insert((repository.into(), revision.clone(), name.into()), facts);
    }

    pub fn unreadable(&self, repository: &str) {
        lock(&self.state).unreadable.insert(repository.into());
    }
}

impl Change for FakeChange {
    fn read_change(&self, repository: &str, revision: &Revision, name: &str) -> ChangeRead {
        self.log
            .push(format!("read_change {repository} {revision} {name}"));
        let line = self.git.first_parents(revision);
        let state = lock(&self.state);
        if state.unreadable.contains(repository) {
            return ChangeRead::CouldNotRead {
                reason: format!("the change {name} could not be read"),
            };
        }
        let facts = line
            .iter()
            .find_map(|at| {
                state
                    .facts
                    .get(&(repository.to_string(), at.clone(), name.to_string()))
            })
            .cloned()
            .unwrap_or_default();
        ChangeRead::Read { facts }
    }

    fn path(&self, name: &str) -> String {
        format!("plans/{name}")
    }
}

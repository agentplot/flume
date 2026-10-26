use crate::{lock, FakeGit, Log};
use flume_seams::{
    BranchRemoved, Holds, MakeOutcome, Place, PlaceRead, PlaceRef, Readiness, ReadyStep, Removed,
    Revision,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

/// The place seam over the fake git: worktrees by location, each
/// repository's readiness, what each place holds and how its teardown
/// goes.
pub struct FakePlace {
    git: Arc<FakeGit>,
    state: Mutex<PlaceState>,
    pub log: Log,
}

#[derive(Default)]
struct PlaceState {
    worktrees: BTreeMap<String, (String, String)>,
    occupied: BTreeSet<String>,
    readiness: BTreeMap<String, Vec<ReadyStep>>,
    holds: BTreeMap<String, Holds>,
    teardown: BTreeMap<String, (String, String)>,
    unreadable: BTreeMap<String, String>,
    refusal: Option<String>,
}

impl FakePlace {
    pub fn new(git: Arc<FakeGit>) -> FakePlace {
        FakePlace {
            git,
            state: Mutex::default(),
            log: Log::default(),
        }
    }

    /// The readiness steps `repository` declares, each passing or failing
    /// with its words.
    pub fn set_readiness(&self, repository: &str, steps: &[(&str, bool, &str)]) {
        lock(&self.state).readiness.insert(
            repository.into(),
            steps
                .iter()
                .map(|(step, passed, words)| ReadyStep {
                    step: step.to_string(),
                    passed: *passed,
                    words: words.to_string(),
                })
                .collect(),
        );
    }

    /// Something stands at `location` that no worktree of flume's made.
    pub fn occupy(&self, location: &str) {
        lock(&self.state).occupied.insert(location.into());
    }

    pub fn set_holds(&self, location: &str, holds: Holds) {
        lock(&self.state).holds.insert(location.into(), holds);
    }

    /// The place's teardown fails at `step` with `words`.
    pub fn fail_teardown(&self, location: &str, step: &str, words: &str) {
        lock(&self.state)
            .teardown
            .insert(location.into(), (step.into(), words.into()));
    }

    pub fn mend_teardown(&self, location: &str) {
        lock(&self.state).teardown.remove(location);
    }

    pub fn unreadable(&self, location: &str, reason: &str) {
        lock(&self.state)
            .unreadable
            .insert(location.into(), reason.into());
    }

    /// Every make refused with `reason`, as a full disk would.
    pub fn refuse_makes(&self, reason: &str) {
        lock(&self.state).refusal = Some(reason.into());
    }

    /// The worktree at `location` removed through its teardown by another
    /// than flume, as a merge queue that removes a landed place does.
    pub fn vanish(&self, location: &str) {
        let mut state = lock(&self.state);
        state.worktrees.remove(location);
        state.holds.remove(location);
    }

    pub fn exists(&self, location: &str) -> bool {
        lock(&self.state).worktrees.contains_key(location)
    }
}

impl Place for FakePlace {
    fn make(
        &self,
        repository: &str,
        revision: &Revision,
        branch: &str,
        location: &str,
    ) -> MakeOutcome {
        self.log
            .push(format!("make {repository} {revision} {branch} {location}"));
        let mut state = lock(&self.state);
        if let Some(reason) = &state.refusal {
            return MakeOutcome::Refused {
                reason: reason.clone(),
            };
        }
        if self.git.branch_exists(repository, branch) {
            return MakeOutcome::BranchExists;
        }
        if state.worktrees.contains_key(location) || state.occupied.contains(location) {
            return MakeOutcome::LocationExists;
        }
        self.git.set_branch(repository, branch, revision);
        state
            .worktrees
            .insert(location.into(), (repository.into(), branch.into()));
        MakeOutcome::Made
    }

    fn ready(&self, place: &PlaceRef, revision: &Revision) -> Readiness {
        self.log
            .push(format!("ready {} {revision}", place.location));
        let state = lock(&self.state);
        let mut steps = vec![];
        for step in state
            .readiness
            .get(&place.repository)
            .cloned()
            .unwrap_or_default()
        {
            let passed = step.passed;
            steps.push(step);
            if !passed {
                break;
            }
        }
        Readiness { steps }
    }

    fn read(&self, place: &PlaceRef) -> PlaceRead {
        self.log.push(format!("read {}", place.location));
        let state = lock(&self.state);
        if let Some(reason) = state.unreadable.get(&place.location) {
            return PlaceRead::CouldNotRead {
                reason: reason.clone(),
            };
        }
        let Some((repository, branch)) = state.worktrees.get(&place.location) else {
            return PlaceRead::Gone;
        };
        let Some(head) = self.git.branch_head(repository, branch) else {
            return PlaceRead::Gone;
        };
        PlaceRead::Live {
            branch: branch.clone(),
            head,
            holds: state
                .holds
                .get(&place.location)
                .cloned()
                .unwrap_or_default(),
        }
    }

    fn remove(&self, place: &PlaceRef) -> Removed {
        self.log.push(format!("remove {}", place.location));
        let mut state = lock(&self.state);
        if let Some((step, words)) = state.teardown.get(&place.location) {
            return Removed::Refused {
                step: step.clone(),
                words: words.clone(),
            };
        }
        state.worktrees.remove(&place.location);
        state.holds.remove(&place.location);
        Removed::Removed
    }

    fn remove_branch(&self, repository: &str, branch: &str, base: &str) -> BranchRemoved {
        self.log
            .push(format!("remove_branch {repository} {branch} {base}"));
        if !self.git.branch_exists(repository, branch) {
            return BranchRemoved::Gone;
        }
        let commits = self.git.lacking(repository, branch, base);
        if !commits.is_empty() {
            return BranchRemoved::Kept { commits };
        }
        self.git.delete_branch(repository, branch);
        BranchRemoved::Removed
    }
}

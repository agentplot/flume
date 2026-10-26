use crate::{lock, FakeGit, FakePlace, Log};
use flume_seams::{
    BaseRevision, MergeQueue, PlaceRef, Revision, Submission, SubmissionRead, SubmissionState,
    Verdict,
};
use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex};

/// What the fake merge queue does with the next submission or wait.
#[derive(Clone, Debug)]
pub enum Plan {
    /// It lands, merged into its base.
    Land,
    Fail(String),
    Withdraw,
    Refuse(String),
    /// It stays under test, and the command waiting on it is killed.
    Hang,
}

/// The merge queue seam over the fake git: where each base lives, what
/// each submission and wait comes to, and every submission of each
/// branch, whoever made it.
pub struct FakeMergeQueue {
    git: Arc<FakeGit>,
    place: Option<Arc<FakePlace>>,
    state: Mutex<QueueState>,
    pub log: Log,
}

#[derive(Default)]
struct QueueState {
    bases: BTreeMap<(String, String), BaseRevision>,
    submits: VecDeque<Plan>,
    waits: VecDeque<Plan>,
    submissions: BTreeMap<(String, String), Vec<Submission>>,
    unreadable: Option<String>,
    removes_places: bool,
    next: u64,
}

impl FakeMergeQueue {
    /// A merge queue that, given the fake place, removes a landed place
    /// through it once told to.
    pub fn new(git: Arc<FakeGit>, place: Option<Arc<FakePlace>>) -> FakeMergeQueue {
        FakeMergeQueue {
            git,
            place,
            state: Mutex::default(),
            log: Log::default(),
        }
    }

    pub fn set_base(&self, repository: &str, base: &str, answer: BaseRevision) {
        lock(&self.state)
            .bases
            .insert((repository.into(), base.into()), answer);
    }

    /// What the next submissions come to, in order; each one not planned
    /// lands.
    pub fn plan_submits(&self, plans: Vec<Plan>) {
        lock(&self.state).submits.extend(plans);
    }

    pub fn plan_waits(&self, plans: Vec<Plan>) {
        lock(&self.state).waits.extend(plans);
    }

    pub fn unreadable(&self, reason: &str) {
        lock(&self.state).unreadable = Some(reason.into());
    }

    pub fn readable(&self) {
        lock(&self.state).unreadable = None;
    }

    /// A landed place is removed through its teardown by the merge queue.
    pub fn removes_places(&self) {
        lock(&self.state).removes_places = true;
    }

    /// A hand's submission of `branch` to `target`, in `state`.
    pub fn hand_submission(
        &self,
        repository: &str,
        branch: &str,
        target: &str,
        state: SubmissionState,
    ) -> Submission {
        let mut queue = lock(&self.state);
        Self::record(&mut queue, repository, branch, target, state)
    }

    /// A hand's submission of `branch` to `target` that lands.
    pub fn hand_landing(&self, repository: &str, branch: &str, target: &str) -> Revision {
        let head = self.git.branch_head(repository, branch).expect("branch");
        let commit = self.git.merge(repository, target, &head);
        let mut queue = lock(&self.state);
        Self::record(
            &mut queue,
            repository,
            branch,
            target,
            SubmissionState::Landed {
                commit: commit.clone(),
            },
        );
        commit
    }

    pub fn submissions(&self, repository: &str, branch: &str) -> Vec<Submission> {
        lock(&self.state)
            .submissions
            .get(&(repository.to_string(), branch.to_string()))
            .cloned()
            .unwrap_or_default()
    }

    fn record(
        queue: &mut QueueState,
        repository: &str,
        branch: &str,
        target: &str,
        state: SubmissionState,
    ) -> Submission {
        queue.next += 1;
        let submission = Submission {
            id: queue.next.to_string(),
            branch: branch.into(),
            target: target.into(),
            state,
        };
        queue
            .submissions
            .entry((repository.into(), branch.into()))
            .or_default()
            .push(submission.clone());
        submission
    }

    fn set_state(&self, repository: &str, submission: &str, branch: &str, state: SubmissionState) {
        let mut queue = lock(&self.state);
        if let Some(found) = queue
            .submissions
            .get_mut(&(repository.to_string(), branch.to_string()))
            .and_then(|all| all.iter_mut().find(|one| one.id == submission))
        {
            found.state = state;
        }
    }

    fn decide(
        &self,
        repository: &str,
        location: &str,
        submission: &Submission,
        plan: Plan,
    ) -> Verdict {
        match plan {
            Plan::Land => {
                let head = self
                    .git
                    .branch_head(repository, &submission.branch)
                    .expect("a submitted branch");
                let commit = self.git.merge(repository, &submission.target, &head);
                let removes = lock(&self.state).removes_places;
                let place_removed = match (&self.place, removes) {
                    (Some(place), true) => {
                        place.vanish(location);
                        true
                    }
                    _ => false,
                };
                self.set_state(
                    repository,
                    &submission.id,
                    &submission.branch,
                    SubmissionState::Landed {
                        commit: commit.clone(),
                    },
                );
                Verdict::Landed {
                    commit,
                    place_removed,
                    branch_removed: false,
                }
            }
            Plan::Fail(words) => {
                self.set_state(
                    repository,
                    &submission.id,
                    &submission.branch,
                    SubmissionState::Failed {
                        words: words.clone(),
                    },
                );
                Verdict::Failed { words }
            }
            Plan::Withdraw => {
                self.set_state(
                    repository,
                    &submission.id,
                    &submission.branch,
                    SubmissionState::Withdrawn,
                );
                Verdict::Withdrawn
            }
            Plan::Refuse(reason) => Verdict::Refused { reason },
            Plan::Hang => panic!(
                "the command waiting on submission {} was killed",
                submission.id
            ),
        }
    }
}

impl MergeQueue for FakeMergeQueue {
    fn submit(&self, place: &PlaceRef, head: &Revision) -> Verdict {
        self.log
            .push(format!("submit {} {head} {}", place.branch, place.base));
        let plan = lock(&self.state).submits.pop_front().unwrap_or(Plan::Land);
        if let Plan::Refuse(reason) = plan {
            return Verdict::Refused { reason };
        }
        let submission = {
            let mut queue = lock(&self.state);
            Self::record(
                &mut queue,
                &place.repository,
                &place.branch,
                &place.base,
                SubmissionState::UnderTest,
            )
        };
        self.decide(&place.repository, &place.location, &submission, plan)
    }

    fn read_submission(&self, repository: &str, branch: &str) -> SubmissionRead {
        self.log
            .push(format!("read_submission {repository} {branch}"));
        let queue = lock(&self.state);
        if let Some(reason) = &queue.unreadable {
            return SubmissionRead::CouldNotRead {
                reason: reason.clone(),
            };
        }
        match queue
            .submissions
            .get(&(repository.to_string(), branch.to_string()))
            .and_then(|all| all.last())
        {
            Some(submission) => SubmissionRead::Found {
                submission: submission.clone(),
            },
            None => SubmissionRead::None,
        }
    }

    fn wait(&self, repository: &str, submission: &Submission) -> Verdict {
        self.log
            .push(format!("wait {repository} {}", submission.id));
        let plan = lock(&self.state).waits.pop_front().unwrap_or(Plan::Land);
        let location = String::new();
        self.decide(repository, &location, submission, plan)
    }

    fn base_revision(&self, repository: &str, base: &str) -> BaseRevision {
        self.log.push(format!("base_revision {repository} {base}"));
        if let Some(answer) = lock(&self.state)
            .bases
            .get(&(repository.to_string(), base.to_string()))
        {
            return answer.clone();
        }
        match self.git.branch_head(repository, base) {
            Some(revision) => BaseRevision::At { revision },
            None => BaseRevision::Refused {
                reason: format!("no branch {base} in {repository}"),
            },
        }
    }
}

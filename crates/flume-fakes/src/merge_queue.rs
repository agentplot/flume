use crate::{lock, FakeGit, Log};
use flume_seams::{
    BaseRevision, Inbox, MergeQueue, PlaceRef, Revision, Stepped, Submission, SubmissionOutcome,
    SubmissionRead, SubmissionState, Submitted,
};
use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex};

/// What the fake merge queue does with a submission: refused or decided
/// at submit, or held and decided by the round that next reaches it.
#[derive(Clone, Debug)]
pub enum Plan {
    /// It lands, merged into its base.
    Land,
    Fail(String),
    /// It conflicts with its base on the files named, with the words.
    Conflict(Vec<String>, String),
    /// Submit finds it in conflict with its base as it takes it in, on
    /// the files named, and answers with the outcome, which it delivers
    /// so no inbox gives it (133).
    ConflictAtSubmit(Vec<String>, String),
    /// It is withdrawn, which no inbox gives (99).
    Withdraw,
    /// Submit refuses it with the reason.
    Refuse(String),
    /// It stays open through every round.
    Hold,
}

/// The merge queue seam over the fake git: where each base lives, what
/// each submission comes to, every submission of each branch whoever
/// made it, and each place's pending outcomes. Submit holds no wait: a
/// submission is decided by a round, which `step` drives unless the
/// queue moves on its own, and its outcome is delivered once through
/// the place's inbox. It removes nothing of a place (113).
pub struct FakeMergeQueue {
    git: Arc<FakeGit>,
    state: Mutex<QueueState>,
    pub log: Log,
}

#[derive(Default)]
struct QueueState {
    bases: BTreeMap<(String, String), BaseRevision>,
    submits: VecDeque<Plan>,
    submissions: BTreeMap<(String, String), Vec<Submission>>,
    /// Each submission's plan and the place it came from, by id.
    plans: BTreeMap<String, (Plan, Option<String>)>,
    /// Each place's outcomes not yet delivered, by location.
    inboxes: BTreeMap<String, Vec<SubmissionOutcome>>,
    unreadable: Option<String>,
    moves_on_its_own: bool,
    next: u64,
}

impl FakeMergeQueue {
    pub fn new(git: Arc<FakeGit>) -> FakeMergeQueue {
        FakeMergeQueue {
            git,
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

    /// Submit, inbox, step and read-submission each answer that the merge
    /// queue could not be read.
    pub fn unreadable(&self, reason: &str) {
        lock(&self.state).unreadable = Some(reason.into());
    }

    pub fn readable(&self) {
        lock(&self.state).unreadable = None;
    }

    /// The queue moves on its own, as one a driver runs: step drives
    /// nothing, and a test runs each round with `run_round`.
    pub fn moves_on_its_own(&self) {
        lock(&self.state).moves_on_its_own = true;
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
        Self::record(&mut queue, repository, branch, target, state, None)
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
            None,
        );
        commit
    }

    /// The newest open submission of `branch` decided now by `plan`, as a
    /// round the queue ran on its own would.
    pub fn decide(&self, repository: &str, branch: &str, plan: Plan) {
        let open = self
            .submissions(repository, branch)
            .into_iter()
            .rev()
            .find(|submission| submission.state.open());
        if let Some(submission) = open {
            lock(&self.state)
                .plans
                .entry(submission.id.clone())
                .or_insert((Plan::Land, None))
                .0 = plan;
            self.settle(repository, &submission);
        }
    }

    /// One round of `base`'s queue: every open submission to it decided by
    /// its plan, a held one left open.
    pub fn run_round(&self, repository: &str, base: &str) {
        let open: Vec<Submission> = lock(&self.state)
            .submissions
            .iter()
            .filter(|((at, _), _)| at == repository)
            .flat_map(|(_, all)| all.iter())
            .filter(|submission| submission.target == base && submission.state.open())
            .cloned()
            .collect();
        for submission in open {
            self.settle(repository, &submission);
        }
    }

    pub fn submissions(&self, repository: &str, branch: &str) -> Vec<Submission> {
        lock(&self.state)
            .submissions
            .get(&(repository.to_string(), branch.to_string()))
            .cloned()
            .unwrap_or_default()
    }

    /// The outcomes pending for the place at `location`, undelivered.
    pub fn pending(&self, location: &str) -> Vec<SubmissionOutcome> {
        lock(&self.state)
            .inboxes
            .get(location)
            .cloned()
            .unwrap_or_default()
    }

    fn record(
        queue: &mut QueueState,
        repository: &str,
        branch: &str,
        target: &str,
        state: SubmissionState,
        from: Option<(Plan, String)>,
    ) -> Submission {
        queue.next += 1;
        let submission = Submission {
            id: queue.next.to_string(),
            branch: branch.into(),
            target: target.into(),
            state,
        };
        if let Some((plan, location)) = from {
            queue
                .plans
                .insert(submission.id.clone(), (plan, Some(location)));
        }
        queue
            .submissions
            .entry((repository.into(), branch.into()))
            .or_default()
            .push(submission.clone());
        submission
    }

    fn set_state(&self, repository: &str, submission: &Submission, state: SubmissionState) {
        let mut queue = lock(&self.state);
        if let Some(found) = queue
            .submissions
            .get_mut(&(repository.to_string(), submission.branch.clone()))
            .and_then(|all| all.iter_mut().find(|one| one.id == submission.id))
        {
            found.state = state;
        }
    }

    /// Decides an open submission by its plan, and leaves its outcome in
    /// its place's inbox.
    fn settle(&self, repository: &str, submission: &Submission) {
        let (plan, location) = lock(&self.state)
            .plans
            .get(&submission.id)
            .cloned()
            .unwrap_or((Plan::Land, None));
        let (state, outcome) = match plan {
            Plan::Hold | Plan::Refuse(_) | Plan::ConflictAtSubmit(..) => return,
            Plan::Land => {
                let head = self
                    .git
                    .branch_head(repository, &submission.branch)
                    .expect("a submitted branch");
                let commit = self.git.merge(repository, &submission.target, &head);
                (
                    SubmissionState::Landed {
                        commit: commit.clone(),
                    },
                    Some(SubmissionOutcome::Landed { commit }),
                )
            }
            Plan::Fail(words) => (
                SubmissionState::Failed {
                    words: words.clone(),
                },
                Some(SubmissionOutcome::Failed { words }),
            ),
            Plan::Conflict(files, words) => {
                let base = self
                    .git
                    .branch_head(repository, &submission.target)
                    .expect("a base");
                (
                    SubmissionState::Conflict {
                        files: files.clone(),
                        words: words.clone(),
                    },
                    Some(SubmissionOutcome::Conflict { files, base, words }),
                )
            }
            Plan::Withdraw => (SubmissionState::Withdrawn, None),
        };
        self.set_state(repository, submission, state);
        if let (Some(outcome), Some(location)) = (outcome, location) {
            lock(&self.state)
                .inboxes
                .entry(location)
                .or_default()
                .push(outcome);
        }
    }
}

impl MergeQueue for FakeMergeQueue {
    fn submit(&self, place: &PlaceRef, head: &Revision) -> Submitted {
        self.log
            .push(format!("submit {} {head} {}", place.branch, place.base));
        let mut queue = lock(&self.state);
        if let Some(reason) = &queue.unreadable {
            return Submitted::Refused {
                reason: reason.clone(),
            };
        }
        let plan = queue.submits.pop_front().unwrap_or(Plan::Land);
        if let Plan::Refuse(reason) = plan {
            return Submitted::Refused { reason };
        }
        if let Plan::ConflictAtSubmit(files, words) = plan {
            let base = self
                .git
                .branch_head(&place.repository, &place.base)
                .expect("a base");
            let submission = Self::record(
                &mut queue,
                &place.repository,
                &place.branch,
                &place.base,
                SubmissionState::Conflict {
                    files: files.clone(),
                    words: words.clone(),
                },
                None,
            );
            return Submitted::Decided {
                submission,
                outcome: SubmissionOutcome::Conflict { files, base, words },
            };
        }
        let submission = Self::record(
            &mut queue,
            &place.repository,
            &place.branch,
            &place.base,
            SubmissionState::Waiting,
            Some((plan, place.location.clone())),
        );
        Submitted::Submitted { submission }
    }

    fn inbox(&self, place: &PlaceRef) -> Inbox {
        self.log.push(format!("inbox {}", place.location));
        let mut queue = lock(&self.state);
        if let Some(reason) = &queue.unreadable {
            return Inbox::CouldNotRead {
                reason: reason.clone(),
            };
        }
        Inbox::Outcomes {
            outcomes: queue.inboxes.remove(&place.location).unwrap_or_default(),
        }
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

    fn step(&self, repository: &str, base: &str) -> Stepped {
        self.log.push(format!("step {repository} {base}"));
        {
            let queue = lock(&self.state);
            if let Some(reason) = &queue.unreadable {
                return Stepped::CouldNotRead {
                    reason: reason.clone(),
                };
            }
            if queue.moves_on_its_own {
                return Stepped::MovesOnItsOwn;
            }
        }
        self.run_round(repository, base);
        Stepped::Stepped
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

use crate::{lock, Log};
use flume_seams::{History, Revision, Touch};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

/// The repositories' branches and commits, which the fake place, history,
/// change and merge queue share as real ones share git.
#[derive(Default)]
pub struct FakeGit {
    state: Mutex<GitState>,
}

#[derive(Default)]
struct GitState {
    next: u64,
    commits: BTreeMap<Revision, Node>,
    branches: BTreeMap<(String, String), Revision>,
    /// Commits whose change their base holds by another commit (67).
    applied: BTreeSet<Revision>,
    /// A branch's head moved once a number of further reads of it are
    /// made: a commit landing while the core reads (25).
    moves: BTreeMap<(String, String), (usize, Revision)>,
}

struct Node {
    parents: Vec<Revision>,
    touches: Vec<String>,
}

impl FakeGit {
    pub fn new() -> Arc<FakeGit> {
        Arc::new(FakeGit::default())
    }

    fn node(state: &mut GitState, parents: Vec<Revision>, touches: &[&str]) -> Revision {
        state.next += 1;
        let revision = Revision(format!("c{}", state.next));
        state.commits.insert(
            revision.clone(),
            Node {
                parents,
                touches: touches.iter().map(|path| path.to_string()).collect(),
            },
        );
        revision
    }

    /// A repository with `branch` at a first commit.
    pub fn init(&self, repository: &str, branch: &str) -> Revision {
        let mut state = lock(&self.state);
        let revision = Self::node(&mut state, vec![], &[]);
        state
            .branches
            .insert((repository.into(), branch.into()), revision.clone());
        revision
    }

    /// A commit on `branch` over its head, touching `touches`.
    pub fn commit(&self, repository: &str, branch: &str, touches: &[&str]) -> Revision {
        let mut state = lock(&self.state);
        let key = (repository.to_string(), branch.to_string());
        let parents = state.branches.get(&key).cloned().into_iter().collect();
        let revision = Self::node(&mut state, parents, touches);
        state.branches.insert(key, revision.clone());
        revision
    }

    /// `branch` made to point at `revision`, made or moved.
    pub fn set_branch(&self, repository: &str, branch: &str, revision: &Revision) {
        lock(&self.state)
            .branches
            .insert((repository.into(), branch.into()), revision.clone());
    }

    pub fn delete_branch(&self, repository: &str, branch: &str) {
        lock(&self.state)
            .branches
            .remove(&(repository.to_string(), branch.to_string()));
    }

    pub fn branch_head(&self, repository: &str, branch: &str) -> Option<Revision> {
        lock(&self.state)
            .branches
            .get(&(repository.to_string(), branch.to_string()))
            .cloned()
    }

    /// A merge of `from` into `target`: a commit on `target` whose
    /// parents are its head and `from`.
    pub fn merge(&self, repository: &str, target: &str, from: &Revision) -> Revision {
        let mut state = lock(&self.state);
        let key = (repository.to_string(), target.to_string());
        let mut parents: Vec<Revision> = state.branches.get(&key).cloned().into_iter().collect();
        parents.push(from.clone());
        let revision = Self::node(&mut state, parents, &[]);
        state.branches.insert(key, revision.clone());
        revision
    }

    /// The base holds `commit`'s change by a commit of its own, as a
    /// change applied by hand or squashed in (67).
    pub fn apply(&self, commit: &Revision) {
        lock(&self.state).applied.insert(commit.clone());
    }

    /// After `reads` further reads of its head, `branch` moves to
    /// `revision`.
    pub fn move_after_reads(
        &self,
        repository: &str,
        branch: &str,
        reads: usize,
        revision: &Revision,
    ) {
        lock(&self.state).moves.insert(
            (repository.into(), branch.into()),
            (reads, revision.clone()),
        );
    }

    fn reachable(state: &GitState, from: &Revision) -> BTreeSet<Revision> {
        let mut seen = BTreeSet::new();
        let mut todo = vec![from.clone()];
        while let Some(revision) = todo.pop() {
            if seen.insert(revision.clone()) {
                if let Some(node) = state.commits.get(&revision) {
                    todo.extend(node.parents.iter().cloned());
                }
            }
        }
        seen
    }

    /// The commits `head` reaches and `start` does not, newest first.
    pub fn since(&self, head: &Revision, start: &Revision) -> Vec<Revision> {
        let state = lock(&self.state);
        Self::since_in(&state, head, start)
    }

    fn since_in(state: &GitState, head: &Revision, start: &Revision) -> Vec<Revision> {
        let before = Self::reachable(state, start);
        let mut commits: Vec<Revision> = Self::reachable(state, head)
            .into_iter()
            .filter(|revision| !before.contains(revision))
            .collect();
        commits.sort_by_key(|revision| std::cmp::Reverse(Self::number(revision)));
        commits
    }

    fn number(revision: &Revision) -> u64 {
        revision.0.trim_start_matches('c').parse().unwrap_or(0)
    }

    /// The commits of `branch` whose change `base` lacks (67).
    pub fn lacking(&self, repository: &str, branch: &str, base: &str) -> Vec<Revision> {
        let state = lock(&self.state);
        let key = |name: &str| (repository.to_string(), name.to_string());
        let (Some(head), Some(base)) = (
            state.branches.get(&key(branch)),
            state.branches.get(&key(base)),
        ) else {
            return vec![];
        };
        Self::since_in(&state, head, base)
            .into_iter()
            .filter(|revision| {
                !state.applied.contains(revision)
                    && state
                        .commits
                        .get(revision)
                        .is_some_and(|node| node.parents.len() < 2)
            })
            .collect()
    }

    pub fn branch_exists(&self, repository: &str, branch: &str) -> bool {
        self.branch_head(repository, branch).is_some()
    }

    fn read_head(&self, repository: &str, branch: &str) -> Option<Revision> {
        let mut state = lock(&self.state);
        let key = (repository.to_string(), branch.to_string());
        let head = state.branches.get(&key).cloned();
        if let Some((reads, revision)) = state.moves.remove(&key) {
            if reads <= 1 {
                state.branches.insert(key, revision);
            } else {
                state.moves.insert(key, (reads - 1, revision));
            }
        }
        head
    }

    fn newest_touching(&self, head: &Revision, start: &Revision, path: &str) -> Option<Touch> {
        let state = lock(&self.state);
        let mut touching: Vec<Revision> = Self::reachable(&state, head)
            .into_iter()
            .filter(|revision| {
                state.commits.get(revision).is_some_and(|node| {
                    node.touches.iter().any(|touched| touched.starts_with(path))
                })
            })
            .collect();
        touching.sort_by_key(|revision| std::cmp::Reverse(Self::number(revision)));
        let commit = touching.into_iter().next()?;
        let since = !Self::reachable(&state, start).contains(&commit);
        Some(Touch { commit, since })
    }

    fn touched_since(&self, head: &Revision, start: &Revision, path: &str) -> bool {
        let state = lock(&self.state);
        Self::since_in(&state, head, start).iter().any(|revision| {
            state
                .commits
                .get(revision)
                .is_some_and(|node| node.touches.iter().any(|touched| touched == path))
        })
    }

    /// The first-parent line from `revision` back, itself first.
    pub fn first_parents(&self, revision: &Revision) -> Vec<Revision> {
        let state = lock(&self.state);
        let mut line = vec![];
        let mut at = Some(revision.clone());
        while let Some(revision) = at {
            at = state
                .commits
                .get(&revision)
                .and_then(|node| node.parents.first().cloned());
            line.push(revision);
        }
        line
    }
}

/// git's history over the fake git, each read logged.
pub struct FakeHistory {
    git: Arc<FakeGit>,
    pub log: Log,
}

impl FakeHistory {
    pub fn new(git: Arc<FakeGit>) -> FakeHistory {
        FakeHistory {
            git,
            log: Log::default(),
        }
    }
}

impl History for FakeHistory {
    fn head(&self, repository: &str, branch: &str) -> Option<Revision> {
        self.log.push(format!("head {repository} {branch}"));
        self.git.read_head(repository, branch)
    }

    fn holds_since(&self, repository: &str, head: &Revision, start: &Revision) -> bool {
        self.log
            .push(format!("holds_since {repository} {head} {start}"));
        !self.git.since(head, start).is_empty()
    }

    fn commits_since(&self, repository: &str, head: &Revision, start: &Revision) -> Vec<Revision> {
        self.log
            .push(format!("commits_since {repository} {head} {start}"));
        self.git.since(head, start)
    }

    fn newest_touching(
        &self,
        repository: &str,
        head: &Revision,
        start: &Revision,
        path: &str,
    ) -> Option<Touch> {
        self.log.push(format!(
            "newest_touching {repository} {head} {start} {path}"
        ));
        self.git.newest_touching(head, start, path)
    }

    fn in_commit_since(
        &self,
        repository: &str,
        head: &Revision,
        start: &Revision,
        path: &str,
    ) -> bool {
        self.log.push(format!(
            "in_commit_since {repository} {head} {start} {path}"
        ));
        self.git.touched_since(head, start, path)
    }

    /// The repository's own `.git`, as for a repository with its main
    /// worktree; unlogged, since every start reads it.
    fn common_dir(&self, repository: &str) -> Option<String> {
        Some(format!("{repository}/.git"))
    }

    fn valid_branch(&self, name: &str) -> bool {
        self.log.push(format!("valid_branch {name}"));
        !name.is_empty()
            && !name.contains("..")
            && !name.contains(' ')
            && !name.starts_with('/')
            && !name.ends_with('/')
            && !name.ends_with(".lock")
            && !name
                .chars()
                .any(|c| c.is_control() || "~^:?*[\\".contains(c))
    }
}

//! Git's history, read through `git` in the repository (section 9;
//! design 11): `rev-parse` for a head, `merge-base --is-ancestor` for a
//! commit since a start, `rev-list <start>..<head>` for the commits
//! since, `log -1 --format=%H -- <path>` for the newest
//! commit touching a path, `log <start>..<head> -- <path>` for a path in
//! a commit since, `rev-parse --git-common-dir` for the repository's
//! common git directory, `symbolic-ref` for its default branch,
//! `check-ref-format --branch`, and `fetch` from `origin` for a branch
//! as origin holds it, `ls-remote --exit-code` telling a branch origin
//! lacks from a fetch that failed (9, 24, 25, 33, 132, C.1). A revision
//! git cannot read holds nothing since and touches nothing.

use crate::git::{Git, REMOTE};
use flume_seams::{History, OriginBranch, Revision, Touch};

/// The core's reading of git's history, through git.
#[derive(Clone, Copy, Debug, Default)]
pub struct GitHistory;

/// Whether `commit` is an ancestor of `of`, or `of` itself; none where
/// git cannot say.
fn ancestor(git: &Git, commit: &str, of: &str) -> Option<bool> {
    git.answer(&["merge-base", "--is-ancestor", commit, of])
        .ok()
}

impl History for GitHistory {
    fn head(&self, repository: &str, branch: &str) -> Option<Revision> {
        Git::at(repository)
            .commit(&format!("refs/heads/{branch}"))
            .ok()
            .map(Revision::new)
    }

    /// `git fetch origin +refs/heads/<branch>:refs/remotes/origin/<branch>`
    /// and the branch at its tracking ref; where the fetch fails, origin
    /// lacks the branch only where `ls-remote --exit-code` exits 2, and a
    /// tracking ref left by an earlier fetch is never read (9, C.1).
    fn origin_branch(&self, repository: &str, branch: &str) -> OriginBranch {
        let git = Git::at(repository);
        match git.remotes() {
            Ok(remotes) if remotes.iter().any(|remote| remote == REMOTE) => {}
            Ok(_) => return OriginBranch::NoOrigin,
            Err(reason) => return OriginBranch::Refused { reason },
        }
        let tracking = match git.fetch_branch(REMOTE, branch) {
            Ok(tracking) => tracking,
            Err(reason) => {
                return match git.remote_holds(REMOTE, branch) {
                    Ok(false) => OriginBranch::Lacks,
                    Ok(true) | Err(_) => OriginBranch::Refused { reason },
                }
            }
        };
        match git.commit(&tracking) {
            Ok(revision) => OriginBranch::At {
                revision: Revision::new(revision),
            },
            Err(reason) => OriginBranch::Refused { reason },
        }
    }

    fn holds_since(&self, repository: &str, head: &Revision, start: &Revision) -> bool {
        // `head` holds a commit `start` lacks exactly where it is not
        // `start`'s ancestor (25).
        ancestor(&Git::at(repository), &head.0, &start.0) == Some(false)
    }

    fn commits_since(&self, repository: &str, head: &Revision, start: &Revision) -> Vec<Revision> {
        Git::at(repository)
            .text(&["rev-list", &format!("{}..{}", start.0, head.0)])
            .map(|commits| commits.lines().map(Revision::new).collect())
            .unwrap_or_default()
    }

    fn newest_touching(
        &self,
        repository: &str,
        head: &Revision,
        start: &Revision,
        path: &str,
    ) -> Option<Touch> {
        let git = Git::at(repository);
        let commit = git
            .text(&["log", "-1", "--format=%H", &head.0, "--", path])
            .ok()?
            .trim()
            .to_string();
        if commit.is_empty() {
            return None;
        }
        let since = ancestor(&git, &commit, &start.0) == Some(false);
        Some(Touch {
            commit: Revision::new(commit),
            since,
        })
    }

    fn in_commit_since(
        &self,
        repository: &str,
        head: &Revision,
        start: &Revision,
        path: &str,
    ) -> bool {
        Git::at(repository)
            .text(&[
                "log",
                "--format=%H",
                &format!("{}..{}", start.0, head.0),
                "--",
                path,
            ])
            .is_ok_and(|commits| !commits.trim().is_empty())
    }

    fn common_dir(&self, repository: &str) -> Option<String> {
        Git::at(repository).common_dir().ok()
    }

    /// The remote `origin`'s default branch where the repository has the
    /// remote, else the one its common git directory's `HEAD` names, as a
    /// bare repository beside its worktrees holds it, and never a branch
    /// that names no commit (20, 132, C.7).
    fn default_branch(&self, repository: &str) -> Option<String> {
        let common = self.common_dir(repository)?;
        Git::at(&common).default_branch().ok()
    }

    fn valid_branch(&self, name: &str) -> bool {
        !name.is_empty() && Git::at("/").holds(&["check-ref-format", "--branch", name])
    }
}

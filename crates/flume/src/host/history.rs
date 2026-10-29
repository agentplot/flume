//! Git's history, read through `git` in the repository (section 9;
//! design 11): `rev-parse` for a head, `merge-base --is-ancestor` for a
//! commit since a start, `rev-list <start>..<head>` for the commits
//! since, `log -1 --format=%H -- <path>` for the newest
//! commit touching a path, `log <start>..<head> -- <path>` for a path in
//! a commit since, `rev-parse --git-common-dir` for the repository's
//! common git directory, and `check-ref-format --branch` (24, 25, 33,
//! C.1). A revision git cannot read holds nothing since and touches
//! nothing.

use crate::git::Git;
use flume_seams::{History, Revision, Touch};

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
        let common = Git::at(repository)
            .text(&["rev-parse", "--path-format=absolute", "--git-common-dir"])
            .ok()?;
        let common = std::path::PathBuf::from(common.trim());
        let common = common.canonicalize().unwrap_or(common);
        Some(common.to_string_lossy().into_owned())
    }

    fn valid_branch(&self, name: &str) -> bool {
        !name.is_empty() && Git::at("/").holds(&["check-ref-format", "--branch", name])
    }
}

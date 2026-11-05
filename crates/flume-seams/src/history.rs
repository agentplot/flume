//! Git's history, which section 9 fixes and the core reads beside the
//! seams (design 3). No seam of Part B.

use crate::Revision;
use serde::{Deserialize, Serialize};

/// The newest commit at a head touching a path, and whether it was made
/// since a start head (24, 25).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Touch {
    pub commit: Revision,
    pub since: bool,
}

/// A branch as the repository's remote `origin` holds it, read after
/// flume's own fetch of it (9, 126, C.1).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginBranch {
    /// `origin/<branch>` as the fetch left it.
    At { revision: Revision },
    /// The repository has no remote `origin`.
    NoOrigin,
    /// `origin` holds no such branch.
    Lacks,
    /// The fetch failed for another cause, in git's words.
    Refused { reason: String },
}

/// What the core reads of git's history (design 3). Every answer is about
/// the repository as it stands when asked.
pub trait History: Send + Sync {
    /// The branch's head, if the branch exists.
    fn head(&self, repository: &str, branch: &str) -> Option<Revision>;

    /// The branch at the repository's `origin`, fetched first, so a copy
    /// of it the host holds is never read in its place (9, C.1).
    fn origin_branch(&self, repository: &str, branch: &str) -> OriginBranch;

    /// Whether `head` holds a commit `start` does not: `head` reaches it
    /// and `start` does not (25).
    fn holds_since(&self, repository: &str, head: &Revision, start: &Revision) -> bool;

    /// The commits `head` holds that `start` does not, newest first (43).
    fn commits_since(&self, repository: &str, head: &Revision, start: &Revision) -> Vec<Revision>;

    /// The newest commit at `head` touching `path`, and whether it was
    /// made since `start` (24).
    fn newest_touching(
        &self,
        repository: &str,
        head: &Revision,
        start: &Revision,
        path: &str,
    ) -> Option<Touch>;

    /// Whether `path` is in a commit at `head` made since `start` (24).
    fn in_commit_since(
        &self,
        repository: &str,
        head: &Revision,
        start: &Revision,
        path: &str,
    ) -> bool;

    /// The repository's common git directory, by its absolute path, from
    /// which its id is taken (33, C.4); none where git cannot say.
    fn common_dir(&self, repository: &str) -> Option<String>;

    /// The repository's default branch, the one its common git
    /// directory's `HEAD` names; none where git cannot say (132, C.7).
    fn default_branch(&self, repository: &str) -> Option<String>;

    /// Whether `name` is a valid branch name (C.1).
    fn valid_branch(&self, name: &str) -> bool;
}

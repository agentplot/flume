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

/// What the core reads of git's history (design 3). Every answer is about
/// the repository as it stands when asked.
pub trait History: Send + Sync {
    /// The branch's head, if the branch exists.
    fn head(&self, repository: &str, branch: &str) -> Option<Revision>;

    /// Whether `head` holds a commit `start` does not: `head` reaches it
    /// and `start` does not (25).
    fn holds_since(&self, repository: &str, head: &Revision, start: &Revision) -> bool;

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

    /// Whether `name` is a valid branch name (C.1).
    fn valid_branch(&self, name: &str) -> bool;
}

//! The change seam's answers (86, 87).

use serde::{Deserialize, Serialize};

/// One task of a change, ticked when done.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub text: String,
    pub ticked: bool,
}

/// read-change at a revision: whether the change exists, whether its
/// planning is complete, its tasks, and whether it is archived (86).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeFacts {
    pub exists: bool,
    pub planning_complete: bool,
    pub tasks: Vec<Task>,
    pub archived: bool,
}

impl ChangeFacts {
    pub fn ticked(&self) -> usize {
        self.tasks.iter().filter(|task| task.ticked).count()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeRead {
    Read { facts: ChangeFacts },
    CouldNotRead { reason: String },
}

/// The change seam (B.3). The same revision always gives the same
/// answer, and reading writes nothing (87).
pub trait Change: Send + Sync {
    /// The change by its name at `revision` (86).
    fn read_change(&self, repository: &str, revision: &crate::Revision, name: &str) -> ChangeRead;

    /// Where the change by its name lives in a place, relative to it: the
    /// prompt's change part, and the path whose newest commit a change
    /// written or tasks ticked is read by (24, design 2).
    fn path(&self, name: &str) -> String;
}

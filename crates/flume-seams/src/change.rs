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

//! The change seam's answers (86, 87).

use serde::{Deserialize, Serialize};

/// One task of a change, ticked when done, and the task group it is in,
/// as an index into its change's `groups` (86).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub text: String,
    pub ticked: bool,
    #[serde(default)]
    pub group: Option<usize>,
}

/// One task group of a change, with its number and title as the change
/// gives them; a group holding the tasks above every heading has
/// neither (86, C.3).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskGroup {
    #[serde(default)]
    pub number: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
}

/// read-change at a revision: whether the change exists, whether its
/// planning is complete, its tasks, its task groups, and whether it is
/// archived (86).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeFacts {
    pub exists: bool,
    pub planning_complete: bool,
    pub tasks: Vec<Task>,
    pub archived: bool,
    #[serde(default)]
    pub groups: Vec<TaskGroup>,
}

impl ChangeFacts {
    pub fn ticked(&self) -> usize {
        self.tasks.iter().filter(|task| task.ticked).count()
    }

    /// The first group, by its index in `groups`, holding a task
    /// unticked; none where every task in a group is ticked. A task in
    /// no group is in none of them (140).
    pub fn first_unticked_group(&self) -> Option<usize> {
        (0..self.groups.len()).find(|&group| self.tasks_of(group).any(|task| !task.ticked))
    }

    /// The tasks of the group at `group` in `groups`, in order.
    pub fn tasks_of(&self, group: usize) -> impl Iterator<Item = &Task> {
        self.tasks
            .iter()
            .filter(move |task| task.group == Some(group))
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

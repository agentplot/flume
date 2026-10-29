//! The change seam's scenarios (B.3, 86, 87).

use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub trait ChangeHarness {
    type Seam: Change + ?Sized;

    fn seam(&self) -> &Self::Seam;

    /// A repository and a commit at which the change `name` stands as
    /// `facts` say.
    fn committed(&self, name: &str, facts: &ChangeFacts) -> (String, Revision);
}

fn task(text: &str, ticked: bool) -> Task {
    Task {
        text: text.into(),
        ticked,
        group: Some(0),
    }
}

fn in_group(text: &str, ticked: bool, group: usize) -> Task {
    Task {
        group: Some(group),
        ..task(text, ticked)
    }
}

fn group(number: &str, title: &str) -> TaskGroup {
    TaskGroup {
        number: Some(number.into()),
        title: Some(title.into()),
    }
}

// ---- the change seam: a profile unit appends its own scenarios at the list's end ----

pub fn scenarios<H: ChangeHarness>() -> Vec<Scenario<H>> {
    vec![
        Scenario {
            name: "change: read-change gives whether it exists, its planning, its tasks and whether archived",
            cites: &["86", "24", "26"],
            run: read_change_gives_its_facts::<H>,
        },
        Scenario {
            name: "change: a change by another name does not exist",
            cites: &["86", "3"],
            run: another_name_is_absent::<H>,
        },
        Scenario {
            name: "change: the same revision gives the same answer",
            cites: &["87", "59"],
            run: the_same_revision_the_same_answer::<H>,
        },
        Scenario {
            name: "change: locate gives where the change lies in the place, naming it",
            cites: &["127", "24", "27"],
            run: the_path_names_the_change::<H>,
        },
        Scenario {
            name: "change: read-change gives each task its group, each group its number and title",
            cites: &["86", "87"],
            run: each_task_in_its_group::<H>,
        },
        Scenario {
            name: "change: tasks above every heading are a group with no number or title, ahead of the others",
            cites: &["86", "87"],
            run: tasks_above_every_heading::<H>,
        },
    ]
}

fn facts(read: ChangeRead) -> Result<ChangeFacts, String> {
    match read {
        ChangeRead::Read { facts } => Ok(facts),
        other => Err(format!("read-change gave {other:?}")),
    }
}

fn read_change_gives_its_facts<H: ChangeHarness>(harness: &H) -> Outcome {
    let written = ChangeFacts {
        exists: true,
        planning_complete: true,
        tasks: vec![task("1.1 one", true), task("1.2 two", false)],
        archived: false,
        groups: vec![group("1", "One")],
    };
    let (repository, revision) = harness.committed("ticking", &written);
    let read = facts(
        harness
            .seam()
            .read_change(&repository, &revision, "ticking"),
    )?;
    ensure_eq!(read, written, "the change read");
    let archived = ChangeFacts {
        exists: false,
        planning_complete: true,
        tasks: vec![task("1.1 one", true)],
        archived: true,
        groups: vec![group("1", "One")],
    };
    let (repository, revision) = harness.committed("filed", &archived);
    let read = facts(harness.seam().read_change(&repository, &revision, "filed"))?;
    ensure!(read.archived, "an archived change read as not archived");
    Ok(())
}

fn another_name_is_absent<H: ChangeHarness>(harness: &H) -> Outcome {
    let (repository, revision) = harness.committed(
        "present",
        &ChangeFacts {
            exists: true,
            ..ChangeFacts::default()
        },
    );
    let read = facts(harness.seam().read_change(&repository, &revision, "absent"))?;
    ensure!(
        !read.exists && !read.archived,
        "an absent change read as {read:?}"
    );
    Ok(())
}

fn the_same_revision_the_same_answer<H: ChangeHarness>(harness: &H) -> Outcome {
    let (repository, revision) = harness.committed(
        "steady",
        &ChangeFacts {
            exists: true,
            planning_complete: false,
            tasks: vec![task("1.1 one", false)],
            archived: false,
            groups: vec![group("1", "One")],
        },
    );
    let first = harness.seam().read_change(&repository, &revision, "steady");
    let second = harness.seam().read_change(&repository, &revision, "steady");
    ensure_eq!(first, second, "a second read of one revision");
    Ok(())
}

fn the_path_names_the_change<H: ChangeHarness>(harness: &H) -> Outcome {
    let path = harness.seam().path("named");
    ensure!(
        path.contains("named"),
        "the path {path} does not name the change"
    );
    ensure!(
        !path.starts_with('/'),
        "the path {path} is not inside the place"
    );
    Ok(())
}

fn each_task_in_its_group<H: ChangeHarness>(harness: &H) -> Outcome {
    let written = ChangeFacts {
        exists: true,
        planning_complete: true,
        tasks: vec![
            in_group("1.1 the grammar", true, 0),
            in_group("1.2 the parser", false, 0),
            in_group("2.1 its tests", false, 1),
        ],
        archived: false,
        groups: vec![group("1", "Parser"), group("2", "Tests")],
    };
    let (repository, revision) = harness.committed("grouped", &written);
    let read = facts(
        harness
            .seam()
            .read_change(&repository, &revision, "grouped"),
    )?;
    ensure_eq!(read, written, "the change read with its groups");
    ensure_eq!(
        read.first_unticked_group(),
        Some(0),
        "the first unticked group"
    );
    ensure_eq!(
        read.tasks_of(1)
            .map(|task| task.text.as_str())
            .collect::<Vec<_>>(),
        vec!["2.1 its tests"],
        "the second group's tasks"
    );
    Ok(())
}

fn tasks_above_every_heading<H: ChangeHarness>(harness: &H) -> Outcome {
    let written = ChangeFacts {
        exists: true,
        planning_complete: true,
        tasks: vec![
            in_group("first, above every heading", true, 0),
            in_group("1.1 under the heading", false, 1),
        ],
        archived: false,
        groups: vec![TaskGroup::default(), group("1", "Parser")],
    };
    let (repository, revision) = harness.committed("headed", &written);
    let read = facts(harness.seam().read_change(&repository, &revision, "headed"))?;
    ensure_eq!(
        read,
        written,
        "the change read with a group above its headings"
    );
    ensure_eq!(
        read.first_unticked_group(),
        Some(1),
        "the first unticked group"
    );
    Ok(())
}

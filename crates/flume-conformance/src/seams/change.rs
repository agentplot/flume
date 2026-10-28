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

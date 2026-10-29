//! Git's history, which the core reads beside the seams (section 9,
//! design 3): what a head holds since a stage's start (24, 25).

use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub trait HistoryHarness {
    type Seam: History + ?Sized;

    fn seam(&self) -> &Self::Seam;

    /// A repository and a branch at its first commit.
    fn branch(&self) -> (String, String, Revision);

    /// A commit on the branch touching `path`.
    fn commit(&self, repository: &str, branch: &str, path: &str) -> Revision;

    /// The branch moved back to `revision`, as an agent's reset does.
    fn reset(&self, repository: &str, branch: &str, revision: &Revision);

    /// The default branch the harness made the repository of
    /// [`HistoryHarness::branch`] with.
    fn made_default(&self) -> String;
}

// ---- git's history: a unit that reads git appends its own scenarios at the list's end ----

pub fn scenarios<H: HistoryHarness>() -> Vec<Scenario<H>> {
    vec![
        Scenario {
            name:
                "history: a head that reaches a commit the start head does not holds a commit since",
            cites: &["25", "24"],
            run: a_commit_since_the_start::<H>,
        },
        Scenario {
            name: "history: a head reset to an ancestor of the start head holds no commit since",
            cites: &["25"],
            run: a_reset_to_an_older_commit::<H>,
        },
        Scenario {
            name: "history: the newest commit touching a path, and whether it is since the start",
            cites: &["24", "25"],
            run: the_newest_commit_touching_a_path::<H>,
        },
        Scenario {
            name: "history: a branch name git refuses is not valid",
            cites: &["9", "2"],
            run: a_branch_name_refused::<H>,
        },
        Scenario {
            name: "history: the commits since a start, newest first, which an end carries",
            cites: &["43", "25"],
            run: the_commits_since_a_start::<H>,
        },
        Scenario {
            name: "history: a repository's default branch, which a role's run is cut from where its table names no base",
            cites: &["132", "9"],
            run: the_default_branch::<H>,
        },
    ]
}

fn the_default_branch<H: HistoryHarness>(harness: &H) -> Outcome {
    let (repository, _, _) = harness.branch();
    ensure_eq!(
        harness.seam().default_branch(&repository),
        Some(harness.made_default()),
        "the default branch"
    );
    Ok(())
}

fn the_commits_since_a_start<H: HistoryHarness>(harness: &H) -> Outcome {
    let (repository, branch, start) = harness.branch();
    ensure_eq!(
        harness.seam().commits_since(&repository, &start, &start),
        Vec::<Revision>::new(),
        "the commits since the start, at the start"
    );
    let first = harness.commit(&repository, &branch, "a.rs");
    let second = harness.commit(&repository, &branch, "b.rs");
    ensure_eq!(
        harness.seam().commits_since(&repository, &second, &start),
        vec![second.clone(), first],
        "the commits since the start"
    );
    Ok(())
}

fn a_commit_since_the_start<H: HistoryHarness>(harness: &H) -> Outcome {
    let (repository, branch, start) = harness.branch();
    ensure!(
        !harness.seam().holds_since(&repository, &start, &start),
        "the start head holds a commit since itself"
    );
    let head = harness.commit(&repository, &branch, "work.rs");
    ensure_eq!(
        harness.seam().head(&repository, &branch),
        Some(head.clone()),
        "the branch's head"
    );
    ensure!(
        harness.seam().holds_since(&repository, &head, &start),
        "a new commit is not since the start"
    );
    Ok(())
}

fn a_reset_to_an_older_commit<H: HistoryHarness>(harness: &H) -> Outcome {
    let (repository, branch, first) = harness.branch();
    let start = harness.commit(&repository, &branch, "a.rs");
    harness.reset(&repository, &branch, &first);
    let head = harness.seam().head(&repository, &branch).ok_or("no head")?;
    ensure!(
        !harness.seam().holds_since(&repository, &head, &start),
        "a head reset to an ancestor holds a commit since the start"
    );
    Ok(())
}

fn the_newest_commit_touching_a_path<H: HistoryHarness>(harness: &H) -> Outcome {
    let (repository, branch, _) = harness.branch();
    let before = harness.commit(&repository, &branch, "plans/one/tasks.md");
    let start = harness.commit(&repository, &branch, "other.rs");
    let touch = harness
        .seam()
        .newest_touching(&repository, &start, &start, "plans/one")
        .ok_or("no commit touches the path")?;
    ensure_eq!(
        (touch.commit, touch.since),
        (before, false),
        "the newest touch before the start"
    );
    let after = harness.commit(&repository, &branch, "plans/one/tasks.md");
    let touch = harness
        .seam()
        .newest_touching(&repository, &after, &start, "plans/one")
        .ok_or("no commit touches the path")?;
    ensure_eq!(
        (touch.commit, touch.since),
        (after.clone(), true),
        "the newest touch since the start"
    );
    ensure!(
        harness
            .seam()
            .in_commit_since(&repository, &after, &start, "plans/one/tasks.md"),
        "the path is in no commit since the start"
    );
    Ok(())
}

fn a_branch_name_refused<H: HistoryHarness>(harness: &H) -> Outcome {
    ensure!(harness.seam().valid_branch("fix/one"), "fix/one is refused");
    for name in ["fix/a b", "fix/..", "fix/one.lock", "fix/~one"] {
        ensure!(
            !harness.seam().valid_branch(name),
            "{name} is taken as valid"
        );
    }
    Ok(())
}

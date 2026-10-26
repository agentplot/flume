//! The merge queue seam's scenarios (B.6, 99, 101, 117, 125), and where
//! the base lives (9).

use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub trait MergeQueueHarness {
    type Seam: MergeQueue + ?Sized;

    fn seam(&self) -> &Self::Seam;

    /// A place whose branch holds a commit its base lacks, and its head.
    fn place(&self) -> (PlaceRef, Revision);

    /// The next decision the merge queue makes fails, naming `words`.
    fn will_fail(&self, words: &str);

    /// The place's work reaches its base inside another branch's landing.
    fn land_elsewhere(&self, place: &PlaceRef);

    /// A hand submits the place's branch, and it stays open.
    fn open_submission(&self, place: &PlaceRef) -> Submission;

    /// The base's current revision lives elsewhere than its local branch.
    fn base_elsewhere(&self, repository: &str, base: &str) -> Revision;

    /// Reading where the base lives fails, as a fetch does.
    fn fetch_fails(&self, repository: &str, base: &str);

    /// The merge queue cannot be read.
    fn unreadable(&self);
}

fn found(read: SubmissionRead) -> Result<Submission, String> {
    match read {
        SubmissionRead::Found { submission } => Ok(submission),
        other => Err(format!("read-submission gave {other:?}")),
    }
}

// ---- the merge queue seam: a profile unit appends its own scenarios at the list's end ----

pub fn scenarios<H: MergeQueueHarness>() -> Vec<Scenario<H>> {
    vec![
        Scenario {
            name: "merge queue: submit lands the branch with the commit it landed as",
            cites: &["117", "111", "113", "15"],
            run: submit_lands::<H>,
        },
        Scenario {
            name: "merge queue: a failed submission names the merge queue's words",
            cites: &["117", "112"],
            run: submit_fails::<H>,
        },
        Scenario {
            name: "merge queue: read-submission gives the newest of a branch, whoever made it",
            cites: &["99", "63", "71"],
            run: read_gives_the_newest::<H>,
        },
        Scenario {
            name: "merge queue: landed is said only of that branch's own submission",
            cites: &["101", "63", "S8"],
            run: landed_only_of_that_branch::<H>,
        },
        Scenario {
            name: "merge queue: wait decides a submission still open",
            cites: &["125", "124"],
            run: wait_decides::<H>,
        },
        Scenario {
            name: "merge queue: the base's revision where work lands, or why not",
            cites: &["9", "15"],
            run: the_base_where_it_lands::<H>,
        },
        Scenario {
            name: "merge queue: a merge queue that cannot be read says so",
            cites: &["99", "68"],
            run: unreadable_says_so::<H>,
        },
    ]
}

fn submit_lands<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, head) = harness.place();
    let commit = match harness.seam().submit(&place, &head) {
        Verdict::Landed { commit, .. } => commit,
        other => return Err(format!("a clean submission came to {other:?}")),
    };
    let submission = found(
        harness
            .seam()
            .read_submission(&place.repository, &place.branch),
    )?;
    ensure_eq!(submission.target, place.base, "the target");
    ensure_eq!(
        submission.state,
        SubmissionState::Landed { commit },
        "the submission"
    );
    Ok(())
}

fn submit_fails<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, head) = harness.place();
    harness.will_fail("test x failed");
    match harness.seam().submit(&place, &head) {
        Verdict::Failed { words } => {
            ensure!(words.contains("test x failed"), "the words {words}")
        }
        other => return Err(format!("a failing submission came to {other:?}")),
    }
    let submission = found(
        harness
            .seam()
            .read_submission(&place.repository, &place.branch),
    )?;
    ensure!(
        matches!(submission.state, SubmissionState::Failed { .. }),
        "the submission read {submission:?}"
    );
    Ok(())
}

fn read_gives_the_newest<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, _) = harness.place();
    ensure_eq!(
        harness
            .seam()
            .read_submission(&place.repository, &place.branch),
        SubmissionRead::None,
        "a branch never submitted"
    );
    let by_hand = harness.open_submission(&place);
    let submission = found(
        harness
            .seam()
            .read_submission(&place.repository, &place.branch),
    )?;
    ensure_eq!(submission.id, by_hand.id, "the newest submission");
    ensure!(
        submission.state.open(),
        "a hand's open submission read {submission:?}"
    );
    Ok(())
}

fn landed_only_of_that_branch<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, _) = harness.place();
    harness.land_elsewhere(&place);
    ensure!(
        !matches!(
            harness
                .seam()
                .read_submission(&place.repository, &place.branch),
            SubmissionRead::Found {
                submission: Submission {
                    state: SubmissionState::Landed { .. },
                    ..
                }
            }
        ),
        "work landed inside another branch's landing read as this branch's"
    );
    Ok(())
}

fn wait_decides<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, _) = harness.place();
    let open = harness.open_submission(&place);
    match harness.seam().wait(&place.repository, &open) {
        Verdict::Landed { .. } | Verdict::Failed { .. } | Verdict::Withdrawn => {}
        other => return Err(format!("a wait came to {other:?}")),
    }
    let submission = found(
        harness
            .seam()
            .read_submission(&place.repository, &place.branch),
    )?;
    ensure!(!submission.state.open(), "a decided submission reads open");
    Ok(())
}

fn the_base_where_it_lands<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, _) = harness.place();
    let elsewhere = harness.base_elsewhere(&place.repository, &place.base);
    ensure_eq!(
        harness.seam().base_revision(&place.repository, &place.base),
        BaseRevision::At {
            revision: elsewhere
        },
        "the base's revision"
    );
    harness.fetch_fails(&place.repository, &place.base);
    ensure!(
        matches!(
            harness.seam().base_revision(&place.repository, &place.base),
            BaseRevision::Refused { .. }
        ),
        "a failed fetch gave a revision"
    );
    Ok(())
}

fn unreadable_says_so<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, _) = harness.place();
    harness.unreadable();
    ensure!(
        matches!(
            harness
                .seam()
                .read_submission(&place.repository, &place.branch),
            SubmissionRead::CouldNotRead { .. }
        ),
        "an unreadable merge queue was read"
    );
    Ok(())
}

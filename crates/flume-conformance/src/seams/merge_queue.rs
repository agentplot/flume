//! The merge queue seam's scenarios (B.6, 99, 101, 117, 125, 126, 133),
//! and where the base lives (9).

use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub trait MergeQueueHarness {
    type Seam: MergeQueue + ?Sized;

    fn seam(&self) -> &Self::Seam;

    /// A place whose branch holds a commit its base lacks, and its head.
    fn place(&self) -> (PlaceRef, Revision);

    /// The next submission the merge queue decides fails, naming `words`.
    fn will_fail(&self, words: &str);

    /// The next submission of the place conflicts with its base on
    /// `file`, as a commit to the base touching it would.
    fn will_conflict(&self, place: &PlaceRef, file: &str);

    /// The next submission stays open until the scenario has read it.
    fn will_hold(&self);

    /// Whatever the merge queue needs beyond a step until the place's
    /// newest submission is decided; nothing where a step decides it.
    fn until_decided(&self, place: &PlaceRef);

    /// A hand withdraws the place's newest submission.
    fn withdraw(&self, place: &PlaceRef);

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

fn submitted<H: MergeQueueHarness>(
    harness: &H,
    place: &PlaceRef,
    head: &Revision,
) -> Result<Submission, String> {
    match harness.seam().submit(place, head) {
        Submitted::Submitted { submission } => Ok(submission),
        other => Err(format!("submit answered {other:?}")),
    }
}

fn outcomes<H: MergeQueueHarness>(
    harness: &H,
    place: &PlaceRef,
) -> Result<Vec<SubmissionOutcome>, String> {
    match harness.seam().inbox(place) {
        Inbox::Outcomes { outcomes } => Ok(outcomes),
        other => Err(format!("the inbox answered {other:?}")),
    }
}

/// Drives the place's base one round, and on until its newest submission
/// is decided.
fn decided<H: MergeQueueHarness>(harness: &H, place: &PlaceRef) -> Outcome {
    match harness.seam().step(&place.repository, &place.base) {
        Stepped::Stepped | Stepped::MovesOnItsOwn => {}
        other => return Err(format!("step answered {other:?}")),
    }
    harness.until_decided(place);
    Ok(())
}

/// The one outcome the inbox gives, and then nothing.
fn the_one_outcome<H: MergeQueueHarness>(
    harness: &H,
    place: &PlaceRef,
) -> Result<SubmissionOutcome, String> {
    let mut first = outcomes(harness, place)?;
    ensure_eq!(first.len(), 1, "outcomes the inbox gave");
    ensure_eq!(
        outcomes(harness, place)?,
        vec![],
        "outcomes an inbox gave again"
    );
    Ok(first.remove(0))
}

// ---- the merge queue seam: a profile unit appends its own scenarios at the list's end ----

pub fn scenarios<H: MergeQueueHarness>() -> Vec<Scenario<H>> {
    vec![
        Scenario {
            name: "merge queue: submit holds no wait, answering once the merge queue holds the submission",
            cites: &["117", "111", "15"],
            run: submit_holds_no_wait::<H>,
        },
        Scenario {
            name: "merge queue: the inbox gives a landing once, with the commit it landed as",
            cites: &["133", "101", "113", "117"],
            run: inbox_gives_a_landing_once::<H>,
        },
        Scenario {
            name: "merge queue: a failed submission's outcome names the merge queue's words",
            cites: &["133", "117", "112"],
            run: a_failure_names_the_words::<H>,
        },
        Scenario {
            name: "merge queue: a conflict's outcome names the files contested and the base tested against",
            cites: &["133", "112"],
            run: a_conflict_names_the_files::<H>,
        },
        Scenario {
            name: "merge queue: step drives the base's queue one round, or says it moves on its own",
            cites: &["125", "124"],
            run: step_drives_a_round::<H>,
        },
        Scenario {
            name: "merge queue: a withdrawal is read from read-submission, never from the inbox",
            cites: &["99", "133"],
            run: a_withdrawal_is_read_not_delivered::<H>,
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
            name: "merge queue: the base's revision where work lands, or why not",
            cites: &["9", "126", "15"],
            run: the_base_where_it_lands::<H>,
        },
        Scenario {
            name: "merge queue: a merge queue that cannot be read says so",
            cites: &["99", "133", "125", "68"],
            run: unreadable_says_so::<H>,
        },
    ]
}

fn submit_holds_no_wait<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, head) = harness.place();
    harness.will_hold();
    let submission = submitted(harness, &place, &head)?;
    ensure_eq!(submission.branch, place.branch, "the branch submitted");
    ensure_eq!(submission.target, place.base, "the target");
    ensure!(
        submission.state.open(),
        "submit waited on the verdict: {submission:?}"
    );
    let newest = found(
        harness
            .seam()
            .read_submission(&place.repository, &place.branch),
    )?;
    ensure_eq!(newest.id, submission.id, "the newest submission");
    ensure_eq!(
        outcomes(harness, &place)?,
        vec![],
        "outcomes of an undecided submission"
    );
    Ok(())
}

fn inbox_gives_a_landing_once<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, head) = harness.place();
    submitted(harness, &place, &head)?;
    decided(harness, &place)?;
    let commit = match the_one_outcome(harness, &place)? {
        SubmissionOutcome::Landed { commit } => commit,
        other => return Err(format!("a clean submission came to {other:?}")),
    };
    let submission = found(
        harness
            .seam()
            .read_submission(&place.repository, &place.branch),
    )?;
    ensure_eq!(
        submission.state,
        SubmissionState::Landed { commit },
        "the submission"
    );
    Ok(())
}

fn a_failure_names_the_words<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, head) = harness.place();
    harness.will_fail("test x failed");
    submitted(harness, &place, &head)?;
    decided(harness, &place)?;
    match the_one_outcome(harness, &place)? {
        SubmissionOutcome::Failed { words } => {
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

fn a_conflict_names_the_files<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, head) = harness.place();
    harness.will_conflict(&place, "work.rs");
    submitted(harness, &place, &head)?;
    decided(harness, &place)?;
    match the_one_outcome(harness, &place)? {
        SubmissionOutcome::Conflict { files, base, words } => {
            ensure!(
                files.iter().any(|file| file == "work.rs"),
                "the files contested {files:?}"
            );
            ensure!(!base.0.is_empty(), "no base revision named");
            ensure!(!words.trim().is_empty(), "no words");
        }
        other => return Err(format!("a conflicting submission came to {other:?}")),
    }
    Ok(())
}

fn step_drives_a_round<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, _) = harness.place();
    ensure!(
        !matches!(
            harness.seam().step(&place.repository, &place.base),
            Stepped::CouldNotRead { .. }
        ),
        "a step of a base with nothing open could not read"
    );
    harness.open_submission(&place);
    decided(harness, &place)?;
    let submission = found(
        harness
            .seam()
            .read_submission(&place.repository, &place.branch),
    )?;
    ensure!(
        !submission.state.open(),
        "a submission a round reached reads open: {submission:?}"
    );
    Ok(())
}

fn a_withdrawal_is_read_not_delivered<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, head) = harness.place();
    harness.will_hold();
    let submission = submitted(harness, &place, &head)?;
    harness.withdraw(&place);
    ensure_eq!(
        outcomes(harness, &place)?,
        vec![],
        "outcomes of a withdrawal"
    );
    let read = found(
        harness
            .seam()
            .read_submission(&place.repository, &place.branch),
    )?;
    ensure_eq!(read.id, submission.id, "the submission withdrawn");
    ensure_eq!(read.state, SubmissionState::Withdrawn, "the submission");
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
        matches!(harness.seam().inbox(&place), Inbox::CouldNotRead { .. }),
        "an unreadable inbox was read"
    );
    ensure!(
        matches!(
            harness.seam().step(&place.repository, &place.base),
            Stepped::CouldNotRead { .. }
        ),
        "an unreadable queue was stepped"
    );
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

//! The merge queue seam's scenarios (B.6, 99, 101, 117, 125, 126, 133),
//! where the base lives (9), and a base place's branch opened, closed
//! and submitted by name (203, 205, 206).

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

    /// A branch of the repository the merge queue lands nothing on, as
    /// one its policy names no table for: its name (126, C.6).
    fn base_unlanded(&self, repository: &str) -> String;

    /// The merge queue cannot be read.
    fn unreadable(&self);

    /// Another branch the merge queue keeps, not the place's base and
    /// holding a submission of its own, that the merge queue refuses to
    /// read from now on, as one refuses a branch whose policy it can no
    /// longer read: its name.
    fn refuse_another(&self, place: &PlaceRef) -> String;

    /// The revision `branch` of the repository stands at, none where it
    /// has no such branch.
    fn branch_head(&self, repository: &str, branch: &str) -> Option<Revision>;

    /// `file` changed on `branch`, one the merge queue lands on, as work
    /// landed there would change it: the branch's new head.
    fn change_on(&self, repository: &str, branch: &str, file: &str) -> Revision;

    /// `base` moved with a change of its own to `file`, which `branch`
    /// also changed, so the branch's head would conflict with it were it
    /// submitted: the base's new revision (210).
    fn move_base_against(&self, repository: &str, base: &str, branch: &str, file: &str)
        -> Revision;
}

fn found(read: impl Into<ReadAcross>) -> Result<Submission, String> {
    match read.into() {
        ReadAcross {
            read: SubmissionRead::Found { submission },
            passed_over,
        } if passed_over.is_empty() => Ok(submission),
        other => Err(format!("read-submission gave {other:?}")),
    }
}

fn submitted<H: MergeQueueHarness>(
    harness: &H,
    place: &PlaceRef,
    head: &Revision,
) -> Result<Submission, String> {
    let head = SubmitHead::Worktree { head: head.clone() };
    match harness.seam().submit(place, &head) {
        Submitted::Submitted { submission } => Ok(submission),
        other => Err(format!("submit answered {other:?}")),
    }
}

fn outcomes<H: MergeQueueHarness>(
    harness: &H,
    place: &PlaceRef,
) -> Result<Vec<SubmissionOutcome>, String> {
    match harness.seam().inbox(place) {
        Inbox::Outcomes { outcomes, .. } => Ok(outcomes),
        other => Err(format!("the inbox answered {other:?}")),
    }
}

/// That `branch` alone was passed over, with words.
fn passed_over_alone(passed_over: &[PassedOver], branch: &str) -> Outcome {
    ensure_eq!(
        passed_over
            .iter()
            .map(|over| over.branch.as_str())
            .collect::<Vec<_>>(),
        vec![branch],
        "the branches passed over"
    );
    ensure!(
        passed_over.iter().all(|over| !over.words.trim().is_empty()),
        "a branch passed over with no words: {passed_over:?}"
    );
    Ok(())
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

fn two_branches_at_once<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, head) = harness.place();
    harness.will_hold();
    let submission = submitted(harness, &place, &head)?;
    let none = format!("{}-none", place.branch);
    let both = harness
        .seam()
        .read_submissions(&place.repository, &[none.as_str(), place.branch.as_str()]);
    ensure_eq!(both.len(), 2, "the answers for two branches");
    ensure_eq!(
        both[0].read,
        SubmissionRead::None,
        "the branch with nothing"
    );
    ensure_eq!(
        found(both[1].clone())?.id,
        submission.id,
        "the place's branch"
    );
    ensure_eq!(
        found(
            harness
                .seam()
                .read_submission(&place.repository, &place.branch)
        )?
        .id,
        submission.id,
        "the place's branch alone"
    );
    Ok(())
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
            name: "merge queue: read-submission asked of two branches at once gives each its own newest, in the order asked, and one of them alone the same",
            cites: &["99"],
            run: two_branches_at_once::<H>,
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
            name: "merge queue: a base the merge queue lands not is said so, for the core to read at origin",
            cites: &["126", "9"],
            run: a_base_it_lands_not::<H>,
        },
        Scenario {
            name: "merge queue: a branch the merge queue cannot read is passed over with its words, and the place's submission on its base is found, followed and landed past it",
            cites: &["99", "133", "101", "15", "117"],
            run: a_refused_branch_decides_itself_alone::<H>,
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
        SubmissionRead::None.into(),
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
                .read_submission(&place.repository, &place.branch)
                .read,
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

fn a_base_it_lands_not<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, _) = harness.place();
    let base = harness.base_unlanded(&place.repository);
    ensure_eq!(
        harness.seam().base_revision(&place.repository, &base),
        BaseRevision::Unlanded,
        "a base the merge queue lands not"
    );
    ensure!(
        matches!(
            harness.seam().base_revision(&place.repository, &place.base),
            BaseRevision::At { .. }
        ),
        "the place's base, which the merge queue lands, is not read"
    );
    Ok(())
}

fn a_refused_branch_decides_itself_alone<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, head) = harness.place();
    let ours = submitted(harness, &place, &head)?;
    let other = harness.refuse_another(&place);
    let across = harness
        .seam()
        .read_submission(&place.repository, &place.branch);
    passed_over_alone(&across.passed_over, &other)?;
    let read = found(across.read)?;
    ensure_eq!(
        read.id,
        ours.id,
        format!("the submission read past {other}")
    );
    ensure!(read.state.open(), "the open submission read {read:?}");
    match harness.seam().inbox(&place) {
        Inbox::Outcomes {
            outcomes,
            passed_over,
        } => {
            ensure_eq!(outcomes, vec![], "outcomes of an undecided submission");
            ensure!(
                passed_over.iter().all(|over| over.branch != place.base),
                "the inbox passed over the base: {passed_over:?}"
            );
        }
        other => return Err(format!("the inbox past {other:?}")),
    }
    decided(harness, &place)?;
    let commit = match the_one_outcome(harness, &place)? {
        SubmissionOutcome::Landed { commit } => commit,
        other => return Err(format!("a clean submission came to {other:?}")),
    };
    let across = harness
        .seam()
        .read_submission(&place.repository, &place.branch);
    passed_over_alone(&across.passed_over, &other)?;
    ensure_eq!(
        found(across.read)?.state,
        SubmissionState::Landed { commit },
        format!("the submission landed past {other}")
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
                .read_submission(&place.repository, &place.branch)
                .read,
            SubmissionRead::CouldNotRead { .. }
        ),
        "an unreadable merge queue was read"
    );
    Ok(())
}

// ---- a base place's branch: run over each profile that opens, closes and submits by name ----

/// The scenarios of a base place's branch: opened at its base's revision
/// and closed, an open refused, and a submission of it by name (203, 205,
/// 206).
pub fn base_place_scenarios<H: MergeQueueHarness>() -> Vec<Scenario<H>> {
    vec![
        Scenario {
            name: "merge queue: open makes a branch at its base's current revision, close removes it into the base, and a second close answers it gone",
            cites: &["205", "206"],
            run: open_then_close::<H>,
        },
        Scenario {
            name: "merge queue: open of a branch already present is refused in the merge queue's words, the branch as it was",
            cites: &["205"],
            run: open_refused::<H>,
        },
        Scenario {
            name: "merge queue: a branch submitted by name at its revision, from the repository, is held with no wait and read-submission of it gives it",
            cites: &["99", "117", "203"],
            run: a_submit_by_name::<H>,
        },
        Scenario {
            name: "merge queue: a branch it lands on is forecast clear onto its base while the two merge, then in conflict on the file both changed once the base moves, with no entry ahead and no submission made by either",
            cites: &["210"],
            run: clear_then_in_conflict::<H>,
        },
    ]
}

/// The branch the scenarios open, beside the place's.
const OPENED: &str = "train/a";

/// A base place on `OPENED` of the place's repository and base, named by
/// a location beside the place's where nothing stands.
fn base_place(place: &PlaceRef) -> PlaceRef {
    PlaceRef {
        repository: place.repository.clone(),
        location: std::path::Path::new(&place.location)
            .with_file_name("train--a")
            .to_string_lossy()
            .into_owned(),
        branch: OPENED.into(),
        base: place.base.clone(),
    }
}

/// `OPENED` made from the place's base, at the revision it was made at.
fn opened<H: MergeQueueHarness>(harness: &H, place: &PlaceRef) -> Result<Revision, String> {
    match harness.seam().open(&place.repository, OPENED, &place.base) {
        BranchOpened::Made { revision } => Ok(revision),
        other => Err(format!("open answered {other:?}")),
    }
}

fn open_then_close<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, _) = harness.place();
    let BaseRevision::At { revision: base } =
        harness.seam().base_revision(&place.repository, &place.base)
    else {
        return Err(format!("{} has no revision to open from", place.base));
    };
    ensure_eq!(opened(harness, &place)?, base, "the revision opened at");
    ensure_eq!(
        harness.branch_head(&place.repository, OPENED),
        Some(base),
        "the branch opened"
    );
    ensure_eq!(
        harness.seam().close(&place.repository, OPENED, &place.base),
        BranchClosed::Removed,
        "a close with nothing landed on the branch"
    );
    ensure_eq!(
        harness.branch_head(&place.repository, OPENED),
        None,
        "the branch closed"
    );
    ensure_eq!(
        harness.seam().close(&place.repository, OPENED, &place.base),
        BranchClosed::Gone,
        "a second close"
    );
    Ok(())
}

fn open_refused<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, head) = harness.place();
    match harness
        .seam()
        .open(&place.repository, &place.branch, &place.base)
    {
        BranchOpened::Refused { reason } => {
            ensure!(!reason.trim().is_empty(), "an open refused with no words")
        }
        other => return Err(format!("open of a branch present answered {other:?}")),
    }
    ensure_eq!(
        harness.branch_head(&place.repository, &place.branch),
        Some(head),
        "the branch an open was refused"
    );
    Ok(())
}

fn a_submit_by_name<H: MergeQueueHarness>(harness: &H) -> Outcome {
    let (place, _) = harness.place();
    let revision = opened(harness, &place)?;
    let base_place = base_place(&place);
    harness.will_hold();
    let submission = match harness.seam().submit(
        &base_place,
        &SubmitHead::Branch {
            branch: OPENED.into(),
            revision,
        },
    ) {
        Submitted::Submitted { submission } => submission,
        other => return Err(format!("submit by name answered {other:?}")),
    };
    ensure_eq!(submission.branch, OPENED, "the branch submitted");
    ensure_eq!(submission.target, place.base, "the target");
    ensure!(
        submission.state.open(),
        "submit by name waited on the verdict: {submission:?}"
    );
    ensure_eq!(
        found(harness.seam().read_submission(&place.repository, OPENED))?.id,
        submission.id,
        "the submission read by its branch"
    );
    Ok(())
}

/// The seam-contracts spec's "Clear, then in conflict": a forecast of a
/// branch that merges onto its base answers clear with its head and the
/// base's revision, and once the base moves with a change to the file
/// the branch changed, in conflict on that file at the base's new
/// revision, meeting the base itself; neither creates an entry (210).
fn clear_then_in_conflict<H: MergeQueueHarness>(harness: &H) -> Outcome {
    const FILE: &str = "README.md";
    let (place, _) = harness.place();
    let repository = &place.repository;
    opened(harness, &place)?;
    let head = harness.change_on(repository, OPENED, FILE);
    let BaseRevision::At { revision: base } = harness.seam().base_revision(repository, &place.base)
    else {
        return Err(format!("{} has no revision to forecast onto", place.base));
    };
    ensure_eq!(
        harness.seam().forecast(repository, OPENED, &place.base),
        Forecast::Clear {
            head: head.clone(),
            onto: base,
        },
        "the forecast while the two merge"
    );
    let moved = harness.move_base_against(repository, &place.base, OPENED, FILE);
    match harness.seam().forecast(repository, OPENED, &place.base) {
        Forecast::Conflict {
            head: at,
            onto,
            files,
            with,
            words,
        } => {
            ensure_eq!(at, head, "the head forecast");
            ensure_eq!(onto, moved, "the base's revision forecast onto");
            ensure_eq!(files, vec![FILE.to_string()], "the files contested");
            ensure!(with.is_empty(), "a conflict with the base met {with:?}");
            ensure!(
                words.contains(FILE),
                "the conflict's words name no {FILE}: {words}"
            );
        }
        other => return Err(format!("the forecast once the base moved: {other:?}")),
    }
    ensure_eq!(
        harness.seam().read_submission(repository, OPENED).read,
        SubmissionRead::None,
        "a submission of the branch forecast"
    );
    Ok(())
}

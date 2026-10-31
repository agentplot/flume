//! The place seam's scenarios (B.1, 78 to 83).

use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;
use std::path::Path;

/// What the place scenarios need beyond the seam: a repository to make
/// places of, and the world arranged around a place.
pub trait PlaceHarness {
    type Seam: Place + ?Sized;

    fn seam(&self) -> &Self::Seam;

    /// A repository, its base branch and the base's head.
    fn repository(&self) -> (String, String, Revision);

    /// A branch made at a commit of its own, as by a hand.
    fn branch(&self, repository: &str, branch: &str) -> Revision;

    /// A commit on a place's branch, made in it.
    fn commit(&self, place: &PlaceRef) -> Revision;

    fn head(&self, repository: &str, branch: &str) -> Option<Revision>;

    /// Something at `location` that no make put there.
    fn occupy(&self, location: &str);

    /// The repository's readiness: each step, passing or failing.
    fn readiness(&self, repository: &str, steps: &[(&str, bool)]);

    /// A file in the place neither committed nor ignored.
    fn untracked(&self, place: &PlaceRef, file: &str);

    /// The place's teardown fails at `step`.
    fn fail_teardown(&self, place: &PlaceRef, step: &str);

    /// The base takes `commit`'s change by a commit of its own.
    fn apply_to_base(&self, repository: &str, base: &str, commit: &Revision);
}

/// The location the seam gives the place `fix--<name>` of `repository`.
fn location<H: PlaceHarness>(harness: &H, repository: &str, name: &str) -> Result<String, String> {
    harness
        .seam()
        .location(repository, &format!("fix--{name}"))
        .map_err(|reason| format!("no location for fix--{name}: {reason}"))
}

fn made<H: PlaceHarness>(harness: &H, name: &str) -> Result<(PlaceRef, Revision), String> {
    let (repository, base, revision) = harness.repository();
    let place = PlaceRef {
        location: location(harness, &repository, name)?,
        branch: format!("fix/{name}"),
        repository: repository.clone(),
        base,
    };
    let outcome = harness
        .seam()
        .make(&repository, &revision, &place.branch, &place.location);
    ensure_eq!(outcome, MakeOutcome::Made, "make");
    Ok((place, revision))
}

// ---- the place seam: a profile unit appends its own scenarios at the list's end ----

pub fn scenarios<H: PlaceHarness>() -> Vec<Scenario<H>> {
    vec![
        Scenario {
            name: "place: each place's location is its own, and the same at every ask",
            cites: &["3", "78"],
            run: a_location_is_the_places_own::<H>,
        },
        Scenario {
            name: "place: a place's location lies outside its repository's directory",
            cites: &["3", "78"],
            run: a_location_outside_the_repository::<H>,
        },
        Scenario {
            name: "place: make cuts a new branch at the revision",
            cites: &["78", "9"],
            run: make_cuts_a_branch::<H>,
        },
        Scenario {
            name: "place: make refuses an existing branch and touches it not",
            cites: &["78", "83", "3"],
            run: make_refuses_an_existing_branch::<H>,
        },
        Scenario {
            name: "place: make refuses an existing location and makes no branch",
            cites: &["78", "83", "3"],
            run: make_refuses_an_existing_location::<H>,
        },
        Scenario {
            name: "place: ready runs each step in order, stopping at the first that fails",
            cites: &["79", "11", "12"],
            run: ready_stops_at_the_first_failure::<H>,
        },
        Scenario {
            name: "place: read gives the branch, the head and what the place holds",
            cites: &["80", "65"],
            run: read_names_what_it_holds::<H>,
        },
        Scenario {
            name: "place: remove runs the teardown, or keeps the place naming the step",
            cites: &["81", "66", "83"],
            run: remove_runs_the_teardown::<H>,
        },
        Scenario {
            name: "place: remove-branch keeps a branch holding a change its base lacks",
            cites: &["82", "67", "I4"],
            run: remove_branch_judges_by_change::<H>,
        },
    ]
}

fn a_location_is_the_places_own<H: PlaceHarness>(harness: &H) -> Outcome {
    let (repository, _, _) = harness.repository();
    let one = location(harness, &repository, "one")?;
    let two = location(harness, &repository, "two")?;
    ensure!(one != two, "two places given one location: {one}");
    ensure_eq!(
        location(harness, &repository, "one")?,
        one,
        "the location asked again"
    );
    Ok(())
}

fn a_location_outside_the_repository<H: PlaceHarness>(harness: &H) -> Outcome {
    let (repository, _, _) = harness.repository();
    let location = location(harness, &repository, "outside")?;
    ensure!(
        !Path::new(&location).starts_with(&repository),
        "the place is in its repository's directory: {location}"
    );
    Ok(())
}

fn make_cuts_a_branch<H: PlaceHarness>(harness: &H) -> Outcome {
    let (place, revision) = made(harness, "cut")?;
    ensure_eq!(
        harness.head(&place.repository, &place.branch),
        Some(revision.clone()),
        "the new branch's head"
    );
    match harness.seam().read(&place) {
        PlaceRead::Live { branch, head, .. } => {
            ensure_eq!(branch, place.branch, "the place's branch");
            ensure_eq!(head, revision, "the place's head");
            Ok(())
        }
        other => Err(format!("read a made place as {other:?}")),
    }
}

fn make_refuses_an_existing_branch<H: PlaceHarness>(harness: &H) -> Outcome {
    let (repository, _, revision) = harness.repository();
    let held = harness.branch(&repository, "fix/taken");
    let location = location(harness, &repository, "taken")?;
    let outcome = harness
        .seam()
        .make(&repository, &revision, "fix/taken", &location);
    ensure_eq!(outcome, MakeOutcome::BranchExists, "make over a branch");
    ensure_eq!(
        harness.head(&repository, "fix/taken"),
        Some(held),
        "the existing branch's head"
    );
    let place = PlaceRef {
        repository,
        location,
        branch: "fix/taken".into(),
        base: "main".into(),
    };
    ensure_eq!(harness.seam().read(&place), PlaceRead::Gone, "the location");
    Ok(())
}

fn make_refuses_an_existing_location<H: PlaceHarness>(harness: &H) -> Outcome {
    let (repository, _, revision) = harness.repository();
    let location = location(harness, &repository, "occupied")?;
    harness.occupy(&location);
    let outcome = harness
        .seam()
        .make(&repository, &revision, "fix/occupied", &location);
    ensure_eq!(outcome, MakeOutcome::LocationExists, "make over a location");
    ensure_eq!(
        harness.head(&repository, "fix/occupied"),
        None,
        "the branch a refused make named"
    );
    Ok(())
}

fn ready_stops_at_the_first_failure<H: PlaceHarness>(harness: &H) -> Outcome {
    let (place, revision) = made(harness, "ready")?;
    let none = harness.seam().ready(&place, &revision);
    ensure!(
        none.steps.is_empty(),
        "a repository declaring no readiness ran {:?}",
        none.steps
    );
    harness.readiness(
        &place.repository,
        &[("toolchain", true), ("setup", false), ("seed", true)],
    );
    let readiness = harness.seam().ready(&place, &revision);
    let steps: Vec<(&str, bool)> = readiness
        .steps
        .iter()
        .map(|step| (step.step.as_str(), step.passed))
        .collect();
    ensure_eq!(
        steps,
        vec![("toolchain", true), ("setup", false)],
        "the steps run"
    );
    ensure!(
        readiness.failed().is_some_and(|step| step.step == "setup"),
        "the failed step is not named"
    );
    Ok(())
}

fn read_names_what_it_holds<H: PlaceHarness>(harness: &H) -> Outcome {
    let (place, _) = made(harness, "holds")?;
    harness.untracked(&place, "notes.txt");
    match harness.seam().read(&place) {
        PlaceRead::Live { holds, .. } => {
            ensure!(
                holds
                    .untracked
                    .iter()
                    .any(|file| file.ends_with("notes.txt")),
                "read names no untracked file: {holds:?}"
            );
            Ok(())
        }
        other => Err(format!("read a made place as {other:?}")),
    }
}

fn remove_runs_the_teardown<H: PlaceHarness>(harness: &H) -> Outcome {
    let (kept, _) = made(harness, "kept")?;
    harness.fail_teardown(&kept, "pre-remove");
    match harness.seam().remove(&kept) {
        Removed::Refused { step, .. } => ensure_eq!(step, "pre-remove", "the failed step"),
        other => return Err(format!("a failing teardown removed as {other:?}")),
    }
    ensure!(
        matches!(harness.seam().read(&kept), PlaceRead::Live { .. }),
        "a refused remove left the place half removed"
    );
    let (place, _) = made(harness, "gone")?;
    ensure_eq!(harness.seam().remove(&place), Removed::Removed, "remove");
    ensure_eq!(
        harness.seam().read(&place),
        PlaceRead::Gone,
        "a removed place"
    );
    Ok(())
}

fn remove_branch_judges_by_change<H: PlaceHarness>(harness: &H) -> Outcome {
    let (place, _) = made(harness, "branch")?;
    let commit = harness.commit(&place);
    ensure_eq!(harness.seam().remove(&place), Removed::Removed, "remove");
    match harness
        .seam()
        .remove_branch(&place.repository, &place.branch, &place.base)
    {
        BranchRemoved::Kept { commits } => {
            ensure!(
                commits.contains(&commit),
                "kept naming {commits:?}, not {commit}"
            )
        }
        other => {
            return Err(format!(
                "a branch holding a change its base lacks: {other:?}"
            ))
        }
    }
    ensure!(
        harness.head(&place.repository, &place.branch).is_some(),
        "a kept branch is gone"
    );
    harness.apply_to_base(&place.repository, &place.base, &commit);
    ensure_eq!(
        harness
            .seam()
            .remove_branch(&place.repository, &place.branch, &place.base),
        BranchRemoved::Removed,
        "a branch whose change its base holds"
    );
    ensure_eq!(
        harness.head(&place.repository, &place.branch),
        None,
        "a removed branch"
    );
    Ok(())
}

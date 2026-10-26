//! Every scenario of the suite, for the coverage check, and the
//! scenarios that hold the suite itself to the statement.

use crate::fake_harness::*;
use crate::scenario::{failures, Cited};
use crate::seams::place::{self, PlaceHarness};
use crate::seams::*;
use crate::{ensure, Outcome, Scenario};
use flume_seams::*;

/// Every scenario of the suite: each seam's, the engine's and the
/// suite's own, by name with its citations.
pub fn every() -> Vec<Cited> {
    let mut every: Vec<Cited> = vec![];
    every.extend(
        place::scenarios::<FakePlaceHarness>()
            .iter()
            .map(Scenario::cited),
    );
    every.extend(
        kinds::scenarios::<FakeKindsHarness>()
            .iter()
            .map(Scenario::cited),
    );
    every.extend(
        change::scenarios::<FakeChangeHarness>()
            .iter()
            .map(Scenario::cited),
    );
    every.extend(
        agent::scenarios::<FakeAgentHarness>()
            .iter()
            .map(Scenario::cited),
    );
    every.extend(
        terminal::scenarios::<FakeTerminalHarness>()
            .iter()
            .map(Scenario::cited),
    );
    every.extend(
        merge_queue::scenarios::<FakeMergeQueueHarness>()
            .iter()
            .map(Scenario::cited),
    );
    every.extend(
        store::scenarios::<FakeStoreHarness>()
            .iter()
            .map(Scenario::cited),
    );
    every.extend(crate::engine::scenarios().iter().map(Scenario::cited));
    every.extend(scenarios().iter().map(Scenario::cited));
    every
}

/// The suite's own scenarios.
pub fn scenarios() -> Vec<Scenario<()>> {
    vec![Scenario {
        name: "suite: the place scenarios fail a place seam that makes over an existing branch",
        cites: &["106", "83"],
        run: a_lax_place_fails,
    }]
}

/// A place seam that admits a make over an existing branch, moving it.
struct LaxPlace(FakePlaceHarness);

impl Place for LaxPlace {
    fn make(
        &self,
        repository: &str,
        revision: &Revision,
        branch: &str,
        location: &str,
    ) -> MakeOutcome {
        self.0.git.delete_branch(repository, branch);
        self.0.place.make(repository, revision, branch, location)
    }

    fn ready(&self, place: &PlaceRef, revision: &Revision) -> Readiness {
        self.0.place.ready(place, revision)
    }

    fn read(&self, place: &PlaceRef) -> PlaceRead {
        self.0.place.read(place)
    }

    fn remove(&self, place: &PlaceRef) -> Removed {
        self.0.place.remove(place)
    }

    fn remove_branch(&self, repository: &str, branch: &str, base: &str) -> BranchRemoved {
        self.0.place.remove_branch(repository, branch, base)
    }
}

impl PlaceHarness for LaxPlace {
    type Seam = LaxPlace;

    fn seam(&self) -> &LaxPlace {
        self
    }

    fn repository(&self) -> (String, String, Revision) {
        self.0.repository()
    }

    fn location(&self, repository: &str, name: &str) -> String {
        self.0.location(repository, name)
    }

    fn branch(&self, repository: &str, branch: &str) -> Revision {
        self.0.branch(repository, branch)
    }

    fn commit(&self, place: &PlaceRef) -> Revision {
        self.0.commit(place)
    }

    fn head(&self, repository: &str, branch: &str) -> Option<Revision> {
        self.0.head(repository, branch)
    }

    fn occupy(&self, location: &str) {
        self.0.occupy(location)
    }

    fn readiness(&self, repository: &str, steps: &[(&str, bool)]) {
        self.0.readiness(repository, steps)
    }

    fn untracked(&self, place: &PlaceRef, file: &str) {
        self.0.untracked(place, file)
    }

    fn fail_teardown(&self, place: &PlaceRef, step: &str) {
        self.0.fail_teardown(place, step)
    }

    fn apply_to_base(&self, repository: &str, base: &str, commit: &Revision) {
        self.0.apply_to_base(repository, base, commit)
    }
}

fn a_lax_place_fails(_: &()) -> Outcome {
    let failed = failures(
        &place::scenarios(),
        || LaxPlace(FakePlaceHarness::default()),
    );
    let citing_83: Vec<&str> = place::scenarios::<LaxPlace>()
        .iter()
        .filter(|scenario| scenario.cites.contains(&"83"))
        .map(|scenario| scenario.name)
        .collect();
    ensure!(
        failed
            .iter()
            .any(|(name, _)| citing_83.contains(&name.as_str())),
        "no scenario citing 83 failed a place that makes over an existing branch: {failed:?}"
    );
    Ok(())
}

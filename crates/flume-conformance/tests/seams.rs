//! Every seam's scenarios, run against its fake.

use flume_conformance::fake_harness::*;
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::*;

#[test]
fn the_fake_place_passes_the_place_scenarios() {
    pass_all("place", &place::scenarios(), FakePlaceHarness::default);
}

#[test]
fn the_fake_kinds_pass_the_kinds_scenarios() {
    pass_all("kinds", &kinds::scenarios(), FakeKindsHarness::new);
}

#[test]
fn the_fake_change_passes_the_change_scenarios() {
    pass_all("change", &change::scenarios(), FakeChangeHarness::default);
}

#[test]
fn the_fake_agent_passes_the_agent_scenarios() {
    pass_all("agent", &agent::scenarios(), FakeAgentHarness::default);
}

#[test]
fn the_fake_terminal_passes_the_terminal_scenarios() {
    pass_all("terminal", &terminal::scenarios(), || {
        FakeTerminalHarness::new(true)
    });
    pass_all("terminal", &terminal::scenarios(), || {
        FakeTerminalHarness::new(false)
    });
}

#[test]
fn the_fake_merge_queue_passes_the_merge_queue_scenarios() {
    pass_all(
        "merge queue",
        &merge_queue::scenarios(),
        FakeMergeQueueHarness::default,
    );
}

#[test]
fn the_fake_store_passes_the_store_scenarios() {
    pass_all("store", &store::scenarios(), FakeStoreHarness::default);
}

#[test]
fn the_fake_history_passes_the_history_scenarios() {
    pass_all(
        "history",
        &history::scenarios(),
        FakeHistoryHarness::default,
    );
}

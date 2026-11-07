//! Each of the eight seams' scenarios, and git's history's, run against
//! its fake (One interface per seam).

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
    pass_all(
        "agent",
        &agent::scenarios(),
        FakeAgentHarness::carrying_none,
    );
    pass_all(
        "agent",
        &agent::scenarios(),
        FakeAgentHarness::taking_no_own,
    );
    pass_all(
        "agent",
        &agent::scenarios(),
        FakeAgentHarness::taking_no_effort,
    );
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
fn the_fake_window_passes_the_window_scenarios() {
    pass_all("window", &window::scenarios(), FakeWindowHarness::default);
}

/// The seam-contracts spec's show over the fake: two shows of a command
/// under `Work Agents`, each opening a window and logged with the command
/// and the title, then the printed line and the refusal as scripted
/// (188, 189).
#[test]
fn show_over_the_fake() {
    use flume_seams::{ViewCommand, Window, WindowShown};
    let window = flume_fakes::FakeWindow::default();
    let command = ViewCommand {
        program: "/bin/fake-terminal".into(),
        arguments: vec!["attach".into(), "work".into()],
        environment: [("XDG_CONFIG_HOME".to_string(), "/scratch/home".to_string())].into(),
    };
    let title = "Work Agents";
    for _ in 0..2 {
        assert_eq!(window.show(&command, title), WindowShown::Opened);
    }
    let shown = "show /bin/fake-terminal attach work titled Work Agents";
    assert_eq!(window.log.of("show"), vec![shown; 2]);
    assert_eq!(
        window.windows(),
        vec![(command.clone(), title.to_string()); 2]
    );
    let printed = WindowShown::Printed {
        line: "XDG_CONFIG_HOME=<set> /bin/fake-terminal attach work".into(),
    };
    window.answers(Some(printed.clone()));
    assert_eq!(window.show(&command, title), printed);
    let refused = WindowShown::Refused {
        reason: "no display".into(),
    };
    window.answers(Some(refused.clone()));
    assert_eq!(window.show(&command, title), refused);
    assert_eq!(window.log.of("show"), vec![shown; 4]);
    assert_eq!(window.windows().len(), 2, "the windows opened");
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

//! The window profile that prints the line (design 12): the window seam's
//! scenarios against it, and the line a person runs, each variable by its
//! name alone (105, 188; core.30).

use flume::profiles::window::printed::PrintedWindow;
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::window::{self, WindowHarness};
use flume_seams::*;
use std::collections::BTreeMap;
use std::time::Duration;

struct Harness {
    seam: PrintedWindow,
}

impl WindowHarness for Harness {
    type Seam = PrintedWindow;

    fn seam(&self) -> &PrintedWindow {
        &self.seam
    }

    fn command(&self) -> ViewCommand {
        command(Some("/scratch/sessions"))
    }

    fn opens(&self) -> bool {
        false
    }

    fn opened(&self) -> Vec<(ViewCommand, String)> {
        vec![]
    }

    fn will_print(&self) -> bool {
        true
    }

    fn will_refuse(&self) -> Option<String> {
        None
    }

    fn will_hang(&self) -> Option<Duration> {
        None
    }
}

/// herdr's attach for the session `work`, needing the sessions' home
/// where one is given.
fn command(home: Option<&str>) -> ViewCommand {
    ViewCommand {
        program: "/bin/herdr".into(),
        arguments: vec!["session".into(), "attach".into(), "work".into()],
        environment: home
            .map(|home| BTreeMap::from([("XDG_CONFIG_HOME".into(), home.into())]))
            .unwrap_or_default(),
    }
}

#[test]
fn the_printed_window_passes_the_window_scenarios() {
    pass_all("window", &window::scenarios(), || Harness {
        seam: PrintedWindow,
    });
}

#[test]
fn a_line_a_person_runs() {
    // 188; core.30.
    assert_eq!(
        PrintedWindow.show(&command(None), "Work Herdr"),
        WindowShown::Printed {
            line: "/bin/herdr session attach work".into()
        }
    );
    let WindowShown::Printed { line } =
        PrintedWindow.show(&command(Some("/scratch/sessions")), "Work Herdr")
    else {
        panic!("the printed window gave no line");
    };
    assert!(line.starts_with("XDG_CONFIG_HOME=<set> "), "{line}");
    assert!(!line.contains("/scratch/sessions"), "{line}");
}

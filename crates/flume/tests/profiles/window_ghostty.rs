//! The window profile through Ghostty (C.8; design 10): the window seam's
//! scenarios against it, built for macOS over a stand-in `open` and built
//! for another system over a stand-in `ghostty`, on any host; the words
//! `open` is given, each variable by `--env` and the title one word; a
//! Ghostty not installed and an `open` that never answers refused; and,
//! elsewhere, the words and the environment `ghostty` is given, its
//! process group its own, the answer given before it ends, and a
//! `ghostty` that cannot be run refused (120, 188, 189).

use crate::common::window::{Run, StandIn};
use crate::common::*;
use flume::host::System;
use flume::profiles::window::ghostty::Ghostty;
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::window::{self, WindowHarness};
use flume_seams::*;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

/// The bound a test's profile states for `open`.
const BOUND: Duration = Duration::from_secs(2);

/// The login environment a test's `ghostty` is started in.
const LOGIN: [(&str, &str); 2] = [("HOME", "/home/person"), ("PATH", "/usr/bin:/bin")];

struct Harness {
    _scratch: Scratch,
    stand_in: StandIn,
    seam: Ghostty,
    system: System,
}

impl Harness {
    fn new(system: System) -> Harness {
        let scratch = Scratch::new("window-ghostty");
        let name = match system {
            System::MacOs => "open",
            System::Other => "ghostty",
        };
        let stand_in = StandIn::new(&scratch.path("stand-in"), name);
        let seam = Ghostty::for_system(system)
            .with_programs(&stand_in.path(), &stand_in.path())
            .with_login(&LOGIN)
            .with_bound(BOUND);
        Harness {
            _scratch: scratch,
            stand_in,
            seam,
            system,
        }
    }
}

/// The window `run` opened, as its words and environment give it back:
/// the command and its title.
fn window_of(run: &Run, system: System) -> (ViewCommand, String) {
    let mut words = run.words.iter().cloned();
    let mut environment = BTreeMap::new();
    match system {
        System::MacOs => {
            assert_eq!(
                words.by_ref().take(2).collect::<Vec<_>>(),
                ["-na", "Ghostty.app"]
            );
            loop {
                match words.next().as_deref() {
                    Some("--env") => {
                        let pair = words.next().expect("a variable after --env");
                        let (name, value) = pair.split_once('=').expect("name=value");
                        environment.insert(name.to_string(), value.to_string());
                    }
                    Some("--args") => break,
                    other => panic!("open given {other:?} before --args"),
                }
            }
        }
        System::Other => {
            environment = run
                .environment
                .iter()
                .filter(|(name, _)| !LOGIN.iter().any(|(login, _)| login == name))
                .map(|(name, value)| (name.clone(), value.clone()))
                .collect();
        }
    }
    let title = words
        .next()
        .and_then(|word| word.strip_prefix("--title=").map(str::to_string))
        .expect("the title forced first");
    assert_eq!(words.next().as_deref(), Some("-e"), "the command after -e");
    let program = words.next().expect("the command's program");
    (
        ViewCommand {
            program,
            arguments: words.collect(),
            environment,
        },
        title,
    )
}

impl WindowHarness for Harness {
    type Seam = Ghostty;

    fn seam(&self) -> &Ghostty {
        &self.seam
    }

    fn command(&self) -> ViewCommand {
        command("/scratch/sessions")
    }

    fn opens(&self) -> bool {
        true
    }

    fn opened(&self) -> Vec<(ViewCommand, String)> {
        let runs = match self.system {
            System::MacOs => self.stand_in.runs(),
            System::Other => self.stand_in.runs_settled(),
        };
        runs.iter().map(|run| window_of(run, self.system)).collect()
    }

    fn will_print(&self) -> bool {
        false
    }

    /// `open` refuses as the stand-in says; a `ghostty` not waited on
    /// gives no words.
    fn will_refuse(&self) -> Option<String> {
        if self.system != System::MacOs {
            return None;
        }
        let words = "Unable to find application named 'Ghostty.app'";
        self.stand_in.refuse_next(words);
        Some(words.into())
    }

    fn will_hang(&self) -> Option<Duration> {
        if self.system != System::MacOs {
            return None;
        }
        self.stand_in.hang_next();
        Some(BOUND)
    }
}

/// herdr's attach for the session `w`, needing the sessions' home `home`.
fn command(home: &str) -> ViewCommand {
    ViewCommand {
        program: "/bin/herdr".into(),
        arguments: vec!["session".into(), "attach".into(), "w".into()],
        environment: BTreeMap::from([("XDG_CONFIG_HOME".into(), home.into())]),
    }
}

#[test]
fn ghostty_on_macos_passes_the_window_scenarios() {
    pass_all("window", &window::scenarios(), || {
        Harness::new(System::MacOs)
    });
}

#[test]
fn ghostty_elsewhere_passes_the_window_scenarios() {
    pass_all("window", &window::scenarios(), || {
        Harness::new(System::Other)
    });
}

#[test]
fn the_words_open_is_given() {
    // 188, C.8: each variable by --env, the title one word, no shell.
    let harness = Harness::new(System::MacOs);
    let answer = harness.seam.show(&command("/scratch/home"), "Work Herdr");
    assert_eq!(answer, WindowShown::Opened);
    let runs = harness.stand_in.runs();
    assert_eq!(runs.len(), 1);
    assert_eq!(
        runs[0].words,
        [
            "-na",
            "Ghostty.app",
            "--env",
            "XDG_CONFIG_HOME=/scratch/home",
            "--args",
            "--title=Work Herdr",
            "-e",
            "/bin/herdr",
            "session",
            "attach",
            "w",
        ]
    );
}

#[test]
fn ghostty_not_installed() {
    // 188: refused in open's words.
    let harness = Harness::new(System::MacOs);
    let words = "Unable to find application named 'Ghostty.app'";
    harness.stand_in.refuse_next(words);
    assert_eq!(
        harness.seam.show(&command("/h"), "Work Herdr"),
        WindowShown::Refused {
            reason: words.into()
        }
    );
    assert!(harness.stand_in.runs().is_empty());
}

#[test]
fn an_open_that_never_answers() {
    // 189: refused naming the bound, nothing waiting on it.
    let harness = Harness::new(System::MacOs);
    harness.stand_in.hang_next();
    let asked = Instant::now();
    let answer = harness.seam.show(&command("/h"), "Work Herdr");
    assert!(asked.elapsed() < BOUND + Duration::from_secs(3));
    let WindowShown::Refused { reason } = answer else {
        panic!("an open that never answered gave {answer:?}");
    };
    assert!(reason.contains("2 seconds"), "{reason}");
}

#[test]
fn the_words_and_environment_ghostty_is_given() {
    // 120, 188: the login environment with the command's over it and
    // nothing of the calling command's, a process group of its own, and
    // the answer before it ends.
    std::env::set_var("HERDR_ENV", "1");
    let harness = Harness::new(System::Other);
    harness.stand_in.hold();
    let answer = harness.seam.show(&command("/scratch/home"), "Work Herdr");
    assert_eq!(answer, WindowShown::Opened);
    let runs = harness.stand_in.runs_at_least(1);
    assert_eq!(runs.len(), 1);
    let run = &runs[0];
    assert!(!run.ended, "the answer came after ghostty ended");
    assert_eq!(
        run.words,
        [
            "--title=Work Herdr",
            "-e",
            "/bin/herdr",
            "session",
            "attach",
            "w"
        ]
    );
    assert_eq!(
        run.environment,
        BTreeMap::from([
            ("HOME".to_string(), "/home/person".to_string()),
            ("PATH".to_string(), "/usr/bin:/bin".to_string()),
            ("XDG_CONFIG_HOME".to_string(), "/scratch/home".to_string()),
        ])
    );
    // SAFETY: getpgid reads a process's group and writes nothing.
    let (group, ours) = unsafe { (libc::getpgid(run.pid as libc::pid_t), libc::getpgid(0)) };
    assert!(group > 0, "the stand-in's group read");
    assert_ne!(group, ours, "ghostty in the test's process group");
    harness.stand_in.release();
}

#[test]
fn no_ghostty_to_run() {
    // 188: refused naming it.
    let seam = Ghostty::for_system(System::Other)
        .with_programs("open", "/no/such/ghostty")
        .with_login(&LOGIN);
    let WindowShown::Refused { reason } = seam.show(&command("/h"), "Work Herdr") else {
        panic!("a ghostty that cannot run opened a window");
    };
    assert!(reason.contains("/no/such/ghostty"), "{reason}");
}

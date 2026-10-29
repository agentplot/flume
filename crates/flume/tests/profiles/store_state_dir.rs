//! The store profile over the host's state directory (C.7; design 10):
//! the store seam's scenarios against it over a scratch state directory,
//! two processes writing at one version, and S12 with the store the
//! engine's.

use crate::common::*;
use flume::profiles::store::state_dir::StateDir;
use flume_conformance::engine::world::{launch, launched, World};
use flume_conformance::scenario::{pass_all, sample_held};
use flume_conformance::seams::store::{self, StoreHarness};
use flume_core::Engine;
use flume_seams::*;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct Harness {
    scratch: Scratch,
    seam: StateDir,
}

impl Harness {
    fn new() -> Harness {
        let scratch = Scratch::new("store");
        let seam = StateDir::at(scratch.path("state/flume"));
        Harness { scratch, seam }
    }
}

impl StoreHarness for Harness {
    type Seam = StateDir;

    fn seam(&self) -> &StateDir {
        &self.seam
    }

    fn another(&self) -> Arc<dyn Store> {
        Arc::new(StateDir::at(self.scratch.path("state/flume")))
    }

    fn repository(&self) -> String {
        self.scratch
            .path("repository")
            .to_string_lossy()
            .into_owned()
    }

    fn record_made(&self, place: &PlaceId, line: &str) {
        use std::io::Write;
        let path = Path::new(&self.seam.folder(place)).join(MADE);
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("made.jsonl");
        writeln!(file, "{line}").expect("a line of made.jsonl");
    }
}

#[test]
fn the_store_profile_passes_the_store_scenarios() {
    pass_all("store", &store::scenarios(), Harness::new);
}

#[test]
fn a_removed_place_leaves_nothing_in_the_state_directory() {
    let harness = Harness::new();
    let id = PlaceId::new("/repository/places/fix--gone");
    let held = sample_held(&id, "/repository");
    let Written::Written { version } = harness.seam.write(&id, None, Some(&held)) else {
        panic!("the first write was refused");
    };
    let folder = harness.seam.folder(&id);
    assert!(
        Path::new(&folder).is_dir(),
        "the place's folder is not made"
    );
    std::fs::write(Path::new(&folder).join("mark"), "").unwrap();
    assert!(matches!(
        harness.seam.write(&id, Some(version), None),
        Written::Written { .. }
    ));
    let left: Vec<String> = std::fs::read_dir(harness.seam.places())
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    assert!(left.is_empty(), "the state directory keeps {left:?}");
    let Written::Written { version: again } = harness.seam.write(&id, None, Some(&held)) else {
        panic!("a place made again was refused");
    };
    assert!(
        again > version,
        "a place made again reused the version {version}"
    );
}

/// The variables a child process of this test binary is handed.
const CHILD_ROOT: &str = "FLUME_TEST_STORE_ROOT";
const CHILD_AT: &str = "FLUME_TEST_STORE_AT";
const CHILD_MARK: &str = "FLUME_TEST_STORE_MARK";
const CHILD_GO: &str = "FLUME_TEST_STORE_GO";
const CHILD: &str = "store_state_dir::a_child_process_writes_at_a_version";

/// The child's half of the two-process write: once the go file stands,
/// one write at the version it is handed, its answer printed. Run by the
/// test below as a process of its own; it does nothing otherwise.
#[test]
fn a_child_process_writes_at_a_version() {
    let Some(root) = std::env::var_os(CHILD_ROOT) else {
        return;
    };
    let at: u64 = std::env::var(CHILD_AT).unwrap().parse().unwrap();
    let mark = std::env::var(CHILD_MARK).unwrap();
    let go = std::env::var_os(CHILD_GO).unwrap();
    let store = StateDir::at(root);
    let id = PlaceId::new("/repository/places/fix--shared");
    let mut held = store.read(&id).expect("the place's held state").state;
    held.place.mark = Some(mark);
    let deadline = Instant::now() + Duration::from_secs(30);
    while !Path::new(&go).exists() && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(5));
    }
    let answer = match store.write(&id, Some(at), Some(&held)) {
        Written::Written { .. } => "written",
        Written::Moved => "moved",
    };
    println!("\nanswer={answer}");
}

#[test]
fn two_processes_writing_at_one_version_leave_one_write() {
    let harness = Harness::new();
    let root = harness.scratch.path("state/flume");
    let id = PlaceId::new("/repository/places/fix--shared");
    let held = sample_held(&id, "/repository");
    let Written::Written { mut version } = harness.seam.write(&id, None, Some(&held)) else {
        panic!("the first write was refused");
    };
    for round in 0..5 {
        let go = harness.scratch.path(&format!("go-{round}"));
        let children: Vec<_> = ["one", "two"]
            .iter()
            .map(|mark| {
                Command::new(std::env::current_exe().unwrap())
                    .args([CHILD, "--exact", "--nocapture", "--test-threads=1"])
                    .env(CHILD_ROOT, &root)
                    .env(CHILD_AT, version.to_string())
                    .env(CHILD_MARK, format!("{mark}-{round}"))
                    .env(CHILD_GO, &go)
                    .stdout(std::process::Stdio::piped())
                    .spawn()
                    .expect("a child process")
            })
            .collect();
        std::fs::write(&go, "").unwrap();
        let answers: Vec<String> = children
            .into_iter()
            .map(|child| {
                let output = child.wait_with_output().expect("the child's answer");
                assert!(output.status.success(), "a child failed");
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .find_map(|line| line.split("answer=").nth(1).map(str::to_string))
                    .unwrap_or_else(|| panic!("the child gave no answer: {stdout}"))
            })
            .collect();
        let written = answers.iter().filter(|answer| *answer == "written").count();
        assert_eq!(written, 1, "round {round}: {answers:?}");
        let read = harness.seam.read(&id).expect("the held state");
        assert!(read.version > version, "the version did not move");
        let mark = read.state.place.mark.clone().unwrap_or_default();
        assert!(mark.ends_with(&format!("-{round}")), "the mark {mark}");
        version = read.version;
    }
}

#[test]
fn s12_a_restart_over_the_state_directory_loses_nothing() {
    let scratch = Scratch::new("restart");
    let root = scratch.path("state/flume");
    let world = World::new();
    let over = |store: Arc<dyn Store>| {
        Engine::new(
            world.resolver.clone(),
            world.terminal.clone(),
            store,
            world.history.clone(),
            world.host.clone(),
            world.defaults.clone(),
        )
    };
    let engine = over(Arc::new(StateDir::at(&root)));
    let id = World::id("fix", "one");
    launched(&engine.launch(&launch("fix", "one", Some("x")))).expect("the launch");
    engine.settle(Some(&id));
    let code = World::identity("fix", "one", "code", 1);
    world.terminal.set_state(&code, AgentState::Working);
    over(Arc::new(StateDir::at(&root))).settle(Some(&id));
    let before = StateDir::at(&root).read(&id).expect("the place").state;
    assert!(
        before
            .stages
            .iter()
            .any(|stage| stage.name == "code" && stage.state == StageState::Begun),
        "the code stage works: {:?}",
        before.stages
    );

    world.host.restart();
    world.terminal.forget_all();
    let rebuilt = over(Arc::new(StateDir::at(&root)));
    let after = StateDir::at(&root)
        .read(&id)
        .expect("the place's held state was lost")
        .state;
    assert_eq!(after, before, "the held state after a restart");
    rebuilt.settle(None);
    let state = StateDir::at(&root)
        .read(&id)
        .and_then(|read| {
            read.state
                .stages
                .into_iter()
                .find(|stage| stage.name == "code")
        })
        .map(|stage| stage.state);
    assert_eq!(
        state,
        Some(StageState::Lost),
        "the code stage after a restart"
    );
}

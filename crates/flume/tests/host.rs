//! Git's history and the host's facts, read for real (design 11;
//! history-and-host spec): the history scenarios against git on scratch
//! repositories, secrets that never repeat, a command's life, a file's
//! write time, the bound the configuration states, and the room lock
//! taken by two processes.

mod common;

use common::*;
use flume::host::config::HostConfig;
use flume::host::history::GitHistory;
use flume::host::{command_of, RealHost};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::history::{self, HistoryHarness};
use flume_seams::*;
use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

struct Harness {
    repository: Repository,
    seam: GitHistory,
}

impl HistoryHarness for Harness {
    type Seam = GitHistory;

    fn seam(&self) -> &GitHistory {
        &self.seam
    }

    fn branch(&self) -> (String, String, Revision) {
        (
            self.repository.name(),
            "main".into(),
            Revision::new(self.repository.head()),
        )
    }

    fn commit(&self, _repository: &str, _branch: &str, path: &str) -> Revision {
        let count = self.repository.git(&["rev-list", "--count", "HEAD"]);
        self.repository.write(path, &format!("commit {count}\n"));
        Revision::new(self.repository.commit(path))
    }

    fn reset(&self, _repository: &str, _branch: &str, revision: &Revision) {
        self.repository
            .git(&["reset", "--quiet", "--hard", &revision.0]);
    }
}

#[test]
fn the_history_scenarios_pass_against_git() {
    pass_all("history", &history::scenarios(), || Harness {
        repository: Repository::new("history"),
        seam: GitHistory,
    });
}

#[test]
fn a_head_reads_none_for_a_branch_that_does_not_exist() {
    let repository = Repository::new("no-branch");
    assert_eq!(GitHistory.head(&repository.name(), "fix/none"), None);
    let start = Revision::new(repository.head());
    assert!(!GitHistory.holds_since(&repository.name(), &Revision::new("0".repeat(40)), &start));
}

fn host(scratch: &Scratch, config: HostConfig) -> RealHost {
    RealHost::new(scratch.path("state/flume"), config, None)
}

#[test]
fn a_thousand_secrets_never_repeat() {
    let scratch = Scratch::new("secrets");
    let host = host(&scratch, HostConfig::default());
    let secrets: BTreeSet<String> = (0..1000).map(|_| host.secret().0).collect();
    assert_eq!(secrets.len(), 1000);
    assert!(secrets
        .iter()
        .all(|secret| secret.len() == 64 && secret.chars().all(|c| c.is_ascii_hexdigit())));
}

#[test]
fn a_command_lives_while_its_process_does() {
    let scratch = Scratch::new("command");
    let host = host(&scratch, HostConfig::default());
    let this = host.command();
    assert!(
        this.starts_with(&format!("{}@", std::process::id())),
        "{this}"
    );
    assert!(host.alive(&this), "this command reads as ended");
    let mut child = Command::new("sleep").arg("30").spawn().expect("a child");
    let named = command_of(child.id()).expect("the child's name");
    assert!(host.alive(&named));
    let (pid, start) = named.split_once('@').unwrap();
    assert!(
        !host.alive(&format!("{pid}@{start}0")),
        "another start of the same id lives"
    );
    child.kill().unwrap();
    child.wait().unwrap();
    assert!(!host.alive(&named), "an ended command reads as living");
    assert!(!host.alive("no command"));
}

#[test]
fn a_files_write_time_by_the_hosts_clock() {
    let scratch = Scratch::new("write-time");
    let host = host(&scratch, HostConfig::default());
    let file = scratch.path("written");
    assert_eq!(host.write_time(&file.to_string_lossy()), None);
    std::fs::write(&file, "").unwrap();
    let written = host
        .write_time(&file.to_string_lossy())
        .expect("a write time");
    assert!(
        written.abs_diff(host.now()) <= 5,
        "{written} against {}",
        host.now()
    );
}

#[test]
fn the_bound_the_configuration_states() {
    let scratch = Scratch::new("bound");
    let file = scratch.path("config/flume/config.toml");
    assert_eq!(
        host(&scratch, HostConfig::read(&file).unwrap()).bound(),
        None
    );
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(&file, "[host]\nagents = 2\n").unwrap();
    let config = HostConfig::read(&file).unwrap();
    assert_eq!(host(&scratch, config.clone()).bound(), Some(2));
    assert_eq!(
        config.terminal, None,
        "a terminal left out is the printed command"
    );
    std::fs::write(&file, "[host\n").unwrap();
    assert!(
        HostConfig::read(&file).is_err(),
        "a file that is no configuration"
    );
}

const CHILD_STATE: &str = "FLUME_TEST_ROOM_STATE";
const CHILD_TAKEN: &str = "FLUME_TEST_ROOM_TAKEN";
const CHILD_FREED: &str = "FLUME_TEST_ROOM_FREED";
const CHILD: &str = "a_child_process_holds_the_room";

/// The child's half of the room-lock test: takes the room, says so,
/// holds it a second and says when it lets it go. Run by the test below
/// as a process of its own; it does nothing otherwise.
#[test]
fn a_child_process_holds_the_room() {
    let Some(state) = std::env::var_os(CHILD_STATE) else {
        return;
    };
    let host = RealHost::new(state, HostConfig::default(), None);
    let guard = host.room();
    std::fs::write(std::env::var_os(CHILD_TAKEN).unwrap(), "").unwrap();
    std::thread::sleep(Duration::from_millis(1000));
    std::fs::write(std::env::var_os(CHILD_FREED).unwrap(), "").unwrap();
    drop(guard);
}

#[test]
fn the_room_lock_is_exclusive_between_processes() {
    let scratch = Scratch::new("room");
    let state = scratch.path("state/flume");
    let taken = scratch.path("taken");
    let freed = scratch.path("freed");
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([CHILD, "--exact", "--nocapture", "--test-threads=1"])
        .env(CHILD_STATE, &state)
        .env(CHILD_TAKEN, &taken)
        .env(CHILD_FREED, &freed)
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("a child process");
    let deadline = Instant::now() + Duration::from_secs(30);
    while !taken.exists() {
        assert!(Instant::now() < deadline, "the child never took the room");
        std::thread::sleep(Duration::from_millis(5));
    }
    let host = RealHost::new(&state, HostConfig::default(), None);
    let guard = host.room();
    assert!(
        freed.exists(),
        "the room was taken while another process held it"
    );
    drop(guard);
    assert!(child.wait().unwrap().success());
    assert!(Path::new(&host.room_lock()).is_file());
}

//! The merge-queue profile over sluice (C.6; design 13) against the
//! pinned sluice, on scratch repositories gated by a command and, where a
//! base lands on a remote, a local bare remote: nothing reaches a forge,
//! a credential or the host's own sluice state, and no multiplexer's
//! session reaches sluice. The merge-queue seam's scenarios run against
//! it; then a submission from a herdr pane's hook, a failed entry
//! collected once, a queue nothing drives, a step beside a live driver, a
//! conflict's base revision and a fetch that fails.

use crate::common::*;
use flume::profiles::merge_queue::sluice::{for_place, Sluice};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::merge_queue::{self, MergeQueueHarness};
use flume_seams::*;
use serde_json::Value;
use std::cell::RefCell;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const POLICY: &str =
    "[tests]\nrunner = \"command\"\ncommand = \"sh check.sh\"\n\n[branches.main]\n";

/// A scratch repository gated by sluice, and the place it holds.
struct Harness {
    repository: Repository,
    seam: Sluice,
    home: PathBuf,
    place: PlaceRef,
    locked: RefCell<Option<PathBuf>>,
}

impl Harness {
    fn new() -> Harness {
        let repository = Repository::new("sluice");
        repository.write(".config/sluice.toml", POLICY);
        repository.write("check.sh", "exit 0\n");
        repository.commit("the gate");
        let home = repository.scratch.path("home");
        fs::create_dir_all(&home).expect("the scratch home");
        let mut seam = Sluice::new()
            .with_env("HOME", Some(&home.to_string_lossy()))
            .with_env(
                "XDG_STATE_HOME",
                Some(&home.join("state").to_string_lossy()),
            )
            .with_env(
                "XDG_CONFIG_HOME",
                Some(&home.join("config").to_string_lossy()),
            );
        for (name, _) in std::env::vars() {
            if name.starts_with("SLUICE_") {
                seam = seam.with_env(&name, None);
            }
        }
        let place = Harness::worktree(&repository, "fix/one", "fix--one", "work.rs");
        Harness {
            repository,
            seam,
            home,
            place,
            locked: RefCell::new(None),
        }
    }

    /// A worktree on `branch` cut from main at `places/<short>`, holding
    /// one commit touching `file`.
    fn worktree(repository: &Repository, branch: &str, short: &str, file: &str) -> PlaceRef {
        let location = repository.dir.join("places").join(short);
        repository.git(&[
            "worktree",
            "add",
            "--quiet",
            "-b",
            branch,
            &location.to_string_lossy(),
            "main",
        ]);
        fs::write(location.join(file), format!("{branch}\n")).expect("the work written");
        git(&location, &["add", file]);
        git(&location, &["commit", "--quiet", "-m", branch]);
        PlaceRef {
            repository: repository.name(),
            location: location.to_string_lossy().into(),
            branch: branch.into(),
            base: "main".into(),
        }
    }

    /// A commit on main, past the hook sluice protects it with.
    fn on_main(&self, path: &str, text: &str) -> Revision {
        self.repository.write(path, text);
        self.repository.git(&["add", "--all"]);
        self.repository.git(&[
            "-c",
            "core.hooksPath=/dev/null",
            "commit",
            "--quiet",
            "-m",
            path,
        ]);
        Revision::new(self.repository.git(&["rev-parse", "main"]))
    }

    /// sluice run by the test in `dir`, in the scratch home, with no
    /// multiplexer's session: its JSON answer.
    fn sluice(&self, dir: &Path, args: &[&str]) -> Value {
        let mut command = Command::new("sluice");
        command.args(args).current_dir(dir).stdin(Stdio::null());
        for (name, _) in std::env::vars() {
            if name.starts_with("SLUICE_") || name.starts_with("HERDR_") || name.starts_with("TMUX")
            {
                command.env_remove(name);
            }
        }
        let output = command
            .env("HOME", &self.home)
            .env("XDG_STATE_HOME", self.home.join("state"))
            .env("XDG_CONFIG_HOME", self.home.join("config"))
            .output()
            .expect("sluice runs");
        serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
            panic!(
                "sluice {}: {}{}",
                args.join(" "),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        })
    }

    /// The newest record sluice holds of `branch`'s submissions to main.
    fn record(&self, branch: &str) -> Option<Value> {
        let status = self.sluice(&self.repository.dir, &["status", "main", "--all", "--json"]);
        status["submissions"]
            .as_array()?
            .iter()
            .filter(|record| record["from"]["worktree"] == branch)
            .max_by_key(|record| record["id"].as_u64())
            .cloned()
    }

    /// Steps main until `branch`'s newest submission is decided.
    fn decide(&self, branch: &str) {
        let deadline = Instant::now() + Duration::from_secs(90);
        loop {
            let state = self
                .record(branch)
                .and_then(|record| record["state"].as_str().map(str::to_string));
            if !matches!(state.as_deref(), Some("queued" | "testing")) {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "{branch}'s submission stayed {state:?}"
            );
            self.sluice(&self.repository.dir, &["step", "main", "--json"]);
            std::thread::sleep(Duration::from_millis(300));
        }
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        if let Some(path) = self.locked.borrow_mut().take() {
            let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o755));
        }
    }
}

impl MergeQueueHarness for Harness {
    type Seam = Sluice;

    fn seam(&self) -> &Sluice {
        &self.seam
    }

    fn place(&self) -> (PlaceRef, Revision) {
        let head = git(Path::new(&self.place.location), &["rev-parse", "HEAD"]);
        (self.place.clone(), Revision::new(head))
    }

    fn will_fail(&self, words: &str) {
        self.on_main(
            ".config/sluice.toml",
            &POLICY.replace("sh check.sh", &format!("echo '{words}' >&2; exit 1")),
        );
    }

    fn will_conflict(&self, _place: &PlaceRef, file: &str) {
        self.on_main(file, "the base's own\n");
    }

    fn will_hold(&self) {}

    fn until_decided(&self, place: &PlaceRef) {
        self.decide(&place.branch);
    }

    fn withdraw(&self, place: &PlaceRef) {
        let id = self.record(&place.branch).expect("a submission")["id"].to_string();
        self.sluice(&self.repository.dir, &["withdraw", &id, "--json"]);
    }

    fn land_elsewhere(&self, place: &PlaceRef) {
        let other = self.repository.dir.join("places/other");
        self.repository.git(&[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "other",
            &other.to_string_lossy(),
            &place.branch,
        ]);
        self.sluice(&other, &["submit", "main", "--no-wait", "--json"]);
        self.decide("other");
        assert_eq!(
            self.record("other").expect("the other submission")["state"],
            "landed",
            "the other branch's landing"
        );
    }

    fn open_submission(&self, place: &PlaceRef) -> Submission {
        let entry = self.sluice(
            Path::new(&place.location),
            &["submit", "main", "--no-wait", "--json"],
        );
        Submission {
            id: entry["id"].to_string(),
            branch: place.branch.clone(),
            target: "main".into(),
            state: SubmissionState::Waiting,
        }
    }

    fn base_elsewhere(&self, _repository: &str, base: &str) -> Revision {
        let remote = self.repository.bare_remote();
        self.repository
            .git(&["remote", "add", "origin", &remote.to_string_lossy()]);
        self.on_main(
            ".config/sluice.toml",
            &format!("{POLICY}remote = \"origin\"\n"),
        );
        self.repository.git(&["push", "--quiet", "origin", base]);
        let elsewhere = self.repository.scratch.path("elsewhere");
        git(
            &self.repository.scratch.root,
            &[
                "clone",
                "--quiet",
                &remote.to_string_lossy(),
                &elsewhere.to_string_lossy(),
            ],
        );
        fs::write(elsewhere.join("remote.rs"), "remote\n").expect("the remote's work");
        git(&elsewhere, &["add", "remote.rs"]);
        git(&elsewhere, &["commit", "--quiet", "-m", "on the remote"]);
        git(&elsewhere, &["push", "--quiet", "origin", base]);
        Revision::new(git(&elsewhere, &["rev-parse", "HEAD"]))
    }

    fn fetch_fails(&self, _repository: &str, _base: &str) {
        let gone = self.repository.scratch.path("gone.git");
        self.repository
            .git(&["remote", "set-url", "origin", &gone.to_string_lossy()]);
    }

    fn unreadable(&self) {
        let records = self.repository.dir.join(".git/sluice");
        fs::create_dir_all(&records).expect("sluice's records");
        fs::set_permissions(&records, fs::Permissions::from_mode(0o000))
            .expect("the records locked");
        *self.locked.borrow_mut() = Some(records);
    }
}

#[test]
fn the_sluice_profile_passes_the_merge_queue_scenarios() {
    pass_all("merge queue", &merge_queue::scenarios(), Harness::new);
}

#[test]
fn a_submission_from_a_herdr_panes_hook_names_the_place_and_no_session() {
    let harness = Harness::new();
    let (place, head) = harness.place();
    let seam = harness
        .seam
        .clone()
        .with_env("HERDR_PANE_ID", Some("w1:p1"))
        .with_env("HERDR_SOCKET_PATH", Some("/nowhere/herdr.sock"))
        .with_env("HERDR_ENV", Some("1"))
        .with_env("TMUX", Some("/nowhere/tmux,1,0"))
        .with_env("TMUX_PANE", Some("%1"));
    let Submitted::Submitted { submission } = seam.submit(&place, &head) else {
        panic!("the submit was refused");
    };
    assert_eq!(submission.state, SubmissionState::Waiting, "the answer");
    let record = harness.record("fix/one").expect("sluice's entry");
    assert_eq!(
        record["from"]["path"].as_str(),
        Some(place.location.as_str()),
        "the entry's submitter"
    );
    assert!(
        record["from"].get("session").is_none(),
        "the entry names a session: {record}"
    );
    assert_eq!(record["state"], "queued", "the entry before any verdict");
}

#[test]
fn a_failed_entry_is_collected_once_with_sluices_notice() {
    let harness = Harness::new();
    let (place, head) = harness.place();
    harness.will_fail("test x failed");
    harness.seam.submit(&place, &head);
    harness.decide("fix/one");
    let Inbox::Outcomes { outcomes } = harness.seam.inbox(&place) else {
        panic!("the inbox could not be read");
    };
    match outcomes.as_slice() {
        [SubmissionOutcome::Failed { words }] => assert!(
            words.starts_with("[sluice] ") && words.contains("test x failed"),
            "the words are not sluice's notice: {words}"
        ),
        other => panic!("the first collect gave {other:?}"),
    }
    assert_eq!(
        harness.seam.inbox(&place),
        Inbox::Outcomes { outcomes: vec![] },
        "the second collect"
    );
}

#[test]
fn a_queue_nothing_drives_is_decided_by_the_settles_steps() {
    let harness = Harness::new();
    let (place, head) = harness.place();
    harness.seam.submit(&place, &head);
    let deadline = Instant::now() + Duration::from_secs(90);
    loop {
        assert_eq!(
            harness.seam.step(&place.repository, "main"),
            Stepped::Stepped,
            "a step"
        );
        match harness.seam.read_submission(&place.repository, "fix/one") {
            SubmissionRead::Found { submission } if !submission.state.open() => {
                assert!(
                    matches!(submission.state, SubmissionState::Landed { .. }),
                    "the entry came to {submission:?}"
                );
                return;
            }
            SubmissionRead::Found { .. } => {}
            other => panic!("read-submission gave {other:?}"),
        }
        assert!(Instant::now() < deadline, "the entry was never decided");
        std::thread::sleep(Duration::from_millis(300));
    }
}

#[test]
fn a_step_beside_a_live_driver_does_nothing_and_answers() {
    let harness = Harness::new();
    let (place, head) = harness.place();
    let mut run = Command::new("sluice");
    run.arg("run").current_dir(&harness.repository.dir);
    for (name, _) in std::env::vars() {
        if name.starts_with("SLUICE_") || name.starts_with("HERDR_") || name.starts_with("TMUX") {
            run.env_remove(name);
        }
    }
    let mut run = run
        .env("HOME", &harness.home)
        .env("XDG_STATE_HOME", harness.home.join("state"))
        .env("XDG_CONFIG_HOME", harness.home.join("config"))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0)
        .spawn()
        .expect("sluice run starts");
    harness.seam.submit(&place, &head);
    let began = Instant::now();
    let stepped = harness.seam.step(&place.repository, "main");
    let took = began.elapsed();
    let deadline = Instant::now() + Duration::from_secs(90);
    let landed = loop {
        match harness
            .record("fix/one")
            .and_then(|record| record["state"].as_str().map(str::to_string))
        {
            Some(state) if state != "queued" && state != "testing" => break state,
            _ if Instant::now() >= deadline => break "undecided".into(),
            _ => std::thread::sleep(Duration::from_millis(300)),
        }
    };
    // SAFETY: a signal to the process group of the run this test started.
    unsafe { libc::kill(-(run.id() as libc::pid_t), libc::SIGTERM) };
    let _ = run.kill();
    let _ = run.wait();
    assert_eq!(stepped, Stepped::Stepped, "the step beside the driver");
    assert!(took < Duration::from_secs(30), "the step waited {took:?}");
    assert_eq!(landed, "landed", "the driver's landing");
}

#[test]
fn a_conflicts_base_revision_and_words_are_sluices() {
    let harness = Harness::new();
    let (place, head) = harness.place();
    let base = harness.on_main("work.rs", "the base's own\n");
    harness.seam.submit(&place, &head);
    harness.decide("fix/one");
    let record = harness.record("fix/one").expect("sluice's entry");
    let Inbox::Outcomes { outcomes } = harness.seam.inbox(&place) else {
        panic!("the inbox could not be read");
    };
    match outcomes.as_slice() {
        [SubmissionOutcome::Conflict {
            files,
            base: tested,
            words,
        }] => {
            let expected = record["tested"]["on"]
                .as_str()
                .map_or(base.clone(), Revision::new);
            assert_eq!(tested, &expected, "the base revision");
            assert_eq!(files, &vec!["work.rs".to_string()], "the files contested");
            assert!(
                words.starts_with("[sluice] ") && words.contains("work.rs"),
                "the words are not sluice's notice: {words}"
            );
        }
        other => panic!("a conflicting entry came to {other:?}"),
    }
}

#[test]
fn a_fetch_that_fails_is_answered_with_its_words() {
    let harness = Harness::new();
    let (place, _) = harness.place();
    harness.base_elsewhere(&place.repository, "main");
    harness.fetch_fails(&place.repository, "main");
    match harness.seam.base_revision(&place.repository, "main") {
        BaseRevision::Refused { reason } => assert!(
            reason.contains("fetch") && reason.contains("origin"),
            "the refusal names not the fetch: {reason}"
        ),
        other => panic!("a failed fetch gave {other:?}"),
    }
}

#[test]
fn what_the_policy_says_of_a_place() {
    let harness = Harness::new();
    harness.on_main(
        ".config/sluice.toml",
        "[tests]\nrunner = \"moon\"\ntoolchain = [\"devenv\", \"print-dev-env\"]\n\n[branches.main]\nremote = \"origin\"\n",
    );
    let place = for_place(&harness.repository.name(), "main").expect("the policy");
    assert_eq!(place.toolchain.as_deref(), Some("devenv print-dev-env"));
    assert_eq!(place.setup.as_deref(), Some("moon setup"));
    assert_eq!(place.teardown, "sluice release");
    assert_eq!(place.base_remote.as_deref(), Some("origin"));
}

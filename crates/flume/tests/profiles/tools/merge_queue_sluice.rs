//! The merge-queue profile over sluice (C.6; design 13) against the
//! pinned sluice, on scratch repositories gated by a command and, where a
//! base lands on a remote, a local bare remote: nothing reaches a forge,
//! a credential or the host's own sluice state, sluice's checkouts keep
//! their scratch in the test's own folder, and no multiplexer's session
//! reaches sluice. The merge-queue seam's scenarios run against
//! it; then a submission from a herdr pane's hook, a failed entry
//! collected once, a queue nothing drives, a step beside a live driver, a
//! conflict's base revision, a conflict sluice answers at the submit
//! itself, through the stand-in for sluice, and a settle beside that
//! submit, which the inbox gives nothing, a pull request answered with
//! no entry and the entry taken in from it read to its landing, through
//! the stand-in for sluice on a base that takes pull requests, a branch
//! whose records sluice cannot read passed over by the inbox and
//! read-submission while the place's landing is collected, every branch
//! asked at once read from one overview and one status of each base, a
//! fetch that fails, and what the policy says of a place, its readiness
//! asked of the pinned sluice's equip and its answer read. A base place's
//! branch is opened and closed by the seam's scenarios, an open of a
//! branch the policy names no table for refused and a close of a branch a
//! submission is queued on kept, each in sluice's words (205, 206), and
//! submitted by name from the repository; over a stand-in origin, a base
//! place's branch is made there, a fix lands on it, it lands on main by
//! name, its landing read from sluice's status with no inbox run, and it
//! is closed on the origin and the host (203, S110). A chore submitted
//! with no agent in it lands by the settles of a caller keeping watch
//! alone, `flume settle --every 1m`, each stepping main once, with no
//! `sluice run` live (124, 125, 208, S109).

use super::walk::{stage, user_scratch, Walk, USER_SCRATCH};
use crate::common::*;
use flume::profiles::merge_queue::sluice::{for_place, Sluice, EQUIP};
use flume::profiles::place::git_worktree::{GitWorktree, Login};
use flume::profiles::place_data;
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::merge_queue::{self, MergeQueueHarness};
use flume_seams::*;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// What a place submits from its worktree at `head`.
fn from_worktree(head: &Revision) -> SubmitHead {
    SubmitHead::Worktree { head: head.clone() }
}

const POLICY: &str =
    "[tests]\nrunner = \"command\"\ncommand = \"sh check.sh\"\n\n[branches.main]\n";

/// The policy with a table for `train/*` beside main's, so sluice opens,
/// lands on and closes `train/a`.
const TRAINS: &str = "[tests]\nrunner = \"command\"\ncommand = \"sh check.sh\"\n\n[branches.\"train/*\"]\n\n[branches.main]\n";

/// The policy that lands `train/*` and main on the stand-in origin.
const TRAINS_ON_ORIGIN: &str = "[tests]\nrunner = \"command\"\ncommand = \"sh check.sh\"\n\n[branches.\"train/*\"]\nremote = \"origin\"\n\n[branches.main]\nremote = \"origin\"\n";

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
        Harness::with_policy(POLICY)
    }

    /// A scratch repository whose policy is `policy`.
    fn with_policy(policy: &str) -> Harness {
        let repository = Repository::new("sluice");
        repository.write(".config/sluice.toml", policy);
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
        let seam = seam.with_env(
            USER_SCRATCH,
            Some(&user_scratch(&repository).to_string_lossy()),
        );
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

    /// `path` written with `text` and committed on `branch`, through a
    /// worktree of its own removed after: the branch's new head.
    fn on_branch(&self, branch: &str, path: &str, text: &str) -> Revision {
        let dir = self.repository.dir.join("places/on-branch");
        let at = dir.to_string_lossy().into_owned();
        self.repository
            .git(&["worktree", "add", "--quiet", &at, branch]);
        std::fs::write(dir.join(path), text).expect("the branch's file written");
        git(&dir, &["add", "--all"]);
        git(
            &dir,
            &[
                "-c",
                "core.hooksPath=/dev/null",
                "commit",
                "--quiet",
                "-m",
                path,
            ],
        );
        let head = git(&dir, &["rev-parse", "HEAD"]);
        self.repository.git(&["worktree", "remove", "--force", &at]);
        Revision::new(head)
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
            .env(USER_SCRATCH, user_scratch(&self.repository))
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
            if !matches!(state.as_deref(), Some("queued" | "testing" | "landing")) {
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

impl Harness {
    /// The seam with a `sluice` first on its `PATH` that writes each
    /// command it is asked, a line each, then runs the pinned sluice; and
    /// where it writes them.
    fn counting(&self) -> (Sluice, PathBuf) {
        let pinned = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
            .map(|dir| dir.join("sluice"))
            .find(|path| path.is_file())
            .expect("the pinned sluice on PATH");
        let bin = self.repository.scratch.path("counted");
        fs::create_dir_all(&bin).expect("the counting sluice's folder");
        let asked = self.repository.scratch.path("sluice-counted");
        let program = bin.join("sluice");
        fs::write(
            &program,
            format!(
                "#!/bin/sh\necho \"$*\" >>'{}'\nexec '{}' \"$@\"\n",
                asked.display(),
                pinned.display()
            ),
        )
        .expect("the counting sluice");
        fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("it runs");
        let path = std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        )))
        .expect("a PATH");
        let seam = self
            .seam
            .clone()
            .with_env("PATH", Some(&path.to_string_lossy()));
        (seam, asked)
    }
}

/// Steps `base` through `seam` until `branch`'s newest submission is
/// decided, and gives it as read-submission reads it.
fn stepped_until_decided(seam: &Sluice, repository: &str, base: &str, branch: &str) -> Submission {
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        assert_eq!(
            seam.step(repository, base),
            Stepped::Stepped,
            "a step of {base}"
        );
        match seam.read_submission(repository, branch).read {
            SubmissionRead::Found { submission } if !submission.state.open() => return submission,
            SubmissionRead::Found { .. } => {}
            other => panic!("read-submission of {branch} gave {other:?}"),
        }
        assert!(
            Instant::now() < deadline,
            "{branch}'s submission to {base} was never decided"
        );
        std::thread::sleep(Duration::from_millis(300));
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
            request: None,
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

    /// `trial`, made from main, whose policy names a table for main
    /// alone, so sluice lands nothing on it (126, C.6).
    fn base_unlanded(&self, _repository: &str) -> String {
        self.repository.git(&["branch", "trial", "main"]);
        "trial".into()
    }

    fn unreadable(&self) {
        let records = self.repository.dir.join(".git/sluice");
        fs::create_dir_all(&records).expect("sluice's records");
        fs::set_permissions(&records, fs::Permissions::from_mode(0o000))
            .expect("the records locked");
        *self.locked.borrow_mut() = Some(records);
    }

    /// `bolt/x`, its policy naming it, given a submission from `unit/x`,
    /// then a policy at its tip that declares `[[checkout.cache]]`, which
    /// the pinned sluice refuses: no round of `bolt/x` has read its revision,
    /// so `sluice status bolt/x` reads the policy at its tip and refuses
    /// the branch, as it refused one on deneb on 2026-11-04.
    fn refuse_another(&self, _place: &PlaceRef) -> String {
        let other = "bolt/x";
        let at = self.repository.dir.join("places/bolt--x");
        self.repository.git(&[
            "worktree",
            "add",
            "--quiet",
            "-b",
            other,
            &at.to_string_lossy(),
            "main",
        ]);
        let policy = format!("{POLICY}\n[branches.\"bolt/*\"]\n");
        let commit = |text: &str| {
            fs::write(at.join(".config/sluice.toml"), text).expect("bolt/x's policy");
            git(&at, &["add", ".config/sluice.toml"]);
            git(
                &at,
                &[
                    "-c",
                    "core.hooksPath=/dev/null",
                    "commit",
                    "--quiet",
                    "-m",
                    "bolt/x's policy",
                ],
            );
        };
        commit(&policy);
        let unit = self.repository.dir.join("places/unit--x");
        self.repository.git(&[
            "worktree",
            "add",
            "--quiet",
            "-b",
            "unit/x",
            &unit.to_string_lossy(),
            other,
        ]);
        fs::write(unit.join("x.rs"), "unit/x\n").expect("the work written");
        git(&unit, &["add", "x.rs"]);
        git(&unit, &["commit", "--quiet", "-m", "unit/x"]);
        let entry = self.sluice(&unit, &["submit", other, "--no-wait", "--json"]);
        assert!(entry["id"].is_u64(), "bolt/x's submission: {entry}");
        commit(&format!(
            "{policy}\n[[checkout.cache]]\nkey = [\"Cargo.lock\"]\npaths = [\"target\"]\n"
        ));
        other.into()
    }

    fn branch_head(&self, _repository: &str, branch: &str) -> Option<Revision> {
        let head = self
            .repository
            .git(&["branch", "--list", branch, "--format=%(objectname)"]);
        (!head.is_empty()).then(|| Revision::new(head))
    }

    fn change_on(&self, _repository: &str, branch: &str, file: &str) -> Revision {
        self.on_branch(branch, file, "the branch's own\n")
    }

    fn move_base_against(
        &self,
        _repository: &str,
        _base: &str,
        _branch: &str,
        file: &str,
    ) -> Revision {
        self.on_main(file, "the base's own\n")
    }
}

#[test]
fn the_sluice_profile_passes_the_merge_queue_scenarios() {
    pass_all("merge queue", &merge_queue::scenarios(), Harness::new);
}

#[test]
fn the_sluice_profile_passes_the_base_place_scenarios() {
    // The seam's base-place scenarios, over a policy that names
    // `train/*`: a branch opened and closed, and submitted by name from
    // the repository (203, 205, 206).
    pass_all(
        "merge queue's base place",
        &merge_queue::base_place_scenarios(),
        || Harness::with_policy(TRAINS),
    );
}

#[test]
fn a_base_places_branch_over_a_scratch_repository() {
    // A base place's branch through the profile alone, on a scratch
    // repository whose policy lands `train/*` and main on a stand-in
    // origin: sluice makes `train/a` there, a fix cut from it lands on it
    // by the settles' steps, the base place's submission of `train/a` by
    // name from the repository lands on main by them, its landing read
    // from sluice's status with no inbox run, and its close removes it on
    // the origin and the host. The profile runs sluice and a fetch alone,
    // so nothing of flume's pushes or moves a branch (14, 202 to 206,
    // S110).
    let harness = Harness::with_policy(TRAINS_ON_ORIGIN);
    let repository = harness.repository.name();
    let remote = harness.repository.bare_remote();
    harness
        .repository
        .git(&["remote", "add", "origin", &remote.to_string_lossy()]);
    harness.repository.git(&["fetch", "--quiet", "origin"]);
    let on_origin = |branch: &str| {
        let line = git(
            &harness.repository.dir,
            &[
                "ls-remote",
                &remote.to_string_lossy(),
                &format!("refs/heads/{branch}"),
            ],
        );
        line.split_whitespace().next().map(str::to_string)
    };
    let main = on_origin("main").expect("main on the origin");

    let BranchOpened::Made { revision } = harness.seam.open(&repository, "train/a", "main") else {
        panic!("train/a was not opened");
    };
    assert_eq!(revision.0, main, "train/a made at main's revision");
    assert_eq!(
        on_origin("train/a").as_deref(),
        Some(main.as_str()),
        "train/a on the origin"
    );

    harness
        .repository
        .git(&["fetch", "--quiet", "origin", "train/a"]);
    let fix = harness.repository.dir.join("places/fix--a");
    harness.repository.git(&[
        "worktree",
        "add",
        "--quiet",
        "-b",
        "fix/a",
        &fix.to_string_lossy(),
        "origin/train/a",
    ]);
    fs::write(fix.join("a.rs"), "fix/a\n").expect("the fix's work");
    git(&fix, &["add", "a.rs"]);
    git(&fix, &["commit", "--quiet", "-m", "fix/a"]);
    let fix_place = PlaceRef {
        repository: repository.clone(),
        location: fix.to_string_lossy().into(),
        branch: "fix/a".into(),
        base: "train/a".into(),
    };
    let head = Revision::new(git(&fix, &["rev-parse", "HEAD"]));
    let Submitted::Submitted { .. } = harness.seam.submit(&fix_place, &from_worktree(&head)) else {
        panic!("the fix's submit to train/a was refused");
    };
    let landed = stepped_until_decided(&harness.seam, &repository, "train/a", "fix/a");
    assert!(
        matches!(landed.state, SubmissionState::Landed { .. }),
        "the fix came to {landed:?}"
    );
    let train = on_origin("train/a").expect("train/a on the origin");
    assert_eq!(
        git(
            &harness.repository.dir,
            &[
                "--git-dir",
                &remote.to_string_lossy(),
                "show",
                &format!("{train}:a.rs")
            ],
        ),
        "fix/a",
        "the fix on train/a"
    );

    let (seam, asked) = harness.counting();
    let base_place = PlaceRef {
        repository: repository.clone(),
        location: harness
            .repository
            .dir
            .join("places/train--a")
            .to_string_lossy()
            .into(),
        branch: "train/a".into(),
        base: "main".into(),
    };
    let submission = match seam.submit(
        &base_place,
        &SubmitHead::Branch {
            branch: "train/a".into(),
            revision: Revision::new(train.clone()),
        },
    ) {
        Submitted::Submitted { submission } => submission,
        other => panic!("train/a's submit by name answered {other:?}"),
    };
    assert_eq!(
        (submission.branch.as_str(), submission.target.as_str()),
        ("train/a", "main"),
        "the submission by name"
    );
    let landed = stepped_until_decided(&seam, &repository, "main", "train/a");
    assert_eq!(landed.id, submission.id, "the landing read by its branch");
    assert!(
        matches!(landed.state, SubmissionState::Landed { .. }),
        "train/a came to {landed:?}"
    );
    let main = on_origin("main").expect("main on the origin");
    assert_eq!(
        git(
            &harness.repository.dir,
            &[
                "--git-dir",
                &remote.to_string_lossy(),
                "show",
                &format!("{main}:a.rs")
            ],
        ),
        "fix/a",
        "train/a's work on main"
    );
    let asked = fs::read_to_string(&asked).expect("sluice was asked");
    assert!(
        asked
            .lines()
            .next()
            .is_some_and(|line| line == "submit main --head train/a --no-wait --json"),
        "the submit by name: {asked}"
    );
    assert!(
        !asked.lines().any(|line| line.starts_with("inbox")),
        "an inbox ran for a submission by name: {asked}"
    );
    let worktrees = harness.repository.git(&["worktree", "list", "--porcelain"]);
    assert!(
        !worktrees.contains("branch refs/heads/train/a"),
        "a worktree on train/a: {worktrees}"
    );
    assert!(
        !Path::new(&base_place.location).exists(),
        "something stands at the base place's location"
    );

    assert_eq!(
        harness.seam.close(&repository, "train/a", "main"),
        BranchClosed::Removed,
        "train/a's close into main"
    );
    assert_eq!(on_origin("train/a"), None, "train/a on the origin");
    assert_eq!(
        harness.branch_head(&repository, "train/a"),
        None,
        "train/a on the host"
    );
}

#[test]
fn a_base_places_branch_forecast_over_a_scratch_repository() {
    // The merge-queue-profile spec's "A base place's branch forecast over
    // a scratch repository": sluice lands main and `train/*` on a stand-in
    // origin; flume opens `train/a`, a fix lands a change to README.md on
    // it, and the forecast of `train/a` onto main is clear at main's
    // revision; a change to README.md lands on main, and the next is in
    // conflict on README.md at main's new revision, meeting main itself.
    // Neither leaves an entry of `train/a` in main's records (203, 210;
    // sluice 64).
    let harness = Harness::with_policy(TRAINS_ON_ORIGIN);
    let repository = harness.repository.name();
    let remote = harness.repository.bare_remote();
    harness
        .repository
        .git(&["remote", "add", "origin", &remote.to_string_lossy()]);
    harness.repository.git(&["fetch", "--quiet", "origin"]);
    let on_origin = |branch: &str| {
        let line = git(
            &harness.repository.dir,
            &[
                "ls-remote",
                &remote.to_string_lossy(),
                &format!("refs/heads/{branch}"),
            ],
        );
        line.split_whitespace().next().map(Revision::new)
    };
    let BranchOpened::Made { .. } = harness.seam.open(&repository, "train/a", "main") else {
        panic!("train/a was not opened");
    };
    // A place on `from`, cut where the origin holds it, landing a change
    // to README.md written `text` on it.
    let lands = |name: &str, from: &str, text: &str| {
        harness
            .repository
            .git(&["fetch", "--quiet", "origin", from]);
        let location = harness.repository.dir.join("places").join(name);
        harness.repository.git(&[
            "worktree",
            "add",
            "--quiet",
            "-b",
            name,
            &location.to_string_lossy(),
            &format!("origin/{from}"),
        ]);
        fs::write(location.join("README.md"), text).expect("the work written");
        git(&location, &["add", "README.md"]);
        git(&location, &["commit", "--quiet", "-m", name]);
        let place = PlaceRef {
            repository: repository.clone(),
            location: location.to_string_lossy().into(),
            branch: name.into(),
            base: from.into(),
        };
        let head = Revision::new(git(&location, &["rev-parse", "HEAD"]));
        let Submitted::Submitted { .. } = harness.seam.submit(&place, &from_worktree(&head)) else {
            panic!("{name}'s submit to {from} was refused");
        };
        let landed = stepped_until_decided(&harness.seam, &repository, from, name);
        assert!(
            matches!(landed.state, SubmissionState::Landed { .. }),
            "{name} came to {landed:?}"
        );
    };
    let no_entry = || {
        let status = harness.sluice(
            &harness.repository.dir,
            &["status", "main", "--all", "--json"],
        );
        let entries: Vec<&Value> = status["submissions"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|record| {
                record["from"]["head"] == "train/a" || record["from"]["worktree"] == "train/a"
            })
            .collect();
        assert!(entries.is_empty(), "train/a's entries on main: {entries:?}");
    };

    lands("fix-a", "train/a", "the branch's own\n");
    let head = on_origin("train/a").expect("train/a on the origin");
    let main = on_origin("main").expect("main on the origin");
    assert_eq!(
        harness.seam.forecast(&repository, "train/a", "main"),
        Forecast::Clear {
            head: head.clone(),
            onto: main,
        },
        "the forecast while the two merge"
    );
    no_entry();

    lands("other", "main", "the base's own\n");
    let main = on_origin("main").expect("main on the origin");
    match harness.seam.forecast(&repository, "train/a", "main") {
        Forecast::Conflict {
            head: at,
            onto,
            files,
            with,
            words,
        } => {
            assert_eq!(at, head, "the head forecast");
            assert_eq!(onto, main, "main's new revision");
            assert_eq!(files, vec!["README.md".to_string()], "the files contested");
            assert!(with.is_empty(), "a conflict with main met {with:?}");
            assert!(
                words.contains("README.md") && words.contains(&main.0),
                "the words: {words}"
            );
        }
        other => panic!("the forecast once main moved: {other:?}"),
    }
    no_entry();
}

#[test]
fn an_open_of_a_branch_the_policy_names_no_table_for_is_refused_in_sluices_words() {
    let harness = Harness::new();
    let repository = harness.repository.name();
    match harness.seam.open(&repository, "other/a", "main") {
        BranchOpened::Refused { reason } => assert!(
            reason.starts_with("sluice branch create: ")
                && reason.contains("no [branches] table")
                && reason.contains("other/a"),
            "the refusal is not sluice's words: {reason}"
        ),
        other => panic!("an open the policy names no table for answered {other:?}"),
    }
    assert_eq!(
        harness.branch_head(&repository, "other/a"),
        None,
        "a branch made"
    );
}

#[test]
fn a_close_sluice_refuses_keeps_the_branch_in_its_words() {
    let harness = Harness::with_policy(TRAINS);
    let repository = harness.repository.name();
    let BranchOpened::Made { revision } = harness.seam.open(&repository, "train/a", "main") else {
        panic!("train/a was not opened");
    };
    let fix = harness.repository.dir.join("places/fix--a");
    harness.repository.git(&[
        "worktree",
        "add",
        "--quiet",
        "-b",
        "fix/a",
        &fix.to_string_lossy(),
        "train/a",
    ]);
    fs::write(fix.join("a.rs"), "fix/a\n").expect("the fix's work");
    git(&fix, &["add", "a.rs"]);
    git(&fix, &["commit", "--quiet", "-m", "fix/a"]);
    let entry = harness.sluice(&fix, &["submit", "train/a", "--no-wait", "--json"]);
    assert!(entry["id"].is_u64(), "the fix's submission: {entry}");
    match harness.seam.close(&repository, "train/a", "main") {
        BranchClosed::Kept { reason } => assert!(
            reason.starts_with("sluice branch delete: ") && reason.contains("queued on train/a"),
            "the refusal is not sluice's words: {reason}"
        ),
        other => panic!("a close with a submission queued answered {other:?}"),
    }
    assert_eq!(
        harness.branch_head(&repository, "train/a"),
        Some(revision),
        "the branch a close was refused"
    );
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
    let Submitted::Submitted { submission } = seam.submit(&place, &from_worktree(&head)) else {
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
    harness.seam.submit(&place, &from_worktree(&head));
    harness.decide("fix/one");
    let Inbox::Outcomes { outcomes, .. } = harness.seam.inbox(&place) else {
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
        Inbox::Outcomes {
            outcomes: vec![],
            passed_over: vec![]
        },
        "the second collect"
    );
}

#[test]
fn every_branch_asked_at_once_is_read_from_one_overview_and_one_status_of_each_base() {
    let harness = Harness::new();
    let (one, head) = harness.place();
    let two = Harness::worktree(&harness.repository, "fix/two", "fix--two", "two.rs");
    let two_head = Revision::new(git(Path::new(&two.location), &["rev-parse", "HEAD"]));
    let submitted = |place: &PlaceRef, head: &Revision| match harness
        .seam
        .submit(place, &from_worktree(head))
    {
        Submitted::Submitted { submission } => submission.id,
        other => panic!("the submit gave {other:?}"),
    };
    let ids = [submitted(&one, &head), submitted(&two, &two_head)];
    let real = Command::new("sh")
        .args(["-c", "command -v sluice"])
        .output()
        .expect("sh runs");
    let real = String::from_utf8_lossy(&real.stdout).trim().to_string();
    let bin = harness.repository.scratch.path("counting");
    fs::create_dir_all(&bin).expect("the counting sluice's folder");
    let ran = harness.repository.scratch.path("sluice-ran");
    let counting = bin.join("sluice");
    fs::write(
        &counting,
        format!(
            "#!/bin/sh\necho \"$1\" >>'{}'\nexec '{real}' \"$@\"\n",
            ran.display()
        ),
    )
    .expect("the counting sluice");
    fs::set_permissions(&counting, fs::Permissions::from_mode(0o755)).expect("it runs");
    let path = std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .expect("a PATH");
    let seam = harness
        .seam
        .clone()
        .with_env("PATH", Some(&path.to_string_lossy()));
    let read = seam.read_submissions(&one.repository, &["fix/one", "fix/two", "fix/none"]);
    let found: Vec<Option<(String, String)>> = read
        .iter()
        .map(|across| match &across.read {
            SubmissionRead::Found { submission } => {
                Some((submission.id.clone(), submission.branch.clone()))
            }
            SubmissionRead::None => None,
            other => panic!("read-submission gave {other:?}"),
        })
        .collect();
    assert_eq!(
        found,
        vec![
            Some((ids[0].clone(), "fix/one".to_string())),
            Some((ids[1].clone(), "fix/two".to_string())),
            None
        ],
        "each branch's own submission"
    );
    let ran = fs::read_to_string(&ran).expect("sluice ran");
    assert_eq!(
        ran.lines().collect::<Vec<_>>(),
        vec!["overview", "status"],
        "sluice's commands, once for the three branches"
    );
}

#[test]
fn a_queue_nothing_drives_is_decided_by_the_settles_steps() {
    let harness = Harness::new();
    let (place, head) = harness.place();
    harness.seam.submit(&place, &from_worktree(&head));
    let deadline = Instant::now() + Duration::from_secs(90);
    loop {
        assert_eq!(
            harness.seam.step(&place.repository, "main"),
            Stepped::Stepped,
            "a step"
        );
        match harness
            .seam
            .read_submission(&place.repository, "fix/one")
            .read
        {
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
        .env(USER_SCRATCH, user_scratch(&harness.repository))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0)
        .spawn()
        .expect("sluice run starts");
    harness.seam.submit(&place, &from_worktree(&head));
    let began = Instant::now();
    let stepped = harness.seam.step(&place.repository, "main");
    let took = began.elapsed();
    let deadline = Instant::now() + Duration::from_secs(90);
    let landed = loop {
        match harness
            .record("fix/one")
            .and_then(|record| record["state"].as_str().map(str::to_string))
        {
            Some(state) if !matches!(state.as_str(), "queued" | "testing" | "landing") => {
                break state
            }
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
    harness.seam.submit(&place, &from_worktree(&head));
    harness.decide("fix/one");
    let record = harness.record("fix/one").expect("sluice's entry");
    let Inbox::Outcomes { outcomes, .. } = harness.seam.inbox(&place) else {
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
fn a_conflict_sluice_answers_at_the_submit_is_its_outcome_never_a_refusal() {
    // sluice answers the submit with the entry's record, in conflict with
    // the one queued ahead of it, exiting as a conflict does, and records
    // it delivered: submit answers the outcome, and no inbox gives it.
    let harness = Harness::new();
    let (place, head) = harness.place();
    let other = Harness::worktree(&harness.repository, "other", "other", "work.rs");
    harness.sluice(
        Path::new(&other.location),
        &["submit", "main", "--no-wait", "--json"],
    );
    let bin = harness.repository.scratch.path("stand-in");
    super::walk::stand_in_sluice(&bin);
    let path = std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .expect("a PATH");
    let seam = harness
        .seam
        .clone()
        .with_env("PATH", Some(&path.to_string_lossy()));
    let answer = seam.submit(&place, &from_worktree(&head));
    let record = harness.record("fix/one").expect("sluice's entry");
    assert_eq!(record["state"], "conflict", "{record}");
    let Submitted::Decided {
        submission,
        outcome: SubmissionOutcome::Conflict { files, base, words },
    } = answer
    else {
        panic!("a conflict at the submit was answered {answer:?}");
    };
    assert_eq!(submission.id, record["id"].to_string(), "the entry");
    assert!(
        matches!(submission.state, SubmissionState::Conflict { .. }),
        "{submission:?}"
    );
    assert_eq!(files, vec!["work.rs".to_string()], "the files contested");
    let ahead = harness.record("other").expect("the entry ahead")["id"].to_string();
    for named in [
        "conflict on main: work.rs with ",
        &format!("#{ahead} ("),
        "other)",
    ] {
        assert!(words.contains(named), "{named} is not in {words}");
    }
    let expected = record["tested"]["on"].as_str().map_or_else(
        || Revision::new(harness.repository.git(&["rev-parse", "main"])),
        Revision::new,
    );
    assert_eq!(base, expected, "the base revision");
    assert_eq!(
        harness.seam.inbox(&place),
        Inbox::Outcomes {
            outcomes: vec![],
            passed_over: vec![]
        },
        "the inbox, once the answer delivered the outcome"
    );
}

#[test]
fn a_settle_beside_the_stand_ins_submit_collects_nothing_its_answer_delivers() {
    // While the stand-in's first submit waits on the rounds that decide
    // it, a settle beside it steps main and collects the place's inbox,
    // as the agent's Stop hook and a caller's settle do: the inbox gives
    // nothing, and the conflict reaches flume once, in the submit's
    // answer with its own words, as a pull request's entry sluice decides
    // as it takes it in reaches the place only by that answer (112, 133).
    let harness = Harness::new();
    let (place, head) = harness.place();
    harness.on_main("work.rs", "the base's own\n");
    let bin = harness.repository.scratch.path("stand-in");
    super::walk::stand_in_sluice(&bin);
    let path = std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .expect("a PATH");
    let seam = harness
        .seam
        .clone()
        .with_env("PATH", Some(&path.to_string_lossy()));
    let (answer, collected) = std::thread::scope(|scope| {
        let submit = scope.spawn(|| seam.submit(&place, &from_worktree(&head)));
        let mut collected = vec![];
        while !submit.is_finished() {
            seam.step(&place.repository, "main");
            if let Inbox::Outcomes { outcomes, .. } = seam.inbox(&place) {
                collected.extend(outcomes);
            }
        }
        (submit.join().expect("the submit"), collected)
    });
    assert_eq!(collected, vec![], "the inbox beside the submit");
    let Submitted::Decided {
        outcome: SubmissionOutcome::Conflict { files, words, .. },
        ..
    } = answer
    else {
        panic!("the submit was answered {answer:?}");
    };
    assert_eq!(files, vec!["work.rs".to_string()], "the files contested");
    assert!(
        words.starts_with("conflict on main: work.rs"),
        "the record's own words: {words}"
    );
    assert_eq!(
        harness.seam.inbox(&place),
        Inbox::Outcomes {
            outcomes: vec![],
            passed_over: vec![]
        },
        "the inbox, once the answer delivered the outcome"
    );
}

#[test]
fn a_pull_request_with_no_entry_is_held_and_the_entry_taken_in_from_it_is_read_to_its_landing() {
    // On a base that takes pull requests, sluice answers the submit with
    // the request and no entry: submit answers it held, never refused.
    // Once an intake takes it in, read-submission finds the entry sluice
    // records from `pull/<n>`, the place's branch its `from.head`, and
    // reads it to its landing; the inbox gives nothing, the outcome
    // having gone to the request.
    let harness = Harness::new();
    let (place, head) = harness.place();
    let bin = harness.repository.scratch.path("stand-in");
    super::walk::stand_in_pull_requests(&bin);
    let path = std::env::join_paths(std::iter::once(bin.clone()).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .expect("a PATH");
    let seam = harness
        .seam
        .clone()
        .with_env("PATH", Some(&path.to_string_lossy()));
    let request = super::walk::PULL_REQUEST.to_string();
    assert_eq!(
        seam.submit(&place, &from_worktree(&head)),
        Submitted::Requested {
            request: request.clone()
        }
    );
    assert_eq!(
        seam.read_submission(&place.repository, &place.branch),
        SubmissionRead::None.into(),
        "before it is taken in"
    );
    let mut take_in = Command::new(bin.join("sluice"));
    for (name, _) in std::env::vars() {
        if name.starts_with("SLUICE_") || name.starts_with("HERDR_") || name.starts_with("TMUX") {
            take_in.env_remove(name);
        }
    }
    let took = take_in
        .arg("take-in")
        .current_dir(&harness.repository.dir)
        .env("PATH", &path)
        .env("HOME", &harness.home)
        .env("XDG_STATE_HOME", harness.home.join("state"))
        .env("XDG_CONFIG_HOME", harness.home.join("config"))
        .env(USER_SCRATCH, user_scratch(&harness.repository))
        .stdin(Stdio::null())
        .output()
        .expect("the stand-in takes the request in");
    assert!(took.status.success(), "{took:?}");
    let entry = harness.record("fix/one").expect("the entry taken in");
    let SubmissionRead::Found { submission } =
        seam.read_submission(&place.repository, &place.branch).read
    else {
        panic!("the entry taken in is not read");
    };
    assert_eq!(submission.id, entry["id"].to_string(), "the entry");
    assert_eq!(submission.request.as_deref(), Some(request.as_str()));
    assert_eq!(submission.target, "main");
    assert!(submission.state.open(), "{submission:?}");
    harness.decide("fix/one");
    let landed = harness.record("fix/one").expect("the entry decided");
    assert_eq!(landed["state"], "landed", "{landed}");
    assert_eq!(
        seam.read_submission(&place.repository, &place.branch).read,
        SubmissionRead::Found {
            submission: Submission {
                state: SubmissionState::Landed {
                    commit: Revision::new(landed["landed_as"].as_str().expect("its commit")),
                },
                ..submission
            }
        },
        "the landing"
    );
    assert_eq!(
        seam.inbox(&place),
        Inbox::Outcomes {
            outcomes: vec![],
            passed_over: vec![]
        },
        "the inbox, the outcome delivered to the request"
    );
}

#[test]
fn a_branch_whose_records_sluice_cannot_read_is_passed_over_and_the_landing_collected() {
    // sluice answers the inbox with every other branch's outcomes and the
    // branch it could not read in its `unread`, exiting as a failure:
    // the inbox passes that branch over with sluice's words, and the
    // place's landing on main is collected once (133).
    let harness = Harness::new();
    let (place, head) = harness.place();
    harness.seam.submit(&place, &from_worktree(&head));
    let other = harness.refuse_another(&place);
    let records = harness
        .repository
        .dir
        .join(".git/sluice")
        .join(&other)
        .join("branch.json");
    assert!(records.exists(), "{other}'s records at {records:?}");
    fs::set_permissions(&records, fs::Permissions::from_mode(0o000)).expect("the records locked");
    *harness.locked.borrow_mut() = Some(records);
    harness.decide("fix/one");
    let landed = harness.record("fix/one").expect("the entry decided");
    assert_eq!(landed["state"], "landed", "{landed}");
    let commit = Revision::new(landed["landed_as"].as_str().expect("its commit"));
    let Inbox::Outcomes {
        outcomes,
        passed_over,
    } = harness.seam.inbox(&place)
    else {
        panic!("the inbox past {other} could not be read");
    };
    assert_eq!(
        outcomes,
        vec![SubmissionOutcome::Landed {
            commit: commit.clone()
        }],
        "the landing collected past {other}"
    );
    let branches: Vec<&str> = passed_over
        .iter()
        .map(|over| over.branch.as_str())
        .collect();
    assert_eq!(branches, vec![other.as_str()], "{passed_over:?}");
    assert!(
        passed_over[0].words.starts_with("sluice inbox: "),
        "{passed_over:?}"
    );
    let Inbox::Outcomes { outcomes, .. } = harness.seam.inbox(&place) else {
        panic!("the second inbox past {other} could not be read");
    };
    assert_eq!(outcomes, vec![], "the second collect");
    let across = harness
        .seam
        .read_submission(&place.repository, &place.branch);
    assert_eq!(
        across
            .passed_over
            .iter()
            .map(|over| over.branch.as_str())
            .collect::<Vec<_>>(),
        vec![other.as_str()],
        "{across:?}"
    );
    assert!(
        matches!(
            across.read,
            SubmissionRead::Found {
                submission: Submission {
                    state: SubmissionState::Landed { .. },
                    ..
                }
            }
        ),
        "{across:?}"
    );
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
    let repository = harness.repository.name();
    let launched = harness.on_main(
        ".config/sluice.toml",
        "[tests]\nrunner = \"moon\"\ntoolchain = [\"devenv\", \"print-dev-env\"]\n\n[branches.main]\nremote = \"origin\"\n",
    );
    let equip = format!("sluice equip main --at {} --json", launched.0);
    let place = for_place(&repository, "main", Some(&launched)).expect("the policy");
    assert_eq!(place.equip.as_deref(), Some(equip.as_str()));
    assert_eq!(place.teardown, "sluice release");
    assert_eq!(place.base_remote.as_deref(), Some("origin"));
    let data =
        place_data(&repository, Some("sluice"), "main", Some(&launched)).expect("the place's data");
    let readiness: Vec<(&str, &str, bool)> = data
        .readiness
        .iter()
        .map(|step| {
            (
                step.name.as_str(),
                step.command.as_str(),
                step.reading.is_some(),
            )
        })
        .collect();
    assert_eq!(
        readiness,
        vec![(EQUIP, equip.as_str(), true)],
        "a readiness of more than sluice's equip, read"
    );
    let teardown: Vec<&str> = data
        .teardown
        .iter()
        .map(|step| step.command.as_str())
        .collect();
    assert_eq!(teardown, vec!["sluice release"]);
    assert_eq!(data.base_remote.as_deref(), Some("origin"));

    // The pinned sluice equips the place as the step asks it, at the
    // revision named, and its answer is read as the readiness's steps.
    let equipping = harness.on_main(
        ".config/sluice.toml",
        "[tests]\nrunner = \"command\"\ncommand = \"sh check.sh\"\ntoolchain = [\"printf\", \"export READY=1\\\\n\"]\n\n[branches.main]\n",
    );
    let data = place_data(&repository, Some("sluice"), "main", Some(&equipping))
        .expect("the place's data");
    let path = std::env::var("PATH").unwrap_or_default();
    let profile = GitWorktree::with_login(
        data,
        Login::given(&[
            ("HOME", &harness.home.to_string_lossy()),
            ("PATH", &path),
            ("LANG", "C"),
        ]),
    );
    let readiness = profile.ready(&harness.place, &equipping);
    let steps: Vec<(&str, bool)> = readiness
        .steps
        .iter()
        .map(|step| (step.step.as_str(), step.passed))
        .collect();
    assert_eq!(steps, vec![("toolchain", true)], "{readiness:?}");
}

/// A `sluice` first on the walk's `PATH` that writes each command it is
/// asked, a line each after the process that asked it, then runs the
/// pinned sluice as the test's own `PATH` finds it.
fn counting_sluice(walk: &Walk) -> PathBuf {
    let pinned = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
        .map(|dir| dir.join("sluice"))
        .find(|path| path.is_file())
        .expect("the pinned sluice on PATH");
    let asked = walk.repository.scratch.path("sluice-asked");
    let program = walk.repository.scratch.path("bin").join("sluice");
    fs::write(
        &program,
        format!(
            "#!/bin/sh\necho \"$PPID $*\" >>'{}'\nexec '{}' \"$@\"\n",
            asked.display(),
            pinned.display()
        ),
    )
    .expect("the counting sluice");
    fs::set_permissions(&program, fs::Permissions::from_mode(0o755)).expect("it runs");
    asked
}

/// The commands the counting sluice was asked, in order, each with the
/// process that asked it.
fn asked(at: &Path) -> Vec<(i32, String)> {
    fs::read_to_string(at)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| {
            let (by, command) = line.split_once(' ')?;
            Some((by.parse().ok()?, command.to_string()))
        })
        .collect()
}

/// Waits until the process that submitted main, the settle the agent's
/// Stop hook started detached once the agent was quiet (121), has ended,
/// so nothing but the test's own commands moves the place after it.
fn until_the_quiet_settle_ends(log: &Path) {
    let deadline = Instant::now() + Duration::from_secs(120);
    let settle = loop {
        if let Some((by, _)) = asked(log)
            .into_iter()
            .find(|(_, command)| command.starts_with("submit main"))
        {
            break by;
        }
        assert!(Instant::now() < deadline, "nothing submitted main");
        std::thread::sleep(Duration::from_millis(200));
    };
    loop {
        // SAFETY: signal 0 to a process of the test's own user only asks
        // whether it stands.
        if unsafe { libc::kill(settle, 0) } != 0 {
            let error = std::io::Error::last_os_error();
            assert_eq!(
                error.raw_os_error(),
                Some(libc::ESRCH),
                "the settle that submitted main, {settle}: {error}"
            );
            return;
        }
        assert!(
            Instant::now() < deadline,
            "the settle that submitted main, {settle}, never ended"
        );
        std::thread::sleep(Duration::from_millis(200));
    }
}

#[test]
fn s109_a_submitted_chore_with_no_agent_lands_by_a_watchs_settles_alone() {
    // S109: a chore's agent commits and ends, and the settle its quiet
    // starts submits it; from then on the test runs nothing but a
    // caller's watch, `flume settle --every 1m`, and no `sluice run` is
    // live. Each settle while the submission is open steps main once, it
    // lands with no agent started after its code stage, and the settle
    // that reads the landing tidies it (121, 124, 125, 208).
    let walk = Walk::new("watch-chore");
    let log = counting_sluice(&walk);
    let place = walk.launch("chore", "one", &["--brief", "tidy the parser"]);
    walk.agent(
        &place,
        json!([
            { "write": { "path": "parser.rs", "text": "fn parse() {}\n" } },
            { "bash": "git add -A && git commit --quiet -m 'tidy the parser'" },
        ]),
    );
    until_the_quiet_settle_ends(&log);
    let submitted = walk.status(&place);
    assert_eq!(submitted["submission"]["submits"], 1, "{submitted}");
    let entry = walk.submissions();
    assert_eq!(entry.len(), 1, "{entry:?}");
    assert!(
        matches!(
            entry[0]["state"].as_str(),
            Some("queued" | "testing" | "landing")
        ),
        "the chore was decided before the watch: {entry:?}"
    );
    let watched = asked(&log).len();
    let steps = |from: usize| {
        asked(&log)[from..]
            .iter()
            .filter(|(_, command)| command.starts_with("step main"))
            .count()
    };
    let began = Instant::now();
    let mut settles: Vec<usize> = vec![];
    loop {
        let from = asked(&log).len();
        walk.flume(&["settle", "--every", "1m"]);
        settles.push(steps(from));
        let status = walk.status(&place);
        if status.is_null() {
            break;
        }
        assert_eq!(stage(&status, "code")["starts"], 1, "{status}");
        assert!(status["agent"].is_null(), "an agent of the chore: {status}");
        assert_eq!(
            walk.host()["watch"],
            json!({"bound": {"seconds": 60}}),
            "the host's watch"
        );
        assert!(
            began.elapsed() < Duration::from_secs(120),
            "the chore never landed: {settles:?} {status}"
        );
        std::thread::sleep(Duration::from_millis(300));
    }

    // Each settle before the one that read the landing stepped main once,
    // and that one at most once, the round it read having landed at its
    // own step or at the step before it.
    let last = settles.len() - 1;
    assert!(
        settles[..last].iter().all(|steps| *steps == 1) && settles[last] <= 1,
        "the steps of each settle: {settles:?}"
    );
    assert!(settles.contains(&1), "no settle stepped main: {settles:?}");
    let asked = asked(&log);
    assert!(
        !asked
            .iter()
            .any(|(_, command)| command.starts_with("run") || command.starts_with("wait")),
        "a sluice run or wait: {asked:?}"
    );
    assert!(
        !asked[watched..]
            .iter()
            .any(|(_, command)| command.starts_with("submit")),
        "a submit under the watch: {asked:?}"
    );

    // sluice landed flume's one submission of the chore on the remote, and
    // the settle that read it tidied the place and its branch.
    let submissions = walk.submissions();
    assert_eq!(submissions.len(), 1, "{submissions:?}");
    assert_eq!(submissions[0]["state"], "landed", "{submissions:?}");
    assert_eq!(
        walk.on_remote("parser.rs").as_deref(),
        Some("fn parse() {}\n")
    );
    assert!(!Path::new(&place).exists(), "the place is left");
    assert!(!walk.has_branch("chore/one"), "the place's branch is left");
}

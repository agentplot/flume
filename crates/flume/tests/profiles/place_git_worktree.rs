//! The place profile over plain git worktree (C.1; design 9): the place
//! seam's scenarios against it on scratch repositories, S9, S10, S16, a
//! repository that binds no merge queue, a make that runs no hook, where a
//! place goes for the bare repository beside its worktrees and for a
//! clone, a place made inside a clone's main worktree read and removed
//! where it stands (record 0027), and the toolchain's environment kept in
//! the place's git directory.

use crate::common::*;
use flume::profiles::place::git_worktree::{GitWorktree, Login, PlaceData, Step};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::place::{self, PlaceHarness};
use flume_seams::*;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// The login environment the profile's steps run with here: nothing of
/// the host user's, a home of the scratch's own.
fn login(scratch: &Scratch) -> Login {
    let path = std::env::var("PATH").unwrap_or_default();
    let home = scratch.path("home");
    std::fs::create_dir_all(&home).expect("a scratch home");
    Login::given(&[
        ("HOME", &home.to_string_lossy()),
        ("PATH", &path),
        ("LANG", "C"),
    ])
}

/// flume's own data directory on the scratch's host.
fn own(scratch: &Scratch) -> PathBuf {
    scratch.path("data/flume")
}

/// The profile bound with `data`, its steps run in the scratch's login
/// environment, and flume's own data directory the scratch's.
fn profile(data: PlaceData, scratch: &Scratch) -> GitWorktree {
    GitWorktree::with_login(data, login(scratch)).with_own(&own(scratch))
}

/// The profile bound with data a scenario changes as it goes, as the
/// resolver binds it anew for each command.
struct Bound {
    data: Mutex<PlaceData>,
    path: String,
    home: String,
    own: PathBuf,
}

impl Bound {
    fn new(scratch: &Scratch) -> Bound {
        let home = scratch.path("home");
        std::fs::create_dir_all(&home).expect("a scratch home");
        Bound {
            data: Mutex::default(),
            path: std::env::var("PATH").unwrap_or_default(),
            home: home.to_string_lossy().into_owned(),
            own: own(scratch),
        }
    }

    fn profile(&self) -> GitWorktree {
        GitWorktree::with_login(
            self.data.lock().unwrap().clone(),
            Login::given(&[("HOME", &self.home), ("PATH", &self.path), ("LANG", "C")]),
        )
        .with_own(&self.own)
    }
}

impl Place for Bound {
    fn location(&self, repository: &str, place: &str) -> Result<String, String> {
        self.profile().location(repository, place)
    }

    fn make(
        &self,
        repository: &str,
        revision: &Revision,
        branch: &str,
        location: &str,
    ) -> MakeOutcome {
        self.profile().make(repository, revision, branch, location)
    }

    fn ready(&self, place: &PlaceRef, revision: &Revision) -> Readiness {
        self.profile().ready(place, revision)
    }

    fn read(&self, place: &PlaceRef) -> PlaceRead {
        self.profile().read(place)
    }

    fn remove(&self, place: &PlaceRef) -> Removed {
        self.profile().remove(place)
    }

    fn remove_branch(&self, repository: &str, branch: &str, base: &str) -> BranchRemoved {
        self.profile().remove_branch(repository, branch, base)
    }
}

struct Harness {
    repository: Repository,
    seam: Bound,
}

impl Harness {
    fn new() -> Harness {
        let repository = Repository::new("place");
        let seam = Bound::new(&repository.scratch);
        Harness { repository, seam }
    }

    /// Where a teardown step's failure for a place is marked.
    fn failing(&self, location: &str, step: &str) -> String {
        let name = Path::new(location)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        self.repository
            .scratch
            .path(&format!("failing/{step}/{name}"))
            .to_string_lossy()
            .into_owned()
    }
}

impl PlaceHarness for Harness {
    type Seam = Bound;

    fn seam(&self) -> &Bound {
        &self.seam
    }

    fn repository(&self) -> (String, String, Revision) {
        (
            self.repository.name(),
            "main".into(),
            Revision::new(self.repository.head()),
        )
    }

    fn branch(&self, repository: &str, branch: &str) -> Revision {
        let dir = Path::new(repository);
        let commit = git(
            dir,
            &["commit-tree", "HEAD^{tree}", "-p", "HEAD", "-m", "by hand"],
        );
        git(dir, &["branch", branch, &commit]);
        Revision::new(commit)
    }

    fn commit(&self, place: &PlaceRef) -> Revision {
        let dir = Path::new(&place.location);
        std::fs::write(dir.join("work.rs"), "fn work() {}\n").expect("the work written");
        git(dir, &["add", "work.rs"]);
        git(dir, &["commit", "--quiet", "-m", "the work"]);
        Revision::new(git(dir, &["rev-parse", "HEAD"]))
    }

    fn head(&self, repository: &str, branch: &str) -> Option<Revision> {
        flume::git::Git::at(repository)
            .commit(&format!("refs/heads/{branch}"))
            .ok()
            .map(Revision::new)
    }

    fn occupy(&self, location: &str) {
        std::fs::create_dir_all(location).expect("the location occupied");
        std::fs::write(Path::new(location).join("by-hand.txt"), "here\n").expect("a file");
    }

    fn readiness(&self, _repository: &str, steps: &[(&str, bool)]) {
        self.seam.data.lock().unwrap().readiness = steps
            .iter()
            .map(|(name, passed)| {
                Step::run(
                    name,
                    if *passed {
                        "echo ready"
                    } else {
                        "echo not ready; exit 1"
                    },
                )
            })
            .collect();
    }

    fn untracked(&self, place: &PlaceRef, file: &str) {
        std::fs::write(Path::new(&place.location).join(file), "notes\n").expect("a file");
    }

    fn fail_teardown(&self, place: &PlaceRef, step: &str) {
        let marker = self.failing(&place.location, step);
        std::fs::create_dir_all(Path::new(&marker).parent().unwrap()).expect("the marks");
        std::fs::write(&marker, "").expect("the mark");
        let mut data = self.seam.data.lock().unwrap();
        if !data.teardown.iter().any(|held| held.name == step) {
            let marks = Path::new(&marker)
                .parent()
                .unwrap()
                .to_string_lossy()
                .into_owned();
            data.teardown.push(Step::run(
                step,
                &format!("test ! -e '{marks}'/\"$(basename \"$PWD\")\""),
            ));
        }
    }

    fn apply_to_base(&self, repository: &str, _base: &str, commit: &Revision) {
        let dir = Path::new(repository);
        git(
            dir,
            &[
                "commit",
                "--quiet",
                "--allow-empty",
                "-m",
                "the base moves on",
            ],
        );
        git(dir, &["cherry-pick", &commit.0]);
    }
}

#[test]
fn the_place_profile_passes_the_place_scenarios() {
    pass_all("place", &place::scenarios(), Harness::new);
}

/// A place made by the profile on `fix/<name>` at the base's head, where
/// the profile says.
fn made(seam: &dyn Place, repository: &Repository, name: &str) -> PlaceRef {
    let place = PlaceRef {
        repository: repository.name(),
        location: seam
            .location(&repository.name(), &format!("fix--{name}"))
            .expect("the place's location"),
        branch: format!("fix/{name}"),
        base: "main".into(),
    };
    let outcome = seam.make(
        &place.repository,
        &Revision::new(repository.head()),
        &place.branch,
        &place.location,
    );
    assert_eq!(outcome, MakeOutcome::Made, "make {name}");
    place
}

/// A commit in the place, of `file`.
fn work(place: &PlaceRef, file: &str) -> String {
    let dir = Path::new(&place.location);
    std::fs::write(dir.join(file), "work\n").expect("the work");
    git(dir, &["add", file]);
    git(dir, &["commit", "--quiet", "-m", file]);
    git(dir, &["rev-parse", "HEAD"])
}

#[test]
fn s10_an_untracked_file_and_a_failed_teardown_keep_their_places() {
    let repository = Repository::new("s10");
    let marks = repository.scratch.path("failing");
    std::fs::create_dir_all(&marks).unwrap();
    std::fs::write(marks.join("fix--teardown"), "").unwrap();
    let profile = profile(
        PlaceData {
            teardown: vec![Step::run(
                "pre-remove",
                &format!(
                    "test ! -e '{}'/\"$(basename \"$PWD\")\" || {{ echo the stack is still up >&2; exit 1; }}",
                    marks.display()
                ),
            )],
            ..PlaceData::default()
        },
        &repository.scratch,
    );
    let holding = made(&profile, &repository, "holding");
    std::fs::write(Path::new(&holding.location).join("notes.txt"), "mine\n").unwrap();
    let teardown = made(&profile, &repository, "teardown");

    match profile.remove(&holding) {
        Removed::Refused { words, .. } => {
            assert!(
                words.contains("notes.txt"),
                "the file is not named: {words}"
            )
        }
        other => panic!("a place holding an untracked file was {other:?}"),
    }
    match profile.remove(&teardown) {
        Removed::Refused { step, words } => {
            assert_eq!(step, "pre-remove");
            assert!(words.contains("the stack is still up"), "{words}");
        }
        other => panic!("a place whose teardown failed was {other:?}"),
    }
    for place in [&holding, &teardown] {
        assert!(
            matches!(profile.read(place), PlaceRead::Live { .. }),
            "{} was not kept",
            place.location
        );
    }
    assert!(Path::new(&holding.location).join("notes.txt").is_file());
}

#[test]
fn s9_a_branch_is_kept_while_it_holds_a_change_its_base_lacks() {
    let repository = Repository::new("s9");
    let profile = profile(PlaceData::default(), &repository.scratch);
    let dropped = made(&profile, &repository, "dropped");
    let lacking = work(&dropped, "dropped.rs");
    let picked = made(&profile, &repository, "picked");
    let taken = work(&picked, "picked.rs");
    for place in [&dropped, &picked] {
        assert_eq!(profile.remove(place), Removed::Removed);
    }
    repository.write("base.txt", "the base moves on\n");
    repository.commit("the base moves on");
    repository.git(&["cherry-pick", &taken]);
    assert_ne!(
        repository.head(),
        taken,
        "the base took the change by a commit of its own"
    );

    assert_eq!(
        profile.remove_branch(&repository.name(), "fix/dropped", "main"),
        BranchRemoved::Kept {
            commits: vec![Revision::new(lacking)]
        }
    );
    assert!(repository
        .git(&["branch", "--list", "fix/dropped"])
        .contains("fix/dropped"));
    assert_eq!(
        profile.remove_branch(&repository.name(), "fix/picked", "main"),
        BranchRemoved::Removed
    );
    assert_eq!(repository.git(&["branch", "--list", "fix/picked"]), "");
    assert_eq!(
        profile.remove_branch(&repository.name(), "fix/picked", "main"),
        BranchRemoved::Gone
    );
}

#[test]
fn a_branch_is_judged_against_the_base_where_it_lands() {
    let origin = Repository::new("landing");
    let remote = origin.bare_remote();
    let repository = Repository::clone_of(&remote, Scratch::new("landing-clone"));
    let local = profile(PlaceData::default(), &repository.scratch);
    let landing = profile(
        PlaceData {
            base_remote: Some("origin".into()),
            ..PlaceData::default()
        },
        &repository.scratch,
    );
    let place = made(&local, &repository, "landed");
    let commit = work(&place, "landed.rs");
    assert_eq!(local.remove(&place), Removed::Removed);
    repository.git(&["cherry-pick", &commit]);
    repository.git(&["push", "--quiet", "origin", "main"]);
    repository.git(&["reset", "--quiet", "--hard", "HEAD~1"]);
    repository.git(&["fetch", "--quiet", "origin"]);

    assert!(matches!(
        local.remove_branch(&repository.name(), "fix/landed", "main"),
        BranchRemoved::Kept { .. }
    ));
    assert_eq!(
        landing.remove_branch(&repository.name(), "fix/landed", "main"),
        BranchRemoved::Removed
    );
}

/// The names of the variables a step's `env` wrote, one per line.
fn names(env: &str) -> BTreeSet<String> {
    env.lines()
        .filter_map(|line| line.split_once('=').map(|(name, _)| name.to_string()))
        .filter(|name| {
            !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        })
        .collect()
}

#[test]
fn s16_readiness_that_fails_at_its_toolchain() {
    let repository = Repository::new("s16");
    let seen = repository.scratch.path("seen-env");
    let setup = repository.scratch.path("setup-ran");
    let profile = profile(
        PlaceData {
            readiness: vec![
                Step::toolchain(
                    "toolchain",
                    &format!(
                        "env > '{}'; echo the toolchain cannot be built >&2; exit 3",
                        seen.display()
                    ),
                ),
                Step::run("setup", &format!("touch '{}'", setup.display())),
            ],
            ..PlaceData::default()
        },
        &repository.scratch,
    );
    let place = made(&profile, &repository, "unready");
    let readiness = profile.ready(&place, &Revision::new(repository.head()));

    assert_eq!(readiness.steps.len(), 1, "{readiness:?}");
    let failed = readiness.failed().expect("a failed step");
    assert_eq!(failed.step, "toolchain");
    assert!(
        failed.words.contains("the toolchain cannot be built"),
        "{}",
        failed.words
    );
    assert!(!setup.exists(), "a step after the failed one ran");

    let env = std::fs::read_to_string(&seen).expect("the step's environment");
    let shells = ["PWD", "SHLVL", "OLDPWD", "_", "BASH_EXECUTION_STRING"];
    let given = ["HOME", "PATH", "LANG"];
    for name in names(&env) {
        assert!(
            given.contains(&name.as_str()) || shells.contains(&name.as_str()),
            "{name} of the caller's environment reached the step"
        );
    }
    assert!(
        std::env::var_os("GIT_CONFIG_GLOBAL").is_some() && !env.contains("GIT_CONFIG_GLOBAL"),
        "the caller's git environment reached the step"
    );
}

#[test]
fn the_toolchain_environment_is_kept_in_the_places_git_directory() {
    let repository = Repository::new("toolchain");
    let torn = repository.scratch.path("torn-down");
    let profile = profile(
        PlaceData {
            readiness: vec![
                Step::toolchain("toolchain", "echo 'export FLUME_TEST_TOOL=present'"),
                Step::run("setup", "test \"$FLUME_TEST_TOOL\" = present"),
            ],
            teardown: vec![Step::run(
                "release",
                &format!(
                    "test \"$FLUME_TEST_TOOL\" = present && touch '{}'",
                    torn.display()
                ),
            )],
            ..PlaceData::default()
        },
        &repository.scratch,
    );
    let place = made(&profile, &repository, "tooled");
    let readiness = profile.ready(&place, &Revision::new(repository.head()));
    assert!(readiness.failed().is_none(), "{readiness:?}");
    assert_eq!(readiness.steps.len(), 2);

    let git_dir = git(
        Path::new(&place.location),
        &["rev-parse", "--absolute-git-dir"],
    );
    let kept = Path::new(&git_dir).join("flume/toolchain.sh");
    assert!(
        kept.is_file(),
        "the toolchain's environment is not at {}",
        kept.display()
    );
    assert!(
        matches!(profile.read(&place), PlaceRead::Live { ref holds, .. } if !holds.any()),
        "the kept environment shows in the place"
    );
    assert_eq!(profile.remove(&place), Removed::Removed);
    assert!(torn.exists(), "the teardown ran outside the toolchain");
    assert!(
        !kept.exists(),
        "the toolchain's environment outlived its place"
    );
}

#[test]
fn with_no_merge_queue_bound_nothing_runs() {
    let repository = Repository::new("unbound");
    let profile = profile(PlaceData::default(), &repository.scratch);
    let place = made(&profile, &repository, "bare");
    let readiness = profile.ready(&place, &Revision::new(repository.head()));
    assert!(readiness.steps.is_empty(), "{readiness:?}");
    let git_dir = git(
        Path::new(&place.location),
        &["rev-parse", "--absolute-git-dir"],
    );
    assert!(!Path::new(&git_dir).join("flume").exists());
    assert_eq!(profile.remove(&place), Removed::Removed);
    assert_eq!(profile.read(&place), PlaceRead::Gone);
    assert_eq!(profile.remove(&place), Removed::Removed, "a second remove");
}

#[test]
fn make_runs_no_hook() {
    let repository = Repository::new("hooks");
    let ran = repository.scratch.path("hook-ran");
    let hook = repository.dir.join(".git/hooks/post-checkout");
    std::fs::write(&hook, format!("#!/bin/sh\ntouch '{}'\n", ran.display())).unwrap();
    std::process::Command::new("chmod")
        .args(["+x", &hook.to_string_lossy()])
        .status()
        .expect("chmod");
    let profile = profile(PlaceData::default(), &repository.scratch);
    made(&profile, &repository, "hooked");
    assert!(!ran.exists(), "a hook ran at make");
}

#[test]
fn a_bare_repository_beside_its_worktrees() {
    let origin = Repository::new("layout");
    let scratch = Scratch::new("layout-bare");
    let project = scratch.path("project");
    std::fs::create_dir_all(&project).unwrap();
    let bare = project.join("project.git");
    git(
        &scratch.root,
        &[
            "clone",
            "--quiet",
            "--bare",
            &origin.name(),
            &bare.to_string_lossy(),
        ],
    );
    git(
        &bare,
        &[
            "worktree",
            "add",
            "--quiet",
            &project.join("main").to_string_lossy(),
            "main",
        ],
    );
    let name = project.to_string_lossy().into_owned();
    let profile = profile(PlaceData::default(), &scratch);
    // The directory holds the bare repository beside its worktrees, so
    // its places go beside them, inside none (C.1; record 0027).
    let location = profile
        .location(&name, "fix--beside")
        .expect("the place's location");
    assert_eq!(location, format!("{name}/places/fix--beside"));
    let place = PlaceRef {
        repository: name.clone(),
        location,
        branch: "fix/beside".into(),
        base: "main".into(),
    };
    let head = Revision::new(origin.head());
    assert_eq!(
        profile.make(&name, &head, &place.branch, &place.location),
        MakeOutcome::Made
    );
    assert!(
        !own(&scratch).exists(),
        "flume's data directory was written"
    );
    assert_eq!(
        git(
            &project.join("main"),
            &["status", "--porcelain", "--untracked-files=all"]
        ),
        "",
        "the main worktree shows the place"
    );
    match profile.read(&place) {
        PlaceRead::Live {
            branch,
            head: read,
            holds,
        } => {
            assert_eq!(branch, "fix/beside");
            assert_eq!(read, head);
            assert!(!holds.any(), "{holds:?}");
        }
        other => panic!("read {other:?}"),
    }
    let main = PlaceRef {
        location: format!("{name}/main"),
        ..place.clone()
    };
    assert!(matches!(profile.read(&main), PlaceRead::Live { .. }));
    assert_eq!(profile.remove(&place), Removed::Removed);
    assert_eq!(profile.read(&place), PlaceRead::Gone);
    assert_eq!(
        profile.remove_branch(&name, "fix/beside", "main"),
        BranchRemoved::Removed
    );
}

#[test]
fn a_working_trees_places_go_under_flumes_own_data_directory() {
    // A clone's directory is its main worktree, so its places go under
    // flume's own data directory by the repository's id, and the clone's
    // own status shows none of them (3, C.1, C.4; record 0027).
    let repository = Repository::new("layout-clone");
    let profile = profile(PlaceData::default(), &repository.scratch);
    let common = repository.dir.join(".git");
    let id = flume_core::repository_id(&common.to_string_lossy());
    let place = made(&profile, &repository, "clone");
    assert_eq!(
        Path::new(&place.location),
        own(&repository.scratch)
            .join("places")
            .join(&id)
            .join("fix--clone")
    );
    assert!(!Path::new(&place.location).starts_with(&repository.dir));
    let inside = repository.dir.join("inside");
    std::fs::create_dir_all(&inside).unwrap();
    assert_eq!(
        profile.location(&inside.to_string_lossy(), "fix--clone"),
        Ok(place.location.clone()),
        "the repository named from a directory inside its worktree"
    );
    assert!(matches!(profile.read(&place), PlaceRead::Live { .. }));
    assert_eq!(
        repository.git(&["status", "--porcelain", "--untracked-files=all"]),
        "",
        "the main worktree shows the place"
    );

    let other = Repository::new("layout-other");
    assert_ne!(
        profile.location(&other.name(), "fix--clone"),
        Ok(place.location.clone()),
        "two repositories' places of one name share a location"
    );

    assert_eq!(profile.remove(&place), Removed::Removed);
    assert_eq!(profile.read(&place), PlaceRead::Gone);
    assert_eq!(
        profile.remove_branch(&repository.name(), "fix/clone", "main"),
        BranchRemoved::Removed
    );
}

#[test]
fn a_place_made_inside_the_main_worktree_is_read_and_removed_where_it_stands() {
    // A place an earlier flume made at `<repository>/places/<kind>--<name>`
    // of a clone is read and removed at that location (record 0027).
    let repository = Repository::new("made-inside");
    let profile = profile(PlaceData::default(), &repository.scratch);
    let place = PlaceRef {
        repository: repository.name(),
        location: format!("{}/places/fix--inside", repository.name()),
        branch: "fix/inside".into(),
        base: "main".into(),
    };
    assert_ne!(
        profile.location(&place.repository, "fix--inside"),
        Ok(place.location.clone())
    );
    repository.git(&[
        "worktree",
        "add",
        "--quiet",
        "-b",
        &place.branch,
        &place.location,
        "main",
    ]);
    let commit = work(&place, "inside.rs");
    match profile.read(&place) {
        PlaceRead::Live { branch, head, .. } => {
            assert_eq!(branch, "fix/inside");
            assert_eq!(head, Revision::new(commit));
        }
        other => panic!("read {other:?}"),
    }
    assert_eq!(profile.remove(&place), Removed::Removed);
    assert_eq!(profile.read(&place), PlaceRead::Gone);
    assert!(!Path::new(&place.location).exists());
    assert!(matches!(
        profile.remove_branch(&place.repository, &place.branch, "main"),
        BranchRemoved::Kept { .. }
    ));
}

#[test]
fn the_main_worktree_is_never_removed() {
    let repository = Repository::new("main-worktree");
    let profile = profile(PlaceData::default(), &repository.scratch);
    let main = PlaceRef {
        repository: repository.name(),
        location: repository.name(),
        branch: "main".into(),
        base: "main".into(),
    };
    assert!(matches!(profile.remove(&main), Removed::Refused { .. }));
    assert!(repository.dir.join("README").is_file());
}

//! The place profile over plain git worktree (C.1; design 9): the place
//! seam's scenarios against it on scratch repositories, S9, S10, S16, a
//! repository that binds no merge queue, a make that runs no hook, every
//! place under flume's own data directory, for the bare repository beside
//! its worktrees as for a clone, a place an earlier flume made inside a
//! clone's main worktree or beside a bare repository's worktrees read and
//! removed where it stands (record 0027), a place readied at the revision
//! its launch read through the resolver over a stand-in for sluice,
//! nothing of a readiness kept in the place's git directory, nothing left
//! at a removed place's location or in its empty group folder whatever
//! git could not delete or a process wrote there, what a failed removal
//! left taken at the next remove (record 0057 and its amendment), and a
//! base place launched and tidied through the core with only its location
//! asked of the profile. No sluice runs here: the pinned one equips a
//! place in the tools layer.

use crate::common::engine::{base_at, PLACE_PROFILE};
use crate::common::*;
use flume::profiles::merge_queue::sluice::{equipped, EQUIP};
use flume::profiles::place::git_worktree::{GitWorktree, Login, PlaceData, Step};
use flume::profiles::{place_data, BuiltIn};
use flume_conformance::engine::world::{kind, launch, launched, World};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::place::{self, Pinned, PlaceHarness};
use flume_seams::*;
use std::collections::BTreeSet;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

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

/// A stand-in for sluice, first on the `PATH` it gives: it records each
/// command it is asked in `asked`, a line each, and answers `answer`,
/// exiting `exit`. The `PATH`, and where it records.
pub(crate) fn stand_in_sluice(scratch: &Scratch, answer: &str, exit: u8) -> (String, PathBuf) {
    let bin = scratch.path("stand-in/bin");
    std::fs::create_dir_all(&bin).expect("the stand-in's directory");
    let asked = scratch.path("stand-in/asked");
    let answers = scratch.path("stand-in/answer");
    std::fs::write(&answers, format!("{answer}\n")).expect("the stand-in's answer");
    let program = bin.join("sluice");
    std::fs::write(
        &program,
        format!(
            "#!/bin/sh\necho \"$*\" >> '{}'\ncat '{}'\nexit {exit}\n",
            asked.display(),
            answers.display()
        ),
    )
    .expect("the stand-in for sluice");
    std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755))
        .expect("the stand-in for sluice made executable");
    let path = format!(
        "{}:{}",
        bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    (path, asked)
}

/// What `sluice equip --json` answers with every step of `steps` run, each
/// with whether it passed.
pub(crate) fn equip_answer(steps: &[(&str, bool)]) -> String {
    let steps: Vec<serde_json::Value> = steps
        .iter()
        .map(|(step, passed)| {
            serde_json::json!({"step": step, "passed": passed, "words": format!("{step} said so")})
        })
        .collect();
    serde_json::json!({"branch": "main", "revision": "0000000", "steps": steps}).to_string()
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

    /// None here, where no sluice runs: whether to install is sluice's,
    /// and the tools layer holds it under the pinned sluice (S89).
    fn pinned(&self, _repository: &str) -> Option<Pinned> {
        None
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
    let after = repository.scratch.path("after-ran");
    let answer = serde_json::json!({"steps": [
        {"step": "toolchain", "passed": false, "words": "the toolchain cannot be built"},
    ]});
    let profile = profile(
        PlaceData {
            readiness: vec![
                Step::read(
                    EQUIP,
                    &format!(
                        "env > '{}'; printf '%s\\n' '{answer}'; exit 1",
                        seen.display()
                    ),
                    equipped,
                ),
                Step::run("after", &format!("touch '{}'", after.display())),
            ],
            ..PlaceData::default()
        },
        &repository.scratch,
    );
    let place = made(&profile, &repository, "unready");
    let readiness = profile.ready(&place, &Revision::new(repository.head()));

    assert_eq!(
        steps(&readiness),
        vec![("toolchain", false)],
        "{readiness:?}"
    );
    let failed = readiness.failed().expect("a failed step");
    assert_eq!(failed.words, "the toolchain cannot be built");
    assert!(!after.exists(), "a step after the failed one ran");

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

/// The steps a readiness ran, each with whether it passed.
fn steps(readiness: &Readiness) -> Vec<(&str, bool)> {
    readiness
        .steps
        .iter()
        .map(|step| (step.step.as_str(), step.passed))
        .collect()
}

/// The resolver, a place's steps run in a scratch home with `path` for
/// their `PATH`.
fn resolver(scratch: &Scratch, path: &str) -> BuiltIn {
    let home = scratch.path("home");
    std::fs::create_dir_all(&home).expect("a scratch home");
    BuiltIn::with_login(&[
        ("HOME", &home.to_string_lossy()),
        ("PATH", path),
        ("LANG", "C"),
    ])
}

/// A binding naming `merge_queue`, the place left to git worktree.
fn binding(merge_queue: Option<&str>) -> Binding {
    Binding {
        merge_queue: merge_queue.map(str::to_string),
        ..Binding::default()
    }
}

#[test]
fn a_place_is_readied_at_the_revision_its_launch_read() {
    let repository = Repository::new("relaunched");
    repository.write(
        ".config/sluice.toml",
        "[tests]\nrunner = \"command\"\ncommand = \"true\"\n\n[branches.main]\n",
    );
    let launched = Revision::new(repository.commit("the gate"));
    let (path, asked) = stand_in_sluice(
        &repository.scratch,
        &equip_answer(&[("toolchain", true)]),
        0,
    );
    let bound = resolver(&repository.scratch, &path)
        .place(&repository.name(), &binding(Some("sluice")))
        .expect("the place bound with sluice's data");
    let place = made(bound.as_ref(), &repository, "relaunched");
    let at_launch = bound.ready(&place, &launched);
    repository.write("moved.txt", "main moves on\n");
    let moved = repository.commit("main moves on");
    let at_retry = bound.ready(&place, &launched);

    for readiness in [&at_launch, &at_retry] {
        assert_eq!(steps(readiness), vec![("toolchain", true)], "{readiness:?}");
        assert_eq!(readiness.steps[0].words, "toolchain said so");
    }
    let asked = std::fs::read_to_string(&asked).expect("what sluice was asked");
    let equip = format!("equip main --at {} --json", launched.0);
    assert_eq!(asked.lines().collect::<Vec<_>>(), vec![equip.as_str(); 2]);
    assert!(!asked.contains(&moved), "readied at main as it moved on");
}

#[test]
fn nothing_is_kept_in_the_places_git_directory() {
    let repository = Repository::new("unkept");
    let launched = Revision::new(repository.head());
    let (path, asked) = stand_in_sluice(
        &repository.scratch,
        &equip_answer(&[("toolchain", true), ("moon setup", true)]),
        0,
    );
    let data = place_data(&repository.name(), Some("sluice"), "main", Some(&launched))
        .expect("the place's data");
    let profile = GitWorktree::with_login(
        data,
        Login::given(&[
            ("HOME", &repository.scratch.path("home").to_string_lossy()),
            ("PATH", &path),
            ("LANG", "C"),
        ]),
    )
    .with_own(&own(&repository.scratch));
    let place = made(&profile, &repository, "unkept");
    let readiness = profile.ready(&place, &launched);
    assert_eq!(
        steps(&readiness),
        vec![("toolchain", true), ("moon setup", true)],
        "{readiness:?}"
    );

    let git_dir = git(
        Path::new(&place.location),
        &["rev-parse", "--absolute-git-dir"],
    );
    assert!(
        !Path::new(&git_dir).join("flume").exists(),
        "the readiness kept {}/flume",
        git_dir
    );
    assert!(
        matches!(profile.read(&place), PlaceRead::Live { ref holds, .. } if !holds.any()),
        "the readiness left something in the place"
    );
    assert_eq!(profile.remove(&place), Removed::Removed);
    let asked = std::fs::read_to_string(&asked).expect("what sluice was asked");
    assert_eq!(
        asked.lines().collect::<Vec<_>>(),
        vec![
            format!("equip main --at {} --json", launched.0),
            "release".to_string()
        ]
    );
}

#[test]
fn with_no_merge_queue_bound_nothing_runs() {
    let repository = Repository::new("unbound");
    let launched = Revision::new(repository.head());
    let data = place_data(&repository.name(), None, "main", Some(&launched)).expect("no data");
    let profile = profile(data, &repository.scratch);
    let place = made(&profile, &repository, "bare");
    let readiness = profile.ready(&place, &launched);
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

/// The directory `project` holding a bare clone of `origin`,
/// `project.git`, beside its worktree `main` (design 9).
fn bare_layout(origin: &Repository, scratch: &Scratch) -> PathBuf {
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
    project
}

#[test]
fn a_bare_repositorys_places_go_under_flumes_own_data_directory() {
    // The directory holds the bare repository beside its worktrees, and
    // its places go under flume's own data directory by the repository's
    // id, as a clone's do, none in the directory itself (3, C.1, C.4;
    // record 0027).
    let origin = Repository::new("layout");
    let scratch = Scratch::new("layout-bare");
    let project = bare_layout(&origin, &scratch);
    let name = project.to_string_lossy().into_owned();
    let profile = profile(PlaceData::default(), &scratch);
    let common = project.join("project.git").canonicalize().unwrap();
    let id = flume_core::repository_id(&common.to_string_lossy());
    let location = profile
        .location(&name, "fix--beside")
        .expect("the place's location");
    assert_eq!(
        Path::new(&location),
        own(&scratch).join("places").join(&id).join("fix--beside")
    );
    assert!(!Path::new(&location).starts_with(&project));
    assert_eq!(
        profile.location(&project.join("main").to_string_lossy(), "fix--beside"),
        Ok(location.clone()),
        "the repository named from its main worktree"
    );
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
        !project.join("places").exists(),
        "a place was made in the repository's directory"
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
fn a_place_made_beside_a_bare_repositorys_worktrees_is_read_and_removed_where_it_stands() {
    // A place an earlier flume made at `<repository>/places/<kind>--<name>`
    // beside a bare repository's worktrees is read and removed at that
    // location, never moved (record 0027).
    let origin = Repository::new("made-beside");
    let scratch = Scratch::new("made-beside-bare");
    let project = bare_layout(&origin, &scratch);
    let name = project.to_string_lossy().into_owned();
    let profile = profile(PlaceData::default(), &scratch);
    let place = PlaceRef {
        repository: name.clone(),
        location: format!("{name}/places/fix--beside"),
        branch: "fix/beside".into(),
        base: "main".into(),
    };
    assert_ne!(
        profile.location(&name, "fix--beside"),
        Ok(place.location.clone())
    );
    git(
        &project.join("project.git"),
        &[
            "worktree",
            "add",
            "--quiet",
            "-b",
            &place.branch,
            &place.location,
            "main",
        ],
    );
    let commit = work(&place, "beside.rs");
    match profile.read(&place) {
        PlaceRead::Live { branch, head, .. } => {
            assert_eq!(branch, "fix/beside");
            assert_eq!(head, Revision::new(commit));
        }
        other => panic!("read {other:?}"),
    }
    assert!(
        !own(&scratch).exists(),
        "the place was moved under flume's data directory"
    );
    assert_eq!(profile.remove(&place), Removed::Removed);
    assert_eq!(profile.read(&place), PlaceRead::Gone);
    assert!(!Path::new(&place.location).exists());
    assert!(matches!(
        profile.remove_branch(&name, &place.branch, "main"),
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

/// The group folder of a place's location, `places/<id>` under flume's
/// own data directory.
fn group(place: &PlaceRef) -> PathBuf {
    Path::new(&place.location)
        .parent()
        .expect("the place's group folder")
        .to_path_buf()
}

/// What a devenv shell entered in a place writes there, ignored by every
/// worktree of `repository` through its common git directory's exclude:
/// `.devenv/state/prek`, and `.moon/cache/outputs` a link to a folder
/// outside the place holding `archive`. The folder linked to.
fn devenv_writes(repository: &Repository, location: &Path) -> PathBuf {
    let exclude = repository.dir.join(".git/info/exclude");
    std::fs::create_dir_all(exclude.parent().unwrap()).unwrap();
    std::fs::write(&exclude, ".devenv/\n.moon/\n").unwrap();
    let outputs = repository.scratch.path("moon-outputs");
    std::fs::create_dir_all(&outputs).unwrap();
    std::fs::write(outputs.join("archive"), "shared\n").unwrap();
    std::fs::create_dir_all(location.join(".devenv/state/prek")).unwrap();
    std::fs::create_dir_all(location.join(".moon/cache")).unwrap();
    std::os::unix::fs::symlink(&outputs, location.join(".moon/cache/outputs")).unwrap();
    outputs
}

#[test]
fn what_git_left_of_a_failed_removal_goes_at_the_next_remove() {
    // `git worktree remove` drops its entry of a place and fails where it
    // cannot delete what the place holds, here a folder of ignored state
    // it may not write in: the remove is refused with git's words and
    // takes nothing more, and the next remove reads the place gone and
    // takes what stands and the empty group folder, running no teardown
    // again (62, 74, 76; record 0057's amendment).
    let repository = Repository::new("half-removed");
    let mark = repository.scratch.path("torn-down");
    let profile = profile(
        PlaceData {
            teardown: vec![Step::run(
                "pre-remove",
                &format!("echo torn down >> '{}'", mark.display()),
            )],
            ..PlaceData::default()
        },
        &repository.scratch,
    );
    let place = made(&profile, &repository, "half");
    let location = Path::new(&place.location);
    let outputs = devenv_writes(&repository, location);
    let stuck = location.join(".devenv/state/stuck");
    std::fs::create_dir_all(&stuck).unwrap();
    std::fs::write(stuck.join("state"), "kept\n").unwrap();
    std::fs::set_permissions(&stuck, std::fs::Permissions::from_mode(0o555)).unwrap();
    let torn_down = || {
        std::fs::read_to_string(&mark)
            .expect("the teardown's mark")
            .lines()
            .count()
    };

    match profile.remove(&place) {
        Removed::Refused { step, words } => {
            assert_eq!(step, "git worktree remove");
            assert!(words.contains("failed to delete"), "{words}");
            assert!(words.contains("no longer lists"), "{words}");
        }
        other => panic!("a removal git failed was {other:?}"),
    }
    assert!(location.exists(), "what git left went in the same call");
    assert_eq!(profile.read(&place), PlaceRead::Gone);
    assert_eq!(torn_down(), 1, "the teardown's runs");

    assert_eq!(profile.remove(&place), Removed::Removed, "the next remove");
    assert!(!location.exists(), "the place's folder still stands");
    assert!(
        !group(&place).exists(),
        "the empty group folder still stands"
    );
    assert!(own(&repository.scratch).join("places").is_dir());
    assert!(
        outputs.join("archive").is_file(),
        "a link was followed out of the place"
    );
    assert_eq!(torn_down(), 1, "the teardown ran again");
    assert_eq!(
        profile.remove(&place),
        Removed::Removed,
        "a remove repeated"
    );
}

#[test]
fn what_a_process_wrote_at_a_gone_places_location_goes() {
    // A process that wrote into a place as its worktree went, a devenv
    // shell's `.devenv` and `.moon` among it, leaves a folder git holds no
    // worktree for: the place reads gone, and its remove takes the folder
    // and the empty group folder, never what a link points at (62; record
    // 0057).
    let repository = Repository::new("gone-written");
    let profile = profile(PlaceData::default(), &repository.scratch);
    let place = made(&profile, &repository, "written");
    repository.git(&["worktree", "remove", &place.location]);
    let location = Path::new(&place.location);
    let outputs = devenv_writes(&repository, location);
    assert_eq!(profile.read(&place), PlaceRead::Gone);

    assert_eq!(profile.remove(&place), Removed::Removed);
    assert!(!location.exists(), "the place's folder still stands");
    assert!(
        !group(&place).exists(),
        "the empty group folder still stands"
    );
    assert!(
        outputs.join("archive").is_file(),
        "a link was followed out of the place"
    );
    assert_eq!(
        profile.remove(&place),
        Removed::Removed,
        "a remove repeated"
    );
}

#[test]
fn a_group_folder_holding_another_place_stays() {
    let repository = Repository::new("group-kept");
    let profile = profile(PlaceData::default(), &repository.scratch);
    let gone = made(&profile, &repository, "gone");
    let stays = made(&profile, &repository, "stays");
    assert_eq!(group(&gone), group(&stays));
    devenv_writes(&repository, Path::new(&gone.location));

    assert_eq!(profile.remove(&gone), Removed::Removed);
    assert!(!Path::new(&gone.location).exists());
    assert!(
        group(&stays).is_dir(),
        "the group folder of a live place went"
    );
    assert!(matches!(profile.read(&stays), PlaceRead::Live { .. }));
}

#[test]
fn a_worktree_another_repository_holds_at_a_places_location_is_kept() {
    // What stands at a gone place's location goes only where no git holds
    // a worktree there.
    let repository = Repository::new("held-elsewhere");
    let other = Repository::new("held-elsewhere-other");
    let profile = profile(PlaceData::default(), &repository.scratch);
    let location = profile
        .location(&repository.name(), "fix--held")
        .expect("the place's location");
    other.git(&["worktree", "add", "--quiet", "-b", "theirs", &location]);
    let place = PlaceRef {
        repository: repository.name(),
        location: location.clone(),
        branch: "fix/held".into(),
        base: "main".into(),
    };
    assert_eq!(profile.read(&place), PlaceRead::Gone);

    match profile.remove(&place) {
        Removed::Refused { words, .. } => {
            assert!(words.contains("git still holds"), "{words}")
        }
        other => panic!("another repository's worktree was {other:?}"),
    }
    assert!(Path::new(&location).join("README").is_file());
}

/// The worktree entry the `.git` file at `location` names removed by
/// hand, as a `git worktree remove` that failed leaves it.
fn drop_entry(location: &Path) {
    let text = std::fs::read_to_string(location.join(".git")).expect("the .git file");
    let gitdir = text
        .trim()
        .strip_prefix("gitdir:")
        .expect("a gitdir")
        .trim()
        .to_string();
    std::fs::remove_dir_all(location.join(gitdir)).expect("the worktree's entry removed");
}

/// The words a remove refused with, or a panic naming what it was.
fn refused(removed: Removed, what: &str) -> String {
    match removed {
        Removed::Refused { words, .. } => words,
        other => panic!("{what} was {other:?}"),
    }
}

#[test]
fn only_a_folder_flume_made_goes_from_a_places_location() {
    // What stands at a place's location git holds no worktree for goes
    // only where the location is in a folder flume makes places in and
    // nothing there says it is another's; the place's own folder, whose
    // worktree entry is gone, goes (62, 64; core.17, core.18; record
    // 0057's amendment).
    let stray = Repository::new("flumes-stray");
    let profile_of = |repository: &Repository| profile(PlaceData::default(), &repository.scratch);
    let elsewhere = stray.scratch.path("elsewhere/fix--stray");
    std::fs::create_dir_all(&elsewhere).unwrap();
    std::fs::write(elsewhere.join("kept"), "kept\n").unwrap();
    let place = PlaceRef {
        repository: stray.name(),
        location: elsewhere.to_string_lossy().into_owned(),
        branch: "fix/stray".into(),
        base: "main".into(),
    };
    let profile = profile_of(&stray);
    assert_eq!(profile.read(&place), PlaceRead::Gone);
    let words = refused(profile.remove(&place), "a folder outside every places/");
    assert!(words.contains(&place.location), "{words}");
    assert!(words.contains("no folder flume makes places in"), "{words}");
    assert!(elsewhere.join("kept").is_file(), "the stray folder went");

    let repository = Repository::new("flumes-theirs");
    let other = Repository::new("flumes-theirs-other");
    let profile = profile_of(&repository);
    let location = profile
        .location(&repository.name(), "fix--theirs")
        .expect("the place's location");
    other.git(&["worktree", "add", "--quiet", "-b", "theirs", &location]);
    drop_entry(Path::new(&location));
    let place = PlaceRef {
        repository: repository.name(),
        location: location.clone(),
        branch: "fix/theirs".into(),
        base: "main".into(),
    };
    assert_eq!(profile.read(&place), PlaceRead::Gone);
    let words = refused(
        profile.remove(&place),
        "another repository's dropped worktree",
    );
    assert!(words.contains("another repository"), "{words}");
    assert!(Path::new(&location).join("README").is_file());

    let repository = Repository::new("flumes-clone");
    let profile = profile_of(&repository);
    let location = profile
        .location(&repository.name(), "fix--clone")
        .expect("the place's location");
    std::fs::create_dir_all(&location).unwrap();
    git(Path::new(&location), &["init", "--quiet"]);
    std::fs::write(Path::new(&location).join("mine"), "mine\n").unwrap();
    let place = PlaceRef {
        repository: repository.name(),
        location: location.clone(),
        branch: "fix/clone".into(),
        base: "main".into(),
    };
    assert_eq!(profile.read(&place), PlaceRead::Gone);
    refused(profile.remove(&place), "a repository of its own");
    assert!(Path::new(&location).join("mine").is_file());

    let repository = Repository::new("flumes-own");
    let profile = profile_of(&repository);
    let place = made(&profile, &repository, "own");
    let location = Path::new(&place.location);
    drop_entry(location);
    std::fs::write(location.join("left"), "left\n").unwrap();
    assert_eq!(profile.read(&place), PlaceRead::Gone);
    assert_eq!(profile.remove(&place), Removed::Removed);
    assert!(
        location.symlink_metadata().is_err(),
        "the place's folder still stands"
    );
}

/// The profile, each call of the place seam named in `asked` before it
/// is passed on.
struct Asked {
    profile: GitWorktree,
    asked: Mutex<Vec<&'static str>>,
}

impl Asked {
    fn ask(&self, call: &'static str) -> &GitWorktree {
        self.asked.lock().unwrap().push(call);
        &self.profile
    }
}

impl Place for Asked {
    fn location(&self, repository: &str, place: &str) -> Result<String, String> {
        self.ask("location").location(repository, place)
    }

    fn make(
        &self,
        repository: &str,
        revision: &Revision,
        branch: &str,
        location: &str,
    ) -> MakeOutcome {
        self.ask("make")
            .make(repository, revision, branch, location)
    }

    fn ready(&self, place: &PlaceRef, revision: &Revision) -> Readiness {
        self.ask("ready").ready(place, revision)
    }

    fn read(&self, place: &PlaceRef) -> PlaceRead {
        self.ask("read").read(place)
    }

    fn remove(&self, place: &PlaceRef) -> Removed {
        self.ask("remove").remove(place)
    }

    fn remove_branch(&self, repository: &str, branch: &str, base: &str) -> BranchRemoved {
        self.ask("remove_branch")
            .remove_branch(repository, branch, base)
    }
}

#[test]
fn a_base_place_is_given_its_location_and_nothing_is_made_there() {
    // A base place launched and then tidied through the core over a
    // scratch repository whose fake merge queue opens and closes its
    // branch: the profile gives its location under flume's own data
    // directory as any place's, and is asked nothing else, so nothing
    // stands there, git lists no worktree for it, and no hook, readiness
    // or teardown runs (3, 66, 78, 202, C.1).
    let repository = Repository::new("base-place");
    let name = repository.name();
    let marks = repository.scratch.path("ran");
    std::fs::create_dir_all(&marks).expect("the marks' folder");
    let mark = |step: &str| marks.join(step);
    let hook = repository.dir.join(".git/hooks/post-checkout");
    std::fs::write(
        &hook,
        format!("#!/bin/sh\ntouch '{}'\n", mark("hook").display()),
    )
    .expect("the hook");
    std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755))
        .expect("the hook made executable");
    let data = PlaceData {
        readiness: vec![Step::run(
            "ready",
            &format!("touch '{}'", mark("ready").display()),
        )],
        teardown: vec![Step::run(
            "teardown",
            &format!("touch '{}'", mark("teardown").display()),
        )],
        base_remote: None,
    };
    let seam = Arc::new(Asked {
        profile: profile(data, &repository.scratch),
        asked: Mutex::default(),
    });
    let world = World::new();
    base_at(&world, &name, &repository.head());
    world.kinds.set_binding(
        &name,
        Binding {
            place: Some(PLACE_PROFILE.into()),
            change: None,
            merge_queue: Some("queue".into()),
        },
    );
    world
        .kinds
        .set_kinds(&name, vec![kind("train", true, vec![])]);
    world.resolver.bind_place(PLACE_PROFILE, seam.clone());

    let mut train = launch("train", "spring", None);
    train.repository = name.clone();
    let status = launched(&world.engine().launch(&train)).expect("the base place launched");
    let location = status.place.0.clone();
    let id = flume_core::repository_id(&repository.dir.join(".git").to_string_lossy());
    assert_eq!(
        Path::new(&location),
        own(&repository.scratch)
            .join("places")
            .join(&id)
            .join("train--spring"),
        "the base place's location"
    );
    assert_eq!(
        world.queue.log.of("open"),
        vec!["open train/spring from main".to_string()],
        "the merge queue's opens"
    );
    let stands = |when: &str| {
        assert!(
            !Path::new(&location).exists(),
            "something stands at {location} {when}"
        );
        let listed = repository.git(&["worktree", "list", "--porcelain"]);
        assert_eq!(
            listed
                .lines()
                .filter(|line| line.starts_with("worktree "))
                .count(),
            1,
            "a worktree beside the main one {when}: {listed}"
        );
    };
    stands("once launched");
    world.engine().settle(None);
    stands("once settled");

    match world.engine().drop_place(&status.place, false) {
        flume_core::Answer::Acted {
            tidy: Some(tidy), ..
        } => assert_eq!(tidy.removed, vec![status.place.clone()], "the tidy"),
        other => panic!("the drop answered {other:?}"),
    }
    world.engine().settle(None);
    assert!(world.held(&status.place).is_none(), "the held state stays");
    assert_eq!(
        world.queue.log.of("close"),
        vec!["close train/spring into main".to_string()],
        "the merge queue's closes"
    );
    stands("once tidied");
    let asked = seam.asked.lock().unwrap().clone();
    assert!(
        !asked.is_empty() && asked.iter().all(|call| *call == "location"),
        "the profile was asked {asked:?}"
    );
    for step in ["hook", "ready", "teardown"] {
        assert!(!mark(step).exists(), "the {step} ran");
    }
}

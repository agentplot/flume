//! The place profile over plain `git worktree` (C.1; design 9). A place
//! is a worktree of the repository on a branch of its own, made with no
//! hook run (78), read by `git status` and the worktree's lock (80),
//! removed by the teardown it was handed and `git worktree remove` with no
//! force (81, core.18), and its branch removed only where `git cherry`
//! against the base names no commit the base lacks (82, 67). The
//! repository is the directory git finds one in, or the one holding a
//! bare repository beside its worktrees (`project/project.git`,
//! `project/main`).
//!
//! A place is made inside no working tree or directory of its repository
//! (record 0027): every place is under flume's own data directory, at
//! `places/<id>/<kind>--<name>`, a bare repository's as a clone's, the id
//! its common git directory's as its agents' identities name it (C.4). A
//! place is read and removed at the location it was made at, wherever
//! that is, one an earlier flume made at `<repository>/places/` among
//! them.
//!
//! The readiness, the teardown and where the base lives are the merge
//! queue profile's to declare; the binary's resolver hands them to this
//! profile as [`PlaceData`] when it binds a repository, the readiness
//! asked at the revision the place's launch read, and neither profile
//! calls the other (core.2). With no merge queue bound the data is empty
//! and nothing runs (11, 66, 79, C.1). Every step runs in the place under
//! `bash -c` with an environment cleared to the host user's login
//! environment, nothing of the caller's session (11, 79). A step handed
//! with a reading of what it printed, as the merge queue's equipping of
//! the place is, gives the readiness the steps that reading gives, the
//! merge queue's own (12, C.1); nothing of the readiness is kept in the
//! place, its git directory among it (C.7).

use crate::git::Git;
use crate::shell::tail;
use flume_seams::{
    BranchRemoved, Holds, MakeOutcome, Place, PlaceRead, PlaceRef, Readiness, ReadyStep, Removed,
    Revision,
};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

/// A reading of what a step printed, whether it passed, on its output and
/// on its errors, as the readiness's steps, each with its outcome and
/// words: the merge queue profile's own, handed as data, so this profile
/// reads nothing of the merge queue's form (core.2).
pub type Reading = fn(bool, &str, &str) -> Vec<ReadyStep>;

/// One step the merge queue's profile declares for a place: its name, the
/// shell line it runs in the place, and the reading of what it printed
/// where the profile gives one.
#[derive(Clone, Debug)]
pub struct Step {
    pub name: String,
    pub command: String,
    /// The readiness's steps of what the step printed; the step itself,
    /// passed or failed with its words, where none.
    pub reading: Option<Reading>,
}

impl Step {
    /// A step that runs `command`.
    pub fn run(name: &str, command: &str) -> Step {
        Step {
            name: name.into(),
            command: command.into(),
            reading: None,
        }
    }

    /// A step that runs `command`, the steps it gives read from what it
    /// printed by `reading`.
    pub fn read(name: &str, command: &str, reading: Reading) -> Step {
        Step {
            reading: Some(reading),
            ..Step::run(name, command)
        }
    }
}

/// What the merge queue's profile declares for the places of one
/// repository, handed to this profile by the binary's resolver (design
/// 9, 13): none of it where no merge queue is bound.
#[derive(Clone, Debug, Default)]
pub struct PlaceData {
    /// The steps that ready a place, in order (79).
    pub readiness: Vec<Step>,
    /// The steps that tear a place down before its worktree is removed
    /// (66, 81).
    pub teardown: Vec<Step>,
    /// The remote the base lands on, whose tracking branch a branch's
    /// commits are judged against; the local base where none (82, C.1).
    pub base_remote: Option<String>,
}

/// The variables of the host user's login environment a step is given
/// beside the login shell's `PATH` (11, 79).
const LOGIN_NAMES: [&str; 6] = ["HOME", "USER", "LOGNAME", "SHELL", "TMPDIR", "LANG"];

/// The login environment a step runs with: the host user's, read once
/// when a step first runs, or one given.
#[derive(Debug)]
pub struct Login {
    given: OnceLock<Vec<(String, String)>>,
}

impl Login {
    /// The host user's login environment: `HOME`, `USER`, `LOGNAME`,
    /// `SHELL`, `TMPDIR` and `LANG` as they stand, and the `PATH` the
    /// user's login shell sets, run with nothing else.
    pub fn of_host() -> Login {
        Login {
            given: OnceLock::new(),
        }
    }

    /// Exactly `variables`, and nothing else.
    pub fn given(variables: &[(&str, &str)]) -> Login {
        let given = OnceLock::new();
        let _ = given.set(
            variables
                .iter()
                .map(|(name, value)| (name.to_string(), value.to_string()))
                .collect(),
        );
        Login { given }
    }

    fn variables(&self) -> &[(String, String)] {
        self.given.get_or_init(|| {
            let mut variables: Vec<(String, String)> = LOGIN_NAMES
                .iter()
                .filter_map(|name| {
                    std::env::var(name)
                        .ok()
                        .map(|value| (name.to_string(), value))
                })
                .collect();
            let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into());
            let path = Command::new(&shell)
                .args(["-l", "-c", "printf %s \"$PATH\""])
                .env_clear()
                .envs(variables.iter().map(|(name, value)| (name, value)))
                .output()
                .ok()
                .filter(|output| output.status.success())
                .and_then(|output| String::from_utf8(output.stdout).ok())
                .filter(|path| !path.trim().is_empty())
                .unwrap_or_else(|| "/usr/bin:/bin:/usr/sbin:/sbin".into());
            variables.push(("PATH".into(), path.trim().to_string()));
            variables
        })
    }
}

/// The place profile over plain `git worktree`, bound for one repository
/// with the merge queue's data.
#[derive(Debug)]
pub struct GitWorktree {
    data: PlaceData,
    login: Login,
    /// flume's own data directory, `$XDG_DATA_HOME/flume`, where it could
    /// be found.
    own: Option<PathBuf>,
}

/// The name a place profile binding gives this profile (C.1).
pub const NAME: &str = "git-worktree";

/// The step named where `git worktree remove` refuses or what the place
/// holds keeps it (81, 65).
const REMOVE_STEP: &str = "git worktree remove";

/// Where the place `place`, `<kind>--<name>`, of `repository` is made
/// (3, C.1; record 0027): under `own`, flume's own data directory, at
/// `places/<id>/<place>`, the id the repository's, of its common git
/// directory (C.4), whether the repository's directory is its main
/// worktree or holds its bare repository beside its worktrees.
pub fn location(repository: &str, place: &str, own: Option<&Path>) -> Result<String, String> {
    let own = own.ok_or_else(|| {
        "every place goes under flume's data directory, and neither $XDG_DATA_HOME nor $HOME is set"
            .to_string()
    })?;
    let common = Git::at(repository)
        .common_dir()
        .unwrap_or_else(|_| repository.to_string());
    Ok(canonical(own)
        .join("places")
        .join(flume_core::repository_id(&common))
        .join(place)
        .to_string_lossy()
        .into_owned())
}

impl GitWorktree {
    /// The profile bound with `data`, its steps run in the host user's
    /// login environment.
    pub fn new(data: PlaceData) -> GitWorktree {
        GitWorktree::with_login(data, Login::of_host())
    }

    /// The profile bound with `data`, its steps run in `login`.
    pub fn with_login(data: PlaceData, login: Login) -> GitWorktree {
        GitWorktree {
            data,
            login,
            own: crate::host::config::flume_data().ok(),
        }
    }

    /// The profile with `own` for flume's own data directory.
    pub fn with_own(self, own: &Path) -> GitWorktree {
        GitWorktree {
            own: Some(own.into()),
            ..self
        }
    }

    /// The worktree entry of `location` in `repository`'s list, if git
    /// holds one: whether its directory is there, and its lock's words.
    fn entry(&self, repository: &str, location: &str) -> Result<Option<Entry>, String> {
        let listed = Git::at(repository).text(&["worktree", "list", "--porcelain"])?;
        let wanted = canonical(Path::new(location));
        let mut found = None;
        for block in listed.split("\n\n") {
            let mut entry: Option<Entry> = None;
            for line in block.lines() {
                if let Some(path) = line.strip_prefix("worktree ") {
                    entry = Some(Entry {
                        path: PathBuf::from(path),
                        lock: None,
                    });
                } else if let (Some(entry), Some(lock)) = (entry.as_mut(), locked(line)) {
                    entry.lock = Some(lock);
                }
            }
            if let Some(entry) = entry.filter(|entry| canonical(&entry.path) == wanted) {
                found = Some(entry);
            }
        }
        Ok(found)
    }

    /// Runs `step` in the place: whether it passed, what it printed on its
    /// output and on its errors, or why it could not run.
    fn run(&self, location: &str, step: &Step) -> Result<(bool, String, String), String> {
        let output = Command::new("bash")
            .arg("-c")
            .arg(&step.command)
            .current_dir(location)
            .env_clear()
            .envs(
                self.login
                    .variables()
                    .iter()
                    .map(|(name, value)| (name, value)),
            )
            .output();
        let output = output.map_err(|error| format!("{} could not run: {error}", step.name))?;
        Ok((
            output.status.success(),
            String::from_utf8_lossy(&output.stdout).into_owned(),
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }

    /// Runs `step` in the place: the steps it gives, read from what it
    /// printed where it has a reading, else itself, passed or failed, with
    /// its words.
    fn steps(&self, location: &str, step: &Step) -> Vec<ReadyStep> {
        let itself = |passed, words: &str| ReadyStep {
            step: step.name.clone(),
            passed,
            words: tail(words),
        };
        match (self.run(location, step), step.reading) {
            (Err(why), _) => vec![itself(false, &why)],
            (Ok((passed, stdout, stderr)), Some(reading)) => reading(passed, &stdout, &stderr),
            (Ok((passed, stdout, stderr)), None) => {
                vec![itself(passed, &format!("{stdout}{stderr}"))]
            }
        }
    }
}

/// A worktree as `git worktree list` gives it.
struct Entry {
    path: PathBuf,
    lock: Option<String>,
}

/// A `locked` line's words, `locked` alone where it gives none.
fn locked(line: &str) -> Option<String> {
    if line == "locked" {
        return Some("locked".into());
    }
    line.strip_prefix("locked ").map(str::to_string)
}

/// `path` as the file system resolves it, its parent where it does not
/// exist yet.
fn canonical(path: &Path) -> PathBuf {
    if let Ok(path) = path.canonicalize() {
        return path;
    }
    match (path.parent(), path.file_name()) {
        (Some(parent), Some(name)) => canonical(parent).join(name),
        _ => path.to_path_buf(),
    }
}

/// What `git status --porcelain=v2 -z` says of a place: its branch, its
/// head and what it holds (80, 65).
fn status(text: &str) -> (Option<String>, Option<String>, Holds) {
    let mut branch = None;
    let mut head = None;
    let mut holds = Holds::default();
    let mut fields = text.split('\0');
    while let Some(field) = fields.next() {
        if let Some(oid) = field.strip_prefix("# branch.oid ") {
            head = Some(oid.to_string());
        } else if let Some(name) = field.strip_prefix("# branch.head ") {
            branch = Some(name.to_string());
        } else if let Some(path) = field.strip_prefix("? ") {
            holds.untracked.push(path.to_string());
        } else if field.starts_with("1 ") {
            holds
                .uncommitted
                .extend(field.splitn(9, ' ').nth(8).map(str::to_string));
        } else if field.starts_with("2 ") {
            holds
                .uncommitted
                .extend(field.splitn(10, ' ').nth(9).map(str::to_string));
            fields.next();
        } else if field.starts_with("u ") {
            holds
                .uncommitted
                .extend(field.splitn(11, ' ').nth(10).map(str::to_string));
        }
    }
    (branch, head, holds)
}

impl Place for GitWorktree {
    fn location(&self, repository: &str, place: &str) -> Result<String, String> {
        location(repository, place, self.own.as_deref())
    }

    fn make(
        &self,
        repository: &str,
        revision: &Revision,
        branch: &str,
        location: &str,
    ) -> MakeOutcome {
        let git = Git::at(repository);
        let reference = format!("refs/heads/{branch}");
        if git.commit(&reference).is_ok() {
            return MakeOutcome::BranchExists;
        }
        if Path::new(location).symlink_metadata().is_ok() {
            return MakeOutcome::LocationExists;
        }
        let made = git.text(&[
            "-c",
            "core.hooksPath=/dev/null",
            "worktree",
            "add",
            "--quiet",
            "-b",
            branch,
            location,
            &revision.0,
        ]);
        match made {
            Ok(_) => MakeOutcome::Made,
            Err(reason) => {
                // All or nothing (83): what the refused add left is undone.
                if Path::new(location).exists() {
                    let _ = std::fs::remove_dir_all(location);
                }
                let _ = git.text(&["worktree", "prune"]);
                if git.commit(&reference).is_ok() {
                    let _ = git.text(&["branch", "-D", branch]);
                }
                MakeOutcome::Refused { reason }
            }
        }
    }

    /// The steps the resolver handed, built for `revision`, the revision
    /// the place's launch read (12, 20).
    fn ready(&self, place: &PlaceRef, _revision: &Revision) -> Readiness {
        let mut readiness = Readiness::default();
        for step in &self.data.readiness {
            readiness.steps.extend(self.steps(&place.location, step));
            if readiness.failed().is_some() {
                break;
            }
        }
        readiness
    }

    fn read(&self, place: &PlaceRef) -> PlaceRead {
        let entry = match self.entry(&place.repository, &place.location) {
            Ok(entry) => entry,
            Err(reason) => return PlaceRead::CouldNotRead { reason },
        };
        let Some(entry) = entry.filter(|entry| entry.path.is_dir()) else {
            return PlaceRead::Gone;
        };
        let text = match Git::at(&place.location).text(&[
            "status",
            "--porcelain=v2",
            "--branch",
            "--untracked-files=all",
            "-z",
        ]) {
            Ok(text) => text,
            Err(reason) => return PlaceRead::CouldNotRead { reason },
        };
        let (branch, head, mut holds) = status(&text);
        holds.lock = entry.lock;
        match (branch, head) {
            (Some(branch), Some(head)) => PlaceRead::Live {
                branch,
                head: Revision::new(head),
                holds,
            },
            _ => PlaceRead::CouldNotRead {
                reason: format!("git status in {} named no branch or head", place.location),
            },
        }
    }

    fn remove(&self, place: &PlaceRef) -> Removed {
        let refused = |words: String| Removed::Refused {
            step: REMOVE_STEP.into(),
            words,
        };
        if Path::new(&place.location).join(".git").is_dir() {
            return refused(format!(
                "{} is the repository's main worktree, which flume never removes",
                place.location
            ));
        }
        match self.read(place) {
            PlaceRead::Gone => {
                // A worktree whose directory is gone leaves git's entry of
                // it, which goes too.
                let _ = Git::at(&place.repository).text(&["worktree", "prune"]);
                return Removed::Removed;
            }
            PlaceRead::CouldNotRead { reason } => return refused(reason),
            PlaceRead::Live { holds, .. } if holds.any() => {
                let mut words = vec![];
                if !holds.uncommitted.is_empty() {
                    words.push(format!("uncommitted: {}", holds.uncommitted.join(", ")));
                }
                if !holds.untracked.is_empty() {
                    words.push(format!(
                        "neither committed nor ignored: {}",
                        holds.untracked.join(", ")
                    ));
                }
                if let Some(lock) = holds.lock {
                    words.push(format!("locked: {lock}"));
                }
                return refused(format!(
                    "{} holds work: {}",
                    place.location,
                    words.join("; ")
                ));
            }
            PlaceRead::Live { .. } => {}
        }
        for step in &self.data.teardown {
            if let Some(failed) = self
                .steps(&place.location, step)
                .into_iter()
                .find(|step| !step.passed)
            {
                return Removed::Refused {
                    step: failed.step,
                    words: failed.words,
                };
            }
        }
        match Git::at(&place.repository).text(&["worktree", "remove", &place.location]) {
            Ok(_) => Removed::Removed,
            Err(words) => refused(words),
        }
    }

    fn remove_branch(&self, repository: &str, branch: &str, base: &str) -> BranchRemoved {
        let git = Git::at(repository);
        if git.commit(&format!("refs/heads/{branch}")).is_err() {
            return BranchRemoved::Gone;
        }
        let base = match &self.data.base_remote {
            Some(remote) => format!("refs/remotes/{remote}/{base}"),
            None => format!("refs/heads/{base}"),
        };
        let cherry = match git.text(&["cherry", &base, &format!("refs/heads/{branch}")]) {
            Ok(cherry) => cherry,
            // A base that cannot be read judges nothing: the branch is kept
            // (67).
            Err(_) => return BranchRemoved::Kept { commits: vec![] },
        };
        let commits: Vec<Revision> = cherry
            .lines()
            .filter_map(|line| line.strip_prefix("+ "))
            .map(|commit| Revision::new(commit.trim()))
            .collect();
        if !commits.is_empty() {
            return BranchRemoved::Kept { commits };
        }
        match git.text(&["branch", "-D", branch]) {
            Ok(_) => BranchRemoved::Removed,
            Err(_) => BranchRemoved::Kept { commits: vec![] },
        }
    }
}

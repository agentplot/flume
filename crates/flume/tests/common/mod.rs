//! Scratch repositories for the profiles' tests. Every git call a test
//! makes, and every one the profile under test makes from this process,
//! runs isolated: no system or global configuration, no credential
//! helper, no prompt, and no repository but the scratch one named
//! (core.22).

#![allow(dead_code)]

pub mod engine;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Once;

static ISOLATED: Once = Once::new();
static COUNT: AtomicUsize = AtomicUsize::new(0);

/// Sets this test process's git environment once, before any git runs:
/// the profiles under test inherit it.
pub fn isolate() {
    ISOLATED.call_once(|| {
        for name in [
            "GIT_DIR",
            "GIT_WORK_TREE",
            "GIT_INDEX_FILE",
            "GIT_COMMON_DIR",
            "GIT_OBJECT_DIRECTORY",
            "GIT_ALTERNATE_OBJECT_DIRECTORIES",
            "GIT_CEILING_DIRECTORIES",
        ] {
            std::env::remove_var(name);
        }
        for (name, value) in [
            ("GIT_CONFIG_NOSYSTEM", "1"),
            ("GIT_CONFIG_GLOBAL", "/dev/null"),
            ("GIT_TERMINAL_PROMPT", "0"),
            ("GIT_ASKPASS", "/usr/bin/false"),
            ("SSH_ASKPASS", "/usr/bin/false"),
            ("GCM_INTERACTIVE", "never"),
            ("GIT_CONFIG_COUNT", "1"),
            ("GIT_CONFIG_KEY_0", "credential.helper"),
            ("GIT_CONFIG_VALUE_0", ""),
            ("GIT_AUTHOR_NAME", "flume tests"),
            ("GIT_AUTHOR_EMAIL", "tests@flume.invalid"),
            ("GIT_COMMITTER_NAME", "flume tests"),
            ("GIT_COMMITTER_EMAIL", "tests@flume.invalid"),
        ] {
            std::env::set_var(name, value);
        }
    });
}

/// A directory of its own under the system's temporary directory,
/// removed when dropped.
pub struct Scratch {
    pub root: PathBuf,
}

impl Scratch {
    pub fn new(label: &str) -> Scratch {
        isolate();
        let root = std::env::temp_dir().join(format!(
            "flume-test-{label}-{}-{}",
            std::process::id(),
            COUNT.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("a scratch directory");
        let root = root.canonicalize().expect("the scratch directory's path");
        Scratch { root }
    }

    pub fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

/// Runs git in `dir`, isolated, and gives its output; panics with git's
/// words where it fails.
pub fn git(dir: &Path, args: &[&str]) -> String {
    isolate();
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .expect("git runs");
    assert!(
        output.status.success(),
        "git {} in {}: {}",
        args.join(" "),
        dir.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// A scratch repository with a worktree, on `main`.
pub struct Repository {
    pub scratch: Scratch,
    pub dir: PathBuf,
}

impl Repository {
    /// A repository on `main` holding one commit.
    pub fn new(label: &str) -> Repository {
        let scratch = Scratch::new(label);
        let dir = scratch.path("repository");
        std::fs::create_dir_all(&dir).expect("the repository's directory");
        git(&dir, &["init", "--quiet", "-b", "main"]);
        let repository = Repository { scratch, dir };
        repository.write("README", "a scratch repository\n");
        repository.commit("the first commit");
        repository
    }

    /// The repository's directory, as flume names a repository.
    pub fn name(&self) -> String {
        self.dir.to_string_lossy().into_owned()
    }

    pub fn git(&self, args: &[&str]) -> String {
        git(&self.dir, args)
    }

    /// Writes `text` at `path` in the worktree.
    pub fn write(&self, path: &str, text: &str) {
        let file = self.dir.join(path);
        if let Some(parent) = file.parent() {
            std::fs::create_dir_all(parent).expect("the file's directory");
        }
        std::fs::write(file, text).expect("the file written");
    }

    /// Removes `path`, a file or a directory, from the worktree.
    pub fn remove(&self, path: &str) {
        let file = self.dir.join(path);
        if file.is_dir() {
            std::fs::remove_dir_all(file).expect("the directory removed");
        } else if file.exists() {
            std::fs::remove_file(file).expect("the file removed");
        }
    }

    /// Commits everything in the worktree on the current branch, and
    /// gives the commit.
    pub fn commit(&self, message: &str) -> String {
        self.git(&["add", "--all"]);
        self.git(&["commit", "--quiet", "--allow-empty", "-m", message]);
        self.head()
    }

    pub fn head(&self) -> String {
        self.git(&["rev-parse", "HEAD"])
    }

    /// A bare clone of this repository beside it, which a second
    /// repository cloned from it names as `origin`.
    pub fn bare_remote(&self) -> PathBuf {
        let remote = self.scratch.path("remote.git");
        git(
            &self.scratch.root,
            &[
                "clone",
                "--quiet",
                "--bare",
                &self.name(),
                &remote.to_string_lossy(),
            ],
        );
        remote
    }

    /// A clone of `remote`, as a host that works from a remote holds it.
    pub fn clone_of(remote: &Path, scratch: Scratch) -> Repository {
        let dir = scratch.path("clone");
        git(
            &scratch.root,
            &[
                "clone",
                "--quiet",
                &remote.to_string_lossy(),
                &dir.to_string_lossy(),
            ],
        );
        Repository { scratch, dir }
    }
}

/// A repository a host made bare beside its worktrees by `git init
/// --bare` rather than by a clone: `project/project.git`, whose `HEAD`
/// names `master`, a branch never made, with a remote as `origin`,
/// fetched with no `origin/HEAD` set, and its main worktree
/// `project/main` on the remote's `main` (design 9).
pub struct Container {
    pub scratch: Scratch,
    /// The directory holding the bare repository and its worktrees, as
    /// flume names the repository.
    pub dir: PathBuf,
}

impl Container {
    pub fn of(remote: &Path, scratch: Scratch) -> Container {
        let dir = scratch.path("project");
        let container = Container { scratch, dir };
        let bare = container.dir.join("project.git");
        std::fs::create_dir_all(&bare).expect("the bare repository's directory");
        git(
            &bare,
            &["init", "--quiet", "--bare", "--initial-branch=master"],
        );
        container.git(&["remote", "add", "origin", &remote.to_string_lossy()]);
        container.git(&[
            "-c",
            "remote.origin.followRemoteHEAD=never",
            "fetch",
            "--quiet",
            "origin",
        ]);
        container.git(&[
            "worktree",
            "add",
            "--quiet",
            &container.dir.join("main").to_string_lossy(),
            "main",
        ]);
        assert_eq!(container.origin_head(), "", "origin/HEAD is set");
        container
    }

    /// The repository's directory, as flume names a repository.
    pub fn name(&self) -> String {
        self.dir.to_string_lossy().into_owned()
    }

    /// Runs git in the bare repository.
    pub fn git(&self, args: &[&str]) -> String {
        git(&self.dir.join("project.git"), args)
    }

    /// The ref `origin/HEAD` names, empty where it is not set.
    pub fn origin_head(&self) -> String {
        self.git(&[
            "for-each-ref",
            "--format=%(symref)",
            "refs/remotes/origin/HEAD",
        ])
    }
}

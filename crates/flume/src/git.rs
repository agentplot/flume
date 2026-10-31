//! git, as the profiles read a repository (section 9): a repository named
//! by its directory, which is either a repository git finds there or the
//! directory holding a bare repository and its worktrees
//! (`project/project.git`, `project/main`; design 9). git is no tool a
//! profile chooses, so this module names none (core.1). Every call asks
//! nothing of a person: git never prompts.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The remote a repository's default branch is read from where it has
/// one (20).
pub const REMOTE: &str = "origin";

/// A repository git is run in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Git {
    /// `-C <dir>` where git finds the repository from a directory, or
    /// `--git-dir <dir>` for the bare repository beside the worktrees.
    at: At,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum At {
    Directory(PathBuf),
    GitDir(PathBuf),
}

impl Git {
    /// The repository `repository` names: git's own repository there, or
    /// the bare repository it holds, `<name>.git` first.
    pub fn at(repository: &str) -> Git {
        let path = Path::new(repository);
        if path.join(".git").exists() || is_git_dir(path) {
            return Git {
                at: At::Directory(path.into()),
            };
        }
        let named = path
            .file_name()
            .map(|name| path.join(format!("{}.git", name.to_string_lossy())));
        if let Some(bare) = named.filter(|bare| is_git_dir(bare)) {
            return Git {
                at: At::GitDir(bare),
            };
        }
        let held = std::fs::read_dir(path).ok().and_then(|entries| {
            let mut bare: Vec<PathBuf> = entries
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|child| child.extension().is_some_and(|ext| ext == "git"))
                .filter(|child| is_git_dir(child))
                .collect();
            bare.sort();
            bare.into_iter().next()
        });
        Git {
            at: held.map_or_else(|| At::Directory(path.into()), At::GitDir),
        }
    }

    /// Where git is run: the directory it finds the repository from, or
    /// the bare repository beside the worktrees.
    pub fn path(&self) -> &Path {
        match &self.at {
            At::Directory(dir) | At::GitDir(dir) => dir,
        }
    }

    /// The repository's common git directory, by its absolute path with
    /// every link resolved.
    pub fn common_dir(&self) -> Result<String, String> {
        let common = self.text(&["rev-parse", "--path-format=absolute", "--git-common-dir"])?;
        let common = PathBuf::from(common.trim());
        let common = common.canonicalize().unwrap_or(common);
        Ok(common.to_string_lossy().into_owned())
    }

    fn command(&self, args: &[&str]) -> Command {
        let mut command = Command::new("git");
        match &self.at {
            At::Directory(dir) => command.arg("-C").arg(dir),
            At::GitDir(dir) => command.arg("--git-dir").arg(dir),
        };
        command.args(args).env("GIT_TERMINAL_PROMPT", "0");
        command
    }

    fn output(&self, args: &[&str]) -> Result<Output, String> {
        self.command(args)
            .output()
            .map_err(|error| format!("git {} could not run: {error}", args.join(" ")))
    }

    /// git's output, or its own words where it refused.
    pub fn bytes(&self, args: &[&str]) -> Result<Vec<u8>, String> {
        let output = self.output(args)?;
        if output.status.success() {
            Ok(output.stdout)
        } else {
            let words = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(format!("git {}: {words}", args.join(" ")))
        }
    }

    /// git's output as text, or its own words where it refused.
    pub fn text(&self, args: &[&str]) -> Result<String, String> {
        let bytes = self.bytes(args)?;
        String::from_utf8(bytes)
            .map_err(|_| format!("git {} gave text that is not UTF-8", args.join(" ")))
    }

    /// Whether git answered yes, which a refusal is not.
    pub fn holds(&self, args: &[&str]) -> bool {
        self.output(args)
            .is_ok_and(|output| output.status.success())
    }

    /// git's yes or no where it answers by its exit, 0 for yes and 1 for
    /// no, as `merge-base --is-ancestor` does; its words where it could
    /// not answer.
    pub fn answer(&self, args: &[&str]) -> Result<bool, String> {
        let output = self.output(args)?;
        match output.status.code() {
            Some(0) => Ok(true),
            Some(1) => Ok(false),
            _ => {
                let words = String::from_utf8_lossy(&output.stderr).trim().to_string();
                Err(format!("git {}: {words}", args.join(" ")))
            }
        }
    }

    /// The commit `revision` names, or why it names none.
    pub fn commit(&self, revision: &str) -> Result<String, String> {
        self.text(&[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{revision}^{{commit}}"),
        ])
        .map(|text| text.trim().to_string())
        .map_err(|_| format!("{revision} names no commit"))
    }

    /// The file at `path` in `revision`, `None` where the revision holds
    /// none there.
    pub fn file(&self, revision: &str, path: &str) -> Result<Option<String>, String> {
        let spec = format!("{revision}:{path}");
        if !self.holds(&["cat-file", "-e", &spec]) {
            return Ok(None);
        }
        self.text(&["show", &spec]).map(Some)
    }

    /// The paths under `directory` in `revision`, relative to it, every
    /// file at any depth.
    pub fn files(&self, revision: &str, directory: &str) -> Result<Vec<String>, String> {
        let directory = directory.trim_end_matches('/');
        let listed = self.text(&[
            "ls-tree",
            "-r",
            "--name-only",
            "-z",
            revision,
            "--",
            &format!("{directory}/"),
        ])?;
        Ok(listed
            .split('\0')
            .filter(|path| !path.is_empty())
            .filter_map(|path| path.strip_prefix(&format!("{directory}/")))
            .map(str::to_string)
            .collect())
    }

    /// The names directly under `directory` in `revision`.
    pub fn names(&self, revision: &str, directory: &str) -> Result<Vec<String>, String> {
        let directory = directory.trim_end_matches('/');
        let listed = self.text(&[
            "ls-tree",
            "--name-only",
            "-z",
            revision,
            "--",
            &format!("{directory}/"),
        ])?;
        Ok(listed
            .split('\0')
            .filter(|path| !path.is_empty())
            .filter_map(|path| path.strip_prefix(&format!("{directory}/")))
            .map(str::to_string)
            .collect())
    }

    /// The repository's remotes, by name.
    pub fn remotes(&self) -> Result<Vec<String>, String> {
        Ok(self
            .text(&["remote"])?
            .lines()
            .map(str::to_string)
            .collect())
    }

    /// The repository's default branch (20, C.2): where it has the
    /// remote, the remote's, the branch `refs/remotes/origin/HEAD` names,
    /// set from the remote's own `HEAD` where the repository lacks it or
    /// it names no commit; where it has none, the branch its own `HEAD`
    /// names. A branch that names no commit is never the default, so a
    /// bare repository's `HEAD` left on a branch never made decides
    /// nothing.
    pub fn default_branch(&self) -> Result<String, String> {
        if !self.remotes()?.iter().any(|remote| remote == REMOTE) {
            let branch = self
                .text(&["symbolic-ref", "--quiet", "--short", "HEAD"])
                .map_err(|_| "it has no remote, and its HEAD names no branch".to_string())?;
            let branch = branch.trim();
            self.commit(&format!("refs/heads/{branch}")).map_err(|_| {
                format!("it has no remote, and its HEAD names {branch}, which holds no commit")
            })?;
            return Ok(branch.to_string());
        }
        if let Some(branch) = self.remote_head() {
            return Ok(branch);
        }
        let listed = self
            .text(&["ls-remote", "--symref", REMOTE, "HEAD"])
            .map_err(|words| format!("{REMOTE}'s HEAD could not be read: {words}"))?;
        let branch = listed
            .lines()
            .find_map(|line| {
                line.strip_prefix("ref: refs/heads/")?
                    .strip_suffix("\tHEAD")
            })
            .ok_or_else(|| format!("{REMOTE}'s HEAD names no branch"))?;
        let tracking = format!("refs/remotes/{REMOTE}/{branch}");
        self.bytes(&[
            "fetch",
            "--quiet",
            REMOTE,
            &format!("+refs/heads/{branch}:{tracking}"),
        ])
        .map_err(|words| format!("the fetch of {branch} from {REMOTE} failed: {words}"))?;
        self.bytes(&["remote", "set-head", REMOTE, branch])
            .map_err(|words| format!("{REMOTE}'s HEAD could not be set to {branch}: {words}"))?;
        self.remote_head()
            .ok_or_else(|| format!("{REMOTE}'s HEAD names {branch}, which holds no commit"))
    }

    /// The branch `refs/remotes/origin/HEAD` names, where it is set and
    /// the branch names a commit.
    fn remote_head(&self) -> Option<String> {
        let named = self
            .text(&[
                "symbolic-ref",
                "--quiet",
                &format!("refs/remotes/{REMOTE}/HEAD"),
            ])
            .ok()?;
        let branch = named
            .trim()
            .strip_prefix(&format!("refs/remotes/{REMOTE}/"))?;
        self.commit(&format!("refs/remotes/{REMOTE}/{branch}"))
            .ok()?;
        Some(branch.to_string())
    }
}

/// Whether `path` is a git directory itself, as a bare repository is.
fn is_git_dir(path: &Path) -> bool {
    path.join("HEAD").is_file() && path.join("objects").is_dir() && path.join("refs").is_dir()
}

/// `path` relative to the directory `base`, with `.` and `..` taken out,
/// as a path inside a revision's tree; `None` where it climbs out of the
/// tree.
pub fn tree_path(base: &str, path: &str) -> Option<String> {
    let mut parts: Vec<&str> = base.split('/').filter(|part| !part.is_empty()).collect();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            part => parts.push(part),
        }
    }
    Some(parts.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_path_in_a_tree_is_resolved_against_its_directory() {
        assert_eq!(
            tree_path(".config", "prompts/code.md"),
            Some(".config/prompts/code.md".into())
        );
        assert_eq!(
            tree_path(".config", "../prompts/code.md"),
            Some("prompts/code.md".into())
        );
        assert_eq!(tree_path(".config", "../../code.md"), None);
        assert_eq!(tree_path("", "./a/b.md"), Some("a/b.md".into()));
    }
}

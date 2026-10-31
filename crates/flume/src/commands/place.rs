//! The repository a command runs in and the place it names (section 9;
//! design 16): the repository is the directory holding a bare repository
//! and its worktrees, or the main worktree, found from the directory the
//! command runs in or `--repository`; a place is named by its location or
//! by `<kind>--<name>` of that repository, at the location the one place
//! profile makes it at (C.1; record 0027). git is no tool a profile
//! chooses, so nothing here names one (core.1).

use crate::git::Git;
use crate::host::config;
use crate::profiles::place::git_worktree;
use crate::profiles::store::state_dir::StateDir;
use flume_seams::{PlaceId, Store};
use std::path::{Path, PathBuf};

/// The repository the command runs in: `given`'s, or the working
/// directory's. From inside any of its worktrees, a place's included, it
/// is the main worktree, or the directory beside the bare repository
/// they share.
pub fn repository(given: Option<&Path>) -> Result<String, String> {
    let directory = match given {
        Some(path) => absolute(path)?,
        None => std::env::current_dir()
            .map_err(|error| format!("the working directory cannot be read: {error}"))?,
    };
    let directory = directory
        .canonicalize()
        .map_err(|error| format!("{}: {error}", directory.display()))?;
    let git = Git::at(&directory.to_string_lossy());
    if git.path() != directory {
        // The directory holds the bare repository its worktrees share.
        return Ok(directory.to_string_lossy().into_owned());
    }
    let common = git
        .text(&["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .map_err(|_| {
            format!(
                "{} is in no repository, and holds none beside its worktrees",
                directory.display()
            )
        })?;
    let common = PathBuf::from(common.trim());
    let common = common.canonicalize().unwrap_or(common);
    // The repository's own git directory, or the bare one its worktrees
    // share: either way the repository is the directory above it.
    let holder = common.parent();
    holder
        .map(|holder| holder.to_string_lossy().into_owned())
        .ok_or_else(|| format!("{} has no directory above it", common.display()))
}

/// The place `named` names: a location where it holds a `/`, else
/// `<kind>--<name>` of the repository `repository` finds, where the place
/// profile makes it. Where the host's store holds no place there and
/// holds one at `<repository>/places/<kind>--<name>`, it is that one,
/// made inside the main worktree before record 0027, which is read and
/// tidied where it stands.
pub fn place(named: &str, repository_given: Option<&Path>) -> Result<PlaceId, String> {
    if named.contains('/') {
        let path = absolute(Path::new(named))?;
        return Ok(PlaceId::new(settled(&path).to_string_lossy().into_owned()));
    }
    let repository = repository(repository_given)?;
    let own = config::flume_data().ok();
    let place = PlaceId::new(git_worktree::location(&repository, named, own.as_deref())?);
    let inside = PlaceId::new(format!("{repository}/places/{named}"));
    if place != inside && !held(&place) && held(&inside) {
        return Ok(inside);
    }
    Ok(place)
}

/// Whether the host's store holds a place at `place`.
fn held(place: &PlaceId) -> bool {
    StateDir::of_host().is_ok_and(|store| store.read(place).is_some())
}

/// The base a launch naming none is cut from: the repository's default
/// branch, the remote's where it has one and its own only where it has
/// none (2, 20).
pub fn default_base(repository: &str) -> Result<String, String> {
    Git::at(repository).default_branch().map_err(|reason| {
        format!("the launch names no base, and {repository} names no default branch: {reason}")
    })
}

/// `path` from the working directory where it is relative.
fn absolute(path: &Path) -> Result<PathBuf, String> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    std::env::current_dir()
        .map(|directory| directory.join(path))
        .map_err(|error| format!("the working directory cannot be read: {error}"))
}

/// `path` with every link resolved as far as it exists, so a location is
/// named the one way a launch named it.
fn settled(path: &Path) -> PathBuf {
    if let Ok(path) = path.canonicalize() {
        return path;
    }
    match (path.parent(), path.file_name()) {
        (Some(parent), Some(name)) => settled(parent).join(name),
        _ => path.to_path_buf(),
    }
}

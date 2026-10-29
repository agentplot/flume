//! The store profile over the host's state directory (C.7; design 10).
//! Each place's held state is one document,
//! `$XDG_STATE_HOME/flume/places/<flat>.json`, its version inside it, and
//! the place's own folder `<flat>/` beside it, the location flattened to
//! one file name (102, 128). A write takes `<flat>.lock`, reads the
//! version, and writes a temporary file renamed over the document, or
//! refuses where the version moved; a new place's first write creates
//! the document by a link that fails where one exists (75, 103); a write
//! of none removes the document and the folder (62). Nothing is written
//! into a repository (104, core.16).

use flume_seams::{HeldState, PlaceId, Store, Versioned, Written};
use std::fs::{self, File, OpenOptions};
use std::io::ErrorKind;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// The store in one state directory, `<state>/flume`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateDir {
    root: PathBuf,
}

/// Temporary files' numbers in this process.
static TEMPORARY: AtomicU64 = AtomicU64::new(0);

impl StateDir {
    /// The host's store, `$XDG_STATE_HOME/flume` (C.7).
    pub fn of_host() -> Result<StateDir, String> {
        crate::host::config::flume_state().map(StateDir::at)
    }

    /// The store under `root`, which holds `places/`.
    pub fn at(root: impl Into<PathBuf>) -> StateDir {
        StateDir { root: root.into() }
    }

    /// The directory the store's documents lie in.
    pub fn places(&self) -> PathBuf {
        self.root.join("places")
    }

    /// A place's location flattened to one file name.
    fn flat(place: &PlaceId) -> String {
        place.0.trim_start_matches('/').replace('/', "%")
    }

    fn document(&self, place: &PlaceId) -> PathBuf {
        self.places().join(format!("{}.json", Self::flat(place)))
    }

    fn lock_file(&self, place: &PlaceId) -> PathBuf {
        self.places().join(format!("{}.lock", Self::flat(place)))
    }

    fn folder_path(&self, place: &PlaceId) -> PathBuf {
        self.places().join(Self::flat(place))
    }

    /// The document at `path`, none where there is none or it cannot be
    /// read as held state.
    fn read_document(path: &Path) -> Option<Versioned> {
        let text = fs::read_to_string(path).ok()?;
        serde_json::from_str(&text).ok()
    }

    /// The place's lock, held while the file lives. A lock taken on a
    /// file that a removal unlinked meanwhile is let go and taken again on
    /// the file that now stands, so two writers never hold two locks.
    fn lock(&self, place: &PlaceId) -> std::io::Result<File> {
        let path = self.lock_file(place);
        loop {
            fs::create_dir_all(self.places())?;
            let file = OpenOptions::new()
                .create(true)
                .truncate(false)
                .write(true)
                .open(&path)?;
            file.lock()?;
            let held = file.metadata()?;
            match fs::metadata(&path) {
                Ok(standing) if standing.ino() == held.ino() && standing.dev() == held.dev() => {
                    return Ok(file)
                }
                Ok(_) => continue,
                Err(error) if error.kind() == ErrorKind::NotFound => continue,
                Err(error) => return Err(error),
            }
        }
    }

    /// A temporary file beside the documents holding `versioned`.
    fn temporary(&self, place: &PlaceId, versioned: &Versioned) -> std::io::Result<PathBuf> {
        let path = self.places().join(format!(
            ".{}.{}-{}.tmp",
            Self::flat(place),
            std::process::id(),
            TEMPORARY.fetch_add(1, Ordering::Relaxed)
        ));
        let text = serde_json::to_string_pretty(versioned).map_err(std::io::Error::other)?;
        fs::write(&path, text)?;
        File::open(&path)?.sync_all()?;
        Ok(path)
    }

    fn written(&self, place: &PlaceId, at: Option<u64>, next: Option<&HeldState>) -> Written {
        let Ok(lock) = self.lock(place) else {
            return Written::Moved;
        };
        let document = self.document(place);
        let current = Self::read_document(&document).map(|versioned| versioned.version);
        if current != at {
            return Written::Moved;
        }
        let Some(next) = next else {
            let _ = fs::remove_dir_all(self.folder_path(place));
            if fs::remove_file(&document).is_err() && document.exists() {
                return Written::Moved;
            }
            let _ = fs::remove_file(self.lock_file(place));
            drop(lock);
            return Written::Written {
                version: at.map_or(0, |version| version + 1),
            };
        };
        let version = next_version(current);
        let versioned = Versioned {
            version,
            state: next.clone(),
        };
        let Ok(temporary) = self.temporary(place, &versioned) else {
            return Written::Moved;
        };
        let placed = match at {
            None => {
                let linked = fs::hard_link(&temporary, &document);
                let _ = fs::remove_file(&temporary);
                linked
            }
            Some(_) => fs::rename(&temporary, &document).inspect_err(|_| {
                let _ = fs::remove_file(&temporary);
            }),
        };
        if placed.is_err() {
            return Written::Moved;
        }
        if at.is_none() {
            let _ = fs::create_dir_all(self.folder_path(place));
        }
        drop(lock);
        Written::Written { version }
    }
}

/// A version past `current` that no earlier document of the place held,
/// even one removed and made again: the clock's nanoseconds where they
/// are past it.
fn next_version(current: Option<u64>) -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_nanos() as u64);
    match current {
        Some(current) => now.max(current + 1),
        None => now.max(1),
    }
}

impl Store for StateDir {
    fn read(&self, place: &PlaceId) -> Option<Versioned> {
        Self::read_document(&self.document(place))
    }

    fn read_every(&self) -> Vec<Versioned> {
        let Ok(entries) = fs::read_dir(self.places()) else {
            return vec![];
        };
        let mut documents: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension == "json")
                    && !path
                        .file_name()
                        .is_some_and(|name| name.to_string_lossy().starts_with('.'))
            })
            .collect();
        documents.sort();
        documents
            .iter()
            .filter_map(|path| Self::read_document(path))
            .collect()
    }

    fn write(&self, place: &PlaceId, at: Option<u64>, next: Option<&HeldState>) -> Written {
        self.written(place, at, next)
    }

    fn folder(&self, place: &PlaceId) -> String {
        self.folder_path(place).to_string_lossy().into_owned()
    }
}

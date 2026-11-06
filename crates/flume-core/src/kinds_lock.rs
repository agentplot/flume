//! `flume kinds lock` (155, 156, C.2; design 6): each whole identity read
//! through the kinds seam and resolved to its content, flume's own as
//! flume carries it and a host's source's as the host holds it, and
//! recorded at its content's hash in the lock named, the host's or a
//! working copy's; `--remove` takes entries out, reading no package, and
//! `--hash` records its one identity only where its content hashes so.
//! The repository's and a launch's kinds are refused, since no lock
//! records them, and a lock that cannot be read is never written over.
//! The lock is read and written whole through the kinds seam, only where
//! its entries change, so the act run twice leaves the same lock (74;
//! core.16, core.17). It fetches nothing and commits nothing.

use crate::launch::{and_list, same, Locks};
use crate::{Answer, Engine, LockedKind};
use flume_seams::*;

/// What `flume kinds lock` is asked (155, C.2): the lock it writes, each
/// kind as written, whether it takes them out, and the hash its one kind
/// must have.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KindsLock {
    pub of: LockOf,
    pub kinds: Vec<String>,
    pub remove: bool,
    pub hash: Option<String>,
}

/// The lock `flume kinds lock` writes: the host's, or the one of the
/// working copy it runs in, by the copy's directory (155).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LockOf {
    Host,
    WorkingCopy { dir: String },
}

impl Engine {
    /// Records or removes each kind named in the lock named, or refuses
    /// naming why, the lock left as it was (155, 156; design 6).
    pub fn lock_kinds(&self, asked: &KindsLock) -> Answer {
        match self.locked(asked) {
            Ok(answer) => answer,
            Err(reason) => Answer::refused(reason),
        }
    }

    fn locked(&self, asked: &KindsLock) -> Result<Answer, String> {
        let kinds = self.resolver.kinds();
        if asked.kinds.is_empty() {
            return Err("flume kinds lock names no kind (155)".into());
        }
        if asked.hash.is_some() && asked.kinds.len() != 1 {
            return Err(format!(
                "--hash names the content of one kind, and {} are named (155)",
                asked.kinds.len()
            ));
        }
        let named = asked
            .kinds
            .iter()
            .map(|text| whole(&*kinds, text))
            .collect::<Result<Vec<_>, _>>()?;
        let lock_file = match &asked.of {
            LockOf::Host => self
                .host
                .kinds_lock()
                .ok_or("the host keeps no lock of kinds (155)")?,
            LockOf::WorkingCopy { dir } => {
                let path = kinds
                    .lock_path()
                    .ok_or("its kinds profile names no repository's lock (155)")?;
                format!("{}/{path}", dir.trim_end_matches('/'))
            }
        };
        let before = match kinds.read_lock_file(&lock_file) {
            LockRead::None => vec![],
            LockRead::Read { entries } => entries,
            LockRead::CouldNotRead { reason } => {
                return Err(format!(
                    "the lock {lock_file} could not be read, and is left as it is: {reason} (155)"
                ))
            }
        };
        let mut entries = before.clone();
        let mut recorded = vec![];
        let mut removed = vec![];
        for named in &named {
            if asked.remove {
                let held = entries.len();
                entries.retain(|entry| !names(named, &entry.kind));
                if entries.len() != held {
                    removed.push(named.spelled.clone());
                }
                continue;
            }
            let package = self.content_of(named)?;
            if let Some(hash) = asked.hash.as_ref().filter(|hash| **hash != package.hash) {
                return Err(format!(
                    "the kind {}'s content hashes to {}, not {hash}, and the lock {lock_file} is left as it was (155, 156)",
                    package.identity, package.hash
                ));
            }
            let entry = LockEntry {
                kind: package.identity,
                hash: package.hash,
            };
            recorded.push(LockedKind {
                kind: entry.kind.spelled.clone(),
                hash: entry.hash.clone(),
            });
            match entries
                .iter_mut()
                .find(|held| same(&held.kind, &entry.kind))
            {
                Some(held) => *held = entry,
                None => entries.push(entry),
            }
        }
        if entries != before {
            kinds
                .write_lock(&lock_file, &entries)
                .map_err(|reason| format!("the lock {lock_file} could not be written: {reason}"))?;
        }
        Ok(Answer::Locked {
            lock_file,
            recorded,
            removed,
        })
    }

    /// The content of the kind `named` names whole: flume's own as flume
    /// carries it, a host's source's as the host holds it (155, 156).
    fn content_of(&self, named: &KindNamed) -> Result<Packaged, String> {
        if named.source.as_deref() == Some(OWN_SOURCE) {
            let own: Vec<Packaged> = self
                .resolver
                .kinds()
                .flumes_own()
                .into_iter()
                .filter(|own| own.identity.name == named.name)
                .collect();
            let held = and_list(own.iter().map(|own| own.identity.spelled.clone()).collect());
            return own
                .into_iter()
                .find(|own| Some(own.identity.version) == named.version)
                .ok_or_else(|| {
                    format!(
                        "the kind {} is not held: flume holds {} (22, 154)",
                        named.spelled,
                        if held.is_empty() {
                            "none of its name".into()
                        } else {
                            held
                        }
                    )
                });
        }
        let source = named.source.as_deref().unwrap_or_default();
        let at = self.host_source(source, named)?;
        let (package, _) = self.package_of(named, &at, || Ok(Locks::none()))?;
        Ok(package)
    }
}

/// `text` read by the kinds seam as a whole identity of a kind a lock
/// records: refused where it names the repository's or a launch's, which
/// run with no lock, or lacks its source or its version (155).
fn whole(kinds: &dyn Kinds, text: &str) -> Result<KindNamed, String> {
    let named = kinds
        .read_named(text)
        .map_err(|reason| format!("the kind {text} names no kind flume reads: {reason} (154)"))?;
    let whose = match named.source.as_deref() {
        Some(REPO_SOURCE) => Some("the repository's"),
        Some(LAUNCH_SOURCE) => Some("a launch's"),
        _ => None,
    };
    if let Some(whose) = whose {
        return Err(format!(
            "the kind {text} is {whose}, which runs with no lock: a lock records flume's own kinds and a host's sources' (155)"
        ));
    }
    if named.source.is_none() || named.version.is_none() {
        return Err(format!(
            "the kind {text} is not named whole: flume kinds lock records a whole identity, <source>/<name>@<version> (155)"
        ));
    }
    Ok(named)
}

/// Whether the whole identity `named` names `identity` (154).
fn names(named: &KindNamed, identity: &KindIdentity) -> bool {
    named.source.as_deref() == Some(identity.source.as_str())
        && named.name == identity.name
        && named.version == Some(identity.version)
}

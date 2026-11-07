//! The kinds profile (C.2; design 7): every declaration, a repository's
//! `.config/flume.toml` at a revision, a launch's file and flume's own
//! built in, read through `weir-kinds`, with no reader of flume's own
//! (core.4). The format's fields are mapped onto the seam's types, every
//! section the format does not name is ignored (23), and a declaration
//! weir refuses is answered as could not be read, naming every field that
//! fails (84). A kind that lands and declares `stages = []` is mapped to a
//! kind with no stages, which weir refuses on a kind that does not land
//! (18, 202; weir 51, 57). A kind's `agents` table, and a `[kinds.<kind>]` table that
//! gives settings to a kind declared elsewhere, are weir's too: each is
//! mapped onto the seam's settings for every stage and for each stage it
//! names, the latter returned apart from the kinds (32, C.2; weir 52,
//! 57, C.4); which declaration they join, and whether the stages they
//! name exist, are the core's to say. Every agents table's `programs`,
//! each program's own settings, is read by flume's one reader of them, a
//! file it names read as a prompt is (195). A prompt is a path relative
//! to its declaration's file, read beside it: at the same revision for a
//! repository's, on disk for a launch's, and from the binary for flume's
//! own, whose versions are each a declaration of its own with its prompts
//! beside it (157).
//!
//! flume's own kinds are packages, each read through weir-kinds' package
//! reader at its identity, `flume/<name>@<version>`, with its content's
//! hash over its declaration's and prompts' paths and bytes (156, 157).
//! A kind as a launch or a role names it is read by weir-kinds' reader of
//! a kind's name, which spells it back, and the identity of a kind of the
//! repository's or a launch's, which lies in no package, is read by
//! weir-kinds' reader of an identity, as is a package's (154; weir C.4).
//! The repository's declaration is [`DECLARATION`] and its lock [`LOCK`],
//! each given the core by its path within the tree (20, 155).
//!
//! A host's source is read through weir-kinds alone (155 to 160, 192; weir
//! 88 to 93, C.4; core.4):
//!
//! - **Its listing**, read-source: every entry of its `kinds_dir`, through
//!   a link or not, that is a directory holding a `kind.toml` and whose
//!   name weir's reader reads, joined to the source's name by `/`, as a
//!   whole identity, in weir's order of identities; every entry of any
//!   other shape is ignored (159).
//! - **A package**, read-package: `<kinds_dir>/<name>@<version>/`, read by
//!   weir-kinds' reader of a package's folder at that identity, with its
//!   content's hash, each prompt read beside its `kind.toml`; a package
//!   weir refuses, or whose agents give an `env` or a program by a
//!   location, is answered as could not be read, naming the package and
//!   every field (157, 158).
//! - **The repository's lock**, [`LOCK`] at a revision as git gives it, and
//!   **a lock on disk**, each read by weir-kinds' lock reader; no file is
//!   no lock (20, 155).
//! - **A lock written**, spelled by weir-kinds' lock writer, written beside
//!   the lock and renamed over it, so a write that fails leaves the lock
//!   as it was and no reader sees half of one (155).
//!
//! [`source_fault`] says which names a host's source may not take: none
//! weir's reader refuses, and none of flume's own source, the
//! repository's or a launch's (154, 191; weir C.4).
//!
//! The repository's binding is the declaration's `[profiles]` table at
//! the default branch: the remote's, after one fetch, where the
//! repository has a remote, and the local default branch otherwise (20,
//! 105, C.2). No other tool's file is read to choose a profile (core.3).
//! Beside `[profiles]`, the repository's `[agents]` is flume's own table
//! of its declaration and no part of any kind: its settings for every
//! kind it runs, read from the sections weir returns unnamed with the
//! kinds at their revision, as a kind's agents table is but naming no
//! stage, stream, environment or pane (21, 32, 195, C.2). A launch's
//! declaration and flume's own are not read for it.

use crate::git::{tree_path, Git, REMOTE};
use crate::own::{self, Reader};
use flume_seams::{
    Binding, BindingRead, Deliverable, Kind, KindIdentity, KindNamed, KindSettings, Kinds,
    KindsRead, LockEntry, LockRead, PackageRead, Packaged, Revision, Settings, SourceRead, Stage,
};
use std::collections::{BTreeMap, BTreeSet};
use std::io::ErrorKind;
use std::path::Path;
use weir_kinds::toml::{Table, Value};
use weir_kinds::{End, Refusal};

/// Where a repository declares its kinds and its binding, in its tree.
pub const DECLARATION: &str = ".config/flume.toml";

/// Where a repository's lock lies, in its tree and in a working copy
/// (155, C.2).
pub const LOCK: &str = ".config/flume.lock";

/// The source of flume's own kinds: flume's name, as weir's identity
/// names a tool's own source (154; weir C.4).
pub const OWN_SOURCE: &str = "flume";

/// One version of one of flume's own kinds, built into the binary: the
/// identity it is held at, as weir's reader is given it, its declaration
/// and each prompt it names by its path within the version's own
/// directory (22, 156, 157, C.2; design 7).
struct Own {
    identity: &'static str,
    declaration: &'static str,
    prompts: &'static [(&'static str, &'static str)],
}

/// Where a package declares its kind, at its root (157; weir C.4).
const KIND_TOML: &str = "kind.toml";

/// flume's own kinds, one declaration per version, each in a directory of
/// its own under `own/`, `<name>@<version>/kind.toml` with its prompts
/// beside it under `prompts/`: change@1 and change@2 side by side, and
/// fix@1 and chore@1 (22, 157, C.2; record 0022).
const OWN: &[Own] = &[
    Own {
        identity: "flume/change@1",
        declaration: include_str!("own/change@1/kind.toml"),
        prompts: &[
            (
                "prompts/change-construct.md",
                include_str!("own/change@1/prompts/change-construct.md"),
            ),
            (
                "prompts/change-code.md",
                include_str!("own/change@1/prompts/change-code.md"),
            ),
            (
                "prompts/change-verify.md",
                include_str!("own/change@1/prompts/change-verify.md"),
            ),
        ],
    },
    Own {
        identity: "flume/change@2",
        declaration: include_str!("own/change@2/kind.toml"),
        prompts: &[
            (
                "prompts/change-construct.md",
                include_str!("own/change@2/prompts/change-construct.md"),
            ),
            (
                "prompts/change-code.md",
                include_str!("own/change@2/prompts/change-code.md"),
            ),
            (
                "prompts/change-verify.md",
                include_str!("own/change@2/prompts/change-verify.md"),
            ),
        ],
    },
    Own {
        identity: "flume/chore@1",
        declaration: include_str!("own/chore@1/kind.toml"),
        prompts: &[(
            "prompts/code.md",
            include_str!("own/chore@1/prompts/code.md"),
        )],
    },
    Own {
        identity: "flume/fix@1",
        declaration: include_str!("own/fix@1/kind.toml"),
        prompts: &[("prompts/code.md", include_str!("own/fix@1/prompts/code.md"))],
    },
];

/// The kinds profile over weir's kind format.
#[derive(Clone, Copy, Debug, Default)]
pub struct Weir;

impl Kinds for Weir {
    fn read_kinds(&self, repository: &str, revision: &Revision) -> KindsRead {
        let git = Git::at(repository);
        if let Err(reason) = git.commit(&revision.0) {
            return KindsRead::CouldNotRead { reason };
        }
        let text = match git.file(&revision.0, DECLARATION) {
            Ok(Some(text)) => text,
            Ok(None) => {
                return KindsRead::Read {
                    kinds: vec![],
                    settings: BTreeMap::new(),
                    agents: Box::default(),
                }
            }
            Err(reason) => return KindsRead::CouldNotRead { reason },
        };
        let directory = Path::new(DECLARATION)
            .parent()
            .map(|parent| parent.to_string_lossy().into_owned())
            .unwrap_or_default();
        read(&text, Whose::Repository, &|path| {
            let at = tree_path(&directory, path)
                .ok_or_else(|| format!("{path} lies outside the repository"))?;
            let text = git
                .file(&revision.0, &at)?
                .ok_or_else(|| format!("{at} is not in {revision}"))?;
            Ok((at, text))
        })
    }

    fn read_declaration(&self, declaration: &str) -> KindsRead {
        let file = Path::new(declaration);
        let text = match std::fs::read_to_string(file) {
            Ok(text) => text,
            Err(error) => {
                return KindsRead::CouldNotRead {
                    reason: format!("{declaration} could not be read: {error}"),
                }
            }
        };
        let directory = file.parent().unwrap_or(Path::new(""));
        read(&text, Whose::Launch, &|path| {
            let at = directory.join(path);
            std::fs::read_to_string(&at)
                .map(|text| (at.display().to_string(), text))
                .map_err(|error| format!("{} could not be read: {error}", at.display()))
        })
    }

    /// Each version of flume's own kinds as a package, read through
    /// weir-kinds' package reader over its declaration and prompts by
    /// their paths within its version's directory, at the identity it is
    /// held at, with its content's hash over those paths and bytes; two
    /// versions of `change` among them (22, 156, 157).
    fn flumes_own(&self) -> Vec<Packaged> {
        OWN.iter()
            .map(|own| {
                own_package(own).unwrap_or_else(|reason| {
                    panic!(
                        "flume's own {} is built in and reads: {reason}",
                        own.identity
                    )
                })
            })
            .collect()
    }

    fn read_binding(&self, repository: &str) -> BindingRead {
        match default_revision(repository) {
            Ok(revision) => binding_at(repository, &revision),
            Err(reason) => BindingRead::CouldNotRead { reason },
        }
    }

    fn read_named(&self, text: &str) -> Result<KindNamed, String> {
        let named: weir_kinds::NamedKind = text
            .parse()
            .map_err(|refusal: weir_kinds::NameRefusal| refusal.to_string())?;
        let version = named
            .version()
            .map(|version| {
                u32::try_from(version)
                    .map_err(|_| format!("`{text}` names a version more than flume counts"))
            })
            .transpose()?;
        Ok(KindNamed {
            source: named.source().map(str::to_string),
            name: named.name().to_string(),
            version,
            spelled: named.to_string(),
        })
    }

    fn identity_of(&self, source: &str, name: &str, version: u32) -> Result<KindIdentity, String> {
        let held = held_at(source, &format!("{name}@{version}"))?;
        let mut problems = vec![];
        let identity = map_identity(&held, &mut problems);
        if problems.is_empty() {
            Ok(identity)
        } else {
            Err(problems.join("; "))
        }
    }

    fn declaration_path(&self) -> Option<String> {
        Some(DECLARATION.into())
    }

    fn lock_path(&self) -> Option<String> {
        Some(LOCK.into())
    }

    fn read_source(&self, source: &str, kinds_dir: &str) -> SourceRead {
        let dir = Path::new(kinds_dir);
        let unread = |words: String| SourceRead::CouldNotRead {
            reason: format!("{kinds_dir}, the directory of the source {source}, {words}"),
        };
        if let Err(refusal) = weir_kinds::check_name(source) {
            return SourceRead::CouldNotRead {
                reason: format!("the source {refusal}"),
            };
        }
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == ErrorKind::NotFound => {
                return unread("does not exist".into())
            }
            Err(error) => return unread(format!("could not be read: {error}")),
        };
        let mut held = BTreeSet::new();
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => return unread(format!("could not be read: {error}")),
            };
            let Some(name) = entry.file_name().to_str().map(str::to_string) else {
                continue;
            };
            let Ok(identity) = held_at(source, &name) else {
                continue;
            };
            if entry.path().join(KIND_TOML).is_file() {
                held.insert(identity);
            }
        }
        let kinds = held
            .iter()
            .filter_map(|identity| {
                let mut problems = vec![];
                let identity = map_identity(identity, &mut problems);
                problems.is_empty().then_some(identity)
            })
            .collect();
        SourceRead::Read { kinds }
    }

    fn read_package(&self, source: &str, kinds_dir: &str, name: &str, version: u32) -> PackageRead {
        let unread = |reason: String| PackageRead::CouldNotRead { reason };
        let entry = format!("{name}@{version}");
        let held = match held_at(source, &entry) {
            Ok(held) => held,
            Err(reason) => return unread(reason),
        };
        let dir = Path::new(kinds_dir).join(&entry);
        if !dir.exists() {
            return unread(format!(
                "the source {source} holds no {entry}: {} does not exist",
                dir.display()
            ));
        }
        let refused_at = |problems: String| {
            unread(format!(
                "the package {held} at {}: {problems}",
                dir.display()
            ))
        };
        let package = match weir_kinds::read_package_dir(&dir, &held) {
            Ok(package) => package,
            Err(refusal) => return refused_at(refused(&refusal)),
        };
        let mut problems = vec![];
        let identity = map_identity(&package.identity, &mut problems);
        let kind = map_kind(
            &package.kind,
            &|path: &str| in_package(&dir, path),
            &mut problems,
        );
        if !problems.is_empty() {
            return refused_at(problems.join("; "));
        }
        PackageRead::Read {
            package: Box::new(Packaged {
                identity,
                kind,
                hash: package.hash.to_string(),
                dir: Some(dir.to_string_lossy().into_owned()),
            }),
        }
    }

    fn read_lock_at(&self, repository: &str, revision: &Revision) -> LockRead {
        let git = Git::at(repository);
        if let Err(reason) = git.commit(&revision.0) {
            return LockRead::CouldNotRead { reason };
        }
        match git.file(&revision.0, LOCK) {
            Ok(None) => LockRead::None,
            Ok(Some(text)) => lock_read(&text, &format!("{LOCK} at {revision}")),
            Err(reason) => LockRead::CouldNotRead { reason },
        }
    }

    fn read_lock_file(&self, path: &str) -> LockRead {
        match std::fs::read_to_string(path) {
            Ok(text) => lock_read(&text, path),
            Err(error) if error.kind() == ErrorKind::NotFound => LockRead::None,
            Err(error) => LockRead::CouldNotRead {
                reason: format!("{path} could not be read: {error}"),
            },
        }
    }

    fn write_lock(&self, path: &str, entries: &[LockEntry]) -> Result<(), String> {
        let mut lock = weir_kinds::Lock::default();
        for entry in entries {
            let identity: weir_kinds::Identity = entry
                .kind
                .spelled
                .parse()
                .map_err(|refusal: weir_kinds::NameRefusal| refusal.to_string())?;
            let hash: weir_kinds::ContentHash = entry
                .hash
                .parse()
                .map_err(|refusal: weir_kinds::HashRefusal| refusal.to_string())?;
            if let Some(given) = lock
                .entries
                .get(&identity)
                .filter(|given| given.hash != hash)
            {
                return Err(format!(
                    "{identity} is given at two hashes, {} and {hash}, and a lock records an identity once",
                    given.hash
                ));
            }
            lock.entries.insert(
                identity,
                weir_kinds::LockEntry {
                    hash,
                    sections: Table::new(),
                },
            );
        }
        write_beside(Path::new(path), &weir_kinds::spell_lock(&lock))
            .map_err(|reason| format!("the lock {path} could not be written: {reason}"))
    }
}

/// Why `name` cannot be the name of a source the host declares: one weir's
/// reader refuses as a name, or the source of flume's own kinds, the
/// repository's or a launch's; none where it can be (154, 191; weir C.4).
pub fn source_fault(name: &str) -> Option<String> {
    if let Err(refusal) = weir_kinds::check_name(name) {
        return Some(format!("its name {refusal}"));
    }
    let whose = match name {
        OWN_SOURCE => "flume's own kinds",
        weir_kinds::REPO => "the repository's kinds",
        weir_kinds::LAUNCH => "a launch's kinds",
        _ => return None,
    };
    Some(format!(
        "its name `{name}` is the source of {whose}, which no host declares"
    ))
}

/// The identity a package of the source `source` is held at, its
/// directory named `entry`, `<name>@<version>`, as weir's reader reads the
/// two joined (159; weir C.4).
fn held_at(source: &str, entry: &str) -> Result<weir_kinds::Identity, String> {
    format!("{source}/{entry}")
        .parse()
        .map_err(|refusal: weir_kinds::NameRefusal| refusal.to_string())
}

/// A lock's text, `named` as its reader names it, read by weir's lock
/// reader (155; weir 91, 93).
fn lock_read(text: &str, named: &str) -> LockRead {
    let lock = match weir_kinds::read_lock(text) {
        Ok(lock) => lock,
        Err(refusal) => {
            return LockRead::CouldNotRead {
                reason: format!("{named}: {}", refused(&refusal)),
            }
        }
    };
    let mut problems = vec![];
    let entries = lock
        .entries
        .iter()
        .map(|(identity, entry)| LockEntry {
            kind: map_identity(identity, &mut problems),
            hash: entry.hash.to_string(),
        })
        .collect();
    if problems.is_empty() {
        LockRead::Read { entries }
    } else {
        LockRead::CouldNotRead {
            reason: format!("{named}: {}", problems.join("; ")),
        }
    }
}

/// `text` written whole at `file`: written beside it, under a name of its
/// own that begins with `.`, and renamed over it, its directory made where
/// it is missing; the file as it was where any step fails (155).
fn write_beside(file: &Path, text: &str) -> Result<(), String> {
    let name = file
        .file_name()
        .ok_or_else(|| format!("{} names no file", file.display()))?;
    if let Some(dir) = file.parent().filter(|dir| !dir.as_os_str().is_empty()) {
        std::fs::create_dir_all(dir).map_err(|error| format!("{}: {error}", dir.display()))?;
    }
    let beside = file.with_file_name(format!(
        ".{}.{}.tmp",
        name.to_string_lossy(),
        std::process::id()
    ));
    let written = std::fs::write(&beside, text)
        .and_then(|()| std::fs::rename(&beside, file))
        .map_err(|error| error.to_string());
    if written.is_err() {
        let _ = std::fs::remove_file(&beside);
    }
    written
}

/// One version of flume's own kinds read as a package (156, 157).
fn own_package(own: &Own) -> Result<Packaged, String> {
    let held: weir_kinds::Identity = own
        .identity
        .parse()
        .map_err(|refusal: weir_kinds::NameRefusal| refusal.to_string())?;
    let files = std::iter::once((KIND_TOML, own.declaration)).chain(own.prompts.iter().copied());
    let package = weir_kinds::read_package(files, &held).map_err(|refusal| refused(&refusal))?;
    let mut problems = vec![];
    let identity = map_identity(&package.identity, &mut problems);
    let kind = map_kind(
        &package.kind,
        &|path: &str| {
            own.prompts
                .iter()
                .find(|(at, _)| *at == path)
                .map(|(at, text)| (at.to_string(), text.to_string()))
                .ok_or_else(|| format!("{path} is not built in"))
        },
        &mut problems,
    );
    if !problems.is_empty() {
        return Err(problems.join("; "));
    }
    Ok(Packaged {
        identity,
        kind,
        hash: package.hash.to_string(),
        dir: None,
    })
}

/// A file of the package at `dir` by its path within it, with the path
/// read: a path that leaves the package, by `..`, an absolute path or a
/// link, is refused, so a prompt or a program's settings file reads
/// nothing outside it (157, 195; core.34, core.50).
fn in_package(dir: &Path, path: &str) -> Result<(String, String), String> {
    let outside = || format!("{path} lies outside the package");
    if Path::new(path).is_absolute() {
        return Err(outside());
    }
    let within = tree_path("", path).ok_or_else(outside)?;
    let at = dir.join(within);
    let unread = |error: std::io::Error| format!("{} could not be read: {error}", at.display());
    let real = at.canonicalize().map_err(unread)?;
    let root = dir.canonicalize().map_err(unread)?;
    if !real.starts_with(&root) {
        return Err(outside());
    }
    std::fs::read_to_string(&real)
        .map(|text| (at.display().to_string(), text))
        .map_err(unread)
}

/// An identity as weir's reader gave it, mapped onto the seam's with its
/// parts and weir's spelling (154; weir 88, C.4).
fn map_identity(identity: &weir_kinds::Identity, problems: &mut Vec<String>) -> KindIdentity {
    let version = u32::try_from(identity.version()).unwrap_or_else(|_| {
        problems.push(format!("{identity}: its version is more than flume counts"));
        0
    });
    KindIdentity {
        source: identity.source().to_string(),
        name: identity.name().to_string(),
        version,
        spelled: identity.to_string(),
    }
}

/// The revision of the repository's default branch (20, C.2): the
/// remote's after one fetch of it where the repository has the remote,
/// and the local branch's otherwise; why there is none where git names
/// no default branch.
pub fn default_revision(repository: &str) -> Result<Revision, String> {
    let git = Git::at(repository);
    let branch = git
        .default_branch()
        .map_err(|reason| format!("{repository} names no default branch: {reason}"))?;
    if !git.remotes()?.iter().any(|remote| remote == REMOTE) {
        return git.commit(&format!("refs/heads/{branch}")).map(Revision);
    }
    let tracking = git.fetch_branch(REMOTE, &branch)?;
    git.commit(&tracking).map(Revision)
}

/// The `[profiles]` table of the repository's declaration at `revision`:
/// each seam it names, none where it names none or there is no
/// declaration (105, C.2, core.3).
pub fn binding_at(repository: &str, revision: &Revision) -> BindingRead {
    let git = Git::at(repository);
    let text = match git.file(&revision.0, DECLARATION) {
        Ok(Some(text)) => text,
        Ok(None) => {
            return BindingRead::Read {
                binding: Binding::default(),
            }
        }
        Err(reason) => return BindingRead::CouldNotRead { reason },
    };
    match profiles(&text) {
        Ok(binding) => BindingRead::Read { binding },
        Err(reason) => BindingRead::CouldNotRead {
            reason: format!("{DECLARATION} at {revision}: {reason}"),
        },
    }
}

/// The binding a declaration's `[profiles]` table names. The table is
/// flume's own, beside the kinds, so it is read whether or not the kinds
/// read.
fn profiles(text: &str) -> Result<Binding, String> {
    let root: Table = text
        .parse()
        .map_err(|error: weir_kinds::toml::de::Error| error.message().trim().to_string())?;
    let Some(table) = root.get("profiles") else {
        return Ok(Binding::default());
    };
    let Some(table) = table.as_table() else {
        return Err("profiles: must be a table naming a profile for each seam".into());
    };
    let mut problems = vec![];
    let mut named = |seam: &str| match table.get(seam) {
        None => None,
        Some(Value::String(name)) if !name.is_empty() => Some(name.clone()),
        Some(_) => {
            problems.push(format!("profiles.{seam}: must be a profile's name"));
            None
        }
    };
    let binding = Binding {
        place: named("place"),
        change: named("change"),
        merge_queue: named("merge_queue"),
    };
    if problems.is_empty() {
        Ok(binding)
    } else {
        Err(problems.join("; "))
    }
}

/// Whose declaration is read: a repository's, which alone gives
/// `[agents]` for every kind, or a launch's; flume's own and a host's
/// source's are packages, read by weir's package reader (21, 32, 157).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Whose {
    Repository,
    Launch,
}

/// Every kind a declaration's text holds, its prompts and its agents
/// tables' settings files read by `beside` from their paths, the settings
/// it gives kinds it does not declare, and, for a repository's, its
/// settings for every kind; or every field that fails it (84, 32, 157).
fn read(text: &str, whose: Whose, beside: Reader) -> KindsRead {
    let declaration = match weir_kinds::read_kinds(text) {
        Ok(declaration) => declaration,
        Err(refusal) => {
            return KindsRead::CouldNotRead {
                reason: refused(&refusal),
            }
        }
    };
    let mut problems = vec![];
    let kinds = declaration
        .kinds
        .values()
        .map(|kind| map_kind(kind, beside, &mut problems))
        .collect();
    let settings = declaration
        .settings
        .values()
        .map(|given| {
            let at = format!("kinds.{}.agents", given.name);
            (
                given.name.clone(),
                map_agents(&given.agents, &at, beside, &mut problems),
            )
        })
        .collect();
    let agents = match whose {
        Whose::Repository => every_kind(&declaration.sections, beside, &mut problems),
        Whose::Launch => Settings::default(),
    };
    if problems.is_empty() {
        KindsRead::Read {
            kinds,
            settings,
            agents: Box::new(agents),
        }
    } else {
        KindsRead::CouldNotRead {
            reason: problems.join("; "),
        }
    }
}

/// A repository's `[agents]`, its settings for every kind it runs, among
/// the declaration's `sections`: read as a kind's agents table is, with a
/// `stages`, a `streams`, an `env` or a `beside` failing it naming the
/// field, since a stage's name is its kind's, a stream and an environment
/// the host's and a pane the launch's (21, 32, 107, 144, 195, C.2).
fn every_kind(sections: &Table, beside: Reader, problems: &mut Vec<String>) -> Settings {
    let Some(agents) = sections.get("agents") else {
        return Settings::default();
    };
    let Some(agents) = agents.as_table() else {
        problems.push("agents: must be a table of settings for every kind".into());
        return Settings::default();
    };
    for (key, words) in [
        (
            "stages",
            "a repository's settings for every kind name no stage, since a stage's name is its kind's: a stage's settings go in [kinds.<kind>.agents.stages.<stage>]",
        ),
        (
            "streams",
            "a repository's settings name no stream, since a stream is the host's: a stream's settings go in the host's [agents.streams.<stream>]",
        ),
        (
            "beside",
            "a repository's settings for every kind name no pane to open beside, which is the launch's to name",
        ),
    ] {
        if agents.contains_key(key) {
            problems.push(format!("agents.{key}: {words}"));
        }
    }
    map_agent(agents, "agents", beside, problems)
}

fn refused(refusal: &Refusal) -> String {
    refusal
        .problems
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("; ")
}

/// The seam's kind of a kind weir-kinds read, every problem it finds
/// pushed onto `problems`. Its label is none: a simple kind's label waits
/// on weir's statement naming the field and weir-kinds reading it, since
/// weir 52 and C.4 name none and weir 94 gives a statechart kind's
/// `label` another meaning, and flume reads no field of the format
/// itself; until then only the host's configuration gives a place a
/// label (180; record 0033; core.4).
fn map_kind(kind: &weir_kinds::Kind, beside: Reader, problems: &mut Vec<String>) -> Kind {
    let at = format!("kinds.{}", kind.name);
    let version = u32::try_from(kind.version).unwrap_or_else(|_| {
        problems.push(format!("{at}.version: is more than flume counts"));
        0
    });
    Kind {
        name: kind.name.clone(),
        version,
        branch_prefix: kind.branch.clone(),
        starts_unready: kind.starts_unready,
        lands: kind.lands,
        agents: map_agents(&kind.agents, &format!("{at}.agents"), beside, problems),
        stages: kind
            .stages
            .iter()
            .enumerate()
            .map(|(n, stage)| map_stage(stage, &format!("{at}.stages[{n}]"), beside, problems))
            .collect(),
        label: None,
    }
}

fn map_stage(
    stage: &weir_kinds::Stage,
    at: &str,
    beside: Reader,
    problems: &mut Vec<String>,
) -> Stage {
    let text = beside(&stage.prompt)
        .map(|(_, text)| text)
        .unwrap_or_else(|reason| {
            problems.push(format!("{at}.prompt: {reason}"));
            String::new()
        });
    let (conversation, deliverables) = match &stage.end {
        End::Conversation => (true, vec![]),
        End::Deliverable(deliverable) => (false, vec![map_deliverable(deliverable)]),
    };
    let send_back_limit = stage.max_send_backs.map(|times| {
        u32::try_from(times).unwrap_or_else(|_| {
            problems.push(format!("{at}.max_send_backs: is more than flume counts"));
            0
        })
    });
    Stage {
        name: stage.name.clone(),
        prompt: text,
        conversation,
        deliverables,
        agents: stage
            .agents
            .iter()
            .enumerate()
            .map(|(n, agent)| map_agent(agent, &format!("{at}.agents[{n}]"), beside, problems))
            .collect(),
        waits: stage.wait,
        quiet_limit: stage.quiet.map(|quiet| quiet.as_secs()),
        send_back_to: stage.on_fail.clone(),
        send_back_limit,
        makes: stage.makes.clone(),
        // weir has refused a repeat on a stage that does not deliver tasks
        // (weir 57); a way of repeating weir names and flume does not run
        // fails here to compile, never to read as not repeating.
        repeats: match stage.repeat {
            None => false,
            Some(weir_kinds::Repeat::PerTaskGroup) => true,
        },
    }
}

fn map_deliverable(deliverable: &weir_kinds::Deliverable) -> Deliverable {
    match deliverable {
        weir_kinds::Deliverable::Commits => Deliverable::Commits,
        weir_kinds::Deliverable::Change => Deliverable::ChangeWritten,
        weir_kinds::Deliverable::Tasks => Deliverable::TasksTicked,
        weir_kinds::Deliverable::Archived => Deliverable::ChangeArchived,
        weir_kinds::Deliverable::Report(path) => Deliverable::Report { path: path.clone() },
        weir_kinds::Deliverable::Committed(path) => {
            Deliverable::CommittedReport { path: path.clone() }
        }
    }
}

/// A kind's `agents` table: the settings for every stage, and for each
/// stage its `stages` table names (32; weir 52, C.4).
fn map_agents(
    agents: &weir_kinds::KindAgents,
    at: &str,
    beside: Reader,
    problems: &mut Vec<String>,
) -> KindSettings {
    KindSettings {
        every_stage: map_agent(&agents.every_stage, at, beside, problems),
        stages: agents
            .stages
            .iter()
            .map(|(stage, table)| {
                let at = format!("{at}.stages.{stage}");
                (stage.clone(), map_agent(table, &at, beside, problems))
            })
            .collect(),
    }
}

/// An agent's settings, its table as written: `program`, `model`,
/// `effort`, `instructions`, `beside`, `mode` and `programs` (32, 107,
/// 162, 195, 197), the effort its program's own word, kept as written
/// for the agent's profile to take or refuse, any other value, a table
/// of programs among it, failing it naming the field (197, C.2), the
/// mode one of flume's four or a table of programs, read as the
/// host's configuration reads it, any other failing it naming the field
/// (169, C.2), and `programs` each program's own settings, read by
/// flume's one reader of them, a file an entry names read by `beside` as
/// a prompt is, so never outside the repository or the package, any part
/// not as it reads them failing it naming the field (157, 195, C.2;
/// core.34); an `env` fails it, naming the field, since an agent's
/// environment is its account's alone (144, C.2), and so does a
/// `keep_open`, since how long an agent is kept open for an answer trades
/// the host's room and its accounts' and is the host's or the launch's to
/// give (158, 198, C.2), and a program named by a location, a `/` in it,
/// since a location is the host's (158); every other key is the runner's,
/// and ignored.
fn map_agent(agent: &Table, at: &str, beside: Reader, problems: &mut Vec<String>) -> Settings {
    let mut text = |name: &str| match agent.get(name) {
        None => None,
        Some(Value::String(value)) => Some(value.clone()),
        Some(_) => {
            problems.push(format!("{at}.{name}: must be a string"));
            None
        }
    };
    let program = text("program");
    let model = text("model");
    let effort = text("effort");
    let instructions = text("instructions");
    let pane = text("beside");
    let mode = agent.get("mode").and_then(|mode| {
        crate::mode::read(mode)
            .map_err(|reason| problems.push(format!("{at}.mode: {reason}")))
            .ok()
    });
    let programs = agent
        .get("programs")
        .and_then(|programs| {
            own::read(programs, at, beside)
                .map_err(|fault| problems.push(fault))
                .ok()
        })
        .unwrap_or_default();
    if agent.contains_key("env") {
        problems.push(format!(
            "{at}.env: an agent's environment is its account's to give, and no declaration gives one"
        ));
    }
    if agent.contains_key("keep_open") {
        problems.push(format!(
            "{at}.keep_open: how long an agent is kept open for an answer is the host's or the launch's to give, and no declaration gives one"
        ));
    }
    if let Some(program) = program.as_deref().filter(|program| program.contains('/')) {
        problems.push(format!(
            "{at}.program: `{program}` names a program by a location, which is the host's, and a declaration names its program by name"
        ));
    }
    Settings {
        program,
        model,
        effort,
        instructions,
        beside: pane,
        mode,
        // A kind names nothing a host owns, a session among it (158).
        session: None,
        programs,
        // Nor how long an agent is kept open for an answer, refused above
        // (158, 198).
        keep_open: None,
    }
}

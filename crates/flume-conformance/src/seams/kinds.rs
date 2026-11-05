//! The kinds seam's scenarios (B.2, 84, 85), and the binding (105); a
//! kind's name, a source's listing, a package and a lock, each read twice
//! to the same answer, and a lock written (154 to 157, 160, 192).

use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;
use std::collections::BTreeMap;

/// What the kinds scenarios need: repositories and declarations holding
/// the kinds a scenario names.
pub trait KindsHarness {
    type Seam: Kinds + ?Sized;

    fn seam(&self) -> &Self::Seam;

    /// A repository declaring `kinds` at a revision.
    fn declaring(&self, kinds: &[Kind]) -> (String, Revision);

    /// A declaration a launch carries holding `kinds`.
    fn launch_declaration(&self, kinds: &[Kind]) -> String;

    /// A repository whose declaration cannot be read at a revision.
    fn unreadable(&self) -> (String, Revision);

    /// A repository binding `binding` at its default branch.
    fn binding(&self, binding: &Binding) -> String;

    /// A source's directory holding each of `kinds` as a package at its
    /// name and version; none where the harness makes no source yet, and
    /// then every scenario of a source passes over it (159).
    fn source(&self, _kinds: &[Kind]) -> Option<SourceDir> {
        None
    }

    /// A source whose directory cannot be listed (192).
    fn unreadable_source(&self) -> Option<SourceDir> {
        None
    }

    /// A source holding a package read-package refuses, with the
    /// package's name and version (157).
    fn refused_package(&self) -> Option<(SourceDir, String, u32)> {
        None
    }

    /// A repository whose lock at a revision records `entries` (20, 155).
    fn lock_at(&self, _entries: &[LockEntry]) -> Option<(String, Revision)> {
        None
    }

    /// A repository at a revision that holds no lock (155).
    fn no_lock_at(&self) -> Option<(String, Revision)> {
        None
    }

    /// A lock on disk recording `entries`, by its path (155).
    fn lock_file(&self, _entries: &[LockEntry]) -> Option<String> {
        None
    }

    /// A path where no lock lies (155).
    fn no_lock_file(&self) -> Option<String> {
        None
    }

    /// A lock on disk that cannot be read (155).
    fn unreadable_lock_file(&self) -> Option<String> {
        None
    }

    /// A lock on disk recording `entries` that a lock written over it
    /// fails to replace (155).
    fn unwritable_lock(&self, _entries: &[LockEntry]) -> Option<String> {
        None
    }
}

/// A source the host declares, by its name and the directory its
/// packages lie in (159, 191).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceDir {
    pub source: String,
    pub kinds_dir: String,
}

/// A kind with one code stage delivering commits, `agents` agents to it.
pub fn bump(agents: usize) -> Kind {
    Kind {
        name: "bump".into(),
        version: 2,
        branch_prefix: "bump".into(),
        starts_unready: false,
        lands: true,
        agents: KindSettings::default(),
        stages: vec![Stage {
            name: "code".into(),
            prompt: "{{brief}}\n\n{{needs}}".into(),
            conversation: false,
            deliverables: vec![Deliverable::Commits],
            agents: vec![
                Settings {
                    program: Some("agent".into()),
                    ..Settings::default()
                };
                agents
            ],
            waits: false,
            quiet_limit: Some(45 * 60),
            send_back_to: Some("code".into()),
            send_back_limit: Some(3),
            makes: vec![],
            repeats: false,
        }],
    }
}

// ---- the kinds seam: a profile unit appends its own scenarios at the list's end ----

pub fn scenarios<H: KindsHarness>() -> Vec<Scenario<H>> {
    vec![
        Scenario {
            name: "kinds: read-kinds gives every kind with its stages",
            cites: &["84", "18", "23"],
            run: read_kinds_gives_each_kind::<H>,
        },
        Scenario {
            name: "kinds: the same revision gives the same kinds, and a launch's declaration reads the same",
            cites: &["85", "19"],
            run: the_same_revision_the_same_kinds::<H>,
        },
        Scenario {
            name: "kinds: a declaration that cannot be read names why",
            cites: &["84"],
            run: an_unreadable_declaration::<H>,
        },
        Scenario {
            name: "kinds: a stage's two agents are kept, so the core can refuse it",
            cites: &["23", "4"],
            run: two_agents_are_kept::<H>,
        },
        Scenario {
            name: "kinds: a stage that declares what it makes is read with its names, one that declares nothing with none",
            cites: &["129", "23"],
            run: what_a_stage_makes::<H>,
        },
        Scenario {
            name: "kinds: flume's own kinds are fix@1, chore@1, change@1 and change@2, each landing and sent back to code, change@2's code stage alone repeating",
            cites: &["22", "19", "S35", "140", "157"],
            run: flumes_own::<H>,
        },
        Scenario {
            name: "kinds: the binding names each seam's profile",
            cites: &["105", "20"],
            run: the_binding::<H>,
        },
        Scenario {
            name: "kinds: read-named gives a kind's source, name and version as written, the same each time, and refuses a text that names no kind",
            cites: &["154"],
            run: read_named::<H>,
        },
        Scenario {
            name: "kinds: flume's own are packages, each at flume/<name>@<version> with its content's hash, the same each time",
            cites: &["156", "157", "22"],
            run: flumes_own_packages::<H>,
        },
        Scenario {
            name: "kinds: read-source gives every package a source's directory holds by identity, the same each time, or why the directory cannot be read",
            cites: &["159", "192"],
            run: read_source::<H>,
        },
        Scenario {
            name: "kinds: read-package gives a package's kind, identity and hash, the same each time, or why it cannot be read",
            cites: &["157", "156", "160"],
            run: read_package::<H>,
        },
        Scenario {
            name: "kinds: the repository's lock at a revision gives its entries, the same each time, and none where the revision holds none",
            cites: &["155", "20", "160"],
            run: read_lock_at::<H>,
        },
        Scenario {
            name: "kinds: a lock on disk gives its entries, the same each time, none where none lies, or why it cannot be read",
            cites: &["155", "160"],
            run: read_lock_file::<H>,
        },
        Scenario {
            name: "kinds: a lock written whole reads back as written, written twice the same, and a write that fails leaves the lock as it was",
            cites: &["155"],
            run: write_lock::<H>,
        },
    ]
}

fn read<H: KindsHarness>(
    harness: &H,
    repository: &str,
    revision: &Revision,
) -> Result<Vec<Kind>, String> {
    match harness.seam().read_kinds(repository, revision) {
        KindsRead::Read { kinds, .. } => Ok(kinds),
        other => Err(format!("read-kinds gave {other:?}")),
    }
}

fn read_kinds_gives_each_kind<H: KindsHarness>(harness: &H) -> Outcome {
    let (repository, revision) = harness.declaring(&[bump(1)]);
    let kinds = read(harness, &repository, &revision)?;
    ensure_eq!(kinds, vec![bump(1)], "the kinds read");
    Ok(())
}

fn the_same_revision_the_same_kinds<H: KindsHarness>(harness: &H) -> Outcome {
    let (repository, revision) = harness.declaring(&[bump(1)]);
    let first = read(harness, &repository, &revision)?;
    let second = read(harness, &repository, &revision)?;
    ensure_eq!(first, second, "a second read of one revision");
    let declaration = harness.launch_declaration(&[bump(1)]);
    ensure_eq!(
        harness.seam().read_declaration(&declaration),
        KindsRead::Read {
            kinds: first,
            settings: BTreeMap::new(),
        },
        "a launch's declaration of the same kind"
    );
    Ok(())
}

fn an_unreadable_declaration<H: KindsHarness>(harness: &H) -> Outcome {
    let (repository, revision) = harness.unreadable();
    match harness.seam().read_kinds(&repository, &revision) {
        KindsRead::CouldNotRead { reason } => {
            ensure!(!reason.is_empty(), "no reason given");
            Ok(())
        }
        other => Err(format!("an unreadable declaration read as {other:?}")),
    }
}

fn two_agents_are_kept<H: KindsHarness>(harness: &H) -> Outcome {
    let (repository, revision) = harness.declaring(&[bump(2)]);
    let kinds = read(harness, &repository, &revision)?;
    ensure_eq!(kinds[0].stages[0].agents.len(), 2, "the stage's agents");
    Ok(())
}

/// A kind whose code stage makes a preview and a spike, and whose verify
/// stage declares nothing it makes (129).
pub fn making() -> Kind {
    let mut kind = bump(1);
    kind.name = "spike".into();
    kind.branch_prefix = "spike".into();
    kind.stages[0].makes = vec!["preview".into(), "spike".into()];
    let mut verify = bump(1).stages.remove(0);
    verify.name = "verify".into();
    verify.send_back_to = None;
    verify.send_back_limit = None;
    kind.stages[0].send_back_to = None;
    kind.stages[0].send_back_limit = None;
    kind.stages.push(verify);
    kind
}

fn what_a_stage_makes<H: KindsHarness>(harness: &H) -> Outcome {
    let (repository, revision) = harness.declaring(&[making()]);
    let kinds = read(harness, &repository, &revision)?;
    ensure_eq!(kinds, vec![making()], "the kinds read");
    ensure_eq!(
        kinds[0].stages[0].makes,
        vec!["preview".to_string(), "spike".to_string()],
        "what the code stage makes"
    );
    ensure!(
        kinds[0].stages[1].makes.is_empty(),
        "a stage declaring nothing reads {:?}",
        kinds[0].stages[1].makes
    );
    Ok(())
}

fn flumes_own<H: KindsHarness>(harness: &H) -> Outcome {
    let own: Vec<Kind> = harness
        .seam()
        .flumes_own()
        .into_iter()
        .map(|own| own.kind)
        .collect();
    let mut names: Vec<String> = own
        .iter()
        .map(|kind| format!("{}@{}", kind.name, kind.version))
        .collect();
    names.sort_unstable();
    ensure_eq!(
        names,
        vec!["change@1", "change@2", "chore@1", "fix@1"],
        "flume's own kinds"
    );
    ensure!(
        own.iter().all(|kind| kind.lands),
        "a kind of flume's own does not land"
    );
    let stages = |kind: &Kind| -> Vec<(String, Vec<Deliverable>, bool)> {
        kind.stages
            .iter()
            .map(|stage| {
                (
                    stage.name.clone(),
                    stage.deliverables.clone(),
                    stage.repeats,
                )
            })
            .collect()
    };
    for kind in &own {
        let unready = kind.name != "change";
        ensure_eq!(kind.branch_prefix, kind.name, "the branch prefix");
        ensure_eq!(kind.starts_unready, unready, "whether it starts unready");
        ensure_eq!(
            kind.stages
                .last()
                .and_then(|stage| stage.send_back_to.clone()),
            Some("code".to_string()),
            "where it goes back, never verify (22, S35)"
        );
        let due = if kind.name == "change" {
            vec![
                (
                    "construct".to_string(),
                    vec![Deliverable::ChangeWritten],
                    false,
                ),
                (
                    "code".to_string(),
                    vec![Deliverable::TasksTicked],
                    kind.version == 2,
                ),
                (
                    "verify".to_string(),
                    vec![Deliverable::ChangeArchived],
                    false,
                ),
            ]
        } else {
            vec![("code".to_string(), vec![Deliverable::Commits], false)]
        };
        ensure_eq!(
            stages(kind),
            due,
            format!(
                "{}@{}'s stages, change@2's code alone repeating",
                kind.name, kind.version
            )
        );
    }
    Ok(())
}

fn the_binding<H: KindsHarness>(harness: &H) -> Outcome {
    let binding = Binding {
        place: Some("trees".into()),
        change: None,
        merge_queue: Some("queue".into()),
    };
    let repository = harness.binding(&binding);
    ensure_eq!(
        harness.seam().read_binding(&repository),
        BindingRead::Read { binding },
        "the binding read"
    );
    Ok(())
}

/// The identity `<source>/<name>@<version>`, as weir's reader spells it.
pub fn identity_of(source: &str, name: &str, version: u32) -> KindIdentity {
    KindIdentity {
        source: source.into(),
        name: name.into(),
        version,
        spelled: format!("{source}/{name}@{version}"),
    }
}

/// A content hash of 64 hex digits made of `digit`, which no package
/// need have.
pub fn hash_of(digit: char) -> String {
    format!("sha256:{}", digit.to_string().repeat(64))
}

/// A lock's entry of `<source>/<name>@<version>` at `hash`.
pub fn entry(source: &str, name: &str, version: u32, hash: &str) -> LockEntry {
    LockEntry {
        kind: identity_of(source, name, version),
        hash: hash.into(),
    }
}

/// `bump` at `version`.
fn bump_at(version: u32) -> Kind {
    Kind { version, ..bump(1) }
}

fn read_named<H: KindsHarness>(harness: &H) -> Outcome {
    let seam = harness.seam();
    for (text, source, name, version) in [
        ("review", None, "review", None),
        ("review@2", None, "review", Some(2)),
        ("acme/review", Some("acme"), "review", None),
        ("acme/review@2", Some("acme"), "review", Some(2)),
    ] {
        let first = seam.read_named(text);
        ensure_eq!(
            first,
            Ok(KindNamed {
                source: source.map(str::to_string),
                name: name.into(),
                version,
                spelled: text.into(),
            }),
            format!("`{text}` read")
        );
        ensure_eq!(seam.read_named(text), first, format!("`{text}` read again"));
    }
    for text in ["acme/", "review@x", "/review"] {
        let first = seam.read_named(text);
        match &first {
            Err(reason) => ensure!(!reason.is_empty(), "`{text}` refused with no reason"),
            Ok(named) => return Err(format!("`{text}` read as {named:?}")),
        }
        ensure_eq!(
            seam.read_named(text),
            first,
            format!("`{text}` refused again")
        );
    }
    Ok(())
}

fn flumes_own_packages<H: KindsHarness>(harness: &H) -> Outcome {
    let own = harness.seam().flumes_own();
    for package in &own {
        let kind = &package.kind;
        ensure_eq!(
            package.identity,
            identity_of("flume", &kind.name, kind.version),
            "a package of flume's own at its kind's name and version"
        );
        ensure!(
            package.hash.starts_with("sha256:"),
            "{}'s hash {}",
            package.identity,
            package.hash
        );
    }
    ensure_eq!(harness.seam().flumes_own(), own, "flume's own read again");
    Ok(())
}

fn read_source<H: KindsHarness>(harness: &H) -> Outcome {
    if let Some(dir) = harness.source(&[bump_at(1), bump_at(2)]) {
        let first = harness.seam().read_source(&dir.source, &dir.kinds_dir);
        ensure_eq!(
            first,
            SourceRead::Read {
                kinds: vec![
                    identity_of(&dir.source, "bump", 1),
                    identity_of(&dir.source, "bump", 2),
                ],
            },
            "the source's listing"
        );
        ensure_eq!(
            harness.seam().read_source(&dir.source, &dir.kinds_dir),
            first,
            "the listing read again"
        );
    }
    if let Some(dir) = harness.unreadable_source() {
        let first = harness.seam().read_source(&dir.source, &dir.kinds_dir);
        match &first {
            SourceRead::CouldNotRead { reason } => ensure!(!reason.is_empty(), "no reason given"),
            read => return Err(format!("an unreadable source read as {read:?}")),
        }
        ensure_eq!(
            harness.seam().read_source(&dir.source, &dir.kinds_dir),
            first,
            "the unreadable source read again"
        );
    }
    Ok(())
}

fn read_package<H: KindsHarness>(harness: &H) -> Outcome {
    if let Some(dir) = harness.source(&[bump_at(1), bump_at(2)]) {
        let read = || {
            harness
                .seam()
                .read_package(&dir.source, &dir.kinds_dir, "bump", 2)
        };
        let first = read();
        let PackageRead::Read { package } = &first else {
            return Err(format!("bump@2 read as {first:?}"));
        };
        ensure_eq!(package.kind, bump_at(2), "the package's kind");
        ensure_eq!(
            package.identity,
            identity_of(&dir.source, "bump", 2),
            "the identity it lies at"
        );
        ensure!(
            package.hash.starts_with("sha256:"),
            "the package's hash {}",
            package.hash
        );
        ensure_eq!(read(), first, "bump@2 read again");
        match harness
            .seam()
            .read_package(&dir.source, &dir.kinds_dir, "bump", 3)
        {
            PackageRead::CouldNotRead { reason } => ensure!(!reason.is_empty(), "no reason given"),
            read => {
                return Err(format!(
                    "a package the source does not hold read as {read:?}"
                ))
            }
        }
        ensure_eq!(
            harness.seam().read_source(&dir.source, &dir.kinds_dir),
            SourceRead::Read {
                kinds: vec![
                    identity_of(&dir.source, "bump", 1),
                    identity_of(&dir.source, "bump", 2),
                ],
            },
            "the listing after the packages were read"
        );
    }
    if let Some((dir, name, version)) = harness.refused_package() {
        let read = || {
            harness
                .seam()
                .read_package(&dir.source, &dir.kinds_dir, &name, version)
        };
        let first = read();
        match &first {
            PackageRead::CouldNotRead { reason } => ensure!(!reason.is_empty(), "no reason given"),
            read => return Err(format!("a refused package read as {read:?}")),
        }
        ensure_eq!(read(), first, "the refused package read again");
    }
    Ok(())
}

/// Two versions of one kind, locked side by side.
fn two_entries() -> Vec<LockEntry> {
    vec![
        entry("acme", "review", 1, &hash_of('1')),
        entry("acme", "review", 2, &hash_of('2')),
    ]
}

fn read_lock_at<H: KindsHarness>(harness: &H) -> Outcome {
    if let Some((repository, revision)) = harness.lock_at(&two_entries()) {
        let first = harness.seam().read_lock_at(&repository, &revision);
        ensure_eq!(
            first,
            LockRead::Read {
                entries: two_entries(),
            },
            "the lock at the revision"
        );
        ensure_eq!(
            harness.seam().read_lock_at(&repository, &revision),
            first,
            "the lock read again"
        );
    }
    if let Some((repository, revision)) = harness.no_lock_at() {
        for read in ["first", "second"] {
            ensure_eq!(
                harness.seam().read_lock_at(&repository, &revision),
                LockRead::None,
                format!("the {read} read of a revision with no lock")
            );
        }
    }
    Ok(())
}

fn read_lock_file<H: KindsHarness>(harness: &H) -> Outcome {
    if let Some(path) = harness.lock_file(&two_entries()) {
        let first = harness.seam().read_lock_file(&path);
        ensure_eq!(
            first,
            LockRead::Read {
                entries: two_entries(),
            },
            "the lock on disk"
        );
        ensure_eq!(
            harness.seam().read_lock_file(&path),
            first,
            "the lock read again"
        );
    }
    if let Some(path) = harness.no_lock_file() {
        for read in ["first", "second"] {
            ensure_eq!(
                harness.seam().read_lock_file(&path),
                LockRead::None,
                format!("the {read} read where no lock lies")
            );
        }
    }
    if let Some(path) = harness.unreadable_lock_file() {
        let first = harness.seam().read_lock_file(&path);
        match &first {
            LockRead::CouldNotRead { reason } => ensure!(!reason.is_empty(), "no reason given"),
            read => return Err(format!("an unreadable lock read as {read:?}")),
        }
        ensure_eq!(
            harness.seam().read_lock_file(&path),
            first,
            "the unreadable lock read again"
        );
    }
    Ok(())
}

fn write_lock<H: KindsHarness>(harness: &H) -> Outcome {
    if let Some(path) = harness.no_lock_file() {
        // Written out of identity order, it reads back in it.
        let mut given = two_entries();
        given.reverse();
        for write in ["first", "second"] {
            harness
                .seam()
                .write_lock(&path, &given)
                .map_err(|reason| format!("the {write} write failed: {reason}"))?;
            ensure_eq!(
                harness.seam().read_lock_file(&path),
                LockRead::Read {
                    entries: two_entries(),
                },
                format!("the lock after the {write} write")
            );
        }
        harness
            .seam()
            .write_lock(&path, &two_entries()[..1])
            .map_err(|reason| format!("the write of one entry failed: {reason}"))?;
        ensure_eq!(
            harness.seam().read_lock_file(&path),
            LockRead::Read {
                entries: two_entries()[..1].to_vec(),
            },
            "the lock written whole, its other entry gone"
        );
    }
    if let Some(path) = harness.unwritable_lock(&two_entries()) {
        match harness.seam().write_lock(&path, &[]) {
            Err(reason) => ensure!(!reason.is_empty(), "no reason given"),
            Ok(()) => return Err("a write that fails answered written".into()),
        }
        ensure_eq!(
            harness.seam().read_lock_file(&path),
            LockRead::Read {
                entries: two_entries(),
            },
            "the lock after a write that failed"
        );
    }
    Ok(())
}

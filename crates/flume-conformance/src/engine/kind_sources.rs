//! Kinds from a host's sources (4, 19 to 22, 154 to 160, 191, 192): a
//! kind named by its identity and read through the kinds seam, a source
//! the host declares searched alone where it is named, a bare name never
//! reaching one, the versions side by side, and nothing from outside run
//! but as a lock records its content; a source's fault refuses only the
//! launches naming it, and a place keeps the kind its launch read; and
//! `flume kinds lock` recording a kind's content, or removing it (74, 155).

use super::world::*;
use crate::seams::kinds::{entry, hash_of};
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_fakes::{package_dir, source_file, HOST_LOCK, SOURCES};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "kind sources: a source's file with no kinds_dir refuses only the launches naming its source, naming the file and kinds_dir, and reads none of its packages",
            cites: &["S96", "4", "159", "191"],
            run: a_file_with_no_kinds_dir,
        },
        Scenario {
            name: "kind sources: a source declared under flume's own source's name leaves flume's own kinds as flume holds them",
            cites: &["154", "191"],
            run: a_file_taking_flumes_name,
        },
        Scenario {
            name: "kind sources: a kinds_dir not yet written refuses a launch naming its source, naming the directory as missing, nothing made",
            cites: &["4", "159", "191"],
            run: a_kinds_dir_not_yet_written,
        },
        Scenario {
            name: "kind sources: two versions side by side each run their own stages and prompts, status naming each identity with its own hash",
            cites: &["154", "159"],
            run: two_versions_side_by_side,
        },
        Scenario {
            name: "kind sources: a part outside its package and an environment each refuse the launch naming the field, nothing made",
            cites: &["S63", "157", "158"],
            run: a_part_outside_and_an_environment,
        },
        Scenario {
            name: "kind sources: a package whose kind is not the identity it lies at refuses the launch naming both versions, nothing made",
            cites: &["157"],
            run: a_package_at_another_identity,
        },
        Scenario {
            name: "kind sources: a source the host does not declare refuses the launch naming it and the sources the host declares, nothing made",
            cites: &["4", "154", "191"],
            run: a_source_the_host_does_not_declare,
        },
        Scenario {
            name: "kind sources: a version the source does not hold refuses the launch naming it and the versions held, nothing made",
            cites: &["4", "154"],
            run: a_version_the_source_does_not_hold,
        },
        Scenario {
            name: "kind sources: the repository's and the launch's kinds named by their sources, a version not held and a declaration not carried refused",
            cites: &["4", "154", "19"],
            run: the_repositorys_and_the_launchs_by_source,
        },
        Scenario {
            name: "kind sources: a host that declares no source runs flume's own newest fix, fix@2, for a bare fix",
            cites: &["154", "191"],
            run: no_sources_directory,
        },
        Scenario {
            name: "kind sources: a source holding fix leaves a bare fix flume's own newest, flume/fix@2, and acme/fix runs the source's acme/fix@1",
            cites: &["S97", "154", "191"],
            run: s97_a_source_that_holds_fix,
        },
        Scenario {
            name: "kind sources: a name only the host's sources hold refuses a bare launch naming each identity, and runs named by its source",
            cites: &["S62", "154", "19"],
            run: s62_a_name_only_the_hosts_sources_hold,
        },
        Scenario {
            name: "kind sources: a kind of a host's source no lock records is refused naming both ways to lock it, and runs once the repository's lock on the base records it",
            cites: &["S60", "4", "155", "159"],
            run: s60_unlocked_then_locked_by_the_repository,
        },
        Scenario {
            name: "kind sources: a prompt edited under a launched place refuses the next launch naming both hashes, and the place runs on with the prompt it launched with",
            cites: &["S61", "21", "156"],
            run: s61_an_edited_prompt,
        },
        Scenario {
            name: "kind sources: two locks giving one identity two hashes refuse the launch naming both locks and both hashes, nothing made",
            cites: &["155"],
            run: two_locks_that_disagree,
        },
        Scenario {
            name: "kind sources: a host's lock that cannot be read refuses the launches of flume's own and of a host's source, and not the repository's",
            cites: &["155"],
            run: a_lock_that_cannot_be_read,
        },
        Scenario {
            name: "kind sources: a repository's lock of flume/change@1 runs change@1 for a bare change, and change@2 named runs with its hash and no lock",
            cites: &["S59", "154", "155", "22"],
            run: s59_the_repository_locks_change_1,
        },
        Scenario {
            name: "kind sources: a version installed and not yet locked leaves a launch naming no version the newest the host's lock records",
            cites: &["154", "155"],
            run: a_version_not_yet_locked,
        },
        Scenario {
            name: "kind sources: a launch naming no version, no lock recording any, is refused naming the newest the source holds as recorded in no lock",
            cites: &["154", "155"],
            run: the_newest_held_unlocked,
        },
        Scenario {
            name: "kind sources: a lock recording flume's own at another hash refuses the launch naming both hashes, nothing made",
            cites: &["156", "155"],
            run: a_lock_of_flumes_own_at_another_hash,
        },
        Scenario {
            name: "kind sources: a source's file renamed over and its lock's entry removed under a running place leave the place its kind and where it came from",
            cites: &["S95", "21", "191"],
            run: s95_the_source_moved_under_a_place,
        },
        Scenario {
            name: "kind sources: each kind's text is read whole by the kinds seam, and each place's identity is the seam's spelling",
            cites: &["154", "19"],
            run: a_kinds_text_read_by_the_seam,
        },
        Scenario {
            name: "kind sources: a source scripted on the host is read through the kinds seam, and a faulty one refused with none of its packages read",
            cites: &["4", "159", "191"],
            run: sources_scripted_on_the_host,
        },
        Scenario {
            name: "kind sources: a package scripted at another hash after a launch refuses the next launch, the first place's kind unchanged, one read of the package per launch",
            cites: &["21", "156"],
            run: a_package_edited_after_a_launch,
        },
        Scenario {
            name: "kind sources: flume kinds lock records a host's source's kind and flume's own, each at its content's hash, in the host's lock and a working copy's, answering the lock and each entry",
            cites: &["155", "156", "74"],
            run: kinds_locked,
        },
        Scenario {
            name: "kind sources: flume kinds lock --hash naming a hash the content does not have is refused naming both, the host's lock as it was",
            cites: &["155", "156"],
            run: a_hash_the_content_does_not_have,
        },
        Scenario {
            name: "kind sources: flume kinds lock of the repository's kind is refused as needing no lock, and of a kind named without its version as not whole, the host's lock as it was",
            cites: &["155"],
            run: an_identity_no_lock_records,
        },
        Scenario {
            name: "kind sources: flume kinds lock --remove takes an entry out, reading no package, and run again changes nothing",
            cites: &["155", "74"],
            run: a_removal_run_twice,
        },
        Scenario {
            name: "kind sources: flume kinds lock never writes over a lock that cannot be read",
            cites: &["155", "74"],
            run: a_lock_kinds_lock_cannot_read,
        },
    ]
}

/// `flume kinds lock` of `kinds` in the host's lock, removing where
/// `remove` says, with `hash` where one is given.
fn lock_host(
    world: &World,
    kinds: &[&str],
    remove: bool,
    hash: Option<&str>,
) -> flume_core::Answer {
    world.engine.lock_kinds(&flume_core::KindsLock {
        of: flume_core::LockOf::Host,
        kinds: kinds.iter().map(|kind| kind.to_string()).collect(),
        remove,
        hash: hash.map(str::to_string),
    })
}

/// The entries the host's lock records on the fake.
fn host_entries(world: &World) -> Vec<LockEntry> {
    match world.kinds.read_lock_file(HOST_LOCK) {
        LockRead::Read { entries } => entries,
        _ => vec![],
    }
}

fn kinds_locked(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    let own = own_hash(&world, "change", 1)?;
    let answer = lock_host(&world, &["acme/review@1", "flume/change@1"], false, None);
    ensure_eq!(
        answer,
        flume_core::Answer::Locked {
            lock_file: HOST_LOCK.into(),
            recorded: vec![
                flume_core::LockedKind {
                    kind: "acme/review@1".into(),
                    hash: hash_of('1'),
                },
                flume_core::LockedKind {
                    kind: "flume/change@1".into(),
                    hash: own.clone(),
                },
            ],
            removed: vec![],
        },
        "the answer"
    );
    ensure_eq!(
        host_entries(&world),
        vec![
            entry("acme", "review", 1, &hash_of('1')),
            entry("flume", "change", 1, &own),
        ],
        "the host's lock"
    );
    let writes = world.kinds.log.count("write_lock");
    lock_host(&world, &["acme/review@1"], false, None);
    ensure_eq!(
        world.kinds.log.count("write_lock"),
        writes,
        "writes of a lock run again"
    );
    let copy = world.engine.lock_kinds(&flume_core::KindsLock {
        of: flume_core::LockOf::WorkingCopy {
            dir: "/work/copy".into(),
        },
        kinds: vec!["acme/review@1".into()],
        remove: false,
        hash: Some(hash_of('1')),
    });
    ensure!(
        matches!(&copy, flume_core::Answer::Locked { lock_file, .. } if lock_file == "/work/copy/.config/flume.lock"),
        "the working copy's lock: {copy:?}"
    );
    ensure_eq!(
        world.kinds.read_lock_file("/work/copy/.config/flume.lock"),
        LockRead::Read {
            entries: vec![entry("acme", "review", 1, &hash_of('1'))]
        },
        "the working copy's lock"
    );
    Ok(())
}

fn a_hash_the_content_does_not_have(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    host_locks(&world, vec![entry("acme", "lint", 1, &hash_of('2'))]);
    let reason = refused(&lock_host(
        &world,
        &["acme/review@1"],
        false,
        Some("sha256:00"),
    ))?;
    ensure!(
        reason.contains("sha256:00") && reason.contains(&hash_of('1')),
        "the refusal names not both hashes: {reason}"
    );
    ensure_eq!(world.kinds.log.count("write_lock"), 0, "locks written");
    ensure_eq!(
        host_entries(&world),
        vec![entry("acme", "lint", 1, &hash_of('2'))],
        "the host's lock"
    );
    Ok(())
}

fn an_identity_no_lock_records(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    let repository = refused(&lock_host(&world, &["repo/bump@1"], false, None))?;
    ensure!(
        repository.contains("repo/bump@1") && repository.contains("the repository's"),
        "the refusal names not the repository's kinds as needing no lock: {repository}"
    );
    let bare = refused(&lock_host(&world, &["acme/review"], false, None))?;
    ensure!(
        bare.contains("acme/review") && bare.contains("<source>/<name>@<version>"),
        "the refusal names not the whole identity as required: {bare}"
    );
    ensure_eq!(world.kinds.log.count("write_lock"), 0, "locks written");
    ensure_eq!(host_entries(&world), vec![], "the host's lock");
    Ok(())
}

fn a_removal_run_twice(_: &()) -> Outcome {
    let world = World::new();
    host_locks(
        &world,
        vec![
            entry("acme", "review", 1, &hash_of('1')),
            entry("acme", "lint", 1, &hash_of('2')),
        ],
    );
    let first = lock_host(&world, &["acme/lint@1"], true, None);
    ensure_eq!(
        first,
        flume_core::Answer::Locked {
            lock_file: HOST_LOCK.into(),
            recorded: vec![],
            removed: vec!["acme/lint@1".into()],
        },
        "the first removal"
    );
    ensure_eq!(
        host_entries(&world),
        vec![entry("acme", "review", 1, &hash_of('1'))],
        "the host's lock after the first removal"
    );
    let second = lock_host(&world, &["acme/lint@1"], true, None);
    ensure_eq!(
        second,
        flume_core::Answer::Locked {
            lock_file: HOST_LOCK.into(),
            recorded: vec![],
            removed: vec![],
        },
        "the second removal"
    );
    ensure_eq!(
        host_entries(&world),
        vec![entry("acme", "review", 1, &hash_of('1'))],
        "the host's lock after the second removal"
    );
    ensure_eq!(world.kinds.log.count("write_lock"), 1, "locks written");
    ensure_eq!(
        world.kinds.log.count("read_package") + world.kinds.log.count("read_source"),
        0,
        "packages read"
    );
    Ok(())
}

fn a_lock_kinds_lock_cannot_read(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    world
        .kinds
        .unreadable_lock_file(HOST_LOCK, "expected `=` at line 1");
    let reason = refused(&lock_host(&world, &["acme/review@1"], false, None))?;
    ensure!(
        reason.contains(HOST_LOCK) && reason.contains("expected `=`"),
        "the refusal names not the lock and its fault: {reason}"
    );
    ensure_eq!(world.kinds.log.count("write_lock"), 0, "locks written");
    Ok(())
}

/// The directories the host's sources acme and beta keep their packages
/// in.
const ACME: &str = "/kinds/acme";
const BETA: &str = "/kinds/beta";

/// A kind of a host's source, `name@version`, of two stages delivering
/// commits, look then fix, each prompt naming its version.
pub fn packaged(name: &str, version: u32) -> Kind {
    let mut packaged = kind(
        name,
        false,
        vec![
            stage(
                "look",
                Some(Deliverable::Commits),
                &format!("look as {name}@{version} looks at {{{{brief}}}}"),
            ),
            stage(
                "fix",
                Some(Deliverable::Commits),
                &format!("fix as {name}@{version} fixes"),
            ),
        ],
    );
    packaged.version = version;
    packaged
}

/// The host declares `source`, its packages in `dir`, holding each
/// `(name, version, digit)` at the hash of that digit.
pub fn holds(world: &World, source: &str, dir: &str, packages: &[(&str, u32, char)]) {
    world.host.set_source(source, dir);
    for (name, version, digit) in packages {
        world
            .kinds
            .set_package(dir, packaged(name, *version), &hash_of(*digit));
    }
}

/// The host's lock records `entries`.
pub fn host_locks(world: &World, entries: Vec<LockEntry>) {
    world.kinds.set_lock_file(HOST_LOCK, entries);
}

/// The launch of `kind` named `name`, with a brief.
fn launching(world: &World, kind: &str, name: &str) -> flume_core::Answer {
    world.engine.launch(&launch(kind, name, Some("the work")))
}

/// The launch refused, and nothing it made: no make, no held state.
fn refused_whole(world: &World, kind: &str, name: &str) -> Result<String, String> {
    let makes = world.place.log.count("make");
    let places = world.store.places().len();
    let reason = refused(&launching(world, kind, name))?;
    ensure_eq!(world.place.log.count("make"), makes, "makes");
    ensure_eq!(world.store.places().len(), places, "places held");
    Ok(reason)
}

/// The place's kind's identity and where it came from, as status gives
/// them.
fn kind_of(world: &World, kind: &str, name: &str) -> Result<(String, KindFrom), String> {
    let status = world.status(&World::id(kind, name))?;
    Ok((
        status.identity.ok_or("status names no identity")?,
        status.kind_from.ok_or("status names no kind_from")?,
    ))
}

fn a_file_with_no_kinds_dir(_: &()) -> Outcome {
    let world = World::new();
    world.host.faulty_source(
        "bad",
        "it gives no kinds_dir, the directory of its packages",
    );
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    host_locks(&world, vec![entry("acme", "review", 1, &hash_of('1'))]);
    let reason = refused_whole(&world, "bad/x@1", "one")?;
    ensure!(
        reason.contains(&source_file("bad")) && reason.contains("kinds_dir"),
        "the refusal names not the file and its fault: {reason}"
    );
    ensure_eq!(
        world.kinds.log.count("read_source") + world.kinds.log.count("read_package"),
        0,
        "the faulty source's packages read"
    );
    launched(&launching(&world, "fix", "one"))?;
    launched(&launching(&world, "acme/review@1", "one"))?;
    Ok(())
}

fn a_file_taking_flumes_name(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "flume", "/kinds/flume", &[("change", 9, '9')]);
    let place = launched(&launching(&world, "flume/change@2", "plan"))?;
    ensure_eq!(
        (place.declared, place.version),
        (Declared::Flume, 2),
        "the kind run"
    );
    ensure_eq!(
        place.identity.as_deref(),
        Some("flume/change@2"),
        "the identity"
    );
    let reason = refused_whole(&world, "flume/change@9", "other")?;
    ensure!(
        reason.contains("flume/change@9") && reason.contains("versions 1, 2"),
        "the refusal names not the versions flume holds: {reason}"
    );
    ensure_eq!(world.kinds.log.count("read_source"), 0, "listings read");
    Ok(())
}

fn a_kinds_dir_not_yet_written(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_source("acme", ACME);
    let reason = refused_whole(&world, "acme/review@1", "one")?;
    ensure!(
        reason.contains(ACME) && reason.contains("does not exist"),
        "the refusal names not the missing directory: {reason}"
    );
    Ok(())
}

fn two_versions_side_by_side(_: &()) -> Outcome {
    let world = World::new();
    holds(
        &world,
        "acme",
        ACME,
        &[("review", 1, '1'), ("review", 2, '2')],
    );
    host_locks(
        &world,
        vec![
            entry("acme", "review", 1, &hash_of('1')),
            entry("acme", "review", 2, &hash_of('2')),
        ],
    );
    launched(&launching(&world, "acme/review@1", "one"))?;
    launched(&launching(&world, "acme/review@2", "two"))?;
    for (name, version, digit) in [("one", 1, '1'), ("two", 2, '2')] {
        let id = World::id("review", name);
        let held = world.held(&id).ok_or("no place")?;
        ensure_eq!(held.place.kind, packaged("review", version), "the kind run");
        ensure_eq!(held.place.declared, Declared::Host, "the declaration");
        world.settle(&id);
        let prompt = world
            .prompt(&id, &World::identity("review", name, "look", 1))
            .ok_or("no prompt")?;
        ensure!(
            prompt.contains(&format!("review@{version} looks")),
            "the prompt is not its version's: {prompt}"
        );
        let (identity, from) = kind_of(&world, "review", name)?;
        ensure_eq!(identity, format!("acme/review@{version}"), "the identity");
        ensure_eq!(from.hash, Some(hash_of(digit)), "the hash");
        ensure_eq!(
            from.kind_dir,
            Some(package_dir(ACME, "review", version)),
            "the kind's directory"
        );
        ensure_eq!(
            from.lock_file.as_deref(),
            Some(HOST_LOCK),
            "the lock that records it"
        );
    }
    Ok(())
}

fn a_part_outside_and_an_environment(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_source("acme", ACME);
    world.kinds.refuse_package(
        ACME,
        "review",
        1,
        "the package acme/review@1: kinds.review.stages.look.standing: ../shared/standing.md lies outside the package",
    );
    world.kinds.refuse_package(
        ACME,
        "lint",
        1,
        "the package acme/lint@1: kinds.lint.agents.env: a kind gives no environment",
    );
    host_locks(
        &world,
        vec![
            entry("acme", "review", 1, &hash_of('1')),
            entry("acme", "lint", 1, &hash_of('2')),
        ],
    );
    let outside = refused_whole(&world, "acme/review@1", "one")?;
    ensure!(
        outside.contains("acme/review@1") && outside.contains("../shared/standing.md"),
        "the refusal names not the path outside: {outside}"
    );
    let env = refused_whole(&world, "acme/lint@1", "one")?;
    ensure!(
        env.contains("acme/lint@1") && env.contains("agents.env"),
        "the refusal names not the field: {env}"
    );
    Ok(())
}

fn a_package_at_another_identity(_: &()) -> Outcome {
    let world = World::new();
    world.host.set_source("acme", ACME);
    world.kinds.refuse_package(
        ACME,
        "review",
        2,
        "the package acme/review@2: kinds.review.version: 1, where the package lies at version 2",
    );
    host_locks(&world, vec![entry("acme", "review", 2, &hash_of('2'))]);
    let reason = refused_whole(&world, "acme/review@2", "one")?;
    ensure!(
        reason.contains("acme/review@2") && reason.contains("version: 1"),
        "the refusal names not both versions: {reason}"
    );
    Ok(())
}

fn a_source_the_host_does_not_declare(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "crew", "/kinds/crew", &[("turn", 1, '1')]);
    let reason = refused_whole(&world, "acme/review@2", "one")?;
    ensure!(
        reason.contains("the source acme")
            && reason.contains("does not declare")
            && reason.contains(&format!("{SOURCES}/acme.toml"))
            && reason.contains("crew"),
        "the refusal names not the source and those declared: {reason}"
    );
    ensure_eq!(world.kinds.log.count("read_source"), 0, "listings read");
    Ok(())
}

fn a_version_the_source_does_not_hold(_: &()) -> Outcome {
    let world = World::new();
    holds(
        &world,
        "acme",
        ACME,
        &[("review", 1, '1'), ("review", 2, '2')],
    );
    let reason = refused_whole(&world, "acme/review@3", "one")?;
    ensure!(
        reason.contains("acme/review@3")
            && reason.contains("acme/review@1 and acme/review@2")
            && reason.contains(ACME),
        "the refusal names not the version and those held: {reason}"
    );
    ensure_eq!(world.kinds.log.count("read_package"), 0, "packages read");
    Ok(())
}

fn the_repositorys_and_the_launchs_by_source(_: &()) -> Outcome {
    let world = World::new();
    let mut bump = kind(
        "bump",
        false,
        vec![stage("code", Some(Deliverable::Commits), "{{brief}}")],
    );
    bump.version = 2;
    world.kinds.set_kinds(REPOSITORY, vec![bump.clone()]);
    let place = launched(&launching(&world, "repo/bump", "one"))?;
    ensure_eq!(
        (place.declared, place.version),
        (Declared::Repository, 2),
        "the kind run"
    );
    let (identity, from) = kind_of(&world, "bump", "one")?;
    ensure_eq!(identity, "repo/bump@2", "the identity");
    ensure_eq!(
        from.declaration_file.as_deref(),
        Some(".config/flume.toml"),
        "the declaration's file"
    );
    ensure!(
        from.revision.is_some() && from.hash.is_none() && from.lock_file.is_none(),
        "where the repository's kind came from: {from:?}"
    );
    let version = refused_whole(&world, "repo/bump@1", "two")?;
    ensure!(
        version.contains("repo/bump@1") && version.contains("version 2"),
        "the refusal names not the version: {version}"
    );
    let carried = refused_whole(&world, "launch/bump", "three")?;
    ensure!(
        carried.contains("launch/bump") && carried.contains("carries none"),
        "the refusal names not the declaration not carried: {carried}"
    );
    ensure_eq!(
        world.kinds.log.count("read_lock_at") + world.kinds.log.count("read_lock_file"),
        0,
        "locks read"
    );
    Ok(())
}

fn no_sources_directory(_: &()) -> Outcome {
    let world = World::with_every_own_version();
    ensure!(
        world.host.sources().is_empty(),
        "the host declares a source"
    );
    let own = launched(&launching(&world, "fix", "one"))?;
    ensure_eq!(own.declared, Declared::Flume, "a bare fix's declaration");
    ensure_eq!(own.identity.as_deref(), Some("flume/fix@2"), "a bare fix");
    Ok(())
}

fn s97_a_source_that_holds_fix(_: &()) -> Outcome {
    let world = World::with_every_own_version();
    holds(&world, "acme", ACME, &[("fix", 1, '7')]);
    host_locks(&world, vec![entry("acme", "fix", 1, &hash_of('7'))]);
    let own = launched(&launching(&world, "fix", "one"))?;
    ensure_eq!(own.declared, Declared::Flume, "a bare fix's declaration");
    ensure_eq!(own.identity.as_deref(), Some("flume/fix@2"), "a bare fix");
    let theirs = launched(&launching(&world, "acme/fix", "two"))?;
    ensure_eq!(theirs.declared, Declared::Host, "acme's fix's declaration");
    ensure_eq!(theirs.identity.as_deref(), Some("acme/fix@1"), "acme's fix");
    Ok(())
}

fn s62_a_name_only_the_hosts_sources_hold(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    holds(&world, "beta", BETA, &[("review", 3, '3')]);
    host_locks(
        &world,
        vec![
            entry("acme", "review", 1, &hash_of('1')),
            entry("beta", "review", 3, &hash_of('3')),
        ],
    );
    let reason = refused_whole(&world, "review", "one")?;
    ensure!(
        reason.contains("acme/review@1 and beta/review@3") && reason.contains("by its source"),
        "the refusal names not the host's identities: {reason}"
    );
    let place = launched(&launching(&world, "beta/review", "one"))?;
    ensure_eq!(
        place.identity.as_deref(),
        Some("beta/review@3"),
        "the kind run"
    );
    Ok(())
}

fn s60_unlocked_then_locked_by_the_repository(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    let reason = refused_whole(&world, "acme/review", "one")?;
    ensure!(
        reason.contains("acme/review@1")
            && reason.contains("recorded in no lock")
            && reason.contains("`flume kinds lock acme/review@1`")
            && reason.contains("`flume kinds lock --host acme/review@1`"),
        "the refusal names not the identity and both ways to lock it: {reason}"
    );
    let bare = refused_whole(&world, "review", "one")?;
    ensure!(
        bare.contains("acme/review@1"),
        "the bare launch's refusal names not the host's: {bare}"
    );
    let locked = world.commit("main", &[".config/flume.lock"]);
    world.kinds.set_lock_at(
        REPOSITORY,
        &locked,
        vec![entry("acme", "review", 1, &hash_of('1'))],
    );
    let place = launched(&launching(&world, "acme/review", "one"))?;
    ensure_eq!(
        place.identity.as_deref(),
        Some("acme/review@1"),
        "the kind run"
    );
    let from = place.kind_from.ok_or("no kind_from")?;
    ensure_eq!(from.hash, Some(hash_of('1')), "the hash");
    ensure_eq!(
        from.lock_file.as_deref(),
        Some(".config/flume.lock"),
        "the lock"
    );
    Ok(())
}

fn s61_an_edited_prompt(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    host_locks(&world, vec![entry("acme", "review", 1, &hash_of('1'))]);
    launched(&launching(&world, "acme/review@1", "one"))?;
    let id = World::id("review", "one");
    world.settle(&id);
    let mut edited = packaged("review", 1);
    edited.stages[1].prompt = "fix as someone else fixes".into();
    world.kinds.set_package(ACME, edited, &hash_of('9'));
    let reason = refused_whole(&world, "acme/review@1", "two")?;
    ensure!(
        reason.contains("acme/review@1")
            && reason.contains(HOST_LOCK)
            && reason.contains(&hash_of('1'))
            && reason.contains(&hash_of('9')),
        "the refusal names not the kind, the lock and both hashes: {reason}"
    );
    world.commit("review/one", &["looked.md"]);
    world.settle(&id);
    let prompt = world
        .prompt(&id, &World::identity("review", "one", "fix", 1))
        .ok_or("the next stage did not start")?;
    ensure!(
        prompt.contains("fix as review@1 fixes"),
        "the next stage's prompt is not as launched: {prompt}"
    );
    Ok(())
}

fn two_locks_that_disagree(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    world
        .kinds
        .set_lock(REPOSITORY, vec![entry("acme", "review", 1, &hash_of('1'))]);
    host_locks(&world, vec![entry("acme", "review", 1, &hash_of('2'))]);
    let reason = refused_whole(&world, "acme/review@1", "one")?;
    ensure!(
        reason.contains(".config/flume.lock")
            && reason.contains(HOST_LOCK)
            && reason.contains(&hash_of('1'))
            && reason.contains(&hash_of('2')),
        "the refusal names not both locks and both hashes: {reason}"
    );
    Ok(())
}

fn a_lock_that_cannot_be_read(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    world
        .kinds
        .unreadable_lock_file(HOST_LOCK, "expected `=` at line 1");
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "bump",
            false,
            vec![stage("code", Some(Deliverable::Commits), "{{brief}}")],
        )],
    );
    for (kind, name) in [("acme/review@1", "one"), ("change", "plan")] {
        let reason = refused_whole(&world, kind, name)?;
        ensure!(
            reason.contains(HOST_LOCK) && reason.contains("expected `=`"),
            "the refusal of {kind} names not the lock and its fault: {reason}"
        );
    }
    launched(&launching(&world, "bump", "one"))?;
    Ok(())
}

/// The hash flume's own `name@version` has on the fake.
fn own_hash(world: &World, name: &str, version: u32) -> Result<String, String> {
    world
        .kinds
        .flumes_own()
        .into_iter()
        .find(|own| own.kind.name == name && own.kind.version == version)
        .map(|own| own.hash)
        .ok_or_else(|| format!("flume holds no {name}@{version}"))
}

fn s59_the_repository_locks_change_1(_: &()) -> Outcome {
    let world = World::new();
    let hash = own_hash(&world, "change", 1)?;
    world
        .kinds
        .set_lock(REPOSITORY, vec![entry("flume", "change", 1, &hash)]);
    let bare = launched(&launching(&world, "change", "plan"))?;
    ensure_eq!(bare.version, 1, "a bare change's version");
    let held = world.held(&World::id("change", "plan")).ok_or("no place")?;
    ensure!(
        !held.place.kind.stages[1].repeats,
        "change@1's code repeats"
    );
    ensure_eq!(
        bare.identity.as_deref(),
        Some("flume/change@1"),
        "the identity"
    );
    let from = bare.kind_from.ok_or("no kind_from")?;
    ensure_eq!(from.hash, Some(hash), "change@1's hash");
    ensure_eq!(
        from.lock_file.as_deref(),
        Some(".config/flume.lock"),
        "change@1's lock"
    );
    let named = launched(&launching(&world, "change@2", "other"))?;
    ensure_eq!(
        named.identity.as_deref(),
        Some("flume/change@2"),
        "the identity"
    );
    let from = named.kind_from.ok_or("no kind_from")?;
    ensure_eq!(
        from.hash,
        Some(own_hash(&world, "change", 2)?),
        "change@2's hash"
    );
    ensure_eq!(from.lock_file, None, "change@2's lock");
    Ok(())
}

fn a_version_not_yet_locked(_: &()) -> Outcome {
    let world = World::new();
    holds(
        &world,
        "acme",
        ACME,
        &[("review", 1, '1'), ("review", 2, '2')],
    );
    host_locks(&world, vec![entry("acme", "review", 1, &hash_of('1'))]);
    let place = launched(&launching(&world, "acme/review", "one"))?;
    ensure_eq!(
        place.identity.as_deref(),
        Some("acme/review@1"),
        "the kind run"
    );
    Ok(())
}

fn the_newest_held_unlocked(_: &()) -> Outcome {
    let world = World::new();
    holds(
        &world,
        "acme",
        ACME,
        &[("review", 1, '1'), ("review", 2, '2')],
    );
    let reason = refused_whole(&world, "acme/review", "one")?;
    ensure!(
        reason.contains("acme/review@2") && reason.contains("recorded in no lock"),
        "the refusal names not the newest held as unlocked: {reason}"
    );
    Ok(())
}

fn a_lock_of_flumes_own_at_another_hash(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_lock(REPOSITORY, vec![entry("flume", "change", 1, &hash_of('0'))]);
    let reason = refused_whole(&world, "change", "plan")?;
    ensure!(
        reason.contains("flume/change@1")
            && reason.contains(&hash_of('0'))
            && reason.contains(&own_hash(&world, "change", 1)?),
        "the refusal names not the kind and both hashes: {reason}"
    );
    Ok(())
}

fn s95_the_source_moved_under_a_place(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    host_locks(&world, vec![entry("acme", "review", 1, &hash_of('1'))]);
    launched(&launching(&world, "acme/review@1", "one"))?;
    let id = World::id("review", "one");
    world.settle(&id);
    let moved = "/kinds/acme-next";
    holds(&world, "acme", moved, &[("review", 2, '2')]);
    host_locks(&world, vec![entry("acme", "review", 2, &hash_of('2'))]);
    world.commit("review/one", &["looked.md"]);
    world.settle(&id);
    let prompt = world
        .prompt(&id, &World::identity("review", "one", "fix", 1))
        .ok_or("the next stage did not start")?;
    ensure!(
        prompt.contains("fix as review@1 fixes"),
        "the next stage's prompt is not as launched: {prompt}"
    );
    let (identity, from) = kind_of(&world, "review", "one")?;
    ensure_eq!(identity, "acme/review@1", "the identity");
    ensure_eq!(
        (from.source_file, from.kind_dir),
        (
            Some(source_file("acme")),
            Some(package_dir(ACME, "review", 1))
        ),
        "where the kind came from"
    );
    let reason = refused_whole(&world, "acme/review@1", "two")?;
    ensure!(
        reason.contains("acme holds no acme/review@1") && reason.contains(moved),
        "the refusal names not the version as one acme does not hold: {reason}"
    );
    Ok(())
}

fn a_kinds_text_read_by_the_seam(_: &()) -> Outcome {
    let world = World::new();
    world
        .kinds
        .set_kinds(REPOSITORY, vec![packaged("review", 1)]);
    holds(&world, "acme", ACME, &[("review", 2, '2')]);
    host_locks(&world, vec![entry("acme", "review", 2, &hash_of('2'))]);
    let texts = ["review", "acme/review", "acme/review@2"];
    for (text, name) in texts.iter().zip(["a", "b", "c"]) {
        launched(&launching(&world, text, name))?;
    }
    ensure_eq!(
        world.kinds.log.of("read_named"),
        texts
            .iter()
            .map(|text| format!("read_named {text}"))
            .collect::<Vec<_>>(),
        "the texts the seam read"
    );
    let spelled: Vec<String> = ["a", "b", "c"]
        .iter()
        .map(|name| kind_of(&world, "review", name).map(|(identity, _)| identity))
        .collect::<Result<_, _>>()?;
    ensure_eq!(
        spelled,
        vec!["repo/review@1", "acme/review@2", "acme/review@2"],
        "the identities, as the seam spelled them"
    );
    ensure_eq!(
        world.kinds.log.of("identity_of"),
        vec!["identity_of repo review@1".to_string()],
        "the identities the seam spelled for a declaration"
    );
    Ok(())
}

fn sources_scripted_on_the_host(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    host_locks(&world, vec![entry("acme", "review", 1, &hash_of('1'))]);
    world
        .host
        .faulty_source("bad", "it is not TOML: expected a key at line 1");
    launched(&launching(&world, "acme/review@1", "one"))?;
    ensure_eq!(
        (
            world.kinds.log.of("read_source"),
            world.kinds.log.of("read_package")
        ),
        (
            vec![format!("read_source acme {ACME}")],
            vec![format!("read_package acme {ACME} review@1")]
        ),
        "acme read through the kinds seam"
    );
    world.kinds.log.clear();
    let reason = refused_whole(&world, "bad/x@1", "one")?;
    ensure!(
        reason.contains("not TOML") && reason.contains(&source_file("bad")),
        "the refusal names not bad's file and fault: {reason}"
    );
    ensure_eq!(
        world.kinds.log.count("read_source") + world.kinds.log.count("read_package"),
        0,
        "bad's packages read"
    );
    Ok(())
}

fn a_package_edited_after_a_launch(_: &()) -> Outcome {
    let world = World::new();
    holds(&world, "acme", ACME, &[("review", 1, '1')]);
    host_locks(&world, vec![entry("acme", "review", 1, &hash_of('1'))]);
    launched(&launching(&world, "acme/review@1", "one"))?;
    ensure_eq!(world.kinds.log.count("read_package"), 1, "packages read");
    let mut edited = packaged("review", 1);
    edited.stages[0].prompt = "look elsewhere".into();
    world.kinds.set_package(ACME, edited, &hash_of('9'));
    let reason = refused_whole(&world, "acme/review@1", "two")?;
    ensure!(
        reason.contains(&hash_of('1')) && reason.contains(&hash_of('9')),
        "the refusal names not both hashes: {reason}"
    );
    ensure_eq!(world.kinds.log.count("read_package"), 2, "packages read");
    let held = world.held(&World::id("review", "one")).ok_or("no place")?;
    ensure_eq!(held.place.kind, packaged("review", 1), "the held kind");
    Ok(())
}

//! Git's history and the host's facts, read for real (design 11;
//! history-and-host spec): the history scenarios against git on scratch
//! repositories, secrets that never repeat, a command's life, a file's
//! write time, an account's reading from its file, the bound the
//! configuration states, its two layers read by a command of its own and
//! written by none, the sources of kinds it declares in `sources/`, each
//! fault its source's own, and the room lock taken by two processes.

mod common;

use common::*;
use flume::host::config::HostConfig;
use flume::host::history::GitHistory;
use flume::host::{command_of, RealHost};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::history::{self, HistoryHarness};
use flume_seams::*;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};

struct Harness {
    repository: Repository,
    seam: GitHistory,
}

impl HistoryHarness for Harness {
    type Seam = GitHistory;

    fn seam(&self) -> &GitHistory {
        &self.seam
    }

    fn branch(&self) -> (String, String, Revision) {
        (
            self.repository.name(),
            "main".into(),
            Revision::new(self.repository.head()),
        )
    }

    fn commit(&self, _repository: &str, _branch: &str, path: &str) -> Revision {
        let count = self.repository.git(&["rev-list", "--count", "HEAD"]);
        self.repository.write(path, &format!("commit {count}\n"));
        Revision::new(self.repository.commit(path))
    }

    fn reset(&self, _repository: &str, _branch: &str, revision: &Revision) {
        self.repository
            .git(&["reset", "--quiet", "--hard", &revision.0]);
    }

    fn made_default(&self) -> String {
        "main".into()
    }

    /// A bare clone beside the repository as its `origin`, and two commits
    /// pushed to it from a clone elsewhere, which the repository never
    /// fetches.
    fn origin_ahead(&self, _repository: &str, branch: &str) -> Revision {
        let remote = self.repository.bare_remote();
        self.repository
            .git(&["remote", "add", "origin", &remote.to_string_lossy()]);
        let elsewhere = self.repository.scratch.path("elsewhere");
        git(
            &self.repository.scratch.root,
            &[
                "clone",
                "--quiet",
                &remote.to_string_lossy(),
                &elsewhere.to_string_lossy(),
            ],
        );
        for file in ["one.rs", "two.rs"] {
            std::fs::write(elsewhere.join(file), "on origin\n").expect("origin's work");
            git(&elsewhere, &["add", file]);
            git(&elsewhere, &["commit", "--quiet", "-m", file]);
        }
        git(&elsewhere, &["push", "--quiet", "origin", branch]);
        Revision::new(git(&elsewhere, &["rev-parse", "HEAD"]))
    }

    fn origin_gone(&self, _repository: &str) {
        let gone = self.repository.scratch.path("gone.git");
        self.repository
            .git(&["remote", "set-url", "origin", &gone.to_string_lossy()]);
    }
}

#[test]
fn the_history_scenarios_pass_against_git() {
    pass_all("history", &history::scenarios(), || Harness {
        repository: Repository::new("history"),
        seam: GitHistory,
    });
}

#[test]
fn a_head_reads_none_for_a_branch_that_does_not_exist() {
    let repository = Repository::new("no-branch");
    assert_eq!(GitHistory.head(&repository.name(), "fix/none"), None);
    let start = Revision::new(repository.head());
    assert!(!GitHistory.holds_since(&repository.name(), &Revision::new("0".repeat(40)), &start));
}

/// The default branch is the remote's, `origin/HEAD` set from the
/// remote's own `HEAD` where the repository lacks it, so a second read
/// answers as the first; the repository's own only where it has no
/// remote; and a bare repository's `HEAD` naming a branch never made
/// decides nothing (20, 132, C.7).
#[test]
fn the_default_branch_is_the_remotes() {
    let origin = Repository::new("default-origin");
    let remote = origin.bare_remote();
    let container = Container::of(&remote, Scratch::new("default"));
    let name = container.name();
    assert_eq!(
        GitHistory.default_branch(&name),
        Some("main".into()),
        "the remote's, not the master the bare repository's HEAD names"
    );
    assert_eq!(
        container.origin_head(),
        "refs/remotes/origin/main",
        "origin/HEAD set from the remote's own"
    );
    assert_eq!(GitHistory.default_branch(&name), Some("main".into()));
    container.git(&["remote", "remove", "origin"]);
    assert_eq!(
        GitHistory.default_branch(&name),
        None,
        "a HEAD naming a branch with no commit"
    );
    container.git(&["symbolic-ref", "HEAD", "refs/heads/main"]);
    assert_eq!(
        GitHistory.default_branch(&name),
        Some("main".into()),
        "the repository's own where it has no remote"
    );
}

/// The repository's common git directory, from which its agents'
/// identities take its id: its own `.git`, or the bare repository its
/// worktrees share, the same from the repository and from a worktree of
/// it (33, C.4).
#[test]
fn the_common_git_directory_names_the_repository() {
    let repository = Repository::new("common");
    let own = repository.dir.join(".git").canonicalize().unwrap();
    assert_eq!(
        GitHistory.common_dir(&repository.name()),
        Some(own.to_string_lossy().into_owned()),
        "the main worktree's"
    );
    let holder = repository.scratch.path("holder");
    std::fs::create_dir_all(&holder).unwrap();
    let bare = holder.join("holder.git");
    git(
        &holder,
        &[
            "clone",
            "--quiet",
            "--bare",
            &repository.name(),
            &bare.to_string_lossy(),
        ],
    );
    let bare = bare.canonicalize().unwrap();
    let from_holder = GitHistory.common_dir(&holder.to_string_lossy());
    assert_eq!(
        from_holder,
        Some(bare.to_string_lossy().into_owned()),
        "the bare repository beside its worktrees"
    );
    assert_ne!(
        flume_core::repository_id(&own.to_string_lossy()),
        flume_core::repository_id(&bare.to_string_lossy()),
        "two repositories' ids"
    );
    assert_eq!(
        GitHistory.common_dir("/"),
        None,
        "a directory in no repository"
    );
}

fn host(scratch: &Scratch, config: HostConfig) -> RealHost {
    RealHost::new(scratch.path("state/flume"), config, None)
}

#[test]
fn a_thousand_secrets_never_repeat() {
    let scratch = Scratch::new("secrets");
    let host = host(&scratch, HostConfig::default());
    let secrets: BTreeSet<String> = (0..1000).map(|_| host.secret().0).collect();
    assert_eq!(secrets.len(), 1000);
    assert!(secrets
        .iter()
        .all(|secret| secret.len() == 64 && secret.chars().all(|c| c.is_ascii_hexdigit())));
}

#[test]
fn a_command_lives_while_its_process_does() {
    let scratch = Scratch::new("command");
    let host = host(&scratch, HostConfig::default());
    let this = host.command();
    assert!(
        this.starts_with(&format!("{}@", std::process::id())),
        "{this}"
    );
    assert!(host.alive(&this), "this command reads as ended");
    let mut child = Command::new("sleep").arg("30").spawn().expect("a child");
    let named = command_of(child.id()).expect("the child's name");
    assert!(host.alive(&named));
    let (pid, start) = named.split_once('@').unwrap();
    assert!(
        !host.alive(&format!("{pid}@{start}0")),
        "another start of the same id lives"
    );
    child.kill().unwrap();
    child.wait().unwrap();
    assert!(!host.alive(&named), "an ended command reads as living");
    assert!(!host.alive("no command"));
}

#[test]
fn a_files_write_time_by_the_hosts_clock() {
    let scratch = Scratch::new("write-time");
    let host = host(&scratch, HostConfig::default());
    let file = scratch.path("written");
    assert_eq!(host.write_time(&file.to_string_lossy()), None);
    std::fs::write(&file, "").unwrap();
    let written = host
        .write_time(&file.to_string_lossy())
        .expect("a write time");
    assert!(
        written.abs_diff(host.now()) <= 5,
        "{written} against {}",
        host.now()
    );
}

/// `text` written whole at `path`: a new file beside it renamed over the
/// old, as a host's reader writes a reading (C.7).
fn written_whole(path: &Path, text: &str) {
    let new = path.with_extension("json.new");
    std::fs::write(&new, text).unwrap();
    std::fs::rename(&new, path).unwrap();
}

/// The store-profile spec's reading read from its file: a scratch file
/// in C.7's form read as it stands at each asking and never written, and
/// a missing one, or one not in the form, answered in words that name no
/// path (184, C.7).
#[test]
fn an_accounts_reading_from_its_file() {
    let scratch = Scratch::new("usage");
    let host = host(&scratch, HostConfig::default());
    let dir = scratch.path("state/usage");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("work-1.json");
    let path = file.to_string_lossy().to_string();
    let refused = host.usage(&path).unwrap_err();
    assert_eq!(refused, "the file is missing");
    written_whole(
        &file,
        r#"{ "read_at": "2026-11-05T14:02:11Z", "login": "fine", "windows": [
            { "name": "5-hour", "used": 42, "resets_at": "2026-11-05T17:00:00Z" },
            { "name": "7-day", "used": 71.5 }
        ] }"#,
    );
    let before = as_it_stands(&file);
    assert_eq!(
        host.usage(&path),
        Ok(Usage {
            read_at: 1_793_887_331,
            windows: vec![
                UsageWindow {
                    name: "5-hour".into(),
                    used: 4_200,
                    resets_at: Some(1_793_898_000),
                },
                UsageWindow {
                    name: "7-day".into(),
                    used: 7_150,
                    resets_at: None,
                },
            ],
        })
    );
    assert_eq!(as_it_stands(&file), before, "the reading was written");
    written_whole(
        &file,
        r#"{ "read_at": "2026-11-05T14:07:11Z", "windows": [{ "name": "5-hour", "used": 48 }] }"#,
    );
    assert_eq!(
        host.usage(&path)
            .map(|usage| (usage.read_at, usage.windows[0].used)),
        Ok((1_793_887_631, 4_800)),
        "the reading as it stands at the next asking"
    );
    written_whole(
        &file,
        r#"{ "read_at": "2026-11-05T14:07:11", "windows": [] }"#,
    );
    let refused = host.usage(&path).unwrap_err();
    assert_eq!(refused, "read_at is not an RFC 3339 time with its offset");
    std::fs::write(&file, "half a {").unwrap();
    let refused = host.usage(&path).unwrap_err();
    assert!(refused.starts_with("the file is not JSON: "), "{refused}");
    let root = scratch.root.to_string_lossy().to_string();
    for words in [
        host.usage(&path).unwrap_err(),
        host.usage(&dir.to_string_lossy()).unwrap_err(),
        host.usage(&scratch.path("nowhere/work-2.json").to_string_lossy())
            .unwrap_err(),
    ] {
        assert!(
            !words.contains(&root) && !words.contains("work-"),
            "a path in {words}"
        );
    }
}

#[test]
fn the_bound_the_configuration_states() {
    let scratch = Scratch::new("bound");
    let file = scratch.path("config/flume/config.toml");
    assert_eq!(
        host(&scratch, HostConfig::read(&file).unwrap()).bound(),
        None
    );
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(&file, "[host]\nagents = 2\n").unwrap();
    let config = HostConfig::read(&file).unwrap();
    assert_eq!(host(&scratch, config.clone()).bound(), Some(2));
    assert_eq!(
        config.terminal, None,
        "a terminal left out is the printed command"
    );
    std::fs::write(&file, "[host\n").unwrap();
    assert!(
        HostConfig::read(&file).is_err(),
        "a file that is no configuration"
    );
}

/// A scratch `XDG_CONFIG_HOME`'s configuration file, and the sources
/// directory beside it, made.
fn configuration(scratch: &Scratch) -> (std::path::PathBuf, std::path::PathBuf) {
    let file = scratch.path("config/flume/config.toml");
    let sources = scratch.path("config/flume/sources");
    std::fs::create_dir_all(&sources).unwrap();
    (file, sources)
}

/// The host's sources as its configuration in `file` reads them.
fn sources(scratch: &Scratch, file: &Path) -> BTreeMap<String, HostSource> {
    host(
        scratch,
        HostConfig::read(file).expect("the configuration read"),
    )
    .sources()
}

/// The host user's home, as `~` is read.
fn home() -> String {
    std::env::var("HOME").expect("a home")
}

#[test]
fn a_source_written_beside_and_renamed() {
    // 159, 191, C.7: `.acme.toml.tmp` is no source; renamed to
    // `acme.toml` it declares acme, its `~` the home.
    let scratch = Scratch::new("source-renamed");
    let (file, dir) = configuration(&scratch);
    let beside = dir.join(".acme.toml.tmp");
    std::fs::write(&beside, "kinds_dir = \"~/kinds/acme\"\n").unwrap();
    assert_eq!(sources(&scratch, &file), BTreeMap::new());
    let acme = dir.join("acme.toml");
    std::fs::rename(&beside, &acme).unwrap();
    assert_eq!(
        sources(&scratch, &file),
        BTreeMap::from([(
            "acme".to_string(),
            HostSource {
                file: acme.to_string_lossy().into_owned(),
                read: HostSourceRead::Read {
                    kinds_dir: format!("{}/kinds/acme", home()),
                    roles: None,
                },
            }
        )])
    );
}

#[test]
fn entries_that_are_no_source() {
    // 191, C.7: `README`, `acme.toml~` and a directory `beta.toml/` beside
    // `acme.toml`, which alone declares a source; a link to a file reads as
    // the file.
    let scratch = Scratch::new("no-source");
    let (file, dir) = configuration(&scratch);
    std::fs::write(dir.join("acme.toml"), "kinds_dir = \"/srv/kinds/acme\"\n").unwrap();
    std::fs::write(dir.join("README"), "kinds_dir = \"/srv/kinds/readme\"\n").unwrap();
    std::fs::write(dir.join("acme.toml~"), "kinds_dir = \"/srv/kinds/old\"\n").unwrap();
    std::fs::create_dir_all(dir.join("beta.toml")).unwrap();
    let declared: Vec<String> = sources(&scratch, &file).into_keys().collect();
    assert_eq!(declared, ["acme"]);
    let managed = scratch.path("managed/crew.toml");
    std::fs::create_dir_all(managed.parent().unwrap()).unwrap();
    std::fs::write(&managed, "kinds_dir = \"/srv/kinds/crew\"\n").unwrap();
    std::os::unix::fs::symlink(&managed, dir.join("crew.toml")).unwrap();
    let read = sources(&scratch, &file);
    assert_eq!(
        read["crew"].read,
        HostSourceRead::Read {
            kinds_dir: "/srv/kinds/crew".into(),
            roles: None,
        }
    );
    assert_eq!(read.len(), 2, "{read:?}");
}

#[test]
fn no_sources_directory() {
    // 154, 191: a configuration directory holding no `sources/` declares
    // no source, and its `config.toml` reads as before.
    let scratch = Scratch::new("no-sources");
    let file = scratch.path("config/flume/config.toml");
    assert_eq!(sources(&scratch, &file), BTreeMap::new());
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(&file, "[host]\nagents = 2\n").unwrap();
    let config = HostConfig::read(&file).unwrap();
    assert_eq!(config.agents, Some(2));
    assert_eq!(host(&scratch, config).sources(), BTreeMap::new());
}

#[test]
fn each_fault_is_its_sources_own() {
    // 4, 154, 159, 191: a file that is not TOML, that gives no
    // `kinds_dir`, one neither absolute nor under `~`, or a key flume
    // does not know, or that takes the name of flume's own source, the
    // repository's, a launch's or a name no identity allows, is its own
    // source's fault, named, and fails neither the configuration nor
    // another source; a `kinds_dir` that does not exist is no fault of
    // the file's.
    let scratch = Scratch::new("source-faults");
    let (file, dir) = configuration(&scratch);
    std::fs::write(&file, "[host]\nagents = 3\n").unwrap();
    let given = "kinds_dir = \"/srv/kinds/x\"\n";
    for (name, text) in [
        ("acme", "kinds_dir = \"/srv/kinds/acme\"\n"),
        ("gone", "kinds_dir = \"/no/such/directory\"\n"),
        ("broken", "kinds_dir = [\n"),
        ("bad", "# no kinds_dir\n"),
        ("relative", "kinds_dir = \"kinds/relative\"\n"),
        ("other", "kinds_dir = \"~other/kinds\"\n"),
        (
            "extra",
            "kinds_dir = \"/srv/kinds/extra\"\nfetch = \"https://kinds.invalid\"\n",
        ),
        ("number", "kinds_dir = 7\n"),
        ("flume", given),
        ("repo", given),
        ("launch", given),
        ("ac.me", given),
    ] {
        std::fs::write(dir.join(format!("{name}.toml")), text).unwrap();
    }
    let config = HostConfig::read(&file).expect("the configuration read whatever its sources");
    assert_eq!(config.agents, Some(3), "the file's own settings");
    let read = host(&scratch, config).sources();
    let fault = |name: &str| match &read[name] {
        HostSource {
            file,
            read: HostSourceRead::CouldNotRead { reason },
        } => {
            assert_eq!(
                file,
                &dir.join(format!("{name}.toml")).to_string_lossy(),
                "{name}'s file"
            );
            reason.clone()
        }
        source => panic!("{name} read as {source:?}"),
    };
    for (name, named) in [
        ("broken", "is not TOML"),
        ("bad", "gives no `kinds_dir`"),
        (
            "relative",
            "`kinds/relative`, which is neither absolute nor under `~`",
        ),
        (
            "other",
            "`~other/kinds`, which is neither absolute nor under `~`",
        ),
        ("extra", "`fetch` is no key"),
        ("number", "`kinds_dir` is not a path"),
        ("flume", "the source of flume's own kinds"),
        ("repo", "the source of the repository's kinds"),
        ("launch", "the source of a launch's kinds"),
        ("ac.me", "`ac.me`"),
    ] {
        let reason = fault(name);
        assert!(reason.contains(named), "{name}: {reason}");
    }
    for (name, kinds_dir) in [("acme", "/srv/kinds/acme"), ("gone", "/no/such/directory")] {
        assert_eq!(
            read[name].read,
            HostSourceRead::Read {
                kinds_dir: kinds_dir.into(),
                roles: None,
            },
            "{name}"
        );
    }
    assert_eq!(read.len(), 12, "{read:?}");
}

#[test]
fn a_sources_directory_that_cannot_be_listed_fails_the_configuration() {
    // C.7: `sources` that is no directory is the configuration's fault,
    // as a `config.toml` that cannot be read is.
    let scratch = Scratch::new("sources-unlisted");
    let file = scratch.path("config/flume/config.toml");
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(scratch.path("config/flume/sources"), "a file").unwrap();
    let Err(reason) = HostConfig::read(&file) else {
        panic!("a sources directory that cannot be listed read");
    };
    assert!(reason.contains("config/flume/sources"), "{reason}");
}

#[test]
fn the_local_layer_declares_no_source() {
    // 159, 191, C.7; core.45: the sources are the managed layer's, read
    // from `sources/` beside `config.toml` whether the local layer is
    // taken or not, and a local file naming a source is not taken.
    let scratch = Scratch::new("local-sources");
    let (managed, dir) = configuration(&scratch);
    let local = scratch.path("config/flume/config.local.toml");
    std::fs::write(&managed, "[host]\nagents = 8\n").unwrap();
    std::fs::write(dir.join("acme.toml"), "kinds_dir = \"/srv/kinds/acme\"\n").unwrap();
    let declared = |config: &HostConfig| config.sources.keys().cloned().collect::<Vec<_>>();

    std::fs::write(&local, "[streams.work]\nagents = 3\n").unwrap();
    let taken = HostConfig::read_layers(&managed, &local).expect("the configuration read");
    assert_eq!(taken.configuration.local, LocalLayer::Taken);
    assert_eq!(declared(&taken), ["acme"]);

    std::fs::write(&local, "[sources.beta]\nkinds_dir = \"/srv/kinds/beta\"\n").unwrap();
    let refused = HostConfig::read_layers(&managed, &local).expect("the configuration read");
    assert!(
        matches!(
            &refused.configuration.local,
            LocalLayer::NotTaken { keys, .. } if keys.iter().any(|key| key.starts_with("sources"))
        ),
        "{:?}",
        refused.configuration.local
    );
    assert_eq!(declared(&refused), ["acme"]);
}

/// A scratch host declaring the streams `east` and `west` and each of
/// `sources`, `(name, roles file's text)`, its roles file written beside
/// its packages' directory; the host read as a command reads it, and the
/// roles files' paths by source (143, 191, 193, C.7).
fn roles_host(
    scratch: &Scratch,
    sources: &[(&str, &str)],
) -> (RealHost, BTreeMap<String, std::path::PathBuf>) {
    let (file, dir) = configuration(scratch);
    std::fs::write(&file, "[streams.east]\n[streams.west]\n").unwrap();
    let mut files = BTreeMap::new();
    for (name, text) in sources {
        let roles = scratch.path(&format!("state/{name}/roles.toml"));
        std::fs::create_dir_all(roles.parent().unwrap()).unwrap();
        std::fs::write(&roles, text).unwrap();
        std::fs::write(
            dir.join(format!("{name}.toml")),
            format!(
                "kinds_dir = \"/srv/kinds/{name}\"\nroles = \"{}\"\n",
                roles.display()
            ),
        )
        .unwrap();
        files.insert(name.to_string(), roles);
    }
    let config = HostConfig::read(&file).expect("the configuration read");
    (host(scratch, config), files)
}

/// The run `kind` of `source` in `/r` at priority 0, with nothing else.
fn bare_run(source: &str, kind: &str) -> RoleRun {
    RoleRun {
        source: source.into(),
        repository: "/r".into(),
        kind: kind.into(),
        priority: 0,
        base: None,
        brief: None,
        name: None,
        intent: None,
        agents: RoleAgents::default(),
        answers: None,
        answerer: false,
    }
}

/// role-waking, "A role configured", "A base and a brief" and "The run's
/// intent and settings", as the host reads them: a source's `roles`
/// names its file, under `~` read as the home, and each key of a run's
/// entry is its run's, its kind its source's own written bare or under
/// the source's name, its repository's `~` the home (7, 32, 132, 147,
/// 193, C.7).
#[test]
fn a_sources_roles_name_each_run() {
    let scratch = Scratch::new("roles-run");
    let (host, _) = roles_host(
        &scratch,
        &[(
            "acme",
            "[roles.\"east/planner\"]\nrepository = \"~/r\"\nkind = \"turn\"\npriority = 5\n\
             base = \"next\"\nbrief = \"Plan the next unit.\"\nname = \"east\"\n\
             intent = \"east-planner\"\nanswers = \"The work's scope.\"\nanswerer = true\n\
             [roles.\"east/planner\".agents]\nmodel = \"opus\"\n\
             [roles.\"east/planner\".agents.stages.turn]\ninstructions = \"Be brief.\"\n\
             [roles.\"west/planner\"]\nrepository = \"/r\"\nkind = \"acme/turn\"\n",
        )],
    );
    assert_eq!(
        host.role("east/planner"),
        Some(RoleRun {
            repository: format!("{}/r", home()),
            priority: 5,
            base: Some("next".into()),
            brief: Some("Plan the next unit.".into()),
            name: Some("east".into()),
            intent: Some("east-planner".into()),
            agents: RoleAgents {
                every_stage: Settings {
                    model: Some("opus".into()),
                    ..Settings::default()
                },
                stages: BTreeMap::from([(
                    "turn".to_string(),
                    Settings {
                        instructions: Some("Be brief.".into()),
                        ..Settings::default()
                    },
                )]),
            },
            answers: Some("The work's scope.".into()),
            answerer: true,
            ..bare_run("acme", "turn")
        })
    );
    assert_eq!(
        host.role("west/planner"),
        Some(bare_run("acme", "turn")),
        "a kind under its source's own name, its priority zero and its base the default branch's"
    );
    assert!(host.roles().refused.is_empty(), "{:?}", host.roles());
}

/// role-waking, "Read as it stands": the source's program writes its
/// roles file beside it and renames it over the first, and the next ask
/// reads `east/ops` beside `east/planner`, nothing of flume's run between;
/// no read writes the file (191, 193; core.16).
#[test]
fn a_roles_file_read_as_it_stands() {
    let scratch = Scratch::new("roles-renamed");
    let planner = "[roles.\"east/planner\"]\nrepository = \"/r\"\nkind = \"turn\"\n";
    let (host, files) = roles_host(&scratch, &[("acme", planner)]);
    let roles = &files["acme"];
    assert_eq!(
        host.roles().declared.keys().collect::<Vec<_>>(),
        ["east/planner"]
    );
    let beside = roles.with_file_name(".roles.toml.tmp");
    std::fs::write(
        &beside,
        format!("{planner}[roles.\"east/ops\"]\nrepository = \"/r\"\nkind = \"turn\"\n"),
    )
    .unwrap();
    std::fs::rename(&beside, roles).unwrap();
    let before = as_it_stands(roles);
    assert_eq!(
        host.roles().declared.keys().collect::<Vec<_>>(),
        ["east/ops", "east/planner"]
    );
    assert_eq!(host.role("east/ops"), Some(bare_run("acme", "turn")));
    assert_eq!(as_it_stands(roles), before, "a read wrote the roles file");
}

/// role-waking, "A standing design agent" and "A standing door" as the
/// host reads them, and "A carried address": a standing agent's entry
/// gives no run and a run's no standing agent; a carried address of a
/// stream the host does not declare is carried (132, 193, C.5, C.7;
/// record 0039's amendment).
#[test]
fn a_standing_agent_and_a_carried_address() {
    let scratch = Scratch::new("roles-standing");
    let (host, _) = roles_host(
        &scratch,
        &[(
            "acme",
            "[roles.\"east/design\"]\nagent = \"east-design\"\n\
             answers = \"A decision the design has not made.\"\n\
             [roles.\"east/operator\"]\nagent = \"east-operator\"\nsession = \"doors\"\nanswerer = true\n\
             [roles.\"east/ops\"]\nrepository = \"/r\"\nkind = \"turn\"\n\
             [roles.\"north/planner\"]\ncarried = true\nanswers = \"The work's scope.\"\n",
        )],
    );
    let standing = host.standing();
    assert_eq!(
        standing.keys().collect::<Vec<_>>(),
        ["east/design", "east/operator"]
    );
    assert_eq!(
        standing["east/operator"],
        StandingAgent {
            source: "acme".into(),
            agent: "east-operator".into(),
            session: Some("doors".into()),
            answers: None,
            answerer: true,
        }
    );
    assert_eq!(host.role("east/design"), None, "a standing address's run");
    assert!(host.role("east/ops").is_some());
    assert_eq!(
        host.declared("north/planner"),
        Ok(RoleEntry::Carried(CarriedRole {
            source: "acme".into(),
            answers: Some("The work's scope.".into()),
        }))
    );
    assert_eq!(
        host.answerer("east"),
        Some(StreamAnswerer::Stands(DeclaredBy {
            address: "east/operator".into(),
            source: "acme".into(),
        }))
    );
}

/// role-waking, "An entry that names a stream", "A stream the host does
/// not declare", "Another source's kind", "A standing agent beside a
/// kind" and the other faults of an entry: each declares nothing for its
/// address, named with why, and every other entry of the file is
/// declared (4, 132, 143, 191, 193, C.7; core.30).
#[test]
fn an_entry_at_fault_declares_nothing_for_its_address() {
    let scratch = Scratch::new("roles-faults");
    let run = "repository = \"/r\"\nkind = \"turn\"\n";
    let entries = [
        (
            "east/a",
            format!("{run}stream = \"west\"\n"),
            "its address names its stream",
        ),
        (
            "south/planner",
            run.to_string(),
            "the stream south, which the host does not declare",
        ),
        (
            "east/b",
            "repository = \"/r\"\nkind = \"beta/turn\"\n".into(),
            "not acme's own",
        ),
        (
            "east/c",
            "repository = \"/r\"\nkind = \"turn@1\"\n".into(),
            "its version the locks'",
        ),
        (
            "east/d",
            "agent = \"east-d\"\nkind = \"turn\"\n".into(),
            "both a standing agent and a kind",
        ),
        (
            "east/e",
            "agent = \"east-e\"\npriority = 5\n".into(),
            "priority beside agent",
        ),
        (
            "east/f",
            format!("{run}env = {{ A = \"b\" }}\n"),
            "an agent's environment is its account's alone",
        ),
        ("east/g", format!("{run}pool = \"work\"\n"), "gives pool"),
        ("east/h", format!("{run}account = \"w\"\n"), "gives account"),
        ("east/i", format!("{run}carried = true\n"), "carried beside"),
        (
            "east/j",
            "carried = true\nagent = \"east-j\"\n".into(),
            "carried beside agent",
        ),
        (
            "east/k",
            "carried = true\nanswerer = true\n".into(),
            "carried beside answerer",
        ),
        (
            "east/l",
            format!("{run}points = 3\n"),
            "points is no word of a role's run",
        ),
        (
            "east/m",
            format!("{run}[roles.\"east/m\".agents]\nenv = {{ A = \"b\" }}\n"),
            "env is no setting of a role's run",
        ),
        ("planner", run.to_string(), "<stream>/<role>"),
    ];
    let mut text = String::from("[roles.\"east/ok\"]\nrepository = \"/r\"\nkind = \"turn\"\n");
    for (address, entry, _) in &entries {
        text.push_str(&format!("[roles.\"{address}\"]\n{entry}"));
    }
    let (host, _) = roles_host(&scratch, &[("acme", &text)]);
    let roles = host.roles();
    assert_eq!(
        roles.declared.keys().collect::<Vec<_>>(),
        ["east/ok"],
        "{roles:?}"
    );
    for (address, _, named) in entries {
        let Err(why) = host.declared(address) else {
            panic!("{address} declared");
        };
        assert!(why.contains(named), "{address}: {why}");
        assert!(
            roles
                .refused
                .iter()
                .any(|refused| refused.address == address && refused.source == "acme"),
            "{address} not named refused"
        );
    }
    assert!(roles.unread.is_empty());
}

/// role-waking, "A roles file that is not TOML": acme's roles file
/// declares no role and is named with why, and beta's `west/ops` is
/// declared; a file that cannot be read, and one giving another key than
/// `roles`, likewise (191, 193).
#[test]
fn a_roles_file_that_cannot_be_read_declares_no_role() {
    let scratch = Scratch::new("roles-unread");
    let (host, files) = roles_host(
        &scratch,
        &[
            ("acme", "[roles.\"east/ops\"\nrepository = \"/r\"\n"),
            (
                "beta",
                "[roles.\"west/ops\"]\nrepository = \"/r\"\nkind = \"turn\"\n",
            ),
            ("gamma", "kinds = 3\n"),
        ],
    );
    std::fs::remove_file(&files["gamma"]).unwrap();
    std::fs::write(
        scratch.path("config/flume/sources/delta.toml"),
        format!(
            "kinds_dir = \"/srv/kinds/delta\"\nroles = \"{}\"\n",
            scratch.path("state/delta/roles.toml").display()
        ),
    )
    .unwrap();
    std::fs::create_dir_all(scratch.path("state/delta")).unwrap();
    std::fs::write(scratch.path("state/delta/roles.toml"), "kinds = 3\n").unwrap();
    let host = {
        drop(host);
        let file = scratch.path("config/flume/config.toml");
        self::host(&scratch, HostConfig::read(&file).unwrap())
    };
    let roles = host.roles();
    assert_eq!(roles.declared.keys().collect::<Vec<_>>(), ["west/ops"]);
    assert_eq!(roles.unread["acme"].file, files["acme"].to_string_lossy());
    assert!(
        roles.unread["acme"].reason.contains("is not TOML"),
        "{roles:?}"
    );
    assert!(
        roles.unread["gamma"].reason.contains("could not be read"),
        "{roles:?}"
    );
    assert!(
        roles.unread["delta"]
            .reason
            .contains("`kinds` is no key of a roles file"),
        "{roles:?}"
    );
    assert!(host
        .declared("east/ops")
        .is_err_and(|why| why.contains("no source")));
}

/// role-waking, "One address, two sources" and "Two addresses, one
/// place": neither is declared, each named with both (3, 4, 132, 193).
#[test]
fn what_two_entries_claim_is_neithers() {
    let scratch = Scratch::new("roles-claimed");
    let (host, _) = roles_host(
        &scratch,
        &[
            (
                "acme",
                "[roles.\"east/planner\"]\nrepository = \"/r\"\nkind = \"turn\"\n\
                 [roles.\"east/ops\"]\nrepository = \"/r\"\nkind = \"turn\"\nname = \"planner\"\n\
                 [roles.\"west/ops\"]\nrepository = \"/r\"\nkind = \"turn\"\nname = \"planner\"\n\
                 [roles.\"west/design\"]\nrepository = \"/s\"\nkind = \"turn\"\nname = \"planner\"\n",
            ),
            ("beta", "[roles.\"east/planner\"]\nagent = \"east-planner\"\n"),
        ],
    );
    let Err(why) = host.declared("east/planner") else {
        panic!("east/planner declared");
    };
    assert!(why.contains("acme") && why.contains("beta"), "{why}");
    for address in ["east/ops", "west/ops"] {
        let Err(why) = host.declared(address) else {
            panic!("{address} declared");
        };
        assert!(
            why.contains("east/ops") && why.contains("west/ops") && why.contains("turn--planner"),
            "{why}"
        );
    }
    assert!(
        host.role("west/design").is_some(),
        "a run of the same name in another repository"
    );
    assert_eq!(host.roles().refused.len(), 4, "{:?}", host.roles());
}

/// history-and-host, "Each stream's door", "No answerer" and "Two behind
/// one stream" (143, 191, 193, C.7).
#[test]
fn the_answerer_behind_each_stream() {
    let scratch = Scratch::new("roles-answerer");
    let (host, files) = roles_host(
        &scratch,
        &[(
            "acme",
            "[roles.\"east/operator\"]\nrepository = \"/r\"\nkind = \"turn\"\nanswerer = true\n\
             [roles.\"west/operator\"]\nrepository = \"/r\"\nkind = \"turn\"\nanswerer = true\n",
        )],
    );
    let door = |address: &str| {
        Some(StreamAnswerer::Stands(DeclaredBy {
            address: address.into(),
            source: "acme".into(),
        }))
    };
    assert_eq!(host.answerer("east"), door("east/operator"));
    assert_eq!(host.answerer("west"), door("west/operator"));
    assert_eq!(host.answerer(DEFAULT_STREAM), None);

    std::fs::write(
        &files["acme"],
        "[roles.\"east/ops\"]\nrepository = \"/r\"\nkind = \"turn\"\n",
    )
    .unwrap();
    for stream in ["east", "west", DEFAULT_STREAM] {
        assert_eq!(host.answerer(stream), None, "{stream}");
    }

    let (host, _) = roles_host(
        &scratch,
        &[
            (
                "acme",
                "[roles.\"east/operator\"]\nrepository = \"/r\"\nkind = \"turn\"\nanswerer = true\n",
            ),
            ("beta", "[roles.\"east/ops\"]\nagent = \"east-ops\"\nanswerer = true\n"),
        ],
    );
    assert_eq!(
        host.answerer("east"),
        Some(StreamAnswerer::Contested {
            roles: vec![
                DeclaredBy {
                    address: "east/operator".into(),
                    source: "acme".into(),
                },
                DeclaredBy {
                    address: "east/ops".into(),
                    source: "beta".into(),
                },
            ],
        })
    );
}

/// history-and-host, "The host's roles" (28, 41, 132, 193).
#[test]
fn the_roles_a_host_answers_with() {
    let scratch = Scratch::new("roles-answering");
    let (host, _) = roles_host(
        &scratch,
        &[(
            "acme",
            "[roles.\"east/planner\"]\nrepository = \"/r\"\nkind = \"turn\"\nanswers = \"The work's scope.\"\n\
             [roles.\"east/ops\"]\nrepository = \"/r\"\nkind = \"turn\"\nanswers = \"A live system.\"\n\
             [roles.\"west/ops\"]\nrepository = \"/r\"\nkind = \"turn\"\nanswers = \"A live system.\"\n\
             [roles.\"north/design\"]\ncarried = true\nanswers = \"A decision.\"\n\
             [roles.\"east/reviewer\"]\nrepository = \"/r\"\nkind = \"turn\"\n",
        )],
    );
    let answering: Vec<(String, String, String)> = host
        .answering()
        .into_iter()
        .map(|role| (role.address, role.stream, role.answers))
        .collect();
    let role = |address: &str, stream: &str, answers: &str| {
        (address.to_string(), stream.to_string(), answers.to_string())
    };
    assert_eq!(
        answering,
        [
            role("east/ops", "east", "A live system."),
            role("east/planner", "east", "The work's scope."),
            role("north/design", "north", "A decision."),
            role("west/ops", "west", "A live system."),
        ]
    );
}

/// A source's `roles` neither absolute nor under `~` is its file's fault,
/// as its `kinds_dir` would be (191, 193, C.7).
#[test]
fn a_sources_roles_path_at_fault() {
    let scratch = Scratch::new("roles-path");
    let (file, dir) = configuration(&scratch);
    std::fs::write(
        dir.join("acme.toml"),
        "kinds_dir = \"/srv/kinds/acme\"\nroles = \"state/roles.toml\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("beta.toml"),
        "kinds_dir = \"/srv/kinds/beta\"\nroles = 3\n",
    )
    .unwrap();
    let read = sources(&scratch, &file);
    for (name, named) in [
        (
            "acme",
            "`roles` is `state/roles.toml`, which is neither absolute nor under `~`",
        ),
        ("beta", "`roles` is not a path"),
    ] {
        let HostSourceRead::CouldNotRead { reason } = &read[name].read else {
            panic!("{name} read");
        };
        assert!(reason.contains(named), "{name}: {reason}");
    }
}

/// role-waking, "The roles in the host's configuration": a `config.toml`
/// giving `[roles]` is unreadable, naming it and that a source declares a
/// role, as a command of its own reads it (193, C.7).
#[test]
fn a_roles_table_in_the_hosts_configuration_is_unreadable() {
    let scratch = Scratch::new("roles-table");
    let (file, _) = configuration(&scratch);
    std::fs::write(
        &file,
        "[streams.east]\n[roles.\"east/planner\"]\nrepository = \"/r\"\nkind = \"turn\"\n",
    )
    .unwrap();
    let Err(reason) = HostConfig::read(&file) else {
        panic!("a configuration with [roles] read");
    };
    assert!(
        reason.contains("[roles]") && reason.contains("a source declares each role"),
        "{reason}"
    );
    let answer = scratch.path("answer.json");
    let status = Command::new(std::env::current_exe().unwrap())
        .args([READER, "--exact", "--nocapture", "--test-threads=1"])
        .env("XDG_CONFIG_HOME", scratch.path("config"))
        .env("XDG_STATE_HOME", scratch.path("state"))
        .env(CHILD_ANSWER, &answer)
        .stdout(std::process::Stdio::null())
        .status()
        .expect("a child process");
    assert!(status.success());
    let answer: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&answer).unwrap()).unwrap();
    let refused = answer["refused"].as_str().unwrap_or_default();
    assert!(
        refused.contains("[roles]") && refused.contains("a source declares each role"),
        "{answer}"
    );
}

const CHILD_ANSWER: &str = "FLUME_TEST_LAYERS_ANSWER";
const READER: &str = "a_child_process_reads_the_hosts_layers";

/// The child's half of the layers test: the host as a command finds it,
/// under the `XDG_CONFIG_HOME` its parent gave it, its configuration and
/// its streams and a fix's label written as JSON. Run by the test below as a process of its
/// own; it does nothing otherwise.
#[test]
fn a_child_process_reads_the_hosts_layers() {
    let Some(answer) = std::env::var_os(CHILD_ANSWER) else {
        return;
    };
    let read = match RealHost::of_host() {
        Ok(host) => serde_json::json!({
            "configuration": host.configuration(),
            "bound": host.bound(),
            "streams": host.streams(),
            "label": host.label_for("/r", "fix"),
        }),
        Err(refused) => serde_json::json!({ "refused": refused }),
    };
    std::fs::write(answer, read.to_string()).unwrap();
}

/// A file's content and write time.
fn as_it_stands(path: &Path) -> (String, std::time::SystemTime) {
    (
        std::fs::read_to_string(path).unwrap(),
        std::fs::metadata(path).unwrap().modified().unwrap(),
    )
}

/// `RealHost` over `config.toml` and `config.local.toml` in a scratch
/// `XDG_CONFIG_HOME`, as a command of its own reads them: the local layer
/// taken key by key, a kind's label among it, a pinned key passed over,
/// an account's variables `<set>`, a local layer setting a pool not
/// taken, and no local file; and neither file's content or write time
/// changed by any read (32, 56, 180, C.7, S83; core.16, core.45).
#[test]
fn the_host_reads_both_layers_and_writes_neither() {
    let scratch = Scratch::new("layers");
    let dir = scratch.path("config/flume");
    std::fs::create_dir_all(&dir).unwrap();
    let managed = dir.join("config.toml");
    let local = dir.join("config.local.toml");
    std::fs::write(
        &managed,
        "[host]\nagents = 8\npin = [\"streams.work.session\"]\n\
         [streams.work]\nagents = 6\npool = \"work\"\n\
         [pools.work]\nstrategy = \"fill-first\"\n\
         accounts = [{ name = \"w\", program = \"agent\", env = { AGENT_HOME = \"~/.agent-w\" } }]\n\
         [kinds.fix]\nlabel = \"construction\"\n",
    )
    .unwrap();
    let read = || {
        let answer = scratch.path("answer.json");
        let _ = std::fs::remove_file(&answer);
        let status = Command::new(std::env::current_exe().unwrap())
            .args([READER, "--exact", "--nocapture", "--test-threads=1"])
            .env("XDG_CONFIG_HOME", scratch.path("config"))
            .env("XDG_STATE_HOME", scratch.path("state"))
            .env(CHILD_ANSWER, &answer)
            .stdout(std::process::Stdio::null())
            .status()
            .expect("a child process");
        assert!(status.success());
        let answer: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&answer).unwrap()).unwrap();
        assert_eq!(answer.get("refused"), None, "{answer}");
        answer
    };
    let value = |answer: &serde_json::Value, key: &str| {
        answer["configuration"]["values"]
            .as_array()
            .unwrap()
            .iter()
            .find(|value| value["key"] == key)
            .map(|value| (value["value"].clone(), value["layer"].clone()))
            .unwrap_or_else(|| panic!("no {key} in {answer}"))
    };

    std::fs::write(
        &local,
        "[streams.work]\nagents = 3\npriority = 30\nsession = \"office\"\n\
         [kinds.fix]\nlabel = \"operation\"\n",
    )
    .unwrap();
    let before = (as_it_stands(&managed), as_it_stands(&local));
    let answer = read();
    assert_eq!(answer["configuration"]["local"]["state"], "taken");
    assert_eq!(answer["bound"], 8);
    assert_eq!(answer["streams"]["work"]["agents"], 3);
    assert_eq!(answer["streams"]["work"]["priority"], 30);
    assert_eq!(answer["streams"]["work"]["pool"], "work");
    assert_eq!(
        answer["label"],
        serde_json::json!({"name": "operation", "from": "host-kind"})
    );
    assert_eq!(
        value(&answer, "streams.work.agents"),
        (serde_json::json!("3"), serde_json::json!("local"))
    );
    assert_eq!(
        value(&answer, "streams.work.pool"),
        (serde_json::json!("\"work\""), serde_json::json!("managed"))
    );
    assert_eq!(
        value(&answer, "pools.work.accounts[0].env.AGENT_HOME"),
        (serde_json::json!(SET), serde_json::json!("managed"))
    );
    assert_eq!(
        answer["configuration"]["passed_over"],
        serde_json::json!([{"key": "streams.work.session", "pin": "streams.work.session"}])
    );
    assert!(
        !answer.to_string().contains(".agent-w"),
        "a variable's value in {answer}"
    );
    assert_eq!(
        (as_it_stands(&managed), as_it_stands(&local)),
        before,
        "a read changed a file"
    );

    std::fs::write(
        &local,
        "[streams.work]\nagents = 3\n\
         [pools.work]\nstrategy = \"least-busy\"\naccounts = [{ name = \"l\", program = \"agent\" }]\n",
    )
    .unwrap();
    let before = (as_it_stands(&managed), as_it_stands(&local));
    let answer = read();
    assert_eq!(
        answer["configuration"]["local"],
        serde_json::json!({"state": "not-taken", "keys": ["pools.work"]})
    );
    assert_eq!(answer["streams"]["work"]["agents"], 6);
    assert_eq!(answer["label"]["name"], "construction");
    assert_eq!(
        value(&answer, "streams.work.agents"),
        (serde_json::json!("6"), serde_json::json!("managed"))
    );
    assert_eq!((as_it_stands(&managed), as_it_stands(&local)), before);

    std::fs::remove_file(&local).unwrap();
    let before = as_it_stands(&managed);
    let answer = read();
    assert_eq!(answer["configuration"]["local"]["state"], "no-file");
    assert_eq!(answer["streams"]["work"]["agents"], 6);
    assert_eq!(as_it_stands(&managed), before);
    assert!(!local.exists(), "a read made the local file");
}

const CHILD_STATE: &str = "FLUME_TEST_ROOM_STATE";
const CHILD_TAKEN: &str = "FLUME_TEST_ROOM_TAKEN";
const CHILD_FREED: &str = "FLUME_TEST_ROOM_FREED";
const CHILD: &str = "a_child_process_holds_the_room";

/// The child's half of the room-lock test: takes the room, says so,
/// holds it a second and says when it lets it go. Run by the test below
/// as a process of its own; it does nothing otherwise.
#[test]
fn a_child_process_holds_the_room() {
    let Some(state) = std::env::var_os(CHILD_STATE) else {
        return;
    };
    let host = RealHost::new(state, HostConfig::default(), None);
    let guard = host.room();
    std::fs::write(std::env::var_os(CHILD_TAKEN).unwrap(), "").unwrap();
    std::thread::sleep(Duration::from_millis(1000));
    std::fs::write(std::env::var_os(CHILD_FREED).unwrap(), "").unwrap();
    drop(guard);
}

#[test]
fn the_room_lock_is_exclusive_between_processes() {
    let scratch = Scratch::new("room");
    let state = scratch.path("state/flume");
    let taken = scratch.path("taken");
    let freed = scratch.path("freed");
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([CHILD, "--exact", "--nocapture", "--test-threads=1"])
        .env(CHILD_STATE, &state)
        .env(CHILD_TAKEN, &taken)
        .env(CHILD_FREED, &freed)
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("a child process");
    let deadline = Instant::now() + Duration::from_secs(30);
    while !taken.exists() {
        assert!(Instant::now() < deadline, "the child never took the room");
        std::thread::sleep(Duration::from_millis(5));
    }
    let host = RealHost::new(&state, HostConfig::default(), None);
    let guard = host.room();
    assert!(
        freed.exists(),
        "the room was taken while another process held it"
    );
    drop(guard);
    assert!(child.wait().unwrap().success());
    assert!(Path::new(&host.room_lock()).is_file());
}

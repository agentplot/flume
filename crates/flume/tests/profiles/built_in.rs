//! The core over the profiles built in (design 16): the engine built over
//! a scratch state directory and host configuration, the resolver from a
//! binding's names to the profiles, the place bound with the merge
//! queue's data, and the terminal the host's configuration names; and
//! the hash of each version of flume's own kinds, which no later release
//! moves (156).

use crate::common::*;
use crate::place_git_worktree::{equip_answer, stand_in_sluice};
use flume::host::config::HostConfig;
use flume::profiles::kinds::weir::{Weir, OWN_SOURCE};
use flume::profiles::{core_over, terminal, BuiltIn, DEFAULT_PROGRAM, SEND_BACKS};
use flume_core::Answer;
use flume_seams::*;
use std::path::Path;
use std::sync::Arc;

/// The resolver, a place's steps run in a scratch home with the test's
/// own `PATH`.
fn built_in(scratch: &Scratch) -> BuiltIn {
    let path = std::env::var("PATH").unwrap_or_default();
    let home = scratch.path("home");
    std::fs::create_dir_all(&home).expect("a scratch home");
    BuiltIn::with_login(&[
        ("HOME", &home.to_string_lossy()),
        ("PATH", &path),
        ("LANG", "C"),
    ])
}

/// A binding naming `merge_queue`, the place left to the one that needs
/// nothing beyond git.
fn binding(merge_queue: Option<&str>) -> Binding {
    Binding {
        merge_queue: merge_queue.map(str::to_string),
        ..Binding::default()
    }
}

/// A place of `repository` on `fix/<name>` made through `seam`, where
/// the seam says.
fn made(seam: &dyn Place, repository: &Repository, name: &str) -> PlaceRef {
    let place = PlaceRef {
        repository: repository.name(),
        location: seam
            .location(&repository.name(), &format!("fix--{name}"))
            .expect("the place's location"),
        branch: format!("fix/{name}"),
        base: "main".into(),
    };
    let outcome = seam.make(
        &place.repository,
        &Revision::new(repository.head()),
        &place.branch,
        &place.location,
    );
    assert_eq!(outcome, MakeOutcome::Made, "make {name}");
    place
}

#[test]
fn the_engine_is_built_over_a_scratch_state_and_configuration() {
    let repository = Repository::new("built");
    let scratch = &repository.scratch;
    let state = scratch.path("state");
    let config = scratch.path("config.toml");
    std::fs::write(&config, "[host]\nagents = 2\n").expect("the host's configuration");
    let config = HostConfig::read(&config).expect("the configuration read");
    let core = core_over(&state, config, None, built_in(scratch)).expect("the core");

    assert_eq!(core.defaults().send_backs, SEND_BACKS);
    assert_eq!(
        core.defaults().settings.program.as_deref(),
        Some(DEFAULT_PROGRAM)
    );
    let Answer::Status { places, host } = core.status(None) else {
        panic!("status answered otherwise");
    };
    assert!(places.is_empty());
    assert_eq!(host.bound, Some(2));

    // A kind that does not land, launched from a declaration the launch
    // carries: the binding is read, the place bound, the base read, the
    // kind resolved and its agent bound to Claude Code's profile, whose
    // command the printed terminal gives, its plugin written under the
    // login environment's home (2, 32, C.4).
    std::fs::create_dir_all(scratch.path("kinds/prompts")).expect("prompts");
    std::fs::write(scratch.path("kinds/prompts/code.md"), "{{brief}}").expect("the prompt");
    std::fs::write(
        scratch.path("kinds/kinds.toml"),
        r#"[kinds.survey]
version = 1
branch = "survey"

[[kinds.survey.stages]]
name = "code"
prompt = "prompts/code.md"
deliverables = ["commits"]
agents = [{}]
"#,
    )
    .expect("the declaration");
    let answer = core.launch(&Launch {
        repository: repository.name(),
        kind: "survey".into(),
        name: "first".into(),
        base: "main".into(),
        brief: Some("look around".into()),
        declaration: Some(scratch.path("kinds/kinds.toml").to_string_lossy().into()),
        recipients: vec!["caller".into()],
        ..Launch::default()
    });
    let Answer::Launched { place } = answer else {
        panic!("the launch answered otherwise: {answer:?}");
    };
    // The place goes under flume's own data directory, the login
    // environment's, as every place does (record 0027).
    let location = Path::new(&place.place.0).to_path_buf();
    assert!(location.is_dir(), "{}", location.display());
    assert!(
        location.starts_with(scratch.path("home/.local/share/flume/places")),
        "{}",
        location.display()
    );
    assert!(location.ends_with("survey--first"));
    let agent = place.agent.expect("the stage's agent");
    assert_eq!(agent.program, DEFAULT_PROGRAM);
    let printed = agent.printed.expect("the command printed");
    assert!(printed.contains("--plugin-dir"), "{printed}");
    assert!(
        printed.contains(
            &*scratch
                .path("home/.local/share/flume/plugins")
                .to_string_lossy()
        ),
        "{printed}"
    );
}

#[test]
fn the_resolver_names_the_profiles_built_in() {
    let scratch = Scratch::new("names");
    let resolver = built_in(&scratch);
    assert_eq!(resolver.place_profiles(), vec!["git-worktree".to_string()]);
    let named = |place: &str| Binding {
        place: Some(place.into()),
        ..Binding::default()
    };
    assert!(resolver.place("/r", &named("git-worktree")).is_some());
    assert!(resolver.place("/r", &Binding::default()).is_some());
    assert!(resolver.place("/r", &named("worktrunk")).is_none());
    assert!(resolver.change("openspec").is_some());
    assert!(resolver.change("other").is_none());
    assert!(resolver.merge_queue("sluice").is_some());
    assert!(resolver.merge_queue("other").is_none());
    assert!(resolver.agent(DEFAULT_PROGRAM).is_some());
    assert!(resolver.agent("other").is_none());
}

#[test]
fn a_terminal_the_host_names_is_bound_or_refused() {
    let scratch = Scratch::new("terminal");
    let state = scratch.path("state");
    for name in [None, Some("printed"), Some("herdr")] {
        assert!(terminal(name, &state, None).is_ok(), "{name:?}");
    }
    let Err(reason) = terminal(Some("tmux"), &state, None) else {
        panic!("a terminal no profile is named was bound");
    };
    assert!(
        reason.contains("tmux") && reason.contains("printed, herdr"),
        "{reason}"
    );
    let config = HostConfig {
        terminal: Some("tmux".into()),
        ..HostConfig::default()
    };
    assert!(core_over(&state, config, None, built_in(&scratch)).is_err());
}

#[test]
fn terminals_and_windows_are_given_by_name() {
    // 105, 188, 190; design 2; the seam-contracts scenario "The
    // terminals built in": the host's own terminal the engine's very one,
    // the Rex profile by its name, built without starting a server, each
    // window profile by its name, and none for a name no profile holds.
    let scratch = Scratch::new("by-name");
    let state = scratch.path("state");
    let resolver = built_in(&scratch).over(&state);
    let herdr = resolver.terminal("herdr").expect("herdr by its name");
    let again = resolver.terminal("herdr").expect("herdr again");
    assert!(Arc::ptr_eq(&herdr, &again), "one herdr for the command");
    let rex = resolver.terminal("rex").expect("Rex by its name");
    assert!(rex.sends(), "the Rex profile types into a running agent");
    assert!(
        !state.join("terminal/rex").exists(),
        "building the Rex profile made something under the state"
    );
    assert!(resolver.terminal("cmux").is_none());
    assert_eq!(resolver.terminal_profiles(), ["printed", "herdr", "rex"]);
    assert_eq!(
        resolver.window_profiles(),
        ["ghostty", "terminal", "printed"]
    );
    for name in ["ghostty", "terminal", "printed"] {
        assert!(resolver.window(name).is_some(), "{name}");
    }
    assert!(resolver.window("kitty").is_none());
    let printed = resolver.window("printed").expect("the printed window");
    let command = ViewCommand {
        program: "/bin/herdr".into(),
        arguments: vec!["session".into(), "attach".into(), "w".into()],
        ..ViewCommand::default()
    };
    assert_eq!(
        printed.show(&command, "Work Herdr"),
        WindowShown::Printed {
            line: "/bin/herdr session attach w".into()
        }
    );
}

#[test]
fn a_place_is_readied_as_the_merge_queue_equips_it() {
    let repository = Repository::new("readied");
    repository.write(".config/sluice.toml", "[branches.main]\n");
    repository.commit("a policy");
    let (path, _) = stand_in_sluice(
        &repository.scratch,
        &equip_answer(&[("toolchain", true)]),
        0,
    );
    let home = repository.scratch.path("home");
    std::fs::create_dir_all(&home).expect("a scratch home");
    let resolver = BuiltIn::with_login(&[
        ("HOME", &home.to_string_lossy()),
        ("PATH", &path),
        ("LANG", "C"),
    ]);

    let queued = resolver
        .place(&repository.name(), &binding(Some("sluice")))
        .expect("the place bound with sluice's data");
    let place = made(queued.as_ref(), &repository, "queued");
    let readiness = queued.ready(&place, &Revision::new(repository.head()));
    let steps: Vec<(&str, bool)> = readiness
        .steps
        .iter()
        .map(|step| (step.step.as_str(), step.passed))
        .collect();
    assert_eq!(steps, vec![("toolchain", true)], "{readiness:?}");

    let unqueued = resolver
        .place(&repository.name(), &binding(None))
        .expect("the place with no merge queue");
    let place = made(unqueued.as_ref(), &repository, "unqueued");
    assert!(unqueued
        .ready(&place, &Revision::new(repository.head()))
        .steps
        .is_empty());
    assert_eq!(unqueued.remove(&place), Removed::Removed);
}

#[test]
fn a_branch_is_judged_against_the_base_where_the_policy_lands_it() {
    let origin = Repository::new("landing-origin");
    origin.write(
        ".config/sluice.toml",
        "[branches.main]\nremote = \"origin\"\n",
    );
    origin.commit("a policy landing main on origin");
    let remote = origin.bare_remote();
    let repository = Repository::clone_of(&remote, Scratch::new("landing-clone"));
    let resolver = built_in(&repository.scratch);
    let local = resolver
        .place(&repository.name(), &binding(None))
        .expect("the place with no merge queue");
    let landing = resolver
        .place(&repository.name(), &binding(Some("sluice")))
        .expect("the place bound with sluice's data");

    let place = made(local.as_ref(), &repository, "landed");
    let dir = Path::new(&place.location);
    std::fs::write(dir.join("landed.rs"), "work\n").expect("the work");
    git(dir, &["add", "landed.rs"]);
    git(dir, &["commit", "--quiet", "-m", "landed.rs"]);
    let commit = git(dir, &["rev-parse", "HEAD"]);
    assert_eq!(local.remove(&place), Removed::Removed);
    repository.git(&["cherry-pick", &commit]);
    repository.git(&["push", "--quiet", "origin", "main"]);
    repository.git(&["reset", "--quiet", "--hard", "HEAD~1"]);
    repository.git(&["fetch", "--quiet", "origin"]);

    assert!(matches!(
        local.remove_branch(&repository.name(), "fix/landed", "main"),
        BranchRemoved::Kept { .. }
    ));
    assert_eq!(
        landing.remove_branch(&repository.name(), "fix/landed", "main"),
        BranchRemoved::Removed
    );
}

#[test]
fn a_recipient_unnamed_is_the_herdr_pane_the_command_runs_in() {
    // 7, 43, C.5, design 7: flume inbox, under a host whose terminal is
    // herdr, with no recipient named and none in its environment,
    // collects for the pane herdr's environment names; no place stands,
    // so herdr is asked nothing.
    let scratch = Scratch::new("recipient-herdr");
    for dir in ["state", "config/flume", "home"] {
        std::fs::create_dir_all(scratch.path(dir)).expect("the host's directory");
    }
    std::fs::write(
        scratch.path("config/flume/config.toml"),
        "[host]\nterminal = \"herdr\"\n",
    )
    .expect("the configuration");
    let socket = scratch.path("no-herdr.sock");
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_flume"));
    for (name, _) in std::env::vars() {
        if name.starts_with("FLUME_") || name.starts_with("HERDR_") || name.starts_with("REX_") {
            command.env_remove(name);
        }
    }
    let output = command
        .current_dir(scratch.path("home"))
        .env("XDG_STATE_HOME", scratch.path("state"))
        .env("XDG_CONFIG_HOME", scratch.path("config"))
        .env("HOME", scratch.path("home"))
        .env("HERDR_PANE_ID", "w1-p2")
        .env("HERDR_SOCKET_PATH", &socket)
        .args(["inbox", "--json"])
        .output()
        .expect("flume runs");
    let text = String::from_utf8_lossy(&output.stdout);
    assert_eq!(output.status.code(), Some(0), "{text}");
    let answer: Answer = serde_json::from_str(text.trim()).expect("the answer's JSON");
    assert_eq!(
        answer,
        Answer::Inbox {
            recipient: format!("herdr:w1-p2@{}", socket.display()),
            ends: vec![],
            waiting: vec![],
        }
    );
}

/// The content hash of each version of flume's own kinds as this release
/// records it, the first that reads locks. A version keeps its content in
/// every later release, so a change to what one holds is a version of its
/// own (156; design 7).
const OWN_HASHES: [(&str, &str); 5] = [
    (
        "flume/change@1",
        "sha256:2dbf6a11e877872619f56a51c192292b306dc7a8f1b7c2728cccdb8992e7d883",
    ),
    (
        "flume/change@2",
        "sha256:c47df4b93a579d76b62c1cf284af218bbc724ae81be67099c678a2cf5765e8ec",
    ),
    (
        "flume/change@3",
        "sha256:faf88896b4bf2be5e0ae9980bc5e17265909f8a6eea3468393548b483c8cb5d3",
    ),
    (
        "flume/chore@1",
        "sha256:49c9a204c18d27348d6ed0c1bb6dbb894bc8816a9ae05c1de3ce7d65e4b61fda",
    ),
    (
        "flume/fix@1",
        "sha256:59dee7a3e1c11f8f0f20abe807812e31cb2398cce047c2a2066636d0e8bfdcf8",
    ),
];

#[test]
fn the_hashes_recorded() {
    // 156; design 7: each version of flume's own hashes to the value this
    // release records, built in as its directory under `own/` reads as a
    // package, and a version whose hash moves is named.
    let own_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/profiles/kinds/weir/own");
    let own = Weir.flumes_own();
    let mut faults = vec![];
    for package in &own {
        let identity = &package.identity;
        match Weir.read_package(
            OWN_SOURCE,
            &own_dir.to_string_lossy(),
            &identity.name,
            identity.version,
        ) {
            PackageRead::Read { package: read } if read.hash == package.hash => {}
            PackageRead::Read { package: read } => faults.push(format!(
                "{identity} is built in at {} and its directory hashes to {}: the binary holds \
                 another content than own/ does",
                package.hash, read.hash
            )),
            PackageRead::CouldNotRead { reason } => {
                faults.push(format!("{identity}'s directory: {reason}"))
            }
        }
        match OWN_HASHES
            .iter()
            .find(|(recorded, _)| *recorded == identity.spelled)
        {
            Some((_, hash)) if *hash == package.hash => {}
            Some((_, hash)) => faults.push(format!(
                "{identity} hashes to {} where this release records {hash}: a version's content \
                 never changes, so what it holds now is a version of its own (156)",
                package.hash
            )),
            None => faults.push(format!(
                "{identity} hashes to {}, and no hash of it is recorded here",
                package.hash
            )),
        }
    }
    for (recorded, _) in OWN_HASHES {
        if !own
            .iter()
            .any(|package| package.identity.spelled == recorded)
        {
            faults.push(format!(
                "{recorded} is recorded here and flume holds it no more"
            ));
        }
    }
    assert!(faults.is_empty(), "{}", faults.join("\n"));
}

#[test]
fn flumes_own_kinds_declare_no_label() {
    // 180: a label is the host's to give flume's own kinds, and none of
    // them declares one.
    use flume::profiles::kinds::weir::Weir;
    let own = Weir.flumes_own();
    assert!(!own.is_empty(), "flume holds no kind of its own");
    for package in own {
        assert_eq!(
            package.kind.label, None,
            "{} declares a label",
            package.identity
        );
    }
}

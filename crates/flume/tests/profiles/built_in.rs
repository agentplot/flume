//! The core over the profiles built in (design 16): the engine built over
//! a scratch state directory and host configuration, the resolver from a
//! binding's names to the profiles, the place bound with the merge
//! queue's data, and the terminal the host's configuration names.

use crate::common::*;
use flume::host::config::HostConfig;
use flume::profiles::{core_over, terminal, BuiltIn, DEFAULT_PROGRAM, SEND_BACKS};
use flume_core::Answer;
use flume_seams::*;
use std::path::Path;

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

/// A place of `repository` on `fix/<name>` made through `seam`.
fn made(seam: &dyn Place, repository: &Repository, name: &str) -> PlaceRef {
    let place = PlaceRef {
        repository: repository.name(),
        location: format!("{}/places/fix--{name}", repository.name()),
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
        ..Launch::default()
    });
    let Answer::Launched { place } = answer else {
        panic!("the launch answered otherwise: {answer:?}");
    };
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
    assert!(repository.dir.join("places/survey--first").is_dir());
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
        assert!(terminal(name, &state).is_ok(), "{name:?}");
    }
    let Err(reason) = terminal(Some("tmux"), &state) else {
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
fn a_place_is_readied_as_the_merge_queues_policy_says() {
    let repository = Repository::new("readied");
    repository.write(
        ".config/sluice.toml",
        "[tests]\ntoolchain = \"printf 'ANSWER=42\\\\n'\"\n",
    );
    repository.commit("a policy");
    let resolver = built_in(&repository.scratch);

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

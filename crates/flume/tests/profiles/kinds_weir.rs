//! The kinds profile over weir's kind format (C.2; design 7): the kinds
//! seam's scenarios against it on scratch repositories, S13, S33, a
//! stage's repeat (140; weir 53, 57), an elaboration section, the binding
//! from the default branch, a place profile flume does not hold, flume's
//! own kinds, one declaration per version with change@1 and change@2
//! side by side, and what their prompts ask (22, 141, 157).

use crate::common::engine::{base_at, engine, PLACE_PROFILE};
use crate::common::*;
use flume::profiles::kinds::weir::{binding_at, Weir, DECLARATION};
use flume_conformance::engine::repeating_stages::GROUPS;
use flume_conformance::engine::settling::{archive, change_at_verify, deliver_fix, fix_begun};
use flume_conformance::engine::world::{launch, launched, refused, World, REPOSITORY};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::kinds::{self, KindsHarness};
use flume_core::Answer;
use flume_fakes::Plan;
use flume_seams::*;
use std::cell::RefCell;
use std::path::Path;
use std::sync::Arc;

/// A kind written in the format, each stage's prompt a file beside the
/// declaration under `prompts/`.
fn written(kinds: &[Kind]) -> (String, Vec<(String, String)>) {
    let mut text = String::new();
    let mut prompts = vec![];
    for kind in kinds {
        text.push_str(&format!(
            "[kinds.{}]\nversion = {}\nbranch = \"{}\"\nstarts_unready = {}\nlands = {}\n\n",
            kind.name, kind.version, kind.branch_prefix, kind.starts_unready, kind.lands
        ));
        for stage in &kind.stages {
            let prompt = format!("prompts/{}-{}.md", kind.name, stage.name);
            prompts.push((prompt.clone(), stage.prompt.clone()));
            text.push_str(&format!(
                "[[kinds.{}.stages]]\nname = \"{}\"\nprompt = \"{prompt}\"\n",
                kind.name, stage.name
            ));
            if stage.conversation {
                text.push_str("conversation = true\n");
            } else {
                let spelled: Vec<String> = stage
                    .deliverables
                    .iter()
                    .map(|deliverable| {
                        format!(
                            "\"{}\"",
                            match deliverable {
                                Deliverable::Commits => "commits".to_string(),
                                Deliverable::ChangeWritten => "change".into(),
                                Deliverable::TasksTicked => "tasks".into(),
                                Deliverable::ChangeArchived => "archived".into(),
                                Deliverable::Report { path } => format!("report:{path}"),
                                Deliverable::CommittedReport { path } =>
                                    format!("committed:{path}"),
                                Deliverable::Unreadable { spelled } => spelled.clone(),
                            }
                        )
                    })
                    .collect();
                text.push_str(&format!("deliverables = [{}]\n", spelled.join(", ")));
            }
            let agents: Vec<String> = stage
                .agents
                .iter()
                .map(|agent| {
                    let mut fields = vec![];
                    for (name, value) in [
                        ("program", &agent.program),
                        ("model", &agent.model),
                        ("instructions", &agent.instructions),
                        ("beside", &agent.beside),
                    ] {
                        if let Some(value) = value {
                            fields.push(format!("{name} = \"{value}\""));
                        }
                    }
                    format!("{{ {} }}", fields.join(", "))
                })
                .collect();
            text.push_str(&format!("agents = [{}]\n", agents.join(", ")));
            if stage.waits {
                text.push_str("wait = true\n");
            }
            if let Some(quiet) = stage.quiet_limit {
                text.push_str(&format!("quiet = \"{quiet}s\"\n"));
            }
            if let Some(to) = &stage.send_back_to {
                text.push_str(&format!("on_fail = \"{to}\"\n"));
            }
            if let Some(times) = stage.send_back_limit {
                text.push_str(&format!("max_send_backs = {times}\n"));
            }
            if !stage.makes.is_empty() {
                let names: Vec<String> = stage
                    .makes
                    .iter()
                    .map(|name| format!("\"{name}\""))
                    .collect();
                text.push_str(&format!("makes = [{}]\n", names.join(", ")));
            }
            if stage.repeats {
                text.push_str("repeat = \"per-task-group\"\n");
            }
            text.push('\n');
        }
    }
    (text, prompts)
}

/// Writes a declaration of `kinds`, with `more` after it, and its prompts
/// beside it at `file` under `root`.
fn write_declaration(root: &Path, file: &str, kinds: &[Kind], more: &str) {
    let (text, prompts) = written(kinds);
    let at = root.join(file);
    std::fs::create_dir_all(at.parent().expect("a directory")).expect("the directory");
    std::fs::write(&at, format!("{more}\n{text}")).expect("the declaration");
    for (prompt, body) in prompts {
        let path = at.parent().expect("a directory").join(prompt);
        std::fs::create_dir_all(path.parent().expect("a directory")).expect("the directory");
        std::fs::write(path, body).expect("the prompt");
    }
}

#[derive(Default)]
struct Harness {
    seam: Weir,
    kept: RefCell<Vec<Repository>>,
    scratch: RefCell<Vec<Scratch>>,
}

impl Harness {
    fn keep(&self, repository: Repository) -> (String, Revision) {
        let named = (repository.name(), Revision::new(repository.head()));
        self.kept.borrow_mut().push(repository);
        named
    }
}

impl KindsHarness for Harness {
    type Seam = Weir;

    fn seam(&self) -> &Weir {
        &self.seam
    }

    fn declaring(&self, kinds: &[Kind]) -> (String, Revision) {
        let repository = Repository::new("kinds");
        write_declaration(&repository.dir, DECLARATION, kinds, "");
        repository.commit("the kinds declared");
        self.keep(repository)
    }

    fn launch_declaration(&self, kinds: &[Kind]) -> String {
        let scratch = Scratch::new("launch");
        write_declaration(&scratch.root, "launch/kinds.toml", kinds, "");
        let file = scratch.path("launch/kinds.toml");
        self.scratch.borrow_mut().push(scratch);
        file.to_string_lossy().into_owned()
    }

    fn unreadable(&self) -> (String, Revision) {
        let repository = Repository::new("unreadable");
        repository.write(
            DECLARATION,
            "[kinds.bump]\nversion = 0\n\n[[kinds.bump.stages]]\nname = \"code\"\n",
        );
        repository.commit("a declaration weir refuses");
        self.keep(repository)
    }

    fn binding(&self, binding: &Binding) -> String {
        let repository = Repository::new("binding");
        repository.write(DECLARATION, &profiles(binding));
        repository.commit("the binding");
        self.keep(repository).0
    }
}

fn profiles(binding: &Binding) -> String {
    let mut text = String::from("[profiles]\n");
    for (seam, name) in [
        ("place", &binding.place),
        ("change", &binding.change),
        ("merge_queue", &binding.merge_queue),
    ] {
        if let Some(name) = name {
            text.push_str(&format!("{seam} = \"{name}\"\n"));
        }
    }
    text
}

#[test]
fn the_kinds_profile_passes_the_kinds_scenarios() {
    pass_all("kinds", &kinds::scenarios(), Harness::default);
}

#[test]
fn a_declaration_weir_refuses_names_every_field_that_fails() {
    let repository = Repository::new("refused");
    repository.write(
        DECLARATION,
        "[kinds.bump]\nversion = 0\n\n[[kinds.bump.stages]]\nname = \"code\"\nprompt = \"p.md\"\nquiet = \"1h30m\"\n",
    );
    let revision = Revision::new(repository.commit("refused"));
    match Weir.read_kinds(&repository.name(), &revision) {
        KindsRead::CouldNotRead { reason } => {
            for field in [
                "kinds.bump.version",
                "kinds.bump.stages[0].deliverables",
                "kinds.bump.stages[0].quiet",
            ] {
                assert!(reason.contains(field), "{field} is not named in {reason}");
            }
        }
        other => panic!("a refused declaration read as {other:?}"),
    }
}

#[test]
fn a_prompt_the_revision_lacks_is_named() {
    let repository = Repository::new("no-prompt");
    repository.write(
        DECLARATION,
        "[kinds.bump]\n\n[[kinds.bump.stages]]\nname = \"code\"\nprompt = \"prompts/gone.md\"\ndeliverables = [\"commits\"]\nagents = [{}]\n",
    );
    let revision = Revision::new(repository.commit("no prompt"));
    match Weir.read_kinds(&repository.name(), &revision) {
        KindsRead::CouldNotRead { reason } => {
            assert!(
                reason.contains("kinds.bump.stages[0].prompt")
                    && reason.contains(".config/prompts/gone.md"),
                "{reason}"
            );
        }
        other => panic!("a missing prompt read as {other:?}"),
    }
}

#[test]
fn a_repository_with_no_declaration_declares_no_kind() {
    let repository = Repository::new("none");
    let revision = Revision::new(repository.head());
    assert_eq!(
        Weir.read_kinds(&repository.name(), &revision),
        KindsRead::Read {
            kinds: vec![],
            settings: Default::default()
        }
    );
    assert_eq!(
        Weir.read_binding(&repository.name()),
        BindingRead::Read {
            binding: Binding::default()
        }
    );
}

#[test]
fn a_repositorys_settings_for_a_kind_it_does_not_declare_and_a_kinds_own() {
    let repository = Repository::new("settings");
    repository.write(
        DECLARATION,
        r#"[kinds.change.agents]
model = "opus"

[kinds.change.agents.stages.verify]
instructions = "Read every task."

[kinds.bump.agents]
program = "agent"
model = "sonnet"

[kinds.bump.agents.stages.code]
model = "haiku"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ instructions = "keep it short" }]
"#,
    );
    repository.write(".config/prompts/bump-code.md", "{{brief}}");
    repository.commit("settings for change@1, and bump's own");
    let revision = Revision::new(repository.head());
    let KindsRead::Read { kinds, settings } = Weir.read_kinds(&repository.name(), &revision) else {
        panic!("the declaration did not read");
    };
    assert_eq!(
        kinds
            .iter()
            .map(|kind| kind.name.as_str())
            .collect::<Vec<_>>(),
        vec!["bump"],
        "settings declare no kind"
    );
    assert_eq!(
        settings.keys().collect::<Vec<_>>(),
        vec!["change"],
        "settings apart from the kinds"
    );
    let change = &settings["change"];
    assert_eq!(change.every_stage.model.as_deref(), Some("opus"));
    assert_eq!(
        change.for_stage("verify").instructions.as_deref(),
        Some("Read every task.")
    );
    let bump = &kinds[0];
    assert_eq!(bump.agents.every_stage.program.as_deref(), Some("agent"));
    assert_eq!(
        bump.agents.for_stage("code").model.as_deref(),
        Some("haiku")
    );
    assert_eq!(
        bump.stages[0].agents[0].instructions.as_deref(),
        Some("keep it short"),
        "the stage's own beside the kind's"
    );
}

/// A one-stage kind `bump` of the repository's at `version`, not landing.
fn bump(version: u32, stages: &[&str]) -> Kind {
    Kind {
        name: "bump".into(),
        version,
        branch_prefix: "bump".into(),
        starts_unready: false,
        lands: false,
        agents: KindSettings::default(),
        stages: stages
            .iter()
            .map(|name| Stage {
                name: (*name).into(),
                prompt: "{{brief}}\n\n{{needs}}".into(),
                conversation: false,
                deliverables: vec![Deliverable::Commits],
                agents: vec![Settings::default()],
                waits: false,
                quiet_limit: None,
                send_back_to: None,
                send_back_limit: None,
                makes: vec![],
                repeats: false,
            })
            .collect(),
    }
}

#[test]
fn s13_a_place_keeps_the_kind_its_launch_resolved() {
    let repository = Repository::new("s13");
    let name = repository.name();
    write_declaration(&repository.dir, DECLARATION, &[bump(1, &["code"])], "");
    let first = repository.commit("bump at version 1");
    let world = World::new();
    world.terminal.comes_up(AgentState::Waiting);
    base_at(&world, &name, &first);
    let at_launch = |place: &str| {
        let mut launched = launch("bump", place, Some("bump the pin"));
        launched.repository = name.clone();
        launched
    };
    let one = launched(&engine(&world, Arc::new(Weir)).launch(&at_launch("one")))
        .expect("the first launch");
    write_declaration(
        &repository.dir,
        DECLARATION,
        &[bump(2, &["code", "check"])],
        "",
    );
    let second = repository.commit("bump at version 2");
    base_at(&world, &name, &second);
    let two = launched(&engine(&world, Arc::new(Weir)).launch(&at_launch("two")))
        .expect("the second launch");
    assert_eq!((one.version, two.version), (1, 2), "the places' versions");
    assert_eq!(one.declared, Declared::Repository);
    let places = match engine(&world, Arc::new(Weir)).status(None) {
        Answer::Status { places, .. } => places,
        other => panic!("status answered {other:?}"),
    };
    let kinds: Vec<(String, u32, usize)> = places
        .iter()
        .map(|place| (place.name.clone(), place.version, place.stages.len()))
        .collect();
    assert_eq!(
        kinds,
        vec![("one".to_string(), 1, 1), ("two".to_string(), 2, 2)],
        "each place's kind at its version, with its stages"
    );
}

#[test]
fn s33_a_repository_with_no_table_refuses_a_fix_naming_the_merge_queue_it_needs() {
    let repository = Repository::new("s33");
    let name = repository.name();
    let world = World::new();
    base_at(&world, &name, &repository.head());
    let mut fix = launch("fix", "one", Some("x"));
    fix.repository = name.clone();
    let reason = refused(&engine(&world, Arc::new(Weir)).launch(&fix)).expect("the launch refused");
    assert!(
        reason.contains("fix") && reason.contains("merge queue"),
        "the refusal names not the kind and the merge queue: {reason}"
    );
    assert_eq!(world.place.log.count("make"), 0, "places made");
    assert!(world.store.places().is_empty(), "held state was left");
}

/// A declaration of `kind` whose stage `code` delivers `delivers` and
/// repeats per task group, beside stages that declare no repeat.
fn repeating(kind: &str, delivers: &str) -> String {
    format!(
        r#"[kinds.{kind}]

[[kinds.{kind}.stages]]
name = "construct"
prompt = "prompts/construct.md"
deliverables = ["change"]
agents = [{{}}]

[[kinds.{kind}.stages]]
name = "code"
prompt = "prompts/code.md"
deliverables = ["{delivers}"]
agents = [{{}}]
repeat = "per-task-group"

[[kinds.{kind}.stages]]
name = "verify"
prompt = "prompts/verify.md"
deliverables = ["archived"]
agents = [{{}}]
"#
    )
}

/// A scratch repository declaring `declaration`, each stage's prompt
/// beside it, and its head.
fn declaring_repeat(name: &str, declaration: &str) -> (Repository, String) {
    let repository = Repository::new(name);
    repository.write(DECLARATION, declaration);
    for stage in ["construct", "code", "verify"] {
        repository.write(
            &format!(".config/prompts/{stage}.md"),
            "{{brief}}\n\n{{needs}}",
        );
    }
    let head = repository.commit("a stage that repeats per task group");
    (repository, head)
}

#[test]
fn a_repeating_stage_is_read_through_weirs_crate() {
    let (repository, head) = declaring_repeat("repeat", &repeating("build", "tasks"));
    let KindsRead::Read { kinds, .. } = Weir.read_kinds(&repository.name(), &Revision::new(head))
    else {
        panic!("the declaration did not read");
    };
    let repeats: Vec<(&str, bool)> = kinds[0]
        .stages
        .iter()
        .map(|stage| (stage.name.as_str(), stage.repeats))
        .collect();
    assert_eq!(
        repeats,
        vec![("construct", false), ("code", true), ("verify", false)],
        "the stage delivering tasks repeats, and every other stage not"
    );
}

#[test]
fn a_repeat_on_a_stage_of_commits_refuses_the_launch_naming_the_field() {
    let (repository, head) = declaring_repeat("repeat-commits", &repeating("build", "commits"));
    let name = repository.name();
    match Weir.read_kinds(&name, &Revision::new(head.clone())) {
        KindsRead::CouldNotRead { reason } => assert!(
            reason.contains("kinds.build.stages[1].repeat"),
            "the read names not the stage's repeat: {reason}"
        ),
        other => panic!("a repeat on a stage of commits read as {other:?}"),
    }
    let world = World::new();
    base_at(&world, &name, &head);
    let mut build = launch("build", "one", Some("x"));
    build.repository = name;
    let reason =
        refused(&engine(&world, Arc::new(Weir)).launch(&build)).expect("the launch refused");
    assert!(
        reason.contains("kinds.build.stages[1].repeat"),
        "the refusal names not the stage's repeat: {reason}"
    );
    assert_eq!(world.place.log.count("make"), 0, "places made");
    assert!(world.store.places().is_empty(), "held state was left");
}

#[test]
fn a_worktrunk_binding_is_refused_naming_the_place_profiles_there_are() {
    let repository = Repository::new("worktrunk");
    repository.write(
        DECLARATION,
        "[profiles]\nplace = \"worktrunk\"\n\n[kinds.notes]\n\n[[kinds.notes.stages]]\nname = \"write\"\nprompt = \"prompts/write.md\"\ndeliverables = [\"commits\"]\nagents = [{}]\n",
    );
    repository.write(".config/prompts/write.md", "{{brief}}");
    let head = repository.commit("a worktrunk binding");
    let name = repository.name();
    assert_eq!(
        binding_at(&name, &Revision::new(head.clone())),
        BindingRead::Read {
            binding: Binding {
                place: Some("worktrunk".into()),
                ..Binding::default()
            }
        }
    );
    let world = World::new();
    base_at(&world, &name, &head);
    let mut notes = launch("notes", "one", Some("x"));
    notes.repository = name;
    let reason =
        refused(&engine(&world, Arc::new(Weir)).launch(&notes)).expect("the launch refused");
    assert!(
        reason.contains("worktrunk") && reason.contains(PLACE_PROFILE),
        "the refusal names not the profile asked for and the ones there are: {reason}"
    );
    assert_eq!(world.place.log.count("make"), 0, "places made");
    assert!(world.store.places().is_empty(), "held state was left");
}

#[test]
fn an_elaboration_section_is_ignored() {
    let scratch = Scratch::new("elaboration");
    std::fs::create_dir_all(scratch.path("prompts")).expect("prompts");
    std::fs::write(scratch.path("prompts/round.md"), "{{brief}}\n\n{{needs}}").expect("prompt");
    std::fs::write(
        scratch.path("kinds.toml"),
        r#"[kinds.sounding]
version = 3
branch = "sounding"

[kinds.sounding.elaboration]
rounds = 2
asks = ["a person", "a model"]

[[kinds.sounding.stages]]
name = "round"
prompt = "prompts/round.md"
deliverables = ["report:{{state}}/conclusion.md"]
agents = [{ program = "agent", effort = "high" }]
makes = ["preview"]
"#,
    )
    .expect("the declaration");
    let read = Weir.read_declaration(&scratch.path("kinds.toml").to_string_lossy());
    let KindsRead::Read { kinds, .. } = read else {
        panic!("an elaboration kind read as {read:?}");
    };
    assert_eq!(kinds.len(), 1);
    let kind = &kinds[0];
    assert_eq!((kind.name.as_str(), kind.version), ("sounding", 3));
    assert!(!kind.lands);
    assert_eq!(
        kind.stages[0].deliverables,
        vec![Deliverable::Report {
            path: "{{state}}/conclusion.md".into()
        }]
    );
    assert_eq!(kind.stages[0].agents[0].program.as_deref(), Some("agent"));
    assert_eq!(kind.stages[0].prompt, "{{brief}}\n\n{{needs}}");
}

#[test]
fn the_binding_is_read_at_origin_head_after_one_fetch() {
    let origin = Repository::new("origin");
    origin.write(DECLARATION, "[profiles]\nplace = \"git-worktree\"\n");
    origin.commit("a binding");
    let remote = origin.bare_remote();
    let clone = Repository::clone_of(&remote, Scratch::new("clone"));
    origin.write(
        DECLARATION,
        "[profiles]\nplace = \"git-worktree\"\nmerge_queue = \"sluice\"\n",
    );
    origin.commit("the merge queue bound");
    origin.git(&["push", "--quiet", &remote.to_string_lossy(), "main"]);
    assert_eq!(
        Weir.read_binding(&clone.name()),
        BindingRead::Read {
            binding: Binding {
                place: Some("git-worktree".into()),
                change: None,
                merge_queue: Some("sluice".into()),
            }
        },
        "the binding at origin/HEAD, which the clone's own main lacks"
    );
    clone.git(&["remote", "set-url", "origin", "/nowhere/remote.git"]);
    match Weir.read_binding(&clone.name()) {
        BindingRead::CouldNotRead { reason } => {
            assert!(reason.contains("fetch"), "{reason}")
        }
        other => panic!("a failed fetch read as {other:?}"),
    }
}

#[test]
fn the_binding_is_read_beside_a_bare_repository() {
    let origin = Repository::new("bare-layout");
    origin.write(DECLARATION, "[profiles]\nchange = \"openspec\"\n");
    origin.commit("a binding");
    let holder = origin.scratch.path("project");
    std::fs::create_dir_all(&holder).expect("the project's directory");
    git(
        &origin.scratch.root,
        &[
            "clone",
            "--quiet",
            "--bare",
            &origin.name(),
            &holder.join("project.git").to_string_lossy(),
        ],
    );
    git(&holder.join("project.git"), &["remote", "remove", "origin"]);
    assert_eq!(
        Weir.read_binding(&holder.to_string_lossy()),
        BindingRead::Read {
            binding: Binding {
                change: Some("openspec".into()),
                ..Binding::default()
            }
        }
    );
}

#[test]
fn flumes_own_repository_binds_git_worktree_openspec_and_sluice() {
    crate::common::isolate();
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let repository = repository.canonicalize().expect("flume's repository");
    let head = git(&repository, &["rev-parse", "HEAD"]);
    assert_eq!(
        binding_at(&repository.to_string_lossy(), &Revision::new(head)),
        BindingRead::Read {
            binding: Binding {
                place: Some("git-worktree".into()),
                change: Some("openspec".into()),
                merge_queue: Some("sluice".into()),
            }
        }
    );
}

/// A kind's name, version, branch prefix, whether it starts unready and
/// lands, and its stages with their deliverables and whether each
/// repeats.
type Summary = (
    String,
    u32,
    String,
    bool,
    bool,
    Vec<(String, Vec<Deliverable>, bool)>,
);

#[test]
fn flumes_own_kinds_are_built_in() {
    // 22, 140, 157, C.2: fix@1, chore@1, change@1 and change@2, change@2's
    // code stage repeating per task group and change@1's not.
    let own = Weir.flumes_own();
    let summary: Vec<Summary> = own
        .iter()
        .map(|kind| {
            (
                kind.name.clone(),
                kind.version,
                kind.branch_prefix.clone(),
                kind.starts_unready,
                kind.lands,
                kind.stages
                    .iter()
                    .map(|stage| {
                        (
                            stage.name.clone(),
                            stage.deliverables.clone(),
                            stage.repeats,
                        )
                    })
                    .collect(),
            )
        })
        .collect();
    let code = |deliverable| vec![("code".to_string(), vec![deliverable], false)];
    let change = |version: u32, repeats: bool| -> Summary {
        (
            "change".into(),
            version,
            "change".into(),
            false,
            true,
            vec![
                ("construct".into(), vec![Deliverable::ChangeWritten], false),
                ("code".into(), vec![Deliverable::TasksTicked], repeats),
                ("verify".into(), vec![Deliverable::ChangeArchived], false),
            ],
        )
    };
    assert_eq!(
        summary,
        vec![
            change(1, false),
            change(2, true),
            (
                "chore".into(),
                1,
                "chore".into(),
                true,
                true,
                code(Deliverable::Commits)
            ),
            (
                "fix".into(),
                1,
                "fix".into(),
                true,
                true,
                code(Deliverable::Commits)
            ),
        ]
    );
    for kind in &own {
        let last = kind.stages.last().expect("a stage");
        assert_eq!(last.send_back_to.as_deref(), Some("code"), "{}", kind.name);
        for stage in &kind.stages {
            assert_eq!(stage.agents.len(), 1, "{}'s {}", kind.name, stage.name);
            assert!(stage.prompt.contains("{{needs}}"), "{}", stage.name);
        }
    }
}

/// change@1's code prompt as flume held it before change@2 (22, 157).
const CHANGE_1_CODE: &str = "You are the code stage of the change {{name}}, working in the place {{place}} on the branch {{branch}}, which was cut from {{base}}.\n\n{{#words}}This stage has been sent back to you with these words:\n\n{{words}}\n\nWhere the words name files contested and the base's revision the merge queue tested against, take that revision in before anything else: run `git merge <the revision the words name>` in this place, which merges it into {{branch}}, resolve each of the files contested, and commit the merge. Never rebase, and never rewrite a commit already on {{branch}}.\n\nWhere the change has been archived on {{branch}}, move it back out of the archive to {{change}} first. Then fix what the words name, and record the fix as a task of the change, ticked.\n\n{{/words}}Build the change at {{change}}: work through its tasks in order, tick each task in its tasks file as it is done, and commit as you go on {{branch}}. Your deliverable is the tasks ticked: the stage is done once a commit on {{branch}} holds every task of the change ticked and you are quiet. Leave the branch where your last commit puts it; what happens to it after this stage is not yours to do.\n\nIf you cannot go on without an answer, stop short: {{needs}}\n";

/// flume's own kind `name` at `version`, as built in.
fn own(name: &str, version: u32) -> Kind {
    Weir.flumes_own()
        .into_iter()
        .find(|kind| kind.name == name && kind.version == version)
        .unwrap_or_else(|| panic!("flume holds no {name}@{version}"))
}

#[test]
fn each_versions_prompts_are_its_own() {
    // 157, C.2, design 7: each of flume's own versions is a declaration of
    // its own, under `own/<name>@<version>/`, whose prompts lie beside it;
    // what the binary holds is what each directory holds, read as a
    // launch's declaration is read from disk.
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/profiles/kinds/weir/own");
    let mut versions: Vec<String> = std::fs::read_dir(&dir)
        .expect("flume's own kinds' directory")
        .map(|entry| {
            entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    versions.sort();
    assert_eq!(versions, ["change@1", "change@2", "chore@1", "fix@1"]);
    let mut read = vec![];
    for version in &versions {
        let declaration = dir.join(version).join("kind.toml");
        match Weir.read_declaration(&declaration.to_string_lossy()) {
            KindsRead::Read { kinds, .. } => {
                let named: Vec<String> = kinds
                    .iter()
                    .map(|kind| format!("{}@{}", kind.name, kind.version))
                    .collect();
                assert_eq!(named, [version.as_str()], "{version}'s one kind");
                read.extend(kinds);
            }
            KindsRead::CouldNotRead { reason } => panic!("{version}: {reason}"),
        }
    }
    assert_eq!(read, Weir.flumes_own(), "the kinds built in, as on disk");
    let one = own("change", 1);
    let two = own("change", 2);
    assert_eq!(
        one.stages[1].prompt, CHANGE_1_CODE,
        "change@1's code prompt"
    );
    assert_ne!(
        two.stages[1].prompt, CHANGE_1_CODE,
        "change@2's code prompt"
    );
    for stage in [0, 2] {
        assert_eq!(
            one.stages[stage].prompt, two.stages[stage].prompt,
            "change@2's {} is change@1's",
            one.stages[stage].name
        );
    }
    assert_eq!(
        own("fix", 1).stages[0].prompt,
        own("chore", 1).stages[0].prompt
    );
}

/// flume's own kinds with `change` at `version` alone, as a flume that
/// held that version of it and no other.
fn own_at(version: u32) -> Vec<Kind> {
    Weir.flumes_own()
        .into_iter()
        .filter(|kind| kind.name != "change" || kind.version == version)
        .collect()
}

/// A world whose own kinds are the profile's built in, `change` at
/// `version`.
fn world_with_own_kinds(version: u32) -> World {
    let world = World::new();
    world.kinds.set_own(own_at(version));
    world
}

/// The prompt the start `n` of the place's stage was opened with.
fn opened(world: &World, kind: &str, name: &str, stage: &str, n: u32) -> String {
    let id = World::id(kind, name);
    world
        .prompt(&id, &World::identity(kind, name, stage, n))
        .unwrap_or_else(|| panic!("{kind} {name}'s {stage} did not start {n}"))
}

/// What no prompt of flume's own asks for (28, 71, 112, core.5).
fn asks_nothing_it_may_not(prompt: &str) {
    let lower = prompt.to_lowercase();
    for word in ["submit", "push", "land the", "wt "] {
        assert!(!lower.contains(word), "the prompt asks {word:?}: {prompt}");
    }
    for line in lower.lines() {
        if line.contains("rebase") {
            assert!(
                line.contains("never rebase"),
                "a rebase not ruled out: {line}"
            );
        }
    }
    assert!(
        prompt.contains("flume needs"),
        "no way to stop short: {prompt}"
    );
}

#[test]
fn no_prompt_of_flumes_own_asks_to_submit() {
    let mut prompts = vec![];
    for version in [1, 2] {
        let world = world_with_own_kinds(version);
        fix_begun(&world, "one").expect("a fix begun");
        prompts.push(opened(&world, "fix", "one", "code", 1));
        launched(
            &world
                .engine
                .launch(&launch("chore", "two", Some("tidy the lock"))),
        )
        .expect("a chore launched");
        world.settle(&World::id("chore", "two"));
        prompts.push(opened(&world, "chore", "two", "code", 1));
        change_at_verify(&world, "plan").expect("a change at verify");
        let change: Vec<String> = ["construct", "code", "verify"]
            .iter()
            .map(|stage| opened(&world, "change", "plan", stage, 1))
            .collect();
        assert!(change[0].contains("change written"), "change@{version}");
        assert!(change[1].contains("tasks ticked"), "change@{version}");
        assert!(
            change[2].contains("verify.md") && change[2].contains("archive"),
            "change@{version}"
        );
        prompts.extend(change);
    }
    for prompt in &prompts {
        asks_nothing_it_may_not(prompt);
        assert!(
            !prompt.to_lowercase().contains("merge"),
            "a merge asked with no conflict's words: {prompt}"
        );
    }
    assert!(prompts[0].contains("commits") && prompts[1].contains("commits"));
}

/// The code prompt a conflict sends `kind`'s place back with, and the
/// base's revision the conflict names.
fn sent_back_with_a_conflict(world: &World, kind: &str, name: &str) -> (String, Revision) {
    world.commit("main", &["src/lib.rs"]);
    let base = world.git.branch_head(REPOSITORY, "main").expect("the base");
    world.queue.plan_submits(vec![Plan::Conflict(
        vec!["src/lib.rs".into(), "Cargo.lock".into()],
        "src/lib.rs conflicts with main".into(),
    )]);
    let id = World::id(kind, name);
    world.settle(&id);
    (opened(world, kind, name, "code", 2), base)
}

#[test]
fn a_code_prompt_sent_back_with_a_conflict_merges_the_base_never_rebasing() {
    let world = world_with_own_kinds(2);
    let mut prompts = vec![];

    let fix = fix_begun(&world, "one").expect("a fix begun");
    deliver_fix(&world, &fix);
    prompts.push(("fix", sent_back_with_a_conflict(&world, "fix", "one")));

    launched(
        &world
            .engine
            .launch(&launch("chore", "two", Some("tidy the lock"))),
    )
    .expect("a chore launched");
    world.settle(&World::id("chore", "two"));
    world.commit("chore/two", &["chore.rs"]);
    prompts.push(("chore", sent_back_with_a_conflict(&world, "chore", "two")));

    change_at_verify(&world, "plan").expect("a change at verify");
    archive(&world, "plan");
    prompts.push((
        "change",
        sent_back_with_a_conflict(&world, "change", "plan"),
    ));

    for (kind, (prompt, base)) in prompts {
        asks_nothing_it_may_not(&prompt);
        for named in [
            "git merge",
            "src/lib.rs, Cargo.lock",
            &base.to_string(),
            "resolve",
            "commit the merge",
            "before anything else",
            "Never rebase",
        ] {
            assert!(
                prompt.contains(named),
                "{kind}'s code prompt names not {named:?}: {prompt}"
            );
        }
        let merge = prompt.find("git merge").expect("the merge");
        let fix = prompt.find("Then fix").expect("the fix");
        assert!(merge < fix, "{kind}: the merge comes after the fix");
    }
}

/// The change place `name` launched at the change flume holds, written as
/// `GROUPS` none ticked, and code's first start begun.
fn change_code_begun(world: &World, name: &str) -> PlaceId {
    launched(
        &world
            .engine
            .launch(&launch("change", name, Some("plan it"))),
    )
    .expect("a change launched");
    let id = World::id("change", name);
    world.settle(&id);
    world.tick_groups(&format!("change/{name}"), name, GROUPS, 0);
    world.settle(&id);
    assert_eq!(world.stage_state(&id, "code"), Some(StageState::Begun));
    id
}

#[test]
fn two_repetitions_prompts_are_equal_up_to_the_group() {
    // 141, C.2, core.31: change@2's code prompt rendered for group 1 and,
    // by a retry with an answer, for group 2: the texts are equal, byte
    // for byte, up to the first character of the group, and the words
    // come after it.
    let world = world_with_own_kinds(2);
    let id = change_code_begun(&world, "plan");
    let first = opened(&world, "change", "plan", "code", 1);
    world.tick_groups("change/plan", "plan", GROUPS, 2);
    world.settle(&id);
    let second = world.held(&id).expect("the place").stages[1]
        .start
        .clone()
        .expect("code's second start");
    let stopped = world.engine.needs(
        &second.identity.to_string(),
        &second.secret.0,
        "Which schema does task 2.1 mean?",
        StopKind::NeedsAnswer,
    );
    assert!(stopped.refusal().is_none(), "the stop-short: {stopped:?}");
    world.settle(&id);
    let retried = world.engine.retry(
        &id,
        flume_core::Retry::Stage("code".into()),
        Some("the one in design 3".into()),
        false,
    );
    assert!(retried.refusal().is_none(), "the retry: {retried:?}");
    let third = opened(&world, "change", "plan", "code", 3);

    let one = first.find("Group 1. One").expect("group 1 in the first");
    let two = third.find("Group 2. Two").expect("group 2 in the third");
    assert_eq!(one, two, "where each prompt's group begins");
    assert_eq!(&first[..one], &third[..two], "the prompts up to the group");
    assert!(!first[..one].contains("{{"), "a part left unfilled");
    let answer = third.find("the one in design 3").expect("the answer");
    assert!(answer > two, "the retry's words come before the group");
    assert!(
        first[one..].contains("The notes: ") && first.find("flume needs") < Some(one),
        "the way to stop short before the group, the notes after: {first}"
    );
}

#[test]
fn a_place_launched_at_change_1_keeps_it() {
    // 21, 22: a place launched by a flume holding change@1 alone is
    // settled by one holding change@1 and change@2; it keeps change@1's
    // stages, its code stage one agent for the whole change, and status
    // names change@1.
    let world = world_with_own_kinds(1);
    let id = change_code_begun(&world, "old");
    world.kinds.set_own(Weir.flumes_own());
    world.tick_groups("change/old", "old", GROUPS, 2);
    world.settle(&id);
    let held = world.held(&id).expect("the place");
    assert_eq!(held.place.kind.version, 1, "the place's kind");
    assert!(!held.place.kind.stages[1].repeats, "code repeats");
    assert_eq!(held.stages[1].starts, 1, "code's starts");
    assert_eq!(world.stage_state(&id, "code"), Some(StageState::Begun));
    let status = world.status(&id).expect("status");
    assert_eq!((status.kind.as_str(), status.version), ("change", 1));
    assert_eq!(status.stages[1].group, None, "a group named for change@1");
    world.tick_groups("change/old", "old", GROUPS, 5);
    world.settle(&id);
    world.settle(&id);
    assert_eq!(world.stage_state(&id, "verify"), Some(StageState::Begun));
    assert_eq!(world.held(&id).expect("the place").stages[1].starts, 1);
}

//! The kinds profile over weir's kind format (C.2; design 7): the kinds
//! seam's scenarios against it on scratch repositories, S13, S33, a
//! stage's repeat (140; weir 53, 57), an agents table's mode (162, 169),
//! an agents table's session read as no setting (32, 158),
//! an elaboration section, the binding
//! from the default branch, a place profile flume does not hold, flume's
//! own kinds, one declaration per version with change@1 to change@4,
//! chore@1 and chore@2, and fix@1 and fix@2 side by side, what their
//! prompts ask, the newest three's one way to ask and the mode they give
//! (22, 28, 141, 157, 162; record 0064); a
//! host's source's listing, its packages and their hashes wherever they
//! lie, the repository's lock at a revision, a lock written and read
//! back, and the packages refused at another identity, naming a part
//! outside them, an environment or a program by a location (155 to 160,
//! 192; S63); and a kind that lands with no stages beside one that does
//! not and a table of settings (18, 202; weir 51, 57); and such a kind
//! naming its landing fix, by an identity with its version, and its
//! bound, in a repository's declaration and a package, and what weir and
//! flume refuse of them, and a base place's launch holding it, or refused
//! where it resolves to no kind (23, 154, 157, 203; weir 52, 57, 89; weir
//! record 0005).

use crate::common::engine::{base_at, engine, PLACE_PROFILE};
use crate::common::*;
use flume::profiles::kinds::weir::{binding_at, Weir, DECLARATION, LOCK};
use flume_conformance::engine::repeating_stages::GROUPS;
use flume_conformance::engine::settling::{archive, change_at_verify, deliver_fix, fix_begun};
use flume_conformance::engine::world::{launch, launched, refused, World, REPOSITORY};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::kinds::{self, entry, KindsHarness, SourceDir};
use flume_core::Answer;
use flume_fakes::Plan;
use flume_seams::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
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

/// Writes each of `kinds` as a package of its own in `kinds_dir`, at
/// `<name>@<version>/` with its `kind.toml` and its prompts (157, 159).
fn write_packages(kinds_dir: &Path, kinds: &[Kind]) {
    for kind in kinds {
        let at = format!("{}@{}/kind.toml", kind.name, kind.version);
        write_declaration(kinds_dir, &at, std::slice::from_ref(kind), "");
    }
}

/// Writes the package `entry`, `<name>@<version>`, in `kinds_dir`: its
/// `kind.toml` holding `declaration`, and each of `files` by its path
/// within it.
fn write_package(kinds_dir: &Path, entry: &str, declaration: &str, files: &[(&str, &str)]) {
    let dir = kinds_dir.join(entry);
    for (path, text) in std::iter::once(&("kind.toml", declaration)).chain(files) {
        let at = dir.join(path);
        std::fs::create_dir_all(at.parent().expect("a directory")).expect("the directory");
        std::fs::write(at, text).expect("the package's file");
    }
}

/// A lock's text recording `entries`, as C.4 spells one.
fn spelled_lock(entries: &[LockEntry]) -> String {
    entries
        .iter()
        .map(|entry| {
            format!(
                "[[kinds]]\nkind = \"{}\"\nhash = \"{}\"\n\n",
                entry.kind.spelled, entry.hash
            )
        })
        .collect()
}

/// A one-stage kind `name` at `version`, its prompt at `prompts/code.md`,
/// with `more` in its stage's table.
fn reviewing(name: &str, version: u32, more: &str) -> String {
    format!(
        "[kinds.{name}]\nversion = {version}\nbranch = \"{name}\"\n\n\
         [[kinds.{name}.stages]]\nname = \"code\"\nprompt = \"prompts/code.md\"\n\
         deliverables = [\"commits\"]\nagents = [{{}}]\n{more}"
    )
}

#[derive(Default)]
struct Harness {
    seam: Weir,
    kept: RefCell<Vec<Repository>>,
    scratch: RefCell<Vec<Scratch>>,
    /// Directories made read-only, made writable again before their
    /// scratch is removed.
    locked: RefCell<Vec<PathBuf>>,
}

impl Harness {
    fn keep(&self, repository: Repository) -> (String, Revision) {
        let named = (repository.name(), Revision::new(repository.head()));
        self.kept.borrow_mut().push(repository);
        named
    }

    /// A scratch directory kept while the harness lives, and the path
    /// `relative` within it.
    fn scratch(&self, label: &str, relative: &str) -> PathBuf {
        let scratch = Scratch::new(label);
        let path = scratch.path(relative);
        self.scratch.borrow_mut().push(scratch);
        path
    }

    /// A source `acme` whose directory is `kinds_dir`.
    fn acme(kinds_dir: &Path) -> SourceDir {
        SourceDir {
            source: "acme".into(),
            kinds_dir: kinds_dir.to_string_lossy().into_owned(),
        }
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        for dir in self.locked.borrow().iter() {
            let _ = std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o755));
        }
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

    fn source(&self, kinds: &[Kind]) -> Option<SourceDir> {
        let kinds_dir = self.scratch("source", "kinds/acme");
        write_packages(&kinds_dir, kinds);
        Some(Harness::acme(&kinds_dir))
    }

    fn unreadable_source(&self) -> Option<SourceDir> {
        let kinds_dir = self.scratch("unreadable-source", "kinds/acme");
        std::fs::create_dir_all(kinds_dir.parent().expect("a directory")).unwrap();
        std::fs::write(&kinds_dir, "a file, which lists nothing").unwrap();
        Some(Harness::acme(&kinds_dir))
    }

    fn refused_package(&self) -> Option<(SourceDir, String, u32)> {
        let kinds_dir = self.scratch("refused-package", "kinds/acme");
        write_package(
            &kinds_dir,
            "bump@2",
            &reviewing("bump", 1, ""),
            &[("prompts/code.md", "{{brief}}")],
        );
        Some((Harness::acme(&kinds_dir), "bump".into(), 2))
    }

    fn lock_at(&self, entries: &[LockEntry]) -> Option<(String, Revision)> {
        let repository = Repository::new("lock-at");
        repository.write(LOCK, &spelled_lock(entries));
        repository.commit("the lock");
        Some(self.keep(repository))
    }

    fn no_lock_at(&self) -> Option<(String, Revision)> {
        Some(self.keep(Repository::new("no-lock-at")))
    }

    fn lock_file(&self, entries: &[LockEntry]) -> Option<String> {
        let path = self.scratch("lock-file", "flume/flume.lock");
        std::fs::create_dir_all(path.parent().expect("a directory")).unwrap();
        std::fs::write(&path, spelled_lock(entries)).unwrap();
        Some(path.to_string_lossy().into_owned())
    }

    fn no_lock_file(&self) -> Option<String> {
        let path = self.scratch("no-lock-file", "flume/flume.lock");
        Some(path.to_string_lossy().into_owned())
    }

    fn unreadable_lock_file(&self) -> Option<String> {
        let path = self.scratch("unreadable-lock", "flume.lock");
        std::fs::write(&path, "[[kinds]\nkind = ").unwrap();
        Some(path.to_string_lossy().into_owned())
    }

    fn unwritable_lock(&self, entries: &[LockEntry]) -> Option<String> {
        let path = self.scratch("unwritable-lock", "flume/flume.lock");
        let dir = path.parent().expect("a directory").to_path_buf();
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(&path, spelled_lock(entries)).unwrap();
        std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o555)).unwrap();
        self.locked.borrow_mut().push(dir);
        Some(path.to_string_lossy().into_owned())
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
            settings: Default::default(),
            agents: Box::default(),
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
    let KindsRead::Read {
        kinds, settings, ..
    } = Weir.read_kinds(&repository.name(), &revision)
    else {
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

#[test]
fn a_kind_that_lands_with_no_stages_beside_one_that_does_not_and_settings() {
    // The kinds-profile spec's "Three tables over a scratch repository":
    // weir's crate reads `stages = []` on a kind that lands as a kind with
    // no stages, refuses it on one that does not, naming its stages, and
    // reads a table that leaves `stages` out and holds `agents` alone as
    // settings for a kind declared elsewhere, flume's own fix; a refusal
    // refuses the declaration whole (18, 23, 57, 202; weir 51, 57; core.4).
    let repository = Repository::new("no-stages");
    let declared = "[kinds.train]\nbranch = \"train\"\nlands = true\nstages = []\n\n\
                    [kinds.fix.agents]\nmodel = \"opus\"\n";
    repository.write(DECLARATION, declared);
    let revision = Revision::new(repository.commit("a kind that lands with no stages"));
    let KindsRead::Read {
        kinds, settings, ..
    } = Weir.read_kinds(&repository.name(), &revision)
    else {
        panic!("a kind that lands with no stages did not read");
    };
    assert_eq!(
        kinds
            .iter()
            .map(|kind| (kind.name.as_str(), kind.lands, kind.stages.len()))
            .collect::<Vec<_>>(),
        vec![("train", true, 0)],
        "train, a kind that lands with no stages"
    );
    assert_eq!(kinds[0].branch_prefix, "train");
    assert_eq!(
        settings.keys().collect::<Vec<_>>(),
        vec!["fix"],
        "fix's table, settings for a kind declared elsewhere"
    );
    assert_eq!(settings["fix"].every_stage.model.as_deref(), Some("opus"));
    repository.write(
        DECLARATION,
        &format!("{declared}\n[kinds.idle]\nlands = false\nstages = []\n"),
    );
    let revision = Revision::new(repository.commit("a kind that does not land with none"));
    match Weir.read_kinds(&repository.name(), &revision) {
        KindsRead::CouldNotRead { reason } => {
            assert!(reason.contains("kinds.idle.stages"), "{reason}");
            assert!(
                !reason.contains("kinds.train") && !reason.contains("kinds.fix"),
                "a table that reads named in {reason}"
            );
        }
        other => panic!("a kind that does not land with no stages read as {other:?}"),
    }
}

/// A declaration of `train`, landing with no stages and `more` in its
/// table, and `mend` at version 1, landing with one code stage delivering
/// commits, its prompt at `prompts/code.md` (23, 202).
fn trained(more: &str) -> String {
    format!(
        "[kinds.train]\nbranch = \"train\"\nlands = true\nstages = []\n{more}\n\n\
         [kinds.mend]\nversion = 1\nbranch = \"mend\"\nlands = true\n\n\
         [[kinds.mend.stages]]\nname = \"code\"\nprompt = \"prompts/code.md\"\n\
         deliverables = [\"commits\"]\nagents = [{{}}]\n"
    )
}

/// `repository` declaring `declaration`, mend's prompt beside it,
/// committed as `message`; the revision.
fn declaring_trained(repository: &Repository, declaration: &str, message: &str) -> Revision {
    repository.write(DECLARATION, declaration);
    repository.write(".config/prompts/code.md", "{{brief}}");
    Revision::new(repository.commit(message))
}

/// Each kind `repository` declares at `revision`, by name, with the kind
/// of its fixes and its bound; panics naming why where it could not be
/// read.
fn landing_fixes(
    repository: &Repository,
    revision: &Revision,
) -> BTreeMap<String, (Option<String>, Option<u32>)> {
    match Weir.read_kinds(&repository.name(), revision) {
        KindsRead::Read { kinds, .. } => kinds
            .into_iter()
            .map(|kind| (kind.name, (kind.fix, kind.fix_limit)))
            .collect(),
        other => panic!("the declaration read as {other:?}"),
    }
}

#[test]
fn a_kind_with_no_stages_names_its_landing_fix_and_its_bound() {
    // The kinds-profile spec's "A landing fix and its bound over a scratch
    // repository": weir's crate reads a kind's `landing_fix` and
    // `max_landing_fixes`, and the profile maps them onto the kind of its
    // fixes, spelled as weir spells the identity, and its bound; a kind
    // that gives neither names neither (23; weir 52; weir record 0005;
    // core.4).
    let repository = Repository::new("landing-fix");
    let declared = format!(
        "{}\n[kinds.yard]\nbranch = \"yard\"\nlands = true\nstages = []\n",
        trained("landing_fix = \"repo/mend@1\"\nmax_landing_fixes = 2")
    );
    let revision = declaring_trained(&repository, &declared, "train names its landing fix");
    let none = (None, None);
    assert_eq!(
        landing_fixes(&repository, &revision),
        BTreeMap::from([
            ("mend".to_string(), none.clone()),
            (
                "train".to_string(),
                (Some("repo/mend@1".to_string()), Some(2))
            ),
            ("yard".to_string(), none),
        ]),
        "each kind's landing fix and its bound"
    );
}

#[test]
fn a_landing_fix_weir_refuses_is_named_in_weirs_words() {
    // The kinds-profile spec's "A landing fix weir refuses": a bare name, a
    // name with no version, and either field on a kind with stages are
    // answered as could not be read in weir's words, naming the field (57;
    // weir record 0005).
    let repository = Repository::new("landing-fix-refused");
    for named in ["mend", "repo/mend"] {
        declaring_trained(
            &repository,
            &trained(&format!("landing_fix = \"{named}\"")),
            &format!("train names {named} its landing fix"),
        );
        let reason = could_not_read(&repository);
        assert!(
            reason.contains("kinds.train.landing_fix") && reason.contains("is no identity"),
            "{named} not refused as no identity: {reason}"
        );
    }
    declaring_trained(
        &repository,
        &format!(
            "{}\n[kinds.bump]\nbranch = \"bump\"\n\
             landing_fix = \"flume/fix@1\"\nmax_landing_fixes = 1\n\n\
             [[kinds.bump.stages]]\nname = \"code\"\nprompt = \"prompts/code.md\"\n\
             deliverables = [\"commits\"]\nagents = [{{}}]\n",
            trained("")
        ),
        "a one-stage kind names a landing fix",
    );
    let reason = could_not_read(&repository);
    for field in ["kinds.bump.landing_fix", "kinds.bump.max_landing_fixes"] {
        assert!(
            reason.contains(&format!("{field}: is on a kind with stages")),
            "{field} not refused as on a kind with stages: {reason}"
        );
    }
}

#[test]
fn a_landing_fix_bound_past_what_flume_counts_is_refused() {
    // The kinds-profile spec's "A bound past what flume counts": weir reads
    // the count, and the profile refuses the declaration naming the field
    // (4, 84).
    let repository = Repository::new("landing-fix-bound");
    declaring_trained(
        &repository,
        &trained("landing_fix = \"repo/mend@1\"\nmax_landing_fixes = 4294967296"),
        "train's bound past what flume counts",
    );
    let reason = could_not_read(&repository);
    assert!(
        reason.contains("kinds.train.max_landing_fixes: is more than flume counts"),
        "{reason}"
    );
}

#[test]
fn fix_and_max_fixes_are_sections_naming_no_landing_fix() {
    // The kinds-profile spec's "The fields' old spellings": weir returns
    // `fix` and `max_fixes` as sections, and flume reads no fix and no
    // bound from them (23; weir record 0005; core.4).
    let repository = Repository::new("old-spellings");
    let revision = declaring_trained(
        &repository,
        &trained("fix = \"repo/mend@1\"\nmax_fixes = 2"),
        "train names its fix in the old spelling",
    );
    assert_eq!(
        landing_fixes(&repository, &revision).get("train"),
        Some(&(None, None)),
        "train's landing fix and its bound"
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
        label: None,
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
        fix: None,
        fix_limit: None,
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

/// A scratch repository `label` binding the fakes' merge queue by the
/// name the world binds it by, and declaring `train` naming `landing_fix`
/// its landing fix with a bound of 2 beside `mend` at version 1; and the
/// launch of `train--spring` on `main` for the recipient `ops`, a session
/// that asks nothing, through the kinds profile, with the world it was
/// launched in and its answer (21, 203).
fn spring_through_the_profile(label: &str, landing_fix: &str) -> (Repository, World, Answer) {
    let repository = Repository::new(label);
    let revision = declaring_trained(
        &repository,
        &format!(
            "[profiles]\nmerge_queue = \"queue\"\n\n{}",
            trained(&format!(
                "landing_fix = \"{landing_fix}\"\nmax_landing_fixes = 2"
            ))
        ),
        "train names its landing fix",
    );
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    base_at(&world, &repository.name(), &revision.0);
    let spring = Launch {
        repository: repository.name(),
        recipients: vec!["ops".into()],
        ..launch("train", "spring", None)
    };
    let answer = engine(&world, Arc::new(Weir)).launch(&spring);
    (repository, world, answer)
}

#[test]
fn a_base_places_launch_holds_the_landing_fix_the_profile_reads() {
    // The kinds-profile spec's "The landing fix held through the profile":
    // the core resolves the landing fix the profile read as a launch's
    // whole identity, holds it with where it came from, and status gives
    // it with the kind's bound (21, 56, 203).
    let (repository, world, answer) = spring_through_the_profile("landing-fix-held", "repo/mend@1");
    launched(&answer).expect("the launch taken");
    let id = PlaceId::new(flume_fakes::place_location(
        &repository.name(),
        "train--spring",
    ));
    let held = world.held(&id).expect("the base place held");
    let fixes_as = held.place.fixes_as.expect("the kind of its fixes held");
    assert_eq!(fixes_as.identity.spelled, "repo/mend@1");
    assert_eq!(fixes_as.from.source, REPO_SOURCE, "where it came from");
    assert_eq!(
        (
            fixes_as.from.declaration_file.as_deref(),
            fixes_as.from.revision.clone()
        ),
        (Some(DECLARATION), Some(Revision::new(repository.head()))),
        "the repository's declaration at the launch's revision"
    );
    let places = match engine(&world, Arc::new(Weir)).status(Some(&id)) {
        Answer::Status { places, .. } => places,
        other => panic!("status answered {other:?}"),
    };
    assert_eq!(
        places
            .iter()
            .map(|place| (place.fix_kind.clone(), place.fix_limit))
            .collect::<Vec<_>>(),
        vec![(Some(fixes_as), Some(2))],
        "train--spring's fix kind and its bound in status"
    );
}

#[test]
fn a_landing_fix_at_a_version_not_declared_refuses_the_launch() {
    // The kinds-profile spec's "A landing fix at a version the repository
    // does not declare": the identity resolves at the version it names, so
    // `repo/mend@2` beside `mend` at version 1 refuses the launch before the
    // merge queue opens anything, and nothing is held (4, 154; core.8,
    // core.34).
    let (_repository, world, answer) =
        spring_through_the_profile("landing-fix-undeclared", "repo/mend@2");
    let reason = refused(&answer).expect("the launch refused");
    assert!(
        reason.contains("repo/mend@2") && reason.contains("declares mend at version 1"),
        "the refusal names neither the landing fix nor the version declared: {reason}"
    );
    assert_eq!(world.queue.log.count("open"), 0, "the merge queue's opens");
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
fn an_environment_in_any_declaration_refuses_the_launch_naming_the_field() {
    // C.2, 158: an agent's environment is its account's alone (144), so an
    // `env` in a repository's agents table for a kind it does not
    // declare, in a kind's own, or in a launch's declaration fails the
    // read, naming the field, and the launch is refused with nothing
    // made.
    let repository = Repository::new("declared-env");
    repository.write(
        DECLARATION,
        r#"[kinds.change.agents]
env = { A = "b" }

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ env = { B = "c" } }]
"#,
    );
    repository.write(".config/prompts/bump-code.md", "{{brief}}");
    repository.commit("environments in the declaration");
    let name = repository.name();
    let head = repository.head();
    match Weir.read_kinds(&name, &Revision::new(head.clone())) {
        KindsRead::CouldNotRead { reason } => {
            for field in [
                "kinds.change.agents.env",
                "kinds.bump.stages[0].agents[0].env",
            ] {
                assert!(
                    reason.contains(field),
                    "the read names not {field}: {reason}"
                );
            }
        }
        other => panic!("a declaration giving an environment read as {other:?}"),
    }
    let world = World::new();
    base_at(&world, &name, &head);
    let mut change = launch("change", "one", Some("x"));
    change.repository = name;
    let reason =
        refused(&engine(&world, Arc::new(Weir)).launch(&change)).expect("the launch refused");
    assert!(
        reason.contains("kinds.change.agents.env"),
        "the refusal names not the field: {reason}"
    );
    assert_eq!(world.place.log.count("make"), 0, "places made");
    assert!(world.store.places().is_empty(), "held state was left");
    let dir = repository.scratch.path("launch");
    std::fs::create_dir_all(dir.join("prompts")).unwrap();
    std::fs::write(dir.join("prompts/bump-code.md"), "{{brief}}").unwrap();
    let file = dir.join("kinds.toml");
    std::fs::write(
        &file,
        "[kinds.bump.agents.stages.code]\nenv = { C = \"d\" }\n\n[[kinds.bump.stages]]\nname = \"code\"\nprompt = \"prompts/bump-code.md\"\ndeliverables = [\"commits\"]\n",
    )
    .unwrap();
    match Weir.read_declaration(&file.to_string_lossy()) {
        KindsRead::CouldNotRead { reason } => assert!(
            reason.contains("kinds.bump.agents.stages.code.env"),
            "the launch's declaration's read names not the field: {reason}"
        ),
        other => panic!("a launch's declaration giving an environment read as {other:?}"),
    }
}

#[test]
fn a_mode_for_every_stage_and_a_table_for_one() {
    // 23, 162, C.2: a kind's agents table gives one of flume's four for
    // every stage, and a stage's a table of programs, kept as written.
    let repository = Repository::new("declared-mode");
    repository.write(
        DECLARATION,
        r#"[kinds.bump.agents]
mode = "edits"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ mode = { claude = "auto" } }]
"#,
    );
    repository.write(".config/prompts/bump-code.md", "{{brief}}");
    repository.commit("a mode for bump and a table for its code");
    let revision = Revision::new(repository.head());
    let KindsRead::Read { kinds, .. } = Weir.read_kinds(&repository.name(), &revision) else {
        panic!("the declaration did not read");
    };
    let bump = &kinds[0];
    assert_eq!(
        bump.agents.every_stage.mode,
        Some(ModeSetting::Flume(Mode::Edits))
    );
    assert_eq!(
        bump.stages[0].agents[0].mode,
        Some(ModeSetting::Programs(BTreeMap::from([(
            "claude".to_string(),
            "auto".to_string()
        )])))
    );
}

#[test]
fn an_effort_for_every_stage_and_one_for_a_stage() {
    // 23, 197, C.2: a kind's agents table gives an effort for every
    // stage, and a stage's agents another, each its program's own word
    // kept as written.
    let repository = Repository::new("declared-effort");
    repository.write(
        DECLARATION,
        r#"[kinds.bump.agents]
effort = "high"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ effort = "xhigh" }]
"#,
    );
    repository.write(".config/prompts/bump-code.md", "{{brief}}");
    repository.commit("an effort for bump and one for its code");
    let revision = Revision::new(repository.head());
    let KindsRead::Read { kinds, .. } = Weir.read_kinds(&repository.name(), &revision) else {
        panic!("the declaration did not read");
    };
    let bump = &kinds[0];
    assert_eq!(bump.agents.every_stage.effort.as_deref(), Some("high"));
    assert_eq!(bump.stages[0].agents[0].effort.as_deref(), Some("xhigh"));
}

#[test]
fn an_effort_for_every_kind_of_a_repository() {
    // 32, 197, C.2: the repository's `[agents]` gives an effort for every
    // kind, apart from its table for fix, which gives its own.
    let repository = Repository::new("every-kind-effort");
    repository.write(
        DECLARATION,
        r#"[agents]
effort = "medium"

[kinds.fix.agents]
effort = "high"
"#,
    );
    let revision = Revision::new(repository.commit("the repository's efforts"));
    let KindsRead::Read {
        settings, agents, ..
    } = Weir.read_kinds(&repository.name(), &revision)
    else {
        panic!("the declaration did not read");
    };
    assert_eq!(agents.effort.as_deref(), Some("medium"));
    assert_eq!(settings["fix"].every_stage.effort.as_deref(), Some("high"));
}

#[test]
fn an_effort_that_is_no_word_refuses_the_launch_naming_the_field() {
    // 4, 197, C.2: a number and a table of programs each fail the read,
    // naming the table's `effort`, in a repository's table for every kind,
    // in its table for a kind it does not declare, in a stage's of a
    // kind's own and in its agents; the launch is refused with nothing
    // made.
    let repository = Repository::new("declared-bad-effort");
    repository.write(
        DECLARATION,
        r#"[agents]
effort = { claude = "high" }

[kinds.change.agents]
effort = 3

[kinds.bump.agents.stages.code]
effort = { claude = "high" }

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ effort = 3 }]
"#,
    );
    repository.write(".config/prompts/bump-code.md", "{{brief}}");
    repository.commit("efforts that are no word");
    let reason = could_not_read(&repository);
    for field in [
        "agents.effort:",
        "kinds.change.agents.effort",
        "kinds.bump.agents.stages.code.effort",
        "kinds.bump.stages[0].agents[0].effort",
    ] {
        assert!(
            reason.contains(field),
            "the read names not {field}: {reason}"
        );
    }
    launch_refused_naming(&repository, "change", &["kinds.change.agents.effort"]);
}

#[test]
fn a_session_in_an_agents_table_gives_no_setting() {
    // 32, 158, C.2: a kind names nothing a host owns, a session among
    // it: `session` in an agents table, for every stage and for one, in
    // a repository's table for a kind it does not declare, in a kind's
    // own and in a stage's agents, reads as no setting, and the settings
    // beside it are read.
    let repository = Repository::new("declared-session");
    repository.write(
        DECLARATION,
        r#"[kinds.change.agents]
session = "x"
model = "opus"

[kinds.change.agents.stages.verify]
session = "y"
instructions = "Read every task."

[kinds.bump.agents]
session = "x"
model = "sonnet"

[kinds.bump.agents.stages.code]
session = "y"
model = "haiku"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ session = "z", instructions = "keep it short" }]
"#,
    );
    repository.write(".config/prompts/bump-code.md", "{{brief}}");
    repository.commit("a session for every stage and for one");
    let revision = Revision::new(repository.head());
    let KindsRead::Read {
        kinds, settings, ..
    } = Weir.read_kinds(&repository.name(), &revision)
    else {
        panic!("the declaration did not read");
    };
    let change = &settings["change"];
    assert_eq!(change.every_stage.session, None, "change's every stage");
    assert_eq!(change.for_stage("verify").session, None, "change's verify");
    assert_eq!(change.every_stage.model.as_deref(), Some("opus"));
    assert_eq!(
        change.for_stage("verify").instructions.as_deref(),
        Some("Read every task.")
    );
    let bump = &kinds[0];
    assert_eq!(bump.agents.every_stage.session, None, "bump's every stage");
    assert_eq!(bump.agents.for_stage("code").session, None, "bump's code");
    assert_eq!(bump.stages[0].agents[0].session, None, "code's own agent");
    assert_eq!(bump.agents.every_stage.model.as_deref(), Some("sonnet"));
    assert_eq!(
        bump.agents.for_stage("code").model.as_deref(),
        Some("haiku")
    );
    assert_eq!(
        bump.stages[0].agents[0].instructions.as_deref(),
        Some("keep it short")
    );
}

#[test]
fn a_mode_that_is_neither_refuses_the_launch_naming_the_field() {
    // 4, 169, C.2: a program's own value given bare, a list and a table
    // naming no program each fail the read, naming the table's `mode`, in
    // a repository's table for a kind it does not declare, in a stage's
    // of a kind's own and in its agents table; the launch is refused with
    // nothing made.
    let repository = Repository::new("declared-bad-mode");
    repository.write(
        DECLARATION,
        r#"[kinds.change.agents]
mode = "auto"

[kinds.bump.agents.stages.code]
mode = {}

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ mode = ["edits"] }]
"#,
    );
    repository.write(".config/prompts/bump-code.md", "{{brief}}");
    repository.commit("modes that are neither");
    let name = repository.name();
    let head = repository.head();
    match Weir.read_kinds(&name, &Revision::new(head.clone())) {
        KindsRead::CouldNotRead { reason } => {
            for field in [
                "kinds.change.agents.mode",
                "kinds.bump.agents.stages.code.mode",
                "kinds.bump.stages[0].agents[0].mode",
                "`auto`",
            ] {
                assert!(
                    reason.contains(field),
                    "the read names not {field}: {reason}"
                );
            }
        }
        other => panic!("a declaration giving modes that are neither read as {other:?}"),
    }
    let world = World::new();
    base_at(&world, &name, &head);
    let mut change = launch("change", "one", Some("x"));
    change.repository = name;
    let reason =
        refused(&engine(&world, Arc::new(Weir)).launch(&change)).expect("the launch refused");
    assert!(
        reason.contains("kinds.change.agents.mode"),
        "the refusal names not the field: {reason}"
    );
    assert_eq!(world.place.log.count("make"), 0, "places made");
    assert!(world.store.places().is_empty(), "held state was left");
}

/// Launches `kind` over `repository` at its head through the kinds
/// profile, and asserts it refused naming each of `fields` with nothing
/// made or held.
fn launch_refused_naming(repository: &Repository, kind: &str, fields: &[&str]) {
    let world = World::new();
    base_at(&world, &repository.name(), &repository.head());
    let mut given = launch(kind, "one", Some("x"));
    given.repository = repository.name();
    let reason =
        refused(&engine(&world, Arc::new(Weir)).launch(&given)).expect("the launch refused");
    for field in fields {
        assert!(
            reason.contains(field),
            "the refusal names not {field}: {reason}"
        );
    }
    assert_eq!(world.place.log.count("make"), 0, "places made");
    assert!(world.store.places().is_empty(), "held state was left");
}

/// The reason `repository`'s declaration at its head cannot be read.
fn could_not_read(repository: &Repository) -> String {
    match Weir.read_kinds(&repository.name(), &Revision::new(repository.head())) {
        KindsRead::CouldNotRead { reason } => reason,
        other => panic!("the declaration read as {other:?}"),
    }
}

#[test]
fn own_settings_for_every_stage_and_for_one() {
    // 23, 157, 195, C.2: a kind's agents table, a stage's agents and a
    // repository's table for a kind it does not declare each give
    // `programs`, kept as JSON values, a stage's file read beside the
    // declaration at the revision, not as the working tree holds it.
    let repository = Repository::new("own-settings");
    repository.write(
        DECLARATION,
        r#"[kinds.change.agents.stages.verify.programs.claude.settings]
statusLine = { type = "command", command = "verify-line" }

[kinds.bump.agents.programs.claude.settings]
tui = "fullscreen"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ programs = { claude = { file = "claude-bump.json" } } }]
"#,
    );
    repository.write(".config/prompts/bump-code.md", "{{brief}}");
    repository.write(
        ".config/claude-bump.json",
        r#"{ "statusLine": { "type": "command", "command": "bump-line" } }"#,
    );
    let revision = Revision::new(repository.commit("own settings for bump and its code"));
    repository.write(".config/claude-bump.json", r#"{ "tui": "default" }"#);
    let KindsRead::Read {
        kinds, settings, ..
    } = Weir.read_kinds(&repository.name(), &revision)
    else {
        panic!("the declaration did not read");
    };
    let bump = &kinds[0];
    assert_eq!(
        bump.agents.every_stage.programs["claude"].settings,
        BTreeMap::from([("tui".to_string(), serde_json::json!("fullscreen"))])
    );
    let code = &bump.stages[0].agents[0].programs["claude"];
    assert!(code.settings.is_empty(), "{code:?}");
    assert_eq!(
        code.file,
        Some(NamedFile {
            written: "claude-bump.json".into(),
            read: ".config/claude-bump.json".into(),
            keys: BTreeMap::from([(
                "statusLine".to_string(),
                serde_json::json!({ "type": "command", "command": "bump-line" })
            )]),
            left_out: vec![],
        }),
        "read at the revision"
    );
    assert_eq!(
        settings["change"].for_stage("verify").programs["claude"].settings["statusLine"],
        serde_json::json!({ "type": "command", "command": "verify-line" })
    );
}

#[test]
fn a_launchs_declaration_reads_its_settings_file_on_disk() {
    // 157, 195, C.2: a launch's declaration's file is read beside it on
    // disk, as its prompts are.
    let scratch = Scratch::new("launch-own");
    let dir = scratch.path("launch");
    std::fs::create_dir_all(dir.join("prompts")).unwrap();
    std::fs::write(dir.join("prompts/bump-code.md"), "{{brief}}").unwrap();
    std::fs::write(dir.join("claude.toml"), "tui = \"fullscreen\"\n").unwrap();
    let file = dir.join("kinds.toml");
    std::fs::write(
        &file,
        "[kinds.bump.agents.programs.claude]\nfile = \"claude.toml\"\n\n[[kinds.bump.stages]]\nname = \"code\"\nprompt = \"prompts/bump-code.md\"\ndeliverables = [\"commits\"]\n",
    )
    .unwrap();
    let KindsRead::Read { kinds, .. } = Weir.read_declaration(&file.to_string_lossy()) else {
        panic!("the launch's declaration did not read");
    };
    let named = kinds[0].agents.every_stage.programs["claude"]
        .file
        .clone()
        .expect("the file");
    assert_eq!(named.read, dir.join("claude.toml").display().to_string());
    assert_eq!(named.keys["tui"], serde_json::json!("fullscreen"));
}

#[test]
fn an_entry_flume_cannot_take_refuses_the_launch_naming_the_field() {
    // 4, 195, C.2: an entry that is no table, and a program's own key
    // beside its file rather than under `settings`, each fail the read
    // naming the field, and the launch is refused with nothing made.
    let repository = Repository::new("own-entry");
    repository.write(
        DECLARATION,
        r#"[kinds.change.agents]
programs = { claude = "x" }

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
agents = [{ programs = { claude = { tui = "fullscreen" } } }]
"#,
    );
    repository.write(".config/prompts/bump-code.md", "{{brief}}");
    repository.commit("entries flume cannot take");
    let reason = could_not_read(&repository);
    for field in [
        "kinds.change.agents.programs.claude:",
        "kinds.bump.stages[0].agents[0].programs.claude.tui:",
        "go under settings",
    ] {
        assert!(
            reason.contains(field),
            "the read names not {field}: {reason}"
        );
    }
    launch_refused_naming(
        &repository,
        "change",
        &["kinds.change.agents.programs.claude"],
    );
}

#[test]
fn a_settings_file_outside_the_repository_or_not_at_the_revision_refuses_the_launch() {
    // 4, 157; core.34: a file by `..` past the repository's root, and one
    // the revision does not hold though the working tree does, each fail
    // the read naming the field and the file.
    let repository = Repository::new("own-file");
    repository.write(
        DECLARATION,
        r#"[kinds.change.agents.programs.claude]
file = "../../claude.json"

[kinds.bump.agents.programs.claude]
file = "claude-bump.json"

[[kinds.bump.stages]]
name = "code"
prompt = "prompts/bump-code.md"
deliverables = ["commits"]
"#,
    );
    repository.write(".config/prompts/bump-code.md", "{{brief}}");
    repository.commit("files flume cannot read");
    repository.write(".config/claude-bump.json", "{}");
    let reason = could_not_read(&repository);
    for field in [
        "kinds.change.agents.programs.claude.file: ../../claude.json lies outside the repository",
        "kinds.bump.agents.programs.claude.file: .config/claude-bump.json is not in",
    ] {
        assert!(
            reason.contains(field),
            "the read names not {field}: {reason}"
        );
    }
    launch_refused_naming(
        &repository,
        "change",
        &[
            "kinds.change.agents.programs.claude.file",
            "outside the repository",
        ],
    );
}

#[test]
fn a_repositorys_agents_for_every_kind() {
    // S102, 21, 32, 195, C.2: the repository's `[agents]`, beside
    // `[profiles]` and no part of any kind, gives its settings for every
    // kind, apart from its settings for one kind; a launch's declaration
    // is not read for it.
    let repository = Repository::new("every-kind");
    let declaration = r#"[profiles]
place = "git-worktree"

[agents]
model = "opus"
mode = "edits"
instructions = "Mind the tagline."

[agents.programs.claude.settings]
attribution = { commit = "Assisted-by: Claude Code", pr = "" }

[kinds.fix.agents]
model = "sonnet"
"#;
    repository.write(DECLARATION, declaration);
    let revision = Revision::new(repository.commit("the repository's agents"));
    let KindsRead::Read {
        kinds,
        settings,
        agents,
    } = Weir.read_kinds(&repository.name(), &revision)
    else {
        panic!("the declaration did not read");
    };
    assert!(kinds.is_empty());
    assert_eq!(agents.model.as_deref(), Some("opus"));
    assert_eq!(agents.mode, Some(ModeSetting::Flume(Mode::Edits)));
    assert_eq!(agents.instructions.as_deref(), Some("Mind the tagline."));
    assert_eq!(
        agents.programs["claude"].settings["attribution"],
        serde_json::json!({ "commit": "Assisted-by: Claude Code", "pr": "" })
    );
    assert_eq!(settings["fix"].every_stage.model.as_deref(), Some("sonnet"));
    assert!(settings["fix"].every_stage.programs.is_empty(), "apart");
    let scratch = Scratch::new("launch-agents");
    let file = scratch.path("kinds.toml");
    std::fs::write(
        &file,
        format!("{declaration}\n[agents.stages.code]\nmodel = \"x\"\n"),
    )
    .unwrap();
    let KindsRead::Read { agents, .. } = Weir.read_declaration(&file.to_string_lossy()) else {
        panic!("the launch's declaration did not read");
    };
    assert_eq!(
        *agents,
        Settings::default(),
        "a launch's [agents] is not read"
    );
}

#[test]
fn a_repositorys_agents_table_naming_a_stage_or_an_environment_refuses_the_launch() {
    // 4, 32, C.2: `stages`, `streams`, `env` and `beside` in a
    // repository's `[agents]` each fail the read naming the field, and
    // the launch is refused with nothing made.
    for (given, field) in [
        ("stages = { code = { model = \"opus\" } }", "agents.stages"),
        (
            "streams = { work = { model = \"opus\" } }",
            "agents.streams",
        ),
        ("env = { X = \"1\" }", "agents.env"),
        ("beside = \"pane\"", "agents.beside"),
    ] {
        let repository = Repository::new("every-kind-refused");
        repository.write(DECLARATION, &format!("[agents]\n{given}\n"));
        repository.commit("a repository's agents flume refuses");
        let reason = could_not_read(&repository);
        assert!(reason.contains(&format!("{field}:")), "{given}: {reason}");
        launch_refused_naming(&repository, "fix", &[field]);
    }
}

/// The words a declaration's `keep_open` is refused in (design 1).
const KEEP_OPEN_REFUSED: &str =
    "how long an agent is kept open for an answer is the host's or the launch's to give";

#[test]
fn a_repositorys_keep_open_refuses_the_launch_naming_the_field() {
    // 4, 158, 198, C.2: the kinds-profile spec's "A repository's
    // keep_open": `keep_open` in a repository's table for a kind it does
    // not declare fails the read naming the field and that the host or the
    // launch gives it, and a launch of a fix is refused with nothing made;
    // so does one in its table for every kind, and in a kind's own and a
    // stage's of a kind it declares.
    for (given, kind, field) in [
        (
            "[kinds.fix.agents]\nkeep_open = \"2h\"\n",
            "fix",
            "kinds.fix.agents.keep_open",
        ),
        ("[agents]\nkeep_open = \"2h\"\n", "fix", "agents.keep_open"),
        (
            "[kinds.bump.agents.stages.code]\nkeep_open = \"2h\"\n\n\
             [[kinds.bump.stages]]\nname = \"code\"\nprompt = \"prompts/bump-code.md\"\n\
             deliverables = [\"commits\"]\n",
            "bump",
            "kinds.bump.agents.stages.code.keep_open",
        ),
    ] {
        let repository = Repository::new("declared-keep-open");
        repository.write(DECLARATION, given);
        repository.write(".config/prompts/bump-code.md", "{{brief}}");
        repository.commit("a keep_open in the declaration");
        let reason = could_not_read(&repository);
        assert!(
            reason.contains(&format!("{field}: {KEEP_OPEN_REFUSED}")),
            "{given}: {reason}"
        );
        launch_refused_naming(&repository, kind, &[field]);
    }
}

#[test]
fn a_packages_keep_open_refuses_the_launch_naming_the_stages_field() {
    // 4, 158, 198, C.2: the kinds-profile spec's "A kind's keep_open": a
    // package whose kind's stage gives `keep_open` in its `agents` entry
    // cannot be read, naming the stage's field, and a launch of that kind
    // from the host's source is refused with nothing made.
    let repository = Repository::new("packaged-keep-open");
    let kinds_dir = repository.scratch.path("kinds/acme");
    write_package(
        &kinds_dir,
        "lint@1",
        &reviewing("lint", 1, "").replace("agents = [{}]", "agents = [{ keep_open = \"5m\" }]"),
        &[("prompts/code.md", "{{brief}}")],
    );
    let field = "kinds.lint.stages[0].agents[0].keep_open";
    let reason = package_refused(&kinds_dir, "lint", 1);
    assert!(
        reason.contains("acme/lint@1") && reason.contains(&format!("{field}: {KEEP_OPEN_REFUSED}")),
        "{reason}"
    );
    let world = World::new();
    base_at(&world, &repository.name(), &repository.head());
    world.host.set_source("acme", &kinds_dir.to_string_lossy());
    let mut lint = launch("acme/lint@1", "one", Some("x"));
    lint.repository = repository.name();
    let reason =
        refused(&engine(&world, Arc::new(Weir)).launch(&lint)).expect("the launch refused");
    assert!(
        reason.contains(field),
        "the refusal names not {field}: {reason}"
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
    assert_eq!(
        kind.stages[0].agents[0].effort.as_deref(),
        Some("high"),
        "the stage's effort, read as written (197)"
    );
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

/// The binding is read at the remote's default branch, though the bare
/// repository's own `HEAD` names a branch never made and no
/// `origin/HEAD` was set; with no remote, that `HEAD` names no default
/// branch, and the binding is not read as empty (20, C.2).
#[test]
fn the_binding_is_read_at_the_remotes_default_branch_beside_a_bare_repository() {
    let origin = Repository::new("remote-default");
    origin.write(DECLARATION, "[profiles]\nmerge_queue = \"sluice\"\n");
    origin.commit("the merge queue bound");
    let remote = origin.bare_remote();
    let container = Container::of(&remote, Scratch::new("container"));
    assert_eq!(
        Weir.read_binding(&container.name()),
        BindingRead::Read {
            binding: Binding {
                merge_queue: Some("sluice".into()),
                ..Binding::default()
            }
        },
        "the binding at origin's main"
    );
    container.git(&["remote", "remove", "origin"]);
    match Weir.read_binding(&container.name()) {
        BindingRead::CouldNotRead { reason } => {
            assert!(reason.contains("master"), "{reason}")
        }
        other => panic!("a binding read where HEAD names no commit: {other:?}"),
    }
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
    // 22, 140, 157, 162, C.2: change@1 to change@4, chore@1 and chore@2,
    // fix@1 and fix@2, change@2's code stage repeating per task group and
    // change@1's, change@3's and change@4's not, all landing, each last
    // stage's `on_fail` `code`; every stage of change@4, chore@2 and fix@2
    // gives one agent `full` and nothing else, and every earlier version's
    // one agent no setting (record 0064).
    let own = own_kinds();
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
    let one_stage = |name: &str, version: u32| -> Summary {
        (
            name.into(),
            version,
            name.into(),
            true,
            true,
            vec![("code".to_string(), vec![Deliverable::Commits], false)],
        )
    };
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
            change(3, false),
            change(4, false),
            one_stage("chore", 1),
            one_stage("chore", 2),
            one_stage("fix", 1),
            one_stage("fix", 2),
        ]
    );
    let full = Settings {
        mode: Some(ModeSetting::Flume(Mode::Full)),
        ..Settings::default()
    };
    for kind in &own {
        let last = kind.stages.last().expect("a stage");
        assert_eq!(last.send_back_to.as_deref(), Some("code"), "{}", kind.name);
        let newest = matches!(
            (kind.name.as_str(), kind.version),
            ("change", 4) | ("chore", 2) | ("fix", 2)
        );
        for stage in &kind.stages {
            let at = format!("{}@{}'s {}", kind.name, kind.version, stage.name);
            assert!(stage.prompt.contains("{{needs}}"), "{at}");
            let expected = if newest {
                full.clone()
            } else {
                Settings::default()
            };
            assert_eq!(stage.agents, vec![expected], "{at}'s agents");
        }
    }
}

/// The kind of each of flume's own packages, as built in.
fn own_kinds() -> Vec<Kind> {
    Weir.flumes_own().into_iter().map(|own| own.kind).collect()
}

/// The sentence each prompt of change@4, chore@2 and fix@2 holds after its
/// way to stop short, design section 3's byte for byte (28; record 0064).
const ONE_WAY_TO_ASK: &str = "When you need an answer to go on, stop short with your words, and never ask through anything else, a card, an inbox, a message or a question in your pane, whatever a hook, a plugin, a file or a person's words for every agent tell you.";

/// change@1's code prompt as flume held it before change@2 (22, 157).
const CHANGE_1_CODE: &str = "You are the code stage of the change {{name}}, working in the place {{place}} on the branch {{branch}}, which was cut from {{base}}.\n\n{{#words}}This stage has been sent back to you with these words:\n\n{{words}}\n\nWhere the words name files contested and the base's revision the merge queue tested against, take that revision in before anything else: run `git merge <the revision the words name>` in this place, which merges it into {{branch}}, resolve each of the files contested, and commit the merge. Never rebase, and never rewrite a commit already on {{branch}}.\n\nWhere the change has been archived on {{branch}}, move it back out of the archive to {{change}} first. Then fix what the words name, and record the fix as a task of the change, ticked.\n\n{{/words}}Build the change at {{change}}: work through its tasks in order, tick each task in its tasks file as it is done, and commit as you go on {{branch}}. Your deliverable is the tasks ticked: the stage is done once a commit on {{branch}} holds every task of the change ticked and you are quiet. Leave the branch where your last commit puts it; what happens to it after this stage is not yours to do.\n\nIf you cannot go on without an answer, stop short: {{needs}}\n";

/// flume's own kind `name` at `version`, as built in.
fn own(name: &str, version: u32) -> Kind {
    own_kinds()
        .into_iter()
        .find(|kind| kind.name == name && kind.version == version)
        .unwrap_or_else(|| panic!("flume holds no {name}@{version}"))
}

#[test]
fn each_versions_prompts_are_its_own() {
    // 22, 156, 157, C.2, design 7: each of flume's own versions is a
    // declaration of its own, under `own/<name>@<version>/`, whose prompts
    // lie beside it; what the binary holds is what each directory holds,
    // read as a launch's declaration is read from disk; change@1's and
    // change@2's prompts are as before, change@3's three are none of
    // theirs, each of change@4's is change@3's with the one way to ask
    // after the line holding its way to stop short, and fix@2's and
    // chore@2's are equal, fix@1's with "or you find work this stage
    // cannot deliver past, " taken out of its last line and the one way
    // to ask after it (core.27; record 0064).
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
    assert_eq!(
        versions,
        ["change@1", "change@2", "change@3", "change@4", "chore@1", "chore@2", "fix@1", "fix@2"]
    );
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
    assert_eq!(read, own_kinds(), "the kinds built in, as on disk");
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
    let three = own("change", 3);
    for (stage, prompt) in three
        .stages
        .iter()
        .map(|stage| (&stage.name, &stage.prompt))
    {
        for earlier in one.stages.iter().chain(&two.stages) {
            assert_ne!(
                prompt, &earlier.prompt,
                "change@3's {stage} is an earlier version's {}",
                earlier.name
            );
        }
    }
    assert_eq!(
        own("fix", 1).stages[0].prompt,
        own("chore", 1).stages[0].prompt
    );
    let four = own("change", 4);
    assert_eq!(four.stages.len(), three.stages.len(), "change@4's stages");
    for (earlier, stage) in three.stages.iter().zip(&four.stages) {
        let needs = earlier.prompt.find("{{needs}}\n").unwrap_or_else(|| {
            panic!(
                "change@3's {} has no line ending in its needs",
                earlier.name
            )
        }) + "{{needs}}\n".len();
        let expected = format!(
            "{}\n{ONE_WAY_TO_ASK}\n{}",
            &earlier.prompt[..needs],
            &earlier.prompt[needs..]
        );
        assert_eq!(
            stage.prompt, expected,
            "change@4's {} is change@3's with the one way to ask",
            stage.name
        );
    }
    let fix_1 = own("fix", 1).stages[0].prompt.clone();
    let found = "or you find work this stage cannot deliver past, ";
    assert_eq!(fix_1.matches(found).count(), 1, "fix@1's last line");
    let expected = format!("{}\n{ONE_WAY_TO_ASK}\n", fix_1.replace(found, ""));
    assert_eq!(
        own("fix", 2).stages[0].prompt,
        expected,
        "fix@2's code is fix@1's"
    );
    assert_eq!(
        own("chore", 2).stages[0].prompt,
        own("fix", 2).stages[0].prompt,
        "chore@2's code is fix@2's"
    );
}

/// flume's own kinds with `change` at `version` alone and `fix` and
/// `chore` at their first, as a flume that held that version of change and
/// no other, and none of the versions after record 0064, whose `full` the
/// world's fake agent maps to nothing until a scenario scripts it (162,
/// core.39).
fn own_at(version: u32) -> Vec<Kind> {
    own_kinds()
        .into_iter()
        .filter(|kind| {
            if kind.name == "change" {
                kind.version == version
            } else {
                kind.version == 1
            }
        })
        .collect()
}

/// A world whose own kinds are the profile's built in, `change` at
/// `version`.
fn world_with_own_kinds(version: u32) -> World {
    let world = World::new();
    world.kinds.set_own(own_at(version));
    world
}

/// A world whose own kinds are every version the profile builds in, as
/// its packages, its fake agent mapping `full` to `all` and `edits` to
/// `edit-files`, so the newest three's agents start (162, 169; record
/// 0064).
fn world_with_every_own_version() -> World {
    let world = World::new();
    world.kinds.set_own_packages(Weir.flumes_own());
    world.agent.map_mode(Mode::Full, "all");
    world.agent.map_mode(Mode::Edits, "edit-files");
    world
}

/// The prompts of a fix's code, a chore's code and a change's construct,
/// code and verify, each rendered for a place launched by its bare name in
/// `world`, named by the version its place runs and its stage.
fn rendered_own(world: &World) -> Vec<(String, String)> {
    fix_begun(world, "one").expect("a fix begun");
    launched(
        &world
            .engine
            .launch(&launch("chore", "two", Some("tidy the lock"))),
    )
    .expect("a chore launched");
    world.settle(&World::id("chore", "two"));
    change_at_verify(world, "plan").expect("a change at verify");
    let mut prompts = vec![];
    for (kind, name, stages) in [
        ("fix", "one", &["code"][..]),
        ("chore", "two", &["code"][..]),
        ("change", "plan", &["construct", "code", "verify"][..]),
    ] {
        let version = world
            .held(&World::id(kind, name))
            .expect("the place")
            .place
            .kind
            .version;
        for stage in stages {
            prompts.push((
                format!("{kind}@{version}'s {stage}"),
                opened(world, kind, name, stage, 1),
            ));
        }
    }
    prompts
}

/// The value the command of the start under `identity` passed for its
/// mode, none where it passed none.
fn mode_passed(world: &World, identity: &Identity) -> Option<String> {
    let (command, _) = world
        .terminal
        .opened(identity)
        .unwrap_or_else(|| panic!("{identity} was not opened"));
    command
        .args
        .windows(2)
        .find(|pair| pair[0] == "--mode")
        .map(|pair| pair[1].clone())
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
    // 28, 71, 112: every prompt of fix@1, chore@1, change@1, change@2 and
    // change@3 rendered for a place.
    let mut prompts = vec![];
    for version in [1, 2, 3] {
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

#[test]
fn the_newest_own_prompts_say_their_one_way_to_ask() {
    // 28, 156; record 0064: change@4's construct, code and verify and
    // fix@2's and chore@2's code rendered for a place each hold the way to
    // stop short and, after it, the one way to ask, and ask nothing the
    // earlier versions' may not; no prompt of change@3, fix@1 or chore@1
    // holds that sentence.
    let newest = rendered_own(&world_with_every_own_version());
    let named: Vec<&str> = newest.iter().map(|(named, _)| named.as_str()).collect();
    assert_eq!(
        named,
        [
            "fix@2's code",
            "chore@2's code",
            "change@4's construct",
            "change@4's code",
            "change@4's verify"
        ],
        "the versions a bare name runs"
    );
    for (named, prompt) in &newest {
        assert!(!prompt.contains("{{"), "{named}: a part left unfilled");
        asks_nothing_it_may_not(prompt);
        assert!(
            !prompt.to_lowercase().contains("merge"),
            "{named}: a merge asked with no conflict's words: {prompt}"
        );
        let needs = prompt
            .find("flume needs")
            .unwrap_or_else(|| panic!("{named} has no way to stop short: {prompt}"));
        let ask = prompt
            .find(ONE_WAY_TO_ASK)
            .unwrap_or_else(|| panic!("{named} says not its one way to ask: {prompt}"));
        assert!(needs < ask, "{named}: the one way to ask before its needs");
    }
    let earlier = rendered_own(&world_with_own_kinds(3));
    let named: Vec<&str> = earlier.iter().map(|(named, _)| named.as_str()).collect();
    assert_eq!(
        named,
        [
            "fix@1's code",
            "chore@1's code",
            "change@3's construct",
            "change@3's code",
            "change@3's verify"
        ],
        "the earlier versions"
    );
    for (named, prompt) in &earlier {
        assert!(
            !prompt.contains(ONE_WAY_TO_ASK),
            "{named} holds the one way to ask: {prompt}"
        );
    }
}

#[test]
fn flumes_newest_own_kinds_give_full_from_the_kind() {
    // 22, 32, 154, 162; record 0064: with the profile's own kinds and the
    // fake agent mapping `full`, a bare fix runs flume/fix@2, its agent's
    // command carrying full's value, its start's settings giving `full`
    // and its record that value, and status `full` from `kind`; the host's
    // `[agents] mode = "edits"` comes first, from `host`; and fix@1 named
    // starts with no mode.
    let world = world_with_every_own_version();
    let id = fix_begun(&world, "one").expect("a fix begun");
    let code = World::identity("fix", "one", "code", 1);
    let status = world.status(&id).expect("status");
    assert_eq!(
        status.identity.as_deref(),
        Some("flume/fix@2"),
        "a bare fix"
    );
    assert_eq!(
        mode_passed(&world, &code).as_deref(),
        Some("all"),
        "the value passed"
    );
    let held = world.held(&id).expect("the place");
    let start = held.stages[0].start.as_ref().expect("code's start");
    assert_eq!(
        (start.settings.mode.clone(), start.start_record.mode.clone()),
        (
            Some(ModeSetting::Flume(Mode::Full)),
            Some("all".to_string())
        ),
        "the start's mode and its record's"
    );
    assert_eq!(
        status.agent.expect("an agent").settings.from.get("mode"),
        Some(&Given {
            layer: Layer::Kind,
            value: "full".into(),
            file: None,
        }),
        "where the mode came from"
    );

    let world = world_with_every_own_version();
    world.host.set_every_agent(Settings {
        mode: Some(ModeSetting::Flume(Mode::Edits)),
        ..Settings::default()
    });
    let id = fix_begun(&world, "one").expect("a fix begun");
    assert_eq!(
        mode_passed(&world, &code).as_deref(),
        Some("edit-files"),
        "the host's value passed"
    );
    assert_eq!(
        world
            .status(&id)
            .expect("status")
            .agent
            .expect("an agent")
            .settings
            .from
            .get("mode"),
        Some(&Given {
            layer: Layer::Host,
            value: "edits".into(),
            file: None,
        }),
        "the host's mode over the kind's"
    );

    let world = world_with_every_own_version();
    launched(&world.engine.launch(&launch("fix@1", "one", Some("x")))).expect("fix@1 launched");
    let id = World::id("fix", "one");
    world.settle(&id);
    let status = world.status(&id).expect("status");
    assert_eq!(status.identity.as_deref(), Some("flume/fix@1"), "fix@1");
    assert_eq!(mode_passed(&world, &code), None, "fix@1's value passed");
    let held = world.held(&id).expect("the place");
    let start = held.stages[0].start.as_ref().expect("code's start");
    assert_eq!(start.start_record.mode, None, "fix@1's record's mode");
    assert!(
        !status
            .agent
            .expect("an agent")
            .settings
            .from
            .contains_key("mode"),
        "status names a layer of fix@1's mode"
    );
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
    // 112: fix@1's, chore@1's, change@2's, change@3's, fix@2's, chore@2's
    // and change@4's code prompts sent back with a conflict's words.
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
        "change@2",
        sent_back_with_a_conflict(&world, "change", "plan"),
    ));

    let world = world_with_own_kinds(3);
    change_at_verify(&world, "plan").expect("a change@3 at verify");
    archive(&world, "plan");
    prompts.push((
        "change@3",
        sent_back_with_a_conflict(&world, "change", "plan"),
    ));

    let world = world_with_every_own_version();
    let fix = fix_begun(&world, "one").expect("a fix@2 begun");
    deliver_fix(&world, &fix);
    prompts.push(("fix@2", sent_back_with_a_conflict(&world, "fix", "one")));
    launched(
        &world
            .engine
            .launch(&launch("chore", "two", Some("tidy the lock"))),
    )
    .expect("a chore@2 launched");
    world.settle(&World::id("chore", "two"));
    world.commit("chore/two", &["chore.rs"]);
    prompts.push(("chore@2", sent_back_with_a_conflict(&world, "chore", "two")));
    change_at_verify(&world, "plan").expect("a change@4 at verify");
    archive(&world, "plan");
    prompts.push((
        "change@4",
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

/// A change place carried to its verify by a flume holding change@3, and
/// the prompts its construct, code and verify were opened with (22, C.2).
fn change_3_rendered() -> (World, PlaceId, [String; 3]) {
    let world = world_with_own_kinds(3);
    let id = change_at_verify(&world, "plan").expect("a change@3 at verify");
    assert_eq!(
        world.held(&id).expect("the place").place.kind.version,
        3,
        "the place's kind"
    );
    let prompts =
        ["construct", "code", "verify"].map(|stage| opened(&world, "change", "plan", stage, 1));
    for prompt in &prompts {
        assert!(!prompt.contains("{{"), "a part left unfilled: {prompt}");
    }
    (world, id, prompts)
}

/// Asserts `prompt`, `stage`'s, names each of `named`.
fn names_each(stage: &str, prompt: &str, named: &[&str]) {
    for named in named {
        assert!(
            prompt.contains(named),
            "change@3's {stage} prompt names not {named:?}: {prompt}"
        );
    }
}

#[test]
fn change_3s_construct_prompt_gives_each_task_its_own_check() {
    // 22, C.2, S119: change@3's construct prompt rendered for a place asks
    // each task for the narrowest check that shows it done, and rules out
    // a task running the repository's whole checks or a task group's.
    let (_, _, [construct, _, _]) = change_3_rendered();
    names_each(
        "construct",
        &construct,
        &[
            "Give each task the narrowest check that shows that task done",
            "one test by name, one package's tests, or a lint of the files the task touches",
            "No task runs the repository's whole checks",
            "no task checks a task group",
            "the task groups are headings for reading",
            "the verify stage runs the repository's whole checks once",
        ],
    );
}

#[test]
fn change_3s_code_prompt_runs_each_tasks_own_check_alone() {
    // 22, C.2, S119: change@3's code prompt rendered for a place has its
    // one agent build the whole change, run each task's own check before
    // its tick, and rules out the repository's whole checks and a task
    // group's; it names no group and no notes, as change@2's does.
    let (_, _, [_, code, _]) = change_3_rendered();
    names_each(
        "code",
        &code,
        &[
            "You build the whole change, every task of it",
            "Work through its tasks in order, the task groups only headings",
            "run the check it names as it names it",
            "tick the task in the change's tasks file once its check passes",
            "Run no check but a task's own",
            "not the repository's whole checks",
            "not a task group's",
            "tasks ticked",
        ],
    );
    let check = code.find("run the check it names").expect("the check");
    let tick = code.find("tick the task").expect("the tick");
    assert!(check < tick, "the tick comes before the check: {code}");
    for named in ["The notes: ", "repetition", "Group 1"] {
        assert!(
            !code.contains(named),
            "{named:?} in change@3's code: {code}"
        );
    }
}

#[test]
fn change_3s_code_prompt_puts_a_retrys_words_last() {
    // 131, core.33: change@3's code prompt rendered for one place without
    // words, and again with its verify's words: the two are equal up to
    // the first character of the words' section, and the second holds the
    // words and asks the fix be recorded as a task naming its own check.
    let (world, id, [_, first, _]) = change_3_rendered();
    let verify = world.held(&id).expect("the place").stages[2]
        .start
        .clone()
        .expect("verify's start");
    let findings = "Task 2.1 is ticked and its test fails.";
    let stopped = world.engine.needs(
        &verify.identity.to_string(),
        &verify.secret.0,
        findings,
        StopKind::FoundWork,
    );
    assert!(stopped.refusal().is_none(), "the stop-short: {stopped:?}");
    world.settle(&id);
    let second = opened(&world, "change", "plan", "code", 2);
    let words = second
        .find("This stage was started again with these words:")
        .expect("the words' section");
    assert_eq!(first, &second[..words], "the prompts up to the words");
    names_each(
        "code",
        &second[words..],
        &[
            findings,
            "record the fix as a task of the change naming its own check, ticked once that \
             check passes",
        ],
    );
}

#[test]
fn change_3s_verify_prompt_runs_the_whole_checks_once_first() {
    // 22, 131, C.2, S119: change@3's verify prompt rendered for a place
    // runs the repository's whole checks once, as the repository's
    // instructions name them, before it checks the specs and the tasks,
    // names their outcome in `verify.md`, and archives the change only
    // where both hold; it names no tool, serving any host's repository.
    let (_, _, [_, _, verify]) = change_3_rendered();
    names_each(
        "verify",
        &verify,
        &[
            "First run the repository's whole checks once",
            "as the repository's instructions name them",
            "run them no second time",
            "every requirement and scenario of its specs met, and every task ticked done",
            "verify.md",
            "the whole checks' outcome among it",
            "Where the whole checks pass and the work holds, archive the change",
            "Where the whole checks fail or the work falls short, archive nothing",
            "stop short having found work",
        ],
    );
    let checks = verify.find("whole checks once").expect("the whole checks");
    let specs = verify.find("its specs met").expect("the specs");
    assert!(checks < specs, "the specs checked before the whole checks");
    let lower = verify.to_lowercase();
    for tool in ["sluice", "moon", "cargo", "devenv", "openspec"] {
        assert!(!lower.contains(tool), "change@3's verify names {tool}");
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
    // The asking agent stopped, so the retry starts a fresh agent rather
    // than answering its session (51, 198).
    let ended = world.engine.stop(&id, false);
    assert!(ended.refusal().is_none(), "the stop: {ended:?}");
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
    world.kinds.set_own_packages(Weir.flumes_own());
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

#[test]
fn a_place_launched_at_change_2_keeps_it() {
    // 21, 22: a place launched by a flume holding change@1 and change@2
    // alone is settled by one holding every version; it keeps change@2's
    // stages, its code stage repeating per task group, and status names
    // change@2.
    let world = World::new();
    world.kinds.set_own(
        own_kinds()
            .into_iter()
            .filter(|kind| kind.name != "change" || kind.version <= 2)
            .collect(),
    );
    let id = change_code_begun(&world, "old");
    world.kinds.set_own_packages(Weir.flumes_own());
    world.tick_groups("change/old", "old", GROUPS, 2);
    world.settle(&id);
    let held = world.held(&id).expect("the place");
    assert_eq!(held.place.kind.version, 2, "the place's kind");
    assert!(held.place.kind.stages[1].repeats, "code repeats not");
    assert_eq!(held.stages[1].starts, 2, "code's starts");
    assert_eq!(world.stage_state(&id, "code"), Some(StageState::Begun));
    let status = world.status(&id).expect("status");
    assert_eq!(
        (
            status.kind.as_str(),
            status.version,
            status.identity.as_deref()
        ),
        ("change", 2, Some("flume/change@2"))
    );
    assert_eq!(
        status.stages[1]
            .group
            .as_ref()
            .map(|group| (group.place, group.of, group.number.clone())),
        Some((2, 3, Some("2".to_string()))),
        "the group status names"
    );
}

/// The package `<name>@<version>` of the source `acme` in `kinds_dir`,
/// read; panics naming why where it could not be.
fn package_at(kinds_dir: &Path, name: &str, version: u32) -> Packaged {
    match Weir.read_package("acme", &kinds_dir.to_string_lossy(), name, version) {
        PackageRead::Read { package } => *package,
        PackageRead::CouldNotRead { reason } => panic!("{name}@{version} not read: {reason}"),
    }
}

/// Why the package `<name>@<version>` of the source `acme` in `kinds_dir`
/// could not be read; panics where it was.
fn package_refused(kinds_dir: &Path, name: &str, version: u32) -> String {
    match Weir.read_package("acme", &kinds_dir.to_string_lossy(), name, version) {
        PackageRead::CouldNotRead { reason } => reason,
        PackageRead::Read { package } => panic!("{name}@{version} read as {package:?}"),
    }
}

#[test]
fn a_packages_hash_wherever_it_lies() {
    // 156; weir 90: one package copied to two directories, one reached
    // through a link, reads the same kind at the same hash, and a byte of
    // one prompt moves its hash.
    let scratch = Scratch::new("hash-wherever");
    let here = scratch.path("here/kinds");
    let there = scratch.path("there/kinds");
    for kinds_dir in [&here, &there] {
        write_package(
            kinds_dir,
            "review@1",
            &reviewing("review", 1, ""),
            &[("prompts/code.md", "Review {{brief}}.\n")],
        );
    }
    let linked = scratch.path("linked");
    std::os::unix::fs::symlink(scratch.path("there"), &linked).unwrap();
    let first = package_at(&here, "review", 1);
    let second = package_at(&linked.join("kinds"), "review", 1);
    assert_eq!(
        (&first.identity, &first.kind, &first.hash),
        (&second.identity, &second.kind, &second.hash),
        "one package in two places"
    );
    assert_eq!(
        (first.dir.as_deref(), second.dir.as_deref()),
        (
            here.join("review@1").to_str(),
            linked.join("kinds/review@1").to_str()
        ),
        "each package's directory as read"
    );
    assert_eq!(first.identity.spelled, "acme/review@1");
    assert_eq!(first.kind.stages[0].prompt, "Review {{brief}}.\n");
    assert_eq!(first.hash.len(), "sha256:".len() + 64, "{}", first.hash);
    std::fs::write(here.join("review@1/prompts/code.md"), "Review {{brief}}!\n").unwrap();
    let edited = package_at(&here, "review", 1);
    assert_ne!(edited.hash, first.hash, "a prompt's byte changed its hash");
    assert_eq!(edited.kind.stages[0].prompt, "Review {{brief}}!\n");
    assert_eq!(package_at(&linked.join("kinds"), "review", 1), second);
}

#[test]
fn a_packages_kind_names_its_landing_fix() {
    // The kinds-profile spec's "A package's landing fix": the package's
    // kind is mapped by the same reader as a repository's, naming the kind
    // of its fixes and its bound (157; weir 89; weir record 0005).
    let scratch = Scratch::new("package-landing-fix");
    let kinds_dir = scratch.path("kinds");
    write_package(
        &kinds_dir,
        "train@1",
        "[kinds.train]\nversion = 1\nbranch = \"train\"\nlands = true\nstages = []\n\
         landing_fix = \"flume/fix@1\"\nmax_landing_fixes = 1\n",
        &[],
    );
    let package = package_at(&kinds_dir, "train", 1);
    assert_eq!(package.identity.spelled, "acme/train@1");
    assert_eq!(
        (package.kind.fix.as_deref(), package.kind.fix_limit),
        (Some("flume/fix@1"), Some(1)),
        "the package's landing fix and its bound"
    );
}

#[test]
fn a_sources_listing() {
    // 159, 192: the directories holding a `kind.toml` under a name weir
    // reads as `<name>@<version>`, and no other entry.
    let scratch = Scratch::new("listing");
    let kinds_dir = scratch.path("kinds/acme");
    for version in [2, 1] {
        write_package(
            &kinds_dir,
            &format!("review@{version}"),
            &reviewing("review", version, ""),
            &[("prompts/code.md", "{{brief}}")],
        );
    }
    std::fs::create_dir_all(kinds_dir.join("notes")).unwrap();
    std::fs::create_dir_all(kinds_dir.join("lint@x")).unwrap();
    std::fs::write(kinds_dir.join("lint@x/README"), "no kind").unwrap();
    std::fs::create_dir_all(kinds_dir.join("lint@1/prompts")).unwrap();
    std::fs::write(kinds_dir.join("lint@1/prompts/code.md"), "{{brief}}").unwrap();
    std::fs::write(kinds_dir.join("fmt@1"), "a file, which is no package").unwrap();
    let read = Weir.read_source("acme", &kinds_dir.to_string_lossy());
    assert_eq!(
        read,
        SourceRead::Read {
            kinds: vec![
                kinds::identity_of("acme", "review", 1),
                kinds::identity_of("acme", "review", 2),
            ],
        }
    );
    let SourceRead::CouldNotRead { reason } =
        Weir.read_source("acme", &scratch.path("kinds/beta").to_string_lossy())
    else {
        panic!("a directory that does not exist read");
    };
    assert!(
        reason.contains("kinds/beta") && reason.contains("does not exist"),
        "{reason}"
    );
}

#[test]
fn the_repositorys_lock_at_a_revision() {
    // 20, 155: `.config/flume.lock` at each revision as git gives it, none
    // at a revision that holds none.
    let change = Weir
        .flumes_own()
        .into_iter()
        .find(|own| own.identity.spelled == "flume/change@1")
        .expect("flume's own change@1");
    let repository = Repository::new("lock-revisions");
    let before = Revision::new(repository.head());
    let recorded = vec![entry("flume", "change", 1, &change.hash)];
    repository.write(LOCK, &spelled_lock(&recorded));
    let locked = Revision::new(repository.commit("change@1 locked"));
    repository.write(LOCK, "");
    let emptied = Revision::new(repository.commit("the lock emptied"));
    std::fs::write(repository.dir.join(LOCK), "not a lock [").unwrap();
    let name = repository.name();
    assert_eq!(
        Weir.read_lock_at(&name, &locked),
        LockRead::Read { entries: recorded }
    );
    assert_eq!(
        Weir.read_lock_at(&name, &emptied),
        LockRead::Read { entries: vec![] }
    );
    assert_eq!(Weir.read_lock_at(&name, &before), LockRead::None);
}

#[test]
fn a_lock_written_and_read_back() {
    // 155; weir 91: a lock of acme/review@1 and acme/review@2 written and
    // read as written, a lock beside nothing left, and a write that fails
    // leaving the file as it was.
    let scratch = Scratch::new("lock-written");
    let path = scratch.path("flume/flume.lock");
    let file = path.to_string_lossy().into_owned();
    let written = vec![
        entry("acme", "review", 1, &kinds::hash_of('1')),
        entry("acme", "review", 2, &kinds::hash_of('2')),
    ];
    Weir.write_lock(&file, &written).expect("the lock written");
    assert_eq!(
        Weir.read_lock_file(&file),
        LockRead::Read {
            entries: written.clone()
        }
    );
    let beside: Vec<_> = std::fs::read_dir(scratch.path("flume"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(
        beside,
        ["flume.lock"],
        "what the write left beside the lock"
    );
    let text = std::fs::read_to_string(&path).unwrap();
    let dir = scratch.path("flume");
    std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o555)).unwrap();
    let failed = Weir.write_lock(&file, &written[..1]);
    std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o755)).unwrap();
    let reason = failed.expect_err("a write into a directory that takes none");
    assert!(reason.contains(&file), "{reason}");
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        text,
        "the lock moved"
    );
    let twice = vec![
        written[0].clone(),
        entry("acme", "review", 1, &kinds::hash_of('3')),
    ];
    let reason = Weir
        .write_lock(&file, &twice)
        .expect_err("one identity at two hashes");
    assert!(reason.contains("acme/review@1"), "{reason}");
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        text,
        "the lock moved"
    );
}

#[test]
fn flumes_own_as_packages() {
    // 156, 157: each version of flume's own a package at its identity,
    // with its hash.
    let own: Vec<(String, usize)> = Weir
        .flumes_own()
        .into_iter()
        .map(|own| {
            assert!(own.hash.starts_with("sha256:"), "{}", own.hash);
            (own.identity.spelled, own.hash.len())
        })
        .collect();
    let length = "sha256:".len() + 64;
    assert_eq!(
        own,
        [
            ("flume/change@1".to_string(), length),
            ("flume/change@2".to_string(), length),
            ("flume/change@3".to_string(), length),
            ("flume/chore@1".to_string(), length),
            ("flume/fix@1".to_string(), length),
        ]
    );
}

#[test]
fn a_package_at_another_identity_is_refused_naming_both_versions() {
    // 157: acme's `review@2/kind.toml` declaring review at version 1.
    let scratch = Scratch::new("another-identity");
    let kinds_dir = scratch.path("kinds/acme");
    write_package(
        &kinds_dir,
        "review@2",
        &reviewing("review", 1, ""),
        &[("prompts/code.md", "{{brief}}")],
    );
    let reason = package_refused(&kinds_dir, "review", 2);
    for named in ["acme/review@1", "acme/review@2", "kinds.review.version"] {
        assert!(reason.contains(named), "{named} not in: {reason}");
    }
    let reason = package_refused(&kinds_dir, "review", 3);
    assert!(
        reason.contains("review@3") && reason.contains("does not exist"),
        "{reason}"
    );
}

#[test]
fn a_part_outside_the_package_an_environment_and_a_program_by_a_location() {
    // S63, 157, 158: standing instructions at `../shared/standing.md`, an
    // agents table's `env`, and a program named by its path, each refusing
    // the package naming the field.
    let scratch = Scratch::new("outside");
    let kinds_dir = scratch.path("kinds/acme");
    std::fs::create_dir_all(kinds_dir.join("shared")).unwrap();
    std::fs::write(kinds_dir.join("shared/standing.md"), "Stand.").unwrap();
    let code = [("prompts/code.md", "{{brief}}")];
    write_package(
        &kinds_dir,
        "review@1",
        &reviewing("review", 1, "standing = \"../shared/standing.md\"\n"),
        &code,
    );
    write_package(
        &kinds_dir,
        "lint@1",
        &format!(
            "{}\n[kinds.lint.agents]\nenv = {{ A = \"b\" }}\n",
            reviewing("lint", 1, "")
        ),
        &code,
    );
    write_package(
        &kinds_dir,
        "fmt@1",
        &reviewing("fmt", 1, "").replace(
            "agents = [{}]",
            "agents = [{ program = \"/opt/agents/bin/agent\" }]",
        ),
        &code,
    );
    let reason = package_refused(&kinds_dir, "review", 1);
    assert!(
        reason.contains("acme/review@1")
            && reason.contains("kinds.review.stages[0].standing")
            && reason.contains("../shared/standing.md"),
        "{reason}"
    );
    let reason = package_refused(&kinds_dir, "lint", 1);
    assert!(
        reason.contains("acme/lint@1") && reason.contains("kinds.lint.agents.env"),
        "{reason}"
    );
    let reason = package_refused(&kinds_dir, "fmt", 1);
    assert!(
        reason.contains("kinds.fmt.stages[0].agents[0].program")
            && reason.contains("/opt/agents/bin/agent"),
        "{reason}"
    );
}

#[test]
fn a_packages_settings_file_is_read_within_it_and_never_outside() {
    // 157, 195; core.34, core.50: a host's source's package names its
    // program's settings file by a path within it, read there; one by
    // `..` or by an absolute path refuses the package naming the field and
    // the path, and one through a link is refused by weir's reader, which
    // takes no link into a package.
    let scratch = Scratch::new("package-own");
    let kinds_dir = scratch.path("kinds/acme");
    std::fs::create_dir_all(kinds_dir.join("shared")).unwrap();
    let outside = kinds_dir.join("shared/claude.json");
    std::fs::write(&outside, "{ \"tui\": \"outside\" }").unwrap();
    let code = [("prompts/code.md", "{{brief}}")];
    let naming = |name: &str, file: &str| {
        format!(
            "{}\n[kinds.{name}.agents.programs.claude]\nfile = \"{file}\"\n",
            reviewing(name, 1, "")
        )
    };
    write_package(
        &kinds_dir,
        "review@1",
        &naming("review", "claude.json"),
        &[code[0], ("claude.json", "{ \"tui\": \"fullscreen\" }")],
    );
    write_package(
        &kinds_dir,
        "up@1",
        &naming("up", "../shared/claude.json"),
        &code,
    );
    let absolute = outside.display().to_string();
    write_package(&kinds_dir, "abs@1", &naming("abs", &absolute), &code);
    write_package(
        &kinds_dir,
        "linked@1",
        &naming("linked", "claude.json"),
        &code,
    );
    std::os::unix::fs::symlink(&outside, kinds_dir.join("linked@1/claude.json")).unwrap();
    let package = package_at(&kinds_dir, "review", 1);
    let named = package.kind.agents.every_stage.programs["claude"]
        .file
        .clone()
        .expect("the file");
    assert_eq!(named.written, "claude.json");
    assert_eq!(named.keys["tui"], serde_json::json!("fullscreen"));
    for (name, path) in [("up", "../shared/claude.json"), ("abs", absolute.as_str())] {
        let reason = package_refused(&kinds_dir, name, 1);
        assert!(
            reason.contains(&format!("kinds.{name}.agents.programs.claude.file"))
                && reason.contains(&format!("{path} lies outside the package"))
                && !reason.contains("outside\\\""),
            "{name}: {reason}"
        );
    }
    let reason = package_refused(&kinds_dir, "linked", 1);
    assert!(
        reason.contains("claude.json") && reason.contains("symbolic link"),
        "{reason}"
    );
}

//! The change profile over OpenSpec (C.3; design 8): the change seam's
//! scenarios against it on scratch repositories, a change of another
//! schema read through the pinned `openspec status`, and the file reading
//! held to `openspec` at five revisions of one change (59).

use crate::common::*;
use flume::git::Git;
use flume::profiles::change::openspec::{status, OpenSpec};
use flume_conformance::scenario::pass_all;
use flume_conformance::seams::change::{self, ChangeHarness};
use flume_seams::*;
use std::cell::RefCell;
use std::process::Command;

/// Writes the change `name` so that it stands as `facts` say: its folder
/// where it exists, its archive's where it is archived, the artifacts
/// before the tasks where its planning is complete, and its tasks.
fn write_change(repository: &Repository, name: &str, facts: &ChangeFacts) {
    let mut folders = vec![];
    if facts.exists {
        folders.push(format!("openspec/changes/{name}"));
    }
    if facts.archived {
        folders.push(format!("openspec/changes/archive/2026-10-28-{name}"));
    }
    for folder in folders {
        repository.write(&format!("{folder}/.openspec.yaml"), "schema: spec-driven\n");
        if facts.planning_complete {
            repository.write(&format!("{folder}/proposal.md"), "# Proposal\n");
            repository.write(&format!("{folder}/specs/one/spec.md"), "# One\n");
            repository.write(&format!("{folder}/design.md"), "# Design\n");
        }
        if facts.planning_complete || !facts.tasks.is_empty() {
            repository.write(&format!("{folder}/tasks.md"), &tasks_file(facts));
        }
    }
}

/// A `tasks.md` holding the change's tasks: those in a group with no
/// number or title, or in none, above every heading, then each group's
/// `## <n>. <title>` heading and its tasks.
fn tasks_file(facts: &ChangeFacts) -> String {
    let line = |task: &Task| {
        format!(
            "- [{}] {}\n",
            if task.ticked { "x" } else { " " },
            task.text
        )
    };
    let headed = |task: &Task| {
        task.group
            .and_then(|group| facts.groups.get(group))
            .is_some_and(|group| group.number.is_some())
    };
    let mut text = String::from("# Tasks\n\n");
    text.extend(facts.tasks.iter().filter(|task| !headed(task)).map(line));
    for (at, group) in facts.groups.iter().enumerate() {
        if let (Some(number), Some(title)) = (&group.number, &group.title) {
            text.push_str(&format!("\n## {number}. {title}\n\n"));
            text.extend(facts.tasks_of(at).map(line));
        }
    }
    text
}

#[derive(Default)]
struct Harness {
    seam: OpenSpec,
    kept: RefCell<Vec<Repository>>,
}

impl ChangeHarness for Harness {
    type Seam = OpenSpec;

    fn seam(&self) -> &OpenSpec {
        &self.seam
    }

    fn committed(&self, name: &str, facts: &ChangeFacts) -> (String, Revision) {
        let repository = Repository::new("change");
        write_change(&repository, name, facts);
        let revision = Revision::new(repository.commit("the change"));
        let named = (repository.name(), revision);
        self.kept.borrow_mut().push(repository);
        named
    }
}

#[test]
fn the_change_profile_passes_the_change_scenarios() {
    pass_all("change", &change::scenarios(), Harness::default);
}

#[test]
fn the_change_lies_at_its_folder() {
    assert_eq!(OpenSpec.path("plan"), "openspec/changes/plan");
}

#[test]
fn reading_writes_nothing() {
    let repository = Repository::new("untouched");
    write_change(
        &repository,
        "plan",
        &ChangeFacts {
            exists: true,
            planning_complete: true,
            tasks: vec![Task {
                text: "1.1 one".into(),
                ticked: false,
                group: Some(0),
            }],
            archived: false,
            groups: vec![TaskGroup {
                number: Some("1".into()),
                title: Some("One".into()),
            }],
        },
    );
    let revision = Revision::new(repository.commit("the change"));
    let before = repository.git(&["status", "--porcelain", "--ignored"]);
    let refs = repository.git(&["for-each-ref"]);
    let worktrees = repository.git(&["worktree", "list", "--porcelain"]);
    OpenSpec.read_change(&repository.name(), &revision, "plan");
    assert_eq!(
        repository.git(&["status", "--porcelain", "--ignored"]),
        before
    );
    assert_eq!(repository.git(&["for-each-ref"]), refs);
    assert_eq!(
        repository.git(&["worktree", "list", "--porcelain"]),
        worktrees
    );
}

/// What the pinned `openspec` gives of the change in the checkout: its
/// existence and planning complete from `status`, its tasks from
/// `instructions apply`, each task's text and whether it is ticked, since
/// OpenSpec gives no task its group.
fn openspec_reads(dir: &std::path::Path, name: &str) -> (bool, bool, Vec<(String, bool)>) {
    let run = |args: &[&str]| {
        let output = Command::new("openspec")
            .args(args)
            .current_dir(dir)
            .env("OPENSPEC_TELEMETRY", "0")
            .env("OPENSPEC_NO_UPDATE_CHECK", "1")
            .env("NO_COLOR", "1")
            .output()
            .expect("the pinned openspec runs");
        serde_json::from_slice::<serde_json::Value>(&output.stdout).ok()
    };
    let status = run(&["status", "--change", name, "--json"]);
    let planning = status
        .as_ref()
        .and_then(|status| status.get("isPlanningComplete"))
        .and_then(|complete| complete.as_bool());
    let exists = planning.is_some();
    let tasks = run(&["instructions", "apply", "--change", name, "--json"])
        .and_then(|apply| apply.get("tasks").cloned())
        .and_then(|tasks| tasks.as_array().cloned())
        .unwrap_or_default()
        .iter()
        .map(|task| {
            (
                task["description"].as_str().unwrap_or_default().to_string(),
                task["done"].as_bool().unwrap_or_default(),
            )
        })
        .collect();
    (exists, planning.unwrap_or(false), tasks)
}

#[test]
fn the_file_reading_agrees_with_openspec_at_five_revisions() {
    let repository = Repository::new("agreement");
    repository.write("openspec/config.yaml", "schema: spec-driven\n");
    repository.write("openspec/specs/.keep", "");
    let folder = "openspec/changes/plan";
    let mut revisions = vec![];

    repository.write(&format!("{folder}/.openspec.yaml"), "schema: spec-driven\n");
    repository.write(&format!("{folder}/proposal.md"), "# Proposal\n");
    revisions.push(repository.commit("created"));

    repository.write(&format!("{folder}/specs/one/spec.md"), "# One\n");
    repository.write(&format!("{folder}/design.md"), "# Design\n");
    revisions.push(repository.commit("specified"));

    repository.write(
        &format!("{folder}/tasks.md"),
        "# Tasks\n\n## 1. One\n\n- [ ] 1.1 the first\n- [ ] 1.2 the second\n\n## 2. Two\n\n- [ ] 2.1 the third\n",
    );
    revisions.push(repository.commit("planned"));

    repository.write(
        &format!("{folder}/tasks.md"),
        "# Tasks\n\n## 1. One\n\n- [x] 1.1 the first\n- [ ] 1.2 the second\n\n## 2. Two\n\n- [ ] 2.1 the third\n",
    );
    revisions.push(repository.commit("one ticked"));

    std::fs::create_dir_all(repository.dir.join("openspec/changes/archive")).expect("the archive");
    std::fs::rename(
        repository.dir.join(folder),
        repository
            .dir
            .join("openspec/changes/archive/2026-10-28-plan"),
    )
    .expect("archived");
    revisions.push(repository.commit("archived"));

    for (n, revision) in revisions.iter().enumerate() {
        let read = match OpenSpec.read_change(&repository.name(), &Revision::new(revision), "plan")
        {
            ChangeRead::Read { facts } => facts,
            other => panic!("revision {n} read as {other:?}"),
        };
        repository.git(&["checkout", "--quiet", revision]);
        let (exists, planning, tasks) = openspec_reads(&repository.dir, "plan");
        assert_eq!(read.exists, exists, "existence at revision {n}");
        if exists {
            assert_eq!(
                read.planning_complete, planning,
                "planning complete at revision {n}"
            );
            assert_eq!(
                read.tasks
                    .iter()
                    .map(|task| (task.text.clone(), task.ticked))
                    .collect::<Vec<_>>(),
                tasks,
                "the tasks at revision {n}"
            );
            if n >= 2 {
                assert_eq!(read.groups.len(), 2, "the groups at revision {n}");
                assert_eq!(read.first_unticked_group(), Some(0), "at revision {n}");
            }
        }
        assert_eq!(read.archived, n == 4, "archived at revision {n}");
    }
    repository.git(&["checkout", "--quiet", "main"]);
}

#[test]
fn a_change_of_another_schema_is_read_through_openspec() {
    let repository = Repository::new("other-schema");
    let git = Git::at(&repository.name());
    let folder = "openspec/changes/plan";
    repository.write(&format!("{folder}/.openspec.yaml"), "schema: spec-driven\n");
    repository.write(&format!("{folder}/proposal.md"), "# Proposal\n");
    let revision = Revision::new(repository.commit("created"));
    assert_eq!(status(&git, &revision, "plan"), Ok(Some(false)));
    assert_eq!(status(&git, &revision, "absent"), Ok(None));
    let before = std::fs::read_dir(std::env::temp_dir())
        .expect("the temporary directory")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("flume-openspec-")
        })
        .count();
    repository.write(
        &format!("{folder}/.openspec.yaml"),
        "schema: a-schema-of-its-own\n",
    );
    let revision = Revision::new(repository.commit("another schema"));
    let read = OpenSpec.read_change(&repository.name(), &revision, "plan");
    match read {
        ChangeRead::Read { facts } => assert!(facts.exists, "{facts:?}"),
        ChangeRead::CouldNotRead { reason } => assert!(
            reason.contains("openspec"),
            "a change of another schema not read through openspec: {reason}"
        ),
    }
    let after = std::fs::read_dir(std::env::temp_dir())
        .expect("the temporary directory")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .starts_with("flume-openspec-")
        })
        .count();
    assert!(after <= before, "a checkout for the read was left");
}

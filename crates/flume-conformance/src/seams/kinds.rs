//! The kinds seam's scenarios (B.2, 84, 85), and the binding (105).

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
    let own = harness.seam().flumes_own();
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

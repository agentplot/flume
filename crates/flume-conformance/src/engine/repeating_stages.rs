//! A stage repeats per task group (30, 140 to 142; record 0019): each
//! start of a repeating stage is one fresh agent given the first group of
//! the change holding a task unticked at its head, or none where every
//! task is ticked; a repetition delivers once its group is ticked in a
//! commit since it started, and flume ends it and starts the next as any
//! start, waiting for room; the stage delivers, and its one end is
//! recorded and delivered, once every task is ticked.

use super::world::*;
use crate::fake_harness::flumes_own;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_fakes::Plan;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "repeating stages: the first repetition of a code stage on a change of three groups records group 1, its identity naming start 1",
            cites: &["30", "33", "48", "140"],
            run: the_first_repetition,
        },
        Scenario {
            name: "repeating stages: a failed submission sending the place back with every task ticked starts one repetition with no group and the merge queue's words, delivered once a commit leaves every task ticked",
            cites: &["112", "140", "S52", "30"],
            run: a_send_back_with_every_task_ticked,
        },
        Scenario {
            name: "repeating stages: three groups take three agents in turn, never two live at once, and only after the third does the stage deliver and verify start",
            cites: &["S50", "30", "40", "140"],
            run: three_groups_three_agents,
        },
        Scenario {
            name: "repeating stages: a repetition given group 1 that ticks groups 1 and 2 is followed by one given group 3",
            cites: &["140", "40"],
            run: more_than_its_group_ticked,
        },
        Scenario {
            name: "repeating stages: a stage of three groups is delivered to its recipient as one end, the stage's, at the head its last repetition delivered at",
            cites: &["43", "140"],
            run: one_notice_for_the_stage,
        },
        Scenario {
            name: "repeating stages: a repetition ending on a full host lets a stage of higher priority start first, and the next repetition waits for room in the queue",
            cites: &["70", "140", "46"],
            run: a_full_host_between_repetitions,
        },
    ]
}

/// A change of three groups, five tasks in all.
pub const GROUPS: &[(&str, &str, usize)] = &[("1", "One", 2), ("2", "Two", 2), ("3", "Three", 1)];

/// The repository's `change`: flume's own change@1 with its code stage
/// repeating per task group (140).
pub fn repeating_change() -> Kind {
    let mut change = flumes_own()
        .into_iter()
        .find(|kind| kind.name == "change")
        .expect("flume's own change");
    change.stages[1].repeats = true;
    change
}

/// The branch of the change place `name`.
pub fn branch(name: &str) -> String {
    format!("change/{name}")
}

/// The repeating change `name` launched with `recipients`, its change
/// written as three groups none ticked, and code's first repetition
/// begun.
pub fn code_begun(world: &World, name: &str, recipients: &[&str]) -> Result<PlaceId, String> {
    world.kinds.set_kinds(REPOSITORY, vec![repeating_change()]);
    let mut launch = launch("change", name, Some("plan it"));
    launch.recipients = recipients
        .iter()
        .map(|recipient| recipient.to_string())
        .collect();
    launched(&world.engine.launch(&launch))?;
    let id = World::id("change", name);
    world.settle(&id);
    world.tick_groups(&branch(name), name, GROUPS, 0);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code's first repetition"
    );
    Ok(id)
}

/// The group the live or last start of the place's code stage records.
pub fn group_of(world: &World, id: &PlaceId) -> Result<Option<GroupHeld>, String> {
    Ok(world.held(id).ok_or("the place is gone")?.stages[1]
        .start
        .clone()
        .ok_or("code never started")?
        .group)
}

/// Each start the terminal opened, in order, and whether one was opened
/// while another of the place's was still open.
fn opened_in_turn(world: &World) -> (Vec<String>, bool) {
    let mut open: Option<String> = None;
    let mut opened = vec![];
    let mut overlapped = false;
    for call in world.terminal.log.calls() {
        let mut words = call.split(' ');
        match (words.next(), words.next()) {
            (Some("open"), Some(identity)) => {
                overlapped |= open.is_some();
                open = Some(identity.to_string());
                opened.push(identity.to_string());
            }
            (Some("close"), Some(identity)) if open.as_deref() == Some(identity) => open = None,
            _ => {}
        }
    }
    (opened, overlapped)
}

fn the_first_repetition(_: &()) -> Outcome {
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    ensure_eq!(
        group_of(&world, &id)?,
        Some(GroupHeld {
            index: 0,
            number: Some("1".into()),
            title: Some("One".into()),
            tasks: vec!["1.1 task".into(), "1.2 task".into()],
        }),
        "the group the first repetition records"
    );
    let held = world.held(&id).ok_or("the place is gone")?;
    let start = held.stages[1].start.clone().ok_or("no start")?;
    ensure_eq!(
        start.identity,
        World::identity("change", "plan", "code", 1),
        "the repetition's identity"
    );
    ensure_eq!(held.stages[1].starts, 1, "code's starts");
    ensure_eq!(
        world.terminal.log.count("open"),
        2,
        "agents opened, construct's and code's"
    );
    ensure_eq!(
        held.stages
            .iter()
            .filter(|stage| stage.start.as_ref().is_some_and(|start| start.live))
            .count(),
        1,
        "agents live"
    );
    Ok(())
}

fn a_send_back_with_every_task_ticked(_: &()) -> Outcome {
    let world = World::new();
    world
        .queue
        .plan_submits(vec![Plan::Fail("check test-y failed".into())]);
    let id = code_begun(&world, "plan", &[])?;
    world.tick_groups(&branch("plan"), "plan", GROUPS, 5);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "verify"),
        Some(StageState::Begun),
        "verify, once every task is ticked"
    );
    super::settling::archive(&world, "plan");
    world.settle(&id);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    let back = World::identity("change", "plan", "code", 2);
    let prompt = world.prompt(&id, &back).ok_or("code did not start again")?;
    ensure!(
        prompt.contains("check test-y failed"),
        "the merge queue's words are not in {prompt}"
    );
    ensure_eq!(group_of(&world, &id)?, None, "the send-back's group");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the send-back's repetition"
    );
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "the send-back's repetition, with nothing committed since it started"
    );
    world.tick_groups(&branch("plan"), "plan", GROUPS, 5);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "code, once a commit since its start leaves every task ticked"
    );
    ensure!(
        world
            .stage_state(&id, "verify")
            .is_some_and(StageState::running),
        "verify did not start again"
    );
    ensure_eq!(
        world.held(&id).map(|held| held.stages[1].starts),
        Some(2),
        "code's starts: one repetition ticking every group, then the send-back's"
    );
    Ok(())
}

fn three_groups_three_agents(_: &()) -> Outcome {
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    for (ticked, next) in [(2, "2"), (4, "3")] {
        world.tick_groups(&branch("plan"), "plan", GROUPS, ticked);
        world.settle(&id);
        ensure_eq!(
            world.stage_state(&id, "code"),
            Some(StageState::Begun),
            "code after {ticked} tasks ticked"
        );
        ensure_eq!(
            group_of(&world, &id)?.and_then(|group| group.number),
            Some(next.to_string()),
            "the next repetition's group"
        );
        ensure_eq!(
            world.stage_state(&id, "verify"),
            Some(StageState::NotReached),
            "verify, while a task is unticked"
        );
        ensure!(
            world.held(&id).is_some_and(|held| held
                .ends
                .iter()
                .all(|end| !matches!(&end.of, EndOf::Stage { stage, .. } if stage == "code"))),
            "a repetition's delivery was recorded as an end of code"
        );
    }
    world.tick_groups(&branch("plan"), "plan", GROUPS, 5);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "code, once every task is ticked"
    );
    ensure!(
        world
            .stage_state(&id, "verify")
            .is_some_and(StageState::running),
        "verify did not start once code delivered"
    );
    let (opened, overlapped) = opened_in_turn(&world);
    let code: Vec<String> = (1..=3)
        .map(|start| World::identity("change", "plan", "code", start).to_string())
        .collect();
    ensure_eq!(
        opened
            .iter()
            .filter(|identity| identity.contains(":code:"))
            .cloned()
            .collect::<Vec<_>>(),
        code,
        "code's agents opened"
    );
    ensure!(!overlapped, "two agents were live at once: {opened:?}");
    Ok(())
}

fn more_than_its_group_ticked(_: &()) -> Outcome {
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    world.tick_groups(&branch("plan"), "plan", GROUPS, 4);
    world.settle(&id);
    ensure_eq!(
        group_of(&world, &id)?,
        Some(GroupHeld {
            index: 2,
            number: Some("3".into()),
            title: Some("Three".into()),
            tasks: vec!["3.1 task".into()],
        }),
        "the next repetition's group"
    );
    ensure_eq!(
        world.held(&id).map(|held| held.stages[1].starts),
        Some(2),
        "code's starts"
    );
    Ok(())
}

fn one_notice_for_the_stage(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let id = code_begun(&world, "plan", &["ops"])?;
    world.tick_groups(&branch("plan"), "plan", GROUPS, 2);
    world.settle(&id);
    world.tick_groups(&branch("plan"), "plan", GROUPS, 4);
    world.settle(&id);
    let last = world.tick_groups(&branch("plan"), "plan", GROUPS, 5);
    world.settle(&id);
    let code = |line: &String| line.contains(" code ");
    ensure_eq!(
        world
            .terminal
            .notices("ops")
            .into_iter()
            .filter(code)
            .collect::<Vec<_>>(),
        vec!["flume: change--plan code delivered; read: flume status change--plan".to_string()],
        "code's notices to ops"
    );
    let ends: Vec<EndOf> = world
        .held(&id)
        .ok_or("the place is gone")?
        .ends
        .into_iter()
        .map(|end| end.of)
        .filter(|of| matches!(of, EndOf::Stage { stage, .. } if stage == "code"))
        .collect();
    ensure_eq!(ends.len(), 1, "code's ends: {ends:?}");
    ensure!(
        matches!(&ends[0], EndOf::Stage { start: 3, state: StageState::Delivered, end, .. }
            if end.head.as_ref() == Some(&last)),
        "code's end: {:?}",
        ends[0]
    );
    Ok(())
}

fn a_full_host_between_repetitions(_: &()) -> Outcome {
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    world.host.set_bound(Some(1));
    launched(
        &world
            .engine
            .launch(&launch("fix", "urgent", Some("mend it"))),
    )?;
    let urgent = World::id("fix", "urgent");
    ensure!(
        world.engine.priority(&urgent, 1).refusal().is_none(),
        "the priority was refused"
    );
    ensure_eq!(
        world.stage_state(&urgent, "code"),
        Some(StageState::WaitingForRoom),
        "the fix, on a full host"
    );
    world.tick_groups(&branch("plan"), "plan", GROUPS, 2);
    world.settle(&id);
    ensure!(
        world
            .stage_state(&urgent, "code")
            .is_some_and(StageState::running),
        "the stage of higher priority did not start at the room the repetition freed"
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::WaitingForRoom),
        "the next repetition, on a full host"
    );
    let queued = match world.engine.status(None) {
        flume_core::Answer::Status { host, .. } => host
            .queue
            .into_iter()
            .map(|queued| (queued.place, queued.stage))
            .collect::<Vec<_>>(),
        other => return Err(format!("status answered {other:?}")),
    };
    ensure_eq!(
        queued,
        vec![(id.clone(), "code".to_string())],
        "the host's queue"
    );
    ensure_eq!(
        world.terminal.log.of("close").len(),
        2,
        "agents ended: construct's and the first repetition's, none for room"
    );
    Ok(())
}

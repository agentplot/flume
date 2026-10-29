//! A stage repeats per task group (30, 140 to 142; record 0019): each
//! start of a repeating stage is one fresh agent given the first group of
//! the change holding a task unticked at its head, or none where every
//! task is ticked; a repetition delivers once its group is ticked in a
//! commit since it started, and flume ends it and starts the next as any
//! start, waiting for room; the stage delivers, and its one end is
//! recorded and delivered, once every task is ticked. `{{group}}` and
//! `{{notes}}` fill for a repeating stage alone, and flume never reads
//! the notes (141; core.31).

use super::world::*;
use crate::fake_harness::flumes_own;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Answer;
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
            name: "repeating stages: flume prompt renders a repeating code stage on group 2 with Group 2., its title and tasks, and the notes' path under the place's folder",
            cites: &["141", "27", "29"],
            run: the_parts_rendered,
        },
        Scenario {
            name: "repeating stages: notes a repetition writes are never read: status, the end and the next prompt hold nothing of them, the file is as the agent left it, and it goes with the place's folder at the tidy",
            cites: &["141", "62"],
            run: notes_never_read,
        },
        Scenario {
            name: "repeating stages: a stage that does not repeat asking for {{group}} is refused naming the part, and no agent starts",
            cites: &["27", "141"],
            run: a_part_asked_for_by_a_stage_that_does_not_repeat,
        },
        Scenario {
            name: "repeating stages: a repetition on group 2 going quiet with a task of it unticked is sent back naming group 2 and the task left",
            cites: &["110", "S51", "140"],
            run: quiet_with_its_group_unticked,
        },
        Scenario {
            name: "repeating stages: a repetition ending on a full host lets a stage of higher priority start first, and the next repetition waits for room in the queue",
            cites: &["70", "140", "46"],
            run: a_full_host_between_repetitions,
        },
        Scenario {
            name: "repeating stages: a caller stopping code's agent on group 4 leaves the stage waiting for a retry, status naming group 4, and the place stalls five minutes from the stop, not from the stage's first start",
            cites: &["51", "142", "130", "S51"],
            run: a_stop_names_the_group,
        },
        Scenario {
            name: "repeating stages: a repetition on group 2 stopping short needing an answer ends naming group 2, its notice too, and the caller's retry starts a fresh agent on group 2 with the question and the answer, never on group 1",
            cites: &["42", "49", "43", "142", "S51"],
            run: a_retry_after_a_stop_short,
        },
        Scenario {
            name: "repeating stages: four repetitions each working twenty minutes and quiet twenty more under a thirty-minute limit are never reported working past it nor stuck, though the stage runs past two hours",
            cites: &["44", "118", "142"],
            run: a_long_stage_with_short_repetitions,
        },
        Scenario {
            name: "repeating stages: the stage's one end carries every line each repetition's run wrote of what it made, in order",
            cites: &["129", "140", "43"],
            run: every_repetitions_made_lines,
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
    change.stages[1].prompt = "Build {{change}}, one task group of it, ticking each task.\n\n{{needs}}\n\n{{group}}\n\nThe notes: {{notes}}{{#words}}\n\n{{words}}{{/words}}".into();
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
    code_begun_on(world, name, recipients, GROUPS)
}

/// The repeating change `name` launched with `recipients`, its change
/// written as `groups` none ticked, and code's first repetition begun.
pub fn code_begun_on(
    world: &World,
    name: &str,
    recipients: &[&str],
    groups: &[(&str, &str, usize)],
) -> Result<PlaceId, String> {
    world.kinds.set_kinds(REPOSITORY, vec![repeating_change()]);
    let mut launch = launch("change", name, Some("plan it"));
    launch.recipients = recipients
        .iter()
        .map(|recipient| recipient.to_string())
        .collect();
    launched(&world.engine.launch(&launch))?;
    let id = World::id("change", name);
    world.settle(&id);
    world.tick_groups(&branch(name), name, groups, 0);
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

/// The notes' path of the place's code stage.
fn notes(world: &World, id: &PlaceId) -> String {
    format!("{}/notes/code.md", world.store.folder(id))
}

fn the_parts_rendered(_: &()) -> Outcome {
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    world.tick_groups(&branch("plan"), "plan", GROUPS, 2);
    world.settle(&id);
    let text = match world.engine.prompt(&id, "code") {
        Answer::Prompt { text, .. } => text,
        other => return Err(format!("prompt answered {other:?}")),
    };
    for named in [
        "Group 2. Two",
        "- 2.1 task",
        "- 2.2 task",
        &format!("The notes: {}", notes(&world, &id)),
    ] {
        ensure!(text.contains(named), "{named} is not in {text}");
    }
    ensure!(!text.contains("1.1 task"), "group 1 is in {text}");
    let second = World::identity("change", "plan", "code", 2);
    let sent = world.prompt(&id, &second).ok_or("no second repetition")?;
    ensure!(
        sent.contains("Group 2. Two"),
        "the second repetition was not sent group 2: {sent}"
    );
    Ok(())
}

fn notes_never_read(_: &()) -> Outcome {
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    let first = world
        .prompt(&id, &World::identity("change", "plan", "code", 1))
        .ok_or("no first prompt")?;
    let path = notes(&world, &id);
    world.host.write_file(&path);
    let written = world.host.now();
    world.host.advance(60);
    world.tick_groups(&branch("plan"), "plan", GROUPS, 2);
    world.settle(&id);
    let second = world
        .prompt(&id, &World::identity("change", "plan", "code", 2))
        .ok_or("no second repetition")?;
    let shared = |text: &str| text.split("Group ").next().unwrap_or_default().to_string();
    ensure_eq!(
        shared(&second),
        shared(&first),
        "the two repetitions' prompts up to the group"
    );
    let status = serde_json::to_string(&world.status(&id)?).map_err(|error| error.to_string())?;
    ensure!(
        !status.contains("notes/code.md"),
        "status names the notes: {status}"
    );
    world.tick_groups(&branch("plan"), "plan", GROUPS, 5);
    world.settle(&id);
    let ends = serde_json::to_string(&world.held(&id).ok_or("the place is gone")?.ends)
        .map_err(|error| error.to_string())?;
    ensure!(!ends.contains("notes"), "an end names the notes: {ends}");
    ensure!(
        world
            .host
            .log
            .calls()
            .iter()
            .all(|call| !call.contains("notes/")),
        "flume asked the host of the notes: {:?}",
        world.host.log.calls()
    );
    ensure_eq!(
        world.host.write_time(&path),
        Some(written),
        "the notes, as the agent left them"
    );
    super::settling::archive(&world, "plan");
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place is kept");
    ensure!(
        world
            .store
            .log
            .of("remove")
            .iter()
            .any(|call| call.contains(&id.0)),
        "the place's held state and its folder, the notes with it, were not removed"
    );
    Ok(())
}

fn a_part_asked_for_by_a_stage_that_does_not_repeat(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "bump",
            false,
            vec![stage(
                "work",
                Some(Deliverable::Commits),
                "{{group}} {{needs}}",
            )],
        )],
    );
    launched(&world.engine.launch(&launch("bump", "one", Some("x"))))?;
    let id = World::id("bump", "one");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(held.stages[0].state, StageState::Refused, "the stage");
    ensure_eq!(
        held.stages[0].refused_part.as_deref(),
        Some("group"),
        "the part named"
    );
    ensure_eq!(world.terminal.log.count("open"), 0, "agents opened");
    Ok(())
}

fn quiet_with_its_group_unticked(_: &()) -> Outcome {
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    world.tick_groups(&branch("plan"), "plan", GROUPS, 2);
    world.settle(&id);
    world.tick_groups(&branch("plan"), "plan", GROUPS, 3);
    world.settle(&id);
    let second = World::identity("change", "plan", "code", 2);
    let words = match world.engine.stop_check(&second.to_string()) {
        Answer::StopCheck { words } => words.ok_or("the repetition was not sent back")?,
        other => return Err(format!("the stop check answered {other:?}")),
    };
    for named in ["Group 2. Two", "Left unticked: 2.2 task"] {
        ensure!(words.contains(named), "{named} is not in {words}");
    }
    ensure!(
        !words.contains("2.1 task"),
        "a ticked task is named: {words}"
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

/// A change of four groups, one task each.
pub const FOUR: &[(&str, &str, usize)] = &[
    ("1", "One", 1),
    ("2", "Two", 1),
    ("3", "Three", 1),
    ("4", "Four", 1),
];

/// The live start of the place's code stage.
fn code_start(world: &World, id: &PlaceId) -> Result<StartHeld, String> {
    world.held(id).ok_or("the place is gone")?.stages[1]
        .start
        .clone()
        .filter(|start| start.live)
        .ok_or_else(|| "code has no live start".to_string())
}

fn a_stop_names_the_group(_: &()) -> Outcome {
    let world = World::new();
    let id = code_begun_on(&world, "plan", &[], FOUR)?;
    for ticked in 1..=3 {
        world.host.advance(20 * 60);
        world.tick_groups(&branch("plan"), "plan", FOUR, ticked);
        world.settle(&id);
    }
    ensure_eq!(
        group_of(&world, &id)?.and_then(|group| group.number),
        Some("4".to_string()),
        "the fourth repetition's group"
    );
    world.host.advance(60);
    let stopped_at = world.host.now();
    let answer = world.engine.stop(&id, true);
    let status = match answer {
        Answer::Acted {
            status: Some(status),
            ..
        } => status,
        other => return Err(format!("the stop answered {other:?}")),
    };
    let code = &status.stages[1];
    ensure_eq!(code.state, StageState::Stopped, "code after the stop");
    ensure_eq!(
        code.group
            .as_ref()
            .map(|group| (group.place, group.of, group.number.clone())),
        Some((4, 4, Some("4".to_string()))),
        "the group the stop's answer names"
    );
    let waiting = status
        .waiting_on
        .ok_or("the stopped stage waits on nobody")?;
    ensure_eq!(
        (waiting.stage.as_deref(), waiting.since),
        (Some("code"), Some(stopped_at)),
        "the retry it waits for, since the stop"
    );
    world.host.advance(4 * 60);
    world.settle(&id);
    let status = world.status(&id)?;
    ensure!(
        !status.stalled,
        "stalled four minutes after the stop, the stage having run an hour"
    );
    ensure_eq!(
        status.stages[1]
            .group
            .as_ref()
            .and_then(|group| group.number.clone()),
        Some("4".to_string()),
        "the group status names"
    );
    world.host.advance(2 * 60);
    world.settle(&id);
    ensure!(
        world.status(&id)?.stalled,
        "not stalled six minutes after the stop"
    );
    Ok(())
}

fn a_retry_after_a_stop_short(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let id = code_begun(&world, "plan", &["ops"])?;
    world.tick_groups(&branch("plan"), "plan", GROUPS, 2);
    world.settle(&id);
    let second = code_start(&world, &id)?;
    let question = "Which schema does task 2.1 mean?";
    let answer = world.engine.needs(
        &second.identity.to_string(),
        &second.secret.0,
        question,
        StopKind::NeedsAnswer,
    );
    ensure!(answer.refusal().is_none(), "the stop-short: {answer:?}");
    world.settle(&id);
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(
        held.stages[1].state,
        StageState::EndedShort,
        "code after the stop-short"
    );
    ensure_eq!(
        held.stages[1]
            .end
            .as_ref()
            .and_then(|end| end.group.as_ref())
            .and_then(|group| group.number.clone()),
        Some("2".to_string()),
        "the group the end names"
    );
    ensure!(
        world.terminal.notices("ops").contains(
            &"flume: change--plan code on group 2. Two needs an answer; read: flume status change--plan"
                .to_string()
        ),
        "the notice of the stop-short: {:?}",
        world.terminal.notices("ops")
    );
    let retried = world.engine.retry(
        &id,
        flume_core::Retry::Stage("code".into()),
        Some("the one in design 3".into()),
        false,
    );
    ensure!(retried.refusal().is_none(), "the retry: {retried:?}");
    let third = code_start(&world, &id)?;
    ensure_eq!(
        third.identity,
        World::identity("change", "plan", "code", 3),
        "the retry's fresh agent"
    );
    ensure_eq!(
        third.group.and_then(|group| group.number),
        Some("2".to_string()),
        "the retry's group"
    );
    let prompt = third.prompt.text;
    for named in [
        "Group 2. Two",
        "The question: Which schema does task 2.1 mean?",
        "The answer: the one in design 3",
    ] {
        ensure!(prompt.contains(named), "{named} is not in {prompt}");
    }
    ensure!(!prompt.contains("Group 1."), "group 1 is in {prompt}");
    Ok(())
}

fn a_long_stage_with_short_repetitions(_: &()) -> Outcome {
    let world = World::new();
    let id = code_begun_on(&world, "plan", &[], FOUR)?;
    let began = world.host.now();
    for ticked in 1..=4 {
        let start = World::identity("change", "plan", "code", ticked as u32);
        world.terminal.set_state(&start, AgentState::Working);
        world.settle(&id);
        world.host.advance(20 * 60);
        world.settle(&id);
        let agent = world.status(&id)?.agent.ok_or("no agent")?;
        ensure_eq!(agent.identity, start.to_string(), "the repetition working");
        ensure!(
            !agent.past_limit,
            "repetition {ticked} reported past its limit after twenty minutes' work"
        );
        world.terminal.set_state(&start, AgentState::Waiting);
        world.settle(&id);
        world.host.advance(20 * 60);
        world.settle(&id);
        ensure_eq!(
            world.stage_state(&id, "code"),
            Some(StageState::Begun),
            "repetition {ticked} after twenty minutes quiet"
        );
        world.tick_groups(&branch("plan"), "plan", FOUR, ticked);
        world.settle(&id);
    }
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Delivered),
        "code, its four repetitions done"
    );
    ensure!(
        world.host.now() - began >= 160 * 60,
        "the stage ran {} seconds",
        world.host.now() - began
    );
    ensure!(
        world.held(&id).is_some_and(|held| held
            .ends
            .iter()
            .all(|end| !matches!(&end.of, EndOf::Stage { stage, state: StageState::Stuck, .. } if stage == "code"))),
        "a repetition was recorded stuck"
    );
    Ok(())
}

fn every_repetitions_made_lines(_: &()) -> Outcome {
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    let lines = [
        r#"{"name":"schema","what":"one"}"#,
        r#"{"name":"api","what":"two"}"#,
        "not one object",
    ];
    for (ticked, line) in [(2, lines[0]), (4, lines[1]), (5, lines[2])] {
        world.store.record_made(&id, line);
        world.tick_groups(&branch("plan"), "plan", GROUPS, ticked);
        world.settle(&id);
    }
    let held = world.held(&id).ok_or("the place is gone")?;
    ensure_eq!(
        held.stages[1].state,
        StageState::Delivered,
        "code, every task ticked"
    );
    let made: Vec<Made> = lines.iter().map(|line| Made::read(line)).collect();
    ensure_eq!(
        held.stages[1].end.as_ref().map(|end| end.made.clone()),
        Some(made.clone()),
        "the stage's end"
    );
    ensure_eq!(
        held.stages[1].made_from,
        None,
        "the count kept once the stage ended"
    );
    let recorded: Vec<Vec<Made>> = held
        .ends
        .iter()
        .filter_map(|end| match &end.of {
            EndOf::Stage { stage, end, .. } if stage == "code" => Some(end.made.clone()),
            _ => None,
        })
        .collect();
    ensure_eq!(recorded, vec![made], "code's one recorded end");
    Ok(())
}

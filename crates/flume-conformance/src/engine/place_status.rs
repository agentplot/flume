//! Status (43, 56 to 62, 69): every fact read where it lives when asked,
//! at one head per place, from one core, and nothing written, sent or
//! started.

use super::account_pools::{account, fix_in, host, pool, spent};
use super::agent_modes::{in_mode, in_programs, the_fakes_map};
use super::agent_settings::for_every_stage;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{
    AccountStatus, Act, Answer, Cache, ModeStatus, MoveStatus, PlaceStatus, PoolStatus, Retry,
    WaitingOn, Waits,
};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "status: asked three times of one place, it answers the same and starts or tells nothing",
            cites: &["S14", "57", "60", "69", "72"],
            run: s14_asked_three_times,
        },
        Scenario {
            name: "status: a change ticked between two asks is read anew, and neither ask writes",
            cites: &["58", "57", "59"],
            run: a_change_ticked_between_asks,
        },
        Scenario {
            name: "status: every fact of clause 56 for every place, side by side",
            cites: &["56", "43", "7", "19", "48"],
            run: every_fact_of_every_place,
        },
        Scenario {
            name: "status: a place no longer held is not answered for",
            cites: &["62", "58"],
            run: a_place_gone,
        },
        Scenario {
            name: "status: a place whose construct ended short waits on a retry of construct, since then, with the question whole",
            cites: &["56", "130", "42", "41"],
            run: waiting_on_an_answer,
        },
        Scenario {
            name: "status: a lost stage, a stopped one and a done round each name the move they wait on",
            cites: &["56", "130", "45", "51", "114"],
            run: the_moves_waited_on,
        },
        Scenario {
            name: "status: a place stalled on a retry of construct gives the move, since when, stalled, and its end delivered to one recipient and pending for another",
            cites: &["56", "130", "43"],
            run: a_stalled_place_with_a_pending_end,
        },
        Scenario {
            name: "status: a live place's clues and its base's count are the ones show was given, with where the terminal shows them",
            cites: &["139", "56", "60"],
            run: clues_in_status,
        },
        Scenario {
            name: "status: a repeating code stage's repetition on the second of three groups gives the group's place 2 of 3 with its number and title, `code 2/3` in the person's form and the clue's title",
            cites: &["56", "58", "139", "142"],
            run: status_mid_stage,
        },
        Scenario {
            name: "status: a repetition adding a fourth group to a change of three gives the count as 4, read at the head",
            cites: &["58", "142"],
            run: a_group_added_mid_stage,
        },
        Scenario {
            name: "status: a stage that moved from a spent account gives its agent's account from the strategy, and the move with why, when and the cache lost",
            cites: &["56", "145", "147"],
            run: a_stage_that_moved,
        },
        Scenario {
            name: "status: the host's pools give each pool's strategy and each account's agents running, bound and spent time, and no value of an account's environment",
            cites: &["56", "144", "145"],
            run: a_pool_with_a_spent_account,
        },
        Scenario {
            name: "status: an agent that took edits from the host's table gives its mode as written from host-kind, the value its profile passed and that it was read back, in the JSON form",
            cites: &["S66", "56", "162", "169"],
            run: a_mode_from_the_hosts_table,
        },
        Scenario {
            name: "status: an agent that took its program's entry of the launch's table gives the table as written from launch, the entry and the value passed",
            cites: &["56", "169"],
            run: a_tables_entry,
        },
        Scenario {
            name: "status: an agent whose profile reads no mode back gives its mode as not read back",
            cites: &["169", "161"],
            run: a_mode_not_read_back,
        },
    ]
}

/// The fix `given` launches, settled once: its agent's status, and the
/// same in the JSON form.
fn fix_with_a_mode(
    world: &World,
    given: &Launch,
) -> Result<(flume_core::AgentStatus, serde_json::Value), String> {
    launched(&world.engine.launch(given))?;
    let id = World::id("fix", &given.name);
    world.settle(&id);
    let status = world.status(&id)?;
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    Ok((status.agent.ok_or("no agent")?, json["agent"].clone()))
}

fn a_mode_from_the_hosts_table(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    world.agent.read_mode_back();
    world
        .host
        .set_settings(None, "fix", for_every_stage(in_mode(Mode::Edits)));
    let (agent, json) = fix_with_a_mode(&world, &launch("fix", "one", Some("mend it")))?;
    ensure_eq!(
        agent.mode,
        Some(ModeStatus {
            passed: "edit-files".into(),
            entry: None,
            read_back: true,
        }),
        "the agent's mode"
    );
    ensure_eq!(
        (
            json["settings"]["mode"].clone(),
            json["settings"]["from"]["mode"].clone(),
            json["mode"].clone()
        ),
        (
            serde_json::json!("edits"),
            serde_json::json!({"layer": "host-kind", "value": "edits"}),
            serde_json::json!({"passed": "edit-files", "entry": null, "read_back": true})
        ),
        "the mode in the JSON form"
    );
    Ok(())
}

fn a_tables_entry(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    world.agent.read_mode_back();
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = in_programs(&[("agent", "careful"), ("other", "edits")]);
    let (agent, json) = fix_with_a_mode(&world, &fix)?;
    ensure_eq!(
        agent.settings.from.get("mode"),
        Some(&Given {
            layer: Layer::Launch,
            value: r#"{ agent = "careful", other = "edits" }"#.into(),
        }),
        "the mode as written and its layer"
    );
    ensure_eq!(
        agent.mode,
        Some(ModeStatus {
            passed: "careful".into(),
            entry: Some("agent".into()),
            read_back: true,
        }),
        "the agent's mode"
    );
    ensure_eq!(
        (
            json["settings"]["mode"].clone(),
            json["mode"]["entry"].clone()
        ),
        (
            serde_json::json!({"agent": "careful", "other": "edits"}),
            serde_json::json!("agent")
        ),
        "the table and the entry in the JSON form"
    );
    Ok(())
}

fn a_mode_not_read_back(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = in_mode(Mode::Edits);
    let (agent, json) = fix_with_a_mode(&world, &fix)?;
    ensure_eq!(
        agent.mode.as_ref().map(|mode| mode.read_back),
        Some(false),
        "the mode read back"
    );
    ensure_eq!(
        json["mode"]["read_back"].clone(),
        serde_json::json!(false),
        "read_back in the JSON form"
    );
    Ok(())
}

fn clues_in_status(_: &()) -> Outcome {
    let world = World::new();
    let one = world.begun("build", Some(Deliverable::TasksTicked), "one")?;
    let two = World::id("build", "two");
    launched(&world.engine.launch(&launch("build", "two", Some("x"))))?;
    world.settle(&two);
    let asking = World::identity("build", "two", "work", 1);
    world.terminal.set_state(&asking, AgentState::Working);
    world.settle(&one);
    world.settle(&two);
    let shown = world.terminal.shown();
    let (_, last) = shown.last().ok_or("nothing shown")?;
    let Answer::Status { places, host } = world.engine().status(None) else {
        return Err("status refused".into());
    };
    let base = host.bases.first().ok_or("no base")?;
    ensure_eq!(host.bases.len(), 1, "the bases");
    ensure_eq!(base.base.as_str(), "main", "the base");
    ensure_eq!(base.count, last.base, "the count show was given");
    ensure_eq!(base.line, last.base.to_string(), "the count's line");
    ensure!(base.shown_at.is_some(), "the base's workspace unnamed");
    let two = places
        .iter()
        .find(|place| place.place == two)
        .ok_or("no place two")?;
    let clues = two.clues.as_ref().ok_or("no clues")?;
    ensure_eq!(
        (clues.title.as_str(), clues.display_agent.as_str()),
        ("work · working", "build--two"),
        "the title and the agent's name"
    );
    ensure_eq!(
        format!("{} · {}", last.stage, last.state),
        clues.title,
        "the title show was given"
    );
    ensure!(
        clues.shown_at.agent.is_some() && clues.shown_at.place.is_some(),
        "the place's tab or its agent's pane unnamed: {:?}",
        clues.shown_at
    );
    Ok(())
}

fn waiting_on_an_answer(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("change", "plan");
    launched(&world.engine.launch(&launch("change", "plan", Some("x"))))?;
    world.settle(&id);
    let construct = World::identity("change", "plan", "construct", 1);
    let secret = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?
        .secret;
    world.host.advance(300);
    let asked = world.host.now();
    let question = "Which design does the brief mean: the queue,\nor the log?";
    let answer = world.engine.needs(
        &construct.to_string(),
        &secret.0,
        question,
        StopKind::NeedsAnswer,
    );
    ensure!(answer.refusal().is_none(), "the stop-short was refused");
    world.host.advance(60);
    world.settle(&id);
    ensure_eq!(
        world.status(&id)?.waiting_on,
        Some(WaitingOn {
            act: Act::Retry,
            stage: Some("construct".into()),
            why: Waits::Answer,
            since: Some(asked),
            words: Some(question.into()),
        }),
        "the move the place waits on"
    );
    Ok(())
}

fn the_moves_waited_on(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    let work = World::identity("build", "one", "work", 1);
    ensure_eq!(world.status(&id)?.waiting_on, None, "a working place");
    world.terminal.kill(&work);
    world.settle(&id);
    let lost = world
        .status(&id)?
        .waiting_on
        .ok_or("a lost stage waits on nobody")?;
    ensure_eq!(
        (lost.act, lost.stage.as_deref(), lost.why),
        (Act::Retry, Some("work"), Waits::Lost),
        "a lost stage"
    );
    let id = world.begun("tend", Some(Deliverable::Commits), "two")?;
    ensure!(
        world.engine.stop(&id, true).refusal().is_none(),
        "the stop was refused"
    );
    let stopped = world
        .status(&id)?
        .waiting_on
        .ok_or("a stopped stage waits on nobody")?;
    ensure_eq!(
        (stopped.act, stopped.why),
        (Act::Retry, Waits::Stopped),
        "a stopped stage"
    );
    let id = world.begun("sweep", Some(Deliverable::Commits), "three")?;
    world.commit("sweep/three", &["swept.rs"]);
    world.settle(&id);
    let done = world
        .status(&id)?
        .waiting_on
        .ok_or("a delivered round waits on nobody")?;
    ensure_eq!(
        (done.act, done.stage, done.why),
        (Act::Done, None, Waits::Delivered),
        "a round delivered whose kind does not land"
    );
    Ok(())
}

fn places(answer: Answer) -> Result<Vec<PlaceStatus>, String> {
    match answer {
        Answer::Status { places, .. } => Ok(places),
        other => Err(format!("status answered {other:?}")),
    }
}

/// The calls that would start, tell or write.
fn acting_calls(world: &World) -> (usize, usize, usize, usize) {
    (
        world.terminal.log.count("open"),
        world.terminal.log.count("send"),
        world.store.log.count("write") + world.store.log.count("remove"),
        world.queue.log.count("submit"),
    )
}

fn s14_asked_three_times(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::TasksTicked), "one")?;
    world.tick("build/one", "one", 1, 2);
    let before = acting_calls(&world);
    let line = places(world.engine().status(Some(&id)))?;
    let session = places(world.engine().status(Some(&id)))?;
    let sidebar = places(world.engine().status(Some(&id)))?;
    ensure_eq!(line, session, "two surfaces' answers");
    ensure_eq!(session, sidebar, "two surfaces' answers");
    ensure_eq!(
        acting_calls(&world),
        before,
        "calls that start, tell or write"
    );
    Ok(())
}

fn a_change_ticked_between_asks(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::TasksTicked), "one")?;
    let work = World::identity("build", "one", "work", 1);
    world.terminal.set_state(&work, AgentState::Working);
    world.tick("build/one", "one", 1, 3);
    let before = acting_calls(&world);
    let first = world.status(&id)?;
    let head = world.tick("build/one", "one", 2, 3);
    let second = world.status(&id)?;
    let ticked = |status: &PlaceStatus| {
        status
            .tasks
            .as_ref()
            .map(|tasks| tasks.iter().filter(|task| task.ticked).count())
    };
    ensure_eq!(
        (ticked(&first), ticked(&second)),
        (Some(1), Some(2)),
        "the tasks ticked"
    );
    ensure_eq!(second.head, Some(head), "the head read");
    ensure_eq!(
        acting_calls(&world),
        before,
        "calls that start, tell or write"
    );
    Ok(())
}

fn every_fact_of_every_place(_: &()) -> Outcome {
    let world = World::new();
    let mut fix = launch("fix", "one", Some("x"));
    fix.mark = Some("team-a slot-2".into());
    launched(&world.engine.launch(&fix))?;
    launched(&world.engine.launch(&launch("change", "plan", Some("y"))))?;
    world.engine().settle(None);
    let fix_id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.agent.refuse_act(&code, "a push of fix/one");
    let all = places(world.engine.status(None))?;
    ensure_eq!(all.len(), 2, "places");
    let one = all
        .iter()
        .find(|place| place.place == fix_id)
        .ok_or("no fix")?;
    ensure_eq!(
        (
            one.repository.as_str(),
            one.kind.as_str(),
            one.version,
            one.declared
        ),
        (REPOSITORY, "fix", 1, Declared::Flume),
        "the repository and kind"
    );
    ensure_eq!(
        (
            one.name.as_str(),
            one.branch.as_str(),
            one.base.as_str(),
            one.lands
        ),
        ("one", "fix/one", "main", true),
        "the name, branch, base and landing"
    );
    ensure_eq!(one.mark.as_deref(), Some("team-a slot-2"), "the mark");
    ensure_eq!(one.stages[0].starts, 1, "the code stage's starts");
    let agent = one.agent.clone().ok_or("no agent")?;
    ensure_eq!(agent.identity, code.to_string(), "the agent's identity");
    ensure_eq!(
        agent.settings.settings.program.as_deref(),
        Some("agent"),
        "the agent's settings"
    );
    ensure_eq!(agent.state, AgentState::Waiting, "the agent's state");
    ensure_eq!(one.delivered, Some(false), "whether the deliverable exists");
    let submission = one.submission.clone().ok_or("no submission")?;
    ensure_eq!(submission.read, SubmissionRead::None, "the submission");
    ensure_eq!(submission.submits, 0, "submissions");
    ensure!(!one.refused_acts.is_empty(), "no refused act");
    let plan = all
        .iter()
        .find(|place| place.kind == "change")
        .ok_or("no change")?;
    ensure!(plan.tasks.is_some(), "no tasks for a change's place");
    Ok(())
}

fn a_place_gone(_: &()) -> Outcome {
    let world = World::new();
    let id = world.begun("build", Some(Deliverable::Commits), "one")?;
    world.engine.drop_place(&id, true);
    let reason = refused(&world.engine.status(Some(&id)))?;
    ensure!(
        reason.contains(&id.0),
        "the refusal names not the place: {reason}"
    );
    ensure!(
        places(world.engine.status(None))?.is_empty(),
        "a gone place is answered for"
    );
    Ok(())
}

fn a_stalled_place_with_a_pending_end(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Working);
    let (id, asked) =
        super::caller_waits::construct_asks(&world, "plan", &["ops", "madswan/planner"], None)?;
    world.host.advance(STALL_AFTER);
    let stalled_at = world.host.now();
    world.settle(&id);
    let status = world.status(&id)?;
    let waiting = status.waiting_on.ok_or("the place waits on nobody")?;
    ensure_eq!(
        (
            waiting.act,
            waiting.stage.as_deref(),
            waiting.why,
            waiting.since
        ),
        (Act::Retry, Some("construct"), Waits::Answer, Some(asked)),
        "the move and since when"
    );
    ensure!(status.stalled, "status does not say stalled");
    let stall = status
        .ends
        .iter()
        .find(|end| matches!(end.end.of, EndOf::Stall { .. }))
        .ok_or("no stall among the ends")?;
    ensure_eq!(
        stall
            .end
            .deliveries
            .iter()
            .map(|delivery| (delivery.recipient.as_str(), delivery.at))
            .collect::<Vec<_>>(),
        vec![("ops", stalled_at)],
        "the stall's deliveries"
    );
    ensure_eq!(
        stall.pending_for,
        vec!["madswan/planner".to_string()],
        "pending for"
    );
    Ok(())
}

fn status_mid_stage(_: &()) -> Outcome {
    use super::repeating_stages::{branch, code_begun, GROUPS};
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    world.tick_groups(&branch("plan"), "plan", GROUPS, 2);
    world.settle(&id);
    let status = world.status(&id)?;
    let code = status
        .stages
        .iter()
        .find(|stage| stage.name == "code")
        .ok_or("no code stage")?;
    let json = serde_json::to_value(code).map_err(|error| error.to_string())?;
    ensure_eq!(
        json["group"],
        serde_json::json!({"place": 2, "of": 3, "number": "2", "title": "Two"}),
        "the group in the JSON form"
    );
    ensure_eq!(code.label(), "code 2/3", "the stage in the person's form");
    ensure!(
        status
            .stages
            .iter()
            .filter(|stage| stage.name != "code")
            .all(|stage| stage.group.is_none()),
        "a stage that does not repeat names a group"
    );
    let clues = status.clues.ok_or("no clues")?;
    ensure!(
        clues.title.starts_with("code 2/3 · "),
        "the clue's title: {}",
        clues.title
    );
    let shown = world.terminal.shown();
    let (_, last) = shown.last().ok_or("nothing shown")?;
    ensure_eq!(last.stage.as_str(), "code 2/3", "the stage show was given");
    Ok(())
}

fn a_group_added_mid_stage(_: &()) -> Outcome {
    use super::repeating_stages::{branch, code_begun, GROUPS};
    let world = World::new();
    let id = code_begun(&world, "plan", &[])?;
    world.tick_groups(&branch("plan"), "plan", GROUPS, 2);
    world.settle(&id);
    let mut four = GROUPS.to_vec();
    four.push(("4", "Four", 1));
    world.tick_groups(&branch("plan"), "plan", &four, 3);
    let group = world.status(&id)?.stages[1]
        .group
        .clone()
        .ok_or("code names no group")?;
    ensure_eq!(
        (group.place, group.of),
        (2, 4),
        "the group's place and the count, read at the head"
    );
    Ok(())
}

fn a_stage_that_moved(_: &()) -> Outcome {
    let world = World::new();
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![account("work-1", None), account("work-2", None)],
    );
    world.host.set_host_pool(Some("work"));
    let id = fix_in(&world, None, "one")?;
    spent(&world, "work", "work-1", world.host.now() + 3600);
    world
        .terminal
        .kill(&World::identity("fix", "one", "code", 1));
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Lost),
        "code once its agent ended"
    );
    world.host.advance(30);
    let answer = world
        .engine()
        .retry(&id, Retry::Stage("code".into()), None, false);
    ensure!(answer.refusal().is_none(), "the retry: {answer:?}");
    let status = world.status(&id)?;
    ensure_eq!(
        status.agent.and_then(|agent| agent.account),
        Some(AccountHeld {
            pool: "work".into(),
            name: "work-2".into(),
            from: AccountFrom::Strategy
        }),
        "the agent's account"
    );
    let at = world.host.now();
    let moved = |on| MoveStatus {
        moved: AccountMove {
            on,
            from: "work-1".into(),
            to: "work-2".into(),
            why: MoveWhy::Spent,
            at,
        },
        cache: Cache::Lost,
    };
    ensure_eq!(
        status.moves,
        vec![
            moved(MoveOn::Stage {
                stage: "code".into()
            }),
            moved(MoveOn::Intent)
        ],
        "the place's moves"
    );
    Ok(())
}

fn a_pool_with_a_spent_account(_: &()) -> Outcome {
    let world = World::new();
    let mut work_1 = account("work-1", Some(4));
    work_1
        .env
        .insert("AGENT_TOKEN".into(), "a-token-of-the-account".into());
    pool(
        &world,
        "work",
        Strategy::FillFirst,
        vec![work_1, account("work-2", None)],
    );
    world.host.set_host_pool(Some("work"));
    let until = world.host.now() + 2 * 3600;
    spent(&world, "work", "work-2", until);
    fix_in(&world, None, "one")?;
    fix_in(&world, None, "two")?;
    let host = host(&world)?;
    ensure_eq!(
        host.pools,
        vec![PoolStatus {
            name: "work".into(),
            strategy: Strategy::FillFirst,
            accounts: vec![
                AccountStatus {
                    name: "work-1".into(),
                    program: "agent".into(),
                    running: 2,
                    bound: Some(4),
                    spent_until: None,
                },
                AccountStatus {
                    name: "work-2".into(),
                    program: "agent".into(),
                    running: 0,
                    bound: None,
                    spent_until: Some(until),
                },
            ],
        }],
        "the host's pools"
    );
    let answer = world.engine().status(None).to_json();
    ensure!(
        !answer.contains("a-token-of-the-account"),
        "status gives a value of an account's environment"
    );
    Ok(())
}

//! Status (43, 56 to 62, 69): every fact read where it lives when asked,
//! at one head per place, from one core, and nothing written, sent or
//! started.

use super::account_pools::{
    account, account_of, begun_fix, code_delivered, fill_first_at_homes, fix_in, fix_of, home,
    host, of_the_surfaces, on, pool, pool_at, reading_at, reads, spent, stops_at_limit,
    two_programs, window, UNREACHED,
};
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
            name: "status: a fake terminal scripted to lack one entry for flume's plugin gives it as the host's terminal setup, and status opens, sends to and closes nothing",
            cites: &["122", "57", "72"],
            run: the_terminals_setup,
        },
        Scenario {
            name: "status: a fix whose code agent works gives its line with every part in the JSON form, `fix--<name> · fix@1 · code · run 1 · working` in full",
            cites: &["56", "61"],
            run: a_line_in_status,
        },
        Scenario {
            name: "status: the place's answer dropping its line or one of the line's fields fails naming the answer and the field",
            cites: &["61"],
            run: a_line_field_dropped,
        },
        Scenario {
            name: "status: a terminal whose configuration binds no key to flume's places gives those words as the host's terminal setup when one place is asked of, and nothing is sent to any pane",
            cites: &["122", "72", "57"],
            run: a_popup_with_no_key,
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
            name: "status: a place whose code start 2 carries start 1's conversation under work-2 gives the agent's conversation from start 1, carried, and code's move from work-1 to work-2 carried, in the JSON and the person's form",
            cites: &["56", "146", "S78"],
            run: a_carried_agent,
        },
        Scenario {
            name: "status: a place whose code start 2 started afresh because work-2's CLAUDE_CONFIG_DIR does not reach start 1's conversation gives the agent's conversation from start 1, not carried, and why by the variable's name and no path, and the move not carried",
            cites: &["56", "S55", "179"],
            run: a_conversation_that_stayed,
        },
        Scenario {
            name: "status: a place whose carried start 2 ended before it said it started and whose start 3 started afresh lists start 2 carried and not begun with why, and start 3's agent from start 1, not carried, and why",
            cites: &["56", "S79", "146"],
            run: a_carried_start_that_did_not_begin,
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
            name: "status: a place whose code started after the intent moved for one program, and whose review started on another program's account from the affinity, gives the intent's move with its program and the review's account from the affinity for its program",
            cites: &["56", "147", "S80"],
            run: an_intent_that_moved_for_one_program,
        },
        Scenario {
            name: "status: the host's pools give each pool's strategy and each account's agents running, bound and spent time, and no value of an account's environment",
            cites: &["56", "144", "145"],
            run: a_pool_with_a_spent_account,
        },
        Scenario {
            name: "status: a most-left pool stating no threshold gives most-left and 90, an account's fresh reading with its window's use as a percentage and its reset, another's stale reading, and a third's missing file with why, in the JSON form, and no reading's path",
            cites: &["56", "184", "185"],
            run: a_pool_with_readings,
        },
        Scenario {
            name: "status: an account whose 7-day window reads 95% used gives what holds it past its threshold, 7-day until its reset, and that window past, in the JSON form",
            cites: &["56", "185"],
            run: an_account_past_its_threshold,
        },
        Scenario {
            name: "status: an intent whose affinity for agent moved off work-1, past its threshold in 5-hour until 17:00, to work-2 gives the move with why threshold, the window and its end, in the JSON form and the person's",
            cites: &["56", "147", "185"],
            run: an_intents_move_past_a_threshold,
        },
        Scenario {
            name: "status: a move a held state holds as an earlier release wrote it, spent with no window or end, is given as written, in the JSON form and the person's",
            cites: &["56", "61"],
            run: a_move_recorded_before,
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
        Scenario {
            name: "status: a code start begun in ask-each where its record named all is named with the mode asked and the mode begun, and no variable, in the JSON form",
            cites: &["56", "161", "162", "S67"],
            run: a_start_in_another_mode,
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

fn a_start_in_another_mode(_: &()) -> Outcome {
    let world = World::new();
    the_fakes_map(&world);
    world.agent.read_mode_back();
    let who = World::identity("fix", "one", "code", 1);
    world.agent.set_began_in(&who, "ask-each");
    world.agent.set_own_work(&who, Reading::Yes);
    let mut fix = launch("fix", "one", Some("mend it"));
    fix.every_stage = in_mode(Mode::Full);
    launched(&world.engine.launch(&fix))?;
    let id = World::id("fix", "one");
    world.settle(&id);
    let status = world.status(&id)?;
    ensure_eq!(
        status
            .unsettled
            .iter()
            .map(|read| (read.stage.as_str(), read.start, read.unsettled.clone()))
            .collect::<Vec<_>>(),
        vec![(
            "code",
            1,
            Unsettled {
                mode: Some(ModeBegan {
                    asked: "all".into(),
                    began: Some("ask-each".into()),
                }),
                ..Unsettled::default()
            }
        )],
        "the start named with both modes and no variable"
    );
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    let read = &json["unsettled"][0];
    ensure_eq!(
        (
            read["mode"].clone(),
            read["missing"].clone(),
            read["different"].clone()
        ),
        (
            serde_json::json!({"asked": "all", "began": "ask-each"}),
            serde_json::json!([]),
            serde_json::json!([])
        ),
        "the start in the JSON form"
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

fn a_line_in_status(_: &()) -> Outcome {
    let world = World::new();
    let id = super::account_pools::begun_fix(&world, "parser")?;
    let status = world.status(&id)?;
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    let line = json["line"].as_object().ok_or("no line in the JSON form")?;
    ensure_eq!(
        line.keys().map(String::as_str).collect::<Vec<_>>(),
        vec!["full", "kind", "name", "run", "short", "stage", "state", "tasks"],
        "the line's fields"
    );
    ensure_eq!(
        status.line.full.as_str(),
        "fix--parser · fix@1 · code · run 1 · working",
        "the line in full"
    );
    Ok(())
}

fn a_line_field_dropped(_: &()) -> Outcome {
    use super::answers::{full, missing};
    let (name, answer) = full().remove(0);
    for (field, path) in [
        ("line", "status.places[].line"),
        ("short", "status.places[].line.short"),
    ] {
        let mut json: serde_json::Value =
            serde_json::from_str(&answer.to_json()).map_err(|error| error.to_string())?;
        let place = &mut json["status"]["places"][0];
        let holder = if field == "line" {
            place
        } else {
            &mut place["line"]
        };
        holder
            .as_object_mut()
            .ok_or("no object")?
            .remove(field)
            .ok_or("no such field")?;
        let missed = missing(name, &json);
        ensure!(
            missed.contains(&format!("the answer status lacks its field {path}")),
            "the fields missed: {missed:?}"
        );
    }
    Ok(())
}

fn a_popup_with_no_key(_: &()) -> Outcome {
    use crate::fake_harness::SETUP;
    let world = World::new();
    let id = super::account_pools::begun_fix(&world, "parser")?;
    world.terminal.set_setup(&[SETUP]);
    let sends = |world: &World| {
        (
            world.terminal.log.count("send"),
            world.terminal.log.count("send_to"),
            world.terminal.every_notice().len(),
        )
    };
    let before = sends(&world);
    let Answer::Status { host, .. } = world.engine().status(Some(&id)) else {
        return Err("status refused".into());
    };
    ensure_eq!(
        host.terminal_setup,
        vec![SETUP.to_string()],
        "the host's terminal setup"
    );
    ensure_eq!(sends(&world), before, "what was sent to a pane");
    Ok(())
}

fn the_terminals_setup(_: &()) -> Outcome {
    use crate::fake_harness::SETUP;
    let world = World::new();
    world.begun("build", Some(Deliverable::Commits), "one")?;
    world.terminal.set_setup(&[SETUP]);
    let calls = |world: &World| {
        ["open", "send", "close"].map(|operation| world.terminal.log.count(operation))
    };
    let before = calls(&world);
    let Answer::Status { host, .. } = world.engine().status(None) else {
        return Err("status refused".into());
    };
    ensure_eq!(
        host.terminal_setup,
        vec![SETUP.to_string()],
        "the host's terminal setup"
    );
    ensure_eq!(calls(&world), before, "the opens, sends and closes");
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
            carried: None,
            window: None,
            until: None,
        },
        cache: Cache::Lost,
    };
    ensure_eq!(
        status.moves,
        vec![
            moved(MoveOn::Stage {
                stage: "code".into()
            }),
            moved(MoveOn::Intent {
                program: Some("agent".into())
            })
        ],
        "the place's moves"
    );
    Ok(())
}

fn an_intent_that_moved_for_one_program(_: &()) -> Outcome {
    let world = World::new();
    two_programs(&world, None);
    let first = of_the_surfaces(&world, "two", "a", None)?;
    code_delivered(&world, &first, "a")?;
    spent(&world, "work", "agent-1", world.host.now() + 3600);
    let id = of_the_surfaces(&world, "two", "b", None)?;
    code_delivered(&world, &id, "b")?;
    let status = world.status(&id)?;
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    ensure_eq!(
        json["moves"],
        serde_json::json!([{
            "on": "intent",
            "program": "agent",
            "from": "agent-1",
            "to": "agent-2",
            "why": "spent",
            "at": world.host.now(),
            "cache": "lost",
        }]),
        "the place's moves in the JSON form"
    );
    ensure_eq!(
        (
            json["agent"]["stage"].as_str(),
            json["agent"]["program"].as_str(),
            json["agent"]["account"]["name"].as_str(),
            json["agent"]["account"]["from"].as_str(),
        ),
        (
            Some("review"),
            Some("other"),
            Some("other-1"),
            Some("affinity")
        ),
        "the review's agent and its account in the JSON form"
    );
    let words = status.moves[0].words();
    ensure!(
        words.starts_with("the intent moved for agent from agent-1 to agent-2"),
        "the move's words: {words}"
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
            threshold: 90,
            accounts: vec![
                AccountStatus {
                    name: "work-1".into(),
                    program: "agent".into(),
                    running: 2,
                    bound: Some(4),
                    spent_until: None,
                    spent_unread: None,
                    reading: None,
                    past: None,
                },
                AccountStatus {
                    name: "work-2".into(),
                    program: "agent".into(),
                    running: 0,
                    bound: None,
                    spent_until: Some(until),
                    spent_unread: None,
                    reading: None,
                    past: None,
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

fn a_pool_with_readings(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    let seventeen = now + 2 * 3600;
    pool_at(
        &world,
        "work",
        Strategy::MostLeft,
        None,
        ["work-1", "work-2", "work-3"]
            .into_iter()
            .map(|name| reading_at(account(name, None)))
            .collect(),
    );
    reads(
        &world,
        "work-1",
        now - 120,
        vec![window("5-hour", 4_200, Some(seventeen))],
    );
    reads(
        &world,
        "work-2",
        now - 20 * 60,
        vec![window("5-hour", 1_250, Some(seventeen))],
    );
    let json = serde_json::to_value(host(&world)?).map_err(|error| error.to_string())?;
    let unbound = |name: &str, reading| {
        serde_json::json!({
            "name": name,
            "program": "agent",
            "running": 0,
            "bound": null,
            "spent_until": null,
            "reading": reading,
        })
    };
    ensure_eq!(
        json["pools"],
        serde_json::json!([{
            "name": "work",
            "strategy": "most-left",
            "threshold": 90,
            "accounts": [
                unbound("work-1", serde_json::json!({
                    "read_at": now - 120,
                    "fresh": true,
                    "windows": [{"name": "5-hour", "used": 42, "resets_at": seventeen, "past": false}],
                })),
                unbound("work-2", serde_json::json!({
                    "read_at": now - 20 * 60,
                    "fresh": false,
                    "windows": [{"name": "5-hour", "used": 12.5, "resets_at": seventeen, "past": false}],
                })),
                unbound("work-3", serde_json::json!({"unread": "the file is missing"})),
            ],
        }]),
        "the host's pools in the JSON form"
    );
    let answer = world.engine().status(None).to_json();
    ensure!(
        !answer.contains(".local/state/usage"),
        "status gives a reading's path: {answer}"
    );
    Ok(())
}

fn an_account_past_its_threshold(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    let monday = now + 3 * 24 * 3600;
    pool_at(
        &world,
        "work",
        Strategy::FillFirst,
        None,
        vec![reading_at(account("work-1", None)), account("work-2", None)],
    );
    reads(
        &world,
        "work-1",
        now,
        vec![
            window("5-hour", 3_000, Some(now + 3600)),
            window("7-day", 9_500, Some(monday)),
        ],
    );
    let json = serde_json::to_value(host(&world)?).map_err(|error| error.to_string())?;
    let accounts = &json["pools"][0]["accounts"];
    ensure_eq!(
        accounts[0]["past"],
        serde_json::json!({"window": "7-day", "until": monday}),
        "what holds work-1 past its threshold"
    );
    ensure_eq!(
        accounts[0]["reading"]["windows"],
        serde_json::json!([
            {"name": "5-hour", "used": 30, "resets_at": now + 3600, "past": false},
            {"name": "7-day", "used": 95, "resets_at": monday, "past": true},
        ]),
        "work-1's windows"
    );
    ensure_eq!(
        (accounts[1].get("past"), accounts[1].get("reading")),
        (None, None),
        "work-2, which names no reading"
    );
    Ok(())
}

fn an_intents_move_past_a_threshold(_: &()) -> Outcome {
    let world = World::new();
    let now = world.host.now();
    let seventeen = now + 2 * 3600;
    pool_at(
        &world,
        "work",
        Strategy::FillFirst,
        None,
        vec![reading_at(account("work-1", None)), account("work-2", None)],
    );
    let first = fix_of(&world, None, "a", Some("the-surfaces"))?;
    ensure_eq!(
        account_of(&world, &first, "code"),
        on("work", "work-1"),
        "the intent's first place's account"
    );
    reads(
        &world,
        "work-1",
        now,
        vec![window("5-hour", 9_200, Some(seventeen))],
    );
    let id = fix_of(&world, None, "b", Some("the-surfaces"))?;
    let status = world.status(&id)?;
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    ensure_eq!(
        json["moves"],
        serde_json::json!([{
            "on": "intent",
            "program": "agent",
            "from": "work-1",
            "to": "work-2",
            "why": "threshold",
            "at": now,
            "window": "5-hour",
            "until": seventeen,
            "cache": "lost",
        }]),
        "the place's moves in the JSON form"
    );
    let words = status.moves[0].words();
    ensure!(
        words.starts_with(&format!(
            "the intent moved for agent from work-1 to work-2 at {now}, work-1 past its threshold in 5-hour until {seventeen},"
        )),
        "the move in the person's form: {words}"
    );
    Ok(())
}

fn a_move_recorded_before(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_in(&world, None, "one")?;
    let before = serde_json::json!({
        "on": "stage",
        "stage": "code",
        "from": "work-1",
        "to": "work-2",
        "why": "spent",
        "at": world.host.now(),
    });
    let mut held = world.held(&id).ok_or("no held state")?;
    held.moves
        .push(serde_json::from_value(before.clone()).map_err(|error| error.to_string())?);
    world.store.put(&id, held);
    let status = world.status(&id)?;
    ensure_eq!(
        status
            .moves
            .iter()
            .map(|moved| (moved.moved.window.clone(), moved.moved.until))
            .collect::<Vec<_>>(),
        vec![(None, None)],
        "the move's window and end"
    );
    let words = status.moves[0].words();
    ensure!(
        words.contains("work-1 spent,") && !words.contains("threshold"),
        "the move in the person's form: {words}"
    );
    let mut given = before;
    given["cache"] = serde_json::json!("lost");
    ensure_eq!(
        serde_json::to_value(&status).map_err(|error| error.to_string())?["moves"],
        serde_json::json!([given]),
        "the move in the JSON form"
    );
    Ok(())
}

/// The fix `one`'s code under work-1 stopped at its usage limit and
/// started again on work-2, where the fake agent answers `carry`; its
/// status and that status in the JSON form.
fn moved_to_work_2(
    world: &World,
    carry: Carry,
) -> Result<(PlaceStatus, serde_json::Value), String> {
    fill_first_at_homes(world);
    world.agent.carry_to(&home("work-2"), carry);
    let id = begun_fix(world, "one")?;
    stops_at_limit(world, "one", 1, Some(world.host.now() + 3600));
    world.settle(&id);
    let status = world.status(&id)?;
    let json = serde_json::to_value(&status).map_err(|error| error.to_string())?;
    Ok((status, json))
}

/// The move of code in a place's status.
fn code_moved(status: &PlaceStatus) -> Result<&MoveStatus, String> {
    status
        .moves
        .iter()
        .find(|moved| {
            moved.moved.on
                == MoveOn::Stage {
                    stage: "code".into(),
                }
        })
        .ok_or_else(|| "status gives no move of code".to_string())
}

fn a_carried_agent(_: &()) -> Outcome {
    let world = World::new();
    let (status, json) = moved_to_work_2(
        &world,
        Carry::Yes {
            session: "s-1".into(),
        },
    )?;
    ensure_eq!(
        json["agent"]["conversation"],
        serde_json::json!({"from": 1, "carried": true}),
        "the agent's conversation in the JSON form"
    );
    ensure_eq!(
        json["moves"][0]["carried"],
        serde_json::json!(true),
        "code's move in the JSON form"
    );
    let words = code_moved(&status)?.words();
    ensure!(
        words.starts_with("code moved from work-1 to work-2")
            && words.contains("its conversation with it"),
        "code's move in the person's form: {words}"
    );
    ensure!(
        !json.to_string().contains("/homes"),
        "status gives a configuration home's path"
    );
    Ok(())
}

fn a_conversation_that_stayed(_: &()) -> Outcome {
    let world = World::new();
    let (status, json) = moved_to_work_2(
        &world,
        Carry::No {
            why: UNREACHED.into(),
        },
    )?;
    ensure_eq!(
        json["agent"]["conversation"],
        serde_json::json!({"from": 1, "carried": false, "why": UNREACHED}),
        "the agent's conversation in the JSON form"
    );
    ensure!(
        UNREACHED.contains("CLAUDE_CONFIG_DIR") && !UNREACHED.contains('/'),
        "the why names no variable or names a path"
    );
    ensure_eq!(
        json["moves"][0]["carried"],
        serde_json::json!(false),
        "code's move in the JSON form"
    );
    let words = code_moved(&status)?.words();
    ensure!(
        words.contains("its conversation stayed behind"),
        "code's move in the person's form: {words}"
    );
    ensure!(
        !json.to_string().contains("/homes"),
        "status gives a configuration home's path"
    );
    Ok(())
}

fn a_carried_start_that_did_not_begin(_: &()) -> Outcome {
    let world = World::new();
    world.agent.carried_starts_end();
    let (status, json) = moved_to_work_2(
        &world,
        Carry::Yes {
            session: "s-1".into(),
        },
    )?;
    let ended = "the start that carried its conversation ended before it said it started";
    ensure_eq!(
        status
            .uncarried
            .iter()
            .map(|read| (read.stage.as_str(), read.start, read.words.as_str()))
            .collect::<Vec<_>>(),
        vec![("code", 2, ended)],
        "the place's carried starts that did not begin"
    );
    ensure_eq!(
        (
            json["agent"]["identity"].clone(),
            json["agent"]["conversation"].clone()
        ),
        (
            serde_json::json!(World::identity("fix", "one", "code", 3).to_string()),
            serde_json::json!({"from": 1, "carried": false, "why": ended})
        ),
        "the agent of start 3 and its conversation in the JSON form"
    );
    ensure_eq!(
        json["uncarried"][0]["words"],
        serde_json::json!(ended),
        "the place's carried start that did not begin in the JSON form"
    );
    Ok(())
}

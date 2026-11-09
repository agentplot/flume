//! Each end delivered once to each recipient (7, 38, 43 to 46, 95, 133;
//! core.11, core.25): the settle that reads a stage's end, a stage lost,
//! a submission's outcome records it on the place and sends it as a
//! notice into each recipient's session that asks no question, recorded
//! with to whom and when; a recipient asking, gone or under the printed
//! command leaves it pending; and a tidied place's document stays while
//! an end of it is pending, until it is delivered or the place dropped,
//! a launch of the same name going on beside it (3, 62). A wait by place
//! answers an end pending for its recipient and delivers it, reads one
//! already delivered, and otherwise follows the host's state under its
//! timeout, settling nothing while a caller keeps watch over the host
//! and every place before each reading while none does, whatever
//! terminal the host binds (119, 122, 208); the inbox collects
//! every end pending for a recipient, each delivered once (75). An end
//! for a role's address the host's configuration gives a standing agent
//! goes into the one agent of its name the terminal finds in the session
//! of the entry's stream at each settle, none, two or a stream the host
//! does not declare leaving it pending, and nothing of where the agent
//! was is kept (132, 143, 190, C.7; core.28).

use super::caller_waits::{asks_in_east, construct_asks};
use super::carried_conversations::{talk, talking};
use super::role_waking::answerer_behind;
use super::settling::{deliver_fix, write_change};
use super::stage_runs::east_and_west;
use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::{Act, Answer, EndStatus, WaitFor, WaitingPlace, Waits};
use flume_fakes::Plan;
use flume_seams::*;
use std::sync::{Arc, Mutex};

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "end delivery: a stage's end is delivered as one notice to its recipient and recorded with to whom and when",
            cites: &["43", "46", "95", "7"],
            run: a_delivery_and_its_record,
        },
        Scenario {
            name: "end delivery: a place settled again sends no second notice",
            cites: &["43", "74", "46"],
            run: settled_twice,
        },
        Scenario {
            name: "end delivery: an agent killed with nothing delivered is a lost end, delivered once with what flume read",
            cites: &["43", "45", "S28"],
            run: an_agent_killed,
        },
        Scenario {
            name: "end delivery: a failure and a later landing are each an end delivered once, with the merge queue's words and the commit",
            cites: &["43", "133", "44"],
            run: a_landing_and_a_failure,
        },
        Scenario {
            name: "end delivery: no notice is sent into a recipient asking a question, and the first settle that reads it not asking delivers it",
            cites: &["43", "95", "38"],
            run: a_recipient_asking,
        },
        Scenario {
            name: "end delivery: a recipient no session holds leaves the end pending, and the settle goes on",
            cites: &["43", "95", "46"],
            run: a_recipient_gone,
        },
        Scenario {
            name: "end delivery: a recipient scripted away and back is delivered to once it is back",
            cites: &["43", "95"],
            run: a_recipient_away_and_back,
        },
        Scenario {
            name: "end delivery: under a terminal that cannot send, nothing is sent and status gives every end pending",
            cites: &["43", "38", "119"],
            run: under_the_printed_command,
        },
        Scenario {
            name: "end delivery: a place landing with its recipient away is tidied, its document kept pending until its recipient confirms its ends, then removed and their deliveries let go of, its name kept known (weir 106, 107)",
            cites: &["43", "62", "66", "67"],
            run: a_landing_with_its_recipient_away,
        },
        Scenario {
            name: "end delivery: a tidied place dropped while an end is pending for boomdrive is refused naming boomdrive and flume confirm, its document and pending ends staying (weir I9)",
            cites: &["43", "62", "52"],
            run: dropped_while_pending,
        },
        Scenario {
            name: "end delivery: a place launched again while a tidied document of its name stays is made, the document's ends pending as they were",
            cites: &["3", "43", "62"],
            run: launched_again,
        },
        Scenario {
            name: "end delivery: a place landing with its recipient away is tidied, and once the recipient collects by the inbox the next settle removes its document",
            cites: &["43", "62", "66"],
            run: collected_after_the_tidy,
        },
        Scenario {
            name: "end delivery: a place launched for ops and boomdrive, which no pane holds, lands, ops has its notices and confirms them, and status gives the tidied place pending for boomdrive",
            cites: &["62"],
            run: kept_for_a_reader,
        },
        Scenario {
            name: "end delivery: a landing of a fix launched for ops and boomdrive, which no pane holds, is one notice to ops and none to boomdrive, its document kept with code's end and the landing pending for ops, handed to it by notice, and for boomdrive, handed to none, until ops confirms them and boomdrive's inbox naming the place confirms them",
            cites: &["43", "62"],
            run: a_landing_for_a_reader,
        },
        Scenario {
            name: "end delivery: a drop of a fix launched for east/planner, which acme declares with a run, and boomdrive records over/dropped pending for boomdrive alone, no notice sent and no run woken, its document kept until boomdrive's inbox collects it",
            cites: &["43", "52", "62"],
            run: a_drop_for_a_role_and_a_reader,
        },
        Scenario {
            name: "end delivery: a drop of a fix launched for east/planner alone records over/dropped pending for no one, sends no notice, wakes no run, and its tidy keeps no document",
            cites: &["43", "52", "62", "132"],
            run: a_drop_for_a_role_alone,
        },
        Scenario {
            name: "end delivery: a drop of a fix launched for ops, waiting in its pane, sends ops no notice, its over/dropped pending for ops and its document kept until ops's inbox collects it",
            cites: &["43", "52", "62"],
            run: a_drop_for_a_session_in_a_pane,
        },
        Scenario {
            name: "end delivery: a done of a place whose kind does not land, launched for boomdrive, records over/done pending for boomdrive, its document kept until collected",
            cites: &["43", "114"],
            run: a_done,
        },
        Scenario {
            name: "end delivery: a recipient restarted while waiting on a submission waits again and is answered at once with the landing, handed to it by the wait and still pending for it",
            cites: &["43", "77", "133"],
            run: a_restarted_recipient_waits_again,
        },
        Scenario {
            name: "end delivery: a wait run again with --confirm answers with the landing, its hand-over and the recipient's confirmation recorded together (weir 103)",
            cites: &["43", "133"],
            run: a_wait_that_confirms,
        },
        Scenario {
            name: "end delivery: a wait that finds nothing pending until its timeout exits still waiting and records nothing delivered",
            cites: &["43"],
            run: the_timeout,
        },
        Scenario {
            name: "end delivery: an end its recipient confirmed is answered to a wait as a read, handing nothing over again",
            cites: &["43", "74"],
            run: a_wait_reads_an_end_delivered,
        },
        Scenario {
            name: "end delivery: while a caller keeps watch a wait calls no seam but the store, and the place moves on only by the caller's settles",
            cites: &["43", "119", "120", "208"],
            run: a_wait_under_a_watch,
        },
        Scenario {
            name: "end delivery: while no caller keeps watch, under any terminal, a wait settles every place before each reading",
            cites: &["43", "119", "122", "208"],
            run: a_wait_attended,
        },
        Scenario {
            name: "end delivery: a wait on what the place has no end of and never will is refused naming why",
            cites: &["43", "2"],
            run: a_wait_refused,
        },
        Scenario {
            name: "end delivery: a restarted recipient collects by the inbox every end pending for it, each handed to it by the inbox and still pending, a second inbox giving them again, and status or another's inbox marks nothing",
            cites: &["43", "57"],
            run: collected_after_a_restart,
        },
        Scenario {
            name: "end delivery: an inbox with --confirm gives every end pending for its recipient, each confirmed by this call, and the next inbox gives nothing",
            cites: &["43", "57"],
            run: collected_and_confirmed,
        },
        Scenario {
            name: "end delivery: of two fixes launched for ops and boomdrive, which no pane holds, landed and tidied, boomdrive's inbox naming the first collects its ends alone, the second's ends and document staying until an inbox names it by its document",
            cites: &["43", "62"],
            run: an_inbox_naming_a_place,
        },
        Scenario {
            name: "end delivery: an inbox naming a place under which no document stands is refused naming it, and marks nothing delivered",
            cites: &["43"],
            run: a_name_under_which_nothing_stands,
        },
        Scenario {
            name: "end delivery: a question collected by the inbox, then a restart: the next inbox collects nothing and gives the place waiting on a retry of construct, since the stop-short, with its words and key",
            cites: &["43", "130", "S94"],
            run: a_question_collected_then_a_restart,
        },
        Scenario {
            name: "end delivery: the inbox of the role a stop-short named gives the place waiting on it, naming the role as the stop-short's addressee",
            cites: &["43", "41", "193"],
            run: the_inbox_of_the_role_asked,
        },
        Scenario {
            name: "end delivery: the inbox of the answerer a stall was given to gives the place waiting, though the answerer is not among its recipients",
            cites: &["43", "130", "193"],
            run: the_answerers_waiting_places,
        },
        Scenario {
            name: "end delivery: the inbox gives a held question with the hold's words, since when and its bound, and the next inbox gives it the same",
            cites: &["43", "130"],
            run: a_held_question_in_the_inbox,
        },
        Scenario {
            name: "end delivery: a stop-short's notice is one line naming the place, its brief's first line cut at 60 characters, construct needing an answer, its key and flume inbox",
            cites: &["43", "S94", "95"],
            run: a_stop_shorts_notice,
        },
        Scenario {
            name: "end delivery: a delivery's notice names its key and flume status",
            cites: &["43", "95"],
            run: a_deliverys_notice,
        },
        Scenario {
            name: "end delivery: a session's own inbox collects as the session the terminal names, never as the address its environment names, and gives none where no terminal names one",
            cites: &["43", "7", "190"],
            run: a_sessions_own_inbox,
        },
        Scenario {
            name: "end delivery: a restarted session's words give the place, the question, its key, its agent kept open and the three ways on, and nothing where nothing waits",
            cites: &["43", "130", "198", "S94"],
            run: a_restarted_sessions_words,
        },
        Scenario {
            name: "end delivery: two inboxes at once deliver each end once",
            cites: &["43", "75"],
            run: two_inboxes_at_once,
        },
        Scenario {
            name: "end delivery: an end for a standing address goes as one notice into the agent of the entry's name waiting in the entry's session, recorded delivered to the address, and no place is made",
            cites: &["43", "132"],
            run: a_standing_agent_waiting,
        },
        Scenario {
            name: "end delivery: no notice goes into a standing agent asking, and the first settle that reads it waiting delivers the end once",
            cites: &["43", "95"],
            run: a_standing_agent_asking,
        },
        Scenario {
            name: "end delivery: no agent of the standing entry's name in its session leaves the end pending, nothing claimed, and the settle goes on",
            cites: &["132", "43"],
            run: no_agent_of_the_name,
        },
        Scenario {
            name: "end delivery: two agents of the standing entry's name in its session each take no notice, and the end stays pending",
            cites: &["132"],
            run: two_agents_of_the_name,
        },
        Scenario {
            name: "end delivery: a standing agent removed and another added under its name takes the next notice, and the removed one no second",
            cites: &["132"],
            run: an_agent_revived_under_its_name,
        },
        Scenario {
            name: "end delivery: a standing entry naming no session is sought in the session its address's stream names, in that stream's terminal",
            cites: &["143", "190", "132", "193"],
            run: the_session_from_the_addresss_stream,
        },
        Scenario {
            name: "end delivery: a standing entry of an address of the default stream naming no session is sought in the terminal's own part where the default stream names no session",
            cites: &["107", "143", "132", "193"],
            run: the_default_streams_own_session,
        },
        Scenario {
            name: "end delivery: a standing entry whose address names a stream the host does not declare asks no terminal, and the end stays pending",
            cites: &["143", "132", "193"],
            run: a_stream_not_declared,
        },
        Scenario {
            name: "end delivery: the inbox collects an end pending for a standing address whose agent is away, handed to the address by the inbox and pending for it, and no later settle sends a notice of it",
            cites: &["43", "132"],
            run: the_inbox_collects_for_a_standing_address,
        },
        Scenario {
            name: "end delivery: a talk read lost while the agent the terminal brought back bare stands in its pane ends with left_in_pane, its one notice to ops saying flume left the agent in its pane",
            cites: &["S113", "43", "77"],
            run: a_lost_talk_with_a_bare_agent,
        },
        Scenario {
            name: "end delivery: a talk whose agent was killed ends with no agent left, its notice saying nothing of one",
            cites: &["43", "45"],
            run: a_lost_talk_with_no_agent,
        },
        Scenario {
            name: "end delivery: a code end handed to ops by notice, ops cleared before it confirms it, stays pending for ops, no second notice going at later settles, and ops's next inbox gives it (S122)",
            cites: &["43", "74"],
            run: handed_and_stopped_before_acting,
        },
        Scenario {
            name: "end delivery: a notice answered sent whose hand-over the host's deliveries could not record is sent no second time, and the next settle records its hand-over at the claim's time",
            cites: &["43", "75"],
            run: a_notice_claimed_and_not_recorded,
        },
        Scenario {
            name: "end delivery: ops confirming a code end's delivery key twice is confirmed at the first time and delivered, the second writing nothing and giving the first time (weir 102)",
            cites: &["43", "74"],
            run: confirmed_twice,
        },
        Scenario {
            name: "end delivery: ops confirming a key of boomdrive's end is refused naming the key and reader, and a key no end is under refused naming key, nothing written (weir 102)",
            cites: &["43"],
            run: a_key_of_another_recipients,
        },
        Scenario {
            name: "end delivery: a fix for the misspelt bomdrive lands and nothing collects its ends; a week on, status gives each expired for bomdrive, a name never confirmed under, the host lists them, the tidied document stays and its drop is refused naming bomdrive (S123; weir 104, 106)",
            cites: &["43", "56", "62"],
            run: a_misspelt_recipient,
        },
        Scenario {
            name: "end delivery: the operator confirming as bomdrive the expired ends' delivery keys delivers them, bomdrive reads known, and the next settle removes the document (weir 104, 106, 107)",
            cites: &["43", "56", "62"],
            run: confirmed_once_expired,
        },
        Scenario {
            name: "end delivery: a fix launched, landed and tidied with nothing pending, then launched again, keys its two code ends' deliveries by two tokens, the second's recorded owed to its recipient (C.7; weir 101)",
            cites: &["43", "3"],
            run: a_place_launched_twice,
        },
        Scenario {
            name: "end delivery: a live place's document written with no token is given one by its settle's write, and its ends' deliveries are keyed by it (C.7; weir 101)",
            cites: &["43", "75"],
            run: a_place_written_before_the_token,
        },
    ]
}

/// A fix named `name` launched for the recipient `ops` and begun.
fn fix_for_ops(world: &World, name: &str) -> Result<PlaceId, String> {
    let mut launch = launch("fix", name, Some("mend it"));
    launch.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&launch))?;
    let id = World::id("fix", name);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    Ok(id)
}

/// The place's ends as status gives them.
fn ends(world: &World, id: &PlaceId) -> Result<Vec<EndStatus>, String> {
    Ok(world.status(id)?.ends)
}

/// The notice a fix `name`'s end is sent as: its brief, what ended, its
/// key and the command that reads it, `flume inbox` where the place waits
/// on its caller's move and `flume status` otherwise (43; design 9).
fn notice(name: &str, what: &str, key: &str, read: &str) -> String {
    format!("flume: fix--{name} (mend it) {what} [{key}]; read: {read}")
}

/// The notice of a fix `name`'s code delivered at its first start, which
/// waits on no move of its caller's.
fn delivered(name: &str) -> String {
    notice(
        name,
        "code delivered",
        "stage/code/1/delivered",
        &format!("flume status fix--{name}"),
    )
}

/// The tidied documents standing on the host.
fn tidied(world: &World) -> Vec<HeldState> {
    world
        .store
        .read_every()
        .into_iter()
        .map(|read| read.state)
        .filter(|held| matches!(held.phase, Phase::Tidied { .. }))
        .collect()
}

fn a_delivery_and_its_record(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![world.confirming(&delivered("one"))],
        "the notices to ops"
    );
    let ends = ends(&world, &id)?;
    let end = ends.first().ok_or("no end recorded")?;
    ensure!(
        matches!(&end.end.of, EndOf::Stage { stage, state: StageState::Delivered, end, .. }
            if stage == "code" && end.head.is_some()),
        "the end: {:?}",
        end.end.of
    );
    ensure_eq!(
        end.end
            .deliveries
            .iter()
            .map(|delivery| (delivery.recipient.as_str(), delivery.at, delivery.by))
            .collect::<Vec<_>>(),
        vec![("ops", world.host.now(), DeliveredBy::Notice)],
        "the delivery recorded"
    );
    ensure_eq!(
        end.pending_for,
        vec!["ops".to_string()],
        "pending for, handed and not confirmed"
    );
    Ok(())
}

fn settled_twice(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    world.settle(&id);
    world.engine().settle(None);
    ensure_eq!(world.terminal.notices("ops").len(), 1, "notices to ops");
    ensure_eq!(ends(&world, &id)?.len(), 1, "ends recorded");
    Ok(())
}

fn an_agent_killed(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Working);
    let id = fix_for_ops(&world, "one")?;
    let code = World::identity("fix", "one", "code", 1);
    world.agent.set_last_words(&code, "I was cut off");
    world.terminal.kill(&code);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![world.confirming(&notice(
            "one",
            "code lost",
            "stage/code/1/lost",
            "flume inbox"
        ))],
        "the notices to ops"
    );
    let ends = ends(&world, &id)?;
    ensure_eq!(ends.len(), 1, "ends recorded");
    ensure!(
        matches!(&ends[0].end.of, EndOf::Stage { state: StageState::Lost, end, .. }
            if end.last_words.as_deref() == Some("I was cut off")),
        "the lost end carries no reading: {:?}",
        ends[0].end.of
    );
    Ok(())
}

fn a_landing_and_a_failure(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world
        .queue
        .plan_submits(vec![Plan::Fail("test x failed".into()), Plan::Land]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    let failed = ends(&world, &id)?
        .into_iter()
        .find(|end| matches!(end.end.of, EndOf::Submission { .. }))
        .ok_or("no submission end")?;
    ensure!(
        matches!(&failed.end.of, EndOf::Submission { submit: 1, verdict: Verdict::Failed { words } }
            if words.contains("test x failed")),
        "the failure: {:?}",
        failed.end.of
    );
    ensure_eq!(
        world.stage_state(&id, "code").map(StageState::running),
        Some(true),
        "code running again"
    );
    world.settle(&id);
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place stands");
    let landed = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    let notices = world.terminal.notices("ops");
    for what in [
        "code delivered".to_string(),
        "submission failed".to_string(),
        format!("submission landed {landed}"),
    ] {
        let sent = notices
            .iter()
            .filter(|text| text.starts_with(&format!("flume: fix--one (mend it) {what} [")))
            .count();
        let once = if what == "code delivered" { 2 } else { 1 };
        ensure_eq!(sent, once, "notices of {what}");
    }
    ensure_eq!(tidied(&world).len(), 1, "documents kept pending for ops");
    world.confirm_every("ops");
    world.engine().settle(None);
    ensure!(
        tidied(&world).is_empty(),
        "a document kept with nothing pending"
    );
    Ok(())
}

fn a_recipient_asking(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Asking);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(
        world.terminal.notices("ops").is_empty(),
        "a notice sent into a question"
    );
    ensure_eq!(
        ends(&world, &id)?
            .first()
            .map(|end| end.pending_for.clone()),
        Some(vec!["ops".to_string()]),
        "pending"
    );
    world.terminal.set_session_state("ops", AgentState::Unknown);
    world.settle(&id);
    ensure!(
        world.terminal.notices("ops").is_empty(),
        "a notice sent into a session read unknown"
    );
    world.terminal.set_session_state("ops", AgentState::Working);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![world.confirming(&delivered("one"))],
        "the notices once ops asks nothing"
    );
    Ok(())
}

fn a_recipient_gone(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent to nobody"
    );
    ensure_eq!(world.queue.log.count("submit"), 1, "the place submitted");
    ensure_eq!(
        ends(&world, &id)?
            .first()
            .map(|end| end.pending_for.clone()),
        Some(vec!["ops".to_string()]),
        "pending"
    );
    Ok(())
}

fn a_recipient_away_and_back(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    world.terminal.remove_session("ops");
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(
        world.terminal.notices("ops").is_empty(),
        "a notice to ops while away"
    );
    world.terminal.add_session("ops", AgentState::Waiting);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![world.confirming(&delivered("one"))],
        "the notices once ops is back"
    );
    Ok(())
}

fn under_the_printed_command(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.cannot_send();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    world.settle(&id);
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent: {:?}",
        world.terminal.every_notice()
    );
    let ends = ends(&world, &id)?;
    ensure!(!ends.is_empty(), "no end recorded");
    for end in &ends {
        ensure_eq!(end.pending_for, vec!["ops".to_string()], "pending");
        ensure!(end.end.deliveries.is_empty(), "a delivery recorded");
    }
    Ok(())
}

/// A fix for `ops` landed while no session is named `ops`, tidied.
fn landed_away(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = fix_for_ops(world, name)?;
    deliver_fix(world, &id);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place's own document");
    ensure!(!world.place.exists(&id.0), "the landed place's worktree");
    Ok(id)
}

fn a_landing_with_its_recipient_away(_: &()) -> Outcome {
    let world = World::new();
    let id = landed_away(&world, "one")?;
    ensure!(
        world.git.branch_head(REPOSITORY, "fix/one").is_none(),
        "the landed place's branch"
    );
    let kept = tidied(&world);
    ensure_eq!(kept.len(), 1, "tidied documents");
    let kept_id = kept[0].place.id.clone();
    let status = world.status(&id)?;
    ensure!(
        matches!(&status.phase, Phase::Tidied { pending_for, .. } if pending_for == &vec!["ops".to_string()]),
        "the tidied place's phase: {:?}",
        status.phase
    );
    ensure!(
        status
            .ends
            .iter()
            .all(|end| end.pending_for == vec!["ops".to_string()]),
        "ends: {:?}",
        status.ends
    );
    world.terminal.add_session("ops", AgentState::Waiting);
    world.engine().settle(None);
    ensure_eq!(
        world.terminal.notices("ops").len(),
        2,
        "the notices once ops is back"
    );
    ensure!(
        world.held(&kept_id).is_some(),
        "the document gone while its ends are handed and not confirmed"
    );
    world.confirm_every("ops");
    world.engine().settle(None);
    ensure!(
        world.held(&kept_id).is_none(),
        "the document kept once every end was delivered"
    );
    let left: Vec<String> = world
        .deliveries()
        .iter()
        .map(weir_delivery::json::write)
        .collect();
    ensure!(
        left.len() == 1 && left[0].contains("weir-known/1") && left[0].contains("\"ops\""),
        "the deliveries once let go of: {left:?}"
    );
    Ok(())
}

/// The end-delivery spec's "Dropped while pending" (62; weir I9).
fn dropped_while_pending(_: &()) -> Outcome {
    let world = World::new();
    let id = landed_with(&world, "one", &["boomdrive"])?;
    let before = tidied(&world);
    ensure_eq!(before.len(), 1, "tidied documents");
    let answer = world.engine().drop_place(&id, false);
    let reason = refused(&answer)?;
    ensure!(
        reason.contains("boomdrive") && reason.contains("flume confirm"),
        "the drop's refusal: {reason}"
    );
    ensure_eq!(
        tidied(&world),
        before,
        "the document after the refused drop"
    );
    ensure!(
        world
            .status(&id)?
            .ends
            .iter()
            .all(|end| end.pending_for == vec!["boomdrive".to_string()]),
        "the ends after the refused drop"
    );
    // Once boomdrive confirms them, the drop takes the document away.
    world.confirm_every("boomdrive");
    let answer = world.engine().drop_place(&id, false);
    ensure!(
        answer.refusal().is_none(),
        "the drop once confirmed: {answer:?}"
    );
    ensure!(tidied(&world).is_empty(), "the document after its drop");
    Ok(())
}

fn launched_again(_: &()) -> Outcome {
    let world = World::new();
    let id = landed_away(&world, "one")?;
    let before = tidied(&world);
    let mut again = launch("fix", "one", Some("mend it again"));
    again.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&again))?;
    ensure_eq!(
        world.held(&id).map(|held| held.phase),
        Some(Phase::Live),
        "the place launched again"
    );
    ensure!(world.place.exists(&id.0), "the place's worktree made again");
    let after = tidied(&world);
    ensure_eq!(after.len(), 1, "tidied documents");
    ensure_eq!(after[0].ends, before[0].ends, "the tidied document's ends");
    ensure!(
        after[0]
            .ends
            .iter()
            .all(|end| end.pending(&after[0].place, &world.owing()) == vec!["ops".to_string()]),
        "the tidied document's ends are not all pending for ops"
    );
    Ok(())
}

/// The ends an inbox's answer collected, by what ended.
fn collected(answer: &Answer) -> Result<Vec<String>, String> {
    match answer {
        Answer::Inbox { ends, .. } => Ok(ends
            .iter()
            .map(|collected| {
                format!(
                    "{}--{} {}",
                    collected.kind,
                    collected.name,
                    flume_core::what_ended(&collected.end.end.of)
                )
            })
            .collect()),
        other => Err(format!("the inbox answered {other:?}")),
    }
}

/// The end a wait answered and whether the wait delivered it; none while
/// still waiting.
fn waited(answer: &Answer) -> Result<Option<(EndStatus, bool)>, String> {
    match answer {
        Answer::Waited { end, delivered, .. } => Ok(end.clone().map(|end| (end, *delivered))),
        other => Err(format!("the wait answered {other:?}")),
    }
}

/// The recipients and the means each delivery of an end went by.
fn deliveries(end: &EndHeld) -> Vec<(String, DeliveredBy)> {
    end.deliveries
        .iter()
        .map(|delivery| (delivery.recipient.clone(), delivery.by))
        .collect()
}

fn collected_after_the_tidy(_: &()) -> Outcome {
    let world = World::new();
    landed_away(&world, "one")?;
    let kept = tidied(&world);
    ensure_eq!(kept.len(), 1, "tidied documents");
    let landed = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    ensure_eq!(
        collected(&world.engine().inbox("ops", None, true))?,
        vec![
            "fix--one code delivered".to_string(),
            format!("fix--one submission landed {landed}"),
        ],
        "what ops collected"
    );
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent to nobody"
    );
    world.engine().settle(None);
    ensure!(
        tidied(&world).is_empty(),
        "the document kept once every end was collected"
    );
    ensure!(
        collected(&world.engine().inbox("ops", None, true))?.is_empty(),
        "a second inbox gave something"
    );
    Ok(())
}

fn a_restarted_recipient_waits_again(_: &()) -> Outcome {
    restarted_and_waited().map(|_| ())
}

/// The end-delivery spec's "A restarted recipient waits again": the
/// landing handed to `ops` by its wait, and still pending for it (43).
fn restarted_and_waited() -> Result<(World, PlaceId), String> {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Working);
    let id = fix_for_ops(&world, "one")?;
    let first = world
        .engine()
        .wait(&id, &WaitFor::Submission, "ops", Some(10), true);
    ensure!(waited(&first)?.is_none(), "the first wait: {first:?}");
    ensure_eq!(first.exit(), 3, "the first wait's exit");
    world.terminal.remove_session("ops");
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place's own document");
    world.terminal.add_session("ops", AgentState::Working);
    world.host.log.clear();
    let again = world
        .engine
        .wait(&id, &WaitFor::Submission, "ops", Some(10), false);
    ensure_eq!(world.host.log.count("sleep"), 0, "the wait slept");
    let (end, delivered) = waited(&again)?.ok_or("still waiting")?;
    let landed = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    ensure!(
        matches!(&end.end.of, EndOf::Submission { verdict: Verdict::Landed { commit, .. }, .. } if *commit == landed),
        "the end answered: {:?}",
        end.end.of
    );
    ensure!(delivered, "the wait did not hand it over");
    ensure_eq!(again.exit(), 0, "the wait's exit");
    ensure_eq!(
        deliveries(&end.end),
        vec![("ops".to_string(), DeliveredBy::Wait)],
        "the landing's hand-overs"
    );
    let landing = landing_of(&world, &id)?;
    ensure!(
        landing.end.delivered_to("ops") && landing.pending_for == vec!["ops".to_string()],
        "the landing as status gives it: {landing:?}"
    );
    Ok((world, id))
}

/// The landing of the place as status gives it.
fn landing_of(world: &World, id: &PlaceId) -> Result<EndStatus, String> {
    world
        .status(id)?
        .ends
        .into_iter()
        .find(|end| matches!(end.end.of, EndOf::Submission { .. }))
        .ok_or("no landing in status".into())
}

/// The end-delivery spec's "A wait that confirms": its hand-over and the
/// recipient's confirmation recorded together (43; weir 103).
fn a_wait_that_confirms(_: &()) -> Outcome {
    let (world, id) = restarted_and_waited()?;
    world.host.advance(60);
    let confirmed = world
        .engine()
        .wait(&id, &WaitFor::Submission, "ops", Some(10), true);
    let (end, delivered) = waited(&confirmed)?.ok_or("still waiting")?;
    ensure!(
        delivered
            && matches!(
                &end.end.of,
                EndOf::Submission {
                    verdict: Verdict::Landed { .. },
                    ..
                }
            ),
        "the wait that confirms: {confirmed:?}"
    );
    ensure!(
        end.pending_for.is_empty(),
        "pending for {:?}",
        end.pending_for
    );
    let landing = landing_of(&world, &id)?;
    ensure_eq!(
        deliveries(&landing.end),
        vec![
            ("ops".to_string(), DeliveredBy::Wait),
            ("ops".to_string(), DeliveredBy::Wait)
        ],
        "the landing's hand-overs"
    );
    let now = world.host.now();
    let owing = world.owing();
    let delivery = owing
        .deliveries
        .values()
        .find(|delivery| delivery.key.ends_with("/submission/1/landed"))
        .ok_or("no delivery of the landing")?;
    ensure_eq!(
        (
            delivery.confirmed.clone(),
            delivery.handovers.last().map(|handover| handover.at)
        ),
        (
            vec![ConfirmedBy {
                recipient: "ops".into(),
                at: now
            }],
            Some(now)
        ),
        "the confirmation and the hand-over it was recorded with"
    );
    Ok(())
}

fn the_timeout(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_for_ops(&world, "one")?;
    let began = world.host.now();
    let answer = world
        .engine()
        .wait(&id, &WaitFor::Stage("code".into()), "ops", Some(60), true);
    ensure!(waited(&answer)?.is_none(), "the wait: {answer:?}");
    ensure_eq!(answer.exit(), 3, "the wait's exit");
    ensure_eq!(world.host.now() - began, 60, "how long the wait followed");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    ensure!(
        world
            .status(&id)?
            .ends
            .iter()
            .all(|end| end.end.deliveries.is_empty()),
        "a delivery recorded"
    );
    Ok(())
}

fn a_wait_reads_an_end_delivered(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure_eq!(world.terminal.notices("ops").len(), 1, "notices to ops");
    world.confirm("ops", "fix--one", &["stage/code/1/delivered"])?;
    let answer = world
        .engine()
        .wait(&id, &WaitFor::Stage("code".into()), "ops", Some(10), true);
    let (end, delivered) = waited(&answer)?.ok_or("still waiting")?;
    ensure!(!delivered, "the wait handed over an end confirmed before");
    ensure_eq!(answer.exit(), 0, "the wait's exit");
    ensure_eq!(
        deliveries(&end.end),
        vec![("ops".to_string(), DeliveredBy::Notice)],
        "the deliveries"
    );
    let other = world.engine().wait(
        &id,
        &WaitFor::Stage("code".into()),
        "planner",
        Some(10),
        true,
    );
    let (end, delivered) = waited(&other)?.ok_or("still waiting")?;
    ensure!(!delivered, "a wait delivered to a recipient not named");
    ensure_eq!(
        end.end.deliveries.len(),
        1,
        "deliveries after another's wait"
    );
    ensure_eq!(world.terminal.notices("ops").len(), 1, "notices to ops");
    Ok(())
}

/// Every call a seam's fake logged, but the store's.
fn seam_calls(world: &World) -> Vec<String> {
    [
        &world.history.log,
        &world.place.log,
        &world.kinds.log,
        &world.change.log,
        &world.terminal.log,
        &world.agent.log,
        &world.queue.log,
        &world.resolver.log,
    ]
    .iter()
    .flat_map(|log| log.calls())
    .collect()
}

/// Clears every seam's fake's log, the store's among them.
fn clear_calls(world: &World) {
    for log in [
        &world.history.log,
        &world.place.log,
        &world.kinds.log,
        &world.change.log,
        &world.terminal.log,
        &world.agent.log,
        &world.queue.log,
        &world.resolver.log,
        &world.store.log,
    ] {
        log.clear();
    }
}

fn a_wait_under_a_watch(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    world.engine.settle_every(30);
    deliver_fix(&world, &id);
    clear_calls(&world);
    let first = world
        .engine()
        .wait(&id, &WaitFor::Stage("code".into()), "ops", Some(20), true);
    ensure!(waited(&first)?.is_none(), "the first wait: {first:?}");
    ensure_eq!(
        seam_calls(&world),
        Vec::<String>::new(),
        "calls but the store's"
    );
    ensure!(world.store.log.count("read") > 0, "the store was not read");
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code moved without the watch"
    );
    world.engine.settle_every(30);
    clear_calls(&world);
    let second = world
        .engine()
        .wait(&id, &WaitFor::Stage("code".into()), "ops", Some(20), true);
    ensure_eq!(
        seam_calls(&world),
        Vec::<String>::new(),
        "calls but the store's"
    );
    let (end, delivered) = waited(&second)?.ok_or("still waiting")?;
    ensure!(
        delivered
            && matches!(
                end.end.of,
                EndOf::Stage {
                    state: StageState::Delivered,
                    ..
                }
            ),
        "the wait after the watch's settle: {second:?}"
    );
    Ok(())
}

/// The end-delivery spec's "Attended under any terminal": no caller
/// keeps watch, so a wait settles every place before each reading (119,
/// 208).
fn a_wait_attended(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    let answer = world
        .engine()
        .wait(&id, &WaitFor::Stage("code".into()), "ops", Some(20), true);
    let (end, delivered) = waited(&answer)?.ok_or("still waiting")?;
    ensure!(
        delivered
            && matches!(
                end.end.of,
                EndOf::Stage {
                    state: StageState::Delivered,
                    ..
                }
            ),
        "the wait: {answer:?}"
    );
    ensure_eq!(world.queue.log.count("submit"), 1, "the place submitted");
    Ok(())
}

fn a_wait_refused(_: &()) -> Outcome {
    let world = World::new();
    let id = fix_for_ops(&world, "one")?;
    let engine = world.engine();
    let none = engine.wait(
        &World::id("fix", "none"),
        &WaitFor::Submission,
        "ops",
        Some(10),
        true,
    );
    ensure!(refused(&none)?.contains("no place"), "{none:?}");
    ensure_eq!(none.exit(), 1, "the refusal's exit");
    let stage = engine.wait(&id, &WaitFor::Stage("review".into()), "ops", Some(10), true);
    ensure!(refused(&stage)?.contains("no stage review"), "{stage:?}");
    world.begun("notes", None, "two")?;
    let two = World::id("notes", "two");
    let submission = engine.wait(&two, &WaitFor::Submission, "ops", Some(10), true);
    ensure!(
        refused(&submission)?.contains("does not land"),
        "{submission:?}"
    );
    Ok(())
}

/// Two fixes for `ops`, each with its code delivered and its submission
/// held, while no session is named `ops`.
fn two_pending(world: &World) -> Result<(), String> {
    world.queue.plan_submits(vec![Plan::Hold, Plan::Hold]);
    for name in ["one", "two"] {
        let id = fix_for_ops(world, name)?;
        deliver_fix(world, &id);
        world.settle(&id);
    }
    Ok(())
}

fn collected_after_a_restart(_: &()) -> Outcome {
    let world = World::new();
    two_pending(&world)?;
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent to nobody"
    );
    world.engine().status(None);
    ensure!(
        collected(&world.engine().inbox("planner", None, true))?.is_empty(),
        "another recipient's inbox collected"
    );
    world.engine.settle_every(30);
    world.terminal.add_session("ops", AgentState::Working);
    let answer = world.engine().inbox("ops", None, false);
    ensure_eq!(
        collected(&answer)?,
        vec![
            "fix--one code delivered".to_string(),
            "fix--two code delivered".to_string(),
        ],
        "what ops collected"
    );
    let Answer::Inbox { ends, .. } = &answer else {
        return Err("no inbox".into());
    };
    for collected in ends {
        ensure!(
            matches!(&collected.end.end.of, EndOf::Stage { end, .. } if end.head.is_some()),
            "an end not whole: {collected:?}"
        );
        ensure_eq!(
            deliveries(&collected.end.end),
            vec![("ops".to_string(), DeliveredBy::Inbox)],
            "the deliveries"
        );
        ensure_eq!(
            collected.end.key.to_string(),
            "stage/code/1/delivered",
            "the end's key"
        );
    }
    for name in ["one", "two"] {
        let status = world.status(&World::id("fix", name))?;
        ensure!(
            status
                .ends
                .iter()
                .all(|end| end.pending_for == vec!["ops".to_string()]),
            "pending after the inbox: {:?}",
            status.ends
        );
    }
    ensure_eq!(
        collected(&world.engine().inbox("ops", None, false))?,
        vec![
            "fix--one code delivered".to_string(),
            "fix--two code delivered".to_string(),
        ],
        "what a second inbox gave"
    );
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent besides the inbox"
    );
    Ok(())
}

/// The end-delivery spec's "Collected and confirmed" (43; weir 103).
fn collected_and_confirmed(_: &()) -> Outcome {
    let world = World::new();
    two_pending(&world)?;
    world.engine.settle_every(30);
    world.terminal.add_session("ops", AgentState::Working);
    collected(&world.engine().inbox("ops", None, false))?;
    let answer = world.engine().inbox("ops", None, true);
    ensure_eq!(
        collected(&answer)?,
        vec![
            "fix--one code delivered".to_string(),
            "fix--two code delivered".to_string(),
        ],
        "what ops collected and confirmed"
    );
    for name in ["one", "two"] {
        let status = world.status(&World::id("fix", name))?;
        ensure!(
            status.ends.iter().all(|end| end.pending_for.is_empty()),
            "pending after the inbox that confirms: {:?}",
            status.ends
        );
    }
    ensure!(
        collected(&world.engine().inbox("ops", None, false))?.is_empty(),
        "a third inbox gave something"
    );
    Ok(())
}

fn two_inboxes_at_once(_: &()) -> Outcome {
    let world = World::new();
    two_pending(&world)?;
    world.engine.settle_every(30);
    let other = Arc::new(Mutex::new(None));
    let engine = world.engine();
    let into = other.clone();
    world.store.before_next_host_write(move || {
        let answer = engine.inbox("ops", None, true);
        *into.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(answer);
    });
    let first = collected(&world.engine().inbox("ops", None, true))?;
    let second = other
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .take()
        .ok_or("the second inbox did not run")?;
    let mut both = first.clone();
    both.extend(collected(&second)?);
    both.sort();
    ensure_eq!(
        both,
        vec![
            "fix--one code delivered".to_string(),
            "fix--two code delivered".to_string(),
        ],
        "each end collected once across both inboxes"
    );
    ensure!(
        world
            .store
            .log
            .calls()
            .iter()
            .any(|call| call.ends_with("moved")),
        "the two inboxes did not meet at one write"
    );
    for name in ["one", "two"] {
        for end in world.status(&World::id("fix", name))?.ends {
            ensure_eq!(end.end.deliveries.len(), 1, "deliveries of {name}'s end");
        }
    }
    Ok(())
}

const PLANNER: &str = "madswan/planner";

/// A fix named `name` launched for `recipient` and begun.
fn fix_for(world: &World, name: &str, recipient: &str) -> Result<PlaceId, String> {
    let mut launch = launch("fix", name, Some("mend it"));
    launch.recipients = vec![recipient.into()];
    launched(&world.engine.launch(&launch))?;
    let id = World::id("fix", name);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        format!("{name}'s code")
    );
    Ok(id)
}

/// A source gives `address` the standing agent `agent`, in `session`
/// where given, its stream its address's (132, 193).
fn stands(world: &World, address: &str, agent: &str, session: Option<&str>) {
    world.host.set_standing(
        address,
        Some(StandingAgent {
            source: "acme".into(),
            agent: agent.into(),
            session: session.map(str::to_string),
            answers: None,
            answerer: false,
        }),
    );
}

/// The planner stands as `madswan-planner` in the session `madswan`, of
/// the stream `madswan` the host declares.
fn planner_stands(world: &World) {
    world.host.set_stream("madswan", Some(Stream::default()));
    stands(world, PLANNER, "madswan-planner", Some("madswan"));
}

/// A fix named `name` for the planner, launched, begun and delivered, its
/// submission held, and settled.
fn delivered_for_planner(world: &World, name: &str) -> Result<PlaceId, String> {
    let id = fix_for(world, name, PLANNER)?;
    deliver_fix(world, &id);
    world.settle(&id);
    Ok(id)
}

/// Who each end of the place stands pending for.
fn pending(world: &World, id: &PlaceId) -> Result<Vec<Vec<String>>, String> {
    Ok(ends(world, id)?
        .into_iter()
        .map(|end| end.pending_for)
        .collect())
}

fn a_standing_agent_waiting(_: &()) -> Outcome {
    let world = World::new();
    planner_stands(&world);
    let at = world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = delivered_for_planner(&world, "one")?;
    ensure_eq!(
        world.terminal.notices(&at),
        vec![world.confirming(&delivered("one"))],
        "the notices to the standing agent"
    );
    ensure_eq!(
        world.terminal.notices(PLANNER),
        Vec::<String>::new(),
        "notices to a session named by the address"
    );
    let ends = ends(&world, &id)?;
    let end = ends.first().ok_or("no end recorded")?;
    ensure_eq!(
        end.end
            .deliveries
            .iter()
            .map(|delivery| (delivery.recipient.as_str(), delivery.at, delivery.by))
            .collect::<Vec<_>>(),
        vec![(PLANNER, world.host.now(), DeliveredBy::Notice)],
        "the hand-over recorded"
    );
    ensure_eq!(
        end.pending_for,
        vec![PLANNER.to_string()],
        "pending for, handed by notice"
    );
    ensure_eq!(world.store.places(), vec![id], "the places on the host");
    ensure_eq!(world.terminal.log.count("open"), 1, "agents opened");
    Ok(())
}

fn a_standing_agent_asking(_: &()) -> Outcome {
    let world = World::new();
    planner_stands(&world);
    let at = world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Asking);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = delivered_for_planner(&world, "one")?;
    ensure!(
        world.terminal.notices(&at).is_empty(),
        "a notice sent into a question"
    );
    ensure_eq!(
        pending(&world, &id)?,
        vec![vec![PLANNER.to_string()]],
        "pending while it asks"
    );
    world.terminal.set_session_state(&at, AgentState::Waiting);
    world.settle(&id);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices(&at),
        vec![world.confirming(&delivered("one"))],
        "the notices once it waits"
    );
    Ok(())
}

fn no_agent_of_the_name(_: &()) -> Outcome {
    let world = World::new();
    planner_stands(&world);
    let other = world
        .terminal
        .add_agent(Some("madswan"), "madswan-ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = delivered_for_planner(&world, "one")?;
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent: {:?}",
        world.terminal.every_notice()
    );
    ensure!(
        world.terminal.notices(&other).is_empty(),
        "a notice to an agent of another name"
    );
    ensure_eq!(world.queue.log.count("submit"), 1, "the place submitted");
    let ends = ends(&world, &id)?;
    ensure!(!ends.is_empty(), "no end recorded");
    for end in &ends {
        ensure_eq!(end.pending_for, vec![PLANNER.to_string()], "pending");
        ensure!(end.end.deliveries.is_empty(), "a delivery claimed");
    }
    Ok(())
}

fn two_agents_of_the_name(_: &()) -> Outcome {
    let world = World::new();
    planner_stands(&world);
    let first = world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Waiting);
    let second = world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = delivered_for_planner(&world, "one")?;
    world.settle(&id);
    ensure!(
        world.terminal.notices(&first).is_empty() && world.terminal.notices(&second).is_empty(),
        "a notice to one of two of the name"
    );
    ensure_eq!(
        pending(&world, &id)?,
        vec![vec![PLANNER.to_string()]],
        "pending"
    );
    Ok(())
}

fn an_agent_revived_under_its_name(_: &()) -> Outcome {
    let world = World::new();
    planner_stands(&world);
    let first = world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold, Plan::Hold]);
    let one = delivered_for_planner(&world, "one")?;
    ensure_eq!(
        world.terminal.notices(&first),
        vec![world.confirming(&delivered("one"))],
        "the first agent's notices"
    );
    world.terminal.remove_agent(&first);
    let revived = world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Waiting);
    ensure!(revived != first, "the revived agent shares its key");
    delivered_for_planner(&world, "two")?;
    world.settle(&one);
    ensure_eq!(
        world.terminal.notices(&revived),
        vec![world.confirming(&delivered("two"))],
        "the revived agent's notices"
    );
    ensure_eq!(
        world.terminal.notices(&first).len(),
        1,
        "the removed agent's notices"
    );
    Ok(())
}

fn the_session_from_the_addresss_stream(_: &()) -> Outcome {
    let world = World::new();
    let other = world.other_terminal("other");
    world.host.set_stream(
        "w",
        Some(Stream {
            session: Some("w-main".into()),
            ..Stream::default()
        }),
    );
    world.host.set_display("w", Some("other"), None, None);
    stands(&world, "w/planner", "w-planner", None);
    let wrong = world
        .terminal
        .add_agent(Some("w-main"), "w-planner", AgentState::Waiting);
    let at = other.add_agent(Some("w-main"), "w-planner", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for(&world, "one", "w/planner")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure_eq!(
        other.notices(&at),
        vec![world.confirming(&delivered("one"))],
        "the notices in w's terminal"
    );
    ensure!(
        world.terminal.notices(&wrong).is_empty(),
        "a notice in a terminal not the stream's"
    );
    ensure_eq!(
        other.log.of("agents_in"),
        vec!["agents_in w-main".to_string()],
        "the parts w's terminal was asked for"
    );
    ensure_eq!(
        world.terminal.log.count("agents_in"),
        0,
        "the host's own terminal asked"
    );
    Ok(())
}

fn the_default_streams_own_session(_: &()) -> Outcome {
    let world = World::new();
    stands(&world, "default/planner", "w-planner", None);
    let elsewhere = world
        .terminal
        .add_agent(Some("w"), "w-planner", AgentState::Waiting);
    let at = world
        .terminal
        .add_agent(None, "w-planner", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for(&world, "one", "default/planner")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure_eq!(
        world.terminal.notices(&at),
        vec![world.confirming(&delivered("one"))],
        "the notices in the terminal's own part"
    );
    ensure!(
        world.terminal.notices(&elsewhere).is_empty(),
        "a notice in a part not the default stream's"
    );
    ensure_eq!(
        world.terminal.log.of("agents_in"),
        vec!["agents_in own".to_string()],
        "the parts asked"
    );
    Ok(())
}

fn a_stream_not_declared(_: &()) -> Outcome {
    let world = World::new();
    stands(&world, "gone/planner", "madswan-planner", None);
    let at = world
        .terminal
        .add_agent(Some("gone"), "madswan-planner", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for(&world, "one", "gone/planner")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure_eq!(world.terminal.log.count("agents_in"), 0, "parts asked");
    ensure!(
        world.terminal.notices(&at).is_empty(),
        "a notice in a stream the host does not declare"
    );
    ensure_eq!(
        pending(&world, &id)?,
        vec![vec!["gone/planner".to_string()]],
        "pending"
    );
    Ok(())
}

fn the_inbox_collects_for_a_standing_address(_: &()) -> Outcome {
    let world = World::new();
    planner_stands(&world);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = delivered_for_planner(&world, "one")?;
    ensure!(
        world.terminal.every_notice().is_empty(),
        "a notice sent while the agent is away"
    );
    ensure_eq!(
        collected(&world.engine().inbox(PLANNER, None, false))?,
        vec!["fix--one code delivered".to_string()],
        "what the inbox collected"
    );
    for end in world.status(&id)?.ends {
        ensure_eq!(
            deliveries(&end.end),
            vec![(PLANNER.to_string(), DeliveredBy::Inbox)],
            "the hand-overs"
        );
        ensure_eq!(
            end.pending_for,
            vec![PLANNER.to_string()],
            "pending for, handed by the inbox"
        );
    }
    let at = world
        .terminal
        .add_agent(Some("madswan"), "madswan-planner", AgentState::Waiting);
    world.settle(&id);
    ensure!(
        world.terminal.notices(&at).is_empty(),
        "a notice of an end the inbox collected"
    );
    Ok(())
}

/// talk's lost end as held state records it, and the notices sent to
/// `ops`.
fn talks_lost_end(world: &World, id: &PlaceId) -> Result<(StageEnd, Vec<String>), String> {
    let held = world.held(id).ok_or("no held state")?;
    let end = held
        .ends
        .iter()
        .find_map(|end| match &end.of {
            EndOf::Stage {
                stage,
                state: StageState::Lost,
                end,
                ..
            } if stage == "talk" => Some(StageEnd::clone(end)),
            _ => None,
        })
        .ok_or("no lost end of talk is recorded")?;
    Ok((end, world.terminal.notices("ops")))
}

fn a_lost_talk_with_a_bare_agent(_: &()) -> Outcome {
    let world = World::new();
    let id = talking(&world)?;
    world.host.restart();
    world.terminal.forget_all();
    world.agent.bring_back_bare(&talk(1));
    world.engine().settle(None);
    let (end, notices) = talks_lost_end(&world, &id)?;
    ensure!(
        end.left_in_pane,
        "talk's lost end does not say an agent was left"
    );
    ensure_eq!(
        notices,
        vec![world.confirming(
            "flume: research--one (weigh the options) talk lost, and flume left the agent the terminal brought back in its pane [stage/talk/1/lost]; read: flume inbox"
        )],
        "the notices to ops"
    );
    Ok(())
}

fn a_lost_talk_with_no_agent(_: &()) -> Outcome {
    let world = World::new();
    let id = talking(&world)?;
    world.terminal.kill(&talk(1));
    world.settle(&id);
    let (end, notices) = talks_lost_end(&world, &id)?;
    ensure!(!end.left_in_pane, "talk's lost end says an agent was left");
    ensure_eq!(
        notices,
        vec![world.confirming(
            "flume: research--one (weigh the options) talk lost [stage/talk/1/lost]; read: flume inbox"
        )],
        "the notices to ops"
    );
    Ok(())
}

/// The places the inbox gave as waiting on its recipient.
fn waiting(answer: &Answer) -> Result<Vec<WaitingPlace>, String> {
    match answer {
        Answer::Inbox { waiting, .. } => Ok(waiting.clone()),
        other => Err(format!("the inbox answered {other:?}")),
    }
}

/// The key of construct's first start, ended short.
fn construct_short() -> EndKey {
    "stage/construct/1/ended-short".parse().expect("a key")
}

fn a_question_collected_then_a_restart(_: &()) -> Outcome {
    let world = World::new();
    let (id, asked) = construct_asks(&world, "plan", &["ops"], None)?;
    let first = world.engine().inbox("ops", None, true);
    ensure_eq!(
        collected(&first)?,
        vec!["change--plan construct needs an answer".to_string()],
        "what ops collected first"
    );
    let before = world.held(&id);
    let again = world.engine().inbox("ops", None, true);
    ensure!(collected(&again)?.is_empty(), "the restart collected again");
    ensure_eq!(world.held(&id), before, "the second inbox wrote the place");
    let waits = waiting(&again)?;
    ensure_eq!(waits.len(), 1, "the places waiting on ops: {waits:?}");
    let place = &waits[0];
    ensure_eq!(
        (
            place.place.clone(),
            place.kind.as_str(),
            place.name.as_str(),
            place.brief.as_deref()
        ),
        (id.clone(), "change", "plan", Some("plan it")),
        "the place"
    );
    ensure_eq!(
        (
            place.waiting_on.act,
            place.waiting_on.stage.as_deref(),
            place.waiting_on.why
        ),
        (Act::Retry, Some("construct"), Waits::Answer),
        "the move"
    );
    ensure!(
        place.waiting_on.since.is_some_and(|since| since >= asked),
        "since when: {:?}",
        place.waiting_on.since
    );
    let stop_short = place.stop_short.clone().ok_or("no stop-short")?;
    ensure_eq!(
        (
            stop_short.words.as_deref(),
            stop_short.needs,
            stop_short.key,
            stop_short.addressee
        ),
        (
            Some("Which design does the brief mean?"),
            Some(StopKind::NeedsAnswer),
            construct_short(),
            None
        ),
        "the stop-short"
    );
    ensure!(place.hold.is_none(), "a hold given: {:?}", place.hold);
    ensure!(
        waiting(&world.engine().inbox("planner", None, true))?.is_empty(),
        "another recipient's inbox gives the place"
    );
    Ok(())
}

fn the_inbox_of_the_role_asked(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    answerer_behind(&world, "east");
    let (id, _) = asks_in_east(&world, &["east/planner"], Some("ops"))?;
    let waits = waiting(&world.engine.inbox("east/ops", None, true))?;
    let place = waits.iter().find(|place| place.place == id).ok_or(format!(
        "east/ops's inbox gives no waiting place: {waits:?}"
    ))?;
    ensure_eq!(
        place
            .stop_short
            .as_ref()
            .and_then(|stop_short| stop_short.addressee.clone()),
        Some("east/ops".to_string()),
        "the stop-short's addressee"
    );
    ensure_eq!(place.stream.as_str(), "east", "the place's stream");
    ensure!(
        waiting(&world.engine.inbox("east/design", None, true))?
            .iter()
            .all(|place| place.place != id),
        "a role neither asked nor a recipient is given the place"
    );
    Ok(())
}

fn the_answerers_waiting_places(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    answerer_behind(&world, "east");
    world.terminal.add_session("ops", AgentState::Waiting);
    world.host.set_recipient(None);
    world.terminal.set_session(Some("ops"));
    let (id, _) = asks_in_east(&world, &[], None)?;
    ensure!(
        waiting(&world.engine.inbox("east/operator", None, true))?
            .iter()
            .all(|place| place.place != id),
        "the answerer is given a place that has not stalled"
    );
    world.host.advance(STALL_AFTER);
    world.settle(&id);
    let held = world.held(&id).ok_or("no place")?;
    ensure!(
        !held.place.recipients.contains(&"east/operator".to_string()),
        "the answerer is a recipient: {:?}",
        held.place.recipients
    );
    let waits = waiting(&world.engine.inbox("east/operator", None, true))?;
    let place = waits.iter().find(|place| place.place == id).ok_or(format!(
        "the answerer's inbox gives no waiting place: {waits:?}"
    ))?;
    ensure_eq!(
        place
            .stop_short
            .as_ref()
            .map(|stop_short| stop_short.key.clone()),
        Some(construct_short()),
        "the stop-short's key"
    );
    Ok(())
}

fn a_held_question_in_the_inbox(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    answerer_behind(&world, "east");
    let (id, _) = asks_in_east(&world, &["east/planner"], None)?;
    world.host.advance(60);
    let held_at = world.host.now();
    let words = "east-planner has it";
    let answer = world
        .engine
        .hold(&id, HoldOn::Waiting, words, Some(60 * 60));
    ensure!(
        answer.refusal().is_none(),
        "the hold was refused: {answer:?}"
    );
    let due = Some(Hold {
        on: HoldOn::Waiting,
        words: Some(words.into()),
        since: held_at,
        bound: Some(60 * 60),
        lapsed: None,
    });
    for read in ["the first", "the next"] {
        let waits = waiting(&world.engine.inbox("east/planner", None, true))?;
        let place = waits
            .iter()
            .find(|place| place.place == id)
            .ok_or(format!("{read} inbox gives no waiting place: {waits:?}"))?;
        ensure_eq!(
            place.hold.clone(),
            due.clone(),
            "the hold {read} inbox gives"
        );
    }
    Ok(())
}

/// A brief whose first line runs past what a notice gives of it.
const LONG_BRIEF: &str = "Every question a place asks reaches someone who answers it, whichever stream asked\n\nThe rest of the brief.";

/// [`LONG_BRIEF`]'s first line as a notice gives it: cut at 60
/// characters, the ellipsis saying where.
const CUT_BRIEF: &str = "Every question a place asks reaches someone who answers it,…";

/// The change `every-question` launched for `ops` with [`LONG_BRIEF`] and
/// begun.
fn every_question(world: &World) -> Result<PlaceId, String> {
    let mut change = launch("change", "every-question", Some(LONG_BRIEF));
    change.recipients = vec!["ops".into()];
    launched(&world.engine.launch(&change))?;
    let id = World::id("change", "every-question");
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "construct"),
        Some(StageState::Begun),
        "construct"
    );
    Ok(id)
}

fn a_stop_shorts_notice(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let id = every_question(&world)?;
    let construct = World::identity("change", "every-question", "construct", 1);
    let secret = world
        .held(&id)
        .and_then(|held| held.stages[0].start.clone())
        .ok_or("no start")?
        .secret;
    let asked = world.host.now();
    let answer = world.engine.needs(
        &construct.to_string(),
        &secret.0,
        "Which design does the brief mean?\nThe brief names two.",
        StopKind::NeedsAnswer,
    );
    ensure!(answer.refusal().is_none(), "the stop-short: {answer:?}");
    world.settle(&id);
    ensure!(
        CUT_BRIEF.trim_end_matches('…').chars().count() <= 60,
        "the brief given is longer than 60 characters"
    );
    let notices = world.terminal.notices("ops");
    let token = world
        .held(&id)
        .and_then(|held| held.place.token)
        .ok_or("the place holds no token")?;
    ensure_eq!(
        notices,
        vec![format!(
            "flume: change--every-question ({CUT_BRIEF}) construct needs an answer, its agent kept open for the answer until {} [stage/construct/1/ended-short]; read: flume inbox; confirm: flume confirm change--every-question/{token}/stage/construct/1/ended-short",
            asked + flume_core::KEEP_OPEN
        )],
        "the notices to ops"
    );
    ensure!(
        notices.iter().all(|notice| !notice.contains('\n')),
        "a notice of more than one line"
    );
    Ok(())
}

fn a_deliverys_notice(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let id = every_question(&world)?;
    write_change(&world, "every-question");
    world.settle(&id);
    let token = world
        .held(&id)
        .and_then(|held| held.place.token)
        .ok_or("the place holds no token")?;
    ensure_eq!(
        world.terminal.notices("ops"),
        vec![format!(
            "flume: change--every-question ({CUT_BRIEF}) construct delivered [stage/construct/1/delivered]; read: flume status change--every-question; confirm: flume confirm change--every-question/{token}/stage/construct/1/delivered"
        )],
        "the notices to ops"
    );
    Ok(())
}

fn a_sessions_own_inbox(_: &()) -> Outcome {
    let world = World::new();
    east_and_west(&world);
    answerer_behind(&world, "east");
    world.keep_watch();
    let (id, _) = asks_in_east(&world, &["east/planner"], None)?;
    world.host.set_recipient(Some("east/planner"));
    world.terminal.set_session(None);
    ensure!(
        world.engine.session_inbox().is_none(),
        "an inbox where no terminal names the session"
    );
    world.terminal.set_session(Some("ops"));
    let answer = world.engine.session_inbox().ok_or("no inbox")?;
    ensure!(
        matches!(&answer, Answer::Inbox { recipient, ends, waiting }
            if recipient == "ops" && ends.is_empty() && waiting.is_empty()),
        "the session's inbox: {answer:?}"
    );
    let stop_short = ends(&world, &id)?
        .into_iter()
        .find(|end| matches!(end.end.of, EndOf::Stage { .. }))
        .ok_or("no stop-short")?;
    ensure!(
        stop_short.pending_for.contains(&"east/planner".to_string()),
        "the stop-short is no longer pending for east/planner: {:?}",
        stop_short.pending_for
    );
    Ok(())
}

fn a_restarted_sessions_words(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.set_session(Some("ops"));
    world.host.set_recipient(None);
    let (_, asked) = construct_asks(&world, "plan", &["ops"], None)?;
    collected(&world.engine().inbox("ops", None, true))?;
    let answer = world.engine.session_inbox().ok_or("no inbox")?;
    let words = flume_core::session_words(&answer).ok_or("no words")?;
    let until = asked + flume_core::KEEP_OPEN;
    for said in [
        "what waits on it, ops, from `flume inbox`".to_string(),
        "change--plan (plan it), in the stream default, waits on a retry of construct".to_string(),
        "Its stop-short stage/construct/1/ended-short needs an answer:".to_string(),
        "  Which design does the brief mean?".to_string(),
        format!("Its agent is kept open for the answer until {until}."),
        "- answer it, naming its key: flume retry change--plan --stage construct --answers stage/construct/1/ended-short --words \"<answer>\"".to_string(),
        "  or, while its agent is kept open: flume tell change--plan --answers stage/construct/1/ended-short \"<answer>\"".to_string(),
        "- hold the move with your words, saying who has it: flume hold change--plan \"<words>\" --waiting --for <time>".to_string(),
        "- leave it: once it stalls, the answerer behind its stream has it.".to_string(),
    ] {
        ensure!(words.contains(&said), "{said:?} not in the words:\n{words}");
    }
    world.terminal.set_session(Some("lead"));
    let nothing = world.engine.session_inbox().ok_or("no inbox")?;
    ensure_eq!(
        flume_core::session_words(&nothing),
        None,
        "the words where nothing waits"
    );
    Ok(())
}

/// A fix named `name` launched for `recipients`, and begun (7, 43).
fn fix_with(world: &World, name: &str, recipients: &[&str]) -> Result<PlaceId, String> {
    let mut launch = launch("fix", name, Some("mend it"));
    launch.recipients = recipients.iter().map(|name| name.to_string()).collect();
    launched(&world.engine.launch(&launch))?;
    let id = World::id("fix", name);
    world.settle(&id);
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Begun),
        "code"
    );
    Ok(id)
}

/// A fix as `fix_with` launches it, landed and tidied in one settle.
fn landed_with(world: &World, name: &str, recipients: &[&str]) -> Result<PlaceId, String> {
    let id = fix_with(world, name, recipients)?;
    deliver_fix(world, &id);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the landed place's own document");
    Ok(id)
}

/// The notices of a first submission's landing sent to `to`.
fn landing_notices(world: &World, to: &str) -> usize {
    world
        .terminal
        .notices(to)
        .iter()
        .filter(|notice| notice.contains("[submission/1/landed]"))
        .count()
}

/// The end of the place keyed `key`, as status gives it.
fn end_keyed(world: &World, id: &PlaceId, key: &str) -> Result<EndStatus, String> {
    ends(world, id)?
        .into_iter()
        .find(|end| end.key.to_string() == key)
        .ok_or_else(|| format!("no end {key}"))
}

/// Whom the one tidied document on the host waits on.
fn kept_pending_for(world: &World) -> Result<Vec<String>, String> {
    let kept = tidied(world);
    ensure_eq!(kept.len(), 1, "tidied documents");
    match &kept[0].phase {
        Phase::Tidied { pending_for, .. } => Ok(pending_for.clone()),
        other => Err(format!("the document's phase: {other:?}")),
    }
}

/// Whether any role's run stands on the host.
fn any_run(world: &World) -> bool {
    world
        .store
        .places()
        .iter()
        .any(|place| world.held(place).is_some_and(|held| held.role.is_some()))
}

/// The end-delivery spec's "Kept for a recipient no pane holds" (62).
fn kept_for_a_reader(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let id = landed_with(&world, "one", &["ops", "boomdrive"])?;
    ensure_eq!(landing_notices(&world, "ops"), 1, "ops's notice");
    world.confirm_every("ops");
    world.engine().settle(None);
    let status = world.status(&id)?;
    ensure!(
        matches!(&status.phase, Phase::Tidied { pending_for, .. } if pending_for == &vec!["boomdrive".to_string()]),
        "the tidied place's phase: {:?}",
        status.phase
    );
    Ok(())
}

/// The end-delivery spec's "A landing for a recipient no pane holds"
/// (43, 62).
fn a_landing_for_a_reader(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let id = landed_with(&world, "one", &["ops", "boomdrive"])?;
    ensure_eq!(
        landing_notices(&world, "ops"),
        1,
        "ops's notices of the landing"
    );
    ensure!(
        world.terminal.notices("boomdrive").is_empty()
            && world
                .terminal
                .every_notice()
                .iter()
                .all(|(to, _)| to == "ops"),
        "a notice went elsewhere than ops: {:?}",
        world.terminal.every_notice()
    );
    let both = vec!["ops".to_string(), "boomdrive".to_string()];
    ensure_eq!(
        kept_pending_for(&world)?,
        both,
        "whom the document waits on"
    );
    for key in ["stage/code/1/delivered", "submission/1/landed"] {
        let end = end_keyed(&world, &id, key)?;
        ensure_eq!(end.pending_for, both, format!("whom {key} waits on"));
        ensure_eq!(
            deliveries(&end.end),
            vec![("ops".to_string(), DeliveredBy::Notice)],
            format!("{key}'s hand-overs")
        );
    }
    world.confirm_every("ops");
    let landed = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    ensure_eq!(
        collected(&world.engine().inbox("boomdrive", Some(&id), true))?,
        vec![
            "fix--one code delivered".to_string(),
            format!("fix--one submission landed {landed}"),
        ],
        "what boomdrive collected"
    );
    world.engine().settle(None);
    ensure!(
        tidied(&world).is_empty(),
        "the document kept once boomdrive collected"
    );
    Ok(())
}

/// The end-delivery spec's "A drop for a role and a reader" (43, 52,
/// 62).
fn a_drop_for_a_role_and_a_reader(_: &()) -> Outcome {
    let world = World::new();
    super::role_waking::configure_at(
        &world,
        "east/planner",
        "Read what is pending for you.",
        super::role_waking::planner_run(None, None),
    );
    let id = fix_with(&world, "one", &["east/planner", "boomdrive"])?;
    let dropped = world.engine().drop_place(&id, true);
    ensure!(dropped.refusal().is_none(), "the drop: {dropped:?}");
    world.engine().settle(None);
    ensure_eq!(
        kept_pending_for(&world)?,
        vec!["boomdrive".to_string()],
        "whom the document waits on"
    );
    ensure_eq!(
        end_keyed(&world, &id, "over/dropped")?.pending_for,
        vec!["boomdrive".to_string()],
        "whom the drop waits on"
    );
    ensure!(
        world.terminal.every_notice().is_empty(),
        "notices sent: {:?}",
        world.terminal.every_notice()
    );
    ensure!(!any_run(&world), "a run of east/planner was launched");
    ensure_eq!(
        collected(&world.engine().inbox("boomdrive", None, true))?,
        vec!["fix--one dropped".to_string()],
        "what boomdrive collected"
    );
    world.engine().settle(None);
    ensure!(
        tidied(&world).is_empty(),
        "the document kept once boomdrive collected"
    );
    ensure!(!any_run(&world), "a run of east/planner was launched");
    Ok(())
}

/// The end-delivery spec's "A drop for a role alone": the drop's end is
/// read while an untracked file keeps the place, then the tidy keeps no
/// document (43, 52, 62, 132).
fn a_drop_for_a_role_alone(_: &()) -> Outcome {
    let world = World::new();
    super::role_waking::configure_at(
        &world,
        "east/planner",
        "Read what is pending for you.",
        super::role_waking::planner_run(None, None),
    );
    let id = fix_with(&world, "one", &["east/planner"])?;
    world.place.set_holds(
        &id.0,
        Holds {
            untracked: vec!["scratch.txt".into()],
            ..Holds::default()
        },
    );
    let dropped = world.engine().drop_place(&id, true);
    ensure!(dropped.refusal().is_none(), "the drop: {dropped:?}");
    ensure_eq!(
        end_keyed(&world, &id, "over/dropped")?.pending_for,
        Vec::<String>::new(),
        "whom the drop waits on"
    );
    world.place.set_holds(&id.0, Holds::default());
    world.engine().settle(None);
    ensure!(world.held(&id).is_none(), "the dropped place's document");
    ensure!(tidied(&world).is_empty(), "a document kept");
    ensure!(
        world.terminal.every_notice().is_empty(),
        "notices sent: {:?}",
        world.terminal.every_notice()
    );
    ensure!(!any_run(&world), "a run of east/planner was launched");
    Ok(())
}

/// The end-delivery spec's "A drop for a session in a pane" (43, 52,
/// 62).
fn a_drop_for_a_session_in_a_pane(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let id = fix_for_ops(&world, "one")?;
    let dropped = world.engine().drop_place(&id, true);
    ensure!(dropped.refusal().is_none(), "the drop: {dropped:?}");
    world.engine().settle(None);
    ensure!(
        world.terminal.notices("ops").is_empty(),
        "ops's notices: {:?}",
        world.terminal.notices("ops")
    );
    ensure_eq!(
        kept_pending_for(&world)?,
        vec!["ops".to_string()],
        "whom the document waits on"
    );
    ensure_eq!(
        end_keyed(&world, &id, "over/dropped")?.pending_for,
        vec!["ops".to_string()],
        "whom the drop waits on"
    );
    ensure_eq!(
        collected(&world.engine().inbox("ops", None, true))?,
        vec!["fix--one dropped".to_string()],
        "what ops collected"
    );
    world.engine().settle(None);
    ensure!(
        tidied(&world).is_empty(),
        "the document kept once ops collected"
    );
    ensure!(
        world.terminal.notices("ops").is_empty(),
        "ops's notices after: {:?}",
        world.terminal.notices("ops")
    );
    Ok(())
}

/// The end-delivery spec's "A done" (43, 114).
fn a_done(_: &()) -> Outcome {
    let world = World::new();
    world.kinds.set_kinds(
        REPOSITORY,
        vec![kind(
            "notes",
            false,
            vec![stage("work", Some(Deliverable::Commits), "{{brief}}")],
        )],
    );
    let mut notes = launch("notes", "one", Some("do the work"));
    notes.recipients = vec!["boomdrive".into()];
    launched(&world.engine.launch(&notes))?;
    let id = World::id("notes", "one");
    world.settle(&id);
    let done = world.engine().done(&id, true);
    ensure!(done.refusal().is_none(), "the done: {done:?}");
    world.engine().settle(None);
    ensure_eq!(
        end_keyed(&world, &id, "over/done")?.pending_for,
        vec!["boomdrive".to_string()],
        "whom the done waits on"
    );
    ensure_eq!(
        kept_pending_for(&world)?,
        vec!["boomdrive".to_string()],
        "whom the document waits on"
    );
    ensure_eq!(
        collected(&world.engine().inbox("boomdrive", None, true))?,
        vec!["notes--one done".to_string()],
        "what boomdrive collected"
    );
    world.engine().settle(None);
    ensure!(
        tidied(&world).is_empty(),
        "the document kept once boomdrive collected"
    );
    Ok(())
}

/// The end-delivery spec's "An inbox naming a place" (43, 62).
fn an_inbox_naming_a_place(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let one = landed_with(&world, "one", &["ops", "boomdrive"])?;
    let first = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    let two = landed_with(&world, "two", &["ops", "boomdrive"])?;
    let second = world.git.branch_head(REPOSITORY, "main").ok_or("no main")?;
    world.confirm_every("ops");
    let answer = world.engine().inbox("boomdrive", Some(&one), true);
    ensure_eq!(
        collected(&answer)?,
        vec![
            "fix--one code delivered".to_string(),
            format!("fix--one submission landed {first}"),
        ],
        "what boomdrive's inbox naming fix--one collected"
    );
    let Answer::Inbox { ends, .. } = &answer else {
        return Err(format!("the inbox answered {answer:?}"));
    };
    ensure!(
        ends[0].end.end.deliveries.iter().any(|delivery| {
            delivery.recipient == "boomdrive" && delivery.by == DeliveredBy::Inbox
        }),
        "the landing's deliveries: {:?}",
        ends[0].end.end.deliveries
    );
    world.engine().settle(None);
    let kept = tidied(&world);
    ensure_eq!(kept.len(), 1, "tidied documents");
    ensure_eq!(
        kept[0].place.location.as_str(),
        two.0.as_str(),
        "the document that stays"
    );
    ensure_eq!(
        end_keyed(&world, &two, "submission/1/landed")?.pending_for,
        vec!["boomdrive".to_string()],
        "whom the second's landing waits on"
    );
    // Its document gone once collected, nothing stands under its name.
    refused(&world.engine().inbox("boomdrive", Some(&one), true))?;
    // The second, named by its document, `<location>@<at>` (62).
    let document = kept[0].place.id.clone();
    ensure!(
        document.0.starts_with(&format!("{}@", two.0)),
        "the document's name: {document}"
    );
    ensure_eq!(
        collected(&world.engine().inbox("boomdrive", Some(&document), true))?,
        vec![
            "fix--two code delivered".to_string(),
            format!("fix--two submission landed {second}"),
        ],
        "what boomdrive's inbox naming fix--two's document collected"
    );
    world.engine().settle(None);
    ensure!(tidied(&world).is_empty(), "a document kept");
    Ok(())
}

/// The end-delivery spec's "A name under which nothing stands" (43).
fn a_name_under_which_nothing_stands(_: &()) -> Outcome {
    let world = World::new();
    let one = landed_with(&world, "one", &["ops", "boomdrive"])?;
    let before = tidied(&world);
    let nothing = World::id("fix", "nothing");
    let reason = refused(&world.engine().inbox("boomdrive", Some(&nothing), true))?;
    ensure!(
        reason.contains(&nothing.0),
        "the refusal names no place: {reason}"
    );
    ensure_eq!(tidied(&world), before, "the documents after the refusal");
    ensure_eq!(
        end_keyed(&world, &one, "submission/1/landed")?.pending_for,
        vec!["ops".to_string(), "boomdrive".to_string()],
        "whom the landing waits on"
    );
    Ok(())
}

/// Whether `token` is eight hex characters, as a launch takes one (C.7).
fn a_token(token: &str) -> bool {
    token.len() == 8 && token.chars().all(|char| char.is_ascii_hexdigit())
}

/// The delivery the host's deliveries hold under `key`, as recorded.
fn delivery_under(world: &World, key: &str) -> Option<weir_delivery::Delivery> {
    world
        .deliveries()
        .into_iter()
        .find_map(|record| match record {
            weir_delivery::Record::Delivery(delivery) if delivery.key().as_str() == key => {
                Some(delivery)
            }
            _ => None,
        })
}

/// The readers weir's `owed` gives the delivery under `key` owed to, now
/// by the fake host's clock (weir 105, 111).
fn owed_to(world: &World, key: &str) -> Result<Vec<String>, String> {
    let now = flume_core::deliveries::time(world.host.now());
    let owed = weir_delivery::owed(&world.deliveries(), &now).map_err(|why| why.to_string())?;
    Ok(owed
        .readers
        .iter()
        .filter(|reader| {
            reader
                .deliveries
                .iter()
                .any(|owed| owed.delivery.key().as_str() == key)
        })
        .map(|reader| reader.reader.to_string())
        .collect())
}

/// The deliveries spec's "A place launched twice": two launches of one
/// place never share a delivery's key (43; weir 101).
fn a_place_launched_twice(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Waiting);
    let id = fix_for_ops(&world, "twice")?;
    let first = world
        .held(&id)
        .and_then(|held| held.place.token)
        .ok_or("the first launch took no token")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    world.confirm_every("ops");
    world.engine().settle(None);
    ensure!(
        world.held(&id).is_none() && tidied(&world).is_empty(),
        "the first place, landed with nothing pending, stands"
    );
    let first_key = format!("fix--twice/{first}/stage/code/1/delivered");
    ensure!(
        world
            .terminal
            .notices("ops")
            .iter()
            .any(|notice| notice.ends_with(&format!("; confirm: flume confirm {first_key}"))),
        "no notice of the first code end names {first_key}: {:?}",
        world.terminal.notices("ops")
    );
    world.terminal.remove_session("ops");
    let id = fix_for_ops(&world, "twice")?;
    let second = world
        .held(&id)
        .and_then(|held| held.place.token)
        .ok_or("the second launch took no token")?;
    ensure!(
        a_token(&first) && a_token(&second) && first != second,
        "the two launches' tokens: {first} and {second}"
    );
    deliver_fix(&world, &id);
    world.settle(&id);
    let second_key = format!("fix--twice/{second}/stage/code/1/delivered");
    let delivery = delivery_under(&world, &second_key)
        .ok_or(format!("no delivery recorded under {second_key}"))?;
    ensure_eq!(
        delivery
            .readers()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        vec!["ops".to_string()],
        "the second delivery's readers"
    );
    ensure_eq!(
        owed_to(&world, &second_key)?,
        vec!["ops".to_string()],
        "whom the second delivery is owed to"
    );
    Ok(())
}

/// The deliveries spec's "A place written before the token": its settle's
/// write gives it one, and its ends' deliveries are keyed by it (43, 75).
fn a_place_written_before_the_token(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "before")?;
    world.engine.update(&id, |held| {
        held.place.token = None;
        flume_core::Step::Write(())
    });
    ensure!(
        world.held(&id).ok_or("no place")?.place.token.is_none(),
        "the document holds a token"
    );
    deliver_fix(&world, &id);
    world.settle(&id);
    let token = world
        .held(&id)
        .ok_or("no place")?
        .place
        .token
        .ok_or("the settle gave the document no token")?;
    ensure!(a_token(&token), "the token given: {token}");
    let key = format!("fix--before/{token}/stage/code/1/delivered");
    ensure!(
        delivery_under(&world, &key).is_some(),
        "no delivery under {key}: {:?}",
        world.deliveries()
    );
    Ok(())
}

/// The fix `one` for `ops`, its submission held, its code delivered and
/// handed to `ops` by one notice.
fn handed_to_ops(world: &World) -> Result<PlaceId, String> {
    world.terminal.add_session("ops", AgentState::Waiting);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(world, "one")?;
    deliver_fix(world, &id);
    world.settle(&id);
    ensure_eq!(world.terminal.notices("ops").len(), 1, "notices to ops");
    Ok(id)
}

/// The end-delivery spec's "Handed and stopped before acting" (43, S122;
/// weir 103).
fn handed_and_stopped_before_acting(_: &()) -> Outcome {
    let world = World::new();
    let id = handed_to_ops(&world)?;
    world.terminal.remove_session("ops");
    world.terminal.add_session("ops", AgentState::Waiting);
    for _ in 0..3 {
        world.host.advance(60);
        world.settle(&id);
    }
    ensure_eq!(
        world.terminal.notices("ops").len(),
        1,
        "notices to ops after it was cleared"
    );
    let end = end_keyed(&world, &id, "stage/code/1/delivered")?;
    ensure_eq!(
        deliveries(&end.end),
        vec![("ops".to_string(), DeliveredBy::Notice)],
        "code's hand-overs"
    );
    ensure_eq!(end.pending_for, vec!["ops".to_string()], "pending for");
    ensure_eq!(
        collected(&world.engine().inbox("ops", None, false))?,
        vec!["fix--one code delivered".to_string()],
        "what ops's next inbox gave"
    );
    Ok(())
}

/// The end-delivery spec's "A notice claimed and not recorded" (43, 75).
fn a_notice_claimed_and_not_recorded(_: &()) -> Outcome {
    let world = World::new();
    world.terminal.add_session("ops", AgentState::Asking);
    world.queue.plan_submits(vec![Plan::Hold]);
    let id = fix_for_ops(&world, "one")?;
    deliver_fix(&world, &id);
    world.settle(&id);
    ensure!(
        world.terminal.notices("ops").is_empty(),
        "a notice sent into a question"
    );
    world.terminal.set_session_state("ops", AgentState::Waiting);
    // The claim is the settle's first write of the place; the host's
    // deliveries refuse every write from it until the settle is over.
    let store = world.store.clone();
    world
        .store
        .before_next_write(move || store.refuse_host_writes(true));
    world.host.advance(30);
    let claimed_at = world.host.now();
    world.settle(&id);
    world.store.refuse_host_writes(false);
    ensure_eq!(world.terminal.notices("ops").len(), 1, "notices sent");
    ensure!(
        world
            .store
            .log
            .of("write_host")
            .iter()
            .any(|write| write.ends_with("refused")),
        "the hand-over's write was not refused"
    );
    ensure!(
        end_keyed(&world, &id, "stage/code/1/delivered")?
            .end
            .deliveries
            .is_empty(),
        "a hand-over recorded while the deliveries refused writes"
    );
    world.host.advance(30);
    world.settle(&id);
    ensure_eq!(world.terminal.notices("ops").len(), 1, "notices sent again");
    let end = end_keyed(&world, &id, "stage/code/1/delivered")?;
    ensure_eq!(
        end.end
            .deliveries
            .iter()
            .map(|delivery| (delivery.recipient.as_str(), delivery.at, delivery.by))
            .collect::<Vec<_>>(),
        vec![("ops", claimed_at, DeliveredBy::Notice)],
        "the hand-over recorded at the claim's time"
    );
    Ok(())
}

/// The version of the host's deliveries document, none where none is.
fn deliveries_version(world: &World) -> Option<u64> {
    world
        .store
        .read_host(&HostKey::Deliveries)
        .map(|read| read.version)
}

/// The end-delivery spec's "Confirmed twice" (43; weir 102).
fn confirmed_twice(_: &()) -> Outcome {
    let world = World::new();
    let id = handed_to_ops(&world)?;
    let key = world
        .delivery_key("fix--one", "stage/code/1/delivered")
        .ok_or("no delivery of code")?;
    world.host.advance(60);
    let first_at = world.host.now();
    let first = world.engine().confirm("ops", std::slice::from_ref(&key));
    ensure_eq!(
        first,
        Answer::Confirmed {
            recipient: "ops".into(),
            confirmed: vec![flume_core::KeyConfirmed {
                key: key.clone(),
                at: first_at,
                before: false,
            }],
        },
        "the first confirmation"
    );
    ensure_eq!(first.exit(), 0, "the first's exit");
    ensure!(
        end_keyed(&world, &id, "stage/code/1/delivered")?
            .pending_for
            .is_empty(),
        "code pending once ops confirmed it"
    );
    let version = deliveries_version(&world);
    world.host.advance(60);
    let second = world.engine().confirm("ops", std::slice::from_ref(&key));
    ensure_eq!(
        second,
        Answer::Confirmed {
            recipient: "ops".into(),
            confirmed: vec![flume_core::KeyConfirmed {
                key,
                at: first_at,
                before: true,
            }],
        },
        "the second confirmation"
    );
    ensure_eq!(
        deliveries_version(&world),
        version,
        "the deliveries written by the second"
    );
    Ok(())
}

/// The end-delivery spec's "A key of another recipient's" (43; weir 102).
fn a_key_of_another_recipients(_: &()) -> Outcome {
    let world = World::new();
    world.queue.plan_submits(vec![Plan::Hold]);
    let mut launch = launch("fix", "theirs", Some("mend it"));
    launch.recipients = vec!["boomdrive".into()];
    launched(&world.engine.launch(&launch))?;
    let id = World::id("fix", "theirs");
    world.settle(&id);
    deliver_fix(&world, &id);
    world.settle(&id);
    let key = world
        .delivery_key("fix--theirs", "stage/code/1/delivered")
        .ok_or("no delivery of code")?;
    let version = deliveries_version(&world);
    let theirs = world.engine().confirm("ops", std::slice::from_ref(&key));
    let reason = refused(&theirs)?;
    ensure!(
        reason.contains(&key) && reason.contains("`reader`"),
        "the refusal of boomdrive's key: {reason}"
    );
    ensure_eq!(theirs.exit(), 1, "the refusal's exit");
    let none = world.engine().confirm("ops", &["no/such/key".to_string()]);
    let reason = refused(&none)?;
    ensure!(
        reason.contains("no/such/key") && reason.contains("`key`"),
        "the refusal of a key no end is under: {reason}"
    );
    ensure_eq!(
        deliveries_version(&world),
        version,
        "the deliveries written by a refusal"
    );
    ensure_eq!(
        end_keyed(&world, &id, "stage/code/1/delivered")?.pending_for,
        vec!["boomdrive".to_string()],
        "code's pending"
    );
    Ok(())
}

/// A fix for the misspelt `bomdrive`, landed and tidied with nothing
/// collected, the fake host's clock a week on and the host settled (43,
/// S123).
fn misspelt_and_expired(world: &World) -> Result<PlaceId, String> {
    let id = landed_with(world, "one", &["bomdrive"])?;
    world.host.advance(ENDS_EXPIRE);
    world.engine().settle(None);
    Ok(id)
}

/// The end-delivery spec's "A misspelt recipient" (43, 56, S123; weir
/// 104, 106).
fn a_misspelt_recipient(_: &()) -> Outcome {
    let world = World::new();
    let id = misspelt_and_expired(&world)?;
    let status = world.status(&id)?;
    ensure_eq!(status.ends.len(), 2, "the ends");
    for end in &status.ends {
        ensure_eq!(
            end.owed,
            vec![OwedTo {
                recipient: "bomdrive".into(),
                expired: true,
                known: false,
            }],
            format!("whom {} is owed to", end.key)
        );
        ensure!(
            end.expires_at.is_some_and(|at| at <= world.host.now()),
            "{}'s expiry: {:?}",
            end.key,
            end.expires_at
        );
    }
    let Answer::Status { host, .. } = world.engine().status(None) else {
        return Err("the host's status was refused".into());
    };
    let mut listed: Vec<(String, String, bool)> = host
        .expired
        .iter()
        .map(|end| (end.key.to_string(), end.recipient.clone(), end.known))
        .collect();
    listed.sort();
    ensure_eq!(
        listed,
        vec![
            (
                "stage/code/1/delivered".to_string(),
                "bomdrive".to_string(),
                false
            ),
            (
                "submission/1/landed".to_string(),
                "bomdrive".to_string(),
                false
            ),
        ],
        "the host's expired ends"
    );
    ensure_eq!(tidied(&world).len(), 1, "the tidied documents");
    let reason = refused(&world.engine().drop_place(&id, false))?;
    ensure!(reason.contains("bomdrive"), "the drop's refusal: {reason}");
    Ok(())
}

/// The end-delivery spec's "Confirmed once expired" (43; weir 104, 106,
/// 107).
fn confirmed_once_expired(_: &()) -> Outcome {
    let world = World::new();
    let id = misspelt_and_expired(&world)?;
    let keys: Vec<String> = world
        .status(&id)?
        .ends
        .iter()
        .filter_map(|end| end.delivery.clone())
        .collect();
    ensure_eq!(keys.len(), 2, "the delivery keys");
    let answer = world.engine().confirm("bomdrive", &keys);
    ensure!(
        matches!(&answer, Answer::Confirmed { confirmed, .. } if confirmed.len() == 2),
        "the confirmation: {answer:?}"
    );
    ensure!(
        world
            .status(&id)?
            .ends
            .iter()
            .all(|end| end.pending_for.is_empty() && end.confirmed.len() == 1),
        "the ends once confirmed"
    );
    ensure!(
        world.owing().known.contains("bomdrive"),
        "bomdrive not known once it confirmed"
    );
    world.engine().settle(None);
    ensure!(tidied(&world).is_empty(), "the document once delivered");
    ensure!(
        world.owing().known.contains("bomdrive"),
        "bomdrive's name let go of with its deliveries"
    );
    Ok(())
}

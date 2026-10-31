//! A place through its whole life (10, 13, 14, 34): its own branch, only
//! its readiness, teardown and agents run in it, no branch moved by flume
//! but its own, no question answered for its agent, and a place kept
//! where it was made though the place seam now makes one elsewhere (3,
//! 76; record 0027).

use super::world::*;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "places: no place takes its base's branch, and no two live places share one",
            cites: &["10", "3", "9"],
            run: a_branch_of_its_own,
        },
        Scenario {
            name: "places: only readiness, teardown and the stages' agents run in a place",
            cites: &["13", "11", "29"],
            run: only_readiness_teardown_and_agents,
        },
        Scenario {
            name: "places: flume moves no branch but the place's own, made at its launch and removed at its tidy",
            cites: &["14", "I3", "71"],
            run: no_branch_moved_but_its_own,
        },
        Scenario {
            name: "places: an agent asking a person is answered nothing by flume",
            cites: &["34", "37", "53", "72"],
            run: no_question_answered,
        },
        Scenario {
            name: "places: a place made where the place seam no longer makes one is read, submitted and tidied where it stands, and a launch of its kind and name is refused",
            cites: &["3", "76", "77", "64", "111"],
            run: an_earlier_place_where_it_stands,
        },
    ]
}

fn a_branch_of_its_own(_: &()) -> Outcome {
    let world = World::new();
    let one = launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let two = launched(&world.engine.launch(&launch("fix", "two", Some("x"))))?;
    ensure!(one.branch != two.branch, "two places share {}", one.branch);
    ensure!(
        one.branch != one.base && two.branch != two.base,
        "a place took its base's branch"
    );
    world.kinds.set_kinds(
        REPOSITORY,
        vec![{
            let mut same = kind(
                "mend",
                true,
                vec![stage("code", Some(Deliverable::Commits), "{{brief}}")],
            );
            same.branch_prefix = "fix".into();
            same
        }],
    );
    let reason = refused(&world.engine.launch(&launch("mend", "one", Some("x"))))?;
    ensure!(
        reason.contains("fix/one"),
        "the refusal names not the branch: {reason}"
    );
    ensure_eq!(world.place.log.count("make"), 3, "makes");
    ensure_eq!(world.store.places().len(), 2, "places held");
    Ok(())
}

fn only_readiness_teardown_and_agents(_: &()) -> Outcome {
    let world = World::new();
    world
        .place
        .set_readiness(REPOSITORY, &[("toolchain", true, "ok")]);
    let id = World::id("fix", "one");
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    world.settle(&id);
    world.commit("fix/one", &["fix.rs"]);
    world.settle(&id);
    ensure!(world.held(&id).is_none(), "the place did not land");
    let run_in_place: Vec<String> = world
        .place
        .log
        .calls()
        .into_iter()
        .filter(|call| call.contains(&id.0))
        .map(|call| call.split(' ').next().unwrap_or_default().to_string())
        .collect();
    ensure_eq!(
        run_in_place,
        vec!["make", "ready", "read", "remove"],
        "what flume ran in the place"
    );
    let opened = world.terminal.log.of("open");
    ensure_eq!(opened.len(), 1, "agents opened");
    let (command, _) = world
        .terminal
        .opened(&World::identity("fix", "one", "code", 1))
        .ok_or("no agent")?;
    ensure_eq!(command.program, "agent", "the program run");
    Ok(())
}

fn no_branch_moved_but_its_own(_: &()) -> Outcome {
    let world = World::new();
    let base = world.git.branch_head(REPOSITORY, "main").ok_or("main")?;
    let id = World::id("fix", "one");
    let mut fix = launch("fix", "one", Some("x"));
    fix.hold_submission = true;
    launched(&world.engine.launch(&fix))?;
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "fix/one"),
        Some(base.clone()),
        "the place's branch at its launch"
    );
    world.settle(&id);
    let work = world.commit("fix/one", &["fix.rs"]);
    world.settle(&id);
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "main"),
        Some(base.clone()),
        "the base before the merge queue lands"
    );
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "fix/one"),
        Some(work.clone()),
        "the place's branch, as its agent left it"
    );
    ensure_eq!(world.queue.log.count("submit"), 0, "submissions while held");
    let answer = world
        .engine
        .retry(&id, flume_core::Retry::Submission, None, false);
    ensure!(
        answer.refusal().is_none(),
        "the retry was refused: {answer:?}"
    );
    ensure!(
        world.git.branch_head(REPOSITORY, "main") != Some(base),
        "the merge queue did not land"
    );
    ensure_eq!(
        world.git.branch_head(REPOSITORY, "fix/one"),
        None,
        "the place's branch after its tidy"
    );
    ensure_eq!(
        world.queue.log.count("submit"),
        1,
        "submissions, flume's alone"
    );
    Ok(())
}

fn no_question_answered(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let code = World::identity("fix", "one", "code", 1);
    world.terminal.comes_up(AgentState::Asking);
    world.agent.set_asking(&code, Reading::Yes);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    for _ in 0..5 {
        world.host.advance(60);
        world.engine().settle(None);
    }
    let _ = world.engine.tell(&id, "yes, go ahead", Some(30));
    ensure!(
        world.terminal.sent_to(&code).is_empty(),
        "flume typed into a question: {:?}",
        world.terminal.sent_to(&code)
    );
    ensure_eq!(
        world.stage_state(&id, "code"),
        Some(StageState::Started),
        "the code stage"
    );
    Ok(())
}

fn an_earlier_place_where_it_stands(_: &()) -> Outcome {
    let world = World::new();
    world.place.locate_inside(true);
    launched(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    let earlier = PlaceId::new(format!("{REPOSITORY}/places/fix--one"));
    ensure!(world.held(&earlier).is_some(), "no place held at {earlier}");
    world.place.locate_inside(false);
    let now = World::id("fix", "one");
    ensure!(now != earlier, "the seam gives the earlier location");
    let reason = refused(&world.engine.launch(&launch("fix", "one", Some("x"))))?;
    ensure!(
        reason.contains("fix/one"),
        "the refusal names not the branch: {reason}"
    );
    ensure!(world.held(&now).is_none(), "a second place was held");
    ensure!(!world.place.exists(&now.0), "a second place was made");
    world.settle(&earlier);
    world.commit("fix/one", &["Cargo.toml"]);
    world.settle(&earlier);
    ensure_eq!(world.queue.log.count("submit"), 1, "submissions");
    ensure!(
        world.held(&earlier).is_none(),
        "held state of the landed place is kept"
    );
    ensure!(!world.place.exists(&earlier.0), "the earlier place stands");
    ensure_eq!(
        world.place.log.of("remove"),
        vec![format!("remove {}", earlier.0)],
        "removes"
    );
    Ok(())
}

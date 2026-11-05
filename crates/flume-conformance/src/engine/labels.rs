//! A place's label (180; core.46): the one the host's configuration
//! gives its repository's kind, else its kind, else its kind's own, else
//! none, read at each start and held on it, so an agent counts toward the
//! label it started under until it ends (32). Status gives each place's
//! label as it would resolve now, with where it came from (56).

use super::settling::{construct_begun, write_change};
use super::world::*;
use crate::{ensure_eq, Outcome, Scenario};
use flume_fakes::place_location;
use flume_seams::*;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "labels: a place's label is the host's for its repository's kind, else for its kind, else its kind's own, each with where it came from",
            cites: &["S85", "180", "56"],
            run: each_source_of_a_label,
        },
        Scenario {
            name: "labels: a label changed between starts leaves the start that ran under the label it started with and the next start under the new one",
            cites: &["32", "180"],
            run: a_label_changed_between_starts,
        },
    ]
}

/// The second repository, which declares `deploy` as the first does.
const SECOND: &str = "/s";

/// A one-stage kind `name` that does not land, so it needs no merge queue.
fn work_kind(name: &str) -> Kind {
    kind(
        name,
        false,
        vec![stage("work", Some(Deliverable::Commits), "{{brief}}")],
    )
}

fn held(name: &str, from: LabelFrom) -> Option<LabelHeld> {
    Some(LabelHeld {
        name: name.into(),
        from,
    })
}

/// The label the start under `identity` holds that it started under.
fn held_label(
    world: &World,
    id: &PlaceId,
    identity: &Identity,
) -> Result<Option<LabelHeld>, String> {
    let state = world.held(id).ok_or("no held state")?;
    let start = state
        .stages
        .iter()
        .filter_map(|stage| stage.start.as_ref())
        .find(|start| &start.identity == identity)
        .ok_or_else(|| format!("no start {identity}"))?;
    Ok(start.label.clone())
}

fn each_source_of_a_label(_: &()) -> Outcome {
    let world = World::new();
    world.git.init(SECOND, "main");
    world
        .kinds
        .set_kinds(REPOSITORY, vec![work_kind("research"), work_kind("deploy")]);
    world.kinds.set_kinds(SECOND, vec![work_kind("deploy")]);
    world.kinds.set_label("deploy", Some("operation"));
    world.host.name_label(None, "research", Some("inception"));
    world
        .host
        .name_label(Some(REPOSITORY), "deploy", Some("construction"));
    let mut in_second = launch("deploy", "two", Some("x"));
    in_second.repository = SECOND.into();
    let cases = [
        (
            launch("research", "one", Some("x")),
            World::id("research", "one"),
            held("inception", LabelFrom::HostKind),
            "a research place",
        ),
        (
            launch("deploy", "one", Some("x")),
            World::id("deploy", "one"),
            held("construction", LabelFrom::HostRepositoryKind),
            "a deploy place in the repository",
        ),
        (
            in_second,
            PlaceId::new(place_location(SECOND, "deploy--two")),
            held("operation", LabelFrom::Kind),
            "a deploy place in /s",
        ),
    ];
    for (launch, id, label, what) in cases {
        let place = launched(&world.engine.launch(&launch))?;
        ensure_eq!(
            place.label,
            label,
            format!("{what}'s label in the launch's answer")
        );
        ensure_eq!(
            world.status(&id)?.label,
            label,
            format!("{what}'s label in status")
        );
        let start = world
            .held(&id)
            .and_then(|held| held.stages[0].start.clone())
            .ok_or_else(|| format!("{what} did not start"))?;
        ensure_eq!(start.label, label, format!("{what}'s start's label"));
    }
    Ok(())
}

fn a_label_changed_between_starts(_: &()) -> Outcome {
    let world = World::new();
    world.host.name_label(None, "change", Some("construction"));
    let id = construct_begun(&world, "one")?;
    let construct = World::identity("change", "one", "construct", 1);
    world.host.name_label(None, "change", Some("operation"));
    world.settle(&id);
    ensure_eq!(
        held_label(&world, &id, &construct)?,
        held("construction", LabelFrom::HostKind),
        "the construct start's label once the kind is operation's"
    );
    ensure_eq!(
        world.status(&id)?.label,
        held("operation", LabelFrom::HostKind),
        "the place's label as it would resolve now"
    );
    write_change(&world, "one");
    world.settle(&id);
    world.settle(&id);
    let code = World::identity("change", "one", "code", 1);
    ensure_eq!(
        held_label(&world, &id, &code)?,
        held("operation", LabelFrom::HostKind),
        "the code start's label"
    );
    Ok(())
}

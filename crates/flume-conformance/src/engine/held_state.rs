//! Held state: one versioned document per place, written by
//! compare-and-set (73 to 77, 102, 103).

use super::world::*;
use crate::scenario::sample_held;
use crate::{ensure, ensure_eq, Outcome, Scenario};
use flume_core::Step;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub fn scenarios() -> Vec<Scenario<()>> {
    vec![
        Scenario {
            name: "held state: a write refused because the version moved is read and decided again",
            cites: &["75", "103"],
            run: a_refused_write_is_decided_again,
        },
        Scenario {
            name: "held state: a first write over a place's held state is refused",
            cites: &["75", "103", "3"],
            run: a_second_first_write,
        },
    ]
}

fn a_refused_write_is_decided_again(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    ensure!(
        world.engine.create(&sample_held(&id, REPOSITORY)).is_some(),
        "the first write was refused"
    );
    let decisions = Arc::new(AtomicUsize::new(0));
    let other = world.engine();
    let other_id = id.clone();
    world.store.before_next_write(move || {
        other.update(&other_id, |held| {
            held.place.mark = Some(format!("{}b", held.place.mark.clone().unwrap_or_default()));
            Step::Write(())
        });
    });
    let counted = decisions.clone();
    world.engine.update(&id, |held| {
        counted.fetch_add(1, Ordering::SeqCst);
        held.place.mark = Some(format!("{}a", held.place.mark.clone().unwrap_or_default()));
        Step::Write(())
    });
    ensure_eq!(decisions.load(Ordering::SeqCst), 2, "the act's decisions");
    ensure_eq!(
        world.held(&id).and_then(|held| held.place.mark),
        Some("ba".to_string()),
        "the mark the two acts left"
    );
    Ok(())
}

fn a_second_first_write(_: &()) -> Outcome {
    let world = World::new();
    let id = World::id("fix", "one");
    let mut held = sample_held(&id, REPOSITORY);
    ensure!(
        world.engine.create(&held).is_some(),
        "the first write was refused"
    );
    held.place.mark = Some("second".into());
    ensure!(
        world.engine.create(&held).is_none(),
        "a second first write was taken"
    );
    ensure_eq!(
        world.held(&id).and_then(|held| held.place.mark),
        None,
        "the held mark"
    );
    Ok(())
}

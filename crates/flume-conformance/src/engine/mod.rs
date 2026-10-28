//! The engine's scenarios over the fakes, each named for its scenario of
//! section 11 where it has one, and each citing its clauses as data.

pub mod answers;
pub mod caller_acts;
pub mod held_state;
pub mod host_queue;
pub mod launching;
pub mod place_status;
pub mod places;
pub mod restart;
pub mod running_through;
pub mod settling;
pub mod stage_runs;
pub mod starts;
pub mod tidying;
pub mod world;

use crate::Scenario;

/// Every engine scenario.
pub fn scenarios() -> Vec<Scenario<()>> {
    let mut every = vec![];
    every.extend(held_state::scenarios());
    every.extend(launching::scenarios());
    every.extend(settling::scenarios());
    every.extend(running_through::scenarios());
    every.extend(stage_runs::scenarios());
    every.extend(starts::scenarios());
    every.extend(caller_acts::scenarios());
    every.extend(place_status::scenarios());
    every.extend(tidying::scenarios());
    every.extend(restart::scenarios());
    every.extend(answers::scenarios());
    every.extend(places::scenarios());
    every.extend(host_queue::scenarios());
    every
}

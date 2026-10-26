//! The engine's scenarios over the fakes, each named for its scenario of
//! section 11 where it has one, and each citing its clauses as data.

pub mod held_state;
pub mod world;

use crate::Scenario;

/// Every engine scenario.
pub fn scenarios() -> Vec<Scenario<()>> {
    let mut every = vec![];
    every.extend(held_state::scenarios());
    every
}

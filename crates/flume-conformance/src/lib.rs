//! flume's conformance suite (106): the seam scenarios, each generic over
//! its seam's trait so a profile unit runs them against its real tool as
//! the fakes pass them here, and the engine's scenarios over the fakes.
//! Every scenario carries the clauses it cites as data, which the
//! coverage check reads.

pub mod clauses;
pub mod coverage;
pub mod fake_harness;
pub mod no_tool;
pub mod scenario;
pub mod seams;
pub mod suite;

pub use scenario::{Outcome, Scenario};

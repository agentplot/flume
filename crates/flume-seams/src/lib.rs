//! The eight seams of Part B of flume's statement, one trait each, the
//! types they pass, and git's history, the host's facts and the
//! telemetry the core hands its signals to beside them (section 9, 216).
//! A profile depends on this crate alone, so it implements its interface
//! and nothing of the core (core.2), and no type or trait here names a
//! tool (core.1).

mod agent;
mod change;
mod held;
mod history;
mod host;
mod kinds;
mod launch;
mod merge_queue;
mod place;
mod prompt;
mod resolver;
mod store;
pub mod telemetry;
mod terminal;
mod window;

pub use agent::*;
pub use change::*;
pub use held::*;
pub use history::*;
pub use host::*;
pub use kinds::*;
pub use launch::*;
pub use merge_queue::*;
pub use place::*;
pub use prompt::*;
pub use resolver::*;
pub use store::*;
pub use telemetry::{
    Attributes, HostReading, PlacesStanding, Sends, Signal, StreamAgents, Telemetry,
};
pub use terminal::*;
pub use window::*;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A commit of a repository, by its id.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Revision(pub String);

impl Revision {
    pub fn new(id: impl Into<String>) -> Self {
        Revision(id.into())
    }
}

impl fmt::Display for Revision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A place, by the location the place profile made it at (C.1), which
/// names one place on the host.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlaceId(pub String);

impl PlaceId {
    pub fn new(location: impl Into<String>) -> Self {
        PlaceId(location.into())
    }
}

impl fmt::Display for PlaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// A moment by the host's clock, in seconds.
pub type Seconds = u64;

/// A span of time as a person writes it, `<n>`, `<n>s`, `<n>m` or `<n>h`,
/// `<n>` digits, in seconds: a launch's stall bound and an agent's time
/// kept open for an answer are written so (130, 198). Any other form is
/// refused in words naming the four.
pub fn read_span(given: &str) -> Result<Seconds, String> {
    let (number, scale) = match given.as_bytes().last() {
        Some(b's') => (&given[..given.len() - 1], 1),
        Some(b'm') => (&given[..given.len() - 1], 60),
        Some(b'h') => (&given[..given.len() - 1], 60 * 60),
        _ => (given, 1),
    };
    Some(number)
        .filter(|number| !number.is_empty() && number.bytes().all(|digit| digit.is_ascii_digit()))
        .and_then(|number| number.parse::<Seconds>().ok())
        .and_then(|number| number.checked_mul(scale))
        .ok_or_else(|| format!("{given} is no time: it is <n>, <n>s, <n>m or <n>h"))
}

#[cfg(test)]
mod round_trip;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_span_in_each_of_its_four_forms() {
        assert_eq!(read_span("90"), Ok(90));
        assert_eq!(read_span("90s"), Ok(90));
        assert_eq!(read_span("10m"), Ok(600));
        assert_eq!(read_span("1h"), Ok(3600));
        assert_eq!(read_span("0"), Ok(0), "no time at all");
    }

    #[test]
    fn a_span_in_none_of_them() {
        for given in [
            "soon", "5d", "", "s", "1.5h", "-1m", "+5", " 5", "5 m", "1h30m", "1H",
        ] {
            assert_eq!(
                read_span(given),
                Err(format!("{given} is no time: it is <n>, <n>s, <n>m or <n>h")),
                "{given:?}"
            );
        }
        assert!(read_span(&format!("{}h", u64::MAX)).is_err(), "past a u64");
    }
}

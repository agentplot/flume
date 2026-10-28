//! The host's own facts the core reads: its clock, a new secret, a
//! file's write time, and whether a command of flume's still lives
//! (design 3). No seam of Part B.

use crate::{Seconds, Secret};

/// The host's facts (design 3).
pub trait Host: Send + Sync {
    /// The host's clock.
    fn now(&self) -> Seconds;

    /// A wait inside a command of flume's (38, 54).
    fn sleep(&self, seconds: Seconds);

    /// A secret no other start has (41).
    fn secret(&self) -> Secret;

    /// A file's write time by the host's clock, if it exists (25).
    fn write_time(&self, path: &str) -> Option<Seconds>;

    /// This command of flume's, as held state names a waiter (124).
    fn command(&self) -> String;

    /// Whether a command of flume's named in held state still lives (124).
    fn alive(&self, command: &str) -> bool;
}

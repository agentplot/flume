//! The host's own facts the core reads: its clock, a new secret, a
//! file's write time, whether a command of flume's still lives, the bound
//! on agents its configuration states, and a lock over the room (design
//! 3, 6). No seam of Part B.

use crate::{Seconds, Secret};

/// The host's room lock, held while this guard lives and released when
/// it is dropped (70, 75).
pub struct RoomGuard(#[allow(dead_code)] Box<dyn Send>);

impl RoomGuard {
    /// A guard that keeps the lock as long as `held` lives.
    pub fn new(held: impl Send + 'static) -> RoomGuard {
        RoomGuard(Box::new(held))
    }
}

impl std::fmt::Debug for RoomGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RoomGuard")
    }
}

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

    /// This command of flume's, as held state names the launch making a
    /// place (2, 76).
    fn command(&self) -> String;

    /// Whether a command of flume's named in held state still lives (76).
    fn alive(&self, command: &str) -> bool;

    /// The bound on agents running that the host's configuration states,
    /// every caller's counted together; none where it states none, and
    /// then the host keeps no queue (70).
    fn bound(&self) -> Option<u32>;

    /// The host-wide lock over the room, held until the guard is dropped,
    /// so two starts at once never pass the bound (70, 75).
    fn room(&self) -> RoomGuard;
}

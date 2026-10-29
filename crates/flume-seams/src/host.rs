//! The host's own facts the core reads: its clock, a new secret, a
//! file's write time, whether a command of flume's still lives, the bound
//! on agents its configuration states, and a lock over the room (design
//! 3, 6). No seam of Part B.

use crate::{KindSettings, Seconds, Secret};
use serde::{Deserialize, Serialize};

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

/// The settings the host's configuration gives a kind's agents, each value
/// as the configuration wrote it (32, C.7).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostSettings {
    /// For the kind in one repository.
    pub repository: KindSettings,
    /// For the kind in every repository.
    pub kind: KindSettings,
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

    /// The recipient the calling command's environment names in
    /// `FLUME_RECIPIENT`, a launch's default recipient; none where it
    /// names none (7).
    fn recipient(&self) -> Option<String>;

    /// The settings the host's configuration gives `kind`'s agents in
    /// `repository`, as it stands now: read at every start, never kept
    /// in held state (32, C.7).
    fn settings(&self, repository: &str, kind: &str) -> HostSettings;

    /// A variable of the host user's login environment, `HOME` among
    /// them, never of the calling command's session; none where it lacks
    /// it (29).
    fn login(&self, name: &str) -> Option<String>;

    /// A line written to the host's log, for what fails nothing, as a
    /// clue that failed to show (139).
    fn log(&self, line: &str);

    /// The host-wide lock over the room, held until the guard is dropped,
    /// so two starts at once never pass the bound (70, 75).
    fn room(&self) -> RoomGuard;
}

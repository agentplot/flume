use crate::{lock, Log};
use flume_seams::{Host, RoomGuard, Seconds, Secret};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Condvar, Mutex};

/// The host: a clock that moves only when a test advances it or a command
/// sleeps on it, numbered secrets, files' write times, the commands of
/// flume's that live, the bound on agents its configuration states, and
/// an exclusive room lock.
pub struct FakeHost {
    state: Mutex<HostState>,
    room: Arc<Room>,
    pub log: Log,
}

/// The room lock: taken while held, waited on by every other taker.
#[derive(Default)]
struct Room {
    taken: Mutex<bool>,
    freed: Condvar,
}

/// Holding the room until dropped.
struct Held(Arc<Room>);

impl Drop for Held {
    fn drop(&mut self) {
        *lock(&self.0.taken) = false;
        self.0.freed.notify_one();
    }
}

struct HostState {
    now: Seconds,
    secrets: u64,
    files: BTreeMap<String, Seconds>,
    command: u64,
    alive: BTreeSet<String>,
    bound: Option<u32>,
}

impl Default for FakeHost {
    fn default() -> Self {
        FakeHost {
            state: Mutex::new(HostState {
                now: 1_000_000,
                secrets: 0,
                files: BTreeMap::new(),
                command: 1,
                alive: BTreeSet::from(["command-1".to_string()]),
                bound: None,
            }),
            room: Arc::default(),
            log: Log::default(),
        }
    }
}

impl FakeHost {
    pub fn advance(&self, seconds: Seconds) {
        lock(&self.state).now += seconds;
    }

    /// A file written at the clock's now.
    pub fn write_file(&self, path: &str) {
        let mut state = lock(&self.state);
        let now = state.now;
        state.files.insert(path.to_string(), now);
    }

    pub fn remove_file(&self, path: &str) {
        lock(&self.state).files.remove(path);
    }

    /// The host restarts: every command of flume's that ran is gone, and
    /// the next runs under a new name.
    pub fn restart(&self) {
        let mut state = lock(&self.state);
        state.alive.clear();
        state.command += 1;
        let next = format!("command-{}", state.command);
        state.alive.insert(next);
    }

    /// Another command of flume's than this one, live.
    pub fn another_command(&self) -> String {
        let mut state = lock(&self.state);
        state.command += 1;
        let name = format!("command-{}", state.command);
        state.alive.insert(name.clone());
        name
    }

    pub fn end_command(&self, command: &str) {
        lock(&self.state).alive.remove(command);
    }

    /// The bound on agents the host's configuration states, or none.
    pub fn set_bound(&self, bound: Option<u32>) {
        lock(&self.state).bound = bound;
    }

    /// Whether the room lock is held now.
    pub fn room_taken(&self) -> bool {
        *lock(&self.room.taken)
    }
}

impl Host for FakeHost {
    fn now(&self) -> Seconds {
        lock(&self.state).now
    }

    fn sleep(&self, seconds: Seconds) {
        self.log.push(format!("sleep {seconds}"));
        self.advance(seconds);
    }

    fn secret(&self) -> Secret {
        let mut state = lock(&self.state);
        state.secrets += 1;
        Secret(format!("secret-{}", state.secrets))
    }

    fn write_time(&self, path: &str) -> Option<Seconds> {
        self.log.push(format!("write_time {path}"));
        lock(&self.state).files.get(path).copied()
    }

    fn command(&self) -> String {
        format!("command-{}", lock(&self.state).command)
    }

    fn alive(&self, command: &str) -> bool {
        lock(&self.state).alive.contains(command)
    }

    fn bound(&self) -> Option<u32> {
        self.log.push("bound");
        lock(&self.state).bound
    }

    fn room(&self) -> RoomGuard {
        self.log.push("room");
        let mut taken = lock(&self.room.taken);
        while *taken {
            taken = self
                .room
                .freed
                .wait(taken)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
        *taken = true;
        RoomGuard::new(Held(self.room.clone()))
    }
}

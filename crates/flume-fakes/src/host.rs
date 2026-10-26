use crate::{lock, Log};
use flume_seams::{Host, Seconds, Secret};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

/// The host: a clock that moves only when a test advances it or a command
/// sleeps on it, numbered secrets, files' write times, and the commands
/// of flume's that live.
pub struct FakeHost {
    state: Mutex<HostState>,
    pub log: Log,
}

struct HostState {
    now: Seconds,
    secrets: u64,
    files: BTreeMap<String, Seconds>,
    command: u64,
    alive: BTreeSet<String>,
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
            }),
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
}

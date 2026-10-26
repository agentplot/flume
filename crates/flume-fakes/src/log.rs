use crate::lock;
use std::sync::Mutex;

/// Every call made of a fake, in order, one line each: the operation's
/// name, then what it was given.
#[derive(Default)]
pub struct Log(Mutex<Vec<String>>);

impl Log {
    pub fn push(&self, line: impl Into<String>) {
        lock(&self.0).push(line.into());
    }

    pub fn calls(&self) -> Vec<String> {
        lock(&self.0).clone()
    }

    /// The calls of one operation.
    pub fn of(&self, operation: &str) -> Vec<String> {
        lock(&self.0)
            .iter()
            .filter(|line| line.split(' ').next() == Some(operation))
            .cloned()
            .collect()
    }

    pub fn count(&self, operation: &str) -> usize {
        self.of(operation).len()
    }

    pub fn clear(&self) {
        lock(&self.0).clear();
    }
}

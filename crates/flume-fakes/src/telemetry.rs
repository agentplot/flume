//! A fake of the telemetry the core hands its signals to: a test sets
//! which signals it sends, none by default, and reads back every signal
//! the core handed it, in order, whatever it sends (216, 217).

use crate::lock;
use flume_seams::{Sends, Signal, Telemetry};
use std::sync::Mutex;

/// Keeps every signal it is handed, in order, and sends what a test set.
#[derive(Default)]
pub struct FakeTelemetry {
    sends: Mutex<Sends>,
    signals: Mutex<Vec<Signal>>,
}

impl FakeTelemetry {
    /// It sends `sends` from here on.
    pub fn set_sends(&self, sends: Sends) {
        *lock(&self.sends) = sends;
    }

    /// Every signal it was handed, in order.
    pub fn signals(&self) -> Vec<Signal> {
        lock(&self.signals).clone()
    }

    /// Every span it was handed, in order.
    pub fn spans(&self) -> Vec<Signal> {
        self.signals()
            .into_iter()
            .filter(|signal| matches!(signal, Signal::Span { .. }))
            .collect()
    }

    /// The events of every record it was handed, in order.
    pub fn events(&self) -> Vec<String> {
        self.signals()
            .into_iter()
            .filter_map(|signal| match signal {
                Signal::Record { event, .. } => Some(event),
                _ => None,
            })
            .collect()
    }

    /// Forget every signal handed so far.
    pub fn clear(&self) {
        lock(&self.signals).clear();
    }
}

impl Telemetry for FakeTelemetry {
    fn sends(&self) -> Sends {
        *lock(&self.sends)
    }

    fn send(&self, signal: Signal) {
        lock(&self.signals).push(signal);
    }
}

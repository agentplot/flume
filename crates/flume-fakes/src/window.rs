use crate::{lock, Log};
use flume_seams::{ViewCommand, Window, WindowShown};
use std::sync::Mutex;

/// The window seam: each show logged with its command and title, and
/// answered opened, or printed or refused as scripted; the windows it
/// opened kept in order, each with the command it runs and its title. It
/// reads no display (188, 189).
#[derive(Default)]
pub struct FakeWindow {
    state: Mutex<WindowState>,
    pub log: Log,
}

#[derive(Default)]
struct WindowState {
    /// What every show answers, opened where none.
    answer: Option<WindowShown>,
    /// The windows opened, each with its command and title.
    windows: Vec<(ViewCommand, String)>,
}

impl FakeWindow {
    /// Every show from now answers `answer`, printed or refused among
    /// them, or opens a window where none.
    pub fn answers(&self, answer: Option<WindowShown>) {
        lock(&self.state).answer = answer;
    }

    /// The windows opened, in order, each with the command it runs and
    /// its title.
    pub fn windows(&self) -> Vec<(ViewCommand, String)> {
        lock(&self.state).windows.clone()
    }
}

impl Window for FakeWindow {
    fn show(&self, command: &ViewCommand, title: &str) -> WindowShown {
        let mut words = vec![command.program.clone()];
        words.extend(command.arguments.iter().cloned());
        self.log
            .push(format!("show {} titled {title}", words.join(" ")));
        let mut state = lock(&self.state);
        let answer = state.answer.clone().unwrap_or(WindowShown::Opened);
        if answer == WindowShown::Opened {
            state.windows.push((command.clone(), title.to_string()));
        }
        answer
    }
}

//! The window seam (B.8, 188, 189): a window of the host's display
//! running the view's command under a title.

use crate::ViewCommand;
use serde::{Deserialize, Serialize};

/// show: a window running the command under the title, the command for
/// a person to run where the host shows no window, or a refusal (188).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowShown {
    Opened,
    Printed { line: String },
    Refused { reason: String },
}

/// The window seam (B.8), bound once for the host. The guarantee: show
/// opens at most one window at each asking, runs nothing in it but the
/// command it is given, and answers within a bound the profile states
/// (189).
pub trait Window: Send + Sync {
    /// A window running `command` under `title`, whatever title the
    /// command sets, handed to the display so that it is the person's;
    /// or the line a person runs where the host shows no window (188).
    fn show(&self, command: &ViewCommand, title: &str) -> WindowShown;
}

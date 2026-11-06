//! The window profile that prints the line (design 12), bound for a
//! stream and a host naming no terminal app: show runs nothing and
//! answers the line a person runs, each variable the command needs as
//! `<name>=<set>` and never its value, then its program and arguments,
//! each word quoted for a POSIX shell where it holds anything a shell
//! reads (105, 188; core.30).

use crate::shell;
use flume_seams::{ViewCommand, Window, WindowShown, SET};

/// The profile's name in the host's configuration, `terminal_app`, and
/// the one bound where it names none (105, 188).
pub const NAME: &str = "printed";

/// The window that prints the line a person runs.
#[derive(Clone, Copy, Debug, Default)]
pub struct PrintedWindow;

/// The line a person runs for `command`, naming each variable it needs
/// and giving none's value.
pub fn line(command: &ViewCommand) -> String {
    let mut words: Vec<String> = command
        .environment
        .keys()
        .map(|name| format!("{name}={SET}"))
        .collect();
    words.push(shell::quote(&command.program));
    words.extend(command.arguments.iter().map(|word| shell::argument(word)));
    words.join(" ")
}

impl Window for PrintedWindow {
    fn show(&self, command: &ViewCommand, _title: &str) -> WindowShown {
        WindowShown::Printed {
            line: line(command),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn a_line_a_person_runs() {
        // 188; core.30.
        let mut command = ViewCommand {
            program: "/bin/herdr".into(),
            arguments: vec!["session".into(), "attach".into(), "work".into()],
            environment: BTreeMap::new(),
        };
        assert_eq!(line(&command), "/bin/herdr session attach work");
        command
            .environment
            .insert("XDG_CONFIG_HOME".into(), "/home/a b".into());
        let printed = line(&command);
        assert_eq!(
            printed,
            "XDG_CONFIG_HOME=<set> /bin/herdr session attach work"
        );
        assert!(!printed.contains("/home/a b"));
    }
}

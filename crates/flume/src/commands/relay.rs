//! `flume relay <place>` (221, 222): a place's live agent's terminal
//! relayed for a caller that carries a person's screen and keys, in JSON
//! lines both ways whatever `--json` says. Out, each line the core gives,
//! `{"screen": …}`, `{"unread": …}` or `{"untyped": …}`, and last the
//! core's answer as `--json` gives it; in, one input a line, `{"text": …}`
//! or `{"keys": [ … ]}` by flume's key names, read on a thread of the
//! command's own and ended with it (120; core.13). The input's close, or
//! an output no longer read, is the caller's close. The command computes
//! nothing of the relay itself: the core relays (core.14).

use super::place;
use flume_core::{Answer, Engine, RelayLine, RelayLink, RelayNext};
use flume_seams::{Key, Typed};
use serde_json::Value;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError};
use std::time::{Duration, Instant};

/// What a line of input that is no input of either form is given back
/// as (222).
const NO_INPUT: &str =
    "a line of input is one JSON object, {\"text\": \"<text>\"} or {\"keys\": [\"<key>\", …]}";

/// Relays the place `named` to `output` and from `input` until the
/// input closes, the output is no longer read or the agent's terminal
/// goes; the answer written last, the exit it gives returned. A refusal,
/// of the place's name, the core's building or the relay, is the only
/// line (4, 61, 221).
pub fn serve(
    named: &str,
    input: impl Read + Send + 'static,
    mut output: impl Write,
    build: &dyn Fn() -> Result<Engine, String>,
    repository: Option<&Path>,
) -> u8 {
    let reached = place::place(named, repository).and_then(|place| Ok((place, build()?)));
    let answer = match reached {
        Ok((place, core)) => {
            let mut link = CommandLink::new(input, &mut output);
            core.relay(&place, &mut link)
        }
        Err(refused) => Answer::refused(refused),
    };
    let _ = writeln!(output, "{}", answer.to_json());
    let _ = output.flush();
    answer.exit()
}

/// The command's side of a relay: the input's lines as they come, read
/// on a thread of the command's own, and the core's lines written to
/// the output as they are given (221).
struct CommandLink<W: Write> {
    lines: Receiver<String>,
    output: W,
    ticked: Instant,
}

impl<W: Write> CommandLink<W> {
    fn new(input: impl Read + Send + 'static, output: W) -> CommandLink<W> {
        let (send, lines) = channel();
        std::thread::spawn(move || {
            for line in BufReader::new(input).lines() {
                let Ok(line) = line else { break };
                if send.send(line).is_err() {
                    break;
                }
            }
        });
        CommandLink {
            lines,
            output,
            ticked: Instant::now(),
        }
    }
}

impl<W: Write> RelayLink for CommandLink<W> {
    fn give(&mut self, line: &RelayLine) -> bool {
        let Ok(written) = serde_json::to_string(line) else {
            return false;
        };
        writeln!(self.output, "{written}").is_ok() && self.output.flush().is_ok()
    }

    fn next(&mut self, every: Duration) -> RelayNext {
        loop {
            let due = self.ticked + every;
            let line = match self
                .lines
                .recv_timeout(due.saturating_duration_since(Instant::now()))
            {
                Ok(line) => line,
                Err(RecvTimeoutError::Timeout) => {
                    self.ticked = Instant::now();
                    return RelayNext::Tick;
                }
                Err(RecvTimeoutError::Disconnected) => return RelayNext::Closed,
            };
            if line.trim().is_empty() {
                continue;
            }
            match typed(&line) {
                Ok(input) => return RelayNext::Typed(input),
                Err(why) => {
                    if !self.give(&RelayLine::Untyped(why)) {
                        return RelayNext::Closed;
                    }
                }
            }
        }
    }
}

/// A line of input read as the person's input, or why it is none: no
/// object of either form, or a key flume does not name, naming it and
/// flume's names (222).
fn typed(line: &str) -> Result<Typed, String> {
    let read: Value = serde_json::from_str(line).map_err(|_| NO_INPUT.to_string())?;
    let Some(object) = read.as_object().filter(|object| object.len() == 1) else {
        return Err(NO_INPUT.into());
    };
    if let Some(text) = object.get("text") {
        return text
            .as_str()
            .map(|text| Typed::Text(text.to_string()))
            .ok_or_else(|| NO_INPUT.into());
    }
    let Some(keys) = object.get("keys").and_then(Value::as_array) else {
        return Err(NO_INPUT.into());
    };
    keys.iter()
        .map(|key| {
            key.as_str()
                .ok_or_else(|| NO_INPUT.to_string())
                .and_then(|name| name.parse::<Key>())
        })
        .collect::<Result<Vec<Key>, String>>()
        .map(Typed::Keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_of_input_read_as_text_keys_or_why_not() {
        assert_eq!(typed(r#"{"text":"yes"}"#), Ok(Typed::Text("yes".into())));
        assert_eq!(
            typed(r#"{"keys":["ctrl+c","enter"]}"#),
            Ok(Typed::Keys(vec![
                "ctrl+c".parse().unwrap(),
                "enter".parse().unwrap()
            ]))
        );
        for no in [
            "hello",
            "[]",
            r#"{"text":1}"#,
            r#"{"keys":"enter"}"#,
            r#"{"keys":[1]}"#,
            r#"{"text":"a","keys":[]}"#,
        ] {
            assert_eq!(typed(no), Err(NO_INPUT.to_string()), "{no}");
        }
        let refused = typed(r#"{"keys":["enter","hyper+x"]}"#).unwrap_err();
        assert!(
            refused.starts_with("hyper+x is no key flume names") && refused.contains("pagedown"),
            "{refused}"
        );
    }
}

//! A session's transcript as Claude Code 2.1.296 writes it, one JSON
//! record a line: `user` turns, whose `message.content` is the prompt's
//! text or blocks, a tool's result among them with its
//! `toolUseResult.backgroundTaskId` where it started work in the
//! background; `assistant` messages, whose content blocks hold text and
//! `tool_use` calls; and `queue-operation` records, an `enqueue` holding a
//! prompt queued while the agent worked. A slash command's turn holds
//! `<command-name>` and `<command-args>`, and the end of work started in
//! the background a `<task-notification>` naming its `<task-id>`.

use super::record::Asking;
use super::{as_received, collapsed};
use serde_json::Value;
use std::collections::BTreeSet;
use std::io::ErrorKind;
use std::path::Path;

/// A transcript read, not yet written, or one that cannot be read.
pub enum Read {
    Read(Transcript),
    NotWritten,
    Unreadable,
}

/// The records of one session's transcript.
pub struct Transcript {
    records: Vec<Value>,
}

/// The text of a record's `message.content`: the string, or its text
/// blocks joined.
fn content_text(content: &Value) -> Option<String> {
    match content {
        Value::String(text) => Some(text.clone()),
        Value::Array(blocks) => {
            let texts: Vec<&str> = blocks
                .iter()
                .filter(|block| block["type"] == "text")
                .filter_map(|block| block["text"].as_str())
                .collect();
            (!texts.is_empty()).then(|| texts.join("\n"))
        }
        _ => None,
    }
}

/// The text between `<tag>` and `</tag>` in `text`, each time it shows.
fn tagged<'a>(text: &'a str, tag: &str) -> Vec<&'a str> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut found = vec![];
    let mut rest = text;
    while let Some(start) = rest.find(&open) {
        let after = &rest[start + open.len()..];
        let Some(end) = after.find(&close) else {
            break;
        };
        found.push(&after[..end]);
        rest = &after[end + close.len()..];
    }
    found
}

impl Transcript {
    /// The transcript at `path`.
    pub fn read(path: &Path) -> Read {
        match std::fs::read_to_string(path) {
            Ok(text) => Read::Read(Transcript::parse(&text)),
            Err(error) if error.kind() == ErrorKind::NotFound => Read::NotWritten,
            Err(_) => Read::Unreadable,
        }
    }

    /// The records of `text`, a line that is no JSON (one being written)
    /// left out.
    pub fn parse(text: &str) -> Transcript {
        Transcript {
            records: text
                .lines()
                .filter_map(|line| serde_json::from_str::<Value>(line).ok())
                .filter(|record| record["isSidechain"] != true)
                .collect(),
        }
    }

    fn of_type<'a>(&'a self, kind: &'a str) -> impl Iterator<Item = &'a Value> + 'a {
        self.records
            .iter()
            .filter(move |record| record["type"] == kind)
    }

    /// Every text a person or flume handed the agent: each user turn's
    /// text and each queued prompt.
    fn prompts(&self) -> Vec<String> {
        let turns = self
            .of_type("user")
            .filter_map(|record| content_text(&record["message"]["content"]));
        let queued = self
            .of_type("queue-operation")
            .filter(|record| record["operation"] == "enqueue")
            .filter_map(|record| record["content"].as_str().map(str::to_string));
        turns.chain(queued).collect()
    }

    /// Whether the transcript holds `prompt` as a user turn or a queued
    /// prompt, a slash command as its name and arguments, paste tags taken
    /// off and whitespace collapsed (89).
    pub fn holds_prompt(&self, prompt: &str) -> bool {
        let wanted = as_received(prompt);
        let command = wanted.strip_prefix('/').map(|rest| {
            let (name, args) = rest.split_once(' ').unwrap_or((rest, ""));
            (format!("/{name}"), args.to_string())
        });
        self.prompts().iter().any(|text| {
            if as_received(text) == wanted {
                return true;
            }
            let Some((name, args)) = &command else {
                return false;
            };
            tagged(text, "command-name").first() == Some(&name.as_str())
                && tagged(text, "command-args")
                    .first()
                    .is_some_and(|given| collapsed(given) == *args)
        })
    }

    /// The id of the call `mark` names: its own, or the last call of its
    /// tool with its input.
    fn call_of(&self, mark: &Asking) -> Option<String> {
        if let Some(id) = &mark.tool_use_id {
            return Some(id.clone());
        }
        self.of_type("assistant")
            .filter_map(|record| record["message"]["content"].as_array())
            .flatten()
            .filter(|block| block["type"] == "tool_use" && block["name"] == mark.tool_name)
            .filter(|block| mark.tool_input.is_null() || block["input"] == mark.tool_input)
            .filter_map(|block| block["id"].as_str().map(str::to_string))
            .last()
    }

    /// Whether the call `mark` names has its answer: a tool result for it
    /// (90).
    pub fn answered(&self, mark: &Asking) -> bool {
        let Some(id) = self.call_of(mark) else {
            return false;
        };
        self.of_type("user")
            .filter_map(|record| record["message"]["content"].as_array())
            .flatten()
            .any(|block| block["type"] == "tool_result" && block["tool_use_id"] == id.as_str())
    }

    /// The work the agent started in the background and the transcript
    /// does not show finished (91): a shell or an agent left running, by
    /// its task's id.
    pub fn background_running(&self) -> BTreeSet<String> {
        let mut started = BTreeSet::new();
        for record in self.of_type("user") {
            let result = &record["toolUseResult"];
            if let Some(id) = result["backgroundTaskId"].as_str() {
                started.insert(id.to_string());
            }
            if result["status"] == "async_launched" {
                if let Some(id) = result["agentId"].as_str() {
                    started.insert(id.to_string());
                }
            }
        }
        let mut finished = BTreeSet::new();
        for text in self.prompts().iter().chain(self.results().iter()) {
            for notification in tagged(text, "task-notification") {
                for id in tagged(notification, "task-id") {
                    finished.insert(id.trim().to_string());
                }
            }
        }
        for record in self.of_type("assistant") {
            for block in record["message"]["content"]
                .as_array()
                .into_iter()
                .flatten()
            {
                if block["type"] == "tool_use" {
                    for key in ["shell_id", "task_id"] {
                        if let Some(id) = block["input"][key].as_str() {
                            if block["name"] == "KillShell" || block["name"] == "TaskStop" {
                                finished.insert(id.to_string());
                            }
                        }
                    }
                }
            }
        }
        started.difference(&finished).cloned().collect()
    }

    /// The text of every tool result.
    fn results(&self) -> Vec<String> {
        self.of_type("user")
            .filter_map(|record| record["message"]["content"].as_array())
            .flatten()
            .filter(|block| block["type"] == "tool_result")
            .filter_map(|block| match &block["content"] {
                Value::String(text) => Some(text.clone()),
                other => content_text(other),
            })
            .collect()
    }

    /// The agent's last message's text (43).
    pub fn last_message(&self) -> Option<String> {
        self.of_type("assistant")
            .filter_map(|record| content_text(&record["message"]["content"]))
            .filter(|text| !text.trim().is_empty())
            .last()
    }
}

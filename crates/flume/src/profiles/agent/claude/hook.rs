//! `flume hook <event>` and `flume ack started|received` (C.4; design
//! 14, 17; record 0016): what each of the plugin's hooks does with the
//! input Claude Code 2.1.296 hands it on standard input, in an agent flume
//! started, whose environment names its start (`FLUME_AGENT`), the start's
//! secret, its place and its stage. Outside such an agent every hook does
//! nothing. A hook never fails and never holds up a start or a turn: what
//! it cannot do it leaves undone, and it always exits 0; it answers only
//! through the JSON Claude Code reads, a refused command's reason or a
//! stop's.

use super::guard;
use super::record::{Asking, Record, Started};
use super::{collapsed, digest, AGENT, PLACE, SECRET, STAGE};
use flume_core::{Answer, Engine};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

/// The variable Claude Code gives a hook naming the plugin's own
/// directory.
pub const PLUGIN_ROOT: &str = "CLAUDE_PLUGIN_ROOT";

/// The hooks' events, by the word `flume hook` takes for each and the
/// event Claude Code names.
pub const EVENTS: [(&str, &str); 8] = [
    ("session-start", "SessionStart"),
    ("user-prompt-submit", "UserPromptSubmit"),
    ("pre-tool-use", "PreToolUse"),
    ("permission-request", "PermissionRequest"),
    ("post-tool-use", "PostToolUse"),
    ("stop", "Stop"),
    ("stop-failure", "StopFailure"),
    ("session-end", "SessionEnd"),
];

/// What the agent's environment says of its start.
#[derive(Clone, Debug, Default)]
pub struct Start {
    pub agent: Option<String>,
    pub secret: Option<String>,
    pub place: Option<String>,
    pub stage: Option<String>,
    /// The plugin's own directory, whose manifest names its version.
    pub plugin_root: Option<PathBuf>,
}

fn variable(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

impl Start {
    /// The start this process's environment names.
    pub fn of_process() -> Start {
        Start {
            agent: variable(AGENT),
            secret: variable(SECRET),
            place: variable(PLACE),
            stage: variable(STAGE),
            plugin_root: variable(PLUGIN_ROOT).map(PathBuf::from),
        }
    }

    /// The plugin's version, as its manifest gives it; none where it
    /// cannot be read.
    pub fn plugin_version(&self) -> String {
        self.plugin_root
            .as_ref()
            .and_then(|root| std::fs::read_to_string(root.join(".claude-plugin/plugin.json")).ok())
            .and_then(|text| serde_json::from_str::<Value>(&text).ok())
            .and_then(|manifest| manifest["version"].as_str().map(str::to_string))
            .unwrap_or_default()
    }

    /// The start's record, the folder found through the core with the
    /// start's secret (128, 135).
    fn record(&self, build: &dyn Fn() -> Result<Engine, String>) -> Result<Record, String> {
        let (Some(agent), Some(secret)) = (&self.agent, &self.secret) else {
            return Err(format!(
                "this agent was not started by flume: {AGENT} or {SECRET} is not set"
            ));
        };
        let folder = build()?.agent_folder(agent, secret)?;
        Ok(Record::named(&folder, agent))
    }
}

/// The started ack: the start's session, its transcript, how it began and
/// the plugin's version, taken only with the start's secret; it clears the
/// asking mark (35, 90, 135).
pub fn ack_started(
    start: &Start,
    started: &Started,
    build: &dyn Fn() -> Result<Engine, String>,
) -> Result<String, String> {
    let record = start.record(build)?;
    record
        .ack_started(started)
        .and_then(|()| record.clear_asking())
        .map_err(|error| format!("{}: {error}", record.dir().display()))?;
    Ok(format!(
        "{} started in session {}",
        start.agent.as_deref().unwrap_or_default(),
        started.session_id
    ))
}

/// The received ack: the submitted prompt's digest, taken only with the
/// start's secret; it clears the asking mark (89, 90).
pub fn ack_received(
    start: &Start,
    digest: &str,
    build: &dyn Fn() -> Result<Engine, String>,
) -> Result<String, String> {
    let record = start.record(build)?;
    record
        .ack_received(digest)
        .and_then(|()| record.clear_asking())
        .map_err(|error| format!("{}: {error}", record.dir().display()))?;
    Ok(format!(
        "{} received the prompt {digest}",
        start.agent.as_deref().unwrap_or_default()
    ))
}

/// The words a refused command is answered with (116).
fn guard_reason(act: &str) -> String {
    format!(
        "flume submits this place once its last stage delivers, so an agent flume started never runs `{act}`. Commit your work on the place's branch; flume does the rest."
    )
}

/// What the hook for `event` does with `input`, in the start `start`
/// names: the JSON Claude Code reads, or nothing. It does nothing outside
/// an agent flume started, and never fails.
pub fn hook(
    event: &str,
    input: &str,
    start: &Start,
    build: &dyn Fn() -> Result<Engine, String>,
    binary: &Path,
) -> String {
    if start.agent.is_none() || start.secret.is_none() {
        return String::new();
    }
    let input: Value = serde_json::from_str(input).unwrap_or(Value::Null);
    // A subagent's input carries its id; the acks skip it, and the guard
    // refuses its command as its agent's (C.4).
    let subagent = input["agent_id"].is_string();
    let text = |key: &str| input[key].as_str().unwrap_or_default().to_string();
    match event {
        "session-start" if !subagent => {
            let started = Started {
                session_id: text("session_id"),
                transcript_path: text("transcript_path"),
                source: text("source"),
                plugin_version: start.plugin_version(),
            };
            let _ = ack_started(start, &started, build);
            String::new()
        }
        "user-prompt-submit" if !subagent => {
            let _ = ack_received(start, &digest(&collapsed(&text("prompt"))), build);
            String::new()
        }
        "pre-tool-use" => match input["tool_name"].as_str() {
            Some("Bash") => {
                let line = input["tool_input"]["command"].as_str().unwrap_or_default();
                let Some(act) = guard::refused(line) else {
                    return String::new();
                };
                if let Ok(record) = start.record(build) {
                    let _ = record.refuse(&format!("{act}: {}", collapsed(line)));
                }
                json!({
                    "hookSpecificOutput": {
                        "hookEventName": "PreToolUse",
                        "permissionDecision": "deny",
                        "permissionDecisionReason": guard_reason(&act),
                    }
                })
                .to_string()
            }
            Some(tool @ ("AskUserQuestion" | "ExitPlanMode")) => {
                mark(start, build, &input, tool, true);
                String::new()
            }
            _ => String::new(),
        },
        "permission-request" => {
            let tool = text("tool_name");
            mark(start, build, &input, &tool, false);
            String::new()
        }
        "post-tool-use" => {
            if let Ok(record) = start.record(build) {
                if let Some(mark) = record.asking() {
                    let answers = match &mark.tool_use_id {
                        Some(id) => input["tool_use_id"] == id.as_str(),
                        None => {
                            input["tool_name"] == mark.tool_name.as_str()
                                && input["tool_input"] == mark.tool_input
                        }
                    };
                    if answers {
                        let _ = record.clear_asking();
                    }
                }
            }
            String::new()
        }
        "stop" => stop(start, build, binary),
        "stop-failure" => {
            if let Ok(record) = start.record(build) {
                let error = text("error");
                let said = [text("last_assistant_message"), text("error_details")]
                    .into_iter()
                    .find(|words| !words.trim().is_empty());
                let words = match (said, error.is_empty()) {
                    (Some(said), false) => format!("{said} ({error})"),
                    (Some(said), true) => said,
                    (None, false) => format!("the turn failed: {error}"),
                    (None, true) => "the turn failed".to_string(),
                };
                let _ = record.set_last_words(&words);
            }
            String::new()
        }
        "session-end" => {
            if let Ok(record) = start.record(build) {
                let _ = record.mark_ended(&text("reason"));
            }
            String::new()
        }
        _ => String::new(),
    }
}

/// Leaves the asking mark on the call `input` names: by its id where
/// `by_id`, else by its tool and input (90).
fn mark(
    start: &Start,
    build: &dyn Fn() -> Result<Engine, String>,
    input: &Value,
    tool: &str,
    by_id: bool,
) {
    if let Ok(record) = start.record(build) {
        let _ = record.mark_asking(&Asking {
            tool_use_id: by_id
                .then(|| input["tool_use_id"].as_str().map(str::to_string))
                .flatten(),
            tool_name: tool.to_string(),
            tool_input: input["tool_input"].clone(),
        });
    }
}

/// The Stop hook: the stop blocked with the core's stop-check words where
/// it answers some, never a conversation's agent; else `flume settle
/// <place> --on-quiet` started detached, so the place moves on the moment
/// its agent is quiet (46, 109, 110, 115, 121).
fn stop(start: &Start, build: &dyn Fn() -> Result<Engine, String>, binary: &Path) -> String {
    let agent = start.agent.as_deref().unwrap_or_default();
    let words = match build().map(|core| core.stop_check(agent)) {
        Ok(Answer::StopCheck { words }) => words,
        _ => None,
    };
    if let Some(words) = words {
        return json!({ "decision": "block", "reason": words }).to_string();
    }
    if let Some(place) = &start.place {
        settle_detached(binary, place);
    }
    String::new()
}

/// `flume settle <place> --on-quiet`, in a process group of its own that
/// outlives the hook, its output nowhere.
fn settle_detached(binary: &Path, place: &str) {
    use std::os::unix::process::CommandExt;
    use std::process::{Command, Stdio};
    let _ = Command::new(binary)
        .args(["settle", place, "--on-quiet"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0)
        .spawn();
}

//! `flume hook <event>` and `flume ack started|received` (C.4; design
//! 14, 17; record 0016): what each of the plugin's hooks does with the
//! input Claude Code 2.1.296 hands it on standard input, in an agent flume
//! started, whose environment names its start (`FLUME_AGENT`), the start's
//! secret, its place and its stage. SessionStart compares the agent's
//! program and environment with its start's record before it acks, and
//! UserPromptSubmit, at the start's first prompt, the mode its session
//! runs in, which 2.1.296 gives no SessionStart input, blocking the prompt
//! where it is another (161, 162). Outside such an agent every hook does
//! nothing. A hook never fails and never holds up a start or a turn: what
//! it cannot do it leaves undone, and it always exits 0; it answers only
//! through the JSON Claude Code reads, a refused command's reason, a
//! blocked prompt's or a stop's.

use super::record::{self, Asking, Failure, Record, Started};
use super::{as_received, collapsed, digest, guard, mode, wrapper, AGENT, PLACE, SECRET, STAGE};
use crate::host::process::{parent_executable, program_path};
use flume_core::{variable_digest, Answer, Engine};
use flume_seams::{ModeBegan, Secret, StartRecord, Unsettled};
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

/// How this process began otherwise than the start's record says its
/// agent runs: each variable the record names that `variable` finds
/// unset, each whose digest keyed by `secret` differs, by name alone, and
/// whether the program the record names, found on `path` as the command
/// finds it and run in a nix wrapper's place, is another than `ran`, the
/// program this process's parent runs. The program is compared only
/// where both are read (161; core.10, core.30).
pub fn compare(
    recorded: &StartRecord,
    secret: &Secret,
    variable: impl Fn(&str) -> Option<String>,
    ran: Option<&Path>,
    path: &str,
) -> Unsettled {
    let mut unsettled = Unsettled::default();
    for (name, digest) in &recorded.env {
        match variable(name) {
            None => unsettled.missing.push(name.clone()),
            Some(value) if &variable_digest(secret, name, &value) != digest => {
                unsettled.different.push(name.clone());
            }
            Some(_) => {}
        }
    }
    let named = program_path(&recorded.program, path)
        .map(|file| wrapper::unwrapped(&file).map_or(file, |wrapped| wrapped.program));
    let canonical = |file: &Path| file.canonicalize().unwrap_or_else(|_| file.to_path_buf());
    if let (Some(named), Some(ran)) = (named, ran) {
        unsettled.program = canonical(&named) != canonical(ran);
    }
    unsettled
}

/// The mode asked and the mode begun, where the start's record names a
/// mode and `began`, the mode the session runs in as the program names it,
/// read through [`mode::began`], is another or none; none where they
/// match or the record names no mode (161, 162; core.10).
pub fn compare_mode(recorded: &StartRecord, began: Option<&str>) -> Option<ModeBegan> {
    let asked = recorded.mode.as_ref()?;
    let began = began.map(mode::began);
    (began != Some(asked.as_str())).then(|| ModeBegan {
        asked: asked.clone(),
        began: began.map(str::to_string),
    })
}

/// The started ack: the start's session, its transcript, how it began and
/// the plugin's version, taken only with the start's secret and only where
/// this process's parent runs the program the start's record names and
/// every variable the record names matches it; it clears the asking mark.
/// The mode is compared at the start's first prompt ([`ack_received`]),
/// since 2.1.296 gives a SessionStart input none. Where anything differs
/// it is not taken: the agent's record says what differed, a variable by
/// name alone, and a settle of the place is started detached, which ends
/// the agent once quiet and starts its stage afresh (35, 90, 121, 135,
/// 161).
pub fn ack_started(
    start: &Start,
    started: &Started,
    build: &dyn Fn() -> Result<Engine, String>,
    binary: &Path,
) -> Result<String, String> {
    let (Some(agent), Some(secret)) = (&start.agent, &start.secret) else {
        return Err(format!(
            "this agent was not started by flume: {AGENT} or {SECRET} is not set"
        ));
    };
    let (folder, recorded) = build()?.start_record(agent, secret)?;
    let record = Record::named(&folder, agent);
    let unsettled = compare(
        &recorded,
        &Secret(secret.clone()),
        |name| std::env::var_os(name).map(|value| value.to_string_lossy().into_owned()),
        parent_executable().as_deref(),
        &std::env::var("PATH").unwrap_or_default(),
    );
    if unsettled != Unsettled::default() {
        return Err(began_otherwise(
            start,
            &record,
            unsettled,
            &started.session_id,
            &started.transcript_path,
            binary,
        ));
    }
    record
        .ack_started(started)
        .and_then(|()| record.clear_asking())
        .map_err(|error| format!("{}: {error}", record.dir().display()))?;
    Ok(format!("{agent} started in session {}", started.session_id))
}

/// Why a received ack was not taken.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unacked {
    /// The start's first prompt came in another mode than its start
    /// asked: the agent is not its stage's, and the prompt is held back
    /// with these words.
    Mode(String),
    /// The ack could not be taken: no start, no secret, or no record.
    Failed(String),
}

impl Unacked {
    pub fn words(self) -> String {
        match self {
            Unacked::Mode(words) | Unacked::Failed(words) => words,
        }
    }
}

/// The received ack: the submitted prompt's digest, its paste tags taken
/// off ([`as_received`]), taken only with the start's secret; it clears
/// the asking mark (89, 90). At the start's first prompt, where its record
/// names a mode and nothing yet was received or read begun otherwise,
/// `began`, the mode the session runs in as the program names it, is
/// compared first ([`compare_mode`]): where it is another or none, the ack
/// is not taken, the agent's record names the mode asked and the mode
/// begun with `session` and `transcript`, the started ack's where it
/// stands, and a settle of the place is started detached, which ends the
/// agent once quiet and starts its stage afresh (161, 162). A later
/// prompt compares no mode, since a started session's mode is its own.
pub fn ack_received(
    start: &Start,
    digest: &str,
    began: Option<&str>,
    session: &str,
    transcript: &str,
    build: &dyn Fn() -> Result<Engine, String>,
    binary: &Path,
) -> Result<String, Unacked> {
    let (Some(agent), Some(secret)) = (&start.agent, &start.secret) else {
        return Err(Unacked::Failed(format!(
            "this agent was not started by flume: {AGENT} or {SECRET} is not set"
        )));
    };
    let (folder, recorded) = build()
        .and_then(|core| core.start_record(agent, secret))
        .map_err(Unacked::Failed)?;
    let record = Record::named(&folder, agent);
    let first = !record.received_any() && record.unsettled().is_none();
    if let Some(mode) = first.then(|| compare_mode(&recorded, began)).flatten() {
        let started = record.started();
        let (session, transcript) = started.as_ref().map_or((session, transcript), |started| {
            (
                started.session_id.as_str(),
                started.transcript_path.as_str(),
            )
        });
        let unsettled = Unsettled {
            mode: Some(mode),
            ..Unsettled::default()
        };
        return Err(Unacked::Mode(began_otherwise(
            start, &record, unsettled, session, transcript, binary,
        )));
    }
    record
        .ack_received(digest)
        .and_then(|()| record.clear_asking())
        .map_err(|error| Unacked::Failed(format!("{}: {error}", record.dir().display())))?;
    Ok(format!("{agent} received the prompt {digest}"))
}

/// The agent's record told that it began as `unsettled` says, in
/// `session` with `transcript`, and a settle of the place started
/// detached; the words saying so, or why the record could not be written
/// (161).
fn began_otherwise(
    start: &Start,
    record: &Record,
    unsettled: Unsettled,
    session: &str,
    transcript: &str,
    binary: &Path,
) -> String {
    let agent = start.agent.as_deref().unwrap_or_default();
    let written = record.set_unsettled(&record::Unsettled {
        unsettled: unsettled.clone(),
        session_id: session.to_string(),
        transcript_path: transcript.to_string(),
    });
    if let Err(error) = written {
        return format!("{}: {error}", record.dir().display());
    }
    if let Some(place) = &start.place {
        settle_detached(binary, place, false);
    }
    format!(
        "{agent} began {}, not as its start recorded, so it is not its stage's agent",
        unsettled.words()
    )
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
            let _ = ack_started(start, &started, build, binary);
            String::new()
        }
        "user-prompt-submit" if !subagent => {
            let acked = ack_received(
                start,
                &digest(&as_received(&text("prompt"))),
                input[mode::FIELD].as_str(),
                &text("session_id"),
                &text("transcript_path"),
                build,
                binary,
            );
            match acked {
                // The prompt is held back, so the agent takes no turn in
                // a mode its start did not ask (161).
                Err(Unacked::Mode(words)) => {
                    json!({ "decision": "block", "reason": words }).to_string()
                }
                _ => String::new(),
            }
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
                let _ = record.set_failure(&Failure {
                    error: error.clone(),
                    words: said.clone().unwrap_or_default(),
                    at: now(),
                });
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

/// The host's clock, in seconds.
fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs())
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
        settle_detached(binary, place, true);
    }
    String::new()
}

/// `flume settle <place>`, with `--on-quiet` where `on_quiet`, in a
/// process group of its own that outlives the hook, its output nowhere.
fn settle_detached(binary: &Path, place: &str, on_quiet: bool) {
    use std::os::unix::process::CommandExt;
    use std::process::{Command, Stdio};
    let mut args = vec!["settle", place];
    if on_quiet {
        args.push("--on-quiet");
    }
    let _ = Command::new(binary)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0)
        .spawn();
}

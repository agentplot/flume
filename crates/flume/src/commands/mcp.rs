//! `flume mcp` (57, 60; section 9; design 12, 17): flume served over the
//! Model Context Protocol on standard input and output, one JSON-RPC
//! message a line, with four tools: `status`, which answers what `flume
//! status --json` answers over the same state and settles nothing; and
//! `message`, `launch` and `wake`, each call turned into the command
//! line's arguments and run through its parser and act, so every default,
//! check and refusal is the command line's, its result the act's JSON and
//! a refusal marked an error. A launch takes no file over MCP: its brief
//! is text. The server runs in its caller's environment, so a launch from
//! a role's run reports to its address (7, 132, 214). Each call builds the
//! core afresh, as each command does.

use super::place;
use flume_core::{Answer, Engine};
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::path::Path;

/// The protocol's version the server answers where the client names
/// none.
pub const PROTOCOL: &str = "2025-06-18";

/// A string property of a tool's input, described.
fn text(description: &str) -> Value {
    json!({ "type": "string", "description": description })
}

/// An array property of a tool's input, each a string, for a flag the
/// command line takes again and again.
fn texts(description: &str) -> Value {
    json!({ "type": "array", "items": { "type": "string" }, "description": description })
}

/// The tools.
fn tools() -> Value {
    json!([
        {
            "name": "status",
            "description": "Every fact of a flume place, or of every place on this host, and of the host, as `flume status --json` gives them. Reads; settles and changes nothing.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "place": text("A place's location, or `<kind>--<name>` in the repository the server runs in; every place where none is given.")
                },
                "additionalProperties": false
            },
            "annotations": { "readOnlyHint": true }
        },
        {
            "name": "message",
            "description": "`flume message`: words to a place's agent, handed as a prompt once it waits, or to a role's address, `<stream>/<role>`, kept as a delivery its session takes at its next wait, the address's run woken where none stands. Answers as `flume message --json` does.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "target": text("A role's address, `<stream>/<role>`; else a place, by its location or `<kind>--<name>` in the repository the server runs in."),
                    "words": text("The words."),
                    "within": text("How long to wait for a place's agent, as `<n>`, `<n>s`, `<n>m` or `<n>h`."),
                    "answers": text("The key of the stop-short the words answer, as the inbox and status give it.")
                },
                "required": ["target", "words"],
                "additionalProperties": false
            },
            "annotations": { "readOnlyHint": false }
        },
        {
            "name": "launch",
            "description": "`flume launch`: a place of a kind for a piece of work, in the repository the server runs in, its brief as text. A launch naming no recipient from a role's run reports to the run's address. Answers as `flume launch --json` does.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "kind": text("The kind, by its name or identity."),
                    "name": text("The place's name."),
                    "brief": text("The brief, as text."),
                    "base": text("The base the place is cut from and lands on."),
                    "recipient": texts("Each recipient of the place's ends: a session's name or a role's address."),
                    "hold": texts("Each stage held, as `<stage>` or `<stage>=<words>`."),
                    "hold_submission": { "type": "boolean", "description": "Hold the place's submission." },
                    "mark": text("The caller's mark."),
                    "priority": { "type": "integer", "description": "The place's priority." },
                    "stall_after": text("How long the place waits on its caller before it is said stalled."),
                    "agent": texts("Each agent setting, as `[<stage>.]<field>=<value>`."),
                    "stream": text("The stream the place runs in."),
                    "intent": text("The intent the place serves.")
                },
                "required": ["kind", "name"],
                "additionalProperties": false
            },
            "annotations": { "readOnlyHint": false }
        },
        {
            "name": "wake",
            "description": "`flume wake`: a role's address woken, its run launched where none stands. Answers as `flume wake --json` does.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "address": text("The role's address, `<stream>/<role>`.")
                },
                "required": ["address"],
                "additionalProperties": false
            },
            "annotations": { "readOnlyHint": false }
        }
    ])
}

/// The command line's words for the tool `name` called with `arguments`:
/// each of its switches by its long name, and its words after `--`, so
/// none is read as a switch; none for a tool of no act's.
fn words(name: &str, arguments: &Value) -> Option<Vec<String>> {
    let string = |key: &str| arguments[key].as_str().map(str::to_string);
    let mut words = vec![name.to_string()];
    let flag = |words: &mut Vec<String>, key: &str, long: &str| {
        if let Some(value) = string(key) {
            words.extend([format!("--{long}"), value]);
        }
    };
    match name {
        "message" => {
            flag(&mut words, "within", "within");
            flag(&mut words, "answers", "answers");
            words.push("--".into());
            words.push(string("target").unwrap_or_default());
            words.push(string("words").unwrap_or_default());
        }
        "launch" => {
            for (key, long) in [
                ("kind", "kind"),
                ("name", "name"),
                ("brief", "brief"),
                ("base", "base"),
                ("mark", "mark"),
                ("stall_after", "stall-after"),
                ("stream", "stream"),
                ("intent", "intent"),
            ] {
                flag(&mut words, key, long);
            }
            for (key, long) in [
                ("recipient", "recipient"),
                ("hold", "hold"),
                ("agent", "agent"),
            ] {
                for value in arguments[key].as_array().into_iter().flatten() {
                    if let Some(value) = value.as_str() {
                        words.extend([format!("--{long}"), value.to_string()]);
                    }
                }
            }
            if arguments["hold_submission"].as_bool() == Some(true) {
                words.push("--hold-submission".into());
            }
            if let Some(priority) = arguments["priority"].as_i64() {
                words.push(format!("--priority={priority}"));
            }
        }
        "wake" => {
            words.push("--".into());
            words.push(string("address").unwrap_or_default());
        }
        _ => return None,
    }
    Some(words)
}

/// An act's tool's answer to `arguments`: the command line's answer to
/// the same words, as its JSON, a refusal marked an error.
fn act(
    name: &str,
    arguments: &Value,
    build: &dyn Fn() -> Result<Engine, String>,
    repository: Option<&Path>,
) -> Option<Value> {
    let mut argv = vec!["flume".to_string(), "--json".to_string()];
    if let Some(repository) = repository {
        argv.extend([
            "--repository".to_string(),
            repository.to_string_lossy().into_owned(),
        ]);
    }
    argv.extend(words(name, arguments)?);
    let outcome = super::run(argv, build);
    Some(json!({
        "content": [{ "type": "text", "text": outcome.text.trim() }],
        "isError": outcome.code != 0,
    }))
}

/// The status tool's answer to `arguments`: the status's JSON, a refusal
/// marked an error.
fn status(
    arguments: &Value,
    build: &dyn Fn() -> Result<Engine, String>,
    repository: Option<&Path>,
) -> Value {
    let answer = (|| -> Result<Answer, String> {
        let named = arguments["place"]
            .as_str()
            .filter(|place| !place.is_empty())
            .map(|place| place::place(place, repository))
            .transpose()?;
        Ok(build()?.status(named.as_ref()))
    })()
    .unwrap_or_else(Answer::refused);
    json!({
        "content": [{ "type": "text", "text": answer.to_json() }],
        "isError": answer.refusal().is_some(),
    })
}

/// Whether `name` names a tool of an act's: `message`, `launch` or `wake`.
fn act_named(name: &Value) -> bool {
    matches!(name.as_str(), Some("message" | "launch" | "wake"))
}

/// The answer to one request, none for a notification.
fn answer(
    request: &Value,
    build: &dyn Fn() -> Result<Engine, String>,
    repository: Option<&Path>,
) -> Option<Value> {
    let id = request.get("id")?.clone();
    let method = request["method"].as_str().unwrap_or_default();
    let result = match method {
        "initialize" => json!({
            "protocolVersion": request["params"]["protocolVersion"]
                .as_str()
                .unwrap_or(PROTOCOL),
            "capabilities": { "tools": {} },
            "serverInfo": { "name": "flume", "version": crate::VERSION },
        }),
        "ping" => json!({}),
        "tools/list" => json!({ "tools": tools() }),
        "tools/call" if request["params"]["name"] == "status" => {
            status(&request["params"]["arguments"], build, repository)
        }
        "tools/call" if act_named(&request["params"]["name"]) => act(
            request["params"]["name"].as_str().unwrap_or_default(),
            &request["params"]["arguments"],
            build,
            repository,
        )?,
        "tools/call" => {
            return Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": { "code": -32602, "message": format!("no tool is named {}", request["params"]["name"]) },
            }))
        }
        _ => {
            return Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": { "code": -32601, "message": format!("no method {method}") },
            }))
        }
    };
    Some(json!({ "jsonrpc": "2.0", "id": id, "result": result }))
}

/// Serves requests read from `input`, one a line, answering each on
/// `output`, until `input` ends.
pub fn serve(
    input: impl BufRead,
    mut output: impl Write,
    build: &dyn Fn() -> Result<Engine, String>,
    repository: Option<&Path>,
) {
    for line in input.lines() {
        let Ok(line) = line else {
            return;
        };
        if line.trim().is_empty() {
            continue;
        }
        let reply = match serde_json::from_str::<Value>(&line) {
            Ok(request) => answer(&request, build, repository),
            Err(error) => Some(json!({
                "jsonrpc": "2.0",
                "id": null,
                "error": { "code": -32700, "message": error.to_string() },
            })),
        };
        if let Some(reply) = reply {
            if writeln!(output, "{reply}")
                .and_then(|()| output.flush())
                .is_err()
            {
                return;
            }
        }
    }
}

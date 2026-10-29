//! `flume mcp` (57, 60; design 17): flume's status served over the Model
//! Context Protocol on standard input and output, one JSON-RPC message a
//! line, with one tool, `status`, which answers what `flume status
//! --json` answers over the same state and settles nothing. Each call
//! builds the core afresh, as each command does.

use super::place;
use flume_core::{Answer, Engine};
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::path::Path;

/// The protocol's version the server answers where the client names
/// none.
pub const PROTOCOL: &str = "2025-06-18";

/// The one tool.
fn tools() -> Value {
    json!([{
        "name": "status",
        "description": "Every fact of a flume place, or of every place on this host, and of the host, as `flume status --json` gives them. Reads; settles and changes nothing.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "place": {
                    "type": "string",
                    "description": "A place's location, or `<kind>--<name>` in the repository the server runs in; every place where none is given."
                }
            },
            "additionalProperties": false
        },
        "annotations": { "readOnlyHint": true }
    }])
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
            "serverInfo": { "name": "flume", "version": env!("CARGO_PKG_VERSION") },
        }),
        "ping" => json!({}),
        "tools/list" => json!({ "tools": tools() }),
        "tools/call" if request["params"]["name"] == "status" => {
            status(&request["params"]["arguments"], build, repository)
        }
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

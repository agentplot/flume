# flume reports each place as a trace

## Why

flume alone knows when each stage of a place started and ended, how it
ended, and what the place waited on between stages and for how long:
its caller, the host's room, the merge queue or a person. Status gives
it while the place stands (56) and nothing keeps it once the place is
tidied (62). The agent program already reports its own turns and
tokens, and sluice its rounds, where a host turns their telemetry on;
a backend can join neither to a place, nor see the time between an
agent's turns.

Record 0067 settles how flume reports a place, and the statement holds
it at boomdrivedev/blueprints main 81288ba (design branch
`design/flume-and-sluice-telemetry`, a1a92d4): clauses 216 to 220,
C.7's `[telemetry]` and S129, with rule core.56. flume sends nothing
today.

## What Changes

- **The host's `[telemetry]` (216, C.7; core.45, core.56).** A table of
  the managed layer alone, in sluice's words for its own: `send`,
  `collector` (an `http://` address on the host's loopback with a port
  and at most a path, which a table that sends must name), `service`,
  `traces`, `metrics`, `logs` and `[telemetry.attributes]`. No table,
  no telemetry. `OTEL_SDK_DISABLED` and `OTEL_<SIGNAL>_EXPORTER=none`
  turn telemetry or one signal off, and no other variable of
  OpenTelemetry's is read, so a caller's or an agent's collector never
  receives flume's. The local layer setting it is a local fault, as
  any key outside C.7's list is. Status gives the table with its layer.
  The host module declares it.
- **One trace per place (217).** The held state names a trace and the
  place's span at the launch, each start's span at the start and each
  wait's span when a settle first reads the wait. A span goes once,
  from the command whose write records its end, with the times the
  held state keeps: the place's from its launch to its tidy, drop or
  done; each start's from its start to how it ended; each wait's from
  the settle that read it to the one that read it over, naming whom it
  waits on and which move, and whether a hold stood over it and
  whether it stalled. A record goes at each change: a start, an end, a
  wait begun or ended, a hold, a stall, a stuck stage.
- **What each signal names (218).** Every span and record names the
  place's repository, kind with its version, name, base, stream, intent
  and mark, and its stage and start; a start's span names its program,
  its conversation and its account. None holds words of a brief, a
  prompt, a stop-short, an answer, a hold or a failure, or a path.
  flume's own warnings and errors go as records in their own words,
  each path and each value shaped as a credential replaced.
- **Each agent gets the place (219).** While flume sends, an agent it
  starts gets the place's facts in `OTEL_RESOURCE_ATTRIBUTES`, over
  what its environment gives there, and, while traces go, its start's
  `TRACEPARENT`. Neither is in the start's record, so telemetry changes
  no started ack (161, 216).
- **Metrics by kind and stage (220).** Each start's time and each
  wait's time, the stalls and stuck stages counted, and at a settle of
  every place on the host the places standing and each stream's agents
  running and room; none names a place, intent, mark, conversation or
  account. A command sends what changed in them since it began, once,
  as it ends.
- **Nothing reads it back (216).** Signals go after a command's answer,
  within a bound, and one that cannot be sent is lost. Status and the
  list send nothing.
- **The OpenTelemetry crates** at sluice's versions, with OTLP over
  HTTP and protobuf, in the binary alone; the core hands its signals
  across one interface and names no crate of them.
- **The clause list** reads the statement at 81288ba.
- **The site** says what flume reports off the host and documents
  `[telemetry]`.

## Capabilities

### New Capabilities

- `telemetry`: what flume sends, to where, when, naming what, and what
  it holds back; each agent given its place's facts and its start's
  context.

### Modified Capabilities

- `host-configuration`: `[telemetry]`, a table of the managed layer
  alone, read and refused as C.7 says, and given by status.
- `held-state`: a place's document names its trace and its place's
  span, each start's span and each open wait.
- `start-integrity`: what flume sets for an agent's telemetry is in no
  start's record.
- `host-module`: the module declares `[telemetry]`.
- `seam-fakes`: a fake that keeps what the core reports.
- `agent-profile`: Claude Code's conversation read from its word that
  it started, for a start's span.

## Impact

- `crates/flume-seams/src/` (the signals and their interface),
  `held.rs` (the trace, spans and open waits).
- `crates/flume-core/src/`: a module for the signals, with the launch,
  start, settle, wait, stall, stuck, hold, end and tidy paths handing
  them on once their write lands; the agent's environment at a start.
- `crates/flume/src/telemetry.rs` (new), `host/config.rs`,
  `main.rs`/`lib.rs` (set up once, flushed after the answer).
- `crates/flume-fakes/`, `crates/flume-conformance/` (scenarios for 216
  to 220 and S129; the clause list at 81288ba).
- `Cargo.toml`, `Cargo.lock`: `opentelemetry`, `opentelemetry_sdk`,
  `opentelemetry-otlp`.
- `nix/modules/options.nix`, `nix/tests/every-key.nix`.
- `site/src/content/docs/reference/configuration.md`, `state.md`.
- `docs/adr/0067-…`: the construct stage's choices, amended in.
- A host with no `[telemetry]` sees no change.

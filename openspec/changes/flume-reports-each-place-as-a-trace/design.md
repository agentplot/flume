# Design

The statement is read at boomdrivedev/blueprints main 81288ba, which
holds a1a92d4: 216 to 220, C.7's `[telemetry]` and S129. Record 0067
decides the shape; rule core.56 binds every part of it. The pages of
`docs/architecture/` this change touches are `core.md` alone.

## 1. Scope

flume sends a place's trace, its records and its metrics to the host's
collector, and hands each agent its place's facts and its start's
context. It sends nothing a host has not turned on, reads nothing back,
and decides nothing by it (216; core.56).

Not in scope: status giving a place's trace, which 56 does not ask
(cheap to add once it does); a plan stage's `review` wait, since flume
runs no plan stage yet (163, unbuilt); sluice's own nesting under an
agent's context, which is sluice 153's.

## 2. One design with sluice

sluice already sends telemetry (sluice records 0014, 0022, 0106), and
C.7 writes flume's `[telemetry]` in sluice's words. flume takes
sluice's form and its crates, and differs only where the statement
does:

- The table's keys, their defaults, `[telemetry.attributes]` with
  `host.name` overriding flume's reading, and a table that sends naming
  its collector or refused, are sluice's (sluice record 0106).
- `opentelemetry`, `opentelemetry_sdk` and `opentelemetry-otlp` at
  sluice's versions, 0.33, OTLP over HTTP in protobuf through the
  crate's blocking reqwest client with no TLS feature, since the
  collector is on the host's loopback.
- Every `OTEL_EXPORTER_OTLP_*` taken out of flume's own environment
  before an exporter is built, as sluice found the crate adds the
  environment's headers to every request whatever the code gives it
  (sluice record 0106, as amended).
- Where flume differs, the statement says so: no command flag and no
  variable of flume's own turns telemetry on (216); no collector is
  taken from the environment, with or without a table (216; core.56);
  a collector is on the loopback alone (C.7).

The owner of the table's form is sluice's statement (sluice C.7);
flume's C.7 cites it. No crate is shared: each tool reads its own
host file.

## 3. The table (216, C.7; core.45, core.50)

`HostConfig` gains `telemetry: Option<TelemetryTable>`, read by
`telemetry_of` in `crates/flume/src/host/config.rs` beside
`ends_expire_of`, and refused, naming the key, as every managed key is:

| key | read | where unset |
|---|---|---|
| `send` | a boolean | `false` |
| `collector` | `http://`, a host of the loopback (`localhost`, an address of `127.0.0.0/8`, `[::1]`), `:` and a port, at most a path; its final `/` taken off | none; refused where `send` is `true` |
| `service` | a non-empty string | `flume` |
| `traces`, `metrics`, `logs` | a boolean | `true` |
| `[telemetry.attributes]` | each a string, none shaped as a credential (core.50's shapes) | none |

Any other key of `[telemetry]` is refused naming it, as `only()` refuses
a stream's. The local layer setting it is a key outside `LOCAL_KEYS`,
so the whole local layer is not taken, as C.7 says of any such key; no
code changes for that. `values_of` gives each value with its layer.

`Host::telemetry()` in `crates/flume-seams/src/host.rs` gives the table,
none by default; `RealHost` and the fake host answer it.

## 4. Which signals go (216)

`crates/flume/src/telemetry.rs` chooses, once, as a command begins:
nothing where the table is absent or its `send` is not `true`; else
each signal the table leaves on and OpenTelemetry's variables leave on,
`OTEL_SDK_DISABLED` at `true` turning off every one and
`OTEL_<SIGNAL>_EXPORTER` at `none` that one, each read trimmed and in
any case, as sluice's `Signals::read` reads them. No other `OTEL_*`
variable is read. The resource is `service.name`, `service.version`,
`host.name` unless the table's attributes name one, and those
attributes; `OTEL_RESOURCE_ATTRIBUTES` and `OTEL_SERVICE_NAME` are not
read (sluice record 0106).

The choice reaches the core as `Sends { traces, metrics, logs }`
through the interface of design 6, so the core gives an agent its
variables only while flume sends (219).

## 5. The held state (217; core.16, core.17)

`crates/flume-seams/src/held.rs` gains, each
`#[serde(default, skip_serializing_if = …)]`:

- `HeldState.trace: Option<TraceHeld>`: `trace`, 32 hex digits; `span`,
  the place's, 16; `from`, the launch's time; `ended`, the time the
  place's span went, none until then.
- `StartHeld.span: Option<String>`: the start's span, 16 hex digits.
- `StartHeld.asking_since: Option<Seconds>`: since when a settle read
  the agent asking a person (90), none while it is not.
- `HeldState.waits: Vec<WaitHeld>`: each open wait, `span`, `on`
  (`caller`, `host`, `merge-queue`, `person`), `move`, `stage`, `since`,
  `held` and `stalled`.

The ids come from the host's source of secrets, so the fakes give
fixed ones. A launch names the trace and the place's span as it writes
the place's first document. A document written before gets them at its
next write, its place's span from that write's time, since flume never
kept when it was launched; a start written before gets its span then,
from its `started_at`.

## 6. The signals and their interface (core.1, core.2, core.14)

`crates/flume-seams/src/telemetry.rs` holds what the core hands on,
naming no crate of OpenTelemetry's:

- `Signal::Span { trace, span, parent, name, from, to, attributes }`;
- `Signal::Record { trace, span, at, event, attributes }`;
- `Signal::Host(HostReading)`: the places standing by kind, state and
  whom they wait on, and each stream's agents running and room;
- `Signal::Warning { at, words }`;

and the trait `Telemetry { fn sends(&self) -> Sends; fn send(&self,
signal: Signal); }`, an `Off` that sends nothing, and the attribute
keys of design 8. `Engine` holds one beside its seams; `profiles::core`
gives it the binary's, and the fakes a keeper.

## 7. Where signals are made (217; core.17)

`Engine::update` is the one place a place's document is written, and
`stamp` already compares a write's next state with the state it read.
`stamp` gains a step, `trace`, that compares them once more and returns
the signals the write makes; `update` hands them to the interface only
once the store answers `Written`, and drops them where it answers
`Moved`, so a refused write sends nothing and the retry decides again.
`Engine::create` does the same for a launch. Nothing else hands a
place's signal on, so no act needs a call of its own.

What `trace` reads between the two states:

| change | signal |
|---|---|
| a `StartHeld` with no span | its span named; a `start` record |
| a start's stage left a live state (`Starting`, `Started`, `Begun`, `Stuck`) for an ended one, or its start replaced | the start's span, from `started_at` to the write's time, its end; an `end` record |
| a stage entered `Stuck` | a `stuck` record |
| a hold put (`holds` gained one) | a `hold` record naming the move held |
| `waiting.stalled` became true | a `stall` record |
| the waits read from the next state differ from `waits` | each gone one's span, from its `since` to now, and a `wait.ended` record; each new one opened with a span and a `wait.begun` record |
| the place's phase left live (`Over` dropped or done; `Gone` or `Tidied` for one that landed) | the place's span, from `from` to now, and `ended` set |

The waits read from a state:

| from | `on` | `move` |
|---|---|---|
| `waiting` whose `why` is `Dialog` | `person` | `dialog` |
| `waiting`, any other `why` | `caller` | `why` in its serde form: `on_hold`, `answer`, `lost`, `never_begun`, `unready`, `refused`, `stopped`, `past_bound`, `submission`, `delivered`, `unsubmitted` |
| a stage `WaitingForRoom` | `host` | `room` |
| a submission sent and not yet read landed or refused | `merge-queue` | `landing` |
| a live start whose `asking_since` is set | `person` | `asking` |

A wait's `held` becomes true while a hold stands over its move, and its
`stalled` while `waiting.stalled` does; each is named on its span when
it ends. A wait whose `on`, `move` or `stage` changes is a new one.

The asking mark is read at settles already (`read_end`, through
`read_agent`). Where it reads `Yes` and the live start has no
`asking_since`, or anything else and it has one, the settle writes the
change, so the person's wait opens and ends at the settles that read it.

A start's span is named as `stamp` writes its `StartHeld`, before
`open` builds its command, so its `TRACEPARENT` names a span the held
state already holds.

How a start ended: `delivered`, `short` (with `needs`, `answer` or
`found`), `finished`, `lost` and `stopped` as 217 names them; a start
whose prompt was never begun ends `never-begun`, and one a fresh start
took over without ending its stage, as after a spent account, ends
`replaced`. 217 lists the ends of a start once begun; these two name
the others rather than miscount them as one of those (record 0067's
amendment).

## 8. What each signal names (218, 220; core.30, core.55, core.56)

| key | value | spans and records | metrics |
|---|---|---|---|
| `flume.repository` | the address, else the folder's last name | yes | yes |
| `flume.kind`, `flume.kind.version` | the kind's name and version | yes | kind only |
| `flume.place` | the place's name | yes | no |
| `flume.base` | the base | yes | no |
| `flume.stream` | the stream | yes | yes |
| `flume.intent` | the launch's name, else `base:<repository>:<base>` | yes | no |
| `flume.mark` | the mark, where it has one | yes | no |
| `flume.stage`, `flume.start` | the stage, the start's number | where it concerns one | stage only |
| `flume.repetition` | the repetition | a start's span | no |
| `flume.program` | the agent's program | a start's span | no |
| `session.id` | the conversation's id, as the agent seam reads it | a start's span | no |
| `flume.account` | `<pool>/<account>` | a start's span | no |
| `flume.start.end`, `flume.start.needs` | how it ended | a start's span | `end` |
| `flume.wait.on`, `flume.wait.move`, `flume.wait.held`, `flume.wait.stalled` | design 7 | a wait's span | `on`, `move` |
| `flume.state` | the place's state in the list's words | no | the host's places |
| `event.name` | the record's change | records | no |

`session.id` is OpenTelemetry's own attribute and the one Claude Code
names its sessions by, so a backend joins the two by it; the core names
no tool in choosing it. A repository with no address is named by its
folder's last name, never its path (core.56). Metrics take their keys
from one list, so a key the table says no to never reaches one.

The agent seam gains `conversation(folder, agent) -> Option<String>`,
none by default, Claude Code's profile reading its `started.json`'s
`session_id`; a span names none where the profile reads none.

A record's body is its `event.name`, and no record or span holds a
start's `words`, a `WaitingOn`'s `words`, a hold's `words`, a brief, a
dialog's words or a path (core.56).

## 9. Each agent (219; core.10, core.30)

`Engine::open` adds, after the agent profile built the command and
before the terminal opens it, while `sends()` gives any signal:

- `OTEL_RESOURCE_ATTRIBUTES`: the pairs the command's environment gives
  it, else the login environment's (`Host::login`), with each pair whose
  key flume names taken out, then `flume.repository`, `flume.kind`,
  `flume.kind.version`, `flume.place`, `flume.base`, `flume.stream`,
  `flume.intent`, `flume.mark`, `flume.stage` and `flume.start`, each
  value percent-encoded as OpenTelemetry's resource reads it;
- while traces go, `TRACEPARENT`, `00-<trace>-<start's span>-01`.

They are set by the core, so every program and every terminal gets
them, the printed command's file among them. They are in no start's
record and never compared: telemetry changes no started ack (216). As
`FLUME_*` are, they are flume's way of starting the program and no
setting of a layer, so status gives no layer for them (core.30).

## 10. Metrics (220)

The binary derives every metric from the signals: a start's span
records its time in `flume.start.duration` (seconds, a histogram); a
wait's in `flume.wait.duration`; a `stall` and a `stuck` record count
in `flume.stalls` and `flume.stuck`; and `Signal::Host`, which only a
settle of every place sends (`Engine::settle(None)`, `settle_all`),
sets `flume.places` and `flume.stream.agents` and `flume.stream.room`.
The meter provider's exporter reads delta temporality, so each command
sends what it measured, once, at its end.

## 11. Sending (216; core.13)

The binary's interface keeps every signal in memory while the command
runs. `Outcome::exit` writes the answer, then calls `finish`: where any
signal is held and any signal goes, it takes `OTEL_EXPORTER_OTLP_*` out
of the environment, builds one exporter per signal sent with the
time left of a three-second bound, exports the spans in one batch built
from the held ids and times, the records in one, and the metrics by one
flush of a meter provider, and drops what remains when the bound ends.
An exporter's failure is silent. Nothing is written to disk to send
again (216).

Status, the list, the MCP server's status and every command that writes
no place's document make no signal, so they send nothing; `flume.log`'s
lines go as `Warning`s from `Host::log`, whichever command writes them.

A command's refusal goes as no record: its words may quote a tool's or
an agent's, which no scrub can tell from flume's own (core.56).
`Host::log` lines are flume's own words. Each core call of `Host::log`
is read in this change, and a line that carries a brief, a prompt, a
stop-short, an answer, a hold's words or a failure's output carries it
no more. A warning's words are scrubbed of each word that begins `/` or
`~/` and each value shaped as a credential, each replaced by `<path>`
or `<set>` (218, core.50's shapes).

## 12. Tests

- `crates/flume/src/host/config.rs`: the table read and refused.
- `crates/flume-seams/src/round_trip.rs`: the held state's new fields,
  and a document written before.
- `crates/flume/src/telemetry.rs`: the choice of signals, the resource,
  the attributes metrics keep, the scrub, and the export through the
  SDK's in-memory exporters (`opentelemetry_sdk`'s `testing` feature,
  a dev-dependency, as sluice's tests use it).
- `crates/flume-fakes/src/telemetry.rs`: a keeper that sends what it is
  told and keeps every signal.
- `crates/flume-conformance/src/engine/telemetry.rs`: the engine's
  scenarios, 216 to 220 and S129, over the fakes.
- `crates/flume/tests/telemetry.rs`: through the binary, a collector on
  the loopback of the test's own that keeps each request's path, headers
  and bytes, so the words a place holds are searched for in the bytes
  sent; a collector that refuses, and one that never answers.

## 13. The clause list (core.19)

`crates/flume-conformance/src/clauses.rs` reads main at 81288ba: 211 to
224 in their parts as the statement places them, S121 to S131 among the
scenarios. This change's scenarios cite 216 to 220 and S129. Every other
clause new to the list is named in `unbuilt()` with the record or change
that builds it, and one an earlier change built is named as `ORIGIN`
names S92, landed before the list held it.

## 14. The record

Record 0067 is amended with the choices of this design that a reader
would not find obvious, each with the path not taken: the table's
collector on the loopback alone and its faults refused (sluice's form);
the crates; the ids from the host's source; a document written before
given its trace at its next write; signals made in `stamp` and handed on
once the write lands; the ends `never-begun` and `replaced`; `session.id`
and the `flume.*` keys; a repository with no address named by its
folder's last name; the agent's variables outside its start's record;
the three-second bound; refusals sent as no record. No rule changes:
core.56 holds as written.

## Risks

- A span reaches the backend only at its end (record 0067). A record at
  each change shows a stage running now.
- A settle run from an agent's hook carries that agent's environment;
  where the host's settings for the agent program give
  `OTEL_TRACES_EXPORTER=none`, the spans that settle ends are lost, as
  216 reads the variable. The host so turns flume's traces off where it
  turns its program's off.
- The SDK's builder must take the held ids and times. Where the pinned
  0.33 does not, the export builds `SpanData` itself; the held state's
  form is the same either way.
- A three-second bound on a slow collector loses a host-wide settle's
  signals rather than holding the command.

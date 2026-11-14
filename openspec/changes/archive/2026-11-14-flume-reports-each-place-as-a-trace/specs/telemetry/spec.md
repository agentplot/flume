## ADDED Requirements

### Requirement: Telemetry is the managed layer's, off without it

flume SHALL send telemetry only where the managed layer's `[telemetry]`
gives `send = true`, and only to the collector that table names, at
its `/v1/traces`, `/v1/metrics` and `/v1/logs`, as OTLP over HTTP in
protobuf, each of traces, metrics and logs unless the table gives that
one `false` (216, C.7; core.56).

#### Scenario: No table, nothing sent

- **WHEN** a place is launched, runs its stages and lands on a host whose managed layer has no `[telemetry]`, a collector listening on the host's loopback
- **THEN** the collector receives nothing, and the place's agents are started with no `OTEL_RESOURCE_ATTRIBUTES` or `TRACEPARENT` of flume's (216)

### Requirement: OpenTelemetry's variables only turn signals off

Of OpenTelemetry's variables in its own environment flume SHALL read
`OTEL_SDK_DISABLED` at `true`, which turns every signal off, and
`OTEL_TRACES_EXPORTER`, `OTEL_METRICS_EXPORTER` and
`OTEL_LOGS_EXPORTER` at `none`, which turn that one off, each value
read trimmed and in any case; no variable SHALL turn a signal on, name
where one goes, or add a header to what is sent (216; core.56).

#### Scenario: A caller's collector is never flume's

- **WHEN** a settle runs with `OTEL_EXPORTER_OTLP_ENDPOINT` naming a second collector and `OTEL_EXPORTER_OTLP_HEADERS` giving a header, on a host whose `[telemetry]` sends to the first
- **THEN** every signal reaches the first collector with no such header, and the second receives nothing (216; core.56)

#### Scenario: OpenTelemetry's variables turn signals off

- **WHEN** a settle that ends a stage runs with `OTEL_TRACES_EXPORTER=none` on a host whose `[telemetry]` sends every signal
- **THEN** the collector receives the settle's records and metrics and no span; and with `OTEL_SDK_DISABLED=true`, nothing (216)

#### Scenario: A variable never turns a signal on

- **WHEN** a settle runs with `OTEL_TRACES_EXPORTER=otlp` on a host whose `[telemetry]` gives `traces = false`
- **THEN** no span is sent (216)

### Requirement: Nothing flume decides reads its telemetry back

No act, answer or end of flume's SHALL read its telemetry, wait on it
before its answer, or differ for it: a command SHALL write its answer
before it sends, SHALL send each signal once within three seconds of
its answer, and SHALL lose what is not sent by then, sending it again
at no later command. Status and the list SHALL send nothing (216;
core.13, core.56).

#### Scenario: A collector that refuses

- **WHEN** a place runs to its landing on a host whose `[telemetry]` names a collector that refuses every request
- **THEN** every answer, exit status, held state and end is the same as on a host with no `[telemetry]`, and the next command sends none of what was refused (216)

#### Scenario: A collector that never answers

- **WHEN** a settle that ends a stage runs against a collector that accepts the connection and never answers
- **THEN** its answer is written before any signal is sent, and the command exits within four seconds of its answer, sending nothing more (216)

#### Scenario: Status sends nothing

- **WHEN** status and the list are asked of a place whose stage has ended since the last settle, on a host that sends
- **THEN** the collector receives nothing from either, and the next settle sends the stage's span (216; core.15, core.52)

### Requirement: One trace per place

Every place SHALL have one trace and one span of its own, named in its
held state at its launch. Beneath the place's span, each start of a
stage SHALL be one span, and each wait of the place one span. Each span
SHALL go once, as the command whose held state's write records its end
ends, with the times the held state keeps; a write refused because the
version moved SHALL send nothing (217; core.16, core.17).

#### Scenario: The place's span

- **WHEN** a place launched at a time is settled to its landing and tidied, on a host that sends
- **THEN** the collector receives one span of the place's trace with no parent, from the launch's time to the tidy's, every other span of the place beneath it (217)

#### Scenario: A dropped place

- **WHEN** a place is dropped while its stage's agent works
- **THEN** its start's span ends at the drop as stopped, and its place's span ends at the drop (217)

#### Scenario: Two settles read one end

- **WHEN** two settles read one stage's delivery at once, one write landing and the other refused for its version
- **THEN** the collector receives the start's span once, from the settle whose write landed (217; core.17)

#### Scenario: A span read after a restart

- **WHEN** a stage delivers while no command of flume's runs, and the host's next settle after a restart reads it
- **THEN** that settle sends the start's span with the start's time and the time it read the delivery (217, 77)

### Requirement: A span for each start of a stage

Each start of a stage SHALL be a span from the start to its end, naming
the stage, the start's number, the repetition where the stage repeats,
the agent's program, the conversation its agent runs and the account it
runs under, and how the start ended: `delivered`, `short` with whether
it needs an answer or found work, `finished`, `lost` or `stopped`; a
start whose prompt was never begun SHALL end `never-begun`, and one a
fresh start took over without ending its stage `replaced` (217, 218).

#### Scenario: A start that ends short and its retry

- **WHEN** a code stage's first start stops short needing an answer and is retried, and its second start delivers
- **THEN** the trace holds two spans of the code stage beneath the place's, the first numbered 1 and ended `short` needing an answer, the second numbered 2 and ended `delivered` (217, S129)

#### Scenario: A lost start

- **WHEN** a start's agent is read lost
- **THEN** its span ends at the settle that read it `lost`, naming its conversation (217, 96)

#### Scenario: A repeating stage's repetitions

- **WHEN** a repeating stage runs three repetitions
- **THEN** each is a span of its own naming its repetition (217, 140)

### Requirement: A span for each wait

Each wait of a place SHALL be a span from the settle that first read it
to the one that read it ended, naming whether a hold stood over it and
whether it stalled. A wait whose whom or move changes SHALL end and the
next SHALL begin at the settle that read the change (217; 130).

#### Scenario: A wait that stalls

- **WHEN** a place waits on its caller under no hold's words past five minutes
- **THEN** its wait's span names that it stalled, and a record of the stall goes when the settle reads it (217, 130)

### Requirement: A wait names whom it waits on

A wait's span SHALL name whom the place waits on and for which move:
its caller, for a hold, an answer, a lost stage, a prompt never begun,
a readiness that failed, a last stage that does not land, or any other
move status gives; the host, for room; the merge queue, from the
submission to its landing or its refusal; or a person, while the agent
asks or its program shows a dialog (217; 70, 111, 130).

#### Scenario: A wait on the caller for an answer

- **WHEN** a start stops short needing an answer, the place waits forty minutes, and a retry is taken
- **THEN** the trace holds a span on the caller for an answer of forty minutes, from the settle that read the stop-short to the retry (217, S129)

#### Scenario: A wait on the merge queue

- **WHEN** a place's last stage delivers, is submitted, and lands fifty minutes later
- **THEN** the trace holds a span on the merge queue from the settle that submitted it to the one that read it landed, naming that it landed (217, S129)

#### Scenario: A wait for room

- **WHEN** a place's next stage waits for its stream's room and starts at a later settle
- **THEN** the trace holds a span on the host for room, from the settle that first left it waiting to the one that started it (217, 70)

#### Scenario: A person while the agent asks

- **WHEN** a stage's agent asks a question of its terminal's person and is answered at a later settle
- **THEN** the trace holds a span on a person from the settle that read it asking to the one that read it working (217, 90)

### Requirement: A record at each change

Each change of a place SHALL also go, when a settle or an act reads it,
as a log record in the place's trace, under the span it concerns: a
start, an end, a wait begun or ended, a hold put or ended, a stall and a
stuck stage. Each record's body SHALL be the change's name alone (217,
218).

#### Scenario: A stage running now

- **WHEN** a stage started an hour ago runs on
- **THEN** the collector holds the start's record under the start's span, the start's span not yet received (217)

#### Scenario: A hold put

- **WHEN** a caller holds a place's submission with words
- **THEN** a record of the hold goes under the place's span naming the move held, and no record holds the hold's words (217, 218)

### Requirement: Every signal names the place and none of its words

Every span and record SHALL name its place's repository, kind and kind's
version, name, base, stream, intent and mark, and the stage and start it
concerns. None SHALL hold any words of a brief, a prompt, a stop-short,
an answer, a hold or a failure, a path, or an agent's output. flume's
own warnings and errors SHALL go as records in their own words, each
path and each value shaped as a credential replaced (218; core.30,
core.50, core.56).

#### Scenario: S129's words stay on the host

- **WHEN** S129 runs on a host that sends, its stop-short's words and its brief holding a distinctive phrase and a path
- **THEN** no span, record or metric the collector receives holds the phrase or the path, and every span and record names the place's repository, kind, version, name, base, stream, intent `accounts` and mark (218, S129)

#### Scenario: A warning in flume's words

- **WHEN** a settle warns that a teardown failed in a folder, the failure naming a token shaped as a credential
- **THEN** a record of the warning goes with flume's words, the folder's path and the token each replaced (218)

### Requirement: Each agent is given its place and its start

While flume sends, each agent it starts SHALL be given `OTEL_RESOURCE_ATTRIBUTES` holding its place's facts, its stage and its start, after the pairs its environment gives that variable, any pair of the same key taken out of those; and, while traces go, `TRACEPARENT` naming its start's span in its place's trace. flume SHALL set no other variable of the agent's telemetry, and SHALL give neither while it sends nothing (219; core.30).

#### Scenario: The agent's variables

- **WHEN** a start runs on a host that sends every signal, its account giving `OTEL_RESOURCE_ATTRIBUTES=team=ops,flume.stage=x`
- **THEN** the agent is started with `OTEL_RESOURCE_ATTRIBUTES` holding `team=ops` and then the place's facts with the start's stage, once each, and a `TRACEPARENT` whose trace is the place's and whose parent is the start's span (219)

#### Scenario: Traces off

- **WHEN** a start runs on a host whose `[telemetry]` gives `traces = false`
- **THEN** the agent gets `OTEL_RESOURCE_ATTRIBUTES` and no `TRACEPARENT` (219)

#### Scenario: A command the agent runs nests under its start

- **WHEN** the agent runs a command that reads `TRACEPARENT` and sends a span to the host's collector
- **THEN** that span's parent is the start's span in the place's trace (219, S129)

### Requirement: Metrics by kind and stage, never by place

flume's metrics SHALL name a repository, a kind, a stage, a stream, how
a start ended, and whom a wait is on and for which move, and no place,
intent, mark, conversation or account: each start's time to its end;
each wait's time; the stalls and the stuck stages counted; and, at a
settle of every place on the host, the places standing by kind, state
and whom they wait on, and each stream's agents running and room. A
command SHALL send what changed in them since it began, once, as it
ends (220).

#### Scenario: A start's time by its kind and stage

- **WHEN** a settle reads a code stage's delivery an hour after its start
- **THEN** the command sends one measure of an hour of the start's time, named by its repository, kind, stage, stream and `delivered`, and by nothing that names its place (220)

#### Scenario: The host's places at a settle of every place

- **WHEN** a watch's settle of every place runs over two places waiting on their caller and one running, in two streams
- **THEN** the command sends the places standing by kind, state and whom they wait on, and each stream's agents running and room; a settle of one place sends neither (220, 121, 208)

#### Scenario: No metric names a place

- **WHEN** S129 runs on a host that sends
- **THEN** no attribute of any metric received names the place's name, its intent, its mark, its conversation or its account (220, S129)

---
number: 67
title: flume reports each place as one trace of its stages and waits, and decides nothing by it
status: accepted
date: 2026-11-14
decision-makers:
- boomdrive-design
---

# flume reports each place as one trace of its stages and waits, and decides nothing by it

Status: accepted, boomdrive-design's own choice, made on the rules written and cheap to change.

Amended 2026-11-14: the construct stage of the change `flume-reports-each-place-as-a-trace` made these choices of its own, on this record, clauses 216 to 220 and C.7 at blueprints main 81288ba, and sluice's records 0014, 0022 and 0106, each cheap to change (that change's design):
- **sluice's form and crates.** `[telemetry]` reads as sluice's does, and a table that does not read is refused naming the key, as every managed key of flume's is. `opentelemetry`, `opentelemetry_sdk` and `opentelemetry-otlp` at sluice's 0.33, through the crate's blocking client with no TLS, with every `OTEL_EXPORTER_OTLP_*` taken out of flume's environment before an exporter is built, as sluice found the crate adds their headers. Not taken: OTLP written by flume's own code over an HTTP client of its own, which would be a second way to send in the family; and a table that does not read taken as telemetry off, which sluice's record 0106 turned down.
- **The collector on the loopback.** `localhost`, an address of `127.0.0.0/8` or `[::1]`, with a port, as C.7 says. Not taken: any host, as sluice's reader takes.
- **Signals made in one place.** The step that stamps each write of a place's document compares the state it read with the one it writes and makes that write's signals, which go only once the write lands. Not taken: a call at each act, which a new act would forget, and sending before the write, which a refused write would send twice.
- **Ids from the host's source of secrets,** so the fakes give fixed ones. A document written before is given its trace at its next write, its place's span from then. Not taken: a span from its first start's time, which would misstate its launch.
- **Two more ends of a start.** `never-begun` for a start whose prompt was never begun, `replaced` for one a fresh start took over without ending its stage, beside 217's five. Not taken: counting either as `lost` or `stopped`, which they are not.
- **The keys.** `session.id`, OpenTelemetry's own, for the conversation, the one Claude Code names its sessions by; every other fact under `flume.`. A repository with no address is named by its folder's last name. Not taken: the folder's path, which core.56 rules out.
- **The agent's variables outside its record.** `OTEL_RESOURCE_ATTRIBUTES` and `TRACEPARENT` are added once the agent profile built the command, after the pairs the agent's environment gives the first, and are never compared at its started ack, so telemetry changes no start. Not taken: recording them as `FLUME_*`'s digests are, which would refuse a start for telemetry's sake.
- **Three seconds.** A command sends after its answer, within three seconds of it, and loses the rest. A command's refusal goes as no record, since its words may quote a tool's or an agent's; flume's log lines go, scrubbed of paths and of values shaped as credentials. Not taken: an unbounded flush, which a collector that never answers would hold, and a refusal's words, which no scrub can hold to core.56.

Amended 2026-11-14: the code stage of the same change made these choices of its own, each cheap to change:
- **A wait on the merge queue names how it ended,** in `flume.wait.outcome`: `landed`, `failed`, `conflict`, `withdrawn` or `refused`, as S129's wait "naming that it landed" asks. Not taken: the outcome on the place's span alone, which a place sent back and submitted again would not show per wait.
- **An account's variable of either name stays out of the record too.** An `OTEL_RESOURCE_ATTRIBUTES` or `TRACEPARENT` an account gives is in no start's record, since flume rewrites the first while it sends, and is named with its layer among the start's settings as any account's variable is. Not taken: its digest in the record, which an agent started on a host that sends would never match.
- **An id is a fresh secret's SHA-256,** its first 32 hex digits a trace's and 16 a span's. Not taken: the secret's own text, which is no id OpenTelemetry takes.
- **The core hands on every signal; the binary keeps them only while it sends.** Not taken: the core asking what goes before each signal, which would leave the fakes nothing to keep where nothing goes.
- **A span only for a start that runs or ends.** A document written before names no span for a start whose end was read before it, and sends none for it. Not taken: spans for such starts, which would go now with an end read long before.
- **The names.** The place's span is `place`, a start's its stage's name, a wait's `wait on <whom>`. A hold's record names its move in `flume.wait.move`, `stage`, `submission` or `waiting`, and a hold's end goes as `hold.ended`. A repeating stage's repetition is its task group's number. Not taken: a key of the hold's own, which would be one more key a backend learns for one record.
- **The host's reading counts the places that stand,** a tidied place's kept document none of them, since its work is over and it holds no agent.

Deciders: boomdrive-design, in its talk with the user of 2026-11-13 on telemetry for flume and sluice.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` 216 to 220, C.7 and S129, on branch `design/flume-and-sluice-telemetry`; sluice's records 0014, 0022 and 0106 and its clause 151; boomdrive's record 0013 and rule core.28; Claude Code's monitoring documentation (`code.claude.com/docs/en/monitoring-usage`), read 2026-11-13.

## Context and problem statement

flume knows what no other tool on the host knows about a piece of work: when each stage of a place started and ended, how it ended, and what the place waited on between stages and for how long. It waits on its caller for an answer or a review, on the host for room, or on the merge queue. Status gives all of it while the place stands (56), and flume keeps none of it once the place is tidied (62). Nothing outside the host sees it.

The rest of a place's time is already reported elsewhere. The agent program exports its own turns, model calls and tool calls, where the host turns that on (boomdrive record 0013), and sluice exports its rounds (sluice record 0014). A person reading a session, an intent or the merge queue in a backend can't join those to a place, nor see the time between the agent's turns.

What flume must decide:

- how a place's life is shaped as telemetry;
- what each signal names, so that the agent's and sluice's signals join it;
- what flume holds back, since flume runs with no daemon of its own (120) and every act of it is a short command.

## Considered options

**The shape of a place's life.**

1. **One trace per place (chosen).** Each start of a stage is a span, and so is each wait. A record goes at each change. The trace's id and the place's own span are named in the held state at the launch. Each span goes when its end is read, with the times the held state keeps, by whichever command of flume's reads it. So no long-lived process holds a span open.
2. **One trace per intent,** its id made from the intent's name, so that flume, sluice and the caller all add to one trace without passing it along. Not taken now. Such a trace lasts days and holds spans from tools that know nothing of each other. The intent's name is the caller's, and a backend groups an intent's places by that name without it.
3. **A span per agent turn,** from flume's own reading of the agent's state. Not taken: the agent program's own telemetry has each turn finer than flume's reading of the terminal, so it would double the volume with coarser data. flume reports the agent asking a person as a wait, since that is the time a person holds the place.
4. **One summary when the place ends.** Not taken: it shows nothing while a place stands, and nothing of one that never ends.

**What each signal names.**

1. **The place's facts on every signal, and handed to the agent it starts (chosen).** Those facts are its repository, kind, name, base, stream, intent and mark, and the stage and start. flume starts each agent with them and with its start's trace context, in OpenTelemetry's own variables. An agent program that exports telemetry then names the place in its own signals, and a command it runs that reads a trace context nests under the stage's start.
2. **The conversation's id alone,** joined in the backend. Kept as well, on each start's span, but on its own it leaves the agent program's metrics without the intent.

**What flume holds back.**

1. **No words, and few metrics (chosen).** Nothing of a brief, a prompt, a stop-short, an answer, a hold or a failure's output goes, and no path. flume's metrics name no place, intent, mark, conversation or account. Each of those goes on spans and records alone, so the volume of metrics grows with the kinds and stages a host runs, not with its places. Metrics go as what changed since the last send, from the command that read them. A gauge of what stands goes only at a settle of the whole host.
2. **Metrics by place.** Not taken: a series per place, sent once a minute for as long as it stands, is the volume that grows without bound.

## Decision outcome

Option 1 of each, written as clauses 216 to 220, C.7's `[telemetry]` and scenario S129, and as rule core.56. As in sluice (record 0014), telemetry is off unless the host's managed configuration turns it on. It goes only to a collector on the host, and nothing flume decides reads it back or waits on it. A collector that refuses costs nothing. A signal that can't be sent is lost, never sent again, and changes no act and no answer.

### Consequences

- Good: a backend shows a session as its stages, its turns and the time between them, and an intent as the stages and waits of its places, each wait saying whom it was on.
- Good: sluice's spans for a test run or a submission nest under the stage's start that ran them, where the agent program passes on the trace context.
- Good: the agent program's own token and cost counters name the place and its intent.
- Bad: a span reaches the backend only at its end, so a stage running now is seen from its start's record until it ends.
- Bad: an agent program that takes no trace context from its environment keeps its turns in traces of its own, joined to the place by the conversation's id and the place's facts.

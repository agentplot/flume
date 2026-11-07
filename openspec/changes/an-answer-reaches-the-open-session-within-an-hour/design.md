# Design

## Context

### What flume does today

On flume's main at 376c17c:

- `Engine::needs` (`crates/flume-core/src/agent_acts.rs`) takes a
  stop-short from any stage, a conversation's among them, ends the
  stage short with the agent's words and whether it needs an answer or
  found work, and leaves the agent live.
- The settle's step for a stage ended short (`settle.rs`) is
  `found_work` alone. It ends the agent once quiet and starts the fix
  round for found work from the kind's last stage. A stage that needs
  an answer takes no step, so its agent stays live, holding the host's,
  its stream's and its account's room, until a retry.
- `Engine::retry` (`acts.rs`) settles every place first where no
  caller keeps watch (`settle_attended`), and refuses where the place's
  agent is live and its stage started or began (`working`). Otherwise
  it ends the place's agent with `end_agent`, one not quiet only when
  forced, and starts the stage afresh. The fresh start's words are
  `answered`'s, "The question: <q>" then "The answer: <a>", kept as
  `StartHeld::words`.
- A retry of a conversation's stage reported lost (`StageState::Lost`)
  ends no agent and carries the lost agent's conversation (49, 179):
  its start waits for room with `CarryCause::Lost` while a process runs
  the conversation or `live` cannot read whether one does (207), takes
  the account the lost start ran under first, and is
  `StartReason::Resumed`, flume's words then the retry's. Where it
  cannot carry, it is `StartReason::Lost`, flume's words before the
  retry's. A carried start that does not begin (`uncarried.rs`) is
  ended once quiet and its stage started afresh with flume's words
  after the start that stopped, or with a lost conversation's retry's
  words.
- `read_end` (`end.rs`) reads a begun stage lost where its agent
  stopped, or where the agent in its pane is not the start's, one the
  terminal brought back bare with no started ack (77). Such an agent
  is left in a conversation's pane (`StageEnd::left_in_pane`), sent
  nothing and ended only by a caller's stop, drop or done (core.10),
  and in any other stage ended once quiet (`end_foreign`).
- `Engine::tell` delivers to any live agent that waits, is not asking
  and said it started, whatever its stage's state. A tell to a stage
  ended short reaches its agent and leaves the stage ended short, and
  the agent may then work on a stage nothing reads.
- `waiting_on` (`status.rs`) gives a stage ended short needing an
  answer as a retry of it, `Waits::Answer`, with the question.
  `refresh_waiting` (`held.rs`) keeps a move's since and its stall
  while `same_move` holds.
- `EndOf::key` (`crates/flume-seams/src/held.rs`) keys a stage's end by
  its stage, its start, its state and a stuck end's first quiet. A
  second stop-short of one start would read as the end recorded
  already, and a wait (`wait.rs`) answers the newest end of the
  stage's current start. Ends stay in `PlaceHeld::ends` for the place's
  life, each with its deliveries.
- `StartReason::words` (`start.rs`) gives a fresh start after a spent
  account its program cannot carry a conversation from (146, 179), or
  after an unsettled start (161), flume's own words alone, and drops
  the earlier start's.
- `Settings` holds `program`, `model`, `instructions`, `beside`,
  `mode`, `session` and `programs`, each taken whole by
  `Settings::under`, and `Engine::resolve` folds them over `Layer`'s
  eight layers, `launch`, `host-repository`, `host-kind`,
  `host-stream`, `host`, `repository`, `kind` and `default`. `--agent`'s
  fields are those seven (`FIELDS` in `commands/launch.rs`).
- The herdr profile types into a pane by `agent send-keys <pane>
  ctrl+c` and `agent prompt <pane> <text>`. The printed command's
  `send` refuses, so a tell is refused under it (C.5).
- The launch's stall bound is read by `seconds` in
  `crates/flume/src/commands/mod.rs`: `<n>`, `<n>s`, `<n>m` or `<n>h`.
- The herdr plugin keeps a watch that settles the host. The statement
  on blueprints main no longer has any terminal settle (122): a place
  moves between flume's commands at its agent's quiet (121) and at the
  settles of a caller keeping watch over the host (208), and
  `every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`,
  in code, takes the plugin's watch out.
- The walkthroughs (`crates/flume/tests/profiles/tools/walk.rs`) run
  flume's binary under the printed command, each agent the stand-in for
  Claude Code (`stand-in/claude.rs`), which takes its one prompt and
  ends its session after its turn. The herdr profile's tests
  (`tools/terminal_herdr.rs`) start a headless herdr server on a
  scratch socket and home, with a shell stand-in that reports its state
  to herdr.

### What the family does

flume's statement on agentplot/blueprints main resolves an agent's
program, model, effort (197), mode, instructions, own settings (195)
and time kept open (198) from the eight layers of clause 32: the
launch's settings; the host's for the place's repository, by kind and
then for every kind; the host's for the kind; the host's for the
place's stream (143); the host's for every agent; the repository's;
the kind's declaration; and flume's own default. The host's four give
everything but a program, and each setting is taken whole from the
first layer that gives it, a stage's own first within each.
`an-agents-program-settings-come-from-flumes-layers` (record 0043,
#128) built those layers on flume's main.

`a-lost-conversation-resumes-its-session` (record 0052, #130) is on
flume's main too: a retry of a conversation reported lost carries its
conversation, and an agent the terminal brought back bare in a
conversation's pane is left for a person (49, 77, 207). Record 0052
leaves a conversation let go after its stop-short's time to this
change (its option 3).

The changes in flight that touch the same code:

- **`an-agents-effort-is-a-setting-of-flumes-layers`** (record 0044;
  clause 197 on blueprints main), in code beside this one. It adds
  `Settings::effort`, read in every agents table and by `--agent
  [<stage>.]effort=<word>`, folded by `Engine::resolve` as the model
  is, with `from["effort"]` naming its layer.
- **`every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`**
  (record 0051; 122, 202 to 206 and 208 on blueprints main),
  in code. No terminal settles: the herdr plugin keeps no watch, and a
  caller keeping watch over the host settles every place on its bound
  (208). It touches `settle.rs`.
- **`every-question-a-place-asks-reaches-someone-who-answers-it`**
  (record 0039), designing. It gives each end a key for callers,
  `stage/<stage>/<start>/<state>` from `EndOf::key`, takes a retry's
  answer once by `--answers <key>`, records it on the stop-short's end
  it answers, and designs how an answer reaches a role's run that has
  ended. It relies on this change for an answer to reach an agent kept
  open, and says a tell that answers takes `--answers` too.

Beside them, `flume-opens-a-streams-multiplexer` (record 0037), every
task ticked, sends to a place through the terminal of its live start
(190). This change asks that same terminal whether it hands a running
agent words.

### What rules what

Record 0046 (accepted by the user on 2026-11-01) rules what this
change builds, and the user approved this design on 2026-11-06. The
statement says it on agentplot/blueprints main from c188bf3, as
decision 13 words it (core.19). Decision 14's amendment of clauses 49,
179 and 198, for a conversation's asking agent lost with its terminal
or its host, waits on blueprints for the user's word.

The rules this design relies on are:

- core.1 and core.2: the core names no terminal, and whether a terminal
  hands a running agent words is the terminal seam's `sends`.
- core.4 and core.34: `keep_open` is no field of the kind format, and a
  declaration that writes it is refused as one that writes `env` is.
- core.10: one agent per stage at a time, and none that is not quiet
  ended unless forced.
- core.11: an answer is sent as a tell is, only to a waiting agent that
  said it started, its input cleared first, received within 60 seconds
  or sent once more, and never into a question.
- core.13: no timer; an agent's time kept open is read passed at a
  settle.
- core.14: the JSON form only gains fields.
- core.16 and core.17: held state on the host by compare-and-set, and
  every step safe to repeat.
- core.24 and core.25: a move and its since; each end delivered once,
  a second stop-short an end of its own.
- core.26: an agent kept open counts toward the room until it is ended.
- core.27 as record 0046 amends it, and core.30's details as amended:
  `keep_open`'s layers.
- core.32: a spent account read from the host's documents.
- core.35: the site's neutral voice.

The one rules page is `docs/architecture/core.md`; this change touches
no area another page would own. Where flume has no rule, the kit
standard's code profile binds (kit:specs.3, kit:testing.5).

## Goals / Non-Goals

**Goals:**

- An answer that comes within its time reaches the agent that asked,
  with what it had read and its prompt cache, and the stage carries on
  there (41, 42).
- No asking agent holds room or an account past its time: flume ends it
  once quiet (70, 145).
- No answer is lost. One handed to a session is recorded on the
  stop-short it answers, one the agent does not take is refused with
  nothing typed, and one past the time is a retry, as today (42, 49).
- A caller's word for an answer stays what it was, a retry naming the
  stage with the answer, and a tell answers within the time too.
- `keep_open` resolves as clause 32 resolves every setting, in the same
  fold as the model and the effort, so no two changes resolve a
  setting two ways.
- Status says, of every place waiting on an answer, whether its agent
  is kept open and until when, or that it is ended (56, 130).

**Non-Goals:**

- A stage for review (163; record 0024), whose tell revises a plan in
  its session.
- A statechart kind's sessions (175), refused by flume today (core.41,
  core.42); its change fits them to clause 198 when it builds them.
- Reading the prompt cache. Whether a request read the cache is the
  agent's record's to say (152; record 0021). The time is a setting,
  never a reading.
- An agent kept open under the printed command, which types into no
  running agent (C.5).
- Carrying the ended agent's conversation to the fresh agent a late
  answer starts. Clause 179 carries a conversation only to a start 146
  makes, and record 0046 rules a fresh agent with the question and the
  answer.
- Ending an asking agent that is not quiet: flume never forces an end
  (31).
- Anything flume answers. The answer is the caller's (34; core.12).
- A notice when an agent stops being kept open. Status says it, and
  the stop-short's notice already gave each recipient its time.
- Answer keys, `--answers`, and who gave an answer: the questions
  change's (decision 12).

## Decisions

### 1. `keep_open`, one of clause 32's settings

`Settings` gains `keep_open: Option<String>`, the value as its layer
wrote it, read with none from a held state written before, written only
where set, and taken whole by `Settings::under`, beside the model.
`Engine::resolve` folds it with every other setting: taken whole from
the first layer that gives it, a stage's own first within each, and
`from["keep_open"]` names its layer and value as written (32, 56). The
start keeps the resolved value in `StartHeld::settings`, read again at
its stop-short. The host's tables are read at each start (32; record
0017), so a host that changes `keep_open` changes it from the next
start on, and an agent already started keeps its own.

`flume-seams` gains `read_span`, the reader `--stall-after` uses today,
moved from `crates/flume/src/commands/mod.rs`. It reads `<n>`, `<n>s`,
`<n>m` or `<n>h`, `<n>` digits, to `Seconds`, and refuses any other form
with words naming the four.

**Which layers give it.** The launch's settings (layer 1) and the
host's four (2 to 5), with flume's own hour, `KEEP_OPEN` in
flume-core beside `QUIET_LIMIT`, in the default layer (8):

| layer | where | `keep_open` |
|---|---|---|
| 1 | `flume launch --agent [<stage>.]keep_open=<time>` | given |
| 2 | `[repos."<repository>".kinds.<kind>.agents]`, its `.stages.<stage>`, then `[repos."<repository>".agents]` | given |
| 3 | `[kinds.<kind>.agents]` and its `.stages.<stage>` | given |
| 4 | `[agents.streams.<stream>]` | given |
| 5 | `[agents]` | given |
| 6 | a repository's `.config/flume.toml`, its `[agents]` and `[kinds.<kind>.agents]` | refuses the launch |
| 7 | a kind's declaration, its agents tables and a stage's | refuses the launch |
| 8 | flume's own | `1h` |

It differs from the model and the effort (197) in two ways, and only
those:

- **Which layers.** An effort is about the stage's work, which a kind's
  author knows. `keep_open` is about what an asking agent holds while
  it waits: the host's room, its stream's and its account's (70, 145),
  traded against how long its account's prompt cache lives (section 3).
  Those are the host's and its accounts', which a repository's file
  shared by every host that runs it, or a kind that travels between
  hosts, cannot know (158). So layers 6 and 7 carry none, as they
  carry no environment, stream, pool or account (32), and one that
  writes it refuses the launch naming the field (4, C.2). A caller who
  knows its answer comes quickly, or late, gives it at the launch.
- **One value.** An effort is its program's own word, so a layer may
  give a table of programs (197). `keep_open` is flume's own time, the
  same for every program and passed to none, so it is one value, and a
  table refuses the table's configuration or the launch, naming the
  table and `keep_open`.

Alternatives considered:

- **Every layer, as the effort.** Rejected for the reason above: a
  kind's `keep_open` would trade a host's room it cannot see.
- **The launch's and the host's kind tables alone.** Rejected: clause
  32 gives the host a table per stream and one for every agent, and a
  host whose room is tight says it once for a stream or for every
  agent.

**Where each layer is read.**

- The host's `agent_settings` (`crates/flume/src/host/config.rs`) reads
  `keep_open` in every agents table it reads, the managed layer and the
  local one over it (C.7; record 0032): a string `read_span` takes,
  kept as written. Anything else refuses the configuration, naming the
  table and the key, as a mode that fails does.
- `--agent [<stage>.]keep_open=<time>` gives it at the launch, checked
  by `read_span`, and `keep_open` joins the fields `--agent` names in
  its refusals (7). The launch's JSON form takes it as `Settings`
  serializes it.
- The kinds profile's `map_agent` refuses `keep_open` in every agents
  table of a declaration, a kind's, a stage's and a repository's, in the
  words "`<table>.keep_open`: how long an agent is kept open for an
  answer is the host's or the launch's to give, and no declaration
  gives one", so the launch is refused naming the field (4, 158, C.2).
  This is flume's refusal of a key a host owns, as for `env`: weir's
  crate returns the table as written, and flume defines no field of
  the format (core.4).

### 2. The terminal says whether it hands a running agent words

The terminal seam gains `sends(&self) -> bool`: whether it can type
words into a running agent's input, as a tell does (53, 95). The herdr
profile answers true. The printed command answers false, as its `send`
refuses every word (C.5). The fake terminal answers as scripted. The
core asks the terminal of the asking agent's start, the one a tell to
it goes through (190), and names no terminal (core.1, core.2).

### 3. A stop-short keeps its asking agent open

`Engine::needs` reads a stop-short that needs an answer, or found work
from a stage that is not the kind's last, which is taken as needing an
answer (131). Where the start's terminal `sends`, the end's
`StageEnd::open_until` is the time flume took the stop-short plus the
start's `keep_open`. Where it does not, `open_until` is none and the
agent is left as today: ended where a person ran it, or by a retry
(C.5). Found work from the kind's last stage starts the fix round as
before and keeps no agent open.

The agent stays live, holding its host's, its stream's and its
account's room (70, 145). Its stage reads ended short, waiting on its
answer (42, 130). It is never stuck, since its agent said something
(44). In a repeating stage the repetition that stopped short is the one
kept open, its group named by its end as today (142).

### 4. The answer handed to the open session

An agent is kept open while all of these hold: its stage is the
place's current one, ended short needing an answer; its `open_until` is
set and later than now; its start is live; and no settle has recorded
its close (decision 5). `Engine::kept_open` reads it from the held
state, beside `Engine::open_session`, which opens a stream's view. Each act reads it again after its own settle, so no act answers
an agent that settle just let go.

`Engine::answer(place, words, act, bound)` is the one path an answer
takes. It types `answered`'s text, the question then the answer as a
retry's prompt gives them (42, 49), into the session as a tell is
typed. `Engine::hand`, taken out of `tell`'s loop and shared with it,
does that. It waits until the agent waits, is not asking and said it
started, within the bound, ten minutes where none is given (54). It
types through the terminal's `send`, which clears the input first. It
reads `received` within 60 seconds of each sending, and sends once more
(38; core.11).

Once the answer is received, one write records it and carries the
stage on:

- the stop-short's end in `PlaceHeld::ends` gains its `answer`, an
  `AnswerHeld { words, act, at, session }`, `session` a `Handed {
  sent_at, received_at }`;
- the start's `answered` count goes up by one;
- the stage reads `Begun`, its `since` now and its `end` none;
- the start's `first_quiet` and `past_limit` are cleared;
- each hold on the stage goes, as a retry naming it ends one (47, 49).

The place is then settled. The ended-short end stays recorded, and
delivered, in `ends` (43). No agent starts and none is ended, and the
stage's start count does not move (48). Where the answer is not
received, or not delivered within the bound, the act is refused with
the tell's words. Nothing more is typed, nothing is recorded, and the
stage still waits on its answer (53, 54).

Two acts carry an answer:

- **A retry** naming the stage with words, as the current stage or by
  its name, while its agent is kept open. `Engine::retry` calls
  `answer` with `Act::Retry` before `working` and `end_agent`, and
  answers as acted (42, 49, 50). A retry with no words is no answer and
  runs as today: the agent is ended once quiet, or forced, and a fresh
  one starts with the question alone. A retry naming another stage, or
  the submission, runs as today. A retry naming a repeating stage kept
  open carries on its repetition, and never starts the stage's first
  group with a task unticked (142).
- **A tell** to a place whose current stage ended short needing an
  answer. While its agent is kept open, `Engine::tell` calls `answer`
  with `Act::Tell` and the tell's bound (53, 55). Once it is not, the
  tell is refused with nothing typed: "the stage <stage> of <place>
  waits on an answer and its agent is no longer kept open; give the
  answer as a retry: flume retry <place> --words <answer>" (53). So no
  tell reaches an agent whose stage ended short without carrying the
  stage on.

A caller that wants a fresh agent to take the answer while its agent is
kept open stops the agent first (51). `flume stop` ends it once quiet,
its stage still ended short, and the next settle reads it let go,
`ClosedBy::Stopped`.

### 5. The agent let go and ended

A new `crates/flume-core/src/open_sessions.rs` holds
`Engine::close_session`, the settle's step for a stage ended short
needing an answer, beside `found_work` (`settle.rs`), and
`Engine::kept_open` and `Engine::answer`. Where the stage's
`open_until` is set and its end records no close, it reads the agent
let go by the first of these that holds:

- `ClosedBy::Stopped`: the start is no longer live, as a caller's stop
  leaves it (51).
- `ClosedBy::AgentEnded`: the agent is read stopped, its pane gone or
  no agent listed in it with the shell foremost (96); or its own record
  says its session ended (134); or the agent in its pane is not the
  start's, one the terminal brought back bare with no started ack under
  the start's identity (77). Each is what a restart of the host or its
  terminal leaves.
- `ClosedBy::AccountSpent`: the start's account is spent. Either the
  agent's own record says so, which `record_spent` then writes to the
  host's documents as `read_spent` does, or the host's documents hold
  it spent because another agent reached its limit (146; core.32). An
  account past its pool's threshold is not spent, and lets no agent go
  (185).
- `ClosedBy::Time`: now is at or past `open_until`.

The causes read from the agent come before the time. A settle that
finds the time passed and the agent ended as well records the agent
ended, since that is what a later answer's retry acts on: a
conversation's agent lost with its host is carried (decision 6), and
one brought back bare in its pane is ended by no retry (core.10).

It records `StageEnd::closed`, a `SessionClosed { at, by, ended_at,
left_in_pane }`, in one write. Then, while the start is live:

- an agent read stopped is read ended at once;
- an agent brought back bare is never the stage's (35, 161). In a
  conversation's pane it is left there, sent nothing and ended by no
  settle, a caller's stop, drop or done ending it (77; core.10), and
  the close records `left_in_pane`. In any other stage it is ended once
  quiet, as `end_foreign` ends one (77, S116);
- the start's own agent, quiet, is ended through the terminal's
  `close`, never forced (31);
- one that is not quiet is left, and a later settle ends it once it is
  quiet.

Once the agent is ended, the start reads `live: false` and `ended_at`
is recorded. A close the terminal refuses is logged by `failed_end` and
tried again at the next settle. The settle then starts what waits for
room (70), so the freed room and account go to the next stage waiting.
The step is safe to repeat: a close recorded is never read again, and
an agent once ended is never ended again (74; core.17).

No process of flume's waits for the time (120; core.13), and no
terminal settles (122). An agent kept open is quiet, so its own quiet
brings no later settle (121). Where a caller keeps watch over the host
(208), its settles read the time passed within the watch's bound.
Where none does, flume runs attended, and the next command of flume's
that acts settles every place first (119); status settles nothing, so
until then status gives the agent kept open with its time past. A
`keep_open` of `0` lets the agent go at the first settle after its
stop-short, which the plugin's on-quiet hook runs as the agent goes
quiet (121, C.4).

### 6. A later answer is a retry

Once the agent is let go, a retry naming the stage with words runs as
before. `working` passes, since the stage is not running. `end_agent`
ends an agent not yet ended, once quiet or forced. The fresh start's
prompt carries the question then the answer as `StartHeld::words` (42,
49, 50). A tell is refused as decision 4 says.

A conversation's stage (109) whose agent was let go
`ClosedBy::AgentEnded`, lost with its terminal or its host, is the one
other case. Its retry goes on with the lost agent's conversation, as a
retry of a conversation reported lost does (49, 179; decision 14).
`Engine::retry` takes it as it takes a conversation's `Lost` stage: it
ends no agent, and one brought back bare in the pane is left as it is.
The start waits for room with `CarryCause::Lost` while a process runs
the conversation (207), takes the account the lost start ran under
first, and is `StartReason::Resumed`, flume's words then the retry's
words, the question and the answer. Where it cannot carry, it is
`StartReason::Lost`, flume's words before the question and the answer.
One let go another way and, before it was ended, brought back bare
in its pane by a restart (`left_in_pane`) is taken so too: it was lost
with its terminal or its host, and no retry ends the bare agent
(core.10). A conversation's agent let go by its time, by its account
spent or by its caller's stop, and ended, carries nothing, and its
retry is the fresh agent of this decision's first paragraph.

The place waits on the same move throughout: a retry of the stage,
`Waits::Answer`, since the stop-short. `same_move` reads the act, the
stage and why, so neither the move's since nor its stall moves when the
agent is let go. A place that stalled is not delivered a stall again
(130; core.24).

### 7. What carries on after an answer

- **The stop check.** The stage is begun, so `stop_words` sends its
  agent back while its deliverable is missing, as it does for any
  running stage (110). The plugin's Stop hook keeps no mark of a
  stop-short of its own: it asks flume's stop check, which reads the
  stage (C.4).
- **Its limits.** The quiet limit runs from flume's first reading of
  the agent quiet after the answer, since the answer clears
  `first_quiet` (44). Working past the limit runs from the latest of
  the stage's beginning, its last answer's `received_at` and the
  place's last head move (118).
- **A second stop-short.** `EndOf::Stage` gains `answered`, the start's
  `answered` count when `record_ends` read the end, written only where
  it is not zero. `EndOf::key` adds it where it is not zero. So a
  stop-short after an answer is an end of its own, recorded once and
  delivered once to each recipient, and every key written before reads
  as it did (43, 74; core.25). Its agent is kept open again for
  `keep_open` from its own time.
- **A wait.** `wait.rs` answers only an end of the stage's current
  start whose `answered` is that start's count now. A wait after an
  answer waits on the stage's next end, and never answers the
  stop-short its answer answered (43).
- **A fresh start flume makes on its own.** After a spent account its
  program cannot carry a conversation from (`StartReason::Spent`), or a
  start begun without its settings (`StartReason::Unsettled`), the
  fresh start's words are flume's own, then the earlier start's
  `words`, then the answer of each of that start's ends that took one
  in its session, as `answered` gives it, in order. Neither a retry's
  words nor an answer is lost with its agent (42, 146, 161). A start
  that carries the conversation (`StartReason::Carried`) carries them
  in it, and its words stay flume's alone (179).

### 8. Status, ends and the JSON form

The JSON form only gains fields (core.14):

- `WaitingOn` gains `open_until`, the time the agent is kept open until
  while it is, and `closed`, the `SessionClosed` once one is recorded;
  both are none for any other move.
- `StageStatus` gains `answers`, each answer its current start took in
  its session, in order, each with its question (56).
- `EndStatus` gains `open_until`: for a stop-short's end, the time its
  agent was kept open until when the end was read, and none where it
  was not kept open (43); and `answer`, the end's `AnswerHeld` once one
  is recorded.
- While the agent is kept open, the current agent is the asking one,
  with `keep_open` among its settings and its layer (56).

The person's form of status prints the move's line as now, then one of:

- "its agent kept open for the answer until <time>: retry with
  --words, or tell it";
- "its agent let go at <time>, its time passed" (or "its agent ended",
  "its account spent" or "its caller stopped it"), then "and ended at
  <time>", "and ended once quiet" or, for a conversation's agent
  brought back bare, "and left in its pane", then ": retry with
  --words".

Each answer prints on a line of its own, with the act it came by, when
it was received, and the question and the answer. A stop-short's notice
(`ends.rs`) adds "its agent kept open for the answer until <time>"
where its end gives one, and the inbox's line for the end, which gives
the notice's words, adds it too. Times print as status prints every
time.

### 9. Held state

Every new field is read with a default and written only where set, so
held state written before reads with no agent kept open, no answers and
no close (75, 103): `Settings::keep_open`; `StageEnd::open_until` and
`closed`; `StartHeld::answered`; `EndHeld::answer`; `EndOf::Stage`'s
`answered`; and `WaitingOn::open_until` and `closed`. `SessionClosed`
serializes `at`, `by` as `time`, `stopped`, `agent_ended` or
`account_spent`, `ended_at`, and `left_in_pane` only where it is true. `AnswerHeld` serializes `words`, `act` as the `Act` it came
by, `at`, and `session` with `sent_at` and `received_at`.

### 10. The stand-in under herdr and the walkthroughs

The stand-in for Claude Code (`crates/flume/tests/profiles/stand-in/claude.rs`)
gains three things, each named in its doc comment:

- **Its state to herdr.** Where it runs in a herdr pane, with
  `HERDR_PANE_ID` and `HERDR_BIN_PATH` in its environment, it reports
  `working` while a turn runs and idle once the turn ends. It reports
  by `pane report-agent <pane> --source <its own> --agent claude
  --state <state>` with the pane's own `HERDR_*` left out, as
  `terminal_herdr.rs`'s shell stand-in does, in the states the pinned
  herdr names (core.29).
- **Later prompts.** A script may be an array of runs, each run an
  array of acts; an array of acts is one run, as today. The first run
  is the start's prompt's. Where another run follows, the session stays
  open once the turn ends. The next prompt typed into its terminal, as
  the pinned herdr's `agent prompt` types it, up to the enter that
  submits it, runs UserPromptSubmit, is written to the transcript as a
  user turn, and makes the next run's acts. Where no run follows, the
  session ends as today. Under the printed command no prompt is typed
  into it, so a script of one run acts as today.
- **Scripts by start.** Where `STAND_IN_SCRIPTS` names a directory, the
  stand-in reads its script from `<stage>-<n>.json` there, by
  `FLUME_STAGE` and the start's number in `FLUME_AGENT`, so each start
  under herdr acts its own. `STAND_IN_SCRIPT` stands as today. It keeps
  each prompt it took in its log, so a walkthrough reads what reached
  the session.

`walk.rs` gains a host under the pinned herdr: `[host] multiplexer =
"herdr"`, a headless herdr server on the walkthrough's scratch socket
and home as `terminal_herdr.rs` starts one, the stand-in first on the
panes' `PATH`, and the walkthrough's account giving `STAND_IN_SCRIPTS`
beside its scratch `CLAUDE_CONFIG_DIR` and token. Every herdr call is
bounded and passes the recorded server's socket (core.29). The
walkthroughs in `crates/flume/tests/profiles/tools/open_sessions.rs`
run on that host in `flume:tools`, each a fix:

- **S4, an answer within the hour.** Code's first start stops short
  needing an answer, "Which file?". Status gives the move as a retry of
  code, its agent kept open until an hour after the stop-short, and
  `keep_open` from `default`. A retry with the words "notes.md" reaches
  the same pane, start 1, whose next run takes the question and the
  answer as its prompt. That run stops short again, "And its text?",
  an end delivered of its own. A tell, "Hello.", reaches the same
  session, and its last run commits. Code started once, status gave
  both answers with their questions and acts, and the place lands (42,
  49, 53 to 55, 198).
- **S104, the time passing, then a late answer.** The host gives
  `[agents] keep_open = "5s"` and `[host] agents = 1`.
  Code stops short, and a second fix waits for room. Once five seconds
  have passed, a settle records the agent let go by its time and ends
  it, its pane closed. Status says the place waits on an answer, its
  agent ended, and the second fix's code starts. A tell is refused,
  naming the retry. A retry with the answer starts code's second start,
  whose prompt carries the question then the answer; it commits and the
  place lands (42, 49, 53, 70, 198).
- **S105, the agent's session ended within the hour.** Code stops
  short, and its script holds no later run, so its session ends. The
  next settle records the agent let go, ended. A retry with the answer
  starts a fresh agent with the question and the answer, and the place
  lands (42, 96, 198).
- **S105, the account spent within the hour.** On a pool of two
  accounts, code's first start stops short, and the walkthrough writes
  the account spent into the host's documents, as another agent
  reaching its limit leaves it. The settle records the agent let go by
  its account and ends it. A retry with the answer starts code on the
  pool's second account with the question and the answer, and the
  place lands (42, 146, 198).

Each scenario of this change's specs over the fakes is an engine
scenario in `crates/flume-conformance/src/engine/open_sessions.rs`,
citing its clauses, its time moved by the fake clock (106). The
conformance suite's clause list gains 198, S104 and S105 at the
blueprints revision the build reads.

### 11. The site

- `guides/callers-and-holds.md`: stopping short, the agent kept open
  and until when, the answer by a retry or a tell, the agent let go, a
  later answer as a retry, and a tell refused then. Its line "A tell is
  never an answer" goes.
- `guides/agent-settings.md`: `keep_open` in the table of settings, the
  layers that give it, flume's hour, `0`, and the refusal in a
  repository's file or a kind.
- `guides/places-and-herdr.mdx`: an agent kept open under herdr, and
  none under the printed command.
- `reference/configuration.md`: `keep_open` in the host's agents
  tables.
- `reference/commands.md`: `--agent keep_open`, and the answer a retry
  or a tell gives.
- `reference/state.md`: a stage end's `open_until` and `closed`, an
  end's `answer`, a start's `answered`, and status's new fields.

All of it speaks to any host's operator and names no host or team of
ours (core.35).

### 12. Agreement with the changes in flight

- **Order.** This change's code builds on main as it stands once
  `an-agents-program-settings-come-from-flumes-layers` landed, which
  built the layers decision 1's table names, as the user's go-ahead of
  2026-11-06 has it. `an-agents-effort-is-a-setting-of-flumes-layers`
  is in code beside it. Each folds its setting in the same
  `Engine::resolve`, reads it in the same host tables and names it in
  the same `--agent` refusals, so whichever lands second merges the
  other's field beside its own in `Settings`, `Settings::under`,
  `agent_settings` and `FIELDS`, and the setting keeps one resolution.
- **Program settings.** `keep_open` is flume's own setting and never a
  key of a program's own settings: no profile reserves it, and a
  layer's `programs.<program>.settings.keep_open` is the program's key,
  passed to the program, which flume reads nothing into (195).
- **Effort.** Clause 197's resolution with decision 1's two
  differences. `keep_open`'s `from` and status lines follow the
  model's, as the effort's do.
- **Lost conversations.** A conversation's asking agent lost with its
  terminal or its host is let go `ClosedBy::AgentEnded`, one brought
  back bare left in its pane as `read_end` leaves one (77; core.10),
  and its answer's retry carries its conversation by #130's path
  (decisions 5, 6 and 14). Any other agent let go starts no carried
  start, so record 0052's option 3 stays untaken.
- **Settles.** An agent kept open is let go at a settle, and no terminal
  settles (122): a caller's watch (208) or flume's next acting command
  (119) reads its time passed, as decision 5 says.
  `every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`
  and this change both add to `settle.rs`'s step for a live place;
  whichever lands second keeps both. A base place runs no agent, so
  none is kept open there (202).
- **Questions.** An answer handed to a session is recorded on the
  stop-short's end it answers, `EndHeld::answer`, the field the
  questions change writes for a keyed retry's answer and reads to take
  an answer once. That change adds who gave it and the key; this one
  adds `session`. `EndOf::key` gains `answered` where it is not zero
  (decision 7), so the questions change's token for a stop-short after
  an answer names it too, `stage/<stage>/<start>/<state>/<answered>`,
  and a retry naming the answered stop-short's key is refused as
  answered. A tell that answers an agent kept open takes `--answers`
  once that change builds it. An answer to a role's run that has ended
  is that change's, and reaches the role's next run as a keyed retry.
- **Multiplexer.** `sends` is asked of the terminal of the asking
  agent's start, the one `flume-opens-a-streams-multiplexer` sends a
  tell through.

### 13. The statement's amendment

These amend agentplot/blueprints `design/flume/requirements.md` from
cd8fd73, and are on its main from c188bf3 (core.19). Clause 198 and
scenarios S104 and S105 are new, and every branch and place on the
host holds them as this change's on 2026-11-06. Each item gives the
clause's new text where it changes, or the words it gains.

- **Section 3, settings**: "its program, its model, its effort, its
  mode, its instructions, its own settings, its environment, where in
  the terminal it runs, such as beside a pane its caller names, and how
  long it is kept open for an answer once it stops short (198)."
- **Section 3, after stop short**: "**kept open**: of an agent that
  stopped short needing an answer, live and waiting for it in its own
  session until its time passes, it stops or its account is spent
  (198)."
- **4** gains, after "one whose kind names something a host owns,
  naming the field (158)": ", and one whose repository's settings give
  how long an agent is kept open, naming the field (198)"; a kind's is
  refused under 158.
- **32**: its first sentence reads "An agent's program, model, effort
  (197), mode, instructions, own settings (195) and time kept open
  (198) are resolved when it starts, each from the first of these that
  gives it:". Item 2 ends "for the model, the effort, the mode, the
  instructions, own settings and the time kept open alone". Item 8 reads "flume's own default, which
  names a program only for an agent with no pool (173), and keeps an
  agent open one hour (198)". The repository's settings and its kinds'
  carry "no environment, no stream, no pool, no account and no time an
  agent is kept open".
- **42**: "A stage that ended short needing an answer starts no next
  stage, whatever else exists. Its place waits for its answer or a
  drop, and the agent's words reach the place's recipients, and status,
  whole and as the agent wrote them. The answer, whoever gives it and
  however it reached the caller, is a retry naming the stage with the
  answer as its words (49), or a tell (53). While the stage's agent is
  kept open (198), flume hands it the question and the answer as a tell
  is handed (54) and records them on the stop-short they answer. The
  stage then carries on in that session under the same start, begun
  again, starting and ending no agent and counting no start (48).
  Otherwise
  the answer is a retry, and the fresh agent's prompt carries the
  question with the answer. A stage that ended short having found work
  is sent back as 131 says."
- **49**: its first sentence ends "..., as 40 and 47 allow; but a retry
  with words naming a stage whose agent is kept open for its answer
  (198) is that answer, which reaches that session as 42 says and
  starts no agent."
- **50**: "A retry ends the place's agent first, one that is not quiet
  only when forced (31), and is refused, naming the place, when the
  place's work is over. A retry that answers an agent kept open (42)
  ends no agent."
- **53**: "A tell hands the agent of a place's current stage its words
  as a prompt, or nothing at all. A tell to a place with no live agent,
  or whose agent is asking, is refused with nothing typed. So is a tell
  to a place whose stage ended short needing an answer while its agent
  is not kept open (198), naming the retry that answers it."
- **54** gains: "An answer handed to an agent kept open (42) is
  delivered as a tell is, within the same bound; one not delivered
  leaves the stage waiting on its answer, with nothing recorded."
- **55**: "A tell never starts, ends, holds or moves a stage, never
  counts as a retry or a finish, and never stands for a deliverable.
  The one exception is a tell that answers an agent kept open (198),
  which carries the stage on in that session (42)."
- **56**: after "the words of a stop-short, and whether it needs an
  answer or found work," it reads "and, for one needing an answer,
  whether its agent is kept open and until when, or when and why it
  was let go and whether it is ended (198), and each answer the stage's
  current start took in its session, with its question (42),".
- **95**: "**send**: given an agent and a text, the text delivered as
  input the agent receives, any text the agent holds typed cleared
  first, or a refusal; no answer, delivered or refused, leaves text
  typed for a hand (38, 53); and, given nothing, whether the terminal
  delivers text to a running agent at all (198)."
- **118**: "The limit runs from the stage's beginning, the last answer
  its agent took in its session (42), or the place's last head move,
  whichever is later."
- **130**: after "status says which move and since when" it reads ",
  and, for an answer, whether the stage's agent is kept open for it and
  until when (198); an agent let go leaves the move and its since as
  they were".
- **146**: "Otherwise the stage starts afresh, its prompt with flume's
  own words: that the stage's earlier agent stopped at its account's
  usage limit, and that what it committed and left is in the place;
  then the words the earlier start was given and each answer it took in
  its session (42), so none is lost."
- **158**: "A kind names nothing a host owns, whichever source declares
  it: no environment, account, pool, stream, location on a host or time
  an agent is kept open (32, 144, 198)." Its refusal reads "A kind's
  declaration that names an environment, a time an agent is kept open,
  or a program by a location, refuses the launch naming the field (4)."
- **161** gains, after "counting toward no bound of the kind's
  (112).": "Its prompt carries the earlier start's words and answers as
  146 says."
- **198** (new, A.6, after 42): "An agent that stops short needing an
  answer is kept open for it where its terminal delivers text to a
  running agent (95), and otherwise is not. It is kept open from its
  stop-short for the time its settings give (32), one hour where none
  gives one, the life of its program's prompt cache. That time is
  flume's own, the same for every program and passed to none. It is
  written `<n>`, `<n>s`, `<n>m` or `<n>h`, and `0` keeps no agent open.
  The launch's settings and the host's configuration give it. An agent
  kept open stays live, holding its room and its account (70, 145), and
  an answer that comes then reaches its session (42). It is let go once
  its time has passed. It is let go sooner once it is read stopped or
  its own record says its session ended (96, 134), as a restart of the
  host or its terminal leaves it. So it is once its account is spent
  (146). A settle reads it let go and records when and why. No process
  of flume's waits for the time (120). Once it is quiet, flume ends it
  (31), freeing its room and its account. The stage keeps waiting on
  its answer (42, 130). A stage carried on after its answer is a
  running stage: its stop check, its quiet limit and working past it
  hold from the answer's receipt (44, 110, 118), and its next
  stop-short is an end of its own (43), its agent kept open again."
- **C.2**: after "an `env` in a repository's `.config/flume.toml`
  refuses the launch naming the field (4), since an environment is an
  account's alone (144, C.7)" it gains "; so does a `keep_open` there
  or in a kind's declaration, since how long an agent is kept open is
  the host's or the launch's (198), and a launch gives it by `flume
  launch --agent [<stage>.]keep_open=<time>`". Its sentence on packages reads
  "A package's agents tables and a kind's declaration from any source
  carry no `env` and no `keep_open`, and name a program by its name
  alone (158)".
- **C.4, the send-back row**: "while the stage's deliverable does not
  exist, the agent has not stopped short since its stage last began,
  and no background task runs and no question is open".
- **C.5, the printed command**: its send reads "refused, so every tell
  is refused, no agent is kept open for an answer (198), and a prompt
  not received is reported never begun (38)".
- **C.7, the host's agents tables**: "each gives `model`, `effort`,
  `mode`, `instructions`, `programs` and `keep_open` (C.2, 198),
  `keep_open` one time, `<n>`, `<n>s`, `<n>m` or `<n>h`, and never a
  table of programs", the three tables that name no stage giving the
  same.
- **S4**: its last sentence becomes "The design agent's answer comes
  back within the hour, a retry of construct with the answer as its
  words. Construct's agent takes the question and the answer in its
  session, and carries on there under its first start and delivers
  (42, 49, 198)."
- **S104** (new): "A code agent stops short needing an answer on a host
  whose `[agents.streams.<stream>]` gives `keep_open = "10m"`, the host
  at its bound with a second place waiting for room. Status says the
  place waits on an answer, its agent kept open until ten minutes after
  the stop-short. No answer comes. The first settle after that time
  reads the agent let go by its time and, the agent being quiet, ends
  it, and the second place's stage starts in the room it freed. Status
  says the place still waits on an answer, its agent ended, since the
  stop-short. A tell of the answer is refused, naming the retry; the
  retry starts a fresh agent whose prompt carries the question and the
  answer (42, 49, 53, 70, 198)."
- **S105** (new): "A construct agent stops short needing an answer, and
  within its hour another agent of its account reaches the account's
  usage limit. The next settle reads the account spent and the agent
  let go, and ends it once it is quiet. A second place's construct
  agent stops short, and its pane is gone after its host restarts; the
  next settle reads it let go, ended. Each place waits on its answer
  with its agent ended. Each answer, a retry, starts a fresh agent with
  the question and the answer, the first on another account of its
  pool (42, 96, 146, 198)."

### 14. A conversation's asking agent lost with its host

A conversation's agent (109) may stop short needing an answer, as any
agent may, and be kept open. Within its time its terminal or its host
may lose it: its pane gone, or the terminal's restore bringing it back
bare, with no word that it started under its start's identity (77). Its
stage said something, so it is not reported lost (45), and clause 49
carries a conversation only for a stage reported lost. Yet its
conversation is lost as record 0052's six were, and a person may be
talking in the bare agent the terminal brought back.

So a conversation's agent kept open and let go `ClosedBy::AgentEnded`
is carried at its answer's retry as 49 carries a conversation reported
lost (decision 6). The retry ends no agent. Its start waits while a
process runs the conversation (207), takes the account the lost start
ran under first, and resumes the session with flume's words and then
the question and the answer. Where it cannot carry, the stage starts
afresh with flume's words before them. Let go any other way, the agent
carries nothing.

This is this change's own choice, written as record 0046's amendment
(proposed) and in core.27's details. The statement's amendment is on
the agentplot/blueprints branch
`design/flume-an-asking-conversation-lost-is-carried` at 6529c04, cut
from main at a4eeec3, committed and not pushed, for the user's word
(core.19):

- **49**: its carry sentence begins "A retry of a conversation's stage
  (109) reported lost (45), or whose agent kept open for its answer
  (198) was lost with its terminal or its host (77), goes on with the
  lost agent's conversation where its program can carry it (179)."
- **179**: "or a conversation's start reported lost" reads "or a
  conversation's start reported lost, or lost while kept open (49)".
- **198** gains, after "as a restart of the host or its terminal leaves
  it.": "A conversation's agent so let go is carried at its answer's
  retry (49), and one its terminal brought back bare is left in its
  pane (77)."

Alternatives considered:

- **A fresh agent with the question and the answer, as for any other
  stage.** Rejected: the retry would still wait for a bare agent a
  person may be talking in (77), then drop what the person said there
  and all the conversation held, which record 0052 brings back whole.
- **Every conversation's agent let go carried, by its time and by its
  caller's stop too.** Rejected as record 0052 rejects its option 3: a
  caller who stopped a conversation meant to, and record 0046 rules a
  fresh agent past the time.

## Risks / Trade-offs

- An agent kept open holds room for its whole time in the worst case: a
  host at its bound whose every agent is asking runs nothing else until
  they are let go. A host whose room is tight sets `keep_open` shorter
  for a stream or for every agent, or `0`, and status names every agent
  so held.
- The time is a setting, not a reading of the cache. Where an account's
  cache lives five minutes, an answer at minute ten reaches a session
  whose next request reads its context in afresh. The agent keeps its
  place in the work, but the request costs what a fresh agent's would.
  The host sets `keep_open` to its accounts' cache.
- A kind's author cannot ask for a longer or shorter time. A caller
  that knows gives it at the launch; a host gives it per repository and
  kind.
- An answer is handed only once the agent waits and is not asking. An
  agent that also put a question to a person in its pane takes no
  answer from flume until that question is answered: the act is refused
  with nothing typed, the stage still waits, and status says the agent
  is asking.
- A retry with words now waits for the agent as a tell does, up to ten
  minutes, before it answers. Before, it started a fresh agent at once.
- flume reads an agent let go only from what it reads of the agent. A
  pane that still shows a live agent whose own record says nothing
  reads kept open, as flume reads such a pane today (96, 134).
- The walkthroughs run under the pinned herdr with a stand-in, so they
  prove the session and the acts, never a real model's reading of its
  cache. Record 0021's cache use in status would show it.
- On a host no caller keeps watch over, an agent past its time holds
  its room and its account until flume's next acting command settles
  (119). A host that wants its room back on time keeps watch (208).
- The order of landing. The effort change is in code beside this one,
  and both add a field to `Settings`, `agent_settings` and `FIELDS`.
  Whichever lands second takes the other's in at its merge, as
  decision 12 says.
- Numbers. Clause 198, S104, S105 and record 0046 hold this change's
  text on every branch and in every place on this host on 2026-11-06,
  and no other draft takes them.

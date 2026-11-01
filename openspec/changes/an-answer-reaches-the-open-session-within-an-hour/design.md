# Design

## Context

flume's tree at 4b0ab8a takes a stop-short in `Engine::needs`
(`crates/flume-core/src/agent_acts.rs`). It ends the stage short with
the agent's words and whether it needs an answer or found work, and
leaves the agent live. The settle's step for a stage ended short
(`settle.rs`) is `found_work` alone. It ends the agent once quiet and
starts the fix round for found work from the kind's last stage. A
stage that needs an answer takes no step, so its agent stays live,
holding the host's, its stream's and its account's room, until a
retry.

`Engine::retry` (`acts.rs`) refuses where the place's agent is live and
its stage started or began (`working`). Otherwise it ends the place's
agent with `end_agent`, one not quiet only when forced, and starts the
stage afresh. The fresh start's words are `answered`'s, "The question:
<q>" then "The answer: <a>", kept as `StartHeld::words`. `Engine::tell`
delivers to any live agent that waits, is not asking and said it
started, whatever its stage's state. So a tell to a stage ended short
reaches its agent and leaves the stage ended short, and the agent may
then work on a stage nothing reads.

`waiting_on` (`status.rs`) gives a stage ended short needing an answer
as a retry of it, `Waits::Answer`, with the question. `refresh_waiting`
(`held.rs`) keeps a move's since and its stall while `same_move` holds.
`EndOf::key` (`crates/flume-seams/src/held.rs`) keys a stage's end by
its stage, its start and its state. A second stop-short of one start
would read as the end recorded already, and a wait (`wait.rs`) answers
the newest end of the stage's current start. `StartReason::words`
(`start.rs`) gives a fresh start after a spent account (`spent.rs`) or
an unsettled start flume's own words alone, and drops the earlier
start's.

The herdr profile types into a pane by `agent send-keys <pane> ctrl+c`
and `agent prompt <pane> <text>`. The printed command's `send` refuses,
so a tell is refused under it (C.5). The launch's stall bound is read
by `seconds` in `crates/flume/src/commands/mod.rs`: `<n>`, `<n>s`,
`<n>m` or `<n>h`. The walkthroughs (`crates/flume/tests/profiles/tools/`)
run flume's binary under the printed command. Each agent is the stand-in
for Claude Code, run as a person runs the printed line: it takes its one
prompt and ends its session after its turn. The live walkthroughs run
under herdr on a scratch server (`live/main.rs`).

Record 0028 (accepted on 2026-11-01) rules what this change builds.
The statement at blueprints 92afa99 does not yet say it, and decision
12 words its amendments.

The rules this design relies on are:

- core.1 and core.2: the core names no terminal, and whether a terminal
  hands a running agent words is the terminal seam's `sends`.
- core.4: `keep_open` is no field of the kind format, and a declaration
  that writes it is refused as one that writes `env` is.
- core.10: one agent per stage at a time, and none that is not quiet
  ended unless forced.
- core.11: an answer is sent as a tell is, only to a waiting agent
  that said it started, its input cleared first, received within 60
  seconds or sent once more, and never into a question.
- core.13: no timer; a session's time is read passed at a settle.
- core.14: the JSON form only gains fields.
- core.16 and core.17: held state on the host by compare-and-set, and
  every step safe to repeat.
- core.24: a move and its since.
- core.25: each end delivered once, a second stop-short an end of its
  own.
- core.26: the open agent counts toward the room until it is ended.
- core.27 as record 0028 amends it, and core.30's details as amended:
  `keep_open`'s layers.
- core.32: a spent account's pause read from the host's documents.
- core.35: the site's neutral voice.

The one rules page is `docs/architecture/core.md`; this change touches
no area another page would own. Where flume has no rule, the kit
standard's code profile binds (kit:specs.3, kit:testing.5).

This design departs from core.19 in one way, as records 0027 and
`_open.md`'s dialog item do. The record is accepted, and the
statement's amendment is the operator's to rule in agentplot/blueprints,
in the words decision 12 gives. Code of this change that lands before
that amendment stands ahead of those clauses, and `_open.md`'s item on
record 0028 says so until the statement does.

## Goals / Non-Goals

**Goals:**

- An answer that comes within its time reaches the agent that asked,
  with its context and its prompt cache, and the stage carries on there
  (41, 42).
- No asking agent holds room or an account past its time: flume ends it
  once quiet (70, 145).
- No answer is lost. One handed to a session is recorded with the
  stage, one the agent does not take is refused with nothing typed, and
  one past the time is a retry as today (42, 49).
- A caller's word for an answer stays what it was, a retry naming the
  stage with the answer, and a tell answers within the time too.
- Status says, of every place waiting on an answer, whether it reaches
  an open session and until when, or that its agent is ended (56, 130).

**Non-Goals:**

- A stage for review (163; record 0024), whose tell revises a plan in
  its session. It is unbuilt, and its own change builds it.
- Reading the prompt cache. Whether a request read the cache is the
  agent's record's to say (152; record 0021, unbuilt). The time is a
  setting, never a reading.
- A session open under the printed command, which types into no running
  agent (C.5).
- Ending an asking agent that is not quiet: flume never forces an end
  (31).
- Anything flume answers. The answer is the caller's (34; core.12).
- A notice when a session closes. Status says it, and the stop-short's
  end already gave each recipient the time it closes.
- Points (record 0025), unbuilt.

## Decisions

### 1. `keep_open`, a setting of three layers

`flume-seams` gains `read_span`, the reader `--stall-after` uses today,
moved from `crates/flume/src/commands/mod.rs`. It reads `<n>`, `<n>s`,
`<n>m` or `<n>h`, `<n>` digits, to `Seconds`, and refuses any other
form with words naming the four. `Settings` gains `keep_open:
Option<String>`, the value as its layer wrote it, read with a default,
written only where set, and taken whole by `Settings::under`.

The host's `agent_settings` (`crates/flume/src/host/config.rs`) reads
`keep_open` in `[kinds.<kind>.agents]`,
`[repos."<repository>".kinds.<kind>.agents]` and their
`.stages.<stage>` tables: a string `read_span` takes, kept as written.
Anything else refuses the configuration, naming the table and the key,
as a mode that fails does (C.7). `--agent [<stage>.]keep_open=<time>`
gives it at the launch, checked by `read_span`, and `keep_open` joins
the fields `--agent` names in its refusals (7). The launch's JSON form
takes it as `Settings` serializes it.

The kinds profile's `map_agent` refuses `keep_open` in every agents
table of a declaration: a kind's, a stage's, and the repository's for a
kind it does not declare. Its words are "`<table>.keep_open`: how long
an agent's session stays open for an answer is the host's or the
launch's to give, and no declaration gives one", so the launch is
refused naming the field (4, 32, C.2). This is flume's refusal of a key
a host owns, as for `env`: weir's crate returns the table as written,
and flume defines no field of the format (core.4).

`Engine::resolve` folds `keep_open` with the rest, taken whole from
the first layer that gives it, a stage's own first in each (32). Only
the launch's and the host's two can give it, since a declaration that
does is refused. Where none gives it, the default layer gives `1h`,
flume-core's `KEEP_OPEN`, one hour, beside `QUIET_LIMIT`.
`from["keep_open"]` is its layer and its value as written, so status
gives it with the agent's other settings (56). The start's settings
keep the resolved value, read again at its stop-short. The host's
tables are read at the start, as for every setting (32; record 0017),
so a host that changes `keep_open` changes it from the next start on.

### 2. The terminal says whether it hands a running agent words

The terminal seam gains `sends(&self) -> bool`: whether it can type
words into a running agent's input, as a tell does (53). The herdr
profile answers true. The printed command answers false, as its `send`
refuses every word (C.5). The fake terminal answers as scripted. The
core asks it where it would hold a session open, and names no terminal
(core.1, core.2).

### 3. A stop-short keeps its asking agent open

`Engine::needs` reads a stop-short that needs an answer, or found work
from a stage that is not the kind's last, which is taken as needing an
answer (131). Where the terminal `sends`, the end's
`StageEnd::open_until` is the time flume took the stop-short plus the
start's `keep_open`, read by `read_span`. Where the terminal does not,
it is none, and the agent is left as today: ended where a person ran
it, or by a retry (C.5). Found work from the kind's last stage starts
the fix round as today and holds no session open.

The agent stays live, holding its host's, its stream's and its
account's room (70, 145). Its stage reads ended short, waiting on its
answer (42, 130). It is never stuck, since its agent said something
(44).

### 4. The answer handed to the open session

A stage's session is open while all of these hold: the stage is the
place's current one, ended short needing an answer; its `open_until` is
set and later than now; its start is live; and no settle has recorded
it closed (decision 5). `Engine::open_session` reads it from the held
state. Each act reads it again after its own settle, so no act answers
a session that settle just closed.

`Engine::answer(place, words, by, bound)` is the one path an answer
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

- `StartHeld::answers` gains `AnswerHeld { question, answer, by,
  sent_at, received_at }`;
- the stage reads `Begun`, its `since` now and its `end` none;
- the start's `first_quiet` and `past_limit` are cleared.

The place is then settled. The ended-short end stays recorded, and
delivered, in `ends` (43). No agent starts and none is ended, and the
stage's start count does not move (48). Where the answer is not
received, or not delivered within the bound, the act is refused with
the tell's words. Nothing more is typed, nothing is recorded, and the
stage still waits on its answer (53, 54).

Two acts carry an answer:

- **A retry** naming the stage with words, as the current stage or by
  its name, while its session is open. `Engine::retry` calls `answer`
  with `Act::Retry` before `working` and `end_agent`, and answers as
  acted (42, 49, 50). A retry with no words is no answer and runs as
  today: the agent is ended once quiet, or forced, and a fresh one
  starts with the question alone. A retry naming another stage, or the
  submission, runs as today.
- **A tell** to a place whose current stage ended short needing an
  answer. While its session is open, `Engine::tell` calls `answer` with
  `Act::Tell` and the tell's bound (53, 55). Once the session has
  closed, the tell is refused with nothing typed: "the stage <stage> of
  <place> waits on an answer and its agent's session is closed; give
  the answer as a retry: flume retry <place> --words <answer>" (53). So
  no tell reaches an agent whose stage ended short without carrying the
  stage on.

A caller that wants a fresh agent to take the answer while the session
is open stops the agent first (51). `flume stop` ends it once quiet,
its stage still ended short, and the next settle reads the session
closed with its agent ended.

### 5. The session closed and its agent ended

A new `crates/flume-core/src/sessions.rs` holds `Engine::close_session`,
the settle's step for a stage ended short needing an answer, beside
`found_work` (`settle.rs`). Where the stage's `open_until` is set and
its end records no close, it reads the session closed by the first of
these that holds:

- `ClosedBy::Time`: now is at or past `open_until`.
- `ClosedBy::AgentEnded`: the start is no longer live, as a stop leaves
  it (51); or the agent is read stopped, its pane gone or no agent
  listed in it with the shell foremost (96); or its own record says its
  session ended (134), as a restart of the host or its terminal leaves
  it.
- `ClosedBy::AccountSpent`: the start's account is spent. Either the
  agent's own record says so, which `record_spent` then writes to the
  host's documents as `read_spent` does, or the host's documents hold
  it spent because another agent reached its limit (146; core.32).

It records `StageEnd::closed`, a `SessionClosed { at, by, ended_at }`,
in one write. Then, while the start is live:

- an agent read stopped is read ended at once;
- a quiet one is ended through the terminal's `close`, never forced
  (31);
- one that is not quiet is left, and a later settle ends it once it is
  quiet.

Once the agent is ended, the start reads `live: false` and `ended_at`
is recorded. A close the terminal refuses is logged by `failed_end` and
tried again at the next settle. The settle then starts what waits for
room (70), so the freed room and account go to the next stage waiting.
The step is safe to repeat: a session recorded closed is never read
again, and an agent once ended is never ended again (74; core.17).

No process of flume's waits for the time (120; core.13). Under a
terminal that keeps watch, the watch settles every place with an agent
in it on its bound (122). Under one that keeps none, the next command of
flume's settles it (119).

### 6. A later answer is a retry

Once the session has closed, a retry naming the stage with words runs
as today. `working` passes, since the stage is not running. `end_agent`
ends an agent not yet ended, once quiet or forced. The fresh start's
prompt carries the question then the answer as `StartHeld::words` (42,
49, 50). A tell is refused as decision 4 says.

The place waits on the same move throughout: a retry of the stage,
`Waits::Answer`, since the stop-short. `same_move` reads the act, the
stage and why, so neither the move's since nor its stall moves when the
session closes. A place that stalled is not delivered a stall again
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
- **A second stop-short.** `EndOf::Stage` gains `answers`, the count of
  answers the start had taken when `record_ends` read the end, written
  only where it is not zero. `EndOf::key` adds it where it is not zero.
  So a stop-short after an answer is an end of its own, recorded once
  and delivered once to each recipient, and every key written before
  reads as it did (43, 74; core.25). The new stop-short holds its own
  session open for `keep_open` from its own time.
- **A wait.** `wait.rs` answers only an end of the stage's current
  start whose `answers` is that start's count of answers now. A wait
  after an answer waits on the stage's next end, and never answers the
  stop-short its answer answered (43).
- **A fresh start flume makes on its own.** After a spent account
  (`read_spent`) or a start begun without its settings (`unsettled.rs`),
  the fresh start's words are flume's own, then the earlier start's
  `words`, then each answer that start took as `answered` gives it, in
  order. Neither a retry's words nor an answer is lost with its agent
  (42, 146, 161).

### 8. Status, ends and the JSON form

The JSON form only gains fields (core.14):

- `WaitingOn` gains `open_until`, the time the session closes while it
  is open, and `closed`, the `SessionClosed` once one is recorded; both
  are none for any other move.
- `StageStatus` gains `answers`, its current start's `AnswerHeld`s in
  order (56).
- `EndStatus` gains `open_until`: for a stop-short's end, the time its
  session was open until when the end was read, and none where no
  session was held open (43).
- While the session is open, the current agent is the asking one, with
  `keep_open` among its settings and its layer (56).

The person's form of status prints the move's line as now, then one of:

- "its session open for the answer until <time>: retry with --words,
  or tell it";
- "its session closed at <time>, its time passed" (or "its agent
  ended", or "its account spent"), then "its agent ended at <time>" or
  "its agent is ended once quiet: retry with --words".

Each answer prints on a line of its own, with the act it came by, when
it was received, and the question and the answer. A stop-short's notice
(`ends.rs`) adds "its session open for the answer until <time>" where
its end gives one. Times print as status prints every time.

### 9. Held state

Every new field is read with a default and written only where set, so
held state written before reads with no session open, no answers and no
close (75, 103): `Settings::keep_open`, `StageEnd::open_until` and
`closed`, `StartHeld::answers`, `EndOf::Stage`'s `answers`, and
`WaitingOn::open_until` and `closed`. `SessionClosed` serializes `at`,
`by` as `time`, `agent_ended` or `account_spent`, and `ended_at`.
`AnswerHeld` serializes `question`, `answer`, `by` as the `Act` it came
by, `sent_at` and `received_at`.

### 10. The stand-in under herdr and the walkthroughs

The stand-in for Claude Code (`crates/flume/tests/profiles/stand-in/claude.rs`)
gains three things, each named in its doc comment:

- **Its state to herdr.** Where it runs in a herdr pane, with
  `HERDR_PANE_ID` and `HERDR_BIN_PATH` in its environment, it reports
  `working` while a turn runs and idle once the turn ends. It reports
  by `pane report-agent <pane> --source <its own> --agent claude
  --state <state>` with the pane's own `HERDR_*` left out, as the herdr
  tools tests' shell stand-in does, in the states the pinned herdr
  names (core.29).
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

`walk.rs` gains a host under the pinned herdr: `[host] terminal =
"herdr"`, a herdr server on the walkthrough's scratch home as the live
harness starts one (`live/main.rs`), the stand-in first on the panes'
`PATH`, and the walkthrough's account giving `STAND_IN_SCRIPTS` beside
its scratch `CLAUDE_CONFIG_DIR` and token. Every herdr call is bounded
and passes the recorded server's socket (core.29). The walkthroughs in
`crates/flume/tests/profiles/tools/open_sessions.rs` run on that host
in `flume:tools`, each a fix:

- **S4, an answer within the hour.** Code's first start stops short
  needing an answer, "Which file?". Status gives the move as a retry of
  code, its session open until an hour after the stop-short, and
  `keep_open` from `default`. A retry with the words "notes.md" reaches
  the same pane, start 1, whose next run takes the question and the
  answer as its prompt. That run stops short again, "And its text?",
  an end delivered of its own. A tell, "Hello.", reaches the same
  session, and its last run commits. Code started once, status gave
  both answers with their questions and acts, and the place lands (41,
  42, 49, 53 to 55).
- **S76, the time passing, then a late answer.** The host gives
  `[kinds.fix.agents] keep_open = "5s"` and `[host] agents = 1`. Code
  stops short, and a second fix waits for room. Once five seconds have
  passed, a settle records the session closed by its time and ends the
  agent, its pane closed. Status says the place waits on an answer, its
  agent ended, and the second fix's code starts. A tell is refused,
  naming the retry. A retry with the answer starts code's second start,
  whose prompt carries the question then the answer; it commits and the
  place lands (41, 42, 49, 53, 70).
- **S77, the agent's session ended within the hour.** Code stops short,
  and its script holds no later run, so its session ends. The next
  settle records the session closed with its agent ended. A retry with
  the answer starts a fresh agent with the question and the answer, and
  the place lands (41, 42, 96).
- **S77, the account spent within the hour.** On a pool of two
  accounts, code's first start stops short and its turn then ends at
  its account's usage limit, its script holding a later run. The settle
  records the account spent and the session closed by it, and ends the
  agent. A retry with the answer starts code on the pool's second
  account with the question and the answer, and the place lands (41,
  42, 146).

Each scenario of this change's specs over the fakes is an engine
scenario in `crates/flume-conformance/src/engine/open_sessions.rs`,
citing its clauses, its time moved by the fake clock (106).

### 11. The site

- `guides/callers-and-holds.md`: stopping short, the open session and
  until when, the answer by a retry or a tell, the session's close, a
  later answer as a retry, and a tell refused then. Its line "A tell is
  never an answer" goes.
- `guides/agent-settings.md`: `keep_open` in the table of settings and
  its three layers, flume's hour, `0`, and the refusal in a
  repository's file.
- `guides/places-and-herdr.mdx`: a session held open under herdr, and
  none under the printed command.
- `reference/configuration.md`: `keep_open` in the host's two tables.
- `reference/commands.md`: `--agent keep_open`, and the answer a retry
  or a tell gives.
- `reference/state.md`: a stage end's `open_until` and `closed`, a
  start's `answers`, and status's new fields.

All of it speaks to any host's operator and names no host or team of
ours (core.35).

### 12. The statement's wording

These amend agentplot/blueprints `design/flume/requirements.md` at
92afa99, for the operator to rule (core.19). Each gives the clause's new
text where it changes, or the words it gains.

- **Section 3, settings**: "what an agent starts with beyond its
  prompt: its program, its model, its mode, its instructions, its
  environment, where in the terminal it runs, such as beside a pane its
  caller names, and how long its session stays open for an answer once
  it stops short (41)."
- **Section 3, a new term after stop short**: "**open**: of an asking
  agent's session, that an answer handed to the place reaches it: from
  its stop-short until the time its settings give has passed, its agent
  stops or its account is spent (41)."
- **32**: its first sentence reads "An agent's program, model, mode,
  instructions and how long its session stays open for an answer (41)
  are resolved when it starts, each from the first of these that gives
  it:". Items 2 and 3 end "for the model, the mode, the instructions
  and how long a session stays open alone". Item 6 reads "flume's own
  default, which names a program only for an agent with no pool (173),
  and keeps a session open one hour". The repository's settings and its
  kinds' carry "no environment, no stream, no pool, no account and no
  time a session stays open".
- **41** gains, after its last sentence: "An agent that stops short
  needing an answer stays live, its session open for that answer from
  the stop-short for the time its settings give, one hour where none
  gives one (32), the life of its program's prompt cache, where the
  host's terminal can hand it a tell (53); where it cannot, no session
  is open. The session closes once that time has passed, once the agent
  is read stopped or its own record says its session ended (96, 134),
  as a restart of the host or its terminal leaves it, or once its
  account is spent (146), whichever comes first. A settle reads it
  closed, and no process of flume's waits for the time (120). flume
  then ends the agent once it is quiet (31), which frees its room and
  its account (70, 145), and the stage keeps waiting on its answer
  (42)."
- **42**: "A stage that ended short needing an answer starts no next
  stage, whatever else exists. Its place waits for its answer or a
  drop, and the agent's words reach the place's recipients, and status,
  whole and as the agent wrote them. The answer, whoever gives it and
  however it reached the caller, is a retry naming the stage with the
  answer as its words (49), or a tell (53). While the stage's agent's
  session is open for it (41), flume hands that agent the question and
  the answer as a tell is handed (54) and records both with the stage,
  and the stage carries on in that session under the same start, begun
  again, starting and ending no agent. Once the session has closed, the
  answer is a retry, and the fresh agent's prompt carries the question
  with the answer. A stage that ended short having found work is sent
  back as 131 says."
- **49**: its first sentence ends "..., as 40 and 47 allow; but a retry
  with words naming a stage ended short needing an answer while its
  agent's session is open for it (41) is that stage's answer, which
  reaches that session as 42 says and starts no agent."
- **50**: "A retry ends the place's agent first, one that is not quiet
  only when forced (31), but for a retry that is an answer to an open
  session (42), which ends none; and it is refused, naming the place,
  when the place's work is over."
- **53**: "A tell hands the agent of a place's current stage its words
  as a prompt, or nothing at all. A tell to a place with no live agent,
  or whose agent is asking, or whose stage ended short needing an
  answer once its agent's session has closed (41), is refused with
  nothing typed, the last naming the retry that answers it."
- **54** gains: "An answer handed to an open session (42) is delivered
  as a tell is, within the same bound; one not delivered leaves the
  stage waiting on its answer, with nothing recorded."
- **55**: "A tell never starts, ends, holds or moves a stage, never
  counts as a retry or a finish, and never stands for a deliverable,
  but for a tell to a place whose stage ended short needing an answer
  while its agent's session is open for it, which is that stage's
  answer and carries it on in that session (42)."
- **56**: after "whether it needs an answer or found work," it reads
  "and, for one needing an answer, whether its agent's session is open
  for it and until when, or when and why it closed and whether its
  agent is ended (41), and each answer its stage's current start took
  in its session, with its question (42),".
- **118**: "The limit runs from the stage's beginning, its last answer
  received in its session (42), or the place's last head move,
  whichever is later."
- **130**: after "status says which move and since when" it reads ",
  and, for an answer, whether it reaches its agent's open session and
  until when, or that the session has closed (41); a session's close
  leaves the move and its since as they were".
- **C.2**: after "A repository's agents tables give a program, a model,
  a mode and instructions, and carry nothing of one host:" it gains "a
  `keep_open` in a repository's or a kind's agents table refuses the
  launch, naming it, as an `env` does, since how long a session stays
  open is the host's or the launch's (32, 41);".
- **C.4, the send-back row**: "while the stage's deliverable does not
  exist, the agent has not stopped short since its stage last began,
  and no background task runs and no question is open".
- **C.5, the printed command**: its send is refused, "so a tell is
  refused and no session is held open for an answer (41, 53)".
- **C.7, the host's agents tables**: "each gives `model`, `mode`,
  `instructions` and `keep_open`, the last as `<n>`, `<n>s`, `<n>m` or
  `<n>h`, and names one stage's by a further `.stages.<stage>` table
  (32, 41)".
- **S4**: its last sentence reads "The design agent's answer comes back
  within the hour as a retry of construct with the answer as its
  words: flume hands construct's agent the question and the answer in
  its open session, records both, and construct carries on there under
  its first start and delivers (41, 42, 49)."
- **S76** (new): "A code agent stops short needing an answer on a host
  whose `[kinds.fix.agents]` gives `keep_open = "10m"`, its host at its
  bound with a second place waiting for room. Status says the place
  waits on an answer in its open session until ten minutes after the
  stop-short. No answer comes. The first settle after that time records
  the session closed by its time and, the agent being quiet, ends it,
  and the second place's stage starts in the room it freed. Status says
  the place still waits on an answer, its agent ended, since the
  stop-short. A tell of the answer is refused, naming the retry; the
  retry starts a fresh agent whose prompt carries the question and the
  answer (41, 42, 49, 53, 70)."
- **S77** (new): "A construct agent stops short needing an answer, and
  within its hour another agent of its account reaches the account's
  usage limit. The next settle reads the account spent and the session
  closed, and ends the construct agent once it is quiet. A second
  place's construct agent stops short, and its pane is gone after its
  host restarts; the next settle reads that session closed with its
  agent ended. Each place waits on its answer with its agent ended, and
  each answer, a retry, starts a fresh agent with the question and the
  answer, the first on another account of its pool (41, 42, 96, 146)."

## Risks / Trade-offs

- An open session holds room for its whole time in the worst case: a
  host at its bound whose every agent is asking runs nothing else until
  their sessions close. A host whose room is tight sets `keep_open`
  shorter, or `0`, and status names every agent so held.
- The time is a setting, not a reading of the cache. Where an account's
  cache lives five minutes, an answer at minute ten reaches a session
  whose next request reads its context in afresh. The agent keeps its
  place in the work, but the request costs what a fresh agent's would.
  The host sets `keep_open` to its accounts' cache.
- An answer is handed only once the agent waits and is not asking. An
  agent that also put a question to a person in its pane takes no
  answer from flume until that question is answered: the act is refused
  with nothing typed, the stage still waits, and status says the agent
  is asking.
- A retry with words now waits for the agent as a tell does, up to ten
  minutes, before it answers. Before, it started a fresh agent at once.
- flume reads a session closed only from what it reads of the agent. A
  pane that still shows a live agent whose own record says nothing
  reads open, as flume reads such a pane today (96, 134).
- The walkthroughs run under the pinned herdr with a stand-in, so they
  prove the session and the acts, never a real model's reading of its
  cache. Record 0021's cache use in status, unbuilt, would show it.

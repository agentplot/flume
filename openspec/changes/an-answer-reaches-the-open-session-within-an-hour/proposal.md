# An answer reaches the open session within an hour

## Why

An agent that stops short needing an answer ends its stage short, and
today its answer is a retry: flume ends that agent and starts a fresh
one, whose prompt carries the question and the answer (42, 49, 50).
Until the retry comes, flume leaves the asking agent live. It holds
the host's room, its stream's and its account's, and the retry then
ends it anyway. The fresh agent reads the place again from nothing:
what the asking agent had read and decided is lost, and so is its
prompt cache. Most answers come within minutes, while the agent that
asked still waits with all of it in hand.

Record 0028 (accepted on 2026-11-01, on the user's card) keeps the
asking agent open for its answer for up to an hour, a setting, one
hour by default, the life of the prompt cache. An answer within that
time goes into the open session and is recorded with the stage, and
the stage carries on there. Past it, flume ends the agent, freeing its
room and its account, and a later answer is a retry as today. An agent
that dies within the hour is the same as past it. Status says which.

## What Changes

- **`keep_open`, how long a session stays open.** An agent's setting,
  `<n>`, `<n>s`, `<n>m` or `<n>h`, resolved at its start from the
  launch's settings, then the host's
  `[repos."<repository>".kinds.<kind>.agents]`, then its
  `[kinds.<kind>.agents]`, a stage's own first in each, else flume's
  own one hour (32; record 0017). A repository's file or a kind's
  declaration that gives it refuses the launch, as an `env` does. `0`
  ends an asking agent once quiet.
- **The asking agent kept open.** A stage ended short needing an
  answer, found work outside the kind's last stage among it, keeps its
  agent live, its session open for the answer until its stop-short's
  time plus `keep_open` (41). The session is open only where the host's
  terminal can hand a running agent words. The terminal seam says
  whether it can: herdr can, and the printed command cannot (53, C.5).
- **The answer handed to the session.** A retry with words naming the
  stage, or a tell to the place, while the session is open, is the
  stage's answer. flume types the question and the answer into the
  session as a tell is typed: once the agent waits, received or sent
  once more (53, 54). It records both with the stage's start, and the
  stage carries on in that session under the same start, begun again,
  starting and ending no agent (42, 49, 50, 55). An answer the agent
  does not take is refused with nothing typed, and the stage still
  waits.
- **The session closed and its agent ended.** A settle reads the
  session closed once its time has passed, once its agent is read
  stopped or its session ended (96, 134), or once its account is spent
  (146), and records when and why. Once the agent is quiet flume ends
  it, never forced, freeing its room and its account (31, 70). The
  stage keeps waiting on its answer. A later answer is a retry with a
  fresh agent whose prompt carries the question and the answer, and a
  tell is refused, naming that retry (42, 49, 53).
- **What carries on.** A stage carried on after its answer runs as any
  running stage does. Its stop check holds, and its limits run from
  the answer's receipt (44, 118). Its next stop-short is an end of its
  own, and a wait on it answers no end read before the answer (43). A
  fresh start flume makes on its own after a spent account or a start
  begun without its settings carries the earlier start's words and
  answers after flume's own (146, 161).
- **Status.** Whether a stage's answer reaches its open session and
  until when, or when and why its session closed and whether its agent
  is ended; each answer a session took, with its question. A
  stop-short's end and its notice say until when its session is open
  (43, 56, 130).
- **The stand-in and the walkthroughs.** The stand-in for Claude Code
  reports its state to herdr when it runs in a herdr pane, and takes a
  later prompt typed into its terminal. Four walkthroughs run under the
  pinned herdr: an answer reaching the same session and recorded; the
  time passing, the agent ended and the place still waiting, then a
  late answer starting a fresh agent with the question and the answer;
  and an agent dying within its time, by its session ending and by its
  account spent.
- **The statement.** Design 12 words the amendments the statement
  needs, which the operator rules in agentplot/blueprints (core.19).
- **Docs.** The site's callers guide, agent-settings guide,
  places-and-herdr guide, and the configuration, commands and state
  references, in the site's neutral voice (core.35).

## Capabilities

### New Capabilities

None: every part of this change belongs to a capability that stands.

### Modified Capabilities

- `caller-acts`: a retry with words, or a tell, to a stage whose
  agent's session is open is its answer, handed to that session; past
  it, a retry starts a fresh agent and a tell is refused.
- `stage-runs`: a stop-short keeps its asking agent open; a settle
  closes the session and ends the agent once quiet; a second stop-short
  of one start is an end of its own; limits run from an answer's
  receipt; a fresh start flume makes carries the earlier start's words
  and answers.
- `running-through`: found work outside the last stage waits on its
  answer, its agent kept open.
- `place-status`: whether an answer reaches its open session, the
  session's close, and each answer a session took.
- `end-delivery`: a stop-short's end says until when its session is
  open, and a wait after an answer waits on the next end.
- `launching`: `keep_open` resolved from the launch's and the host's
  layers, else flume's hour.
- `store-profile`: the host's agents tables give `keep_open`.
- `kinds-profile`: a declaration that gives `keep_open` refuses the
  launch.
- `command-line`: `--agent [<stage>.]keep_open=<time>`.
- `seam-contracts`: the terminal says whether it hands a running agent
  words; the held state's open session, its close and its answers.
- `seam-fakes`: the fake terminal scripts whether it hands words, and
  the fake agent a session ended or an account spent after a stop-short.

## Impact

- agentplot/blueprints: the statement at 92afa99 has every answer a
  retry. Design 12 words section 3's settings, 32, 41, 42, 49, 50, 53
  to 56, 118, 130, C.2, C.4's send-back row, C.5, C.7 and S4, with new
  scenarios S76 and S77, for the operator to amend (core.19). Until
  then `docs/architecture/_open.md` names the wait. weir is untouched:
  `keep_open` is no field of the kind format, and a declaration that
  writes it is refused by flume as one that writes `env` is (core.4).
- `crates/flume-seams`: `read_span`, `Settings::keep_open`,
  `Terminal::sends`, `StageEnd::open_until` and `closed`,
  `SessionClosed`, `ClosedBy`, `StartHeld::answers`, `AnswerHeld`,
  `EndOf::Stage`'s `answers`, `WaitingOn::open_until` and `closed`;
  status fields, only added.
- `crates/flume-core`: `KEEP_OPEN`; `settings.rs` folds `keep_open`;
  `agent_acts.rs` records the session's time; `acts.rs` hands an answer
  to an open session through `retry` and `tell`; a new `sessions.rs`
  reads a session closed and ends its agent; `settle.rs` steps it;
  `start.rs` carries an earlier start's words and answers; `wait.rs`,
  `ends.rs` and `status.rs` give the session.
- `crates/flume`: the host's agents tables, the kinds profile's
  refusal, `--agent keep_open`, the herdr and printed profiles'
  `sends`, the person's form of status and of a notice; the stand-in's
  herdr state and later prompts; walkthroughs under the pinned herdr.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake terminal's
  `sends`, the fake agent's ended session and spent account after a
  stop-short, and engine scenarios citing their clauses.
- `site/`: the guides and references.
- `docs/architecture/_open.md`: the item on record 0028 keeps only the
  statement's amendment once the code is built.
- No new crate, plugin or site part, so no new moon project; no new
  dependency.

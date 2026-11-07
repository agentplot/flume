# An answer reaches the open session within an hour

An answer given within the hour lands in the agent that asked, which
keeps what it had read and carries its stage on. Past the hour, flume
ends the agent and frees its room and its account, and the answer
starts a fresh agent with the question and the answer, as it does today.

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

Record 0046, accepted by the user on 2026-11-01 ("Keep it open an
hour, then end it"), keeps the asking agent open for its answer for up
to an hour, a setting, one hour by default, the life of the prompt
cache. An answer within that time goes into the open session and is
recorded with the stage, and the stage carries on there. Past it,
flume ends the agent, freeing its room and its account, and a later
answer is a retry as today. An agent that dies within the hour is the
same as past it. Status says which.

## What Changes

- **`keep_open`, one of clause 32's settings.** How long an asking
  agent is kept open, `<n>`, `<n>s`, `<n>m` or `<n>h`, resolved at its
  start as the model is: taken whole from the first layer that gives
  it, a stage's own first within each. The launch's settings and the
  host's four layers give it, for a repository and kind, a kind, a
  stream and every agent; flume's own is one hour; `0` keeps no agent
  open. A repository's file or a kind's declaration that gives it
  refuses the launch, as an `env` does, since what it trades, the
  host's room and its accounts' cache, is the host's (158). It is one
  value, never a table of programs.
- **The asking agent kept open.** A stage ended short needing an
  answer, found work outside the kind's last stage among it, keeps its
  agent live, kept open for the answer until its stop-short's time
  plus `keep_open`. An agent is kept open only where its terminal hands
  a running agent words. The terminal seam says whether it can: herdr
  can, and the printed command cannot (53, 95, C.5).
- **The answer handed to the session.** A retry with words naming the
  stage, or a tell to the place, while its agent is kept open, is the
  stage's answer. flume types the question and the answer into the
  session as a tell is typed: once the agent waits, received or sent
  once more (53, 54). It records the answer on the stop-short it
  answers, where the questions change records a keyed retry's, and the
  stage carries on in that session under the same start, begun again,
  starting and ending no agent (42, 49, 50, 55). An answer the agent
  does not take is refused with nothing typed, and the stage still
  waits.
- **The agent let go and ended.** A settle reads the agent let go once
  its time has passed, once it is read stopped or its session ended
  (96, 134), or once its account is spent (146), and records when and
  why. Once the agent is quiet flume ends it, never forced, freeing its
  room and its account (31, 70). The stage keeps waiting on its answer.
  A later answer is a retry with a fresh agent whose prompt carries the
  question and the answer, and a tell is refused, naming that retry
  (42, 49, 53). A bare agent the terminal brought back after a restart
  is left in a conversation's pane and ended once quiet elsewhere (77),
  and a conversation's agent so lost is carried at its answer's retry,
  as a lost conversation's retry carries it (49, 179, 207).
- **What carries on.** A stage carried on after its answer runs as any
  running stage does. Its stop check holds, and its limits run from the
  answer's receipt (44, 118). Its next stop-short is an end of its own,
  keyed apart, and a wait on it answers no end read before the answer
  (43). A fresh start flume makes on its own after a spent account or a
  start begun without its settings carries the earlier start's words
  and answers after flume's own (146, 161).
- **Status.** Whether a stage's agent is kept open for its answer and
  until when, or when and why it was let go and whether it is ended;
  each answer its session took, with its question; `keep_open` with
  its layer. A stop-short's end and its notice say until when its agent
  is kept open (43, 56, 130).
- **The stand-in and the walkthroughs.** The stand-in for Claude Code
  reports its state to herdr when it runs in a herdr pane, and takes a
  later prompt typed into its terminal. Four walkthroughs run under the
  pinned herdr: an answer reaching the same session and recorded; the
  time passing, the agent ended and the place still waiting, then a
  late answer starting a fresh agent with the question and the answer;
  and an agent dying within its time, by its session ending and by its
  account spent.
- **The statement.** Clause 198 and scenarios S104 and S105, with
  section 3 and clauses 4, 32, 42, 49, 50, 53 to 56, 95, 118, 130, 146,
  158, 161, C.2, C.4, C.5, C.7 and S4 amended, on blueprints main from
  c188bf3 (core.19). Clauses 49, 179 and 198 carry a conversation's
  asking agent lost with its host at its answer's retry, on the
  blueprints branch `design/flume-an-asking-conversation-lost-is-carried`
  for the user's word (design 14).
- **Docs.** The site's callers guide, agent-settings guide,
  places-and-herdr guide, and the configuration, commands and state
  references, in the site's neutral voice (core.35).

## Capabilities

### New Capabilities

None: every part of this change belongs to a capability that stands.

### Modified Capabilities

- `caller-acts`: a retry with words, or a tell, to a stage whose agent
  is kept open is its answer, handed to that session; past it, a retry
  starts a fresh agent and a tell is refused.
- `stage-runs`: a stop-short keeps its asking agent open; a settle lets
  the agent go and ends it once quiet; a second stop-short of one start
  is an end of its own; limits run from an answer's receipt; a fresh
  start flume makes carries the earlier start's words and answers.
- `running-through`: found work outside the last stage waits on its
  answer, its agent kept open.
- `place-status`: whether an agent is kept open, its letting go, each
  answer a session took, and `keep_open` with its layer.
- `end-delivery`: a stop-short's end says until when its agent is kept
  open, and a wait after an answer waits on the next end.
- `launching`: `keep_open` resolved from the launch's and the host's
  layers, else flume's hour.
- `store-profile`: the host's agents tables give `keep_open`.
- `kinds-profile`: a declaration that gives `keep_open` refuses the
  launch.
- `command-line`: `--agent [<stage>.]keep_open=<time>`.
- `seam-contracts`: the terminal says whether it hands a running agent
  words; the held state's agent kept open, its letting go and its
  answers.
- `seam-fakes`: the fake terminal scripts whether it hands words, and
  the fake agent a session ended after a stop-short.
- `clause-coverage`: the suite knows 198, S104 and S105.

## Impact

- **Order.** The code builds on flume's main once
  `an-agents-program-settings-come-from-flumes-layers` landed, and folds
  `keep_open` in the same resolution and host tables as the model.
  `an-agents-effort-is-a-setting-of-flumes-layers`, in code beside it,
  folds the effort there too, and whichever lands second takes the
  other's field in. A conversation's asking agent lost with its host is
  carried at its answer's retry by the lost-conversation retry on main
  (record 0052), and no terminal settles an agent kept open (122, 208).
  `every-question-a-place-asks-reaches-someone-who-answers-it` reads
  the answer this change records on a stop-short's end, and keys a
  stop-short after an answer apart (design 12, 14).
- agentplot/blueprints: main from c188bf3 holds design 13's
  amendment, and design 14's waits on its branch. weir is untouched: `keep_open`
  is no field of the kind format, and a declaration that writes it is
  refused by flume as one that writes `env` is (core.4).
- `crates/flume-seams`: `read_span`, `Settings::keep_open`,
  `Terminal::sends`, `StageEnd::open_until` and `closed`,
  `SessionClosed`, `ClosedBy`, `StartHeld::answered`, `EndHeld::answer`,
  `AnswerHeld`, `Handed`, `EndOf::Stage`'s `answered`,
  `WaitingOn::open_until` and `closed`; status fields, only added.
- `crates/flume-core`: `KEEP_OPEN`; `settings.rs` folds `keep_open`;
  `agent_acts.rs` records the time kept open; a new `open_sessions.rs`
  reads an agent kept open, hands it an answer through `retry` and
  `tell`, and lets it go and ends it at a settle; `settle.rs` steps it;
  `start.rs` carries an earlier start's words and answers; `wait.rs`,
  `ends.rs` and `status.rs` give it.
- `crates/flume`: the host's agents tables, the kinds profile's
  refusal, `--agent keep_open`, the herdr and printed profiles'
  `sends`, the person's form of status and of a notice; the stand-in's
  herdr state and later prompts; walkthroughs under the pinned herdr.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake terminal's
  `sends`, the fake agent's ended session after a stop-short, engine
  scenarios citing their clauses, and the clause list.
- `site/`: the guides and references.
- No new crate, plugin or site part, so no new moon project; no new
  dependency.

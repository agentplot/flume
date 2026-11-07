# Design

## Context

On flume main at 376c17c, an agent's `flume needs --found` reaches
`Engine::needs` (`crates/flume-core/src/agent_acts.rs`). It ends the
stage `EndedShort` with `StageEnd { needs: Some(StopKind::FoundWork),
words, .. }`. A settle of a place whose current stage stands
`EndedShort` calls `found_work` (`crates/flume-core/src/settle.rs`,
`crates/flume-core/src/submit.rs`). `found_work` acts only where
`sent_back_by(held, index)` holds: the place is live, the stage is the
kind's last, it ended short having found work, and the submission waits
on nothing. It ends the agent once quiet and calls `fix_round`.
`fix_round` reads the last stage's `send_back_to` (weir's `on_fail`) as
the target, the last stage itself where it names none. Past the last
stage's `send_back_limit` (flume's three by default), it writes
`submission.waiting` with the cause and the words. Otherwise it counts
a going-back and resets every stage from the target, and the settle
starts the target with the findings as its words.

`waiting_on` (`crates/flume-core/src/status.rs`) gives the move a place
waits on, and every write of held state records it as `held.waiting`
(`crates/flume-core/src/held.rs`), which status, the line and the list
read. For an `EndedShort` stage it answers a retry of that stage,
`Waits::Answer`, with the agent's words. The exception is a found from
the last stage, for which it answers nothing, since the settle sends
the place back. A place past its bound with `submission.waiting` set
after a found or a failed submission answers `Waits::PastBound` on the
target, read again there from `send_back_to`.

fix@1 and chore@1 declare one stage, `code`, with `on_fail = "code"`
(`crates/flume/src/profiles/kinds/weir/own/fix@1/kind.toml`). So a found
from fix's code sends the place back to fix's code, up to three times.
On crew's `fix--a-landing-in-another-hosts-queue-reads-as-in-the-queue`,
runs 1 to 3 each found a blocker outside the place and did so.

Every prompt's `{{needs}}`, the stop check's words and a carried
start's prompt take the agent profile's `stop_short`
(`crates/flume-core/src/start.rs`, `agent_acts.rs`). Claude Code's
(`crates/flume/src/profiles/agent/claude/mod.rs`) reads: run `flume
needs "<words>"` when you cannot go on without an answer, saying what
you need, or `flume needs --found "<words>"` when you found work this
stage cannot deliver past, saying what you found; then stop.

The statement on agentplot/blueprints main at 734d60c amends 131: a
found from the kind's last stage starts the fix round where the fix
round is a stage before the one that found. From any other stage, and
from a last stage that is its kind's fix round, found work is taken as
needing an answer (42), since no stage before it can do that work
again. 48 now names a found beside a failed submission as what sends a
place back. 41 already says a found is work a stage before the agent's
own must do again.

## Goals / Non-Goals

**Goals:**

- A found from a last stage that is its kind's fix round waits on an
  answer, as a found from any other stage does (42, 131).
- One reading of a kind's fix round for the send-back, the bound's
  move and the found (23, 112, 131).
- Every agent told that a found is work a stage before its own must do
  again (28, 41).

**Non-Goals:**

- A new version of fix, chore or change. flume's own kinds keep their
  content in each version (156).
- Keeping an agent open for its answer (198), which is record 0046's
  change. A found taken as needing an answer is a stop-short waiting on
  its answer, and that change keeps it open as it keeps any such.
- Statechart kinds, which flume runs none of (175).
- Moving the conformance suite's clause list. 734d60c adds and removes
  no clause (106).

## Decisions

### 1. One reading of a kind's fix round

`submit.rs` gains `pub(crate) fn fix_round_of(held: &HeldState) ->
Option<usize>`: the position among `held.stages` of the stage the kind's
last stage names in `send_back_to`, else the last stage's own, and none
for a kind with no stages. `fix_round` takes its target from it.
`waiting_on` takes its `PastBound` target from it too. The rules stay as
they are: weir refuses an `on_fail` that names no stage of the kind, so
the fallback to the last stage stands only for a last stage naming none
(23; core.4).

The other choice, each caller reading `send_back_to` as today, keeps
three copies of one rule. This change adds a fourth reader, the found.

### 2. A found sends the place back only to a stage before it

`sent_back_by(held, index)` also requires `fix_round_of(held) <
Some(index)`: the fix round is a stage before the last, the one that
found (131). Where it is the last stage itself, `found_work` returns
false and changes nothing. The stage stays `EndedShort` with its words
and its kind, as a found from construct does today. No stage starts,
`submission.send_backs` is not counted and `submission.waiting` is not
written. The agent that stopped short is kept as a stop-short that
needs an answer keeps its agent today: no settle ends it, and a
caller's act ends it, a retry among them once the agent is quiet (49;
`end_agent` in `acts.rs`). Its room and its account are held the same.

A found's end delivered to the place's recipients stays `<stage> found
work`, with the words whole (`ends.rs`'s `what_ended`): the agent said
which it did, and status says which (41; core.27). What the place waits
on is the answer.

### 3. Status, the line and the list say it waits on an answer

`waiting_on`'s `EndedShort` branch answers nothing only where
`sent_back_by(held, index)` holds, the one reading the settle uses, so
status never says a place waits on an answer that the next settle sends
back, nor the reverse. Everywhere else it answers `Act::Retry` of that
stage, `Waits::Answer`, since when the stage ended, and the findings.
`held.waiting` records it at the write that ends the stage (`held.rs`).
So the line reads `waiting on an answer`, the list gives the move as it
gives any answer's, and the place may stall as any place waiting on its
caller under no hold's words (130; core.24, core.52).

`waiting_on`'s past-bound branch keeps reading a found beside a failed
submission. A found reaches it only where its fix round is a stage
before it, and a place a found from its own fix round left past its
bound before this change keeps waiting so, with its words, until its
caller retries or drops it.

### 4. The retry that answers it

A caller's retry of the stage that found starts it with a fresh agent.
Its words are the findings as the question, then the answer, through
`answered` in `acts.rs` as for any stop-short (42, 49). It is a
caller's start and counts no going-back (48). The stages after it run
again in order, and the place is submitted as before. Nothing in
`Engine::retry` changes.

### 5. The words every agent is told

Claude Code's `stop_short` reads:

> run `flume needs "<words>"` when you cannot go on without an answer,
> saying what you need, or `flume needs --found "<words>"` when you
> found work a stage before this one must do again, saying what you
> found; then stop.

The same words go to every start of every stage, and name no stage, no
identity and nothing that differs between starts. So a repetition's
prompt still matches the one before it up to its group (141; core.31).
They reach the prompt's `{{needs}}`, or its end where the template has
none, the stop check's words and a carried start's prompt, wherever
`stop_short` already goes (28, 110). `flume needs --found`'s help in
`commands/mod.rs` reads "The stage found work a stage before its own
must do again." `StopKind`'s and `StopShort`'s documentation in
`flume-seams/src/agent.rs` say so too (41, 92). The fake agent's
`stop_short` takes the same sense in its own words, and the test stand-ins in
`crates/flume/tests/commands.rs`, `terminal_herdr.rs` and the seams'
`round_trip.rs` keep the words they give, which no test reads for its
sense.

Two other wordings were weighed. Naming the fix round in the words,
"work `code` must do again", differs between kinds and tells a fix
round's agent to name itself. Leaving `--found` out of the words of a
stage with no stage before it needs `stop_short` to know the stage's
place in its kind, and changes the seam for an end that 131 already
reads as an answer. The plain words keep the seam as it is.

flume's own kinds keep their prompts (156). fix@1's and chore@1's code
still say "or you find work this stage cannot deliver past, stop short:
{{needs}}". That is their content, locked by its hash (core.34), and
`{{needs}}` now fills it with the words above. An agent of fix's code
that finds work anyway is answered as one that asks. change@1's and
change@2's verify prompts already say a found sends the change back to
its code stage.

### 6. The tools beside flume

weir's format names the stage a failed submission sends a place back to
(`on_fail`, weir 52). flume alone reads a found against it, and weir
reads no stop-short. crew and the runner answer a stop-short by a
retry or a drop, as core.27 has it, so a fix place that asks now
reaches them as a question rather than a place past its bound. No other
tool of the family sends a place back. Nothing to reconcile.

### 7. Rules and records

Record 0011's amendment of 2026-11-06, the design agent's, decides
this. Rule core.27 is amended with it, and the site's mirror follows
(core.19, core.35). The change relies on core.4 (weir's `on_fail`, read
through weir's crate), core.9 (a stage's end from a stop-short carrying
its start's secret), core.14 (the line and the list from the one core),
core.24 (a place waiting on its caller may stall), core.27, core.31 (the
way to stop short the same for every start), core.34 and 156 (flume's
own kinds keep their content) and core.1 (the core names no tool: the
words are the agent profile's). No area page exists. It departs from
no rule.

### 8. Checks

- The running-through scenarios in
  `crates/flume-conformance/src/engine/running_through.rs`, over the
  fakes' fix kind (`code` naming `code`) and a kind of one stage naming
  no `on_fail` (core:test, conformance:test, conformance:coverage).
- Claude Code's words in `crates/flume/tests/profiles/agent_claude.rs`,
  and `--found`'s help in `crates/flume/tests/commands.rs` (flume:test).
- A walkthrough under the stand-in for Claude Code in
  `crates/flume/tests/profiles/tools/walkthroughs.rs`. A fix whose code
  stops short with `flume needs --found` waits on an answer, with one
  agent opened. Its caller's retry starts code once, its prompt holding
  the findings and the answer. It then lands (flume:tools).

## Risks / Trade-offs

- [A fix round's agent that found work its own stage could redo now
  waits on its caller] → the caller's retry starts that fresh agent with
  the findings. The words tell the agent to redo such work rather than
  stop.
- [A repository's kind of one stage that relied on a found restarting
  it] → that stage's found reads as a question: status and the end give
  its words, and a retry restarts it. The site's kinds page says so.
- [fix@1's prompt and the new words differ in tone] → its text is
  locked (156). The words decide which command the agent runs, and
  either command waits on its caller.

## Migration Plan

None. A place already launched is read by the new rule at its next
settle (21). Its kind's content is unchanged, and only flume's reading
of a found is new.

## Open Questions

None.

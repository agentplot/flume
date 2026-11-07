# A notice waits for its run's prompt

## Why

sluice refused #159 at 2a2101a, flume's `.config/sluice.toml` setting
`flaky = "fail"`: S93's walkthrough failed at 14:19:59Z on 2026-11-07,
the run of `east/ops` read lost, and passed on the same inputs at
14:29:03Z. The run's agent had taken construct's notice as its prompt.

A role's run is woken inside a settle, and its start sends its prompt
at once only where its agent then says it started and reads waiting. On
a loaded host a reading can miss, an answer of herdr's late past its
bound among them, and the prompt is left to the settles that follow.
The next settle settles the place whose end is pending before the run.
It finds the run's agent waiting in the address's session and types
the notice into it. The agent takes the notice as its first prompt and
works its turn on it. Its stage's own prompt comes after, or never,
and is never received; the stand-in, whose script runs one turn,
stops, and the run reads lost with its work committed. Claude Code
would go on working on the notice, and a long turn would read its
start never begun.

Clause 37 sends a stage's prompt only to an agent that said it started
and waits, and 38 has it received or sent again; clause 43 delivers a
role's end into the pane of its live run. Neither holds while the
run's agent takes a notice ahead of its prompt.

## What Changes

- **An end for a role's address waits while the address's run has not
  taken its prompt (37, 38, 43, 132).** While the run's current start is
  starting, or started and its prompt not yet received, the settle
  sends no notice into the run's session, and the end stays pending.
  The first settle that reads the start begun delivers it, unless the
  run's `flume inbox` collected it first. A standing agent of the
  address is delivered to as before.
- **S93's walkthrough says what each agent did when it stalls.** Its
  wait, and S94's, gives every start's stand-in log with every place's
  line.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `role-waking`: a role's run takes no notice before its current start
  took its prompt.

## Impact

- `crates/flume-core`: `standing.rs`, the delivery's reach of a role's
  run held while its current start awaits its prompt.
- `crates/flume-conformance`: role-waking's scenario of a run not yet
  given its prompt.
- `crates/flume`: the walkthroughs' `settled_until` and the herdr host's
  `logs`.
- `docs/`: record 0058, proposed; rule core.11's details.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool. The statement is unchanged: 37, 38 and 43 already
  say this.

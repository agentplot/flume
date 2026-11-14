# Every message to a role waits at its address

## Why

crew's standing roles are typed into from several sides at once: a
caller's `flume tell`, every end a place sends them as a notice, and the
operator at the keyboard. A notice is typed into a working session as
readily as into a waiting one, so two tasks interleave in one turn, and
herdr's ctrl+c before each send clears what the operator had half typed
(C.5). A message to a role has no home of its own either: `flume tell`
names a place, and a role with no run live has no place to tell.

The operator ruled on 2026-11-11 and 2026-11-12, accepted as record
0069 on 2026-11-14 (2d180f0b): every message and end for a role is a
delivery pending at the role's address, handed to its session as one
prompt at its next wait by its agent profile's deliver part, a Claude
Code mod first and the terminal the fallback, and the verb is `message`,
replacing `tell`. The statement says so in clauses 43, 53 to 55, 214 and
215 and C.4's deliver row (boomdrivedev/blueprints
`design/flume/requirements.md`, main 81288ba), and C.7 gives a message's
delivery key on the branch
`design/flume-a-message-to-a-role-is-a-delivery-at-its-address`
(9ddd402), which this change's construct wrote and the orchestrator
pushes.

## What Changes

- **`flume message` replaces `flume tell` (53 to 55).** The command, its
  act, its answer's `act`, the herdr plugin's key and every word of the
  site and the commands' help say `message`. `flume tell` stays a hidden
  alias that runs `message`, so a caller not yet moved keeps working;
  held state that recorded an answer by `tell` reads it as `message`.
- **A message to a role's address is a delivery at it (53, 214, C.7).**
  `flume message <stream>/<role> "<words>"` records a weir delivery
  keyed `message/<stream>/<role>/<token>`, its one reader the address,
  its words with who sent them, and answers at once with its key; it
  wakes the address's run where none is live, as an end does (132), and
  is refused, recording nothing, for an address no source declares, two
  declare or whose stream the host does not declare.
- **A recipient takes every delivery pending for it as one prompt at its
  next wait (43, 54, 214, 215).** An end's notice and a message are
  handed only while the recipient's session waits and asks nothing,
  never while it works: every one pending for it and handed to it by
  nothing goes in one prompt, in the order recorded, each with how to
  confirm it. Each is claimed beside the host's deliveries before the
  send and recorded handed on its answer, so two settles hand it once.
  A role's run that wakes for its address carries the batch in its
  first prompt; one whose stage waits on an answer takes nothing until
  it has its answer.
- **The agent seam's deliver part (215, C.4).** A profile says whether
  its program takes words by its own channel. Claude Code's is the
  deliver part of flume's plugin's hooks module, `hooks/flume.ts`, beside
  the place's line, which at each turn's end
  and while its session is idle runs `flume hook deliver` and submits
  what it prints by `$.prompt.submit`, leaving the input box as the
  person left it. The pinned 2.1.296 is not yet checked to run a hooks
  module's `$.prompt.submit` in an interactive session, read on 2.1.322
  (record 0069), so the profile answers deliver not offered and the
  terminal hands every batch until the chore that moves the pin turns
  it on. A profile that says nothing of it offers none.
- **A message to a place's agent goes by the deliver part where offered
  (54).** It waits for the agent's take within its bound, as it waits
  for the terminal today, and is withdrawn with nothing typed where the
  bound passes first.
- **A launch from a role's run reports to its address (7, 214).** A
  launch naming no recipient, run in a role's run, has the run's
  address as its recipient, so the place's ends wait there with the
  role's other deliveries.
- **Status and the inbox (43, 56, 214).** Status gives, for each role's
  address, the messages pending at it and each place whose work is not
  over that reports to it. The inbox collects messages beside ends, and
  `flume confirm` takes a message's key.
- **MCP (section 9).** `flume mcp` serves `message`, `launch` and
  `wake` beside `status`, each the command line's act, so a role's
  session launches its projects as places from its tools (S128).
- **Rules and record.** core.11 and core.25 say a message and a notice
  reach a recipient only at its wait, by the deliver part or the
  terminal, and core.27, core.37 and core.54 say `message` for `tell`,
  each amended to record 0069; record 0069 gains the construction
  agent's choices, each with the path not taken.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `caller-acts`: a message replaces a tell, to a place's agent or to a
  role's address.
- `command-line`: `flume message`, its target, its `--answers`, and
  `tell` as its alias.
- `end-delivery`: a notice handed only at its recipient's wait, every
  delivery pending for it in one prompt, by the deliver part or the
  terminal; the inbox gives messages.
- `role-waking`: a message wakes an address as an end does, a run's
  first prompt carries its batch, and a launch from a run reports to
  its address.
- `deliveries`: a message's delivery key, the claims kept beside the
  records, and a message's delivery let go of once confirmed.
- `place-status`: an address's messages and the places reporting to
  it; an answer's act named `message`.
- `seam-contracts`: the agent seam's deliver part, a session sent
  nothing while it works, and an answer by `message` read where `tell`
  was written.
- `agent-profile`: Claude Code's deliver part, its variable and its
  answer at the pinned release.
- `claude-code-plugin`: the deliver mod, `flume hook deliver`, and
  `message`, `launch` and `wake` over MCP.
- `herdr-plugin`: the key that messages a place's agent.
- `terminal-profiles`: herdr's and Rex's `send_to` refuse a working
  session.
- `stage-runs`: a stuck stage is woken by a message.

## Impact

- `crates/flume-seams`: `Act::Message`, read from `tell`; the agent
  seam's `deliver`; the host deliveries document's claims; a message's
  delivery and its status types.
- `crates/flume-core`: `acts.rs` (`message`), a new `batch.rs` for what
  a recipient takes and who hands it, `ends.rs`, `deliveries.rs`,
  `wake.rs`, `launch.rs`, `wait.rs` (the inbox and confirm), `start.rs`
  (a run's first prompt), `answer.rs` (status).
- `crates/flume-fakes`: the fake agent's deliver and take; the fake
  terminal's working session.
- `crates/flume-conformance`: scenarios for each spec scenario above.
- `crates/flume`: `commands/mod.rs`, `commands/mcp.rs`, `print.rs`,
  `profiles/agent/claude/` (deliver, its hook), the herdr and Rex
  profiles' `send_to`.
- `plugins/claude`: `hooks/flume.ts`, in place of `hooks/line.ts`, its
  tests, `hooks/hooks.json`,
  `tests/hooks.sh`; `plugins/herdr`: its key and its tests.
- `docs/architecture/core.md` and the site's copy of the rules;
  `docs/adr/0069-…`; the site's guides and command reference.
- Callers: a caller that runs `flume tell` keeps working through the
  alias; one that reads an answer's `act` reads `message`. A role's
  session is no longer typed into while it works.

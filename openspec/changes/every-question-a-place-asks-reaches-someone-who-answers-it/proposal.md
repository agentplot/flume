# Every question a place asks reaches someone who answers it

## Why

The user, 2026-11-05: "Agents doing work in Flume are using Flume needs
but I don't know that anything is paying attention to those Flume needs
calls."

They are right. A stage's agent that cannot go on runs `flume needs`,
its stage ends short, and the place waits on its caller's retry (41,
42, 130). Whether anyone reads the question turns on how the place was
launched:

- **From a shell with no terminal session**, a launch names no
  recipient (7). On deneb that is 34 of 37 places, the orchestrator's.
  Their ends, and the stalls after them, went to no one. The only
  reader was the orchestrator's own script polling `flume status`.
  While it was not running, overnight from 2026-11-04 to 11-05, two
  changes waited about nine hours on questions nobody saw.
- **From the runner**, a place names `madswan/planner`. The host's
  configuration names no run for that address, so nothing wakes for it
  (132), and seven ends of one place stand pending. The runner never
  collects ends. It copies `flume status` into crew's records, and the
  standing planner sees a question only when it reads its board. crew
  launches role runs through the runner rather than flume's wake,
  because flume names a role's run by the role alone and two
  partitions' planners in one repository would be one place.
- **From a herdr pane**, an end is delivered into that pane. A
  restarted agent there gets a one-line notice about a place it never
  saw, an end delivered just before the restart is never shown again,
  and flume's plugin does nothing at the start of a session flume did
  not start.

Nothing bounds how long a question waits on recipients who do not
answer. A question that is the user's is put on a card by hand, under a
key of whoever makes it. Nothing stops two answerers from retrying one
question, or a late answer from starting a stage that has moved on.

## What Changes

This change is the design. It builds nothing until the user has read
it and said yes to record 0039. Its tasks are the build that follows.

- **The decision proposed.** Record 0039, proposed. Every place has a
  recipient that collects its ends or has a run woken for it. The host's
  answerer stands behind every place's recipients, and every stall
  reaches it. An answer names the stop-short it answers and is taken
  once. A restarted recipient sees every place that waits on it. A
  question that is the user's becomes one card, made by the answerer's
  run, never by flume.
- **The host's answerer.** `[host] answerer = "<partition>/<role>"`, and
  `[streams.<name>] answerer`, which comes first for its places: a
  role's address the configuration names a run for (132). It is the
  recipient of a launch that names none and runs in no session (7).
- **A launch no one would answer is refused.** A launch that names no
  recipient, runs in no session and finds no answerer is refused,
  saying how to name one. So is a launch naming an address that the
  configuration names no run for and that no session serves (4, 132).
- **A session serves an address.** `flume serve <address>`, run in a
  terminal's session, makes that session the address's while it is
  live. Its ends are delivered into it, and no run is woken for an end
  it can be delivered. A standing role is reached as a running one is.
- **A role's run is named by its whole address.**
  `<kind>--<partition>-<role>`, so two partitions' runs of one role are
  two places. flume becomes the one launcher of a role's run. The
  runner and crew's follower of card answers carry their own events to
  it by `flume wake` (132).
- **The bound.** A stall is delivered to each recipient and to the
  host's answerer where it is none of them. The stall carries the words
  and the key of the stop-short it waits on (43, 130).
- **A hold on the waiting move.** `flume hold <place> --waiting
  "<words>" [--for <time>]` says a caller is making the move the place
  waits on, so the place does not stall. Past its bound the hold lapses
  and the place stalls (130).
- **An answer is taken once.** Every end has a key, which every notice
  and every collected end gives. `flume retry <place> --stage <stage>
  --answers <key> --words "<answer>"` is refused, naming who answered
  and when, once the place no longer waits on that stop-short (42, 49).
- **What a restarted recipient sees.** `flume inbox` also gives every
  place that waits on the recipient, delivered or not, with the move,
  since when, the question's words and key, and a hold's words, and
  marks nothing for them. flume's Claude Code plugin's SessionStart, in
  a session flume did not start, collects for that session and the
  address it serves, and gives the agent what the inbox gives. A notice
  names the place, its brief's first line, the end and its key (43).
- **Status** gives each place's recipients with where each came from,
  the host's answerer, each address's serving session, a hold on the
  waiting move with its bound, and an answer's key and who gave it
  (56).
- **Changes named in other repositories**, to be launched there on the
  user's yes and not written here:
  - agentplot/blueprints: flume's statement, as design 11 words it.
    sluice's is not amended.
  - afterthought/crew: the planner's brief keeps the answerer's
    protocol. A standing role serves its address. A card for a place's
    question is keyed by the place and the end. `crew answer` carries
    the key, and takes a place crew did not launch. Role runs are named
    in flume's `[roles]` and woken by flume. Rules roles.6 and roles.7
    and the change `roles-run-as-flume-kinds` Decision 1 change with
    it.
  - agentplot/boomdrive: the runner offers a role's run by `flume wake`
    and launches none, and passes an answer's key with its retry.
- **The site** says who answers a place's question, the host's answerer,
  serving an address, the hold on the waiting move and the answer's
  key.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `launching`: a launch's recipients, ending at the host's answerer, and
  the launch refused where no one would answer or an address would wake
  nothing.
- `end-delivery`: a notice that names the place's brief and the end's
  key, and an inbox that gives what waits on its recipient.
- `caller-waits`: a hold on the waiting move with its bound, and a stall
  delivered to the host's answerer with the question it waits on.
- `caller-acts`: a retry that names the key of the stop-short it
  answers.
- `role-waking`: a run named by its whole address, a session that
  serves an address, and the host's answerer, which must be an address
  the host wakes.
- `history-and-host`: the host's answerer, and each stream's, read from
  the configuration.
- `claude-code-plugin`: SessionStart in a session flume did not start
  gives that session what waits on it.
- `place-status`: who answers a place, and the answers it took.
- `command-line`: `serve`, `hold --waiting` and `retry --answers`.
- `clause-coverage`: the suite reads the statement at the revision that
  holds the amendment, and cites S91 to S94.

## Impact

- **Before the build.** The user's yes on the design and record 0039,
  and on its two other questions (design, Open Questions).
  agentplot/blueprints: flume's statement amended on main as design 11
  words it (core.19). The build waits on nothing else: crew's and
  Boomdrive's parts follow it, and a host keeps working while they do.
- `docs/adr/0039-…`: proposed here, and accepted on the user's yes.
  `docs/architecture/core.md`: core.24, core.25 and core.28 amended, and
  a rule drawn for the answerer and the answer taken once. Records 0009,
  0013 and 0014 carry an amendment line.
- `crates/flume-seams`: `HeldState`'s hold on the waiting move and its
  bound; `EndHeld`'s answer; `EndOf::Stall`'s stop-short; `Host`'s
  answerer and served addresses; `role_of` naming the whole address.
- `crates/flume-core`: `launch.rs` (recipients and the refusals),
  `ends.rs` (the notice, the stall to the answerer, delivery to a served
  session), `held.rs` (`read_stall`), `wake.rs` (the run's name, a
  served address), `wait.rs` (`inbox`'s waiting places), `acts.rs`
  (`retry`'s key, `hold`'s waiting move), `status.rs`.
- `crates/flume`: the command line (`serve`, `hold --waiting`, `retry
  --answers`), the host's configuration reader, the store's served
  addresses, and the Claude Code profile's SessionStart hook.
- `crates/flume-conformance`: the clause list and the engine's scenarios
  for S91 to S94.
- `plugins/claude/`: the hooks' description and tests, the SessionStart
  of a session flume did not start among them.
- `site/`: `guides/callers-and-holds.md`, `guides/roles-and-wake.md`,
  `guides/claude-code.mdx`, `reference/configuration.md`,
  `reference/commands.md`, `reference/state.md` and the rules page.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency.

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
  Their ends, and the stalls after them, went to no one. While the
  orchestrator's polling script was not running, overnight from
  2026-11-04 to 11-05, two changes waited about nine hours on questions
  nobody saw.
- **From the runner**, a place names `madswan/planner`. Every
  main-level role of crew's stands as a session in a herdr pane,
  outside flume, so nothing runs at that address, and seven ends of one
  place stand pending. A standing planner sees a question only on
  crew's board.
- **From a herdr pane**, an end is delivered into that pane. A
  restarted agent there gets a one-line notice about a place it never
  saw, and flume's plugin does nothing at the start of a session flume
  did not start.

crew already designed and built roles that answer at their addresses:
its change `roles-run-as-flume-kinds` makes each main-level role, the
design agent, the planner, ops, the reviewer and the operator, a Flume
kind, run whenever something is pending for it. No role has moved onto
its kind. flume names a role's run by the role alone, so two
partitions' planners would be one place, and a run never ends of
itself, so crew had the runner launch and end role runs instead of
flume. Every place crew launches names the planner, though a question
may be the design agent's or ops's. Nothing bounds how long a question
waits on a role that does not answer, and nothing stops two answerers
from retrying one question.

## What Changes

This change is the design, revised on the user's rulings of
2026-11-05: record 0039 accepted; flume starts a role's run, the runner
handing crew's events to it by `flume wake`; and the answerer is not
the planner alone, but crew's roles, each a Flume kind at its own
address. It builds nothing until the user has read it. Its tasks are
the build that follows.

- **The decision.** Record 0039, accepted on 2026-11-05 and revised to
  the rulings. Every main-level role answers as a Flume kind at its own
  address. A question reaches the role it is for. The partition's door,
  its operator, stands behind every stall. An answer names the
  stop-short it answers and is taken once. A restarted recipient sees
  every place that waits on it. A question that is the user's becomes
  one card, made by the role whose question it is.
- **flume launches a role's run, and ends it.** The host's `[roles]`
  entry gains `name`, the run's place's name, else
  `<partition>-<role>`, so two partitions' runs of one role are two
  places; `intent`; the run's agents' settings; `answers`, the words of
  what the role answers; and `carried`, for an address whose run is on
  another host. A run is done and tidied by flume once its turn
  delivers. A run's own stall goes to the door.
- **A question goes to the role whose it is.** `flume needs --to
  <address>` addresses one question to a role of the place's partition,
  and the stop-short part of every prompt lists those roles with what
  each answers (28, 41). Its end goes to that role alone.
- **The host's answerer.** `[host] answerer` and `[streams.<name>]
  answerer`, a role's address the configuration names a run for: the
  recipient of a launch that names none and runs in no session (7), and
  a recipient of every stall.
- **Launches no one would answer are refused.** A launch with no
  recipient, no session and no answerer, a launch naming an address
  that wakes nothing where no answerer stands behind it, and a launch
  that would take an answerer that wakes nothing (4).
- **The bound.** A stall is delivered to the place's recipients, to the
  role the stop-short named, and to the answerer behind the stream, with
  the question's words, key and addressee (130).
- **A hold on the waiting move.** `flume hold <place> "<words>" --waiting
  [--for <time>]` says a role is on the question, so the place does not
  stall. Past its bound the hold lapses and the place stalls again.
- **An answer is taken once.** Every end has a key, which every notice
  and every collected end gives. `flume retry <place> --stage <stage>
  --answers <key> --words "<answer>"` is refused, naming who answered
  and when, once the place no longer waits on that stop-short (42, 49).
- **What a restarted recipient sees.** `flume inbox` also gives every
  place that waits on the recipient, delivered or not, with the move,
  since when, the question's words, key and addressee, and a hold's
  words, and marks nothing for them. flume's Claude Code plugin's
  SessionStart, in a session flume did not start, collects for that
  session and gives the agent what the inbox gives. A notice names the
  place, its brief's first line, the end and its key (43).
- **Status** gives each place's recipients with where each came from,
  a stop-short's addressee, the host's and each stream's answerer, each
  address's run or that it is carried or wakes nothing, a hold on the
  waiting move with its bound, and an answer's key and who gave it (56).
- **Changes named in other repositories**, to be made there once the
  user has read the design, and not written here:
  - agentplot/blueprints: flume's statement, as design 11.1 words it.
    sluice's is not amended.
  - afterthought/crew: its turn collects flume's inbox and holds,
    answers, passes or cards each question; each brief takes its part of
    the routing; the operator stands behind every stall; `crew answer`
    carries the key and takes any place; a card for a place's question
    is keyed by the place and the end; roles.7 holds as written, and the
    change `roles-run-as-flume-kinds` Decisions 1 and 6 give way to
    flume's launch.
  - agentplot/boomdrive: the runner wakes a role by `flume wake`, and
    launches and ends no role's run; it passes an answer's key with its
    retry.
  - The host's configuration and teams file, swancloud's on deneb: each
    partition's roles moved onto their kinds one at a time, in the order
    design 12 gives, the operator first with flume's release.
- **The site** says who answers a place's question, addressing a role,
  the host's answerer, a role's run's name and its end, the hold on the
  waiting move and the answer's key.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `launching`: a launch's recipients, ending at the answerer behind its
  stream, and the launch refused where no one would answer.
- `end-delivery`: a notice that names the place's brief and the end's
  key, and an inbox that gives what waits on its recipient.
- `caller-waits`: a stall delivered to the role a question named and to
  the answerer, and a hold on the waiting move with its bound.
- `caller-acts`: a retry that names the key of the stop-short it
  answers.
- `stage-runs`: the way to stop short listing the roles an agent may
  address, and a stop-short addressed to one of them.
- `role-waking`: a run named per address, with its settings and intent,
  done at its delivery, its stall to the answerer, and an address
  carried to another host.
- `history-and-host`: the host's answerer, each stream's, and the
  `[roles]` entry's new keys, read from the configuration.
- `claude-code-plugin`: SessionStart in a session flume did not start
  gives that session what waits on it.
- `place-status`: who answers a place, and the answers it took.
- `command-line`: `needs --to`, `hold --waiting` and `retry --answers`.
- `clause-coverage`: the suite reads the statement at the revision that
  holds the amendment, and cites 191 and S92 to S95.

## Impact

- **Before the build.** The user's read of the design and their answers
  to its Open Questions. agentplot/blueprints: flume's statement
  amended on main as design 11.1 words it (core.19).
- `docs/adr/0039-…`: accepted by the user on 2026-11-05, revised to
  their rulings. `docs/architecture/core.md`: core.24, core.25 and
  core.28 amended, and a rule drawn for the answerer, the addressed
  question and the answer taken once. Records 0009, 0013 and 0014 carry
  an amendment line.
- `crates/flume-seams`: `RoleRun`'s `name`, `intent`, `agents`,
  `answers` and `carried`; `Host`'s answerer; `HeldState`'s hold on the
  waiting move; `EndHeld`'s answer; a stop-short's addressee;
  `EndOf::Stall`'s stop-short.
- `crates/flume-core`: `launch.rs` (recipients and the refusals),
  `ends.rs` (the notice, the stall to the addressee and the answerer, a
  stop-short's end to its addressee), `held.rs` (`read_stall`),
  `wake.rs` (the run's name and settings, its done at delivery),
  `wait.rs` (`inbox`'s waiting places), `acts.rs` (`needs --to`,
  `retry`'s key, `hold`'s waiting move), the prompt's stop-short part,
  `status.rs`.
- `crates/flume`: the command line (`needs --to`, `hold --waiting`,
  `retry --answers`), the host's configuration reader, and the Claude
  Code profile's SessionStart hook.
- `crates/flume-conformance`: the clause list and the engine's scenarios
  for 191 and S92 to S95.
- `plugins/claude/`: the hooks' description and tests, the SessionStart
  of a session flume did not start among them.
- `site/`: `guides/callers-and-holds.md`, `guides/roles-and-wake.md`,
  `guides/claude-code.mdx`, `reference/configuration.md`,
  `reference/commands.md`, `reference/state.md` and the rules page.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency.

# A host's work runs in streams on pools of accounts

## Why

A host runs flume's places under one queue and one bound (record 0010),
and the host's configuration picks an account per kind by an `env`
(record 0017). That leaves work and personal places sharing one queue,
and an account that reaches its usage limit stops its stage until
someone edits the host's file and retries. Record 0020 (accepted
2026-10-29) gives a host named streams, each with its own bound,
priority and pool of accounts. A spent account is paused for every
flume process on the host, and an intent keeps to the account its
prompt cache is on (143 to 147, 151, 153, S53 to S56). The user wants
streams and pools built before flume goes live, so no host ships
configuration with one account per kind.

Record 0016, amended 2026-10-29 on the user's idea, adds a check at each
start. flume records what the agent is to run under, and the agent's
own SessionStart hook compares its environment with that record before
it acks. A pane brought back without its environment, or a program
started again by hand, is then ended and started afresh with its
settings, never left running a stage under the wrong account (35, 135,
161, S65).

## What Changes

- **Streams.** `[streams.<stream>]` in the host's configuration, with
  `agents`, `priority` and `pool`. A place's stream comes from the
  launch, else the intent's affinity, else the host's for its
  repository and kind, for its repository, for its kind, else
  `default`. It is fixed at the launch, and a launch naming an
  undeclared stream is refused (143).
- **The queue across streams.** A stage starts only while the host, its
  stream and an account of its pool all have room. What waits starts by
  the stream's priority, then the place's, then arrival. A stream at its
  bound, or with no usable account, is passed over, never held (70;
  core.26).
- **Account pools.** `[pools.<pool>]` with `strategy`, `fill-first` or
  `least-busy`, and `accounts`, each a name, a program, an `env` and an
  optional `agents`. An account is chosen under the room lock at each
  start of an agent that has a pool (144, 145; core.32).
- **A spent account.** The Claude Code profile reads a usage limit from
  the StopFailure hook's `rate_limit` and its words, in the form the
  pinned 2.1.296 writes them, with the reset time they name. flume
  pauses the account in the host's own documents until then, or for an
  hour, and every flume process honours the pause. Once the spent agent
  is quiet its stage starts afresh on another usable account, or waits
  for one, with no caller's word and no going-back counted (146, 151,
  153).
- **Affinity.** An intent, named by the launch or else its repository
  and base, keeps to its account and stream while that account is
  usable. It moves when the account is spent or full and never moves
  back. It is dropped an hour after its last place is gone. Status says
  when work moved and that it lost its cache (147).
- **Eight layers.** An agent's settings come from the account, the
  launch, the host's for the repository and kind, the host's for the
  stream and kind, the host's for the kind, the repository's, the kind's
  and flume's default (32; core.30).
- **Kinds stay host-neutral.** A kind's declaration, a repository's file
  and a launch's declaration name no account, pool or stream, and are
  refused naming the field. weir's format gains nothing (record 0022;
  core.4).
- **The start checked against its settings.** At each start flume
  records the agent's program, its account and a digest of each variable
  of its environment, keyed by the start's secret, never a value. The
  SessionStart hook compares before it acks. On a difference it acks
  nothing and tells flume, which ends that agent once quiet and starts
  the stage afresh. A second difference in a row is never begun (161;
  core.10).
- **Docs.** The site's agent-settings guide and its configuration,
  state and commands references, in the site's neutral voice (core.35).
- **Nothing breaks.** A host's configuration written for record 0017,
  with an `env` per kind and no stream or pool, keeps working as before:
  its places run in `default`, bounded by `[host] agents` alone, under
  the environment their kinds' tables give.

## Capabilities

### New Capabilities

- `streams`: a host's streams, a place's stream and where it came from,
  the refusal of an undeclared stream, and each stream's part of status.
- `account-pools`: pools and their accounts, the choice of an account
  under the room lock, a spent account's pause in the host's own
  documents and the fresh start of its stage, and an intent's affinity.
- `start-integrity`: a start's record of what its agent is to run under,
  the comparison before the started ack, and the fresh start of a stage
  whose agent began without its settings.

### Modified Capabilities

- `host-queue`: two more bounds, the stream's and the pool's, and the
  order by stream priority before place priority.
- `launching`: eight layers in place of six; a launch's stream and
  intent; a kind or repository naming an account, pool or stream
  refused.
- `store-profile`: the host's configuration gains streams, pools and
  their keys; the host's own documents, accounts and intents.
- `agent-profile`: the usage limit read from Claude Code's words, and
  the started ack taken only where the start's record matches.
- `claude-code-plugin`: SessionStart compares before it acks;
  StopFailure records the time it ran.
- `place-status`: a place's stream, intent, account and moves; each
  stream and pool of the host.
- `command-line`: `flume launch --stream` and `--intent`.
- `role-waking`: a role's run names its stream.
- `seam-contracts`: the agent seam's `spent` and the start's record; the
  store's host documents.
- `seam-fakes`: the fakes script a spent account, a start without its
  settings and the host's documents.
- `history-and-host`: the host's facts give its streams and pools.
- `clause-coverage`: the suite's list at blueprints 8a11fbb, record
  0020's clauses and 161 built.

## Impact

- agentplot/blueprints: the statement amended at 8a11fbb for the start's
  record (35, 135, 161, C.4, S65) and C.4's reading of the usage-limit
  words. Nothing in weir changes: weir-kinds returns a kind's `agents`
  table unread, and flume reads and refuses its keys (core.4).
- `crates/flume-seams`: `Stream`, `Pool`, `Account`, `Strategy`;
  `Layer::Account` and `Layer::HostStream`; `Launch` and `PlaceRecord`
  gain stream and intent; `StartHeld` gains account and record; the
  agent seam's `spent` and `started_without`; the store's host
  documents; status fields, only added.
- `crates/flume-core`: `room.rs` across streams and accounts,
  `settings.rs` in eight layers, the account's choice and affinity in
  the claim, the spent and unsettled fresh starts in the settle, status.
- `crates/flume`: `host/config.rs`, the state directory's host
  documents, the Claude Code profile's `spent` and its hook's comparison,
  the command line and MCP launch, the person's status form, and the
  walkthroughs with the stand-in.
- `crates/flume-fakes`, `crates/flume-conformance`: the fakes' new
  answers, the engine scenarios citing 143 to 147, 151, 153, 161 and
  S53 to S56 and S65, and the clause list.
- `plugins/claude`: the hooks' tests for the comparison.
- `site/`: the guides and references.
- New crates from crates.io, pinned in `Cargo.lock`: `chrono-tz` for the
  reset's zone, and `hmac` beside the `sha2` the tree holds for the
  digests. No new crate of the workspace, plugin or site part, so no
  new moon project.

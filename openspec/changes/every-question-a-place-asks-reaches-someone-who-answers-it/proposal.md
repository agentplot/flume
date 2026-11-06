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

The address itself names nothing flume knows. Clause 132 gives it as
`<partition>/<role>`, and no clause says what a partition is: flume
reads the address as a key of the host's `[roles]` table and reads its
first part for nothing, beside the streams that already divide a host's
work (143). The user, of it: "Do we need the partition part of the
address? We're already going to use streams to partition where agents
run. I guess I was just assuming that communication had to stay within
a stream but we actually probably don't want that. I feel like we just
invented this address thing as well and it's a little weak."

crew already designed and built roles that answer at their addresses:
its change `roles-run-as-flume-kinds` makes each main-level role, the
design agent, the planner, ops, the reviewer and the operator, a Flume
kind, run whenever something is pending for it. No role has moved onto
its kind. flume reads a role's run from the host's own configuration,
one `[roles]` entry per address, though every fact of it is crew's and
crew already derives each from its partitions. It names a role's run by
the role alone, so two partitions' planners would be one place, and a
run never ends of itself, so crew had the runner launch and end role
runs instead of flume. Every place crew launches names the planner,
though a question may be the design agent's or ops's. Nothing bounds
how long a question waits on a role that does not answer, and nothing
stops two answerers from retrying one question. A question that is the
user's is put to them by whoever reads it, in words of its own, and a
construction agent may ask the user in the Inbox itself, where no
answerer of the place sees it.

## What Changes

This change is the design, on the user's rulings of 2026-11-05: record
0039 accepted; flume starts a role's run, the runner handing crew's
events to it by `flume wake`; the answerer is crew's roles, each a Flume
kind at its own address; crew's roles are crew's to declare, never
repeated in each host's configuration; planning is a label of each
partition's stream; an address is a role of a stream; a role's run
reads flume's list for its address; the operator may put a question to
the user; the roles move in the user's order, the planner and the design
agent once flume gives an agent's effort; and the word label keeps
flume's meaning alone. Then on the user's answers of 2026-11-06: cards
are written by code, never by an agent, and no construction agent
writes a card or an Inbox message; Pending You and the Inbox together,
the user's question a Pending You card that links to a thread in the
partition's Inbox when it needs elaboration; and an answer reaches
whoever asked, never a session the user picks. It builds nothing until
the user has read it. Its tasks are the build that follows.

- **The decision.** Record 0039, written to the rulings. An address is
  a role of a stream. Every main-level role answers as a Flume kind at
  its own address, declared once by crew. A question reaches the role
  it is for, in any stream. The stream's door, its operator, stands
  behind every stall. An answer names the stop-short it answers and is
  taken once. A restarted recipient sees every place that waits on it.
  A place's question that is the user's is one Pending You card that
  crew's code raises on the word of the role whose question it is,
  built from flume's facts as crew's code builds a batch's card, keyed
  by the place and the end so it is raised once. The user answers on
  the card, and crew's code, run by Pending You's notice, carries their
  words to the place as its retry, naming the key. Where the question
  needs talk, the card links to a thread in the Inbox of the stream's
  partition, held by the role as its address, whose replies reach the
  role by crew's follower whichever session holds it, or by an
  elaboration's named agent, there while the user talks. Where each
  partition's Inbox runs is swancloud's design
  `the-plannotator-inbox-runs-on-deneb-for-the-users-devices`. No
  construction agent writes a card or an Inbox message, and crew's own
  cards stay as they are.
- **An address is a role of a stream.** `<stream>/<role>`: the role a
  source declares for a stream the host declares (143). `madswan/ops`
  is ops of the stream `madswan`. A role's run runs in its address's
  stream. An address whose stream the host does not declare declares
  nothing unless its source carries it to another host. A stop-short
  names its own stream's role by the role alone, `--to ops`, and any
  other stream's by its whole address, `--to swancloud/ops`. A launch's
  recipient and a wake write an address whole. `flume wake`, a run's
  session name and its inbox stand as built.
- **crew declares its roles once, and flume reads them from crew.** A
  source the host declares (191) may name `roles`, the file in which it
  declares the roles it answers, which the source's own tool writes and
  flume never does. crew writes its file from its partitions, each of
  which runs its work in one stream: each address with its run's kind,
  one of crew's own, its repository, name, intent, settings and the
  words of what it answers; each partition's operator standing behind
  its stream; and each role whose stream runs on another host, carried.
  Ten partitions are ten addresses of one package, `crew/crew-<role>`.
  A fault stays its file's, its entry's or its address's. The host's
  `[roles]` table leaves the configuration.
- **What of the host remains.** crew declared as a source, the lock of
  crew's kinds, and its streams with their room and `planning`'s
  reserve: whose tools run under its accounts, what content it trusts,
  and how much of the machine each stream gets.
- **flume launches a role's run, and ends it.** A run is named by its
  entry's `name`, else `<stream>-<role>`, so two streams' runs of one
  role are two places, and is launched in its address's stream with its
  entry's intent and settings. A run is done and tidied by flume once its
  turn delivers. A run's own stall goes to the door.
- **A question goes to the role whose it is.** `flume needs --to <role
  or address>` addresses one question to a role of any stream, and the
  stop-short part of every prompt lists the roles with what each
  answers, the place's own stream's by name and every other's whole
  (28, 41). Its end goes to that role alone.
- **The answerer behind a stream.** A role a source declares to stand
  behind its address's stream, crew's operator of each partition: the
  recipient of a launch that names none and runs in no session (7), and
  a recipient of every stall of a place in that stream.
- **Launches no one would answer are refused.** A launch with no
  recipient, no session and no answerer behind its stream, and a launch
  naming an address that wakes nothing where no answerer stands behind
  it (4).
- **The bound.** A stall is delivered to the place's recipients, to the
  role the stop-short named, and to the answerer behind the place's own
  stream, with the question's words, key and addressee (130).
- **A hold on the waiting move.** `flume hold <place> "<words>" --waiting
  [--for <time>]` says a role is on the question, so the place does not
  stall. Past its bound the hold lapses and the place stalls again.
- **An answer is taken once.** Every end has a key, which every notice
  and every collected end gives. `flume retry <place> --stage <stage>
  --answers <key> --words "<answer>"`, and a tell that answers an agent
  kept open, `flume tell <place> --answers <key> "<answer>"`, are
  refused, naming who answered and when, once the place no longer waits
  on that stop-short (42, 49, 53). Record 0046, its statement on
  blueprints main at c188bf3 (clause 198, S104, S105), keeps the asking
  agent open for its answer, and the key holds whichever way it goes.
- **What a restarted recipient sees.** `flume inbox` also gives every
  place that waits on the recipient, delivered or not, with the move,
  since when, the question's words, key and addressee, and a hold's
  words, and marks nothing for them. flume's Claude Code plugin's
  SessionStart, in a session flume did not start, collects for that
  session and gives the agent what the inbox gives. A notice names the
  place, its brief's first line, the end and its key (43).
- **Status** gives each place's recipients with where each came from,
  a stop-short's addressee, each stream's answerer with its source,
  each source's roles or why they cannot be read, each address's run or
  that it is carried or wakes nothing, a hold on the waiting move with
  its bound, and an answer's key and who gave it (56).
- **Changes named in other repositories**, to be made there once the
  user has read the design, and not written here:
  - agentplot/blueprints: flume's statement, as design 12.1 words it
    against main at c188bf3, section 3's address, its new clause 193 and
    scenarios S93, S94, S98 and S99. sluice's is not amended.
  - afterthought/crew: its kinds as packages declaring the label
    `planning`; its roles file, written from its partitions at their
    streams' addresses; the teams file naming each partition's one
    stream; its turn reading flume's list for its address and holding,
    answering, passing or saying a question is the user's; `crew
    question`, whose code raises the place's question's card from
    flume's facts, once under its key, with a thread where it needs
    talk; `crew _answered` carrying the card's answer to the place as
    its retry with the key; the follower reading each role's threads as
    its address and closing each card whose place no longer waits;
    each brief's part of the routing, a role writing no card and, in the
    Inbox, only as its address; the operator behind every stall of its
    stream; `crew answer` carrying the key and taking any place; roles.7
    holding as written, and the change `roles-run-as-flume-kinds`
    Decisions 1, 3 and 6 giving way to flume's launch and crew's roles
    file.
  - agentplot/boomdrive: the runner wakes a role by `flume wake`, and
    launches and ends no role's run; it passes an answer's key with its
    retry.
  - The host, on deneb: crew declared as a source and its kinds locked,
    the streams `agentplot`, `madswan` and `swancloud` with
    `planning`'s reserve, and no construction kind's agents given the
    Inbox's or Pending You's tools, with flume's release and the
    operators' move;
    each later role moved by crew alone, in the order design 13 gives,
    the planner and the design agent once flume's change
    `an-agents-effort-is-a-setting-of-flumes-layers` is released.
  - Plannotator, outside the family: a watch on replies to a session no
    host wakes, and an identity for an address never read as ended,
    asked of it for a role's thread (design 12.5).
- **The site** says what an address is, who answers a place's question,
  a source's roles, addressing a role of any stream, the answerer behind
  a stream, a role's run's name and its end, the hold on the waiting
  move and the answer's key.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `launching`: a launch's recipients, ending at the answerer behind its
  stream, and the launch refused where no one would answer.
- `end-delivery`: a notice that names the place's brief and the end's
  key, and an inbox that gives what waits on its recipient.
- `caller-waits`: a stall delivered to the role a question named and to
  the answerer behind the place's stream, and a hold on the waiting move
  with its bound.
- `caller-acts`: a retry, or a tell that answers an agent kept open,
  that names the key of the stop-short it answers.
- `stage-runs`: the way to stop short listing the roles an agent may
  address, its own stream's by name and every other's whole, and saying
  it is the agent's one way to ask; and a stop-short addressed to one
  of them.
- `role-waking`: an address a role of a stream; the roles read from the
  sources and never from the host's configuration; a fault kept to its
  file, entry or address; a run named per address in its address's
  stream, with its settings and intent, done at its delivery, its stall
  to the answerer; and an address carried to another host.
- `history-and-host`: the answerer behind a stream and the host's
  answering roles, read from the sources.
- `claude-code-plugin`: SessionStart in a session flume did not start
  gives that session what waits on it.
- `place-status`: who answers a place, the sources' roles, and the
  answers it took.
- `command-line`: `needs --to`, `hold --waiting`, `retry --answers`,
  `tell --answers`, and `wake`'s address as `<stream>/<role>`.
- `clause-coverage`: the suite reads the statement at the revision that
  holds the amendment, and cites 193, S93, S94, S98 and S99.

## Impact

- **Before the build.** The user's read of the design.
  agentplot/blueprints: flume's statement amended on main as design 12.1
  words it (core.19). This repository: the change
  `a-host-declares-where-its-kinds-come-from` landed, since a source's
  declaration is read by it, and, before the answer's key, record
  0046's change `an-answer-reaches-the-open-session-within-an-hour`,
  since an answer handed to an agent kept open is recorded where the key
  takes it once.
- `docs/adr/0039-…`: accepted by the user on 2026-11-05, written to
  their rulings. `docs/architecture/core.md`: core.24, core.25 and
  core.28 amended, and a rule drawn for the answerer, the addressed
  question and the answer taken once. Records 0009, 0013, 0014 and 0040
  carry an amendment line.
- `crates/flume-seams`: `is_address` and `role_of` read as
  `<stream>/<role>`, and the address's stream beside them; `RoleRun`'s
  `name`, `intent`, `agents`, `answers`, `answerer` and `carried`, its
  source, and its stream its address's; `Host`'s answerer for a stream
  and the host's answering roles; `HeldState`'s hold on the waiting
  move; `EndHeld`'s answer; a stop-short's addressee; `EndOf::Stall`'s
  stop-short.
- `crates/flume-core`: `launch.rs` (recipients and the refusals),
  `ends.rs` (the notice, the stall to the addressee and the answerer, a
  stop-short's end to its addressee), `held.rs` (`read_stall`),
  `wake.rs` (the address's stream, the run's name, stream and settings,
  its done at delivery, the refusal's words), `wait.rs` (`inbox`'s
  waiting places), `acts.rs` (`needs --to` with a bare role read in the
  place's stream, `retry`'s and a tell's key, `hold`'s waiting move),
  the prompt's stop-short part, `status.rs`.
- `crates/flume`: the host's configuration reader (a source's `roles`
  and its file, an entry's address checked against the host's streams,
  `[roles]` refused), the command line (`needs --to`, `hold --waiting`,
  `retry --answers`, `tell --answers`, `wake`'s help), and the Claude
  Code profile's
  SessionStart hook.
- `crates/flume-fakes`: the fake host's sources' roles.
- `crates/flume-conformance`: the clause list and the engine's scenarios
  for 193, S93, S94, S98 and S99.
- `plugins/claude/`: the hooks' description and tests, the SessionStart
  of a session flume did not start among them.
- `site/`: `guides/callers-and-holds.md`, `guides/roles-and-wake.md`,
  `guides/claude-code.mdx`, `reference/configuration.md`,
  `reference/commands.md`, `reference/state.md` and the rules page.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency.

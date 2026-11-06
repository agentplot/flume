# Design

## In one page

A stage's agent that cannot go on without an answer runs `flume needs`.
Whether anyone reads the question turns on how its place was launched.
A place launched from a shell has no recipient at all. A place that
names a role's address reaches no one, because no role runs at its
address: every main-level role of crew's stands as a Claude Code
session in a herdr pane, outside flume. A place launched from a herdr
pane reaches that pane, whatever now runs in it. Nothing bounds how
long a question may wait on recipients who do not answer. On deneb on
2026-11-05, 34 of 37 places had no recipient. Two changes waited about
nine hours on questions only a polling script could see, and that
script was not running.

crew already designed how a question reaches the role whose it is,
and built it: its change `roles-run-as-flume-kinds` (archived
2026-10-31 on `teamless/deneb`, records 0047 to 0052). Each main-level
role, the design agent, the planner, ops, the reviewer and the
operator, is a Flume kind in crew's release, run at the role's address
whenever something is pending for it. No partition has moved a role
onto its kind yet. This design takes that as its base and finishes it
across the family, so that every question reaches the role it is for
and, past a bound, the stream's door:

1. **An address is a role of a stream.** `madswan/ops` is the role
   `ops` of the stream `madswan`, a stream the host declares (143). A
   role's run runs in its address's stream. A question names a role of
   its own stream by the role alone, `ops`, and any other stream's role
   by its whole address. The stream bounds where a place's agents run,
   never whom they may ask.
2. **crew declares its roles once, and every main-level role answers as
   a Flume kind at its own address.** crew is a source of the host's
   kinds (191). Its source's declaration names, beside its packages,
   the file in which crew declares its roles, which crew writes from its
   own partitions: every address with what its run is launched with.
   Ten partitions' planners are ten addresses of one package,
   `crew/crew-planner`, one in each partition's stream. flume reads the
   roles from crew, and the host's configuration holds no entry for a
   role. flume launches a role's run on an end pending for its address
   or a carrier's `flume wake`, names it per address, and declares it
   done once its turn delivers. The runner carries crew's own events to
   flume by `flume wake`.
3. **A question goes to the role whose it is.** A place's owner is the
   role the launch names: the planner for a bolt's work, ops for a fix of
   main, a role for its own landing work. The asking agent may address
   its question to another role instead, `flume needs --to ops` for its
   own stream's, `flume needs --to swancloud/ops` for another's. flume
   lists the roles, with what each answers in its source's words, in
   every stage's prompt. A role that takes a question not its own passes
   it on.
4. **The door stands behind every stall.** crew declares each stream's
   operator, its door, to stand behind the stream. A place waiting under
   nobody's words past its stall bound is delivered stalled to its
   owner, to the role its question named, and to the door of its own
   stream. A role that is on a question says so with a hold on it, whose
   bound, once passed, stalls the place again. A launch from a shell
   takes its stream's door as its recipient.
5. **An answer is taken once.** Every end has a key. A retry that names
   the key of the stop-short it answers is refused once the place no
   longer waits on it.
6. **A restarted recipient sees what waits on it.** The inbox gives
   every place waiting on its recipient. flume's plugin gives it to a
   person's session at its start, and every role's run collects it at
   its start. A notice names the place, its brief and the end's key.
7. **A place's question that is the user's is asked once, in the
   user's Plannotator Inbox**, by the role whose question it is, or by
   the door at a stall: in the Inbox of the role's stream's partition,
   in its address's thread, under an Inbox identity of the address's
   that outlives each run, keyed by the place and the end. The user's
   answer waits in the Inbox for that identity. crew's follower reads it
   at its minute and makes it an ask of the role, the runner wakes the
   role by `flume wake`, and its next run reads flume's list for its
   address and the thread, and retries with the key. Plannotator offers
   no watch and no identity for an address, so the design works around
   both and asks Plannotator for them. crew's own cards stay on Pending
   You until crew moves them (decision 10).

What remains of the host is crew declared as a source, the lock of
crew's kinds, and its streams with their room. Each is the host's own:
which tools run under its accounts, the content it trusts, and how much
of its machine each stream gets (decision 2).

`flume serve`, a session serving an address, is left out. Once roles
run as kinds, only a standing role would need it, and standing roles
are what each move retires (decision 11.5).

sluice's statement needs no amendment. crew's roles file, rules,
briefs and `crew answer`, and Boomdrive's role offers and retry, change
with this in their own repositories (decision 12). Moving each
partition's roles onto their kinds is a step of its own, in the user's
order: the operator with flume's release, then ops, the reviewer, the
planner and the design agent, the last two once flume's change
`an-agents-effort-is-a-setting-of-flumes-layers` gives their effort
(decision 13).

## Context

### What showed it

The user, 2026-11-05: "Agents doing work in Flume are using Flume needs
but I don't know that anything is paying attention to those Flume needs
calls."

`flume status --json` on deneb that afternoon, flume 0.6.4:

| recipients | places | what became of their ends |
|---|---|---|
| none | 34 | every end pending for no one. Fifteen stop-shorts so far, five needing an answer and ten that found work, and their fifteen stalls, each delivered to no one |
| a herdr pane | 2 | one question delivered, after which the pane's agent stopped. One place has been never begun for 70 hours, with a stop-short pending for its pane |
| `madswan/planner` | 1 | landed and tidied. Its document stays for seven ends pending for the address, none delivered. Status: "the host's configuration names no role for madswan/planner" |

Three places then waited on an answer, all with no recipient and all
stalled, for 0.6, 1.4 and 14.2 hours.

### How each tool carries an end or a question today

**flume.**

- **Recipients.** A launch records its recipients once: the ones it
  names, else `$FLUME_RECIPIENT`, else the terminal's session, else
  none (7, 8; `launch.rs`, `make_place`). A role's run gets no
  recipient of its own.
- **Addresses.** Clause 132: "A recipient may be named by a role's
  address, `<partition>/<role>`, rather than by a session." C.7 keys
  the host's `[roles]` table by it. No clause says what a partition is:
  the word is in 132 and C.7 alone. The site calls the first part a
  group, `<group>/<role>` (`guides/roles-and-wake.md`). In the code
  (`flume-seams/src/host.rs`, `is_address`, `role_of`; `wake.rs`):
  - any recipient of the form `<word>/<word>` is an address, and every
    other name a session's;
  - `Host::role` looks the whole address up in `[roles]`, and `role_of`
    takes the part after `/` to name the run `<kind>--<role>`;
  - the part before `/` is read by nothing;
  - the run's agent takes the whole address as its session's name, so
    its `flume inbox` collects for it;
  - `flume wake <address>` launches the run, refusing a name of any
    other form, and `flume launch --recipient <address>` names one;
  - status lists each address an end stands pending for, with why its
    run was not woken.

  So an address is a name a caller makes up and a host's configuration
  repeats as a key. flume checks its form and nothing else, and its
  first part names nothing flume knows, beside the streams that already
  divide a host's work (143).
- **Delivery.** The settle records each end once, by its key, and sends
  each recipient a one-line notice through the terminal: `flume:
  <kind>--<name> <what ended>; read: flume status <kind>--<name>`.
  It sends only while the terminal reads the session working or
  waiting, and records the end delivered on the terminal's answer.
  Asking, unknown, not found or refused leaves the end pending for a
  later settle (43; core.11; `ends.rs`, `deliver_ends` and `notice`).
- **The wait and the inbox.** `flume wait` answers by place, and
  `flume inbox` collects and marks every end pending for a recipient
  (`wait.rs`).
- **Waking a role.** A settle that finds an end pending for an address
  with no run standing launches the kind the host's `[roles]` table
  names for it, and a carrier may ask for the same by `flume wake`. The
  table gives the run's repository, kind, priority, base, brief and
  stream, and nothing of its agents' settings or its intent. A run's
  kind lands nothing, so once its stage delivers it waits on a caller's
  `flume done` that nothing gives (114): a run never ends of itself.
- **Sources.** Record 0040, accepted on 2026-11-05 and stated on
  blueprints' main at ff616eb, has the host declare each source of its
  kinds from outside in a file of its own, `sources/<source>.toml` of
  its managed layer, naming the directory of the source's packages
  (191, 192, C.7). A launch names such a kind by its source,
  `crew/crew-planner`, and it runs only as a lock records it (154, 155).
  The change that builds it, `a-host-declares-where-its-kinds-come-from`,
  is in flight.
- **Streams.** A host declares its streams, each by its name, with its
  bound, priority, pool and terminal, and every place runs in exactly
  one (143, 190). A kind's label, such as `planning`, is a share of its
  stream's agents, and a stream may reserve agents for a label (180,
  181).
- **The stall.** A place waiting on its caller under no hold's words
  for its stall bound, five minutes unless the launch sets one, is
  stalled. The stall is an end, delivered once to each recipient
  (130; `held.rs`, `read_stall`).
- **The answer.** A retry starts the stage afresh, the stop-short's
  words before its own, and checks nothing about which question it
  answers (42, 49; `acts.rs`, `retry`).
- **The stop-short.** `flume needs` takes the agent's words and whether
  it needs an answer or found work, and nothing about whom the words
  are for (41).
- **The plugin.** flume's Claude Code plugin does nothing in a session
  flume did not start (`claude/hook.rs`, `hook`). A caller's own
  session gets nothing at its start, though 43 says a caller that
  starts collects what is pending for it.

**crew** (afterthought/crew at `teamless/deneb`, 805679d7, the branch
deneb runs).

- **The partitions.** crew's teams file, `~/.config/crew/teams.json`,
  version 5, holds each partition: its name, in the field `label`, its
  herdr `partition` and `session`, its `blueprints` repositories, the
  first of which a role's run is cut in, its `state` repository, its
  review cadence, the `machine` its main level runs on, a `roles` table
  of models and efforts, and `flume`, which gives each of the five roles
  `false`, a standing agent, or the stream its run takes. swancloud's
  `lib/crew-teams.nix` writes the file on each host, and crew never
  does (`plugin/lib/crew.py`, `TEAMS`, `load`). The file on deneb names
  every partition every host runs, wldn's among them, whose main level
  runs on chuck-herdr-alpha. Every address crew writes is the
  partition's name and the role, `madswan/planner`, and each partition's
  name is its stream's (decision 1).
- **The roles as they run.** Every partition's `flume` names each of
  the five roles `false`. `crew main up` starts the design agent, the
  planner, ops and the operator as standing Claude Code sessions in
  herdr panes, outside flume, and the reviewer fresh for each batch.
- **Which question is whose, in the briefs** (`plugin/roles/`).
  - The planner: "Its places' questions are yours." Every place crew
    launches for a bolt names `<stream>/planner`. The planner gets the
    answer from whoever has it, "the design agent for a design question,
    `{{MAIN_OPS}}` for a live system, the user through a card", and
    gives it to the place by `crew answer`. Work outside a bolt's goal
    is its to queue.
  - The design agent answers "a design question the planner,
    `{{MAIN_OPS}}` or a Flume place brings", from the design as written,
    and gives the answer to the place itself by `crew answer`.
  - Main-level ops answers for a live system, and a fix of main it
    launches names it, so it answers that place's questions.
  - The operator is "the partition's door: whatever comes from outside
    reaches the partition through you". It carries a design question to
    the design agent, work and bolts to the planner, and a landing, a
    deploy or a live system to ops, and answers nothing itself. It posts
    no card: "your part is to make sure the user sees a wait that has
    none."
  - The reviewer reads one batch and writes one file. It is asked
    nothing.
- **What reaches a standing role.** A standing role learns of a place's
  question only from crew's board, which reads the runner's copy of
  flume's status. `tell_planner` runs for proposals only.
- **The roles as kinds, built and unused.** `plugin/flume/kinds.toml`
  declares `crew-design`, `crew-planner`, `crew-reviewer`, `crew-ops`
  and `crew-operator`, each one stage `turn` that lands nothing and
  delivers a report in the place's folder. They are one declaration
  file in crew's release, not packages, and declare no label. A turn
  starts with `crew role start`, which takes everything pending for the
  role, the asks in `asks.rec` and the places the runner's records show
  waiting on the role's address, and ends with `crew role end`. A tell,
  a send, a capture, a landing, a card's answer, a batch and `crew role
  open` to a role that runs as its kind are each an ask. A turn the
  user opened keeps the conversation in its pane until they are done
  (Decision 13).
- **crew's own declaration of its roles.** crew writes a role record,
  `roles.rec` on `<stream>/main` of the partition's state repository,
  from the teams file at `crew main up` and beside each ask (`plan.py`,
  `roles_lines`): each role's address, host, repository, kinds, kind,
  run's name (the partition's), stream, priority, intent, model and effort
  (Decision 6). Every fact a role's run is launched with is already
  crew's, derived from its partitions.
- **Why the runner, not flume, launches a role's run.** The change's
  Decision 1 set flume's `[roles]` aside because flume names a run by
  the role alone, so madswan's and swancloud's planners, whose first
  blueprints repository is the same, would be one place, and because
  "the user chose the runner as the launcher". The runner reads the
  role record and launches with `--kinds` naming crew's file (R1),
  only while something is pending (R2), and declares the run done (R3).
  Rule roles.7, from record 0049, says the other: "a role with no live
  run is woken by Flume, never by a process of crew's". The change
  departed from it. Record 0052's amendment says the runner names no
  kinds file, source or lock, which R1's `--kinds` also departs from.
- **Answers.** `crew answer <work> "<words>"` appends an `Act: retry`
  request for the runner to carry, from the planner, the design agent or
  main-level ops alone (`REQUESTERS`). It refuses a place crew did not
  launch.
- **Asking the user.** A role asks the user on a card in its own name,
  at most two questions at a time (briefs.11; records 0005, 0031), on
  Pending You, the hosted card queue. crew's commands raise cards of
  their own there: a batch, a proposal, a kit's landings, a deploy's
  offer, a decision record. With a role running as its kind, `crew
  _answered` writes a `card` ask of the role for each change on a card
  in its name, run by Pending You's notice from crew's follower.
- **The follower.** `crew _follow`, a job swancloud runs on each host,
  holding one lock a host, reads every partition with an operator here
  once a minute and writes the rail, the board and the agents' views
  (`plugin/bin/crew`, `_follow`; `plugin/lib/board.py`).

**The Boomdrive runner** (agentplot/boomdrive at 57145f6).

- **Launching.** It launches crew's work with `flume --json launch`,
  one `--recipient` per `Recipient:` of crew's request, and
  `$FLUME_RECIPIENT` removed from its environment (core.15 there).
- **Ends.** It never calls `flume inbox`, `wait` or `wake`. Each pass it
  reads `flume --json status` and copies a place's status, whole, to
  crew's `<stream>/runner/<host>:places.rec` when it changes.
- **Answers.** It carries crew's retry requests into `flume retry`.
- **Role runs** (`crates/boomdrive-crew/src/roles.rs`). For a role
  record naming its host, it offers a launch of the role's run, kind
  `crew/crew-<role>`, name the partition's, stream, priority, intent and model
  from the record, while an ask or a place of any host's waiting on the
  role is pending and no run stands. It declares the run done once its
  turn delivered.
- **How it runs.** It is a loop, `boomdrive run`. On deneb it is the
  launchd service `swancloud.boomdrive` (swancloud #399), which keeps it
  running. An answer of crew's reaches flume at its next pass.

**sluice** (agentplot/sluice at main).

- **The recipient.** An outcome goes to the submitting session alone,
  recorded at the submit as the multiplexer names it, or to no session
  (20, 50, 51).
- **Carrying it.** The deciding round pushes a notice once through
  herdr and never pushes again, asking or not. An outcome not delivered
  stays pending, with no bound (`step.rs`, `perform_outbox`;
  `profiles/worktrunk/herdr.rs`).
- **Collecting it.** A session that starts collects by `sluice inbox`
  before anything else, and sluice's plugin's SessionStart tells the
  agent to (51, 135; `plugins/sluice/scripts/session-context.sh`).
- **flume's places.** flume submits with no multiplexer variables and
  reads each outcome by `sluice inbox` at its settles, then delivers it
  to its own recipients as an end (C.6). A place's outcome reaches its
  caller through flume, so sluice's carrying is not on a question's
  path.

**The user's queue: the Plannotator Inbox** (0.28.48 as the host pins
it, read in `plannotator inbox --help` and the source its binary
bundles).

- **Where it is.** One local Inbox for each data folder, the one
  `PLANNOTATOR_DATA_DIR` in the calling process's environment names,
  else `~/.plannotator`, answering agents on its machine alone. A
  message is filed under a project, the folder its `project_path`
  names, else the sending session's.
- **Which Inbox on deneb.** One runs today, `~/.plannotator`, and every
  agent writes to it. swancloud's design
  `the-plannotator-inbox-runs-on-deneb-for-the-users-devices`
  (designing, its record 0052) keeps one Inbox per partition, named by
  the partition's name, which its stream and its herdr session carry,
  beside deneb's one Inbox, which keeps today's threads (its Decisions
  1 and 5). It gives an agent flume starts its stream's partition's
  Inbox by `PLANNOTATOR_DATA_DIR` in the agent's own environment, and
  how flume gives a stream's agents that variable is its question for
  flume's design, still the user's to answer (its Decision 7). It leaves
  `flume needs` reaching the Inbox to this change.
- **Questions.** A `:::question` block in a message is a question the
  person answers on a card of the Inbox's, under a key the Inbox gives
  it, `<message id>/<key>`. A send's `idempotency_key` returns the first
  message when sent again, and is refused with another body.
  `supersedes` withdraws open questions, so the person can no longer
  answer them, and `resolve_message` archives a thread.
- **Threads.** A send naming `thread` joins the newest open thread of
  that name in the project, from any session, or starts one.
  `read_thread` with no id lists the project's threads the caller sent
  in, each `unread` while the person wrote in it for the caller and no
  agent has read it.
- **Where a reply goes.** To the session that asked. It wakes that
  session only while it is live and runs under a host the Inbox wakes:
  Claude Code, Pi, OpenCode, DeepSeek Harness, Codex or Qwen Code. A
  reply to an asker that ended, or to a session of no such host, is
  saved: it waits in the Inbox, its thread `unread` for that session,
  and nothing runs (`packages/core/inbox-delivery.ts`,
  `inboxDeliveryState`). The person may hand a thread to a live session
  by hand.
- **Who a call is.** `plannotator inbox call [--session <id>] <action>
  <json>` acts from a script as the session `--session` names: an id of
  `[A-Za-z0-9][A-Za-z0-9._-]{0,127}`, with an optional host before a
  colon. A shell that names a session of its own, as every Claude Code
  session's does through `PLANNOTATOR_SESSION_TAG`, has another id
  refused, `session_mismatch`; a shell that names none takes any id
  (`apps/hook/server/inbox-call-command.ts`, `inboxCallerOf`).
- **No watch.** Nothing runs a command when the person answers: the
  Inbox offers no command a carrier can hang on an answer.

**The orchestrator.** A Claude Code session on deneb launches the
host's agentplot work. It runs `env -u HERDR_ENV -u HERDR_PANE_ID -u
HERDR_SOCKET flume launch …` with no `--recipient`, so its places have
none. A script of its own reads the text of `flume status` every 45
seconds, picks the lines that wait on their caller, and dedupes them by
hand. The session then answers a question itself or puts it to the
user under a key it makes up, and retries.

**deneb's host configuration** declares the streams `madswan`,
`madswan-planning`, `swancloud` and `swancloud-planning`, one pool, a
stream for each of its repositories under `[repos]`, no `[roles]` and
no `sources/`. The user ruled on 2026-11-05 that planning is a label of
each partition's work stream and not a stream of its own, and that each
partition's stream opens in the herdr session of its own name:
`madswan`, `swancloud`, and `agentplot`, now a partition of its own.
Each partition then runs its work in exactly one stream, which bears
its name.

### Where it breaks

1. **No recipient.** Clause 7 lets a launch have none, and on deneb that
   is most of them. Nothing but a poller sees their ends, and a poller
   that is down sees nothing.
2. **An address names nothing flume knows.** Its first part is a
   partition, which no clause defines and no code reads, beside the
   streams that already divide a host's work. The statement calls it a
   partition, the site a group, and crew fills it with a partition's
   name.
3. **No role runs at its address.** Every role stands, so the address
   crew's places name wakes nothing, and a standing role sees a question
   only on its board.
4. **Two designs of waking a role.** flume's `[roles]` wake and the
   runner's role offers do one thing. crew chose the runner for a defect
   of flume's: two partitions' runs of one role share a name. And
   flume's run of a role never ends of itself.
5. **A role declared on every host.** flume reads a role's run from the
   host's own configuration, one entry per address. A host of crew's
   would repeat every partition's every role, though all of it is
   crew's, written by crew from its partitions into its role record
   already.
6. **One door for every question.** Every place crew launches names the
   planner, so a question about a live system or the design goes
   through the planner, and a question about the orchestrator's work has
   no role at all.
7. **No bound.** A question delivered to a recipient that does not act,
   or pending for one that is gone, waits for ever. The stall is said to
   the same recipients who did not answer.
8. **A restarted recipient.** The notice is one line, which a new agent
   in the pane cannot place. An end delivered just before the restart is
   gone from the inbox. flume's plugin gives a caller's session nothing
   at its start.
9. **Answered twice, or too late.** A retry names no question. Two
   answerers each restart the stage, and an answer that comes after the
   place moved on starts the stage again over later work.
10. **The user's question.** Each answerer puts it to the user by hand
    under a key of its own. Nothing ties what the user is asked to the
    place's question, so a second reader asks again, and nothing
    withdraws a question answered some other way. A reply in the
    Plannotator Inbox reaches only the session that asked, and a role's
    run has ended by the time the user answers.

### Rules and records relied on

- **flume's statement** (agentplot/blueprints main at 2287669).
  Section 3's recipient and stream; 7, 8, 28, 32, 41, 42, 43, 49, 56,
  62, 69, 70, 72, 114, 130, 132, 143, 154, 155, 158, 180, 181, 190, 191,
  192 and 197. 69 says
  flume does the same for every caller. 72 says flume tells no person
  anything by itself. 120 says it runs no process of its own.
- **flume's records.**
  - 0009: an end is delivered once inside the settle, a wait is by
    place, and a caller's start collects.
  - 0011: an answer is a retry.
  - 0013: a recipient may be a role's address, woken by flume.
  - 0014: the words delivered, pending, collect and notice.
  - 0022 and 0040: a kind from outside is a package of a source the host
    declares in a file of its own, run only as a lock records it.
  - The record on the unlanded change
    `an-answer-reaches-the-open-session-within-an-hour`, accepted by the
    user on 2026-11-01: within an hour an answer reaches the asking
    agent's open session. That answer is the same retry this design
    guards with its key.
- **flume's rules.** core.1 (no person's queue named outside a profile),
  core.4 and core.34 (a kind names nothing a host owns, and runs from
  outside only as a lock records it), core.11 (a notice never sent into
  a question), core.13 (no process of flume's own), core.24, core.25,
  core.27 and core.28 (holds, delivery, the kind running through, and
  roles), core.30 (an agent's settings from their layers, and a location
  on the host the managed layer's), core.35 (no partition on a page or
  in a command's help, output or refusal).
- **crew's rules and records.** roles.5 (crew builds no mechanism its
  tools hold), roles.6 (records are the truth, a session's start
  collects), roles.7 (flume wakes a role), roles.9 (the runner turns
  launches into flume's); records 0047 move 6, 0048, 0049, 0052; the
  change `roles-run-as-flume-kinds`.
- **Boomdrive's rule core.15.** The runner adds no recipient of its own.
- **The user's boundary.** A design stops at flume and its host's
  configuration. Whatever writes that configuration is one configurer
  among many.

## Goals / Non-Goals

**Goals:**

- An address names what flume knows: a role, and the stream it answers
  in.
- Every main-level role answers as a Flume kind at its own address, and
  a question reaches the role it is for, in any stream.
- A role is declared once, by the tool whose role it is, for every
  stream and host it runs on. No host's configuration holds an entry
  for a role.
- Every question a place asks reaches a recipient whose run is woken
  for it, or one that collects it, whatever launched the place.
- A question that no role takes up within the place's stall bound
  reaches its stream's door.
- A question is answered once. A second answer, or a late one, starts
  nothing.
- A recipient that restarted sees every question still waiting on it,
  with enough to act on.
- A question that is the user's reaches them once, in their Plannotator
  Inbox, and their answer reaches the place as its retry's words with no
  hand carrying it, though the run that asked has ended.
- One owner for each part across flume, crew, Boomdrive and sluice.

**Non-Goals:**

- flume deciding which role a question is for, which questions are a
  person's, or telling a person anything itself (69, 72). The asking
  agent, the role and the door decide.
- A question kept to its place's stream. The stream bounds the room a
  place's agents take, and nothing of whom they ask.
- flume reading crew's own records or teams file. crew writes its roles
  in flume's form.
- A person's queue, the Plannotator Inbox or any other, named in
  flume's code or its kinds (core.1, core.35).
- A process of flume's that watches the user's queue or waits for
  answers (120).
- Changing what an answer is. It stays a retry naming the stage with
  the answer as its words, the question before it (42, 49). The open
  session within the hour is the other change's.
- A session standing in for a role's address. Standing roles leave as
  their partitions move them (decision 11.5).
- sluice's own outcomes, which are not questions. They stay sluice's
  (decision 12.2; Found beside).
- Moving crew's own cards off Pending You. The cards crew's commands
  raise, and the follower's Pending You notice that runs `crew
  _answered` for them, stay until crew moves them to the Inbox (the
  user, 2026-11-05).
- Building any of it before the user has read this design.

## Decisions

### 1. An address is a role of a stream

The user, 2026-11-05, of `<partition>/<role>`: "Do we need the
partition part of the address? We're already going to use streams to
partition where agents run. I guess I was just assuming that
communication had to stay within a stream but we actually probably
don't want that. I feel like we just invented this address thing as
well and it's a little weak."

**What an address is in flume's statement today.** 132 gives it as
`<partition>/<role>`, a recipient named in place of a session. A
settle that finds an end pending for it with no run standing, or a
carrier's `flume wake <address>`, launches the run the host's `[roles]`
table names for it (C.7), and the run's agent takes the address as its
session's name. That much is built: `is_address`, `flume wake`,
`--recipient <address>`, a run's inbox collecting for its address, and
status's list of addresses with ends pending. Nothing defines a
partition. flume reads the address as a key of the host's table, takes
the run's name from the part after the slash, and reads the part before
it for nothing.

**What this design makes of it.** An address is `<stream>/<role>`: the
role a source of the host declares (decision 2) for a stream the host
declares (143). `madswan/ops` is ops of the stream `madswan`.

- **The stream is the one flume already has.** The host declares it,
  with its room, pool and terminal, and every place runs in one. Since
  planning is a label of a stream's work (180; the user's ruling of
  2026-11-05), each of crew's partitions runs its work in exactly one
  stream, and the partition's roles are that stream's: crew writes
  `<stream>/<role>` for each. On deneb each stream bears its partition's
  name, so `madswan/planner`, the address the runner's places already
  name, keeps its letters and names the role `planner` of the stream
  `madswan`.
- **A role's run runs in its address's stream**, under its kind's
  label, so its room, pool and terminal are that stream's (143, 180,
  190). A source names no other stream for it.
- **An address's stream is one the host declares.** An address whose
  stream the host does not declare is an address of this host only
  where its source declares it carried, its run on another host
  (decision 11.4). Otherwise its entry declares nothing, and status
  names it and the stream (4, 143).
- **A bare role names its role in the asking place's stream.** `flume
  needs --to ops` from a place in `madswan` is `madswan/ops`. A bare
  name is read as a role only where nothing but a role can be meant, a
  stop-short's `--to`. A recipient a launch names, and a wake, write an
  address whole, since a session's name is a bare word too (7), and a
  carrier has no place whose stream would complete it.
- **A question may go to any stream's role.** `flume needs --to
  swancloud/ops` from a place in `madswan` reaches swancloud's ops,
  whose run runs in `swancloud`. A launch may name any stream's role as
  its recipient. The stream bounds where a place's agents run and how
  many, never whom they ask.
- **The rest of 132 stands as built.** An address is a recipient that a
  launch, a stop-short or a carrier names; `flume wake <address>` wakes
  it; its run's agent takes it as its session's name and collects for
  it; status lists the addresses with ends pending. The form a name
  takes, two words joined by a slash, is the same.

`flume wake`'s help and refusal, which say `<partition>/<role>`, then
say `<stream>/<role>`, which core.35 asks of a command's words.

Rejected:

- **A partition beside the stream**, 132 as stated. A second division of
  the host's work, which flume cannot check, beside the one it holds.
  Every host's keys and every source's addresses would repeat it.
- **The role alone, always in the place's stream.** No question could
  reach another stream's role, and a carrier's wake and a launch's
  recipient have no place whose stream would complete it.
- **A role and a stream given apart**, `--to ops --stream swancloud`.
  The same name in two flags, where a launch's recipients are one name
  each, and a wake or a status line would need the pair too.
- **A role for the whole host**, `ops`, one per host. Two streams' ops
  would be one run, which the user refused for the planner: "an agent
  might need something from ops separately from the planner or the
  designer", of each partition's own.
- **A bare role as a launch's recipient**, meaning the launch's stream.
  A launch's bare name is a session's (7), so `--recipient planner`
  could be either.

### 2. crew declares its roles once, and every main-level role answers as a Flume kind at its own address

crew's change `roles-run-as-flume-kinds`, as built on `teamless/deneb`,
is the base. Its five kinds, its turn (`crew role start`, act,
`crew role end`), its asks and its `crew role open` stand as crew built
them. Each role answers at its address in its partition's stream:
`madswan/design`, `madswan/planner`, `madswan/ops`, `madswan/reviewer`
and `madswan/operator`. On it, this design sets six things.

**crew is a source of the host, and declares its roles there.** A host
declares each source of its kinds from outside in a file of its own,
naming where the source's packages lie (191). That declaration may also
name `roles`, the file in which the source declares the roles it
answers. crew's, on every host that runs crew's main levels:

```toml
# $XDG_CONFIG_HOME/flume/sources/crew.toml, of the host's managed layer
kinds_dir = "<crew's release>/kinds"              # crew/crew-<role>@<version>
roles     = "~/.local/state/crew/flume/roles.toml"  # crew writes it
```

crew writes the roles file whole, from its own partitions, as it writes
its role record today: at `crew main up` and beside each ask. It holds
an entry for each address of each partition's stream, in flume's form:

```toml
[roles."madswan/operator"]
kind = "crew-operator"                 # one of crew's own kinds
repository = "/…/repositories/afterthought/blueprints"
name = "madswan"
intent = "madswan-operator"
answers = "Whatever reaches the stream's work from outside it, and a question no role took up."
answerer = true                        # stands behind the stream madswan

[roles."madswan/operator".agents]
model = "opus"

[roles."madswan/ops"]
kind = "crew-ops"
repository = "/…/repositories/afterthought/blueprints"
name = "madswan"
intent = "madswan-ops"
answers = "A live system: what is deployed, a credential, a host's tools, a deploy, a landing that failed, main red."

[roles."wldn/planner"]
carried = true                         # wldn's stream runs on another host
answers = "The work's scope: work outside a bolt's goal, which bolt a finding belongs in, the order of work."
```

Ten partitions' streams hold ten addresses of each role, every one run
by the same package, `crew/crew-<role>`, which the host locks once. A
role's kind is one of its source's own, named by its name, its version
the locks' (154, 155). flume reads every source's roles as they stand at
each launch, wake, start, stall and status, and writes nothing in them
(191).

**What a run needs, and whose it is.** 132 launches an address's run as
a place. Every fact of that launch is crew's but the room it runs in:

| what a role's run is launched with | whose | why |
|---|---|---|
| its kind | crew's: a package of its release, the same for every stream | 157 |
| its repository, by its location on this host, and its base | crew's: the partition's first blueprints repository, where its runner keeps it | 9, 132 |
| its stream | its address's, which crew writes for the partition's stream | decision 1 |
| its label, `planning` | crew's kinds, which declare it | 180 |
| its name, intent, priority, points and brief | crew's, as any caller's launch gives them | 7, 70, 147, 165 |
| its agents' settings, the model and the effort among them | crew's role definitions, as a launch's settings | 7, 32, 197 |
| what it answers, and whether it stands behind its stream | crew's briefs: the operator is the door | decisions 3, 6 |
| that it runs on another host | crew's: the partition's `machine` | decision 11.4 |
| the content of its kind, trusted | the host's lock | 155 |
| its stream's bound, pool and accounts | the host's | 143, 144 |

**What of the host remains.** Three things, each the host's and not
crew's:

1. **crew declared a source**, `sources/crew.toml`: where crew's
   packages and crew's roles lie on this host. It is the host's because
   the host decides whose kinds and whose roles run under its accounts
   on its machine, and because a location on the host is the managed
   layer's (191, C.7; core.30). A tool that could declare itself would
   run its roles on any host it reached. It is one file for crew, the
   same on every host but for its paths, and names no stream and no
   role.
2. **The lock of crew's kinds** (155): the host's trust in the content
   that will drive agents with its accounts, each version recorded once
   for every stream's runs (core.34).
3. **Its streams**: each stream it runs, `madswan`, `swancloud` and
   `agentplot` on deneb, with its bound, pool, priority and terminal
   (143, 190), and the `planning` label's reserve in each (181). They
   are the host's because they are its agents and its accounts. crew's
   addresses say which stream a role answers in, the host how much room
   that stream has, and an address whose stream the host does not
   declare wakes nothing (decision 1). A host declares its streams for
   every place it runs, roles or not.

Nothing else. The host's configuration has no `[roles]` table and no
answerer. Who stands behind a stream is crew's to say, since the door
is the partition's operator.

**flume launches a role's run.** A settle that finds an end pending for
an address with no run standing, or a carrier's `flume wake <address>`,
launches the run its source declares for the address, in the address's
stream (132). The runner launches no role's run. It carries crew's own
events, an ask in crew's records and a place on another host waiting on
the role, by `flume wake <address>` on the role's host (the user's
ruling of 2026-11-05). crew's follower writes the user's answer in the
Inbox as an ask (decision 10), and the runner wakes the role for it.

**A run is named per address.** An entry's `name` names the run's
place, and flume names it `<stream>-<role>` where the entry names none.
crew gives the stream's name, so madswan's planner runs as
`crew-planner--madswan`, the place crew's `FLUME_PLACE` reading takes,
and swancloud's as `crew-planner--swancloud`. A run an earlier flume
named by the role alone is found by its role field, which the run's
lookup reads first (`wake.rs`, `run_of`).

**A run is over once its turn delivers.** A role's run lands nothing,
and nothing is its caller. flume declares it done at the settle that
reads its last stage delivered, with no caller's word, and tidies it
(114). The next end pending for the address wakes another run, whose
start takes what came since (crew's Decision 7). This is the runner's
R3, done where the run is launched.

**A run's own stall goes to the door.** A role's run has no recipient.
Where it waits on its caller, lost, never begun or stopped short, its
stall goes to the answerer behind its stream, and never to its own
address, so the door learns that a role cannot run (decision 6).

**A fault stays its own.** As a source's fault refuses only the launches
naming it (191):

- a roles file that cannot be read declares no role, and status names
  it and why;
- an entry that cannot be read declares nothing for its address, and
  status names the address and the key: one that names a kind not its
  source's, an `env`, a pool, an account or a `stream`, whose address
  names a stream the host does not declare and is not carried, or that
  gives `carried` beside a run's keys or `answerer`;
- an address two entries declare, of one source or two, and two
  addresses whose runs would be one place, of one repository, kind and
  name, are declared by neither, and status names both;
- an address declared by nothing wakes nothing, as decision 5 has it.

So a source a program dropped in never stops the host's other work,
and a role crew declared wrong never wakes as another's.

Rejected:

- **The roles in the host's configuration**, a `[roles]` entry per
  address, as 132 and C.7 have them on main. Every host repeats every
  stream's every role, and whoever writes the host's configuration
  renders crew's partitions into it. The user ruled on 2026-11-05 that
  roles are never repeated for each host: "they are an artifact of
  crew". The table leaves C.7.
- **The roles in crew's package.** A package is one kind at one
  version, the same on every host (157). The addresses are crew's
  partitions' streams, and which host runs each one's main level.
- **flume reads crew's role record or teams file.** flume would read a
  caller's own formats, `roles.rec` on a branch of a partition's state
  repository or the teams file, and every other tool with roles would
  need a reader of its own in flume.
- **A seam for roles, with a profile of crew's.** A seam serves the
  tools flume integrates with in each of its parts (core.2). This would
  be one for one caller, and crew's format in flume's code.
- **A template flume fills**, one entry per role with the streams
  listed, flume making the addresses. Partitions differ in their models
  and in which roles have moved, and crew writes the file by program,
  so a flat entry per address costs nothing.
- **The runner as the launcher**, crew's Decision 1. Two launchers of
  one thing, and a question's wake waits on the runner's pass. The user
  ruled flume the launcher on 2026-11-05.
- **A session that serves an address**, so standing roles are reached
  by their addresses. It only bridges the roles that have not moved
  (decision 11.5).

### 3. Which needs are whose

From crew's briefs, a need is the role's whose work it is:

| the need | whose | the brief that says so |
|---|---|---|
| A decision the design has not made: how something should behave, a spec silent or at odds with the code, a rule's reading, a record to write | the design agent | design.md, *Answering a design question* |
| The work's scope: work outside the bolt's goal, which bolt a finding belongs in, a unit, chore or fix to add, the order of work, and work found that a stage before must do again (`--found`, 131) | the planner | planner.md, "Its places' questions are yours", "Work outside a bolt's goal" |
| A live system: a reading of what is deployed, a credential, a host's tools, a deploy, a landing that failed, main red | main-level ops | design.md, "from `{{MAIN_OPS}}` for a live system"; main-ops.md, a fix of main |
| A choice that is the user's alone: their name, money, plans, anything public or hard to undo | the user, asked once in the Plannotator Inbox by the role whose subject it is | design.md, *Decision records*; capture's rule |
| None of these, or the agent cannot tell | the place's owner, the role its launch names | — |
| A question that no role took up within the bound | the operator, the door of the place's stream | operator.md, "the partition's door", "make sure the user sees a wait that has none" |

The table holds in every stream. Which stream's role a need is for is
the asking agent's to tell: a question about swancloud's deployed queue
is swancloud's ops's, whichever stream the asking place runs in.

The reviewer is asked nothing: it reads one batch and writes one file.
crew's roles file gives it no `answers`, so no prompt lists it.

### 4. Who picks

Four could pick the role a question goes to.

- **The launch.** The caller knows whom the work is for. crew names the
  planner for a bolt's work, ops for a fix of main, and a role for its
  own landing work (`crew role land`). The launch cannot know which
  question a stage will ask.
- **The asking agent.** It alone knows what it needs. It holds the
  change, the rules and the brief, and can tell a design decision from a
  reading of a live system from a scope question, and which stream's
  system it is about. It does not know the host's roles unless it is
  told them, and it may pick wrong.
- **The kind.** A kind names nothing a host owns, an address among them
  (core.4, core.34, 158). Its stage cannot know its question in advance.
- **A door, crew's operator, for every question.** It routes by the same
  table the asking agent can read. It costs a woken run and a hop on
  every question, and its brief has it carry and never answer.

The design takes three of them, each where it knows most:

- **The launch names the owner.** A place's recipients are its owner, the
  role its launch names, or the door of its stream where it names none
  (decision 5).
- **The asking agent may address one question to another role**:
  `flume needs [--found] --to <role or address> "<words>"`. The
  stop-short part of every stage's prompt lists each address the agent
  may name, with the words its source gives for what it answers: every
  address a source declares with `answers`, a run's or a carried one's
  (decision 11.4), but the place's own role's where the place is a
  role's run. The place's own stream's roles come first, each by the
  role alone; every other stream's follow, each by its whole address.
  The stop-short's end goes to that address alone, and not to the
  place's recipients. A name outside the list is refused, naming the
  list, so the agent can ask again. With no `--to`, the end goes to the
  place's recipients. The list is read from the sources at each start,
  and comes after every part of the prompt that a stage's repetitions
  share (core.31).
- **A role that takes a question not its own passes it.** It tells the
  role whose it is, by `crew tell`, which is an ask for a role that runs
  as its kind, with the place, the key and the words, and holds the
  waiting move, "passed to madswan/design", with a bound (decision 7).
- **The door stands behind every stall**, the last to pick (decision 6).

So "an agent might need something from ops separately from the planner
or the designer" (the user, 2026-11-05) is `flume needs --to ops`, and
a need of another stream's ops is `flume needs --to swancloud/ops`. A
wrong pick costs one pass, which the door would cost on every question.

A role still standing is not in the list, since crew declares no run
for it. An agent whose question is a standing role's asks the owner,
who carries it to that role as today.

Rejected:

- **Every question to the door first.** Two runs and a hop for each
  question, where most are the owner's.
- **Every question to every role of the stream**, the first to answer
  taking it by its key. Every role woken for every question, and a
  question about a live system read by the design agent.
- **flume reading the words to choose a role.** flume would judge a
  question, which 69 leaves to its callers.
- **Only the place's own stream's roles in the list.** The user:
  "communication had to stay within a stream but we actually probably
  don't want that". An agent that needs another stream's system would
  have to go through its owner.

### 5. Who answers each kind of launch

A place's recipients are resolved at its launch, the first of these
that gives any:

1. the recipients the launch names, `--recipient`, one or several, each
   a session's name or a role's whole address, of any stream;
2. `$FLUME_RECIPIENT` in the launching command's environment;
3. the session the launching command runs in, as the terminal names it
   (190);
4. the answerer behind the place's stream, the role a source declares to
   stand behind it (decision 6).

| how the place was launched | its recipients | the roles its agent may address | behind every stall |
|---|---|---|---|
| a shell or a script with no session, naming none, the orchestrator's among them | its stream's door, `agentplot/operator` for the `agentplot` stream | its own stream's roles by name, `design`, `planner`, `ops`, each that runs as its kind; every other stream's by its address | the door is the recipient |
| a herdr pane, by `/launch` or a person's command | that pane's session | the same | its stream's door |
| the runner, for crew's work | the role crew names: `<stream>/planner` for a bolt's work, `<stream>/ops` for a fix of main, a role for its own landing work | the same | its stream's door |
| any launch naming a role's address | that address: its run's pane, its run woken where none is live | the same | its stream's door |
| a role's run, launched by flume | none | the same, but its own address | its stream's door, never the run's own address |

A launch is refused, making nothing (4; core.8):

- where it finds none of the four, naming each and how to give one: a
  `--recipient`, `$FLUME_RECIPIENT`, a session, or a stream a role
  stands behind;
- where its stream's answerer is two roles' at once, naming both;
- where it names a role's address no source declares, or two declare,
  and no answerer stands behind its stream, naming the address. Where an
  answerer stands behind it, the launch is taken: the address's ends
  stay pending, status says it wakes nothing, and its questions reach
  the door at their stall. So a partition moves its roles one at a
  time, and the runner's launches naming a planner that still stands
  keep working.

The recipients are fixed at the launch (8). The answerer behind them is
read at each stall.

The orchestrator strips herdr's variables so that its places are not
its pane's. Its launches run in the `agentplot` stream and take that
stream's door, and their agents address agentplot's design agent,
planner or ops by name, or another stream's by its address. It needs
no script to poll status.

### 6. The bound: a stall reaches the owner, the asked role and the door

A role a source declares may stand behind its address's stream,
`answerer = true`: it is that stream's answerer. crew declares each
partition's operator to stand behind the partition's stream, so
`madswan/operator` stands behind `madswan`, `swancloud/operator` behind
`swancloud`, and `agentplot/operator` behind `agentplot`. One role
stands behind a stream at most: where two do, of one source or two, the
stream has none, status names both, and a launch that would take it is
refused naming both. A stream no role stands behind has none: a launch
there names its recipient or runs in a session, names no address that
wakes nothing, and its stalls go to the place's recipients and the role
its stop-short named. The answerer is read from the sources as they
stand when asked, at each launch and at each stall, and never kept, as
the stream and the pool are (143).

The place's stall bound, five minutes unless the launch sets another
(130), is the one bound. Once a place has waited on its caller that
long under no hold's words, it is stalled, and its stall is delivered
once to each of these:

- each recipient of the place, its owner;
- the role the stop-short it waits on named by `--to`, of whichever
  stream;
- the answerer behind the place's own stream, unless it is one of
  those, and never a role's run's own address.

The door is the asking place's stream's, whichever stream's role the
question named: the door stands behind the places of its stream, and a
question asked of another stream's role is still its place's work.

The stall carries the move the place waits on, and where that move
answers a stop-short, the stop-short's words, whether it needs an
answer or found work, its key and whom it named, and the words of a
hold that lapsed. A reader who never saw the place has the question
whole. It is delivered as any end is: into its run's pane, or pending
and waking the run (43, 132).

A hold's bound passing stalls the place again, a new stall with a key
of its own, delivered to the same three. So a question is never left
under nobody's words longer than the stall bound before its owner and
the door know, and a role that took it and let it lie is found out at
its hold's bound.

**When the role does not answer.**

| what happened | what the bound does |
|---|---|
| The role's run was never launched or never began: no account, no room in its stream, its kind unlocked, a refusal | Its ends stay pending, and status says why the run was not woken. At the stall the owner and the door have the question. |
| The run took the question and its turn ended with neither an answer nor a hold | At the stall the owner and the door have it. |
| The run held it and let the hold lie | At the hold's bound the place stalls again, to the owner, the asked role and the door, with the hold's words. |
| The role still stands, and crew declares no run for its address | The end stays pending. At the stall the door has it, and carries it to the standing role as crew does today, or to the user. |
| The door's own run cannot run | Its run's stall goes to no address, since it is its own stream's answerer, and stands in status under the host's addresses (Risks). |

An end of a stall recorded before this change names no answerer. The
first settle after the build reads it as the answerer's too, where the
place still waits under no hold's words, so the questions waiting on
deneb when the build lands reach the door at once.

Rejected:

- **An answerer in the host's configuration**, `[host] answerer` and a
  stream's own. The door behind a stream is the partition's operator,
  which is crew's to say, and a host would repeat it for every stream
  it runs.
- **A second bound for the door, longer than the stall.** Another knob
  on every host, and the stall already says the place waits on its
  caller with nobody's words. A launch that wants more room gives
  `--stall-after`.
- **The door only for ends no recipient collected.** That misses a role
  that took the question and did nothing, which is what the user is
  asking about.
- **Every end to the door at once.** Two answerers from the first
  minute, and the role whose question it is loses its turn.
- **The asked role's stream's door too.** Two doors for one question,
  each able to ask the user, where the asking place's door already
  stands behind it.
- **A chain of answerers ordered by flume.** flume would order roles,
  which is the sources' and the kinds' to decide. The owner, the asked
  role and one door, with holds that lapse, are enough.

### 7. A hold on the waiting move

A caller may hold the move the place waits on, with words saying who
makes it and how, and an optional bound:

```sh
flume hold <place> "madswan-planner has it" --waiting --for 1h
flume hold <place> "passed to madswan/design" --waiting --for 1h
flume hold <place> "asked the user in the Inbox, msg_01K…" --waiting --for 1d
```

This is a hold as 130 already has them, with its words and since when,
put on the place's waiting move rather than on a stage not yet started.
A later hold on the same move replaces the words and the bound. A place
waiting under a hold's words is never stalled, so its question does not
go to the door. Status gives the hold with its words, since when and
its bound. A retry naming the stage or the submission ends it, as the
move made (47, 49). A drop ends it with the place. Where the hold gives
a bound, the settle that reads the bound passed lapses the hold: status
says it lapsed and when, and the place stalls at once, its stall
delivered as decision 6 has it. Where it gives none, it stands until the
move is made, as every hold does. It is refused, naming why, on a place
that waits on no move of its caller's.

Rejected:

- **A command of its own, `flume claim`.** It would be a second kind of
  words on a waiting place, beside the hold's, with the same effect on
  the stall.
- **A hold implied by an inbox's read.** A read marks nothing (43), and
  a role's run that collects a question has not yet said it will
  answer it.

### 8. An answer is taken once, by its key

Every end already has a key in held state, by which the settle records
it once (`EndOf::key`). This change gives it to callers as one token:

- `stage/<stage>/<start>/<state>` for a stage's end, such as
  `stage/construct/1/ended-short`;
- `submission/<n>/<outcome>` for a submission's;
- `stall/<since>` for a stall.

Each notice, each collected end, each waiting place the inbox gives,
and status give the key. A stop-short's key names its start, so a
fresh agent that asks again asks under a new key, a new question.

`flume retry <place> --stage <stage> --answers <key> --words "<answer>"`
is taken only while the place waits on the stop-short `<key>` names.
Otherwise it is refused, naming the place, the answer that stop-short
took, who gave it and when, or naming the move the place waits on now.
A retry so taken records on the stop-short's end its answer: the words,
who gave them, as the retrying command's recipient names it, and when.
A retry with no `--answers` is taken as today, so every caller keeps
working.

The open-session change types an answer into the asking agent's open
session within the hour. That answer is the same retry and takes the
same key, so an answer is taken once whether it reaches the open
session or a fresh agent. A tell carries no key: it is no answer (55),
until the open-session change makes a tell within the hour one, and
then it takes `--answers` too.

### 9. What a restarted recipient sees

**The inbox gives what waits on it.** `flume inbox`'s answer keeps its
`ends`, every end pending for the recipient, each delivered by the
inbox. It gains `waiting`: every place on the host that waits on its
caller and that this recipient is a recipient of, was named by the
stop-short it waits on, or was given a stall of as the answerer. Each is
given with:

- the place, its kind, its name, its stream and the first line of its
  brief;
- the move it waits on, and since when;
- where the move answers a stop-short, its words, whether it needs an
  answer or found work, its key and the address it named;
- a hold on the move, with its words, since when and its bound.

`waiting` delivers and marks nothing (43; core.25), so a recipient that
collected a question, then restarted, still finds it.

**A role's run** collects at its start: `crew role start` for crew's
asks, then `flume inbox --json` for its address, which takes the role
from `FLUME_PLACE` (132). Every question waiting on the role is in it,
with its key, whichever stream asked it, however many runs came and
went.

**A person's session.** flume's Claude Code plugin's SessionStart hook,
in a session flume did not start, acts:

1. It runs `flume inbox` as the session the terminal names, never as an
   address, so a person's session never takes a role's ends. Under the
   printed command, which names no session, it does nothing.
2. Where the answer holds an end or a waiting place, it gives the agent
   the answer's words as the session's added context. Each place gets
   its question, its key, and the three ways on: answer by retry with
   the key, hold the move with words, or leave it, and the door has it
   at the stall.
3. Where the answer holds nothing, it gives nothing.

It never blocks the start, under its 20 seconds' timeout, and a flume
that cannot be reached gives nothing. This is sluice's design for a
session's start (sluice 51, 135), so the two tools do one thing one
way.

**The notice.** It stays one line, and names more:

```
flume: change--every-question (Every question a place asks reaches…) construct needs an answer [stage/construct/1/ended-short]; read: flume inbox
```

It names the place, its brief's first line cut at 60 characters, the
end, its key, and the command that reads it whole: `flume inbox` for an
end that waits on a move, which gives the question and how to answer,
and `flume status <place>` for any other. It names no fact to act on
blindly (core.25).

Rejected:

- **Recording, at the launch, which conversation in the pane launched
  the place, and not delivering to another.** herdr gives no
  conversation's identity of a pane flume did not start, and the stall
  reaches the door either way.
- **A notice holding the question's words.** Words are many lines, and
  a notice is typed into a pane (core.11).

### 10. The answering run's protocol, and the user's one question

flume's part ends at the role: it reaches the role, gives it the
question with its key, takes the hold that says the role is on it, and
takes the answer once. What the role does is its kind's. The design
gives the protocol each answering role's run follows, for crew's turn
and briefs, and for any source's own answering kind:

1. **Collect.** `crew role start`, then `flume inbox --json` for its
   address, reading `ends` and `waiting`. The run reads flume's own list
   of what waits on its address, on the user's word of 2026-11-05: "Yes:
   a role's run reads Flume's list for its address".
2. **Hold what it takes**, before anything else: `flume hold <place>
   "<run> has it" --waiting --for 1h`, so a turn that runs long does not
   stall the place.
3. **Answer what is its own**: `crew answer <work or place> --answers
   <key> "<answer>"`, which the runner carries as `flume retry …
   --answers <key>`.
4. **Pass what is another role's**, of its own stream or another's:
   `crew tell <stream>-<role>` with the place, the key and the words,
   and hold "passed to <address>" with a bound.
5. **Ask the user what is the user's**, once, in the Plannotator Inbox
   of its stream's partition (*Which Inbox*, below):
   - **Hold first**: "asking the user in the Inbox, thread
     madswan/planner", with a bound, so a second reader sees the
     question taken before it is asked; once sent, hold again naming the
     message, "asked the user in the Inbox, msg_01K…".
   - **As its address, not as the run.** Every Inbox call the run makes
     is `plannotator inbox call --session crew:<stream>-<role>`, in an
     environment without the run's own `PLANNOTATOR_SESSION_TAG` and
     with the `PLANNOTATOR_DATA_DIR` its run was started with, so it
     writes as its address's Inbox identity, which outlives the run, in
     its partition's Inbox, and not as the run's Claude Code session,
     which ends at its delivery.
     It passes `project_path`, the repository its entry names, so every
     run of the address files in one project.
   - **In its address's thread.** `thread` is the address,
     `madswan/planner`: a role's questions to the user are one thread,
     every run of the role joining it, as the Inbox's own rule gives a
     work stream one thread.
   - **One question for an end.** A `:::question` block giving the
     question whole, the place, the first line of its brief and the
     end's key, sent under the `idempotency_key`
     `flume/<host>/<digest of the place's location>/<end's key>`. The
     run first reads its thread for an open question naming the end's
     key. A second run, a second role or the same run after a restart
     that sends it again gets the first message back, or a refusal where
     its words differ, and asks nothing new.
6. **Withdraw a question that no longer waits**: a retry refused for its
   key, a question answered in a pane, a place dropped. The next send in
   the thread names it in `supersedes`, so the user can no longer answer
   it, and `resolve_message` archives the thread once nothing in it
   waits.
7. **The door, on a stall.** The operator's run reads the stall's words
   and any lapsed hold:
   - nobody had the question: it passes it once to the role whose it is,
     by decision 3's table, of whichever stream, and holds with a bound;
   - a role had it and its hold lapsed, or the question is the user's:
     where the lapsed hold names a question already put to the user, it
     holds again naming the message and asks nothing, since the user's
     answer reaches the role that asked through that role's own Inbox;
     otherwise it asks the user once, in its own address's thread under
     the same key, which its Inbox refuses where a role of its own stream
     already asked it there. It never asks the user twice for a
     question.

**The answer's way back to a run that ended.** A role's run is over at
its delivery, so the session that asked is gone when the user answers.
The answer reaches the role's next run so:

1. The user answers in the Inbox. The reply goes to the address's
   identity, `crew:madswan-planner`, of no host the Inbox wakes, so it
   waits in the Inbox, its thread `unread` for that identity.
2. **crew's follower** reads, at its minute, the threads of each role on
   its host that runs as its kind, as that role's identity (`read_thread`
   with no id) in that role's partition's Inbox, and for each thread
   `unread` writes an ask of the role, keyed by the person's last message
   in it, so a later minute writes no second ask. It reads nothing of the reply, as `crew _answered` reads
   nothing of a card.
3. **The runner** wakes the role for the ask at its pass, `flume wake
   madswan/planner` (decision 2).
4. **The next run**, at its start, takes the ask by `crew role start`,
   reads flume's list for its address, where the place still waits with
   the hold naming the message, reads its thread as its identity, which
   settles it, takes the answer to the question naming the end's key,
   and retries with `--answers <key>` and the user's words, so the fresh
   agent's prompt carries the question with the answer (42, 49). A
   question the user skipped or set aside is the role's to ask again or
   to leave to the hold's bound.

Any run of the role woken for something else reads the reply at its
start too, since the waiting place's hold names the message. An answer
so reaches the place within the follower's minute and the runner's pass.
Where either is down, the hold's bound passes and the place stalls to
its owner, the role its question named and the door, each woken by
flume alone: the role that asked, where it is one of them, reads its
thread at its run's start, and the door finds the question asked and
asks nothing (point 7).

**What Plannotator lacks**, read against 0.28.48:

- **A watch.** Nothing runs when the person answers a session no host
  wakes, so crew's follower reads each identity at its minute. A command
  the Inbox runs for each reply to such a session, `plannotator inbox
  watch [--session <id>] -- <command>`, would carry an answer at once,
  and the follower would hang its writing of the ask on it.
- **An identity for an address.** A reply goes to the session that
  asked, and a role's runs are many sessions. The design has each call
  drop the run's own `PLANNOTATOR_SESSION_TAG` and name `--session
  crew:<stream>-<role>`, which the Inbox takes from a shell that names
  no session. The Inbox's own words offer dropping the variable only for
  a shell older than its session, so this is a use it does not promise.
  An identity a caller names beside its session, owned by no host, would
  make it one.
- **An address as an id.** An id holds no `/`, so the identity spells
  the address `<stream>-<role>`. The thread's name gives it whole.
- **A key of the asker's.** The Inbox keys a question by a key of its
  own. The end's key rides in the question's words and in the send's
  `idempotency_key`, whose refusal of other words is how a second asker
  learns the question was asked.

None stops the design. The first two are asks of Plannotator (12.5), and
whether the moves wait for them is the user's (Open Questions).

**Never asked twice** is then held at four points:

- the hold, which keeps the question from the door while its role works
  on it;
- the send's `idempotency_key`, which gives a second asker of the same
  Inbox the first message or refuses its words;
- the answer's key, which keeps a second answer from starting anything;
- the withdrawal, which takes down a question answered elsewhere.

**Which queue.** A place's question put to the user, by a role's run or
by the door at a stall, goes to the Plannotator Inbox, and so does one
a person's own session puts to the user (the user's ruling of
2026-11-05, Rejected). A person's own session asks as itself, in the
Inbox its own environment names, after holding the move, and the Inbox
wakes it with the reply while it is live. crew's own cards stay on
Pending You. The user, 2026-11-05: "crew's own cards stay on Pending You
until crew moves them to the Inbox". The cards crew's commands raise, a
batch, a proposal, a kit's landings, a deploy's offer and a decision
record, and the follower's Pending You notice that runs `crew
_answered` for them, stand as they are.

**Which Inbox.** A role's run writes to its stream's partition's Inbox,
by the `PLANNOTATOR_DATA_DIR` its run is started with, as swancloud's
design `the-plannotator-inbox-runs-on-deneb-for-the-users-devices`
(designing, its record 0052) gives a stream's agents their Inbox: one
Inbox per partition, named by the partition's name, which is its
stream's (its Decisions 1 and 7). How flume gives a stream's agents
that variable is the question that design puts to flume's design, still
the user's to answer (its Decision 7). Until it is answered and built, a
run is started with none, and writes to deneb's one Inbox,
`~/.plannotator`, as every agent flume starts does. crew's follower,
which reads every partition on its host, reads each role's identity in
that role's partition's Inbox: by the data folder of the partition's
Inbox, which swancloud gives crew with the partition in its teams file
once it gives the partition's stream's agents that Inbox, and in deneb's
one Inbox while it gives none. So the role's runs and the follower read
one Inbox, the role's partition's, whichever stream the asking place
runs in, and a question a role asks for another stream's place shows in
the role's own partition's window. Each Inbox call names its Inbox by the
variable alone, and nothing of flume's names a data folder, so this
design and swancloud's need nothing of each other in either order.

Rejected:

- **flume asks the user, through a seam for a person's queue.** It
  breaks 72 and core.1, and flume would judge which questions are a
  person's.
- **The user asked by whoever first reads the question, with no hold.**
  That is today's orchestrator, and two readers ask twice.
- **Pending You for a place's question**, the hosted card queue crew's
  roles ask the user on today. The user, 2026-11-05, of the operator
  putting a stalled question to the user: "I think you're referring to
  an operator and that's fine. I think I would be transitioning from
  pending you to Planeter inbox though." crew's own cards stay there
  until crew moves them (*Which queue*).
- **The asking run's own session as the asker.** The reply would wait
  for a session that ended, and a later run, another session, would
  never find it `unread` as its own.
- **A thread for each question**, named by the place and the end. The
  user would have a thread for every question of every role, where the
  Inbox gives a work stream one thread, and a role is one. The end's key
  in each question keeps them apart in its address's thread.
- **The user handing the thread to the next run.** A role's next run is
  not live until it is woken, and the hand-off would be the user's act,
  which nothing asks of them.

### 11. What a role's run needs from flume, crew and the runner

#### 11.1 From flume

- **Addresses on streams**: `<stream>/<role>`, its stream one the host
  declares or its address carried; a bare role in `--to` read as the
  asking place's stream's; `flume wake`'s help and refusal naming
  `<stream>/<role>` (decision 1).
- **Roles read from their source**: a source's declaration's `roles`,
  the file the source writes, read as it stands, each entry's kind one
  of the source's own and locked, and a fault confined to its file, its
  entry or its address (decision 2). The host's `[roles]` table leaves
  the configuration.
- **Whole-address names**: an entry's `name`, else `<stream>-<role>`,
  and two addresses making one place declared by neither (decision 2).
- **flume as the launcher**: an entry's `intent` and `agents` beside the
  keys of 132, the run launched in its address's stream, its settings
  standing as a launch's in their layers (decision 2).
- **A run over at its delivery**, done and tidied by flume (decision 2).
- **The answerer**: a role standing behind its address's stream, read
  from the sources at each launch and each stall (decision 6).
- **The bound**: the stall to the owner, the asked role and the door, a
  run's own stall to the door, the hold on the waiting move with a bound
  that lapses (decisions 6, 7).
- **The answer's key**: `--answers <key>`, taken once (decision 8).
- **Routing**: `flume needs --to <role or address>`, and the roles
  listed in the stop-short part of every prompt with their sources'
  `answers`, the place's own stream's by name and every other's whole
  (decision 4).
- **What waits**: the inbox's `waiting`, the notice's brief and key, the
  plugin's SessionStart for a person's session (decision 9).
- **A carrier's wake**: `flume wake <address>`, as built (132).

#### 11.2 From crew

- **Its roles, declared once.** crew writes its roles file from its
  partitions, whole, beside it and renamed over it, at `crew main up`
  and beside each ask, as it writes `roles.rec`:
  - each address `<stream>/<role>` of a role that runs as its kind on
    this host, the stream the partition's: its kind, the partition's
    first blueprints repository where the runner keeps it on this host,
    the stream's name as its name, `<stream>-<role>` as its intent, its
    model in `agents`, and its `answers` from its brief;
  - each partition's operator standing behind its stream;
  - each address of a role whose partition's main level runs on another
    host, `carried`, with its `answers`;
  - no entry for a role that still stands, and no `answers` for the
    reviewer.
- **Its kinds as packages**, `crew/crew-<role>@<version>`, laid out under
  its release's kinds directory as record 0040 has them, each declaring
  the label `planning` (180). The host locks them by `flume kinds lock
  --host` (155).
- **Each partition's one stream.** The teams file's `flume` names the
  partition's stream once, and says for each role only whether it runs
  as its kind. A role moves on the user's word (decision 13).
- **The turn** (`plugin/flume/turn.md`): collect flume's list for the
  run's address after `crew role start`, and follow decision 10's
  protocol, asking the user a place's question in its partition's
  Plannotator Inbox as its address's identity, `crew:<stream>-<role>`,
  in its address's thread.
- **The follower**: at its minute's read, for each role on its host that
  runs as its kind, list that role's threads as its identity in that
  role's partition's Inbox, and write an ask of the role for each
  thread `unread`, keyed by the person's last message in it, reading
  nothing of the reply (decision 10, *Which Inbox*). Its Pending You
  notice, which runs `crew _answered` for crew's own cards, stays
  beside that read as it is. Once Plannotator offers a watch, the
  follower hangs its writing of the ask on it in place of the minute's
  read of the Inbox.
- **The teams file**: each partition's Inbox's data folder beside the
  partition, which swancloud's `lib/crew-teams.nix` writes once
  swancloud gives the partition's stream's agents that Inbox, and none
  until then.
- **`crew answer`** carries the key as `Answers:`, takes a place on the
  host that crew did not launch by its place's name, and is open to the
  operator beside the planner, the design agent and ops.
- **The briefs**: each role's part of decision 3's table and decision
  10's protocol, a question of another stream's system addressed to
  that stream's role; the operator's, the door behind every stall of
  its stream, which may put a stalled question to the user once in the
  Plannotator Inbox; briefs.11: a place's question is what a role asks
  the user in the Inbox, at most two open at a time as today, and its
  other questions and crew's commands' cards stay on Pending You.
- **The records and rules**: roles.7 holds again as written, since flume
  wakes a role. The change `roles-run-as-flume-kinds` Decisions 1, 3 and
  6, and its departure from roles.7, give way. Record 0049 part 1 holds,
  its role addresses declared by crew's own roles file rather than the
  host's `[roles]`, each the role of its partition's stream, and the
  runner carrying crew's events by `flume wake` where it named the card
  queue's own wake. The change's record of the user's word of
  2026-10-31, "crew reads nothing of Flume's but what the runner carries
  back", gains their word of 2026-11-05: "Yes: a role's run reads
  Flume's list for its address".

#### 11.3 From the runner

- **Role offers become wakes.** For each role whose record names its
  host, while crew's records hold an ask pending for the role, or a
  place on another host waits on it, the runner runs `flume wake
  <address>`. It launches no role's run, names no kinds file, and
  declares no run done.
- **A role's run's status** is carried into crew's records keyed by the
  address, read from flume's status by the place's role, as R4 has it.
- **The retry's key**: a request's `Answers:` is passed as `--answers
  <key>`, and flume's refusal of an answer already taken is the
  request's outcome, refused, in flume's words.
- **Its service.** On deneb it runs as the launchd service
  `swancloud.boomdrive`, which keeps it running. An answer of crew's
  reaches flume, and an ask the follower wrote for the user's answer
  wakes its role, at its next pass.

#### 11.4 An address on another host

crew's records carry a place's question across hosts: a place on one
host waiting on a role whose main level runs on another is read from
that host's runner's records (crew's Decision 5). Its address names a
stream of that other host, which the place's host does not declare.
crew's roles file on the place's host declares the address carried:
`carried = true`, with its `answers`, and no run's keys and no
`answerer`. flume launches nothing for it, takes launches naming it,
lists it for `--to` by its whole address, and keeps its ends pending for
the tool that carries them. The role's host's runner wakes the run
there, and the run answers by `crew answer`, which the place's host's
runner carries to `flume retry`. Status says the address is carried. On
deneb, crew's file carries wldn's roles, whose stream runs on
chuck-herdr-alpha.

#### 11.5 `flume serve`: no purpose left

`flume serve <address>` would have a session take an address's ends
while it is live. Once roles run as kinds, nothing needs it:

- **A role the user opens to talk** is a run: `crew role open` writes an
  ask, the role's run greets the user in its pane and keeps the
  conversation until they are done (crew's Decision 13). Its pane is the
  run's, reached by the address as any run's.
- **A person's own session** is reached as a session, and collects at
  its start (decision 9).
- **A standing role** is the one thing serving would reach, and standing
  roles are what each partition's moves retire. Until a role moves, its
  questions reach the door at their stall, and the door carries them to
  the standing role as crew does today.

So serving is a stop-gap for the time between moves, and is left out.

#### 11.6 One owner for each part

| part | owner | the others |
|---|---|---|
| A stream, which an address names | the host (143) | a source writes its addresses on the host's streams; flume checks each is declared |
| Which roles answer, and what each run is launched with | the source whose roles they are, crew for its partitions (193) | the host declares the source, locks its kinds and declares its streams (143, 155, 191); flume reads |
| A place's recipients, and the answerer behind its stream | flume resolves them (7, 193) | crew declares each stream's door; the runner passes crew's recipients unchanged (Boomdrive core.15) |
| Which role a question is for, of which stream | the asking agent, else the place's owner | flume lists the roles; the role passes what is not its own |
| Delivering an end, and keeping it pending | flume (43) | the runner's copy of status is a view for crew's board and its cross-host records, never a delivery; sluice's outcome for a place reaches flume at its settle (C.6) |
| Launching a role's run, and ending it at its delivery | flume (132) | crew's follower writes crew's events as asks, and the runner carries them to flume by `flume wake` |
| The bound | flume (130, 193) | a role says it is on a question by a hold |
| A session's start collecting | each tool for its own: flume's plugin for flume's ends, sluice's for sluice's | one design, sluice's (sluice 51) |
| Which questions are a person's, and asking the user | the role whose question it is, or the door at a stall | flume decides nothing (69, 72) |
| Keeping the user's answer for a role whose run ended | the Plannotator Inbox of the role's stream's partition, for the address's identity | swancloud's design gives a stream's agents their partition's Inbox (its Decision 7), deneb's one Inbox until then |
| crew's own cards, and carrying their answers | Pending You, and the follower's notice running `crew _answered` | until crew moves them to the Inbox (the user, 2026-11-05) |
| Carrying the user's answer to the role's next run | crew's follower, as an ask, and the runner's wake | Plannotator offers no watch (decision 10) |
| Taking an answer once | flume (42, 49) | `crew answer` and the runner carry the key |

### 12. The statements' amendments, for the user to approve

#### 12.1 flume's statement (agentplot/blueprints, `design/flume/requirements.md`, on main at 2287669)

Each amendment quotes main's words as they stand at 2287669.

**Section 3.**

- Replace "**recipient**: whoever an end of a place goes to: a caller's
  session, or a role's address (43, 132)." with: "**recipient**:
  whoever an end of a place goes to: a caller's session, or a role's
  address, the answerer behind its stream among them (43, 132, 193)."
- Add **address**: "of a role, `<stream>/<role>`: the role a source of
  the host declares for a stream (193), which a launch, a stop-short or
  a carrier names in place of a session; a stop-short names its own
  stream's role by the role alone (41, 132)."
- Add **answerer**: "of a stream, the role a source of the host declares
  to stand behind the places that run there: the recipient of a launch
  that names none and runs in no session, and a recipient of every stall
  of a place that is not its own (193)."
- Add **key**: "of an end, the name flume gives it, which no other end
  of the place has, by which a recipient and an answer name it (42,
  43)."
- In **on hold**, "of a stage or a submission, that it waits on its
  caller's word before it starts.", after "before it starts", add: "; of
  the move a place waits on, that a caller said in its words that it is
  making it (130)".

**Clause 4.** Append: "So is a launch that names no recipient and runs
in no session where no answerer stands behind its stream, saying how to
name one (7, 193), or where two roles stand behind it, naming both
(193); and one that names a role's address no source of the host
declares, or two declare, where no answerer stands behind its stream,
naming the address (132)."

**Clause 7.** Replace "A recipient the launch names none of is the
session that launched it, as the terminal it runs in names it, or none
where no terminal names one (190)." with: "A launch that names no
recipient has the session that launched it, as the terminal it runs in
names it (190), else the answerer behind the place's stream (193). A
recipient a launch names is a session's name, or a role's address
written whole (132)."

**Clause 8.** Replace "Its recipients are fixed at its launch." with:
"Its recipients are fixed at its launch; the answerer behind its
stream, as the sources declare it when a stall is read, is delivered
its stalls besides (130, 193)."

**Clause 28.** After "filled in for the place and the stage", add: ",
with each role's address its words may be for, its own stream's by the
role alone and every other stream's whole, and the words its source
gives for what that role answers (41, 193)".

**Clause 41.** After "(131).", add: "It may name the role its words are
for: the address of a role a source of the host declares with words of
what it answers, of any stream, or the role alone for a role of the
place's own stream (132, 193). The stop-short's end then goes to that
address and not to the place's recipients. A name that is none of those
is refused, naming them."

**Clause 42.** After "and the fresh agent's prompt carries the question
with the answer.", add: "An answer may name the stop-short's key, and
is then taken only while the place waits on that stop-short, so a
question is answered once: a second answer, or one that comes after the
place moved on, is refused naming the answer taken, who gave it and
when (49)."

**Clause 43.**

- After "it counts delivered on the terminal's answer, with no reading
  back.", add: "A notice names the place, the first line of its brief,
  the end and its key, and where to read the end whole."
- Replace "by `flume inbox`, which delivers each and marks it delivered;
  status and a read mark nothing." with: "by `flume inbox`, which
  delivers each and marks it delivered, and gives with them every place
  that waits on its caller and that it is a recipient of, was named by
  the stop-short the place waits on, or was given a stall of, delivered
  or not, with the move and since when, the words, key and addressee of
  a stop-short it waits on and a hold's words, so a caller that
  restarted sees each question still its own; that list, status and a
  read mark nothing."

**Clause 49.** Append: "A retry that names a stop-short's key is
refused, naming the place, the answer the stop-short already took, who
gave it and when, or the move the place waits on now, unless the place
waits on that stop-short (42)."

**Clause 56.**

- After "the words of a stop-short, and whether it needs an answer or
  found work, or of a failed submission;", add: "the role's address a
  stop-short named; each end's key, and for a stop-short answered, its
  answer with who gave it and when; the place's recipients, each with
  where it came from;".
- After "a hold's words", add: ", a hold on the waiting move with its
  bound or that it lapsed,".
- After "for each stream," and before "its points bound", add: "its
  answerer with the source that declares it, or that it has none
  (193),".
- Replace "and each source the host declares, the kinds it holds by
  identity, or why it cannot be read (191)." with: "each source the host
  declares, the kinds it holds by identity and the roles it declares, or
  why they cannot be read (191, 193); and for each role's address an end
  stands pending for, its run, or that it is carried or wakes nothing
  and why (132)."

**Clause 130.**

- After "a retry naming the stage or the submission ends the hold (47,
  49).", add: "A caller may also hold the move the place waits on it
  for, with words saying who makes it and how, such as a role that took
  the question or an answer asked of a person, and a bound
  after which the hold lapses; a later hold on that move replaces it,
  and a retry naming the stage or the submission ends it as the move
  made."
- Replace "and each recipient of the place is delivered once that it is
  stalled and which move is theirs (43)." with: "and each recipient of
  the place, the role a stop-short it waits on named (41), and the
  answerer behind the place's own stream where it is none of them, is
  delivered once that it is stalled, which move is theirs, and the
  words, key and addressee of the stop-short it waits on (43, 193). A
  hold on the waiting move whose bound passed is no hold from then, and
  the place stalls again."
- Replace "and flume watches none and ends no hold itself (70, 72)."
  with: "and flume watches none and ends no hold itself, but lapses a
  hold on the waiting move once its bound passes (70, 72)."

**Clause 132.**

- Replace "A recipient may be named by a role's address,
  `<partition>/<role>`, rather than by a session. A run of the role is a
  place of the kind the host's configuration names for the address
  (C.7), named by the role, so at most one run of an address is live at
  once (3)" with: "A recipient may be named by a role's address,
  `<stream>/<role>`, the role a source of the host declares for a stream
  the host declares (143, 193), rather than by a session; an address of
  any stream may be named from a place of any other. A run of the role
  is a place of the kind its source names for the address, in the
  address's stream, named as the source names it for the address, else
  by its stream and its role, so at most one run of an address is live
  at once (3) and two streams' runs of one role are two places".
- After "and its agent takes the address as its session's name.", add:
  "Its repository, base, priority, intent, points, brief and settings
  are those its source declares for the address, as a launch's are (7,
  32). A run is over once its last stage delivers: flume declares it
  done at the settle that reads the delivery, with no caller's word, and
  tidies it (114), so an end that comes later wakes another run. A
  run's own stall goes to the answerer behind its stream and never to
  its own address (130, 193)."
- In "flume launches the address's kind at the priority the
  configuration gives it", replace "the configuration gives it" with
  "its source declares".
- Replace "An address the configuration names no kind for is woken by
  nothing, and status says the ends pending for it." with: "An address
  no source declares, or two declare, is woken by nothing, and so is one
  whose stream the host does not declare. Where its source declares it
  carried, its run is on another host, its stream that host's, and its
  ends stay pending for the tool that carries them there. Otherwise a
  launch naming it is taken only where an answerer stands behind the
  place's stream (4, 193). Status says the ends pending for each
  address, and whether it is woken, carried or wakes nothing."

**Clause 143.** Replace "the host's bound and the runs it wakes for
roles (132) are the host's alone, and its terminal, view and window
program stand for each stream that names none (186, 190)." with: "the
host's bound is the host's alone, and its terminal, view and window
program stand for each stream that names none (186, 190); the runs it
wakes for roles are its sources', each in the stream its address names,
with that stream's room, pool and settings (132, 193)."

**Clause 191.**

- Replace "Each source has a declaration of its own, named by the
  source, which names where the source's packages lie." with: "Each
  source has a declaration of its own, named by the source, which names
  where the source's packages lie and may name where the source
  declares the roles it answers (193)."
- Replace "each launch, each wake of a role, each lock its caller asks
  for and each status" with: "each launch, each wake of a role, each
  start, each stall, each lock its caller asks for and each status".
- Replace "It writes nothing in them or where a source's packages lie."
  with: "It writes nothing in them or where a source's packages or
  roles lie."

**Clause 192.** Replace "given where a source's packages lie, the
identity of every package it holds; or that it could not be read" with:
"given where a source's packages lie, the identity of every package it
holds, and given where it declares its roles, each address it declares
with the run it names or that it is carried; or that either could not
be read".

**New clause 193**, in A.12 after 132: "A source the host declares
(191) may declare the roles it answers, each at its address,
`<stream>/<role>`, the role of a stream the host declares (143): with
the kind of its own its run is launched as and what the run is launched
with (132), or that the role's run is on another host, its stream that
host's; and the words of what the role answers, which a stage's prompt
gives (28). The host declares the source and its streams and locks its
kinds, and nothing of its roles: a role is declared once, by the source
whose kinds run it, at an address for each stream it answers in. A role
a source declares may stand behind its address's stream: it is that
stream's answerer. A launch that names no recipient and runs in no
session has the answerer behind its stream as its recipient (7), and
every stall of a place is delivered to that answerer besides the
place's recipients and the role its stop-short named (130), so a
question that no one collects or answers within the place's stall bound
reaches a run woken to answer it. An address two sources declare, or a
stream two roles stand behind, is declared by neither, and a launch
that would take it is refused naming both (4); a declaration that
cannot be read, and an address whose stream the host does not declare
where its run is not on another host, declare nothing, and status names
each and why (56). flume decides nothing in this: which roles a source
declares, which stands behind a stream, what its kind does, which
questions its run answers itself, passes to another role of any stream
or carries to a person, and how, are the source's and the run's (69,
72). A stream no role stands behind has no answerer: a launch there
names its recipient or runs in a session and names no address that
wakes nothing (4), and its stalls go to the place's recipients and the
role a stop-short named."

**C.4**, after "Each hook calls the binary that started the agent by its
absolute path, written into the plugin with its version, and never fails
or holds up the agent's start or turn.", add: "In a session flume did not
start, the plugin's SessionStart hook runs `flume inbox` as the session
the terminal names, and gives the agent what it answers as the session's
context, or nothing where nothing is pending or waiting, or no session
is named; it never holds up the start (43)."

**C.7.**

- Replace the bullet "The roles a host wakes are a table of the same
  configuration: `[roles."<partition>/<role>"]` with `repository`,
  `kind`, `priority`, `base`, the repository's default branch where it
  names none, and an optional `brief`, `stream` and `points`, the place
  a woken run is launched as (132), stated by whoever administers the
  host. An address the table names nothing for is woken by nothing."
  with: "The roles a host wakes are declared by its sources, never by a
  table of its configuration (193). A source's file
  may give `roles`, an absolute path or one under `~`: the file the
  source's own tool writes whole, beside it and renamed over it, which
  flume reads as it stands and never writes. It holds
  `[roles."<stream>/<role>"]` with `kind`, a name of the source's own
  kinds, its version the locks' (154, 155); `repository`, by its
  location on the host as a launch names it; `priority`; `base`, the
  repository's default branch where it names none; and an optional
  `brief`, `points`, `name`, the run's place's name, `<stream>-<role>`
  where it gives none; `intent`; `answers`, the words of what the role
  answers that a stage's prompt gives (28); `answerer = true`, where
  the role stands behind its address's stream; and under
  `[roles."<stream>/<role>".agents]`, the run's settings as a launch's
  (7); or `carried = true`, with an optional `answers` and nothing else,
  for an address whose run is on another host. The run is launched in
  its address's stream. An entry that names a kind not its source's, an
  `env`, a pool, an account or a `stream`, whose address names a stream
  the host does not declare and that is not carried, or that mixes
  `carried` with a run's keys or `answerer`, declares nothing for its
  address, and status names it. A configuration that gives a `[roles]`
  table is unreadable, naming it and that a source declares a role."
- In the sources bullet, replace "Each gives `kinds_dir`, an absolute
  path or one under `~`: the directory holding the source's packages,
  each at `<name>@<version>/` with its `kind.toml` at its root (157,
  159, 191)." with: "Each gives `kinds_dir`, an absolute path or one
  under `~`: the directory holding the source's packages, each at
  `<name>@<version>/` with its `kind.toml` at its root (157, 159, 191);
  and may give `roles`, the file in which the source declares its roles
  (193)."

**Section 9**, the command line's list: after "`needs` for an agent;",
add: "`hold`, `wait`, `inbox` and `wake` for its callers;".

**The callers table, crew's row.**

- Replace "it names each place's recipient by a role's address, so a
  role with no live run is woken to collect an end (132)" with: "it is a
  source of the host's that declares its roles once, each of a
  partition's roles at its address in the partition's stream, running
  crew's one kind for it, and names each place's recipient by the
  address of the role the work is for, so a role with no live run is
  woken to collect an end, and each partition's operator stands behind
  its stream (132, 193)".
- Replace "its word to a place is a retry answering a stop-short that
  asked" with: "its word to a place is a retry answering a stop-short
  that asked, naming its key (42)".

**Clause 193 and the scenarios.** Main at 2287669 holds the clauses 186
to 192 and 195 to 197, and the designs in flight hold 198 and 199, so
the new clause is 193. Main holds the scenarios S90 to S92, S95 to S97
and S100 to S103, and the designs in flight hold S104 to S108, so these
four are S93, S94, S98 and S99.

- **S93.** A script with no terminal session launches a change in the
  stream `east`, naming no recipient, on a host whose source `acme`
  declares runs for `east/operator`, standing behind `east`,
  `east/planner` and `east/ops`, each with its words. The place's
  recipient is `east/operator`. Its construct needs a reading of a
  deployed queue's settings, and stops short naming `ops`, which its
  prompt listed among its own stream's roles. The settle wakes
  `east/ops`, whose run collects the question with its key and the
  place's brief, holds the waiting move, reads the settings and retries
  construct naming the key. The fresh agent's prompt carries the
  question with the answer. A retry naming the same key from a person's
  pane a minute later is refused, naming ops's answer and when.
  `east/operator` was woken for none of it (7, 28, 41, 42, 43, 49, 130,
  132, 193).
- **S94.** The operator launches a change in `east` from a herdr pane
  on that host. Its construct stops short needing an answer, naming no
  role. The notice is delivered into the pane, naming the place, its
  brief's first line and the end's key. The pane's agent is restarted
  before it acts. At its start, flume's plugin gives the new agent the
  place waiting on it, with the question and the key. The agent does
  nothing with it. Five minutes after the stop-short the place is
  stalled. The stall, with the question and its key, is delivered to
  the pane and to `east/operator`, whose run is woken (43, 130, 193,
  C.4).
- **S98.** A place in `east`, whose recipient is `east/planner`, stops
  short naming `west/design`, a role of another stream, which its
  prompt listed by its address. The run of `west/design`, in `west`,
  takes it, holds the waiting move "asked the user, msg_1" for a day,
  and asks the user once, keyed by the place and the end. A second run
  of `west/design`, woken by another end, reads the same question, sees
  the hold, and asks the user nothing. The day passes with no answer.
  The hold lapses, and the place's stall is delivered to its owner
  `east/planner`, to `west/design` and to `east/operator`, the door of
  the place's own stream, naming the lapsed hold's words. The
  operator's run holds again naming the question already asked, and
  asks nothing (41, 130, 132, 193).
- **S99.** A host declares the streams `east` and `west` and one
  source, `acme`, holding the package `acme/turn@1`, which the host's
  lock records, and naming the file where acme declares its roles.
  acme's file declares `east/planner` and `west/planner`, each running
  `turn` in one repository, named `east` and `west`, and carries
  `north/planner`, whose stream the host does not declare. The host's
  configuration names no role. Ends come at once for `east/planner` and
  `west/planner`. Two places are launched, `turn--east` in the stream
  `east` and `turn--west` in `west`, and each run collects its own
  address's ends. Each run's turn delivers, and flume declares it done
  and tidies it with no caller's word. The next end for `east/planner`
  launches a new run. A launch naming `north/planner` is taken, and its
  end stays pending, status naming the address carried (3, 114, 132,
  143, 155, 191, 193).

#### 12.2 sluice's statement (`design/sluice/requirements.md`)

None. A place's question never travels through sluice. A submission's
outcome for a place reaches flume at its settle (C.6), and flume carries
it on as an end, under this design's bound and answerer. sluice's own
delivery to a person's session is already the design this change gives
flume for a session's start (51, 135). Where sluice's carrying differs
from flume's, a push never made again and a push into an asking pane,
is Found beside.

#### 12.3 crew (afterthought/crew: records and rules, crew has no statement)

- **roles.6**: after "a session's start collecting what is pending for
  it", add: "and reading what still waits on it, by each tool's inbox".
- **roles.7** stands as written: "a role with no live run is woken by
  Flume, never by a process of crew's". Its details gain: "Not ruled
  out: crew declaring its roles to Flume in its own roles file, which
  the host names as crew's source, each at its address in its
  partition's stream; the runner carrying crew's events to flume by
  `flume wake <address>`; a place's agent addressing its question to the
  role whose it is, `flume needs --to <role>` in its own stream or
  `--to <stream>/<role>` in another."
- **The change `roles-run-as-flume-kinds`**, a record amending it, as
  the design agent writes one:
  - Decision 1: the kinds reach the host as packages of crew's source,
    which the host locks, and crew declares each role in its roles file;
    the runner's launch gives way to flume's.
  - Decision 3: a partition's roles are at the addresses of its one
    stream, each run in that stream under the `planning` label, and the
    teams file's `flume` names the stream once and says for each role
    only whether it runs as its kind.
  - Decision 6: the role record keeps what crew's board and cross-host
    records read, and the roles file is written beside it from the same
    facts; the runner wakes a role on its host; R1 and R3 leave, R2
    becomes `flume wake`, R4 stays.
  - Its departure from roles.7 is withdrawn.
- **The roles file**: written by crew whole, from its partitions, as
  11.2 has it, where its source's declaration names it.
- **The turn and the briefs**:
  - `plugin/flume/turn.md`: after `crew role start`, `flume inbox --json`
    for the run's address; each question held, answered, passed or put
    to the user as decision 10 has it.
  - Each brief its part of decision 3's table and decision 10.
  - The operator's brief: the door behind every stall of its stream,
    passing a stalled question once to the role whose it is, of any
    stream, or putting it to the user once in the Plannotator Inbox, on
    the user's word of 2026-11-05, "I think you're referring to an
    operator and that's fine"; the brief's word that the operator asks
    the user nothing gives way.
  - briefs.11: a place's question is what a role asks the user in the
    Plannotator Inbox, in its stream's partition's Inbox, as its
    address's identity `crew:<stream>-<role>`, in its address's thread,
    under the `idempotency_key` `flume/<host>/<digest of the
    location>/<key>`, at most two open at a time as today. Its other
    questions, and the cards crew's commands raise, stay on Pending You,
    on the user's ruling of 2026-11-05: "crew's own cards stay on
    Pending You until crew moves them to the Inbox".
- **The follower** (`crew _follow`): at its minute's read, the threads
  of each role on its host that runs as its kind, read as that role's
  identity in that role's partition's Inbox, each thread `unread` an ask
  of the role, as decision 10 has it. Its Pending You notice, which runs
  `crew _answered` for crew's own cards, stays as it is.
- **The teams file**: each partition's Inbox's data folder, written by
  swancloud once it gives the partition's stream's agents that Inbox,
  and read by the follower; none until then, deneb's one Inbox.
- **`crew answer`** carries the key, `Answers:`, takes any place on the
  host by its place's name, and is open to the operator
  (`REQUESTERS`).
- **Record 0049**: an amendment naming flume record 0039. A role is
  woken by flume from crew's own declaration of it, at its address in
  its partition's stream. The follower writes the user's Inbox answers
  to a place's question as asks, and a card's answer on Pending You
  reaches its role as the `card` ask `crew _answered` writes; the runner
  carries each ask to flume by `flume wake`, where the record had
  Pending You's wake mod send it.
- **The change `roles-run-as-flume-kinds`'s word of 2026-10-31**, "crew
  reads nothing of Flume's but what the runner carries back", gains the
  user's word of 2026-11-05 for a role's run: "Yes: a role's run reads
  Flume's list for its address".

#### 12.4 Boomdrive (agentplot/boomdrive: records and specs, Boomdrive has no statement in blueprints)

- **Role runs**, `crew-control-plane` and `crates/boomdrive-crew/src/roles.rs`:
  a role wanted is woken by `flume wake <address>` on its record's host;
  the runner launches no role's run, passes no `--kinds` for one, and
  declares none done; a role's run is read from flume's status by its
  role and carried into crew's records by its address.
- **Retries**, `work-into-flume`: a request's `Answers:` is passed as
  `--answers <key>`, and flume's refusal of an answer already taken is
  recorded as the request's outcome, refused, in flume's words.
- **The status copy**: crew reads it for its board and for a place on
  another host, and no question on the same host depends on it. A record
  says so.

#### 12.5 Plannotator (not of the family: what the design asks of it)

Plannotator has no statement in agentplot/blueprints, and the design
needs no change of it to run. Two asks, put to whoever keeps it on the
host, would close what decision 10 works around:

- **A watch**: a command the Inbox runs for each reply to a session no
  host wakes, `plannotator inbox watch [--session <id>] -- <command>`,
  so crew's follower carries an answer when it is given rather than at
  its next minute.
- **An identity for an address**: a name a caller gives beside its own
  session, which a reply is addressed to and every later caller giving
  it reads as its own, so a role's run no longer drops its session's
  `PLANNOTATOR_SESSION_TAG` to write as its address.

### 13. Moving each partition's roles onto their kinds, in order

Each move is the user's word, one role at a time (crew's roles.2 and
Decision 3): the teams file says the role runs as its kind, crew ends
its standing agent at its next stop, and crew's roles file declares its
run at its stream's address. Nothing of the host's changes at a move.
The order is the user's, of 2026-11-05: "That order; planner and design
agent wait for an effort setting".

1. **Each stream's operator, with flume's release.** agentplot's,
   madswan's and swancloud's operators move, each standing behind its
   stream. The doors must run before the release reaches deneb, since a
   launch from a shell there takes its stream's door, and is refused
   where none stands. It runs at its account's effort until flume gives
   one.
2. **Each stream's ops.** It answers questions about live systems, and
   its fixes of main name it. It runs at its account's effort until
   flume gives one.
3. **Each stream's reviewer.** It answers no question.
4. **Each stream's planner**, then 5. **its design agent**, once flume
   gives an agent's effort as a setting of its layers (197), which
   flume's change `an-agents-effort-is-a-setting-of-flumes-layers`
   builds, and
   crew's roles file gives each its extra-high effort in `agents`.
   Until they move, an agent cannot address them by `--to`, the
   runner's places naming the planner keep working, and their questions
   reach the door at the stall, which carries them to the standing
   planner or design agent as today.

Each step after the first takes agentplot first, since the
orchestrator's work runs in its stream, then madswan and swancloud.
wldn's main level runs on chuck-herdr-alpha, which still runs crew's
team path, and moves when the user moves that host.

### 14. Order across the repositories

1. **The user's read** of this design, and their answer to its open
   question.
2. **agentplot/blueprints.** flume's statement amended on main as 12.1
   words it (core.19).
3. **This repository.** The change `a-host-declares-where-its-kinds-come-from`
   landed, since a source's declaration is read by it; then this
   change's build, released.
4. **afterthought/crew.** Its kinds as packages declaring `planning`,
   its roles file at its streams' addresses, the turn, the follower's
   read of the Inbox, the briefs, `crew answer` and the records of 12.3.
5. **agentplot/boomdrive.** The wakes and the retry's key of 12.4.
6. **The host**, in one deploy with decision 13's first step and
   flume's release: crew's source declared, crew's kinds locked, and its
   streams `agentplot`, `madswan` and `swancloud` with `planning`'s
   reserve in each.
7. **The later moves**, one role at a time, as decision 13 orders them,
   each crew's alone; the planner's and the design agent's once flume's
   change `an-agents-effort-is-a-setting-of-flumes-layers` is released.

Steps 4 and 5 work against the released flume and need nothing of each
other. A retry with no key is taken as today, and the runner's launches
keep working throughout. swancloud's design
`the-plannotator-inbox-runs-on-deneb-for-the-users-devices` needs none of
these steps, nor they it: a role's run writes to the Inbox its
`PLANNOTATOR_DATA_DIR` names, deneb's one Inbox until swancloud gives
its stream's agents their partition's (decision 10, *Which Inbox*).

## Alternatives considered

1. **Each caller collects for itself** (record 0039's option 1). This is
   today. A shell's places are seen only by a poller, and a poller that
   is down loses the night. Record 0009 already refused polling for the
   load it puts on a host.
2. **Refuse a launch with no recipient, nothing more** (option 2). It
   closes the first gap only.
3. **This design** (option 3, as the user's rulings of 2026-11-05 shape
   it).
4. **flume carries the user's question to a person's queue itself**
   (option 4). It breaks 72 and core.1, and flume would judge which
   questions are a person's, which 69 leaves to callers.
5. **A standing watcher carries questions** (option 5). This is the
   runner's copy and the orchestrator's script: a second reader of
   flume's ends, lagging by its pass and losing everything while it is
   down.
6. **One answerer for a host**, a planner, behind every place. The user
   refused it: "Theoretically an agent might need something from ops
   separately from the planner or the designer."
7. **A session serving an address beside the roles as kinds.** A bridge
   for standing roles, which the moves retire (decision 11.5).
8. **The runner keeps launching role runs**, flume's wake unused. Two
   launchers. The user ruled flume the launcher.
9. **Roles and answerers in each host's configuration**, as 132 and C.7
   have roles on main. Every host repeats crew's partitions. The user
   ruled roles crew's to declare (decision 2).
10. **An address of a partition of its own, beside the stream**, as 132
    has it on main. A second division of the host's work that flume
    cannot check, which the user questioned: "We're already going to use
    streams to partition where agents run" (decision 1).

## Where the statements and the code disagree

- **Clause 43 says a caller that starts collects what is pending for
  it.** flume's plugin does nothing at the start of a session flume did
  not start (`hook.rs`, `hook`). sluice's plugin collects (51, 135).
  Decision 9 brings flume's code to its clause.
- **Clause 132 has a role's run launched by flume and never ended.** Its
  kind lands nothing, and nothing gives the `flume done` that 114 waits
  for. crew's runner did it. Decision 2 makes it flume's.
- **core.35 rules out a partition in a command's help, output or
  refusal.** `flume wake`'s help and its refusal, and the configuration
  reader's refusal of a key, say `<partition>/<role>` (`commands/mod.rs`,
  `wake.rs`, `config.rs`), and the site says `<group>/<role>`. Decision
  1 has each say `<stream>/<role>`.
- **crew's roles.7 says Flume wakes a role.** crew's change
  `roles-run-as-flume-kinds` has the runner launch role runs instead.
  Decision 2 and 12.3 bring them back together, on the user's ruling.
- **crew's record 0052, amended, says the runner names no kinds file.**
  Its R1 passes `--kinds` naming crew's file. Decision 2 has the host
  declare crew's source and the runner launch nothing of a role's.
- **crew's planner brief says a place's question reaches the planner.**
  Nothing tells a standing planner. Decisions 2 and 13 give the planner
  its run at its address.
- **crew's change records the user's word of 2026-10-31**, "crew reads
  nothing of Flume's but what the runner carries back". A role's run
  reads flume's list for its address at its start, on the user's word
  of 2026-11-05, "Yes: a role's run reads Flume's list for its address",
  and 12.3 adds it to the change's record.
- **crew's operator brief has the operator ask the user nothing.** The
  door puts a stalled question to the user once in the Plannotator
  Inbox, on the user's word of 2026-11-05 (decision 10), and 12.3 amends
  the brief.
- **flume's CLAUDE.md says each tool's statement is in
  agentplot/blueprints.** Boomdrive has none there. Its records and
  specs stand for one, so 12.4 is written against them.
- **Section 9's command line** names no `hold`, `wait`, `inbox` or
  `wake`, though each is built and specified. 12.1 adds them.

## Found beside

- **The open-session record's number.** The record on the unlanded
  change `an-answer-reaches-the-open-session-within-an-hour`, accepted
  on 2026-11-01, is numbered 0028, which main's statechart record took.
  It takes the next free number when it lands. This design's key is
  written to cover its answer (decision 8).
- **An answer's way back runs through crew's follower and the
  runner.** A question reaches its role through flume alone. The answer
  reaches flume by `crew answer` at the runner's next pass, and the
  user's answer in the Inbox wakes its role at the follower's minute and
  the runner's next pass. On deneb the runner is the launchd service
  `swancloud.boomdrive`, which keeps it running.
- **sluice pushes once, into whatever the pane shows.** A push not made
  is never made again, and a push is sent into a pane that asks a
  question (`step.rs`, `perform_outbox`; `herdr.rs`). flume tries again
  at each settle while pending and never sends into a question
  (core.11). A reconciliation of the push, sluice taking flume's rule,
  is sluice's to weigh.
- **A sluice outcome submitted from a plain shell** stays pending for no
  session. Outcomes are not questions, and sluice's failed submissions
  blame and send back on their own, so no answerer is proposed for
  them.
- **The place `research--sluice-signs-its-landings`** has been never
  begun for 70 hours, with a stop-short pending for its pane. Its words
  point to a card the user was asked on, `req_4d00977773964d858c61`.

## Risks / Trade-offs

- **A wrong pick.** An agent that addresses the wrong role, or the right
  role of the wrong stream, costs a pass. The list gives each role's
  words, and a role passes what is not its own.
- **A longer prompt on a host of many streams.** The list gives every
  stream's answering roles, so it grows with the host's streams. It
  comes after every part a stage's repetitions share (core.31), so it
  costs no cached prefix, and a role a source declares with no `answers`
  is not listed.
- **A stream renamed.** An entry whose address names a stream the host
  no longer declares declares nothing, and status names it and the
  stream, until its source writes the new name. A place launched before
  keeps its recipients (8), and its ends to the old address stay
  pending, status naming why.
- **Asked twice by a pane and the door.** A person's session that asks
  the user in its own conversation without holding the move lets the
  stall reach the door, which may ask the user too. The notice, the
  plugin's context at a start and the site all say to hold before
  asking.
- **Runs woken by stalls.** Each stall the door gets may wake its run,
  one agent session. One run collects a burst (132), and a role's hold
  at its take keeps the door out of the questions roles are working on.
- **The door is its stream's last word.** Where the door's own run
  cannot run, its run's stall goes to no address, and its failure is in
  status under the host's addresses. Nothing of flume's tells a person
  (72).
- **crew's roles file is on every place's path.** A file crew wrote
  wrong, or has not yet written, leaves its addresses waking nothing and
  its streams with no door, so a launch from a shell into them is
  refused, naming the file's fault. Every other source and stream runs
  on, and status names the fault.
- **A stream no role stands behind.** A launch from a shell into it,
  such as the host's `default` stream, is refused. The refusal names the
  ways on: `--recipient`, `$FLUME_RECIPIENT`, a session, or a stream a
  role stands behind. A host gives its repositories their partitions'
  streams, as deneb's `[repos]` does.
- **Refused launches.** A script that launches with no recipient into a
  stream no role stands behind is refused after this change. Every place
  a refusal now stops is one whose questions reached no one before.
- **Effort.** The planner and the design agent stand until flume's
  change `an-agents-effort-is-a-setting-of-flumes-layers` is released,
  and their questions reach the door at the stall until then. The
  operator, ops and the reviewer run at their account's effort until it
  is (decision 13).
- **A session's start costs a flume call.** Every session the user
  starts runs `flume inbox` under a 20 seconds' bound, and prints
  nothing when nothing waits.
- **A hold's bound is a timer read at settles.** Under a terminal that
  keeps no watch, a hold lapses only at the next acting command (119).
  Under herdr, the watch reads it within its bound (122).
- **An answer's way back runs through the runner.** `crew answer` waits
  on the runner's next pass, and the user's answer in the Inbox on the
  follower's minute besides, since Plannotator offers no watch.
- **An Inbox identity Plannotator does not promise.** A role's run
  writes as its address by dropping its session's
  `PLANNOTATOR_SESSION_TAG`, which the pinned Plannotator takes and a
  later one may refuse. A refused call leaves the question held and
  unasked, and the hold's bound brings it to the door. crew's turn is
  checked against the Plannotator the host pins.

## Migration Plan

- **Places launched before the release** keep their recipients (8),
  none among them. Their stalls take their stream's answerer from the
  release on. A stall recorded before it is read as the answerer's too
  at the first settle after the build (decision 6), so the questions
  waiting when the release lands reach the door once.
- **The seven ends pending for `madswan/planner`** are delivered once
  the planner's run takes the address. Its letters are the same, and
  the stream `madswan` is declared on deneb. None is a question.
- **A run standing under the old name** `<kind>--<role>` is found by its
  role field, and its next run takes its entry's name.
- **A host's `[roles]` table** leaves its configuration. deneb has none.
  A host with one has it refused naming the table, and moves its roles
  into a source of its own, at addresses of its streams.
- **deneb**, with decision 13's first step:
  - its configuration, the host's operator's to write: `sources/crew.toml`
    naming crew's release's kinds directory and crew's roles file; crew's
    kinds locked on the host by `flume kinds lock --host`; the streams
    `agentplot`, `madswan` and `swancloud`, each opening in the herdr
    session of its own name and stating `planning`'s reserve, in place of
    `madswan-planning` and `swancloud-planning`; and `[repos]` giving the
    orchestrator's repositories the `agentplot` stream. No `[roles]` and
    no answerer;
  - crew's, from its teams file: its roles file declaring each
    partition's roles at its stream's addresses, each operator standing
    behind its stream, and wldn's roles carried.
- **The orchestrator** stops its status script. Its launches name no
  recipient and take the `agentplot` door.
- **A role's Inbox**, once swancloud gives its stream's agents their
  partition's, is that partition's, and a question its runs asked before
  waits in deneb's one Inbox. crew's follower reads each role's identity
  in deneb's one Inbox too until no thread of it there is open, so such
  a question is answered where it was asked, and asked nowhere again.
- **Rollback** is the earlier release. The fields this change adds, an
  answer on an end, a hold on the waiting move, a stop-short's
  addressee, are fields an earlier flume ignores (core.14). An earlier
  flume reads a source's `roles` as a key it does not know, a fault of
  that source's alone (191), so crew's roles go quiet with the rollback,
  its questions reach no door, and every other launch runs on.

## Open Questions

For the user:

1. **Plannotator's two gaps** (decision 10, 12.5). The Inbox has no
   watch, so crew's follower reads each role's identity at its minute,
   and no identity for an address, so a role's run writes as its
   address by dropping its session's `PLANNOTATOR_SESSION_TAG`. Do the
   moves build on Plannotator 0.28.48 as it is, or does the operator's
   move wait for Plannotator to offer a watch and an identity for an
   address?

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
   own partitions: every address with what its run is launched with,
   or the standing agent that holds it until the user moves the role.
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
7. **A place's question that is the user's is one Pending You card,
   written by code.** The role whose question it is, or the door at a
   stall, says the question is the user's by a command of crew's, and
   crew's code raises the card from flume's facts, as it raises a
   batch's, keyed by the place and the end so it is raised once. The
   user answers on the card. Pending You's notice runs crew's code,
   which carries the user's words to the place as its retry, naming the
   key. Where the question needs talk, the card links to a thread in
   the Inbox of the stream's partition, held by the role as its
   address, whose replies reach the role by crew's follower whichever
   session holds it, or by an elaboration's named agent, there while
   the user talks. No construction agent writes a card or an Inbox
   message. crew's cards stay on Pending You for good, crew's rail of
   decisions, raised by its code, each linking a thread in the
   partition's Inbox when it needs elaboration (decision 10).

What remains of the host is crew declared as a source, the lock of
crew's kinds, and its streams with their room. Each is the host's own:
which tools run under its accounts, the content it trusts, and how much
of its machine each stream gets (decision 2).

`flume serve`, a session serving an address, is left out. A role not
yet moved onto its kind is reached at its address as the standing agent
its source declares, and each move retires one (decisions 2 and 11.5).

sluice's statement needs no amendment. crew's roles file, rules,
briefs, `crew answer` and the question's card, and Boomdrive's role
offers and retry, change with this in their own repositories (decision
12). Moving each
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
- **Asking the user.** crew's own six cards, the proposal's, the
  batch's, the landing's, the deploy's, the step's and the record's,
  are built by crew's code from the facts of their events (record 0031,
  briefs.13; `plugin/lib/cards.py`) and posted on Pending You, the
  hosted card queue, in a role's name, under a key of crew's, so a card
  posted again under its key is found and not raised twice
  (`plan.py`, `raise_batch`, `row_card_change`; `cards.py`,
  `post_key`, `card_under`). No agent writes them. A role's own
  question goes through Pending You's own tools in its session, at most
  two at a time, and crew's card hook records it (briefs.11; records
  0005, 0031). Pending You's notice, `pendingyou watch -- crew
  _answered`, which crew's follower runs, runs crew's code at once for
  each change on a card: for a card in the name of a role that runs as
  its kind, it writes a `card` ask of the role, keyed by the card's
  version, and reads nothing else of the card (`card_answered`,
  `card_ask`).
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

**The user's queues: Pending You and the Plannotator Inbox.** The user,
2026-11-06: "We are trying to use Pending You and Inbox together, not
as alternatives ... I almost think that everything will be a card in
Pending You and then, when appropriate, there would be a thread in the
inbox." And, the same day, when the orchestrator took that for its own
work alone: "that's the instruction for you but it's also the instruction for the design of crew". So crew's cards stay on Pending You for good,
crew's rail of decisions, raised by its code, a card linking a thread
in the partition's Inbox when it needs elaboration.

**Pending You** (0.39.0 as the host gives it) holds cards, each asked in
an assistant's name, its `sessionLabel`, under an idempotency key of
the asker's, and answered by the user on the card. Its notice,
`pendingyou watch -- <command>`, runs the command for each change on a
card of the host's sign-in, naming the card, its version and the event
(`PENDINGYOU_REQUEST_ID`, `PENDINGYOU_VERSION`, `PENDINGYOU_EVENT`),
so a card's answer reaches code at once, whichever session asked it.

**The Plannotator Inbox** (0.28.48 as the host pins it, read in
`plannotator inbox --help` and the source its binary bundles):

- **Where it is.** One local Inbox for each data folder, the one
  `PLANNOTATOR_DATA_DIR` in the calling process's environment names,
  else `~/.plannotator`, answering agents on its machine alone. A
  message is filed under a project, the folder its `project_path`
  names, else the sending session's.
- **Which Inbox on deneb.** One runs today, `~/.plannotator`, and every
  agent writes to it. swancloud's design
  `the-plannotator-inbox-runs-on-deneb-for-the-users-devices`, being
  redone on the user's answers of 2026-11-06, owns where each
  partition's Inbox runs: one Inbox per partition, behind the tailnet's
  Caddy route. This design names no Inbox's location and needs nothing
  of that design in either order.
- **Who tells an agent to write there.** Plannotator's prompt hook tells
  every Claude Code session on the host, a flume stage's agent among
  them, to send a decision only the person can make to the Inbox as a
  `:::question` and keep going. So a construction agent today may ask
  the user in the Inbox, beside or instead of `flume needs`, where no
  answerer of the place's sees it. swancloud's fix
  `a-places-agent-asks-through-flume-never-the-inbox`, launched
  2026-11-05, is its host's part of that.
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
  `inboxDeliveryState`). Once the asker has ended, the person must pick
  a live session to hand the thread to. A role's run ends at every
  delivery, so a reply to the run's own session waits for a pick.
- **How the person sees an agent.** Each call may give `agent_name`,
  "How the person sees you", which the Claude Code tool fills from its
  session's name where it has one. flume's agents have none, so every
  agent shows as Claude. The user, 2026-11-06: "None of the agents have
  names and they all just say Claude".
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
    under a key of its own, in words of its own, and a construction
    agent may ask the user in the Inbox itself. Nothing ties what the
    user is asked to the place's question, so a second reader asks
    again, and nothing withdraws a question answered some other way. The
    user's answer comes back by whoever reads it, by hand. A reply in
    the Plannotator Inbox reaches only the session that asked, and a
    role's run has ended by the time the user answers.

### Rules and records relied on

- **flume's statement** (agentplot/blueprints main at a4eeec3).
  Section 3's recipient, stream and kept open; 7, 8, 28, 32, 41, 42, 43,
  49, 53, 55, 56, 62, 69, 70, 72, 109, 114, 130, 132, 143, 154, 155, 158,
  180, 181, 190, 191, 192, 195, 197 and 198. 69 says
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
  - 0046, accepted by the user on 2026-11-01 and carried by the unlanded
    change `an-answer-reaches-the-open-session-within-an-hour`, its
    statement on blueprints main at c188bf3 (clause 198, S104, S105): an
    agent that stops short needing an answer is kept open for it, an
    hour by default, and an answer by a retry or a tell then reaches its
    session, recorded on the stop-short it answers. That answer is the
    one this design takes once by its key.
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
  launches into flume's), briefs.13 (crew's own cards built from their
  events' facts); records 0031, 0047 move 6, 0048, 0049, 0052; the
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
- A question that is the user's reaches them once, as one Pending You
  card written by code, with a thread in its partition's Inbox where it
  needs talk, and their answer reaches the place as its retry's words
  by code, with no agent or hand carrying it.
- An answer reaches whoever asked, never a session the user picks.
- No construction agent writes a card or an Inbox message: it asks by
  `flume needs` alone.
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
  the answer as its words, the question before it, or a tell while the
  asking agent is kept open (42, 49, 53, 198). Keeping an agent open,
  and handing an answer to its session, are record 0046's change's.
- A session standing in for a role's address. Standing roles leave as
  their partitions move them (decision 11.5).
- sluice's own outcomes, which are not questions. They stay sluice's
  (decision 12.2; Found beside).
- Moving crew's rail off Pending You. crew's cards stay there for good,
  raised by its code, and the follower's Pending You notice runs `crew
  _answered` for them. What crew changes of its rail, a role's own
  question raised by its code and a card's thread, is crew's (12.3).
- Designing the elaboration kind, which is sounding's (capture design;
  crew's record 0019). This design says only what a question's thread
  asks of one.
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

[roles."madswan/design"]
agent = "madswan-design"               # stands until the user moves it
answers = "A decision the design has not made: how something should behave, a spec's reading, a record to write."

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

**A role not yet moved is declared standing.** Until the user moves a
role onto its kind, crew's file gives its address `agent`, the name the
stream's terminal knows its standing agent by, with `session` where it
lives in another session than its stream's, and `answers` and
`answerer` as a run's entry gives them. Its ends go into that agent's
pane as notices and wake no run, as record 0013's amendment has them.
Its stream is its address's, so the entry names none, and the agent is
found in that stream's terminal (190). A move is crew's file trading
`agent` for `kind` and the run's keys, and nothing of the host's
changes. The change `a-roles-address-may-be-held-by-a-standing-agent`
builds the standing agent's delivery over the host's `[roles]`, and
this change reads the same entry from a source's roles file: whichever
lands second carries the entry's reading to where the other left it
(record 0039's amendment).

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
ruling of 2026-11-05). crew's follower writes a change on a role's card,
and the user's reply in a role's thread, as an ask of the role (decision
10), and the runner wakes the role for it.

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

A role's run whose kind lands, the code stage's own reading, is not
declared done: 114 refuses a done on a place whose kind lands, so the
settle that reads its last stage delivered submits it as any place's
(111), and it is tidied once it lands or is dropped (63, 66). The next
end for its address wakes another run once it is tidied. crew's roles'
kinds land nothing, so no run of theirs takes this way; the path not
taken was to refuse a landing kind in a role's entry, which would have
the reader open each kind's package to read whether it lands. Statement
132 says a run is declared done once its last stage delivers, with no
exception. The exception is written for 132 on agentplot/blueprints'
branch `design/flume-a-roles-run-whose-kind-lands`, for the
orchestrator to land (core.19).

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
  names a stream the host does not declare and is not carried, that
  gives `agent` beside `kind` or a run's other keys, or that gives
  `carried` beside a run's keys, `agent` or `answerer`; and, the code
  stage's own reading, one that writes its kind with a version, since
  C.7 has the entry name the kind and the locks give the version, and
  one that gives `points`, which this flume takes for no place, as it
  refuses a stream's `points` (165), rather than one taken and its
  points dropped unread. Statement 132 and C.7 list points among what a
  source declares for a run. That holds once 165 is built, when an
  entry's points are taken as a launch's; until then the entry is
  refused, as a stream's points are;
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
  by their addresses. It only bridges the roles that have not moved,
  which their standing entries reach (decision 11.5).
- **The standing entry left in the host's `[roles]`** while a run's
  entry moves to the source. Two declarers of crew's roles, and every
  host repeating crew's partitions, which the user refused: "they are
  an artifact of crew".
- **Standing roles dropped**, their questions reaching the door at the
  stall. The user approved C.5's standing row on 2026-11-06 for the
  time before each move, and a question to a standing planner would
  wait out the stall bound for nothing.

### 3. Which needs are whose

From crew's briefs, a need is the role's whose work it is:

| the need | whose | the brief that says so |
|---|---|---|
| A decision the design has not made: how something should behave, a spec silent or at odds with the code, a rule's reading, a record to write | the design agent | design.md, *Answering a design question* |
| The work's scope: work outside the bolt's goal, which bolt a finding belongs in, a unit, chore or fix to add, the order of work, and work found that a stage before must do again (`--found`, 131) | the planner | planner.md, "Its places' questions are yours", "Work outside a bolt's goal" |
| A live system: a reading of what is deployed, a credential, a host's tools, a deploy, a landing that failed, main red | main-level ops | design.md, "from `{{MAIN_OPS}}` for a live system"; main-ops.md, a fix of main |
| A choice that is the user's alone: their name, money, plans, anything public or hard to undo | the user, on one Pending You card crew's code raises on the word of the role whose subject it is | design.md, *Decision records*; capture's rule |
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
  share (core.31). The part says too that a stop-short is the agent's
  one way to ask: it writes no card and no message to a person, since
  whoever answers the question decides whether it is a person's (69,
  72).
- **A role that takes a question not its own passes it.** It tells the
  role whose it is, by `crew tell`, which is an ask for a role that runs
  as its kind, with the place, the key and the words, and holds the
  waiting move, "passed to madswan/design", with a bound (decision 7).
- **The door stands behind every stall**, the last to pick (decision 6).

So "an agent might need something from ops separately from the planner
or the designer" (the user, 2026-11-05) is `flume needs --to ops`, and
a need of another stream's ops is `flume needs --to swancloud/ops`. A
wrong pick costs one pass, which the door would cost on every question.

A role still standing is in the list where crew's file gives its
standing agent words of what it answers, and a stop-short naming it
reaches that agent's pane as a notice (132; record 0039's amendment).

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
| The role still stands, its standing agent declared for its address | The end goes into the agent's pane as a notice. Where the terminal finds no agent of its name, or more than one, the end stays pending, and at the stall the owner and the door have it. |
| The door's own run cannot run | Its run's stall goes to no address, since it is its own stream's answerer, and stands in status under the host's addresses (Risks). |

An end of a stall recorded before this change names no answerer. The
first settle after the build reads it as the answerer's too, where the
place still waits under no hold's words, so the questions waiting on
deneb when the build lands reach the door at once. A stall this change
records says it read the answerer, `answerer_read`, so a stall that
reached no answerer because none stood behind the stream is never read
again; the settle that reads an earlier stall reads the stop-short the
place waits on with it, so the door has the question whole. Reading
every stall that names no answerer at every settle was the other way:
it reads the sources for every stalled place at every settle, and
reaches an answerer declared after the stall.

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
makes it and how, and an optional bound in the forms `--stall-after`
takes, `<n>`, `<n>s`, `<n>m` or `<n>h`:

```sh
flume hold <place> "madswan-planner has it" --waiting --for 1h
flume hold <place> "passed to madswan/design" --waiting --for 1h
flume hold <place> "on the user's card req_01K…" --waiting --for 24h
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

A place stalled before the hold reads not stalled once it is put. The
hold ends once the place waits on another move or none, whatever made
the move: a retry, a drop, or a stage that went on. A lapsed hold stays
recorded, with when it lapsed, until the move is made or a later hold
replaces it, and holds nothing. The stall its lapse makes is since the
lapse, so its key, `stall/<since>`, is its own. A bound is given only
to a hold on the waiting move; a hold on a stage or the submission that
gives one is refused.

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
  `stage/construct/1/ended-short`, with `/<answered>` after it where
  the start had taken that many answers in its session when the end was
  read, as record 0046's change gives `EndOf::key`, so a second
  stop-short of one start kept open is a key of its own;
- `submission/<n>/<outcome>` for a submission's;
- `stall/<since>` for a stall.

Each notice, each collected end, each waiting place the inbox gives,
and status give the key. A stop-short's key names its start, so a
fresh agent that asks again asks under a new key, a new question.

An answer may name the key of the stop-short it answers:

```sh
flume retry <place> --stage <stage> --answers <key> --words "<answer>"
flume tell <place> --answers <key> "<answer>"
```

It is taken only while the place waits on the stop-short `<key>`
names. Otherwise it is refused, naming the place, the answer that
stop-short took, who gave it and when, or naming the move the place
waits on now. An answer so taken records on the stop-short's end the
words, who gave them, as the answering command's recipient names it,
and when, in the record where 0046's change records an answer handed
to a session (`EndHeld::answer`), which adds the session it reached.
An answer with no `--answers` is taken as today, so every caller keeps
working.

Record 0046, on blueprints main at c188bf3 (198), keeps an agent that
stops short needing an answer open for it, an hour by default, and an
answer by a retry with words or by a tell then reaches its session
(42, 49, 53). So an answer is taken once whether it reaches the open
session or a fresh agent: the key is read before either. A tell
answers only an agent kept open, and is refused naming the retry
otherwise (53), so `--answers` on a tell is read only there.

The held key, `EndOf::key`, stays as it is: a settle records an end
once by it, and every document written before keys as it did. The
token callers see is made from the same parts and read back from the
token alone. A token in any other form is refused, naming the forms. A
number written otherwise than the key writes it, `/0` or `01`, is
refused too, so each end has one token. A stuck start's token leaves out
the quiet reading its held key names. A stall's stop-short carries the
token, since only this change writes it.

An answer naming a key is checked before anything is ended, started or
typed. It is refused where the place does not wait on that stop-short,
or where its end holds an answer already; the refusal names the answer,
who gave it and when where there is one, and the move the place waits
on now in either case. It is refused too where it names another stage
than the stop-short's, or gives no words. Every retry with words of the
stage whose stop-short the place waits on records the answer on that
end, whether the agent kept open takes it or a fresh agent carries it,
the session recorded only for the first. The command line records as
who gave it the recipient the command acts as where it names none:
`$FLUME_RECIPIENT`, the role of the run it runs in, or the terminal's
session.

**Two answers at once are taken once.** A plain read of the key, then
handing the words, then recording them, would let two answers checked
together both be typed: the hand waits up to ten minutes for the agent
to wait again (53), and the second's check lands in that time. So an
answer claims its stop-short's end by compare-and-set before it is
typed into the agent kept open, and before a fresh agent is started to
carry one that names its key (75). The claim is taken only while the
place waits on that stop-short, its end holds no answer and no other
answer's claim holds. A second answer that finds a claim is refused with
nothing typed, naming who gives the first and when it was claimed. The
record clears the claim, and the command lets it go where its answer is
not recorded, a hand the agent did not take among them. A claim holds
for the hand's bound and five minutes past it, so one left by a command
ended mid-hand lapses and a later answer may take the stop-short. An
answer with no key that a fresh agent carries claims nothing, and is
taken as today. The path not taken was to record the answer before
handing it and take it back where the hand failed: a record that is
taken back reads, between the two writes, as an answer the agent never
had.

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

The answerer is given a place only for a stall of the move the place
waits on now, one whose since is that move's or a lapsed hold's after
it, so an answerer is not handed a place for a stall it had of an
earlier move the place has since left. The hold given is the one on the
move that still holds; a lapsed hold holds nothing, and status gives it.
The brief's first line is its first line that holds anything, trimmed.

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
   its question, its key, whether its agent is kept open and until
   when (198), and the three ways on: answer with the key, by a retry
   or, while its agent is kept open, a tell; hold the move with words;
   or leave it, and the door has it at the stall.
3. Where the answer holds nothing, it gives nothing.

The core makes the words (`session_words`), as it makes every line a
surface prints (core.14), and the hook hands them on as
SessionStart's `additionalContext`. The ways on name the commands as
sections 7 and 8 give them, with the place by its kind and name:
`flume retry <place> --stage <stage> --answers <key> --words
"<answer>"`, `flume tell <place> --answers <key> "<answer>"` while its
agent is kept open, and `flume hold <place> "<words>" --waiting --for
<time>`. A place that waits on any other move is given that move's own
command in place of the answer.

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

An end waits on a move where the place, as its held state stands when
the notice is sent, waits on a move of its caller's. That is read from
the place, not from the end's kind: a stop-short, a lost stage or a
stall points at the inbox, and so does a stage delivered whose place
then waits on a held submission or on done; a delivery the next stage
follows, and an end sent after the place moved on, point at status. A
brief cut ends in an ellipsis where it was cut, after its first 60
characters, trailing spaces dropped.

Rejected:

- **Recording, at the launch, which conversation in the pane launched
  the place, and not delivering to another.** herdr gives no
  conversation's identity of a pane flume did not start, and the stall
  reaches the door either way.
- **A notice holding the question's words.** Words are many lines, and
  a notice is typed into a pane (core.11).

### 10. The answering run's protocol, and the user's one card

The user, 2026-11-06: "with the previous crew design, the whole point
was that all of the pending U card integration was statically defined
in the Python code, not done by agents. I think for most of the
construction kinds, that's what we want. For the elaboration kind only,
we would maybe be customizing the MCP servers and instructions for
those. I think that while those are open, they should be named agents
talking to these tools and they're there when I respond to them."

flume's part ends at the role: it reaches the role, gives it the
question with its key, takes the hold that says the role is on it, and
takes the answer once. What the role does is its kind's, and how a
question reaches the user and its answer comes back is crew's code. The
design gives the protocol each answering role's run follows, for crew's
turn, briefs and code, and for any source's own answering kind.

**Who writes to the user.**

| who | a card | an Inbox message |
|---|---|---|
| A construction kind's agent: every stage of a change, a fix, a chore or a plan | never | never. It asks by `flume needs` alone (decision 4) |
| A role's run | never. It says a place's question is the user's by `crew question`, and crew's code raises the card. A role's own question is raised by crew's code too, on its rail (12.3) | in a question's thread, as its address, never as its run's session |
| An elaboration's agent, while open | never | in its own thread, as a named agent, there when the user answers |
| crew's code | a place's question's card, crew's own six (briefs.13), and a role's own question's | a question's thread's first message, where the question needs talk |

A construction agent is kept to `flume needs` in two ways. flume's way
to stop short says so (decision 4). The host's layers give the Inbox's
and Pending You's tools, and their instructions, to an elaboration
kind's agents alone and to no construction kind's (32, 33, 195), so no
prompt hook tells a construction agent to send the user a decision. On
deneb Plannotator's prompt hook tells every session so today, a flume
stage's agent among them. Taking it out of a construction kind's
settings is the host's, as swancloud's fix
`a-places-agent-asks-through-flume-never-the-inbox` does on the hosts
swancloud configures.

**The protocol.**

1. **Collect.** `crew role start`, then `flume inbox --json` for its
   address, reading `ends` and `waiting`. The run reads flume's own list
   of what waits on its address, on the user's word of 2026-11-05: "Yes:
   a role's run reads Flume's list for its address".
2. **Hold what it takes**, before anything else: `flume hold <place>
   "<run> has it" --waiting --for 1h`, so a turn that runs long does not
   stall the place.
3. **Answer what is its own**: `crew answer <work or place> --answers
   <key> "<answer>"`, which the runner carries as `flume retry …
   --answers <key> --words "<answer>"`. It reaches the asking agent's
   session while that agent is kept open, and a fresh agent otherwise
   (42, 49, 198).
4. **Pass what is another role's**, of its own stream or another's:
   `crew tell <stream>-<role>` with the place, the key and the words,
   and hold "passed to <address>" with a bound.
5. **Say what is the user's**: hold first, "putting it to the user",
   then `crew question <place> <key>`, with `--talk` where it needs talk,
   and hold again naming the card, "on the user's card req_…", with a
   bound of a day or more. The run writes nothing of the card. crew's
   code builds and raises it from flume's facts (*The question's card*,
   below).
6. **The door, on a stall.** The operator's run reads the stall's words
   and any lapsed hold:
   - nobody had the question: it passes it once to the role whose it is,
     by decision 3's table, of whichever stream, and holds with a bound;
   - a role had it and its hold lapsed: where the lapsed hold names the
     user's card, it holds again naming the card and raises nothing,
     since the card's answer reaches the place by code; where the
     question is the user's and no card names it, it says so by `crew
     question`, which finds a card raised under the end's key and raises
     none again.

**The question's card.** `crew question <place> <key> [--talk]` is
crew's code from end to end:

- **It reads the question from flume**: the waiting place as `flume
  --json inbox` gives it for the role's address, or the place's status
  where the role took it by a pass: the stop-short's words whole,
  whether it needs an answer or found work, its key and addressee, the
  place's kind, name and stream, the first line of its brief, and
  whether its agent is kept open and until when (198). It refuses,
  naming the move the place waits on now, where the place no longer
  waits on that key.
- **It builds one card**, a seventh of crew's own built from facts, in
  `cards.py` as briefs.13 builds the six: the question in the asking
  agent's words, whole; the place, its brief's first line and its
  stream; the role that put it to the user; and one answer in the
  user's own words. No agent rewords the question.
- **It raises it once.** It posts the card on Pending You in the role's
  name, `<stream>-<role>`, as a batch's card is posted in the
  planner's, filed in the kit of the place's repository, under the
  idempotency key `flume/<host>/<digest of the place's location>/<end's
  key>`. A second `crew question` for the same end, by any role or run,
  the door's at a stall among them, finds the card under its key and
  raises nothing (`post_key`, `card_under`).
- **Where it needs talk**, `--talk`, it gives the card a thread in the
  Inbox and links it from the card (*The question's thread*, below).

**The answer, carried by code.** The user answers on the card, once.
Pending You's notice runs `crew _answered` at once. For a question's
card, crew's code reads the answer, the user's own words, and writes
the request `Act: retry` naming the place and the stage, with
`Answers: <end's key>` and those words. The runner carries it as `flume
retry <place> --stage <stage> --answers <key> --words "<the user's
words>"`. flume takes it once (decision 8). While the asking agent is
kept open it reaches that session (49, 198), and otherwise a fresh
agent's prompt carries the question with the answer (42). No agent
reads the answer or carries it, and no role is woken for it, since
nothing of the place waits on a role once the user has answered. A
retry refused for its key is the request's outcome, in flume's words,
and crew's code closes the card saying the question was answered
elsewhere. A message the user writes on the card without answering it
is talk, not an answer: it reaches the role in whose name the card is
as the `card` ask `crew _answered` writes today.

**The question's thread, where it needs talk.** The card links to one
thread in the Inbox of the asking place's stream's partition, the
thread named for the place, `<kind>--<name>`, so a place's talk is one
thread. It is held by one of two:

- **The role, as its address.** crew's code opens the thread with the
  question whole and the card's link as its first message. Every Inbox
  call of crew's code or of the role's runs for it is `plannotator inbox
  call --session crew:<stream>-<role>`, from an environment without the
  run's own `PLANNOTATOR_SESSION_TAG`, giving `agent_name` the address,
  `madswan/planner`, so the user sees which role writes and never a
  bare Claude. A reply is addressed to the address's identity, which no
  host wakes, so it waits in the Inbox, its thread `unread` for that
  identity, and the user picks no session. crew's follower reads, at its
  minute, the threads of each role on its host that runs as its kind,
  as that role's identity in its partition's Inbox, and for each thread
  `unread` writes an ask of the role, keyed by the person's last message
  in it, so a later minute writes no second ask. The runner wakes the
  role for it by `flume wake`, and the role's run reads the thread as
  its address and replies there. So whatever answers a role's question,
  card or thread, reaches the role at its address by code, whichever
  session holds the role.
- **An elaboration's agent**, where the talk needs someone there as the
  user answers. `crew question --talk` then opens an elaboration of the
  question, a place of the elaboration kind crew names for the
  partition, in the asking place's stream, launched through the runner
  as crew's launches are (roles.9), its brief the question whole and
  the card's link. Its stage is a conversation with the user (109). Its
  agent is a named agent: it passes its place's name, as its prompt
  gives it, as Plannotator's `agent_name`. Its kind's settings give it
  the Inbox's and Pending You's tools and their instructions (32, 195).
  It opens the thread as its own session and stays open while the talk
  lasts, there when the user answers, so a reply reaches its live
  session. crew's code links the card to the thread, found by its name.
  The elaboration writes no card and answers nothing to the place: the
  user answers on the card, and code carries it. crew's code finishes
  the elaboration (109) once the card is answered or closed, and the
  elaboration resolves its thread then. An elaboration's agent started
  again is a fresh session. It reads its thread by its name before
  anything else and writes in it, so the user's next reply reaches it,
  and what the user wrote between waits in the thread it has read.

The elaboration kind is sounding's (capture design; crew's record 0019
leaves elaboration to sounding). Until a host holds one, a question's
thread is the role's.

**The answer reaches whoever asked.**

| who asked | the answer | how it reaches the asker |
|---|---|---|
| A place, by `flume needs`, put to the user | the user's words on its card | crew's code, the runner, `flume retry --answers` (42, 49) |
| A role, on a card of crew's own | the card's change | `crew _answered`'s `card` ask of the role, and the runner's `flume wake`, as today |
| A role, in a question's thread | the user's reply | crew's follower's ask of the role, and the runner's `flume wake` |
| An elaboration's agent, while open | the user's reply in its thread | the Inbox, into its live session |

None is a session the user picks.

**What Plannotator lacks**, for a role's thread, read against 0.28.48:

- **A watch.** Nothing runs when the person answers a session no host
  wakes, so crew's follower reads each role's identity at its minute. A
  command the Inbox runs for each reply to such a session, `plannotator
  inbox watch [--session <id>] -- <command>`, as Pending You's notice
  runs one for a card, would carry a reply at once.
- **An identity for an address.** A reply goes to the session that
  asked, and a role's runs are many sessions. The design has each call
  drop the run's own `PLANNOTATOR_SESSION_TAG` and name `--session
  crew:<stream>-<role>`, which the Inbox takes from a shell that names
  no session. The Inbox's own words offer dropping the variable only for
  a shell older than its session, so this is a use it does not promise.
  An identity a caller names beside its session, owned by no host and
  never read as ended, would make it one, and would never ask the
  person to pick a session for it.
- **An address as an id.** An id holds no `/`, so the identity spells
  the address `<stream>-<role>`, and `agent_name` gives it whole.

Neither stops the design: a place's question reaches the user and its
answer comes back through Pending You and its notice, which need
neither. They are asks of Plannotator (12.5).

**Never asked twice** is held at four points:

- the hold, which keeps the question from the door while its role works
  on it or the user has it;
- the card's idempotency key, which gives a second raiser the first card
  and raises nothing;
- the answer's key, which keeps a second answer from starting anything;
- the withdrawal: crew's follower, at its minute, reads the runner's
  copy of flume's status, as its board does, closes each question's card
  whose place no longer waits on its key, saying why, resolves its
  thread, and finishes its elaboration.

**Which Inbox.** A question's thread is in the Inbox of its asking
place's stream's partition. Where each partition's Inbox runs is
swancloud's design `the-plannotator-inbox-runs-on-deneb-for-the-users-devices`,
being redone on the user's answers of 2026-11-06: one Inbox per
partition, behind the tailnet's Caddy route. A role's run and crew's
follower reach a partition's Inbox as that design gives that
partition's agents theirs, and deneb's one Inbox until it does. Nothing
of flume's names an Inbox, so this design and swancloud's need nothing
of each other in either order.

Rejected:

- **flume raises the card, through a seam for a person's queue.** It
  breaks 72 and core.1, and flume would judge which questions are a
  person's.
- **A role's run writes the card, or asks the user in the Inbox in its
  own words.** The user, 2026-11-06: "the whole point was that all of
  the pending U card integration was statically defined in the Python
  code, not done by agents". A card an agent writes rewords the
  question, and nothing but its own care keeps it to one card for one
  end.
- **The asking construction agent asks the user itself**, in the Inbox
  or on a card. No answerer of the place sees the question, and the
  reply goes to a session flume ends within its time kept open. The
  user, 2026-11-06: no construction agent writes a card or an Inbox
  message.
- **A place's question in the Inbox, and crew's own cards on Pending
  You.** The user, 2026-11-06: "We are trying to use Pending You and
  Inbox together, not as alternatives".
- **A card raised for every question that reaches the door.** The user
  would be asked what a role could answer. Which questions are the
  user's is the role's to say (69).
- **The answer carried by the role's next run**, which reads the card
  and retries. An agent would carry the user's words, a run would be
  woken to carry them, and a role's turn would stand between the user
  and the place.
- **A thread whose replies go to a run's own session.** The run has
  ended by the reply, and the user would have to pick a session. The
  user, 2026-11-06: an answer reaches whoever asked, never a session
  the user picks.
- **A thread for each question**, named by the place and the end. The
  card is the question's. A thread is the place's talk, and the card
  names its end's key.

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
- **The answer's key**: `--answers <key>` on a retry, and on a tell
  that answers an agent kept open (198), taken once (decision 8).
- **Routing**: `flume needs --to <role or address>`, and the roles
  listed in the stop-short part of every prompt with their sources'
  `answers`, the place's own stream's by name and every other's whole,
  the part saying a stop-short is the agent's one way to ask (decision
  4).
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
  protocol: hold, answer by `crew answer`, pass by `crew tell`, and say
  what is the user's by `crew question`, writing no card itself.
- **`crew question <place> <key> [--talk]`**, crew's code (decision 10):
  the question read from flume, the place still waiting on the key;
  one card, a seventh of crew's own built from facts in `cards.py`, the
  question in the asking agent's words whole, posted in the role's name
  under the idempotency key `flume/<host>/<digest of the place's
  location>/<end's key>`, found and not raised again under that key;
  and, with `--talk`, the place's thread in its stream's partition's
  Inbox, opened as the role's address, or an elaboration of the
  question where crew names an elaboration kind for the partition, the
  thread linked from the card.
- **`crew _answered`**, run by Pending You's notice as today: for a
  question's card, the user's answer written as the request `Act:
  retry` with `Answers: <end's key>` and the user's words, and no ask
  of a role; a message on the card, the `card` ask of the role it is in
  the name of, as today; every other card as today.
- **The follower** (`crew _follow`): at its minute's read, for each role
  on its host that runs as its kind, the role's threads read as its
  address's identity in its partition's Inbox and an ask of the role
  for each thread `unread`, keyed by the person's last message in it,
  reading nothing of the reply; each question's card whose place no
  longer waits on its key, read from the runner's copy of flume's
  status, closed, its thread resolved and its elaboration finished. Its
  Pending You notice, which runs `crew _answered`, stays as it is. Once
  Plannotator offers a watch, the follower hangs its asks for a thread
  on it in place of the minute's read of the Inbox.
- **The teams file**: for each partition, how its Inbox is reached,
  written by whoever writes the teams file once swancloud's design
  gives each partition its Inbox, and none until then, deneb's one
  Inbox; and the elaboration kind it names for a question's talk, none
  until a host holds one.
- **`crew answer`** carries the key as `Answers:`, takes a place on the
  host that crew did not launch by its place's name, and is open to the
  operator beside the planner, the design agent and ops.
- **The briefs**: each role's part of decision 3's table and decision
  10's protocol, a question of another stream's system addressed to
  that stream's role; the operator's, the door behind every stall of
  its stream, which may say a stalled question is the user's by `crew
  question`, once; briefs.11: a place's question is put to the user by
  `crew question` alone, never on a card or in the Inbox in a role's own
  words, and a role writes in the Inbox only as its address, with its
  address as `agent_name`. A role's own questions are cards crew's code
  raises on its rail, as its six are, each linking a thread in the
  partition's Inbox where it needs elaboration.
- **The records and rules**: roles.7 holds again as written, since flume
  wakes a role. The change `roles-run-as-flume-kinds` Decisions 1, 3 and
  6, and its departure from roles.7, give way. Record 0049 part 1 holds,
  its role addresses declared by crew's own roles file rather than the
  host's `[roles]`, each the role of its partition's stream, and the
  runner carrying crew's events by `flume wake` where it named the card
  queue's own wake. Record 0031 and briefs.13 gain the question's card
  beside the six. The change's record of the user's word of
  2026-10-31, "crew reads nothing of Flume's but what the runner carries
  back", gains their word of 2026-11-05: "Yes: a role's run reads
  Flume's list for its address", and `crew question`'s read of the
  place's question from flume beside it.

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
  `swancloud.boomdrive`, which keeps it running. An answer of crew's,
  the user's answer on a question's card among them, reaches flume, and
  an ask the follower wrote for a reply in a role's thread wakes its
  role, at its next pass.

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
runner carries to `flume retry`. Status says the address is carried. A
settle records nothing of a carried address as unwoken, as it records
nothing of a standing one, and a `flume wake` of it on the place's host
is refused, saying its source declares it carried, its run on another
host: the code stage's own reading, since the wake belongs on the
role's host. On deneb, crew's file carries wldn's roles, whose stream
runs on chuck-herdr-alpha.

#### 11.5 `flume serve`: no purpose left

`flume serve <address>` would have a session take an address's ends
while it is live. Once roles run as kinds, nothing needs it:

- **A role the user opens to talk** is a run: `crew role open` writes an
  ask, the role's run greets the user in its pane and keeps the
  conversation until they are done (crew's Decision 13). Its pane is the
  run's, reached by the address as any run's.
- **A person's own session** is reached as a session, and collects at
  its start (decision 9).
- **A standing role** is reached at its address without serving: its
  source declares the standing agent that holds it (decision 2), and
  its ends go into that agent's pane. Each partition's moves retire the
  standing entries one at a time.

So serving has no purpose left, and is left out.

#### 11.6 One owner for each part

| part | owner | the others |
|---|---|---|
| A stream, which an address names | the host (143) | a source writes its addresses on the host's streams; flume checks each is declared |
| Which roles answer, and what each run is launched with or which standing agent holds it | the source whose roles they are, crew for its partitions (193) | the host declares the source, locks its kinds and declares its streams (143, 155, 191); flume reads |
| A place's recipients, and the answerer behind its stream | flume resolves them (7, 193) | crew declares each stream's door; the runner passes crew's recipients unchanged (Boomdrive core.15) |
| Which role a question is for, of which stream | the asking agent, else the place's owner | flume lists the roles; the role passes what is not its own |
| Delivering an end, and keeping it pending | flume (43) | the runner's copy of status is a view for crew's board and its cross-host records, never a delivery; sluice's outcome for a place reaches flume at its settle (C.6) |
| Launching a role's run, and ending it at its delivery | flume (132) | crew's follower writes crew's events as asks, and the runner carries them to flume by `flume wake` |
| The bound | flume (130, 193) | a role says it is on a question by a hold |
| A session's start collecting | each tool for its own: flume's plugin for flume's ends, sluice's for sluice's | one design, sluice's (sluice 51) |
| Which questions are a person's | the role whose question it is, or the door at a stall | flume decides nothing (69, 72) |
| A place's question's card, raised once | crew's code, `crew question`, on Pending You | the role says the question is the user's and writes nothing of the card; no construction agent writes a card or an Inbox message |
| Carrying the user's answer on the card to the place | crew's code, run by Pending You's notice, and the runner's retry | flume takes it once (42, 49) |
| A question's thread, where it needs talk | the role as its address, or an elaboration's named agent while open | the Inbox of the stream's partition, where swancloud's design runs it |
| Carrying a reply in a role's thread to the role | crew's follower, as an ask, and the runner's wake | Plannotator offers no watch (decision 10) |
| crew's rail of decisions: its cards, raised by its code, and carrying their answers | crew's code, on Pending You for good, and the follower's notice running `crew _answered` | a card links a thread in the partition's Inbox where it needs elaboration |
| Which agents get the Inbox's and Pending You's tools | the host's layers (32, 195): an elaboration kind's agents alone | flume's way to stop short says a stop-short is the one way to ask (28) |
| Taking an answer once | flume (42, 49) | `crew answer`, `crew _answered` and the runner carry the key |

### 12. The statements' amendments, for the user to approve

#### 12.1 flume's statement (agentplot/blueprints, `design/flume/requirements.md`, on main at a4eeec3)

Each amendment quotes main's words as they stand at a4eeec3, which
holds record 0046's 198, S104 and S105, record 0048's 199, S106 and
S107, record 0049's 200, 201 and S108, the program settings' 195, 196
and S100 to S102, the settles' 202 to 206, 208, S109 to S112 and S118,
record 0052's 207 and S113 to S117, and record 0013's standing agent in
section 3's recipient, 43, 132, C.5 and C.7. A standing agent's entry
is declared with its source's roles, as record 0039's amendment has it.
The amendment is committed on the branch
`design/flume-every-question-a-place-asks-reaches-someone-who-answers-it`
at 6d2598e, cut from main at a4eeec3, where the statement's voice splits
some of the sentences below; that commit's text is the amendment.

**Section 3.**

- Replace "**recipient**: whoever an end of a place goes to: a caller's
  session, or a role's address, held by a run of its kind or by a
  standing agent (43, 132)." with: "**recipient**: whoever an end of a
  place goes to: a caller's session, or a role's address, held by a run
  of its kind or by a standing agent, the answerer behind its stream
  among them (43, 132, 193)."
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
gives for what that role answers (41, 193). It is the agent's one way
to ask for what its stage needs: whoever answers the question decides
whether it is a person's, and how it reaches them (69, 72)".

**Clause 41.** After "(131).", add: "It may name the role its words are
for: the address of a role a source of the host declares with words of
what it answers, of any stream, or the role alone for a role of the
place's own stream (132, 193). The stop-short's end then goes to that
address and not to the place's recipients. A name that is none of those
is refused, naming them."

**Clause 42.** After "Otherwise the answer is a retry, and the fresh
agent's prompt carries the question with the answer.", add: "Every
answer is recorded on the stop-short it answers, with who gave it and
when. An answer, a retry's or a tell's, may name the stop-short's key,
and is then taken only while the place waits on that stop-short, so a
question is answered once, whether its answer reaches the agent kept
open or a fresh one: a second answer, or one that comes after the place
moved on, is refused naming the answer taken, who gave it and when (49,
53)."

**Clause 43.**

- After "it counts delivered on the terminal's answer, with no reading
  back.", add: "A notice names the place, the first line of its brief,
  the end and its key, and where to read the end whole."
- Replace "or of the standing agent the host's configuration names for
  it." with: "or of the standing agent its source declares for it
  (193)."
- Replace "by `flume inbox`, which delivers each and marks it delivered;
  status and a read mark nothing." with: "by `flume inbox`, which
  delivers each and marks it delivered, and gives with them every place
  that waits on its caller and that it is a recipient of, was named by
  the stop-short the place waits on, or was given a stall of, delivered
  or not, with the move and since when, the words, key and addressee of
  a stop-short it waits on and a hold's words, so a caller that
  restarted sees each question still its own; that list, status and a
  read mark nothing."

**Clause 49.** After "and flume judges no order of stages but its own
starts (40).", before the lost conversation's retry, add: "A retry that names a stop-short's key is
refused, naming the place, the answer the stop-short already took, who
gave it and when, or the move the place waits on now, unless the place
waits on that stop-short (42)."

**Clause 53.** Append: "A tell that answers an agent kept open may name
the stop-short's key, and is then refused as 49 says unless the place
waits on that stop-short (42, 198)."

**Clause 56.**

- After "or of a failed submission;", add: "each end's key; for a
  stop-short, the role's address it named and each answer it took, with
  who gave it and when (42); the place's recipients, each with where it
  came from;".
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

**Clause 132.** Replace it whole, as main at a4eeec3 has it with record
0013's standing agent, with: "A caller may name a recipient by a role's
address, `<stream>/<role>`, rather than by a session: the role a source
of the host declares for a stream the host declares (143, 193). A place
of any stream may name an address of any other. A run of the role is a
place of the kind its source names for the address, in the address's
stream. Its name is the one its source gives it, else its stream and its
role joined. So at most one run of an address is live at once (3), and
two streams' runs of one role are two places. Its agent takes the
address as its session's name. Its repository, base, priority, intent,
points, brief and settings are those its source declares for the
address, as a launch's are (7, 32). A run is over once its last stage
delivers. flume declares it done at the settle that reads the delivery,
with no caller's word, and tidies it (114), so an end that comes later
wakes another run. A run's own stall goes to the answerer behind its
stream and never to its own address (130, 193). Where a settle finds an
end pending for an address with no run live or waiting for room, or a
caller or another tool's carrier asks flume to wake the address, flume
launches the address's kind at the priority its source declares. A wake
of an address whose run is live or waiting does nothing. The run's start
collects every end pending for the address (43), so whatever came while
no run was live is one run's. flume decides nothing in this: which
address is a place's recipient, what the role's kind does and what its
run decides are the caller's (69, 70, 72). Nothing wakes an address no
source declares, or two declare, or one whose stream the host does not
declare. Where its source declares it carried, its run is on another
host, its stream that host's, and its ends stay pending for the tool
that carries them there. A launch naming an address that wakes nothing
and is not carried is taken only where an answerer stands behind the
place's stream (4, 193). Status says the ends pending for each address,
and whether it is woken, held by a standing agent, carried or wakes
nothing. Its source may instead declare a standing agent for an address:
an agent flume did not open, in the terminal session it lives in (C.7).
The source names it by the name the host's terminal knows it by. An end
for that address goes into the standing agent's pane as a notice, as any
end does (43), and wakes no run. flume opens nothing for the address and
keeps no watch over it: it delivers at the settles that read the end.
Where the terminal finds no agent of that name in that session, or more
than one, the end stays pending, and status says where the address lives
and why nothing went. The address stays the recipient a caller names, so
which agent holds it is its source's alone, and `flume inbox` collects
for it as for any recipient; a role that moves from a standing agent to
a run changes its source's entry for the address and nothing of the
host's. An entry giving an address both a standing agent and a kind
declares nothing for it, and status names it, since the two are two
agents of one role (3, 193)."

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
with the run it names, the standing agent that holds it or that it is
carried; or that either could not be read".

**New clause 193**, in A.12 after 132: "A source the host declares (191)
may declare the roles it answers, each at its address on a stream the
host declares (143). Each role runs as one of the source's own kinds,
launched with what the source declares; or a standing agent holds it
(132); or its run is on another host, its stream that host's. The source
also declares the words of what the role answers, which a stage's prompt
gives (28). The host declares the source and its streams and locks its
kinds, and declares nothing of its roles. The source whose kinds run a
role declares it once, at an address for each stream it answers in. A
role a source declares, run or standing, may stand behind its address's
stream: it is that stream's answerer. A launch that names no recipient
and runs in no session has the answerer behind its stream as its
recipient (7). flume delivers every stall of a place to that answerer
besides the place's recipients and the role its stop-short named (130).
So a question that no one collects or answers within the place's stall
bound reaches a run woken, or a standing agent told, to answer it. An
address two sources declare belongs to neither, and so does a stream two
roles stand behind: a launch that would take either is refused naming
both (4). A declaration that cannot be read declares nothing, and so
does an address whose stream the host does not declare where its run is
not on another host; status names each and why (56). flume decides
nothing in this (69, 72). Which roles a source declares, which stands
behind a stream and what its kind or its standing agent does are the
source's. Which questions a run answers itself, passes to another role
of any stream or carries to a person, and how, are the run's. A stream
no role stands behind has no answerer: a launch there names its
recipient or runs in a session and names no address that wakes nothing
(4), and its stalls go to the place's recipients and the role a
stop-short named."

**C.4**, after "Each hook calls the binary that started the agent by its
absolute path, written into the plugin with its version, and never fails
or holds up the agent's start or turn.", add: "In a session flume did not
start, the plugin's SessionStart hook runs `flume inbox` as the session
the terminal names, and gives the agent what it answers as the session's
context, or nothing where nothing is pending or waiting, or no session
is named; it never holds up the start (43)."

**C.5**, the herdr table's standing row: replace "for an address the
host's configuration gives a standing agent (132, C.7)" with "for an
address its source declares a standing agent for (132, 193, C.7)".

**C.7.**

- Replace the bullet that begins "The roles a host wakes are a table
  of the same configuration" and ends "The table is the managed
  layer's.", record 0013's standing agent among it, with:

  > - A host's sources declare the roles it wakes, and no table of its
  >   configuration does (193). A source's file may give `roles`, an
  >   absolute path or one under `~`. That file is the source's own
  >   tool's, written whole beside it and renamed over it; flume reads it
  >   as it stands and never writes it. Each entry is a
  >   `[roles."<stream>/<role>"]` table. It gives `kind`, a name of the
  >   source's own kinds, its version the locks' (154, 155), and
  >   `repository`, by its location on the host as a launch names it. It
  >   gives `priority`, and `base`, the repository's default branch where
  >   it names none. It may also give `brief`, `points` and `intent`. Its
  >   `name` names the run's place, else `<stream>-<role>` does. It may
  >   give `answers`, the words of what the role answers that a stage's
  >   prompt gives (28), and `answerer = true`, where the role stands
  >   behind its address's stream. Under
  >   `[roles."<stream>/<role>".agents]` it gives the run's settings as a
  >   launch's (7). flume launches the run in its address's stream. An
  >   entry may instead give `agent`, the name the host's terminal knows a
  >   standing agent by. Beside it, it may give `session`, the terminal
  >   session the agent lives in, and `answers` and `answerer`. An entry
  >   naming no session finds its agent in the one the host names for its
  >   address's stream (143), in that stream's terminal (190), and the
  >   address's ends go into that agent and wake no run (132). Or an entry
  >   may give `carried = true`, with an optional `answers` and nothing
  >   else, for an address whose run is on another host. A configuration
  >   that gives a `[roles]` table is unreadable, naming it and that a
  >   source declares a role. An entry declares nothing for its address,
  >   and status names it, where it
  >   - names a kind not its source's, an `env`, a pool, an account or a
  >     `stream`;
  >   - names in its address a stream the host does not declare, and is
  >     not carried;
  >   - gives `agent` beside `kind` or a run's other keys; or
  >   - mixes `carried` with a run's keys, `agent` or `answerer`.
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
  that asked, naming its key (42), with the user's own words where its
  role put the question to the user on a card its own code raised".

**S38.** Replace "launches the planner's kind as the host's
configuration names it" with "launches the planner's kind as its source
declares it (193)".

**Clause 193 and the scenarios.** Main at a4eeec3 holds the clauses 186
to 192 and 195 to 208, so the new clause is 193. Main holds the
scenarios S90 to S92, S95 to S97 and S100 to S118, so these four are
S93, S94, S98 and S99. No other branch of agentplot/blueprints takes
any of them.

- **S93.** A script with no terminal session launches a change in the
  stream `east`, naming no recipient, on a host whose source `acme`
  declares runs for `east/operator`, standing behind `east`,
  `east/planner` and `east/ops`, each with its words. The place's
  recipient is `east/operator`. Its construct needs a reading of a
  deployed queue's settings, and stops short naming `ops`, which its
  prompt listed among its own stream's roles. The settle wakes
  `east/ops`, whose run collects the question with its key and the
  place's brief, holds the waiting move, reads the settings and retries
  construct naming the key, within the hour construct's agent is kept
  open. That agent takes the question with the answer in its session
  and carries on under its first start, and status gives the answer on
  the stop-short, from `east/ops`. A retry naming the same key from a
  person's pane a minute later is refused, naming ops's answer and when.
  `east/operator` was woken for none of it (7, 28, 41, 42, 43, 49, 130,
  132, 193, 198).
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
  takes it, holds the waiting move "on the user's card" for a day, and
  puts it to the user once, keyed by the place and the end. A second run
  of `west/design`, woken by another end, reads the same question, sees
  the hold, and asks the user nothing. The day passes with no answer.
  The hold lapses, and the place's stall is delivered to its owner
  `east/planner`, to `west/design` and to `east/operator`, the door of
  the place's own stream, naming the lapsed hold's words. The
  operator's run holds again naming the question already asked, and
  asks nothing (41, 130, 132, 193).
- **S99.** A host declares the streams `east` and `west` and one source,
  `acme`, holding the package `acme/turn@1`, which the host's lock
  records, and naming the file where acme declares its roles. acme's
  file declares `east/planner` and `west/planner`, each running `turn`
  in one repository, named `east` and `west`; gives `east/design` to the
  standing agent `east-design`; and carries `north/planner`, whose
  stream the host does not declare. The host's configuration names no
  role. Ends come at once for `east/planner` and `west/planner`. Two
  places are launched, `turn--east` in the stream `east` and
  `turn--west` in `west`, and each run collects its own address's ends.
  Each run's turn delivers, and flume declares it done and tidies it
  with no caller's word. The next end for `east/planner` launches a new
  run. An end for `east/design` goes into `east-design`'s pane as a
  notice and launches nothing. acme's file then gives `east/design` the
  kind `turn` in place of the agent, and the next end for it launches a
  run, nothing of the host's changed. A launch naming `north/planner` is
  taken, and its end stays pending, status naming the address carried
  (3, 114, 132, 143, 155, 191, 193).

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
    for the run's address; each question held, answered by `crew
    answer`, passed by `crew tell`, or said to be the user's by `crew
    question`, as decision 10 has it.
  - Each brief its part of decision 3's table and decision 10.
  - The operator's brief: the door behind every stall of its stream,
    passing a stalled question once to the role whose it is, of any
    stream, or saying once by `crew question` that it is the user's, on
    the user's word of 2026-11-05, "I think you're referring to an
    operator and that's fine"; the brief's word that the operator asks
    the user nothing gives way.
  - briefs.11: a place's question is put to the user by `crew question`
    alone, never on a card or in the Inbox in a role's own words, on the
    user's word of 2026-11-06: "the whole point was that all of the
    pending U card integration was statically defined in the Python
    code, not done by agents". A role writes in the Inbox only in a
    question's thread, as its address's identity `crew:<stream>-<role>`,
    with its address as `agent_name`. A role's own questions, at most two
    open at a time, are cards crew's code raises on its rail, as its six
    are, each linking a thread in the partition's Inbox where it needs
    elaboration, on the user's word of 2026-11-06: "that's the instruction for you but it's also the instruction for the design of crew".
- **`crew question <place> <key> [--talk]`**, a command of crew's code,
  as 11.2 has it: the question read from flume, one card built from its
  facts in `cards.py` and raised once under its key in the role's name,
  and, with `--talk`, the place's thread, the role's or an
  elaboration's, linked from the card. Record 0031 and briefs.13 gain
  the question's card beside the six, built from facts as the six are.
- **`crew _answered`**: a question's card's answer written as the
  request `Act: retry` with `Answers:` and the user's words, and no ask
  of a role; a message on the card, and every other card, as today.
- **The follower** (`crew _follow`): at its minute's read, the threads
  of each role on its host that runs as its kind, read as that role's
  identity in that role's partition's Inbox, each thread `unread` an ask
  of the role; and each question's card whose place no longer waits on
  its key closed, its thread resolved and its elaboration finished, as
  decision 10 has it. Its Pending You notice, which runs `crew
  _answered`, stays as it is.
- **The teams file**: for each partition, how its Inbox is reached, once
  swancloud's design gives each partition its Inbox, and none until
  then, deneb's one Inbox; and the elaboration kind the partition names
  for a question's talk, none until a host holds one.
- **`crew answer`** carries the key, `Answers:`, takes any place on the
  host by its place's name, and is open to the operator
  (`REQUESTERS`).
- **Record 0049**: an amendment naming flume record 0039. A role is
  woken by flume from crew's own declaration of it, at its address in
  its partition's stream. A card's answer on Pending You reaches its
  role as the `card` ask `crew _answered` writes, a question's card's
  answer reaches its place as the retry crew's code writes, and the
  follower writes a reply in a role's thread as an ask; the runner
  carries each ask to flume by `flume wake`, where the record had
  Pending You's wake mod send it.
- **The change `roles-run-as-flume-kinds`'s word of 2026-10-31**, "crew
  reads nothing of Flume's but what the runner carries back", gains the
  user's word of 2026-11-05 for a role's run, "Yes: a role's run reads
  Flume's list for its address", and `crew question`'s read of the
  place's question.

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
needs no change of it to run: a place's question reaches the user, and
its answer comes back, through Pending You and its notice. Two asks,
put to whoever keeps it on the host, would close what decision 10 works
around for a role's thread:

- **A watch**: a command the Inbox runs for each reply to a session no
  host wakes, `plannotator inbox watch [--session <id>] -- <command>`,
  as Pending You's notice runs one for a card, so crew's follower
  carries a reply when it is given rather than at its next minute.
- **An identity for an address**: a name a caller gives beside its own
  session, which a reply is addressed to and every later caller giving
  it reads as its own, never read as ended and never handed by the
  person to a session, so a role's run no longer drops its session's
  `PLANNOTATOR_SESSION_TAG` to write as its address.

#### 12.6 The host's configuration

No clause changes for it, since 32 and 195 already layer a kind's own
settings. A host that runs an elaboration kind gives that kind's agents
the Inbox's and Pending You's tools and their instructions in its
layers for that kind, and gives them to no construction kind's agents,
so no construction agent is told to write a card or an Inbox message.
Whoever writes the host's configuration writes it so: on the hosts
swancloud configures, its fix
`a-places-agent-asks-through-flume-never-the-inbox` is that part.

### 13. Moving each partition's roles onto their kinds, in order

Each move is the user's word, one role at a time (crew's roles.2 and
Decision 3): the teams file says the role runs as its kind, crew's roles
file trades the address's standing agent for its run, and crew ends the
standing agent at its next stop. Nothing of the host's changes at a
move.
The order is the user's, of 2026-11-05: "That order; planner and design
agent wait for an effort setting".

1. **Each stream's operator, with flume's release.** agentplot's,
   madswan's and swancloud's operators move, each standing behind its
   stream. crew's file may declare an operator behind its stream as its
   standing agent until the move, so a launch from a shell into its
   stream takes a door either way, and none is refused for want of one.
   It runs at its account's effort until flume gives one. `crew
   question` and the question's card come with it, since the door may
   say a stalled question is the user's.
2. **Each stream's ops.** It answers questions about live systems, and
   its fixes of main name it. It runs at its account's effort until
   flume gives one.
3. **Each stream's reviewer.** It answers no question.
4. **Each stream's planner**, then 5. **its design agent**, once flume
   gives an agent's effort as a setting of its layers (197), which
   flume's change `an-agents-effort-is-a-setting-of-flumes-layers`
   builds, and
   crew's roles file gives each its extra-high effort in `agents`.
   Until they move, crew's file declares each as its standing agent
   with its words, so an agent addresses it by `--to` and its ends go
   into its pane, and the runner's places naming the planner keep
   working.

Each step after the first takes agentplot first, since the
orchestrator's work runs in its stream, then madswan and swancloud.
wldn's main level runs on chuck-herdr-alpha, which still runs crew's
team path, and moves when the user moves that host.

### 14. Order across the repositories

1. **The user's read** of this design.
2. **agentplot/blueprints.** flume's statement amended on main as 12.1
   words it (core.19).
3. **This repository.** The change `a-host-declares-where-its-kinds-come-from`
   landed, since a source's declaration is read by it, and record
   0046's change `an-answer-reaches-the-open-session-within-an-hour`,
   since an answer handed to an agent kept open is recorded where this
   change's key takes it once; then this change's build, released.
4. **afterthought/crew.** Its kinds as packages declaring `planning`,
   its roles file at its streams' addresses, the turn, `crew question`
   and the question's card, `crew _answered`'s retry, the follower's
   read of the roles' threads and its closing of answered cards, the
   briefs, `crew answer` and the records of 12.3.
5. **agentplot/boomdrive.** The wakes and the retry's key of 12.4.
6. **The host**, in one deploy with decision 13's first step and
   flume's release: crew's source declared, crew's kinds locked, its
   streams `agentplot`, `madswan` and `swancloud` with `planning`'s
   reserve in each, and no construction kind's agents given the Inbox's
   or Pending You's tools (12.6).
7. **The later moves**, one role at a time, as decision 13 orders them,
   each crew's alone; the planner's and the design agent's once flume's
   change `an-agents-effort-is-a-setting-of-flumes-layers` is released.

Steps 4 and 5 work against the released flume and need nothing of each
other. A retry with no key is taken as today, and the runner's launches
keep working throughout. swancloud's design
`the-plannotator-inbox-runs-on-deneb-for-the-users-devices`, which owns
where each partition's Inbox runs, needs none of these steps, nor they
it: a role's thread is in deneb's one Inbox until that design gives
each partition its own (decision 10, *Which Inbox*).

## Alternatives considered

1. **Each caller collects for itself** (record 0039's option 1). This is
   today. A shell's places are seen only by a poller, and a poller that
   is down loses the night. Record 0009 already refused polling for the
   load it puts on a host.
2. **Refuse a launch with no recipient, nothing more** (option 2). It
   closes the first gap only.
3. **This design** (option 3, as the user's rulings of 2026-11-05 and
   their answers of 2026-11-06 shape it).
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
11. **A place's question put to the user by the role's run, in the
    Inbox, as its address.** An agent would write what the user is
    asked, which the user ruled is code's, and Pending You, where the
    user answers, would not hold it (decision 10).
12. **The asking agent asks the user itself**, on a card or in the
    Inbox. No answerer of the place sees the question, and the user's
    reply reaches a session flume ends (decision 10).

## Where the statements and the code disagree

- **Clause 43 says a caller that starts collects what is pending for
  it.** flume's plugin does nothing at the start of a session flume did
  not start (`hook.rs`, `hook`). sluice's plugin collects (51, 135).
  Decision 9 brings flume's code to its clause.
- **Clause 132 has a role's run launched by flume and never ended.** Its
  kind lands nothing, and nothing gives the `flume done` that 114 waits
  for. crew's runner did it. Decision 2 makes it flume's.
- **Clause 132, as amended, declares every role's run done at its last
  stage's delivery**, and 114 refuses a done on a place whose kind
  lands. Decision 2 submits a run whose kind lands as any place's, and
  the exception is written for 132 on agentplot/blueprints' branch
  `design/flume-a-roles-run-whose-kind-lands`.
- **Clause 132 and C.7 give a role's entry points**, and 165 is not
  built. Decision 2 refuses an entry that gives `points` until it is.
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
  door says once by `crew question` that a stalled question is the
  user's, on the user's word of 2026-11-05 (decision 10), and 12.3
  amends the brief.
- **The user's words of 2026-11-06 that crew's card integration is
  code, not agents', and that this is the instruction for crew's design
  too, and crew's `cards.py`**, which has a role's own question go
  through Pending You's tools in its session. 12.3 brings a role's own
  question onto crew's code, on its rail.
- **Clause 41 has a stage's agent ask by its stop-short**, and
  Plannotator's prompt hook tells every session on deneb, a flume
  stage's agent among them, to send a decision to the Inbox. Decision 4
  says so in the stop-short's part, and 12.6 keeps the hook out of a
  construction kind's settings.
- **flume's CLAUDE.md says each tool's statement is in
  agentplot/blueprints.** Boomdrive has none there. Its records and
  specs stand for one, so 12.4 is written against them.
- **Section 9's command line** names no `hold`, `wait`, `inbox` or
  `wake`, though each is built and specified. 12.1 adds them.

## Found beside

- **An answer's way back runs through the runner.** A question reaches
  its role through flume alone. The answer reaches flume by `crew
  answer`, or by `crew _answered` for the user's card, at the runner's
  next pass, and a reply in a role's thread wakes its role at the
  follower's minute and the runner's next pass. On deneb the runner is
  the launchd service `swancloud.boomdrive`, which keeps it running.
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
  and their questions go into their standing agents' panes until then.
  The
  operator, ops and the reviewer run at their account's effort until it
  is (decision 13).
- **A session's start costs a flume call.** Every session the user
  starts runs `flume inbox` under a 20 seconds' bound, and prints
  nothing when nothing waits. The inbox settles every place first while
  no caller keeps watch. Where Claude Code ends the hook after the inbox
  collected and before it printed, the ends it collected stand
  delivered by the inbox and that session is not told them; the places
  that wait on it survive, since `waiting` marks nothing, and the next
  `flume inbox` gives them.
- **A hold's bound is a timer read at settles.** Under a terminal that
  keeps no watch, a hold lapses only at the next acting command (119).
  Under herdr, the watch reads it within its bound (122).
- **An answer's way back runs through the runner.** `crew answer` and
  the user's answer on a card wait on the runner's next pass, and a
  reply in a role's thread on the follower's minute besides, since
  Plannotator offers no watch. An answer that comes within the asking
  agent's time kept open reaches its session; one later starts a fresh
  agent with the question and the answer (42, 198).
- **An Inbox identity Plannotator does not promise.** A role writes in
  its thread as its address by dropping its run's
  `PLANNOTATOR_SESSION_TAG`, which the pinned Plannotator takes and a
  later one may refuse. A refused call leaves the thread unwritten. The
  card stands, and the user answers on it. crew's code is checked
  against the Plannotator the host pins.
- **A question's talk is slow until a host holds an elaboration kind.**
  A role's thread is answered at the follower's minute and a run's
  start, by no agent there as the user writes. The card is the answer's
  one place either way.
- **crew's code reads flume's facts.** `crew question` reads the
  question from flume's JSON. A field it cannot read refuses the raise,
  naming it, and the hold's bound brings the question to the door, so a
  question is never put to the user in words crew's code guessed.

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
- **A host's `[roles]` table**, its runs' entries and its standing
  agents' alike, leaves its configuration. A host with one has it
  refused naming the table, and moves its entries into the roles file
  of the source whose roles they are, or of a source of its own, at
  addresses of its streams.
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
    partition's roles at its stream's addresses, a run's entry for each
    role that runs as its kind and a standing agent's for each not yet
    moved, each operator standing behind its stream, and wldn's roles
    carried.
- **The orchestrator** stops its status script. Its launches name no
  recipient and take the `agentplot` door.
- **A role's thread**, once swancloud's design gives each partition its
  Inbox, is in that partition's. A thread opened before stays in
  deneb's one Inbox, and crew's follower reads each role's identity
  there too until no thread of it there is open, so a talk goes on where
  it began.
- **Questions waiting when the release lands**, asked of no one or put
  to the user by hand, reach the door at their stall. The door says by
  `crew question` which are the user's, and each gets its one card.
- **Rollback** is the earlier release. The fields this change adds, an
  answer on an end, a hold on the waiting move, a stop-short's
  addressee, are fields an earlier flume ignores (core.14). An earlier
  flume reads a source's `roles` as a key it does not know, a fault of
  that source's alone (191), so crew's roles go quiet with the rollback,
  its questions reach no door, and every other launch runs on.

## Open Questions

None. The parts the constructing agent drew from the user's words are
put to them in record 0039, proposed until they read them in this
design.

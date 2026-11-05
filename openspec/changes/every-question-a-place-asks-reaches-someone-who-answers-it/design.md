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
`<label>/<role>` whenever something is pending for it. No partition
has moved a role onto its kind yet. This design takes that as its base
and finishes it across the family, so that every question reaches the
role it is for and, past a bound, the partition's door:

1. **Every main-level role answers as a Flume kind at its own
   address.** flume launches a role's run, from the host's `[roles]`
   entry for its address, on an end pending for it or on a carrier's
   `flume wake`. The runner carries crew's own events to flume by
   `flume wake`, and launches no role's run. A run is named per address,
   so two partitions' planners are two places, and it is over once its
   turn delivers.
2. **A question goes to the role whose it is.** A place's owner is the
   role the launch names: the planner for a bolt's work, ops for a fix of
   main, a role for its own landing work. The asking agent may address
   its question to another role of the same partition instead,
   `flume needs --to madswan/ops`. flume lists those roles, with what
   each answers, in every stage's prompt. A role that takes a question
   not its own passes it on.
3. **The door stands behind every stall.** The host's answerer, and each
   stream's, is the partition's operator, its door. A place waiting
   under nobody's words past its stall bound is delivered stalled to its
   owner, to the role its question named, and to the door. A role that
   is on a question says so with a hold on it, whose bound, once passed,
   stalls the place again. A launch from a shell takes the door as its
   recipient.
4. **An answer is taken once.** Every end has a key. A retry that names
   the key of the stop-short it answers is refused once the place no
   longer waits on it.
5. **A restarted recipient sees what waits on it.** The inbox gives
   every place waiting on its recipient. flume's plugin gives it to a
   person's session at its start, and every role's run collects it at
   its start. A notice names the place, its brief and the end's key.
6. **A question that is the user's becomes one card**, made by the
   role whose question it is, or by the door at a stall, in its own
   name, keyed by the place and the end. The card's answer comes back
   through crew's follower and the runner's `flume wake` to that role's
   next run, which retries with the key.

`flume serve`, a session serving an address, is left out. Once roles
run as kinds, only a standing role would need it, and standing roles
are what each move retires (decision 10.5).

sluice's statement needs no amendment. crew's rules, briefs and
`crew answer`, and Boomdrive's role offers and retry, change with this
in their own repositories (decision 11). Moving each partition's roles
onto their kinds is a step of its own, in an order the user approves
(decision 12).

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
- **Waking a role.** A settle that finds an end pending for
  `<partition>/<role>` with no run standing launches the kind the
  host's `[roles]` table names for it, and a carrier may ask for the
  same by `flume wake`. The run is named by the role alone,
  `<kind>--<role>` (`wake.rs`; `flume-seams/src/host.rs`, `role_of`).
  The table gives the run's repository, kind, priority, base, brief and
  stream, and nothing of its agents' settings or its intent. A run's
  kind lands nothing, so once its stage delivers it waits on a caller's
  `flume done` that nothing gives (114): a run never ends of itself.
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

- **The roles as they run.** Every partition's `flume` in the teams file
  names each of the five roles `false`. `crew main up` starts the design
  agent, the planner, ops and the operator as standing Claude Code
  sessions in herdr panes, outside flume, and the reviewer fresh for
  each batch. The teams file on deneb names three partitions: madswan
  and swancloud, whose main levels run on deneb, and wldn, whose main
  level runs on chuck-herdr-alpha.
- **Which question is whose, in the briefs** (`plugin/roles/`).
  - The planner: "Its places' questions are yours." Every place crew
    launches for a bolt names `<label>/planner`. The planner gets the
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
  delivers a report in the place's folder. A turn starts with
  `crew role start`, which takes everything pending for the role, the
  asks in `asks.rec` and the places the runner's records show waiting on
  the role's address, and ends with `crew role end`. A tell, a send, a
  capture, a landing, a card's answer, a batch and `crew role open` to a
  role that runs as its kind are each an ask. A turn the user opened
  keeps the conversation in its pane until they are done (Decision 13).
  The runner launches the run (Decisions 1 and 6).
- **Why the runner, not flume, launches a role's run.** The change's
  Decision 1 set flume's `[roles]` aside because flume names a run by
  the role alone, so madswan's and swancloud's planners, whose first
  blueprints repository is the same, would be one place, and because
  "the user chose the runner as the launcher". Rule roles.7, from record
  0049, says the other: "a role with no live run is woken by Flume,
  never by a process of crew's". The change departed from it.
- **Answers.** `crew answer <work> "<words>"` appends an `Act: retry`
  request for the runner to carry, from the planner, the design agent or
  main-level ops alone (`REQUESTERS`). It refuses a place crew did not
  launch.
- **Cards.** A role asks the user on Pending You, in its own name, at
  most two questions at a time (briefs.11; records 0005, 0031). With a
  role running as its kind, crew's follower runs `pendingyou watch`, and
  `crew _answered` writes a `card` ask of the role for each change on a
  card in its name.

**The Boomdrive runner** (agentplot/boomdrive at 57145f6).

- **Launching.** It launches crew's work with `flume --json launch`,
  one `--recipient` per `Recipient:` of crew's request, and
  `$FLUME_RECIPIENT` removed from its environment (core.15 there).
- **Ends.** It never calls `flume inbox`, `wait` or `wake`. Each pass it
  reads `flume --json status` and copies a place's status, whole, to
  crew's `<label>/runner/<host>:places.rec` when it changes.
- **Answers.** It carries crew's retry requests into `flume retry`.
- **Role runs** (`crates/boomdrive-crew/src/roles.rs`). For a role
  record naming its host, it offers a launch of the role's run, kind
  `crew/crew-<role>`, name the label, stream, priority, intent and model
  from the record, while an ask or a place of any host's waiting on the
  role is pending and no run stands. It declares the run done once its
  turn delivered.
- **How it runs.** It is a loop, `boomdrive run`. On deneb it is run by
  hand, still as `flywheel` 0.0.0, with no service keeping it alive. Its
  last pass was 00:28Z on 2026-11-05. While it is down no answer of
  crew's reaches flume, and crew's copy of status goes stale.

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

**The user's queues.**

- **Pending You** is a hosted card service. A script posts through
  `pendingyou mcp`, under the host's sign-in, with `post_request`. A
  repeated `idempotencyKey` never makes a second card. `get_request`
  reads the answer, `ack_answer` closes the card and `cancel_request`
  withdraws it. Its wake mod starts a turn only in the session that
  posted the card. `pendingyou watch -- <command>` runs a command each
  time an answer is ready.
- **The Plannotator Inbox** is local and per project. `:::question`
  blocks are its questions, and a repeated `idempotency_key` returns the
  first message. A reply goes only to the session that sent the message.
  A later run can read the thread, but nothing wakes it.

**The orchestrator.** A Claude Code session on deneb launches the
host's agentplot work, which runs in the `madswan` stream. It runs
`env -u HERDR_ENV -u HERDR_PANE_ID -u HERDR_SOCKET flume launch …` with
no `--recipient`, so its places have none. A script of its own reads
the text of `flume status` every 45 seconds, picks the lines that wait
on their caller, and dedupes them by hand. The session then answers a
question itself or posts a Pending You card under a key it makes up,
and retries.

**deneb's host configuration** declares the streams `madswan`,
`madswan-planning`, `swancloud` and `swancloud-planning`, one pool, and
no `[roles]`.

### Where it breaks

1. **No recipient.** Clause 7 lets a launch have none, and on deneb that
   is most of them. Nothing but a poller sees their ends, and a poller
   that is down sees nothing.
2. **No role runs at its address.** Every role stands, so the address
   crew's places name wakes nothing, and a standing role sees a question
   only on its board.
3. **Two designs of waking a role.** flume's `[roles]` wake and the
   runner's role offers do one thing. crew chose the runner for a defect
   of flume's: two partitions' runs of one role share a name. And
   flume's run of a role never ends of itself.
4. **One door for every question.** Every place crew launches names the
   planner, so a question about a live system or the design goes
   through the planner, and a question about the orchestrator's work has
   no role at all.
5. **No bound.** A question delivered to a recipient that does not act,
   or pending for one that is gone, waits for ever. The stall is said to
   the same recipients who did not answer.
6. **A restarted recipient.** The notice is one line, which a new agent
   in the pane cannot place. An end delivered just before the restart is
   gone from the inbox. flume's plugin gives a caller's session nothing
   at its start.
7. **Answered twice, or too late.** A retry names no question. Two
   answerers each restart the stage, and an answer that comes after the
   place moved on starts the stage again over later work.
8. **The user's card.** Each answerer makes its card by hand under a key
   of its own. Nothing ties the card to the question, so a second reader
   asks again, and nothing withdraws a card whose question was answered
   some other way.

### Rules and records relied on

- **flume's statement** (agentplot/blueprints a465ba7). 7, 8, 28, 41,
  42, 43, 49, 56, 62, 69, 70, 72, 114, 130, 132 and 190. 69 says flume
  does the same for every caller. 72 says flume tells no person anything
  by itself. 120 says it runs no process of its own.
- **flume's records.**
  - 0009: an end is delivered once inside the settle, a wait is by
    place, and a caller's start collects.
  - 0011: an answer is a retry.
  - 0013: a recipient may be a role's address, woken by flume.
  - 0014: the words delivered, pending, collect and notice.
  - 0022: a kind from a package runs only as a lock records it.
  - The record on the unlanded change
    `an-answer-reaches-the-open-session-within-an-hour`, accepted by the
    user on 2026-11-01: within an hour an answer reaches the asking
    agent's open session. That answer is the same retry this design
    guards with its key.
- **flume's rules.** core.1 (no person's queue named outside a profile),
  core.4 and core.34 (a kind names nothing a host owns), core.11 (a
  notice never sent into a question), core.13 (no process of flume's
  own), core.24, core.25, core.27 and core.28 (holds, delivery, the
  kind running through, and roles), core.30 (an agent's settings from
  their layers).
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

- Every main-level role answers as a Flume kind at its own address, and
  a question reaches the role it is for.
- Every question a place asks reaches a recipient whose run is woken
  for it, or one that collects it, whatever launched the place.
- A question that no role takes up within the place's stall bound
  reaches the partition's door.
- A question is answered once. A second answer, or a late one, starts
  nothing.
- A recipient that restarted sees every question still waiting on it,
  with enough to act on.
- A question that is the user's reaches them as one card, and their
  answer reaches the place as its retry's words with no hand carrying
  it.
- One owner for each part across flume, crew, Boomdrive and sluice.

**Non-Goals:**

- flume deciding which role a question is for, which questions are a
  person's, or telling a person anything itself (69, 72). The asking
  agent, the role and the door decide.
- A person's queue, Pending You, the Plannotator Inbox or any other,
  named in flume's code or its kinds (core.1, core.35).
- A process of flume's that watches cards or waits for answers (120).
- Changing what an answer is. It stays a retry naming the stage with
  the answer as its words, the question before it (42, 49). The open
  session within the hour is the other change's.
- A session standing in for a role's address. Standing roles leave as
  their partitions move them (decision 10.5).
- sluice's own outcomes, which are not questions. They stay sluice's
  (decision 11.2; Found beside).
- Building any of it before the user has read this design.

## Decisions

### 1. Every main-level role answers as a Flume kind at its own address

crew's change `roles-run-as-flume-kinds`, as built on `teamless/deneb`,
is the base. Its five kinds, its turn (`crew role start`, act,
`crew role end`), its asks, its `crew role open` and the teams file's
per-role `flume` stand as crew built them. Each role answers at its
address, `<label>/<role>`: `madswan/design`, `madswan/planner`,
`madswan/ops`, `madswan/reviewer` and `madswan/operator`. On it, this
design sets five things.

**flume launches a role's run.** A settle that finds an end pending for
an address with no run standing, or a carrier's `flume wake <address>`,
launches the run from the host's `[roles]` entry for the address (132).
The runner launches no role's run. It carries crew's own events, an ask
in crew's records and a place on another host waiting on the role, by
`flume wake <address>` on the role's host (the user's ruling of
2026-11-05). crew's follower writes a card's answer as an ask, as it
does, and the runner wakes the role for it.

**A run is named per address.** `[roles."<address>"] name` names the
run's place, and flume names it `<partition>-<role>` where the entry
names none. crew's entries give the label, so madswan's planner runs as
`crew-planner--madswan`, the place crew's `kind_role_of` reads, and
swancloud's as `crew-planner--swancloud`. The host's configuration is
unreadable where two addresses would make one place, one repository,
kind and name, and says which two. A run an earlier flume named by the
role alone is found by its role field, which the run's lookup reads
first (`wake.rs`, `run_of`).

**The entry carries what the runner's launch carried.** Beside
`repository`, `kind`, `priority`, `base`, `brief`, `stream` and
`points`, the entry gives the run's `intent` (147) and its agents'
settings, `[roles."<address>".agents]`, the run's model among them,
which stand as the launch's settings in their layers (32; core.30). The
kind is crew's package, `crew/crew-<role>`, which the host's lock
records (core.34). The entry's `answers` gives, in the configurer's
words, what the role answers, for the stop-short part of every prompt
(decision 3).

**A run is over once its turn delivers.** A role's run lands nothing,
and nothing is its caller. flume declares it done at the settle that
reads its last stage delivered, with no caller's word, and tidies it
(114). The next end pending for the address wakes another run, whose
start takes what came since (crew's Decision 7). This is the runner's
R3, done where the run is launched.

**A run's own stall goes to the door.** A role's run has no recipient.
Where it waits on its caller, lost, never begun or stopped short, its
stall goes to the answerer behind its stream, and never to its own
address, so the door learns that a role cannot run (decision 5).

Rejected:

- **The runner as the launcher**, crew's Decision 1. Two launchers of
  one thing, and a question's wake waits on a loop that deneb runs by
  hand. The user ruled flume the launcher on 2026-11-05.
- **A session that serves an address**, so standing roles are reached
  by their addresses. It only bridges the roles that have not moved
  (decision 10.5).

### 2. Which needs are whose

From crew's briefs, a need is the role's whose work it is:

| the need | whose | the brief that says so |
|---|---|---|
| A decision the design has not made: how something should behave, a spec silent or at odds with the code, a rule's reading, a record to write | the design agent | design.md, *Answering a design question* |
| The work's scope: work outside the bolt's goal, which bolt a finding belongs in, a unit, chore or fix to add, the order of work, and work found that a stage before must do again (`--found`, 131) | the planner | planner.md, "Its places' questions are yours", "Work outside a bolt's goal" |
| A live system: a reading of what is deployed, a credential, a host's tools, a deploy, a landing that failed, main red | main-level ops | design.md, "from `{{MAIN_OPS}}` for a live system"; main-ops.md, a fix of main |
| A choice that is the user's alone: their name, money, plans, anything public or hard to undo | the user, on one card made by the role whose subject it is | design.md, *Decision records*; capture's rule |
| None of these, or the agent cannot tell | the place's owner, the role its launch names | — |
| A question that no role took up within the bound | the operator, the partition's door | operator.md, "the partition's door", "make sure the user sees a wait that has none" |

The reviewer is asked nothing: it reads one batch and writes one file.

### 3. Who picks

Four could pick the role a question goes to.

- **The launch.** The caller knows whom the work is for. crew names the
  planner for a bolt's work, ops for a fix of main, and a role for its
  own landing work (`crew role land`). The launch cannot know which
  question a stage will ask.
- **The asking agent.** It alone knows what it needs. It holds the
  change, the rules and the brief, and can tell a design decision from a
  reading of a live system from a scope question. It does not know the
  partition's roles unless it is told them, and it may pick wrong.
- **The kind.** A kind names nothing a host owns, an address among them
  (core.4, core.34, 158). Its stage cannot know its question in advance.
- **A door, crew's operator, for every question.** It routes by the same
  table the asking agent can read. It costs a woken run and a hop on
  every question, and its brief has it carry and never answer.

The design takes three of them, each where it knows most:

- **The launch names the owner.** A place's recipients are its owner, the
  role its launch names, or the door where it names none (decision 4).
- **The asking agent may address one question to another role**:
  `flume needs [--found] --to <address> "<words>"`. The stop-short part
  of every stage's prompt lists each address the agent may name, with
  the words the host's configuration gives for what it answers: every
  address the configuration names a run for, or carries (decision
  10.4), in the partition of the place's recipients, else of the
  answerer behind its stream. The stop-short's end goes to that address
  alone, and not to the place's recipients. A name outside the list is
  refused, naming the list, so the agent can ask again. With no `--to`,
  the end goes to the place's recipients. The list is read from the
  configuration at each start, and comes after every part of the prompt
  that a stage's repetitions share (core.31).
- **A role that takes a question not its own passes it.** It tells the
  role whose it is, by `crew tell`, which is an ask for a role that runs
  as its kind, with the place, the key and the words, and holds the
  waiting move, "passed to madswan/design", with a bound (decision 6).
- **The door stands behind every stall**, the last to pick (decision 5).

So "an agent might need something from ops separately from the planner
or the designer" (the user, 2026-11-05) is `flume needs --to
madswan/ops`. A wrong pick costs one pass, which the door would cost on
every question.

A role still standing is not in the list, since the host's
configuration names no run for it. An agent whose question is a
standing role's asks the owner, who carries it to that role as today.

Rejected:

- **Every question to the door first.** Two runs and a hop for each
  question, where most are the owner's.
- **Every question to every role of the partition**, the first to answer
  taking it by its key. Every role woken for every question, and a
  question about a live system read by the design agent.
- **flume reading the words to choose a role.** flume would judge a
  question, which 69 leaves to its callers.

### 4. Who answers each kind of launch

A place's recipients are resolved at its launch, the first of these
that gives any:

1. the recipients the launch names, `--recipient`, one or several, each
   a session's name or a role's address;
2. `$FLUME_RECIPIENT` in the launching command's environment;
3. the session the launching command runs in, as the terminal names it
   (190);
4. the answerer behind the place's stream: the address the
   configuration names for the stream, else for the host (decision 5).

| how the place was launched | its recipients | the roles its agent may address | behind every stall |
|---|---|---|---|
| a shell or a script with no session, naming none, the orchestrator's among them | the stream's door, `madswan/operator` for the `madswan` stream | `madswan/design`, `madswan/planner`, `madswan/ops`, each that runs as its kind | the door is the recipient |
| a herdr pane, by `/launch` or a person's command | that pane's session | the roles of the door's partition | the stream's door |
| the runner, for crew's work | the role crew names: `<label>/planner` for a bolt's work, `<label>/ops` for a fix of main, a role for its own landing work | the partition's roles | the stream's door |
| any launch naming a role's address | that address: its run's pane, its run woken where none is live | the partition's roles | the stream's door |
| a role's run, launched by flume | none | none | the stream's door, else the host's, never the run's own address |

A launch is refused, making nothing (4; core.8):

- where it finds none of the four, naming each and how to give one;
- where it names a role's address the configuration names no run for and
  does not carry, and no answerer stands behind its stream, naming the
  address. Where an answerer stands behind it, the launch is taken: the
  address's ends stay pending, status says it wakes nothing, and its
  questions reach the door at their stall. So a partition moves its
  roles one at a time, and the runner's launches naming a planner that
  still stands keep working;
- where it would take an answerer the configuration names no run for and
  does not carry, naming the answerer and its key.

The recipients are fixed at the launch (8). The answerer behind them is
read at each stall.

The orchestrator strips herdr's variables so that its places are not
its pane's. Its launches take the `madswan` stream's door, and their
agents address the design agent, the planner or ops by `--to`. It needs
no script to poll status.

### 5. The bound: a stall reaches the owner, the asked role and the door

The host's configuration names the answerer behind its places:

```toml
[host]
answerer = "madswan/operator"

[streams.swancloud]
answerer = "swancloud/operator"   # comes first for the places that run in swancloud
```

The answerer is a role's address the configuration names a run for, or
carries (decision 10.4). It is one address, the host's last word on
who stands behind a place, and one per stream at most. A host whose
partitions have a door names each partition's operator for that
partition's streams. A host that names no answerer has none: there
every launch names its recipient or runs in a session, names no address
that wakes nothing, and the stalls go to the place's recipients alone.
The answerer is read from the configuration as it stands when asked, at
each launch and at each stall, and never kept, as the stream and the
pool are (143).

The place's stall bound, five minutes unless the launch sets another
(130), is the one bound. Once a place has waited on its caller that
long under no hold's words, it is stalled, and its stall is delivered
once to each of these:

- each recipient of the place, its owner;
- the role the stop-short it waits on named by `--to`;
- the answerer behind its stream, unless it is one of those, and never a
  role's run's own address.

The stall carries the move the place waits on, and where that move
answers a stop-short, the stop-short's words, whether it needs an answer
or found work, its key and whom it named, and the words of a hold that
lapsed. A reader who never saw the place has the question whole. It is
delivered as any end is: into its run's pane, or pending and waking the
run (43, 132).

A hold's bound passing stalls the place again, a new stall with a key
of its own, delivered to the same three. So a question is never left
under nobody's words longer than the stall bound before its owner and
the door know, and a role that took it and let it lie is found out at
its hold's bound.

**When the role does not answer.**

| what happened | what the bound does |
|---|---|
| The role's run was never launched or never began: no account, no room in its pool, a refusal | Its ends stay pending, and status says why the run was not woken. At the stall the owner and the door have the question. |
| The run took the question and its turn ended with neither an answer nor a hold | At the stall the owner and the door have it. |
| The run held it and let the hold lie | At the hold's bound the place stalls again, to the owner, the asked role and the door, with the hold's words. |
| The role still stands, and the configuration names no run for its address | The end stays pending. At the stall the door has it, and carries it to the standing role as crew does today, or to the user. |
| The door's own run cannot run | Its run's stall goes to the host's answerer where that is another address. Where the door is the host's answerer, its failure stands in status under the host's addresses (Risks). |

An end of a stall recorded before this change names no answerer. The
first settle after the build reads it as the answerer's too, where the
place still waits under no hold's words, so the questions waiting on
deneb when the build lands reach the door at once.

Rejected:

- **A second bound for the door, longer than the stall.** Another knob
  on every host, and the stall already says the place waits on its
  caller with nobody's words. A launch that wants more room gives
  `--stall-after`.
- **The door only for ends no recipient collected.** That misses a role
  that took the question and did nothing, which is what the user is
  asking about.
- **Every end to the door at once.** Two answerers from the first
  minute, and the role whose question it is loses its turn.
- **A chain of answerers ordered by flume.** flume would order roles,
  which is the host's and the kinds' to decide. The owner, the asked
  role and one door, with holds that lapse, are enough.

### 6. A hold on the waiting move

A caller may hold the move the place waits on, with words saying who
makes it and how, and an optional bound:

```sh
flume hold <place> "madswan-planner has it" --waiting --for 1h
flume hold <place> "passed to madswan/design" --waiting --for 1h
flume hold <place> "asked the user on card req_4d00…" --waiting --for 1d
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
delivered as decision 5 has it. Where it gives none, it stands until the
move is made, as every hold does. It is refused, naming why, on a place
that waits on no move of its caller's.

Rejected:

- **A command of its own, `flume claim`.** It would be a second kind of
  words on a waiting place, beside the hold's, with the same effect on
  the stall.
- **A hold implied by an inbox's read.** A read marks nothing (43), and
  a role's run that collects a question has not yet said it will
  answer it.

### 7. An answer is taken once, by its key

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

### 8. What a restarted recipient sees

**The inbox gives what waits on it.** `flume inbox`'s answer keeps its
`ends`, every end pending for the recipient, each delivered by the
inbox. It gains `waiting`: every place on the host that waits on its
caller and that this recipient is a recipient of, was named by the
stop-short it waits on, or was given a stall of as the answerer. Each is
given with:

- the place, its kind, its name and the first line of its brief;
- the move it waits on, and since when;
- where the move answers a stop-short, its words, whether it needs an
  answer or found work, its key and whom it named;
- a hold on the move, with its words, since when and its bound.

`waiting` delivers and marks nothing (43; core.25), so a recipient that
collected a question, then restarted, still finds it.

**A role's run** collects at its start: `crew role start` for crew's
asks, then `flume inbox --json` for its address, which takes the role
from `FLUME_PLACE` (132). Every question waiting on the role is in it,
with its key, however many runs came and went.

**A person's session.** flume's Claude Code plugin's SessionStart hook,
in a session flume did not start, now acts:

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

### 9. The answering run's protocol, and the user's one card

flume's part ends at the role: it reaches the role, gives it the
question with its key, takes the hold that says the role is on it, and
takes the answer once. What the role does is its kind's. The design
gives the protocol each answering role's run follows, for crew's turn
and briefs, and for any host's own answering kind:

1. **Collect.** `crew role start`, then `flume inbox --json`, reading
   `ends` and `waiting`.
2. **Hold what it takes**, before anything else: `flume hold <place>
   "<run> has it" --waiting --for 1h`, so a turn that runs long does not
   stall the place.
3. **Answer what is its own**: `crew answer <work or place> --answers
   <key> "<answer>"`, which the runner carries as `flume retry …
   --answers <key>`.
4. **Pass what is another role's**: `crew tell <label>-<role>` with the
   place, the key and the words, and hold "passed to <address>" with a
   bound.
5. **Card what is the user's.**
   - **Hold first**: "asked the user on card <id>", with a bound, so a
     second reader sees the question taken before the card exists.
   - **One card**, in the role's own Pending You name, under the key
     `flume/<host>/<digest of the place's location>/<end's key>`. A
     second run, a second role or the same run after a restart gets the
     first card back rather than a second (Pending You's
     `idempotencyKey`). The card gives the question whole, the place
     and its brief.
   - **The answer comes back** by crew's path: Pending You's notice runs
     `crew _answered`, which writes a `card` ask of the role; the runner
     wakes the role with `flume wake <address>`; its run reads the
     card's answer in its own name at its start and retries with the
     key and the user's words, so the fresh agent's prompt carries the
     question with the answer (42, 49). It then closes the card.
6. **Withdraw a card whose question no longer waits**: a retry refused
   for its key, a question answered in a pane, a place dropped.
7. **The door, on a stall.** The operator's run reads the stall's words
   and any lapsed hold:
   - nobody had the question: it passes it once to the role whose it is,
     by decision 2's table, and holds with a bound;
   - a role had it and its hold lapsed, or the question is the user's:
     it puts it to the user on one card under the same key, or, where
     the hold names a card the user has not answered, holds again naming
     that card. It never makes a second card for a question.

**Never asked twice** is then held at four points:

- the hold, which keeps the question from the door while its role works
  on it;
- the card's key, which keeps a second card from being made;
- the answer's key, which keeps a second answer from starting anything;
- the withdrawal, which takes down a card answered elsewhere.

**Which queue.** Pending You for a role's run, as crew already asks: a
run ends and is woken again by its address, and Pending You's notice
reaches crew's follower for any card on the host. The Plannotator Inbox
answers only the session that asked, so a run that ended is never woken
by it. The Inbox suits a person's own session that stays, asking about
a place of its own after holding the move.

Rejected:

- **flume makes the card, through a seam for a person's queue.** It
  breaks 72 and core.1, and flume would judge which questions are a
  person's.
- **The card made by whoever first reads the question, with no hold.**
  That is today's orchestrator, and two readers make two cards.

### 10. What a role's run needs from flume, crew and the runner

#### 10.1 From flume

- **Whole-address names**: `[roles."<address>"] name`, else
  `<partition>-<role>`, and a configuration giving two addresses one
  place refused (decision 1).
- **flume as the launcher**: `[roles]` with `intent` and `agents` beside
  its keys, the kind a locked package, and the run's settings in their
  layers (decision 1).
- **A run over at its delivery**, done and tidied by flume (decision 1).
- **The bound**: the stall to the owner, the asked role and the door, a
  run's own stall to the door, the hold on the waiting move with a bound
  that lapses (decisions 5, 6).
- **The answer's key**: `--answers <key>`, taken once (decision 7).
- **Routing**: `flume needs --to <address>`, and the roles listed in the
  stop-short part of every prompt with the configuration's `answers`
  (decision 3).
- **What waits**: the inbox's `waiting`, the notice's brief and key, the
  plugin's SessionStart for a person's session (decision 8).
- **A carrier's wake**: `flume wake <address>`, as built (132).

#### 10.2 From crew

- **The teams file's `flume`** names a stream for each role that runs as
  its kind, as built. A role moves on the user's word (decision 12).
- **Its kinds as a package**, `crew/crew-<role>`, carried by its
  release and locked on each host by `flume kinds lock`, as the runner
  already names them.
- **The turn** (`plugin/flume/turn.md`): collect flume's inbox after
  `crew role start`, and follow decision 9's protocol.
- **`crew answer`** carries the key as `Answers:`, takes a place on the
  host that crew did not launch by its place's name, and is open to the
  operator beside the planner, the design agent and ops.
- **The briefs**: each role's part of decision 2's table and decision
  9's protocol; the operator's, the door behind every stall, which may
  put a stalled question to the user on one card.
- **The records and rules**: roles.7 holds again as written, since flume
  wakes a role. The change `roles-run-as-flume-kinds` Decision 1, and its
  departure from roles.7, give way. Record 0049 part 1 holds, the runner
  carrying crew's events by `flume wake` where it named Pending You's
  wake mod.
- **Where the host's `[roles]` comes from.** The entries are written
  from the teams file by whoever writes the host's flume configuration,
  swancloud on deneb, as it writes the teams file (record 0052).

#### 10.3 From the runner

- **Role offers become wakes.** For each role whose record names its
  host, while crew's records hold an ask pending for the role, or a
  place on another host waits on it, the runner runs `flume wake
  <address>`. It launches no role's run and declares none done.
- **A role's run's status** is carried into crew's records keyed by the
  address, read from flume's status by the place's role, as R4 has it.
- **The retry's key**: a request's `Answers:` is passed as `--answers
  <key>`, and flume's refusal of an answer already taken is the
  request's outcome, refused, in flume's words.
- **A service.** While the runner is down, no answer of crew's reaches
  flume (Found beside).

#### 10.4 An address on another host

crew's records carry a place's question across hosts: a place on one
host waiting on a role whose main level runs on another is read from
that host's runner's records (crew's Decision 5). The place's host
names the address carried: `[roles."<address>"] carried = true`, with
no repository or kind. flume launches nothing for it, takes launches
naming it, and keeps its ends pending for the tool that carries them.
The role's host's runner wakes the run there, and the run answers by
`crew answer`, which the place's host's runner carries to `flume retry`.
Status says the address is carried. deneb has none today: madswan's and
swancloud's work runs on deneb, and wldn's on its own host.

#### 10.5 `flume serve`: no purpose left

`flume serve <address>` would have a session take an address's ends
while it is live. Once roles run as kinds, nothing needs it:

- **A role the user opens to talk** is a run: `crew role open` writes an
  ask, the role's run greets the user in its pane and keeps the
  conversation until they are done (crew's Decision 13). Its pane is the
  run's, reached by the address as any run's.
- **A person's own session** is reached as a session, and collects at
  its start (decision 8).
- **A standing role** is the one thing serving would reach, and standing
  roles are what each partition's moves retire. Until a role moves, its
  questions reach the door at their stall, and the door carries them to
  the standing role as crew does today.

So serving is a stop-gap for the time between moves, and is left out.

#### 10.6 One owner for each part

| part | owner | the others |
|---|---|---|
| A place's recipients, and the answerer behind its stream | flume (7, 191) | the runner passes crew's recipients unchanged (Boomdrive core.15); crew writes the address |
| Which role a question is for | the asking agent, else the place's owner | flume lists the roles; the role passes what is not its own |
| Delivering an end, and keeping it pending | flume (43) | the runner's copy of status is a view for crew's board and its cross-host records, never a delivery; sluice's outcome for a place reaches flume at its settle (C.6) |
| Launching a role's run, and ending it at its delivery | flume (132) | the runner and crew's follower carry crew's events to it by `flume wake` |
| The bound | flume (130, 191) | a role says it is on a question by a hold |
| A session's start collecting | each tool for its own: flume's plugin for flume's ends, sluice's for sluice's | one design, sluice's (sluice 51) |
| Which questions are a person's, and the card | the role whose question it is, or the door at a stall | flume decides nothing (69, 72) |
| Carrying a card's answer to a run | crew's follower, as an ask, and the runner's wake | — |
| Taking an answer once | flume (42, 49) | `crew answer` and the runner carry the key |

### 11. The statements' amendments, for the user to approve

#### 11.1 flume's statement (agentplot/blueprints, `design/flume/requirements.md`, at a465ba7)

**Section 3.**

- Replace **recipient** with: "whoever an end of a place goes to: a
  caller's session, or a role's address, the answerer behind its stream
  among them (43, 132, 191)."
- Add **answerer**: "of a host or a stream, the role's address its
  configuration names to stand behind the places that run there: the
  recipient of a launch that names none and runs in no session, and a
  recipient of every stall of a place that is not its own (191)."
- Add **key**: "of an end, the name flume gives it, which no other end
  of the place has, by which a recipient and an answer name it (42,
  43)."
- In **on hold**, after "before it starts", add: "; of the move a place
  waits on, that a caller said in its words that it is making it
  (130)".

**Clause 4.** Append: "So is a launch that names no recipient and runs
in no session where no answerer stands behind its stream, saying how to
name one (7, 191); one that names a role's address the host's
configuration names no run for and does not carry, where no answerer
stands behind its stream, naming the address (132); and one that would
take an answerer the configuration names no run for and does not carry,
naming it."

**Clause 7.** Replace "A recipient the launch names none of is the
session that launched it, as the terminal it runs in names it, or none
where no terminal names one (190)." with: "A launch that names no
recipient has the session that launched it, as the terminal it runs in
names it (190), else the answerer behind the place's stream (191)."

**Clause 8.** Replace "Its recipients are fixed at its launch." with:
"Its recipients are fixed at its launch; the answerer behind its
stream, as the configuration names it when a stall is read, is
delivered its stalls besides (130, 191)."

**Clause 28.** After "filled in for the place and the stage", add: ",
with each role's address its words may be for and the words the host's
configuration gives for what that role answers (41, 132)".

**Clause 41.** After "(131).", add: "It may name the role its words are
for: an address the host's configuration names a run for, or carries,
in the partition of the place's recipients, else of the answerer behind
its stream (132, 191). The stop-short's end then goes to that address
and not to the place's recipients. A name outside those is refused,
naming them."

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
  found work, or of a failed submission;", add: "the role a stop-short
  named; each end's key, and for a stop-short answered, its answer with
  who gave it and when; the place's recipients, each with where it came
  from;".
- After "a hold's words", add: ", a hold on the waiting move with its
  bound or that it lapsed,".
- After "for the host and for each of its streams,", add: "its answerer;
  for each role's address an end stands pending for, its run, or that it
  is carried or wakes nothing;".

**Clause 130.**

- After "a retry naming the stage or the submission ends the hold (47,
  49).", add: "A caller may also hold the move the place waits on it
  for, with words saying who makes it and how, such as a role that took
  the question or an answer asked of a person on a card, and a bound
  after which the hold lapses; a later hold on that move replaces it,
  and a retry naming the stage or the submission ends it as the move
  made."
- Replace "and each recipient of the place is delivered once that it is
  stalled and which move is theirs (43)." with: "and each recipient of
  the place, the role a stop-short it waits on named (41), and the
  answerer behind its stream where it is none of them, is delivered once
  that it is stalled, which move is theirs, and the words, key and
  addressee of the stop-short it waits on (43, 191). A hold on the
  waiting move whose bound passed is no hold from then, and the place
  stalls again."
- Replace "and flume watches none and ends no hold itself (70, 72)."
  with: "and flume watches none and ends no hold itself, but lapses a
  hold on the waiting move once its bound passes (70, 72)."

**Clause 132.**

- Replace "named by the role, so at most one run of an address is live
  at once (3)" with: "named as the configuration names it for the
  address, else by its partition and its role, so at most one run of an
  address is live at once (3) and two partitions' runs of one role are
  two places".
- After "and its agent takes the address as its session's name.", add:
  "Its settings, stream, priority, intent and points are those the
  configuration names for the address, as a launch's are (7, 32). A run
  is over once its last stage delivers: flume declares it done at the
  settle that reads the delivery, with no caller's word, and tidies it
  (114), so an end that comes later wakes another run. A run's own stall
  goes to the answerer behind its stream and never to its own address
  (130, 191)."
- Replace "An address the configuration names no kind for is woken by
  nothing, and status says the ends pending for it." with: "An address
  the configuration names no kind for is woken by nothing. Where the
  configuration carries it, its run is on another host, and its ends
  stay pending for the tool that carries them there. Otherwise a launch
  naming it is taken only where an answerer stands behind the place's
  stream (4, 191). Status says the ends pending for each address, and
  whether it is woken, carried or wakes nothing."

**New clause 191**, in A.12 after 132: "A host's configuration may name
its answerer, a role's address it names a run for or carries (132), and
a stream may name its own, which comes before the host's for the places
that run in it (143). A launch that names no recipient and runs in no
session has the answerer behind its stream as its recipient (7), and
every stall of a place is delivered to that answerer besides the
place's recipients and the role its stop-short named (130), so a
question that no one collects or answers within the place's stall bound
reaches a run woken to answer it. flume decides nothing in this: which
address stands behind a stream, what its kind does, which questions its
run answers itself, passes to another role or carries to a person, and
how, are the host's and the run's (69, 72). A host whose configuration
names no answerer has none: a launch there names its recipient or runs
in a session and names no address that wakes nothing (4), and its
stalls go to the place's recipients and the role a stop-short named."

**C.4**, after the hooks' table: "In a session flume did not start, the
plugin's SessionStart hook runs `flume inbox` as the session the
terminal names, and gives the agent what it answers as the session's
context, or nothing where nothing is pending or waiting, or no session
is named; it never holds up the start (43)."

**C.7**, the `[roles]` bullet: after "and an optional `brief`, `stream`
and `points`", add: ", `name`, the run's place's name, `<partition>-<role>`
where it gives none, two addresses making one place leaving the
configuration unreadable; `intent`; `answers`, the words of what the role
answers that a stage's prompt gives (28); and, under
`[roles."<partition>/<role>".agents]`, the run's settings as a launch's
(7); or `carried = true` alone, for an address whose run is on another
host". After that bullet, add: "The host's answerer is `[host] answerer =
"<partition>/<role>"`, and a stream's `[streams.<name>] answerer`, read
as the configuration stands at each launch and each stall (191)."

**Section 9**, the command line's list: after "`needs` for an agent;",
add: "`hold`, `wait`, `inbox` and `wake` for its callers;".

**The callers table, crew's row.**

- Replace "it names each place's recipient by a role's address, so a
  role with no live run is woken to collect an end (132)" with: "it
  names each place's recipient by the address of the role the work is
  for, each role running as its kind, so a role with no live run is
  woken to collect an end, and the operator's address stands behind its
  streams (132, 191)".
- Replace "its word to a place is a retry answering a stop-short that
  asked" with: "its word to a place is a retry answering a stop-short
  that asked, naming its key (42)".

**Scenarios.**

- **S92.** A script with no terminal session launches a change in the
  stream `work`, naming no recipient, on a host whose configuration
  names `acme/operator` the stream's answerer and names runs for
  `acme/operator`, `acme/planner` and `acme/ops`. The place's recipient
  is `acme/operator`. Its construct needs a reading of a deployed
  queue's settings, and stops short naming `acme/ops`, one of the three
  its prompt listed. The settle wakes `acme/ops`, whose run collects
  the question with its key and the place's brief, holds the waiting
  move, reads the settings and retries construct naming the key. The
  fresh agent's prompt carries the question with the answer. A retry
  naming the same key from a person's pane a minute later is refused,
  naming ops's answer and when. `acme/operator` was woken for none of
  it (7, 28, 41, 42, 43, 49, 130, 132, 191).
- **S93.** The operator launches a change from a herdr pane on that
  host. Its construct stops short needing an answer, naming no role.
  The notice is delivered into the pane, naming the place, its brief's
  first line and the end's key. The pane's agent is restarted before it
  acts. At its start, flume's plugin gives the new agent the place
  waiting on it, with the question and the key. The agent does nothing
  with it. Five minutes after the stop-short the place is stalled. The
  stall, with the question and its key, is delivered to the pane and to
  `acme/operator`, whose run is woken (43, 130, 191, C.4).
- **S94.** A place's construct stops short naming `acme/design`. The
  design run takes it, holds the waiting move "passed to the user on
  card req_1" for a day, and puts it to the user on one card keyed by
  the place and the end. A second design run, woken by another end,
  reads the same question, sees the hold, and makes no card. The day
  passes with no answer. The hold lapses, and the place's stall is
  delivered to its owner `acme/planner`, to `acme/design` and to
  `acme/operator`, naming the lapsed hold's words. The operator's run
  holds again naming the card, and makes none (41, 130, 191).
- **S95.** Ends come at once for `madswan/planner` and
  `swancloud/planner`, whose runs the host's configuration names in one
  repository and one kind, `crew/crew-planner`, named `madswan` and
  `swancloud`. Two places are launched, `crew-planner--madswan` and
  `crew-planner--swancloud`, and each run collects its own address's
  ends. Each run's turn delivers, and flume declares it done and tidies
  it with no caller's word. The next end for `madswan/planner` launches
  a new run (3, 114, 132).

#### 11.2 sluice's statement (`design/sluice/requirements.md`)

None. A place's question never travels through sluice. A submission's
outcome for a place reaches flume at its settle (C.6), and flume carries
it on as an end, under this design's bound and answerer. sluice's own
delivery to a person's session is already the design this change gives
flume for a session's start (51, 135). Where sluice's carrying differs
from flume's, a push never made again and a push into an asking pane,
is Found beside.

#### 11.3 crew (afterthought/crew: records and rules, crew has no statement)

- **roles.6**: after "a session's start collecting what is pending for
  it", add: "and reading what still waits on it, by each tool's inbox".
- **roles.7** stands as written: "a role with no live run is woken by
  Flume, never by a process of crew's". Its details gain: "Not ruled
  out: the runner carrying crew's events to flume by `flume wake
  <address>`; a place's agent addressing its question to the role whose
  it is, `flume needs --to <label>/<role>`."
- **The change `roles-run-as-flume-kinds`**, a record amending it, as
  the design agent writes one:
  - Decision 1: the kinds reach the host as a package its lock records,
    named by the host's `[roles]`, each entry naming the run's place by
    the label; the runner's launch gives way to flume's.
  - Decision 6: the role record says where the role runs and the runner
    wakes it there; R1 and R3 leave, R2 becomes `flume wake`, R4 stays.
  - Its departure from roles.7 is withdrawn.
- **The turn and the briefs**:
  - `plugin/flume/turn.md`: after `crew role start`, `flume inbox --json`;
    each question held, answered, passed or carded as decision 9 has it.
  - Each brief its part of decision 2's table and decision 9.
  - The operator's brief: the door behind every stall, passing a stalled
    question once to the role whose it is, or putting it to the user on
    one card; this departs from "You post no Pending You card".
  - briefs.11: a card for a place's question is keyed
    `flume/<host>/<digest of the location>/<key>`.
- **`crew answer`** carries the key, `Answers:`, takes any place on the
  host by its place's name, and is open to the operator
  (`REQUESTERS`).
- **Record 0049**: an amendment naming flume record 0039. A role is woken
  by flume; the runner carries crew's events and card answers to it by
  `flume wake`, in place of Pending You's wake mod.

#### 11.4 Boomdrive (agentplot/boomdrive: records and specs, Boomdrive has no statement in blueprints)

- **Role runs**, `crew-control-plane` and `crates/boomdrive-crew/src/roles.rs`:
  a role wanted is woken by `flume wake <address>` on its record's host;
  the runner launches no role's run and declares none done; a role's run
  is read from flume's status by its role and carried into crew's
  records by its address.
- **Retries**, `work-into-flume`: a request's `Answers:` is passed as
  `--answers <key>`, and flume's refusal of an answer already taken is
  recorded as the request's outcome, refused, in flume's words.
- **The status copy**: crew reads it for its board and for a place on
  another host, and no question on the same host depends on it. A record
  says so.

### 12. Moving each partition's roles onto their kinds, in order

Each move is the user's word, one role at a time (crew's roles.2 and
Decision 3): the teams file names a stream for the role, crew ends its
standing agent at its next stop, and the host's configuration names its
run. The order, for the user to approve:

1. **Each partition's operator, with flume's release.** madswan's and
   swancloud's operators move, and deneb's configuration names
   `madswan/operator` the host's answerer and each partition's operator
   its streams' answerer. The door must run before the release reaches
   deneb, since a launch from a shell there takes the answerer, and is
   refused where none stands. The operator's effort is medium, so
   flume's carrying no effort costs it little.
2. **Each partition's ops.** It answers questions about live systems,
   and its fixes of main name it. Its effort is high.
3. **Each partition's reviewer.** It answers no question, and moves
   whenever the user likes. crew's design recommended it first, as the
   least risk.
4. **Each partition's planner**, then 5. **its design agent.** Both run
   at extra-high effort, and flume carries no effort setting, so a run
   starts at its account's. crew's design recommended waiting for one.
   Until they move, an agent cannot address them by `--to`, the
   runner's places naming the planner keep working, and their questions
   reach the door at the stall, which carries them to the standing
   planner or design agent as today.

madswan moves first, since the orchestrator's agentplot work runs in
its stream; swancloud follows each step. wldn's main level runs on
chuck-herdr-alpha, which still runs crew's team path, and moves when the
user moves that host.

### 13. Order across the repositories

1. **The user's read** of this design, and their answers to its Open
   Questions.
2. **agentplot/blueprints.** flume's statement amended on main as 11.1
   words it (core.19).
3. **This repository.** The build, released.
4. **afterthought/crew.** The turn, the briefs, `crew answer` and the
   records of 11.3, and the package lock of its kinds.
5. **agentplot/boomdrive.** The wakes and the retry's key of 11.4.
6. **The host's configuration and teams file**, swancloud's to write,
   with decision 12's first step: the operators move, the answerers and
   their `[roles]` entries are named, crew's package is locked, and
   flume's release reaches deneb in the same deploy.
7. **The later moves**, one role at a time, as decision 12 orders them.

Steps 4 and 5 work against the released flume and need nothing of each
other. A retry with no key is taken as today, and the runner's launches
keep working throughout.

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
   for standing roles, which the moves retire (decision 10.5).
8. **The runner keeps launching role runs**, flume's wake unused. Two
   launchers, and a question's wake on a loop run by hand. The user
   ruled flume the launcher.

## Where the statements and the code disagree

- **Clause 43 says a caller that starts collects what is pending for
  it.** flume's plugin does nothing at the start of a session flume did
  not start (`hook.rs`, `hook`). sluice's plugin collects (51, 135).
  Decision 8 brings flume's code to its clause.
- **Clause 132 has a role's run launched by flume and never ended.** Its
  kind lands nothing, and nothing gives the `flume done` that 114 waits
  for. crew's runner did it. Decision 1 makes it flume's.
- **crew's roles.7 says Flume wakes a role.** crew's change
  `roles-run-as-flume-kinds` has the runner launch role runs instead.
  Decision 1 and 11.3 bring them back together, on the user's ruling.
- **crew's planner brief says a place's question reaches the planner.**
  Nothing tells a standing planner. Decisions 1 and 12 give the planner
  its run at its address.
- **flume's CLAUDE.md says each tool's statement is in
  agentplot/blueprints.** Boomdrive has none there. Its records and
  specs stand for one, so 11.4 is written against them.
- **Section 9's command line** names no `hold`, `wait`, `inbox` or
  `wake`, though each is built and specified. 11.1 adds them.

## Found beside

- **flume carries no effort setting.** crew's planner and design agent
  run at extra-high effort, and their moves wait on one or on the user's
  word to run them at their account's (decision 12). An effort setting,
  layered as the model is, is a change of its own for flume.
- **The open-session record's number.** The record on the unlanded
  change `an-answer-reaches-the-open-session-within-an-hour`, accepted
  on 2026-11-01, is numbered 0028, which main's statechart record took.
  It takes the next free number when it lands. This design's key is
  written to cover its answer (decision 7).
- **The runner on deneb** is still the `flywheel` binary, run by hand,
  with no service keeping it alive. While it is down, no `crew answer`
  reaches flume, and no ask of crew's wakes a role. A question's way to
  its role runs through flume alone; the answer's way back through
  `crew answer`, and a card's answer through the runner's wake, still
  run through the runner.
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
  point to Pending You card `req_4d00977773964d858c61`.

## Risks / Trade-offs

- **A wrong pick.** An agent that addresses the wrong role costs a pass.
  The list gives each role's words, and a role passes what is not its
  own.
- **Asked twice by a pane and a card.** A person's session that asks the
  user in its own conversation without holding the move lets the stall
  reach the door, which may card the user too. The notice, the plugin's
  context at a start and the site all say to hold before asking.
- **Runs woken by stalls.** Each stall the door gets may wake its run,
  one agent session. One run collects a burst (132), and a role's hold
  at its take keeps the door out of the questions roles are working on.
- **The door is the host's last word.** Where the door's own run cannot
  run, its run's stall goes to the host's answerer only where that is
  another partition's door. Where it is the host's answerer itself, its
  failure is in status under the host's addresses, and nothing of
  flume's tells a person (72).
- **Refused launches.** A script that launches with no recipient on a
  host that names no answerer is refused after this change. The refusal
  names the three fixes: `--recipient`, `$FLUME_RECIPIENT`, or
  `[host] answerer`. Every place a refusal now stops is one whose
  questions reached no one before.
- **Effort.** The planner and the design agent wait to move, or run at
  their account's effort (decision 12).
- **A session's start costs a flume call.** Every session the user
  starts runs `flume inbox` under a 20 seconds' bound, and prints
  nothing when nothing waits.
- **A hold's bound is a timer read at settles.** Under a terminal that
  keeps no watch, a hold lapses only at the next acting command (119).
  Under herdr, the watch reads it within its bound (122).
- **An answer's way back runs through the runner.** `crew answer` waits
  on the runner's next pass, and on the runner running.

## Migration Plan

- **Places launched before the release** keep their recipients (8),
  none among them. Their stalls take the stream's answerer from the
  release on. A stall recorded before it is read as the answerer's too
  at the first settle after the build (decision 5), so the questions
  waiting when the release lands reach the door once.
- **The seven ends pending for `madswan/planner`** are delivered once
  the planner's run takes the address. None is a question.
- **A run standing under the old name** `<kind>--<role>` is found by its
  role field, and its next run takes the entry's name.
- **deneb's configuration**, the host's operator's to write, with
  decision 12's first step. The recommendation:
  - `[host] answerer = "madswan/operator"`;
  - `answerer = "madswan/operator"` for `madswan` and
    `madswan-planning`, and `"swancloud/operator"` for `swancloud` and
    `swancloud-planning`;
  - `[roles."<label>/<role>"]` for each role as it moves: the
    repository `afterthought/blueprints`, the kind `crew/crew-<role>`,
    `name` the label, the stream the teams file names, priority 0, the
    intent `<label>-<role>`, the model in `agents`, and `answers` from
    the role's definition;
  - crew's kinds package locked on the host.
- **The orchestrator** stops its status script. Its launches name no
  recipient and take the `madswan` door.
- **Rollback** is the earlier release. The fields this change adds, an
  answer on an end, a hold on the waiting move, a stop-short's
  addressee, are fields an earlier flume ignores (core.14), and the
  host's `[roles]` keys it adds make the earlier release refuse the
  configuration, so a rollback takes them out with it.

## Open Questions

For the user:

1. **The order of the moves** (decision 12): the operator with flume's
   release, then ops, then the reviewer, then the planner and the design
   agent; madswan first, swancloud after each step, wldn with its host.
   And do the planner and the design agent wait for an effort setting
   in flume, or move at their account's effort?
2. **Who picks** (decision 3): the launch names the owner, the asking
   agent may address another role of its partition by `flume needs
   --to`, and the operator, the door, stands behind every stall and may
   put a stalled question to the user on one card. The last departs
   from the operator's brief, "You post no Pending You card".
3. **A role's run reads flume's inbox.** crew's change records the
   user's word of 2026-10-31, "crew reads nothing of Flume's but what
   the runner carries back". A role's run is a place flume launched on
   the host, and its turn collects its address's questions by `flume
   inbox`, with their keys, at its start, which the runner's copy gives
   only at its next pass. Is the run's agent reading flume's inbox
   within that word?

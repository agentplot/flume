# Design

## In one page

A stage's agent that cannot go on without an answer runs `flume needs`.
Whether anyone reads the question turns on how its place was launched.
A place launched from a shell has no recipient at all. A place that
names a role's address reaches no one where the host's configuration
names no run for the address. A place launched from a herdr pane
reaches that pane, whatever now runs in it. Nothing bounds how long a
question may wait on recipients who do not answer it. On deneb on
2026-11-05, 34 of 37 places had no recipient. Two changes waited about
nine hours on questions only a polling script could see, and that
script was not running.

The family already carries ends in four ways that do not meet:

- flume delivers each end to the place's recipients, or keeps it
  pending (43).
- The Boomdrive runner copies `flume status` into crew's records.
- crew's board reads that copy.
- The host's orchestrator polls `flume status` with a script of its own.

The answer goes back by `flume retry`, by hand or through crew's
request and the runner's next pass.

This design makes flume's delivery the one owner of carrying a place's
end to whoever answers it, and closes each gap in it:

1. **Every place has a recipient that answers or wakes.** A launch's
   recipients are the ones it names. Failing those, they are
   `$FLUME_RECIPIENT`, then the launching session, then the host's
   answerer, a role's address the host's configuration names. A launch
   that would still have none is refused, and so is one naming an
   address that wakes nothing and that no session serves.
2. **A session may serve an address.** A standing role's session is
   reached by its address as a role's run is. A role's run is named by
   its whole address, so two partitions' planners are two places, and
   flume is the one launcher of a role's run.
3. **The host's answerer stands behind every place.** Every stall is
   delivered to it as well as to the place's recipients, with the
   question the place waits on. A recipient that is making the move
   says so with a hold on it, and the place then does not stall.
4. **An answer is taken once.** Every end has a key. A retry that names
   the key of the stop-short it answers is refused once the place no
   longer waits on it, so neither a second answerer nor a late card
   starts a stage again.
5. **A restarted recipient sees what waits on it.** The inbox gives
   every place waiting on its recipient, not only the ends still
   pending. flume's plugin gives it to a session at its start. A notice
   names the place and its brief.
6. **A question that is the user's becomes one card.** The answerer's
   run makes it, never flume (72). The run holds the place's waiting
   move with the card's words, keys the card by the place and the end,
   and retries with the key once the card's carrier wakes it with the
   user's answer.

sluice's statement needs no amendment. crew's rules, its planner's brief
and its card key change with this, and so do Boomdrive's role offers
and its retry, each in its own repository (decision 11).

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
  host's `[roles]` table names for it. The run is named by the role
  alone, `role_of`, and a carrier may ask for the same by `flume wake`
  (132; `wake.rs`; `flume-seams/src/host.rs`, `role_of`).
- **The stall.** A place waiting on its caller under no hold's words
  for its stall bound, five minutes unless the launch sets one, is
  stalled. The stall is an end, delivered once to each recipient
  (130; `held.rs`, `read_stall`).
- **The answer.** A retry starts the stage afresh, the stop-short's
  words before its own, and checks nothing about which question it
  answers (42, 49; `acts.rs`, `retry`).
- **The plugin.** flume's Claude Code plugin does nothing in a session
  flume did not start (`claude/hook.rs`, `hook`). A caller's own
  session gets nothing at its start, though 43 says a caller that
  starts collects what is pending for it.

**The Boomdrive runner** (agentplot/boomdrive at 00bda37).

- **Launching.** It launches crew's work with `flume --json launch`,
  one `--recipient` per `Recipient:` of crew's request, and
  `$FLUME_RECIPIENT` removed from its environment (`flume.rs`; core.15
  there). crew writes `<label>/planner` as the recipient
  (`runner.py`).
- **Ends.** It never calls `flume inbox`, `wait` or `wake`. Each pass it
  reads `flume --json status` and copies a place's status, whole, to
  crew's `<label>/runner/<host>:places.rec` when it changes. It reads
  no end, so nothing is ever delivered.
- **Answers.** It carries crew's retry requests into `flume retry`.
- **Role runs.** It offers a role's run, kind `crew/crew-planner`
  launched with crew's `--kinds`, for a `Role` record in crew's
  `roles.rec`, while a place naming the role's address waits with an
  answer, lost, past its bound, on hold or stalled (`roles.rs`).
- **How it runs.** It is a loop, `boomdrive run`. On deneb it is run by
  hand, still as `flywheel` 0.0.0, with no service keeping it alive.
  Its last pass was 00:28Z on 2026-11-05. While it is down no answer of
  crew's reaches flume, and crew's copy of status goes stale: it still
  shows a place waiting at construct hours after that place landed.

**crew** (afterthought/crew at 651a641c, the commit deneb runs).

- **Questions.** `stage_of` reads a stop-short in the runner's copy as
  "asks at <stage>" or "found at <stage>". The planner's band counts
  "places wait on an answer", and `crew bolts` lists them
  (`runner.py`, `board.py`).
- **Answers.** `crew answer <work> "<words>"` appends an `Act: retry`
  request, with the stage and the words, to `requests.rec`, for the
  runner to carry. It refuses a place crew did not launch
  (`plan.py`, `runner.py`).
- **The planner's brief.** The questions of places that name the
  planner are its own. It answers from the records, or asks the design
  agent or main-level ops, or puts a Pending You card to the user, keyed
  `ask/<name>/<about>`, at most two at a time (`planner.md`, `crew.py`;
  briefs.11; records 0005, 0031).
- **Standing roles.** On deneb every role stands. The planner is a pane,
  `madswan-planner`. It gets new work only as tells and card answers in
  its pane, and `tell_planner` runs only for proposals, so a place's
  question never reaches that pane.
- **crew's rules.** roles.6 says a session's start collects what is
  pending for it. roles.7 says every place crew launches names a role's
  address, and a role with no live run is woken by flume, never by a
  process of crew's.
- **Why the runner launches role runs.** crew's change
  `roles-run-as-flume-kinds`, Decision 1, set flume's `[roles]` aside.
  flume names a role's run `<kind>--<role>` by the address's second word
  alone, so madswan's and swancloud's planners, whose first repository
  is the same, would be one place. It also says "the user chose the
  runner as the launcher".

**sluice** (agentplot/sluice at main; its statement at 9f1f0fd).

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
  first message. A reply goes only to the session that sent the message,
  and a sender with no session is "nobody in particular". A later run
  can read the thread, but nothing wakes it.

**The orchestrator.** A Claude Code session on deneb launches the
host's agentplot work. It runs `env -u HERDR_ENV -u HERDR_PANE_ID -u
HERDR_SOCKET flume launch …` with no `--recipient`, so its places have
none. A script of its own reads the text of `flume status` every 45
seconds, picks the lines that wait on their caller, and dedupes them by
hand. The session then answers a question itself or posts a Pending You
card under a key it makes up, and retries. Its transcript holds 243
`post_request` calls, 75 Inbox messages and 61 `AskUserQuestion` calls.

### Where it breaks

1. **No recipient.** Clause 7 lets a launch have none, and on deneb that
   is most of them. Nothing but a poller sees their ends, and a poller
   that is down sees nothing.
2. **An address that wakes nothing.** Clause 132 has an address with no
   kind "woken by nothing", and still takes a launch naming it. The
   runner's places name one, and nobody learns their ends.
3. **A role that stands has no way to be reached by its address.** A
   standing planner's pane is not a role's run. flume can deliver to the
   address only in a run's pane.
4. **Two owners of waking a role.** flume's `[roles]` wake and the
   runner's role offers do one thing. crew left flume's for a defect of
   flume's: two partitions' runs of one role share a name.
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

- **flume's statement.** 7, 8, 42, 43, 49, 56, 62, 69, 70, 72, 130 and
  132. 69 says flume does the same for every caller. 72 says flume tells
  no person anything by itself. 120 says it runs no process of its own.
- **flume's records.**
  - 0009: an end is delivered once inside the settle, a wait is by
    place, and a caller's start collects.
  - 0011: an answer is a retry.
  - 0013: a recipient may be a role's address, woken by flume.
  - 0014: the words delivered, pending, collect and notice.
  - The record on the unlanded change
    `an-answer-reaches-the-open-session-within-an-hour`, accepted by the
    user on 2026-11-01: within an hour an answer reaches the asking
    agent's open session. It is numbered 0028 on its branch, the number
    main's statechart record took, so it lands under a new number. That
    answer is the same retry this design guards with its key.
- **flume's rules.**
  - core.1: no person's queue is named outside a profile.
  - core.11: a notice is never sent into a question.
  - core.13: no process of flume's own.
  - core.24, core.25 and core.28: holds, delivery and roles.
- **crew's rules.** roles.5: crew builds no mechanism its tools hold.
  roles.6: a session's start collects. roles.7: flume wakes a role.
- **Boomdrive's rule core.15.** The runner adds no recipient of its own.
- **The user's boundary.** A design stops at flume and its host's
  configuration. Whatever writes that configuration is one configurer
  among many.

## Goals / Non-Goals

**Goals:**

- Every question a place asks reaches a recipient that collects it, or
  one that a run is woken for, whatever launched the place.
- A question that no recipient takes up within the place's stall bound
  reaches the host's answerer.
- A question is answered once. A second answer, or a late one, starts
  nothing.
- A recipient that restarted sees every question still waiting on it,
  with enough to act on.
- A question that is the user's reaches them as one card, and their
  answer reaches the place as its retry's words with no hand carrying
  it.
- One owner for each part across flume, crew, Boomdrive and sluice.

**Non-Goals:**

- flume deciding which questions are a person's, or telling a person
  anything itself (69, 72). The answerer's run decides, and the card is
  its own.
- A person's queue, Pending You, the Plannotator Inbox or any other,
  named in flume's code or its kinds (core.1, core.35).
- A process of flume's that watches cards or waits for answers (120).
- Changing what an answer is. It stays a retry naming the stage with
  the answer as its words, the question before it (42, 49). The open
  session within the hour is the other change's.
- sluice's own outcomes, which are not questions. They stay sluice's
  (decision 10; Found beside).
- Building any of it before the user's yes.

## Decisions

### 1. Who answers, for each kind of launch

A place's recipients are resolved at its launch, the first of these
that gives any:

1. the recipients the launch names, `--recipient`, one or several, each
   a session's name or a role's address;
2. `$FLUME_RECIPIENT` in the launching command's environment;
3. the session the launching command runs in, as the terminal names it,
   `herdr:<pane>@<socket>` under herdr;
4. the host's answerer: the address the configuration names for the
   place's stream, else for the host (decision 2).

A launch that finds none of the four is refused, naming each and how to
give one, and makes nothing (4; core.8). A role's run keeps having no
recipient of its own (132). The recipients are fixed at the launch (8),
and the answerer behind them is read at each stall (decision 5).

| how the place was launched | its recipients | behind them |
|---|---|---|
| a shell or a script with no session, naming none | the stream's answerer, else the host's; refused where the host names neither | none: the answerer is the recipient |
| a script naming a recipient it collects as, `--recipient ci` | `ci`, collected by `flume inbox --recipient ci` | the host's answerer, at each stall |
| a herdr pane, by `/launch` or a person's command | that pane's session | the host's answerer, at each stall |
| the runner, for crew's work | the address crew writes, `<label>/planner` | the host's answerer, at each stall |
| any launch naming a role's address | the address: its run's pane, or the session serving it (decision 3) | the host's answerer, at each stall |

The orchestrator strips herdr's variables so that its places are not
its pane's. Its launches then take the host's answerer. It may instead
name its own recipient, and collect for it or serve it. It no longer
needs a script to poll status.

### 2. The host's answerer

The host's configuration names it:

```toml
[host]
answerer = "madswan/planner"

[streams.work]
answerer = "work/ops"     # comes first for the places that run in work
```

The answerer is a role's address (132). The configuration must also
name a run for it, `[roles."<address>"]`, or a session must serve it
(decision 3). A launch that would take an answerer that has neither is
refused, naming it, as a launch naming such an address is. A host that
names no answerer has none. There every launch must name its recipient
or run in a session, and the stalls go to the recipients alone. The
answerer is read from the configuration as it stands when asked, at
each launch and at each stall, and never kept, as the stream and the
pool are (143; `history-and-host`).

The answerer is one address rather than a list, and one per stream at
most. It is the host's last word on who answers. A host that wants a
chain, design before ops before the user, writes it into its answerer's
kind, as crew's planner already holds one (briefs.11).

### 3. An address wakes or is served

A launch naming a role's address is refused, naming the address, when
the host's configuration names no run for it and no session serves it
(4, 132). The runner's launches naming `madswan/planner` on deneb today
would be refused, which is the point: they reach no one.

**Serving.** `flume serve <address>`, run in a terminal's session,
records in the host's state that this session serves the address, with
the session's name and since when. The record is a document under the
state directory, `addresses/<digest>.json`, written by compare-and-set
as every document is (C.7; core.16). While the terminal reads that
session live, the address's ends are delivered into it, as into a run's
pane, and a settle wakes no run for an end it can deliver there. A
session serves until the terminal reads it stopped or gone, or until
another session serves the address. The later saying wins, and status
names both. `flume serve` answers what `flume inbox --recipient
<address>` answers (decision 8), so a session that starts serving
collects in the same act. A run of the address, when one is live, comes
before a serving session. A wake of an address a live session serves
does nothing, and says the session.

A standing role is then reached by its address as a running one is.
crew's session-start hook for a standing role runs `flume serve
<label>/<role>`. flume's own plugin does the same at the start of a
session whose `$FLUME_RECIPIENT` names an address (decision 8).

Rejected:

- **An address bound to a pane in the host's configuration.** Panes come
  and go, and the configuration is the host's, not the session's.
- **A served address that wakes its kind even while served.** That is
  two answerers for one address.
- **Serving read from an inbox alone.** An inbox is a read for anyone
  who asks, and a script collecting for an address would take its
  deliveries away from its pane.

### 4. A role's run is named by its whole address

A role's run is the place `<kind>--<partition>-<role>`, rather than
`<kind>--<role>`. The `/` of the address cannot stand in a branch's
last part, and `-` is legal in both words, so the name is the
partition, `-`, then the role. Two addresses that would give one name,
`a-b/c` and `a/b-c`, are told apart by the place's role field, which
the run's lookup already reads first (`wake.rs`, `run_of`). The second
of them to be woken is refused, naming the first, as any launch of a
name that stands is (3). A run standing under the old name is found by
its role field, so a host carries on across the release.

With that, crew's reason for launching role runs itself is gone. flume
is the one launcher of a role's run (decision 10). The runner keeps
deciding when crew's own events want a role, a tell, a capture or a
card's answer, and carries each to flume by `flume wake <address>`
(132). crew's follower of Pending You's notices does the same for a
card's answer. The role's kind is the one the host's `[roles]` names.
crew's kinds reach it as a package the host's lock records (core.34),
since `flume wake` carries no `--kinds`. Whether flume or the runner
launches role runs is the user's to rule, because crew's change records
that they chose the runner (Open Questions).

### 5. The bound: a stall reaches the host's answerer

The place's stall bound, five minutes unless the launch sets another
(130), is the one bound. Once a place has waited on its caller that
long under no hold's words, it is stalled, and its stall is delivered
once to each of these:

- each recipient of the place, as today;
- the host's answerer, read from the configuration when the stall is
  read, unless it is one of the recipients.

The stall carries the move the place waits on, and where that move
answers a stop-short, the stop-short's words and key. So a reader who
never saw the place has the question whole. The stall is delivered to
the answerer as any end is: into its run's pane or its serving session,
or pending and waking its run (43, 132).

A stall reaches the answerer when no one has collected the question,
because the pane is gone, the address wakes nothing, or the run did not
start. It also reaches the answerer when a recipient collected the
question and did not act on it within the bound. This is the "someone
is paying attention" the user asked for. Past the place's stall bound,
the host's answerer knows. A recipient that is making the move keeps
the answerer out of it by saying so (decision 6).

An end of a stall recorded before this change names no answerer. The
first settle after the build reads it as the host's answerer's too, so
the questions waiting on deneb when the build lands reach the answerer
at once.

Rejected:

- **A second bound for the answerer, longer than the stall.** Another
  knob on every host, and the stall already says the place waits on its
  caller with nobody's words. A launch that wants more room gives
  `--stall-after`.
- **The answerer only for ends no recipient collected.** That misses a
  pane whose agent took the notice and restarted, and a recipient that
  took it and does nothing, which is what the user is asking about.
- **Every end to the answerer at once.** That is two answerers from the
  first minute, and the launching session, which knows the work best,
  loses its turn to answer.

### 6. A hold on the waiting move

A caller may hold the move the place waits on, with words saying who
makes it and how, and an optional bound:

```sh
flume hold <place> "asked the user on card req_4d00…" --waiting --for 1h
```

This is a hold as 130 already has them, with its words and since when,
put on the place's waiting move rather than on a stage not yet started.
A place waiting under a hold's words is never stalled, so its question
does not go to the answerer. Status gives the hold with its words, since
when and its bound. A retry naming the stage or the submission ends it,
as the move made (47, 49). A drop ends it with the place. Where the hold
gives a bound, the settle that reads the bound passed lapses the hold:
status says it lapsed and when, and the place stalls at once, its stall
delivered as decision 5 has it. Where it gives none, it stands until the
move is made, as every hold does (the ruling of 2026-10-26, "a hold is a
state status names, never a wait status warns of"). It is refused,
naming why, on a place that waits on no move of its caller's.

The bound is how an answerer that asked a person is woken again when no
carrier brings the answer back (decision 9). An hour suits Pending You
on a host whose follower runs.

Rejected:

- **A command of its own, `flume claim`.** It would be a second kind of
  words on a waiting place, beside the hold's, with the same effect on
  the stall.
- **A claim implied by an inbox's read.** A read marks nothing (43).

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
session or a fresh agent.

A tell carries no key: it is no answer (55), until the open-session
change makes a tell within the hour one, and then it takes `--answers`
too.

### 8. What a restarted recipient sees

**The inbox gives what waits on it.** `flume inbox`'s answer keeps its
`ends`, every end pending for the recipient, each delivered by the
inbox. It gains `waiting`: every place on the host that waits on its
caller and that this recipient is a recipient of. For the host's
answerer, that is every stalled place whose stall it was given. Each is
given with:

- the place, its kind, its name and the first line of its brief;
- the move it waits on, and since when;
- where the move answers a stop-short, its words, whether it needs an
  answer or found work, and its key;
- a hold on the move, with its words, since when and its bound.

`waiting` delivers and marks nothing (43; core.25), so a recipient that
collected a question, then restarted, still finds it. `flume serve`
gives the same answer.

**A session's start.** flume's Claude Code plugin's SessionStart hook,
in a session flume did not start, now acts:

1. Where `$FLUME_RECIPIENT` names a role's address, it runs `flume serve
   <address>`. Otherwise it runs `flume inbox` as the session the
   terminal names. Under the printed command, which names no session,
   it does nothing.
2. Where the answer holds an end or a waiting place, it gives the agent
   the answer's words as the session's added context. Each place gets
   its question, its key, and the three ways on: answer by retry with
   the key, hold the move with words, or leave it, and the host's
   answerer has it at the stall.
3. Where the answer holds nothing, it gives nothing.

It never blocks the start, under its 20 seconds' timeout, and a flume
that cannot be reached gives nothing. This is sluice's design for a
session's start (sluice 51, 135), so the two tools now do one thing
one way.

**The notice.** It stays one line, and names more:

```
flume: change--every-question (Every question a place asks reaches…) construct needs an answer [stage/construct/1/ended-short]; read: flume inbox
```

It names the place, its brief's first line cut at 60 characters, the
end, its key, and the command that reads it whole. That is `flume inbox`
for an end that waits on a move, which gives the question and how to
answer, and `flume status <place>` for any other. It names no fact to
act on blindly (core.25).

Rejected:

- **Recording, at the launch, which conversation in the pane launched
  the place, and not delivering to another.** herdr gives no
  conversation's identity of a pane flume did not start. A new agent in
  the pane is as likely to be the person's next session as a stranger,
  and the stall reaches the answerer either way.
- **A notice holding the question's words.** Words are many lines, and
  a notice is typed into a pane (core.11).

### 9. The user's question becomes one card

flume's part ends at the answerer: it reaches the answerer, gives it
the question with its key, takes the hold that says the answerer is on
it, and takes the answer once. What the answerer does is its kind's.
The design gives the protocol the answerer's run follows, for crew's
planner to write into its brief and for any host's own answering kind:

1. **Collect.** At its start the run collects for its address, `flume
   inbox` or `flume serve`, and reads `ends` and `waiting` (132).
2. **Answer what it can.** A question its records, its goals or another
   role answer is answered by `flume retry <place> --stage <stage>
   --answers <key> --words "<answer>"`, through whatever carries the
   run's acts to flume. For crew that is `crew answer`, carrying the key.
3. **Take a question that is the user's.**
   - **Hold first.** It holds the waiting move: `flume hold <place>
     "<words naming the card>" --waiting --for <bound>`.
   - **Then one card.** It makes one card in the user's queue under a
     key made from the host, the place's location and the end's key,
     `flume/<host>/<digest of the location>/<key>`. A second run, a
     second answerer, or the same run after a restart then gets the
     first card back rather than a second (Pending You's
     `idempotencyKey`, the Inbox's `idempotency_key`). The card gives
     the question whole, the place and its brief, and the address to
     wake when it is answered.
   - **Hold before card.** Holding first means a second reader sees the
     question taken before the card exists.
4. **Carry the answer back.** When the user answers, the queue's carrier
   runs `flume wake <address>` (132). On deneb that is crew's follower
   of Pending You's notices, `crew _answered`, or `pendingyou watch --
   flume wake <address>`. The woken run reads the card's answer and
   retries with the key and the user's words, and the fresh agent's
   prompt carries the question with the answer (42, 49). It then closes
   the card. On a host with no carrier, the hold's bound wakes the run:
   the hold lapses, the place stalls, and the stall reaches the address,
   whose run reads the card again.
5. **Withdraw a card whose question was answered.** A run that reads a
   card whose question no longer waits withdraws it. That covers a
   retry refused with "already answered", a question answered in a
   pane, and a place dropped.

**Which queue.** That is the answerer's kind's choice, and the design
recommends Pending You for a role's run. A run ends and is woken again
by address, and Pending You has a carrier that runs a command for any
answer on the host. The Inbox answers only the session that asked, so a
run that ended is never woken by it. The Inbox suits a session that
stays, a person's own pane, when it asks the user about a place of its
own, after holding the move. crew already asks the user on Pending You
and not the Inbox (`crew.py`).

**Never asked twice** is then held at four points:

- the hold, which keeps the question from the answerer while its
  recipient asks;
- the card's key, which keeps a second card from being made;
- the answer's key, which keeps a second answer from starting anything;
- the withdrawal, which takes down a card answered elsewhere.

Rejected:

- **flume makes the card, through a seam for a person's queue.** That
  is option 4 of the record: it breaks 72 and core.1, and flume would
  judge which questions are a person's.
- **The card made by whoever first reads the question, with no hold.**
  That is today's orchestrator, and two readers make two cards.

### 10. One owner for each part

| part | owner | the others |
|---|---|---|
| A place's recipients, and the host's answerer | flume (7, 190) | the runner passes crew's recipients unchanged (Boomdrive core.15); crew writes the address |
| Delivering an end, and keeping it pending | flume (43) | the runner's copy of status is a view for crew's board, never a delivery; sluice's outcome for a place reaches flume at its settle (C.6) |
| Waking a role's run | flume (132) | the runner and crew's follower carry their events to it by `flume wake` |
| A standing role reached by its address | flume's serving (132) | crew's session-start hook runs `flume serve` |
| A session's start collecting | each tool for its own: flume's plugin for flume's ends, sluice's for sluice's | one design, sluice's (sluice 51) |
| Which questions are a person's, and the card | the answerer's kind: crew's planner on a crew host | flume decides nothing (69, 72) |
| Carrying a card's answer to a run | the queue's carrier, by `flume wake` | crew's follower on deneb |
| Taking an answer once | flume (42, 49) | `crew answer` and the runner carry the key |

### 11. The statements' amendments, for the user to approve

#### 11.1 flume's statement (agentplot/blueprints, `design/flume/requirements.md`, at 9f1f0fd)

**Section 3.**

- Replace **recipient** with: "whoever an end of a place goes to: a
  caller's session, or a role's address, the host's answerer among them
  (43, 132, 190)."
- Add **answerer**: "of a host or a stream, the role's address its
  configuration names to answer for it: the recipient of a launch that
  names none and runs in no session, and a recipient of every stall of
  a place that is not its own (190)."
- Add **serve**: "of a session, that it said it answers for a role's
  address, so that while it is live the address's ends are delivered
  into it (132)."
- Add **key**: "of an end, the name flume gives it, which no other end
  of the place has, by which a recipient and an answer name it (42,
  43)."
- In **on hold**, after "before it starts", add: "; of the move a place
  waits on, that a caller said in its words that it is making it
  (130)".

**Clause 4.** Append: "So is a launch that names no recipient and runs
in no session on a host whose configuration names no answerer, saying
how to name one (7, 190), and one that names a role's address, or would
take an answerer, that the host's configuration names no run for and no
session serves, naming the address (132)."

**Clause 7.** Replace "A recipient the launch names none of is the
session that launched it, as the terminal names it, or none where it
names none." with: "A launch that names no recipient has the session
that launched it, as the terminal names it, else the host's answerer
(190)."

**Clause 8.** Replace "Its recipients are fixed at its launch." with:
"Its recipients are fixed at its launch, and the host's answerer, as
its configuration names it when a stall is read, is delivered the
place's stalls besides (130, 190)."

**Clause 42.** After "and the fresh agent's prompt carries the question
with the answer.", add: "An answer may name the stop-short's key, and
is then taken only while the place waits on that stop-short, so a
question is answered once: a second answer, or one that comes after the
place moved on, is refused naming the answer taken, who gave it and
when (49)."

**Clause 43.**

- After "and otherwise stays pending; it counts delivered on the
  terminal's answer, with no reading back.", add: "A notice names the
  place, the first line of its brief, the end and its key, and where to
  read the end whole."
- Replace "a recipient named by a role's address is delivered to in the
  pane of that role's live run, and where none is live the end stays
  pending and wakes the role (132)" with: "a recipient named by a role's
  address is delivered to in the pane of that role's live run, else of
  the session that serves the address, and where neither is live the
  end stays pending and wakes the role (132)".
- Replace "by `flume inbox`, which delivers each and marks it delivered;
  status and a read mark nothing." with: "by `flume inbox`, which
  delivers each and marks it delivered, and gives with them every place
  it is a recipient of that waits on its caller, delivered or not, with
  the move and since when, the words and key of a stop-short it waits
  on and a hold's words, so a caller that restarted sees each question
  still its own; that list, status and a read mark nothing."

**Clause 49.** Append: "A retry that names a stop-short's key is
refused, naming the place, the answer the stop-short already took, who
gave it and when, or the move the place waits on now, unless the place
waits on that stop-short (42)."

**Clause 56.**

- After "the words of a stop-short, and whether it needs an answer or
  found work, or of a failed submission;", add: "each end's key, and
  for a stop-short answered, its answer with who gave it and when; the
  place's recipients, each with where it came from;".
- After "and since when, a hold's words", add: ", a hold on the waiting
  move with its bound or that it lapsed,".
- After "for the host and for each of its streams," add: "its answerer;
  for each role's address an end stands pending for, its run, the
  session serving it and since when;".

**Clause 130.**

- After "a retry naming the stage or the submission ends the hold (47,
  49).", add: "A caller may also hold the move the place waits on it
  for, with words saying who makes it and how, such as an answer asked
  of a person on a card, and a bound after which the hold lapses; a
  retry naming the stage or the submission ends it as the move made."
- Replace "and each recipient of the place is delivered once that it is
  stalled and which move is theirs (43)." with: "and each recipient of
  the place, and the host's answerer where it is none of them, is
  delivered once that it is stalled, which move is theirs, and the words
  and key of the stop-short it waits on (43, 190). A hold on the waiting
  move whose bound passed is no hold from then, and the place stalls."
- Replace "and flume watches none and ends no hold itself (70, 72)."
  with: "and flume watches none and ends no hold itself, but lapses a
  hold on the waiting move once its bound passes (70, 72)."

**Clause 132.**

- Replace "named by the role, so at most one run of an address is live
  at once (3)" with: "named by the address, its partition and its role,
  so at most one run of an address is live at once (3) and two
  partitions' runs of one role are two places".
- After "a wake of an address whose run is live or waiting does
  nothing.", add: "A session may serve an address by saying so: while
  the terminal reads it live, the address's ends are delivered into it
  as into a run's pane (43), and no run is woken for an end delivered
  there; it serves until the terminal reads it stopped or gone, or
  another session serves the address, and a wake of an address a live
  session serves does nothing."
- Replace "An address the configuration names no kind for is woken by
  nothing, and status says the ends pending for it." with: "An address
  the configuration names no kind for is woken by nothing, and a launch
  naming it is refused while no session serves it (4); where none
  serves it, its ends stay pending, status says so, and the place's
  stalls reach the host's answerer (130, 190)."

**New clause 190**, in A.12 after 132: "A host's configuration may name
its answerer, a role's address that it names a run for or that a
session serves (132), and a stream may name its own, which comes before
the host's for the places that run in it (143). A launch that names no
recipient and runs in no session has the answerer as its recipient (7),
and every stall of a place is delivered to the answerer besides the
place's recipients (130), so a question that no recipient collects or
answers within the place's stall bound reaches a run woken to answer
it. flume decides nothing in this: which address answers, what its kind
does, which questions its run answers itself and which it carries to a
person, and how, are the host's and the run's (69, 72). A host whose
configuration names no answerer has none, and a launch there names its
recipient or runs in a session (4)."

**C.4**, after the hooks' table: "In a session flume did not start, the
plugin's SessionStart hook runs `flume serve` of the address
`$FLUME_RECIPIENT` names, where it names one, and `flume inbox` as the
session otherwise, and gives the agent what it answers as the session's
context, or nothing where nothing is pending or waiting; it never holds
up the start (43, 132)."

**C.5**, after "found again by that start's herdr name as an agent's
pane is (132).", add: "A session that serves an address is reached by
the pane and socket it served from."

**C.7**, after the `[roles]` bullet, add: "The host's answerer is
`[host] answerer = "<partition>/<role>"`, and a stream's
`[streams.<name>] answerer`, read as the configuration stands at each
launch and each stall (190). A session's serving of an address is host
state, one document per address under the state directory, with the
session and since when, written by compare-and-set (132)."

**Section 9**, the command line's list: after "`needs` for an agent;",
add: "`hold`, `wait`, `inbox`, `wake` and `serve` for its callers;".

**The callers table, crew's row.**

- Replace "it names each place's recipient by a role's address, so a
  role with no live run is woken to collect an end (132)" with: "it
  names each place's recipient by a role's address, a role that stands
  serving it, so a role with no live run or serving session is woken to
  collect an end (132)".
- Replace "its word to a place is a retry answering a stop-short that
  asked" with: "its word to a place is a retry answering a stop-short
  that asked, naming its key (42)".

**Scenarios.**

- **S91.** A script with no terminal session launches a change, naming
  no recipient, on a host whose configuration names `ops/answerer` its
  answerer, with a run. Its construct stops short needing an answer.
  The settle finds the end pending for the address with no run, and
  wakes it. The run's start collects the end with its key and the
  place's brief. The run cannot answer from what it reads. It holds the
  place's waiting move with words naming the card it makes for the
  user, keyed by the place and the end, and ends, and the place does
  not stall. The user answers the card, and its carrier wakes
  `ops/answerer`. The new run reads the answer and retries construct
  naming the key, and the fresh agent's prompt carries the question
  with the answer. A retry naming the same key from the operator's pane
  a minute later is refused, naming the run's answer and when (7, 42,
  43, 49, 130, 132, 190).
- **S92.** The operator launches a change from a herdr pane on that
  host. Its construct stops short needing an answer. The notice is
  delivered into the pane, naming the place, its brief's first line and
  the end's key. The pane's agent is restarted before it acts. At its
  start, flume's plugin gives the new agent the place waiting on it,
  with the question and the key. The agent does nothing with it. Five
  minutes after the stop-short the place is stalled. The stall, with
  the question and its key, is delivered to the pane and to
  `ops/answerer`, whose run is woken (43, 130, 190, C.4).
- **S93.** The runner launches a change naming `madswan/planner`, on a
  host whose configuration names no run for that address and where no
  session serves it. The launch is refused naming the address, and
  nothing is made. The planner's standing session runs `flume serve
  madswan/planner`, and the same launch is taken. Its construct
  delivers, and the end is delivered into the planner's pane. The
  planner's session ends. The next end stands pending and nothing is
  woken, and status says the address has no run and no session serving
  it (4, 43, 132).
- **S94.** Ends come at once for `madswan/planner` and
  `swancloud/planner`, whose runs the host's configuration names in one
  repository and one kind, `planner`. Two places are launched,
  `planner--madswan-planner` and `planner--swancloud-planner`, and each
  run collects its own address's ends (3, 132).

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
- **roles.7**:
  - Replace "every place crew launches names its recipient by a role's
    address, never a session" with: "every place crew launches names its
    recipient by a role's address, never a session, and a standing
    role's session serves its address by `flume serve`".
  - Replace "a role with no live run is woken by Flume, never by a
    process of crew's" with: "a role with no live run is woken by Flume,
    never by a process of crew's or the runner's, which carry crew's
    events to it by `flume wake`".
- **briefs.11**, and the planner's brief, `plugin/roles/planner.md`:
  - **The protocol.** The answerer's protocol of decision 9: collect at
    its start; answer by `crew answer` with the key; hold the waiting
    move before a card; one card keyed `flume/<host>/<digest of the
    location>/<key>`; withdraw a card whose question no longer waits.
  - **Answering as the host's answerer.** The planner answers the
    questions of places crew did not launch where the host names it its
    answerer, so `crew answer` takes any place on the host by its name,
    and its request names the place and the key (`Answers:`).
- **Record 0049**: an amendment naming flume record 0039. The card
  wake's `flume wake <address>` is crew's follower's, `crew _answered`,
  for a role that runs as its kind. A standing role serves its address.
- **The change `roles-run-as-flume-kinds`, Decision 1**: its rejection
  of flume's `[roles]` falls with flume's naming by the whole address.
  crew's kinds become a package the host's lock records, named by the
  host's `[roles]`, and the runner's role offers become wakes. This is
  the user's ruling (Open Questions).

#### 11.4 Boomdrive (agentplot/boomdrive: records and specs, Boomdrive has no statement in blueprints)

- **Role offers.** `crew-control-plane`'s role offers: an offer is made
  by `flume wake <address>`, and the runner launches no role's run with
  `--kinds`. A role "wanted" for a place's end needs no offer at all,
  since flume wakes it at the settle that reads the end.
- **Retries.** `work-into-flume`'s retry: a request's `Answers:` is
  passed as `--answers <key>`, and flume's refusal of an answer already
  taken is recorded as the request's outcome, refused, in flume's
  words.
- **The status copy.** The runner's copy of status into crew's records
  is a view, and a record says so: crew reads it for its board, and no
  question's delivery depends on it.

### 12. Order across the repositories

1. **The user's yes.** On the design, record 0039 and the two Open
   Questions.
2. **agentplot/blueprints.** flume's statement amended on main as 11.1
   words it (core.19).
3. **afterthought/crew.** The standing roles' session-start hook runs
   `flume serve <label>/<role>`, its failure ignored, so it lands before
   or after flume's release with no lockstep.
4. **This repository.** The build, released.
5. **The host's configuration.** Before a release reaches a host whose
   launches name an address, the host's configuration names its
   answerer and a run for each address, or the address's session serves
   it. Otherwise the refusal of an address that wakes nothing refuses
   the runner's launches there (decision 3; Migration Plan).
6. **afterthought/crew and agentplot/boomdrive.** The planner's
   protocol, `crew answer`'s key and any place, the runner's wakes and
   its retry's key. Each works against the released flume. None blocks
   flume's build, since a retry with no key is taken as today.

## Alternatives considered

1. **Each caller collects for itself** (record 0039's option 1). This is
   today. A shell's places are seen only by a poller, and a poller that
   is down loses the night. Record 0009 already refused polling for the
   load it puts on a host.
2. **Refuse a launch with no recipient, nothing more** (option 2). It
   closes the first gap only. An address that wakes nothing, a recipient
   that never answers and a restarted pane stay as they are, and a
   script names a recipient nobody collects as.
3. **This design** (option 3).
4. **flume carries the user's question to a person's queue itself**
   (option 4). It breaks 72 and core.1. flume would judge which
   questions are a person's, which 69 leaves to callers. It would also
   own every queue's dedupe and carrier, which are the queue's and the
   answerer's.
5. **A standing watcher carries questions** (option 5). This is the
   runner's copy and the orchestrator's script. Either is a second
   reader of flume's ends beside its delivery, lags by its pass, loses
   everything while it is down, and needs a process that runs between
   flume's commands, which the family refuses.
6. **Every place's recipient is the host's answerer, always.** It is
   simpler, but the launching session, which knows the work, never gets
   the question first.
7. **The answerer as a list, or a chain in flume.** flume would order
   answerers, which is the host's and the kind's to decide. One address,
   whose kind holds the chain, is enough.
8. **The runner owns waking roles, and flume's 132 wake goes.** This
   keeps the user's choice of the runner. It puts a question's wake on a
   loop that, on deneb, runs by hand, and it leaves flume's statement
   with a wake that no host uses. Recommended against; the user rules
   (Open Questions).

## Where the statements and the code disagree

- **Clause 43 says a caller that starts collects what is pending for
  it.** flume's plugin does nothing at the start of a session flume did
  not start (`hook.rs`, `hook`). sluice's plugin collects (51, 135).
  Decision 8 brings flume's code to its clause.
- **crew's roles.7 says Flume wakes a role.** crew's change
  `roles-run-as-flume-kinds` has the runner launch role runs instead.
  The rule holds by its letter, since the runner is not a process of
  crew's, but not by record 0049's design. Decision 4 and 11.3 make
  them agree.
- **crew's planner brief says a place's end is told to the planner.**
  Nothing tells a standing planner: `tell_planner` runs for proposals
  only. Decision 3 gives the standing planner its address.
- **flume's CLAUDE.md says each tool's statement is in
  agentplot/blueprints.** Boomdrive has none there. Its records and
  specs stand for one, so 11.4 is written against them.
- **Section 9's command line** names no `hold`, `wait`, `inbox` or
  `wake`, though each is built and specified. 11.1 adds them with
  `serve`.

## Found beside

- **The open-session record's number.** The record on the unlanded
  change `an-answer-reaches-the-open-session-within-an-hour`, accepted
  on 2026-11-01, is numbered 0028, which main's statechart record took.
  It takes the next free number when it lands, and its change's tasks
  name the old one throughout. This design's key is written to cover its
  answer (decision 7).
- **The runner on deneb** is still the `flywheel` binary, at
  `~/.config/flywheel/`, run by hand, with no service keeping it alive.
  While it is down, no `crew answer` reaches flume. This design keeps a
  question's way to its answerer off that loop. The answer's way back
  through `crew answer` still runs through it, so the runner's service
  matters to every crew answer.
- **sluice pushes once, into whatever the pane shows.** A push not made
  is never made again, and a push is sent into a pane that asks a
  question (`step.rs`, `perform_outbox`; `herdr.rs`). flume tries again
  at each settle while pending and never sends into a question
  (core.11). The outcome stays pending and a session's start collects
  it, so nothing is lost. A reconciliation of the push, sluice taking
  flume's rule, is sluice's to weigh and not part of this design.
- **A sluice outcome submitted from a plain shell** stays pending for no
  session, as a flume end did before this design. Outcomes are not
  questions, and sluice's failed submissions blame and send back on
  their own, so no answerer is proposed for them.
- **The place `research--sluice-signs-its-landings`** has been never
  begun for 70 hours, with a stop-short pending for its pane. Its words
  point to Pending You card `req_4d00977773964d858c61`.

## Risks / Trade-offs

- **Asked twice by a pane and a card.** A recipient's session that asks
  the user in its own conversation without holding the move lets the
  stall reach the answerer, which may card the user too. The notice,
  the plugin's context at a start and the site all say to hold before
  asking. A launch that expects slow answers gives `--stall-after`.
- **Runs woken by stalls.** Each stall the answerer gets may wake its
  run, one agent session. One run collects a burst (132), and a hold
  stops the repeats. A host whose answerer would be woken too often
  gives its streams' places a longer stall bound, or an answerer of
  their own.
- **Refused launches.** A script that launched with no recipient on a
  host that names no answerer is refused after this change. The refusal
  names the three fixes: `--recipient`, `$FLUME_RECIPIENT`, or `[host]
  answerer`. Every place a refusal now stops is one whose questions
  reached no one before.
- **The runner's launches refused until deneb's configuration names a
  run for `madswan/planner` or its session serves it.** The order in
  decision 12, step 5, and the Migration Plan hold this off.
- **A session's start costs a flume call.** Every session the user
  starts runs `flume inbox` under a 20 seconds' bound, and prints
  nothing when nothing waits.
- **A pane serves on after its agent changed.** Serving is the pane's,
  as a recipient is. A new agent in a serving pane gets the address's
  ends until another session serves it. crew's hook serves again at
  each standing role's start, so a role's own session takes the address
  back.
- **A hold's bound is a timer read at settles.** Under a terminal that
  keeps no watch, a hold lapses only at the next acting command (119).
  Under herdr, the watch reads it within its bound (122).

## Migration Plan

- **Places launched before the release** keep their recipients (8),
  none among them. Their stalls take the host's answerer from the
  release on. A stall recorded before it is read as the answerer's too
  at the first settle after the build (decision 5), so the questions
  waiting when the release lands reach the answerer once.
- **The seven ends pending for `madswan/planner`** are delivered once a
  run or a session takes the address. None is a question.
- **A run standing under the old name** `<kind>--<role>` is found by its
  role field, and its next run takes the whole address's name.
- **deneb's configuration**, the host's operator's to write, before the
  release reaches it. The recommendation, put to the user:
  - `[host] answerer = "madswan/planner"`;
  - while the planner stands, its session serves the address at its
    start, by crew's hook;
  - once it runs as its kind, `[roles."madswan/planner"]` names its
    repository and crew's planner kind.
- **The orchestrator** stops its status script. Its launches either
  name no recipient and take the answerer, or name a recipient it
  serves.
- **Rollback** is the earlier release. The documents this change adds,
  the served addresses and an answer on an end, are fields an earlier
  flume ignores (core.14).

## Open Questions

For the user, each on the design as a whole:

1. **Record 0039.** Accept it, so flume's statement is amended as 11.1
   words it and the build starts?
2. **Who launches a role's run: flume or the runner?** crew's change
   records that the user chose the runner as the launcher. The design
   recommends flume, the runner carrying crew's events by `flume wake`,
   now that flume names a run by its whole address. That way a
   question's wake never waits on the runner's loop. Keeping the runner
   is alternative 8.
3. **deneb's answerer.** The recommendation is `madswan/planner`: crew's
   chain already makes a place's question the planner's (briefs.11),
   served by the standing planner's session until it runs as its kind.
   Or should the orchestrator's places name an address of the
   orchestrator's own, with the planner as the host's answerer behind
   it?

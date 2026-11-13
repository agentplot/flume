---
title: Callers, holds and ends
description: Who answers a place and is told how its stages end, how flume delivers an end once or keeps it pending, flume wait and flume inbox, holds with words and the hold on the waiting move, stops short and a question put to a role, the agent kept open for its answer, the answer taken once by its key, retries and stalls.
sidebar:
  order: 4
---

A caller is whoever launches a place and acts on it after: you at a
shell, a session running `/launch`, or a tool of your own. flume runs
the kind through on its own and tells the caller each end. It waits on
the caller only where a hold, a question or a failure says so, and it
says which, since when, and in whose words.

## Who answers a place

Each place has recipients, who are told how its stages end. A launch
takes them from the first of these that gives any:

1. `--recipient`, once per recipient: a terminal's session by its name,
   or a role's address, `<stream>/<role>`, of any stream
   ([roles and wake](/flume/guides/roles-and-wake/));
2. `$FLUME_RECIPIENT` in the launching command's environment;
3. the terminal session the launch ran in: under herdr,
   `herdr:<pane>@<socket>`, the pane that ran `flume launch`;
4. the answerer behind the place's stream, the role a source of the host
   declares to stand behind it
   ([the answerer behind a stream](/flume/guides/roles-and-wake/#the-answerer-behind-a-stream)).

A launch that finds none of the four is refused, with nothing made, so
no place runs with nobody to tell:

```console
$ flume launch --kind fix --name the-parser-reads-tabs --brief "The parser drops a tab after a quote."
refused: the launch finds no one to answer it: it names no --recipient, $FLUME_RECIPIENT is unset, the terminal names no session it runs in, and no source of the host declares a role standing behind the stream default; give it one of the four
```

A launch is refused too where it would take the answerer and two roles
stand behind its stream, naming both, and where it names a role's
address that no source declares, or two declare, while no answerer
stands behind its stream. Where an answerer stands behind it, such a
launch is taken: the address's ends stay pending, status says it wakes
nothing, and its questions reach the answerer at their stall.

The recipients are fixed at the launch, and status gives them with
where they came from:

```text
  recipients: east/operator, from the stream's answerer, declared by acme
```

Three more may hear of a place's question, each in its turn:

- **the role its question names.** A stage's agent may put one
  question to a role of any stream, `flume needs --to <role>`, and that
  role alone is told ([stopping short](#stopping-short));
- **the answerer behind its stream**, at its stall, besides its
  recipients and the role its question named ([stalls](#stalls));
- **whoever answers it once**, by the question's key
  ([the answer's key](#the-answers-key)).

`flume wait` and `flume inbox` act as a recipient the same way:
`--recipient`, else `$FLUME_RECIPIENT`, else the role a role's run
serves, else the terminal session the command runs in.

## Ends

flume records an **end** each time something a caller would act on
happens:

| End | What it says |
|---|---|
| `<stage> delivered` | the stage's deliverable exists and its agent is quiet |
| `<stage> finished` | a conversation ended by `flume finish` |
| `<stage> needs an answer` | the agent stopped short with a question, in its words, and until when its agent is kept open for the answer |
| `<stage> found work` | the agent stopped short with work a stage before its own must do again, in its words; where it waits on an answer, until when its agent is kept open for the answer |
| `<stage> never begun` | the agent never said it started, or never took its prompt; one that says so later goes on, its stage begun |
| `<stage> lost` | the agent stopped with nothing delivered; a conversation's goes on in its session at a retry ([a lost conversation](#a-lost-conversation)) |
| `<stage> stuck` | the agent stayed quiet past the stage's limit with nothing delivered |
| `submission landed <commit>` | the merge queue landed the place's work |
| `submission failed` | the merge queue's checks failed, or the work conflicts with its base, with the merge queue's words and any contested files |
| `submission withdrawn` | the submission left the merge queue without landing |
| `submission refused` | the merge queue refused the submission |
| `stalled: <move>` | the place waited on its caller under no hold's words past its bound |
| `dropped`, `done` | the place was dropped, or declared done; sent as no notice, it waits for each recipient that is no role's address ([the end of a place's work](#the-end-of-a-places-work)) |

Each end is a **delivery** to each of its recipients, recorded once
inside the settle that reads it, and **pending** for each recipient
until that recipient confirms it by its key. flume hands it over: once,
as a one-line notice through the terminal into the recipient's own
pane, and again by a wait or the inbox whenever asked. A hand-over
delivers nothing. A recipient that was handed an end and stopped before
it acted on it is handed it again by its next inbox, and the end stays
pending until it confirms it. The notice to the recipient `me` reads,
for example,
`flume: fix--the-parser-reads-tabs (The parser drops a tab after a quote.) code delivered [stage/code/1/delivered]; read: flume status fix--the-parser-reads-tabs; confirm: flume confirm --recipient me fix--the-parser-reads-tabs/0a1b2c3d/stage/code/1/delivered`.
It names the place, the first line of its brief cut at 60 characters,
what ended, the end's key, the command that reads it whole, and how to
confirm it: as the recipient it was sent to, by name, and by its
delivery's key. The agent in a pane runs under its pane's own name,
which is not the recipient's name when the end was owed to a role's
address or to a session by its name, so the command names the
recipient. The command that reads it is
`flume inbox` where the place now waits on a move of its caller's,
which gives the question and how to answer it, and
`flume status <place>` otherwise. flume records each hand-over, to whom,
when and by what. A recipient asking a question, or one the terminal
cannot read or find, gets nothing typed into its pane, and a later
settle sends the notice once it can. A notice goes to a recipient once,
while nothing else has handed it the end. Under the printed command
nothing is ever typed. Status shows each end, whom it was handed to,
and whom it is pending for, each recipient under it:

```text
  ended stage/code/1/delivered: code delivered; handed to me; pending for me
    me: pending
```

A recipient confirms an end by its delivery's key, which the notice,
the inbox, a wait and status each give with the command that confirms
it as that recipient:

```console
$ flume confirm fix--the-parser-reads-tabs/0a1b2c3d/stage/code/1/delivered --recipient me
fix--the-parser-reads-tabs/0a1b2c3d/stage/code/1/delivered confirmed by me at 1794150120
```

`flume inbox --confirm` and `flume wait --confirm` confirm what they
give as they give it, so a caller that acts on what it collects does it
in one call. A place whose work is over keeps only its record while an
end of it stays pending, and the record goes once each end is
confirmed by each recipient.

An end nobody confirms is never dropped. Once the host's span,
`[host] ends_expire`, a week where unset, passes since the end was
read, status shows it **expired** with its recipient, and the host's
status lists every expired end. A name nothing was ever confirmed under
shows as never confirmed, so a misspelt recipient stands out, and the
operator confirms its ends as that name:

```text
  ended submission/1/landed: submission landed d185a879b51dba33555c036392b50443f4dfc5e8; pending for bomdrive
    bomdrive: pending, expired since 1794754920, never confirmed
```

## flume wait

`flume wait <place>` waits as a recipient on one stage's end,
`--stage <stage>`, or on the submission's outcome, `--submission`:

```console
$ flume wait fix--the-parser-reads-tabs --submission --recipient me --confirm
~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs submission landed d185a879b51dba33555c036392b50443f4dfc5e8 for me: handed over and confirmed by this wait
```

- An end pending for the recipient is answered at once and recorded
  handed to it by the wait. It stays pending until the recipient
  confirms it, as `--confirm` does with the hand-over.
- One the recipient confirmed is answered as read, handing nothing
  over.
- Otherwise the wait follows the host's state until an end comes, or
  until `--timeout` passes, as `<n>`, `<n>s`, `<n>m` or `<n>h`.

It exits `0` for a stage delivered or finished or a submission landed,
`2` for any other end, and `3` for still waiting when the timeout
passes. flume keys a wait by the place, never by a process or a pane, so
a caller whose session restarted waits again and loses nothing. While
a caller keeps watch over the host, `flume settle --every`, the wait
only reads; while none does, it settles every place before each
reading after its first.

## flume inbox

`flume inbox` gives every end pending for a recipient, on every place
of the host, and records each handed to it. It confirms nothing: the
next inbox gives them again, until the recipient confirms each, by
`flume confirm` or by `--confirm`, which confirms each as it gives it:

```console
$ flume inbox --recipient me --confirm
fix--the-parser-reads-tabs code delivered [stage/code/1/delivered]; read: flume status ~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs; confirmed
$ flume inbox --recipient me
nothing pending for me
```

Beneath what it collected, the inbox gives every place that waits on
its caller and on the recipient: one it is a recipient of, one whose
stop-short named it, or one whose stall reached it as the answerer
behind its stream. Each comes with the move it waits on and since when,
the stop-short's words and key, the role it named, and a hold on the
move. The inbox marks nothing for these, so a recipient that collected
a question and then restarted still finds it:

```console
$ flume inbox --recipient east/ops
nothing pending for east/ops
change--plan (one of two designs), in east, waits on east/ops; read: flume status ~/.local/share/flume/places/f5f5e14159a8/change--plan
  waits on its caller to retry construct (answer) since 1793287734: Which design does the brief mean?
  its stop-short stage/construct/1/ended-short needs an answer, asking east/ops
```

A session that starts runs it to catch up on what it missed. Under
herdr, flume's Claude Code plugin does that at a session's start, as the
pane the session runs in, confirming on receipt, and hands the agent
what the inbox gives
([a session of your own](/flume/guides/claude-code/#a-session-of-your-own)).
Reading `flume status` hands nothing over and confirms nothing.

### The end of a place's work

A caller that reads a place's last state from flume, rather than from a
notice in a pane, names itself one more recipient at the launch, by a
name no pane holds, as `--recipient ci`. Naming any recipient drops the
defaults, so it names the others the place's ends are for beside it:

- flume sends a name no pane holds no notice. Each end of the place
  waits for it until it confirms it, as for a recipient whose session is
  gone.
- A drop or a done is an end too, keyed `over/dropped` or `over/done`.
  flume sends no notice of it to any recipient, and wakes no role for
  it. It waits for each recipient that is no role's address, and for no
  one where every recipient is one.
- Once the place is tidied, its record stays while an end is pending,
  so `flume status <place>` still reads it:

```text
  recipients: me, ci, from the launch
  ended stage/code/1/delivered: code delivered; handed to me; pending for ci
    ci: pending
    me: confirmed at 1794150120
  ended submission/1/landed: submission landed d185a879b51dba33555c036392b50443f4dfc5e8; handed to me; pending for ci
    ci: pending
    me: confirmed at 1794150130
```

The caller then collects and confirms them by naming the place:

```console
$ flume inbox fix--the-parser-reads-tabs --recipient ci --confirm
fix--the-parser-reads-tabs code delivered [stage/code/1/delivered]; read: flume status ~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs; confirmed
fix--the-parser-reads-tabs submission landed d185a879b51dba33555c036392b50443f4dfc5e8 [submission/1/landed]; read: flume status ~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs; confirmed
```

An inbox naming a place collects that place's ends alone, from the
records `flume status` reads under the name: the place's own, else
those kept once it was tidied, `<place>@<at>` naming one. It reads
them before it moves anything on, so it collects what the caller read
and nothing tidied since. A name under which no record stands is
refused, and nothing is handed over. A `flume wait` as that name
collects the end it waits on too. The next settle removes the record
once every end of it is confirmed by each recipient, and lets go of
their deliveries. A caller that never confirms keeps the record on the
host, its ends shown expired once the host's span passes; `flume drop`
of it is refused while an end is pending, naming each recipient and
`flume confirm`.

## Holds

A hold stops a kind running through at one stage, or at the
submission, with your words saying what it waits for. At the launch:

```console
$ flume launch --kind chore --name tidy-the-readme --brief "Say what the parser reads." \
    --hold code="Wait until the release is cut."
launched ~/.local/share/flume/places/f5f5e14159a8/chore--tidy-the-readme
~/.local/share/flume/places/f5f5e14159a8/chore--tidy-the-readme: chore@1 (flume), live
  branch chore/tidy-the-readme from main, head d185a879b51dba33555c036392b50443f4dfc5e8
  stage code: on_hold, started 0 times
  submission: none, submitted 0 times, sent back 0 times
  waits on its caller to retry code (on_hold) since 1793287734: Wait until the release is cut.
  code on hold since 1793287734: Wait until the release is cut.
  recipients: me
```

`--hold <stage>=<words>` holds a stage, and `--hold-submission` the
submission. After the launch, `flume hold` does the same:

```sh
flume hold change--csv-export --stage verify "Wait until the schema change is reviewed."
flume hold change--csv-export --submission "Hold until the release is cut."
```

`flume hold` takes a stage not yet started since the current stage
began, and a stage waiting for room leaves the queue. It refuses a
stage already started, a submission already made and not yet decided,
a second hold on the same thing, and a place whose work is over.

Only a retry naming the stage or the submission ends a hold:

```sh
flume retry chore--tidy-the-readme --stage code
flume retry change--csv-export --submission
```

flume watches nothing for a hold's event and ends none itself. The
words are yours to make good: they can name a review, a release or
anything flume cannot see.

A stage whose declaration says `wait = true` is held the same way,
with no words, until a retry names it.

### A hold on the waiting move

A place that waits on its caller stalls once nobody's words say who
is on it. Whoever takes the move
says so with a hold on it, `--waiting`, and an optional bound,
`--for`, in the same times as `--stall-after`:

```sh
flume hold change--plan "east-ops has it" --waiting --for 1h
flume hold change--plan "passed to west/ops" --waiting --for 1h
flume hold change--plan "put to a person, waiting on their answer" --waiting --for 24h
```

While the hold holds, the place is not stalled, so no stall of it
reaches anyone. A place already stalled reads not stalled once
the hold is put. A later hold on the move replaces the words, the bound
and since when. Status gives the hold beneath the move:

```text
  waits on its caller to retry construct (answer) since 1793287841: Which design does the brief mean?
  the move it waits on on hold since 1793287900, for 3600 seconds: east-ops has it
```

The hold ends once the place waits on another move or none, whatever
made the move: a retry, a drop or a stage that went on. Where it gives
a bound, the settle that reads the bound passed lapses it: status says
it lapsed and when, `lapsed at <time>`, and the place stalls at once,
with the lapsed hold's words in its stall. Where it gives none, it
holds until the move is made. A lapsed hold holds nothing and stays in
status until the move is made or a later hold replaces it.

`flume hold --waiting` refuses a place that waits on no move of its
caller's, naming the place. A bound is the waiting move's alone: a hold
on a stage or the submission that gives `--for` is refused.

**Hold before you put a question to a person.** A person's answer can
take a day. Hold the move first, saying the question is going to a
person, then hold again naming where it waits, with a bound of a day or
more. The place then stalls to nobody while the person has it, and
stalls again at the bound if their answer never comes.

## Stopping short

An agent that cannot deliver says why, from inside the place:

```sh
flume needs "Which README, the root's or the parser's?"
flume needs --found "The tokenizer test depends on a fixture nobody committed."
```

`flume needs` runs only in an agent flume started, with its start's
secret. The stage ends short with the words whole, and status says
whether it needs an answer or found work:

```text
  stage code: ended_short, started 2 times, saying: Which README, the root's or the parser's?
  waits on its caller to retry code (answer) since 1793287841: Which README, the root's or the parser's?
    its agent kept open for the answer until 1793291441: retry with --words, or tell it
  ended stage/code/2/ended-short: code needs an answer; pending for me
```

A question starts no next stage: the place waits on its answer. Found
work is work a stage before the agent's own must do again. From the
kind's last stage it starts the kind's fix round on its own, as a failed
submission does, where the fix round is a stage before it, as a
change's `code` is before its `verify`
([kinds](/flume/guides/kinds/#how-a-kind-runs-through)). From any other
stage, and from a last stage that is its kind's fix round, as fix's and
chore's `code` is, no stage before it can do the work again: the place
waits on its answer as a question does, the findings its question.

A stop-short is a stage's agent's one way to ask. Every prompt's way to
stop short says so: the agent writes no card and no message to a person
itself, since whoever answers the question decides whether it is a
person's ([prompt parts](/flume/reference/prompt-parts/#what-flume-adds)).

### Asking a role

An agent's question may be another role's than the place's recipients':
a reading of a live system, say, where the place's owner plans the
work. Every prompt lists, after its way to stop short, each role the
agent may name, with what each answers in its source's words: every
role a source of the host declares with `answers`, the place's own
stream's first by the role alone, and every other stream's after by its
whole address. A role's run is not offered its own address.

```text
Where your words are for one of these roles rather than for the place's recipients, name it with `--to`, as `flume needs --to <name> "<words>"` or `flume needs --found --to <name> "<words>"`, the name written as listed here:
- `operator`: Whatever reaches the stream's work from outside it, and a question no role took up.
- `ops`: A live system: what is deployed, a credential, a deploy, a landing that failed.
- `west/ops`: A live system of the stream west.
```

```sh
flume needs --to ops "Which queue is deployed on the staging host?"
flume needs --to west/ops "Is west's deploy of the parser still running?"
```

A role alone, `ops`, is the place's own stream's; an address,
`west/ops`, is that stream's. flume reads the list from the sources at
each start, and records it with the start. A name the prompt did not
list is refused, naming those it did, and the stage goes on, so the
agent can ask again:

```text
refused: the stop-short names `design`, which its prompt did not list: it may name `operator`, `ops` and `west/ops`, or none
```

The stop-short's end goes to that role alone, in place of the place's
recipients: into its run's pane, or pending, waking its run in its own
stream. Status names the role it asked:

```text
  stage construct: ended_short, started 1 times, asking west/ops, saying: Is west's deploy of the parser still running?
```

With no `--to`, the end goes to the place's recipients. flume reads
nothing of the words to choose a role: the agent picks, and a role that
takes a question not its own passes it on
([an answering run's way with a question](/flume/guides/roles-and-wake/#an-answering-runs-way-with-a-question)).

## The agent kept open

The agent that stopped short needing an answer stays open for it, for
an hour unless its `keep_open` setting says otherwise ([how long an
agent is kept open](/flume/guides/agent-settings/#how-long-an-agent-is-kept-open)).
Its stage still reads ended short, waiting on its answer. Its notice
says until when:

```text
flume: chore--tidy-the-readme (Say what the parser reads.) code needs an answer, its agent kept open for the answer until 1793291441 [stage/code/1/ended-short]; read: flume inbox
```

While it is kept open, the agent holds the host's room, its stream's
and its account's, as a working agent does. An agent is kept open only
where its multiplexer can type words into a running agent: under herdr,
and never under the printed command, where its stage waits on a retry
as before.

### The answer in its session

While the agent is kept open, a retry with `--words` naming its stage,
or a tell to the place, is the stage's answer. flume hands it to the
agent that asked, as a tell is handed: once the agent waits, read back
as received, or sent once more. The agent reads the question and the
answer:

```text
The question: Which README, the root's or the parser's?

The answer: The root's README.
```

```console
$ flume retry chore--tidy-the-readme --words "The root's README."
$ flume tell chore--tidy-the-readme "The root's README."
```

Either way, the stage carries on in that session under the same start,
with everything the agent had read and decided. No agent starts and
none is ended, and the stage's start count does not move. flume records
the answer on the stop-short it answered, and status gives each answer
the stage's start took in its session beneath the stage:

```text
  stage code: begun, started 2 times
    answered by retry, received at 1793288012: asked Which README, the root's or the parser's?; answered The root's README.
```

An answer the agent does not take within the act's bound, ten minutes
or a tell's `--within`, is refused with nothing more typed, and the
stage still waits on its answer.

The stage then runs as any running stage does. Its quiet limit runs
from the answer. It may stop short again: that is an end of its own,
delivered once, and its agent is kept open again for its own time. A
`flume wait` after an answer waits on that next end, never on the one
the answer answered.

A retry with no words is no answer: it ends the agent once quiet, or
with `--force`, and starts a fresh one with the question alone. To have
a fresh agent take your answer while the agent is kept open,
`flume stop <place>` it first, then retry with your words.

### Let go

A settle lets the agent go, and records when and why, once the first
of these holds:

- its caller stopped it, by `flume stop`;
- it stopped, or its session ended, as when the host or the terminal
  restarts;
- its account was spent, its own turn or another agent's having reached
  the account's limit;
- its time passed.

Once the agent is quiet, flume ends it, never forced, and its room and
its account go to whatever waits for them. The stage keeps waiting on
its answer, and the place's wait keeps its since: a place that stalled
is not told so again. Status says how it was let go and whether it is
ended:

```text
  waits on its caller to retry code (answer) since 1793287841: Which README, the root's or the parser's?
    its agent let go at 1793291441, its time passed, and ended at 1793291441: retry with --words
```

flume keeps no timer of its own. A settle reads the time passed: one
herdr's watch runs as any agent changes state, the settle every command
that acts runs first where no watch is kept, or your own `flume
settle`. Until one does, status gives the agent kept open with its time
past.

Once the agent is let go, a retry with `--words` starts a fresh agent,
whose prompt carries the question and the answer ([retrying](#retrying)).
A tell is refused with nothing typed, naming that retry:

```text
refused: the stage code of ~/.local/share/flume/places/f5f5e14159a8/chore--tidy-the-readme waits on an answer and its agent is no longer kept open; give the answer as a retry: flume retry ~/.local/share/flume/places/f5f5e14159a8/chore--tidy-the-readme --words <answer>
```

A conversation's agent let go because it stopped with its host or its
terminal goes on in its session at the retry, as [a lost
conversation](#a-lost-conversation) does. An agent the terminal brought
back in a conversation's pane is left there, `and left in its pane`,
and only your `flume stop`, `drop` or `done` ends it.

## Retrying

`flume retry <place>` starts the current stage again with a fresh
agent, `--stage <stage>` another, and the stages after it run again in
order. `--words` gives your answer, after the question its stage
stopped on. While the stage's agent is kept open, the retry hands both
to that agent instead ([the answer in its session](#the-answer-in-its-session)).
Otherwise:

```console
$ flume retry chore--tidy-the-readme --words "The root's README."
```

The new agent's prompt carries both:

```text
This stage has been sent back to you with these words:

The question: Which README, the root's or the parser's?

The answer: The root's README.
```

A retry ends the place's agent first; `--force` ends one that is not
quiet. A retry never ends an agent whose stage has started or begun:
it is refused, forced or not, naming the agent, and
`flume stop <place>` ends it first. `flume retry <place> --submission`
submits the place's head again and starts no agent. A tell answers a
stage only while its agent is kept open; otherwise only a retry
restarts a stage that ended.

### A lost conversation

A conversation's stage reads `<stage> lost` when its agent stops with
nothing delivered, as when the host or the terminal restarts. Its
retry goes on in the lost agent's session, with its conversation as it
stood ([the conversation
carried](/flume/guides/agent-settings/#the-conversation-carried)):

```console
$ flume retry ask--the-parsers-history --stage talk --words "Carry on where we stopped."
```

The terminal may have brought the lost agent back in its pane as it
restarted, or a person may have resumed the session in a pane of their
own. flume leaves either as it is: it sends that process nothing, and
ends none of it. The lost end says so: `talk lost, and flume left the
agent the terminal brought back in its pane`. Only your `flume stop`,
`drop` or `done` ends an agent left so.

While a process runs the session, the retry is taken at once, ends
nothing, and its stage waits, as for room and never stalled:

```text
  talk waits for its conversation to end since 1793290000: process 4242, started 1793289400
```

Once that process ends, the next settle starts the stage in the same
session, so what a person said there goes on with the retry. Nothing
they sent is lost. A drop or a done of the place keeps its worktree
while a process runs its conversation, `kept while its session runs`,
naming the process. A retry of a lost stage that is not a conversation
starts afresh from what its place holds.

## The answer's key

Every end has a key, which every notice, every collected end, every
waiting place the inbox gives and status give:

| End | Key |
|---|---|
| a stage's | `stage/<stage>/<start>/<state>`, as `stage/construct/1/ended-short`, with `/<answered>` after it where the start had taken that many answers in its session, so a second stop-short of one agent kept open has a key of its own |
| a submission's | `submission/<n>/<outcome>`, as `submission/1/landed` |
| a stall's | `stall/<since>`, as `stall/1793288141` |

A stop-short's key names its start, so a fresh agent that asks again
asks under a new key.

An answer may name the key of the stop-short it answers, by a retry or,
while its agent is kept open, a tell:

```sh
flume retry change--plan --stage construct --answers stage/construct/1/ended-short --words "The second design."
flume tell change--plan --answers stage/construct/1/ended-short "The second design."
```

flume takes it only while the place waits on that stop-short and no
answer was taken for it. It checks the key before anything is ended,
started or typed, so an answer refused for its key changes nothing. The
refusal names the answer taken, who gave it and when, and the move the
place waits on now:

```text
refused: the stop-short stage/construct/1/ended-short of the place ~/.local/share/flume/places/f5f5e14159a8/change--plan was answered by east/ops at 1793288012: The second design.; an answer is taken once, and it waits on no move of its caller's now
```

Two answers given at once are taken once as well. Before an answer is
typed into the agent kept open, or before a fresh agent is started to
carry one that names its key, flume claims the stop-short for it. A
second answer that finds the claim is refused with nothing typed,
naming who gives the first and when it was claimed. The claim goes once
the first answer is recorded, or once the agent did not take it, so a
later answer is then taken.

A key naming another stage than the retry's, or an answer with no
words, is refused too, and a key in any other form is refused naming
the forms. The answer so taken is recorded on the stop-short's end
with its words, the key, and who gave it: the recipient the answering
command acts as, `$FLUME_RECIPIENT`, else the role of the run it runs
in, else the terminal's session. Status gives it beneath the end:

```text
  ended stage/construct/1/ended-short: construct needs an answer; answered at 1793288012 by east/ops, by a retry naming its key: The second design.
```

An answer with no `--answers` is taken as before, so two answerers
naming no key can both act. Name the key wherever more than one
recipient, role or answerer may see the question.

## Stalls

A place waits on its caller only as status says: which move, since
when, and the hold's or the agent's words. Some waits carry no
hold's words, such as a stage never begun or a fix round past its
bound. A place whose agent shows a dialog before it takes its prompt
waits on a person to answer it in the agent's pane, with the dialog's
words. A place waiting so is **stalled** after five minutes, or the
launch's `--stall-after`, and its stall is delivered once to each of
these:

- each recipient of the place;
- the role its stop-short named by `--to`, of whichever stream;
- the answerer behind the place's own stream, where one stands, and
  never a role's run's own address.

The stall carries the move, and where the move answers a stop-short,
its words, whether it needs an answer or found work, its key and the
role it asked, so a reader who never saw the place has the question
whole:

```text
  ended stall/1793288141: stalled: a retry of construct; its stop-short needs an answer, asking west/ops, saying: Is west's deploy of the parser still running?; pending for east/operator, west/ops
```

A hold on the waiting move keeps the place from stalling while it
holds, and its lapse stalls the place again, a stall with a key of its
own, carrying the lapsed hold's words, to the same recipients
([a hold on the waiting move](#a-hold-on-the-waiting-move)). A place
waiting for room on the host is never stalled, and a hold you put never
stalls.

## The other acts

| Command | What it does |
|---|---|
| `flume stop <place>` | ends the place's agent and nothing else; the stage waits for a retry or a drop |
| `flume drop <place>` | gives the work up: ends its agent, starts nothing more, and tidies the place, keeping a branch with commits not landed |
| `flume done <place>` | declares the work of a kind that does not land complete, and tidies the place |
| `flume finish <place>` | ends a conversation's agent; the conversation counts as delivered |
| `flume priority <place> <n>` | sets the place's priority in the host's queue |
| `flume tell <place> <words>` | hands the agent your words as a prompt, once it waits; to a stage whose agent is kept open, they are its answer, `--answers <key>` naming the stop-short they answer |

`stop`, `drop`, `done` and `finish` end an agent only once it is
quiet, unless you pass `--force`. Under the printed command flume
cannot end an agent: the act is refused until the agent's own session
ends where you ran it.

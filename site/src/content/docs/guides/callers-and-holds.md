---
title: Callers, holds and ends
description: Who is told how a place's stages end, how flume delivers an end once or keeps it pending, flume wait and flume inbox, holds with words, stops short, the agent kept open for its answer, retries and stalls.
sidebar:
  order: 4
---

A caller is whoever launches a place and acts on it after: you at a
shell, a session running `/launch`, or a tool of your own. flume runs
the kind through on its own and tells the caller each end. It waits on
the caller only where a hold, a question or a failure says so, and it
says which, since when, and in whose words.

## Recipients

Each place has recipients, who are told how its stages end. A launch
names them with `--recipient`, once per recipient. With none, the
recipient is `$FLUME_RECIPIENT`, else the terminal session the launch
ran in: under herdr, `herdr:<pane>@<socket>`, the pane that ran
`flume launch`. A recipient may also be a role's address, which
[roles and wake](/flume/guides/roles-and-wake/) covers.

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
| `<stage> found work` | the agent stopped short with work it cannot deliver past, in its words; from a stage that is not the kind's last, until when its agent is kept open for the answer |
| `<stage> never begun` | the agent never said it started, or never took its prompt; one that says so later goes on, its stage begun |
| `<stage> lost` | the agent stopped with nothing delivered; a conversation's goes on in its session at a retry ([a lost conversation](#a-lost-conversation)) |
| `<stage> stuck` | the agent stayed quiet past the stage's limit with nothing delivered |
| `submission landed <commit>` | the merge queue landed the place's work |
| `submission failed` | the merge queue's checks failed, or the work conflicts with its base, with the merge queue's words and any contested files |
| `submission withdrawn` | the submission left the merge queue without landing |
| `submission refused` | the merge queue refused the submission |
| `stalled: <move>` | the place waited on its caller under no hold's words past its bound |

Each end is delivered **once** to each recipient, inside the settle
that reads it, as a one-line notice through the terminal into the
recipient's own pane, such as
`flume: fix--the-parser-reads-tabs code delivered; read: flume status fix--the-parser-reads-tabs`. flume records on the place that it was
delivered, to whom and when. A recipient asking a question, or one the
terminal cannot read or find, gets nothing typed into its pane; the end stays
**pending** for a later settle, a wait or the inbox. Under the printed
command nothing is ever typed, so every end stays pending until
collected. Status shows each end and whom it is pending for:

```text
  ended: code delivered; pending for me
  ended: submission landed d185a879b51dba33555c036392b50443f4dfc5e8; pending for me
```

A place whose work is over keeps only its record while an end of it
stays pending, and the record goes once each end is delivered.

## flume wait

`flume wait <place>` waits as a recipient on one stage's end,
`--stage <stage>`, or on the submission's outcome, `--submission`:

```console
$ flume wait fix--the-parser-reads-tabs --submission --recipient me
~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs submission landed d185a879b51dba33555c036392b50443f4dfc5e8 for me: delivered by this wait
```

- An end pending for the recipient is answered at once and recorded
  delivered by the wait.
- One already delivered is answered as read, delivering nothing again.
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

`flume inbox` collects every end pending for a recipient, on every
place of the host, prints each and marks each delivered in the same
write, so two inboxes at once deliver each end once:

```console
$ flume inbox --recipient me
fix--the-parser-reads-tabs code delivered; read: flume status ~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs
$ flume inbox --recipient me
nothing pending for me
```

A session that starts runs it to catch up on what it missed. Reading
`flume status` delivers nothing.

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
  ended: code needs an answer; pending for me
```

A question starts no next stage: the place waits on its answer. Found
work from the kind's last stage starts the kind's fix round on its own,
as a failed submission does
([kinds](/flume/guides/kinds/#how-a-kind-runs-through)). Found work
from any other stage waits on its answer as a question does.

## The agent kept open

The agent that stopped short needing an answer stays open for it, for
an hour unless its `keep_open` setting says otherwise ([how long an
agent is kept open](/flume/guides/agent-settings/#how-long-an-agent-is-kept-open)).
Its stage still reads ended short, waiting on its answer. Its notice
says until when:

```text
flume: chore--tidy-the-readme code needs an answer, its agent kept open for the answer until 1793291441; read: flume status chore--tidy-the-readme
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

## Stalls

A place waits on its caller only as status says: which move, since
when, and the hold's or the agent's words. Some waits carry no
hold's words, such as a stage never begun or a fix round past its
bound. A place whose agent shows a dialog before it takes its prompt
waits on a person to answer it in the agent's pane, with the dialog's
words. A place waiting so is **stalled** after five minutes, or the
launch's `--stall-after`, and each recipient is told that once. A
place waiting for room on the host is never stalled, and a hold you
put never stalls.

## The other acts

| Command | What it does |
|---|---|
| `flume stop <place>` | ends the place's agent and nothing else; the stage waits for a retry or a drop |
| `flume drop <place>` | gives the work up: ends its agent, starts nothing more, and tidies the place, keeping a branch with commits not landed |
| `flume done <place>` | declares the work of a kind that does not land complete, and tidies the place |
| `flume finish <place>` | ends a conversation's agent; the conversation counts as delivered |
| `flume priority <place> <n>` | sets the place's priority in the host's queue |
| `flume tell <place> <words>` | hands the agent your words as a prompt, once it waits; to a stage whose agent is kept open, they are its answer |

`stop`, `drop`, `done` and `finish` end an agent only once it is
quiet, unless you pass `--force`. Under the printed command flume
cannot end an agent: the act is refused until the agent's own session
ends where you ran it.

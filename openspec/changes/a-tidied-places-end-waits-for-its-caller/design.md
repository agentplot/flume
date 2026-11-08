# Design

## Context

A place's ends are recorded on its held state in the write that reads
them (`crates/flume-core/src/held.rs`, `record_ends`): each stage's
start delivered, ended short, finished, lost, never begun or stuck;
each outcome of its submission but a refusal at submit; a landing read
from the merge queue's record; a stall. An end stands pending for each
of its recipients until a delivery to that name is recorded on it, by a
notice, a wait or the inbox (`crates/flume-seams/src/held.rs`,
`EndHeld::recipients`, `pending`, `delivered_to`). A drop or a done
records no end: it makes the work over (`acts.rs`, `give_up`) and the
place is tidied.

One settle reads a landing and tidies the place (`settle.rs`,
`settle_place`): the submission's step makes the work over, the next
step runs `tidy_place` (`tidy.rs`), which removes the worktree and the
branch, delivers the place's ends (`ends.rs`, `deliver_ends`) and then
keeps the document apart, as `<location>@<at>`, only while an end of it
stands pending (`keep_for_ends`). A later settle removes a kept
document once nothing of it is pending (`settle_tidied`). `flume status
<place>` reads the live document, else the documents kept for it
(`status.rs`, `status`; `ends.rs`, `tidied_of`), and `flume inbox`
collects every end pending for its recipient on the host (`wait.rs`,
`inbox`).

Record 0060 decides that a launch names its caller and that the end of
the place's work waits for that caller. The statement says it at
agentplot/blueprints `design/flume-a-tidied-places-end-waits-for-its-caller`,
b08a773: 7 and 8 (a launch's caller, fixed), 43 (the end of the work is
the caller's, collected by the inbox or a wait, never by a notice, waking
no role; a drop or a done an end for the caller alone; an inbox naming a
place), 56 (status gives the caller), 62 (the document stays for it),
the runner's row of the callers table, and S121.

The rules this design relies on are `docs/architecture/core.md`'s;
there is no area page. Rule core.25 is amended for it.

### What the family does

Read on 2026-11-08 at each tool's main.

- **Boomdrive's runner** launches with `--recipient` only where its
  control plane names one, runs every `flume` command with
  `FLUME_RECIPIENT` unset so a place's recipients are only its launch's,
  and carries each place's `flume --json status` on every pass, a tidied
  place's `P@<at>` among them (`openspec/specs/work-into-flume/spec.md`,
  "Flume runs with no recipient of the runner's own", "A place tidied
  while the pass answers its wants"). Its change
  `the-runner-carries-each-places-end` waits on this one to pass
  `--caller` and collect.
- **crew** reads a place's end only from the runner's report (crew
  record 0056, point 4).
- **sluice** records a landing as an entry of its own, which the runner
  carries too. It cannot say a place was dropped or done, and a landing
  read before flume tidied is the same fact read twice.

So a place's end has one owner, flume, which keeps it until the caller
that launched the place has read it; the runner carries it, and crew
reads the runner. Nothing of flume's reads the runner or crew.

## Goals / Non-Goals

**Goals:**

- A place's caller reads the place's last state, landed, dropped or
  done, from flume at any moment after its tidy, until it says it has.
- A caller collects what it has read and nothing tidied since.
- Nothing changes for a place whose launch names no caller.

**Non-Goals:**

- The runner's passing `--caller` and collecting, Boomdrive's change.
- A refusal at submit as an end: it leaves the place waiting on its
  caller, and status gives it (112).
- Any limit on how long a tidied document waits for a caller that never
  collects.

## Decisions

### 1. A launch names its caller

`Launch` and `PlaceRecord` gain `caller: Option<String>`, the second
`#[serde(default, skip_serializing_if = "Option::is_none")]`, so held
state written before reads with none (75). `flume launch --caller
<name>` and the launch's JSON form give it. The launch keeps it as
given (8). An empty name refuses the launch before anything is made
(core.8). It is any name: a session's, a role's address or a word.
flume never seeks it in a terminal and never wakes it. A wake's launch
names none (132), and so does a launch that names none, whatever
`FLUME_RECIPIENT` or its session give (7). Status gives it on
`PlaceStatus::caller`, skipped where none, and the person's form prints
`caller: <name>` after the recipients' line (56; core.14).

Not taken: reading the caller from `FLUME_RECIPIENT` or the session.
Every person's launch would then keep each tidied document until that
session's next inbox, and the runner would have to set the variable,
which makes it the recipient of every launch that names none (7). Not
taken either: the runner as a `--recipient`, which drops the defaults,
the answerer among them (7, 193), and is owed every end.

### 2. The end of a place's work, and a drop's or a done's end

The end of the place's work is its submission's landed end, as
`record_ends` writes it now from the verdict or from `Phase::Over` or
`Phase::Gone` with `Over::Landed`, or a new end, `EndOf::Over { over }`,
which `record_ends` writes once where the phase is `Over` or `Gone` with
`Over::Dropped` or `Over::Done`. It is written in the same write as the
phase, so a repeat records nothing (74; core.17). Its key is
`over/dropped` or `over/done`, which `EndKey` reads back from that token
and refuses in any other form, `over/landed` among them. `END_KEY_FORMS`
names it, and `what_ended` words it `dropped` or `done`.

`EndHeld::recipients` gives an over end none, so `deliver_ends` sends
no notice of it and `wake_roles` wakes no role for it: the place's
recipients learn no drop, as today (core.28).

### 3. Whom an end is pending for

`EndHeld::pending(recipients)` stays what a notice and a wake are owed.
Beside it, the end is owed to the caller where it is the end of the
work and no delivery to the caller's name by `Inbox` or `Wait` is
recorded on it. A `Notice` to that name counts for none, so a caller
that is also a recipient keeps its end pending after its notice, and
its inbox or its wait collects both at once. Whom an end is pending for,
given the place's recipients and its caller, is its recipients pending
then the caller where owed, each once. It is what status's
`EndStatus::pending_for`, `HeldState::pending_for`, the inbox and the
wait read. `deliver_ends` and `wake_roles` read the recipients' pending
alone, so no notice is sent to the caller and a caller written as an
address wakes no run (43, 132; core.11, core.28).

### 4. The tidied document

`keep_for_ends` and `settle_tidied` read `HeldState::pending_for`,
which names the caller, so the document is kept, and `Phase::Tidied`'s
`pending_for` names the caller, until the caller collects (62). Its
`caller` is kept with the rest of the place's record. A drop of the
document removes it as now (62). The list leaves a landed or dropped
place out unless asked for every place, as now (core.52).

### 5. An inbox naming a place

`Engine::inbox(recipient, place: Option<&PlaceId>)`. With a place, it
reads the documents status reads under that name: the live document,
else the documents `tidied_of` gives, `<location>@<at>` naming one
document. The lookup is one function that status and the inbox share
(core.14). It collects only their ends and gives only their entries of
`waiting`. Where no document stands under the name it is refused, as
status refuses it, and marks nothing. With none, it reads every place,
as now. The session's own inbox at its start names none (C.4).

So the runner collects a place it has carried, never one a settle tidied
after its listing was read. The whole inbox would collect that place's
end and remove a document the runner never carried.

### 6. A failed or refused submission

A failure sends the place back to its fix round or leaves it waiting on
its caller (112, 131), and a refusal at submit leaves it waiting on its
caller. Either way the place stands and status gives the outcome (56).
A drop that follows is the end of its work, and the kept document holds
the failure beside it. So the brief's ends all read from flume: landed
and dropped from the tidied document, failed and refused from the
standing place.

### 7. The JSON form

The form gains `caller` on a place and in a tidied document, an end of
`of` `over`, the key form `over/<dropped|done>`, and the caller in an
end's and a tidied phase's `pending_for`. Nothing is removed or renamed
(61; core.14).

## Risks / Trade-offs

- [A caller that never collects keeps its tidied documents on the host]
  → status and `flume list --all` give them, pending for that caller,
  and a drop removes one (62). Only a caller that names itself is kept
  for.
- [A reader of the form that rejects an unknown `of`] → the form only
  gains values (61). The herdr plugin reads a place's line, which this
  does not change, and the runner carries status whole.
- [Two changes touch `status.rs`] → the open change
  `a-hosts-full-status-read-takes-seconds` changes how status reads its
  places. This one adds only the lookup of a name's documents, shared
  with the inbox, and one field.

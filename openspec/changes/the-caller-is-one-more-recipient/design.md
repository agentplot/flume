# Design

## Context

The bolt's first change, `a-tidied-places-end-waits-for-its-caller`,
archived at 98738b16, gave a launch a caller apart from its recipients
(`Launch::caller`, `PlaceRecord::caller`), and an end of the place's
work owed to it by a rule of its own (`EndHeld::for_caller`,
`pending_for_caller`, `owed`). It recorded a drop or a done as an end,
`EndOf::Over`, keyed `over/dropped` or `over/done`, with no recipient,
and gave the inbox a place.

Record 0060, rewritten on the user's answer of 2026-11-08, folds the
caller into the recipients. The statement says it at agentplot/blueprints
`design/flume-a-tidied-places-end-waits-for-its-caller`, over b08a773:
43 (a recipient no pane holds, a drop's or a done's end, an inbox naming
a place), the runner's row of the callers table, and S121. Clauses 7, 8,
56 and 62 read there as on main. Rule core.25 is amended for it.

### What the family does

Read on 2026-11-08. Boomdrive's runner launches with `--recipient` where
its control plane names one, a role's address, and runs `flume` with
`FLUME_RECIPIENT` unset; its change `the-runner-carries-each-places-end`
waits on this one to name itself a recipient and collect by `flume inbox
<place>` once its report holds the place. crew reads a place's end only
from the runner's report (crew record 0056). sluice records a landing,
and cannot say a place was dropped. So flume owns a place's end and
keeps it for each recipient until that recipient collects it.

## Goals / Non-Goals

**Goals:**

- One list of whom an end is owed to: the place's recipients.
- A reader names itself a recipient and reads a tidied place's last
  state, landed, dropped or done, until it collects it.
- No role's run is woken, and no document kept, for a drop alone.

**Non-Goals:**

- The runner's naming itself and collecting, Boomdrive's change.
- One inbox shared across the family, weir's to design next.
- A wait answering a drop: a wait names a stage or the submission, and
  the inbox collects a drop's end.

## Decisions

### 1. No caller

`Launch::caller`, `PlaceRecord::caller`, `PlaceStatus::caller`, `flume
launch --caller` and the person's form's `caller:` line go, with the
empty caller's refusal. `EndHeld::for_caller`, `pending_for_caller` and
`owed` go; `pending_to` takes the recipients alone, and `EndStatus::of`
and `HeldState::pending_for` read `pending` again. A wake's launch had
none, so nothing replaces it.

Held state the bolt wrote with a `caller` reads with it ignored, as
serde reads a field it does not know, and is written back without it.
The form main gives never had `caller`, so nothing leaves it (61;
core.14); `FIELDS` in the conformance suite loses its `caller` paths and
keeps the `over` ones.

### 2. A recipient no pane holds

Nothing changes in the settle. `deliver_ends` seeks each recipient
(`reached`); a name no terminal session bears is found nowhere, so no
notice is sent and nothing is claimed, and the end stays pending for it,
as for a recipient whose session is gone. `keep_for_ends` and
`settle_tidied` keep the document while `HeldState::pending_for` names
anyone. The spec now says it of a name no pane ever held.

### 3. A drop's or a done's end

`EndHeld::recipients` gives an over end the place's recipients that are
not addresses (`is_address`), each once: it is the drop's end alone,
never a stop-short's addressee or a stall's answerer. `EndHeld::noticed`
is false for it and true for every other end. `deliver_ends` skips an
end not noticed, so no notice of a drop reaches a pane, a session's
among them. `wake_roles` reads addresses from `pending_for`, which holds
none of an over end. So a drop pending for the runner keeps the
document until the runner's inbox; one pending only for role addresses
is pending for no one, and the tidy keeps nothing.

Not taken: a drop pending for role addresses too, which would wake a
role's run (132) to hear what a caller did, or keep a document for a run
that collects only at its start. Not taken: a drop sent as a notice to a
session in a pane, which the user's answer rules out ("never sent as a
notice"). A session recipient so keeps a dropped place's document until
its inbox, its session's start collecting it (C.4).

### 4. An inbox naming a place

Unchanged from the bolt: `Engine::inbox(recipient, place)` reads the
documents status reads under the name before its settle, and collects
only theirs.

## Risks / Trade-offs

- [The runner is owed every end of its places] → it collects them with
  the place's by `flume inbox <place>`, once its report holds the place.
- [A session's dropped place is kept until its inbox] → its start
  collects it (C.4); status and `flume list --all` give it, and a drop of
  the document removes it (62).

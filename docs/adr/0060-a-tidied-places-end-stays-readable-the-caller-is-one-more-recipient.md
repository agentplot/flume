---
number: 60
title: A tidied place's end stays readable, the caller one more recipient
status: proposed
date: 2026-11-08
decision-makers:
- Chuck Swanberg
---

# A tidied place's end stays readable, the caller one more recipient

Status: proposed on 2026-11-08 by the code stage of the fix `a-tidied-places-end-stays-readable--the-caller-is-one-more-recipient`, on the user's answer of the same day on agentplot-design's card. It is proposed until the user has seen it as a record, and accepted on their yes.

Deciders: the user, whose answer on the card was "Runner as one more recipient now; one shared inbox in weir next." agentplot-design's brief carries it: "Fold the caller into the recipients. A launch names no caller and flume launch takes no --caller. The runner names itself a recipient beside the role it names today, so no default recipient is dropped (7). A recipient no pane holds is sent no notice, and its ends stand pending until it collects them by flume inbox or a wait. A drop or a done is recorded as an end, as built, but is never sent as a notice, wakes no role, and stands pending for no role address, so no role's run is woken and no tidied document is kept for a role alone. flume inbox <place> stays, so a reader collects only what it has carried. The tidied document stays while an end stands pending (62)." The code stage's own: how a drop's end names its recipients in held state, and that `caller` leaves the form unreleased.

Sources: `agentplot/blueprints:design/flume/requirements.md` on main at 871ff7d, clauses 7, 8, 43, 52, 56, 62, 69, 114, 132 and 193, and as amended for this record on the branch `design/flume-a-tidied-places-end-waits-for-its-caller` at 479a6ab, over b08a773; agentplot/crew's record 0056 at 9c71cd16, under which crew learns a place's end only from the runner's report; agentplot/boomdrive's `openspec/specs/work-into-flume/spec.md` on main at e82cd25, "Flume runs with no recipient of the runner's own" and "A place tidied while the pass answers its wants"; the stop-short of `change--the-runner-carries-each-places-end`'s construct, 2026-11-08; the change `a-tidied-places-end-waits-for-its-caller`, archived on `bolt/a-tidied-places-end-stays-readable` at 98738b16; `crates/flume-core/src/ends.rs` (`deliver_ends`, `keep_for_ends`, `settle_tidied`), `standing.rs` (`reached`), `wake.rs` (`wake_roles`), `wait.rs` (`inbox`) and `held.rs` (`record_ends`); `crates/flume-seams/src/held.rs` (`EndHeld::recipients`, `EndHeld::pending`).

## Context and problem statement

The runner launches each place its control plane asks for and carries the place's status back on every pass. crew reads a place's end only from that report (crew record 0056). Once a place is tidied, the runner can read it only from the document flume keeps while an end of it stands pending (62). That document is gone before the runner's next pass whenever nothing of it stands pending.

One settle reads a landing and tidies the place (`settle_place`, `tidy_place`, `keep_for_ends`). The runner names the work's role its recipient, `madswan/planner`, and runs every `flume` command with `FLUME_RECIPIENT` unset, so a place's recipients are only those its launch names. The landing reaches the planner's run as a notice, nothing stays pending, and the document goes inside the settle that read the landing. Fix agentplot/485 went this way on 2026-11-08 between two passes a minute apart: its last carried status reads live, the next status was refused, and crew read the fix as submitted. A drop recorded no end at all, so a dropped place's document went at its tidy whatever its recipients.

The bolt `a-tidied-places-end-stays-readable` first built a caller apart from the recipients: `flume launch --caller <name>`, the end of the place's work pending for that name alone. The user's answer folds it into the recipients.

## Considered options

1. **A caller apart from the recipients.** `flume launch --caller <name>` keeps the name with the place. The end of its work, its landing, drop or done, stands pending for that name until it collects it by `flume inbox` or a wait. No notice goes to it, and a notice to a recipient of the same name does not count for it.
2. **A set time.** flume keeps every tidied place's document for a set time after its tidy, whatever stands pending, long enough for a reader's next pass.
3. **The runner one more recipient.** The runner names itself a recipient beside the role it names, `--recipient <its name>`. No pane holds that name, so it is sent no notice, and every end of the place stands pending for it until it collects it, the tidied document with it (62). A drop or a done is an end that no notice carries and no role is woken for.

## Decision outcome

Option 3, as the user answered: "Runner as one more recipient now; one shared inbox in weir next."

- **No caller.** A launch names no caller, and `flume launch` takes no `--caller`. A reader that reads a place's last state from flume names itself a recipient beside the others its launch names (7). The runner already names a role's address, so naming itself beside it drops none of the defaults: naming any recipient already dropped them (7, 193).
- **A recipient no pane holds is sent no notice.** The settle seeks each recipient in the terminal (`reached`). A name no session bears is found nowhere, so no notice goes to it. Its ends stay pending until it collects them by `flume inbox` or a wait as itself (43). That is how flume already treats a recipient whose session is gone. The tidy keeps the place's document while an end of it stands pending (62).
- **A drop or a done is an end no notice carries.** It is recorded as the bolt built it, `over/dropped` or `over/done`, in the write that makes the work over. Its recipients are the place's recipients that are no role's address. No notice is sent of it to any of them, and no role is woken for it. So it stands pending for the runner, or for a session that launched the place, until each collects it, and for no one where every recipient is a role's address. No role's run is woken to hear what a caller did, and no tidied document is kept for a role alone.
- **An inbox may name a place.** `flume inbox <place>` stays as the bolt built it, so a reader collects only the documents it read under that name, never one a settle tidied after it read its listing (43, 62).

Option 1 gives flume two lists of whom an end is owed to, with two rules for counting it delivered, where one list serves: a name no pane holds is already a recipient no notice reaches. Option 2 keeps documents no one reads for the whole time. A reader slower than the time still loses the end. The time is a guess, and clause 62 keeps a tidied document only while an end is pending.

### Consequences

- Good: the runner reads every place it launched, landed, dropped or done, at its next pass, whatever settle or person tidied it, and collects only what its report holds.
- Good: one list of recipients, one way of counting an end delivered, and no field of the form for a caller.
- Bad: the runner's name is owed every end of its places, each stage's and each stall too, and collects them all by `flume inbox <place>`.
- Bad: a session recipient whose place is dropped or declared done keeps the place's document until that session collects the end. Its session's next start collects it (C.4), and a drop of the document removes it (62).
- Neutral: `caller` and `--caller` never reached main: the bolt held them alone, so the form main gives loses no field (61). Held state the bolt wrote with a `caller` reads with it ignored.
- Neutral: the statement is amended for it on `design/flume-a-tidied-places-end-waits-for-its-caller` at 479a6ab, over b08a773 (43, the runner's row, S121), not yet on blueprints' main; clauses 7, 8, 56 and 62 read there as on main. Rule core.25 carries it.
- Neutral: the runner naming itself a recipient and collecting after it saves its report is Boomdrive's change `the-runner-carries-each-places-end`, which waits on this one.
- Neutral: one inbox shared across the family is weir's to design next, as the user's answer says. This record leaves it open.

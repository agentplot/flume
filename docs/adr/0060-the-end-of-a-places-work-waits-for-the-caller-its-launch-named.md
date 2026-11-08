---
number: 60
title: The end of a place's work waits for the caller its launch named
status: proposed
date: 2026-11-08
decision-makers:
- the construct stage of the change a-tidied-places-end-waits-for-its-caller
---

# The end of a place's work waits for the caller its launch named

Status: proposed on 2026-11-08 by the construct stage of the change `a-tidied-places-end-waits-for-its-caller`. Its direction is the answer the change's brief gives to the question the construct of Boomdrive's change `the-runner-carries-each-places-end` asked on 2026-11-08: option 1, "Flume keeps a launched place's tidied record for its caller until the caller collects it, for instance an end pending for the caller collected by flume inbox". How flume knows the caller, which ends are the caller's and how the caller collects one are the stage's own choices, none of them the user's alone; the design agent settles them.

Deciders: the construct stage, on the brief: "A place's end, landed, failed, refused or dropped, stays pending for the caller that launched it until the caller collects it with flume inbox, so the document of a tidied place stays readable by its caller, as flume 62 keeps it while an end stands pending."

Sources: `agentplot/blueprints:design/flume/requirements.md` on main at 871ff7d, clauses 7, 8, 43, 52, 56, 62, 69, 114 and 193, and as amended for this record on the branch `design/flume-a-tidied-places-end-waits-for-its-caller` at b08a773; agentplot/crew's record 0056 at 9c71cd16, under which crew learns a place's end only from the runner's report; agentplot/boomdrive's `openspec/specs/work-into-flume/spec.md` on main at e82cd25, "Flume runs with no recipient of the runner's own" and "A place tidied while the pass answers its wants"; the stop-short of `change--the-runner-carries-each-places-end`'s construct, 2026-11-08; `crates/flume-core/src/settle.rs` (`settle_place`), `tidy.rs` (`tidy_place`), `ends.rs` (`deliver_ends`, `keep_for_ends`, `settle_tidied`), `wait.rs` (`inbox`) and `acts.rs` (`give_up`); `crates/flume-seams/src/held.rs` (`record_ends`, `EndHeld::pending`).

## Context and problem statement

The runner launches each place its control plane asks for and carries the place's status back on every pass. crew reads a place's end only from that report (crew record 0056). Once a place is tidied, the runner can read it only from the document flume keeps while an end of it stands pending (62). That document is gone before the runner's next pass whenever nothing of it stands pending. The runner had no way to make something stand pending for it.

One settle reads a landing and tidies the place: the submission's step reads it landed and makes the work over, the next step tidies, delivers the place's ends and keeps the document only for the recipients still owed an end (`settle_place`, `tidy_place`, `keep_for_ends`). The runner names the work's role its recipient, `madswan/planner`, and runs every `flume` command with `FLUME_RECIPIENT` unset, so a place's recipients are only those its launch names. The landing reaches the planner's run as a notice, nothing stays pending, and the document goes inside the settle that read the landing. Fix agentplot/485 went this way on 2026-11-08 between two passes a minute apart: its last carried status reads live, and the next status was refused. crew read the fix as submitted. A drop records no end at all, so a dropped place's document goes at its tidy whatever its recipients.

The statement already speaks of ends pending for a caller "of the places it launched" (43), but a launch carried no caller, and flume read only recipients.

## Considered options

1. **A launch names its caller, and the end of the place's work waits for it.** `flume launch --caller <name>` keeps the name with the place. The end of its work, its landing or a drop or a done, stands pending for that caller until the caller collects it by `flume inbox` or a wait as itself. A drop or a done is an end for the caller alone. An inbox may name one place.
2. **The caller read from the launching command,** `$FLUME_RECIPIENT`, else the session it runs in.
3. **The runner names itself a recipient,** `--recipient <its name>`.
4. **Every end of the place pending for its caller,** not the end of its work alone.
5. **A drop or a done delivered to the place's recipients too.**
6. **The caller collects by the whole inbox, or by `flume drop` of the tidied document.**

## Decision outcome

Option 1.

- **The caller.** A launch may name its caller, `--caller <name>`: a name of the launching caller's own, a session's, a role's address or any other word, never empty. It is fixed at the launch (8), kept on the place's record as `caller`, given by status and kept in a tidied place's document. It is no owner: any caller acts on the place as before (69). A role's run, launched by a wake, has none, and a launch that names none keeps nothing for anyone.
- **The end of a place's work is the caller's.** Its landing is its submission's landed end, as now. Its drop or its done is recorded as an end too, `over/dropped` or `over/done`, in the write that makes the work over, its one recipient the caller and none where no caller is named. The place's recipients are sent no notice of it and no role is woken for it, as today, when they learn no drop.
- **It counts delivered to the caller only by the caller's own read.** A delivery to the caller's name by `flume inbox` or by a wait counts. A notice never does: no notice is sent to the caller, and one sent to a recipient of the same name leaves the caller's end pending. A caller's end wakes no role, whatever its name.
- **The document stays until then.** The tidy keeps a place's document while an end of it stands pending for a recipient or for its caller, and the settle that reads it collected removes it (62).
- **An inbox may name a place.** `flume inbox <place>` collects the ends pending for its recipient of the documents `flume status <place>` reads under that name, and gives that place's waiting entry alone. `<place>@<at>` names one tidied document. A name under which no document stands is refused as status refuses it.
- **A failed or refused submission** leaves the place standing, sent back to its fix round or waiting on its caller (112, 131), and status gives its outcome as before. The drop that may follow is the end of the place's work, and its kept document holds the failure with it.

Option 2 keeps a tidied document for every session that launches a place until that session runs an inbox, which a person's session does only at its start. The runner leaves `FLUME_RECIPIENT` unset so that its launches' recipients are only those they name. Set, it would become the recipient of every launch that names none, in place of the answerer behind the stream (7, 193). Option 3 has the same fault, since naming any recipient drops the defaults (7). It also sends every end and stall to the runner, and leaves the runner's name sought as a session to notify. A drop would still record no end. Option 4 leaves an end pending for the caller on every stage of every place it launched, which no caller reads: a place's earlier ends stand in its live status (56), and its tidied document keeps every one. Option 5 wakes a role's run for an address with no run (132) only to tell it what a caller did. Under option 6, the whole inbox would collect the end of a place tidied after the runner read its listing and before it collected, so a document would go that the runner never carried. `flume drop` of the tidied document works today, but under a name that a place launched again also answers, a drop ends live work.

### Consequences

- Good: the runner reads every place it launched, landed, dropped or done, at its next pass, whatever settle or person tidied it, and collects only what its report holds.
- Good: a notice's `flume status <place>` still finds the document of a place with a caller while that caller has not collected.
- Bad: a caller that names itself and never collects keeps each tidied document of its places on the host, given by status and `flume list --all`, until it collects or a caller drops the document (62).
- Neutral: the form gains `caller` on a place, an end of `over`, and the key form `over/<dropped|done>`; nothing is removed from it (61).
- Neutral: the statement is amended for it on `design/flume-a-tidied-places-end-waits-for-its-caller` at b08a773 (7, 8, 43, 56, 62, the runner's row, S121), not yet on blueprints' main. Rule core.25 carries it.
- Neutral: the runner passing `--caller` and collecting after it saves its report is Boomdrive's change `the-runner-carries-each-places-end`, which waits on this one.

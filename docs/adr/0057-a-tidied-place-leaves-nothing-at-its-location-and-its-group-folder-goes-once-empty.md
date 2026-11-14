---
number: 57
title: A tidied place leaves nothing at its location, and its group folder goes once empty
status: accepted
date: 2026-11-07
decision-makers:
- chore/a-tidied-place-leaves-no-folder
---

# A tidied place leaves nothing at its location, and its group folder goes once empty

Status: accepted on 2026-11-08 by agentplot-design. Written by the code stage of the chore `a-tidied-place-leaves-no-folder`: its direction is the brief's and the statement's, and its choices are the stage's own, cheap to change and none of them the user's alone, so the design agent settles them.

Deciders: the code stage, on the brief: "Flume's tidy of a finished place removes the place's folder, its .devenv and .moon state with it, and the place's group folder once it holds no place".

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 00d90e9, clauses 62, 64 to 66, 74, 81 and C.1, and C.1 as this record amends it on blueprints' branch `design/flume-a-tidied-place-leaves-no-folder` at 723ab51, proposed; rules core.17 and core.18; record 0027, which put every place under flume's own data directory; git 2.54.0's `git worktree remove`, which drops a worktree's entry and its git directory even where deleting the worktree's folder failed, "there's no going back from here" (`builtin/worktree.c`); the places of deneb on 2026-11-07.

## Context and problem statement

On deneb on 2026-11-07, 14 tidied places' folders stood under `~/.local/share/flume/places/`: four in `b3e4c15d5ebe`, three in `d6933fb6e0c9`, two each in `a8147ee247e9` and `af10b39ad94a`, and one each in `061708ef066d`, `661ef9eae50a` and `8becac0add71`. Ten group folders stood empty. Each place's folder held part of what a devenv shell entered in the place writes there, `.devenv/state/prek`, `.devenv/profile`, `.devenv/run`, `.moon/cache/outputs` and `.pre-commit-config.yaml`, and no `.git`.

`git worktree remove` deletes the worktree's folder, then its git directory. Where any part of the folder's deletion fails it says `failed to delete`, goes on to delete the git directory, and exits non-zero. A folder it may not write in fails it. So does a process that writes into the place while git deletes it, since the folder git emptied is no longer empty when git removes it. The place profile read that exit as a refusal, so the tidy kept the place. The next tidy read the place gone, since git held no worktree there, and asked the place seam nothing. The held state went, and the folder and the group folder above it stood for good.

## Considered options

1. **What stands at a place's location once git holds no worktree there goes with the place.** The place profile's remove takes it after `git worktree remove`, whatever git exited, and for a place read gone. It removes nothing through a link, makes each folder writable where its removal is refused, and asks again, three times in all, while a process writing there races it. Then it removes the group folder, `places/<id>` under flume's own data directory, once that holds no other place. A tidy hands a place read gone to the place seam's remove, as it does a place read live and holding nothing, and a refused remove keeps the place for the next tidy.
2. **`git worktree remove --force`.** It deletes no more than the plain remove where the folder cannot be written or is written meanwhile. It also removes a worktree holding uncommitted or unignored work, which core.18 rules out.
3. **A sweep of flume's data directory for folders no place names.** It would clear the folders already standing. But flume would remove what its held state does not name, which core.17 and core.18 rule out. A folder at a location some other tool chose would be removed on a guess.

## Decision outcome

Option 1.

- **Only once git has let go.** The removal runs only where `git worktree list` names no worktree at the location, after the place was read holding nothing uncommitted, unignored or locked and its teardown passed (65, 66). Where a `.git` file there names a git directory that still stands, the location is some repository's worktree, and the remove is refused naming it. A `.git` folder, the repository's main worktree, is refused as before (64).
- **Never through a link.** `.moon/cache/outputs` links into the repository's common git directory, where every worktree's moon archives live. The link goes, and what it points at stays.
- **Three removals.** A process still writing into the place, an agent's background command among them, can race the removal once. A third failure refuses the remove with what could not be removed, the place kept as a failed teardown is (66), and the next tidy tries again.
- **The group folder.** `places/<id>` goes once it holds no place, by a removal that refuses a folder holding anything. A place at `<repository>/places/<kind>--<name>`, where an earlier flume made it (record 0027), leaves its repository's `places/` folder alone.
- **What already stands.** The 14 folders and ten group folders on deneb are named by no place's held state, so flume leaves them to the host's operator.

### Consequences

- Good: a tidied place leaves nothing on the host but a branch 67 keeps, as clause 62 says, and a launch of a place of the same kind and name is no longer refused by its leftover folder (3, 78).
- Good: a tidy repeated after a refused removal converges, because a place read gone is removed rather than forgotten (74).
- Bad: a process that writes into the place after its last removal, once the held state is gone, still leaves a folder. Nothing of flume's reads that location again.
- Bad: a leftover whose first removal is refused is walked twice, once to make it writable and once to remove it.

## Amendment, 2026-11-14: what git left of a failed removal goes at a later settle, and only from a folder flume made

Status: proposed. The direction is the brief of the change `a-place-whose-folder-git-dropped-is-removed`, which the planner queued from agentplot-design's signal. The choices below are that change's construct stage's own, each cheap to change and none of them the user's alone, so the design settles them.

Deciders: the planner's brief, afterthought/crew-state `plan.rec` at `agentplot/main` 8bd54b10: "A folder of Flume's own place that git no longer lists reads as removed, and Flume removes what is left of it itself at a later settle, never in the same call as git's failed removal and never a folder it did not make, so no place is held for good". agentplot-design's signal 2026-11-12-agentplot-design-a25d71de/01: "Flume's own teardown of its places runs the same git removal and could hit the same leftover; that is Flume's and is left open here, reported to the orchestrator." The construct stage.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at main 81288ba, clauses 64 to 66, 74, 76, 80, 81 and C.1; boomdrivedev/sluice's change `a-place-whose-folder-is-gone-is-freed` and its record 0115, on sluice's main since 7ce112b; `crates/flume/src/profiles/place/git_worktree/mod.rs` at 4e7fa04d.

### Context and problem statement

From 2026-11-10, sluice held three of agentplot/sluice's try checkouts on deneb for good. git's removal of each stopped at `Directory not empty`, dropped git's entry for the worktree, and every later try was refused with `is not a working tree` (sluice's record 0115). flume never asks git again to remove a place git no longer lists. It reads the place gone (80), and this record's removal takes what is left. But it took what was left in the same call as git's failed removal, three tries 100 ms apart, while the process that made git fail may still be writing there. It also took whatever stood at the location the place's held state names. It refused only a `.git` folder, or a `.git` file whose git directory stands.

### Considered options

1. **Refuse the failed removal, and take what is left at a later settle, only at a location of flume's that holds nothing of another's** (chosen).
2. **This record as it stood: take what is left in the same call.** It races the writer that made git fail. sluice takes the next round for that reason (its record 0115).
3. **Answer that the remove passed, and leave the folder.** The held state would go, and nothing would name the folder (76; core.17).
4. **Trust the location the held state names.** A document naming a folder flume never made, by a hand's edit or a fault, would have that folder removed whole.

### Decision outcome

Option 1.

- **Refused, at git's step.** Where `git worktree remove` fails and git then lists no worktree at the location, the remove is refused at `git worktree remove`. Its words are git's, then that git no longer lists the place and flume removes what is left of it at the next settle. Nothing more is removed in that call. The tidy keeps the place in its held state, named so in status (66, 76).
- **At a later settle.** Each settle steps a place whose work is over, a kept one among them, with no caller's word. The next reads the place gone, and its remove takes what is left as this record says. It runs no teardown again, since the teardown ran once, before git's removal. A `flume tidy` on a host a caller keeps watch over settles every place before the one it names, so its second settle may be the later one. That is still a later remove, never the one git failed in.
- **Where git passed.** A `git worktree remove` that passed is followed in the same call by the removal of what a process wrote there after it, as before.
- **Only from a folder flume made.** What stands at a location goes only where the location is directly in a group folder under flume's own `places/`, or in its repository's own `places/`, where an earlier flume made it (record 0027). It goes only where it holds no `.git` folder, and no `.git` file but one naming its repository's own worktree entry that no longer stands. Anything else is refused naming why, and what stands is left whole. A group folder under any id is flume's, so a place of a repository whose clone moved (record 0053) is still removed where it stands.

### Consequences

- Good: a writer still at work when git failed has until the next settle to finish. No place is held for good by git's refusal, since flume never asks git again for a place git no longer lists.
- Good: a held document naming a folder flume did not make has nothing removed.
- Bad: a place whose git removal failed stays listed, kept, until the next settle.
- Bad: a place made under a data directory the host has since moved has its leftover refused, named in status, and left to the host's operator.

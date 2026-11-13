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

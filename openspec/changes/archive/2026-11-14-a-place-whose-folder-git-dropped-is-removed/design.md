# Design

## Context

**The brief.** A folder of flume's own place that git no longer lists
reads as removed. flume removes what is left of it itself at a later
settle, never in the same call as git's failed removal and never a
folder it did not make, so no place is held for good.

**Sources.**
- Signal 2026-11-12-agentplot-design-a25d71de/01 (afterthought/crew-state,
  `agentplot/main`), and the planner's entry for this unit in
  `plan.rec` at `agentplot/main` 8bd54b10.
- boomdrivedev/blueprints `design/flume/requirements.md` at main
  81288ba: 64 to 66 (flume removes only what it made, keeps what holds
  work, and removes through the teardown), 74 (every act safe to
  repeat), 76 ("Every place flume made is in its held state until it is
  removed. No act leaves a worktree or a branch of flume's that its held
  state does not name."), 80 (read), 81 (remove, "or a refusal naming
  the step that failed"), and C.1's git worktree row.
- boomdrivedev/sluice's change `a-place-whose-folder-is-gone-is-freed`,
  archived on sluice's main since 7ce112b, and sluice's record 0115:
  gone read from git's own list, what is left removed by sluice at the
  next round, "never in the same call, right after `git worktree remove`
  fails. A writer that made git fail is likely still writing."
- This repository's record 0057 and rule core.18, at 4e7fa04d.

**flume today, at 4e7fa04d.**
- `GitWorktree::read`
  (`crates/flume/src/profiles/place/git_worktree/mod.rs`) gives `Gone`
  where `git worktree list --porcelain` names no worktree at the
  location, matched by canonical path, or names one whose folder is
  gone. A list that cannot be read is `CouldNotRead`, never gone.
- `GitWorktree::remove` refuses a location holding a `.git` folder,
  reads the place, and for a place read gone runs `git worktree prune`
  and `left`. For a place read live and holding nothing it runs the
  teardown, then `git worktree remove`, then reads git's list again.
  Where git lists no worktree there, whatever git exited, it calls
  `left` in the same call.
- `left` refuses where a `.git` file at the location names a git
  directory that still stands. Otherwise it removes what stands by
  `clear`, never through a link, three tries 100 ms apart, each folder
  made writable after the first. Then it removes the group folder once
  empty, where that is directly under `<own>/places`.
- `Engine::tidy_place` (`crates/flume-core/src/tidy.rs`) hands a place
  read live and holding nothing, or read gone, to the seam's remove. A
  refusal keeps the place, `Kept::Teardown { step, words }`, in its
  held state. Every settle steps a place whose phase is `Over`, so a
  kept place is tidied again at each settle (`settle.rs`, `step`).

So a flume place is not held for good today: a later tidy reads it
gone and never asks git again. What the brief changes is when what is
left goes, and what flume will remove.

## Goals / Non-Goals

**Goals:**
- What git left of a failed `git worktree remove` is removed at a later
  settle, never in the call git failed in.
- The place stays in its held state, named in status, until then (76).
- What stands at a location is removed only where the location and
  what it holds are flume's own.

**Non-Goals:**
- A change to the seam, the core, the fakes or the held state's form.
- A sweep of folders under `places/` that no held state names.
- A teardown run for a place read gone from the start.
- `git worktree prune`'s reach, which stays as it is.

## Decisions

### D1. A failed removal git no longer lists is refused at once

In `remove`, after `git worktree remove` and the second read of git's
list, the case where git failed and lists no worktree at the location
returns `Removed::Refused` and removes nothing more:

- `step`: `git worktree remove`, as every refusal of that step.
- `words`: git's words, then `; git no longer lists <location> as a
  worktree, and flume removes what is left of it at the next settle`.

The other cases stand: a list that cannot be read is refused with its
reason; git failing while it still lists the place is refused with
git's words; git passing while it still lists the place is refused
naming it; git passing and listing no worktree there calls `left`.

The tidy keeps the place, `Kept::Teardown`, which status gives with its
step and words, and the list as `kept by git worktree remove` (56).
Nothing new is held: the reason is the words.

Not taken:
- **Take what is left in the same call, as record 0057 had it.** The
  writer that made git fail, a devenv shell or an agent's background
  command, is likely still writing. Three tries 100 ms apart race it.
- **Answer `Removed` and leave the folder for later.** The tidy would
  let go of the held state, and nothing would name the folder again (76;
  core.17).
- **Also defer what a process wrote after a `git worktree remove` that
  passed.** That costs a kept place and a settle in the common case,
  where `left` finds nothing but the empty group folder. Git did not
  fail, so the brief does not ask it.

### D2. The later settle takes what is left

No code of the core changes. Every settle steps a place whose work is
over, a kept one among them, with no caller's word (`step`'s `Over`
arm). The next settle's tidy reads the place gone, since git lists no
worktree there (80), and hands it to the seam's remove, as record 0057
has it. The remove's gone path runs `git worktree prune` and `left`.
It runs no teardown, so the teardown, sluice's release where sluice is
the merge queue, ran once, before git's removal in the first call. A
`left` refused again keeps the place for the next settle, and each
settle tries once.

A `flume tidy` on a host a caller keeps watch over settles every place
(`settle_attended`) and then the place it names. Its second settle may
be the later one. That is a later call of the seam's remove, never the
one git failed in, which is all the brief asks.

### D3. Only a folder flume made

`left` checks two things before it removes anything, and only where
something stands at the location. Where nothing stands, it goes on to
the group folder as today.

1. **The location is one flume gives a place.** Its parent is a group
   folder directly under `<own>/places`, flume's own data directory, or
   its parent is `<repository>/places`, where an earlier flume made a
   place (record 0027). Paths are compared by `canonical`. A group
   folder under any id counts, so a place of a repository whose clone
   moved keeps its old id and is still removed where it stands (record
   0053). Otherwise the remove is refused: `<location> is in no folder
   flume makes places in, and flume leaves what stands there`.
2. **Nothing there says it is another's.** A `.git` at the location
   decides:
   - none: removed, as what a process wrote after git deleted the
     folder (record 0057);
   - a folder: refused, `<location> holds a repository of its own`;
   - a file naming a git directory that stands: refused as today,
     `<location> is the worktree of <gitdir>, which git still holds`;
   - a file naming a git directory whose parent is not the repository's
     own `<common>/worktrees`: refused, `<location> is a worktree of
     another repository, <gitdir>`;
   - a file naming the repository's own entry under
     `<common>/worktrees`, which no longer stands: removed, as what
     git's failed removal left;
   - a file that cannot be read or names no `gitdir:`, or a repository
     whose common directory cannot be read: refused, naming why.

   A relative `gitdir:` is read against the location, as `linked` reads
   it. `<common>` is `Git::at(repository).common_dir()`, compared by
   `canonical`. That resolves a parent that no longer stands, as
   `worktrees/` once git pruned its last entry.

The checks read files. They start no process but the one
`common_dir()` runs, and only where a `.git` file is there to judge.
Each refusal keeps the place, named in status, for the host's operator
(64, 66; core.18). flume sweeps nothing no held state names (core.17).

Not taken:
- **The exact location `location()` gives the place.** A repository
  whose clone moved gets another id, and its earlier places would all
  be refused.
- **Trust the held state's location.** That is today's reading. A
  document naming `$HOME`, by a hand's edit or a fault, would have it
  removed whole once git lists no worktree there.

### D4. The removal itself

`clear` and the group folder stay as record 0057 has them: never
through a link, each folder made writable where its removal is refused,
three tries, and the group folder removed only once empty and only
directly under `<own>/places`.

### D5. The seam and the statement

The seam's signatures and docs are unchanged. 81's remove is "its
removal with its repository's teardown run, or a refusal naming the
step that failed", and D1's refusal names `git worktree remove`. 83's
"all or nothing" is flume's own act: git dropping its entry before it
failed is git's, and the refusal says so. The place reads gone, as 80
reads it, and the next remove completes it. The fakes already stand for
it: a place the fake place `vanish`es and whose remove it refuses.
Rules core.1 and core.2 hold: git is named only in the profile.

The statement holds as written. C.1's remove is `git worktree remove`,
and how the profile takes what git leaves is the profile's own, as
record 0057 took it.

### D6. The family

sluice's record 0115 and record 0057's amendment state one rule for
each tool's own places. A folder git no longer lists is gone, and what
is left goes at the next round or settle, never in the call git failed
in. A read-only folder is made writable, and no link is followed. Each
tool removes only what it made, sluice its checkouts under its
`sluice/` folder and flume its places under `places/`. Neither removes
the other's, so neither calls the other. sluice's reading matches git's
entries by device and inode through gix. flume's matches them by
canonical path from `git worktree list`, which comes to the same for a
folder that stands. A shared crate for it would be the family's to
propose. This change does not.

### Rules relied on

- core.17: no act leaves a worktree or a branch of flume's its held
  state does not name (D1, D3).
- core.18: flume removes only places it made, through the teardown,
  never through a link (D3, D4); its details amended with this change.
- core.1, core.2: git named only in the profile, and the seam unchanged
  (D5).
- core.21: each task's check is one test, one test target or one
  task.

## Risks / Trade-offs

- [A writer that keeps writing into what is left] → each settle tries
  once, three removals, and keeps the place named with what could not
  be removed until the writer stops. That is record 0057's, one settle
  later.
- [A data directory the host moved] → a place made under the old one
  has its leftover refused, named in status, and left to the operator.
- [A `.git` file git wrote relative to the location] → D3 reads it
  against the location, as `linked` does, so the repository's own entry
  is still recognised.
- [A place held one settle longer] → the list shows it kept by `git
  worktree remove` until then.

## Migration Plan

Nothing to migrate: no held state or record changes form. A place kept
today by a refused `left` is tidied again by the next settle under the
new code, as before.

## Task notes

**1.1 The failed removal refused.** In `remove`, split today's `(_,
Ok(None)) => self.left(place)` arm into `(Ok(_), Ok(None))`, which
calls `left` as today, and `(Err(words), Ok(None))`, which returns
`refused` with D1's words. The module's doc comment and `remove`'s
comment say git's failed removal is refused and what is left goes at a
later remove (record 0057's amendment).

`what_git_left_of_a_failed_removal_goes_at_the_next_remove` replaces
`a_place_git_half_removes_leaves_no_folder` in
`crates/flume/tests/profiles/place_git_worktree.rs`:
1. A profile whose `PlaceData` has one teardown step, `Step::run`
   appending a line to a mark file in the scratch, outside the place.
   Make the place `half`, give it `devenv_writes`, and make the
   read-only `.devenv/state/stuck` folder holding `state`, as the test
   it replaces does.
2. The first remove is `Removed::Refused` at `git worktree remove`.
   Its words hold git's `failed to delete` and `no longer lists`. The
   location still stands, `profile.read` is `Gone`, and the mark holds
   one line.
3. The second remove is `Removed::Removed`. Nothing stands at the
   location, the empty group folder is gone, `outputs/archive` stands,
   and the mark still holds one line.
4. A third remove is `Removed::Removed` (74).

**1.2 Only a folder flume made.** `left` keeps D3's checks ahead of
`clear`. They are pure functions of the location, `own`, the repository
and the files there, kept beside `linked`, which they take over.

`only_a_folder_flume_made_goes_from_a_places_location` in the same
file, each part on its own scratch repository:
1. **Outside every `places/`.** A `PlaceRef` whose location is
   `<scratch>/elsewhere/fix--stray`, holding a file `kept`, with no
   worktree there. `read` is `Gone`. `remove` is refused naming the
   location, and `kept` stands.
2. **Another repository's dropped worktree.** At
   `profile.location(&repository, "fix--theirs")`, a second repository
   adds a worktree. Its entry is then removed by hand: read the `.git`
   file's `gitdir:` and remove that directory, as git's failed removal
   leaves it. `read` is `Gone`. `remove` is refused naming another
   repository, and the worktree's `README` stands.
3. **A repository of its own.** At `profile.location(&repository,
   "fix--clone")`, `git init` and a file. `remove` is refused, and the
   file stands.
4. **The place's own.** A place made with `made`, its entry removed by
   hand as in part 2, and a file written in it. `read` is `Gone`.
   `remove` is `Removed::Removed`, and nothing stands at the location.

The module's other tests show D3 accepts what flume made:
`what_a_process_wrote_at_a_gone_places_location_goes`,
`a_group_folder_holding_another_place_stays`,
`a_worktree_another_repository_holds_at_a_places_location_is_kept`,
and the tests of places an earlier flume made inside a clone and beside
a bare repository's worktrees.

**2.1 The later settle.** In
`crates/flume-conformance/src/engine/tidying.rs`, `a_place_read_gone`
keeps its first half: `vanish`, `fail_teardown` at `git worktree
remove`, and `done`, which keeps the place after one remove. After
`mend_teardown`, it settles the place with `world.engine.settle(Some(&id))`,
not a tidy. The answer is `Answer::Settled`, its `tidy.removed` names
the place, and the fake place's log counts two removes. The scenario's
name gains "and removed by the next settle", and its cites gain `76`.

**3.1 The site.** Each page says it in the reader's words (core.35):
- `reference/commands.md`, `flume tidy`, the worktree's bullet: where
  `git worktree remove` fails having let the place go, the place is
  kept with git's words, and the next settle removes what is left of
  its folder. Only a folder in a `places/` folder flume makes places
  in, holding nothing of another repository's, is removed so.
- `guides/places-and-herdr.mdx`, "Tidying", step 2: the same, after
  "Once git lets the place go".

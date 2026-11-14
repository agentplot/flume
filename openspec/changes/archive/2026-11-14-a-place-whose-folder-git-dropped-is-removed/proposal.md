# A place whose folder git dropped is removed

## Why

agentplot-design's signal of 2026-11-12 (afterthought/crew-state,
`agentplot/main`, 2026-11-12-agentplot-design-a25d71de/01): "Flume's
tidy removes a place's worktree with the same git removal that left
sluice three try checkouts held for good, git dropping its record of a
worktree part way through and refusing every retry as not a working
tree, so a Flume place could stay held the same way". sluice's change
`a-place-whose-folder-is-gone-is-freed`, on sluice's main since
7ce112b, reads such a folder as gone and removes what is left of it at
the next round (sluice's record 0115).

`git worktree remove` deletes the folder first and stops at its first
failure, a folder it may not write in or one a process writes into
meanwhile. It then drops its own entry for the worktree whatever that
did, and exits non-zero (record 0057). flume's place profile already
reads a place git no longer lists as gone, whether or not its folder
stands, and never asks git to remove it again, so no retry is refused
as sluice's were. Two things are left:

- **It takes what is left in the same call as git's failed removal.**
  `remove` goes straight on to `left`, three removals 100 ms apart,
  while the process that made git fail is likely still writing there
  (`crates/flume/src/profiles/place/git_worktree/mod.rs`). sluice takes
  it at the next round for that reason.
- **It takes whatever stands at the location the held state names.**
  `remove` refuses a `.git` folder and `left` a `.git` file whose git
  directory stands. Nothing checks that the location is one the place
  profile gives a place, or that a `.git` file there names the
  repository's own dropped worktree. A held document naming a folder
  flume never made, by a hand's edit or a fault, would have that folder
  removed whole once git lists no worktree there.

Every place flume made is in its held state until it is removed (76).

## What Changes

- **A failed removal git no longer lists is refused, and nothing more
  is done in that call.** Where `git worktree remove` fails and git then
  lists no worktree at the place's location, the remove is refused at
  its step, `git worktree remove`. Its words are git's, then that git no
  longer lists the place and flume removes what is left of it at the
  next settle. The tidy keeps the place in its held state, named so in
  status (66, 76, 81).
- **A later settle removes what is left.** Each settle steps a place
  whose work is over, a kept one among them, with no caller's word. It
  reads the place gone, as git lists no worktree there (80), and the
  remove takes what is left by record 0057's removal, running no
  teardown again. A remove refused again keeps the place for the next.
- **Only from a folder flume made.** What stands at a location goes
  only where the location is directly in a group folder under flume's
  own `places/`, or in its repository's own `places/`, where an earlier
  flume made it. It goes only where nothing there says it is another's:
  no `.git` folder, and no `.git` file but one naming the repository's
  own worktree entry that no longer stands. Otherwise the remove is
  refused naming why, the place kept and named in status, and what
  stands left whole (64; core.17, core.18).
- **A removal git finished is followed as before.** Where `git worktree
  remove` passes, what a process wrote at the location after it is
  still taken in the same call (record 0057).
- **The record and the rule.** Record 0057's amendment of 2026-11-14,
  proposed, and rule core.18's details, written with this change. The
  site's `flume tidy` reference and its tidying guide say it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `place-profile`: what git left of a failed removal goes at a later
  remove, never in the same call; what stands at a place's location
  goes only where the location and what it holds are flume's own.
- `tidying`: a place whose remove was refused is tidied again by each
  later settle, with no caller's word, until its remove passes (76).

## Impact

- `crates/flume/src/profiles/place/git_worktree/mod.rs`: `remove`'s
  case where git failed and lists no worktree at the location, and the
  guard `left` keeps before it removes anything.
- `crates/flume/tests/profiles/place_git_worktree.rs`: the test of
  git's half removal reworked and renamed, and one new test.
- `crates/flume-conformance/src/engine/tidying.rs`: `a_place_read_gone`
  has its second remove made by a settle.
- `docs/adr/0057-…md`'s amendment and `docs/architecture/core.md`'s
  core.18 details, written with this change.
- `site/src/content/docs/reference/commands.md` and
  `site/src/content/docs/guides/places-and-herdr.mdx`.
- No seam, core, fake, held-state form or command changes. No new
  crate, plugin or site part, so no new moon project, and no new
  dependency or tool. The statement holds as written (64 to 66, 76, 80,
  81, C.1).

### Out of scope

- **A sweep of folders under `places/` that no held state names**,
  those record 0057 left to the host's operator among them (core.17).
- **`git worktree prune`**, which the remove of a place read gone runs
  as before.
- **A teardown for a place read gone from the start**, whose worktree
  another removed. Its remove runs none, as before.

# A tidied place leaves no folder

## Why

On deneb on 2026-11-07, 14 tidied places' folders stood under
`~/.local/share/flume/places/`, four in `b3e4c15d5ebe`, three in
`d6933fb6e0c9`, two each in `a8147ee247e9` and `af10b39ad94a`, one
each in `061708ef066d`, `661ef9eae50a` and `8becac0add71`, and ten
group folders stood empty. Each folder held only what a devenv shell
entered in the place writes there: `.devenv/state/prek`,
`.devenv/profile`, `.devenv/run`, `.moon/cache/outputs` (a link into
the repository's common git directory) and `.pre-commit-config.yaml`,
with no `.git`.

`git worktree remove` deletes the place's folder and, where any part of
that deletion fails, still drops its entry of the worktree and exits
non-zero: a process that writes into the place while git deletes it, or
a folder git may not write in, leaves the folder standing. The place
profile read that exit as a refusal and the tidy kept the place; the
next tidy read the place gone, because git holds no worktree there, and
went on without asking the place seam to remove anything, so the held
state went and the folder stayed for ever, with the group folder above
it. Clause 62 keeps nothing of a tidied place; the folder is something.

## What Changes

- The place profile's remove takes what still stands at a place's
  location once git holds no worktree there, after `git worktree
  remove` or for a place read gone, never through a link, each folder
  made writable where its removal is refused, and asked again while a
  process writing there races it; then the place's group folder,
  `places/<id>` under flume's own data directory, once it holds no other
  place. It refuses where a `.git` file there names a git directory that
  still stands (62, 81, C.1).
- A tidy hands a place it reads gone to the place seam's remove, as it
  does a place read live and holding nothing; a remove refused keeps the
  place, named with its step, for the next tidy (62, 66, 81).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `place-profile`: nothing of a removed place stands at its location.
- `tidying`: a place read gone is removed all the same.

## Impact

`crates/flume/src/profiles/place/git_worktree/mod.rs` and
`crates/flume-core/src/tidy.rs`, with the profile's tests over scratch
repositories and a tidying scenario over the fakes. Record 0057,
proposed, and rule core.18's details. The site's tidying guide and
`flume tidy`'s reference. The statement's C.1, amended for record 0057
on agentplot/blueprints' branch
`design/flume-a-tidied-place-leaves-no-folder`, proposed. The folders already standing on
a host are not named by any place's held state, and flume leaves them to
the host's operator (core.17, core.18).

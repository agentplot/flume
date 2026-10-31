---
number: 27
title: A place is made inside no working tree of its repository, beside a bare repository's worktrees or under flume's own data directory for a clone
status: proposed
date: 2026-10-31
decision-makers:
- Chuck Swanberg
---

# A place is made inside no working tree of its repository, beside a bare repository's worktrees or under flume's own data directory for a clone

Status: proposed, 2026-10-31.

Deciders: the user, in the brief of the fix `a-place-is-never-inside-a-working-tree` on 2026-10-31: "a place is never made inside a working tree of its repository. Where the repository is a bare container (git directory beside its worktrees), places stay where they are now; otherwise they go beside the checkout or in flume's own data directory, the choice the statement's place profile allows [...] Places that already exist where they are now are read and tidied as before." The choice between beside the checkout and flume's own data directory, the directory's layout, and how a place is found by its kind and name are the fixing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 56bd21b: section 3's repository, clauses 2, 3, 8, 9, 78, 83, 132, C.1 (the make column, `<repository>/places/<kind>--<name>`), C.4 (an agent's identity names its repository by the id of its common git directory), C.7 (kinds' packages under `$XDG_DATA_HOME/flume/kinds/`), and section 9's storage rule, host state under the host's own directories "for config, state, data and cache, by its kind"; `agentplot/blueprints:design/capture/design.md`, "Where state lives" (data is what cannot be fetched again; flume's worktree layer is the place); records 0006 and 0014; `agentplot/sluice` record 0013 (a place is a plain git worktree, under `<common>/sluice/.places/`); rules core.3, core.8, core.16, core.17, core.18, core.19; `crates/flume-core/src/launch.rs` (the location built as `{repository}/places/{kind}--{name}`), `crates/flume/src/commands/place.rs` (the repository as the directory holding a bare repository beside its worktrees, or the main worktree).

## Context and problem statement

C.1 makes a place at `<repository>/places/<kind>--<name>`. flume names a repository by a directory: the one holding a bare repository beside its worktrees (`project/project.git`, `project/main`), or, for a clone, its main worktree, the directory above its common git directory. In the first layout `places/` sits beside the worktrees, inside none of them. In the second it sits inside the main worktree.

On deneb on 2026-10-31, flume 0.3.6 launched a place in a clone at `~/Code/github_agentplot/flywheel-cloud/main` and made it at `~/Code/github_agentplot/flywheel-cloud/main/places/research--the-domain-model`, inside the main checkout's working tree. There `git status` shows it untracked, the repository's formatters and task runner sweep it, sluice refuses the main worktree's untracked files, and a commit can take it. flume's site told a repository to list `places/` in its `.gitignore`. That hides the place from git, not from every tool that walks the tree, and asks something of every repository flume runs in.

## Considered options

1. Keep `<repository>/places/`, a clone's `.gitignore` listing it.
2. Beside the checkout: `places/` in the directory above it, or a directory named for the checkout beside it.
3. Under flume's own data directory: `$XDG_DATA_HOME/flume/places/<id>/<kind>--<name>`, the id the repository's.
4. Under the clone's common git directory, `<common>/flume/places/`, as sluice keeps its own places.

## Decision outcome

Option 3 for a clone. A bare repository's places stay beside its worktrees.

1. **Where.** The place profile says where a place is made (C.1), through the place seam's `location`. Where the repository's directory holds its bare repository beside its worktrees, the place is at `<repository>/places/<kind>--<name>`, as before. Otherwise the repository's directory is a working tree of it, and the place is at `$XDG_DATA_HOME/flume/places/<id>/<kind>--<name>`, `~/.local/share/flume/places/...` where `$XDG_DATA_HOME` is unset. The id is the repository's, the first twelve hex digits of its common git directory's SHA-256, which its agents' identities already carry (C.4). Option 2 writes into a directory neither the repository nor flume owns. Two clones side by side share it, so one's `fix--one` refuses the other's launch (3), and a clone nested in another repository's tree puts its places inside that one's. Option 4 keeps the place under the main worktree's directory, inside `.git`, where a tool that walks the tree with its hidden files finds it. A place holds work that cannot be fetched again until it lands, which the family's storage rule keeps in the host's data directory (section 9; the capture design's "Where state lives").
2. **Read and tidied where it stands.** A place's id is the location it was made at, kept in its held state, and it is read, submitted, removed and tidied there, wherever that is. A place named by `<kind>--<name>` is the one at the profile's location. Where the host's store holds no place there and holds one at `<repository>/places/<kind>--<name>`, it is that one, which a flume before this record made inside the main worktree. A role's run is found by its repository, kind and name, not by a location (3, 132).
3. **A launch that cannot say where.** The core asks the place seam where the place goes. A launch whose location cannot be found, as for a clone on a host where neither `$XDG_DATA_HOME` nor `$HOME` is set, is refused naming why, and makes nothing (2, 8).

### Consequences

- The place seam gains `location`, which every profile and fake implements. The conformance suite's place scenarios make each place where the seam says, and a scenario holds that a place's location is its own and the same at each ask.
- Nothing is asked of a repository: no `.gitignore` entry, and a clone's status shows none of its places.
- A clone's places lie under the host's data directory, backed up where the host backs data up, and away from its checkout. `flume status` names each place's location.
- Rule core.40 holds it.
- C.1 says `<repository>/places/<kind>--<name>` for every repository. Its make column is to be amended in the statement to say a clone's places are under flume's own data directory (core.19). Until it is, the code stands ahead of C.1, an item in `docs/architecture/_open.md`.

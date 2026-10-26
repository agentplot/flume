---
number: 4
title: Kinds are the family's one format, and a repository binds its profiles
status: accepted
date: 2026-10-22
decision-makers:
- Chuck Swanberg
---

# Kinds are the family's one format, and a repository binds its profiles

Status: accepted by the user on 2026-10-26, on their card.

Deciders: the user, in the capture design's rulings 3 and 12 and in flume's statement (the shared kind format; the `[profiles]` table; no-tool profiles by default, 4015b33).
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 18 to 24, 84, 85, 105, 106, C.1 to C.3, C.6, section 9; `agentplot/blueprints:design/weir/requirements.md` A.8, C.4; `agentplot/blueprints:design/capture/design.md`, "Launching: flume".

## Context and problem statement

flume, sounding and later Flywheel each run work through stages. If each declared stages its own way, a repository would declare the same work three times, and a kind written for one tool would mean nothing to the next. And flume meets five tools at its seams; if it guessed which from their files, a repository that uses none of them could not launch at all.

## Considered options

1. flume's own kind format, with sounding and Flywheel translating theirs.
2. One kind format defined and read once in the family's shared crates (weir), of which flume runs the places-and-stages part and ignores every section it does not know; a repository binds each seam's profile in the `[profiles]` table of `.config/flume.toml`, read at its default branch, with the profiles that need no tool where it binds none: plain `git worktree`, no change profile, no merge queue.
3. flume detects its profiles from the repository's files (`.config/sluice.toml`, `openspec/`, `.config/wt.toml`).

## Decision outcome

Option 2. A kind resolves to exactly one declaration, the launch's over the repository's over flume's own; a place runs its kind and keeps its binding as each resolved at its launch, and a later change to a declaration never moves a place already launched. flume's own declaration holds exactly two kinds, `fix` and `change`, and both land. A kind that lands where the repository binds no merge queue is refused at the launch.

Option 1 makes three formats of one idea. Option 3 makes a repository's tools decide flume's behaviour without the repository saying so, and the no-tool default impossible.

### Consequences

- flume links weir's kinds crate and defines no field of the format itself; `agents`, `deliverables`, `on_fail` and `max_send_backs` are the format's names (C.2).
- A kind whose stage names more than one agent or more than one deliverable is refused, since flume runs that much of the format and no more (23).
- A repository with no `[profiles]` table gets plain git worktrees and neither a change nor a merge-queue profile (C.2).

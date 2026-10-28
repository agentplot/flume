---
number: 15
title: The answers raised writing flume's first two changes - recipients at the launch, a pending end outlives the worktree, a conflict's fix merges the base, and every end is delivered
status: accepted
date: 2026-10-28
decision-makers:
- Chuck Swanberg
---

# The answers raised writing flume's first two changes

Status: accepted by the user on 2026-10-28, in their design session ("yes to all recommendations"; "change 2: yes to all"), on the questions the OpenSpec changes `flume-runs-a-kind-through` and `flume-runs-unattended-and-wakes-roles` raised.

Deciders: the user; the recommendations are the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at e5f92f6, clauses 7, 8, 14, 43, 62, 112, 130, 134, C.6, C.7, S1, S19; records 0002, 0007, 0009, 0012, 0013, 0014; rules core.5, core.11.

## Context and problem statement

Writing the OpenSpec changes for flume's first build and for running unattended found places the statement left open or contradicted itself: who a place's ends go to, what happens to a finished place whose end nobody has collected yet, how a fix resolves a conflict when flume merges nothing, whether an agent's death reaches anyone, what a terminal with no state of its own reads an agent's stop from, which terminal a host uses, and what a woken role's place is cut from.

## Considered options

The recommendations, each against the alternative the writing agents found: keep the worktree until every end is delivered, or keep only the place's document; recipients fixed at the launch or added after; the fix agent merges the base in, or flume moves the branch; lost and stuck reach only status, or are delivered as ends.

## Decision outcome

- **Recipients are named at the launch** (7, 8), the launching session by default as the terminal names it, `$FLUME_RECIPIENT` overriding it, a role run its address; a hold at the launch carries its words (`--hold <stage>=<words>`).
- **A pending end outlives the worktree, not the place's record** (62): a finished place is tidied as always, and its document alone stays until every end is delivered or its caller drops it.
- **A conflict's fix takes the base in by a merge its agent commits** (112, 14): the merge queue's words carry the base revision tested against; the send-back stage's agent merges that revision in its place, resolves the files contested, and commits, before anything else. It is the agent's commit in its place, as every commit is (14); flume merges nothing (core.5), the guard refuses no `git merge` into the place's branch, and nothing rebases.
- **Lost, never begun and stuck are delivered ends** (43, 130), with what flume read, so a role learns its place died; a notice waits out an open question and counts delivered on the terminal's answer.
- **The agent seam gains `ended`** (134), the program's own record that a session ended, which the printed command reads an agent's stop from.
- **The host names its terminal**, `[host] terminal`, `herdr` or `printed`, printed where it names none; another terminal, cmux among them, is another profile of the same seam. **`[roles]` takes `base` and an optional `brief`.**
- **A settle steps the queue whenever the submission is undecided** (C.6): beside a live `sluice run` the step finds the lock held and does nothing.
- **S1 and S19 follow the clauses**: flume tidies a landed place, and a failed change goes back to code.
- **Record 0007 is accepted** (it was, on 2026-10-27; `_open.md` still said proposed).

### Consequences

- Rules core.5 and core.11 carry it; `_open.md` drops record 0007's item.
- The two OpenSpec changes' tasks follow it; weir-kinds gains `makes` as the second change's first step of its made-things group.

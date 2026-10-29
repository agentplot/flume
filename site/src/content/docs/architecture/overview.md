---
title: How flume is built
description: A core that knows git, the seven seams it reaches everything else through, and the profiles that bind each one.
sidebar:
  order: 1
---

A core that knows git, and seven seams it reaches everything else
through. They say how a place is made, where kinds are declared, how a
change is read, how an agent is started and read, where it runs, how
work lands, and where state is kept. Each seam is bound by a profile, so one core
works with herdr or a printed command, with sluice or no merge queue,
with OpenSpec or no change tool.

## The statement

The design is a statement of invariants in
[agentplot/blueprints](https://github.com/agentplot/blueprints),
`design/flume/requirements.md`: what is always true of flume and what
it never does, as numbered clauses, written so that any program
satisfying them is flume. The
[design book](https://agentplot.github.io/blueprints/flume/) explains
it chapter by chapter. This repository cites its clauses by number,
keeps the [rules](/flume/architecture/rules/) a change must respect,
and records each decision the statement left open in `docs/adr/`, one
record each. A clause that no longer holds is changed in the statement
first, never worked around in code.

## The core

The core makes places, starts stages, reads their ends, submits,
delivers ends and tidies, using git alone. It holds nothing between
commands. Every act reads a place's held state with its version,
decides, and writes it back at that version; a write refused because
the version moved is read and decided again. So two commands acting on
one place at once leave it as one or the other would have, and every
act is safe to repeat.

A **settle** is the one act that moves places on: it reads every fact
of a place again and takes each step that is then allowed. It runs
inside a command, started by an agent going quiet, by herdr's watch,
or by a caller, and flume runs no process of its own between commands.

## The seams

| Seam | What the core asks of it | Profiles |
|---|---|---|
| **the place** | make a worktree on a branch from a revision; make it ready; read it; remove it, and its branch only when the base holds its commits | `git-worktree`: plain `git worktree`, readied and torn down by the merge queue's policy |
| **the kinds** | the kinds a repository declares at a revision, a launch's file declares, and flume's own; the repository's binding of its tools | the kind format, read from `.config/flume.toml`, a launch's `--kinds` file and the binary |
| **the change** | a change's facts at a revision: written, its tasks ticked, archived | `openspec`: OpenSpec's files, read with git |
| **the agent** | the command that starts an agent; whether it started, received a prompt, is asking, has work of its own running, ended; its last words; the way it stops short | `claude`: Claude Code with flume's plugin, read through its hooks' records and its transcript |
| **the terminal** | open an agent in a place; send it text; read its state and screen; close it; deliver a notice to a session; tidy what it made; show hints; keep a watch | `herdr`, and `printed`, a command printed for a person to run |
| **the merge queue** | where the base lives; submit a place; collect its outcomes; read the submission of a branch; one round of the base's queue | `sluice` |
| **the store** | a place's held state with its version; a write from that version; the place's own folder | the host's state directory |

The repository's `[profiles]` table binds the place, the change and the
merge queue; the host's configuration binds the terminal; the agent is
bound by the program an agent's settings name. A seam left unbound is
the no-tool profile, or unbound, and never guessed from another tool's
files.

The core names no tool: `herdr`, `sluice`, `claude`, `openspec` and
the rest appear only in profiles, and a profile implements its seam's
interface and nothing of the core. One profile never calls another.
The binary that binds them both hands the place profile what the merge
queue's policy says of a place: its readiness, its teardown and where
the base lives.

## The crates

| Crate | What it holds |
|---|---|
| `flume-seams` | the seams' interfaces and the types they share: held state, kinds, settings, ends |
| `flume-core` | the core: launch, start, settle, submit, deliver, tidy, and every caller's act |
| `flume-fakes` | a fake of every seam, for the core's tests |
| `flume-conformance` | the conformance suite: each clause of the statement as scenarios run against the fakes, and the clauses each covers |
| `flume` | the binary: the commands, the profiles, and the host's configuration and state |

Beside them, `plugins/claude` is the Claude Code plugin, which the
binary carries and writes for each agent, and `plugins/herdr` the herdr
plugin, a watch and a popup in shell.

## Surfaces

Every surface answers from the one core through one form: the command
line's `--json`, the MCP server's `status`, and what the herdr plugin
reads from `flume status --json`. The form only ever gains fields, and
no surface computes a stage's end, a status, a submission or a tidy of
its own.

## What flume never does

- **It decides nothing about what to work on.** A caller names the
  repository, the kind, the brief, the name and the base.
- **It lands nothing.** It submits to the merge queue and reads the
  outcome; the merge queue decides. No code path commits, merges,
  rebases or pushes in a place, and the only branch flume moves is a
  place's own, at its launch and at its tidy.
- **It ends no stage on a quiet pane.** A stage ends on its
  deliverable, read at one head, or on its agent's word that it stops
  short.
- **It answers no question that widens what an agent may do.** It
  answers only whether the agent may work in the place's folder.
- **It removes nothing it did not make,** never a place holding work,
  and never a branch holding commits its base lacks.
- **It runs nothing between its commands,** and no command waits on
  the merge queue.

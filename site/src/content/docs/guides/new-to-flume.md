---
title: New to flume
description: What flume is, and the words the rest of the site uses for it, a launch, a kind, a stage, a place, a hold and the host's queue.
sidebar:
  order: -3
---

Six things to know before you launch anything. Each part names the
guide that says the rest.

## What flume is

flume carries one piece of work into a worktree of its own and follows
it until it lands. You **launch** it with a repository, a kind, a brief,
a name and a base. flume makes the worktree and runs the kind's stages in
it with a fresh agent each. It submits the result to the merge queue
once the last stage delivers, and removes the worktree once the work
lands or is dropped.

flume decides nothing about what to work on. You, a script, or a tool
of your own names the work; flume runs it the same way whoever asks.
It starts the agents itself: Claude Code, in a herdr pane, or as a
command it prints for you to run.

[Launching a fix](/flume/guides/walkthrough/) shows one launch from
start to landing, with each command's output.

## Kinds and stages

A **kind** is a list of stages. Each **stage** names its prompt and the
one **deliverable** that ends it: commits, an OpenSpec change written,
the change's tasks ticked, the change archived, or a report. flume has
three kinds built in:

| Kind | Stages | What lands |
|---|---|---|
| `fix` | code | a fix, as commits |
| `chore` | code | a chore, as commits |
| `change` | construct, code, verify | an OpenSpec change, written, built and archived |

A stage ends when its deliverable exists and its agent is quiet. It
never ends because the pane went quiet, the agent's turn ended or its
process exited. A repository writes kinds of its own in
`.config/flume.toml`. [Kinds](/flume/guides/kinds/) says how.

## Places

A **place** is the worktree flume makes for one launch, at
`<repository>/places/<kind>--<name>`, on a branch of its own,
`<prefix>/<name>`, cut from the base. The prefix is the kind's: `fix`,
`chore` or `change` for the built-in kinds. Every stage of the work
runs in that one place, one agent at a time, and the place lasts until
the work lands or is dropped. flume never removes a place it did not
make, a place holding uncommitted work, or a branch holding commits its
base lacks. [Places and herdr](/flume/guides/places-and-herdr/) says
where the agents run and how a place is removed.

## A fresh agent for each stage

Each stage starts a new agent with the stage's prompt, so no stage
carries another's conversation. flume sends the prompt only after the
agent says it started, and counts it sent only when the agent says it
received it: Claude Code's own hooks tell flume both.
[The Claude Code plugin](/flume/guides/claude-code/) says how.

## Holds, ends and who is told

flume runs a kind through on its own: a stage's delivery starts the
next stage, and the last stage's delivery submits the place. A **hold**
stops that at a stage or at the submission, with your words saying what
it waits for, until you retry it. Each **end** of a stage, a
submission's outcome and a stall goes once to each **recipient** of the
place, and stays pending until it is delivered.
[Callers, holds and ends](/flume/guides/callers-and-holds/) covers
holds, `flume wait`, `flume inbox` and stalls, and
[roles and wake](/flume/guides/roles-and-wake/) covers a recipient that
is a role rather than a session.

## The host's queue

The host's configuration may set a bound on the agents running at once,
every caller's counted together. A stage starts only while the host is
under it, and otherwise waits for room in priority order. Every launch
is taken, and none is refused for want of room. A host with no bound
has no queue. The [configuration reference](/flume/reference/configuration/)
lists the setting.

flume runs no process of its own between commands. A **settle** moves
places on: it reads each stage's deliverable, starts the next stage,
submits, reads the merge queue's outcome, delivers ends and tidies.
Under herdr, flume's plugin runs `flume settle` whenever an agent
changes state; without it, each command of flume's that acts settles
first.

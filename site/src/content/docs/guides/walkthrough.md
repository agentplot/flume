---
title: Launching a fix
description: One fix launched, worked, submitted to sluice, landed and tidied, with each command and what it printed.
sidebar:
  order: -2
---

One fix, from its launch to its landing, in a small repository on one
machine. The commands and their output are real: they come from a run
of flume against a scratch repository, with its path shortened to
`~/src/acme` and the prompts cut where they repeat.

This run uses the printed command, the terminal flume uses when the
host names none: flume prints each agent's command and you run it.
Under herdr, flume opens each agent in a pane itself and a watch
settles every place as its agent changes state, so every `flume settle`
below happens with no command of yours.
[Places and herdr](/flume/guides/places-and-herdr/) says how to set
that up.

## 1. The repository

flume needs the repository to name the merge queue its work lands
through. `acme` binds sluice in `.config/flume.toml`:

```toml
[profiles]
place = "git-worktree"
merge_queue = "sluice"
```

and gates `main` with sluice in `.config/sluice.toml`, here with one
test command:

```toml
[tests]
runner = "command"
command = "tests/suite"

[branches.main]
parallel = 1
```

flume reads both at the base's current revision, so they are committed
on `main`. `~/src/acme` is a clone, so its places go under flume's
data directory, by the repository's id, and never show in its status.
[sluice's own guide](https://agentplot.github.io/sluice/guides/walkthrough/)
says how to set sluice up.

## 2. Launch

```console
$ flume launch --kind fix --name the-parser-reads-tabs \
    --brief "The parser drops a tab after a quote." --recipient me
launched ~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs
~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs: fix@1 (flume), live
  branch fix/the-parser-reads-tabs from main, head 84130b3cd2adae9a0ac2117884849d513da80181
  stage code: starting, started 1 times
  agent flume:f5f5e14159a8:fix--the-parser-reads-tabs:code:1 (claude): unknown
    program = claude (from default)
    run: cd ~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs && FLUME_AGENT=flume:f5f5e14159a8:fix--the-parser-reads-tabs:code:1 FLUME_PLACE=~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs FLUME_SECRET=3c1a… FLUME_STAGE=code claude --plugin-dir ~/.local/share/flume/plugins/claude/0.0.0-5d294893e56e 'You are the code stage of the fix the-parser-reads-tabs, …'
  the deliverable is missing
  submission: none, submitted 0 times, sent back 0 times
  recipients: me
  cannot read: the agent's state
  cannot read: whether the agent's own work runs
```

flume made the worktree `fix--the-parser-reads-tabs` on the
branch `fix/the-parser-reads-tabs`, cut from `main`, and started the
fix's one stage, `code`. Each line says one fact:

- `fix@1 (flume)`: the kind and its version, and where it was declared,
  here flume's own.
- `stage code: starting`: the agent has not yet said it started.
- `agent flume:…:code:1`: the agent's identity, naming the repository,
  the place, the stage and the start. No other live agent on the host
  carries it.
- `program = claude (from default)`: each of the agent's settings, with
  the layer it came from ([agent settings](/flume/guides/agent-settings/)).
- `run:`: the command to run, since this host prints it. It starts
  Claude Code in the place with flume's plugin and the stage's prompt.
- `recipients: me`: who is told how the work ends. With no
  `--recipient`, it is `$FLUME_RECIPIENT`, else the terminal session the
  launch ran in.

`flume prompt fix--the-parser-reads-tabs code` prints the same prompt
without starting anything:

```text
You are the code stage of the fix the-parser-reads-tabs, working in the place
~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs on the branch fix/the-parser-reads-tabs,
which was cut from main.

The parser drops a tab after a quote.

The repository declares no readiness.

Do the work the brief names, and commit it on fix/the-parser-reads-tabs in this
place. Your deliverable is commits: the stage is done once fix/the-parser-reads-tabs
holds your commits and you are quiet. …

If you cannot go on without an answer, or you find work this stage cannot deliver
past, stop short: run `flume needs "<words>"` when you cannot go on without an
answer, …
```

## 3. The agent works

Run the printed line. Claude Code starts in the place, and flume's
plugin tells flume it started and that it received the prompt. The next
settle reads that:

```console
$ flume settle
settled ~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs
$ flume status fix--the-parser-reads-tabs
~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs: fix@1 (flume), live
  branch fix/the-parser-reads-tabs from main, head 84130b3cd2adae9a0ac2117884849d513da80181
  stage code: begun, started 1 times
  …
  the deliverable is missing
```

The agent fixes the parser and commits on the place's branch. The
deliverable now exists, and the stage still runs until the agent is
quiet:

```console
$ flume status fix--the-parser-reads-tabs
~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs: fix@1 (flume), live
  branch fix/the-parser-reads-tabs from main, head 3abbf3b6b8ed59e66b11146dff00e7f9d418f6c3
  stage code: begun, started 1 times
  …
  the deliverable exists
```

Had the agent tried to stop with no commit, the plugin's Stop hook
would have sent it back to work, naming the deliverable it lacks.

## 4. The submission

Once the agent ends, a settle reads the stage delivered at the
branch's head and submits the place, since `fix` lands:

```console
$ flume settle
settled ~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs
$ flume status fix--the-parser-reads-tabs
~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs: fix@1 (flume), live
  branch fix/the-parser-reads-tabs from main, head 3abbf3b6b8ed59e66b11146dff00e7f9d418f6c3
  stage code: delivered, started 1 times, ended at 3abbf3b6b8ed59e66b11146dff00e7f9d418f6c3
  the deliverable exists
  submission: found {"submission":{"branch":"fix/the-parser-reads-tabs","id":"1","state":"under_test","target":"main"}}, submitted 1 times, sent back 0 times
  recipients: me
  ended: code delivered; pending for me
host: 0 agents running, no bound
```

flume ran `sluice submit main --no-wait --json` in the place and waits
on nothing. sluice tests `main` with the fix merged:

```console
$ sluice status main
main at 84130b3 · 1 in the queue · testing 1 at once (up to 1)
  testing main + #1 fix/the-parser-reads-tabs · 2s of 1h00m
```

`ended: code delivered; pending for me` is the stage's end, kept for
the recipient `me` until it is delivered.

## 5. The landing and the tidy

Each settle after the submission reads sluice's record, and runs one
round of sluice's queue where no `sluice run` drives it. Once sluice
lands the fix, the next settle reads the landing and tidies the place:

```console
$ flume settle
$ flume status fix--the-parser-reads-tabs
~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs@1793287699: fix@1 (flume), tidied {"at":1793287699,"over":{"landed":{"commit":"d185a879b51dba33555c036392b50443f4dfc5e8"}},"pending_for":["me"]}
  branch fix/the-parser-reads-tabs from main
  stage code: delivered, started 1 times, ended at 3abbf3b6b8ed59e66b11146dff00e7f9d418f6c3
  recipients: me
  ended: code delivered; pending for me
  ended: submission landed d185a879b51dba33555c036392b50443f4dfc5e8; pending for me
$ git log --oneline main
d185a87 chore(land): #1 fix/the-parser-reads-tabs
3abbf3b fix(parser): keep a tab after a quote
84130b3 chore: start
```

The worktree is gone, and so is the branch, since `main` holds every
commit of it. flume keeps only the place's record, since two ends are
still pending for `me`.

## 6. Reading the ends

`flume wait` waits as a recipient for one end, here the submission's,
and delivers it:

```console
$ flume wait fix--the-parser-reads-tabs --submission --recipient me
~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs submission landed d185a879b51dba33555c036392b50443f4dfc5e8 for me: delivered by this wait
```

It exits `0`. `flume inbox` collects everything else pending for the
recipient, on every place of the host:

```console
$ flume inbox --recipient me
fix--the-parser-reads-tabs code delivered; read: flume status ~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs
$ flume inbox --recipient me
nothing pending for me
$ flume status
no place stands on this host
host: 0 agents running, no bound
```

With every end delivered, flume removed the place's record too. Under
herdr, each end reaches the recipient's own pane as a notice at the
settle that reads it, and the inbox is for a session that restarted.

## What a change adds

`flume launch --kind change` runs three stages in turn, each with a
fresh agent: `construct` writes the OpenSpec change, `code` ticks its
tasks, and `verify` checks the work and archives the change. The
repository binds the change tool too:

```toml
[profiles]
place = "git-worktree"
change = "openspec"
merge_queue = "sluice"
```

`flume status` then gives the change's tasks ticked as the agent ticks
them. [Kinds](/flume/guides/kinds/) has each stage's deliverable.

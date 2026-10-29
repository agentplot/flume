---
title: Landing through sluice
description: How a place's work lands through sluice, from the submission flume makes to the landing it reads and the fix round a failure starts.
sidebar:
  order: 7
---

flume lands nothing itself. Once a landing kind's last stage delivers,
flume submits the place to the merge queue, reads the outcome at the
settles that follow, and tidies the place once the work lands. The
merge queue flume works with is [sluice](https://agentplot.github.io/sluice/),
which tests each submission merged with everything queued ahead of it
and lands it only when that passes.

## Binding sluice

A repository names its merge queue in `.config/flume.toml`:

```toml
[profiles]
merge_queue = "sluice"
```

A launch of a kind that lands is refused in a repository that binds no
merge queue. flume never assumes one because sluice is installed or the
repository holds a `.config/sluice.toml`.

Once bound, flume reads the repository's sluice policy,
`.config/sluice.toml`, at the base's current revision, for what it
says of a place:

| In `.config/sluice.toml` | What flume does with it |
|---|---|
| `[branches."<base>"] remote` | cuts each place's branch from that remote's copy of the base, after one fetch, and judges a branch's commits against it at the tidy |
| `[tests] toolchain` | runs it first in each new place; its output is the environment of every later step |
| `[tests] runner = "moon"` | runs `moon setup` in each new place |
| always | runs `sluice release` in the place before removing it |

## The submission

Once the last stage has delivered and its agent has ended, a settle
runs, in the place:

```sh
sluice submit <base> --no-wait --json
```

flume strips every terminal multiplexer's session from that command's
environment: herdr's `HERDR_*`, tmux's and zellij's. So sluice records
the place's worktree as the submitter and pushes no outcome into a
pane. flume never passes `--check`, never waits on the verdict, and
never submits through any other tool. What sluice does with the
submission, testing it on the host or opening a pull request where the
base takes them, is sluice's policy.

The agents never submit. flume's Claude Code plugin refuses
`sluice submit`, `git push`, and worktrunk's `wt submit` and `wt merge`
in an agent flume started, and tells it to commit and leave the rest to
flume.

`--hold-submission` at the launch, or `flume hold <place> --submission
"<words>"` before the last stage delivers, holds the submission until
`flume retry <place> --submission`.

## Reading the outcome

Each settle after the submission:

1. runs `sluice inbox --json` in the place, which gives each outcome
   sluice holds for the place's worktree once;
2. runs one round of the base's queue, `sluice step <base>`, which does
   nothing where a `sluice run` on the host drives the queue.

flume reads landed, failed and conflicting outcomes from sluice's
record, never from an agent and never from the base's history. A
submission sluice already held for the branch from before the launch
counts for nothing, so a place reusing an earlier place's name never
reads that place's outcome.

## What each outcome does

- **Landed.** The work is over. flume records the landing commit,
  delivers `submission landed <commit>` to each recipient, and tidies
  the place: the worktree goes, and the branch with it, since the base
  now holds its commits.
- **Failed.** flume starts the kind's fix round: the stage the kind's
  last stage names for a failure, `code` for the built-in kinds, with
  sluice's words, naming the failed check, in its prompt. The stages
  after it run again in order, and the place is submitted again once
  the last delivers.
- **Conflict.** The same fix round, with words naming the contested
  files and the base revision sluice tested against. The built-in
  prompts tell the agent to merge that revision into the place's
  branch, resolve each file and commit, never to rebase or rewrite a
  commit already on the branch.
- **Withdrawn** or **refused.** flume records the outcome and delivers
  it, and the place waits for you: `flume retry <place> --submission`
  submits it again, and `flume drop <place>` gives it up.

Each fix round counts toward the kind's bound, three goings-back unless
the last stage sets `max_send_backs`, together with found work from
the last stage. Past the bound the place waits on you with the last
words, and a retry starts it again.

## What sluice never does to a place

sluice removes nothing of a place: its worktree and its branch are
flume's to remove, at the tidy, by flume's own judgement of which
commits the base lacks. sluice never moves the place's branch, and
flume moves only that branch, at the launch and at the tidy.

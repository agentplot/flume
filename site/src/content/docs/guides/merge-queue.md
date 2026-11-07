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

Once bound, flume reads where the base lives from the repository's
sluice policy, `.config/sluice.toml`, at the base's current revision,
and has sluice make each place ready and release it:

| In `.config/sluice.toml` | What happens in a place |
|---|---|
| `[branches."<base>"] remote` | flume cuts each place's branch from that remote's copy of the base, after one fetch, and judges a branch's commits against it at the tidy |
| `[tests] toolchain`, `[tests] runner` | flume runs `sluice equip <base> --at <revision> --json` in each new place, at the revision its launch read, and sluice equips it as it equips its own runs' working copies: the toolchain, then the runner's install of what the place pins, its version check and its setup ([making a place ready](/flume/guides/places-and-herdr/#making-a-place-ready)) |
| always | flume runs `sluice release` in the place before removing it |

flume reads no `[tests]` key itself. Which steps a place's readiness
runs, and what they install, is sluice's to read from the policy and
the place. The sluice that runs them is the one the host user's login
`PATH` finds, 0.67.0 or later, the first release with `sluice equip`.

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

sluice can decide a submission as it takes it in, as it can on a base
that takes pull requests when the work conflicts with the base. Its
answer to the submit is then the entry's record, and sluice counts the
outcome delivered, so no inbox gives it. flume reads that outcome from
the answer in the same settle, as it reads one from the inbox. A
conflict there starts the fix round as any other does, never a wait as
if sluice refused the submission.

A submitted place has no agent left in it, so no agent's quiet runs
those settles. A caller keeping watch over the host runs them: `flume
settle --every <time>`, run on that bound from a timer of the host's or
whatever else keeps time there, settles every place, an agent in it or
none. So every submitted place has its outcome read, and its base's
queue run one round, within the bound
([keeping watch over the host](/flume/guides/places-and-herdr/#keeping-watch-over-the-host)).
While no caller keeps watch, flume runs attended: every command of
flume's that acts settles every place first, so a submitted place
moves at your next such command. Status says which of the two holds,
under the host's part.

## A base that takes pull requests

Where the base takes its work as pull requests, sluice pushes the
place's branch and opens its pull request, or updates the one open.
Until sluice takes the request in as an entry, its answer names the
pull request and no entry. The request may wait for a review the base
asks for, or for the host that takes the base's requests in. The
submission is then open and waiting, never refused, and the place
waits on nobody. Status names the pull request and says it is not yet
taken in, and each settle runs one round of the base's queue, which
takes the request in where this host takes them in.

Once sluice takes the request in, flume follows the entry it made,
which sluice records as submitted from `pull/<n>` and from the place's
branch. sluice delivers that entry's outcome to the pull request
rather than to the place, so flume reads the landing, the failure or
the conflict from sluice's record of the entry. An outcome the inbox
still holds for the place counts too, and flume acts on whichever
comes first, once. After a fix round flume submits again, sluice
updates the same pull request, and the entry that failed counts for
nothing while the update waits to be taken in.

## Whose outcome counts

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
  files, the entries ahead it conflicts with where sluice names them,
  and the base revision sluice tested against. Where sluice found the
  conflict before any test, that revision is the base's current one.
  The built-in prompts tell the agent to merge that revision into the
  place's branch, resolve each file and commit, never to rebase or
  rewrite a commit already on the branch.
- **Withdrawn** or **refused.** A pull request closed without landing
  counts as refused. flume records the outcome and delivers
  it, and the place waits for you: `flume retry <place> --submission`
  submits it again, and `flume drop <place>` gives it up.

Each fix round counts toward the kind's bound, three goings-back unless
the last stage sets `max_send_backs`, together with found work from
the last stage where its `on_fail` names a stage before it. Found work
from a last stage that is its own fix round, as fix's and chore's
`code` is, sends nothing back and counts nothing: the place waits on an
answer. Past the bound the place waits on you with the last words, and
a retry starts it again.

## Branches that land on branches

Work can land on a branch that itself lands: several changes gathered
on one branch, tested there one by one as each lands, and landed on
`main` together once the last is in. In flume that branch is a place's,
a base place: a place whose kind lands and has no stages. It runs no
agent and has no worktree. It is its branch, which sluice makes and
removes at flume's asking, and the state flume holds of it. The same
holds at every depth: a base place's branch can itself be the base of
another base place.

### A kind with no stages

flume carries no such kind. A repository declares one, as here, or a
source on the host or a launch's file does ([a kind with no stages](/flume/guides/kinds/#a-kind-with-no-stages)):

```toml
# .config/flume.toml
[profiles]
merge_queue = "sluice"

[kinds.train]
branch = "train"
lands = true
stages = []
```

flume tells a base place by its kind's shape alone, a kind that lands
with `stages = []`, never by the kind's name, its branch's prefix or
how deep it sits. Its branch is one sluice lands work on, so the
repository's sluice policy names a table for it beside the base's:

```toml
# .config/sluice.toml
[branches."train/*"]

[branches.main]
```

### Launching one, and the places on its branch

```sh
flume launch --kind train --name spring --base main
```

flume writes the place's state, then has sluice make the branch,
`sluice branch create train/spring --from main --json`, run in the
repository. Nothing else is made: no worktree, no readiness, no agent,
and the place takes none of the host's room. Where sluice refuses the
branch, the launch is refused with sluice's words and leaves nothing.
The place is named as any other, `train--spring`, by the location its
repository's places take, and nothing stands at that location.

Places launched with `--base train/spring` are cut from that branch,
where sluice says it lives, and land on it through sluice, as places
on `main` land on `main`. A fix, a chore or a change runs through as
it would anywhere. A launch on a branch flume holds is refused, naming
the place that holds it, in three cases:

- the branch is a place's that is no base place, since its worktree
  checks the branch out and a landing would move it under the place's
  agent;
- the base place's submission is open, since work landing on its branch
  then would land after the revision sluice is testing;
- the base place's work is over.

A branch no place of flume's holds, your own or one a tool made, takes
launches as any base does.

### Its submission

A base place has no last stage, so nothing submits it on its own. While
a place on its branch is live, the base place waits on them, never
stalled, and status says so. Once every one has landed or been dropped,
it waits on you, and stalls as any wait on its caller does unless a
hold's words keep it: `flume hold train--spring --submission "<words>"`.
You submit it when the branch holds what it should:

```sh
flume retry train--spring --submission
```

The retry is refused, naming them, while a place on its branch is live.
Otherwise flume runs, in the repository,

```sh
sluice submit main --head train/spring --no-wait --json
```

since the place has no worktree to submit from. That needs sluice
0.71.0 or later, the first release that takes a branch named as its
head from no worktree. sluice tests the branch merged with what is
queued ahead of it on `main`, as it tests any submission.

### Where its outcome is read

No inbox gives a base place's outcome, since sluice delivers an inbox
to a worktree. flume reads it from sluice's record of the branch, the
entry `sluice status main --all --json` gives for `train/spring`, at
the settles after the submission, and acts on it once. The outcome
reaches the place's recipients as any end does. A caller keeping watch
moves it as it moves every submitted place.

### A failed submission

A failed or conflicting submission starts nothing, since the kind
names no stage to send it back to. The base place waits on you with
sluice's words: the failed check, or the contested files and the
revision of `main` sluice tested against. To fix it, launch a fix with
`--base train/spring` and those words as its brief. A conflict's fix
merges that revision of `main` into its branch first, as any conflict's
fix does. Once the fix lands on `train/spring`, retry the base place's
submission. flume launches no fix of its own.

### Dropping it, and its tidy

`flume drop train--spring` is refused, naming them, while a place on
its branch is live: drop those first. A landed or dropped base place
is tidied with no worktree to remove and no teardown to run. flume has
sluice remove the branch, `sluice branch delete train/spring --into
main --json`, and never deletes it itself. Where sluice keeps the
branch, holding a submission or work `main` lacks, the answer to the
act that removed the place names it once with sluice's words. While
it stands, a launch that would make the branch again is refused.

## What sluice never does to a place

sluice removes nothing of a place: its worktree and its branch are
flume's to remove, at the tidy, by flume's own judgement of which
commits the base lacks. sluice never moves the place's branch, and
flume moves only that branch, at the launch and at the tidy.

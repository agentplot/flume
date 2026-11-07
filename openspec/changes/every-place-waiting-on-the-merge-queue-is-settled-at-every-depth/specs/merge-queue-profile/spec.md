## ADDED Requirements

### Requirement: sluice makes and removes a base place's branch

The sluice profile SHALL answer open by `sluice branch create <branch>
--from <base> --json` and close by `sluice branch delete <branch>
--into <base> --json`, each run in the repository with every
multiplexer's session stripped and `GIT_TERMINAL_PROMPT=0`: exit 0 its
answer, exit 1 a branch gone where sluice names that branch, exit 2 or
any other a refusal or a branch kept in its `error.message` (205, 206,
C.6; sluice 104, 105).

#### Scenario: A branch the policy names no table for

- **WHEN** flume opens `other/a` in a scratch repository whose policy names no table matching it
- **THEN** open answers refused with sluice's words, and no branch is made (205)

#### Scenario: A close sluice refuses

- **WHEN** flume closes `train/a` into `main` while a submission to `train/a` is still queued
- **THEN** close answers kept with sluice's words, and the branch stands (206)

### Requirement: sluice takes a base place's submission by name from the repository

The sluice profile SHALL submit a branch by name by `sluice submit
<base> --head <branch> --no-wait --json` run in the repository, read as
any submit's answer, and SHALL run no `sluice inbox` for it, reading its
outcome from read-submission's entry whose `from.head` is the branch,
with its words, files and `tested.on`, against the pinned sluice that
takes `--head` without `--check` and records no worktree as its
submitter (99, 117, 124, 125, 133, 203, C.6; sluice 124).

#### Scenario: A base place's branch over a scratch repository

- **WHEN** the pinned sluice lands `main` and `train/*` of a scratch repository with a stand-in origin, flume opens `train/a` from `main`, a fix cut from `train/a` is submitted to it and settles step it until it lands, and the base place's submission of `train/a` by name is stepped until it lands on `main`
- **THEN** sluice made `train/a` at `main`'s revision on the stand-in origin, the fix landed on `train/a`, `train/a` landed on `main` with no worktree on it anywhere, flume read the landing from `sluice status` with no `sluice inbox` run for it, and closing it into `main` removed it there and on the host, no command of flume's pushing or moving a branch itself (14, 202 to 206, S110)

### Requirement: A submitted place with no agent lands by a watch's settles

Against the pinned sluice, with no `sluice run` live, a place submitted
with no agent in it SHALL land by the steps of the settles a caller
keeping watch runs, `flume settle --every`, with no other caller's act
and no `sluice wait`; beside a live `sluice run`, those settles' steps
SHALL do nothing and the driver SHALL land it (124, 125, 208, C.6;
sluice 135).

#### Scenario: Settled by a watch, no agent

- **WHEN** a chore in a scratch repository delivers, its agent ends and it is submitted to the pinned sluice, and the test then runs `flume settle --every 1m` until it lands
- **THEN** each settle ran one `sluice step main`, the chore landed with no agent started after its code stage, and the settle after the landing tidied it (124, 208, S109)

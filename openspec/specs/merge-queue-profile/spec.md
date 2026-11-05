# merge-queue-profile Specification

## Purpose
The merge-queue seam, Part B.6, through sluice: a place's submission
made with no wait on its verdict, its outcomes read from sluice's inbox
at the settles that follow, one round stepped at each settle that finds
it undecided, and where the base lives.

## Requirements

### Requirement: Submitted through sluice with no wait

The sluice profile SHALL submit by `sluice submit <base> --no-wait
--json` in the place, its environment stripped of every multiplexer's
session, never with `--check` and never through `wt submit`, answering
with the entry once sluice holds it, or the refusal with sluice's words
(111, 113, 117, C.6, core.6).

#### Scenario: A submission from a herdr pane's hook

- **WHEN** the profile submits from a process whose environment carries `HERDR_*` words
- **THEN** sluice's entry names the place's worktree as its submitter and no session, and the answer comes before any verdict

### Requirement: Outcomes collected from sluice's inbox

The profile SHALL collect a place's pending outcomes by `sluice inbox
--json` in the place: landed with its commit, failed with the merge
queue's words, or in conflict with the files contested, each carrying
sluice's `notice` as its words, a conflict's base revision read from
the record's `tested.on`, each delivered once; and SHALL read a
withdrawal from read-submission, never from the inbox (99, 133, C.6).

#### Scenario: A conflict's base revision

- **WHEN** sluice records a place's entry in conflict, tested on a base revision
- **THEN** the outcome's base is the record's `tested.on`, its words sluice's `notice`, and nothing is parsed from the notice (112)

#### Scenario: A failed entry collected once

- **WHEN** sluice fails a submitted place's entry and the inbox is collected twice
- **THEN** the first collect gives the failure with sluice's notice, and the second gives nothing (101, 133)

#### Scenario: Seam scenarios against sluice

- **WHEN** the merge-queue seam's scenarios run against the profile on scratch repositories landing through the pinned sluice
- **THEN** each passes: landed is said only of an entry sluice landed, and nothing of the place is removed (101, 113)

### Requirement: One round at each undecided settle

The profile SHALL drive one round by `sluice step <base>` at each settle
that finds the submission undecided, with no read of whether a `sluice
run` is live, since beside one the step finds the branch's lock held and
does nothing (124, 125; sluice clause 135).

#### Scenario: A step beside a live driver

- **WHEN** a `sluice run` drives the base and a settle steps it
- **THEN** the step does nothing and the settle goes on (124)

### Requirement: Read, base and the place's data

The profile SHALL read the newest entry of a branch from `sluice status
<base> --json`; answer the base at `origin/<base>` after one fetch
where `.config/sluice.toml` lands the base on a remote and the local
base otherwise; and give the place profile the readiness, `sluice equip
<base> --at <revision> --json` with the reading of its steps, the
teardown (`sluice release`) and the base's ref, reading no `[tests]`
key for a place (9, 11, 20, 126, C.1, C.6, core.3).

#### Scenario: A queue nothing drives

- **WHEN** a place's entry waits on a base no `sluice run` drives and a settle steps it
- **THEN** one round runs and the entry is decided (124, 125)

#### Scenario: A fetch that fails

- **WHEN** the base lands on a remote that refuses the fetch
- **THEN** base answers why, and the launch is refused naming it (9, core.7)

#### Scenario: What the policy says of a place

- **WHEN** the policy at main names the moon runner, the toolchain `devenv print-dev-env` and `origin` as main's remote, and a place on main is readied at the revision R its launch read
- **THEN** the place's data is one readiness step, `sluice equip main --at R --json`, with the profile's reading of it, `sluice release` as its teardown and `origin` as the base's remote, and names no toolchain and no `moon setup` (11, 20, C.1, C.6)

#### Scenario: The steps sluice equipped with

- **WHEN** `sluice equip` answers the steps `toolchain`, `proto install` and `moon --version` passed and `moon setup` failed with its words, exiting 1
- **THEN** the reading gives those four steps in that order, each with its outcome, the last with sluice's words cut as any step's are (12, 79, C.6)

#### Scenario: An answer with no steps

- **WHEN** `sluice equip` answers a refusal with its `error.message`, or nothing the profile can read, or cannot be run
- **THEN** the reading gives one failed step, `sluice equip`, with sluice's message, its errors, or why it could not run (12, C.6)

### Requirement: A base sluice lands not

The profile's base SHALL answer that sluice lands no such base where
`.config/sluice.toml` names no table for the base, its own or a
pattern's, or no revision of the base holds a policy, so the core reads
the base at origin; and SHALL answer the local base where the base's
table names no remote, sluice landing it there (126, C.1, C.6).

#### Scenario: A base the policy names no table for

- **WHEN** base is asked of `trial`, a branch made from main whose policy names a table for main alone
- **THEN** it answers that sluice lands no such base, and main's at a revision (126, C.6)

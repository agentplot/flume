## ADDED Requirements

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
sluice's `notice` as its words, a conflict's carrying the base's revision sluice tested
against, each delivered once; and SHALL read a
withdrawal from read-submission, never from the inbox (99, 133, C.6).

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
<base> --json`; answer the base
at `origin/<base>` after one fetch where `.config/sluice.toml` lands the
base on a remote and the local base otherwise; and give the place
profile the readiness, the teardown (`sluice release` in the place) and
the base's ref, read from `.config/sluice.toml` once sluice is bound
(9, 99, 125, 126, C.1, C.6, core.3).

#### Scenario: A queue nothing drives

- **WHEN** a place's entry waits on a base no `sluice run` drives and a settle steps it
- **THEN** one round runs and the entry is decided (124, 125)

#### Scenario: A fetch that fails

- **WHEN** the base lands on a remote that refuses the fetch
- **THEN** base answers why, and the launch is refused naming it (9, core.7)

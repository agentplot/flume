## Purpose

A base place: a place whose kind lands and has no stages, whose branch
other places name as their base and land on through the merge queue,
and which lands on its own base once its caller says so. Its branch is
the merge queue's to make and to close, its worktree checks out no
branch, it runs no agent, and it is submitted by name on its caller's
retry once no place on its branch is live; the same at every depth, and
knowing no use of one by name (10, 18, 202 to 206).

## ADDED Requirements

### Requirement: A base place's launch makes its branch through the merge queue and starts nothing

The core SHALL take a place whose kind lands and has no stages as a
base place, by that shape alone. Its launch SHALL write the place's
held state, then ask the merge queue seam's open for its branch at its
base, then make its worktree detached at the revision open answered,
and SHALL run no readiness, start no agent and count no room or points
(10, 11, 70, 165, 202, 205; core.17).

#### Scenario: A base place launched on main

- **WHEN** a repository whose fake merge queue lands `main` launches `train--spring`, a kind of the repository's with `lands = true` and `stages = []`
- **THEN** the fake merge queue logs `open train/spring from main` after the place's held state is written, the fake place logs `make <location> detached at <revision>` at the revision open answered, no readiness runs, the fake terminal opens nothing, and status gives the place live with no stage, waiting on its caller (202, S110)

### Requirement: A base place's launch is refused whole

A base place's launch whose open is refused SHALL be refused with the
merge queue's words, and one whose worktree cannot be made SHALL close
the branch it opened; either leaves nothing it made. A launch of a base
place that gives points SHALL be refused, naming them (2, 8, 165, 205,
206; core.8).

#### Scenario: The merge queue refuses the branch

- **WHEN** the fake merge queue refuses `open train/spring from main` with `the policy names no table for train/spring`
- **THEN** the launch is refused with those words, no worktree is made, and the store holds no document of the place (2, 8, 205)

#### Scenario: The worktree cannot be made

- **WHEN** open makes `train/spring` and the fake place refuses the location
- **THEN** the launch is refused naming the location, the fake merge queue logs `close train/spring into main`, and nothing of the place is held (8, 17, 206)

#### Scenario: Points given to a base place

- **WHEN** a launch of `train--spring` gives points 3 on a stream that bounds points
- **THEN** it is refused naming the points, and nothing is made (165)

### Requirement: A place on a base place's branch is cut and lands there

A launch whose base is a live base place's branch SHALL cut its place
at that branch where the merge queue's profile says it lives and SHALL
submit it there, as on any base; and a base place SHALL be launched on
another base place's branch the same way, at any depth (9, 15, 126,
202).

#### Scenario: Two changes on a base place's branch

- **WHEN** `train--spring` is live and two changes are launched with `--base train/spring`, each runs through, and the fake merge queue lands each
- **THEN** each was cut at the revision the merge queue gives `train/spring`, submitted with `train/spring` as its target, and tidied once it landed, and the base place still waits on its caller (15, 202, S110)

#### Scenario: A base place two deep

- **WHEN** `train--spring` is live on `main`, `train--week` is launched with `--base train/spring`, and a fix is launched with `--base train/week`
- **THEN** the fake merge queue logs `open train/week from train/spring`, the fix is cut at `train/week` and lands there, and no part of the core read a kind's name or a depth to do it (202)

### Requirement: A launch on a place's branch is refused where it would land under a checkout or after a submission

A launch whose base is the branch of a live place of the same
repository SHALL be refused, naming that place, where the place is no
base place, or where the base place's submission is open or its work is
over; a base that is no place's branch SHALL be launched on as before
(10, 63, 111, 204).

#### Scenario: On a change's branch

- **WHEN** a launch names as its base the branch of a live change place
- **THEN** it is refused naming the change, and nothing is made (10, 204, S111)

#### Scenario: On a submitted base place's branch

- **WHEN** `train--spring` has been submitted and its submission is under test, and a fix is launched with `--base train/spring`
- **THEN** it is refused naming `train--spring`, and nothing is made (204, S111)

#### Scenario: On a branch no place holds

- **WHEN** a change is launched with `--base bolt/old`, a branch the fake merge queue lands on that no place of flume's holds
- **THEN** it is launched as before, cut where the merge queue says `bolt/old` lives (9, 126)

### Requirement: A base place is submitted by name on its caller's retry once no place on its branch is live

A base place SHALL never be submitted by a settle on its own. A retry
naming its submission SHALL be refused, naming them, while a place
whose base is its branch is live, and otherwise SHALL submit its
branch through the merge queue seam by name, at the revision the merge
queue's base reads for it, holding no wait (47, 49, 111, 117, 203;
core.6).

#### Scenario: A retry while a change still works

- **WHEN** a change on `train/spring` still works and the caller retries `train--spring`'s submission
- **THEN** the retry is refused naming the change, and the fake merge queue logs no submit (203, S111)

#### Scenario: A retry once every place on the branch landed

- **WHEN** both changes on `train/spring` have landed and the caller retries `train--spring`'s submission
- **THEN** the fake merge queue logs `submit train/spring named` with `main` as its target, the place's status gives it submitted, and the next settles step `main` (117, 124, 203, S110)

#### Scenario: No settle submits it

- **WHEN** `train--spring` has no place on its branch and twenty settles run with no retry
- **THEN** the fake merge queue logs no submit, and the place waits on its caller (111, 203)

### Requirement: A base place's failed submission waits on its caller

A failed or conflicting submission of a base place SHALL start no stage
and no agent: the place SHALL wait on its caller with the merge queue's
words, delivered to its recipients, and a fix launched on its branch
SHALL be taken while it so waits (43, 112, 203, 204).

#### Scenario: A failed submission, a fix, a retry

- **WHEN** the fake merge queue fails `train--spring`'s submission naming the check `test`, the caller launches a fix with `--base train/spring` and those words as its brief, the fix lands on `train/spring`, and the caller retries `train--spring`'s submission, which lands
- **THEN** no agent started in `train--spring`, its status gave the words while it waited, its recipient was delivered the failure once, and its work is over once it lands (112, 203, S112)

#### Scenario: A conflicting submission

- **WHEN** the fake merge queue finds `train--spring`'s submission in conflict on `README.md` against `main` at `c0ffee`
- **THEN** the place waits on its caller, its words naming `README.md` and `c0ffee`, and no stage starts (112, 203, S112)

### Requirement: A base place's drop and tidy

A drop of a base place SHALL be refused, naming them, while a place
whose base is its branch is live. A base place whose work is over SHALL
be removed as any place, through its teardown, and its branch SHALL
then go by the merge queue seam's close into its base, never by the
place seam's branch removal; a branch the merge queue keeps SHALL be
named once in the answer to the act that removed the place, and a
launch that would make it again refused while it stands (3, 52, 64 to
67, 204, 206).

#### Scenario: A drop while a change works

- **WHEN** a change on `train/spring` works and the caller drops `train--spring`
- **THEN** the drop is refused naming the change, and the base place stays live (52, 204)

#### Scenario: The landed base place tidied

- **WHEN** `train--spring`'s submission lands on `main`
- **THEN** the next settle removes its worktree through the fake place, then the fake merge queue logs `close train/spring into main`, the fake place logs no branch removal, and no held state of the place stays once its ends are delivered (66, 67, 204, S110)

#### Scenario: The merge queue keeps the branch

- **WHEN** a dropped `train--spring`, with nothing on its branch live, is tidied and the fake merge queue keeps `train/spring` saying `work main lacks`
- **THEN** the answer to the drop names `train/spring` kept with those words, once, and a launch of `train--spring` is refused while the branch stands (3, 67, 206)

### Requirement: Status says what a base place waits on

Status SHALL give a base place's head as the merge queue's base reads
its branch, never from its worktree; the live places whose base is its
branch as `on_branch`; and its wait: on those places while any is live,
never stalled; on its caller once none is and it is not submitted,
stalling as any wait on a caller does; and on the merge queue once
submitted. The place's line and the list SHALL give
`<n> on its branch · open` while places on its branch are live (56,
58, 80, 126, 130, 200, 203).

#### Scenario: Waiting on its places

- **WHEN** two changes on `train/spring` are live and status is asked of `train--spring` an hour after its launch
- **THEN** it gives `waits_on` `places`, `on_branch` naming both changes, the head the fake merge queue reads for `train/spring`, and no stall; its line ends `2 on its branch · open` (56, 130, 203)

#### Scenario: Waiting on its caller

- **WHEN** both changes have landed and six minutes pass with no retry and no hold
- **THEN** status gives `train--spring` waiting on its caller for its submission since the settle that read none live, stalled, and each recipient is delivered the stall once (43, 130, 203)

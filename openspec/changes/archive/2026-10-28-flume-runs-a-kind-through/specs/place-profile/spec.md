## ADDED Requirements

### Requirement: A place is a plain git worktree

The place profile SHALL make a place by `git worktree add -b
<prefix>/<name> <location> <revision>` at `<repository>/places/<kind>--<name>`
with no hook run, read it by `git status --porcelain=v2` and the
worktree's lock, remove it by the teardown it was handed and `git
worktree remove` with no force, and remove its branch by `git branch -D`
only when `git cherry` against the base names no commit the base lacks;
every operation all or nothing (78 to 83, C.1).

#### Scenario: Seam scenarios over the profile

- **WHEN** the place seam's scenarios run against the profile over scratch repositories
- **THEN** each passes: make touches no branch or location that exists, and each operation is all or nothing (83)

#### Scenario: An untracked file and a failed teardown

- **WHEN** one place holds a file neither committed nor ignored and another's teardown step fails
- **THEN** both are kept, naming the file and the step (S10)

#### Scenario: A branch with a commit its base lacks

- **WHEN** a dropped place's branch holds a commit whose change its base lacks, and another's commit was cherry-picked onto the base
- **THEN** the first branch is kept naming the commit, and the second deleted (67, S9)

### Requirement: Readiness and teardown as the merge queue declares

The place profile SHALL run the readiness steps and the teardown the
merge queue's profile declares for the repository, handed to it by the
binary's resolver, in the place, in order, with an environment cleared
to the host user's login environment; what the toolchain step makes
SHALL be kept in the place's own git directory; with no merge queue
bound nothing SHALL run (11, 12, 66, 79, C.1, C.7, core.2).

#### Scenario: Readiness that fails at its toolchain

- **WHEN** a scratch repository's toolchain step exits non-zero
- **THEN** ready gives the step and its words, the steps after it are not run, and nothing of the caller's environment reached the step (S16)

#### Scenario: No merge queue bound

- **WHEN** a place is made in a repository that binds no merge queue
- **THEN** ready runs nothing and remove runs no teardown step

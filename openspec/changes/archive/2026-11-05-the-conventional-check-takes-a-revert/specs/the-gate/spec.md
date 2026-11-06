## ADDED Requirements

### Requirement: The commit hooks take a commit git revert wrote as CI does

The commit-msg hook SHALL take a message `git revert` wrote, its first
line starting `Revert "`, as CI's conventional check skips the commit by
`convco check --ignore-reverts`, and SHALL hand every other message to
convco, so a session and CI hold a commit to one rule (core.22).

#### Scenario: A revert committed after `--no-commit`

- **WHEN** `root:hooks` commits in its fresh worktree a revert `git revert --no-commit` prepared, with the message git wrote, and then a message with no type
- **THEN** convco passes the revert, which is committed, and refuses the message with no type

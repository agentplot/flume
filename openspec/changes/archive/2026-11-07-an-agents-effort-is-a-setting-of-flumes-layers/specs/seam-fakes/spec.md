## ADDED Requirements

### Requirement: The fake agent scripts the efforts it takes

The fake agent SHALL script the words it takes as an effort and the
variable it names as read over them, each passing the agent seam's
scenarios; its command SHALL pass `--effort <word>` where the settings
give one, and its log SHALL name each `effort` it is asked, so a test
reads what the profile was given (106, 197).

#### Scenario: A scripted effort taken and passed

- **WHEN** a test scripts the fake agent to take `low` and `high`, and a fix launches with `--agent effort=high`
- **THEN** the fake's log shows `effort` asked of `high` at the launch and at the start, and the start's command passing `--effort high` (88, 197)

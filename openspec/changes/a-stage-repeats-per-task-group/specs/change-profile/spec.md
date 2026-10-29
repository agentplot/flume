## ADDED Requirements

### Requirement: Each task with its task group

The change seam SHALL give each task the task group it is in, each group
with its number and title. The OpenSpec profile SHALL read each
`## <n>. <title>` heading of `tasks.md` as a group holding the tasks
under it, and tasks above every heading as a group with no number or
title; reading SHALL stay a read of the revision alone (86, 87, C.3).

#### Scenario: Groups read from headings

- **WHEN** a scratch change's `tasks.md` holds `## 1. Parser` with two tasks and `## 2. Tests` with one
- **THEN** read-change gives two groups, `1` titled `Parser` and `2` titled `Tests`, and each task in its group, in order

#### Scenario: Tasks above every heading

- **WHEN** a `tasks.md` holds a task before its first group heading
- **THEN** that task is in a group of its own with no number or title, ahead of the others

#### Scenario: The fake change's groups

- **WHEN** the change seam's scenarios run against the fake change with groups scripted, and against the profile
- **THEN** both pass the same scenarios for groups (106)

## ADDED Requirements

### Requirement: The fakes script a base place's branch

The fake merge queue SHALL answer open and close as scripted, by
default making the branch at its base's revision and removing it, and
log each as `open <branch> from <base>` and `close <branch> into
<base>`; a branch it opened SHALL be one it lands on. It SHALL log a
submit by name as `submit <branch> named`. The fake place SHALL make a
detached place and log it as `make <location> detached at <revision>`
(106, 202, 205, 206).

#### Scenario: Scripted open and close

- **WHEN** a test scripts the fake merge queue to refuse `open train/x from main` with words, and to keep `train/y` at its close with words
- **THEN** open of `train/x` answers refused with them, close of `train/y` answers kept with them, and the log names each ask (205, 206)

#### Scenario: A detached place

- **WHEN** a test makes a fake place with no branch at a revision
- **THEN** its read gives the revision and no branch, and the log names `make <location> detached at <revision>` (78)

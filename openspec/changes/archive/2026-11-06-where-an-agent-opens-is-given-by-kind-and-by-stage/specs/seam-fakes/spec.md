## MODIFIED Requirements

### Requirement: The fake terminal scripts runs

The fake terminal SHALL answer runs as scripted for each part asked,
and otherwise that the part does not run, its own part named `own`,
logging each asking as one `runs <part> <part>…`, the parts in the
order asked, and starting nothing (106, 201).

#### Scenario: A scripted runs

- **WHEN** a test scripts the fake terminal to read `work` running, and asks it runs of `work` and then of `office`
- **THEN** it answers `work` running and `office` not running, its log naming `runs work` and `runs office` and no open (201)

#### Scenario: Several parts in one runs

- **WHEN** a test scripts the fake terminal to read `work` running, and asks it runs of `work`, `office` and its own part in one call
- **THEN** it answers `work` running, `office` not running and `own` not running, in that order, its log naming the one `runs work office own` (201)

## ADDED Requirements

### Requirement: The fake merge queue scripts a forecast

The fake merge queue SHALL answer forecast as a test scripts it for a
repository and a branch, clear at the base's revision where nothing is
scripted, a scripted answer standing at every ask until scripted again,
and log each ask as `forecast <branch> onto <base>`; a forecast SHALL
add no submission to its record (106, 210).

#### Scenario: Scripted, then cleared

- **WHEN** a test asks the fake merge queue to forecast `train/x` onto `main`, then scripts it in conflict on `README.md` with `#7` ahead and asks twice, then scripts it clear and asks again
- **THEN** the answers are clear at `main`'s revision, in conflict on `README.md` with `#7` twice, then clear; the log names four forecasts; and the record holds no submission of `train/x` (210)

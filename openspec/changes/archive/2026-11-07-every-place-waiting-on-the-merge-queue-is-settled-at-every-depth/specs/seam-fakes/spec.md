## ADDED Requirements

### Requirement: The fakes script a base place's branch

The fake merge queue SHALL answer open and close as scripted, by
default making the branch at its base's revision and removing it, and
log each as `open <branch> from <base>` and `close <branch> into
<base>`; a branch it opened SHALL be one it lands on. It SHALL log a
submit by name as `submit <branch> named`, and decide such a
submission in its record alone, its inbox giving nothing for it (106,
133, 202, 205, 206).

#### Scenario: Scripted open and close

- **WHEN** a test scripts the fake merge queue to refuse `open train/x from main` with words, and to keep `train/y` at its close with words
- **THEN** open of `train/x` answers refused with them, close of `train/y` answers kept with them, and the log names each ask (205, 206)

#### Scenario: A submission by name decided in the record

- **WHEN** a test submits `train/z` by name and scripts the fake merge queue to land it
- **THEN** read-submission of `train/z` gives it landed, and the inbox gives nothing for any place (133, 203)

### Requirement: The fakes keep the host's watch and settle nothing

The fake store SHALL hold the host's watch as any host document, so a
test makes flume watched by settling with a bound and lets the watch
lapse by the fake clock; the fake terminal SHALL settle nothing and
answer nothing of a watch, and SHALL log the clues show gives it, the
line's short form among them (106, 122, 139, 208).

#### Scenario: A watch the clock lets lapse

- **WHEN** a test settles every place with a bound of thirty seconds and moves the fake clock past a minute
- **THEN** the engine reads attended, and reads unattended again after the test's next settle with a bound (119, 208)

#### Scenario: Show's clues in the log

- **WHEN** a settle shows a place's clues to the fake terminal
- **THEN** its log names the place, its stage and state, its line's short form and its base's count (139)

## ADDED Requirements

### Requirement: The fake host scripts each account's reading

The fake host SHALL hold a reading or a refusal for each path a test
sets, answer that the file is missing for any other, and pass the
host's reading scenario; no fake SHALL read a real file (106, 184).

#### Scenario: A scripted reading

- **WHEN** a test sets the fake host's reading for `work-1`, then sets it again with a later `read_at`
- **THEN** the core reads the later reading at the next choice, and the fake's log shows each reading asked (184)

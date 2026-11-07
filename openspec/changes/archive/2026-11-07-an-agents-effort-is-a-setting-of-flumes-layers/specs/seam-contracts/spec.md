## ADDED Requirements

### Requirement: The agent seam takes an effort or refuses it

The agent seam SHALL say whether its program takes an effort's word,
with words naming the effort, the program and the words it takes where
it does not, refusing every word by default; SHALL name the variable
its program reads over the effort, none by default; and its command
SHALL pass the settings' effort as its profile's way gives it, and none
where the settings give none (88, 170, 197).

#### Scenario: A profile that names no way to give an effort

- **WHEN** an agent profile that keeps the seam's defaults is asked of `high`
- **THEN** it refuses with words naming `high`, and names no variable read over it (197)

#### Scenario: The fake's efforts over the seam's scenarios

- **WHEN** the agent seam's scenarios run against the fake agent scripted to take `low` and `high` and to name `FAKE_EFFORT` as read over them
- **THEN** `low` and `high` are taken, any other word is refused naming it and both words, the variable read is `FAKE_EFFORT`, and the fake's command passes the effort taken (88, 197)

### Requirement: The settings carry an effort

`Settings` SHALL carry an optional effort, taken whole by
`Settings::under` as the model is, serialized to JSON and back
unchanged, and a held state written before it SHALL read with none
(32, 197).

#### Scenario: A held state with an effort

- **WHEN** a held state whose launch gives verify the effort `max` and whose kind gives every stage `high` is written and read
- **THEN** it reads back unchanged, and the same state written without any effort reads with none (32, 197)

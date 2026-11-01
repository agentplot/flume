## ADDED Requirements

### Requirement: launch gives keep_open by --agent

`flume launch --agent [<stage>.]keep_open=<time>` SHALL give the
setting in the forms `--stall-after` takes, `<n>`, `<n>s`, `<n>m` or
`<n>h`, and SHALL refuse a value of any other form, naming the field
and the forms (7, 32, 41).

#### Scenario: A stage's time

- **WHEN** a caller launches with `--agent code.keep_open=15m`
- **THEN** the launch's settings give code `keep_open` `15m` (7, 32)

#### Scenario: A time in another form

- **WHEN** a caller launches with `--agent keep_open=soon`
- **THEN** the launch is refused naming `keep_open` and the forms it takes, and nothing is made (7)

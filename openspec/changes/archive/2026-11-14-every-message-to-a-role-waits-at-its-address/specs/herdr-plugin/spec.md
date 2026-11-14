## MODIFIED Requirements

### Requirement: Keys act through flume's commands

The plugin's keys SHALL open a place's pane, message its agent, stop it
and retry its stage, each by herdr's focus or by one flume command,
`flume message`, `flume stop` or `flume retry`, the popup's key for a
message `m`, and SHALL never send to, open or close an agent's pane
itself (53, 107, section 9).

#### Scenario: Retry from the sidebar

- **WHEN** the retry key is pressed on a place whose construct ended short
- **THEN** the stand-in flume records one `flume retry` naming the place and its stage, and the stand-in herdr records no send to any pane

#### Scenario: Message, stop and open, each one key

- **WHEN** `m` is pressed with words, then `s` with the force asked, then `o`, on a working place
- **THEN** the stand-in flume records `flume message` naming the place with the words, then `flume stop` naming it forced, herdr focuses the place's tab, and the stand-in herdr records no send to any pane

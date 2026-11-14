## ADDED Requirements

### Requirement: flume's own kinds give full

A stage of change@4, chore@2 or fix@2 SHALL give its agent the mode
`full` in the kind's layer, the last of 32's, so a mode the launch, the
host or the repository gives SHALL come first; status SHALL name `full`
from `kind` where no earlier layer gives one, and a place of change@3,
chore@1 or fix@1 SHALL be given no mode by its kind (22, 32, 162, 169;
record 0064).

#### Scenario: A bare fix in full

- **WHEN** a fix is launched by its bare name, flume holding fix@1 and fix@2, on a host whose configuration, repository and launch give no mode, and the agent's profile maps `full`
- **THEN** the place runs fix@2, its agent's command carries the value the profile maps `full` to, its start's record names that value, and status gives the mode `full` from `kind` (22, 154, 162)

#### Scenario: The host's mode over the kind's

- **WHEN** the same fix is launched on a host whose `[agents]` gives `mode = "edits"`
- **THEN** its agent starts with the value the profile maps `edits` to, status naming `host` as where its mode came from (32, 162)

#### Scenario: An earlier version named

- **WHEN** `fix@1` is launched on a host whose configuration, repository and launch give no mode
- **THEN** its agent's command carries no mode and status gives no mode with the agent (154, 162)

## MODIFIED Requirements

### Requirement: No mode where no layer gives one

Where no layer gives a mode, flume SHALL give the agent none: its
profile is passed no mode, the start's record SHALL name none, and its
word at its start SHALL be compared with no mode, so the account's own
configuration decides (32, 161, 162).

#### Scenario: A host and a repository that say nothing

- **WHEN** `fix@1`, whose declaration gives no mode, is launched on a host whose configuration, repository and launch give none
- **THEN** its agent's command carries no mode, its start's record names none, its started ack and its first prompt are taken whatever mode it reports, and status gives no mode with the agent (S66, 161, 162)

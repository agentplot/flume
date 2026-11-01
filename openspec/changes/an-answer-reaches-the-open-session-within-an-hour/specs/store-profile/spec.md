## ADDED Requirements

### Requirement: keep_open in the host's agents tables

The host's configuration SHALL read `keep_open` in
`[kinds.<kind>.agents]`, `[repos."<repository>".kinds.<kind>.agents]`
and their `.stages.<stage>` tables as `<n>`, `<n>s`, `<n>m` or `<n>h`,
kept as written, and SHALL refuse any other value, naming the table and
the key (32, C.7).

#### Scenario: A time for a kind and for one stage

- **WHEN** the configuration gives `[kinds.change.agents] keep_open = "30m"` and `[kinds.change.agents.stages.verify] keep_open = "0"`
- **THEN** it reads `30m` for change's agents and `0` for verify's, each as written (32, C.7)

#### Scenario: A time in another form

- **WHEN** the configuration gives `[kinds.fix.agents] keep_open = "an hour"`
- **THEN** the configuration is refused naming `kinds.fix.agents.keep_open` and the forms it takes (C.7)

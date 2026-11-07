## ADDED Requirements

### Requirement: keep_open in the host's agents tables

The host's configuration SHALL read `keep_open` in every agents table it
reads, for a repository, a kind, a stream and every agent, and in each
`.stages.<stage>` table, in its managed layer and its local one over
it, as a string `<n>`, `<n>s`, `<n>m` or `<n>h`, kept as written, and
SHALL refuse any other value, a table of programs among them, naming
the table and the key (32, 198, C.7).

#### Scenario: A time for a kind and for one stage

- **WHEN** the configuration gives `[kinds.change.agents] keep_open = "30m"` and `[kinds.change.agents.stages.verify] keep_open = "0"`
- **THEN** it reads `30m` for change's agents and `0` for verify's, each as written (32, C.7)

#### Scenario: A time for a stream and for every agent

- **WHEN** the configuration gives `[agents.streams.work] keep_open = "10m"` and `[agents] keep_open = "45m"`
- **THEN** it reads `10m` for the stream `work`'s agents and `45m` for every agent, each as written (32, C.7)

#### Scenario: A time in another form

- **WHEN** the configuration gives `[kinds.fix.agents] keep_open = "an hour"`
- **THEN** the configuration is refused naming `kinds.fix.agents.keep_open` and the forms it takes (C.7)

#### Scenario: A table of programs

- **WHEN** the configuration gives `[agents] keep_open = { claude = "1h" }`
- **THEN** the configuration is refused naming `agents.keep_open` and that it is one time for every program (198, C.7)

## ADDED Requirements

### Requirement: SessionStart compares the start's record before it acks

The SessionStart hook SHALL compare its program, read from its parent
process, and the digest of each variable the start's record names, in
its own environment and keyed by `FLUME_SECRET`, with the start's
record before it acks; on any difference it SHALL ack nothing, record
the start's variables that differ by name, start a settle in the
background and exit 0 (35, 121, 161, C.4).

#### Scenario: A variable with another value

- **WHEN** the hook runs for a start whose record names `CLAUDE_CONFIG_DIR` and its environment holds another value
- **THEN** no started ack is written, the agent's record names `CLAUDE_CONFIG_DIR` as different with no value, a settle of the place is started, and the hook exits 0 (161)

#### Scenario: Another program

- **WHEN** the hook's parent runs a program other than the one the start's record names
- **THEN** no started ack is written and the record says the program differed (161)

#### Scenario: A session flume did not start

- **WHEN** the hook runs with no `FLUME_AGENT` in its environment
- **THEN** it compares nothing, writes nothing and exits 0 (C.4)

### Requirement: StopFailure records its error and when it ran

The StopFailure hook SHALL record, beside the agent's last words, the
failure's `error` and the host's time when it ran, so the profile reads
a usage limit and its reset from them (43, 151, C.4).

#### Scenario: A usage limit's failure

- **WHEN** the hook runs with `error` `rate_limit` and a usage limit's words
- **THEN** the agent's record holds the words as its last words, the error and the time the hook ran (43, 151)

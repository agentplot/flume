## MODIFIED Requirements

### Requirement: An agent started as Claude Code with flume's plugin

The agent profile SHALL start an agent as `claude` in the place, with
the plugin of the flume that started it loaded by `--plugin-dir`, the
settings' model and instructions, an environment of the host user's
login environment with its account's variables over it, and
`FLUME_AGENT`, `FLUME_SECRET`, `FLUME_PLACE` and `FLUME_STAGE`, the
identity naming the repository by an id from its common git directory;
and the prompt as its argument under the printed command (29, 32, 33,
88, 144, C.4).

#### Scenario: The command for a code stage

- **WHEN** the profile gives the command for a stage with a model and instructions under an account giving `CLAUDE_CONFIG_DIR`
- **THEN** it runs `claude` with `--plugin-dir`, `--model` and `--append-system-prompt`, the account's `CLAUDE_CONFIG_DIR`, the four `FLUME_*` words, and nothing of the caller's session

## ADDED Requirements

### Requirement: A usage limit read from Claude Code's own words

The Claude Code profile's `spent` SHALL read yes only where the
StopFailure hook recorded `rate_limit` with words that open as the
pinned release's usage-limit messages do, with the reset its words'
` · resets <when> (<zone>)` part names, read in that zone from when
the hook ran, a time of day being the first such time no more than a
minute before the hook ran, or none; a `rate_limit` with other words SHALL read no
(146, 151, C.4).

#### Scenario: A session limit with a time of day

- **WHEN** the hook ran at 13:10 in New York and recorded `rate_limit` with "You've hit your session limit · resets 3pm (America/New_York)"
- **THEN** `spent` reads yes until 15:00 that day in New York (151, C.4)

#### Scenario: A weekly limit with a date

- **WHEN** the words are "You've hit your weekly limit · resets Nov 2, 3:30pm (UTC) · progress saved"
- **THEN** `spent` reads yes until 2 November at 15:30 UTC of the hook's year (151, C.4)

#### Scenario: A reset past midnight

- **WHEN** the hook ran at 23:40 in UTC and the words say "resets 1am (UTC)"
- **THEN** `spent` reads yes until 01:00 UTC the next day (151)

#### Scenario: No reset in the words

- **WHEN** the words are "You've hit your limit"
- **THEN** `spent` reads yes with no reset (146, 151)

#### Scenario: A model under high load

- **WHEN** the hook recorded `rate_limit` with "Opus is experiencing high load, please use /model to switch to Sonnet"
- **THEN** `spent` reads no, and the words stay the agent's last words (43, 151)

### Requirement: The words pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source of its usage-limit words to stand byte for byte as the profile
reads them: its list of usage-limit openings, the `You've hit your`
template, the ` · resets ` part and the zone's suffix (core.22, C.4).

#### Scenario: The pinned release read

- **WHEN** the test reads the program devenv.nix pins, the file its `claude` wrapper runs
- **THEN** each fragment the profile's reading rests on is found in it, and the test names any that is not (C.4)

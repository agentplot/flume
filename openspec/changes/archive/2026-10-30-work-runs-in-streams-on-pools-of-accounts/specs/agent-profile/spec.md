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

### Requirement: The credentials Claude Code keeps from its hooks

The Claude Code profile's `withheld` SHALL name the credential
variables the pinned release keeps from every process it starts, its
hooks among them: its own list of credentials, the file-descriptor ones
it spreads into that list, and the memory service's token (161, C.4).

#### Scenario: The account's token

- **WHEN** a start's account gives `CLAUDE_CONFIG_DIR` and `CLAUDE_CODE_OAUTH_TOKEN`
- **THEN** `withheld` names `CLAUDE_CODE_OAUTH_TOKEN` and not `CLAUDE_CONFIG_DIR`, the start's record names `CLAUDE_CONFIG_DIR` alone of the two, and the SessionStart hook, whose environment holds no token, acks (161)

### Requirement: Own work read before the first turn

The Claude Code profile's `own_work` SHALL read no for a session whose
transcript the release has not yet written, since it writes the
transcript at the session's first message and own work starts only in a
turn; a transcript that cannot be read SHALL read unknown (91, 93).

#### Scenario: Idle at its prompt

- **WHEN** an agent's SessionStart hook recorded its session and transcript, and the agent sits at its prompt with no turn taken and no transcript written
- **THEN** `own_work` reads no and `asking` reads no (31, 90, 91)

### Requirement: The words pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source of its usage-limit words to stand byte for byte as the profile
reads them: its list of usage-limit openings, the `You've hit your`
template, the ` · resets ` part and the zone's suffix (core.22, C.4).

#### Scenario: The pinned release read

- **WHEN** the test reads the program devenv.nix pins, the file its `claude` wrapper runs
- **THEN** each fragment the profile's reading rests on is found in it, and the test names any that is not (C.4)

### Requirement: The credentials pinned to the release the tree holds

A `check-tools` test SHALL require the source the profile's `withheld`
rests on to stand byte for byte in the pinned Claude Code: its lists of
credentials, the memory service's pair, the test of the environment
against the list, the names its child environment drops and the drop,
and the hook runner's environment built on that child environment
(core.22, C.4).

#### Scenario: What the pinned release keeps from its hooks

- **WHEN** the test reads the program devenv.nix pins
- **THEN** each fragment `withheld` rests on is found in it, and the test names any that is not (161, C.4)

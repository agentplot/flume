## RENAMED Requirements

- FROM: `### Requirement: An agent's settings are resolved from six layers at each start`
- TO: `### Requirement: An agent's settings are resolved from eight layers at each start`

## MODIFIED Requirements

### Requirement: An agent's settings are resolved from eight layers at each start

At each start an agent's program, model, instructions and environment
SHALL each come from the first that gives it of: the account chosen
from its pool, for its program and variables alone; the launch's
settings; the host's for the repository and kind; the host's for the
stream and kind; the host's for the kind; the repository's for a kind
it does not declare; the kind's declaration; flume's default. Within
each, a stage's own SHALL come first (32; records 0017, 0020).

#### Scenario: Settings from four layers

- **WHEN** the host's configuration gives change@1's agents `env = { CLAUDE_CONFIG_DIR = "~/.claude-work" }`, the repository gives change@1's code stage `model = "opus"` without declaring it, and the launch gives verify `instructions`
- **THEN** construct's agent starts with `CLAUDE_CONFIG_DIR` under the host user's home, code's with the repository's model as well, and verify's with the launch's instructions (S44)

#### Scenario: The stream's layer

- **WHEN** the host gives `[kinds.change.agents] model = "sonnet"` and `[streams.work.kinds.change.agents] model = "opus"`, and `[repos."/r".kinds.change.agents] model = "haiku"`
- **THEN** a change place of `/s` in `work` starts with `opus` from `host-stream`, and one of `/r` in `work` with `haiku` from `host-repository`, since the repository's word comes before the stream's (32)

#### Scenario: The account over the launch

- **WHEN** a launch gives `env.CLAUDE_CONFIG_DIR` and the account chosen from the stage's pool names its own
- **THEN** the agent starts under the account's value, from `account`, and status shows the launch's as passed over by the account (32, 144)

### Requirement: The host's configuration is read at each start

The host's layers, its streams' and its pools' SHALL be read as they
stand at each start, and an agent already started SHALL keep what it
started with. A start whose host layers give a program no agent
profile binds, or a variable the login environment lacks, SHALL leave
the stage refused, naming it and its layer, starting no agent, until a
retry (8, 27, 32).

#### Scenario: The host's configuration changes while a stage works

- **WHEN** the host's configuration changes `CLAUDE_CONFIG_DIR` while code's agent works
- **THEN** code's agent keeps the value it started with, and verify's starts with the new one (S44, 8)

#### Scenario: A later start refused

- **WHEN** after a launch the host's configuration gives verify `env = { X = "$NOT_SET" }` and the login environment lacks `NOT_SET`
- **THEN** verify is refused naming the variable and the layer, no agent starts, and a retry after the host's configuration is mended starts it (27, 32)

#### Scenario: An account added to a pool

- **WHEN** an account is added to a stage's pool while the place's code agent works under another
- **THEN** code's agent is unchanged, and verify's start may take the new account (32, 145)

## ADDED Requirements

### Requirement: A kind and a repository name no account, pool or stream

A launch SHALL be refused naming the field, leaving nothing made, where
a repository's `.config/flume.toml` or a launch's declaration gives
`stream`, `pool` or `account` in a kind's `agents` table, or `stream`
or `pool` in a `[kinds.<kind>]` table; the kind format SHALL gain no
field for any of them (4, 32, 144; core.4).

#### Scenario: A pool in a repository's file

- **WHEN** a repository's `.config/flume.toml` gives `[kinds.change.agents] pool = "work"`
- **THEN** a launch of `change` there is refused naming `pool` and the table, and nothing is made (4, 144)

#### Scenario: A stream in a launch's declaration

- **WHEN** a launch's declaration gives its kind `stream = "work"`
- **THEN** the launch is refused naming `stream`, and the caller is left to pass `--stream` (4, 143)

### Requirement: A host's configuration from before streams runs as before

A host's configuration that declares no stream and names no pool SHALL
run every place in `default` under the host's bound alone, each agent
under the environment its kind's and repository's tables give, with
the same settings, layers and starts as before this change (70, 144;
record 0017).

#### Scenario: Record 0017's configuration

- **WHEN** a host's configuration of record 0017's shape, `[host] agents = 2` and `[kinds.change.agents] env = { CLAUDE_CONFIG_DIR = "~/.claude-work" }`, runs three change places
- **THEN** two start and one waits, each agent under that variable from `host-kind` and no account, and status names the stream `default` from `default` (70, 144)

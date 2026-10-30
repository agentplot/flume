## MODIFIED Requirements

### Requirement: A launch whose binding or settings flume cannot run is refused whole

A launch SHALL be refused, naming the program, field or stage and its
layer, and SHALL leave nothing made, when the binding names a place
profile flume does not hold, when a stage's agent resolves to a program
no agent profile binds or the host's terminal cannot run, when a
declaration gives an agent an `env`, or when settings name a stage the
kind does not hold (4, 32, 158, C.1, C.2).

#### Scenario: A program no agent profile binds

- **WHEN** a change is launched with `--agent code.program=nothing-here`
- **THEN** the launch is refused naming the program, the stage `code` and the layer `launch`, and nothing is made (4, 32)

#### Scenario: An environment in a repository's file

- **WHEN** a repository's `.config/flume.toml` gives `[kinds.change.agents] env = { A = "b" }` and a change is launched
- **THEN** the launch is refused naming the field, and nothing is made (C.2, core.30)

#### Scenario: A place profile flume does not hold

- **WHEN** a repository's binding names a place profile the resolver does not hold
- **THEN** the launch is refused naming the place profiles the resolver holds, and nothing is made (C.1, core.3)

### Requirement: An agent's settings are resolved from six layers at each start

At each start an agent's program, model and instructions SHALL each
come from the first that gives it of: the launch's settings; the host's
for the repository and kind; the host's for the kind; the repository's
for a kind it does not declare; the kind's declaration; flume's
default. The host's two SHALL give the model and instructions alone.
Within each, a stage's own SHALL come first, and each setting SHALL be
taken whole (32; records 0017, 0020).

#### Scenario: Settings from four layers

- **WHEN** the host's pool has one account giving `CLAUDE_CONFIG_DIR = "~/.claude-work"`, the repository gives change@1's code stage `model = "opus"` without declaring it, and the launch gives verify `instructions`
- **THEN** construct's agent starts with `CLAUDE_CONFIG_DIR` under the host user's home from `account`, code's with the repository's model as well, and verify's with the launch's instructions (S44)

#### Scenario: A model for each kind

- **WHEN** the host gives `[kinds.change.agents] model = "opus"`, `[kinds.fix.agents] model = "sonnet"` and `[repos."/r".kinds.fix.agents] model = "haiku"`
- **THEN** a change place's agents start with `opus` and a fix place's of `/s` with `sonnet`, each from `host-kind`, and a fix place's of `/r` with `haiku` from `host-repository` (32)

### Requirement: The host's configuration is read at each start

The host's layers, its streams', its pools' and their accounts' SHALL
be read as they stand at each start, and an agent already started SHALL
keep what it started with. A start whose account names a variable the
login environment lacks SHALL leave the stage refused, naming it and the
account, starting no agent, until a retry (8, 27, 32).

#### Scenario: The host's configuration changes while a stage works

- **WHEN** the host's configuration changes the account's `CLAUDE_CONFIG_DIR` while code's agent works under it
- **THEN** code's agent keeps the value it started with, and verify's starts with the new one (S44, 8)

#### Scenario: A later start refused

- **WHEN** after a launch the host's configuration gives the stage's account `env = { X = "$NOT_SET" }` and the login environment lacks `NOT_SET`
- **THEN** verify is refused naming the variable and the account, no agent starts, and a retry after the host's configuration is mended starts it (27, 32)

#### Scenario: An account added to a pool

- **WHEN** an account is added to a stage's pool while the place's code agent works under another
- **THEN** code's agent is unchanged, and verify's start may take the new account (32, 145)

## REMOVED Requirements

### Requirement: The environment merges by variable and the host's values are read against the login environment

**Reason**: an agent's environment comes from its account alone, so there is nothing to merge; the requirement below says how an account's values are read (record 0020).

## ADDED Requirements

### Requirement: The environment is the account's, its values read against the login environment

An agent's environment SHALL be the variables of the account chosen
for it and nothing else's, over the login environment; no launch, host
table, repository or kind SHALL give one. An account's value SHALL read
a leading `~` as the host user's home and `$VAR` or `${VAR}` from the
login environment, never the caller's session (29, 32, 144).

#### Scenario: The account's variables

- **WHEN** a stage's account gives `env = { CLAUDE_CONFIG_DIR = "~/.claude-work-1", TOOL_HOME = "$HOME/tool" }`
- **THEN** the agent starts with both read against the host user's home, status naming each from `account`, and no other variable of its settings (29, 32)

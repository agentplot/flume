## RENAMED Requirements

- FROM: `### Requirement: An agent's settings are resolved from six layers at each start`
- TO: `### Requirement: An agent's settings are resolved from eight layers at each start`

## MODIFIED Requirements

### Requirement: An agent's settings are resolved from eight layers at each start

At each start an agent's program, model, mode, instructions and own
settings SHALL each come from the first of eight layers that gives it:
the launch's; the host's for the repository, for its kind then for
every kind; the host's for the kind, for the stream, for every agent;
the repository's, for a kind it does not declare then for every kind;
the kind's; flume's default. The host's four SHALL give no program.
Within each, a stage's own SHALL come first, each setting taken whole
(32, 195).

#### Scenario: Settings from four layers

- **WHEN** the host's pool has one account giving `CLAUDE_CONFIG_DIR = "~/.claude-work"`, the repository gives change@1's code stage `model = "opus"` without declaring it, and the launch gives verify `instructions`
- **THEN** construct's agent starts with `CLAUDE_CONFIG_DIR` under the host user's home from `account`, code's with the repository's model as well, and verify's with the launch's instructions (S44)

#### Scenario: A model for each kind

- **WHEN** the host gives `[kinds.change.agents] model = "opus"`, `[kinds.fix.agents] model = "sonnet"` and `[repos."/r".kinds.fix.agents] model = "haiku"`
- **THEN** a change place's agents start with `opus` and a fix place's of `/s` with `sonnet`, each from `host-kind`, and a fix place's of `/r` with `haiku` from `host-repository` (32)

#### Scenario: A mode beside a model

- **WHEN** the host gives `[repos."/r".kinds.fix.agents] mode = "edits"` and `[kinds.fix.agents] model = "sonnet"`, and the repositories `/r` and `/s` each give `[kinds.fix.agents] mode = "ask"` in their own file
- **THEN** a fix place's agent of `/r` starts in `edits` from `host-repository` with `sonnet` from `host-kind`, and one of `/s` in `ask` from `repository` (32, 162)

#### Scenario: A model for a stream and for every agent

- **WHEN** the host gives `[agents] model = "sonnet"`, `[agents.streams.work] model = "opus"` and `[kinds.fix.agents] model = "haiku"`, and the repository's file gives `[kinds.change.agents] model = "other"`
- **THEN** a change place's agents start with `opus` from `host-stream` in `work` and with `sonnet` from `host` in `default`, and a fix place's with `haiku` from `host-kind` in either (32, 143)

#### Scenario: A model for every kind of a repository

- **WHEN** the host gives `[repos."/r".agents] model = "opus"` and `[kinds.fix.agents] model = "haiku"`, and the repository `/s`'s file gives `[agents] model = "sonnet"`
- **THEN** a fix's and a chore's agents in `/r` start with `opus` from `host-repository`, a fix's in `/s` with `haiku` from `host-kind`, and a chore's in `/s` with `sonnet` from `repository` (32)

## MODIFIED Requirements

### Requirement: An agent's settings are resolved from six layers at each start

At each start an agent's program, model, mode and instructions SHALL
each come from the first that gives it of: the launch's settings; the
host's for the repository and kind; the host's for the kind; the
repository's for a kind it does not declare; the kind's declaration;
flume's default. The host's two SHALL give the model, the mode and the
instructions alone. Within each, a stage's own SHALL come first, and
each setting SHALL be taken whole (32, 162; records 0017, 0020, 0024).

#### Scenario: Settings from four layers

- **WHEN** the host's pool has one account giving `CLAUDE_CONFIG_DIR = "~/.claude-work"`, the repository gives change@1's code stage `model = "opus"` without declaring it, and the launch gives verify `instructions`
- **THEN** construct's agent starts with `CLAUDE_CONFIG_DIR` under the host user's home from `account`, code's with the repository's model as well, and verify's with the launch's instructions (S44)

#### Scenario: A model for each kind

- **WHEN** the host gives `[kinds.change.agents] model = "opus"`, `[kinds.fix.agents] model = "sonnet"` and `[repos."/r".kinds.fix.agents] model = "haiku"`
- **THEN** a change place's agents start with `opus` and a fix place's of `/s` with `sonnet`, each from `host-kind`, and a fix place's of `/r` with `haiku` from `host-repository` (32)

#### Scenario: A mode beside a model

- **WHEN** the host gives `[repos."/r".kinds.fix.agents] mode = "edits"` and `[kinds.fix.agents] model = "sonnet"`, and the repositories `/r` and `/s` each give `[kinds.fix.agents] mode = "ask"` in their own file
- **THEN** a fix place's agent of `/r` starts in `edits` from `host-repository` with `sonnet` from `host-kind`, and one of `/s` in `ask` from `repository` (32, 162)

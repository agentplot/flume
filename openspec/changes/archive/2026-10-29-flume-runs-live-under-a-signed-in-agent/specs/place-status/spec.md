## MODIFIED Requirements

### Requirement: Status gives a place's current agent

Status SHALL give a place's current agent with its identity, its
settings each with the layer it came from, each value as that layer
wrote it but an environment variable's, which status names with the
value `<set>` and never gives, its state and whether it works past its
limit, and the command to run under a terminal that prints it, which
names the file holding its environment and holds no value of it (32,
56; record 0017).

#### Scenario: Each setting with its layer

- **WHEN** status is asked of a change whose code agent took its model from the repository and `CLAUDE_CONFIG_DIR = "~/.claude-work"` from the host's configuration for the kind
- **THEN** the agent's settings give the model from `repository` and the variable `CLAUDE_CONFIG_DIR` from `host-kind` as `<set>` (32, 56, S44)

#### Scenario: A token in no answer and no record

- **WHEN** the host's configuration gives change's agents `CLAUDE_CODE_OAUTH_TOKEN`, a launch gives them another token, and the launch, status as text and as JSON, and a settle are asked
- **THEN** no answer holds either token's value; the host's is in no file of flume's state but the printed command's environment file, the launch's in none but that and the place's held state, which keeps the launch's layer for later starts, and each of those the host's user alone may read (32, 56)

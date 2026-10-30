## MODIFIED Requirements

### Requirement: Status gives a place's current agent

Status SHALL give a place's current agent with its identity, its
settings each with the layer it came from and its variables each from
its account, each value as written but a variable's, which status names
with the value `<set>` and never gives, its state and whether it works
past its limit, and the command to run under a terminal that prints it,
which names the file holding its environment and holds no value of it
(32, 56; records 0017, 0020).

#### Scenario: Each setting with its layer

- **WHEN** status is asked of a change whose code agent took its model from the repository and `CLAUDE_CONFIG_DIR = "~/.claude-work"` from its account
- **THEN** the agent's settings give the model from `repository` and the variable `CLAUDE_CONFIG_DIR` from `account` as `<set>` (32, 56, S44)

#### Scenario: A token in no answer and no record

- **WHEN** the stage's account gives `CLAUDE_CODE_OAUTH_TOKEN`, and the launch, status as text and as JSON, and a settle are asked
- **THEN** no answer holds the token's value, and it is in no file of flume's state but the printed command's environment file, which the host's user alone may read (32, 56)

## ADDED Requirements

### Requirement: Status gives a place's stream, intent, account and moves

Status SHALL give a place's stream and intent, each with where it came
from, its current agent's account with whether it came from the
affinity or the strategy, each move of a stage or the intent between
accounts with from, to, why, when and that the cache was lost, and each
start that began without its settings, naming its variables alone (56,
143, 145 to 147, 161).

#### Scenario: A stage that moved

- **WHEN** status is asked of a place whose code stage moved from work-1 to work-2 when work-1 was spent
- **THEN** the agent's account reads work-2 from the strategy, and the place's moves give code, work-1, work-2, spent, when, and the cache lost, in the JSON and the person's form (56, 146)

#### Scenario: A start without its settings

- **WHEN** status is asked of a place whose code start began without `CLAUDE_CONFIG_DIR`
- **THEN** it names the start and `CLAUDE_CONFIG_DIR` as missing, and no value (56, 161)

### Requirement: Status gives the host's pools

Status SHALL give, for the host, each pool with its strategy and each
account with its agents running, its bound and the time it stops being
spent, and never an account's environment's value (56, 144 to 146).

#### Scenario: A pool with a spent account

- **WHEN** status is asked while work-1 runs two agents under a bound of four and work-2 is spent until 15:00
- **THEN** the pool `work` gives fill-first, work-1 with two running and a bound of four, and work-2 spent until 15:00 (56, 146)

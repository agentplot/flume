## MODIFIED Requirements

### Requirement: Status gives a place's stream, intent, account and moves

Status SHALL give a place's stream and intent, each with where it came
from, its current agent's account with whether it came from the
intent's affinity for the agent's program or the strategy, each move of
a stage or the intent between accounts with from, to, why, when and
that the cache was lost, an intent's naming the program whose account
moved, and each start that began without its settings, naming its
variables alone (56, 143, 145 to 147, 161).

#### Scenario: A stage that moved

- **WHEN** status is asked of a place whose code stage moved from work-1 to work-2 when work-1 was spent
- **THEN** the agent's account reads work-2 from the strategy, and the place's moves give code, work-1, work-2, spent, when, and the cache lost, in the JSON and the person's form (56, 146)

#### Scenario: An intent that moved for one program

- **WHEN** status is asked of a place whose code started on agent-2 after the intent moved for `agent` from agent-1, spent, and whose review started on other-1 from the affinity
- **THEN** the place's moves give the intent's move with the program `agent`, and the person's form reads that the intent moved for `agent` from agent-1 to agent-2; the review's agent reads other-1 from the affinity, and its line says from the affinity for `other` (56, 147, S80)

#### Scenario: A start without its settings

- **WHEN** status is asked of a place whose code start began without `CLAUDE_CONFIG_DIR`
- **THEN** it names the start and `CLAUDE_CONFIG_DIR` as missing, and no value (56, 161)

## ADDED Requirements

### Requirement: The fake terminal scripts whether it hands words

The fake terminal SHALL script whether it can type into a running
agent, and the fake agent SHALL script a session that ends and an
account spent after a stop-short, so the engine's scenarios read each
(41, 53, 146).

#### Scenario: A fake terminal that hands no words

- **WHEN** the fake terminal is scripted to hand no words to a running agent
- **THEN** it says it cannot, and a send to it is refused with nothing typed (53)

#### Scenario: A fake agent's session ended after its stop-short

- **WHEN** the fake agent is scripted to end its session after a stop-short
- **THEN** the fake reads the agent stopped and its session ended from then on (96, 134)

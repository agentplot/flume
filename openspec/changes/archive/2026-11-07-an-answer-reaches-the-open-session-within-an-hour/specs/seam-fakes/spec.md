## ADDED Requirements

### Requirement: The fake terminal scripts whether it hands words

The fake terminal SHALL script whether it can type into a running
agent, and the fake agent SHALL script a session that ends after a
stop-short, so the engine's scenarios read each (53, 96, 134, 198).

#### Scenario: A fake terminal that hands no words

- **WHEN** the fake terminal is scripted to hand no words to a running agent
- **THEN** it says it cannot, and a send to it is refused with nothing typed (53)

#### Scenario: A fake agent's session ended after its stop-short

- **WHEN** the fake agent is scripted to end its session after a stop-short
- **THEN** the fake reads the agent stopped and its session ended from then on (96, 134)

### Requirement: The fake host runs another process while a command sleeps

The fake host SHALL take what another process does while a command of
flume's sleeps on its clock, and run it once, at the next sleep, after
the clock moves, so the engine's scenarios place a settle an agent's
quiet started inside an act's wait (121, 198).

#### Scenario: Another process at the next sleep

- **WHEN** a test gives the fake host what another process does, advances its clock, then a command sleeps twice
- **THEN** it runs once, at the first sleep, after that sleep moved the clock, and not before

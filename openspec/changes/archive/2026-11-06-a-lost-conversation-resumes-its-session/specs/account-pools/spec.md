## ADDED Requirements

### Requirement: A lost conversation's start takes the account that holds it first

A start that carries a conversation lost with its terminal or its host
SHALL take the account its lost start ran under, while that account is
declared in the pool its stream takes, is of the stage's program and is
usable, before the intent's affinity and the strategy, and SHALL move no
intent's affinity by it. Otherwise it SHALL take the account 145
chooses, waiting for one as 145 says, and a move of the stage from the
lost start's account SHALL be recorded as any stage's is (49, 145, 147).

#### Scenario: The lost start's account while the intent keeps another

- **WHEN** a lost talk's start ran under work-2, the intent's affinity for its program is work-1, both usable, and the caller retries talk
- **THEN** talk starts under work-2 from its conversation, the intent's affinity stays work-1, and no move is recorded (145, 147, S113)

#### Scenario: The lost start's account spent

- **WHEN** a lost talk's start ran under work-1, work-1 is spent until 15:00 when the caller retries talk, and work-2 is usable
- **THEN** talk starts under work-2 by the strategy, and the place's moves give talk from work-1 to work-2, spent, with whether its conversation went with it (145, 146, S114)

#### Scenario: The lost start's account gone from the pool

- **WHEN** a lost talk's start ran under work-1, and the host's configuration no longer declares work-1 in the stream's pool when the caller retries talk
- **THEN** talk starts on the account 145 chooses, and the carry is asked of that account (145, 179)

#### Scenario: Every account spent

- **WHEN** a lost talk is retried while every account of its pool is spent, the soonest until 15:00
- **THEN** talk waits for room, and the start that takes work-1 once its reset passes carries the conversation from work-1 (145, 179)

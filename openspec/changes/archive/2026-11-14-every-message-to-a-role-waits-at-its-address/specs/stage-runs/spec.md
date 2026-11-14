## MODIFIED Requirements

### Requirement: Stuck, lost and working past its limit

A stage SHALL read stuck once its agent has been quiet past its limit,
30 minutes where none is declared, with nothing delivered or said; lost
once its agent stopped with nothing delivered or said; and its agent
working past its limit when it works that long with nothing delivered,
from the latest of the stage's beginning, its last answer's receipt in
its session and the place's last head move. The core SHALL end no agent
on any of these accounts (44, 45, 118, 198).

#### Scenario: Stuck under a program with no send-back

- **WHEN** the fake agent gives no way to send back and stays quiet past the limit with nothing delivered
- **THEN** status reports the stage stuck naming it and the agent, the agent is not ended, and after a message wakes it and it ticks the last task the stage ends at its deliverable (S17)

#### Scenario: Limits after an answer

- **WHEN** a code stage began two hours before its agent received an answer in its session, and the agent then works 20 minutes with nothing delivered under a 30-minute limit
- **THEN** it is not read working past its limit, and it is read so once it has worked 30 minutes since the answer (44, 118)

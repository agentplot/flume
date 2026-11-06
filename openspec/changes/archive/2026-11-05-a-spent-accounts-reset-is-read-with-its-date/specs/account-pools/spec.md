## MODIFIED Requirements

### Requirement: A spent account is paused for every flume process

Where an agent's own record says its account reached a usage limit,
flume SHALL record the account spent in the host's own documents until
the reset the record names, or for an hour where it names none its
profile reads, marking that hour so status gives the words, and no
start of any flume process on the host SHALL take it before then;
flume SHALL ask no agent program or its maker whether a limit has reset
(146, 151, 153).

#### Scenario: A reset at 15:00

- **WHEN** an agent's turn fails with its account's usage limit, the record naming a reset at 15:00, and another flume process starts a stage of another place on the same pool before then
- **THEN** that start passes the account over, a start after 15:00 may take it again, and status gives no words for the pause (146, S55)

#### Scenario: No reset named

- **WHEN** the record names a usage limit and no reset its profile reads
- **THEN** the account is spent for an hour from the reading, its document marks the hour, and status gives the agent's words with the account while the hour stands (56, 146)

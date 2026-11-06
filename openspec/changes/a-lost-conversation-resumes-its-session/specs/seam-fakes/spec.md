## ADDED Requirements

### Requirement: The fakes bring an agent back bare

The fake terminal and the fake agent SHALL together bring a start's
agent back bare, as a terminal's restore does: its pane holding an
agent again, waiting, whose started ack reads no, at a test's word, by
one helper that logs `bring back bare <identity>`; and the fake terminal
SHALL read it working, waiting or stopped as the test scripts (77, 106).

#### Scenario: A bare agent brought back

- **WHEN** a test brings a talk start's agent back bare and settles the place
- **THEN** the core reads the start's agent waiting with its started ack no, talk reads lost, and the fakes' log shows `bring back bare` for that identity (77)

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

### Requirement: The fake agent scripts a live session

The fake agent SHALL script, per session, the processes live answers
that run it, or that it cannot read it with words, answering none
otherwise; its log SHALL name each `live` it answers (106, 207).

#### Scenario: A scripted live session that ends

- **WHEN** a test scripts the session `s-1` live as process 4242, asks live, then scripts it ended and asks again
- **THEN** live answers runs with process 4242, then none, and the fake's log shows both readings (207)


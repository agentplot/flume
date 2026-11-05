## ADDED Requirements

### Requirement: A retry answers a stop-short by its key

A retry naming a stop-short's key by `--answers <key>` SHALL be taken
only while the place waits on that stop-short, and SHALL record on its
end the answer's words, who gave them, as the retrying command's
recipient names it, and when. Otherwise it SHALL be refused, naming the
answer taken, who gave it and when, or the move the place waits on now.
A retry with no key SHALL be taken as before (42, 49).

#### Scenario: One answer taken

- **WHEN** a construct ended short needing an answer under the key `stage/construct/1/ended-short`, and the answerer's run retries construct with that key and the user's words
- **THEN** a fresh agent starts whose prompt carries the question, then the words, and status gives the end answered by the run's address, with when (S91)

#### Scenario: A second answer to the same question

- **WHEN** a retry from the pane `ops` names the same key a minute later
- **THEN** it is refused naming the run's address, its words and when, and no agent is ended or started (S91)

#### Scenario: An answer after the place moved on

- **WHEN** construct, retried with an answer, delivers, code starts, and a late retry names construct with the first stop-short's key
- **THEN** it is refused naming the move the place waits on now, none, and code goes on

#### Scenario: A retry with no key

- **WHEN** a caller retries a stage that ended short with words and no key
- **THEN** it is taken as before, and the end records the answer with no key named

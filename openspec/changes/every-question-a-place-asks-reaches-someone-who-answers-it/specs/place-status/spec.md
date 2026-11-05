## ADDED Requirements

### Requirement: Status gives who answers a place

Status SHALL give each place's recipients, each with where it came
from; each end's key, and a stop-short's answer with who gave it and
when; a hold on the waiting move with its words, since when, its bound
or that it lapsed; and for the host and each stream, its answerer, and
for each address an end stands pending for, its run and the session
serving it, with since when (56, 130, 132, 190).

#### Scenario: Recipients and their sources

- **WHEN** one place names `ops` at its launch and another took the host's answerer `ops/answerer`
- **THEN** status gives the first `ops` from the launch and the second `ops/answerer` from the host's answerer, in the JSON form and the person's

#### Scenario: An answered question

- **WHEN** a construct's stop-short was answered by a retry naming its key from the run of `ops/answerer`
- **THEN** status gives the end's key with the answer's words, `ops/answerer` and when

#### Scenario: A served address

- **WHEN** a session serves `madswan/planner` and an end stands pending for it
- **THEN** status gives the address with no run and that session, since when it served

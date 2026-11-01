## MODIFIED Requirements

### Requirement: Status says what a waiting place waits on

Status SHALL give a stop-short's words and whether it needs an answer or
found work, or a failed submission's words, and, for a place waiting on
its caller, the move it waits on and since when, a hold's words among
it, and, for an answer, whether its agent's session is open for it and
until when, or when and why it closed and whether its agent is ended
(41, 56, 130).

#### Scenario: A place waiting on an answer

- **WHEN** status is asked of a place whose construct ended short needing an answer at a time t, its agent's session open
- **THEN** it gives the move waited on as a retry of construct, since t, the question whole, and the session open until t plus its `keep_open`, in the JSON form and the person's (56, 130)

#### Scenario: Its session closed

- **WHEN** status is asked of that place once its session closed by its time and its agent was ended
- **THEN** it gives the same move since t, and the session closed with when, `time`, and when its agent was ended (41, 130, S76)

## ADDED Requirements

### Requirement: Status gives each answer a session took

Status SHALL give, with each stage, every answer its current start took
in its session, in order, with its question, the act it came by, and
when it was sent and received, in the JSON form and the person's (42,
56).

#### Scenario: Two answers in one session

- **WHEN** status is asked of a fix whose code took one answer by a retry and one by a tell in its first start's session
- **THEN** code gives both in order, each with its question, its answer, `retry` or `tell`, and when it was sent and received (42, 56)

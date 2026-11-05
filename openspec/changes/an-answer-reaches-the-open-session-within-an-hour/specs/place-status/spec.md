## MODIFIED Requirements

### Requirement: Status says what a waiting place waits on

Status SHALL give a stop-short's words and whether it needs an answer or
found work, or a failed submission's words, and, for a place waiting on
its caller, the move it waits on and since when, a hold's words among
it, and, for an answer, whether its agent is kept open for it and until
when, or when and why it was let go and whether it is ended (56, 130,
198).

#### Scenario: A place waiting on an answer

- **WHEN** status is asked of a place whose construct ended short needing an answer at a time t, its agent kept open
- **THEN** it gives the move waited on as a retry of construct, since t, the question whole, and its agent kept open until t plus its `keep_open`, in the JSON form and the person's (56, 130)

#### Scenario: Its agent let go

- **WHEN** status is asked of that place once its agent was let go by its time and ended
- **THEN** it gives the same move since t, and its agent let go with when, `time`, and when it was ended (130, 198, S104)

## ADDED Requirements

### Requirement: Status gives each answer a session took

Status SHALL give, with each stage, every answer its current start took
in its session, in order, with its question, the act it came by, and
when it was sent and received, in the JSON form and the person's (42,
56).

#### Scenario: Two answers in one session

- **WHEN** status is asked of a fix whose code took one answer by a retry and one by a tell in its first start's session
- **THEN** code gives both in order, each with its question, its answer, `retry` or `tell`, and when it was sent and received (42, 56)

### Requirement: Status gives keep_open with its layer

Status SHALL give an agent's `keep_open` with its settings, as written,
with the layer it came from, flume's own one hour named `default`
(32, 56, 198).

#### Scenario: From the host's table for every agent

- **WHEN** the host gives `[agents] keep_open = "20m"` and no other layer gives one
- **THEN** status gives the agent's `keep_open` `20m` from `host` (32, 56)

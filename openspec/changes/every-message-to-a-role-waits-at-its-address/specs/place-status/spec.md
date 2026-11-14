## MODIFIED Requirements

### Requirement: Status gives the roles a host wakes

Status SHALL give, for each stream, its answerer with the source that
declares it; each source's roles, or why they cannot be read; and for
each address an end or a message stands pending for, its run, or that
it is carried or wakes nothing and why, each message pending at it with
its key, who sent it, when, its words' first line, each hand-over and
whether it is expired, and each place whose work is not over that names
it among its recipients, each read at the ask (56, 132, 191, 193, 214;
core.15).

#### Scenario: An address that wakes nothing

- **WHEN** an end stands pending for `east/planner`, which no source declares, on a place whose stream's answerer is `east/operator`
- **THEN** status gives the address with no run and that no source declares it, and the stream's answerer `east/operator`

#### Scenario: A role's messages and the places reporting to it

- **WHEN** the run of `east/ops` launched two places naming no recipient, both still working, and a message from `ops` is pending at `east/ops`, handed to it by nothing
- **THEN** status gives `east/ops` with its run, `messages` holding the message's key, `ops`, when and its first line, handed to none, and `reporting` naming both places; the person's form gives the same under the address (56, 214, S128)

#### Scenario: A place whose work is over

- **WHEN** one of those places lands and is tidied
- **THEN** status gives `reporting` naming the other alone

### Requirement: Status gives each answer a session took

Status SHALL give, with each stage, every answer its current start took
in its session, in order, with its question, the act it came by, and
when it was sent and received, in the JSON form and the person's (42,
56).

#### Scenario: Two answers in one session

- **WHEN** status is asked of a fix whose code took one answer by a retry and one by a message in its first start's session
- **THEN** code gives both in order, each with its question, its answer, `retry` or `message`, and when it was sent and received (42, 56)

## ADDED Requirements

### Requirement: Status gives who answers a place

Status SHALL give each place's recipients, each with where it came
from; the role a stop-short named; each end's key, and a stop-short's
answer with who gave it and when; a hold on the waiting move with its
words, since when, its bound or that it lapsed; for the host and each
stream, its answerer; and for each address an end stands pending for,
its run, or that it is carried or wakes nothing (56, 130, 132, 191).

#### Scenario: Recipients and their sources

- **WHEN** one place names `ops` at its launch and another took the host's answerer `acme/operator`
- **THEN** status gives the first `ops` from the launch and the second `acme/operator` from the host's answerer, in the JSON form and the person's

#### Scenario: An answered question

- **WHEN** a construct's stop-short, naming `acme/ops`, was answered by a retry naming its key from the run of `acme/ops`
- **THEN** status gives the end's key, `acme/ops` as its addressee, and the answer's words with `acme/ops` and when

#### Scenario: An address that wakes nothing

- **WHEN** an end stands pending for `acme/planner`, which `[roles]` names nothing for, on a host whose answerer is `acme/operator`
- **THEN** status gives the address with no run and that it wakes nothing, and the host's answerer `acme/operator`

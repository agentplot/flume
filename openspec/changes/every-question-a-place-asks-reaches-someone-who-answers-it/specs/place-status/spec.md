## ADDED Requirements

### Requirement: Status gives who answers a place

Status SHALL give each place's recipients, each with where it came
from; the role a stop-short named; each end's key, and a stop-short's
answer with who gave it and when; and a hold on the waiting move with
its words, since when, its bound or that it lapsed (56, 130, 193).

#### Scenario: Recipients and their sources

- **WHEN** one place names `ops` at its launch and another took its stream's answerer `acme/operator`
- **THEN** status gives the first `ops` from the launch and the second `acme/operator` from the stream's answerer, declared by the source `acme`, in the JSON form and the person's

#### Scenario: An answered question

- **WHEN** a construct's stop-short, naming `acme/ops`, was answered by a retry naming its key from the run of `acme/ops`
- **THEN** status gives the end's key, `acme/ops` as its addressee, and the answer's words with `acme/ops` and when

### Requirement: Status gives the roles a host wakes

Status SHALL give, for each stream, its answerer with the source that
declares it; each source's roles, or why they cannot be read; and for
each address an end stands pending for, its run, or that it is carried
or wakes nothing and why (56, 132, 191, 193).

#### Scenario: An address that wakes nothing

- **WHEN** an end stands pending for `acme/planner`, which no source declares, on a place whose stream's answerer is `acme/operator`
- **THEN** status gives the address with no run and that no source declares it, and the stream's answerer `acme/operator`

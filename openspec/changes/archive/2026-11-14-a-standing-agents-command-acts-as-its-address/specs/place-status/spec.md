## MODIFIED Requirements

### Requirement: Status gives who answers a place

Status SHALL give each place's recipients, each with where it came
from, a recipient a standing agent's launch gave with that agent and
the source that declares it; the role a stop-short named; each end's
key, and a stop-short's answer with who gave it and when; and a hold on
the waiting move with its words, since when, its bound or that it
lapsed (56, 130, 132, 193).

#### Scenario: Recipients and their sources

- **WHEN** one place names `ops` at its launch and another took its stream's answerer `east/operator`
- **THEN** status gives the first `ops` from the launch and the second `east/operator` from the stream's answerer, declared by the source `acme`, in the JSON form and the person's

#### Scenario: A recipient from a standing agent's launch

- **WHEN** a place was launched naming no recipient from the pane of `east-planner`, which `acme` gives `east/planner`
- **THEN** status gives `east/planner` from the standing agent `east-planner`, declared by `acme`: in the JSON form `recipients_from` with `from` `standing`, `agent` and `source`, and in the person's as `east/planner, from the standing agent east-planner that launched it, declared by acme` (56, 132)

#### Scenario: An answered question

- **WHEN** a construct's stop-short in `east`, naming `ops`, was answered by a retry naming its key from the run of `east/ops`
- **THEN** status gives the end's key, `east/ops` as its addressee, and the answer's words with `east/ops` and when

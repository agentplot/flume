## MODIFIED Requirements

### Requirement: Status gives what waits, what was made and what is pending

Status SHALL give, beside clause 56's other facts, each place's
recipients; each end with its delivery's key and expiry, its
hand-overs, and per recipient whether it is pending, expired or
delivered, and whether its name is known; for a place waiting on its
caller, the move, since when, a hold's words and whether it is stalled;
a role run's address; each stage's made lines; and the ends pending for
an address the host names no role for (43, 56, 129, 130, 132; weir 104
to 106).

#### Scenario: A stalled place with a pending end

- **WHEN** status is asked of a place stalled on a retry of construct whose end was handed to `ops` by notice and confirmed by it, and is pending for `madswan/planner`
- **THEN** the answer gives the move, since when, stalled, the end's hand-over to `ops` with when, its confirmation by `ops` with when, and pending for `madswan/planner`, a name never confirmed under

#### Scenario: The fields kept

- **WHEN** the suite's field list is read after this change
- **THEN** it holds the recipients, the ends with their deliveries and `pending_for`, each end's `delivery`, `expires_at`, `owed` and `confirmed`, the waiting move, the hold's words, stalled, the role address, the made lines and the host's `expired`, and an answer lacking one fails the suite (61)

## ADDED Requirements

### Requirement: Status gives every end expired on the host

Status SHALL give, for the host, each end expired for a recipient, of
every place and every tidied document, with its place, its key, its
delivery's key, the recipient, whether its name is known and when it
expired, reading the host's deliveries once for the ask, and the
person's form SHALL print each (43, 56; weir 104, 106; core.15).

#### Scenario: Two expired ends

- **WHEN** two tidied fixes' landings are pending for `bomdrive` past the host's span, and status is asked of the host
- **THEN** the JSON form's `expired` gives both, each naming `bomdrive`, `known` false, its place and its delivery's key, and the person's form prints a line for each saying `never confirmed`

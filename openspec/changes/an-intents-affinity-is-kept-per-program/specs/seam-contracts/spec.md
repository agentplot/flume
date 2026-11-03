## MODIFIED Requirements

### Requirement: The types of streams, pools and a start's record

The seams SHALL pass one type each for a stream, a pool, its strategy
and its accounts, an account chosen and where from, a move between
accounts, an intent's naming the program whose account moved, an
intent's affinity with an account for each program, and a start's record
of what its agent runs under, held as the start's `start_record` and
holding digests and no value; each SHALL serialize to JSON and back
unchanged. An affinity written before SHALL read with its one account
and its pool, and its intent's moves with no program, and SHALL never be
written so (143 to 147, 153, 161).

#### Scenario: A held state with a stream, an account and a record

- **WHEN** a held state whose place has a stream with `from`, an intent, and a start with an account and a start's record is written and read
- **THEN** it reads back unchanged, and the record holds each variable's digest and no value (143 to 147, 161)

#### Scenario: An affinity of two programs

- **WHEN** an affinity holding an account for `agent` and one for `other`, and an intent's move for `agent`, is written and read
- **THEN** it reads back unchanged, and it once left the move's `from` for `agent` and never for `other` (147, 153)

#### Scenario: An affinity written before

- **WHEN** the JSON of an affinity with one `pool` and `account` and an intent's move with no program is read, and written again
- **THEN** it reads with that pool and account, no account for any program and the move with no program, and is written with no `pool` or `account` key once they are taken away (147, 153)

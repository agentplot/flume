## ADDED Requirements

### Requirement: serve, hold --waiting and retry --answers

The command line SHALL take `flume serve <address>`, `flume hold <place>
<words> --waiting [--for <time>]` with the time in the forms
`--stall-after` takes, and `flume retry <place> --answers <key>`, each
answering from the one core in the JSON form and the person's, and each
key as the inbox and status give it (42, 49, 61, 130, 132).

#### Scenario: The forms

- **WHEN** `flume serve madswan/planner --json`, `flume hold change--x "asked on card req_1" --waiting --for 1h --json` and `flume retry change--x --stage construct --answers stage/construct/1/ended-short --words "yes" --json` run in turn against the fakes
- **THEN** each answers in the JSON form, the serve with what an inbox gives, the hold with its words and bound, and the retry with the place's status

#### Scenario: A hold's bound in a wrong form

- **WHEN** `flume hold change--x "words" --waiting --for soon` runs
- **THEN** it is refused naming the forms a time takes, and nothing is held

#### Scenario: A key in a wrong form

- **WHEN** `flume retry change--x --answers construct` runs
- **THEN** it is refused naming the forms a key takes, and no agent is ended

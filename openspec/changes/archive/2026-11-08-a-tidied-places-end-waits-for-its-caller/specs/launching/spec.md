## ADDED Requirements

### Requirement: A launch may name its caller

A launch SHALL take a caller, a name of the launching caller's own, and
SHALL record it on the place, where nothing after the launch changes it;
status SHALL give it, and a tidied place's document SHALL keep it. A
caller written empty SHALL refuse the launch whole, naming it (7, 8,
56).

#### Scenario: A caller named

- **WHEN** a launch names the recipient `madswan/planner` and the caller `boomdrive`
- **THEN** status gives the place's one recipient `madswan/planner`, from the launch, and its caller `boomdrive`, and a retry and a hold after the launch leave the caller as it was (7, 8, 56)

#### Scenario: An empty caller

- **WHEN** a launch names an empty caller
- **THEN** it is refused naming the caller, and nothing of it is made (4, 7; core.8)

### Requirement: A caller is no recipient and no owner

A caller SHALL change none of the place's recipients, and SHALL give no
right over the place that another caller lacks. A launch that names no
caller SHALL have none, whatever `FLUME_RECIPIENT` or its session give,
and a role's run, launched by a wake, SHALL have none (7, 69, 132).

#### Scenario: No caller named

- **WHEN** a launch names no recipient and no caller, with `FLUME_RECIPIENT` set to `ops`
- **THEN** the place's recipient is `ops`, from the variable, and status gives it no caller (7)

#### Scenario: A woken run has no caller

- **WHEN** an end pending for `east/ops`, which the scratch source `acme` declares with a run, wakes that run
- **THEN** status gives the run's place no caller (132)

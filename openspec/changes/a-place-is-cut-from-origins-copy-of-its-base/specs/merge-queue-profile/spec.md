## MODIFIED Requirements

### Requirement: Read, base and the place's data

The profile SHALL read the newest entry of a branch from `sluice status
<base> --json`; answer the base at `<remote>/<base>` after one fetch
where `.config/sluice.toml` lands the base on a remote, the local base
where the base's table names no remote, and that sluice lands no such
base where the policy names no table for it or no revision of it holds
a policy; and give the place profile the readiness, the teardown
(`sluice release` in the place) and the base's ref, read from
`.config/sluice.toml` once sluice is bound (9, 99, 125, 126, C.1, C.6,
core.3).

#### Scenario: A queue nothing drives

- **WHEN** a place's entry waits on a base no `sluice run` drives and a settle steps it
- **THEN** one round runs and the entry is decided (124, 125)

#### Scenario: A fetch that fails

- **WHEN** the base lands on a remote that refuses the fetch
- **THEN** base answers why, and the launch is refused naming it (9, core.7)

#### Scenario: A base the policy names no table for

- **WHEN** base is asked of `trial`, a branch made from main whose policy names a table for main alone
- **THEN** it answers that sluice lands no such base, and main's at a revision (126, C.6)

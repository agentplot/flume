## MODIFIED Requirements

### Requirement: Read, base and the place's data

The profile SHALL read the newest entry of each branch asked from
`sluice overview --json` and `sluice status <base> --all --json` for
each base the overview names, run once for every branch of one
repository asked at once; answer the base at `origin/<base>` after one fetch
where `.config/sluice.toml` lands the base on a remote and the local
base otherwise; and give the place profile the readiness, `sluice equip
<base> --at <revision> --json` with the reading of its steps, the
teardown (`sluice release`) and the base's ref, reading no `[tests]`
key for a place (9, 11, 20, 99, 126, C.1, C.6, core.3).

#### Scenario: Every branch asked at once

- **WHEN** read-submission is asked at once of `fix/one` and `fix/two`, each submitted to main, and `fix/none`, submitted nowhere
- **THEN** sluice runs `overview` once and `status main --all` once, and the answer gives `fix/one` and `fix/two` each its own entry and `fix/none` none (99, C.6)

#### Scenario: A queue nothing drives

- **WHEN** a place's entry waits on a base no `sluice run` drives and a settle steps it
- **THEN** one round runs and the entry is decided (124, 125)

#### Scenario: A fetch that fails

- **WHEN** the base lands on a remote that refuses the fetch
- **THEN** base answers why, and the launch is refused naming it (9, core.7)

#### Scenario: What the policy says of a place

- **WHEN** the policy at main names the moon runner, the toolchain `devenv print-dev-env` and `origin` as main's remote, and a place on main is readied at the revision R its launch read
- **THEN** the place's data is one readiness step, `sluice equip main --at R --json`, with the profile's reading of it, `sluice release` as its teardown and `origin` as the base's remote, and names no toolchain and no `moon setup` (11, 20, C.1, C.6)

#### Scenario: The steps sluice equipped with

- **WHEN** `sluice equip` answers the steps `toolchain`, `proto install` and `moon --version` passed and `moon setup` failed with its words, exiting 1
- **THEN** the reading gives those four steps in that order, each with its outcome, the last with sluice's words cut as any step's are (12, 79, C.6)

#### Scenario: An answer with no steps

- **WHEN** `sluice equip` answers a refusal with its `error.message`, or nothing the profile can read, or cannot be run
- **THEN** the reading gives one failed step, `sluice equip`, with sluice's message, its errors, or why it could not run (12, C.6)

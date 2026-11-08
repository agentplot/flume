## ADDED Requirements

### Requirement: The flake is a project of its own

The flake SHALL be the moon project `nix`, at `nix/`, listed in
`.moon/workspace.yml` with `language: 'nix'`. Its `format`, `lint`,
`typecheck` and `test` tasks SHALL be in the check-fast layer. Its
`build` task SHALL be in no layer, with a comment above it saying the
gate's sandbox denies the daemon and the network it needs (core.21).

#### Scenario: The build task's layer

- **WHEN** `root:layers` reads the `nix` project's tasks
- **THEN** `format`, `lint`, `typecheck` and `test` each carry `check-fast`, and `build` carries none with a comment above it saying why the gate cannot run it

### Requirement: The flake's checks need neither the network nor the daemon

`nix:typecheck` and `nix:test` SHALL evaluate `nix/tests` offline,
read-only and against a dummy store, from the inputs `devenv.nix`
fetches outside the sandbox and gives as `FLUME_FLAKE_INPUTS`, and
SHALL refuse to run where those are not given.

#### Scenario: The checks in the gate's sandbox

- **WHEN** `nix:typecheck` and `nix:test` run under `sluice exec`, in the gate's sandbox
- **THEN** each evaluates and reports `nix: <check>: every case holds`, having written nothing to any store

#### Scenario: No inputs given

- **WHEN** `nix:test`'s script cases run `nix/check.sh test` with `FLUME_FLAKE_INPUTS` unset
- **THEN** it exits non-zero saying the devenv shell sets it

### Requirement: CI builds the flake's package

CI SHALL run `nix:build` in the devenv shell on Linux and macOS for each
pull request and each push to main, so a package that no longer builds
fails before a release carries it.

#### Scenario: The workflow's build job

- **WHEN** `nix:test` reads `.github/workflows/ci.yml`
- **THEN** a job `flake` runs on `ubuntu-latest` and `macos-latest` on each pull request and each push to main, and its step runs `devenv shell -- moon run nix:build`

### Requirement: The flake's tools are the versions the tree pins

nix, nixfmt, statix and deadnix SHALL be the versions `devenv.nix`
pins, and `root:toolchain` SHALL hold each on `PATH` to its pin
(core.22).

#### Scenario: The nix tools at their pins

- **WHEN** `root:toolchain` runs in the devenv shell
- **THEN** it names nix, nixfmt, statix and deadnix each at the version `devenv.nix` pins, and fails naming the tool and both versions where one is not

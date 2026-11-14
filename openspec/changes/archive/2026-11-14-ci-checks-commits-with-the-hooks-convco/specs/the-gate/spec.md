## ADDED Requirements

### Requirement: CI's conventional check runs the hook's convco

CI's conventional check SHALL run `convco check --ignore-reverts` with
the convco of the nixpkgs `devenv.lock` pins, the one the commit-msg
hook runs, and never the flake registry's, so a session and CI hold a
commit to one convco (core.22).

#### Scenario: The workflow's conventional job

- **WHEN** `nix:test` reads `.github/workflows/ci.yml`
- **THEN** the job `conventional` reads the nixpkgs `devenv.lock` pins and runs `convco check --ignore-reverts` from it, and names no `nixpkgs#convco` of the registry

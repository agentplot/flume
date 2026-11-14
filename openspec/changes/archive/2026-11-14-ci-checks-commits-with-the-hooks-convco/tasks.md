# Tasks

The work is on `fix/main--conventional-builds-convco`.

## 1. The check

- [x] 1.1 Have CI's conventional job run `convco check --ignore-reverts` from the nixpkgs `devenv.lock` pins, the hook's convco (core.22). True when the job's command over v0.21.2..HEAD runs convco 0.6.3 and finds "no errors in 52 commits", and cache.nixos.org holds that convco for x86_64-linux.
- [x] 1.2 Have `nix:test` hold the job to it by the workflow's text. True when `sluice exec main -- moon run nix:test` passes, and fails with the job's step at `nix shell nixpkgs#convco`.

## 2. The specs

- [x] 2.1 Archive this change so the specs reconcile (core.19). True when `openspec validate --specs --strict` passes.

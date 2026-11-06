# Tasks

The work is on `chore/the-conventional-check-takes-a-revert`.

## 1. The check and the hook

- [x] 1.1 Have CI's conventional job run `convco check --ignore-reverts` from the newest v* tag to the head (core.22). True when the job's command over v0.10.0..main, which holds b2fb0a6, passes its 64 other commits under convco 0.7.2, nixpkgs-unstable's, and 0.6.3, devenv's, and fails on b2fb0a6 without the flag.
- [x] 1.2 Have the convco hook's entry be `conventional` (devenv.nix), which passes a message whose first line starts `Revert "` and hands every other to `convco check --from-stdin --config .versionrc`; have `check-hooks.sh` commit a revert `git revert --no-commit` prepared and refuse a message with no type (core.21, core.22). True when `sluice exec main -- moon run root:hooks` passes.
- [x] 1.3 Say in `.versionrc` and `cliff.toml` what each does with a revert. True when git-cliff 2.14.2's `--unreleased --bumped-version` and its notes over v0.10.0..main are the same with b2fb0a6 skipped and without.

## 2. The gate

- [ ] 2.1 Run the gate's checks. True when `sluice exec main` passes.
- [ ] 2.2 Archive this change so the specs reconcile (core.19). True when `openspec validate --specs --strict` passes.

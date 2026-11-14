# CI checks commits with the hook's convco

## Why

CI's `conventional` job fails on main at 8bc4162 and every landing since
at least ae188b7, so `binaries` and `release` are skipped and no release
of flume ships. The job ran `nix shell nixpkgs#convco`, the flake
registry's nixpkgs, which floats: on 2026-11-14 its convco 0.7.2 was not
in the binary cache, and its build from source aborted, "The environment
variable LIBGIT2_NO_VENDOR has been set but no compatible system libgit2
could be found" (no `libgit2-experimental` for libgit2-sys
0.18.8+1.9.7). CI and the commit-msg hook also ran two convcos, 0.7.2
and 0.6.3, where the-gate holds a session and CI to one rule (core.22).

## What Changes

- **CI's conventional check runs the hook's convco.** The job reads the
  nixpkgs `devenv.lock` pins, the input devenv's packages and so the
  convco hook's come from, and runs `convco check --ignore-reverts` from
  that nixpkgs: 0.6.3 at c2f38fe, which cache.nixos.org holds for
  x86_64-linux, so nothing is built. The check moves with the shell's
  lock: a convco the shell can no longer get breaks every session's
  shell before it reaches CI.
- **`nix:test` holds the job to it,** by the workflow's text as it holds
  the job `flake`.

Not taken: `nix shell --inputs-from . nixpkgs#convco`, as timber's and
sluice's jobs run it, which reads `flake.lock`'s nixpkgs, convco 0.7.2
at f9a75f4, cached today but a second convco beside the hook's; and the
check run in the devenv shell, as the job `fast` runs, the same convco
for the cost of installing devenv and entering its shell.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `the-gate`: CI's conventional check runs the convco the commit-msg
  hook runs, held by `nix:test`.

## Impact

- `.github/workflows/ci.yml`: the conventional job's step.
- `nix/tests/workflow.nix`: the job's test, in `nix:test`.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool.

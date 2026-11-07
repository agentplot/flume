# flume publishes its own flake and module

## Why

flume's managed file, `~/.config/flume/config.toml`, is written today
by the host's own tooling, outside anything of flume's. The Boomdrive
flake configures only its runner and builds flume itself from a tag,
`flake = false`, since flume publishes no flake. A host that runs flume
without Boomdrive has no way to write flume's file through Nix at all.

The user ruled on 2026-11-06 that flume publishes a flake of its own
with a `services.flume` module for macOS and NixOS. The module writes
flume's whole managed file from options named as the file names them,
and no other tool's file. The Boomdrive flake takes flume's flake as an
input and re-exports its module, so a host adds the Boomdrive flake
alone, and flume stays usable without Boomdrive: "Flume's flake,
re-exported", "Yes, all of Flume's file" (record 0055, accepted;
agentplot/boomdrive record 0023, accepted). The statement's 143 and C.7
name the file's tables and keys this module writes.

## What Changes

- **A flake at the root.** `flake.nix` and `flake.lock` make
  `github:agentplot/flume` a flake. It gives `packages.<system>.flume`,
  and `default` the same, for aarch64-darwin, aarch64-linux and
  x86_64-linux, built from the tree itself. Its `nixpkgs` and
  `rust-overlay` inputs are named so that a flake taking it, Boomdrive's
  among them, can make them follow its own.
- **A module for each platform.** `darwinModules.flume` and
  `nixosModules.flume`, each also `default`, give `services.flume`:
  `enable`, `package`, `settings` and, on NixOS, `user`. `settings` is
  flume's managed file, each key named as C.7 names it: `host`,
  `streams`, `pools` with their `accounts`, `kinds`, `repos` and
  `agents`. The tables and keys C.7 types are declared for their types.
  A key it does not declare passes to the file, and flume judges it.
  Nothing flume defaults is defaulted again.
- **The whole managed file, and no other.** The module renders
  `settings` to TOML at evaluation, puts it in the store, and links the
  user's `~/.config/flume/config.toml` to it at activation. A file
  there that is no link is moved aside once. It never writes
  `config.local.toml`, a source's file under `sources/`, the host's
  lock, or any other tool's file (C.7; core.45). flume itself still
  writes neither layer.
- **No credential in the store.** A string value of `settings` shaped
  as a credential by flume's own shapes (core.50) fails evaluation,
  naming its path and never its value. The store is readable by every
  user. A `$VAR` naming a variable of the login environment passes (29).
- **The gate holds it.** A moon project `nix` checks the flake's format,
  lint, evaluation and tests in the check-fast layer, offline and
  against no store. Each rendered file is compared byte for byte and
  read by flume's own reader of the host's configuration. A `nix:build`
  task in no layer builds the package, and CI runs it on Linux and
  macOS.
- **The operator's pages.** The site's install page says how to
  install and configure flume with Nix, and its configuration reference
  says the managed layer may be written by the module.

## Capabilities

### New Capabilities

- `flake`: the flake's package for each system, built from the tree,
  its inputs a taking flake can follow, and the outputs it gives.
- `host-module`: `services.flume` on nix-darwin and NixOS. It covers
  the options, the rendered managed file, the credential refusal, the
  link at activation, and what the module never writes.

### Modified Capabilities

- `the-gate`: the flake's own moon project, its pinned tools, and its
  package built by CI on Linux and macOS.

## Impact

- New: `flake.nix`, `flake.lock`, and `nix/`, which holds the moon
  project, the package, the git crates' hashes, the modules, the
  activation script, the lock reader, the check script and the
  evaluated tests with their rendered files.
- `devenv.nix`: nix, nixfmt, statix and deadnix pinned on the shell,
  and the flake's inputs given to the checks as `FLUME_FLAKE_INPUTS`.
  `check-toolchain.sh` holds the new pins.
- `.moon/workspace.yml` lists the `nix` project. `treefmt.nix` excludes
  `nix/tests/files/**`, which the tests compare byte for byte.
  `crates/flume/moon.yml`'s test inputs gain the rendered files.
- `crates/flume-core`: the credential shapes' cases move to a table,
  `tests/credential-shapes.json`, which the core's tests and the
  module's port both read.
- `crates/flume/tests/host.rs`: the module's rendered files read whole
  by `HostConfig`.
- `.github/workflows/ci.yml`: a job builds the flake's package.
- `docs/architecture/core.md`: core.45's details name the module as
  the host's writer of the managed layer, never of the local one
  (record 0055).
- `site/`: the install page and the configuration reference. Also
  `README.md`'s file table and `CLAUDE.md`'s list of projects.
- No change to the statement: C.7 already has the managed layer
  "written by whoever manages the host and never by flume", and a host's
  Nix running flume's module is that writer. No crate's behavior changes.
- After this lands and is released, agentplot/boomdrive takes flume's
  flake at its tag, re-exports its modules, and drops its own build of
  flume, `nix/packages/flume.nix` and `flume-git-crates.nix`. That is
  Boomdrive's change, not this one.

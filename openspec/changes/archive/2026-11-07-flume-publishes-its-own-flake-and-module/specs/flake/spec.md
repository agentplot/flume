## ADDED Requirements

### Requirement: The flake gives flume's package for each system it releases

The repository's root SHALL be a flake whose `packages.<system>` gives
`flume`, and `default` as the same derivation, for aarch64-darwin,
aarch64-linux and x86_64-linux and for no other system. The package
SHALL be built from the flake's own tree, with Rust at the version
`.prototools` names and the crates `Cargo.lock` names. Its version SHALL
be `Cargo.toml`'s `[workspace.package]` version, and its main program
`flume` (record 0055).

#### Scenario: Every system's package evaluates

- **WHEN** `nix:typecheck` evaluates the flake's outputs against the locked inputs
- **THEN** `packages` gives exactly aarch64-darwin, aarch64-linux and x86_64-linux, each holding `flume` and `default`, each a derivation whose path evaluates, and `default` the same derivation as `flume`

#### Scenario: The package's version and toolchain are the tree's

- **WHEN** `nix:test` reads the package of each system
- **THEN** its version is `Cargo.toml`'s `[workspace.package]` version, its Rust is the version `.prototools` names, and its `meta.mainProgram` is `flume`

#### Scenario: The package builds

- **WHEN** CI runs `nix:build` on Linux and on macOS
- **THEN** `nix build .#default` builds flume for that runner's system

### Requirement: Every git crate is fetched by a hash taken at Cargo.lock's revision

`nix/packages/git-crates.nix` SHALL give, for each git package of
`Cargo.lock` by `<name>-<version>`, the revision its hash was taken at
and the hash, and SHALL give nothing else. Each entry's revision SHALL
be the one `Cargo.lock` names for that package.

#### Scenario: A git crate's revision moves without its hash

- **WHEN** `nix:test` runs over a `Cargo.lock` whose weir revision is not the one `git-crates.nix` gives for `weir-kinds-0.0.0`
- **THEN** it fails naming `weir-kinds-0.0.0`, both revisions and that the hash moves with the revision

#### Scenario: A git crate with no entry

- **WHEN** `nix:test` runs over a `Cargo.lock` holding a git package `git-crates.nix` does not name
- **THEN** it fails naming the package

### Requirement: A taking flake can follow the flake's inputs

The flake SHALL name its nixpkgs input `nixpkgs` and its Rust toolchain's
input `rust-overlay`, with `rust-overlay`'s and `nix-darwin`'s `nixpkgs`
following the flake's. It SHALL make each package with its own
`nixpkgs` and `rust-overlay`, so a flake that makes them follow its own
gets the derivations its own inputs give. It SHALL carry no `nixConfig`.

#### Scenario: The inputs a taker follows

- **WHEN** `nix:test` reads `flake.nix`'s inputs and `flake.lock`
- **THEN** the inputs are exactly `nixpkgs`, `rust-overlay` and `nix-darwin`, `rust-overlay` and `nix-darwin` each following `nixpkgs`, and the flake has no `nixConfig`

### Requirement: The flake gives a module for each platform

The flake SHALL give `darwinModules.flume` and `nixosModules.flume`,
with `default` as the same module on each, and its outputs SHALL be
`packages`, `darwinModules` and `nixosModules` alone. Each module SHALL
carry a key that names it, so a configuration that imports it twice,
directly and through another flake's module, declares `services.flume`
once.

#### Scenario: The flake's outputs

- **WHEN** `nix:test` reads the flake's outputs
- **THEN** they are exactly `packages`, `darwinModules` and `nixosModules`, and each module set holds exactly `flume` and `default`, `default` the same module as `flume`

#### Scenario: A module imported twice

- **WHEN** `nix:test` evaluates a nix-darwin system, and a NixOS system, each importing the flake's module directly and again inside another module's `imports`, with `services.flume.enable` set
- **THEN** each evaluates, and `services.flume` is declared once

# flume, built from this tree (the change
# flume-publishes-its-own-flake-and-module, design 2), as Boomdrive's
# nix/packages/flume.nix builds it: Rust at the version .prototools names,
# from rust-overlay, so the package and the gate compile with one toolchain
# (core.22); the crates by Cargo.lock's own checksums, and its git crates by
# the hashes git-crates.nix holds beside their revisions; and the version
# Cargo.toml's [workspace.package] carries, 0.0.0 on main and the release's
# at its tag. The source is the whole tree, since the binary's crates read
# files under plugins/claude/ and .config/ too. The gate runs flume's
# tests, which need git, herdr and sluice, so the build runs none.
{
  lib,
  makeRustPlatform,
  rust-bin,
  src,
  platforms,
}:
let
  read = file: builtins.fromTOML (builtins.readFile "${src}/${file}");
  rust = rust-bin.stable.${(read ".prototools").rust}.minimal;
  rustPlatform = makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };
in
rustPlatform.buildRustPackage {
  pname = "flume";
  inherit ((read "Cargo.toml").workspace.package) version;
  inherit src;
  cargoLock = {
    lockFile = "${src}/Cargo.lock";
    outputHashes = lib.mapAttrs (_: crate: crate.hash) (import ./git-crates.nix);
  };
  cargoBuildFlags = [
    "-p"
    "flume"
    "--bin"
    "flume"
  ];
  doCheck = false;
  meta = {
    description = "Places and their agents";
    homepage = "https://github.com/agentplot/flume";
    license = lib.licenses.mit;
    mainProgram = "flume";
    inherit platforms;
  };
}

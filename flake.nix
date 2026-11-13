# flume's own flake (record 0055): its package for each system it
# releases, built from this tree, and services.flume, a module for
# nix-darwin and NixOS that writes flume's whole managed file. A flake that
# takes it, Boomdrive's among them, can make its nixpkgs and rust-overlay
# follow its own, so the package is the derivation its own inputs give.
# The development shell is devenv's (devenv.nix); everything else of the
# flake's is under nix/, the moon project `nix`.
{
  description = "flume: places and their agents";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # The tests' nix-darwin. A host's configuration uses its own, which
    # changes no package.
    nix-darwin = {
      url = "github:nix-darwin/nix-darwin";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      # The systems flume releases.
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];

      # One system's packages, made with the flake's own nixpkgs and
      # rust-overlay (the change flume-publishes-its-own-flake-and-module,
      # design 2).
      packagesOf =
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          flume = pkgs.callPackage ./nix/packages/flume.nix {
            src = self;
            platforms = systems;
          };
        in
        {
          inherit flume;
          default = flume;
        };
      packages = lib.genAttrs systems packagesOf;

      # A module of a platform, keyed so a configuration that imports it
      # twice, directly and through another flake's module, declares
      # services.flume once (design 4).
      moduleOf = platform: file: {
        _file = file;
        key = "github:boomdrivedev/flume#${platform}Modules.flume";
        imports = [ (import file { inherit packages; }) ];
      };
    in
    {
      inherit packages;

      # services.flume, flume's managed file written whole from settings
      # named as the file names them (record 0055).
      darwinModules = rec {
        flume = moduleOf "darwin" ./nix/modules/darwin.nix;
        default = flume;
      };
      nixosModules = rec {
        flume = moduleOf "nixos" ./nix/modules/nixos.nix;
        default = flume;
      };
    };
}

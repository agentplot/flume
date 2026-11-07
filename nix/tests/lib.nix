# What every case reads: flake.nix imported as an attribute set, and its
# outputs called with the lock's inputs and `self` as this tree, as
# flake-compat calls them (the change
# flume-publishes-its-own-flake-and-module, design 11, as Boomdrive's
# nix/tests/lib.nix).
{ paths }:
let
  inherit ((import ../inputs.nix { inherit paths; })) inputs;
  flake = import ../../flake.nix;
  lock = builtins.fromJSON (builtins.readFile ../../flake.lock);
  outputs = flake.outputs (inputs // { inherit self; });
  # The tree by its path, never copied: the checks write nothing.
  self = outputs // {
    outPath = toString ../..;
    inherit inputs outputs;
    _type = "flake";
  };
in
{
  inherit
    flake
    inputs
    lock
    outputs
    self
    ;
  inherit (inputs.nixpkgs) lib;

  # The systems flume releases, which the flake gives.
  systems = [
    "aarch64-darwin"
    "aarch64-linux"
    "x86_64-linux"
  ];

  # A failure where `holds` is false, as a list of what fails.
  expect = holds: what: if holds then [ ] else [ what ];

  # A nix-darwin system of aarch64-darwin with the flake's module enabled
  # for the primary user alice, at /Users/alice, and the module `given`.
  darwinSystem =
    given:
    inputs.nix-darwin.lib.darwinSystem {
      modules = [
        outputs.darwinModules.default
        {
          nixpkgs.hostPlatform = "aarch64-darwin";
          system.primaryUser = "alice";
          users.users.alice.home = "/Users/alice";
          system.stateVersion = 6;
          services.flume.enable = true;
        }
        given
      ];
    };

  # The messages of the assertions a system fails.
  failures =
    evaluated: map (a: a.message) (builtins.filter (a: !a.assertion) evaluated.config.assertions);
}

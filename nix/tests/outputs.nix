# The flake spec's outputs (the change
# flume-publishes-its-own-flake-and-module, design 4 and 5): flake.nix's
# outputs called with the lock's inputs are packages, darwinModules and
# nixosModules alone, each module set flume and default, default the same
# module as flume, and neither module declares a service of flume's.
t:
let
  inherit (t) lib outputs;
  given = builtins.attrNames outputs;
  modules = [
    "darwinModules"
    "nixosModules"
  ];

  # A module set's failures: flume and default alone, default flume's,
  # each by the key that names it.
  setOf =
    name:
    let
      set = outputs.${name};
      key = "github:agentplot/flume#${name}.flume";
    in
    t.expect (
      builtins.attrNames set == [
        "default"
        "flume"
      ]
    ) "${name} gives ${toString (builtins.attrNames set)}, not default and flume"
    ++
      t.expect ((set.flume.key or null) == key)
        "${name}.flume's key is ${toString (set.flume.key or null)}, not ${key}"
    ++ t.expect (
      (set.default.key or null) == key && (set.default._file or null) == (set.flume._file or null)
    ) "${name}.default is not ${name}.flume";

  darwin = t.darwinSystem { };
  nixos = t.nixosSystem { };
  named = names: builtins.filter (lib.hasInfix "flume") names;
  jobs =
    with darwin.config.launchd;
    builtins.attrNames user.agents ++ builtins.attrNames agents ++ builtins.attrNames daemons;
  units =
    with nixos.config.systemd;
    builtins.attrNames services
    ++ builtins.attrNames user.services
    ++ builtins.attrNames timers
    ++ builtins.attrNames user.timers;
in
{
  "The flake's outputs" =
    t.expect (
      given == [
        "darwinModules"
        "nixosModules"
        "packages"
      ]
    ) "the outputs are ${toString given}, not packages, darwinModules and nixosModules"
    ++ lib.concatMap setOf (builtins.filter (name: outputs ? ${name}) modules);

  "No service of flume's" =
    t.expect (
      named jobs == [ ]
    ) "the nix-darwin module declares the launchd jobs ${toString (named jobs)}"
    ++ t.expect (
      named units == [ ]
    ) "the NixOS module declares the systemd units ${toString (named units)}";
}

# nix:typecheck: every package of every system evaluates to a derivation,
# `default` is `flume`, and each module evaluates in a minimal system of
# its platform (the flake spec; the change
# flume-publishes-its-own-flake-and-module, design 11).
t:
let
  inherit (t) lib outputs systems;

  # A package's failures: none where it is a derivation whose path
  # evaluates.
  evaluates =
    system: name: package:
    let
      drvPath = builtins.tryEval (builtins.seq package.drvPath true);
    in
    if !lib.isDerivation package then
      [ "packages.${system}.${name} is not a derivation" ]
    else
      t.expect drvPath.success "packages.${system}.${name} does not evaluate to a derivation";

  # A system's packages: flume and default, each evaluating, default
  # flume's.
  givesSet =
    system:
    let
      packages = outputs.packages.${system};
      given = builtins.attrNames packages;
      set = [
        "default"
        "flume"
      ];
      evaluated = lib.concatLists (lib.mapAttrsToList (evaluates system) packages);
    in
    t.expect (given == set) "packages.${system} gives ${toString given}, not ${toString set}"
    ++ evaluated
    ++ lib.optionals (evaluated == [ ] && packages ? default && packages ? flume) (
      t.expect (
        packages.default.drvPath == packages.flume.drvPath
      ) "packages.${system}.default is not packages.${system}.flume"
    );
in
{
  "Every system's package evaluates" =
    t.expect (
      builtins.attrNames outputs.packages == lib.sort lib.lessThan systems
    ) "packages gives ${toString (builtins.attrNames outputs.packages)}, not ${toString systems}"
    ++ lib.concatMap (
      system: if outputs.packages ? ${system} then givesSet system else [ "packages gives no ${system}" ]
    ) systems;

  # The nix-darwin module in a minimal system of its platform, whose
  # derivation throws on a failed assertion.
  "The nix-darwin module" =
    t.expect (builtins.tryEval (t.darwinSystem { }).system.drvPath).success
      "a nix-darwin system with services.flume enabled does not evaluate";

  # The NixOS module in a minimal system of its platform, whose toplevel
  # throws on a failed assertion.
  "The NixOS module" =
    t.expect (builtins.tryEval (t.nixosSystem { }).config.system.build.toplevel.drvPath).success
      "a NixOS system with services.flume enabled does not evaluate";
}

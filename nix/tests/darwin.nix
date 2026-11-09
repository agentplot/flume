# The host-module spec on nix-darwin (the change
# flume-publishes-its-own-flake-and-module, design 4, 5 and 9): the module
# evaluated in a system of aarch64-darwin whose primary user is alice, at
# /Users/alice, its system profile and its post-activation read, and the
# module imported twice. The post-activation runs the sources step after
# the link, with each source of nix/tests/sources.nix and its file, or
# with none (the change flumes-module-writes-the-hosts-sources, design 7).
t:
let
  inherit (t) lib;
  set = t.outputs.packages.aarch64-darwin;

  base = t.darwinSystem { };
  sourced = t.darwinSystem { services.flume.sources = import ./sources.nix; };
  # NixOS's user activation with no source, beside nix-darwin's.
  nixos =
    builtins.unsafeDiscardStringContext
      (t.nixosSystem { }).config.system.userActivationScripts.script;
  activate = builtins.unsafeDiscardStringContext "${../modules/activate.sh}";
  none = t.darwinSystem { services.flume.package = null; };
  noHome = t.darwinSystem { users.users.alice.home = lib.mkForce null; };
  twice = t.darwinSystem { imports = [ { imports = [ t.outputs.darwinModules.flume ]; } ]; };

  profileOf = evaluated: map (p: p.outPath) evaluated.config.environment.systemPackages;
  flumes =
    evaluated:
    builtins.filter (p: (p.pname or null) == "flume") evaluated.config.environment.systemPackages;

  # Compared as text: a regular expression may not hold a store path.
  activationOf =
    evaluated:
    builtins.unsafeDiscardStringContext evaluated.config.system.activationScripts.postActivation.text;
  # As the primary user, under their home, with the file rendered from
  # the default settings, and then the sources step with `sources`, each
  # source's name and its file rendered from it.
  link =
    sources: evaluated:
    builtins.unsafeDiscardStringContext ''
      sudo --user=alice -- /usr/bin/env HOME=/Users/alice \
        /bin/sh ${../modules/activate.sh} link ${(evaluated.pkgs.writeText "flume-config.toml" (builtins.readFile ./files/empty.toml)).outPath} || true
      sudo --user=alice -- /usr/bin/env HOME=/Users/alice \
        /bin/sh ${../modules/activate.sh} sources${t.sourcesGiven evaluated.pkgs sources} || true
    '';
  runs =
    sources: evaluated:
    t.expect (lib.hasInfix (link sources evaluated) (activationOf evaluated)) "the post-activation runs no ${builtins.toJSON (link sources evaluated)}: ${builtins.toJSON (activationOf evaluated)}";
  links = runs [ ];

  # Every launchd job nix-darwin declares, by its name.
  jobs =
    evaluated:
    let
      inherit (evaluated.config) launchd;
    in
    builtins.attrNames launchd.user.agents
    ++ builtins.attrNames launchd.agents
    ++ builtins.attrNames launchd.daemons;
  says = evaluated: words: lib.hasInfix words (toString (t.failures evaluated));
in
{
  "Enabled on nix-darwin" =
    t.expect (t.failures base == [ ]) "the system fails: ${toString (t.failures base)}"
    ++ t.expect (builtins.elem set.flume.outPath (profileOf base)) "the system profile holds not the flake's aarch64-darwin flume"
    ++ links base
    ++ t.expect (builtins.elem "services.flume.enable" base.config.system.requiresPrimaryUser) "system.requiresPrimaryUser names no services.flume.enable"
    ++ t.expect (
      !lib.any (lib.hasInfix "flume") (jobs base)
    ) "a launchd job is declared for flume: ${toString (jobs base)}";

  "Sources on nix-darwin" =
    t.expect (t.failures sourced == [ ]) "the system fails: ${toString (t.failures sourced)}"
    ++ runs [
      "acme"
      "tools"
    ] sourced;

  "No source declared" =
    links base
    ++ t.expect (lib.hasInfix "${activate} sources || true" (lib.last (lib.splitString "${activate} link " nixos))) "NixOS's user activation runs no activate.sh sources with no source after the link: ${builtins.toJSON nixos}";

  "No package on nix-darwin" =
    t.expect (t.failures none == [ ]) "the system fails: ${toString (t.failures none)}"
    ++ t.expect (flumes none == [ ]) "the system profile holds a flume from the module"
    ++ links none;

  "No home for the primary user" =
    t.expect (t.failures noHome != [ ]) "the module evaluates for a primary user with no home"
    ++
      lib.concatMap
        (
          words: t.expect (says noHome words) "the failure names no ${words}: ${toString (t.failures noHome)}"
        )
        [
          "alice"
          "users.users.alice.home"
        ];

  "The module imported twice on nix-darwin" =
    let
      evaluates = (builtins.tryEval twice.system.drvPath).success;
      declared = builtins.length twice.options.services.flume.enable.declarations;
    in
    t.expect evaluates "a system importing the module twice does not evaluate"
    ++ lib.optionals evaluates (
      t.expect (declared == 1) "services.flume.enable is declared ${toString declared} times"
    );
}

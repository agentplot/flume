# The host-module spec on NixOS (the change
# flume-publishes-its-own-flake-and-module, design 4, 5 and 9): the module
# evaluated in a system of x86_64-linux whose normal users are alice, the
# module's, and bob, its system profile and its user activation read, and
# the module imported twice.
t:
let
  inherit (t) lib;
  set = t.outputs.packages.x86_64-linux;

  base = t.nixosSystem { };
  none = t.nixosSystem { services.flume.package = null; };
  noUser = t.nixosSystem { services.flume.user = lib.mkForce "carol"; };
  twice = t.nixosSystem { imports = [ { imports = [ t.outputs.nixosModules.flume ]; } ]; };

  profileOf = evaluated: map (p: p.outPath) evaluated.config.environment.systemPackages;
  flumes =
    evaluated:
    builtins.filter (p: (p.pname or null) == "flume") evaluated.config.environment.systemPackages;

  # Compared as text: a regular expression may not hold a store path.
  activationOf =
    evaluated: builtins.unsafeDiscardStringContext evaluated.config.system.userActivationScripts.script;
  # alice's alone: the link with the file rendered from the default
  # settings, and nothing for any other user.
  link =
    evaluated:
    builtins.unsafeDiscardStringContext ''
      if [ "$(id -un)" = alice ]; then
        ${evaluated.pkgs.runtimeShell} ${../modules/activate.sh} link ${(evaluated.pkgs.writeText "flume-config.toml" (builtins.readFile ./files/empty.toml)).outPath} || true
      fi
    '';
  links =
    evaluated:
    t.expect (lib.hasInfix (link evaluated) (activationOf evaluated)) "the user activation runs no ${builtins.toJSON (link evaluated)}: ${builtins.toJSON (activationOf evaluated)}";

  # Every systemd unit NixOS declares, the system's and the users', by its
  # name.
  units =
    evaluated:
    let
      inherit (evaluated.config) systemd;
    in
    builtins.attrNames systemd.services
    ++ builtins.attrNames systemd.user.services
    ++ builtins.attrNames systemd.timers
    ++ builtins.attrNames systemd.user.timers;
  says = evaluated: words: lib.hasInfix words (toString (t.failures evaluated));
in
{
  "Enabled on NixOS" =
    t.expect (t.failures base == [ ]) "the system fails: ${toString (t.failures base)}"
    ++ t.expect (builtins.elem set.flume.outPath (profileOf base)) "the system profile holds not the flake's x86_64-linux flume"
    ++ links base
    ++
      t.expect (!lib.any (lib.hasInfix "flume") (units base))
        "a systemd unit is declared for flume: ${toString (builtins.filter (lib.hasInfix "flume") (units base))}";

  "No package on NixOS" =
    t.expect (t.failures none == [ ]) "the system fails: ${toString (t.failures none)}"
    ++ t.expect (flumes none == [ ]) "the system profile holds a flume from the module"
    ++ links none;

  "No such user on NixOS" =
    t.expect (t.failures noUser != [ ]) "the module evaluates for a user users.users does not declare"
    ++
      lib.concatMap
        (
          words: t.expect (says noUser words) "the failure names no ${words}: ${toString (t.failures noUser)}"
        )
        [
          "services.flume.user"
          "users.users.carol"
        ];

  "The module imported twice on NixOS" =
    let
      evaluates = (builtins.tryEval twice.config.system.build.toplevel.drvPath).success;
      declared = builtins.length twice.options.services.flume.enable.declarations;
    in
    t.expect evaluates "a system importing the module twice does not evaluate"
    ++ lib.optionals evaluates (
      t.expect (declared == 1) "services.flume.enable is declared ${toString declared} times"
    );
}

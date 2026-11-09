# The nix-darwin module, darwinModules.flume (the change
# flume-publishes-its-own-flake-and-module, design 5, 9 and 10): flume in
# the system profile, and its managed file, rendered from `settings` at
# evaluation (design 7), linked at ~/.config/flume/config.toml for
# nix-darwin's primary user, since nix-darwin activates as root; after it,
# each source `sources` declares, rendered the same way, linked at
# ~/.config/flume/sources/<source>.toml, and the module's link of a source
# no longer declared removed (the change
# flumes-module-writes-the-hosts-sources, design 5 and 7). flume runs no
# service (core.13), so the module declares no launchd agent: the next
# command reads the new files. The activation runs each step as the
# primary user under their own home, as Boomdrive's does, and a step that
# fails says why and stops nothing else.
{ packages }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.flume;
  toml = import ./toml.nix { inherit lib; };
  file = pkgs.writeText "flume-config.toml" (toml.text cfg.settings);
  # Each source the host declares, by the order of the sources' names, and
  # its file, rendered under its own option and named as the module's in
  # the store (the change flumes-module-writes-the-hosts-sources, design 3
  # and 5).
  sourceFile =
    name:
    pkgs.writeText "flume-source-${name}.toml" (
      toml.render "services.flume.sources.${name}" cfg.sources.${name}
    );
  sourceArgs = lib.concatMapStrings (
    name: " ${lib.escapeShellArg name} ${lib.escapeShellArg (sourceFile name)}"
  ) (builtins.attrNames cfg.sources);

  user = config.system.primaryUser;
  home = if user == null then null else config.users.users.${user}.home or null;
in
{
  imports = [ (import ./options.nix { inherit packages; }) ];

  config = lib.mkIf cfg.enable {
    system.requiresPrimaryUser = [ "services.flume.enable" ];

    assertions = [
      {
        assertion = user == null || home != null;
        message = "services.flume writes the managed file of nix-darwin's primary user ${toString user}, whose home users.users.${toString user}.home is not set: set it, the home whose ~/.config/flume/config.toml the module links.";
      }
    ];

    environment.systemPackages = lib.optional (cfg.package != null) cfg.package;

    system.activationScripts.postActivation.text = ''
      sudo --user=${lib.escapeShellArg user} -- /usr/bin/env HOME=${lib.escapeShellArg home} \
        /bin/sh ${./activate.sh} link ${file} || true
      sudo --user=${lib.escapeShellArg user} -- /usr/bin/env HOME=${lib.escapeShellArg home} \
        /bin/sh ${./activate.sh} sources${sourceArgs} || true
    '';
  };
}

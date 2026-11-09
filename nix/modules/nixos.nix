# The NixOS module, nixosModules.flume (the change
# flume-publishes-its-own-flake-and-module, design 5, 9 and 10): flume in
# the system profile, and its managed file, rendered from `settings` at
# evaluation (design 7), linked at ~/.config/flume/config.toml for
# services.flume.user by that user's own activation; after it, each source
# `sources` declares, rendered the same way, linked at
# ~/.config/flume/sources/<source>.toml, and the module's link of a source
# no longer declared removed (the change
# flumes-module-writes-the-hosts-sources, design 5 and 7); any other
# user's activation does nothing. flume runs no service (core.13), so the
# module declares no systemd unit: the next command reads the new files. A
# step that fails says why and stops nothing else.
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

  inherit (cfg) user;
  account = config.users.users.${user} or null;
in
{
  imports = [ (import ./options.nix { inherit packages; }) ];

  options.services.flume.user = lib.mkOption {
    type = lib.types.str;
    example = "alice";
    description = "The user whose ~/.config/flume/config.toml the module links, a normal or a system user users.users declares.";
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = account != null && (account.isNormalUser || account.isSystemUser);
        message = "services.flume.user is ${user}, whom users.users.${user} does not declare: declare the user whose ~/.config/flume/config.toml the module links, or name one users.users declares.";
      }
    ];

    environment.systemPackages = lib.optional (cfg.package != null) cfg.package;

    # Run by every user's activation, as that user.
    system.userActivationScripts.flume.text = ''
      if [ "$(id -un)" = ${lib.escapeShellArg user} ]; then
        ${pkgs.runtimeShell} ${./activate.sh} link ${file} || true
        ${pkgs.runtimeShell} ${./activate.sh} sources${sourceArgs} || true
      fi
    '';
  };
}

# Evaluations that fail, each read by its words in nix/tests/scripts.sh,
# since a failure's message cannot be read by the evaluation it ends: a
# mistyped declared key, and a path where the file takes a string (the
# host-module spec).
let
  t = import ./lib.nix { paths = builtins.fromJSON (builtins.getEnv "FLUME_FLAKE_INPUTS"); };
  inherit (t) lib;
  toml = import ../modules/toml.nix { inherit lib; };
  pkgs = t.inputs.nixpkgs.legacyPackages.x86_64-linux;
  text =
    settings:
    toml.text
      (lib.evalModules {
        modules = [
          (import ../modules/options.nix { inherit (t.outputs) packages; })
          {
            options.assertions = lib.mkOption {
              type = lib.types.listOf lib.types.unspecified;
              default = [ ];
            };
          }
          {
            _module.args = { inherit pkgs; };
            services.flume = {
              enable = true;
              inherit settings;
            };
          }
        ];
      }).config.services.flume.settings;
in
{
  mistyped = text { streams.work.agents = "five"; };
  path = text { kinds.change.agents.programs.claude.file = ./every-key.nix; };
}

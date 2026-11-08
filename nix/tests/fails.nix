# Evaluations that fail, each read by its words in nix/tests/scripts.sh,
# since a failure's message cannot be read by the evaluation it ends: a
# mistyped declared key, a path where the file takes a string (the
# host-module spec), and the package of a system the flake gives none for
# (the change flume-publishes-its-own-flake-and-module, design 5).
let
  t = import ./lib.nix { paths = builtins.fromJSON (builtins.getEnv "FLUME_FLAKE_INPUTS"); };
  inherit (t) lib;
  toml = import ../modules/toml.nix { inherit lib; };
  linux = t.inputs.nixpkgs.legacyPackages.x86_64-linux;

  # options.nix enabled with `settings` under `pkgs`.
  evaluated =
    pkgs: settings:
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
    }).config.services.flume;
  text = settings: toml.text (evaluated linux settings).settings;

  # x86_64-linux's packages, under a system the flake gives none for.
  elsewhere = linux // {
    stdenv = linux.stdenv // {
      hostPlatform = linux.stdenv.hostPlatform // {
        system = "riscv64-linux";
      };
    };
  };
in
{
  mistyped = text { streams.work.agents = "five"; };
  path = text { kinds.change.agents.programs.scribe.file = ./every-key.nix; };
  unknownSystem = (evaluated elsewhere { }).package.outPath;
}

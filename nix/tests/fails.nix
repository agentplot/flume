# Evaluations that fail, each read by its words in nix/tests/scripts.sh,
# since a failure's message cannot be read by the evaluation it ends: a
# mistyped declared key, a telemetry collector off the host's loopback, a
# path where the file takes a string (the host-module spec), and the
# package of a system the flake gives none for (the change
# flume-publishes-its-own-flake-and-module, design 5); and a
# source with no `kinds_dir`, one whose `kinds_dir` is under no home and
# one with a path value, each rendering its file (the change
# flumes-module-writes-the-hosts-sources, design 1 and 3).
let
  t = import ./lib.nix { paths = builtins.fromJSON (builtins.getEnv "FLUME_FLAKE_INPUTS"); };
  inherit (t) lib;
  toml = import ../modules/toml.nix { inherit lib; };
  linux = t.inputs.nixpkgs.legacyPackages.x86_64-linux;

  # options.nix enabled with `settings` and `sources` under `pkgs`.
  evaluate =
    pkgs: settings: sources:
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
            inherit settings sources;
          };
        }
      ];
    }).config.services.flume;
  evaluated = pkgs: settings: evaluate pkgs settings { };
  text = settings: toml.text (evaluated linux settings).settings;
  # The file of the source acme, given `acme`.
  sourceText =
    acme: toml.render "services.flume.sources.acme" (evaluate linux { } { inherit acme; }).sources.acme;

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
  collectorOffTheLoopback = text { telemetry.collector = "http://otel.example:4318"; };
  path = text { kinds.change.agents.programs.scribe.file = ./every-key.nix; };
  unknownSystem = (evaluated elsewhere { }).package.outPath;
  noKindsDir = sourceText { roles = "~/roles.toml"; };
  kindsDirUnderNoHome = sourceText { kinds_dir = "kinds/acme"; };
  sourcePath = sourceText {
    kinds_dir = "/opt/kinds/acme";
    shelf = ./every-key.nix;
  };
}

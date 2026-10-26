{ pkgs, ... }:
let
  # treefmt's rustfmt: the toolchain .prototools pins, which proto installs
  # through rustup on entering the shell, so the hook and the crates' format
  # tasks run one rustfmt (core.21, core.22).
  rustfmt = pkgs.writeShellApplication {
    name = "rustfmt";
    runtimeInputs = [
      pkgs.git
      pkgs.gnused
      pkgs.rustup
    ];
    text = ''
      root="$(git rev-parse --show-toplevel)"
      version="$(sed -n 's/^rust = "\(.*\)"$/\1/p' "$root/.prototools")"
      exec rustup run "$version" rustfmt "$@"
    '';
  };
in
{
  settings.excludes = [
    "*.md"
    "*.mdx"
    "*.lock"
    "*.json"
    ".devenv/**"
    ".moon/cache/**"
    "site/**"
    # cargo's build directory: what it writes, no source.
    "target/**"
  ];

  programs.rustfmt.enable = true;
  programs.rustfmt.edition = "2021";
  programs.rustfmt.package = rustfmt;

  programs.deadnix.enable = true;
  programs.nixfmt.enable = true;
  settings.formatter.deadnix.pipeline = "nix";
  settings.formatter.deadnix.priority = 1;
  settings.formatter.nixfmt.pipeline = "nix";
  settings.formatter.nixfmt.priority = 2;

  programs.taplo.enable = true;
  programs.yamlfmt.enable = true;
  programs.yamlfmt.settings.formatter = {
    type = "basic";
    retain_line_breaks_single = true;
  };
}

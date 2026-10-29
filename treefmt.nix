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
  # A tracked file no formatter claims and no exclude names fails the check
  # (core.21): a new file type is decided in the change that adds it.
  settings.on-unmatched = "fatal";

  # What no formatter owns, each with its reason (core.21).
  settings.excludes = [
    # Prose: the site's pages and the README are Vale's (site:prose), and
    # no formatter here rewraps prose.
    "*.md"
    "*.mdx"
    # Written by cargo, devenv and bun in their own layouts.
    "*.lock"
    # Drawn images and rendered diagrams; no formatter keeps them stable.
    "*.svg"
    # Vale's own configuration and word list, which no formatter reads.
    ".vale.ini"
    "styles/config/vocabularies/**"
    # Lists in gitignore syntax.
    ".gitignore"
    ".worktreeinclude"
    # The licence's text, kept as published.
    "LICENSE"
    # The site's components: Biome formats only an .astro file's
    # frontmatter, and only experimentally.
    "site/**/*.astro"
    # Claude Code's transcripts and screens, kept as the pinned release
    # wrote them for the agent profile's and the plugin's tests; a
    # formatter would rewrite what the readings are checked against.
    "crates/flume/tests/profiles/fixtures/**"
    "plugins/claude/tests/fixtures/**"
    # Build outputs and caches.
    "target/**"
    ".devenv/**"
    ".moon/cache/**"
    "site/node_modules/**"
    "site/dist/**"
    "site/.astro/**"
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

  # shellcheck lints the shell scripts, then shfmt lays them out.
  programs.shellcheck.enable = true;
  programs.shfmt.enable = true;
  programs.shfmt.indent_size = 2;
  settings.formatter.shellcheck.pipeline = "shell";
  settings.formatter.shellcheck.priority = 1;
  settings.formatter.shfmt.pipeline = "shell";
  settings.formatter.shfmt.priority = 2;
  # The herdr plugin's stand-ins: #!/bin/sh scripts with no extension.
  settings.formatter.shellcheck.includes = [ "plugins/herdr/tests/fake/*" ];
  settings.formatter.shfmt.includes = [ "plugins/herdr/tests/fake/*" ];

  # .prototools is TOML and .versionrc convco's YAML, under names with no
  # extension.
  programs.taplo.enable = true;
  settings.formatter.taplo.includes = [ ".prototools" ];
  programs.yamlfmt.enable = true;
  settings.formatter.yamlfmt.includes = [ ".versionrc" ];
  programs.yamlfmt.settings.formatter = {
    type = "basic";
    retain_line_breaks_single = true;
  };

  # Biome formats and lints the TypeScript, JavaScript, CSS and JSON in
  # one pass (`biome check`), its recommended rules and nothing more; a
  # lint error fails `treefmt --ci`. Its settings are here, not in a
  # biome.json, since treefmt-nix hands Biome a generated one.
  programs.biome.enable = true;
  programs.biome.formatCommand = "check";
  programs.biome.settings = {
    formatter = {
      indentStyle = "space";
      indentWidth = 2;
      lineWidth = 100;
    };
    javascript.formatter = {
      quoteStyle = "double";
      trailingCommas = "all";
      semicolons = "always";
    };
    json.formatter.trailingCommas = "none";
    linter.rules.recommended = true;
  };
  # actionlint reads the workflows' keys and expressions, which yamlfmt
  # only lays out.
  programs.actionlint.enable = true;
}

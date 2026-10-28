{
  pkgs,
  lib,
  config,
  ...
}:

let
  # proto from its own release build: nixpkgs trails it by minor versions or
  # more. Linux takes the static musl build, so no loader needs patching.
  proto =
    let
      version = "0.63.1";
      system = pkgs.stdenv.hostPlatform.system;
      target =
        {
          aarch64-darwin = "aarch64-apple-darwin";
          x86_64-darwin = "x86_64-apple-darwin";
          aarch64-linux = "aarch64-unknown-linux-musl";
          x86_64-linux = "x86_64-unknown-linux-musl";
        }
        .${system} or (throw "proto ${version}: no release build for ${system}");
      hashes = {
        aarch64-apple-darwin = "8bfde4c862381eeb6e82d0858bb6215732cbd1e384e4eae45da8fa08095e0ffd";
        x86_64-apple-darwin = "aaaff78c7125de1b765f00177f35da75ee16fb1363e07632ace27213ff5d893b";
        aarch64-unknown-linux-musl = "111f5b6a38d3979ebafbc095c8d91ef69c99d1f018addb3582179d4ab91b3441";
        x86_64-unknown-linux-musl = "8596b98ccd833d1203e71931ba25e866d3239f9d3859c922a035a7b830f29d17";
      };
    in
    pkgs.stdenvNoCC.mkDerivation {
      pname = "proto";
      inherit version;
      src = pkgs.fetchurl {
        url = "https://github.com/moonrepo/proto/releases/download/v${version}/proto_cli-${target}.tar.xz";
        sha256 = hashes.${target};
      };
      sourceRoot = "proto_cli-${target}";
      installPhase = ''
        runHook preInstall
        install -Dm755 -t $out/bin proto proto-shim
        runHook postInstall
      '';
    };

  # The application toolchain .prototools pins (moon), on PATH: what the
  # shell activates, and what a git hook, which runs outside any shell a
  # session opened, activates for itself. proto's home is this repository's,
  # in the git common dir so every worktree shares it, and only this
  # repository's pins are read.
  protoHome = ''
    PROTO_HOME="$(${lib.getExe pkgs.git} rev-parse --path-format=absolute --git-common-dir)/proto"
    export PROTO_HOME
  '';
  activate = ''
    ${protoHome}
    eval "$(${proto}/bin/proto activate bash --export --no-bin --config-mode upwards 2>/dev/null)"
  '';

  # The lint gate, which the clippy-gate hook runs: every crate, every
  # target, a warning failing it.
  lint = pkgs.writeShellApplication {
    name = "lint";
    runtimeInputs = [ pkgs.git ];
    text = ''
      cd "$(git rev-parse --show-toplevel)"
      ${activate}
      cargo clippy --workspace --all-targets -- -D warnings
    '';
  };

  # target/ grows unseen across worktrees until the disk is full.
  disk = pkgs.writeShellApplication {
    name = "disk";
    runtimeInputs = [
      pkgs.git
      pkgs.coreutils
      pkgs.gawk
    ];
    text = ''
      root="$(git rev-parse --show-toplevel)"
      limit=''${FLUME_TARGET_LIMIT_GB:-25}
      target=0
      [ -d "$root/target" ] && target=$(($(du -sk "$root/target" | cut -f1) / 1048576))
      used="$(df -P "$root" | awk 'NR == 2 { print $5 }' | tr -d '%')"
      echo "target/ ''${target} GB, the disk ''${used} per cent full"
      status=0
      if [ "$target" -gt "$limit" ]; then
        echo "target/ is ''${target} GB, over the ''${limit} GB bar: run 'cargo clean' here and in the worktrees beside this one" >&2
        status=1
      fi
      if [ "$used" -ge 90 ]; then
        echo "the disk is ''${used} per cent full: a build on a full disk is slow and its processes are killed" >&2
        status=1
      fi
      exit $status
    '';
  };

  # The commit hooks: prek, with the hook config this worktree's devenv
  # shell links, or the main checkout's before it has (git's main worktree,
  # or in a bare repository the worktree `main` beside it), and with
  # neither the commit goes through. devenv 2.4's own install writes the
  # config's absolute path into the hooks directory every worktree shares
  # (cachix/devenv#3270); this is sluice's shim, which stays once devenv's
  # fix lands. root:hooks installs the same shims in a scratch clone
  # ($FLUME_HOOKS).
  hookShim =
    type:
    pkgs.writeScript "prek-${type}" ''
      #!/bin/sh
      # flume's ${type} hook, installed by devenv.nix: prek, with the hook
      # config this worktree's devenv shell links, or the main checkout's
      # before it has.

      HERE="$(cd "$(dirname "$0")" && pwd)"
      PREK="${lib.getExe config.git-hooks.package}"

      # Check if the full path to prek is executable, otherwise fallback to PATH
      if [ ! -x "$PREK" ]; then
          PREK="prek"
      fi

      if [ -e .pre-commit-config.yaml ]; then
          exec "$PREK" hook-impl --hook-dir "$HERE" --script-version 4 --hook-type=${type} -- "$@"
      fi
      common="$(git rev-parse --path-format=absolute --git-common-dir)"
      if [ "$(git config --bool core.bare)" = true ]; then
          main="''${common%/*}/main"
      else
          main="''${common%/*}"
      fi
      exec "$PREK" hook-impl --hook-dir "$HERE" --script-version 4 --hook-type=${type} \
          --config "$main/.pre-commit-config.yaml" --skip-on-missing-config -- "$@"
    '';
  hookTypes = [
    "pre-commit"
    "commit-msg"
  ];
  hookShims = pkgs.runCommand "flume-hooks" { } ''
    mkdir -p $out
    ${lib.concatMapStrings (type: ''
      install -m 755 ${hookShim type} $out/${type}
    '') hookTypes}
  '';

  # Vale checks the site's prose (.vale.ini, styles/Flume). From its own
  # release: nixpkgs trails it. Bump the version and the four hashes from
  # the release's checksums file.
  valeVersion = "3.24.0";
  valeAssets = {
    aarch64-darwin = {
      os = "macOS_arm64";
      sha256 = "87b513f26499f6657c15cf7175ba5b058a7c1ff2bb2d4e07265db2dd43451e44";
    };
    x86_64-darwin = {
      os = "macOS_64-bit";
      sha256 = "eeb39e86f1daac27cc2a77fef3af1824d50559bdc22329fb25fb921c80c79472";
    };
    aarch64-linux = {
      os = "Linux_arm64";
      sha256 = "104cbd349279e50c4cb40adb70adf345d93cc4a3a0191f9e68bdccb97086b582";
    };
    x86_64-linux = {
      os = "Linux_64-bit";
      sha256 = "867534ddb678abca7f214bc4706ead0922bd5252ffe8035fc227013f66c8b214";
    };
  };
  valeAsset = valeAssets.${pkgs.stdenv.hostPlatform.system};
  vale = pkgs.stdenvNoCC.mkDerivation {
    pname = "vale";
    version = valeVersion;
    src = pkgs.fetchurl {
      url = "https://github.com/errata-ai/vale/releases/download/v${valeVersion}/vale_${valeVersion}_${valeAsset.os}.tar.gz";
      inherit (valeAsset) sha256;
    };
    sourceRoot = ".";
    installPhase = "install -Dm755 vale $out/bin/vale";
  };
  # Claude Code, which the agent profile starts and the plugin's checks run:
  # sluice's version, so every host runs one engine, and never the user's
  # own `claude`. Bump the version and the four checksums from the
  # release's manifest.zst.json.
  claudeVersion = "2.1.296";
  claudeAssets = {
    aarch64-darwin = {
      platform = "darwin-arm64";
      sha256 = "26426aa3dcf41b0f554951766c3c583cf243345ca3d32fa590caf84685be5f97";
    };
    x86_64-darwin = {
      platform = "darwin-x64";
      sha256 = "6f1d6f926a303c423eb4815a01764165ed4375054bfa8ef6203e55fc806f50ac";
    };
    aarch64-linux = {
      platform = "linux-arm64";
      sha256 = "23aa91437a06eff80a65bfc32b0e4b5a6dda3e821019c543696c154274e56913";
    };
    x86_64-linux = {
      platform = "linux-x64";
      sha256 = "fac1d78be9446052ac084878c04c2648abb6b22d53279a685fa19ee517d74410";
    };
  };
  claudeAsset = claudeAssets.${pkgs.stdenv.hostPlatform.system};
  claude = pkgs.stdenvNoCC.mkDerivation {
    pname = "claude-code";
    version = claudeVersion;
    src = pkgs.fetchurl {
      url = "https://downloads.claude.ai/claude-code-releases/${claudeVersion}/${claudeAsset.platform}/claude.zst";
      inherit (claudeAsset) sha256;
    };
    dontUnpack = true;
    # The binary carries its program after the runtime; stripping drops it.
    dontStrip = true;
    nativeBuildInputs = [
      pkgs.zstd
      pkgs.makeBinaryWrapper
    ]
    ++ lib.optionals pkgs.stdenv.hostPlatform.isElf [ pkgs.autoPatchelfHook ];
    installPhase = ''
      mkdir -p $out/bin
      unzstd -q $src -o $out/bin/.claude
      chmod 755 $out/bin/.claude
      makeWrapper $out/bin/.claude $out/bin/claude \
        --set DISABLE_AUTOUPDATER 1 \
        --set DISABLE_INSTALLATION_CHECKS 1
    '';
  };

  # OpenSpec, which root:specs and root:changes validate with and the
  # change profile reads other schemas through: sluice's release, built as
  # nixpkgs builds its older one. Bump the version, then each hash from
  # the mismatch the build reports.
  openspec = pkgs.openspec.overrideAttrs (
    finalAttrs: _: {
      version = "1.14.3";
      src = pkgs.fetchFromGitHub {
        owner = "Fission-AI";
        repo = "OpenSpec";
        tag = "v${finalAttrs.version}";
        hash = "sha256-tdTX0nkDe/aNZgLckh6/B6BtoSb40w0Y5qzg+d1LjHU=";
      };
      pnpmDeps = pkgs.fetchPnpmDeps {
        inherit (finalAttrs) pname version src;
        pnpm = pkgs.pnpm_10;
        fetcherVersion = 3;
        hash = "sha256-fKm4sBdTDeQfIiE0k301jtRbiuJrBE7YIEBu8RZxuXI=";
      };
    }
  );

  # herdr, which the terminal profile drives: the release its commands are
  # checked against. One binary per platform; bump the version and the
  # four checksums from the release's assets.
  herdrVersion = "0.9.5";
  herdrAssets = {
    aarch64-darwin = {
      platform = "macos-aarch64";
      sha256 = "fb77c2232a05a8035421382c894392f4174e4e38a18e48af3dd0290157c3a7e2";
    };
    x86_64-darwin = {
      platform = "macos-x86_64";
      sha256 = "5fb0a908ec7f6745e3e245738b06bfad56c46391c391ab3c0955fbead01f1369";
    };
    aarch64-linux = {
      platform = "linux-aarch64";
      sha256 = "17d7536f2da50859523680ef279dffcac8e4a235f4e46fe8da6c2ec44141897c";
    };
    x86_64-linux = {
      platform = "linux-x86_64";
      sha256 = "14d9e8bc8b4a9e0118d77c32266f76d461f09b40342eb23752960bd89f59929d";
    };
  };
  herdrAsset = herdrAssets.${pkgs.stdenv.hostPlatform.system};
  herdr = pkgs.stdenvNoCC.mkDerivation {
    pname = "herdr";
    version = herdrVersion;
    src = pkgs.fetchurl {
      url = "https://github.com/ogulcancelik/herdr/releases/download/v${herdrVersion}/herdr-${herdrAsset.platform}";
      inherit (herdrAsset) sha256;
    };
    dontUnpack = true;
    nativeBuildInputs = lib.optionals pkgs.stdenv.hostPlatform.isElf [ pkgs.autoPatchelfHook ];
    installPhase = "install -Dm755 $src $out/bin/herdr";
  };

  # sluice, which the merge-queue profile drives: its newest release. sluice
  # publishes no x86_64-darwin build. Its builds are Nix's: on Linux the
  # loader, on macOS libiconv, are store paths of the machine that built
  # them, set here to this tree's. Bump the version and the three checksums
  # from the release's assets.
  sluiceVersion = "0.54.0";
  sluiceHashes = {
    aarch64-darwin = "43839d0f960756ac7aa2046961bca1a896fef94eec701e5a3ca492656dac7dac";
    aarch64-linux = "d80058cc42768d59aa44ac88c11481ff0c828f1f15214788bcaaf2ecc6b8ac6c";
    x86_64-linux = "161d4e6e0cf6662754b6a2665a215584887dffdf0aa44884393e1fe173dd74e7";
  };
  sluiceSystem = pkgs.stdenv.hostPlatform.system;
  sluice = pkgs.stdenv.mkDerivation {
    pname = "sluice";
    version = sluiceVersion;
    src = pkgs.fetchurl {
      url = "https://github.com/agentplot/sluice/releases/download/v${sluiceVersion}/sluice-${sluiceSystem}.tar.gz";
      sha256 =
        sluiceHashes.${sluiceSystem}
          or (throw "sluice ${sluiceVersion}: no release build for ${sluiceSystem}");
    };
    sourceRoot = ".";
    nativeBuildInputs = lib.optionals pkgs.stdenv.hostPlatform.isElf [ pkgs.autoPatchelfHook ];
    buildInputs = lib.optionals pkgs.stdenv.hostPlatform.isDarwin [ pkgs.libiconv ];
    installPhase = ''
      runHook preInstall
      install -Dm755 -t $out/bin sluice wt-submit
      runHook postInstall
    '';
    postFixup = lib.optionalString pkgs.stdenv.hostPlatform.isDarwin ''
      for bin in $out/bin/*; do
        for lib in $(otool -L "$bin" | awk '/\/nix\/store\/.*libiconv/ { print $1 }'); do
          install_name_tool -change "$lib" ${pkgs.libiconv}/lib/libiconv.2.dylib "$bin"
        done
      done
    '';
  };

in
{
  # The system tools are this file's, pinned by devenv.lock; moon and Rust
  # are .prototools', installed by the flume:proto:install task on entering
  # the shell and activated by enterShell. Claude Code, OpenSpec, herdr and
  # sluice are the releases pinned above, never the host's (core.22). The
  # site's tools are bun, and Playwright with the one browser it drives,
  # from Nix, for the site's screenshots (site/scripts/shots.mjs).
  packages = [
    pkgs.git
    pkgs.jq
    proto
    vale
    claude
    openspec
    herdr
    sluice
    pkgs.bun
    pkgs.playwright-test
  ];

  # root:hooks drives a commit through the hooks devenv installs: the
  # shims, and the hook config the shell links into a worktree.
  env.FLUME_HOOKS = "${hookShims}";
  env.FLUME_HOOKS_CONFIG = "${config.git-hooks.configFile}";

  env.PLAYWRIGHT_BROWSERS_PATH = "${pkgs.playwright-driver.browsers-chromium}";

  # moon's task archives (.moon/cache/outputs) are a link into the git common
  # dir, so every worktree shares them. The rest of .moon/cache records the
  # checkout's absolute paths and stays its own.
  enterShell = ''
    if _moon_root="$(git rev-parse --show-toplevel 2>/dev/null)" &&
      _moon_common="$(git rev-parse --path-format=absolute --git-common-dir 2>/dev/null)"; then
      _moon_cache="$_moon_root/.moon/cache"
      [ -L "$_moon_cache" ] && rm -f "$_moon_cache"
      mkdir -p "$_moon_common/moon-outputs" "$_moon_cache"
      if [ ! -L "$_moon_cache/outputs" ]; then
        rm -rf "$_moon_cache/outputs"
        ln -s "$_moon_common/moon-outputs" "$_moon_cache/outputs"
      fi
      # moon puts $MOON_HOME/bin on every task's PATH too: this repository's,
      # beside proto's, so no moon of the user's runs in a task.
      MOON_HOME="$_moon_common/moon-home"
      export MOON_HOME
    fi
    unset _moon_root _moon_common _moon_cache
    # A sluice run is handed its share of the host's cores (SLUICE_JOBS):
    # cargo builds and runs tests with that many, so the runs the host's
    # bound lets go at once share its cores rather than each taking them
    # all. A session's own shell, with no SLUICE_JOBS, keeps every core.
    if [ -n "''${SLUICE_JOBS:-}" ]; then
      CARGO_BUILD_JOBS="$SLUICE_JOBS"
      RUST_TEST_THREADS="$SLUICE_JOBS"
      export CARGO_BUILD_JOBS RUST_TEST_THREADS
    fi
    # In a session's own shell rustc runs under sccache, whose server and
    # cache are the user's, under their home, so the agents' worktrees on
    # one host reuse each other's compiled crates ($RUSTC_WRAPPER
    # --show-stats). A run sluice starts, told by the SLUICE_CHECKOUT it
    # sets, runs rustc unwrapped, and so does CI, told by the CI its
    # runner sets: its cache starts empty, and its gate builds as
    # `sluice exec`'s does (core.22).
    if [ -z "''${SLUICE_CHECKOUT:-}" ] && [ -z "''${CI:-}" ]; then
      RUSTC_WRAPPER="${pkgs.sccache}/bin/sccache"
      export RUSTC_WRAPPER
    fi
    # Only activates: sluice sources this inside its executor, where
    # nothing may be installed.
    ${activate}
  '';

  # The versions .prototools lists, installed on entering the shell, before
  # enterShell activates them. A task, not part of enterShell: the script
  # `devenv print-dev-env` prints holds enterShell and none of the tasks, so
  # a check run sourcing it never installs.
  tasks."flume:proto:install" = {
    exec = ''
      ${protoHome}
      ${proto}/bin/proto install --config-mode upwards
    '';
    before = [ "devenv:enterShell" ];
  };

  treefmt.enable = true;
  treefmt.config = {
    imports = [ ./treefmt.nix ];
    projectRootFile = "devenv.nix";
  };

  scripts.prose = {
    description = "The site's prose, checked by Vale (.vale.ini)";
    exec = ''
      set -euo pipefail
      cd "$(git rev-parse --show-toplevel)"
      vale site/src/content/docs README.md "$@"
    '';
  };
  scripts.site-build = {
    description = "The site, built into site/dist: moon run site:build";
    exec = ''
      set -euo pipefail
      cd "$(git rev-parse --show-toplevel)"
      moon run site:build "$@" </dev/null
    '';
  };
  scripts.lint = {
    description = "The lint gate: cargo clippy -D warnings";
    exec = ''exec ${lint}/bin/lint "$@"'';
  };
  scripts.disk = {
    description = "What target/ and the disk have grown to";
    exec = ''exec ${disk}/bin/disk "$@"'';
  };
  scripts.site-shots = {
    description = "Screenshots of the built site at phone and desktop widths, in both themes";
    exec = ''
      set -euo pipefail
      cd "$(git rev-parse --show-toplevel)/site"
      bun scripts/shots.mjs "$@"
    '';
  };
  # The floor under every commit, installed into the common hooks directory
  # on `devenv shell`. convco refuses a message that is not a conventional
  # commit (.versionrc); treefmt formats what is staged; clippy fails a
  # warning when a Rust file or the cargo manifests change.
  git-hooks.hooks = {
    convco = {
      enable = true;
      settings.configPath = ".versionrc";
    };
    treefmt.enable = true;
    clippy-gate = {
      enable = true;
      name = "clippy (-D warnings)";
      entry = "${lint}/bin/lint";
      files = "\\.rs$|Cargo\\.(toml|lock)$";
      pass_filenames = false;
    };
  };

  # The shims above, in place of devenv's own install.
  tasks."devenv:git-hooks:install".exec = lib.mkForce ''
    set -euo pipefail
    hooks="$(${lib.getExe pkgs.git} rev-parse --path-format=absolute --git-path hooks)"
    mkdir -p "$hooks"
    ${lib.concatMapStrings (type: ''
      cmp -s ${hookShims}/${type} "$hooks/${type}" || install -m 755 ${hookShims}/${type} "$hooks/${type}"
    '') hookTypes}
  '';
}

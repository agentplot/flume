# Tasks

`<base>` is the branch this place was cut from and lands on, as
CLAUDE.md's Checks has it.

## 1. The packages outside the sandbox

- [x] 1.1 Add `site/packages.nix`: `bun.lock` read with its trailing commas taken out, each package whose os and cpu take the host's fetched by the lock's integrity, a registry's from npm's and the GitHub one from GitHub's API, and bun's cache written by the pinned bun installing the lock from those tarballs served on `127.0.0.1`, its folders renamed as npm's and its links dropped; the GitHub package unpacked at `@GH@<tag>@@@1` with its `.bun-tag`. In `devenv.nix` give its path as `env.FLUME_SITE_PACKAGES`, a comment saying the shell fetches it outside the sandbox (record 0059). Check: `devenv shell -- sh -c 'test -d "$FLUME_SITE_PACKAGES"'`.

## 2. The install with no network

- [x] 2.1 Add `site/scripts/install.sh`, which refuses where `FLUME_SITE_PACKAGES` is unset, saying the devenv shell sets it, copies the cache into a scratch folder under `TMPDIR` and runs `bun install --frozen-lockfile --offline --cache-dir` the copy, removing it after. `site:install` runs it, its inputs and `site:build`'s naming `packages.nix` and `scripts/install.sh`; `site/README.md` says where the packages come from (record 0059). Check: `sluice exec <base> -- moon run site:build`.

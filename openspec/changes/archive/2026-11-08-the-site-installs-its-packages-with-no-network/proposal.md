# The site installs its packages with no network

## Why

Main's gate failed #165 and #166 at `site:install`: `bun install
--frozen-lockfile` asked npm's registry from the gate's sandbox, whose
network reaches localhost alone, and the sandbox refused the name
lookup, 297 lines like `error: DNSResolveFailed downloading tarball
picomatch@2.3.2`. It had passed before only because bun found every
package in the host user's own cache, which the sandbox lets a run read
and which no declared cache of the host's is; deneb's stood empty on
2026-11-08 (record 0059).

## What Changes

- **The packages are fetched outside the sandbox.** `site/packages.nix`
  reads `site/bun.lock` and fetches each package the host's os and cpu
  take by the integrity the lock gives it, and the pinned bun writes its
  cache from them in a build. `devenv.nix` builds it as the shell is
  made, which the merge queue does outside the sandbox when it equips a
  place, and gives its path as `FLUME_SITE_PACKAGES`.
- **`site:install` reaches no registry.** `site/scripts/install.sh`
  copies the cache to a scratch folder and runs `bun install
  --frozen-lockfile --offline` from the copy, refusing to run where
  `FLUME_SITE_PACKAGES` is not set.
- **The policy is unchanged.** `.config/sluice.toml` keeps `network =
  "localhost"`.

## Capabilities

### New Capabilities

### Modified Capabilities

- `the-gate`: the site's packages are installed with no network, from
  what the devenv shell fetches outside the sandbox.

## Impact

`site/packages.nix`, `site/scripts/install.sh`, `site/moon.yml`,
`site/README.md`, `devenv.nix` and record 0059. Nothing of flume's
crates or plugins, and no clause of the statement: the gate's sandbox is
the tree's own policy (core.21, core.22).

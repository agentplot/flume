---
number: 59
title: The site's packages are fetched outside the gate's sandbox and installed with no network
status: accepted
date: 2026-11-08
decision-makers:
- fix/flume-names-each-repository-by-its-address--site-install-reaches-its-registry
---

# The site's packages are fetched outside the gate's sandbox and installed with no network

Status: accepted by the user on 2026-11-09, on their card. Proposed on 2026-11-08 by the code stage of the fix `flume-names-each-repository-by-its-address--site-install-reaches-its-registry`. The brief left the choice between installing within what the sandbox allows and opening the policy's network to the registry; keeping the network shut, and how the packages reach the sandbox, are the stage's own choices, cheap to change.

Deciders: the code stage, on the brief: "Make site:install pass in main's gate without reaching beyond what the sandbox allows, or change the policy in this bolt so the task that fetches packages may reach the registry, as flume's records allow."

Sources: sluice's #165 and #166 on main, each failed by `site:install`, its log at `flume.git/sluice/.logs/main/c147/site:install.log` on deneb, 297 lines like `error: DNSResolveFailed downloading tarball picomatch@2.3.2`; `.config/sluice.toml`'s `[executor]`, `network = "localhost"`; sluice's `crates/sluice/src/profiles/checks.rs` at its main, whose Seatbelt profile for `"localhost"` denies every other connection, name lookups through mDNSResponder's socket among them; `boomdrivedev/blueprints:design/sluice/requirements.md`, clauses 35 ("A cache changes how fast a check runs, never what it reports"), 83 and 84, and C.3's sandbox row; the change flume-publishes-its-own-flake-and-module, design 11, whose `FLUME_FLAKE_INPUTS` the devenv shell fetches outside the sandbox; bun 1.4.2, as `devenv.nix` pins it; rules core.21 and core.22.

## Context and problem statement

`site:install` ran `bun install --frozen-lockfile` in the gate's sandbox, whose network reaches localhost alone. It passed on main through 2026-11-07, 20 to 48 seconds a build, though the sandbox's profile has refused every name lookup since sluice wrote it on 2026-10-12: bun found every package in a cache it could read, the host user's own `~/.bun/install/cache`, which the sandbox lets a run read and a session's install fills, and which no declared cache of the host's is. Once it stood empty on deneb, as it did on 2026-11-08, `site:install` asked npm's registry, the sandbox refused the name lookup, and main's gate failed #165 and #166, both of which change `devenv.nix`, an input of `site:build`. What the gate reported had turned on what a session left in the host user's home.

## Considered options

1. **The devenv shell fetches the packages, and `site:install` installs from them with no network.** `site/packages.nix` reads `bun.lock`, fetches each package the host's os and cpu take by the integrity the lock gives it, and has the pinned bun write its cache from them in a build, from a registry served on the build's own loopback. `devenv print-dev-env`, which the merge queue runs outside the sandbox to equip a place, builds it, and `devenv.nix` gives its path as `FLUME_SITE_PACKAGES`. `site:install` copies it to a scratch folder and runs `bun install --frozen-lockfile --offline` from the copy.
2. **`[executor] network = true`.** The policy's network is one for every check: every check of the tree could then reach any address, and the gate would fail whenever npm's registry did not answer. sluice gives no task a network of its own.
3. **A fixed-output derivation running `bun install` with the network.** One hash for each system the shell runs on, written by hand at each move of `bun.lock`, and the Linux one only from a Linux builder. A stale hash would fail every devenv shell, not the site alone.
4. **bun's cache written by Nix from the tarballs.** bun names a prerelease's folder by a hash of its tag, `get-tsconfig@5.0.0-0ea33bea221d7ae0@@@1`, which Nix cannot compute.
5. **The bun cache declared as one of the host's shared caches.** The host's configuration is no file of this repository's, and a declared cache changes how fast a check runs, never whether it passes (sluice 35): a cold one would fail the gate the same way.

## Decision outcome

Option 1.

- **No hash is kept.** Each package's fetch is fixed by the lock's own integrity, so a moved lock fetches what it names at the next shell, and a tarball that does not match it fails the fetch.
- **bun lays out its own cache.** The build serves the lock's tarballs on `127.0.0.1` and installs the lock from there with `--cache-dir` the output. bun names a folder by its registry's host where that is not npm's, `@@127.0.0.1`, which the build takes out, so each folder is named as bun names one of npm's registry; the build drops the links bun keeps beside its folders, whose targets are absolute. On deneb on 2026-11-08 the 297 folders it wrote for aarch64-darwin were named and held byte for byte as bun's own install from npm wrote them; the x86_64-linux build, on the host's Linux builder, wrote the Linux packages.
- **GitHub's package.** bun fetches `@agentplot/site-theme` from GitHub's API, not a registry, so the build unpacks it where bun looks, `@GH@<tag>@@@1` with its tag in `.bun-tag`, fetched from the same API URL by the lock's integrity.
- **A writable copy.** The store keeps the cache read-only, and bun copies a folder's mode with it, so a package holding a nested `node_modules` could not take one; `site/scripts/install.sh` copies the cache into a scratch folder under `TMPDIR` and removes it after, and refuses to run where `FLUME_SITE_PACKAGES` is not set.
- **The network stays shut.** `.config/sluice.toml` is unchanged.

### Consequences

- Good: the gate's `site:install` reads nothing of the host user's home and reaches no registry; it took 4.6 seconds in a sandbox like the gate's on deneb, the copy included.
- Good: CI's shell, on Linux and macOS, builds the same cache and installs from it as the gate does.
- Bad: a move of bun's pin can change how bun names its cache's folders; `site:install` then fails naming a package not in the cache, and the build's rename is the place to look.
- Bad: each install copies about 230 MB.
- Neutral: boomdrivedev/boom installs its site the same way under the same network, and stands where flume stood until its host's cache empties.

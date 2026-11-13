# Tasks

Every command runs from the place's root, inside `devenv shell`.

## 1. The addresses of the moved tools name boomdrivedev

- [x] 1.1 The README, CLAUDE.md, `openspec/config.yaml`, the rules in `docs/architecture/core.md` and the site's rules page, the conformance suite's comment, `cliff.toml`, `.vale.ini` and the Vale styles, the pages workflow, the flake's module key and homepage and its tests, the weir crates in `Cargo.toml` and `Cargo.lock`, the shell's sluice release, the site's configuration and pages, and the plugins' repository and install line name boomdrivedev and its sites; boomdrive-web, flywheel-next and flywheel stay agentplot's (record 0066). Verify that `git grep -n -E '(^|[^@_.-])agentplot(/(blueprints|boom|boomdrive|boomdrive-cloud|crew|flume|site-theme|sluice|strategy|timber|weir)([^-a-z]|$)|\.github\.io)' -- ':!openspec/changes/archive' ':!docs/adr/0034-*'` prints nothing, then `sluice exec main -- moon run nix:test root:marketplace`.
- [x] 1.2 `site/bun.lock` takes site-theme from boomdrivedev/site-theme at v0.1.3 with the tag and integrity its tarball now has, written by the pinned bun with `bun install --lockfile-only`. Verify with `sluice exec main -- moon run site:install site:build site:prose`.
- [x] 1.3 The records' citations and live mentions name boomdrivedev, record 0034's history and record 0027's paths as written; record 0066 says which addresses moved and which names stay. Verify with `sluice exec main -- moon run root:adrs`.

## 2. The spec

- [x] 2.1 The delta moves clause-coverage's citation of the statement. Verify with `openspec validate flumes-addresses-name-boomdrivedev --strict`, then archive with `openspec archive flumes-addresses-name-boomdrivedev --yes`.

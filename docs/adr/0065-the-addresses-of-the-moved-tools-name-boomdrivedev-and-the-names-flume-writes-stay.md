---
number: 65
title: The addresses of the moved tools name boomdrivedev, and the names flume writes stay
status: accepted
date: 2026-11-12
decision-makers:
- Chuck Swanberg
---

# The addresses of the moved tools name boomdrivedev, and the names flume writes stay

Status: accepted on the brief of the fix `flumes-addresses-name-boomdrivedev`, 2026-11-12, which decides the addresses in its words: "Every agentplot/ address in flume's tree of a tool that has moved names boomdrivedev/ instead: its flake inputs, its site's address, its README, policies and skills." Which names are no address, and which mentions are history, are the fix's own choices below, cheap to change.

Deciders: the brief, above; the fix's code stage, for the choices.
Sources: the brief; `gh repo list boomdrivedev` and `gh repo list agentplot`, read on 2026-11-12; the sites' answers on 2026-11-12; record 0034, whose split of live mentions from history is the precedent; timber's record 0032 on its fix `timbers-addresses-name-boomdrivedev`, which moves timber's tree the same way; afterthought/kit-standard `docs/orgs/`, which holds `agentplot.md` and no org named boomdrivedev; the OpenSpec change `flumes-addresses-name-boomdrivedev`.

## Context and problem statement

blueprints, boom, boomdrive, boomdrive-cloud, crew, flume, site-theme, sluice, strategy, timber and weir moved from the agentplot owner on GitHub to boomdrivedev. boomdrive-web, flywheel-next and the archived flywheel stay agentplot's. GitHub redirects a moved repository's old address, and its pages do not move with a redirect: on 2026-11-12 https://boomdrivedev.github.io/flume/, https://boomdrivedev.github.io/sluice/ and https://boomdrivedev.github.io/blueprints/flume/ answered 200, and the same paths on the agentplot owner's pages answered 404. A commit cited in the agentplot owner's blueprints is the same commit in boomdrivedev/blueprints.

The move changed one thing beyond an address. bun fetches `@agentplot/site-theme` as GitHub's tarball of its tag, and GitHub names the tarball's top folder after the owner, so the tarball for v0.1.3 now unpacks under `boomdrivedev-site-theme-4d05a31` from either address, and its hash is no longer the one `site/bun.lock` gave. A host without that tarball in its store could not build the shell's site packages (record 0059).

flume's tree said agentplot in four kinds of place: addresses of the moved tools, addresses of the tools that stay, names flume writes or is installed by that take agentplot as a namespace, and history. Three questions are left to the fix:

1. Do the names that take agentplot as a namespace move?
2. Which mentions in the records move?
3. Does `site/bun.lock` move by its text alone?

## Considered options

For the names, which are the herdr plugin's id `agentplot.flume`, its action `agentplot.flume.places` and its state directory, the Claude Code marketplace's owner and the plugin's author, `agentplot`, the site theme's package name `@agentplot/site-theme`, the kit standard's org in `.config/kit.toml`, the licence's holder and the streams and roles named agentplot:

1. Keep them.
2. Move them to boomdrivedev.

For the records:

1. Move every citation and live mention, as record 0034 moved the renamed repositories, and keep history as written: what a record says GitHub or a host answered on a past day, a person's quoted words, and the archived OpenSpec changes.
2. Move every mention, history among it.
3. Move none.

For `site/bun.lock`:

1. Change its addresses as text.
2. Install the lock again with the bun the shell pins, so the package's tag and integrity are the ones its new address serves.

## Decision outcome

The names stay, option 1. A name is no address, and the brief moves addresses. A host that installed the herdr plugin or the Claude Code plugin holds it under these names, and a host's herdr configuration binds a key to `agentplot.flume.places`; moving them would leave each host's plugin installed twice and its key bound to nothing. The site theme's package is still named `@agentplot/site-theme` at v0.1.3, and the kit standard knows an org `agentplot` and none named boomdrivedev. A stream or a role named agentplot, `agentplot/design` among them, is a host's word and names no repository.

The records are option 1. A record is read for its citations, which a reader follows, and the moved repositories' pages answer only under boomdrivedev, so each citation moves at the revision it names. Record 0034's sources and map say what GitHub answered and what the brief mapped on 2026-11-04, and keep their words, as record 0034 kept the history it found; its citation of the statement and its live mentions move. Record 0027's paths are a host's folders on a day. Option 2 changes what a record says happened, and option 3 leaves every citation resting on GitHub's redirect and every page link on a 404.

`site/bun.lock` is option 2. Its package's address, tag and integrity move together, `boomdrivedev-site-theme-4d05a31` and its hash, written by bun 1.4.2 with `bun install --lockfile-only`; no other package of the lock changes. Option 1 leaves the lock naming a hash GitHub no longer serves.

### Consequences

- The README, CLAUDE.md, `openspec/config.yaml`, the rules' narrative and citations in `docs/architecture/core.md` and the site's rules page, the conformance suite's comment, `cliff.toml`, `.vale.ini` and the Vale styles' links, the pages workflow, the site's configuration and pages, the Claude Code plugin's repository and the herdr plugin's install line name boomdrivedev and its sites.
- The flake's module key is `github:boomdrivedev/flume#<platform>Modules.flume`, the package's homepage is boomdrivedev/flume, and the weir crates in `Cargo.toml` and `Cargo.lock` come from boomdrivedev/weir at the same revision, so `nix/packages/git-crates.nix` keeps its hashes. The shell fetches sluice's release from boomdrivedev/sluice, the same file at the same hash.
- The clause-coverage spec names boomdrivedev/blueprints main, through the OpenSpec change `flumes-addresses-name-boomdrivedev`.
- boomdrive-web, flywheel-next and flywheel keep agentplot's addresses wherever flume names them, those repositories having stayed.
- `openspec/config.yaml` keeps the local paths of one host's checkouts, whose folder names are the host's.
- No rule changes: this record moves addresses once and binds no later diff.

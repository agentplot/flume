# flume

One launch, carried into one worktree of its own at
`<repository>/places/<kind>--<name>`, on `<prefix>/<name>` cut from the
base, and worked through its kind's stages. Each stage is a fresh agent
whose prompt is confirmed against its transcript; once its deliverable
exists and its agent is quiet, the next stage starts. The place is
tidied once the work lands or is dropped. It runs: a fix and a change
are carried through to their landing by the walkthroughs of the
`check-tools` layer.

The design is in agentplot/blueprints: the capture design,
`design/capture/design.md`, section "Launching: flume", and its plan,
`plan.md` beside it (decision 12, steps L1 to L4, and P6 and C7 where it
meets millpond and crew). flume's statement,
`design/flume/requirements.md`, is written and ruled, and amended as
the operator rules (plan step L1). This repository cites the
statement's clauses by number and never restates them. Where the site
and the design disagree, the design wins.

## Rules

The rules are in `docs/architecture/`: `core.md` binds wherever a change
is made, and an area page, when one exists, binds the paths its
frontmatter names. The decisions behind them are `docs/adr/`, one MADR
record each, proposed until the operator's yes. A design or a commit
that relies on a rule cites its id; one that departs from a rule says
which and why. Where `docs/architecture/` and the narrative docs
disagree, `docs/architecture/` wins. A rule is sent to `core.md` or its
area page with an id, the next free number in its area, amended in the
same commit as the decision that changes it. The kit standard binds
beside them as its `code` profile says (`.config/kit.toml`).

## Checks

Every tool runs through the shell, one command at a time:
`devenv shell -- <command>`. moon runs the checks, tagged by layer
(`.moon/workspace.yml`); `.config/sluice.toml` says which layers each
branch requires:

| layer | what | who requires it |
|---|---|---|
| `check-fast` | the site's prose (Vale) and its build; each crate's format (rustfmt), lint (clippy) and tests over the fakes, and every profile over real git and files on scratch repositories; the Claude Code plugin's hooks against the built binary; the herdr plugin's manifest, its lint (shellcheck) and its scripts against stand-in herdr and flume under sh and dash; the conformance suite's clause coverage; the root's format (treefmt), toolchain, records, specs, open changes, commit hooks, layers and versions, every file that carries flume's version giving the workspace's | every branch, and CI on Linux and macOS |
| `check-tools` | the sluice and herdr profiles against the pinned tools, the herdr plugin linked into the pinned herdr, and the walkthroughs with the stand-in for Claude Code (`crates/flume/tests/profiles/stand-in/`) | main |
| `check-live` | the walkthroughs with the pinned Claude Code under herdr (`crates/flume/tests/profiles/live/`) | no branch: run by hand in the devenv shell, `FLUME_LIVE_CLAUDE_CODE_OAUTH_TOKEN=<token> devenv shell -- moon run flume:live`, the token a test account's from `claude setup-token` |

Run suites through sluice, so the agents on this host take turns under
its bound: once your work is committed, `sluice exec main` for the
gate's checks, and while iterating `sluice exec main -- moon run
<project>:<task>`. The projects are those `.moon/workspace.yml` lists:
`seams`, `core`, `fakes`, `conformance` and `flume` (the crates under
`crates/`), `plugin-claude` (`plugins/claude`), `plugin-herdr` (`plugins/herdr`), `site` and `root`. A quick, narrow call, `cargo check` or one unit test,
is fine to run directly. `devenv shell -- moon run :#check-fast` runs
the fast layer, `:#check-tools` the tools layer; `site-build` and
`prose` run one each, `lint` the workspace's clippy, `disk` what `target/` has grown to, and `moon run
<crate>:test` one crate's tests (`seams`, `core`, `fakes`,
`conformance`, `flume`). The site's own notes are `site/README.md`.

## Commits

Stage by pathspec and commit the paths you wrote: `git add <paths>`, then
`git commit -m "<subject>" -- <paths>`. Never `-a`, never `add .`.
Conventional Commits subjects, scope the part of the tree touched; the
commit-msg hook refuses any other, and the pre-commit hook runs treefmt
and, when a Rust file or the cargo manifests change, clippy, in every
worktree (`root:hooks`).

@docs/architecture/core.md

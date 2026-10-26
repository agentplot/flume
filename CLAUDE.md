# flume

One launch, carried into one worktree of its own at
`<repository>/places/<kind>--<name>`, on `<prefix>/<name>` cut from the
base, and worked through its kind's stages. Each stage is a fresh agent
whose prompt is confirmed against its transcript; once its deliverable
exists and its agent is quiet, the next stage starts. The place is
tidied once the work lands or is dropped. Designed, not built.

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
`devenv shell -- <command>`. moon runs the checks, tagged by layer:

| layer | what | how long |
|---|---|---|
| `check-fast` | the site's prose (Vale) and its build; each crate's format (rustfmt), lint (clippy) and tests over the fakes (cargo); the conformance suite's clause coverage | under a minute |

`devenv shell -- moon run :#check-fast` runs them; `site-build` and
`prose` run one each, and `moon run <crate>:test` one crate's tests
(`seams`, `core`, `fakes`, `conformance`, `flume`). The site's own
notes are `site/README.md`.

## Commits

Stage by pathspec and commit the paths you wrote: `git add <paths>`, then
`git commit -m "<subject>" -- <paths>`. Never `-a`, never `add .`.
Conventional Commits subjects, scope the part of the tree touched; the
commit-msg hook refuses any other.

@docs/architecture/core.md

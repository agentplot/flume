# flume

One launch, carried into one worktree of its own at
`<repository>/places/<kind>--<name>` on `<kind>/<name>` and worked
through its kind's stages. Each stage is a fresh agent whose prompt is
confirmed against its transcript, and ends when its deliverable exists.
The place is tidied once the work lands or is dropped. Designed, not
built.

The design is in agentplot/blueprints: the capture design,
`design/capture/design.md`, section "Launching: flume", and its plan,
`plan.md` beside it (decision 12, steps L1 to L4, and P6 and C7 where it
meets millpond and crew). flume's statement,
`design/flume/requirements.md`, is written from them next (plan step
L1). This repository cites the statement's clauses by number and never
restates them. Where the site and the design disagree, the design wins.

## Rules

flume's rules are written after its statement, into
`docs/architecture/` with the decisions behind them in `docs/adr/`, and
then imported here. Until then the kit standard binds as its `code`
profile says (`.config/kit.toml`).

## Checks

Every tool runs through the shell, one command at a time:
`devenv shell -- <command>`. moon runs the checks, tagged by layer:

| layer | what | how long |
|---|---|---|
| `check-fast` | the site's prose (Vale) and its build | seconds |

`devenv shell -- moon run :#check-fast` runs them; `site-build` and
`prose` run one each. The site's own notes are `site/README.md`.

## Commits

Stage by pathspec and commit the paths you wrote: `git add <paths>`, then
`git commit -m "<subject>" -- <paths>`. Never `-a`, never `add .`.
Conventional Commits subjects, scope the part of the tree touched; the
commit-msg hook refuses any other.

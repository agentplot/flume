# flume

flume runs: a fix and a change are carried from their launch to their
landing through sluice, and their place tidied, by the walkthroughs its
checks run. The design is in
[agentplot/blueprints](https://github.com/agentplot/blueprints).

flume carries one piece of work into one worktree and follows it until
it lands. A launch names a repository, a kind, a brief, a name and a
base, the branch the work is cut from and lands on. flume makes the
place, a worktree on a branch of its own, and runs the kind's stages
there, each with a fresh agent. Once the work lands or is dropped, flume
tidies the place.

<p align="center"><img src="docs/diagrams/flume.svg" alt="Five launches, each naming its kind, its points and its stream, wait for room in one host's streams: work, priority 10, under its bounds on agents and points in flight, and personal, priority 0, under its pace of 8 points a day, each on a pool of accounts. Each launch with room starts in a place of its own, its stages lit in turn with a fresh agent each; its last delivery submits it, sluice lands it on main and flume tidies the place." width="900"></p>

- **The site:** https://agentplot.github.io/flume
- **The design book:** https://agentplot.github.io/blueprints/flume/
- **The statement**, flume's invariants as numbered clauses, is
  `design/flume/requirements.md` in agentplot/blueprints.
- **The Claude Code plugin:** `/plugin marketplace add agentplot/flume#release`,
  then `/plugin install flume@flume`.

## What it does

- **One launch, one place.** `flume launch` makes a worktree of the
  repository, `<kind>--<name>`, on the branch `<prefix>/<name>` cut
  from the base, under flume's data directory, inside no working tree
  of the repository, whether it is a clone or a bare repository beside
  its worktrees. The prefix is the kind's,
  `fix/`, `chore/` or `change/` for flume's own. The merge queue's
  policy says what a worktree needs to be ready.
- **A kind is a list of stages.** A fix is code, then a submission to
  sluice. A change is construct, which writes the OpenSpec change, then
  code, which ticks its tasks, then verify, which archives it, and a
  submission. A repository or a launch may declare kinds of its own.
- **A fresh agent for each stage.** Each stage starts a new agent in a
  herdr pane, or as a command flume prints. flume sends the stage's
  prompt only once the agent's own hook says it started, and sends it
  again when no hook says it was received.
- **A stage ends on its deliverable:** commits, the change written, its
  tasks ticked, the change archived or a report, made since the stage
  started.
  Once it exists and the agent is quiet, the next stage starts. An agent
  that stops short says what it needs, and the place waits for a retry
  or a drop. A quiet pane ends nothing.
- **One queue for the host.** Every launch on a host joins one queue.
  A place's first stage starts only while the host, its stream and an
  account of the stream's pool have room. Waiting places start by their
  stream's priority, then their own, and flume refuses no launch for
  want of room. A place's points, and a stream's bound on points in flight and its
  pace, are decided and wait on their build.
- **Tidied once it is done.** When the work lands or is dropped, flume
  removes the place. It removes only places it made, and keeps a branch
  that holds commits its base lacks.
- **Its state, live.** `flume status --json` gives each place's kind,
  its stage, its agent's state, whether the deliverable exists, and its
  submission, with a change's tasks ticked.

Two things depend on the host's own setup. An instruction with
characters beyond ASCII reaches the agent whole only when the pane's
shell has a UTF-8 locale, `LANG` set to one, for instance
`en_US.UTF-8`. Under herdr, flume reports each workspace's summary of
its places as a metadata token, `flume`; it shows only where the host's
herdr sidebar names `$flume` in its rows (record 0018).

In a watermill the flume is the channel that carries water from the
pond to the wheel, a measured flow to where it does work. flume carries
one piece of work to one worktree, where an agent does it; sluice gates
what lands.

## This repository

| path | what |
|---|---|
| `crates/` | the crates, each a moon project: `flume-seams` (`seams`), `flume-core` (`core`), `flume-fakes` (`fakes`), `flume-conformance` (`conformance`) and the binary `flume` (`flume`), whose `tests/profiles/stand-in/` stands in for Claude Code |
| `plugins/claude` | flume's Claude Code plugin, the project `plugin-claude` |
| `plugins/herdr` | flume's herdr plugin, the project `plugin-herdr` |
| `docs/architecture/`, `docs/adr/` | the rules and the decisions behind them |
| `site/` | the project `site`: the site, Astro with Starlight; its diagrams are drawn at build time (`site/README.md`) |
| `docs/diagrams/` | the README's diagram, generated from the site's |
| `devenv.nix` | the shell: moon through proto, Vale, bun, and Playwright with its Chromium |
| `styles/Flume` | the prose rules, the design books' voice in Vale |
| `.config/kit.toml` | the kit standard's profile this repository follows |

moon runs the checks (`.moon/workspace.yml`), each tagged with its
layer: `check-fast` (every crate's format, lint and tests, the plugin's,
the site's prose and build, and the root's checks over the whole tree),
`check-tools` (the sluice and herdr profiles against the pinned tools,
and the walkthroughs) and `check-live` (the walkthroughs with the pinned
Claude Code under herdr, run by hand with a test account's token in
`FLUME_LIVE_CLAUDE_CODE_OAUTH_TOKEN`).

```sh
devenv shell -- moon run :#check-fast
devenv shell -- site-build   # the site, through moon
devenv shell -- prose        # the prose, through Vale
```

## License

MIT

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

<p align="center"><img src="docs/diagrams/flume.svg" alt="A launch from your session, millpond or headwater runs down the flume into a worktree on change/csv-export, made ready by the repository's declaration; construct, code, verify and submit each start a fresh agent whose prompt is confirmed in its transcript and end when their deliverable exists; sluice lands the change and flume tidies the place." width="900"></p>

- **The site:** https://agentplot.github.io/flume
- **The design book:** https://agentplot.github.io/blueprints/flume/
- **The statement**, flume's invariants as numbered clauses, is
  `design/flume/requirements.md` in agentplot/blueprints.
- **The Claude Code plugin:** `/plugin install flume --marketplace agentplot/flume`.

## What it does

- **One launch, one place.** `flume launch` makes a worktree of the
  repository at `<repository>/places/<kind>--<name>`, on the branch
  `<prefix>/<name>` cut from the base. The prefix is the kind's,
  `sounding/` for a question. The repository's own declaration of what
  a worktree needs makes it ready.
- **A kind is a list of stages.** A fix is code, then a submission to
  sluice. A change is construct, which writes the OpenSpec change, then
  code, which ticks its tasks, then verify and a submission. A question
  is sounding's session, which ends on `sounding conclude`. A
  repository or a launch may declare kinds of its own.
- **A fresh agent for each stage.** Each stage starts a new agent in a
  herdr pane. flume sends the stage's prompt only once the agent accepts
  input, and sends it again when the agent's transcript lacks it.
- **A stage ends on its deliverable:** commits, the change written, its
  tasks ticked, a report or the landing, made since the stage started.
  Once it exists and the agent is quiet, the next stage starts. An agent
  that stops short says what it needs, and the place waits for a retry
  or a drop. A quiet pane ends nothing.
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

flume is one of a family. headwater reports releases, eddy lists
meetings, millpond keeps the inbox, sounding works questions and sluice
lands changes. flume stands alone, and launches the same for your own
`/launch`, millpond's send, headwater's fix key, sounding and crew's
stages.

In a watermill the flume is the channel that carries water from the
pond to the wheel, a measured flow to where it does work. flume carries
one item from millpond to one worktree, where an agent does the work;
sluice gates what lands, and the Flywheel keeps turning.

## This repository

| path | what |
|---|---|
| `crates/` | the crates, each a moon project: `flume-seams` (`seams`), `flume-core` (`core`), `flume-fakes` (`fakes`), `flume-conformance` (`conformance`) and the binary `flume` (`flume`), whose `tests/profiles/stand-in/` stands in for Claude Code |
| `plugins/claude` | flume's Claude Code plugin, the project `plugin-claude` |
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

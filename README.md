# flume

**Designed, not built.** The design is settled in
[agentplot/blueprints](https://github.com/agentplot/blueprints); no code
exists yet.

flume carries one piece of work into one worktree and follows it until
it lands. A launch names a repository, a kind, a brief and a name. flume
makes the place, a worktree on the branch `<kind>/<name>`, and runs the
kind's stages there, each with a fresh agent. Once the work lands or is
dropped, flume tidies the place.

- **The site:** https://agentplot.github.io/flume
- **The design book:** https://agentplot.github.io/blueprints/flume/
- **The statement**, flume's invariants as numbered clauses, comes
  next, as `design/flume/requirements.md` in agentplot/blueprints.

## What it does

- **One launch, one place.** `flume launch` makes a worktree of the
  repository at `<repository>/places/<kind>--<name>`, on
  `<kind>/<name>`, and makes it ready by the repository's own
  declaration of what a worktree needs.
- **A kind is a list of stages.** A fix is code, then a submission to
  sluice. A change is construct, which writes the OpenSpec change, then
  code, which ticks its tasks, then verify and a submission. A question
  is sounding's session, which ends on `sounding conclude`. A
  repository may declare kinds of its own.
- **A fresh agent for each stage.** Each stage starts a new agent in a
  herdr pane. flume sends the stage's prompt only once the agent accepts
  input, and sends it again when the agent's transcript lacks it.
- **A stage ends on its deliverable.** The next stage starts when the
  deliverable exists, or when the agent stops short and says what it
  needs. A quiet pane ends nothing.
- **Tidied once it is done.** When the work lands or is dropped, flume
  removes the place. It removes only places it made, and keeps a branch
  that holds commits its target lacks.
- **Its state, live.** `flume status --json` gives each place's kind,
  its stage, its agent's state, whether the deliverable exists, and its
  submission, with a change's tasks ticked.

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
| `site/` | the site, Astro with Starlight; its diagrams are drawn at build time (`site/README.md`) |
| `devenv.nix` | the shell: moon through proto, Vale, bun, and Playwright with its Chromium |
| `styles/Flume` | the prose rules, the design books' voice in Vale |
| `.config/kit.toml` | the kit standard's profile this repository follows |

```sh
devenv shell -- site-build   # the site, through moon
devenv shell -- prose        # the prose, through Vale
```

## License

MIT

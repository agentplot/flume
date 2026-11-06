# flume for herdr

flume's plugin for [herdr](https://herdr.dev): the watch that settles
flume's places as their agents change, and the places in flight in
herdr's sidebar and a popup, with keys that act through flume's
commands.

```sh
herdr plugin install agentplot/flume/plugins/herdr --ref v<version>
```

Install it at the tag of the flume you run, so the plugin and the
binary are one version. It needs herdr 0.9.5 or later, `flume` and `jq`
on the `PATH` herdr's server runs with, on Linux or macOS. Set
`terminal = "herdr"` under `[host]` in flume's configuration
(`$XDG_CONFIG_HOME/flume/config.toml`) so flume opens its agents in
herdr.

## What it does

- **The watch.** herdr starts it when its server starts; one runs per
  server. Every two seconds it reads every pane's agent state in one
  listing. When the agent of one of flume's places changes state, or an
  agent comes up or goes in one of the server's panes, as a stage's new
  agent does, it runs `flume settle`: a nudge, so what that change lets
  move moves at once, such as the next stage of a program with no hook
  of its own, an agent that crashed or was killed, or a stage waiting
  for the room another agent held. It keeps no bound, and no time
  passing settles there: a place with no agent in it, as one whose
  submission waits on the merge queue, moves on at a settle of
  someone's, never the watch's. At every tick it writes
  `flume status --json` for the sidebar and the popup. It runs nothing
  of flume's but `flume settle` and `flume status`. At every tick it
  also reads herdr's configuration file and writes down what it lacks
  for the plugin, the popup's key or the sidebar's rows, which
  `flume status` gives under the line naming the host's watch. It logs
  the same once at its start, and tells no one.
- **Display clues.** From the status it writes it gives each agent's
  pane the title `<stage> · <state>` and the name `<kind>--<name>`, and
  each base's workspace a count of its places: running, asking, waiting
  on their caller and stalled, reporting them again whenever status
  changes them. They show what flume's status says, and nothing flume
  decides reads them back.
- **A row on each agent's sidebar entry**, the `$flume` token: its
  place's line, the words flume's status line gives, less the place's
  name and kind: the stage with its group, how many times it started,
  the change's tasks ticked and the place's state, as in
  `code 1/2 · run 1 · tasks 2/5 · working`.
- **The count on each workspace's row**, the `$flume` token of the
  workspace: `1 running · 0 asking · 1 waiting on caller · 0 stalled`.
- **The popup** (`agentplot.flume.places`): every place in flight, each
  its number and its line, as in
  `▶ 1  fix--one · fix@1 · code · run 1 · working`, with its location
  beneath, the chosen one marked `▶`. Each key acts at once, with no
  Enter: the arrow keys or `j` and `k` move the choice, and `1` to `9`
  choose the place so numbered. `o` opens the chosen place's tab, `t`
  tells its agent the words you type, `s` stops its agent and `r`
  retries its stage; where words are needed, `t`'s words, `r`'s and
  whether `s` or `r` should force, it asks for them as a line, ended by
  Enter. `q` or Escape closes it. Each key runs one flume command,
  `flume tell`, `flume stop` or `flume retry`, or focuses the place's
  tab. No key types into an agent's pane.

## Keeping watch over the host

The watch decides nothing of how flume runs. Whatever keeps time on
your host, a timer or a supervisor of your own, keeps watch over it by
running `flume settle --every <time>` on that bound, which settles every
place, an agent in it or none. While its newest settle began
less than twice its bound ago, flume's commands leave the settling to
it. While nothing does, the plugin running or not, flume runs attended:
each command of flume's that acts settles every place first.
`flume status` says which.

## Setting it up

The plugin changes none of herdr's configuration. Add the popup's key
and the sidebar's rows to herdr's `config.toml` yourself, or to
whatever generates it: the file `HERDR_CONFIG_PATH` names, else
`$XDG_CONFIG_HOME/herdr/config.toml`, else
`~/.config/herdr/config.toml`. Until it holds both, `flume status`
says what it lacks under the line naming the host's watch.

### The popup's key

herdr's plugins declare no keys: a key reaches the popup only through
an entry of your own. Add this one, or bind another key the same way:

```toml
[[keys.command]]
key = "prefix+f"
type = "plugin_action"
command = "agentplot.flume.places"
description = "flume: places in flight"
```

`prefix+f` is free among herdr's own keys.

### The sidebar's rows

```toml
[ui.sidebar.agents]
rows = [
  ["state_icon", "machine", "workspace", "tab"],
  ["agent"],
  ["$flume"],
]

[ui.sidebar.spaces]
rows = [
  ["state_icon", "workspace", "$flume"],
  ["branch", "git_status"],
]
```

The rows above are herdr's own plus flume's; if you have rows of your
own, add only the `$flume` entries.

## Where it keeps its state

Under herdr's state directory for the plugin,
`$XDG_STATE_HOME/herdr/plugins/agentplot.flume`, one directory per
herdr server: the watch's lease, the status it last wrote and its log,
`watch.log`, and what herdr's configuration lacks, `setup.json`. flume
reads the lease to know the watch runs, and `setup.json` beside a lease
that is fresh to say what herdr's configuration lacks.

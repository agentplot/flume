# flume for herdr

flume's plugin for [herdr](https://herdr.dev): the places in flight in
a popup, with keys that act through flume's commands, and the rows flume
shows in herdr's sidebar. It keeps no process and runs no settle.

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

- **Nothing between herdr's start and your asking.** The plugin keeps
  no process: it runs no `flume settle` and reads no status on a timer.
  Places move when their agents go quiet, at each settle of a caller
  keeping watch over the host, and at each command of flume's that acts.
- **At herdr's start**, once for each server, it reads herdr's
  configuration file and writes down what it lacks for the plugin, the
  popup's key or the sidebar's rows, which `flume status` gives under
  the line naming the host's watch while that server runs. It logs the
  same, tells no one, and exits. Where an earlier release of the plugin
  left its watch running on that server, it ends it.
- **Display clues**, which flume itself shows at each settle that
  changes one, through its herdr profile, and which the plugin neither
  computes nor copies: each agent's pane its title `<stage> · <state>`
  and the name `<kind>--<name>`, and each base's workspace a count of
  its places. Nothing flume decides reads them back.
- **A row on each agent's sidebar entry**, the `$flume` token: the
  short form of its place's line, the words flume's status line gives
  less the place's name and kind: the stage with its group, how many
  times it started, the change's tasks ticked and the place's state, as
  in `code 1/2 · run 1 · tasks 2/5 · working`. It is one of flume's
  clues, shown at the settles that change it.
- **The count on each workspace's row**, the `$flume` token of the
  workspace: `1 running · 0 asking · 1 waiting on caller · 0 stalled`.
- **The popup** (`agentplot.flume.places`): every place in flight, read
  from `flume status --json` when you open it and again after each key
  that acts, and never on a timer: each its number and its line, as in
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

The plugin keeps no watch and decides nothing of how flume runs.
Whatever keeps time on your host, a timer or a supervisor of your own,
keeps watch over it by running `flume settle --every <time>` on that
bound, which settles every place, an agent in it or none. While its
newest settle began less than twice its bound ago, flume's commands
leave the settling to it. While nothing does, flume runs attended: each
command of flume's that acts settles every place first, and what no
agent's quiet reports, an agent killed with no word or a submission the
merge queue decided, moves at the next such command. `flume status`
says which.

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
herdr server: `setup.json`, the server's socket and what herdr's
configuration lacks, and the startup's log, `startup.log`. flume says
what a server's `setup.json` lacks while the socket it names is
present.

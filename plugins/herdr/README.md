# flume for herdr

flume's plugin for [herdr](https://herdr.dev): the watch that moves
every place on with no command of yours, and the places in flight in
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
  listing. When the agent of one of flume's places changes state, an
  agent comes up or goes in one of the server's panes, as a stage's new
  agent does, or the bound passes, it runs `flume settle`, then writes
  `flume status --json` for the sidebar and the popup. The bound is the one flume's
  status gives, thirty seconds. A place moves on without you: a stage's
  next start, its submission, the landing and the tidy, and an agent
  that crashed, hung or was killed is read within the bound. While the
  watch runs, flume's commands leave the settling to it; once it stops,
  each command settles every place first, as flume does with no watch.
  It runs nothing of flume's but `flume settle` and `flume status`.
- **Display clues.** After each settle it gives each agent's pane the
  title `<stage> · <state>` and the name `<kind>--<name>`, and each
  base's workspace a count of its places: running, asking, waiting on
  their caller and stalled. They show what flume's status says, and
  nothing flume decides reads them back.
- **A row on each agent's sidebar entry**, the `$flume` token: the
  kind, the stage, its state, the change's tasks ticked, the
  submission, and whether the place waits on you or is stalled, as in
  `change · code · working · 2/5`.
- **The count on each workspace's row**, the `$flume` token of the
  workspace: `1 running · 0 asking · 1 waiting on caller · 0 stalled`.
- **The popup** (`agentplot.flume.places`): every place in flight with
  its row, the chosen one marked `▶`. Each key acts at once, with no
  Enter: the arrow keys or `j` and `k` move the choice, and `1` to `9`
  choose the place so numbered. `o` opens the chosen place's tab, `t`
  tells its agent the words you type, `s` stops its agent and `r`
  retries its stage; where words are needed, `t`'s words, `r`'s and
  whether `s` or `r` should force, it asks for them as a line, ended by
  Enter. `q` or Escape closes it. Each key runs one flume command,
  `flume tell`, `flume stop` or `flume retry`, or focuses the place's
  tab. No key types into an agent's pane.

## Setting it up

The plugin changes none of herdr's configuration. Add the key and the
sidebar rows to herdr's `config.toml` yourself, or to whatever
generates it:

```toml
[[keys.command]]
key = "prefix+f"
type = "plugin_action"
command = "agentplot.flume.places"
description = "flume: the places in flight"

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
`watch.log`. flume reads the lease to know the watch runs.

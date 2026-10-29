---
title: State
description: What flume keeps on the host and where, what it keeps in a place's own folder, made.jsonl among it, and the fields of flume status --json.
sidebar:
  order: 3
---

flume keeps the live state of each place it runs on the host, as plain
files, and nothing in any repository's content. A place's state goes
when the place is tidied. Everything else a reader needs, a stage's
deliverable, a change's tasks, a submission's outcome, flume reads
where it lives each time it is asked, and keeps no copy between asks.

## On the host

```
$XDG_STATE_HOME/flume/              ~/.local/state/flume by default
  room.lock                         held while a start takes the host's room
  flume.log                         a timed line per display hint or notice that did not go through
  places/
    <place>.json                    the place's held state, one document, with its version
    <place>.lock                    held while the document is written
    <place>/                        the place's own folder
      made.jsonl                    what a stage's run made, a JSON line each
      claude/<identity>/            the plugin's record of one agent's start
        started.json                the started ack: session, transcript, plugin version
        received                    the digests of the prompts the agent received
        asking.json                 the asking mark, while a question is open
        refused                     each act the guard refused
        last-words                  why the agent's last turn failed
        ended                       the session ended
      printed/<identity>            a mark: this start's command was printed once
      herdr/<digest>/               a start's pane shell, and the link putting its program first on PATH
    <place>@<time>.json             a tidied place's document, kept while an end of it is pending
  terminal/herdr/                   flume's records of the herdr panes, tabs and workspaces it made
    <identity>.json                 an agent's pane: its id, server and herdr name
    tabs/<digest>.json              a place's tab
    workspaces/<digest>.json        a repository and base's workspace
    sessions/<digest>.json          a recipient's pane, found by its name
```

`<place>` is the place's location with its leading `/` dropped and
each `/` written `%`, one file name per place. Each act on a place
takes `<place>.lock`, reads the document's version, and writes a new
document over it only at that version. So two commands acting on one
place at once leave it as one or the other would have. A tidy removes
the document and the folder together, `made.jsonl` with it.

## Elsewhere on the host

| Path | What |
|---|---|
| `$XDG_CONFIG_HOME/flume/config.toml` | the host's configuration ([configuration](/flume/reference/configuration/#the-hosts-configuration)) |
| `$XDG_DATA_HOME/flume/plugins/claude/<version>-<digest>/` | the Claude Code plugin flume hands its agents, written once per version and binary |
| `<place's git directory>/flume/toolchain.sh` | the environment the place's toolchain step printed, which later steps run under; it goes with the worktree |
| `$XDG_STATE_HOME/herdr/plugins/agentplot.flume/` | the herdr plugin's lease, its last status and `watch.log`, one directory per herdr server |

## In the repository

Nothing of flume's. A place is a worktree under
`<repository>/places/`, on a branch of its own, and holds only what its
agents commit. flume writes no record into a repository's tree or onto
a branch.

## The file made.jsonl

A stage whose kind declares `makes` records each thing it made as one
JSON line in the place's own `made.jsonl`, whose path the prompt part
`{{made}}` gives. A line names its thing by `name` and says what it is,
with its address, or the branch and revision that bring it up:

```json
{"name":"preview","what":"the tokenizer behind a flag","branch":"spike/try-it","revision":"9ba6b13"}
{"name":"notes","what":"the benchmark's raw numbers","address":"https://example.com/acme/bench/42"}
```

Only `name` is flume's to read: a stage is not delivered while a thing
it declares has no line since the stage started. Every other field is
the run's own, and flume carries each line as written in the stage's
end, in status, and in a wait's and the inbox's answer. It builds,
checks and rewrites no address. A line that is no JSON object is
carried as its text.

## The answer of flume status --json

```json
{"status":{"places":[{…}],"host":{…}}}
```

Each place:

| Field | |
|---|---|
| `place`, `repository` | the place's location and its repository |
| `kind`, `version`, `declared` | the kind, its version, and where it was declared: `flume`, `repository` or `launch` |
| `name`, `branch`, `base`, `mark` | as the launch gave them |
| `lands` | whether the kind lands |
| `phase` | `live`, or `tidied` with when and why: `landed` with its commit, `dropped` or `done` |
| `readiness` | each readiness step, whether it passed and what it printed |
| `stages` | each stage: `name`, `state`, `starts`, `end`, `conversation`, and the part or setting it was refused for |
| `current` | the current stage's name |
| `agent` | the current agent: `identity`, `stage`, `program`, `settings` with `from`, the layer of each, `state`, `started`, `asking`, `own_work`, `past_limit`, `screen`, `printed`, `last_words` and `notes` |
| `head` | the place's branch head |
| `delivered` | whether the current stage's deliverable exists |
| `tasks` | a change's tasks ticked and in all |
| `submission` | `read`, what the merge queue holds; `submits`; `send_backs`; `verdict`; `waiting`, why the place waits on its submission; `held` |
| `refused_acts` | each act the guard refused |
| `cannot_read` | each fact flume could not read when asked |
| `failures` | each act that failed, in flume's words |
| `recipients` | the place's recipients |
| `waiting_on`, `stalled` | what the place waits on its caller for, since when and in whose words; whether it is stalled |
| `holds`, `hold_words` | each hold and its words |
| `priority` | the place's priority |
| `ends` | each end, with whom it was delivered to, when and by what, `notice`, `wait` or `inbox`, and whom it is pending for |
| `clues` | the display hints: the stage, the state, the pane's title and name, and where herdr shows them |

A stage's `state` is one of `not_reached`, `waiting_on_readiness`,
`on_hold`, `waiting_for_room`, `starting`, `started`, `begun`, `delivered`,
`ended_short`, `finished`, `never_begun`, `lost`, `stuck`, `stopped`
and `refused`. An agent's `started`, `asking` and `own_work` are `yes`,
`no` or `unknown`.

The host:

| Field | |
|---|---|
| `running` | agents running on the host, every caller's |
| `bound`, `room` | the host's bound, and how many more may start; `null` with no bound |
| `queue` | the stages waiting for room, in the order they start |
| `watch` | the terminal's watch, `none` with no herdr plugin running |
| `bases` | for each repository and base, its places running, asking, waiting on their caller and stalled, and the line shown in herdr |
| `addresses` | each role's address with ends pending, and why its run was not woken |
| `terminal_kept` | what a tidy kept in the terminal because something flume did not make holds it |

The JSON only ever gains fields, so a tool reading it keeps working
across releases.

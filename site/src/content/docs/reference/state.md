---
title: State
description: What flume keeps on the host and where, the host's own documents of accounts and intents among it, what it keeps in a place's own folder, made.jsonl among it, and the fields of flume status --json and flume list --json.
sidebar:
  order: 3
---

flume keeps the live state of each place it runs on the host, as plain
files, and nothing in any repository's content. A place's state goes
when the place is tidied. Each spent account and each intent's
affinity is a document of the host's own beside them. Everything else
a reader needs, a stage's deliverable, a change's tasks, a submission's
outcome, an account's reading, flume reads where it lives each time it
is asked, and keeps no copy between asks.

## On the host

```
$XDG_STATE_HOME/flume/              ~/.local/state/flume by default
  room.lock                         held while a start takes the host's room
  kinds-lock.lock                   held while flume kinds lock reads and writes a lock of kinds
  flume.log                         a timed line per display hint or notice that did not go through
  places/
    <place>.json                    the place's held state, one document, with its version
    <place>.lock                    held while the document is written
    <place>/                        the place's own folder
      made.jsonl                    what a stage's run made, a JSON line each
      claude/<identity>/            the plugin's record of one agent's start
        own-settings.json           the agent's own settings as the layers resolved them, which --settings names
        started.json                the started ack: session, transcript, its length at the first ack, plugin version
        received                    the digests of the prompts the agent received
        asking.json                 the asking mark, while a question is open
        refused                     each act the guard refused
        last-words                  why the agent's last turn failed
        failure.json                the failed turn's error, its words and when the hook ran
        unsettled.json              the agent began otherwise than its start recorded: each variable by name, whether the program differed, its session and transcript
        ended                       the session ended
      printed/<identity>            a mark: this start's command was printed once
      herdr/<digest>/               a start's pane shell, and the link putting its program first on PATH
    <place>@<time>.json             a tidied place's document, kept while an end of it is pending for a recipient or its caller
  accounts/<pool>/<account>.json    an account's spent time
  intents/<digest>.json             an intent's affinity
  watch.json                        the host's watch: when a caller's newest flume settle --every began, and its bound
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

`own-settings.json` is one JSON object, the agent's own settings for
its program as the layers resolved them, each key as its layer or its
file gave it and never a key left out. flume writes it whole before
the terminal opens the agent, a new file renamed over the old,
readable by the host's user alone (mode `0600`), and only where the
layers give a key. It is never a file a layer named.

### The host's own documents

Beside the places lie three documents of the host's own. flume writes
each whole the same way, under a lock of its own, by compare-and-set on
its version, readable by the host's user alone (mode `0600`). Every
flume process on the host reads them.

| Document | What it holds |
|---|---|
| `accounts/<pool>/<account>.json` | the account spent: `until`, the time it is spent until; `words`, the usage limit's words; `recorded_at`, when flume recorded it; and `unread`, `true` where flume read no reset in the words, so `until` is an hour after `recorded_at`, left out otherwise. A later time already recorded stands. |
| `intents/<digest>.json` | the intent's affinity: its `stream`; `accounts`, for each program its agents run, keyed by the program as an account's `program` names it, the `pool` and `account` the first of them on that program started on, as moved since; each move between accounts, with `on` `intent`, `program`, the program whose account moved, `from`, `to`, `why`, `spent`, `threshold` or `full`, `at`, and, where `why` is `threshold`, `window` and `until`, the window that held the account and until when; and `gone`, when its last place went. flume removes it on a read an hour after `gone`. |
| `watch.json` | the host's watch: `began`, when the newest `flume settle --every` began, by the host's clock, in seconds; and `bound`, the bound it stated, in seconds. Each `flume settle --every` writes it before it settles. A caller keeps watch while `began` is less than twice `bound` ago, and flume runs attended once it is not, or where no `watch.json` stands. flume never removes it. |

`<digest>` is the SHA-256, in hex, of `name:<name>` for an intent the
launch named, else of `base:<repository id>:<base>`.

An intent's document an earlier release wrote holds one `pool` and
`account` and no `accounts`, and its moves name no `program`. flume
reads that account as the entry of `accounts` for the program the
host's configuration gives it, and each move as that program's. Where
the pool no longer declares the account, the intent keeps its stream
alone. flume writes the document with `accounts` at its next change,
and never writes a lone `pool` or `account`.

### A start's record

Each start in a place's held state carries `start_record`: the
agent's `program`, its `account`, and `env`, a digest of each variable
by its name, leaving out those the agent program keeps from its hooks,
such as Claude Code's `CLAUDE_CODE_OAUTH_TOKEN`. Each digest is HMAC-SHA256, keyed by the start's secret,
over the variable's name, a zero byte and its value, in hex; no value
is kept. A start that carries a conversation, a spent agent's or a
lost one's, also records `session`, the session it resumes ([the
conversation
carried](/flume/guides/agent-settings/#the-conversation-carried)).
Every start after a spent agent or a lost conversation's retry records
`conversation` in the held state, as status gives it. A stage waiting
to carry a conversation, or to start a lost one afresh, keeps in its
`room` `carry`: `start`, the start whose conversation it carries,
`cause`, `lost` for a conversation reported lost and left out for a
spent agent, `carry` and `why`. Beside it, `live` holds each process
that ran that conversation when flume last read, by its `pid` and
`started`, when it started; or `live_unread`, what flume could not
read, naming a variable or the host's processes and never a path. A
lost stage's `end` holds `left_in_pane`, `true` where flume left the
agent the terminal brought back in its pane. Each start records `terminal`, the
multiplexer its agent opened in, with `from`, and the place keeps
`terminals`, each multiplexer a start of it opened an agent in, once,
so its tidy reaches every one. A start an earlier release wrote records
none, and its agent is read as the host's. The place's `unsettled`
lists each start whose agent began otherwise than its record says
([the start check](/flume/guides/agent-settings/#the-start-check)).

### An agent kept open and its answers

A stage that ended short needing an answer keeps, in its `end`,
`open_until`, the time its agent is kept open for the answer until: the
time flume took the stop-short and the start's `keep_open`. It is left
out where the agent was not kept open, as under the printed command.
Once a settle lets the agent go, the end records `closed`:

| Field | |
|---|---|
| `at` | when the settle read the agent let go |
| `by` | why: `time`, its time passed; `stopped`, its caller stopped it; `agent_ended`, it stopped, its session ended, or the terminal brought back an agent in its pane that is not its start's; `account_spent`, its account was spent |
| `ended_at` | when flume ended it; left out until it is |
| `left_in_pane` | `true` where the terminal brought back an agent in a conversation's pane and flume left it there; left out otherwise |

flume records the answer to a stop-short on its end in the place's
`ends`, as `answer`. It holds `words`, the answer; `act`, `retry` or
`tell`; `at`, when the act began; `by`, who gave it, as the answering
command's recipient named it, left out where it named none; `key`, the
key the answer named, left out where it named none; and `session`,
where the agent kept open took it. That holds `sent_at`, when flume
last typed it, and `received_at`, when the agent's own record read it
received. Every retry with words of the stage whose stop-short the
place waits on records the answer, a fresh agent's with no `session`.
While an answer is being typed into the agent kept open, or one naming
its key starts a fresh agent, the end holds `claim`: `by` and `key` as
an answer gives them, `at`, when it was claimed, and `until`, past which
a later answer may take the stop-short. A second answer that finds the
claim is refused with nothing typed, and the claim goes once the answer
is recorded or not taken. A start keeps `answered`, how many answers its agent took
in its session, left out while it is none. An end of a stage read after
an answer carries `answered`, the start's count then, in its `of`, so
each stop-short of one start is an end of its own. The start's
`settings` keep `keep_open` as its layer wrote it.

A held state an earlier release wrote reads with no agent kept open, no
answers and no close.

### The question, the hold and the stall

A stop-short that named a role by `flume needs --to` records, in its
stage's `end`, `addressee`, the role's address whole. Its end stands
pending for that address alone.

A hold in the place's `holds` names what it holds by `on`: `stage`,
with its `stage`, `submission`, or `waiting`, the move the place waits
on its caller for. It keeps its `words` and `since`, and a hold on the
waiting move `bound`, how long it holds in seconds, and `lapsed`, when
the settle that read its bound passed lapsed it. Each is left out where
it has none.

A stall's end, `of` `stall`, keeps `since`, when the place began to
wait under no hold's words, and `waiting`, the move. It keeps too
`answerer`, the role standing behind the place's stream as the sources
declared it when the stall was read; `stop_short`, where the move
answers one, with its `words`, `needs`, `key` and `addressee`;
`lapsed`, the words of the hold whose lapse stalled the place again;
and `answerer_read`, `true` where this flume read the answerer and the
stop-short. A stall an earlier release recorded has neither, and the
first settle that reads it while the place still waits under no hold's
words reads them for it.

A launch keeps where its recipients came from, `recipients_from`:
`from` `launch`, `environment`, `session`, or `answerer` with its
`source`. A place an earlier release launched keeps none.

### The kind a place keeps

A place's held state keeps the whole kind its launch read, its prompts
among it, as `kind`, and where it came from as `kind_from`, written
once at the launch with the fields status gives. Nothing a source, a
package or a lock does after the launch reaches it
([a place keeps its kind](/flume/guides/kinds/#a-place-keeps-its-kind)).
A place an earlier release launched keeps no `kind_from`.

A start whose program a layer gave own settings also holds `own`.
Its `files` are each file a layer named for the program, most specific
first, with its `layer` and its path as `written` and as `read`. Each
file's `left_out` gives the keys its program's profile reserves that
flume left out of it, unread, by their paths alone. Its `passed` is the
arguments of its command that name the file it wrote, as `--settings
<file>`. The record compares no own setting, since no release of the
program reads them back. The place's held state keeps the launch's
`settings`, each program's file with its keys as the launch read it,
and `repository_agents`, the repository's `[agents]` as the launch read
it.

## Elsewhere on the host

| Path | What |
|---|---|
| `$XDG_CONFIG_HOME/flume/config.toml` | the host's configuration ([configuration](/flume/reference/configuration/#the-hosts-configuration)) |
| `$XDG_CONFIG_HOME/flume/sources/<source>.toml` | a source of kinds the host declares, which flume reads and never writes ([the host's sources](/flume/reference/configuration/#the-hosts-sources)) |
| `$XDG_CONFIG_HOME/flume/flume.lock` | the host's lock of kinds, written by `flume kinds lock --host` ([the locks of kinds](/flume/reference/configuration/#the-locks-of-kinds)) |
| `$XDG_DATA_HOME/flume/kinds/<source>/` | where a program that fetches a source's packages may put them, each at `<name>@<version>/`; flume reads them and never writes them |
| `$XDG_DATA_HOME/flume/plugins/claude/<version>-<digest>/` | the Claude Code plugin flume hands its agents, written once per version and binary |
| `$XDG_DATA_HOME/flume/places/<id>/<kind>--<name>/` | a place, a worktree of its repository outside the repository's own directory; `<id>` is the repository's, as its agents' identities carry it. A base place, whose kind lands with no stages, takes its name from its location, and nothing stands there |
| the file an account's `usage` names | the account's reading, which something of the host's writes whole and flume reads at each choice of an account and whenever status is asked, and never writes, keeps or renews ([an account's reading](/flume/guides/agent-settings/#an-accounts-reading)) |
| `$XDG_STATE_HOME/herdr/plugins/agentplot.flume/` | the herdr plugin's `setup.json`, its server's socket and what herdr's configuration lacks for the plugin, and `startup.log`, one directory per herdr server |

## In the repository

Nothing of flume's, but the lock of kinds `flume kinds lock` writes into
a working copy's `.config/flume.lock` where you run it there, which you
commit. A place is a worktree on a branch of its own,
under flume's data directory, inside no working tree or directory of
the repository, whether the repository is a clone or a bare repository
beside its worktrees. It holds only what its agents commit. A base
place has no worktree: it is its branch, which the merge queue makes
and removes at flume's asking, and the state flume holds of it
([branches that land on
branches](/flume/guides/merge-queue/#branches-that-land-on-branches)).
flume writes no record into a repository's tree or onto a branch.

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
| `kind`, `version`, `declared` | the kind, its version, and where it was declared: `flume`, `repository`, `launch`, or `host` for a source the host declares |
| `identity` | the kind's identity as its launch read it, as `acme/review@2`; none for a place an earlier release launched |
| `kind_from` | where the kind came from, as its launch read it, each field given only where it applies. `identity` gives its `source`, `name`, `version` and `spelled`, and `source` the source's name. A source the host declares gives `source_file` and `kind_dir`, the package's directory. The repository's gives `declaration_file`, `.config/flume.toml`, and `revision`, the base's, and a launch's `declaration_file`, the file's path. A package gives `hash`, and `lock_file`, the lock that recorded it: the repository's where it does, else the host's |
| `name`, `branch`, `base`, `mark` | as the launch gave them |
| `lands` | whether the kind lands |
| `phase` | `live`, or `tidied` with when and why: `landed` with its commit, `dropped` or `done`, and `pending_for`, whom an end of it is pending for, the caller its launch named among them until it collects the end of the place's work. A place whose work is over that still stands is `over`, with why and `kept`, what keeps it: among them `session_runs`, kept while its session runs, each process that runs its conversation in `processes`, by its `pid` and `started`, or `unread`, what flume could not read |
| `readiness` | each readiness step, whether it passed and what it printed |
| `stages` | each stage: `name`, `state`, `starts`, `end`, `conversation`, and the part or setting it was refused for. A lost conversation's `end` holds `left_in_pane`, `true` where flume left the agent the terminal brought back in its pane, left out otherwise. A stop-short's `end` holds `open_until` and `closed` ([an agent kept open and its answers](#an-agent-kept-open-and-its-answers)). `answers`, each answer the stage's current start took in its session, in order: `question`, the question it answered, `words`, `act`, `at` and `session`, with `sent_at` and `received_at`; left out where there are none |
| `current` | the current stage's name |
| `agent` | the current agent: `identity`, `stage`, `program`, `settings` with `from`, the layer of each, each variable `env.<NAME>` from `account` as `<set>`; in `settings`, `programs` with the agent's program's entry alone, and `keep_open` as its layer wrote it, its layer in `from` as every setting's; in `from` each of its own settings, `programs.<program>.settings.<key>`, with its `layer`, its `value` as written and its `file` where it came from one; `own`, where a layer gave its program own settings, with `passed`, the arguments that named the file its start wrote, `read_back`, `false`, and `files`, each file a layer named with its `layer`, `written`, `read` and `left_out`, each key left out with its `key` and its `owner`; `account`, its `pool`, `name` and `from`, `affinity`, the intent's affinity for the agent's `program`, `strategy`, or `conversation`, the account a lost conversation's start ran under, which holds it; `mode`, where a layer gave one, with `passed`, the value its program was given, `entry`, the program whose entry of a table it took, and `read_back`, whether its program's word reads the mode back; `effort`, where a layer gave one, with `word`, as its layer wrote it, `layer`, `passed`, the value its program was given, `read_back`, `false`, since no program reads its effort back, and `outranked_by`, the variable its program reads over the effort, by its name, where the agent's account's `env` names it, left out otherwise; `state`, `started`, `asking`, `own_work`, `past_limit`, `screen`, `dialog`, the words of a dialog it shows before it takes its prompt, `printed`, `last_words` and `notes`; `session`, the herdr session it opened in, `name`, `null` for the terminal's own, and `from`: `launch`, `stream-stage`, the session its stream names for its kind's stage, `stream-kind`, the one its stream names for its kind, `stream`, `stream-name` or `own`, none where it opened beside a pane; `terminal`, the multiplexer it opened in, `profile` and `from`: `stream`, `host` or `neither`, `null` for a start an earlier release wrote, whose agent is in the host's; `left_in_pane`, `true` for an agent the terminal brought back in a lost conversation's pane, which flume left there and sends nothing, left out otherwise; and, for the agent of a start after a spent agent or a lost conversation's retry, `conversation`: `from`, the start whose conversation it carries, `cause`, `lost` where that start's conversation was reported lost and left out where its agent was spent, `carried`, and `why` where it was not carried, naming a variable and never a path |
| `head` | the place's branch head; a base place's as the merge queue reads its branch, nothing read at its location |
| `delivered` | whether the current stage's deliverable exists |
| `tasks` | a change's tasks ticked and in all |
| `submission` | `read`, what the merge queue holds; `request`, the pull request flume's last submit opened or updated where the base takes them, and `read`'s entry names it once taken in; `submits`; `send_backs`; `verdict`; `waiting`, why the place waits on its submission; `held` |
| `refused_acts` | each act the guard refused |
| `cannot_read` | each fact flume could not read when asked; among them, each branch the merge queue could not read, in its words, which flume reads past unless it is the place's base, where the place's own submission is |
| `failures` | each act that failed, in flume's words |
| `recipients`, `recipients_from` | the place's recipients, and where its launch took them from: `from` `launch`, `environment`, `$FLUME_RECIPIENT`, `session`, the session it ran in, or `answerer` with the `source` that declares the answerer behind its stream; `recipients_from` left out for a place an earlier release launched and for a role's run |
| `caller` | the caller its launch named, fixed by the launch, for which the end of the place's work waits; it is no recipient. Left out where the launch named none, as for a role's run and a place an earlier release launched |
| `waiting_on`, `stalled` | what the place waits on its caller for, since when and in whose words, or, with `act` `answer` and `why` `dialog`, on a person to answer its agent's dialog, in the dialog's words; for an answer, `open_until`, the time its stage's agent is kept open until, while it is, and `closed`, once a settle let it go, as the stage's `end` gives it; for a base place with no place on its branch live, `act` `retry` and `why` `unsubmitted`, or `submission` with the merge queue's words where its submission failed, conflicted or was withdrawn; whether it is stalled |
| `waits_on` | whom a live place waits on, as the list gives it: `caller`, `host`, `session`, `merge_queue`, or `places`, a base place's while a place on its branch is live, which is never stalled; left out where it waits on nobody |
| `worktree` | `false`, for a base place alone, which has no worktree; left out for every other place |
| `on_branch` | for a base place alone, the places being made or live whose base is its branch, by their locations; left out for every other place |
| `waits_on_session` | the stage waiting on its session, as its line says `waiting on its session`: `stage`; `processes`, each process that runs the conversation its start would carry, by its `pid` and `started`; or `unread`, what flume could not read, naming a variable or the host's processes and never a path; and `since`, when it began waiting. It is never stalled. Left out where no stage waits so |
| `holds`, `hold_words` | each hold and its words: `on`, `stage` with its `stage`, `submission`, or `waiting`, the move the place waits on; `words`; `since`; and for a hold on the waiting move, `bound`, in seconds, and `lapsed`, when it lapsed, each left out where it has none |
| `priority` | the place's priority |
| `stream` | the stream the place runs in, `name` and `from`: `launch`, `intent`, `host-repository-kind`, `host-repository`, `host-kind` or `default` |
| `label` | the place's label as it would resolve at a start now, `name` and `from`: `host-repository-kind`, `host-kind` or `kind`; `null` where nothing gives one |
| `intent` | the intent the place serves: `from` `launch` with its `name`, or `base` with its `repository` and `base`; and its `digest` |
| `moves` | each move between accounts, of a stage (`on` `stage` with its `stage`) or of the intent (`on` `intent` with its `program`, the program whose account moved, none on a move an earlier release recorded): `from`, `to`, `why`, `spent`, `threshold` or `full`, `at`, and `cache`, `lost`; `window` and `until`, where `why` is `threshold`, the window that held the account and until when; and `carried`, whether a stage's conversation went with it, where its agent was spent |
| `unsettled` | each start whose agent began without its settings: `stage`, `start`, `at`, the variables `missing` and `different` by name, `program`, whether the program differed, `session`, `true` where it came up in another session than the one its start resumes, and `mode`, where the mode did, with `asked` and `began` |
| `uncarried` | each start that carried a conversation, a spent agent's or a lost one's, and did not begin: `stage`, `start`, `at` and `words`, what happened; its stage started afresh after it |
| `ends` | each end, with its `key` as callers name it, `stage/<stage>/<start>/<state>[/<answered>]`, `submission/<n>/<outcome>`, `stall/<since>` or `over/<dropped\|done>`; whom it was delivered to, when and by what, `notice`, `wait` or `inbox`; and `pending_for`, whom it is pending for. The end of the place's work, its landing or its drop's or done's end, is pending for the caller its launch named besides its recipients until the caller collects it by the inbox or a wait, a notice to its name counting for none. A drop's or a done's end, `of` `over` with its `over`, `dropped` or `done`, has no recipient: it is pending for the caller alone, none where the launch named none. A stop-short's gives `open_until`, the time its agent was kept open until when the end was read, `answer`, once one answered it, with `by` and `key`, and in its `end` the `addressee` it named. A stall's gives its `answerer`, `stop_short` and `lapsed` ([the question, the hold and the stall](#the-question-the-hold-and-the-stall)) |
| `clues` | the display hints: the stage, the state, the pane's title and name, and where herdr shows them. The settles that change them show the terminal these with the line's `short` beside them, which herdr gives the agent's pane as its `flume` token, the sidebar's row |
| `line` | the place's one line: `name`, `<kind>--<name>`; `kind`, `<kind>@<version>`; `stage`, the current stage with a repeating stage's group, as `code 1/2`; `run`, how many times that stage started; `tasks`, the change's tasks ticked at the head, as `2/5`; `state`, the place's state in words; `full`, the parts joined by ` · `; and `short`, `full` less `name` and `kind`, as the herdr sidebar shows it. A base place's `state` is `<n> on its branch · open` while places on its branch are live, then `waiting on its caller`, `submitted` and the rest as any place's. `stage`, `run` and `tasks` are `null` where the place has none, and `full` and `short` leave them out. It is the line `flume status --line` gives, read from the place's held state, its change and its agent's own record alone ([the status line](/flume/guides/claude-code/#the-status-line) lists the states) |

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
| `queue` | the stages waiting for room, in the order they start, each with its stream, its `label`, its priority and the room it wants. That room, by `room`, is the host's, `host`; its stream's, `stream`, at its `bound`; its stream's past what other labels keep free, `kept-free`, naming those `labels`; or a usable account of its pool, `pool`, with `until`, the soonest time an account of the pool is neither spent nor past its threshold, `null` where each is only at its bound. A stage whose pool holds no account of its program waits for none: it is refused ([a pool that cannot run a stage](/flume/guides/agent-settings/#a-pool-that-cannot-run-a-stage)). |
| `streams` | each stream: `name`, `session`, the herdr session its agents open in with `from`, `stream`, `stream-name` or `own`; `multiplexer` and `terminal_app`, each its `profile` and `from`, `stream`, `host` or `neither`; `view`, its `view`, `terminal` or `app`, and `from`; `title`, its `title` and `from`, `stream` or `stream-name`; `priority`, `running`, `bound`, `room` and `queue`, `bound` and `room` `null` where the stream has no bound of its own; `labels`, each with its `name`, `reserve`, `keep_free`, `running` and `waiting`; `answerer`, the role standing behind it as the sources declare it, `{"stands":{"address":…,"source":…}}`, or `{"contested":{"roles":[…]}}` where two or more each declare to, so none does, left out where no role stands behind it; `sessions`, each session it names for its kinds and their stages, `name`, `kinds` and `stages`, each `name`, the kind or `<kind>.<stage>`, and `layer`, `managed` or `local`, `kinds` or `stages` left out where it names the session for none; `faults`, each of its tables for a kind or a stage that refuses the starts that read it, `table`, its dotted key, and `refused`, the refusal's words; `reserves`, `held`, `passed-over` where the stream states no bound, or `none` where it gives no label a share; `refused`, why its labels, its multiplexer or its terminal app refuse it, `null` where none does; and `view_refused`, the terminal app's refusal alone, which refuses its `flume open` and none of its stages, left out where it holds none |
| `pools` | each pool: `name`, `strategy`, `threshold`, `90` where the pool states none, and `accounts`, each account's `name`, `program`, `running`, `bound` and `spent_until`, `null` where it is not spent; `spent_unread`, while it is spent for an hour because flume read no reset in its agent's words, those words; `reading`, where the account names a `usage`; and `past`, while it is past its threshold, `window`, the window that holds it, and `until`, when that window resets, or, for a window that names no reset, when its reading stops being fresh. A `reading` is `read_at`, when the host read the allowance, `fresh`, whether that was less than fifteen minutes ago, and `windows`, each with its `name`, `used`, the percentage the reading gives, `resets_at` where it gives one, and `past`, whether that window holds the account past its threshold now; or `unread`, why there is none, naming no path |
| `watch` | the watch a caller keeps over the host, `{"bound":{"seconds":…}}`, the bound its newest `flume settle --every` stated; or `none` where no caller keeps watch, and every command of flume's that acts settles every place first |
| `watch_settled` | when the newest `flume settle --every` began, by the host's clock; left out where the host holds no watch |
| `watch_lapsed` | when the watch lapsed, twice its bound after its newest settle began, given while flume runs attended after one; left out while a caller keeps watch or where none ever has |
| `terminals` | each multiplexer the host binds, the host's own first, then each stream's by the stream's name, each once: `profile`, `kept`, what a tidy kept there, and `watch`, `none` for every one, since no multiplexer keeps watch or settles |
| `bases` | for each repository and base, its places running, asking, waiting on their caller and stalled, and the line shown in herdr |
| `addresses` | each role's address with ends pending, every address a source gives an agent that already runs, pending or not, and the answerer's run whose stall reached no one: `address`; `role`, the run's entry as its source declares it, with `stream`, the address's stream it runs in, `null` for any other address; `run`, the place of its run that stands; `pending`, each end with its `place`, `key` and `what`; `unwoken`, why its run was not woken, `null` for an agent's address; `carried`, its source and `answers`, where its source declares it carried to another host; `wakes_nothing`, why nothing is declared for it, as no source declares it or its entry is at fault; `stalled`, the newest stall of its run that reached no one, since the run is its own stream's answerer, with its `place`, `key` and `what`; and `standing`, `null` for a run's address, else `agent`, the name the multiplexer knows it by, `stream`, `session`, its `name`, left out for the multiplexer's own, and `from`: `role`, the entry's own, `stream`, `stream-name` or `own`, `terminal`, the profile the stream's multiplexer names, `null` where the host declares no such stream, and `found`, what the multiplexer found when asked: `{"one":{"at":…,"state":…}}`, the agent at the session name a notice reaches it by with its state, `working`, `waiting`, `asking`, `stopped` or `unknown`; `"none"`; `{"several":{"count":…}}`; or `{"unread":{"reason":…}}`, in the multiplexer's words or flume's |
| `terminal_kept` | what a tidy kept in the host's own multiplexer because something flume did not make holds it |
| `terminal_setup` | what the host's own multiplexer's configuration lacks for flume's plugin there, each in words a person acts on, naming the file read: under herdr, the popup's `[[keys.command]]` entry or the sidebar rows' `$flume`, as the plugin's startup read herdr's configuration for each server still running; empty where nothing is missing, no server's startup wrote any or the terminal reads none |
| `configuration` | the host's configuration: `local`, its local layer's `state`, `no-file`, `taken` or `not-taken` with each of its `keys` the local layer may not set or its `fault`; `values`, every value by its dotted `key`, as TOML writes it, with its `layer`, `managed` or `local`, an account's variables `<set>`; and `passed_over`, each local `key` a `pin` holds |
| `sources` | each source of kinds the host declares, read when asked: `name`, `source_file`, `kinds_dir`, `kinds`, the identity of every package its directory holds, and `cannot_read`, the fault where the file or the directory cannot be read; `roles_file`, the roles file it names; `roles`, each address it declares; `roles_refused`, each `address` it declares nothing for with the `reason`; and `roles_cannot_read`, why its roles file declares no role, each left out where it has none |

## The answer of flume wake --json

```json
{"woken":{"address":"east/ops","woke":"launched","place":{…}}}
{"held":{"address":"east/design","standing":{…},"pending":[{…}]}}
```

`woken` answers a run's address: `woke`, `launched` where this wake
launched the run, else `live` or `waiting` for the run that stands, and
`place`, that run's place as status gives one. `held` answers an
address an agent already holds, where nothing was launched: `standing`
and `pending` as the host's `addresses` give them, read in the same
ask.

## The answer of flume inbox --json

```json
{"inbox":{"recipient":"east/ops","ends":[{…}],"waiting":[{…}]}}
```

`ends` gives each end the inbox collected and recorded delivered:
`place`, `kind`, `name`, and `end`, as status's `ends` gives one, its
`key` among it. `waiting` gives each place that waits on its caller and
on the recipient, marked nothing: `place`, `kind`, `name`, `stream`;
`brief`, its brief's first line that holds anything, trimmed, left out
where it has none; `waiting_on`, the move as status gives it;
`stop_short`, where the move answers one, its `words`, `needs`, `key`
and `addressee`; and `hold`, the hold on the move while it holds.

## The answer of flume list --json

```json
{"list":{"places":[{…}],"streams":[{…}]}}
```

Read from each place's held state, a live agent's asking mark, and each
stream's multiplexer once, as [`flume
list`](/flume/reference/commands/#flume-list) says. A field the place or
the stream has none of is left out. Each place, by repository, then by
`<kind>--<name>`, one that landed or was dropped only under `--all`:

| Field | |
|---|---|
| `place`, `repository` | the place's location, which a tidied place's document gives with when it was tidied, and its repository |
| `name` | `<kind>--<name>` |
| `kind` | `<kind>@<version>` |
| `stage` | the stage the place's line names, without a repeating stage's group; left out for a place that is not live |
| `state` | the place's state in its line's words |
| `waits_on` | whom the place waits on: `caller`, `host`, `session`, a process that runs the conversation its waiting stage's start would carry, `merge_queue`, or `places`, a base place's while a place on its branch is live |
| `kept` | what keeps a place whose work is over, in its line's words: `kept by <step>`, `kept holding work`, `kept holding a lock`, `kept holding work and a lock`, `kept while its agent runs`, `kept while its session runs`, `kept unread`, `being tidied` or `tidied` |
| `line` | the parts joined by ` · `, as `flume list` prints it |

Each stream, in status's order:

| Field | |
|---|---|
| `stream` | the stream's name |
| `session` | `name`, `null` for the multiplexer's own, and `from`: `stream`, `stream-name` or `own` |
| `multiplexer` | `profile` and `from`: `stream`, `host` or `neither` |
| `runs` | what the multiplexer read of the session: `{"runs":{"name":…}}`, `{"not_running":{"name":…}}`, each with the name the multiplexer knows it by; `{"no_part":{"reason":…}}`, where it holds no session; or `{"unread":{"reason":…}}`, where it could not read it, in its words |
| `uncarried` | in place of `runs`, why no multiplexer was asked: the stream's names no profile flume carries |
| `sessions` | each other session the stream names for a kind or a stage, by name: `name`, and `runs` as above, read in the same reading as the stream's own; left out where the stream names none |
| `line` | `<stream> · session <name> · <runs>` and each other session after it, as `flume list` prints it |

The JSON only ever gains fields, so a tool reading it keeps working
across releases.

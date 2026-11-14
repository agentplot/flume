# store-profile Specification

## Purpose
The store seam, Part B.7, in the host's state directory: each place's
held state written whole under the store's lock, and the host's
configuration read from its file with each setting's layer.

## Requirements

### Requirement: One document per place in the host's state directory

The store profile SHALL keep each place's held state as one document
under `$XDG_STATE_HOME/flume/places/`, with its version, written whole
by rename under a lock and refused when the version moved, a new place's
created only where none exists, and its own folder beside it, gone with
the document (62, 102 to 104, 128, C.7).

#### Scenario: Seam scenarios over the profile

- **WHEN** the store seam's scenarios run against the profile over a scratch state directory
- **THEN** each passes, two processes writing at one version leaving one write

#### Scenario: A restart loses nothing

- **WHEN** a second engine is built over the same state directory while a stage works
- **THEN** it reads the place whole (S12, 77)

### Requirement: The host's configuration

The host's configuration SHALL be read from
`$XDG_CONFIG_HOME/flume/config.toml` at every start: `[host]` `agents`,
`terminal` and `pool`; `[streams.<stream>]`; `[pools.<pool>]`; agents'
model, mode and instructions by kind and by repository and kind, each
with `.stages.<stage>`; and `stream` where C.7 names it, and no
`[roles]` (193). A key left out SHALL be no bound, the printed command,
no stream, pool or settings (32, 70, 105, 143, 144, 162, C.7).

#### Scenario: A bound stated

- **WHEN** the configuration states `[host] agents = 2`
- **THEN** the host's bound reads two

#### Scenario: Settings for a kind and for a repository's kind

- **WHEN** the configuration gives `[kinds.change.agents] model = "opus"` and `[repos."/r".kinds.change.agents.stages.verify] model = "sonnet"`
- **THEN** the host's settings for change@1 in `/r` read `opus` for the kind at the host-kind layer and `sonnet` for verify at the host-repository layer, and for change@1 elsewhere only `opus` (32, C.7)

#### Scenario: C.7's example

- **WHEN** the configuration is C.7's example, with streams `work` and `personal`, pools `work` and `personal`, and `[repos."~/Code/acme"] stream = "work"`
- **THEN** acme's places read the stream `work` from `host-repository` and the pool `work` from the stream, whose accounts read work-1, bounded at four, then work-2, each with its `CLAUDE_CONFIG_DIR` under the host user's home (143, 144, C.7)

#### Scenario: A configuration that refuses

- **WHEN** the configuration gives a stream `pool = "missing"` that no `[pools]` table declares, or a pool `strategy = "round-robin"`
- **THEN** every launch and start is refused naming the key, and status says the host's configuration cannot be read (4, C.7)

### Requirement: The host's own documents

The store profile SHALL keep each account's spent time and reading in
one document at `accounts/<pool>/<account>.json` and each intent's
affinity at `intents/<digest>.json` under `$XDG_STATE_HOME/flume/`,
each with its version, written whole by compare-and-set under the
store's lock and readable by the host's user alone, and reached by
every flume process on the host; a document an earlier release wrote
SHALL read as written (146, 147, 153, 184, C.7).

#### Scenario: Two processes record one account spent

- **WHEN** two engines over one state directory each write an account's spent time at the version both read
- **THEN** one write lands and the other reads the version moved and writes again on what it reads (153)

#### Scenario: A spent time seen by another process

- **WHEN** one engine records work-1 spent until 15:00
- **THEN** a second engine built over the same state directory reads work-1 spent until 15:00, and the file's mode is 0600 (146, 153)

#### Scenario: An intent's document an earlier release wrote

- **WHEN** `intents/<digest>.json` holds an affinity with one `pool` and `account`, as an earlier release wrote it, and an affinity of two programs is then written over it
- **THEN** the first reads with that pool and account, and the file then holds an account for each program and no `pool` or `account` key (147, 153)

#### Scenario: A reading beside a spent time

- **WHEN** a document of work-1 holding a spent time until 15:00 and a reading is written over the state directory
- **THEN** `accounts/work/work-1.json` holds `until` and `reading`, the reading's `taken_at` and each reset RFC 3339's with their offset and each window's `used` a share from 0 to 100, its mode 0600, and a second store over the same state directory reads both (153, 184, C.7)

#### Scenario: An account's document an earlier release wrote

- **WHEN** `accounts/work/work-1.json` holds `until`, `words` and `recorded_at` alone, as an earlier release wrote it, and a document holding those and a reading is then written over it at its version
- **THEN** the first reads spent with no reading, and the file then holds the spent keys as they stood and the reading (153)

### Requirement: A mode in the host's agents tables

A mode in an agents table of the host's configuration SHALL be read as
one of flume's four or a table of programs whose entries are strings,
and any other SHALL refuse the configuration naming the table and
`mode` (162, 169, C.7).

#### Scenario: A mode for a kind and a table for a repository's stage

- **WHEN** the configuration gives `[kinds.fix.agents] mode = "edits"` and `[repos."/r".kinds.fix.agents.stages.code] mode = { claude = "auto" }`
- **THEN** the host's settings for fix@1 in `/r` read `edits` for the kind at the host-kind layer and the table for code at the host-repository layer (32, 162, C.7)

#### Scenario: A mode that is neither

- **WHEN** the configuration gives `[kinds.fix.agents] mode = "auto"`, or `mode = 3`, or `mode = { claude = 3 }`
- **THEN** every launch and start is refused naming `kinds.fix.agents` and `mode` (162, 169, C.7)

### Requirement: The host's tables for every agent and for a stream

The host's configuration SHALL read `[agents]`, settings for every
agent, and `[agents.streams.<stream>]`, for every agent in one stream,
each giving `model`, `effort`, `mode`, `instructions` and `programs`
as the kind tables do. A `program`, `env`, `beside` or `stages` there, or a stream
the host does not declare, SHALL refuse the configuration naming the
table and the key (32, 143, 195, 197, C.7).

#### Scenario: Settings for every agent and for a stream

- **WHEN** the configuration gives `[agents] model = "sonnet"`, `[agents.programs.claude] file = "~/.config/agent-launcher/claude-settings.json"` with `settings = { statusLine = { type = "command", command = "line" } }`, `[streams.work]`, `[agents.streams.work] model = "opus"` and `[agents.streams.work.programs.claude.settings] tui = "default"`
- **THEN** the host's settings read `sonnet`, the status line and the file's keys, the file under the host user's home, at the `host` layer, and `opus` and `tui` at the `host-stream` layer for `work` (32, C.7)

#### Scenario: A table these two cannot hold

- **WHEN** the configuration gives `[agents] program = "claude"`, or `[agents.stages.code] model = "opus"`, or `[agents.streams.missing] model = "opus"` with no `[streams.missing]`
- **THEN** every launch and start is refused naming the table and the key, `stages` naming `[kinds.<kind>.agents.stages.<stage>]` as where a stage's settings go (C.7)

### Requirement: Own settings in the host's agents tables

Every agents table of the host's configuration SHALL read `programs`, a
table of entries each with `file` and `settings`, kept as JSON values,
and any other shape SHALL refuse the configuration naming the table and
`programs` (195, C.7).

#### Scenario: Own settings for a kind and for a repository's stage

- **WHEN** the configuration gives `[kinds.fix.agents.programs.claude.settings] tui = "fullscreen"` and `[repos."/r".kinds.fix.agents.stages.code.programs.claude] file = "fix-code.json"`, a file beside the configuration giving a `statusLine`
- **THEN** the host's settings for fix@1 in `/r` read `tui` at `host-kind` and, for code, the file's `statusLine` at `host-repository` with the file's path (32, 195, C.7)

#### Scenario: A programs that is no table of entries

- **WHEN** the configuration gives `[agents] programs = "claude"`, or `[agents.programs] claude = 3`
- **THEN** every launch and start is refused naming the table and `programs` (C.7)

### Requirement: The host's settings files are read on the host at each start

A file a host's agents table names SHALL be read on the host with the
configuration at each start: absolute, under `~/` read as the host
user's home, or relative to the configuration's directory, and read
where it leads when it is a link. One that cannot be read, does not
parse, or is named neither `.json` nor `.toml` SHALL refuse the
configuration naming the file (195, C.7).

#### Scenario: A file flume cannot read

- **WHEN** the configuration gives `[agents.programs.claude] file = "settings.yaml"`, or names a `.json` file that does not parse
- **THEN** every launch and start is refused naming `agents.programs.claude.file` and the file (C.7)

### Requirement: The host's table for every kind of a repository

The host's configuration SHALL read `[repos."<repository>".agents]`,
settings for every agent of every kind in that repository, by its
location as `[repos."<repository>"]` is, giving `model`, `effort`,
`mode`, `instructions` and `programs` as the host's kind tables do. A
`program`, `env`, `beside`, `stages` or `streams` there SHALL refuse
the configuration naming the table and the key (32, 195, 197, C.7).

#### Scenario: A fork's table for every kind

- **WHEN** the configuration gives `[repos."~/Code/upstream-fork".agents.programs.claude.settings] attribution = { commit = "", pr = "", sessionUrl = false }`
- **THEN** the host's settings for every kind of the repository at `~/Code/upstream-fork`, under the host user's home, read `attribution` as written, at the `host-repository` layer beneath its kinds' tables (S102, 32, C.7)

#### Scenario: A repository's table that names a stage

- **WHEN** the configuration gives `[repos."/r".agents.stages.code] model = "opus"`, or `[repos."/r".agents] program = "claude"`
- **THEN** every launch and start is refused naming `repos."/r".agents` and the key (C.7)

### Requirement: An effort in the host's agents tables

An effort in an agents table of the host's configuration SHALL be read
as a string, kept as written, in every agents table the host's
configuration holds, and as a key the local layer may set over the
managed one; any other value SHALL refuse the configuration naming the
table and `effort` (32, 197, C.7).

#### Scenario: An effort for a kind and one for a repository's stage

- **WHEN** the configuration gives `[kinds.fix.agents] effort = "high"` and `[repos."/r".kinds.fix.agents.stages.code] effort = "xhigh"`
- **THEN** the host's settings for fix@1 in `/r` read `high` for the kind at the host-kind layer and `xhigh` for code at the host-repository layer (32, 197, C.7)

#### Scenario: An effort for every agent from the local layer

- **WHEN** the managed layer gives `[agents] effort = "medium"` and the local layer gives `[agents] effort = "high"`
- **THEN** the host's settings for every agent read `high` at the host layer, and status names the local layer as where it came from (32, 197, C.7)

#### Scenario: An effort that is no word

- **WHEN** the configuration gives `[kinds.fix.agents] effort = 3`
- **THEN** every launch and start is refused naming `kinds.fix.agents` and `effort` (197, C.7)

### Requirement: keep_open in the host's agents tables

The host's configuration SHALL read `keep_open` in every agents table it
reads, for a repository, a kind, a stream and every agent, and in each
`.stages.<stage>` table, in its managed layer and its local one over
it, as a string `<n>`, `<n>s`, `<n>m` or `<n>h`, kept as written, and
SHALL refuse any other value, a table of programs among them, naming
the table and the key (32, 198, C.7).

#### Scenario: A time for a kind and for one stage

- **WHEN** the configuration gives `[kinds.change.agents] keep_open = "30m"` and `[kinds.change.agents.stages.verify] keep_open = "0"`
- **THEN** it reads `30m` for change's agents and `0` for verify's, each as written (32, C.7)

#### Scenario: A time for a stream and for every agent

- **WHEN** the configuration gives `[agents.streams.work] keep_open = "10m"` and `[agents] keep_open = "45m"`
- **THEN** it reads `10m` for the stream `work`'s agents and `45m` for every agent, each as written (32, C.7)

#### Scenario: A time in another form

- **WHEN** the configuration gives `[kinds.fix.agents] keep_open = "an hour"`
- **THEN** the configuration is refused naming `kinds.fix.agents.keep_open` and the forms it takes (C.7)

#### Scenario: A table of programs

- **WHEN** the configuration gives `[agents] keep_open = { claude = "1h" }`
- **THEN** the configuration is refused naming `agents.keep_open` and that it is one time for every program (198, C.7)

### Requirement: The host's deliveries document in the state directory

The store profile SHALL keep the host's deliveries at `deliveries.json`
under `$XDG_STATE_HOME/flume/`, with its version, written whole by
compare-and-set under its own lock, readable by the host's user alone
and reached by every flume process on the host, its records as weir's
JSON spells them (C.7; weir 112; core.16).

#### Scenario: Seen by another process

- **WHEN** one engine over a scratch state directory records an end's delivery, and a second engine built over the same directory reads the host's deliveries
- **THEN** the second reads the delivery as written, and the file's mode is 0600 (C.7)

### Requirement: The host's span for an end

The host's configuration SHALL read `[host] ends_expire` as `keep_open`
is written, `<n>`, `<n>s`, `<n>m` or `<n>h`, from the managed layer
alone, giving a week where it is not given, refusing `0` and any other
form naming the key, and status SHALL give it with its layer (43, 56,
C.7; core.45; weir 104).

#### Scenario: A span stated

- **WHEN** the managed layer gives `[host] ends_expire = "48h"`
- **THEN** the host's span reads 172800 seconds from the managed layer, and an end read then expires 48 hours after it was read

#### Scenario: No span stated

- **WHEN** neither layer gives `ends_expire`
- **THEN** the host's span reads 604800 seconds, flume's own

#### Scenario: A span refused

- **WHEN** the managed layer gives `[host] ends_expire = "0"`, or `"2d"`
- **THEN** the configuration is refused naming `host.ends_expire`, as any `[host]` key out of its form is (4, C.7)

#### Scenario: A span in the local layer

- **WHEN** only the local layer gives `[host] ends_expire = "1h"`
- **THEN** the local layer is not taken, status naming the key, and the span reads a week (C.7; core.45)

### Requirement: A pool's strategy and threshold in the configuration

The host's configuration SHALL read a pool's `strategy`, `fill-first`,
`least-busy` or `most-left`, and its optional `threshold`, a whole
number from 1 to 100, each in the managed layer alone, and an account's
`name`, `program`, `env` and `agents` alone; it SHALL refuse an
account's `usage` as every word an account does not hold, naming the
account and the key, with no alias kept, and any other value of the
pool's keys, naming the key (144, 184, 185, C.7; record 0062).

#### Scenario: A most-left pool

- **WHEN** the configuration gives `[pools.work] strategy = "most-left"`, `threshold = 85`, and an account `{ name = "work-1", program = "claude" }`
- **THEN** the pool reads most-left with a threshold of 85, and work-1 names no reading (184, C.7)

#### Scenario: C.7's example

- **WHEN** the configuration is C.7's example, its accounts of `work` naming no reading
- **THEN** each of work-1 and work-2 reads, and `work` reads fill-first with a threshold of 90 (185, C.7)

#### Scenario: An account naming usage

- **WHEN** the configuration gives an account `{ name = "work-1", program = "claude", usage = "~/.local/state/usage/work-1.json" }`
- **THEN** every launch and start is refused naming `[pools.work.accounts[0]]` and `usage` as no word of an account (184, C.7; record 0062)

#### Scenario: Values that refuse

- **WHEN** a pool gives `threshold = 0`, then `101`, then `"90%"`, or `strategy = "round-robin"`
- **THEN** every launch and start is refused naming the key, and the strategy's words name fill-first, least-busy and most-left (4, C.7)

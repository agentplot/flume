# Design

## Context

flume's tree at b6cd1b5 resolves an agent's program, model, mode and
instructions from six layers at each start. `Engine::layers` in
`crates/flume-core/src/settings.rs` lists them most specific first, the
launch's, the host's for the repository and kind, the host's for the
kind, the repository's, the kind's and flume's default. `Engine::resolve`
folds them from the most general to the most specific through
`Settings::under`, keeping each setting's layer in `from`, and
`Engine::resolution` binds the program to its agent profile and maps the
mode through `Agent::mode`. `Settings` (`crates/flume-seams/src/kinds.rs`)
holds `program`, `model`, `instructions`, `beside` and `mode`. The
host's `agent_settings` (`crates/flume/src/host/config.rs`) reads the
`[kinds.<kind>.agents]` and `[repos."<repository>".kinds.<kind>.agents]`
tables, and reads a repository's key with a leading `~` as the home. The
kinds profile's `map_agent` (`crates/flume/src/profiles/kinds/weir/mod.rs`)
reads every agents table of a declaration from the table weir-kinds
returns as written, and reads a stage's prompt by a path relative to its
declaration's file, at the revision for a repository's, on disk for a
launch's and from the binary for flume's own (157). `commands/launch.rs`
reads `--agent`. A stream (`[streams.<stream>]`) holds `agents`, its
bound, `priority` and `pool`, and gives no settings (143; record 0020).
The Claude Code profile's `command`
(`crates/flume/src/profiles/agent/claude/mod.rs`) passes `--plugin-dir`,
`--model`, `--permission-mode` and `--append-system-prompt`, with the
account's environment and the four `FLUME_*` variables. `profiles.rs`
binds Claude Code's profile alone; record 0026's common profile is not
built yet.

What prompted the change: on the user's host, a session a person starts
is given its program's settings by one file the host's launcher passes
Claude Code with `--settings`. Read on 2026-11-01, that file gives
`statusLine`, `tui` (`"fullscreen"`), `outputStyle`,
`alwaysThinkingEnabled`, `hooks` on nine events, `enabledPlugins`,
`plugins`, `enableAllProjectMcpServers`, `teammateMode` and
`autoUpdaterStatus`. It also gives three keys this design leaves to
another setting or to a person: `env`, `permissions.defaultMode` beside
`permissions.allow`, and `skipDangerousModePermissionPrompt`. An agent
flume starts runs with its account's configuration home alone and gets
none of it, so its pane has neither the status line nor the full-screen
renderer. The user asked for one generic way, for every program flume
runs, defaulted for the host and possibly different by stream. Every
layer, the host's default first, is to be able to name a whole settings
file by path as well as give keys inline, the file's keys under the
inline ones of the same layer, so a host hands flume's agents the file
its people use.

The pinned Claude Code 2.1.296 bears it out. `--settings
<file-or-json>`, "Path to a settings JSON file or a JSON string to load
additional settings from", loads the `flagSettings` source, which the
release orders `["userSettings","projectSettings","localSettings","flagSettings","policySettings"]`.
It merges the sources as a deep merge: objects key by key, a list
joined with the lists beneath it and its duplicates dropped, and
anything else from the later source. So a key given by `--settings`
lays over the account's own `settings.json` and the repository's
`.claude/settings.json` without removing what they say. Its settings
schema gives the two keys this design's examples use:
`tui:()=>U(["default","fullscreen"])`, "Terminal UI renderer.
"fullscreen" uses the flicker-free alt-screen renderer with virtualized
scrollback", and `statusLine:()=>u({type:C("command"),command:o(),padding:v().optional(),…})`.
It also holds keys that are other settings' or the account's: `env`,
`model`, `permissions.defaultMode`, the credential helpers
`apiKeyHelper`, `awsAuthRefresh`, `awsCredentialExport`,
`gcpAuthRefresh` and `otelHeadersHelper`, the sign-in keys
`forceLoginMethod` and `forceLoginOrgUUID`, `disableAllHooks`, and two
that record a person's acceptance of a warning,
`skipDangerousModePermissionPrompt` ("Whether the user has accepted the
bypass permissions mode dialog") and `skipAutoPermissionPrompt`.

The user, 2026-11-02, gave the change its first use beyond the pane:
what an agent's commits and pull requests say about who made them must
be configurable where it makes sense, starting with the tagline,
whether a commit or a pull request says `Generated with` or
`Co-Authored-By` anything, and in what words. An upstream project
refused the user's pull requests over Claude Code's `Generated with
Claude Code` line. A fork of someone else's project is to carry none
while the user's own repositories keep theirs, each repository's word
given by its `.config/flume.toml` or by the host's
`[repos."<repository>"]` table. The pinned release's settings schema
gives that word as `attribution`: `commit`, "Attribution text for git
commits, including any trailers. Empty string hides attribution.";
`pr`, "Attribution text for pull request descriptions. Empty string
hides attribution."; and `sessionUrl`, whether a session started from
the web or by Remote Control appends its link. `false` in its place
hides all three, and the deprecated `includeCoAuthoredBy` is read only
where `attribution` gives neither text. Where `attribution` gives one
text and not the other, the other is the release's own,
`Co-Authored-By: <model> <noreply@anthropic.com>` for a commit and
`🤖 Generated with [Claude Code](…)` for a pull request. Codex 0.165.0
adds `Co-authored-by: Codex <noreply@openai.com>` and `Generated with
[Codex](…)` by instructions of its own.

Other work meets this change. Record 0033, as the user ruled on
2026-11-05, declares a host's MCP servers in its managed layer, under
`[mcp.<server>]` with the labels whose places may have them, and hands
each agent its place's share at its start (183): servers are a table of
the host's, not own settings. Record 0032 gives the host's
configuration a managed layer, `config.toml`, and a local one,
`config.local.toml`, read over it key by key, the local one setting
only the keys C.7 lists, a kind's agents table among them. Record 0038
gives `Agent::command` the place's own folder, and Claude Code's
profile writes the standing prompt to `standing-prompt.md` beside the
start's record, named by `--append-system-prompt-file`. The change
`every-agent-shows-its-place-in-its-pane`, approved on 2026-11-05,
passes Claude Code `--settings` with flume's own `statusLine`, whose
hook prints the place's line above the session's own status line.
Design 17 says what each takes from this one. Records 0032 and 0033
come with the change `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer`,
and record 0038 with the fix `every-stages-agent-starts-whatever-its-arguments-hold`;
whichever of them lands before this change's group touching the same
code is merged into it there.

The rules this design relies on are core.1 and core.2 (only an agent
profile names its program, its flags or its keys; the core names
`programs` and a program's name as data alone), core.4 (`programs` is
read from the agents table weir returns), core.8 (the launch checks
every layer before anything is made), core.10 (a start's record
compares what its profile reads back, and own settings are not read
back), core.12 (flume never accepts a warning, so the keys that record
an acceptance never reach the program), core.14 (the JSON form only
gains fields), core.16 (the start's file is in the place's own folder,
not the repository's content), core.17 (writing it again is the same
act), core.22 and core.39 (each profile's way and reserved keys are
flume's own code, checked against the release it pins), core.30 (an
agent's settings come from clause 32's layers alone, its environment
from its account alone, and status names each setting's layer), core.34
(a kind's file is a part of its package, and a place's kind is not read
again after its launch) and core.35 (the site's neutral voice). It
departs from core.30 in two places. core.30 rules out "settings given by
a stream", which decision 3 answers. Its details also rule out `~` read
anywhere but in an account's `env`, and a host's file path's leading `~`
is read as the host already reads a repository's key, which decision 2
answers. The first task group amends core.30 with the record that
decides both (core.19). The one rules page is `docs/architecture/core.md`;
this change touches no area another page would own. Where flume has no
rule, the kit standard's code profile binds (kit:specs.3, kit:testing.5).

## Goals / Non-Goals

**Goals:**

- A host, a stream, a repository, a kind or a launch gives an agent
  program's own settings in that program's own words, inline, from a
  settings file it names, or both, and they reach that program and no
  other (195).
- A host names the file its people's sessions get, and its agents get
  the same keys, all but those another setting or a person owns (195).
- A host says once what every agent gets, and a stream says what its
  agents get differently (32, 143).
- A host that says nothing changes nothing: nothing is passed, and the
  account's own configuration decides (162, 195).
- No own setting carries a credential, an environment variable, a
  model, a mode or a person's acceptance past the setting that owns it
  (29, 34, 144, 162, 172; core.12, core.30).
- Every refusal names the key, the program, the stage, the layer and
  the file it came from, and never a value (4, 32).
- A repository says once, for every kind run in it, what its agents'
  commits and pull requests say about who made them, by its own
  `.config/flume.toml` or by the host's table for it, so a fork of
  someone else's project carries no tagline while the host's own
  repositories keep theirs (design 16).

**Non-Goals:**

- The MCP servers an agent gets. The research place decides them, as a
  setting beside this one (design 17).
- flume's own status line. The held change builds it on this change's
  file (design 17).
- A word of flume's for attribution. A program's tagline is its own
  key, Claude Code's `attribution` and whatever Codex's release names,
  given under its program's name as every own setting is. A word of
  flume's that each profile maps, as a mode is, waits for a second
  program whose entry takes own settings (design 16).
- A host's defaults given by a control plane. A future control plane
  that gives a host its default attribution, or any default, writes
  the host's configuration as any configurer does; flume reads that
  configuration alone.
- A commit's signature, author and committer. The host's git and the
  merge queue own them: git signs and names the author as the host's
  configuration says, sluice as its own does, and no layer of flume's
  gives either.
- What the merge queue writes. A landing's commit and anything the
  merge queue writes when it submits a place are the merge queue's own
  words (core.6).
- Every other program's way. Codex, Gemini CLI, Copilot CLI and
  OpenCode run through record 0026's common profile, which is not built;
  their entries take own settings once their way is checked against
  their release (design 10). Until then every program but Claude Code
  refuses own settings given for it.
- Reading own settings back from the running program. No pinned release
  reports the settings it loaded to a hook, so the start's record names
  none and status says they are not read back.
- Translating a setting from one program's words to another's, or
  merging inside a value (195; core.50).

## Decisions

### 1. A program's entry: a file and its own keys

Every agents table may give `programs`, a table whose keys are programs'
names. Each entry is flume's, and says where that program's own
settings come from in that table: `file`, the path of a settings file in
the program's own words, and `settings`, keys given inline as the
program spells them. An entry gives either or both.

```toml
[agents.programs.claude]
file = "~/.config/agent-launcher/claude-settings.json"

[agents.programs.claude.settings]
tui = "fullscreen"
statusLine = { type = "command", command = "~/.config/flume/status-line.sh" }
```

Any other key of an entry fails its table, naming the key and saying a
program's own settings go under `settings`. The program's keys sit one
level down, so no key a program has or gains collides with `file`, and
a word flume later gives an entry has room beside the two.

There is no bare form, unlike a mode or a model, since no own setting
means the same on two programs. A program's name is the last component
of the program the settings name, as `program_name` reads it for a
mode's table (169). The core takes only the entry of the agent's
program for the agent, so a Codex setting given beside a Claude Code one
never reaches Claude Code. An entry for a program no stage runs is
checked (design 7) and passed to nobody, as a mode's table's entry for
another program is.

`programs` is the key because it names what the table holds, programs'
settings by program, and keeps clear of the words an agents table
already holds. A program's name directly under `agents` would read as
well, but would collide with any setting flume adds later and leave a
misspelt key silently ignored as the runner's.

The values are kept as JSON values, `serde_json::Value`, since held
state and the launch's form are JSON. TOML's strings, numbers,
booleans, arrays and tables read into them one for one; a TOML date or
time, which JSON cannot hold, fails naming the key, to be given as a
string. A profile whose program reads TOML writes the values back as
TOML (design 10).

### 2. A named file, read where its layer lives

`file` names one file whose top level is a table of the program's own
settings, read as JSON where its name ends `.json` and as TOML where it
ends `.toml`. Any other name, a file that does not parse, and a top
level that is not a table fail the table, naming the file. The file's
top-level keys are the table's own settings beneath its `settings`: a
key `settings` gives is taken from `settings` whole, and every other key
as the file gives it.

```toml
# The host's default for every agent: the file its people's sessions get,
# which gives tui = "fullscreen", a statusLine and more, with a status
# line of the host's own over the file's.
[agents.programs.claude]
file = "~/.config/agent-launcher/claude-settings.json"
settings = { statusLine = { type = "command", command = "~/.config/flume/status-line.sh" } }
```

An agent on this host gets `tui` from the file and `statusLine` from
`settings`. Within a layer that names stages, a stage's table, its
`settings` over its file, comes before the table for every stage, its
`settings` over its file, as a stage's own comes first today (32), and
in a repository's two layers that comes before the table for every
kind, its `settings` over its file (design 3). So a stage's file
outranks every stage's inline keys: the stage's table is the more
specific, and the file is the coarser half of each table.

A path is read where its layer lives, when its layer is read:

| layer | `file` names | read |
|---|---|---|
| the host's, each of its tables | a file on the host: absolute, beginning `~/` read against the host user's home as a repository's key is, or relative to the configuration's own directory, `$XDG_CONFIG_HOME/flume/` | at each start, with the host's configuration (C.7) |
| the repository's, for a kind it does not declare and for every kind | a file of the repository, relative to `.config/flume.toml`'s directory | at the launch, at the revision the launch read, as a prompt is (21) |
| the kind's declaration | a file of its package, relative to its declaration's file | at the launch, beside it as its prompts are: at the revision for a repository's kind, on disk for a launch's (157) |
| the launch's | a file on the host, which the command line names relative to its working directory or absolutely, and the JSON form absolutely | at the launch |

A repository's or a kind's file outside its repository or its package,
by `..`, an absolute path or a link, fails its declaration (core.34). A
host's file may be a link, read where it leads, so a host whose people's
file moves at each rebuild names a link its configurer keeps. No `$VAR`
is read in a path (core.30).

A file read at the launch is kept, once checked (design 7), with the
launch's settings or the place's kind: its path and its keys as read.
It is not read again, since the place's kind and the repository's
settings are what the launch read (21, 32). A host's file is read again
at each start, as the host's configuration is, so a host that changes
the file its people get changes its next agent's.

flume never passes a named file to its program as it stands. It passes
the keys the layers resolved (design 9). A key the file gives that
another setting owns never reaches the program, and a layer's own keys
lie over the file's.

### 3. Eight layers: the host's for every agent and for a stream, and a repository's for every kind

Clause 32's layers become eight, most specific first:

1. the launch's settings;
2. the host's, for the place's repository: for its kind,
   `[repos."<repository>".kinds.<kind>.agents]`, then for every kind,
   `[repos."<repository>".agents]`;
3. the host's, for the kind;
4. the host's, for the place's stream, `[agents.streams.<stream>]`;
5. the host's, for every agent, `[agents]`;
6. the repository's: for a kind it does not declare,
   `[kinds.<kind>.agents]`, then for every kind, `[agents]`;
7. the kind's declaration;
8. flume's default.

Within every layer that names stages, a stage's own comes first, then
the table for every stage of the kind, then, in the two layers of a
repository, the table for every kind.

**A repository's word for every kind.** Whether a project takes an
agent's tagline is the project's rule (design 16), the same for every
kind and every stage that works in it and different between two
repositories on one host. `[agents]` cannot say it, and a kind's table
would have to be repeated for every kind, missing a kind added later.
So each of the two layers of a repository gains a table for every
kind, beneath its tables for one kind as a kind's table for every
stage lies beneath a stage's own. Status names their layers as it names
the kind tables', `host-repository` and `repository`, since each is
that layer's own word: the host's for a repository, and the repository's
for itself. The layers stay eight, and `host-repository` is the host's
word for the repository.

The host's `[repos."<repository>".agents]` comes before the host's for
the kind. It names where the work lands, whose rules a fork's upstream
sets, and the kind's table names the work for every repository. It
gives what the host's kind tables give, and `programs`, and names no
stage and no stream. It is how a host speaks for a repository it may
not write to, a fork of someone else's project among them, whose own
`.config/flume.toml` would travel upstream in the pull request.

The repository's `[agents]` is flume's own table of
`.config/flume.toml`, beside `[profiles]` and `[prompt]`: the
repository's and no part of any kind, which weir returns among the
sections it does not name, so flume reads it as it reads `[profiles]`
and the kind format is not touched (23; core.4). It is read with the
kinds at the revision the launch reads them, kept as the launch read
it, and travels with the repository to every host that runs it (20,
21). It gives what the repository's kind tables give, `program` among
them, and `programs`; it names no stage, no stream, no `env` and no
`beside`. A launch's declaration and a package's `kind.toml` are not
read for it, as they are not for `[profiles]`.

It applies to every kind the repository runs, its own among them, and
so comes before a kind the repository declares, as the repository's
layer comes before the kind's. A repository whose own kind should
differ on a key leaves that key out of `[agents]` and gives it in each
kind's table.

`[streams.<stream>]` already holds `agents` as its bound on agents
running, so a stream's settings cannot be `[streams.<stream>.agents]`.
They are a sub-table of the host's own: `[agents]` gives every agent on
the host its settings, and `[agents.streams.<stream>]` narrows it to
one stream, as `.stages.<stage>` narrows a kind's table to one stage.
The stream keeps to being a share of the host's room and a pool, and
the host's settings stay in one place.

```toml
[agents.programs.claude]
file = "~/.config/agent-launcher/claude-settings.json"

[agents.streams.personal.programs.claude.settings]
statusLine = { type = "command", command = "~/.config/flume/status-line-personal.sh" }
tui = "default"
```

Neither table names a stage, since a stage's name is its kind's: a
`stages` table under either fails the configuration, naming it and
`[kinds.<kind>.agents.stages.<stage>]` as where a stage's own settings
go, and so does one under either table for every kind of a repository. `[agents.streams.<stream>]` naming a stream the host does not
declare fails the configuration naming it, as a stream's pool that no
`[pools]` table declares does; `default` always stands. Neither gives
`program`, since the host chooses a program by the accounts it pools
(record 0020), nor `env`, which is an account's alone (144), nor
`beside`, which is the launch's and the stage's alone (107).

The stream's table comes after the host's for a repository and for a
kind, and before the host's for every agent. The repository and kind
tables name the work and where it lands, the stream's names a share of
the host the work runs in, and `[agents]` names nothing. A host that wants a
stream's word to stand for a kind leaves that key out of its kind
table.

Both come before the repository's and the kind's, as every host table
does today. A host that wants the last word on its agents states it in
its own layers (record 0024), and a stream whose model or status line
the host chose for its pool's accounts would otherwise lose to any kind
that names one. The cost is that a kind's own status line never shows
on a host that gives one, and a repository's own `[agents]` never
overrides a host's `[agents]` or kind table; the user ruled this
order on 2026-11-05. Under it a host that gives a default lets one
repository differ by `[repos."<repository>".agents]`.

A stream's settings amend 143, which said "a stream gives its agents no
settings (32)", and core.30's details, which ruled out "settings given
by a stream (record 0020, as amended 2026-10-29)". Record 0020 kept a stream
from giving settings because "a stream's word on a kind's model would
be a third host table for what the host's word for the kind already
says". It does not hold for own settings: what a host's agents show and
do differs by the share of the host they run in, which no kind table
can say. Nor does it hold for the model as it did. The kind table says
its word for every stream and the stream's for every kind, and a stream
is where an allowance is spent, through its pool, which is why the
user ruled that the new tables give the model, the mode and the
instructions too. Record 0043 amends record 0020, 143 and core.30
together (core.19).

**The managed and local layers.** The host's configuration is read as
record 0032 reads it: its local layer, `config.local.toml`, over its
managed one, `config.toml`, key by key, before the eight layers are
folded. Every agents table of the host's, the three new ones among
them, is one the local layer may set, as a kind's agents table is, and
the managed layer may pin any key of one. Each own setting,
`programs.<program>.settings.<key>`, and each
`programs.<program>.file`, is one key of that fold, so a local layer
replaces one own setting, or names another file, without restating the
managed layer's other keys. A named file in either layer is read
relative to the configuration's directory, the one both files share.
Record 0032 put the agents tables in the local layer because they say
how the host's work runs this hour, a stream's model or mode among
them; a host's status line, renderer or tagline is the same kind of
word, and the managed layer pins what it means to keep.

`Layer` gains `HostStream`, named `host-stream` in status, and `Host`,
named `host`. `HostRepository` and `Repository` keep their names and
gain the tables for every kind.

### 4. Key by key, each key taken whole

Each top-level key of a program's own settings is one setting, taken
whole from the first layer that gives it, and within a layer from its
`settings` before its file (design 2). The core folds them as it folds
every setting: for each program and each key, the more specific
source's value where it gives one and the more general one's otherwise.
`from` records `programs.<program>.settings.<key>` with its layer, the
file where the key came from one, and its value as written.

So with the host's file above and the personal stream's table, an agent
in `personal` gets the stream's `statusLine` and `tui = "default"`, and
the file's `hooks`, `outputStyle` and the rest. An agent in any other
stream gets the file's `tui = "fullscreen"` and its status line. The
other two ways were weighed.

- **The whole entry from one layer**, as a mode's table is taken. Every
  layer that gives any key of a program would drop every key the layers
  beneath gave. A stream giving one key would have to restate the
  host's file, and a kind giving a permission would silently drop the
  host's `tui`. A host's default would hold only where nothing else
  spoke.
- **A deep merge**, tables merged at every depth. Two layers' status
  lines would merge into one object neither wrote, a command from one
  and a padding from the other, and two lists would join in an order
  the program does not define. That is flume deciding what a program's
  value means, which it does not (core.50). A deep merge also leaves a
  layer no way to replace a table it means to replace, a hook list or
  a permission list, since a key it leaves out would show through from
  beneath.

A top-level key is the coarsest unit that keeps one layer's value as
that layer wrote it and lets a layer replace it whole. Status names one
source per key. A layer that changes part of a key's value restates the
key; the docs say so with an example. No layer removes a key a later
layer gives, since TOML has no null; a layer replaces it. A host that
names its people's file but wants none of its hooks for its agents
gives `settings = { hooks = {} }`, which replaces the file's `hooks`
whole.

### 5. The account's configuration underneath, and nothing where nothing is given

flume passes the agent's program the keys the layers resolved and
nothing else, and the program lays them over its account's own
configuration as it lays its own sources. For Claude Code that is the
release's deep merge of `flagSettings` over the user's, the project's
and the local settings (design 9): a key flume passes wins where it is
a value, merges where it is an object, and joins where it is a list,
and every key flume does not pass is the account's or the repository's
as today. That merge is the program's, read from its release, never
flume's own.

Where no layer gives an entry for the agent's program, or the entries
resolve to no key, the profile is given nothing, passes nothing, and
the start is as it is today: no `--settings`, no file written. The
account's own configuration decides, as it does for a mode no layer
gives (162).

### 6. Keys another setting owns: refused where written, left out of a file

Each agent profile names the keys of its program's settings that
another of flume's settings or the account owns, each with its owner. A
nested key is named by its path, `permissions.defaultMode`. For Claude
Code 2.1.296:

| keys | owned by |
|---|---|
| `env` | the account, whose `env` is an agent's whole environment (29, 144; core.30) |
| `model` | flume's `model` (172) |
| `permissions.defaultMode` | flume's `mode` (162) |
| `apiKeyHelper`, `awsAuthRefresh`, `awsCredentialExport`, `gcpAuthRefresh`, `otelHeadersHelper`, `forceLoginMethod`, `forceLoginOrgUUID` | the account's sign-in (144) |
| `disableAllHooks` | flume's plugin, whose hooks are the start's ack, the guard and the stop check (35, 115, 116, C.4) |
| `skipDangerousModePermissionPrompt`, `skipAutoPermissionPrompt` | a person, whose acceptance of a warning flume never gives (34; core.12) |

Without the first three, status would name a model, a mode or an
environment from one layer while the program ran another, which
core.30 rules out. Without the sign-in keys, a layer would choose an
agent's account behind its pool. Without `disableAllHooks`, a layer
would turn off the hooks the start's ack is read from, and every start
would be reported never begun with no word of why. Without the two
acceptances, a layer would answer, for a person, the warning Claude
Code shows before a mode that widens what an agent may do.

The core applies the profile's list one way for every program:

- **Written in a table's `settings`**, a reserved key refuses the
  launch, or at a later start the stage, naming the key, the program,
  the layer and its owner. The host, the repository or the kind wrote it
  for flume's agents, and it belongs in the setting that owns it.
- **Given by a named file**, a reserved key is left out, unread, and
  status names it with its file, its layer and its owner (design 12). A
  named file is a program's settings as a person's sessions get them,
  which hold a person's environment, mode and acceptances, and a host
  names it whole for the rest. `permissions.defaultMode` is left out of
  `permissions`, and the rest of `permissions` is kept as the file gives
  it. That is the one change flume makes inside a value, and only to
  take out a key its profile reserves. Refusing instead would keep the
  user's own file from ever being named, since it gives three reserved
  keys.

On the user's host's file, `env`, `permissions.defaultMode` and
`skipDangerousModePermissionPrompt` are left out, and `statusLine`,
`tui`, `hooks`, `permissions.allow` and the rest are taken. The
environment the people's sessions get from that file's `env` reaches an
agent only through its account's `env` in flume's configuration, where
status names each variable and never its value.

`statusLine`, `tui` and `hooks` are not reserved. A status line and the
renderer are the cases that prompted the change. Hooks are the
account's to give today, and a hook of the layers runs beside flume's
plugin's, as one of the account's does. Claude Code's MCP keys,
`enableAllProjectMcpServers`, `enabledMcpjsonServers` and
`disabledMcpjsonServers`, stay open until the MCP setting stands
(design 17).

The reserved keys are code in the profile, beside its map of modes,
and a `check-tools` test finds each in the pinned release's settings
schema (design 9; core.22, core.39). A key the release drops fails the
gate at the bump.

### 7. No credential: checked once the reserved keys are out, before anything is held

A credential belongs in an account's `env`, which status names and
never prints (core.30). Every string of an own setting, at any depth,
inline or from a file, is checked once the profile's reserved keys are
out of the named files, so a token in a person's `env` is never read.
The check comes before anything is held, passed or printed.

The core checks where it first takes a layer. At the launch it checks
every entry of every layer, each program's against its own profile's
reserved keys, or against none where no profile binds the program,
before anything is made (core.8). At each later start it checks the
host's layers, read as they stand then. A layer's reader keeps what it
read in memory alone until the core has checked it. Nothing unchecked is
written to held state, a log or an answer.

A string is refused, naming its key's path, its layer and its file where
it came from one, and never its value, where it:

- opens with a prefix an issuer gives its tokens, followed by sixteen or
  more letters, digits, `_` or `-`: `sk-`, `ghp_`, `gho_`, `ghu_`,
  `ghs_`, `ghr_`, `github_pat_`, `glpat-`, `xoxb-`, `xoxp-`, `xoxa-`,
  `xapp-`, `AKIA`, `ASIA`, `AIza`, `ya29.`, `npm_`, `pypi-`, `hf_`,
  `sk_live_` and `rk_live_`;
- holds a private key's opening, `-----BEGIN` and `PRIVATE KEY-----`;
- is a JSON web token, three runs of base64url joined by dots, the
  first opening `eyJ`;
- holds, apart from any `$VAR` or `${VAR}` in it, a run of 32 or more
  letters, digits, `_` and `-` that mixes upper case, lower case and
  digits;
- is the value of a key whose name, in lower case with `_` and `-`
  taken out, is or ends with `token`, `secret`, `password`, `passwd`,
  `apikey`, `privatekey`, `accesskey`, `clientsecret`, `authorization`
  or `credentials`, and is not empty and names no variable by `$VAR`
  or `${VAR}`.

The refusal says that a credential goes in the account's `env` and that
the program reads it from its environment. It refuses the launch, or at
a later start the stage, until a retry (4, 27, 32).

The check is the core's, program-neutral, in `crates/flume-core/src/own.rs`
beside the fold, so every layer passes one rule. It rests on shapes, not
meaning. A credential in no known shape, a short password, passes it,
which is why status and the refusals print values only of own settings
that passed and why the docs say where a credential goes. A long mixed
identifier in a command is the false refusal it risks; the refusal
names the key so the host can move the value into a script the command
runs. Every string of the user's host's file passes, its store paths'
digests being lower case.

flume reads nothing else inside a value. It reads no `~` and no `$VAR`
in one, since core.30 keeps both to an account's `env`; a status line's
command reaches the program as written and its shell reads it there.

### 8. The agent seam

The agent seam gains the part **own settings** (196):

- `Agent::reserved(&self) -> Vec<Reserved>`: each key of its program's
  settings that another setting or the account owns, by its path, with
  its owner's words; none by default. The core refuses each where a
  table's `settings` gives it and leaves it out of a named file
  (design 6).
- `Agent::own(&self, given: &BTreeMap<String, Value>) -> Result<(), String>`
  says whether the profile can pass `given`, its program's keys as the
  layers resolved them: `Ok`, or words saying its program gives no way.
  By default every non-empty entry is refused, "its agent profile passes
  no settings of its program's own", as a profile that maps no mode
  refuses one (170).
- `Agent::command` gains `own: &BTreeMap<String, Value>`, the agent's
  resolved keys alone, which the core takes by the program's name and
  never hands another program's, and `folder: &str`, the start's folder
  in the place's own folder.
- `AgentCommand` gains `files: BTreeMap<String, String>`, each an
  absolute path under that folder with its text. The core writes each
  before the terminal opens the agent, written beside and renamed over,
  readable by the host user alone, and refuses the start as not opened
  where a path is outside the folder. The printed command's line names
  the file as any argument (C.5). It goes with the place's folder at
  its tidy (62).
- The store seam gains `Store::write_file(place, path, text)`, clause
  128's file in the place's folder, as blueprints 352aa32 amends it on
  the branch `design/flume-the-store-writes-a-starts-files`, through
  which the core writes them: the place's folder is the store's, and the
  core reaches nothing on the host but through a seam (core.2). The core
  checks every path is under the folder before it writes any, and the
  store refuses one outside it too. Writing them from the core with the
  standard library was the other way, and would put the one file write
  of the core's outside every seam.

`Engine::resolution` asks `Agent::own` once the program is bound and
the mode mapped, where the agent's resolved keys are not empty, and
refuses in design 13's words.

An account of a pool is usable for a stage whose layers name no
program only where its program's profile takes the stage's keys for it
(173), so a pool of two programs starts the stage on the one that can.
Record 0026's change builds that with the rest of 173, an account's
program naming the stage's where there is a pool, and carries its
scenario, "A pool of two programs": until then flume's default layer
names a program for every stage, so no stage here has layers naming
none, and the clause list names 173 unbuilt by that change
(`clauses::PROFILES`). Its filter is the account's program's
`Resolved::own_for` refused by nothing (`settings::own_refused`).

The start's record names no own settings and compares none: no
release reads them back, so `Agent::own_read_back` is not added, and
status says they were not read back. A start brought back without them
is a start without the `FLUME_*` variables too, which acks nothing and
is never the stage's (77; core.10).

### 9. Claude Code takes them by `--settings <file>`

Where the agent's resolved keys for `claude` are not empty, the
profile's command writes them as one JSON object, its keys in order, to
`<folder>/claude/<identity>/own-settings.json` beside the start's
record, and passes `--settings <that path>` after `--plugin-dir`. Where
they are empty, it passes no `--settings` and writes nothing. The file
holds each key from its layer's `settings` or its file and never a key
left out. It is never a named file, which the profile does not pass.

A file rather than a JSON string, because the layers' values can be
long and hold the shell's own quoting, a hook's command or a status
line's, which a JSON string would carry through herdr's typed line and
the printed command a second time. A file also holds exactly what the
program read, for a person to look at, and is where the held change's
hook reads the layers' status line (design 17).

`claude/settings.rs` holds the reserved keys with their owners, the
file's name, and `fragments()`, which a `check-tools` test in
`crates/flume/tests/profiles/tools/claude_words.rs` requires byte for
byte in the pinned release (C.4; core.22):

- `--settings <file-or-json>` and its words, an option of one value;
- the sources' order,
  `return["userSettings","projectSettings","localSettings","flagSettings","policySettings"]`;
- the merge that joins a source's lists with those beneath it,
  `return D([...e,...n])`, and takes the later source's value
  otherwise, by the settings merge's branch for two lists,
  `if(Array.isArray(e)&&Array.isArray(n)){if(s==="fallbackModel")return n;return D([...e,...n])}`,
  since 2.1.296 holds `return D([...e,...n])` once more outside it;
- each reserved key as the settings schema names it, `apiKeyHelper:`,
  `disableAllHooks:`, `skipDangerousModePermissionPrompt:` and the
  rest, each by its declaration through the opening of its
  description,
  `apiKeyHelper:()=>o().optional().describe("Path to a script that outputs authentication values")`,
  since `model:` stands elsewhere in the release too;
- the attribution the docs give as the worked example (design 16), by
  its schema's two texts, `Attribution text for git commits, including
  any trailers. Empty string hides attribution.` and `Attribution text
  for pull request descriptions. Empty string hides attribution.`

A bump that renames the option, moves the flag's source below the
user's, drops a reserved key, or changes what an empty attribution
means fails the gate until the profile and the docs are read again from
the new release. `attribution` is not a reserved key and is named
nowhere else in the profile: the fragment pins the docs' example, not a
key flume treats apart.

### 10. Every other program's way

Each program's way is written into Part C when its entry is built, and
checked against the release it pins (core.39). What each release on the
host offers, read from its `--help` and its package on 2026-11-01:

| program | how it takes its own settings | beneath them |
|---|---|---|
| Codex 0.165.0 | `-c <dotted.path>=<value>` for each value at a leaf, its value TOML | `$CODEX_HOME/config.toml` and the project's `.codex/config.toml`, each leaf flume does not pass |
| Gemini CLI 0.66.0 | `GEMINI_CLI_SYSTEM_SETTINGS_PATH` naming a file flume writes | the user's and the workspace's settings; but the file stands in for the host's own system settings file, so its entry offers the part only once that is decided |
| GitHub Copilot CLI 1.0.104 | none for one run: `copilot config` writes the account's own file | refused |
| OpenCode 1.19.4 | `OPENCODE_CONFIG_CONTENT`, its JSON config | its global and project configuration, merged |

Codex passes a leaf by its path so the account's other keys in the same
table stay beneath it. That is Codex's own override at a leaf, not a
merge of flume's, and the layers still give the top-level key whole. A
Codex file a layer names is TOML, read as design 2 reads one, and its
keys are passed as resolved, never as the file. OpenCode's variable is
set by its profile's command as the `FLUME_*` variables are, and
recorded by digest in the start's record (161). It is flume's way of
starting the program, not a variable a layer gives, which core.30's
details are amended to say. Until each entry is built, `Agent::own`'s
default refuses, so a host that gives `programs.codex` for a stage
running Codex sees its launch refused naming the program and the key.

### 11. Read in every layer

One reader, `crates/flume/src/own.rs` beside `mode.rs`, takes an agents
table's `programs`: a table of entries, each with `file`, a string, and
`settings`, a table, read into JSON values. It reads the file through a
function its caller gives, as the kinds profile reads a prompt, and
fails naming `<table>.programs`, the entry's key or the file where any
part is not as design 1 and 2 say. It runs no credential check, which is
the core's (design 7).

- The host's `agent_settings` calls it for `[kinds.<kind>.agents]`,
  `[repos."<repository>".kinds.<kind>.agents]`, their `.stages.<stage>`,
  `[repos."<repository>".agents]`, `[agents]` and
  `[agents.streams.<stream>]`, reading a file by design 2's host paths.
  `HostConfig` gains `every_agent: Settings`, `streams_agents:
  BTreeMap<String, Settings>` and `repos_agents: BTreeMap<String,
  Settings>`, the last by the repository's location as `repos` is.
  `HostSettings` gains the settings for the place's repository for
  every kind, for the place's stream, which its held state names, and
  for every agent, which `Engine::layers` reads at each start.
- The kinds profile's `map_agent` calls it for every agents table of a
  declaration, a kind's, a stage's and the repository's for a kind it
  does not declare, reading a file by the function `read` is given for
  its prompts, so it is read at the same revision or on disk and never
  outside the repository (157; core.34). It pushes a problem naming the
  field where it fails (4, 23, C.2). Weir's C.4 names `programs` in an
  agents table, the reader's to check, as it names `mode`, so weir-kinds
  returns it as written and flume's pin stays (core.4).
- The kinds profile reads a repository's `[agents]` from the sections
  weir returns unnamed, at the revision, with `map_agent` and the same
  file reader, refusing `stages`, `streams`, `env` and `beside` there
  naming the field. `read_kinds` returns it as `KindsRead::Read`'s
  `agents`, and the launch keeps it in the place's held state as
  `repository_agents`, beside `repository_settings`. `read_declaration`
  and the packages do not read it.
- `--agent [<stage>.]programs.<program>.settings.<key>=<value>` gives
  one key, its value read as JSON where it parses as JSON and as the
  text written otherwise, so `tui=fullscreen` is a string and
  `statusLine={"type":"command","command":"line"}` a table. Several
  build one entry; one key given twice for the same stage refuses the
  launch naming it. `--agent [<stage>.]programs.<program>.file=<path>`
  names a file, made absolute against the command's working directory
  and read at the launch. `programs` joins the settings every `--agent`
  refusal lists. The launch's JSON form takes `programs` as `Settings`
  serializes it, a file by its absolute path.

`Settings` gains `programs: BTreeMap<String, ProgramSettings>`.
`ProgramSettings` holds `file: Option<NamedFile>` and `settings:
BTreeMap<String, Value>`, and `NamedFile` the path as written, the path
read, its keys as read and `left_out`, the reserved keys' paths the
core took out. Each is read with a default and written only where it
holds something, so held state written before them reads with none, as
`KindsRead::Read`'s `agents` and the held `repository_agents` are.

### 12. Status and the JSON form

The JSON form only gains fields (core.14). An agent's settings give
`programs` with its program's entry alone. `from` gives each
`programs.<program>.settings.<key>` with its layer, its file where it
came from one, and its value as written (56). `AgentStatus` gains
`own: Option<OwnStatus>`, none where no layer gave the agent's program
an entry. It holds `passed`, how the profile passed them, `--settings
<path>` for Claude Code: the arguments of the start's command that name
a file the command lists, each with the argument before it, recorded on
the start when it is opened, none where it passed no key. A seam part
of its own for those words was the other way; the command already says
it, for every profile that passes a file. It also holds `read_back`,
false, and `files`: each file a layer named for the agent's program,
with its layer, its path as written and as read, and the keys left out
of it with their owners, the owners read from the profile when status
is asked.

The person's form prints, under the agent, each key on its own line
with its layer and its file. Its value is printed as JSON where the line
is a hundred characters or fewer, and as `{…}` or `[…]` otherwise, since
a people's file's hooks run long; the JSON form gives every value whole.
Then each file's keys left out with their owners, and "own settings not
read back".

The host's part of status lists every value of the host's configuration
as written, with its layer (core.45). An own setting a host table gives
inline whose value holds a string shaped as a credential is listed as
`<set>`, as an `env` table's values are: the launch and every start
refuse it, and no answer gives it before the check passes it (design 7;
core.50).

### 13. Refusals at the launch and at a later start

`Engine::launch` takes every layer before anything is made, with the
host's configuration as it stands then, as it does for a mode (4;
core.8). A layer's reader fails first, where a table or a file is not
as design 1 and 2 say: the host's configuration then refuses every
launch and start (C.7), a declaration cannot be read (4, C.2), and a
launch's `--agent` is refused. The core then checks every entry
(design 7), refusing a reserved key a table's `settings` gives and a
value shaped as a credential, and asks `Engine::resolution` of every
stage. A stage whose resolved keys its profile refuses is refused with
"the stage `<stage>`'s own settings for `<program>`, `<key>` from the
layer `<layer>`, are refused: `<words>`", naming the file after the
layer where the key came from one. At a later start, a host table or a
host's file changed after the launch leaves the stage refused in the
same words, starting no agent, until a retry (27, 32).

### 14. The stand-in and the walkthroughs

The stand-in for Claude Code (`crates/flume/tests/profiles/stand-in/claude.rs`)
reads `--settings <file-or-json>`, and merges it over the
`settings.json` of its `CLAUDE_CONFIG_DIR` and the place's
`.claude/settings.json` and `.claude/settings.local.json`, as the
release's sources merge. It writes the merged settings to the file
`STAND_IN_SETTINGS` names, so a walkthrough sees what the program read.
Its doc comment names this beside its acts.

Walkthroughs in `crates/flume/tests/profiles/tools/walkthroughs.rs`,
with the stand-in under the printed command:

- S100: a host whose `[agents.programs.claude]` names a scratch file,
  `people-settings.json`, as a host's launcher would pass its people.
  The file gives `tui = "fullscreen"`, a `statusLine` running
  `people-line`, `env = { PEOPLE = "1" }`, `permissions = { allow =
  ["Bash(git log:*)"], defaultMode = "bypassPermissions" }` and
  `skipDangerousModePermissionPrompt = true`. The host's `settings`
  gives a `statusLine` running `host-line` over it, and its
  `[agents.streams.work.programs.claude.settings]` a `statusLine`
  running `work-line`. A scratch repository's `.config/flume.toml`
  gives `[kinds.fix.agents.programs.claude] file = "claude-fix.json"`,
  whose `.config/claude-fix.json` gives `alwaysThinkingEnabled = false`.
  The account's own `settings.json` gives `cleanupPeriodDays`. A fix in
  `work`: its printed command holds `--settings` naming a file under the
  place's folder and not `people-settings.json`. The stand-in read
  `work-line`, the file's `tui`, the file's `permissions` without
  `defaultMode`, the repository's `alwaysThinkingEnabled` and the
  account's `cleanupPeriodDays`, and no `env` or
  `skipDangerousModePermissionPrompt` of the file's. Status names each
  key with `host-stream`, `host` with the file, and `repository` with
  its file, and names the three keys left out with their owners. A fix
  in `default` read `host-line` and the file's `tui`. A fix on a host
  and a repository that give no entry prints no `--settings`, writes no
  file, and lands.
- S101: launches refused, each naming the key and its layer, and the
  file where there is one, with no value in the refusal or in status,
  and nothing made: `--agent programs.claude.settings.env={"X":"1"}`; a
  host whose `[agents.programs.claude.settings]` gives `apiKeyHelper`; a
  repository whose `[kinds.fix.agents.programs.claude.settings]` gives
  `permissions = { defaultMode = "bypassPermissions" }`; a launch giving
  a status line whose command holds a value shaped as a token; a host
  file whose `statusLine` command holds one; a host whose file does not
  exist; and a repository whose `file` is `../../claude.json`, past the repository's root.
- S102: three scratch repositories on one host. The host's
  `[repos."<fork>".agents.programs.claude.settings]` gives `attribution
  = { commit = "", pr = "", sessionUrl = false }`, for a fork of a
  project that refuses a tagline; the repository `ours` commits a
  `.config/flume.toml` whose `[agents.programs.claude.settings]` gives
  `attribution = { commit = "Assisted-by: Claude Code", pr = "" }`; and
  `plain` gives none. A fix and a chore in `fork`: each stage's
  `--settings` file holds `attribution` exactly as the host wrote it,
  the stand-in read it, and status names
  `programs.claude.settings.attribution` from `host-repository`. A fix
  in `ours`: its file holds the repository's `attribution`, from
  `repository`. A fix in `plain` prints no `--settings`, and the
  stand-in read no `attribution`. Each lands.

### 15. Own settings reaching a real Claude Code agent

One walkthrough in `crates/flume/tests/profiles/live/walkthroughs.rs`
runs a fix under herdr with the pinned Claude Code, on a walkthrough
host whose `[agents.programs.claude]` names a scratch settings file and
whose `[repos."<the scratch repository>".agents.programs.claude.settings]`
gives `attribution = { commit = "", pr = "", sessionUrl = false }`.
The file gives `tui = "fullscreen"` and a `hooks` table with a
SessionStart hook that writes a file of the scratch directory's. The
hook's file is written once the session starts, which only the named
file's keys reaching the release through `--settings` could do, and the
code agent's started ack is taken as well, so the layers' hook ran
beside flume's plugin's. The commit the code agent made holds no
`Co-Authored-By` trailer, which the release's own text asks it to add.
Status names `programs.claude.settings.hooks` and
`programs.claude.settings.tui` from `host` with the file and
`programs.claude.settings.attribution` from `host-repository`, and the
place lands.

### 16. A worked example: what a commit says about who made it

Claude Code's `attribution` is one top-level key of its settings, so
it is given, resolved, checked and passed as every own setting is, and
nothing in the core, the readers or the profile's command names it.

A host whose fork of someone else's project must carry no tagline says
so for every kind and every stage run there:

```toml
[repos."~/Code/upstream-fork".agents.programs.claude.settings]
attribution = { commit = "", pr = "", sessionUrl = false }
```

A repository of the host's own that wants its own words says so in its
`.config/flume.toml`, for every kind run in it, and the words travel
with it to every host:

```toml
[agents.programs.claude.settings]
attribution = { commit = "Assisted-by: Claude Code", pr = "" }
```

A repository neither names keeps its account's attribution: no layer
gives the key, and Claude Code writes what the account's
`settings.json` says, else its own default (design 5).

Every agent of a place in the fork is started with `attribution` in its
`--settings` file as the host wrote it, and status names
`programs.claude.settings.attribution` from `host-repository`. The
release lays it over the account's settings, and with both texts empty
and no session link, nothing it asks of a commit or a pull request
names who made it.

The value is taken whole (design 4). A layer that gives `attribution =
{ commit = "" }` alone hides the commit's trailer and leaves the pull
request's text to the account's settings or the release's default,
since the release fills a text `attribution` leaves out. The docs say
to give both texts, and `sessionUrl = false`, for none. `attribution =
false`, which the 2.1.296 release reads as all three, passes as written
too, but older releases refuse it, so the docs show the table.

Why a repository's layers: whether a project takes a tagline is the
project's rule, the same for every kind and stage that works in it and
different between two repositories on one host (design 3). A fork's own
`.config/flume.toml` is no place for its upstream's rule, since a file
committed there would travel in the pull request the upstream reviews,
so a fork's word is the host's `[repos."<repository>".agents]`. Under
the order the user ruled, that table comes before every host
table but the repository and kind's, so the fork carries no tagline
whatever the host's default. A repository's own `[agents]` stands
where no host table gives the key.

Codex 0.165.0 adds its own trailer and pull request line by
instructions of its own (Context). Which key of its configuration, if
any, turns them off is read from its release when its entry is built
(design 10), and a host or a repository gives it under
`programs.codex` then, in the same tables. The other programs' releases
are read for theirs the same way.

`attribution` is credential-free text, and the check of design 7 reads
it as any value: `Co-Authored-By: Claude <noreply@anthropic.com>` and
`Assisted-by: Claude Code` pass it.

### 17. What the work beside this one takes from it

**The host's MCP servers** (record 0033, clause 183) are a table of the
managed layer, `[mcp.<server>]` with its `labels`, and each agent is
handed its place's share at its start. They are not own settings, and
their change takes from this one:

- **The command's files.** Claude Code's servers file, for
  `--mcp-config <file> --strict-mcp-config`, is written beside this
  change's file through `AgentCommand::files`.
- **The credential check.** Design 7's check applies to every server
  definition, so a server's credential is named by a variable or a
  helper and never by its value.
- **The reserved keys.** Each profile adds its program's own MCP keys
  to `Agent::reserved`: Claude Code's `enableAllProjectMcpServers`,
  `enabledMcpjsonServers` and `disabledMcpjsonServers`, Codex's
  `mcp_servers`, Gemini CLI's `mcpServers` and OpenCode's `mcp`. Then an
  agent's servers come from the host's table alone: such a key is
  refused where a layer writes it and left out of a named file, as
  `env` is. Until that change lands a host may give them as own
  settings, which is how a host lets an agent past Claude Code's
  approval of a repository's servers today.

**Record 0038's standing prompt** is a file of the start's own beside
its record, named by `--append-system-prompt-file`. Where both are
built, the start's folder is the one argument `Agent::command` takes
for both, and the standing prompt is one of the files `AgentCommand`
lists, so the core writes every file a command needs the one way.

**The status-line change** (`every-agent-shows-its-place-in-its-pane`)
should take:

- **One `--settings`.** The release reads one `--settings`. Its
  profile should put flume's `statusLine` into the file this change
  writes, at every start once it lands, rather than pass a JSON string
  of its own.
- **The layers' status line, kept.** flume's `statusLine` replaces the
  layers' `statusLine` key in the file. The layers' value, from a
  table's `settings` or a named file, is the session's own status line,
  which its hook runs beneath flume's line. It is taken first, ahead of
  the local, project and user settings, since the flag outranks them.
  The hook reads it from the start's resolved settings, never from a
  named file, so a host's status line, the one its people's file gives
  among them, is never lost to flume's.
- **Its design 4 and core.30 sentence.** "The JSON holds no setting of
  the six layers" no longer holds. The file holds the layers' own
  settings and flume's status line. Status names the layers'
  `statusLine` with its layer and its file, and says flume's line wraps
  it.
- **One reading of `--settings`** in the stand-in, a file or a JSON
  string, with this change's merge of the sources beneath it.
- **The shared fragments** of `--settings` and the sources' order,
  pinned once.

### 18. The statement, the record and the rules

The statement at blueprints 94b4edb, on the branch
`design/flume-an-agents-program-settings` with main at 1b0275f merged in,
holds this change (core.19):

- **195** (A.4), an agent's own settings: a table of programs in every
  layer of 32, each entry a named file and keys in its program's own
  words, the file read where its layer lives and its keys beneath the
  layer's own, each top-level key taken whole, only the agent's
  program's entry taken, nothing passed where nothing is given, a
  reserved key refused where written and left out of a named file, a
  value shaped as a credential refused before anything holds it, and
  status giving each key with its layer and its file.
- **196** (B.4), the agent seam's **own settings** part: a program's
  reserved keys with their owners, and how its program is started with
  the keys resolved, or that it gives no way.
- **32**: eight layers, as design 3 orders them, the host's tables for
  a stream and for every agent and a repository's two tables for every
  kind giving the model, the mode, the instructions and own settings,
  and the repository's `[agents]` a program as its kind tables do.
- **143**: a stream's agents take the settings the host's configuration
  gives for the stream.
- **158**: a kind's own settings and its file are parts of its package.
- **4**, **56**, **88**, **171** (an eleventh reading: own settings not
  offered), **173** (an account usable only where its program takes the
  stage's keys), section 3's **settings** and two new terms, **own
  settings** and **named file**.
- **C.2**: `programs` in every agents table and the repository's
  `[agents]` beside `[profiles]` and `[prompt]`; **C.4**: the command
  row's `--settings`, the reserved keys, `attribution` as the example,
  and the common profile's own-settings row; **C.7**: `[agents]`,
  `[agents.streams.<stream>]`, `[repos."<repository>".agents]`, the
  host's file paths, and every agents table of the host's one the local
  layer may set, each own setting and named file a key of its own.
- **S100** and **S101**, design 14's first two walkthroughs, and **S102**,
  the tagline across a fork and a repository of the host's own.
- **weir C.4**: `programs` in an agents table, the reader's to check.

Record 0043 states decisions 1 to 7 and 16 as the user ruled, and
amends records 0017, 0020 and 0024. **core.30**'s text reads "the
layers clause 32 orders"; its details rule out a program or a pool
chosen by any host table and the new tables' forbidden keys, and leave
open a stream's settings, a host's file path read with a leading `~/`,
and a profile's own variable for its program's settings. **core.50**,
in "Stages and agents", is decision 1 to 7's rule: a program's own
settings reach that program alone, each top-level key taken whole and
passed as written, never the named file as it stands; a reserved key
refused where written and left out of a file; a credential-shaped value
refused before anything holds it; and nothing passed where nothing is
given.

## The user's rulings

The user approved this design on 2026-11-05, with the recommended
answer to each of its two questions:

1. `model`, `mode` and `instructions` gain the new tables too: every
   setting but the program and the environment is given in every
   layer, the repository's `[agents]` giving a program as its kind
   tables do.
2. The host's new tables come before the repository's and the kind's,
   as every host table does (record 0024). A host lets one repository
   differ by `[repos."<repository>".agents]`.

## Risks / Trade-offs

- **A named file brings its people's hooks and plugins.** The user's
  host's file gives hooks on nine events and six plugins, which then
  run in every agent beside flume's plugin. A host that wants less gives
  the key in its `settings`, `hooks = {}` among them, which replaces
  the file's whole.
- **A layer's hook runs beside flume's plugin.** A `hooks` key can deny
  a tool or block a stop, as the account's own hooks can today. Of
  flume's needs, only the start's ack would be lost with hooks off,
  which `disableAllHooks` reserved keeps. A layer's hook that denies
  `flume needs` would keep an agent from stopping short, which status
  shows as a stuck stage.
- **A host's file changes between stages.** It is read at each start,
  so a place's stages may run under two versions of it, as they may
  under two versions of the host's configuration (record 0017).
- **The credential check rests on shapes.** It misses a credential in
  no known shape and may refuse a long mixed identifier. The refusal
  names the key so the host can move the value, and the docs say where
  a credential goes.
- **A key changed in part must be restated.** A stream that wants the
  host's permissions and one more rule gives the whole `permissions`
  key. The docs show it.
- **The host's word wins.** Under the order the user ruled, a
  kind that ships its own status line shows it only on a host that
  gives none.
- **Own settings are not read back.** A program that ignores a key, or
  a release that changes how it merges, shows no difference at the
  start. The `check-tools` fragments fail at the bump, and the live
  walkthrough proves a named file's keys reach the real program.
- **A tagline given in part.** `attribution = { commit = "" }` alone
  leaves the pull request's text to the account or the release's
  default, since the key is taken whole and the release fills what it
  leaves out. The docs give both texts and `sessionUrl = false` for
  none, and S102 tests that form.
- **A repository's `[agents]` comes before its own kind.** A
  repository whose own kind should differ from its `[agents]` on a key
  moves the key into each kind's table (design 3).
- **A host's word for a repository follows its location.**
  `[repos."<repository>"]` names a repository by its location on the
  host, so a fork cloned again elsewhere carries its upstream's rule
  only once its new location is named, as its stream does today.
- **Other changes touch the same code.** The status-line change adds
  to the command and the stand-in's `--settings`; record 0038's fix
  adds the start's folder to `Agent::command`; records 0032 and 0033's
  change reads the host's configuration in two layers and adds `label`
  and `session` to the host's tables. Whichever lands first is merged
  into the group of this change that touches the same code, and the
  stand-in reads one `--settings`.

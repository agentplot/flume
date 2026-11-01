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
tables, the kinds profile's `map_agent`
(`crates/flume/src/profiles/kinds/weir/mod.rs`) reads every agents table
of a declaration from the table weir-kinds returns as written, and
`commands/launch.rs` reads `--agent`. A stream (`[streams.<stream>]`)
holds `agents`, its bound, `priority` and `pool`, and gives no settings
(143; record 0020). The Claude Code profile's `command`
(`crates/flume/src/profiles/agent/claude/mod.rs`) passes `--plugin-dir`,
`--model`, `--permission-mode` and `--append-system-prompt`, with the
account's environment and the four `FLUME_*` variables. `profiles.rs`
binds Claude Code's profile alone; record 0026's common profile is not
built yet.

What prompted the change: a host's own launcher passes Claude Code a
settings file by `--settings`, so a session a person starts gets its
status line, its project servers' approval and the rest. An agent flume
starts runs with its account's configuration home alone and gets none
of it. The user asked for one generic way, for every program flume runs,
defaulted for the host and possibly different by stream.

The pinned Claude Code 2.1.296 bears it out. `--settings
<file-or-json>`, "Path to a settings JSON file or a JSON string to load
additional settings from", loads the `flagSettings` source, which the
release orders `["userSettings","projectSettings","localSettings","flagSettings","policySettings"]`.
It merges the sources as a deep merge: objects key by key, a list
joined with the lists beneath it and its duplicates dropped, and
anything else from the later source. So a key given by `--settings`
lays over the account's own `settings.json` and the repository's
`.claude/settings.json` without removing what they say. Its settings
schema also holds keys that are other settings' or the account's:
`env`, `model`, `permissions.defaultMode`, the credential helpers
`apiKeyHelper`, `awsAuthRefresh`, `awsCredentialExport`,
`gcpAuthRefresh` and `otelHeadersHelper`, the sign-in keys
`forceLoginMethod` and `forceLoginOrgUUID`, `disableAllHooks`, and two
that record a person's acceptance of a warning,
`skipDangerousModePermissionPrompt` ("Whether the user has accepted the
bypass permissions mode dialog") and `skipAutoPermissionPrompt`.

Two places run beside this one in the repository. The research
conversation `research--mcp-servers-for-flumes-agents` is converging, as
of its transcript on 2026-11-01, on an `mcp` setting of the same layers,
whose servers the host's configuration defines once and each profile
passes its own way (Claude Code by `--mcp-config <file>
--strict-mcp-config`). The held change `every-agent-shows-its-place-in-its-pane`
passes Claude Code `--settings` with a JSON string holding flume's own
`statusLine`, whose hook prints the place's line above the session's
own status line. Design 15 says what each takes from this one. A third,
`change--an-answer-reaches-the-open-session-within-an-hour`, adds
`keep_open` to the same `Settings` and host tables.

The rules this design relies on are core.1 and core.2 (only an agent
profile names its program, its flags or its keys; the core names
`programs` and a program's name as data alone), core.4 (`programs` is
read from the agents table weir returns), core.10 (a start's record
compares what its profile reads back, and own settings are not read
back), core.12 (flume never accepts a warning, so the keys that record
an acceptance are refused), core.14 (the JSON form only gains fields),
core.16 (the start's file is in the place's own folder, not the
repository's content), core.17 (writing it again is the same act),
core.22 and core.39 (each profile's way and reserved keys are flume's
own code, checked against the release it pins), core.30 (an agent's
settings come from clause 32's layers alone, its environment from its
account alone, and status names each setting's layer) and core.35 (the
site's neutral voice). It departs from core.30 in one place: core.30
rules out "settings given by a stream". Decision 2 says why, and the
first task group amends core.30 with the record that decides it
(core.19). The one rules page is `docs/architecture/core.md`; this
change touches no area another page would own. Where flume has no rule,
the kit standard's code profile binds (kit:specs.3, kit:testing.5).

## Goals / Non-Goals

**Goals:**

- A host, a stream, a repository, a kind or a launch gives an agent
  program's own settings in that program's own words, and they reach
  that program and no other (175).
- A host says once what every agent gets, and a stream says what its
  agents get differently (32, 143).
- A host that says nothing changes nothing: nothing is passed, and the
  account's own configuration decides (162, 175).
- No own setting carries a credential, an environment variable, a
  model, a mode or a person's acceptance past the setting that owns it
  (29, 34, 144, 162, 172; core.12, core.30).
- Every refusal names the key, the program, the stage and the layer,
  and never a value (4, 32).

**Non-Goals:**

- The MCP servers an agent gets. The research place decides them, as a
  setting beside this one (design 15).
- flume's own status line. The held change builds it on this change's
  file (design 15).
- Every other program's way. Codex, Gemini CLI, Copilot CLI and
  OpenCode run through record 0026's common profile, which is not built;
  their entries take own settings once their way is checked against
  their release (design 9). Until then every program but Claude Code
  refuses own settings given for it.
- Reading own settings back from the running program. No pinned release
  reports the settings it loaded to a hook, so the start's record names
  none and status says they are not read back.
- Translating a setting from one program's words to another's, or
  merging inside a value (175; core.43).

## Decisions

### 1. Own settings are a table of programs

Every agents table may give `programs`: a table whose keys are
programs' names and whose values are each that program's own settings,
as the program spells them.

```toml
[agents.programs.claude]
statusLine = { type = "command", command = "~/.config/flume/status-line.sh" }
```

There is no bare form, unlike a mode or a model, since no own setting
means the same on two programs. A program's name is the last component
of the program the settings name, as `program_name` reads it for a
mode's table (169). The core takes only the entry of the agent's
program and never reads another's, so a Codex setting given beside a
Claude Code one never reaches Claude Code. An entry for a program no
stage runs is kept and passed to nobody, as a mode's table's entry for
another program is.

`programs` is the key because it names what the table holds, programs'
settings by program, and keeps clear of the words an agents table
already holds. A program's name directly under `agents` would read as
well, but would collide with any setting flume adds later and leave a
misspelt key silently ignored as the runner's.

The values are kept as JSON values, `serde_json::Value`, since held
state and the launch's form are JSON. TOML's strings, numbers,
booleans, arrays and tables read into them one for one; a TOML date or
time, which JSON cannot hold, is refused naming the key, to be given as
a string. A profile whose program reads TOML writes the values back as
TOML (design 9).

### 2. Eight layers: the host's for every agent and for a stream

Clause 32's layers become eight, most specific first:

1. the launch's settings;
2. the host's, for the repository and kind;
3. the host's, for the kind;
4. the host's, for the place's stream, `[agents.streams.<stream>]`;
5. the host's, for every agent, `[agents]`;
6. the repository's, for a kind it does not declare;
7. the kind's declaration;
8. flume's default.

`[streams.<stream>]` already holds `agents` as its bound on agents
running, so a stream's settings cannot be `[streams.<stream>.agents]`.
They are a sub-table of the host's own: `[agents]` gives every agent on
the host its settings, and `[agents.streams.<stream>]` narrows it to
one stream, as `.stages.<stage>` narrows a kind's table to one stage.
The stream keeps to being a share of the host's room and a pool, and
the host's settings stay in one place.

```toml
[agents.programs.claude]
statusLine = { type = "command", command = "~/.config/flume/status-line.sh" }

[agents.streams.personal.programs.claude]
statusLine = { type = "command", command = "~/.config/flume/status-line-personal.sh" }
spinnerTipsEnabled = false
```

Neither table names a stage, since a stage's name is its kind's: a
`stages` table under either refuses the configuration, naming it and
`[kinds.<kind>.agents.stages.<stage>]` as where a stage's own settings
go. `[agents.streams.<stream>]` naming a stream the host does not
declare refuses the configuration naming it, as a stream's pool that no
`[pools]` table declares does; `default` always stands. Neither gives
`program`, since the host chooses a program by the accounts it pools
(record 0020), nor `env`, which is an account's alone (144), nor
`beside`, which is the launch's and the stage's alone (107).

The stream's table comes after the host's for a kind and for a
repository and kind, and before the host's for every agent. The kind
tables name the work itself, the stream's names a share of the host the
work runs in, and `[agents]` names nothing. A host that wants a
stream's word to stand for a kind leaves that key out of its kind
table.

Both come before the repository's and the kind's, as every host table
does today. A host that wants the last word on its agents states it in
its own layers (record 0024), and a stream whose model or status line
the host chose for its pool's accounts would otherwise lose to any kind
that names one. The cost is that a kind's own status line never shows
on a host that gives one; the user rules on it (question 2).

This departs from core.30's details, which rule out "settings given by
a stream (record 0020, as amended 2026-10-29)", and from 143's "a
stream gives its agents no settings (32)". Record 0020 kept a stream
from giving settings because "a stream's word on a kind's model would
be a third host table for what the host's word for the kind already
says". It does not hold for own settings: what a host's agents show and do
differs by the share of the host they run in, which no kind table can
say. Nor does it hold for the model as it did. The kind table says its
word for every stream and the stream's for every kind, and a stream is
where an allowance is spent, through its pool (question 1). The record this change's first group writes amends record
0020, 143 and core.30 together (core.19).

`Layer` gains `HostStream`, named `host-stream` in status, and `Host`,
named `host`. In every layer that names stages, a stage's own comes
before the settings for every stage, as today (32).

### 3. Key by key, each key taken whole

Each top-level key of a program's own settings is one setting, taken
whole from the first layer that gives it. The core folds them as it
folds every setting: `Settings::under` takes, for each program and each
key, the more specific layer's value where it gives one and the more
general one's otherwise, and `from` records `programs.<program>.<key>`
with its layer and its value as written.

So the host's status line stands while a stream gives
`spinnerTipsEnabled`, and a stream's status line replaces the host's
whole. The other two ways were weighed.

- **The whole entry from one layer**, as a mode's table is taken. Every
  layer that gives any key of a program would drop every key the layers
  beneath gave, so a stream giving one key would have to restate the
  host's status line, and a kind giving a permission would silently
  drop it. A host's default would hold only where nothing else spoke.
- **A deep merge**, tables merged at every depth. Two layers' status
  lines would merge into one object neither wrote, a command from one
  and a padding from the other, and two lists would join in an order
  the program does not define. That is flume deciding what a program's
  value means, which it does not (core.43). A deep merge also leaves a
  layer no way to replace a table it means to replace, a hook list or
  a permission list, since a key it leaves out would show through from
  beneath.

A top-level key is the coarsest unit that keeps one layer's value as
that layer wrote it and lets a layer replace it whole. Status names one
layer per key. A layer that changes part of a key's value restates the
key; the docs say so with an example. No layer removes a key a later
layer gives, since TOML has no null; a layer replaces it.

### 4. The account's configuration underneath, and nothing where nothing is given

flume passes the agent's program the keys the layers resolved and
nothing else, and the program lays them over its account's own
configuration as it lays its own sources. For Claude Code that is the
release's deep merge of `flagSettings` over the user's, the project's
and the local settings (design 8): a key flume passes wins where it is
a value, merges where it is an object, and joins where it is a list,
and every key flume does not pass is the account's or the repository's
as today. That merge is the program's, read from its release, never
flume's own.

Where no layer gives an entry for the agent's program, the profile is
given nothing, passes nothing, and the start is as it is today: no
`--settings`, no file written. The account's own configuration decides,
as it does for a mode no layer gives (162).

### 5. Keys another setting owns

Each agent profile names the keys of its program's settings that
another of flume's settings or the account owns, each with its owner.
A layer that gives one is refused, naming the key, the program, the
layer, and where the setting belongs. A nested key is named by its
path, `permissions.defaultMode`, and the profile looks inside the
top-level key that holds it. For Claude Code 2.1.296:

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

`statusLine` and `hooks` are not reserved. A status line is the case
that prompted the change. Hooks are the account's to give today, and a
hook of the layers runs beside flume's plugin's, as one of the
account's does. Claude Code's MCP keys, `enableAllProjectMcpServers`,
`enabledMcpjsonServers` and `disabledMcpjsonServers`, stay open until
the MCP setting stands (design 15).

The reserved keys are code in the profile, beside its map of modes,
and a `check-tools` test finds each in the pinned release's settings
schema (design 8; core.22, core.39). A key the release drops fails the
gate at the bump.

### 6. No credential, wherever it is read

A credential belongs in an account's `env`, which status names and
never prints (core.30). Every string in an own setting, at any depth,
is checked where its layer is read: the host's configuration when it is
read at a start, a declaration and the repository's file when the
kinds profile reads them, and the launch's `--agent` and JSON form. A
string is refused, naming its key's path and its layer and never its
value, where it:

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
the program reads it from its environment. A host's configuration
holding one refuses every launch and start, as one that cannot be read
does (C.7); a declaration or a repository's file holding one refuses
the launch (4).

The check is the core's, program-neutral, in `flume-seams` beside
`SET`, so every reader shares one rule. It rests on shapes, not
meaning. A credential in no known shape, a short password, passes it,
which is why status and the refusals print values only of own settings
that passed and why the docs say where a credential goes. A long mixed
identifier in a command is the false refusal it risks; the refusal
names the key so the host can move the value into a script the command
runs.

flume reads nothing else inside a value. It reads no `~` and no `$VAR`
in one, since core.30 keeps both to an account's `env`; a status line's
command reaches the program as written and its shell reads it there.

### 7. The agent seam

The agent seam gains the part **own settings** (176):

- `Agent::own(&self, given: &BTreeMap<String, Value>) -> Result<(), String>`
  says whether the profile can pass `given`, its program's entry as the
  layers resolved it: `Ok`, or words naming each reserved key with its
  owner, or that its program gives no way. By default every non-empty
  entry is refused, "its agent profile passes no settings of its
  program's own", as a profile that maps no mode refuses one (170).
- `Agent::command` gains `own: &BTreeMap<String, Value>`, the agent's
  entry alone, which the core takes by the program's name and never
  hands another program's, and `folder: &str`, the start's folder in
  the place's own folder.
- `AgentCommand` gains `files: BTreeMap<String, String>`, each an
  absolute path under that folder with its text. The core writes each
  before the terminal opens the agent, written beside and renamed over,
  readable by the host user alone, and refuses the start as not opened
  where a path is outside the folder. The printed command's line names
  the file as any argument (C.5). It goes with the place's folder at
  its tidy (62).

`Engine::resolution` asks `Agent::own` once the program is bound and
the mode mapped, where the agent's entry is not empty, and refuses in
design 12's words. An account of a pool is usable for a stage whose
layers name no program only where its program's profile takes the
stage's entry for it (173; record 0026), so a pool of two programs
starts the stage on the one that can.

The start's record names no own settings and compares none: no
release reads them back, so `Agent::own_read_back` is not added, and
status says they were not read back. A start brought back without them
is a start without the `FLUME_*` variables too, which acks nothing and
is never the stage's (77; core.10).

### 8. Claude Code takes them by `--settings <file>`

Where the agent's entry for `claude` is not empty, the profile's
command writes it as one JSON object, its keys in order, to
`<folder>/claude/<identity>/own-settings.json` beside the start's
record, and passes `--settings <that path>` after `--plugin-dir`. Where
it is empty, it passes no `--settings` and writes nothing.

A file rather than a JSON string, because the layers' values can be
long and hold the shell's own quoting, a hook's command or a status
line's, which a JSON string would carry through herdr's typed line and
the printed command a second time. A file also holds exactly what the
program read, for a person to look at, and is where the held change's
hook reads the layers' status line (design 15).

`Agent::own` refuses each key of design 5's table with its owner,
looking inside `permissions` for `defaultMode`, and takes every other
key as written.

`claude/settings.rs` holds the reserved keys, the file's name, and
`fragments()`, which a `check-tools` test in
`crates/flume/tests/profiles/tools/claude_words.rs` requires byte for
byte in the pinned release (C.4; core.22):

- `--settings <file-or-json>` and its words, an option of one value;
- the sources' order,
  `return["userSettings","projectSettings","localSettings","flagSettings","policySettings"]`;
- the merge that joins a source's lists with those beneath it,
  `return D([...e,...n])`, and takes the later source's value
  otherwise;
- each reserved key as the settings schema names it, `apiKeyHelper:`,
  `disableAllHooks:`, `skipDangerousModePermissionPrompt:` and the
  rest.

A bump that renames the option, moves the flag's source below the
user's, or drops a reserved key fails the gate until the profile is
read again from the new release.

### 9. Every other program's way

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
merge of flume's, and the layers still give the top-level key whole.
OpenCode's variable is set by its profile's command as the `FLUME_*`
variables are, and recorded by digest in the start's record (161). It
is flume's way of starting the program, not a variable a layer gives,
which core.30's details are amended to say. Until each entry is built,
`Agent::own`'s default refuses, so a host that gives `programs.codex`
for a stage running Codex sees its launch refused naming the program
and the key.

### 10. Read in every layer

One reader takes an agents table's `programs`: a table of tables, each
key a program's name and each value a table, converted into JSON values
with the credential check of design 6. Anything else fails naming
`<table>.programs` or the key.

- The host's `agent_settings` calls it for `[kinds.<kind>.agents]`,
  `[repos."<repository>".kinds.<kind>.agents]`, their `.stages.<stage>`,
  `[agents]` and `[agents.streams.<stream>]`, and `HostConfig` gains
  `every_agent: Settings` and `streams_agents: BTreeMap<String,
  Settings>`. `HostSettings` gains the settings for the place's stream,
  which its held state names, and for every agent, which
  `Engine::layers` reads at each start.
- The kinds profile's `map_agent` calls it for every agents table of a
  declaration, a kind's, a stage's and the repository's for a kind it
  does not declare, pushing a problem naming the field where it fails
  (4, 23, C.2). Weir's C.4 names `programs` in an agents table, the
  reader's to check, as it names `mode`, so weir-kinds returns it as
  written and flume's pin stays (core.4).
- `--agent [<stage>.]programs.<program>.<key>=<value>` gives one key,
  its value read as JSON where it parses as JSON and as the text
  written otherwise, so `outputStyle=Explanatory` is a string and
  `spinnerTipsEnabled=false` a boolean. Several build one entry; one
  key given twice for the same stage refuses the launch naming it.
  `programs` joins the settings every `--agent` refusal lists. The
  launch's JSON form takes `programs` as `Settings` serializes it.

`Settings` gains `programs: BTreeMap<String, BTreeMap<String, Value>>`,
read with a default and written only where it holds an entry, so held
state written before it reads with none.

### 11. Status and the JSON form

The JSON form only gains fields (core.14). An agent's settings give
`programs` with its program's entry alone, and `from` each
`programs.<program>.<key>` with its layer and its value as written
(56). `AgentStatus` gains `own: Option<OwnStatus>`, none where no layer
gave the agent's program an entry: `passed`, the profile's words for
how it passed them, `--settings <path>` for Claude Code, and
`read_back`, false. The person's form prints each key on its own line
under the agent with its value as JSON and its layer, then "own
settings not read back".

### 12. Refusals at the launch and at a later start

`Engine::launch` asks `Engine::resolution` of every stage before
anything is made, with the host's configuration as it stands then, as
it does for a mode (4; core.8). A stage whose agent's entry its profile
refuses is refused with "the stage `<stage>`'s own settings for
`<program>`, `<key>` from the layer `<layer>`, are refused: `<words>`".
At a later start, a host table changed after the launch leaves the
stage refused in the same words, starting no agent, until a retry
(27, 32). A credential-shaped value is refused where its layer is read
(design 6), so a launch never resolves one.

### 13. The stand-in and the walkthroughs

The stand-in for Claude Code (`crates/flume/tests/profiles/stand-in/claude.rs`)
reads `--settings <file-or-json>`, and merges it over the
`settings.json` of its `CLAUDE_CONFIG_DIR` and the place's
`.claude/settings.json` and `.claude/settings.local.json`, as the
release's sources merge. It writes the merged settings to the file
`STAND_IN_SETTINGS` names, so a walkthrough sees what the program read.
Its doc comment names this beside its acts.

Walkthroughs in `crates/flume/tests/profiles/tools/walkthroughs.rs`,
with the stand-in under the printed command:

- S76: a host whose `[agents.programs.claude]` gives a `statusLine`
  and `includeCoAuthoredBy = false`, whose `[agents.streams.work.programs.claude]`
  gives another `statusLine`, and a scratch repository whose
  `.config/flume.toml` gives `[kinds.fix.agents.programs.claude]
  outputStyle = "Explanatory"`, with the account's own `settings.json`
  giving `cleanupPeriodDays`. A fix in `work`: its printed command holds
  `--settings` naming a file under the place's folder, the stand-in read
  the work stream's status line, the host's `includeCoAuthoredBy`, the
  repository's output style and the account's `cleanupPeriodDays`, and
  status names each key with `host-stream`, `host` and `repository`. A
  fix in `default` read the host's status line. A fix on a host and a
  repository that give no entry prints no `--settings`, writes no file,
  and lands.
- S77: launches refused, each naming the key and its layer, no value in
  the refusal or in status, and nothing made: `--agent
  programs.claude.env={"X":"1"}`; a host whose `[agents.programs.claude]`
  gives `apiKeyHelper`; a repository whose `[kinds.fix.agents.programs.claude]`
  gives `permissions = { defaultMode = "bypassPermissions" }`; and a
  launch giving a status line whose command holds a value shaped as a
  token.

### 14. Own settings reaching a real Claude Code agent

One walkthrough in `crates/flume/tests/profiles/live/walkthroughs.rs`
runs a fix under herdr with the pinned Claude Code, on a walkthrough
host whose `[agents.programs.claude]` gives a `hooks` table with a
SessionStart hook that writes a file of the scratch directory's. The
file is written once the session starts, which only `--settings`
reaching the release could do, and the code agent's started ack is
taken as well, so the layers' hook ran beside flume's plugin's. Status
names `programs.claude.hooks` from `host`, and the place lands.

### 15. What the two places beside this one take from it

**The MCP research** (`research--mcp-servers-for-flumes-agents`)
should take:

- **Its own setting, beside this one.** `mcp` should be its own setting
  of the same eight layers, `[agents]` and `[agents.streams.<stream>]`
  among them, rather than own settings. A server is defined once by the
  host and is the same server for every program, so `mcp` is flume's
  word that each profile maps, as a mode is. Own settings are each
  program's own and are never mapped.
- **The merge unit.** Each server's name is taken whole from the first
  layer that names it, as each top-level key of own settings is.
- **The command's files.** Claude Code's servers file, for
  `--mcp-config <file> --strict-mcp-config`, is written beside this
  change's file through `AgentCommand::files`.
- **The credential check.** Design 6's check applies to every server
  definition, so a server's credential is named by a variable or a
  helper and never by its value.
- **The reserved keys.** Once `mcp` stands, each profile adds its
  program's own MCP keys: Claude Code's `enableAllProjectMcpServers`,
  `enabledMcpjsonServers` and `disabledMcpjsonServers`, Codex's
  `mcp_servers`, Gemini CLI's `mcpServers` and OpenCode's `mcp`. Then an
  agent's servers come from one setting. Until then a host may give
  them as own settings. That is a fix it has today for a repository's
  server that stops an agent at Claude Code's approval dialog, and the
  MCP record says how those keys move.
- **Status's shape.** `mcp.<name>`, each with its layer.

**The held status-line change** (`every-agent-shows-its-place-in-its-pane`)
should take:

- **One `--settings`.** The release reads one `--settings`. Its
  profile should put flume's `statusLine` into the file this change
  writes, at every start once it lands, rather than pass a JSON string
  of its own.
- **The layers' status line, kept.** flume's `statusLine` replaces the
  layers' `statusLine` key in the file. The layers' value is the
  session's own status line that its hook runs beneath flume's line,
  first, ahead of the local, project and user settings, since the flag
  outranks them. The hook reads it from the start's held settings, so
  a host's status line is never lost to flume's.
- **Its design 4 and core.30 sentence.** "The JSON holds no setting of
  the six layers" no longer holds. The file holds the layers' own
  settings and flume's status line. Status names the layers'
  `statusLine` with its layer and says flume's line wraps it.
- **One reading of `--settings`** in the stand-in, a file or a JSON
  string, with this change's merge of the sources beneath it.
- **The shared fragments** of `--settings` and the sources' order,
  pinned once.

### 16. The statement and the rules, drafted for the user's ruling

The statement at blueprints main, 92afa99, holds no clause for a
program's own settings and says a stream gives no settings. Drafts:

- **175** (A.4). An agent's own settings are one of its settings (32):
  a table of programs, each entry that program's own settings as its
  program spells them, its status line, hooks or tools' permissions
  among them. Every layer of 32 may give them. Each top-level key of an
  entry is a setting of its own, taken whole from the first layer that
  gives it, and only the agent's program's entry is taken, so no
  program is given another's. flume reads nothing inside a value,
  merges no two layers' values of one key, reads no `~` or variable in
  one, and passes them as the agent's profile takes them (176), its
  program laying them over its account's configuration as it lays its
  own; where no layer gives the agent's program an entry, flume passes
  none and the account's configuration decides (144). A key the profile
  names as another setting's or the account's, the environment, the
  model, the mode, a sign-in, the hooks flume's start rests on or a
  person's acceptance of a warning (34), is refused naming the key and
  what owns it; so is a value shaped as a credential, wherever it is
  read, naming the key and never the value (29); and so is an entry for
  a program whose profile takes none. The launch is refused so (4), or,
  at a later start, the stage (32). Status gives each with the agent
  and the layer it came from (56).
- **176** (B.4). **own settings**: given a program's own settings as
  the layers resolved them, how the program is started with them, by a
  file the start writes in the place's own folder, by arguments or by
  a variable of the program's; or each key refused with what owns it;
  or that the program gives no way to take them (175).
- **32**: eight layers, the host's for the place's stream and for
  every agent fourth and fifth, which name no stage; each top-level key
  of own settings taken whole as a setting (175); and, on question 1's
  answer, the model, the mode and the instructions given by both.
- **143**: "a stream gives its agents no settings (32)" becomes "a
  stream's agents take the settings the host's configuration gives for
  the stream (32)".
- **158**: a kind's own settings are its program's content, which flume
  reads for no host's location; a kind whose own settings name one is
  tied to the hosts that hold it, as one whose prompt names one is.
- **4**, **56**, **88**, **171** (a tenth reading: own settings not
  offered, refused until the entry names its release's way), **173**
  (an account usable only where its program takes the stage's entry)
  and section 3's **settings** and a new term, **own settings**.
- **C.4**: the command row's `--settings` and design 5's reserved keys;
  the common profile's own settings row. **C.7**: `[agents]`,
  `[agents.streams.<stream>]` and `programs` in every agents table.
- **S76** and **S77**, as design 13's walkthroughs.
- **weir C.4**: `programs` in an agents table, the reader's to check.

The rules: **core.30**'s text reads "the layers clause 32 orders", its
details lose "settings given by a stream" and gain a profile's own
variable for its program's settings among what is not ruled out. A new
rule, **core.43**, in "Stages and agents":

> A program's own settings are given under its name and reach no other
> program; each top-level key is taken whole from the first layer that
> gives it and passed as written, the way its profile takes them, never
> translated, merged inside or read by flume; a key its profile reserves
> for another setting or the account, and a value shaped as a
> credential, are refused; and where no layer gives one, nothing is
> passed.

Its details rule out a Codex setting reaching Claude Code; a key mapped
from one program's spelling to another's; two layers' values merged
inside one key; `~` or `$VAR` read in an own setting; design 5's keys
given; a credential-shaped value taken, or printed by a refusal or
status; `--settings` passed, or a file written, where no layer gives an
entry; and own settings taken silently by a profile that gives no way.
They leave open the program's own merge over its account's
configuration, a host's path in a kind's own settings (158), and
flume's own status line wrapped around the layers'. The record is the
next free number at its writing, 0030 at main b6cd1b5. The place
`change--an-answer-reaches-the-open-session-within-an-hour` holds a
record numbered 0028 that main's 0028 already takes, so whichever lands
second renumbers.

## Questions for the user's ruling

1. **Do `model`, `mode` and `instructions` gain the two host tables
   too?** Recommended: yes, every setting but the program and the
   environment in every layer. A stream is where the host's allowance
   is spent, through its pool's accounts, so a stream on a smaller
   model or in a narrower mode is a host's word no kind table can say.
   One rule for every setting is simpler to write and to read in
   status. If no, `[agents]` and `[agents.streams.<stream>]` refuse
   `model`, `mode` and `instructions`, naming `[kinds.<kind>.agents]`
   as where they go, and the agent-modes and launching deltas keep the
   six layers for them.
2. **Do the two host tables come before the repository's and the
   kind's?** Recommended: yes, the host's word, as every host table is
   today (record 0024). The cost is that a kind's own status line or
   model never shows where the host gives one. The other order makes
   them defaults the work overrides: launch, host-repository, host-kind,
   repository, kind, host-stream, host, default.

## Risks / Trade-offs

- **A layer's hook runs beside flume's plugin.** A `hooks` key can deny
  a tool or block a stop, as the account's own hooks can today. Of
  flume's needs, only the start's ack would be lost with hooks off,
  which `disableAllHooks` reserved keeps. A layer's hook that denies
  `flume needs` would keep an agent from stopping short, which status
  shows as a stuck stage.
- **The credential check rests on shapes.** It misses a credential in
  no known shape and may refuse a long mixed identifier. The refusal
  names the key so the host can move the value, and the docs say where
  a credential goes.
- **A key changed in part must be restated.** A stream that wants the
  host's permissions and one more rule gives the whole `permissions`
  key. The docs show it.
- **The host's word wins.** With question 2's recommended order, a
  kind that ships its own status line shows it only on a host that
  gives none.
- **Own settings are not read back.** A program that ignores a key, or
  a release that changes how it merges, shows no difference at the
  start. The `check-tools` fragments fail at the bump, and the live
  walkthrough proves the file reaches the real program.
- **Three changes touch `Settings` and the host's tables.** This one,
  the held status-line change and the open-session change each add to
  `Settings`, `Engine::resolve` and `agent_settings`. Whichever lands
  second merges the others' fields, and its stand-in reads one
  `--settings`.

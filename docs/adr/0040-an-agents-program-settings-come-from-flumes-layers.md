---
number: 40
title: An agent's program settings come from flume's layers, a named file and keys for each program, each key taken whole, and a repository's word for every kind
status: proposed
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# An agent's program settings come from flume's layers, a named file and keys for each program, each key taken whole, and a repository's word for every kind

Status: proposed. The user approved its design on 2026-11-05 in the Plannotator Inbox, answering "An agent's program settings come from Flume's layers: approve, so its code starts?" with "Approve: start its code (your recommendation)". That approval took the design as it stood, with the recommended answers to its two questions: the host's new tables give the model, the mode and the instructions too, and they come before the repository's and the kind's. The record waits on the user's yes once they have seen it as a record, at their review of the change `an-agents-program-settings-come-from-flumes-layers`.

Deciders: the user, 2026-11-01: "We just need a generic way that the coding agent can be configured through the config for Flume." It holds for every program flume runs, defaulted for the host and different by stream where a host says so, and every layer, the host's default first, names a whole settings file by path as well as giving keys inline, the file's keys under the inline ones, so a host hands flume's agents the file its people use. The user, 2026-11-02: what an agent's commits and pull requests say about who made them is configurable where it makes sense, starting with the tagline; a fork of someone else's project carries none while the host's own repositories keep theirs, each repository's word given by its own `.config/flume.toml` or by the host's `[repos."<repository>"]` table. The user, 2026-11-05: the two questions, as above. The shape of a program's entry, where each file is read, the merge unit, the reserved keys and their treatment, the credential check, the way Claude Code takes them, and how the new tables meet the host's managed and local layers are the writing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 068cbc0, clauses 4, 32, 56, 88, 143, 158, 171, 173, C.2, C.4, C.7 and section 3's settings as amended there, and 191, 192 and S95 to S97 added, and `design/weir/requirements.md` C.4 at the same revision; records 0017, 0020, 0024, 0026, 0032, 0033 and 0038; rules core.1, core.2, core.4, core.8, core.10, core.12, core.14, core.16, core.17, core.22, core.30, core.34, core.35, core.39; the change's design, `openspec/changes/an-agents-program-settings-come-from-flumes-layers/design.md`, decisions 1 to 7, 16 and 18; `crates/flume-core/src/settings.rs` (`Engine::layers`, `Engine::resolve`, `Engine::resolution`), `crates/flume-seams/src/kinds.rs` (`Settings`), `crates/flume/src/host/config.rs` (`agent_settings`), `crates/flume/src/profiles/kinds/weir/mod.rs` (`map_agent`), `crates/flume/src/profiles/agent/claude/mod.rs` (`command`); Claude Code 2.1.296's `--help` for `--settings <file-or-json>` and its bundle's settings schema and sources' order, read on 2026-11-01 and 2026-11-02; the `--help` and packages of Codex 0.165.0, Gemini CLI 0.66.0, GitHub Copilot CLI 1.0.104 and OpenCode 1.19.4, read on 2026-11-01.

## Context and problem statement

An agent flume starts gets its program, its model, its mode, its instructions and its environment from clause 32's layers, and nothing else of its program's own settings. A session a person starts on a host can be handed one settings file by the host's launcher, Claude Code's `--settings`, which gives its status line, its full-screen renderer, its hooks, its plugins and more. An agent flume starts runs with its account's configuration home alone, so no layer of flume's can give a status line, a renderer, a hook or a tool's permission, nor name the file a host's people use.

The pinned Claude Code 2.1.296 loads `--settings <file-or-json>` as its flag settings, which it orders after the user's, the project's and the local settings, and merges the sources itself: objects key by key, lists joined with their duplicates dropped, anything else from the later source. Its schema holds keys that are flume's or the account's: `env`, `model`, `permissions.defaultMode`, the credential helpers and sign-in keys, `disableAllHooks`, and two that record a person's acceptance of a warning. It also holds `attribution`, the text its commits and pull requests carry, empty for none. An upstream project refused the user's pull requests over Claude Code's `Generated with Claude Code` line.

A stream gave its agents no settings (143; record 0020), and the host's tables gave settings for a kind or a repository and kind alone (record 0017).

## Considered options

For where a program's own settings sit in an agents table:

1. `programs`, a table of programs, each entry flume's, with `file`, a settings file in the program's own words, and `settings`, keys as the program spells them.
2. The program's name directly under `agents`, its keys beneath.
3. A bare table of keys, taken by whichever program runs.

For how the layers' values of a program's settings combine:

1. The whole entry from the first layer that gives one, as a mode's table is taken.
2. A deep merge, tables merged at every depth.
3. Each top-level key taken whole from the first layer that gives it, and within a layer from its `settings` before its file.

For a key another setting or the account owns:

1. Refused wherever it appears, a named file's among them.
2. Refused where a layer's `settings` writes it, and left out of a named file, status naming it.
3. Passed, the program deciding.

For a stream's settings and the host's default:

1. `[streams.<stream>.agents]`.
2. `[agents]` for every agent on the host, and `[agents.streams.<stream>]` beneath it for one stream.
3. None: a host gives settings by kind alone.

For a repository's word for every kind:

1. A kind table for every kind, written again for each.
2. The host's `[repos."<repository>".agents]` and the repository's own `[agents]` in `.config/flume.toml`, each beneath its layer's tables for one kind.

For where the host's new tables come (the user's second question):

1. Before the repository's and the kind's, as every host table does.
2. After them, as defaults the work overrides.

For which settings the new tables give (the user's first question):

1. Every setting but the program and the environment, as the host's kind tables do, the repository's `[agents]` giving a program as its kind tables do.
2. Own settings alone.

For how Claude Code takes them:

1. A JSON string, `--settings '<json>'`.
2. A file the start writes in the place's own folder, `--settings <file>`.
3. The named file as it stands.

## Decision outcome

Placement option 1, merge option 3, reserved-key option 2, stream option 2, repository option 2, order option 1, new-table option 1, and passing option 2.

**A program's entry: a file and its own keys** (191). Every agents table may give `programs`, whose keys are programs' names. Each entry gives `file`, `settings` or both; any other key fails its table, naming the key and saying a program's own settings go under `settings`. A program's name is the last component of the program the settings name, as for a mode's table (169). The core takes only the entry of the agent's program, so a Codex setting never reaches Claude Code; an entry for a program no stage runs is checked and passed to nobody. Values are kept as JSON values; a TOML date or time, which JSON cannot hold, fails naming the key.

**A named file, read where its layer lives.** A file is JSON where its name ends `.json` and TOML where it ends `.toml`, its top level a table, any other name, a file that does not parse and a top level that is not a table failing naming the file. Its keys lie beneath its table's `settings`.

| layer | `file` names | read |
|---|---|---|
| the host's, each of its tables | a file on the host: absolute, under `~/` read as the host user's home as a repository's key is, or relative to the configuration's own directory | at each start, with the host's configuration, through a link where it is one (C.7) |
| the repository's, for a kind and for every kind | a file of the repository, relative to `.config/flume.toml`'s directory | at the launch, at the revision the launch read (21) |
| the kind's declaration | a file of its package, relative to its declaration's file | at the launch, as its prompts are (157) |
| the launch's | a file on the host, relative to the command's working directory or absolute | at the launch |

A repository's or a kind's file outside its repository or package, by `..`, an absolute path or a link, fails its declaration (core.34). No `$VAR` is read in a path (core.30). A file read at the launch is kept with the launch's settings or the place's kind and not read again; a host's is read again at each start, as the host's configuration is. flume never passes a named file as it stands: it passes the keys the layers resolved.

**Eight layers** (32). Most specific first: the launch's; the host's for the place's repository, for its kind and then for every kind, `[repos."<repository>".agents]`; the host's for the kind; the host's for the place's stream, `[agents.streams.<stream>]`; the host's for every agent, `[agents]`; the repository's, for a kind it does not declare and then for every kind, its own `[agents]`; the kind's declaration; flume's default. Within a layer that names stages, a stage's own comes first, then the table for every stage, then, in a repository's two layers, the table for every kind. `[agents]` and `[agents.streams.<stream>]` give `model`, `mode`, `instructions` and `programs`, and name no stage, no `program`, no `env` and no `beside`, each refusing the configuration naming it, as a stream it does not declare does. `[repos."<repository>".agents]` gives what the host's kind tables give and names no stage and no stream. The repository's `[agents]` is flume's own table of `.config/flume.toml`, beside `[profiles]` and `[prompt]` and no part of any kind, read at the revision with the kinds; it gives what the repository's kind tables give, a program among it, and names no stage, no stream, no `env` and no `beside`. Status names the new layers `host-stream` and `host`; the tables for every kind keep their layers' names, `host-repository` and `repository`.

**The host's managed and local layers.** Every agents table of the host's, the new three among them, is one the local layer may set, as a kind's agents table is (record 0032), and the managed layer may pin any key of one. Each own setting, `programs.<program>.settings.<key>`, and each `programs.<program>.file`, is one key of the managed and local fold, so the local layer replaces one own setting without restating the managed layer's others.

**Key by key, each key taken whole.** Each top-level key of a program's own settings is one setting, taken from the first layer that gives it, and within a layer from its `settings` before its file. `from` records `programs.<program>.settings.<key>` with its layer, its file where it came from one, and its value as written. A layer that changes part of a key's value restates the key; no layer removes a key a later one gives, but replaces it, `hooks = {}` among them.

**Nothing where nothing is given.** flume passes the keys the layers resolved and nothing else, the program laying them over its account's configuration as it lays its own sources. Where no layer gives the agent's program a key, the profile is given nothing and passes nothing, and the account's configuration decides (162).

**Keys another setting owns.** Each agent profile names its program's keys that another setting or the account owns, each with its owner, a nested key by its path. A reserved key a table's `settings` writes refuses the launch, or at a later start the stage, naming the key, the program, the layer and its owner. One a named file gives is left out, unread, the rest of its table kept, and status names it with its file, its layer and its owner; that is the one change flume makes inside a value. For Claude Code 2.1.296: `env`, the account's (29, 144); `model`, flume's model (172); `permissions.defaultMode`, flume's mode (162); `apiKeyHelper`, `awsAuthRefresh`, `awsCredentialExport`, `gcpAuthRefresh`, `otelHeadersHelper`, `forceLoginMethod` and `forceLoginOrgUUID`, the account's sign-in (144); `disableAllHooks`, flume's plugin (35, C.4); and `skipDangerousModePermissionPrompt` and `skipAutoPermissionPrompt`, a person's acceptance of a warning (34; core.12). Claude Code's MCP keys stay open own settings until the host's MCP servers are built (record 0033), whose change adds them to this list.

**No credential.** Every string of an own setting, at any depth, inline or from a file, is checked once the reserved keys are out of the named files and before anything is held, passed or printed: at the launch every entry of every layer, and at each later start the host's layers as they stand then. A string is refused, naming its key's path, its layer and its file and never its value, where it opens with a prefix an issuer gives its tokens followed by sixteen or more letters, digits, `_` or `-`; holds a private key's opening; is a JSON web token; holds, apart from a `$VAR` or `${VAR}`, a run of 32 or more letters, digits, `_` and `-` mixing upper case, lower case and digits; or is a non-empty value naming no variable of a key whose name ends with `token`, `secret`, `password`, `passwd`, `apikey`, `privatekey`, `accesskey`, `clientsecret`, `authorization` or `credentials`. The refusal says a credential goes in the account's `env`. The check is the core's, one rule for every program, and rests on shapes: a credential in no known shape passes, which is why status and refusals print values only of settings that passed it.

**The agent seam's own settings part** (192). A profile names its reserved keys and says whether it can pass the keys resolved for its program; by default none is reserved and every non-empty entry is refused. Its command is given the resolved keys and the start's folder, and lists the files it needs written there, each written beside and renamed over, readable by the host's user alone, before the terminal opens the agent, as record 0038 writes the standing prompt's. No release reads own settings back, so the start's record names none and status says so. An account of a pool is usable for a stage whose layers name no program only where its program's profile takes the stage's keys for it (173), which record 0026's change builds with the rest of 173, since until then flume's default layer names a program for every stage. Until a program's entry names its release's way, checked against it, own settings given for it refuse the launch (171).

**Claude Code takes them by `--settings <file>`.** Where the resolved keys are not empty, its profile writes them as one JSON object to `own-settings.json` beside the start's record and passes `--settings <that path>` after `--plugin-dir`; where they are empty it passes nothing and writes nothing. A `check-tools` test finds `--settings`, the sources' order, the merge that joins lists, each reserved key and `attribution`'s two texts in the pinned release, so a bump that moves one fails the gate (core.22, core.39).

**The worked example: what a commit says about who made it.** Claude Code's `attribution` is one top-level key, given, resolved, checked and passed as every own setting is, named by nothing in the core, the readers or the profile's command. A host whose fork must carry no tagline gives `[repos."~/Code/upstream-fork".agents.programs.claude.settings] attribution = { commit = "", pr = "", sessionUrl = false }`; a repository of its own gives its own words in its `.config/flume.toml`'s `[agents.programs.claude.settings]`, which travel with it; a repository neither names keeps its account's. A fork's word is the host's because a file committed in the fork travels in the pull request its upstream reviews. The docs give both texts and `sessionUrl = false` for none, since the key is taken whole and the release fills a text it leaves out. Codex's tagline, and the other programs', are read from their releases when their entries are built, and given under their names in the same tables. A commit's signature, author and committer are the host's git's and the merge queue's, and what the merge queue writes is its own (core.6): no layer of flume's gives them.

### Why not the others

Placement option 2 would collide with any word flume adds to an agents table later and leave a misspelt key silently taken as a program's; option 3 has no meaning, since no own setting means the same to two programs. Merge option 1 drops every key beneath a layer that gives any one: a stream giving one key would restate the host's file, and a kind giving a permission would silently drop the host's renderer. Merge option 2 has flume decide what a program's value means, two layers' status lines merged into one object neither wrote, and leaves a layer no way to replace a table whole. Reserved-key option 1 keeps the file a host's people use from ever being named, since it gives three reserved keys; option 3 lets a layer give an agent an environment, a model, a mode or an account behind the settings that own them, or turn off the hooks the start's ack is read from, or accept a warning for a person. Stream option 1 cannot be, since `[streams.<stream>]` holds `agents` as its bound; option 3 leaves a host no way to say what every agent gets, nor how a stream's share of the host differs, which no kind table can say. Repository option 1 misses a kind added later. Order option 2 would let any kind override what a host chose for its pool's accounts, and its cost, a kind's own status line never showing where the host gives one, is the host's to waive by leaving the key out of its own layers. New-table option 2 leaves a stream on a smaller model or in a narrower mode, a host's word no kind table can say, unsayable. Passing option 1 carries a hook's command or a status line's through herdr's typed line and the printed command with a second layer of quoting, and leaves nothing for a person to read of what the program took; option 3 passes the keys another setting owns.

### Consequences

- Clauses 4, 32, 56, 88, 143, 158, 171, 173, C.2, C.4, C.7 and section 3's settings are amended, and 191, 192, S95, S96 and S97 added, with weir's C.4, on blueprints at 068cbc0, on the branch `design/flume-an-agents-program-settings` until it reaches main (core.19). The change drafted them as 175, 176 and S76 to S78, numbers main has since taken.
- Clause 128 and C.7's store row give the store a file written into the place's folder, through which the core writes a command's files, at blueprints 352aa32 on the branch `design/flume-the-store-writes-a-starts-files` until it reaches main (design 8; core.2, core.19).
- Rule core.30 is amended and core.50 drawn from this record.
- Weir's C.4 names `programs` in an agents table, the reader's to check, as it names `mode`, so weir-kinds returns it as written and flume's pin of it stays (core.4).
- The research `research--mcp-servers-for-flumes-agents` and record 0033 decide the host's MCP servers, a table of the managed layer, not own settings; their change writes a server file for Claude Code through the same command files and adds Claude Code's MCP keys to its reserved list. The change `every-agent-shows-its-place-in-its-pane` puts flume's status line into the same `--settings` file, the layers' `statusLine` kept as the session's own beneath it.
- The change `an-agents-program-settings-come-from-flumes-layers` builds it.

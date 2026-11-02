# An agent's program settings come from flume's layers

## Why

A session a person starts on a host can carry the agent program's own
settings, such as Claude Code's status line and its full-screen
renderer, through one settings file the host's launcher passes. An
agent flume starts gets none of them. It runs its program with its
account's configuration home alone, and no layer of flume's can name a
status line, a renderer, a hook or a tool's permission, or the file a
host's people use.

The user, 2026-11-01: "We just need a generic way that the coding agent
can be configured through the config for Flume." It must hold for every
program flume runs (record 0026), each with its own settings, and tie
flume to no one setting. It must be defaulted for the host and may
differ by stream. Every layer, the host's default first, must be able
to name a whole settings file by path as well as give keys inline, the
file's keys under the inline ones, so a host hands flume's agents the
file its people use. The account's own configuration stays underneath
(32, 143, 158, 162, C.4, C.7).

The user, 2026-11-02: what an agent's commits and pull requests say
about who made them must be configurable where it makes sense,
starting with the tagline, whether a commit or a pull request says
`Generated with` or `Co-Authored-By` anything, and in what words. An
upstream project refused the user's pull requests over Claude Code's
`Generated with Claude Code` line. A fork of someone else's project is
to carry none while the host's own repositories keep theirs, each
repository's word given by its own `.config/flume.toml` or by the
host's `[repos."<repository>"]` table (32, 175, C.2, C.7).

## What Changes

- **A program's own settings.** Every layer that gives an agent's
  settings may give `programs`, a table of programs. Each entry names
  that program's `file`, a settings file in the program's own words,
  and its `settings`, keys given inline:
  `[agents.programs.claude] file = "~/.config/agent-launcher/claude-settings.json"`
  and `[agents.programs.claude.settings] tui = "fullscreen"`. Only the
  entry for the agent's program is taken, so a setting for one program
  never reaches another. flume translates nothing in them (175).
- **A named file in every layer.** A file is JSON or TOML by its name,
  read where its layer lives: the host's on the host at each start,
  relative to the configuration's directory, absolute or under `~/`;
  the repository's and the kind's within their repository or package,
  at the launch's revision, beside the kind's prompts; the launch's at
  the launch. Its keys lie beneath its table's `settings`, and a stage's
  table comes before every stage's (21, 157, 175; core.34).
- **Two host layers.** `[agents]` gives settings for every agent on the
  host, and `[agents.streams.<stream>]` for every agent in one stream.
  They come after the host's tables for a repository and for a kind,
  and before the repository's and the kind's, so clause 32's layers
  become eight. Neither names a stage (32, 143).
- **A repository's word for every kind.** The host's
  `[repos."<repository>".agents]` and the repository's own `[agents]`
  in `.config/flume.toml`, flume's table beside `[profiles]`, give
  settings for every agent working in that repository, whatever its
  kind. Each lies in its layer, `host-repository` or `repository`,
  beneath that layer's table for the kind; neither names a stage. The
  host's table speaks for a repository it may not write to, a fork
  among them (32, C.2, C.7).
- **A worked example: the tagline.** Claude Code's own `attribution`,
  its commit and pull request text, empty for none, is given as any
  own setting:
  `[repos."~/Code/upstream-fork".agents.programs.claude.settings]
  attribution = { commit = "", pr = "", sessionUrl = false }` keeps
  every agent's commits and pull requests in the fork free of a
  tagline, and a repository's own `.config/flume.toml` gives its own
  words for every kind. A test reads that layer's value in the agent's
  `--settings` file, and a walkthrough, S78, in what the program read
  (175).
- **Key by key.** Each top-level key of a program's settings is one
  setting, taken whole from the first layer that gives it, and within a
  layer from its `settings` before its file. A layer never merges inside
  a key and never removes a key; `hooks = {}` replaces a file's hooks
  (32, 175).
- **None where none is given.** With no key for the agent's program in
  any layer, nothing is passed, and the account's own configuration
  decides as today (162, 175).
- **Passed as the program takes them.** Each agent profile passes its
  program's settings its own way or refuses them. Claude Code's takes
  them by `--settings <file>`: a JSON file flume writes in the start's
  folder, holding the keys the layers resolved and never a named file
  as it stands, which the release lays over the user's, the project's
  and the local settings. Every other program refuses them until its
  entry names its way, checked against its release (88, 170, 176, C.4).
- **Keys another setting owns.** Each profile names its program's keys
  that a flume setting, the account or a person owns. Written in a
  layer's `settings`, one is refused, naming the key and its owner.
  Given by a named file, one is left out and status names it, so a
  people's file that gives `env`, `permissions.defaultMode` and
  `skipDangerousModePermissionPrompt` can still be named. For Claude
  Code these are `env`, `model`, `permissions.defaultMode`, its
  credential helpers and sign-in keys, `disableAllHooks`, and the keys
  that record a person's acceptance of a warning (core.12, core.30).
- **No credential.** A value shaped as a credential is refused once the
  reserved keys are out and before anything holds it, naming the key,
  the layer and the file and never the value. A credential belongs in
  the account's `env` (29, 144).
- **Refusals.** A launch is refused, naming the key, the program, the
  stage, the layer and the file, where a stage's program cannot take
  the settings given for it. A host's table or file changed after the
  launch refuses the stage at its start in the same words, until a
  retry (4, 32).
- **Status.** Each of the current agent's own settings with its layer
  and file, as `programs.claude.settings.tui` from `host` with
  `~/.config/agent-launcher/claude-settings.json`, each key left out of
  a file with its owner, and how the profile passed them (56).
- **The command line.** `flume launch --agent
  [<stage>.]programs.<program>.settings.<key>=<value>`, the value read
  as JSON where it parses and as the text written otherwise, and
  `--agent [<stage>.]programs.<program>.file=<path>`.
- **Outside this change.** A word of flume's for attribution mapped by
  each profile; a host's defaults given by a future control plane,
  which writes the host's configuration as any configurer does; a
  commit's signature, author and committer, which the host's git and
  sluice own; and what the merge queue writes.
- **For the user's ruling.** Whether `model`, `mode` and `instructions`
  gain the new tables too (recommended: yes). Whether the two host
  layers come before the repository's and the kind's, as every host
  table does today (recommended), or after them as defaults (design,
  questions).

## Capabilities

### New Capabilities

- `own-settings`: a program's own settings as a table of programs, each
  entry a named file and inline keys, in every layer, key by key, a
  repository's for every kind among them; nothing passed where none is
  given; each profile's way or refusal; reserved keys refused where
  written and left out of a file; credential-shaped values refused.

### Modified Capabilities

- `launching`: clause 32's layers become eight, the host's for every
  agent and for a stream among them, a repository's two layers each
  gain a table for every kind, and own settings resolve key by key.
- `agent-modes`: the mode is given in the eight layers (on the user's
  yes to the first question).
- `store-profile`: the host's `[agents]`, `[agents.streams.<stream>]`
  and `[repos."<repository>".agents]` tables, and `programs` with its
  files in every agents table of the host's.
- `streams`: a stream's agents take settings from its host table.
- `kinds-profile`: a declaration's agents tables give `programs`, a
  file read beside the declaration as its prompts are, and a
  repository's `[agents]` gives its settings for every kind.
- `command-line`: `--agent` gives `programs.<program>.settings.<key>`
  and `programs.<program>.file`.
- `agent-profile`: Claude Code passes its settings by `--settings
  <file>`, names its reserved keys, carries a repository's
  `attribution` as written, and rests on the pinned release.
- `place-status`: an agent's own settings, each with its layer and its
  file, and the keys left out.
- `seam-contracts`: the agent seam's own-settings part, its reserved
  keys, a command's files in the start's folder, and the types, a
  repository's settings for every kind among them.
- `seam-fakes`: the fake agent scripts its way, its reserved keys and
  none.
- `clause-coverage`: the list at the statement's revision holding 175,
  176, S76, S77 and S78.

## Impact

- agentplot/blueprints: the statement gains 175, 176, S76, S77 and S78
  and amends 4, 32, 56, 88, 143, 158, 171, 173, C.2, C.4, C.7 and
  section 3's settings, drafted in design 18 for the user's ruling. Clause 143's "a
  stream gives its agents no settings" and core.30's rule against a
  stream's settings change first (core.19). weir's C.4 names `programs`
  in an agents table, the reader's to check, so weir-kinds returns it
  as written and flume's pin stays (core.4).
- `docs/adr`: a record of the decision, written once the user rules,
  amending records 0017, 0020 and 0024; core.30 amended and core.43
  drawn with it.
- `crates/flume-seams`: `Settings::programs` with `ProgramSettings` and
  `NamedFile`, `Layer::HostStream` and `Layer::Host`, `KindsRead::Read`'s
  `agents` and the held `repository_agents`, `Agent::reserved`,
  `Agent::own`, the folder and `AgentCommand::files` of `Agent::command`;
  status fields, only added.
- `crates/flume-core`: a new `own.rs` leaves a file's reserved keys
  out, refuses those written, checks for credentials and folds own
  settings key by key; `settings.rs` folds eight layers; `launch.rs`
  and `start.rs` refuse, write the command's files and pass them;
  `accounts.rs` reads an account usable only where its program takes
  what is given; `status.rs` gives them.
- `crates/flume`: a new `own.rs` reading an agents table's `programs`
  and its files; the host's configuration with
  `[repos."<repository>".agents]`, the kinds profile's agents tables and
  a repository's `[agents]`, and `--agent`; the Claude Code profile's `--settings` and its
  reserved keys in a new `claude/settings.rs`; the person's form of
  status; the stand-in's `--settings`; walkthroughs and one live
  walkthrough.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake agent's
  way and reserved keys, a new `engine/own_settings.rs`, and the clause
  list.
- `site/`: the agent-settings, kinds and Claude Code guides, the
  tagline among their examples, and the configuration, commands and
  state references.
- The research place `research--mcp-servers-for-flumes-agents` and the
  held change `every-agent-shows-its-place-in-its-pane` each take
  something from this one (design 17).
- No new crate, plugin or site part, so no new moon project; no new
  dependency (`serde_json` and `toml` are the workspace's already).

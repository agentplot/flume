## ADDED Requirements

### Requirement: services.flume installs flume where enabled

The module SHALL give `services.flume.enable`, `services.flume.package`
and `services.flume.settings`. Where enabled, it SHALL put `package` in
the system profile, the flake's own package for the host's system by
default, and SHALL install none where `package` is null, writing the
managed file either way (record 0055).

#### Scenario: No package

- **WHEN** `nix:test` evaluates a nix-darwin system, and a NixOS system, each with `services.flume.enable` set and `services.flume.package` null
- **THEN** neither system profile holds a `flume` from the module, and each activation still links the file

### Requirement: The file is one user's, and flume runs no service

On nix-darwin the module SHALL write the primary user's file, requiring
a primary user whose home is set. On NixOS it SHALL write the file of
`services.flume.user`, a user `users.users` declares, and do nothing in
any other user's activation. Neither module SHALL declare a service,
since flume runs none (core.13).

#### Scenario: Enabled on nix-darwin

- **WHEN** `nix:test` evaluates a nix-darwin system of aarch64-darwin whose primary user is alice, at `/Users/alice`, with `services.flume.enable` set
- **THEN** the system profile holds the flake's aarch64-darwin `flume`, the post-activation runs the link as alice under `/Users/alice` with the file rendered from `settings`, `system.requiresPrimaryUser` names `services.flume.enable`, and no launchd agent is declared for flume

#### Scenario: Enabled on NixOS

- **WHEN** `nix:test` evaluates a NixOS system of x86_64-linux with `services.flume.enable` set and `services.flume.user` alice, a normal user, beside the normal user bob
- **THEN** the system profile holds the flake's x86_64-linux `flume`, the user activation runs the link with the file rendered from `settings` where the user is alice and does nothing for bob, and no systemd unit is declared for flume

#### Scenario: No user to write for

- **WHEN** `nix:test` evaluates a nix-darwin system whose primary user's home is unset, or a NixOS system whose `services.flume.user` no `users.users` entry declares
- **THEN** evaluation fails with an assertion naming the user and what to set

### Requirement: settings names the managed file's tables and keys

`services.flume.settings` SHALL be free-form TOML whose keys are the
managed file's, as 143 and C.7 name them. It SHALL declare `host`,
`streams`, `pools` with their `accounts`, `kinds`, `repos` and `agents`,
SHALL type each key C.7 gives one type that flume reads at the revision
the module is built from, and SHALL leave an agents table free-form
whole (record 0055).

#### Scenario: A mistyped declared key

- **WHEN** `nix:test` evaluates the options with `settings.streams.work.agents = "five"`
- **THEN** evaluation fails naming `services.flume.settings.streams.work.agents`

### Requirement: Only the keys flume reads are declared

The module SHALL declare no key flume does not read at the revision it
is built from, so a stream's `points` and `pace` pass undeclared until
flume reads them (record 0025). It SHALL declare no word written before
the current one, so `host.terminal`, which flume reads as `multiplexer`
and refuses beside it, passes undeclared (record 0055).

#### Scenario: The declared keys

- **WHEN** `nix:test` reads the options `services.flume.settings` declares
- **THEN** `host` declares `multiplexer` and no `terminal`, and a stream declares neither `points` nor `pace`

### Requirement: An unset key is left out and an undeclared key passes

Every declared key and table SHALL be null unless set, and a null SHALL
be left out of the file, so flume's own default holds. A key the module
does not declare SHALL pass to the file as written, for flume to judge
when it reads the file.

#### Scenario: A key the module does not declare

- **WHEN** `nix:test` renders `settings` holding `host.terminal = "printed"` and a stream's `points = 8`, neither declared
- **THEN** the file holds both as written, and no key that `settings` left unset

### Requirement: Each table merges across modules

Each table of `settings` SHALL merge across the modules that set it, so
values two modules give are written into one file.

#### Scenario: Two modules' settings merge

- **WHEN** `nix:test` evaluates the options with one module setting `settings.repos."acme/acme".stream = "work"` and another setting `settings.streams.work.agents = 5`
- **THEN** the file holds `[repos."acme/acme"]` with `stream = "work"` and `[streams.work]` with `agents = 5`

### Requirement: The file is rendered at evaluation

The module SHALL render `settings` to TOML at evaluation. Its first line
SHALL say the module wrote it from `services.flume.settings`. Top-level
keys SHALL come before tables, each sorted by name, and a key SHALL be
bare where TOML allows. A non-empty list of tables only SHALL be written
as `[[...]]` sections, any other list inline, and a float with a point
or an exponent. One `settings` SHALL give one text on both platforms.

#### Scenario: Every key renders its file

- **WHEN** `nix:test` renders `nix/tests/every-key.nix`, whose `settings` sets every declared table and key, an address and a folder under `repos`, and an account's `env` naming `$HOME`
- **THEN** the text is `nix/tests/files/every-key.toml` byte for byte

#### Scenario: No settings

- **WHEN** `nix:test` renders the default `settings`
- **THEN** the text is `nix/tests/files/empty.toml` byte for byte: the first line alone

### Requirement: A value the file cannot hold fails evaluation

Any value of `settings` but a string, a boolean, an integer, a float, a
list or a table SHALL fail evaluation, naming its path and the values
the file takes.

#### Scenario: A path value

- **WHEN** `nix:test` renders `settings` holding a path value at `kinds.change.agents.programs.claude.file`
- **THEN** evaluation fails naming `services.flume.settings.kinds.change.agents.programs.claude.file` and the values the file takes

### Requirement: flume reads the rendered file whole

flume's own reader of the host's configuration SHALL read each file the
module renders with no fault, under the names the file gives each key
(C.7).

#### Scenario: flume reads the rendered files

- **WHEN** `the_modules_files_read_whole` in `flume:test` reads each file under `nix/tests/files` with `HostConfig`
- **THEN** each reads with no fault, and `every-key.toml` gives the streams, pools, accounts, kinds, labels, repositories and agents settings `every-key.nix` sets

### Requirement: Nothing shaped as a credential reaches the store

The module SHALL fail evaluation where a string of `settings` is shaped
as a credential by flume's own shapes (core.50), read with its key's
name. The failure SHALL name the string's path, never its value, and
say to name a variable of the login environment instead (29). A string
naming a variable in place of a value SHALL pass.

#### Scenario: A token in an account's env

- **WHEN** `nix:test` evaluates the options with an account's `env.CLAUDE_CODE_OAUTH_TOKEN` set to a token's text
- **THEN** evaluation fails naming `services.flume.settings.pools.<pool>.accounts[0].env.CLAUDE_CODE_OAUTH_TOKEN`, the failure does not hold the token's text, and it says to name a variable as `$NAME`

#### Scenario: A variable named instead

- **WHEN** `nix:test` evaluates the options with that `env` value `$AGENT_TOKEN`
- **THEN** evaluation succeeds and the file holds `$AGENT_TOKEN`

### Requirement: One table of cases holds both implementations of the shapes

The module's shapes and flume-core's `shaped_as_credential` SHALL be
held to one table of cases, `crates/flume-core/tests/credential-shapes.json`,
each case shaped or passing in both as the table says.

#### Scenario: One table of cases for both

- **WHEN** `nix:test` runs every case of `credential-shapes.json` through the module's shapes, and flume-core's `own::tests` run every case through `shaped_as_credential`
- **THEN** each case is shaped, or passes, as the table says, in both

### Requirement: The activation links flume's path

The activation SHALL link the user's `~/.config/flume/config.toml` to
the rendered file, making `~/.config/flume/` where it is missing and
replacing a link there. A file or folder there that is no link SHALL be
moved once to `config.toml.before-nix`, both paths named. Where that
already stands, it SHALL change neither, name both, and fail the link
alone.

#### Scenario: A fresh home

- **WHEN** `nix/tests/scripts.sh` runs `link` in a scratch home with no `~/.config/flume/`
- **THEN** `~/.config/flume/config.toml` is a link to the file, and the folder holds nothing else

#### Scenario: A file aside already

- **WHEN** `link` runs in a scratch home holding a file `config.toml` and a `config.toml.before-nix`
- **THEN** it exits non-zero naming both paths, and both files are as they were

#### Scenario: A link of an earlier activation

- **WHEN** `link` runs where `config.toml` links to another store file
- **THEN** it links to the new file and moves nothing aside

### Requirement: The activation writes no other file

The activation SHALL write nothing but the link and a file it moves
aside: not `config.local.toml`, a source's file, the host's lock,
flume's state or data, nor any other tool's file (C.7; core.45).

#### Scenario: A hand-written file there

- **WHEN** `link` runs in a scratch home whose `~/.config/flume/` holds a file `config.toml`, a `config.local.toml`, a `sources/` folder and a `flume.lock`
- **THEN** the file is moved to `config.toml.before-nix`, `config.toml` links to the module's file, both paths are named, and `config.local.toml`, `sources/` and `flume.lock` are byte for byte as they were

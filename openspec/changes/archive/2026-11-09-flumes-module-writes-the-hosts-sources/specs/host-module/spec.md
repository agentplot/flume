## ADDED Requirements

### Requirement: services.flume.sources declares each source's file

The module SHALL give `services.flume.sources`, an attribute set of the
host's sources by name, empty by default, each a free-form TOML table
that is the file `~/.config/flume/sources/<source>.toml`, its keys named
as the file names them (191, C.7). It SHALL declare `kinds_dir`, with no
default, and `roles`, null unless set and then left out of the file,
each a string that is an absolute path or one under `~`. A key it does
not declare SHALL pass to the file as written, for flume to judge.

#### Scenario: No kinds_dir

- **WHEN** `nix/tests/scripts.sh` evaluates a source `acme` that gives `roles = "~/roles.toml"` alone and renders its file
- **THEN** evaluation fails naming `services.flume.sources.acme.kinds_dir`

#### Scenario: A kinds_dir under no home

- **WHEN** `nix/tests/scripts.sh` evaluates a source `acme` whose `kinds_dir` is `kinds/acme` and renders its file
- **THEN** evaluation fails naming `services.flume.sources.acme.kinds_dir`

#### Scenario: A key the module does not declare in a source

- **WHEN** `nix:test` renders a source `acme` giving `kinds_dir = "/opt/kinds/acme"` and `shelf = "upper"`, which the module does not declare
- **THEN** its file holds both as written and no `roles`

### Requirement: A source's name is one flume takes

The module SHALL fail evaluation where a source's name is not one or
more ASCII letters, digits, `_` and `-` (weir C.4), or is `flume`,
`repo` or `launch` (154, 191), with an assertion naming the source's
option, `services.flume.sources.<name>` with the name quoted where it is
no bare key, and why.

#### Scenario: Names flume refuses

- **WHEN** `nix:test` evaluates the options with each of the sources `flume`, `repo`, `launch`, `.acme`, `ac.me` and `a/b`, each giving `kinds_dir = "/opt/kinds"`
- **THEN** each fails with an assertion naming its option and why, and a source `acme-2_b` giving the same passes

### Requirement: Each source's file is rendered at evaluation

The module SHALL render each source to TOML at evaluation by the rules
it renders `settings` by, its first line saying the module wrote it from
`services.flume.sources.<source>`. A value the file cannot hold SHALL
fail evaluation, naming its path under `services.flume.sources.<source>`
and the values the file takes. The text rendered from `settings` SHALL
be unchanged.

#### Scenario: The sources render their files

- **WHEN** `nix:test` renders the sources of `nix/tests/sources.nix`, `acme` with `kinds_dir` and `roles` under `~` and `tools` with an absolute `kinds_dir` alone
- **THEN** their texts are `nix/tests/files/sources/acme.toml` and `nix/tests/files/sources/tools.toml` byte for byte, and the texts of `every-key.nix` and of the default `settings` are their files as before

#### Scenario: A path value in a source

- **WHEN** `nix/tests/scripts.sh` renders a source `acme` giving `kinds_dir = "/opt/kinds/acme"` and a path value at `shelf`
- **THEN** evaluation fails naming `services.flume.sources.acme.shelf` and the values the file takes

### Requirement: flume reads the rendered sources whole

flume's own reader of the host's sources SHALL read each source file
the module renders with no fault (191, C.7).

#### Scenario: flume reads the rendered sources

- **WHEN** `the_modules_sources_read_whole` in `flume:test` reads `nix/tests/files/sources` with `read_sources`
- **THEN** it declares `acme` and `tools`, each read with no fault: `acme` at the home's `.local/share/flume/kinds/acme` with its roles at the home's `.local/state/acme/flume/roles.toml`, and `tools` at `/opt/kinds/tools` with no roles

### Requirement: The activation links each declared source

The activation SHALL link `~/.config/flume/sources/<source>.toml` to the
rendered file of each source declared, replacing a link there, by a
link made beside it under a name beginning with `.` and renamed over
it, so no reader finds a linked source missing. It SHALL make
`sources/` where it is missing and a source is declared, and never
remove it. It SHALL say a line only where it moved, linked or removed
something, or failed.

#### Scenario: Sources in a fresh home

- **WHEN** `nix/tests/scripts.sh` runs `sources acme <file> tools <file>` in a scratch home with no `~/.config/flume/`
- **THEN** `sources/acme.toml` and `sources/tools.toml` are links to their files, the folder holds nothing else, and the step says it linked each

#### Scenario: A source's link of an earlier activation

- **WHEN** `sources acme <file>` runs where `sources/acme.toml` links to another file of acme's
- **THEN** it links to the new file, moves nothing aside, and the folder holds no name beginning with `.`

#### Scenario: No source in a fresh home

- **WHEN** `sources` runs with no source in a scratch home with no `~/.config/flume/`
- **THEN** it exits 0, says nothing and makes no folder

### Requirement: A declared source's file that is no link moves aside once

Where a declared source's `<source>.toml` is a file or a folder that is
no link, the activation SHALL move it once to
`<source>.toml.before-nix` and name both paths. Where that already
stands, it SHALL change neither, name both, link the other sources and
fail.

#### Scenario: A hand-written file of a declared source

- **WHEN** `sources acme <file>` runs where `sources/acme.toml` is a file
- **THEN** the file is moved to `sources/acme.toml.before-nix`, `sources/acme.toml` links to the module's file, and both paths are named

#### Scenario: A source's file aside already

- **WHEN** `sources acme <file> tools <file>` runs where `sources/acme.toml` is a file and `sources/acme.toml.before-nix` stands
- **THEN** it exits non-zero naming both paths, both files are as they were, and `sources/tools.toml` links to its file

### Requirement: A sources that is no folder is left as it stands

Where `~/.config/flume/sources` is there and is no folder, a file or a
link, the activation SHALL link and remove nothing in it, and SHALL
fail naming it where a source is declared.

#### Scenario: A sources that is no folder

- **WHEN** `sources acme <file>` runs where `~/.config/flume/sources` is a file, and then `sources` runs there with no source
- **THEN** the first exits non-zero naming it, the second exits 0 and says nothing, and the file is as it was

### Requirement: The activation removes only its own links

Where a source is no longer declared, the activation SHALL remove its
file where it is a link leading to a file whose name ends in
`-flume-source-<source>.toml`, the module's own, and SHALL leave every
other entry of `sources/` as it stands.

#### Scenario: A source no longer declared

- **WHEN** `sources tools <file>` runs where `sources/` holds `acme.toml` linking to a file named `…-flume-source-acme.toml`, a file `mine.toml`, a link `theirs.toml` to a file named `theirs.toml`, and `old.toml.before-nix`
- **THEN** `acme.toml` is gone and the step says it removed it, `tools.toml` links to its file, and `mine.toml`, `theirs.toml` and `old.toml.before-nix` are as they were, `theirs.toml` leading where it did

### Requirement: Each platform runs the sources step after the link

On nix-darwin the post-activation, and on NixOS the user activation of
`services.flume.user`, SHALL run the activation's sources step after the
link, as the same user under the same home, with each source declared
and its rendered file, `flume-source-<source>.toml`, in the order of the
sources' names, and with none where none is declared. Its failure SHALL
stop nothing else.

#### Scenario: Sources on nix-darwin

- **WHEN** `nix:test` evaluates the nix-darwin system of alice with `services.flume.sources` set from `nix/tests/sources.nix`
- **THEN** the post-activation runs, after the link, `activate.sh sources acme <file> tools <file> || true` as alice under `/Users/alice`, each file `flume-source-<source>.toml` rendered from its source

#### Scenario: Sources on NixOS

- **WHEN** `nix:test` evaluates the NixOS system with `services.flume.user` alice and the same sources
- **THEN** the user activation runs the same step after the link where the user is alice, and nothing for bob

#### Scenario: No source declared

- **WHEN** `nix:test` evaluates either system with no source
- **THEN** its activation runs `activate.sh sources` with no source after the link

## MODIFIED Requirements

### Requirement: flume reads the rendered file whole

flume's own reader of the host's configuration SHALL read each file the
module renders with no fault, under the names the file gives each key
(C.7).

#### Scenario: flume reads the rendered files

- **WHEN** `the_modules_files_read_whole` in `flume:test` reads each file directly under `nix/tests/files` with `HostConfig`, passing over the folder `sources/`
- **THEN** each reads with no fault, and `every-key.toml` gives the streams, pools, accounts, kinds, labels, repositories and agents settings `every-key.nix` sets

### Requirement: Nothing shaped as a credential reaches the store

The module SHALL fail evaluation where a string of `settings`, or of a
source of `sources`, is shaped as a credential by flume's own shapes
(core.50), read with its key's name. The failure SHALL name the
string's path, never its value, and say to name a variable of the login
environment instead (29). A string naming a variable in place of a
value SHALL pass.

#### Scenario: A token in an account's env

- **WHEN** `nix:test` evaluates the options with an account's `env.CLAUDE_CODE_OAUTH_TOKEN` set to a token's text
- **THEN** evaluation fails naming `services.flume.settings.pools.<pool>.accounts[0].env.CLAUDE_CODE_OAUTH_TOKEN`, the failure does not hold the token's text, and it says to name a variable as `$NAME`

#### Scenario: A variable named instead

- **WHEN** `nix:test` evaluates the options with that `env` value `$AGENT_TOKEN`
- **THEN** evaluation succeeds and the file holds `$AGENT_TOKEN`

#### Scenario: A token in a source's file

- **WHEN** `nix:test` evaluates the options with a source `acme` giving `kinds_dir = "/opt/kinds/acme"` and `token`, a key the module does not declare, set to a token's text
- **THEN** evaluation fails naming `services.flume.sources.acme.token`, and the failure does not hold the token's text

### Requirement: The activation writes no other file

The activation SHALL write nothing but the link of `config.toml`, the
link of each source `services.flume.sources` declares and each file it
moves aside, and SHALL remove nothing but a link of its own whose source
is no longer declared. It SHALL not write `config.local.toml`, a
source's file it did not link, a source's roles file or anything in its
`kinds_dir`, the host's lock, flume's state or data, nor any other
tool's file (191, C.7; core.45).

#### Scenario: A hand-written file there

- **WHEN** `link` runs in a scratch home whose `~/.config/flume/` holds a file `config.toml`, a `config.local.toml`, a `sources/` folder and a `flume.lock`
- **THEN** the file is moved to `config.toml.before-nix`, `config.toml` links to the module's file, both paths are named, and `config.local.toml`, `sources/` and `flume.lock` are byte for byte as they were

#### Scenario: The sources beside the rest of the folder

- **WHEN** `sources acme <file>` runs in a scratch home whose `~/.config/flume/` holds `config.toml`, `config.local.toml` and `flume.lock`
- **THEN** `sources/acme.toml` links to its file, and `config.toml`, `config.local.toml` and `flume.lock` are byte for byte as they were

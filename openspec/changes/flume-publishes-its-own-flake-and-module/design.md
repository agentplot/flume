# Design

## Context

Record 0055 (accepted by the user on 2026-11-06) decides that flume
publishes a flake of its own. Its `services.flume` module, for macOS
and NixOS, writes flume's whole managed file,
`~/.config/flume/config.toml`, from options named as the file names
them: its host, streams, pools with their accounts, kinds and
repositories. It writes no other tool's file. agentplot/boomdrive's
record 0023 (accepted the same day) has the Boomdrive flake take flume's
flake as a locked input and re-export its module, so a host adds the
Boomdrive flake alone and each tool can be used without it.

The Boomdrive flake already does all of this for its runner, and builds
flume from a tag by a package of its own. Its change
`boomdrive-publishes-a-flake-of-the-family-and-the-runners-service`
(archived 2026-11-06, agentplot/boomdrive, design Decisions 1 to 14) is
the family's design for a tool's flake. Its tree at `nix/` is the
working model this change follows: the package, the rendered file, the
link at activation, and the gate that evaluates without a store. Where
this design says "as Boomdrive's", it means that design's decision and
that tree's file at agentplot/boomdrive main. The family has one design
for a tool's flake, and flume follows it rather than inventing another.
Where flume's differs, the difference is named below with its reason.

The statement fixes what the module writes: the managed layer's tables
and keys (143, C.7), named files relative to the configuration's
directory (195), an account's `env` read against the host user's login
environment (29), and the local layer as the operator's, never the
managed layer's writer's (C.7).

## Goals / Non-Goals

Goals:

- A flake a host can take alone, or through Boomdrive's, for flume's
  package and its module, on aarch64-darwin, aarch64-linux and
  x86_64-linux.
- `services.flume.settings` is flume's whole managed file, each key
  named as C.7 names it, rendered to the text flume reads.
- Nothing shaped as a credential reaches the store.
- The gate proves the flake by evaluation, and CI proves that the
  package builds.

Non-goals:

- The local layer, `config.local.toml`. It is the operator's, or a
  control plane's (C.7), and the module never writes it.
- A source's file under `sources/`, the host's lock and the kinds'
  packages. They belong to the source's tool and to `flume kinds lock`
  (191, 155).
- Reading an account's usage. That is record 0055's open design. The
  module declares `usage` as C.7 states it today.
- The herdr plugin's package, a binary cache, herdr, Ghostty and the
  Boomdrive app. Those stay Boomdrive's (its record 0023).
- A home-manager module. Record 0055 names macOS and NixOS, and
  Boomdrive's modules are nix-darwin's and NixOS's.
- Naming a repository by its address. That is record 0055's other part,
  built by its own change on this bolt. The module takes a `repos` key
  as written, an address or a folder.

## Decisions

### 1. Where it lives, and its project

`flake.nix` and `flake.lock` are at the root, so `github:agentplot/flume`
is the flake. Everything else of it is under `nix/`, as Boomdrive's
Decision 1 lays it out:

```
nix/moon.yml
nix/inputs.nix              the lock's inputs, fetched or located, for the shell and the tests
nix/packages/flume.nix      flume, from this tree
nix/packages/git-crates.nix each git crate of Cargo.lock: its revision and hash
nix/modules/options.nix     services.flume, shared by both modules
nix/modules/toml.nix        the file's text
nix/modules/credential.nix  flume's shapes of a credential
nix/modules/darwin.nix
nix/modules/nixos.nix
nix/modules/activate.sh     the link
nix/check.sh                the project's type check and test
nix/tests/                  the evaluated tests, the rendered files and the link's cases
```

The moon project is `nix`, at `nix/`, `language: 'nix'`, listed in
`.moon/workspace.yml` (core.21). It has `format`, `lint`, `typecheck`
and `test` in the check-fast layer, and `build` in no layer (Decision
11).

Alternatives:

- **The flake under `nix/`**, as `github:agentplot/flume?dir=nix`. Not
  taken: every reference would carry the query, and Boomdrive's flake is
  at its root.
- **The flake's checks in the root project.** Not taken: core.21 gives
  each part with its own tooling a project of its own.

### 2. The package

`nix/packages/flume.nix` builds flume as Boomdrive's
`nix/packages/flume.nix` does, with `src` this tree, the flake's
`self`:

- Rust at the version `.prototools` names, from rust-overlay's
  `rust-bin.stable.<version>.minimal`, so the package and the gate
  compile with one toolchain (core.22).
- The crates by `Cargo.lock`'s checksums, and each git crate by
  `nix/packages/git-crates.nix`. That file maps each git package of
  `Cargo.lock`, by `<name>-<version>`, to the revision its hash was
  taken at and the hash of `fetchgit`'s tree there. Today these are
  `weir-capture-0.0.0` and `weir-kinds-0.0.0`, both at weir's
  `5562af52`. A revision moves with its hash, in one commit, and
  `nix:test` fails where an entry's revision is not `Cargo.lock`'s or a
  git package has no entry.
- `-p flume --bin flume`, its version `Cargo.toml`'s
  `[workspace.package]` version. That is `0.0.0` on main and the
  release's at a release tag, which the release job writes
  (set-version.sh), so no new file carries flume's version.
- `doCheck = false`: flume's tests need git, herdr and sluice, and the
  gate runs them.
- `meta.mainProgram = "flume"`, its platforms the flake's systems, and
  its licence MIT.

`packages.<system>` is `{ flume; default = flume; }` for aarch64-darwin,
aarch64-linux and x86_64-linux, the systems flume releases. Each is made
with the flake's own nixpkgs and rust-overlay.

Alternatives:

- **The release archives fetched by hash.** Not taken: a host that
  trusts no binary cache could not build, and every release would move
  three hashes in the flake.
- **The tree filtered to what the build reads.** Not taken: the
  binary's crates read files outside `crates/`, under `plugins/claude/`
  and `.config/`. A filter would fail on the first file it missed,
  while the whole tree only costs a rebuild per commit.

### 3. The inputs

- `nixpkgs`, `github:NixOS/nixpkgs/nixos-unstable`, the branch
  Boomdrive's flake takes.
- `rust-overlay`, `github:oxalica/rust-overlay`, its `nixpkgs`
  following the flake's.
- `nix-darwin`, `github:nix-darwin/nix-darwin`, its `nixpkgs`
  following the flake's, for the tests' darwin systems alone. A host's
  configuration uses its own, which changes no package.

A taking flake writes `inputs.flume.inputs.nixpkgs.follows = "nixpkgs"`
and `inputs.flume.inputs.rust-overlay.follows = "rust-overlay"`. With
Boomdrive's, flume's package is then the derivation Boomdrive's cache
holds. The flake has no `nixConfig`, since it fills no cache of its
own.

### 4. The outputs

| output | what |
|---|---|
| `packages.<system>.flume`, `.default` | Decision 2 |
| `darwinModules.flume`, `.default` | `nix/modules/darwin.nix` |
| `nixosModules.flume`, `.default` | `nix/modules/nixos.nix` |

Nothing else: no overlay, no app, no development shell (the shell is
devenv's) and no `checks`, since the gate runs the project's tasks and
`nix flake check` cannot run in its sandbox (Boomdrive's Decision 14).

Each module is an attribute set with `_file` and `key =
"github:agentplot/flume#<platform>Modules.flume"`, whose `imports` holds
the module itself. The module system takes a module once per key, so a
host that imports flume's module directly and through Boomdrive's
declares `services.flume` once. When two revisions of flume's module
are imported, the first one taken wins. A host that imports both takes
them at one revision by following one input.

### 5. The options

Both modules import `nix/modules/options.nix`, as Boomdrive's Decision
8 has its two:

| option | type | default | |
|---|---|---|---|
| `services.flume.enable` | bool | false | install flume and write its file |
| `services.flume.package` | package or null | the flake's `flume` for the host's system | null installs none, and the file is still written |
| `services.flume.settings` | the file | `{ }` | Decisions 6 and 7 |
| `services.flume.user` | string | none | NixOS: whose file it is |

On nix-darwin the user is `system.primaryUser`, which the module asks
for by `system.requiresPrimaryUser`, as Boomdrive's does, and whose
`users.users.<user>.home` it asserts set. On NixOS it is
`services.flume.user`, asserted declared by `users.users` as a normal or
a system user.

`package` defaults to the flake's own package for the host's system, so
under Boomdrive's following inputs it is the path Boomdrive's cache
holds. An unknown system fails evaluation, naming the systems the flake
gives. `environment.systemPackages` gains `package` where it is not
null.

flume runs no service (core.13), so neither module declares a launchd
agent or a systemd unit.

### 6. `settings`: the file's tables and keys

`settings` follows RFC 42: free-form TOML, with the tables and keys
C.7 names declared for their types and documentation. Each declared
key, a table among them, is nullable with no default, and a null is
left out of the file, so flume's own default holds and the two never
drift (Boomdrive's Decision 8). So a table the host never sets writes
nothing, and one it sets empty writes its header (Decision 7). A key the
module does not declare passes to the file, and flume judges it when it
reads the file.

| table | declared keys |
|---|---|
| `host` | `agents` (unsigned), `multiplexer`, `pool`, `terminal_app` (strings), `view` (`terminal` or `app`), `pin` (list of strings) |
| `streams.<stream>` | `agents`, `points` (unsigned), `priority` (integer), `pool`, `session`, `multiplexer`, `terminal_app`, `title` (strings), `view` (`terminal` or `app`), `pace` (`points` unsigned, `per` one of `hour`, `day`, `week`), `labels.<label>` (`reserve`, `keep_free`, unsigned), `kinds.<kind>` (`session`, and `stages.<stage>.session`) |
| `pools.<pool>` | `strategy` (`fill-first`, `least-busy` or `most-left`), `threshold` (1 to 100), `accounts`, a list of `name`, `program`, `usage` (strings), `env` (attribute set of strings) and `agents` (unsigned) |
| `kinds.<kind>` | `stream`, `label` (strings), `agents` (a free-form table) |
| `repos.<repository>` | `stream` (string), `agents` (a free-form table), `kinds.<kind>` (`stream`, `label`, `agents`) |
| `agents` | a free-form table |

The rule is that a key is typed where C.7 gives it one type and a fixed
set of words, and an agents table is free-form whole. An agents table's
keys are many, layered and judged at each start with status naming each
(32, 195 to 198), and a second copy of those rules in Nix would drift
from flume's.

The module names no tool (core.1). A multiplexer, a terminal app or a
program is a string the host writes, and flume reads it against its
profiles (190; C.8). `host.terminal`, the older word for `multiplexer`,
is not declared, so it passes to the file as written and flume reads it
as C.7 says.

The module never checks a `repos` key. It may be an address,
`<owner>/<name>`, or a folder, as record 0055 has it.

Each table is an attribute set of submodules, so two modules' values
merge into one file. When Boomdrive's module sets
`services.flume.settings.repos."acme/acme".stream` and the host sets
`streams`, both are written.

Alternatives:

- **Only declared keys.** Not taken: a key a later flume reads would
  need the module first.
- **One option per key at the top**, `services.flume.streams`. Not
  taken: the option and the file would give each key two names, where
  record 0055 asks for the file's names.
- **Every agents table key typed.** Not taken, for the drift said above.

### 7. The file

- **Rendered in Nix**, as Boomdrive's Decision 9 has it, so the gate
  compares the text without a build. `nix/modules/toml.nix` follows
  Boomdrive's `nix/modules/toml.nix`, with two changes. flume's file
  takes floats, written with a point or an exponent so they never read
  as integers. It also takes any list: a non-empty list of tables only
  is written as `[[...]]` sections, and any other list inline, a table
  in it as an inline table. A program's own settings hold such lists
  (195). It refuses any other value, a path or a derivation among them,
  naming its path. Top-level keys come before tables, each sorted by
  name. A key is bare where it can be and quoted otherwise, so
  `[repos."acme/acme"]`. An empty table is written as its header, since
  `[repos."<repository>"]` alone names a repository flume reads. The
  first line is `# Written by flume's module from services.flume.settings;
  a change goes there.`, and the text ends in a newline.
- **In the store**, as `pkgs.writeText "flume-config.toml"`.
- **Linked at flume's path.** The activation links the user's
  `~/.config/flume/config.toml`, the path record 0055 names, to that
  file, as Boomdrive's links the runner's. A link there is replaced. A
  file or folder there that is no link is moved once to
  `config.toml.before-nix`, and the activation names both paths. Where
  `config.toml.before-nix` is there already, it changes neither, names
  both, and fails the link alone. The rest of the activation goes on,
  and flume reads the file left in place. `nix/modules/activate.sh link
  <file>` does this, as Boomdrive's `activate.sh link` does. It makes
  `~/.config/flume/` where it is missing and touches nothing else in it:
  not `config.local.toml`, `sources/`, `flume.lock`, nor a named file.
- **flume reads it as before.** flume reads
  `$XDG_CONFIG_HOME/flume/config.toml` at each command. A named file's
  relative path is read from that directory (195), the link's folder and
  not the store's, so `file = "people-settings.json"` beside it reads as
  it did.

The text is held to flume's own reader. Each file under
`nix/tests/files` is rendered byte for byte by `nix:test`, and read
whole by `HostConfig` in `flume:test`. So a key the module writes is a
key flume reads, under the name the file gives it. Boomdrive's
`module_files.rs` ties its rendered files to its runner's reader the
same way.

Alternatives:

- **`pkgs.formats.toml`.** Not taken: its text is a build's output,
  which the gate cannot read without building (Boomdrive's Decision 9).
- **Boomdrive's renderer taken from Boomdrive's flake.** Not taken:
  Boomdrive takes flume's flake, so flume cannot take Boomdrive's, and
  flume must stand without Boomdrive (record 0055). The renderer follows
  Boomdrive's design and cites it. The family's one design is
  Boomdrive's Decision 9, and flume's two additions are named above.
- **A real file written at activation, at 0600.** Not taken, as
  Boomdrive's Decision 9 says: it needs a writer of its own on each
  platform. Decision 8 keeps the file free of credentials, so the store
  hides nothing a link would show.

### 8. No credential in the store

The store is readable by every user. A host's hand-written file is not,
and may hold what the store must not.

- `nix/modules/credential.nix` is flume's own `shaped_as_credential`
  (flume-core's `own.rs`, design 7 of the change that made it, as
  core.50 applies it) in Nix. It covers an issuer's prefix opening a run
  of sixteen or more, a private key's opening, a JSON web token, a run
  of 32 or more mixing upper case, lower case and digits apart from any
  variable named, and the value of a key named as a secret's that names
  no variable.
- `options.nix` walks every string of `settings`, each read with the
  name of the key whose value it is, a list's items with the list's key.
  The first shaped as a credential fails an assertion, naming its path,
  such as `services.flume.settings.pools.work.accounts[0].env.CLAUDE_CODE_OAUTH_TOKEN`,
  and never its value. The message says to name a variable of the host
  user's login environment instead, `$NAME`, which flume reads at each
  start and the store never holds (29).
- **One table of cases.** The cases of flume-core's three tests,
  `each_shape_is_a_credential`, `a_value_that_passes` and
  `a_variable_named_rather_than_a_value`, move into
  `crates/flume-core/tests/credential-shapes.json`, each a `name`, a
  `text` and whether it is `shaped`. Those tests read it, and
  `nix:test` runs every case through the port. So the two
  implementations of one rule are held to one table, and a case added
  for either is a case for both.

flume's file read by hand is unchanged: flume refuses an own setting
shaped as a credential at a launch (core.50) and gives an `env` value as
`<set>` in status, as today. The module's refusal is stricter only
because the store is where it writes.

Alternatives:

- **Boomdrive's rule, a key named for a credential.** Not taken for
  flume's file: flume already has one rule for a credential's shape, and
  a second rule for the same file would refuse and pass different
  values. Boomdrive's rule stays its runner's, for its runner's file.
- **flume's binary run over the file at build.** Not taken: the text is
  in the store as soon as the derivation is instantiated, so a check
  after evaluation is too late.

### 9. Activation on each platform

- **nix-darwin.** `system.activationScripts.postActivation` runs
  `activate.sh link <file>` as the primary user, under their own home,
  by `sudo --user=<user> -- /usr/bin/env HOME=<home> /bin/sh`, and
  `|| true` so a failed link stops nothing else, as Boomdrive's does.
- **NixOS.** `system.userActivationScripts.flume` runs `activate.sh link
  <file>` where `id -un` is `services.flume.user`, and does nothing for
  any other user.

There is no restart step, since flume runs no service. The next command
reads the new file, and the next agent's start takes its settings (32).

### 10. What the module never writes

The module writes `~/.config/flume/config.toml`, the link and, where
one stood, `config.toml.before-nix`. It writes nothing else of flume's:
`config.local.toml`, `sources/`, `flume.lock`, `$XDG_STATE_HOME/flume/`
and `$XDG_DATA_HOME/flume/` stay as they stand (C.7; core.45). It writes
nothing of another tool's. A setting another tool needs is set through
that tool's module by whoever composes them, Boomdrive's module among
them (Boomdrive's record 0023).

### 11. The gate and CI

As Boomdrive's Decision 14:

- **The inputs are there first.** `devenv.nix` reads `flake.lock`
  through `nix/inputs.nix`, which fetches each locked input by
  `builtins.fetchTree` during `devenv print-dev-env`, outside the
  sandbox. It gives the tasks their paths as `FLUME_FLAKE_INPUTS`.
- **The tasks read no store.** `nix/check.sh typecheck|test` runs the
  pinned nix as `nix eval --raw --offline --read-only --impure --store
  'dummy://?store=/flume-check/store' --extra-experimental-features
  nix-command --file nix/tests <check>`, its configuration, caches and
  state under a scratch folder. `nix/tests/lib.nix` calls `flake.nix`'s
  `outputs` with the inputs and `self` as this tree. Each failure is one
  line, `nix: <case>: <what>`.
- **`nix:format`**: `nixfmt --check` on `flake.nix` and `nix/`.
- **`nix:lint`**: `statix check` and `deadnix --fail` on the same files.
- **`nix:typecheck`**: every package of every system evaluates to a
  derivation, `default` is `flume`, each module evaluates in a minimal
  system of its platform, and an unknown system's evaluation names the
  systems.
- **`nix:test`**: the evaluated cases of the flake, host-module and
  the-gate specs, the CI workflow's `flake` job held by its text among
  them, and `nix/tests/scripts.sh`'s cases of `activate.sh` and
  `check.sh`, run by `sh` in scratch homes.
- **`nix:build`**, in no layer: `nix build --no-link .#default` for
  the host's system. It needs the nix daemon and the network, which the
  gate's sandbox denies, so a comment above it says why (root:layers).
  CI's new `flake` job runs it on `ubuntu-latest` and `macos-latest` in
  the devenv shell, for each pull request and each push to main. So a
  package that no longer builds, a hash a weir bump left behind among
  causes, fails before Boomdrive takes the tag.
- **`flume:test`** gains `the_modules_files_read_whole` in
  `tests/host.rs`, which reads each file under `nix/tests/files` with
  `HostConfig`. Its task's inputs gain `/nix/tests/files/**/*`.
- **Pins.** nix, nixfmt, statix and deadnix are `devenv.nix`'s, each
  asserted to its version, and `check-toolchain.sh` holds them (core.22).
  statix answers no `--version`, so the check reads its version from
  the name of its program's store path, as Boomdrive's does. treefmt
  already runs nixfmt and deadnix over every `.nix` file. `treefmt.nix`
  excludes `nix/tests/files/**`, which the tests compare byte for byte
  (core.21).

### 12. The operator's pages

The site's install page gains "Install with Nix". It shows the flake's
input, a module imported on nix-darwin or NixOS, and a short `settings`.
It says the module writes `~/.config/flume/config.toml` and never the
local layer, and that no credential goes in `settings`, only a variable
named by `$NAME`. The configuration reference says the managed layer
may be written by the module. Both speak to any host's operator and name
Nix, nix-darwin and NixOS as products flume integrates with (core.35).
README's file table gains `flake.nix` and `nix/`, and CLAUDE.md's list of
projects gains `nix`.

### Rules relied on

- core.13: no process of flume's, so the module runs no service.
- core.21: the `nix` project, one task per check, and every tracked file
  formatted or excluded with its reason.
- core.22: nix, nixfmt, statix and deadnix pinned.
- core.45: flume writes neither layer. Its details gain a line saying
  that a host's Nix writing the managed layer through flume's module is
  the host's manager writing it, and that the module writes neither the
  local layer nor a credential (record 0055).
- core.50: flume's shapes of a credential, and nothing shaped as one held.
- core.1: the module names no tool.
- core.35: the site speaks to any host's operator.
- core.16: nothing of flume's in a repository's content. The module
  writes the host's configuration, never a repository.
- kit:specs.3 and kit:testing.5: each scenario a moon task proves.

### Rules departed from

None.

## Risks / Trade-offs

- **A link into the store outlives `enable`.** A host that turns the
  module off keeps the link until it removes it, and once the store
  path is collected, flume reads no file there, as with none. This is
  Boomdrive's trade too.
- **Another `XDG_CONFIG_HOME`.** A shell that sets one reads another
  file. The module links the default path, which record 0055 names.
- **Two revisions of the module.** The first one imported wins
  (Decision 4). A host that imports flume's module directly and through
  Boomdrive's takes both at one revision by following one input.
- **The port can drift from the core.** The shared table holds them to
  every case either has, but a shape added to flume-core with no case
  would pass the port. A shape is added with its case, as flume-core's
  tests already ask.
- **A whole-tree source.** Every commit changes the package's
  derivation. A host takes a release tag, so it rebuilds once per
  release.

## Migration Plan

A host that writes `~/.config/flume/config.toml` by its own tooling
moves those values into `services.flume.settings` and enables the
module. The first activation moves the hand-written file to
`config.toml.before-nix` and links the module's file. Its
`config.local.toml` stays as it is. Boomdrive's change, after this
lands and is released, takes flume's flake at its tag, re-exports
`darwinModules.flume` and `nixosModules.flume`, sets `services.flume`
options from its owner block, and drops its own build of flume.

## Open Questions

None for this change. How flume reads an account's usage stays record
0055's open design, and the module's `usage` key follows C.7 until then.

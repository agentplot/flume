# Design

## Context

flume's module, `services.flume` for nix-darwin and NixOS, renders
`config.toml` from `settings` at evaluation, puts it in the store and
links the user's `~/.config/flume/config.toml` to it at activation
(record 0055; the archived change
`2026-11-07-flume-publishes-its-own-flake-and-module`, its design 5 to
10). That change kept a source's file out of the module on purpose. It
said a source's file belongs to the source's tool. Its activation
writes nothing under `sources/`, and core.45's details rule out the
module writing a source's file.

agentplot/boomdrive's record 0034 (accepted 2026-11-09 by
agentplot-design), point 2, settles who writes a source's file on a
host Nix manages. The sources are flume's managed layer (core.45,
record 0040), so flume's module writes them from options named as the
file names them, as it writes `config.toml`. The source's tool still
writes what the file names: crew writes its roles file and makes its
kinds folder (crew record 0057, points 1 and 5), and the host declares
the source. swancloud writes `sources/crew.toml` by an activation of its
own today, as a real file renamed into place, and removes it on a Mac
that declares crew no more (`lib/claude-account-tools.nix`). That writer
goes once Boomdrive's module declares crew's source through flume's.

The statement fixes the file. `sources/<source>.toml` gives
`kinds_dir`, an absolute path or one under `~`, and may give `roles`,
the same (191, 193, C.7). flume reads every file there whose name ends
in `.toml` and begins with no `.`, a link as the file it leads to
(`read_sources`, held by `entries_that_are_no_source` in
`crates/flume/tests/host.rs`). It refuses a key it does not know, and a
source's name weir's names do not allow or that is `flume`, `repo` or
`launch`, each refusing the launches that name the source alone. A
person, or a program that manages the host, writes a source's file
whole, replaces it whole and removes it, touching no other source's
file (191).

## Goals / Non-Goals

Goals:

- A host Nix manages declares each of its sources through
  `services.flume.sources`, and keeps no writer of its own for flume's
  managed layer.
- A source's file the module writes is one flume reads with no fault.
- The module's activation never touches a source it did not write.

Non-goals:

- The roles file a source names, and the packages in its `kinds_dir`.
  Those are the source's own tool's (C.7; crew record 0057), and the
  module neither writes them nor makes their folders.
- crew's source. Boomdrive's module declares it through this option in
  its own unit, `the-set-declares-crews-source-to-flume`.
- The host's lock, `flume.lock`. It is `flume kinds lock`'s (155).
- `config.toml`'s link. It stays as the archived change built it.

## Decisions

### 1. One option per source, named as the file names it

`services.flume.sources` is an attribute set of sources, by name,
default `{ }`. Each source is a free-form TOML table, as a table of
`settings` is, whose declared keys are the file's:

| key | type | |
|---|---|---|
| `kinds_dir` | string matching `/.*`, `~` or `~/.*` | required: no default |
| `roles` | the same, or null | null leaves it out of the file |

So the option's path names the file and its keys:
`services.flume.sources.acme.kinds_dir` is `kinds_dir` in
`sources/acme.toml`. The option sits beside `settings`, not under it,
since a source is a file of its own and no table of `config.toml`.

`kinds_dir` has no default, so a source the host declares without one
fails evaluation, naming the option, where flume would read a file that
refuses every launch naming the source. A path neither absolute nor
under `~` fails the type the same way. C.7 gives each key that one
form, so the module types it, as design 6 of the archived change types
a key C.7 gives one type. A key the module does not declare passes to
the file as written, and flume judges it, as with `settings`. A later
flume's key then needs no newer module, which matters to a host that
keeps its own build of flume under the set's module (boomdrive record
0034, point 7).

A `kinds_dir` may be a store path, `"${pkgs.acme-kinds}/kinds"`. The
rendered file then refers to it, so the store keeps the packages for as
long as it keeps the file.

Alternatives:

- **Under `settings`, as `settings.sources.<source>`.** Not taken: the
  file holds no `[sources]` table, and flume refuses a `config.toml`
  that gives one. The option's path would name a table no file has.
- **Only declared keys.** Not taken, for the archived design 6's
  reason: a key a later flume reads would need the module first.
- **`kinds_dir` taking a Nix path, `./kinds`.** Not taken: the renderer
  refuses a path value anywhere in `settings`, and a string, `"${./kinds}"`,
  puts the folder in the store just as well.

### 2. A source's name flume takes, checked at evaluation

The module asserts each source's name. It must be one or more ASCII
letters, digits, `_` and `-`, which is weir's name (`weir_kinds::check_name`,
weir C.4). It must also not be `flume`, `repo` or `launch`, the sources
of flume's own kinds, the repository's and a launch's (`source_fault`
in `crates/flume/src/profiles/kinds/weir/mod.rs`). Where one fails, the
assertion names `services.flume.sources.<name>`, the name quoted where
it is no bare key, and says why.

flume would take such a file as a fault of its own source alone, so
this check is the module's, for three reasons. The name is the file's
name, and the module makes the file. A name holding `/` would write
outside `sources/`. A name beginning with `.` would be passed over by
flume with no word in status.

Alternatives:

- **Let flume judge the name, as it judges an undeclared key.** Not
  taken: a name with `/` lands the file elsewhere, and a leading `.`
  hides the source, neither of which flume can name.
- **Read weir's rule from the crate at evaluation.** Not taken: Nix
  cannot. The rule is weir's C.4, one character class and three words.
  `the_modules_sources_read_whole` holds the module's rendered sources
  to flume's reader (decision 6).

### 3. Each source rendered at evaluation

Each source's file is rendered by `nix/modules/toml.nix`, the renderer
of `config.toml`, so one set of rules gives every file of the managed
layer its text (the archived design 7). The renderer is given the
option it renders. Its first line names it, as `# Written by flume's
module from services.flume.sources.acme; a change goes there.`, and its
failures name the value's path under it, as
`services.flume.sources.acme.kinds_dir`. `config.toml`'s first line and
its failures stay byte for byte as they are, since `settings` is the
option there.

Each file goes into the store as `pkgs.writeText
"flume-source-<source>.toml"`. The name marks the store file as the
module's (decision 5).

### 4. No credential in the store

The credential walk of `options.nix` covers `sources` as it covers
`settings`. Each string of a source is read with its key's name, by the
same shapes (`nix/modules/credential.nix`; core.50). The first one
shaped as a credential fails the same assertion, naming its path,
`services.flume.sources.<source>.<key>`, and never its value. A source's
declared keys are paths, but an undeclared key passes, and the store
holds whatever it holds.

### 5. The activation links each source and removes only its own

`nix/modules/activate.sh` gains a step:

```
activate.sh sources [<source> <file>]...
```

run as the user whose file it is, after `link`, with each source the
module declares and its rendered file, in the order of the sources'
names. It works in `~/.config/flume/sources/`:

- **A declared source** is linked at `<source>.toml`, as `config.toml`
  is. Where the link there already leads to its file, nothing changes.
  Where a link there leads elsewhere, it is replaced. Where a file or a
  folder there is no link, it is moved once to
  `<source>.toml.before-nix`, which flume passes over since its name
  does not end in `.toml`, and both paths are named. Where
  `<source>.toml.before-nix` stands already, neither is changed, both
  are named, and that source alone is not linked: the step goes on with
  the others and fails at its end.
- **The link is renamed into place.** The new link is made beside the
  file under `.<source>.toml.<pid>`, a name flume passes over, and
  renamed over `<source>.toml`. So a reader at any instant finds the old
  file or the new one and never no source, as 191 has a source's writer
  rename its new file over the old. An old link that leads to a folder
  is removed first, since a rename over it would move the new link into
  that folder.
- **A source no longer declared** is removed where its file is a link
  the module made: a link `<name>.toml` leading to a file whose name
  ends in `-flume-source-<name>.toml`. That is the store name of
  decision 3. Every other entry is left as it stands: a person's own
  file or link, another program's, a file moved aside, and a temporary
  name.
- **The folder.** It is made where it is missing and a source is
  declared, and left missing where none is. It is never removed. Where
  `sources` is there and is no folder, a file or a link, nothing is
  linked or removed in it. Where a source is declared, the step names it
  and fails. Where none is, it says nothing.
- **Its words.** It says a line, beginning `flume:`, only where it
  moved, linked or removed something, or failed. It exits 1 where any
  source was not linked, and 0 otherwise.

The step runs at every activation of an enabled module, with no source
too. So a host that drops its last source loses the module's link to
it at the next switch.

Alternatives:

- **Link the whole `sources/` folder to one folder in the store.** It
  is simpler, and swaps every source at once. Not taken: 191 has each
  source's file written, replaced and removed whole without touching
  another's. A link over the folder would move a person's own source
  aside, and a program that manages the host, a marketplace's among
  them (record 0040), could add none beside the module's.
- **Real files written at 0644, as swancloud's activation writes
  them.** Not taken: `config.toml` is a link, and a link shows which
  option wrote the file. A real file the module wrote cannot be told
  from a person's by anything it holds, and its first line can be
  copied.
- **A list of what the last activation linked, kept in the folder.** Not
  taken: it is state of the module's own in flume's folder, and it
  drifts once a person removes a link by hand. The store file's name
  marks the module's links without one.
- **`ln -sfn`, as `config.toml`'s link is made.** Not taken for the
  sources: flume reads them at each launch, wake and start (191), and
  `ln -sfn` leaves an instant with no file.

### 6. flume reads the rendered sources whole

`nix/tests/sources.nix` declares two sources:

- `acme`, whose `kinds_dir` is `~/.local/share/flume/kinds/acme` and
  whose `roles` is `~/.local/state/acme/flume/roles.toml`;
- `tools`, whose `kinds_dir` is `/opt/kinds/tools` and which gives no
  roles.

Their files are `nix/tests/files/sources/acme.toml` and `tools.toml`.
`nix:test` holds each to its text byte for byte. `flume:test` reads the
folder with `flume::host::config::read_sources`, the reader flume's
host uses, and finds each source read with no fault: `acme` at the
home's kinds folder with its roles file, and `tools` at its folder with
none. So `nix/tests/files/` reads as a configuration directory reads,
`config.toml`'s renderings beside `sources/`.
`the_modules_files_read_whole` reads the folder's files alone and
passes over `sources/`. `crates/flume/moon.yml`'s test inputs already
hold `/nix/tests/files/**/*`, and `treefmt.nix` already excludes
`nix/tests/files/**`.

### 7. Each platform runs the step

- **nix-darwin.** `system.activationScripts.postActivation` runs
  `activate.sh sources …` after the link, as the primary user under
  their home, by the same `sudo --user=<user> -- /usr/bin/env
  HOME=<home> /bin/sh`, each argument quoted by `lib.escapeShellArg`,
  and `|| true`.
- **NixOS.** `system.userActivationScripts.flume` runs it after the
  link, inside the same check that `id -un` is `services.flume.user`,
  and `|| true`.

So a failed source stops neither the link nor the rest of the
activation, as a failed link stops nothing.

### 8. The record and the rule

Record 0055's second point, "flume publishes its own module", gains a
dated amendment line. It cites agentplot/boomdrive's record 0034, point
2, and this change's design: the module writes each source
`services.flume.sources` declares, at `sources/<source>.toml`, as a
link into the store renamed into place. It removes only its own links,
and writes no source it does not declare.

core.45's details change in the same commit (`docs/architecture/core.md`'s
own rule for a rule and its decision):

- Its "Also rules out" says flume's module writing a source's file *it
  does not declare*, and adds removing a source's file it did not link.
- Its "Not ruled out" says a host's Nix writing `config.toml` whole *and
  each source's file it declares whole*.

Each cites record 0055 as amended, with boomdrive record 0034 among the
sources. core.45's words stand, since the rule is that flume writes
neither layer, and the module is the host's manager writing it (record
0055).

No clause changes. 191 already has a source's file written whole by a
program that manages the host. C.7's "writes none" is flume's, as its
"never by flume" of `config.toml` was when the module took that file.

### 9. The operator's pages

`site/src/content/docs/install.mdx`, under "Install with Nix", shows a
source declared through `services.flume.sources`. It says the module
links `~/.config/flume/sources/<source>.toml` for each and removes only
its own links, and that the roles file and the packages are the
source's tool's. `site/src/content/docs/reference/configuration.md`,
under "The host's sources", says flume's Nix module writes each source
it declares. Both speak to any host's operator (core.35). Their example
source is `acme`.

### Rules relied on

- core.45: flume writes neither layer. The module is the host's
  manager, and its details are amended by decision 8.
- core.50: nothing shaped as a credential reaches the store.
- core.1: the module names no tool. A source's name and its paths are
  strings the host writes.
- core.13: the activation is the host's switch, and no process of
  flume's.
- core.16: nothing of flume's goes in a repository's content.
- core.21: the files the tests compare are excluded by name already.
- kit:specs.3 and kit:testing.5: each scenario a moon task proves.

### Rules departed from

None.

## Risks / Trade-offs

- **A store file's name marks the module's links.** A person's own link
  to a file whose name ends in `-flume-source-<name>.toml` is taken for
  the module's and removed once its source is not declared. No other
  writer names a file so.
- **A writer of the host's own beside the module.** A host that keeps
  its own writer of a source the module declares sees the two take
  turns: its file is moved aside once, and the next activation fails
  that source, naming both paths. A host drops its writer in the switch
  that declares the source through the module.
- **The links outlive `enable`.** A host that turns the module off
  keeps its links until it removes them, as with `config.toml`'s.
- **weir's name rule in Nix.** A change of weir's rule leaves the
  module's check behind until it is changed too. The rendered sources
  are read by flume's reader, so a name the module takes and flume
  refuses fails `flume:test` once a test source carries it.

## Migration Plan

swancloud's activation writes `sources/crew.toml` as a real file today.
Once flume's release holds this change, Boomdrive's unit declares
crew's source through `services.flume.sources.crew`, and swancloud
drops its writer in the same switch. The first activation then moves
swancloud's file to `crew.toml.before-nix` once, names both paths, and
links the module's file. A Mac that declares crew no more loses the
module's link at its next switch, as swancloud's `rm -f` removed its
file. A host removes `crew.toml.before-nix` by hand once it has read it.

## Open Questions

None.

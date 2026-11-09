# flume's module writes the host's sources

## Why

A host's sources of kinds are files of flume's managed layer, each
`sources/<source>.toml` beside `config.toml`, naming where the source's
packages lie and where it declares its roles (191, C.7; record 0040).
flume's Nix module writes `config.toml` whole from
`services.flume.settings` (record 0055), but writes no source's file.
So a host Nix manages keeps a writer of its own for the rest of flume's
managed layer: swancloud's activation writes `sources/crew.toml` beside
the module's link today.

agentplot/boomdrive's record 0034 (accepted 2026-11-09 by
agentplot-design), point 2, decides that a host's sources are flume's
module's: "Flume's module writes them from options named as the file
names them, as it writes `config.toml`." It rules out a host keeping its
own writer of flume's managed layer, and crew declaring its own source
inside flume's folder. Its unit `the-set-declares-crews-source-to-flume`
waits on this one: the Boomdrive module then declares crew's source
through flume's module, naming the roles file crew writes and crew's
kinds folder (crew record 0057, point 1).

## What Changes

- **An option for each source.** `services.flume.sources.<source>`
  declares the file `sources/<source>.toml`, each key named as the file
  names it: `kinds_dir`, required, and `roles`, each an absolute path or
  one under `~`. A key the module does not declare passes to the file,
  and flume judges it, as `settings` does.
- **A name flume takes.** A source's name that flume would refuse, or
  pass over, fails evaluation, naming the option: anything but ASCII
  letters, digits, `_` and `-` (weir C.4), and `flume`, `repo` and
  `launch`.
- **Rendered as `config.toml` is.** Each source's file is rendered at
  evaluation by the module's renderer, its first line naming
  `services.flume.sources.<source>`, and put in the store. A string
  shaped as a credential fails evaluation, as in `settings`.
- **Linked one by one.** The activation links each declared source's
  `~/.config/flume/sources/<source>.toml` to its file, renamed into
  place so no reader finds the source missing. A file there that is no
  link moves aside once. A link of the module's own whose source is no
  longer declared is removed. Every other entry of `sources/`, a
  person's own source or another program's among them, is left as it
  stands.
- **flume reads them whole.** flume's own reader of the host's sources
  reads each rendered file with no fault.
- **The record and the rule.** Record 0055 is amended, and core.45's
  details with it: the module writes each source it declares, and still
  writes no source it does not.
- **The operator's pages.** The install page shows a source declared
  through the module, and the configuration reference says the module
  writes the sources it declares.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `host-module`: `services.flume.sources`, each source's rendered file,
  its name's check, the credential refusal over it, the activation's
  links of the sources, and what the activation removes and never
  writes.

## Impact

- `nix/modules/options.nix`: `services.flume.sources`, the names'
  assertion, and the credential walk over the sources.
- `nix/modules/toml.nix`: the renderer takes the option it renders, for
  its first line and its failures. The settings' text is unchanged.
- `nix/modules/darwin.nix` and `nix/modules/nixos.nix`: each declared
  source rendered and the activation's sources step run after the link.
- `nix/modules/activate.sh`: `sources`.
- `nix/tests/`: the sources' cases, `sources.nix`, and the rendered
  files under `nix/tests/files/sources/`.
- `crates/flume/tests/host.rs`: `the_modules_sources_read_whole`, and
  `the_modules_files_read_whole` reading the folder's files alone.
- `docs/adr/0055-…`: an amendment. `docs/architecture/core.md`: core.45's
  details.
- `site/`: the install page and the configuration reference.
- No change to the statement: 191 has a source's declaration written
  whole by "a person, or a program that manages the host", and a host's
  Nix running flume's module is such a program; C.7's "writes none" is
  flume's own, which this changes no more than `config.toml`'s link did
  (record 0055). No crate's behavior changes.
- After this lands and is released, agentplot/boomdrive's unit
  `the-set-declares-crews-source-to-flume` declares crew's source
  through the module, and swancloud drops its own writer of
  `sources/crew.toml`. Those are their changes, not this one.

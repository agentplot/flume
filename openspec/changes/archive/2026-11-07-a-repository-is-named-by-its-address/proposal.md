# A repository is named by its address

## Why

The host's configuration names a repository by its folder:
`[repos."~/Code/acme"]` gives it a stream, its kinds' streams and labels,
and its agents' settings (C.7). On a host where another program keeps
the clones, the repository is written once in that program's file as
`<owner>/<name>` and again in flume's by the folder the clone happens to
stand in, so a clone that moves leaves flume's table naming nothing.
The user ruled that flume names a repository by its address and stays
usable on a host with no other tool, finding no clone by its address
(record 0055, accepted 2026-11-06). Clause 143 now says the
configuration names a repository by its address, `<owner>/<name>` as
its clone's `origin` remote gives it on its forge, matched exactly, and
a repository whose clone has no forge origin by its folder, flume asking
no other tool where a repository is (143, C.7).

## What Changes

- flume reads a repository's address from its clone's
  `remote.origin.url`: the path of a URL that names a host, its
  `.git` dropped, when it is exactly `<owner>/<name>`. An origin that
  is a path on the host, a `file://` URL, any other path, or no origin
  at all gives no address, and the repository is named by its folder
  (143, C.7).
- Wherever the host's configuration names a repository,
  `[repos."<repository>"]` and every table under it, flume takes the
  address's table where the configuration has one, whole, and the
  folder's table otherwise, so a host's file written by folder reads as
  it did. A key is matched exactly, with no star (143, C.7; record
  0055).
- A launch reads the address from the clone it runs in or is given, and
  keeps it with the place, so the place's later starts and settles take
  the host's tables for that address as the configuration stands then.
  A place launched before keeps its folder's tables. flume finds no
  clone by its address: `--repository` and a role's `repository` are
  folders, as before (8, 32, 143, C.7).
- Status gives a place's address beside its repository. No part of the
  origin's URL but the address is kept, given or written, since a URL
  may hold a credential (56; core.30).
- Rule core.55, drawn from record 0055, and the site's configuration
  reference and agent-settings guide name a repository by its address.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `host-configuration`: a repository's tables are named by its address,
  else its folder.
- `history-and-host`: the repository's origin read through git.
- `launching`: a launch keeps its repository's address, its place's
  starts take the tables for it, and status gives it.

## Impact

`crates/flume-seams/src/history.rs` and `host.rs` (the history's
`origin_url`, a repository's name handed to the host's tables),
`held.rs` (a place's address); `crates/flume-core/src/address.rs`
(new), `launch.rs`, `settings.rs`, `room.rs` and `status.rs`;
`crates/flume/src/host/history.rs`, `host/mod.rs` and `host/config.rs`;
`crates/flume-fakes/src/git.rs` and `host.rs`; the history, streams,
labels, agent-settings and launching scenarios in
`crates/flume-conformance`; tests in `crates/flume/tests/host.rs` and
`commands.rs`. Rule core.55 in `docs/architecture/core.md`. The site's
`reference/configuration.md` and `guides/agent-settings.md`.

Not in this change: flume's own Nix module and flume's own reading of
each account's usage, the other two points of record 0055, each a
change of its own; the latter waits on its design (184).

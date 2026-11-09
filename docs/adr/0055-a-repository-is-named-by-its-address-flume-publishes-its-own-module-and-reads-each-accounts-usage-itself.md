---
number: 55
title: A repository is named by its address, flume publishes its own module, and flume reads each account's usage itself
status: accepted
date: 2026-11-06
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# A repository is named by its address, flume publishes its own module, and flume reads each account's usage itself

Status: accepted by the user on 2026-11-06, in agentplot-design's pane: "yes 55", after asking that flume stay usable without Boomdrive, which the decision's first point says.

Amended 2026-11-09 by agentplot/boomdrive's record 0034, point 2, accepted by agentplot-design on 2026-11-09, and by the change `flumes-module-writes-the-hosts-sources`'s design: flume's module also writes each source `services.flume.sources` declares, `sources/<source>.toml`, as a link into the store renamed into place; it removes only its own links, and writes no source it does not declare.

Amends records 0017, 0020 and 0043 in part where a repository is named by its folder, and record 0035 in part where the host writes an account's reading, each marked so where it stands.

Deciders: the user, in the talk `research--the-flakes-configuration-holds-streams-and-repositories` of 2026-11-06 (agentplot/boomdrive `design/research/the-flakes-configuration-holds-streams-and-repositories.md` and `design/the-flakes-configuration/design.md`). On how flume finds a repository, asking: "the repository is once under BoomDrive, right? It is in Flume's config as well. Are you saying that currently Flume stores a folder path but we would replace it with the address? How would it end up resolving? Would Flume become responsible for taking that and resolving it through BoomDrive, or through a known BoomDrive location or something?", and choosing "One block by owner" for a repository's stream, each owner written once with its stream and its repositories' names. On flume's module: "Flume's flake, re-exported", and on what it writes: "Yes, all of Flume's file". On an account's usage, reading the host's sketch that gave each account a file another program writes: "It should be Flume's responsibility to read the usage, not expect it from some external third party." agentplot-design for the shape below: the address read from the clone's `origin`, the folder kept where a repository has none, and how flume reads usage left as flume's open design.

Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 143, 184 and C.7, as amended 2026-11-06; records 0017, 0020, 0032, 0035 and 0043; agentplot/boomdrive's design of the talk (`design/the-flakes-configuration/design.md`), its record 0020 (the Boomdrive flake) and its units `the-runner-reports-each-repository-it-keeps-and-its-clones-folder` and `the-flake-names-each-kept-repositorys-clone-folder` of the bolt `clones-are-found-from-boomdrive`; agentplot/boomdrive's record 0034, point 2 (a host's sources are flume's module's), for the amendment of 2026-11-09.

## Context and problem statement

The host's configuration names a repository by its folder: `[repos."~/Code/acme"]` gives it a stream, a kind's settings and its agents' settings (C.7; records 0017, 0020, 0043). A host where the Boomdrive runner keeps each clone writes the repository once in the runner's file as `<owner>/<name>` and again in flume's by the folder the clone happens to stand in, and a clone that moves leaves flume's table naming nothing. The Boomdrive bolt `clones-are-found-from-boomdrive` was to have the runner report each clone's folder and the flake write it into flume's table.

flume's file is written today by the host's fleet, outside any module of flume's, and the Boomdrive flake configures only the runner. The user wants one flake for a host, with each tool usable on its own.

An account's reading of its allowance is a file the host writes for flume (184; record 0035), and on deneb a program of the fleet's writes it. Clause 184 says flume asks no agent program or its maker how much is left, reads no sign-in and fetches nothing. The user ruled that reading flume's own.

## Considered options

1. **The address, read from the clone.** flume names a repository by `<owner>/<name>`, read from its clone's `remote.origin.url`, matched against `[repos."<owner>/<name>"]`; a repository with no forge origin is still named by its folder. flume asks Boomdrive nothing.
2. The folder kept as the key, and the runner reporting each clone's folder for the flake to write (the queued units of `clones-are-found-from-boomdrive`). flume's file then changes whenever a clone moves, and a host without the runner still writes folders by hand.
3. The Boomdrive flake owning flume's module. flume could not be configured by Nix without Boomdrive.

## Decision outcome

Option 1, with flume's own module and flume's own reading of usage.

- **A repository is named by its address.** Wherever the host's configuration names a repository, `[repos."<repository>"]` and the tables under it, `<repository>` is its address, `<owner>/<name>`, as the clone's `origin` remote gives it on its forge. flume reads it from the clone a launch names and matches it exactly, with no star. A repository whose clone has no `origin` on a forge is named by its folder, as today. A table naming a folder still matches the clone in that folder, so a host's file written before this reads as it did. flume asks no other tool where a repository is, and never finds a clone by its address: a command runs in a clone or is given one, and flume reads that clone's address to find its settings. So flume needs no setting for where clones are kept, and a host without Boomdrive, its clones anywhere, configures it the same way.
- **flume publishes its own module.** flume publishes a flake of its own with a `services.flume` module, for macOS and NixOS, that writes flume's whole managed file, `~/.config/flume/config.toml`, from options named as the file names them: its host, streams, pools with their accounts, kinds and repositories. As amended on 2026-11-09, it also writes each source `services.flume.sources` declares, `sources/<source>.toml` beside `config.toml`, from options named as that file names them, as a link into the store renamed into place, so no reader finds the source missing. It removes only its own links, a link it made whose source is no longer declared, and writes no source it does not declare. It writes no other tool's file. The Boomdrive flake takes flume's flake as an input and re-exports its module, so a host adds the Boomdrive flake alone.
- **flume reads each account's usage itself.** An account's allowance left is read by flume from the agent program's own source, and no host writes a reading for flume. How flume reads it, whether the program's own record or its maker's report under the account's sign-in, and what of 184's `usage` file stays, is flume's design still to make. It must weigh 146's rule that flume reads no sign-in and fetches nothing. Until it is made, an account's `usage` file reads as 184 says.

### Consequences

- The statement's 143 and C.7 name a repository by its address with the folder where it has none, and 184 and C.7 say the reading is flume's own, its how open, each marked proposed, at agentplot/blueprints.
- Boomdrive's units that give flume each clone's folder are not needed once this is built.
- A repository whose two clones on one host share an address is one repository to flume's configuration: both take its table.
- No unit is queued for the usage reading until its design is made.

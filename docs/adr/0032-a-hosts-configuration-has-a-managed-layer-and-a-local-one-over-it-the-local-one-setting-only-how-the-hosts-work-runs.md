---
number: 32
title: A host's configuration has a managed layer and a local one over it, the local one setting only how the host's work runs and the managed one pinning what it keeps
status: accepted
date: 2026-11-03
decision-makers:
- Chuck Swanberg
---

# A host's configuration has a managed layer and a local one over it, the local one setting only how the host's work runs and the managed one pinning what it keeps

Status: accepted by the user on 2026-11-07, on their card. The user decided it on 2026-11-03 on their Pending You card (req_99408baa): "Yes: operational keys only, manager may pin." It is stated on boomdrivedev/blueprints main at 4b4c5e0, which amends record 0020 and the capture design's storage rule.

Deciders: the user, who chose two layers, the local one setting only the keys that run the host's work, and the managed one pinning keys. The writing agent's: what a local file that cannot be read does, how a pin names its keys, what a pin that names nothing is, how status names a key the local layer may not set, a stream the local layer alone names, and how status gives each value and its layer.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 4b4c5e0, clause 32, section 9, C.7 and S83; `boomdrivedev/blueprints:design/capture/design.md` at 4b4c5e0, "Host state goes by kind"; records 0010, 0017 and 0020; rules core.16, core.30; `crates/flume/src/host/config.rs` (`HostConfig::read`, `parse`, `config_file`), `crates/flume/src/host/mod.rs` (`RealHost::of_host`).

## Context and problem statement

The host's configuration is one file, `$XDG_CONFIG_HOME/flume/config.toml`. It holds two sorts of thing. Some say what the host is: its accounts and their environments, its pools, its terminal, its roles and its bound. Others say how its work runs this hour: how many agents a stream runs, which stream ranks first, which stream a repository's places go to, and a kind's model.

A host is often managed. A fleet tool renders the file from what its manager declares, and writes it again whenever the manager changes something. The host's operator, or a control plane acting for them, wants to move a stream's bound or a kind's stream between those writes. Today it must edit the managed file and lose its edit at the next render, or render the file itself and take the accounts with it.

## Considered options

1. One file, as today.
2. Two layers: the managed `config.toml`, and a local `config.local.toml` beside it read over it key by key. The local layer may set only the keys that run the host's work, and the managed layer may pin any of those keys.
3. As 2, the local layer setting any key.
4. flume's commands change the configuration, as `flume stream set work agents=3`.
5. Variables of the calling command's environment over the file.

## Decision outcome

Option 2.

1. **Two files.** `config.toml` is the managed layer, written by whoever manages the host and never by flume. `config.local.toml` beside it is the local layer, written by the host's operator or a control plane acting for them, and never by flume. Each reading of the host's configuration, at each start as clause 32 says, reads both. A key the local layer sets replaces the managed layer's. A host with no local file reads the managed layer alone, as today.
2. **What the local layer may set.** C.7's list: a stream's `agents`, `priority`, `points`, `pace` and `session`; a label's `reserve` and `keep_free` (record 0033); the `stream` a repository, a repository's kind or a kind is given, and the `label` a repository's kind or a kind is given; and a kind's agents table, for every repository or for one. Each key is taken whole: a `pace` table, and a `mode` given as a table of programs, are one key each (core.30). A stream the local layer alone names is declared by it, with no pool of its own, so it takes the host's pool.
3. **What it may not.** The accounts, the pools, a stream's `pool`, `[host]`, `[roles]` and every other table are the managed layer's alone. A local file that sets one is not taken at all: every reading is of the managed layer alone, and status names each such key, until the file is mended. Status names the key at its table's entry, `pools.work` for anything in `[pools.work]` and `streams.work.pool` for a stream's pool. A local file that cannot be read as TOML, or a value it sets that its key does not take, is not taken either, and status gives the fault. A control plane's bad write never stops the host's work, since the managed layer runs alone meanwhile.
4. **Pins.** The managed layer pins keys by `pin` under `[host]`, a list of dotted keys in TOML's spelling, `"streams.work.agents"` or `'repos."~/Code/acme".stream'`. A pin may name a table, `"streams.work"`, pinning each key the local layer may set under it. A key the local layer sets that a pin holds is passed over, and status names it with its pin; the rest of the local layer is taken. A pin that names nothing the local layer may set is a fault of the managed layer, refused naming the pin as any other fault of it is.
5. **Status.** Status gives every value of the host's configuration, each with the layer it came from, `managed` or `local`: every key an agent's settings, a stream, a pool or the host reads. It gives whether the local layer was taken, and if not, each key or the fault why not, and each key passed over with its pin. An account's environment is given by each variable's name and `<set>`, never its value (core.30).
6. **flume writes neither.** No command of flume's writes either file (core.16).

### Why not the others

Option 1 leaves a control plane rendering the whole file, accounts and environments with it, or losing its edits at the manager's next render. Option 3 lets a local file change accounts and pools, the environments that sign agents in, which belong to whoever manages the host. Option 4 has flume write the host's configuration and keep a second record of what its manager declared. Option 5 makes a setting depend on which shell ran a command, which clause 29 rules out for an environment.

The writing agent's choices: a local file whose fault would refuse every command is passed over whole, since the local layer is the one a program writes hour to hour and the managed layer still describes a working host. The other path, refusing every command until the local file is mended, would let a control plane's bad write stop every place. A pin naming a table spares a manager from listing every key of a stream. A stream only the local layer names stays usable, since its pool is the managed layer's to give.

### Consequences

- Clause 32, section 9, C.7 and the capture design's storage rule are amended, and S83 added, on blueprints main at 4b4c5e0 (core.19).
- Rule core.45 is drawn from this record.
- The host's facts gain the configuration's values with their layers, which status gives.
- Until record 0025's change builds a stream's `points` and `pace`, the reader refuses them in either layer as it does today, so a local file setting them is not taken, its fault named.
- The change `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer` builds it.

# A label's share is owed and reserved

## Why

The user, reading deneb's Boomdrive configuration on 2026-11-09: "when
settings are a number liek this one: threshold = 90; a comment should
explain the units/options", and "\"keep_free\" - blurgh. don't love this
config key. there are better ways to name config keys". Record 0063,
accepted that day, renames a label's two numbers and has every number
flume reads say its unit, range and default (principles 13).

A stream's `[streams.<stream>.labels.<label>]` still reads `reserve`
and `keep_free`. `reserve` beside sluice's `[runs] reserved`, which
means the opposite of lending, tells a reader wrong, and `keep_free`
says nothing of what it holds. The statement now says `owed` and
`reserved` (70, 181, 182, C.7), and core.26 and core.46 say them, so
flume reads words its statement and rules no longer hold. The module's
options and the configuration reference give some numbers' units and
not others: `threshold` does not say it is a percent of a usage window,
a stream's `priority` does not say which way it orders, and the host's
`agents` reads 0 though a stream's and an account's refuse it.

## What Changes

- **A label gives `owed` and `reserved` (181, 182, C.7).** The reader
  takes `owed`, how many of the stream's agents the label's places are
  first in line for, and `reserved`, how many of those are held for it
  alone, each a whole number of agents from 0, 0 where none is given.
  `reserve` and `keep_free` are no words of a label's table: a managed
  file naming one refuses the stream's starts naming the key, as any
  key the table does not hold, with no alias. The local layer may set
  `owed` and `reserved` in place of the old two; a local file setting
  `reserve` or `keep_free` is not taken (C.7; core.45).
- **The refusals, the queue and status say it (4, 56, 70, 181).** A
  label holding more reserved than it is owed, labels owed past their
  stream's bound, a stage waiting on agents another label holds
  reserved, and a stream whose labels' shares are passed over are each
  worded in `owed` and `reserved`. The JSON form gains each label's
  `owed` and `reserved` beside `reserve` and `keep_free`, which it keeps
  with the same values, since the form only ever gains fields (61;
  core.14).
- **The host's `agents` is a whole number from 1 (70, C.7).** `[host]
  agents = 0` refuses the configuration naming the key, as a stream's
  and an account's `agents = 0` do.
- **Every number says its unit, range and default.** The module's
  option for each number flume reads, and the configuration reference,
  say what it counts, its range and its default: the host's, a
  stream's and an account's `agents`, a stream's `priority`, a label's
  `owed` and `reserved`, a pool's `threshold`, `keep_open` and `[host]
  ends_expire`. The module declares `[host] ends_expire` and types each
  `agents` from 1. A stream's `points` and `pace`, which flume does not
  read until record 0025's change, stay undeclared.
- **The statement** says the same at boomdrivedev/blueprints, on the
  branch `design/flume-a-labels-share-is-owed-and-reserved` at
  695c446, cut from main at 81288ba: C.7's numbers each with their
  range and default, and S84 in 181's words. The orchestrator pushes
  it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `stream-labels`: a label's table holds `owed` and `reserved`; what a
  label is owed and holds reserved; what a stream's labels refuse,
  `reserve` and `keep_free` among the keys refused; status gives each
  label's `owed` and `reserved`.
- `host-queue`: a stage waits on agents another label holds reserved,
  and a label below what it is owed starts first.
- `host-configuration`: the local layer sets a label's `owed` and
  `reserved`.
- `seam-contracts`: the host's facts give a label's share as `owed` and
  `reserved`.
- `streams`: the host's and a stream's bound are each a whole number
  from 1.
- `host-module`: a label's options are `owed` and `reserved`, each
  number's option says its unit, range and default, and `[host]
  ends_expire` is declared.

## Impact

- `crates/flume-seams`: `Share`'s `owed` and `reserved`.
- `crates/flume`: the host's reader (`host/config.rs`: the labels'
  table, `LOCAL_KEYS`, `[host] agents`), the person's form of status
  (`commands/print.rs`), and `tests/host.rs`'s reading of the module's
  file.
- `crates/flume-core`: `room.rs`'s shares and refusals, `answer.rs`'s
  `LabelStatus` and the queue's words.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake host's
  shares, the labels', host queue's, streams' and answers' scenarios.
- `nix/modules/options.nix`, `nix/tests/every-key.nix`,
  `nix/tests/files/every-key.toml`, `nix/tests/options.nix`.
- `site/src/content/docs/`: the configuration reference, the agent
  settings guide, the state reference and the roadmap.
- `docs/adr/0063`: the construction's choices. Rules core.26 and core.46
  already say `owed` and `reserved`.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool.
- A host file naming `reserve` or `keep_free` refuses its stream's
  starts, or is not taken where the local layer names it, until it
  names `owed` and `reserved`; one stating `[host] agents = 0` refuses.
  Boomdrive's drive option renames with it, which is Boomdrive's to
  follow (record 0063).

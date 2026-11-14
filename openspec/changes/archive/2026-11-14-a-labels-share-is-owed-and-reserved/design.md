# Design

The statement is read at boomdrivedev/blueprints
`design/flume-a-labels-share-is-owed-and-reserved` at 695c446, which is
main at 81288ba with C.7's numbers each given their range and default
and S84 put in 181's words (section 8): clauses 4, 56, 61, 70, 143, 165
to 167, 180 to 182, 185, 198, C.7, S84 and S85. Record 0063 (accepted
2026-11-09) decides the names and the words; principles 13 is the
family's rule behind it. Where this design and the statement disagree,
the statement wins (core.19).

The rules relied on: core.14 (one JSON form that only ever gains
fields); core.26 (a label's below what it is `owed` first, room past
what other labels hold `reserved`); core.45 (a local file setting a key
C.7 does not list is not taken); core.46 (a label's table holds `owed`
and `reserved` alone, `reserve` and `keep_free` no words of it); core.1
and core.2 (the reader and the queue name no tool; the change goes
through the host seam's `Share` and no profile); core.21 and core.22 (no
new project, file type or tool); core.35 (the site speaks to any host's
operator). core.26 and core.46 already say `owed` and `reserved`, amended
with record 0063. No area page exists; core.md binds every path this
change touches.

## 1. The share's names (181, 182, C.7)

The host seam's `Share` (`crates/flume-seams/src/host.rs`) holds
`owed` and `reserved`, each `u32`, 0 where its table gives none, and
`unknown`, every other key its table holds. Its JSON spells them `owed`
and `reserved`. The queue's `LabelRoom` (`crates/flume-core/src/room.rs`)
holds the same two and the agents running; its methods are named for
them: a label runs below what it is owed, and the agents a label holds
reserved and does not use are `reserved` less its running, from 0.

The seam's JSON is no stored document: `Share` crosses the seam in
memory, and its round trip is a test of the seam's types. So the fields
are renamed with no alias, and the round trip reads `{"owed":2}` as
owed 2 and reserved 0.

## 2. The reader and the local layer (181, 182, C.7; core.45, core.46)

`shares_of` (`crates/flume/src/host/config.rs`) reads `owed` and
`reserved` from each `[streams.<stream>.labels.<label>]`, each a whole
number of agents from 0, refusing the configuration where either is
not, as it refuses a share that is no count today, in the words "is not
a whole number of agents from 0". Every other key is kept unknown, and
`reserve` and `keep_free` are other keys: a managed file naming one
reads, and each start of its stream is refused naming the key (section
3). `LOCAL_KEYS` gives `streams.*.labels.*.owed` and
`streams.*.labels.*.reserved` in place of the old two, so a local file
setting `reserve` or `keep_free` is not taken at all, status naming the
key (core.45).

Not taken: reading `reserve` as `owed` and `keep_free` as `reserved`
for a while. Record 0063 decides "with no alias kept", and a host's file
is its module's, mended by the module's next release.

## 3. The refusals (4, 181, 182)

`fault_of` keeps its order (another key, then more reserved than owed,
then owed past the bound) and words each in the statement's:

- another key: `streams.work.labels.operation.reserve: a label's table
  holds owed and reserved alone; its stream's pool, points and pace are
  its budget`;
- more reserved than owed: `the label operation of the stream work
  holds 2 reserved of the 1 it is owed, and a label holds reserved no
  more than it is owed`;
- owed past the bound: `the stream work's labels are owed 6 in all,
  operation 3 and construction 3, past its bound of 5`, naming the
  stream and what its labels are owed (181).

## 4. The queue's words (70, 181)

A stage short of room its stream's other labels hold reserved wants, in
words, `the stream work's room, the rest held reserved for operation`.
Its JSON keeps `"room": "kept-free"` with its `stream` and `labels`. The
tag is a value a caller matches on, and the form only ever gains
(61; core.14), so a reader that matches `kept-free` reads on.

Not taken: a tag `reserved`, which drops the value a reader knows; and
both tags at once, which a tagged value cannot carry.

## 5. Status (56, 61, 181; core.14)

Each stream's label in the JSON form gains `owed` and `reserved` and
keeps `reserve` and `keep_free`, each pair the same number: `reserve`
is `owed` and `keep_free` is `reserved`. The answers' field list gains
`status.host.streams[].labels[].owed` and `.reserved`. The stream's
`reserves`, `held`, `passed-over` or `none`, keeps its name and values,
which say whether what its labels are owed is held.

The person's form says a label as `label operation: owed 2, reserved 1,
1 agents running, 1 waiting`, and a stream that states no bound `what
its labels are owed is passed over: it states no bound`. The state
reference gives `owed` and `reserved`, and `reserve` and `keep_free` as
the same numbers under the names a label's table gave before record
0063.

Not taken: renaming `reserve` and `keep_free` in the JSON form, which
removes two fields a reader may read (61; core.14).

## 6. The host's bound from 1 (70, C.7)

`[host] agents` is read as a stream's and an account's `agents` are, by
`bound_of`: a whole number from 1, none where it is left out; `[host]
agents = 0` refuses the configuration naming the key. `bound_of`'s words
become "is not a whole number from 1", the range record 0063 gives each
`agents`. Today the host's bound takes 0, a host where no stage ever
starts, while a stream's and an account's refuse it; nothing in the tree
sets it.

Not taken: the reference saying "from 0" for the host alone, which keeps
one bound unlike the others and unlike the record's words.

## 7. The module and its words (C.7; record 0055)

`nix/modules/options.nix` gives each number's option a description that
says what it counts, its range and its default, in the record's words:

| Option | Type | Says |
|---|---|---|
| `host.agents` | `types.ints.positive` | agents of flume's places running at once on the host, every caller's and stream's together, a whole number from 1; no bound where unset |
| `host.ends_expire` | a string `<n>`, `<n>s`, `<n>m` or `<n>h`, `<n>` from 1 | how long an end stays pending for a recipient before status shows it expired, `<n>` or `<n>s` seconds, `<n>m` minutes or `<n>h` hours, at least a second; flume's `168h` where unset |
| a stream's `agents` | `types.ints.positive` | the stream's agents running at once, within the host's bound, a whole number from 1; the host's bound alone where unset |
| a stream's `priority` | `types.int` | its priority among streams, a whole number, negative allowed, higher starting first; flume's 0 where unset |
| a label's `owed` | `types.ints.unsigned` | the stream's agents the label's places are first in line for, lent while it has nothing waiting, a whole number of agents from 0; flume's 0 where unset |
| a label's `reserved` | `types.ints.unsigned` | of those, the agents held for it alone, never lent, a whole number of agents from 0 and no more than `owed`; flume's 0 where unset |
| a pool's `threshold` | `types.ints.between 1 100` | a whole percent, from 1 to 100, of any of an account's usage windows, past which the account takes no new start until that window resets; flume's 90 where unset |
| an account's `agents` | `types.ints.positive` | agents running under the account at once, a whole number from 1; no bound of its own where unset |
| `agents` (every agents table) | free-form | read whole by flume at each start, its `keep_open` a time, `<n>` or `<n>s` seconds, `<n>m` minutes or `<n>h` hours, `0` keeping none, one hour where no layer gives one |

The label's options are `owed` and `reserved`, and the module declares
neither `reserve` nor `keep_free`, so either passes to the file as an
undeclared key does, and flume refuses it (section 2). `host.ends_expire`
is declared, since flume reads it and its description is where a host's
author reads its unit. `nix/tests/every-key.nix` sets the label's two
and `host.ends_expire`, rendered byte for byte in
`nix/tests/files/every-key.toml`, and `the_modules_files_read_whole`
reads them back.

A stream's `points` and `pace` stay undeclared: flume refuses them in a
stream until record 0025's change reads them (the change
flume-publishes-its-own-flake-and-module, design 6), and that change
declares them with their words, as C.7 now gives them. An account's
`usage` is left as it stands: the bolt's change
`flume-reads-each-accounts-usage-from-its-agent-programs-windows` takes
it out with record 0062, so the two changes touch neighbouring lines of
the module and the reference and no line both.

Not taken: a unit in each name, `threshold_percent` or `agents_max`,
which record 0063 weighed and did not take.

## 8. The statement (core.19)

C.7 gave the labels' and the threshold's ranges, and not the host's or a
stream's `agents`, a stream's `priority`'s direction, `points` and
`pace`'s ranges or `keep_open`'s default, which record 0063 has C.7 say;
and S84 still said "keeping one free". The construct stage amended both
on blueprints' branch `design/flume-a-labels-share-is-owed-and-reserved`,
cut from main at 81288ba, commit 695c446, for the orchestrator to push.
The module's and the reference's words follow it; no line of this change
rests on words main lacks but those.

## 9. The site (core.35)

The configuration reference gives each number's values, meaning and
default as section 7's table says, the label's table as `owed` and
`reserved`, the local layer's keys, and the labels' refusals in section
3's words. The agent settings guide's labels section, its local layer
list and its status example say `owed` and `reserved`; the state
reference gives section 5's fields; the roadmap's labels row says a
label is owed agents and may hold some reserved.

## 10. The record

Record 0063 gains the construction's choices, each with the path not
taken: the JSON form's kept fields and tag (sections 4 and 5), the host's
bound from 1 (section 6), `host.ends_expire` declared and `points` and
`pace` left to record 0025's change (section 7). Its status stays
accepted: the choices are the construction agent's, cheap to change.

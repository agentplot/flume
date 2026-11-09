# Design

## Context

### What the user decided

Record 0056, accepted by the user on their card on 2026-11-07, has a
base place's kind name the kind its fixes run as and how many it runs.
flume launches that fix at a settle when the merge queue forecasts the
base place's branch in conflict with its base, or its submission fails
(203). Its amendment, settled by agentplot-design the same day, gave
weir's format two fields for it, `fix` and `max_fixes`, the fix kind
named by a name its reader resolves.

The user then read that example as the same kind doing its own fix,
and in agentplot-design's pane asked for the whole identity and a name
that says the merge queue:

> Wouldn't it include the fully qualified change name, including the
> version?

> Should 'fix' be the right term, or should it be something more
> related to the merge queue … since this is specifically for being
> kicked back from that?

weir's record 0005 decides it, accepted on their card the same day:
"Yes, write it into both formats". A simple kind that lands with no
stages may give `landing_fix`, the identity `<source>/<name>@<version>`
of the kind a place runs when the merge queue sends its branch back,
and `max_landing_fixes`, how many it runs before its place waits on its
caller. A bare name, or an identity with no version, is refused naming
the field, so one version of a kind always means one fix. The statement
carries it in flume 23 and weir 52, 57, 89, 94 and C.4, at
agentplot/blueprints 8eec6dd.

### What happens today

- **The core is built.** `Kind::fix` and `Kind::fix_limit` exist in
  `flume-seams`. A base place's launch resolves `Kind::fix` in
  `fix_kind_of` (`crates/flume-core/src/launch.rs`) by
  `resolve_kind`, as a launch's kind name resolves (154). It holds the
  identity as `PlaceRecord::fixes_as` (21), and the settle forecasts and
  launches fixes within `fix_limit`, else `Defaults::send_backs`
  (`crates/flume-core/src/fixes.rs`). The engine's scenarios prove it
  over kinds built in Rust.
- **The kinds profile names no fix.** `map_kind`
  (`crates/flume/src/profiles/kinds/weir/mod.rs`) sets `fix: None` and
  `fix_limit: None` for every kind, since weir-kinds at the pinned
  `5562af5` reads neither field.
- **weir-kinds reads both at weir's main.** At `daacda3`, which landed
  `bolt/weir-kinds-name-their-fix`, `weir_kinds::Kind` gains
  `landing_fix: Option<Identity>` and `max_landing_fixes: Option<u64>`.
  It refuses either on a kind whose stages read as one or more, and a
  `landing_fix` that is no identity with its version. `fix` and
  `max_fixes` are sections now. Between `5562af5` and `daacda3` nothing
  else of weir-kinds or weir-capture changed, and no manifest did.
- **The docs say no kind names a fix.** The merge-queue guide says a
  kind's declaration has no field for it yet, and the kinds guide and
  the configuration reference list neither field.

### Rules relied on

- core.4: flume defines no field of the format and reads the part 23
  names through weir's crate. The two fields are read from
  `weir_kinds::Kind`, never from its sections.
- core.22: weir-kinds is taken at one revision, never a range.
- core.34: a kind refers outside its package only to its landing fix,
  by its identity with its version, resolved at its place's launch and
  no part of its package or its hash. This change's construct amended
  it to weir's record 0005.
- core.41: a simple kind is read and run as clauses 18 to 25 say. The
  statechart kind's landing fix (175, weir 94) waits for flume to run
  statechart kinds.
- core.53: a base place is told by its shape alone. Nothing here reads
  a kind's name, a prefix or a depth.
- core.8: a landing fix that resolves to nothing refuses the launch
  before anything is made.
- core.14: the JSON form only gains fields. Status's `fix_kind` and
  `fix_limit` keep their names.
- core.19: the statement holds. 23 says what weir's record 0005 says,
  and 157's "a name its reader resolves as a launch's name resolves
  (154)" covers a whole identity, the fullest form of a launch's name.
- core.35: the guides speak to any host's operator. They name flume's
  own `fix@1` as flume's, and no caller of ours.

The only page of `docs/architecture/` this change touches is `core.md`;
there are no area pages.

## Goals / Non-Goals

**Goals:**

- Read `landing_fix` and `max_landing_fixes` of a simple kind through
  weir's crate onto the seam's `Kind::fix` and `Kind::fix_limit`, from a
  repository's declaration, a launch's and a package alike.
- Prove a base place launched under such a kind holds its landing fix
  as the core resolves it, and is refused where the identity resolves
  to nothing.
- Give both fields to the reader in the guides and the reference.

**Non-Goals:**

- Any change to the core's resolution, forecast, fix launch or bound,
  which record 0056's change built and its scenarios hold.
- A statechart kind's landing fix (175, 177), designed when flume runs
  statechart kinds.
- crew's bolt kind naming its landing fix, crew's follow-up C2.
- Renaming the seam's fields or status's.

## Decisions

### 1. weir-kinds moves to weir's main at daacda3

`Cargo.toml` pins weir-kinds at
`daacda39e59469029b2841f889c6cbd8cd7839d9`, weir's main once
`bolt/weir-kinds-name-their-fix` landed (#6), and `Cargo.lock` moves
weir-kinds and weir-capture with it. It's the first revision of weir's
main that reads both fields. Taking the bolt's own head instead would
pin a branch weir has since landed and may tidy. The comment above the
pin stays as written. No engine of weir's is pinned yet, so core.42's
pin moves with nothing.

### 2. map_kind maps the two fields and nothing else

```rust
fix: kind.landing_fix.as_ref().map(ToString::to_string),
fix_limit: kind.max_landing_fixes.map(|times| {
    u32::try_from(times).unwrap_or_else(|_| {
        problems.push(format!("{at}.max_landing_fixes: is more than flume counts"));
        0
    })
}),
```

The identity is spelled by weir's `Display`, `<source>/<name>@<version>`,
as every identity flume answers is (core.4). The core reads it back by
the kinds seam's `read_named` when the base place launches, so flume's
own code parses nothing. A count past `u32` is a problem naming the
field, as `version` and `max_send_backs` are, and so refuses the
declaration whole (84). `fix` and `max_fixes` reach the kind's sections,
which flume ignores (23). The same `map_kind` serves a repository's
declaration, a launch's and a package's, so a package's landing fix
needs no code of its own.

### 3. The seam keeps flume's words

`Kind::fix` and `Kind::fix_limit` keep their names: the seam's fields
are the statement's vocabulary, never the format's (core.4), and
status's `fix_kind` and `fix_limit` may only gain fields (61; core.14).
Their docs say they hold the kind's landing fix and its bound as the
kind's reader gives them, which weir's format gives as an identity with
its version (23; weir record 0005). The core resolves whatever the field
holds as a launch's name. A kind built in Rust may still name a bare
name, as the engine's scenarios' `train` names `mend`. No reader of a
declaration can give one, and the core checks no form of its own, since
spelling and parsing an identity is weir's (core.4).

### 4. The bound left out, and a bound of 0

A `max_landing_fixes` left out leaves `fix_limit` none, and the core
takes flume's default for a stage's bound, `Defaults::send_backs`, 3 in
the binary, as 58 leaves a field to its reader. A bound of 0 launches no
fix: the first conflict or failure leaves the base place waiting on its
caller with the merge queue's words, as `fixes::decide` already does. A
bound with no `landing_fix` holds nothing to launch, so the base place is
never forecast, as a kind naming no fix.

### 5. The scenarios over the profile

`crates/flume/tests/profiles/kinds_weir.rs` holds the kinds-profile
spec's scenarios over scratch repositories and a scratch source. The
launch scenarios build the engine over the fakes with the profile in
place of the fake kinds (`common::engine::engine`). The scratch
declaration binds the fakes' merge queue under the profile name the
world binds it by. The engine's own scenarios in
`flume-conformance` are untouched: they prove the core over kinds built
in Rust, and these prove the profile feeds the core the same.

### 6. The docs

- **The kinds guide**: "A kind with no stages" gives both fields in its
  example, naming flume's own `flume/fix@1`, and says the identity
  needs its source and version. It says the bound left out is three,
  and that either field on a kind with stages is refused. "A kind's
  fields" lists both.
- **The configuration reference**: "A kind" lists both, and its
  `stages` row says `[]` on a kind that lands.
- **The merge-queue guide**: "Its fixes" gives the fields in place of
  saying a declaration has none. The paragraph on a kind that names no
  fix stays.

### 7. The record and the rules

This change's construct amended them at 15d15448, citing weir's record
0005, the user's, accepted. Record 0056's amendment line names
`landing_fix` and `max_landing_fixes` and this change. core.34 has a kind
refer outside its package only to its landing fix, by its identity with
its version, and its details rule out a landing fix taken by its name
alone or at another version. core.4's details rule out either field read
from a kind's sections. The site's rules page mirrors core.34. Each
applies a decision the user accepted to wording. No new record is
needed: the decision is weir's record 0005 and flume's 0056.

## Who owns what

| piece | owner | flume's part |
|---|---|---|
| the fields' spelling and the identity's form | weir, record 0005 | read through weir-kinds (core.4) |
| which landing fix a kind names | the kind's declaration or package | map it, resolve it at the launch |
| resolving the identity, the forecast, the fix launch, the bound | flume's core (203) | built by record 0056's change |
| crew's bolt kind naming its landing fix | afterthought/crew, C2 | none |

## Alternatives Considered

- **Rename the seam's and status's fields to `landing_fix`.** The seam
  speaks the statement's words, and status's JSON may only gain
  fields (core.14). Adding `landing_fix` beside `fix_kind` would say one
  thing twice.
- **Have the core refuse a `Kind::fix` that is no whole identity.**
  weir's crate already refuses it at the read, and the core would parse
  a kind's name itself, which is weir's (core.4). The fakes' kinds stay
  free to name a bare name, which the core resolves as any launch's
  name.
- **Map `fix` and `max_fixes` as well, for a kind written before weir's
  record 0005.** No release of weir's main read them as fields, so no
  declaration flume read ever had them. Reading them would make flume
  define a field of the format (core.4).
- **Move the clause list's `REVISION` to 8eec6dd.** 8eec6dd adds and
  removes no clause of flume's, and the list's module says it changes
  with a clause added or removed. The citations here name 8eec6dd where
  they rest on its words.
- **Pin weir-kinds at the bolt's head rather than weir's main.** It
  would pin a branch that landed and may be tidied, where weir's main
  holds the same content for good.

## Risks / Trade-offs

- **weir's main may move again before this lands.** The pin is an exact
  revision (core.22), so a later weir change reaches flume only by a
  chore of its own.
- **A kind built in Rust may still name a bare name.** Only the engine's
  scenarios do, and the core resolves it as a launch's name. A reader of
  a declaration never gives one.
- **A repository that wrote `fix` reads no fix, silently.** weir returns
  it as a section and flume ignores sections (23). The guides give only
  weir's spelling, and no declaration of flume's own or of its scenarios
  ever used `fix`.

## Migration Plan

Nothing held changes shape. A base place launched before this release
holds its kind as its launch read it (21), with no `fix`, so it is never
forecast and waits on its caller as before; its caller's next base place
takes the field once its kind names it.

## Open Questions

None.

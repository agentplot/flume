# Design

## Context

### The user's words and record 0056

On 2026-11-07, in agentplot-operator's pane, relayed by it, the user
asked why the fix of two bolts main had moved past was not launched:

> Why couldn't the fix have been launched? That is my fucking point and
> you know that

> After we rework Boomdrive and Flume, the fixes should be automatic
> because Flume will settle. At least according to the diagrams, it
> should all be in the Rust code. It should detect and launch a new fix

> Just make sure it's in the fucking design because it should be.

Record 0056 decides it, accepted by the user on their card the same
day: a base place's kind names the kind its fixes run as; a settle
forecasts a waiting base place's branch against its base through the
merge queue; a forecast conflict, or a failed or conflicting
submission, launches that fix on the branch with the merge queue's
words; one fix at a time, each counted toward the kind's bound; and the
submission stays the caller's retry. The statement carries it in 1,
203 and I6, accepted at agentplot/blueprints 24d36f8, and its main at
31f9f1c holds it with this design's amendment (decision 13).

This change lands on the base place
`bolt/flume-launches-a-conflicting-bolts-fix`, a bolt of the kind this
design teaches flume to fix.

### What happens today

- **A base place's failure starts nothing.** `submit.rs`'s outcome
  arm, for a base place, writes `held.submission.waiting` with the
  merge queue's words and returns. The base place waits on its caller
  (`WaitsOn::Caller`), stalls after five minutes, and the failure is
  delivered to its recipients as any end (43, 203).
- **Nothing forecasts a base place.** The merge queue seam
  (`crates/flume-seams/src/merge_queue.rs`) has submit, inbox,
  read-submissions, step, base, open and close, and no forecast. A
  base place's settle reads its submission's record and does nothing
  more while it waits on its caller.
- **flume already launches a place inside a settle.** A role's run is
  launched by `wake.rs` through `Engine::make_place` with a `Launch`
  built from the role's entry, and a launch refused because another
  settle's run now stands is taken as that run (132; core.28). The fix
  launch takes the same path.
- **The kind format has no word for it.** weir-kinds, pinned at
  5562af52, reads `on_fail` and `max_send_backs` on a stage alone, and
  a kind declaring `stages = []` has no stage to carry them. weir 89
  says a package names no other kind. flume defines no field of the
  format (core.4), so weir's format must gain the kind a kind with no
  stages names for its fixes, and its bound, before flume can read
  them: weir's follow-up W2, and flume's mapping of them after it
  ("Named follow-ups").
- **sluice already forecasts a named head.** The pinned sluice, 0.71.0,
  takes `sluice submit <base> --check --head <branch> --json` from
  anywhere in the repository. It answers `{conflict, head, onto,
  ahead}`, `conflict` null and exit 0 where the head would merge, and
  `conflict.paths` with `conflict.with[]` and exit 2 where it would not.
  It forms the merge on the host reader's document where that reader
  keeps its tick, and fetches nothing then (sluice 64, 129). A
  conflict with the branch itself has an empty `with`.

### What the statement leaves open

Record 0056 and 203 say what happens. They leave these to the design,
each decided below:

- how the kind names its fix in weir's format, against weir 89 and 157,
  which say a package names no other kind (decision 1);
- when the fix kind resolves, at the base place's launch or at each
  fix's (decision 2);
- the merge queue seam's forecast, which Part B does not hold (decision
  3);
- what a forecast that conflicts with entries queued ahead, not with the
  base, does (decision 5);
- how a settle acts on one forecast or one failure once (decision 6);
- the fix's name, stream, priority, intent, recipients, settings and
  points; 165 refuses a launch with no points on a stream that bounds
  them (decision 7);
- a fix whose launch is refused (decision 8);
- what status gives (decision 10).

### Rules relied on

This change touches the area core.md binds; docs/architecture has no
area page. It relies on core.1 and core.2 (the core names no tool, and
reaches the merge queue through its one interface), core.3 (sluice's
profile runs sluice's commands once bound), core.4 (the kind format is
weir's), core.5 (flume merges nothing: the fix's agent takes the base
in), core.6 (a base place submitted only on its caller's retry), core.8
(a launch refused whole), core.13 (the forecast and the launch run
inside a settle, waiting on nothing), core.14 (the JSON form only gains
fields), core.15 (a forecast is no fact of a deliverable, a change or a
submission, so the settle's reading of it is held), core.16 and core.17
(held state by compare-and-set, every act safe to repeat), core.24 (a
stall), core.27 (running through), core.28 (a place flume launches in a
settle), core.34 (a kind's name resolved as clause 154 orders), core.38
(points) and core.53 (base places). It amends core.53's rule and
details, core.34's rule and details, and core.6's and core.36's
details (decision 14).

## Goals / Non-Goals

**Goals:**

- A base place whose kind names a fix is taken back to its base within
  the host's watch of the forecast or the failure that found it, with no
  caller, agent or person deciding it (203, 208).
- One fix at a time, each forecast and each failure acted on once, and
  never past the kind's bound (23, 203).
- Nothing names a bolt, a unit, a depth or a kind: the base place is told
  by its kind's shape, and its fix by the kind its kind names (202;
  core.53).
- The submission to the base stays the caller's, so a send to main still
  waits on the user (203; record 0023).
- A base place whose kind names no fix behaves as today.

**Non-Goals:**

- weir's fields themselves, the kinds profile's mapping of them, and
  crew's bolt kind naming its fix: the follow-ups below.
- A forecast of places with stages: sluice 64 forecasts a session's head
  for its agent already, and 112 and 131 send such a place back on its
  own submission.
- Reading a base's own checks after a landing: core.36 keeps that a
  caller's launch.
- Points: this flume takes a place's points from nothing (165 is
  unbuilt). Decision 7 says where a fix's would stand.

## Decisions

### 1. The kind names its fix and its bound

A kind that lands with `stages = []` may name the kind its fixes run as,
by a kind's name or a whole identity, and how many fixes it runs before
its place waits on its caller. weir's format gains both on such a kind
(W2), refusing either on a kind with stages, which names its fix round
by `on_fail` and its bound by `max_send_backs`. The spelling is weir's
to settle; this design calls them `fix` and `max_fixes`:

```toml
[kinds.train]
branch = "train"
lands = true
stages = []
fix = "fix"        # the kind its fixes run as, resolved as a launch names a kind
max_fixes = 2      # how many fixes before the base place waits on its caller
```

flume's `Kind` gains `fix: Option<String>` and `fix_limit:
Option<u32>`, which the kinds profile maps from weir's two fields and
from nothing else (core.4), in flume's follow-up unit
a-base-places-fix-kind-is-read-from-weirs-format once W2 lands. Until
then a kind in weir's format names no fix, and its base place waits on
its caller as before (203). A kind read before reads as naming none
(serde default). A bound left out is flume's default for a stage's,
`Defaults::send_backs` (2), as 58 leaves a field left out to its reader.

The named kind is a name, not a part. weir 89 and flume 157 keep a
package whole: no part outside it, and no stage that extends, includes
or hands over to another kind's. A fix kind's name is resolved by the
reader at a launch, as a role's entry names its kind, and the package's
hash covers the name alone. The statement's weir 89 says so, which W2
builds, and 157 matches it (decision 13).

### 2. The fix kind resolves at the base place's launch

A base place's launch resolves the name its kind gives as a launch
names its kind (154): the launch's declaration, else the repository's
own, else flume's own, a name with a source only where a lock records it
(core.34). It reads at the base's revision the launch read (20). The
launch is refused whole, naming the field and why, before the merge
queue opens anything, where the name resolves to no kind, to one that
does not land, to one with no stages, to one flume runs not (4), or to
one that reads a change while the repository binds no change profile
(core.8). Its stream, its agents' programs and the rest are read at
each fix's launch, as any launch reads them.

The base place keeps the identity it resolved and where it came from as
`PlaceRecord::fixes_as` (21), and each fix launches that identity. So a
later change to a declaration never changes the kind a base place
already launched runs its fixes as. Where the identity no longer
resolves at a fix's launch, as when the repository moved its own kind
to another version, that launch is refused as decision 8 says.

### 3. The forecast is the merge queue seam's

The merge queue seam gains one operation, the statement's new Part B
clause 210:

```rust
/// The conflicts `branch`'s head would meet were it submitted to `base`
/// now, placed after every entry already submitted; it creates no entry
/// and moves nothing, and is never read as a verdict (203, 210; sluice
/// 64, 65).
fn forecast(&self, repository: &str, branch: &str, base: &str) -> Forecast;

pub enum Forecast {
    /// The head merges onto the base's revision `onto` and every entry
    /// ahead of it.
    Clear { head: Revision, onto: Revision },
    /// It does not: the files contested, the base's revision, the
    /// entries ahead it meets, none where it meets the base itself, and
    /// the merge queue's words.
    Conflict { head: Revision, onto: Revision, files: Vec<String>, with: Vec<String>, words: String },
    /// The merge queue could not form it.
    CouldNotRead { reason: String },
}
```

The sluice profile answers it by `sluice submit <base> --check --head
<branch> --json`, run in the repository with every multiplexer's
session stripped and `GIT_TERMINAL_PROMPT=0`, as its other commands
are: exit 0 with `conflict` null is clear; exit 2 with `conflict` is a
conflict, `paths` the files, each of `with[]` named `#<id>` with its
`from.worktree` or `from.head`; anything else, or exit 2 with an
`error`, could not read, with its `error.message`. `head` and `onto`
come from the answer. Its words are sluice's text form, `would conflict:
<paths> with <branch or entries>`, under which the profile writes the
two lines a conflicting submission's words carry, `The files contested:`
and `The base's revision the merge queue tested against:`, so a fix
reads either alike (`verdict_of`). C.6 gains the row, and its "flume
never passes `--check`" becomes "never passes it to a submit". A
repository that binds no merge queue has no base place (4), so no
profile but sluice's and the fakes answers a forecast; the stand-ins
`crates/flume/tests/commands.rs` holds answer could not read.

### 4. When a settle forecasts

A settle forecasts a base place, once, where all of these hold:

- it is live, and its kind names a fix;
- no place whose base is its branch is being made or live (`on_branch`
  empty), which takes in its own live fix;
- its submission is not open: never made, or decided failed, in
  conflict, withdrawn or refused.

A base place whose last submission landed is over and is never
forecast. It is forecast past its kind's bound too, so status keeps
saying whether it still conflicts, but nothing is launched then
(decision 8). Each settle of the base place forecasts it: the watch's,
an acting command's while attended, and a settle naming it (46, 119,
208). A settle of another place never forecasts it.

### 5. What launches a fix

Three things launch one, each at the settle that reads it:

- **A forecast in conflict with the base itself**, `with` empty. The
  fix's words are the forecast's.
- **A failed submission.** The words are the merge queue's: the checks
  blamed and their notice (112).
- **A conflicting submission.** The words carry the files contested and
  the base's revision the merge queue tested against (112).

A forecast in conflict only with entries queued ahead of the head,
`with` naming them, launches nothing. Those entries are not on the base,
so the fix could take in no revision that holds them, and they may yet
fail or be withdrawn. The settle holds the forecast, status names the
entries, and the next settle forecasts again; once they land, the
conflict is with the base and launches the fix.

### 6. Each forecast and each failure acted on once

Every fix records its cause on the base place:

```rust
pub enum FixCause {
    Forecast { head: Revision, onto: Revision },
    Failed { submission: String },
    Conflicted { submission: String },
}
```

A forecast whose head and base revision equal a fix's cause, or a
failure whose submission, by the merge queue's id, is one, launches
nothing more. So a fix a
caller dropped is not launched again for the same forecast: the base
place waits on its caller until its branch or its base moves (decision
9). A fix that landed moved the branch's head, so the next forecast is
a new one, clear or not.

The settle writes the fix onto the base place before it launches, by
compare-and-set on the base place's version: its number, the next one,
its cause, its name, and the time (75). Then it launches. A settle that
finds a fix written with no place standing for it launches it, so a
command that died between the two loses nothing. Two settles that both
launch one fix have one of them refused because the place stands (3),
and the refused one takes the standing place as the fix, as `wake.rs`
takes a role's run (core.17).

### 7. The fix's launch

The settle builds a `Launch` from the base place and calls
`Engine::make_place` inside the settle, as `wake.rs` does:

| field | the fix's | why |
|---|---|---|
| repository | the base place's | 202 |
| kind | `fixes_as`, the identity resolved at the base place's launch | decision 2; 21 |
| name | `<base place's name>-<n>`, `n` the fix's number from 1 | unique while the base place stands (3); a branch's name says whose fix it was |
| base | the base place's branch | 202, 203 |
| brief | one line saying what the fix is for, a blank line, and the merge queue's words whole | 112, 203; core.36 keeps the words unrewritten |
| priority | the base place's, as its caller last set it | 70 |
| stream | the base place's | 143 |
| intent | the base place's, where its launch named one | 147; an intent keyed by its base is the fix's own branch's |
| recipients | the base place's, from `RecipientsFrom::BasePlace` | 43; core.54: the fix's ends and questions reach whoever asked for the base place |
| stall bound | the base place's | 130 |
| settings, holds, mark | none | the fix kind's own, the repository's and the host's layers give its agents' settings (32); its submission lands on the branch through the merge queue as any place's (111) |
| points | none, counted against no bound or pace | as a fix round's start puts no new points in flight (165 to 167) |

The brief's first line, which a notice shows (43), is one of:

- `Take <base> at <onto> into <branch>, which the merge queue forecast to conflict with it on <files>: merge that revision in and commit it, resolving the files contested, before anything else.`
- `Take <base> at <onto> into <branch>, whose submission conflicted with it on <files>: merge that revision in and commit it, resolving the files contested, before anything else.`
- `Make <branch> pass the checks of <base> that failed on its submission.`

The first two say what 112 and 203 ask of a conflict's fix. A kind's own
prompt renders a send-back's words only under `{{#words}}`, as flume's
own fix@1 does, and a launch's first start carries no words (27, 49), so
the brief is where that instruction reaches a fix of any kind. flume
decides nothing in it: the kind decides which fix, and the merge queue's
words decide what it does (I6).

The fix's `PlaceRecord::fix_of` names the base place, the fix's number
and its cause. It is an ordinary place from then on: cut where the base
place's branch lives (126), made ready by its merge queue's equipping
(core.47), run through, submitted to the branch and tidied once it lands
(111, 113). Its first start waits for room as any place's (70).

### 8. One at a time, the bound, and a refused launch

- **One at a time.** A live fix is a place on the branch, so decision 4
  forecasts nothing while it lives. A failure is acted on only while no
  fix of the base place's is live (`fixes` read against the store).
- **The bound.** Each fix counts toward the kind's bound, by forecast
  and by failure together. A conflict or a failure read with the bound
  spent launches nothing: the base place waits on its caller, its
  waiting words the merge queue's words and `Its kind's <n> fixes were
  launched, as many as it allows`, and stalls and is delivered as any
  wait on its caller (130, 43). The count is never reset: past the
  bound, the caller launches its own fix and retries the submission, or
  drops the base place.
- **A refused launch.** A fix whose launch `make_place` refuses, its
  kind no longer resolving, its name's branch standing, its stream gone,
  counts as a fix and is recorded refused with the refusal's words. The
  base place waits on its caller with them: `The fix <n>, <kind>, was
  not launched: <reason>`, then the merge queue's words. Counting it
  keeps a settle from trying the same refused launch at every watch,
  since such a refusal is structural; no launch is refused for room
  (70).
- **A kind that names no fix.** The base place waits on its caller with
  the merge queue's words, as today, and is never forecast.

### 9. The fix's end, written onto its base place

A fix's own settle that reads its work over writes how it ended onto
the base place's `fixes`, by compare-and-set: landed with its commit,
or dropped. It writes it before the fix's tidy removes anything, and
the tidy waits on that write, so a fix's end is never lost and a fix
held live whose place does not stand is always one written and not yet
launched, which decision 6 launches. A base place whose document is
gone takes nothing. Once its fix ended:

- **landed**: the base place is forecast again at its next settle,
  and waits on its caller where that forecast is clear (203);
- **dropped**: the base place waits on its caller, its words naming the
  fix and that it was dropped, and decision 6 launches nothing more for
  the same forecast or failure.

### 10. Status

Status's `PlaceStatus` gains, each left out where it does not apply
(61; core.14):

- for a base place, `fix_kind`, the identity its fixes run as with where
  it came from, or nothing where its kind names none; `fix_limit`, its
  bound; `forecast`, the newest forecast a settle read of it, with when,
  its head and the base's revision, and clear, in conflict with its
  files and the entries ahead it meets, or could not be read with why;
  and `fixes`, each with its number, its place, its cause, when it was
  launched and how it ended: live, landed with its commit, dropped, or
  refused with its words;
- for a fix, `fix_of`, the base place it was launched for, its number
  and its cause, and its recipients' source as `base-place`.

The person's form gives a base place's `fix kind fix (flume's own), 1
of 2 fixes`, its forecast, `forecast clear at <rev>` or `forecast
conflicts on README.md with main at <rev>`, and a fix's `launched by
flume for train--spring (fix 1, a forecast conflict)`. The base place's
line and the list are unchanged: a live fix counts among `<n> on its
branch`, and a base place waiting on its caller gives its wait in the
words status gives it (139, 200).

A forecast is no fact of a deliverable, a change or a submission (58;
core.15), and status forms no merge. The settle writes the forecast it
read onto the base place only where its outcome, head or base revision
differs from the one held, so an unchanged forecast writes nothing.

### 11. The seams, the profiles and the fakes

- `flume-seams`: `MergeQueue::forecast` with `Forecast`, no default
  method; `Kind::fix` and `Kind::fix_limit`; `PlaceRecord::fixes_as:
  Option<FixKind { identity, from: KindFrom }>` and `fix_of:
  Option<FixOf { place, number, cause }>`; `HeldState::forecast:
  Option<ForecastHeld { at, forecast }>` and `fixes: Vec<BaseFix {
  number, name, place, cause, launched, ended }>` with `FixEnded {
  Live, Landed { commit }, Dropped, Refused { reason } }`;
  `RecipientsFrom::BasePlace`; each round-tripped, a document written
  before reading as naming none.
- The sluice profile: forecast as decision 3 has it.
- Every stand-in a test holds: forecast answers could not read, naming
  the stand-in.
- The kinds profile: as it stands; flume's follow-up unit
  a-base-places-fix-kind-is-read-from-weirs-format maps weir's two
  fields once W2 lands.
- The fake merge queue: `answer_forecast(repository, branch, Forecast)`,
  clear at the base's revision where nothing is scripted, each ask
  logged `forecast <branch> onto <base>`; a forecast scripted once
  answers at every ask until scripted again.

### 12. How it is held

A base place's document gains `forecast` and `fixes`, and its record
`fixes_as`; a fix's record gains `fix_of`. Each is written by
compare-and-set (75). `fixes_as` is written once, at the launch, and
never again (21). A held state written before reads with none of them,
so a base place launched before this release runs as one whose kind
names no fix, since its kind as held has no `fix` (21), and its next
launch picks the field up.

### 13. The statement's amendment

On agentplot/blueprints' main at 31f9f1c, each clause marked as flume
record 0056's amendment, settled by agentplot-design as mechanism
within the decision the user accepted. The words:

- **23**, after "how many times a place may go back before it waits on
  its caller": "One that lands with no stages may name the kind its
  fixes run as, by its name or its identity, and how many fixes it runs
  before its place waits on its caller (203)."
- **56**, after the place's points: "for a base place, the kind its
  fixes run as with where it came from, the newest forecast a settle
  read of its branch, when and what it found, and each fix flume
  launched for it, why and how it ended; for a place flume launched as a
  base place's fix, that base place (203);"
- **157**, at its end: "A kind that lands with no stages may name the
  kind its fixes run as (23): a name its reader resolves as a launch's
  name resolves (154), and no part of the package, whose hash covers the
  name and not the kind it names."
- **165**, after "it is counted against no bound or pace (202)": "A base
  place's fix, which flume launches (203), gives no points and is
  counted against no bound or pace, as a fix round's start puts no new
  points in flight (166, 167)."
- **203**, its sentences from "While it so waits" to "Its submission
  stays its caller's retry." become: "While it so waits, unsubmitted or
  its submission failed, with no place on its branch being made or live,
  each settle forecasts its branch's head against its base through the
  merge queue (210; sluice 64). A conflict so forecast with the base
  itself, or a failed or conflicting submission of it, launches on its
  branch a place of the kind its kind names for its fixes, resolved at
  the base place's launch (23), named by the base place's name and the
  fix's number joined by `-`, in its stream, at its priority, serving
  the intent its launch named, delivering to its recipients, and with
  the merge queue's words as its brief; for a conflict, that fix takes
  in the revision the words name (112). A forecast that conflicts only
  with entries queued ahead of it launches nothing and is forecast again
  at the next settle. flume acts on each forecast, by its head and its
  base's revision, and on each submission once. While the fix is live no
  second is launched; each fix counts toward its kind's bound (23), one
  whose launch was refused among them, past which the base place waits
  on its caller, said and delivered (130, 43); a refused launch leaves
  it waiting on its caller with the refusal's words; and a kind that
  names no fix leaves it waiting on its caller with the merge queue's
  words. Its submission stays its caller's retry."
- **210**, new, in B.6 after 206: "**forecast**: given a repository, a
  branch the merge queue lands on and its base, the conflicts the
  branch's head would meet were it submitted to the base now, placed
  after every entry already submitted: clear, with the head and the
  base's revision it was formed on; or in conflict, with those, the
  files contested, each entry ahead it meets, none where it meets the
  base itself, and the merge queue's words; or that the merge queue
  could not form it, with why (203; sluice 64, 65). A forecast creates
  no entry, moves no branch, and is never read as a verdict."
- **C.6**: the operations table gains "forecast | `sluice submit <base>
  --check --head <branch>`, run in the repository, which forms the
  candidate sluice 64 forms for a head not yet submitted and submits
  nothing (203, 210)"; the submit row's "flume never passes `--check`"
  becomes "flume never passes `--check` to a submit"; and the commands
  table gains "forecast | `sluice submit <base> --check --head <branch>
  --json` answers `{conflict, head, onto, ahead}`: `conflict` null and
  exit 0 where clear; exit 2 with `conflict.paths`, the files
  contested, and `conflict.with[]`, the entries ahead it meets, each
  with its `id` and `from`, where it conflicts; a refusal exits 2 with
  its `error.message`. The profile reads it of sluice 0.71.0".
- **S112**: "A base place's submission" becomes "A base place, whose
  kind names no fix: its submission".
- **S120**, new: "A base place's kind names flume's own fix as the kind
  of its fixes, and both changes on its branch have landed; no caller
  retries its submission. Another place lands on `main` a change to
  `README.md`, which the base place's branch changed too. At the host's
  next watch, flume forecasts the branch against `main` through sluice,
  finds it in conflict on `README.md` at `main`'s new revision, and
  launches a fix on the branch, named for the base place and its first
  fix, with sluice's words as its brief, in the base place's stream and
  delivering to its recipient. The fix's agent merges that revision in,
  resolves `README.md`, and the fix lands on the branch. The next
  forecast is clear, and the base place waits on its caller, who retries
  its submission, which lands. Status gave the fix as launched by flume
  for the base place. Had `main` moved twice more under it, each move
  conflicting, its kind's bound of two fixes would have been spent and
  the base place would have waited on its caller, said and delivered
  (23, 43, 130, 203, 210)."
- **weir**, with W2: 52 gains "A kind that lands with no stages may name
  the kind its fixes run as, by a name its reader resolves, and how many
  fixes it runs before its place waits on its caller"; 57 refuses either
  on a kind with stages, naming the field; 89's "no other kind" gains
  "but the kind a kind with no stages names for its fixes, a name its
  reader resolves and no part of the package"; and C.4's `train` gains
  both fields.

### 14. The rules and the record

Written by this change's construct, citing record 0056, and held by its
build:

- **core.53**, its rule, after "its outcome read from the merge queue's
  record of its branch;": "while it waits on its caller with no place
  on its branch live, each settle forecasts its branch against its base
  through the merge queue, and a conflict so forecast with the base, or
  a failed or conflicting submission, launches the fix its kind names on
  its branch, one at a time, each forecast and submission acted on once,
  within its kind's bound;". Its details: the rules-out item "a stage,
  an agent or a fix of flume's own started on a base place's failed
  submission (112; core.36)" becomes "a stage or an agent started in a
  base place, or a fix of a kind its kind does not name launched for it
  (112; core.36)", and it gains: "a fix launched while one of the base
  place's is live, past its kind's bound, again for a forecast's head
  and base revision or a submission already acted on, for a forecast
  that conflicts only with entries queued ahead, or as a kind resolved
  at its own launch rather than the base place's (203); a forecast while
  a place on its branch is being made or live, or its submission open
  (203); the base place submitted after its fix lands by anything but
  its caller's retry (203; core.6); a fix given points or held by a
  points bound or pace (165)". Not ruled out gains: "a base place whose
  kind names no fix waiting on its caller after a failure, as before; a
  caller's own fix launched on its branch beside flume's (203; record
  0056)". Sources gain 23, 157, 165, 210 and record 0056.
- **core.34**, its rule's "a kind refers to nothing outside its
  package" becomes "a kind refers to nothing outside its package but the
  kind a kind with no stages names for its fixes, by name"; its details'
  not ruled out gains "a kind that lands with no stages naming the kind
  its fixes run as, resolved at its place's launch and no part of its
  package or its hash (23, 157, 203; record 0056)".
- **core.6**, its details: the rules-out item "`--check` passed to the
  merge queue" becomes "`--check` passed to a submit"; not ruled out
  gains "the forecast's `sluice submit <base> --check --head <branch>
  --json`, which submits nothing (210, C.6)".
- **core.36**, its details' not ruled out gains "a base place's fix,
  which flume launches on a forecast conflict or a failed or conflicting
  submission of the base place, its kind naming it, which reads no
  base's checks (203; core.53; record 0056)".
- **Record 0056** gains an amendment line: "Amended 2026-11-07 by the
  change a-conflicting-base-place-gets-its-fix-launched, settled by
  agentplot-design as mechanism within the decision the user accepted:
  the forecast is the merge queue seam's (210, C.6); the fix kind is
  named by weir's format and resolved at the base place's launch; a
  forecast that conflicts only with entries queued ahead launches
  nothing; each forecast and each submission is acted on once; the fix
  takes the base place's stream, priority, named intent and recipients,
  and no points; a refused launch counts toward the bound (23, 56, 157,
  165, 203, 210, C.6, S112, S120). The statement carries it, with weir
  52, 57, 89 and C.4, at agentplot/blueprints 31f9f1c. weir's format
  gains the fix kind by weir's unit a-kind-with-no-stages-names-its-fix,
  and flume reads it by its unit
  a-base-places-fix-kind-is-read-from-weirs-format after it; until then
  a kind in weir's format names no fix, and its base place waits on its
  caller as before (203)." core.53's and core.34's marks name the
  amendment as settled, and the site's rules page mirrors both rules.

## Who owns what

| piece | owner | flume's part |
|---|---|---|
| the kind of a base place's fixes and its bound, in the format | weir (W2) | read through weir-kinds (core.4), by the follow-up unit after W2 |
| which fix kind a bolt names | crew's bolt kind, in its package (C2) | resolve and launch it |
| forming the forecast | sluice (64) | ask it through the seam, by one command |
| when to forecast, what to launch, the bound | flume (203) | this change |
| what the fix does | the fix kind's prompt and the merge queue's words | carry the words as the brief |
| submitting the base place | its caller (203) | unchanged |

### Named follow-ups

- **agentplot/weir, W2: a kind with no stages names its fix**, queued
  as weir's unit a-kind-with-no-stages-names-its-fix. weir 52, 57, 89
  and C.4 as design 13 words them, at agentplot/blueprints 31f9f1c,
  with weir's own record; weir-kinds' `Kind` gains the kind a kind that
  lands with no stages names for its fixes and its bound, read on such a
  kind and refused on any other naming the field, and `weir check` lists
  them. The pack holds a kind with both, and one with stages that names
  either, refused.
- **flume, a-base-places-fix-kind-is-read-from-weirs-format**, after
  W2. weir-kinds' pin in `Cargo.toml` moves to the revision of weir's
  main that lands W2, and `map_kind` maps weir's two fields of a kind
  with no stages onto `Kind::fix` and `Kind::fix_limit` in weir's
  spelling and from nothing else; the kinds profile's scenarios read a
  `train` naming `mend` with a bound of 2, a `yard` naming none, and a
  kind with stages naming a fix refused in weir's words, over a scratch
  repository; and the kinds guide's kind with no stages gives both
  fields in weir's spelling (4, 23, 157; weir 52, 57, 89; core.4,
  core.22, core.35). Until it lands, a kind in weir's format names no
  fix, and its base place waits on its caller as before (203).
- **afterthought/crew, C2: a bolt's kind names its fix.** Once crew's
  bolts are base places (crew record 0056), crew's bolt kind names the
  kind its fixes run as, and crew retires its own bolt-against-main
  forecast in the conductor's wait (sluice C.5's crew row) and the
  planner's start of a take-main fix, since flume does both. Record 0056
  names it among its consequences. No clause of flume's changes for it.

No follow-up for sluice, whose pinned 0.71.0 forecasts a named head, or
for Boomdrive, whose runner keeps the watch that settles (208).

## Alternatives Considered

- **The caller forecasts and launches the fix** (record 0056's option
  2). Every caller builds the same watch, and one that does not leaves a
  bolt waiting, as crew did for 9h 46m. The user ruled for flume.
- **flume submits a proven base place on its own** (record 0056's
  option 3). The submission to main waits on the user's send (record
  0023), so it answers nothing before the send.
- **The base place's launch names its fix kind**, a launch's field,
  rather than its kind. It needs no weir change and no amendment of 89.
  Record 0056 has the kind name it, as a kind with stages names its fix
  round, so every bolt of one kind fixes alike and no caller repeats it.
- **The host's or the repository's configuration names a fix kind by
  the base place's kind's name**, as settings are given by a kind's
  name. That makes the fix a host's or repository's choice, and a kind
  names it nowhere a reader of the kind sees.
- **Resolve the fix kind at each fix's launch.** A kind that resolves to
  nothing is then found at a settle hours after the launch, with a
  conflict standing and nobody fixing it, and a later declaration
  changes the fixes of a base place already launched (21).
- **Launch a fix on any forecast conflict**, entries ahead among them.
  The fix could merge no revision holding those entries, and they may
  fail and leave the queue, so the fix would chase work that never
  lands.
- **Launch again after a fix is dropped.** The caller who dropped it
  would have it launched again at the next watch. Decision 6 waits for
  the branch or the base to move.
- **Count no refused launch.** A structural refusal would be tried at
  every watch for ever.
- **Tidy a fix whether or not its end reached its base place**, a fix
  held live with no place standing written over at the base place's
  next settle, its end unread. That settle could not tell such a fix
  from one written and never launched, which decision 6 launches, so it
  would launch a landed fix again; the build's own choice, cheap to
  change.
- **Carry the words as the fix's first start's words** rather than in
  its brief, so a kind's `{{#words}}` renders them. A launch's first
  start carries no words (27, 49), and the brief reaches a fix of any
  kind.
- **A new version of flume's own fix carrying the merge instruction.**
  The brief's first line carries it for every fix kind, a crew's among
  them, and fix@1's content stays as released (156).
- **Skip a forecast whose head and base's revision have not moved**,
  keeping the last. sluice's reader already makes a forecast a merge
  with no fetch (sluice 64), and the held forecast is written only where
  it changed; a skip would hold a forecast formed against queued entries
  that have since moved.

## Risks / Trade-offs

- **A merge at each settle of each waiting base place.** Record 0056's
  bad consequence. sluice forms it on its reader's document with no
  fetch where the reader keeps its tick, and a host holds few base places
  at once; the watch's bound sets the rate.
- **A fix that cannot resolve the conflict.** It stops short or is
  dropped; the base place waits on its caller once the bound is spent,
  said and delivered, as any place past its bound does (131).
- **The brief's first line is flume's words.** It restates what 112 and
  203 ask of a conflict's fix, and the merge queue's words follow it
  whole (core.36). A kind that wants more says so in its own prompt.
- **Recipients shared with the base place.** The fix's stage ends and
  stop-shorts reach the base place's recipients, which is who asked for
  the work (core.54); a caller that wants them elsewhere launches its
  own fix.
- **Until the follow-ups land, no kind names a fix.** The seams, the
  core, status and the sluice profile build and test over the fakes,
  whose kinds are built in Rust. A kind in weir's format names no fix
  until W2 and flume's follow-up unit after it land, and its base place
  waits on its caller as before (203).

## Migration Plan

Nothing held changes shape for what stands: every new field is read as
none from a document written before. A base place launched before this
release has no `fix` in its kind as held (21), so it is never forecast
and behaves as today; its caller's next base place picks the field up
once its kind names one. Status only gains fields. A bolt of crew's
gains flume's fix only once C2 names its fix kind.

## Open Questions

None for this change. W2 settles the fields' spelling, and flume's
follow-up unit follows it.

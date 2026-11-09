# Design

## Context

flume records each end of a place once, on the place's held state, as an
`EndHeld` with a list of `Delivery`, to whom, when and by what
(`DeliveredBy::Notice`, `Wait`, `Inbox`). `EndHeld::pending` is every
recipient it has no delivery to. `deliver_ends` (`crates/flume-core/src/ends.rs`)
claims a notice's delivery in held state, sends it, and takes the claim
back where the send was refused. A wait and the inbox (`wait.rs`) push a
delivery in the write that reads the end, so a second inbox gives
nothing. `wake_roles` (`wake.rs`) wakes an address an end is pending for,
and `keep_for_ends` and `settle_tidied` keep a tidied place's document
while `HeldState::pending_for` names anyone.

weir's record 0006, accepted by the user on their card on 2026-11-08,
gives flume and sluice one form for this. weir's statement holds it at
agentplot/blueprints 3c92bcb (A.14 100 to 107, B.7 108 to 113, C.8, I9,
S22 to S25), and weir's main 52b95528 holds `weir-delivery`, its records,
their JSON and recutils spellings, the calls `record`, `hand`, `confirm`
and `owed` over a store's records, and the `Store` seam; and
`weir-pack`, whose `conform_deliveries` holds any deliverer's store to
the pack's 21 delivery cases.

flume's statement is amended for it at
`design/flume-ends-are-weirs-deliveries`, 01b637a, over record 0060's
amendment at 479a6ab merged with main at 3c92bcb: the vocabulary
(delivered, handed over, collect, notice), 43, 56, 62, 132, C.4, C.7,
the runner's row and S121 to S123. Record 0061, proposed, decides what
weir leaves its deliverer, and rules core.11, core.25 and core.28 carry
it, at df54bb9c. This design relies on core.4 (the format's crate
spells it), core.13 (no process waits for an expiry), core.14 (one form,
only gaining fields), core.15 (status reads where facts live, once per
ask), core.16 and core.17 (host state by compare-and-set, every act safe
to repeat), core.22 (the pins), core.25 and core.28, and touches the
area of `docs/architecture/core.md` alone, which has no area pages.

### What the family does

Read on 2026-11-08. weir owns the form and holds no store; each
deliverer plugs its own in (weir 86, 112). sluice keeps a submission's
outcome on its entry, delivered once to its submitter's session (sluice
51), and moves onto the form in its own change,
`sluices-inbox-is-weirs-delivery-form`, not yet written; flume's
merge-queue profile reads sluice's inbox as today until then. Boomdrive's
runner collects a tidied place's end by `flume inbox <place>` (its
change `the-runner-carries-each-places-end`); S121 now has it collect by
`flume inbox --confirm`. crew's role runs collect by `flume inbox` at
their start (crew record 0047). The command spellings below, `confirm`
and `--confirm`, are offered to sluice to share.

## Goals / Non-Goals

**Goals:**

- An end stays pending for a recipient until that recipient confirms
  it, whatever handed it over.
- An end nobody confirms is shown expired, with its recipient, and never
  dropped; a name nothing was confirmed under is shown as such.
- flume's records are weir's, written by weir's crate, its store held to
  weir's pack.

**Non-Goals:**

- sluice's move onto the form, and the merge-queue profile confirming
  sluice's deliveries after it.
- crew's role kinds and the runner confirming what they act on.
- A notice sent again after a time: a notice goes once per recipient,
  and the inbox and a wait give a pending end again whenever asked.
- Letting go of a live place's done deliveries before its document goes.

## Decisions

### 1. weir at one revision

`Cargo.toml`'s `[workspace.dependencies]` takes `weir-kinds`,
`weir-delivery` and `weir-pack` from `https://github.com/agentplot/weir`
at `52b95528a495100e34fca39109d563d8bf75673a`, one revision for all
three, the comment above them saying so (core.4, core.22). weir-kinds and
weir-capture did not change between daacda3 and 52b95528, so the kinds
profile reads as before. `crates/flume-seams` and `crates/flume-core`
take `weir-delivery`; `crates/flume-conformance` takes `weir-pack`. The
statechart engine, when it comes, is taken at the same revision (core.42).

`nix/packages/git-crates.nix` gives each git package Cargo.lock holds,
`weir-capture-0.0.0`, `weir-kinds-0.0.0`, `weir-delivery-0.0.0` and
`weir-pack-0.0.0`, the revision and its hash in one commit, the hash
taken as the file's comment says.

### 2. The host's deliveries document

`HostKey::Deliveries` names one host document, `HostDocument::Deliveries`,
holding `records`, every record of C.8 in the order added (C.7; weir
112). Each record is a `DeliveryRecord` in `crates/flume-seams`, a
newtype over `weir_delivery::Record` that serializes to the object weir's
`json::write` spells and reads back through weir's `json::read`, a
refusal naming the record's field; flume spells no field of it
(core.4). The state directory keeps it at `deliveries.json` beside
`watch.json`, with its version, written whole under its own lock,
readable by the host's user alone, as every host document is.

`crates/flume-core/src/deliveries.rs` holds weir's `Store` over flume's
store seam, `HostDeliveries`: `read` gives the document's records, none
where there is no document; `add` reads the document's version and
writes the records after the others at that version, every one or none,
a moved version an error. The engine's own writes go through
`Engine::deliveries(|records| -> Vec<Record>)`, which reads the records
and the version, asks one of weir's calls over them, and writes what it
gives at that version, reading again on a moved version, as `update`
does for a place (75; core.17). So no call of weir's answers over
records another process has added to since.

A store seam scenario holds both stores to weir's pack: it runs
`weir_pack::conform_deliveries(&Pack::embedded(), …)` over
`HostDeliveries` on a fresh store per case, against the fakes' store and
the state directory (weir 87, 113).

### 3. An end is recorded as a delivery

A launch takes a token, eight hex characters of the host's secret, kept
as `PlaceRecord::token`. A place's document written before has none; the
first write of a settle that reads it gives it one. An end's delivery key
is `<kind>--<name>/<token>/<end key>`, its end key as `EndKey`'s
`Display` spells it, so two launches of one place never share a key
(weir 101).

`record_ends` (`crates/flume-core/src/held.rs`) records ends in the
place's write as today. After that write, the settle records each end's
delivery through weir's `record`: its readers `EndHeld::recipients`
gives, its message the end's notice (decision 4), at the end's
`read_at`, with the host's span (decision 10). An end with no recipient,
a drop of a place whose recipients are all roles' addresses, records no
delivery. `record` answering the key held writes nothing (weir 101), so
recording again is the same act (core.17).

A record's time is the host clock's seconds written RFC 3339 in UTC,
`2026-11-08T15:02:00Z`, by the workspace's chrono, and read back to
seconds by its instant; status gives every time in seconds, as it gives
every other (56).

Two writes, the place's and the deliveries', cannot be one. Every
settle, wait and inbox therefore records the delivery of each end it
reads that has none before it reads what is owed, and status reads such
an end as that record would read: owed to each of its recipients, or as
decision 11 gives an old one.

### 4. The message and the notice

An end's message is its notice, the one line `notice` in `ends.rs`
words, gaining `; confirm: flume confirm <delivery key>` after its read
command. It is fixed when the end's delivery is recorded, the read
command chosen from the place as that settle's write left it, and every
notice of the end sends the message as recorded (weir 100; core.11).

### 5. A notice is a hand-over

`deliver_ends` sends a notice of an end to each recipient that has not
confirmed it, to which nothing has handed it, and for which no notice is
claimed. It claims the notice on the place, `EndHeld::noticing`, a
`NoticeClaim` of the recipient and when, in a compare-and-set write
before the send, so two settles at once send one notice (75). On the
terminal's answer sent it records the hand-over through weir's `hand`,
`by` `notice`, confirming nothing, and keeps the claim; on a send
refused or a session not found it takes the claim back, which is held
state of flume's and no record of weir's (weir 113). A claim whose
hand-over the deliveries document lacks is recorded at the next settle
at the claim's time, so a crash between the send and the record sends no
second notice. A standing agent's notice goes the same way (132, 190).

### 6. The wait, the inbox and confirming

A wait reads the newest end of its stage's start or submission as today.
Where it is pending for the recipient, the wait records a hand-over by
`wait` and answers with it; where the recipient confirmed it, it answers
it as a read and hands nothing over. `flume wait --confirm` passes
`confirms` to `hand`, so the hand-over and the confirmation are written
together (weir 103).

The inbox gives every end pending for the recipient, of every document
or of those a place's name reads, owed or expired, handed before or not,
and records a hand-over by `inbox` for each. `flume inbox --confirm`
confirms each on receipt. A second inbox without `--confirm` gives the
same ends again. `Collected` gains `confirmed`, true where this call
confirmed it.

`flume confirm <key>…` confirms each key through weir's `confirm` as
the recipient the command names, its default as `flume inbox`'s is (7,
132). Each key gives the time it was confirmed
and whether it was confirmed already, weir's `Already`; a key no end is
under, or a recipient not among its readers, is refused in weir's words,
naming the key and the field, and confirms nothing of the keys after it
(weir 102). The answer is `Answer::Confirmed { recipient, confirmed }`.

The plugin's SessionStart runs `session_inbox`, which hands each end
over by `session-start` and confirms it on receipt, since the hook gives
the agent the inbox's words whole (C.4; weir 103).

### 7. Waking a role

`wake_roles` wakes an address an end is pending for only where nothing
has handed that end to it: no hand-over of it to the address, and no
notice claimed. A run's inbox records hand-overs, so an end a run was
handed and did not confirm wakes no run after it; the next run another
end wakes is handed it by its own inbox (132, S122; core.28). Status's
addresses list each address an end is pending for as today.

### 8. What status gives

`EndStatus` keeps every field and gains (61; core.14):

- `delivery`, the delivery's key, none for an end with no recipient;
- `expires_at`, by the host's clock;
- `owed`, each recipient that has not confirmed it, with `expired` and
  `known`, the last whether a `weir-known/1` names it (weir 104, 106);
- `confirmed`, each recipient's confirmation and when.

`deliveries` becomes each hand-over, `{recipient, at, by}`, read from
the deliveries document, `by` one of `notice`, `wait`, `inbox` and
`session_start`, and an old end's as decision 11 records them.
`pending_for` names each recipient in `owed`. `HostStatus` gains
`expired`, each end expired for a recipient on the host, with its place,
its key, its delivery's key, the recipient, `known` and `expires_at`
(56). One status reads the deliveries document once, beside each place's
own (core.15). The person's form gives, under each end, its recipients
pending, expired and confirmed, and a name never confirmed under as
`never confirmed`; and, for the host, each expired end.

### 9. A tidied document stays until every end is delivered

`HeldState::pending_for` gives the recipients each end is owed to,
read with the deliveries document. `keep_for_ends` and `settle_tidied`
keep the document while it names anyone, so a tidied place's document
goes once every end is delivered to each recipient (62; weir 107). The
settle that removes it then lets go of every record of its ends'
deliveries, the delivery with its hand-overs and confirmations, each of
which is done; `weir-known/1` records stay (weir 106). A settle also lets
go of any done delivery whose place has no document, as after a removal
whose let-go did not land. A drop of a tidied document while
`pending_for` names anyone is refused, naming each recipient and `flume
confirm` (62; core.25). A drop of a live place is as before: its end is
recorded, and its document kept while pending.

### 10. The span

`[host] ends_expire` is read by the host configuration as `keep_open`
is, `<n>`, `<n>s`, `<n>m` or `<n>h`, from the managed layer alone, as
every `[host]` key is (C.7; core.45); 604800 seconds where it is not
given; `0` refused, since weir's span is at least one second. `Host`
gains `ends_expire()`, read at each delivery's record and fixed there
(weir 104). Expiry is read at each ask and settle against the host's
clock; no process waits for it (core.13). Status gives the value with
its layer (56).

### 11. What was written before

`EndHeld::deliveries` is the old form's list. It is read and never
added to; status fills its copy of the end's list from the hand-overs,
so the form keeps `deliveries` (61). The first settle, wait or inbox that reads
an end holding one records its delivery as decision 3 does, then, for
each old delivery, `hand` with its recipient, its way (`notice`, `wait`
or `inbox`) and its time, confirming on receipt, since the old form
counted it delivered and its recipient was told so. A recipient with no
old delivery is owed the end. Once the deliveries document holds those
records, the place's next write empties the list. `hand` answers a
reader that confirmed with `Confirmed` and adds nothing, so reading the
old form twice writes nothing twice (core.17).

### 12. What the person and the site read

`flume confirm --help`, `flume wait --help` and `flume inbox --help` say
that a hand-over delivers nothing until the recipient confirms, and how,
to any host's operator (core.35). The site's commands, state and
configuration references and its callers-and-holds, roles-and-wake and
claude-code guides say it the same way.

## Risks / Trade-offs

- [One host document takes every delivery's write] → Writes are few: one
  per end, hand-over and confirmation. Each reads and writes at a
  version, and a moved version reads again.
- [Callers that collect once are handed the same ends again] → That is
  the form's point. The proposal names the callers that must confirm,
  and `--confirm` keeps theirs one call.
- [A misspelt recipient keeps a tidied document] → It shows expired in
  status, with its name never confirmed under, and the operator confirms
  as that name.
- [An upgrade mid-flight] → Decision 11 counts every old delivery
  confirmed, so no end delivered before is sent again.

## Migration Plan

None beyond decision 11: a host's held state upgrades as each settle
reads it.

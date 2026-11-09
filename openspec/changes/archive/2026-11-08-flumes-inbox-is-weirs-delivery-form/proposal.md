# flume's inbox is weir's delivery form

## Why

flume counts an end delivered to a recipient the moment it is handed
over: a notice once the terminal took it, a wait once it answered, the
inbox once it listed it. A recipient that stops before it acts on what
it was handed never sees it again, and an end nobody collects stays
pending for good, unseen. The user asked on 2026-11-08 "Is it actually
tracking what was read?", and weir's record 0006, accepted on their card
the same day, gives flume and sluice one form for what a tool owes a
reader, a delivery, and one crate, `weir-delivery`: owed to each reader
until that reader confirms it by its key, a hand-over confirming
nothing, an expiry fixed when recorded and then shown, never dropped.
Its order of work names this change: "Flume, then sluice, move onto the
crate, each in a change of its own."

The statement is amended for it at agentplot/blueprints
`design/flume-ends-are-weirs-deliveries`, 01b637a, over record 0060's
amendment at 479a6ab merged with main at 3c92bcb: the vocabulary, 43,
56, 62, 132, C.4, C.7, the runner's row and S121 to S123, citing weir's
100 to 113, C.8 and I9. Record 0061, proposed, holds the rules weir
leaves its deliverer; core.11, core.25 and core.28 carry them, at
df54bb9c.

## What Changes

- **An end is a delivery (43; weir 100, 101).** The settle that records
  an end records its delivery through `weir-delivery`, under the key
  `<kind>--<name>/<token>/<end key>`, the token one the place's launch
  took; its readers the end's recipients; its message the end's notice.
- **The host's deliveries are one document (C.7; weir 112, 113).** Every
  record of weir's C.8 for the host's ends, in the order added, in one
  host document beside the places', written whole by compare-and-set.
  flume's store of it passes weir's pack.
- **Hand-overs deliver nothing (43; weir 103, I9).** A notice, a wait,
  the inbox and a session's start each record a hand-over; the end stays
  pending for the recipient. A notice goes to a recipient once, while
  nothing has handed the end to it. A wait and the inbox give a pending
  end each time they are asked, until it is confirmed.
- **A recipient confirms (43; weir 102, 103).** **BREAKING** for a caller
  that collects: `flume confirm <key>…` confirms as the recipient the
  command names, and `flume wait --confirm` and `flume inbox --confirm`
  confirm on receipt. The Claude Code plugin's SessionStart confirms on
  receipt (C.4).
- **Expiry and known names (43, 56; weir 104, 106).** `[host]
  ends_expire`, a week where unset: an end pending past it shows expired
  with its recipient in status, the inbox and a wait, and the host's
  status lists each one. Status says, for each recipient, whether
  anything was ever confirmed under its name.
- **Waking (132; core.28).** A role is woken for an end pending for its
  address that was never handed to it; one a run was handed and did not
  confirm wakes no run again, and the next run is handed it.
- **A tidied document stays until delivered (62; weir 107).** It goes
  once every end is delivered to each recipient, and its deliveries are
  let go of after it; its drop is refused while an end is pending.
- **What was written before.** An end with deliveries of the old form is
  recorded at the first settle, wait or inbox that reads it, each old
  delivery a hand-over confirmed at its time.
- **The form only gains fields (61).** An end keeps `deliveries`, each
  hand-over now, and `pending_for`, and gains `delivery`, `expires_at`,
  `owed` and `confirmed`.

## Capabilities

### New Capabilities

- `deliveries`: flume's store of weir's deliveries, the host's one
  document, its key, its records, held to weir's pack.

### Modified Capabilities

- `end-delivery`: an end is a delivery; hand-overs deliver nothing; a
  recipient confirms; expiry and known names; a tidied document stays
  until every end is delivered, and its drop is refused while one is
  pending; an end written before reads delivered where it was.
- `role-waking`: a role is woken for an end never handed to its
  address, and a run collects what its address was handed before.
- `place-status`: each end's delivery, by recipient, and the host's
  expired ends.
- `command-line`: `flume confirm`, and `--confirm` on `flume wait` and
  `flume inbox`.
- `claude-code-plugin`: SessionStart confirms on receipt.
- `store-profile`: the host's deliveries document in the state
  directory, and `[host] ends_expire`.
- `seam-contracts`: an end pending until confirmed; the types of the
  deliveries document, the place's token and a notice's claim, and an
  end's old deliveries read.

## Impact

- `Cargo.toml`, `Cargo.lock`, `nix/packages/git-crates.nix`: weir at
  main 52b95528, weir-kinds' revision moved there with weir-delivery
  beside it and weir-pack for the suite. weir-kinds and weir-capture are
  unchanged between daacda3 and 52b95528.
- `crates/flume-seams`: `held.rs` (`PlaceRecord::token`, `EndHeld`'s
  notice claims, its old `deliveries` read and never written),
  `store.rs` (`HostKey::Deliveries`, `HostDocument::Deliveries`),
  `round_trip.rs`.
- `crates/flume-core`: a new `deliveries.rs`, weir's `Store` over the
  store seam and the engine's record, hand, confirm and owed;
  `ends.rs`, `wait.rs`, `wake.rs`, `standing.rs`, `held.rs`,
  `status.rs`, `answer.rs`, `tidy.rs`, `launch.rs`.
- `crates/flume`: `commands/mod.rs` (`flume confirm`, `--confirm`),
  `commands/print.rs`, the host configuration's `[host] ends_expire`,
  the Claude Code hook's SessionStart, `tests/commands.rs`,
  `tests/profiles/store_state_dir.rs`.
- `crates/flume-fakes`: the store's host documents and the host's span.
- `crates/flume-conformance`: the store seam's pack scenario; the
  end-delivery, role-waking, place-status and answers scenarios.
- `site/`: the commands, state and configuration references; the
  callers-and-holds, roles-and-wake and claude-code guides.
- No new crate, plugin or site part, so no new moon project, and no new
  tool. The suite's clause list stays at 9c060ab, so no scenario cites
  S121 to S123 until a chore moves it.
- Callers: crew's role kinds, Boomdrive's runner (its change
  `the-runner-carries-each-places-end`, which S121 now has collect by
  `flume inbox --confirm`) and sluice's own move onto the form
  (`sluices-inbox-is-weirs-delivery-form`) are theirs.

# Design

The statement is read at boomdrivedev/blueprints main 81288ba: the
vocabulary's reading, 144 to 147, 152, 153, 171, 174, 184, 185, 211 and
C.7, with C.4's `windows` row and the common profile's `windows` unknown
on `design/flume-claude-codes-windows-row` at bd79555, cut from that
main. The decision is record 0062, accepted by the user on 2026-11-09,
and its amendment of 2026-11-14, this change's construct's choices
within it; `design/principles.md` 14 is why. Where this design and the
statement disagree, the statement wins (core.19).

The rules relied on: core.1 and core.2 (the core names no program; the
Claude Code profile builds the seam's `windows` part and the core alone
writes the host's documents); core.13 (the module runs in the agent
program's own process and the hook ends with its command); core.14
(status's JSON form only gains fields); core.16 and core.17 (the
account's document written whole by compare-and-set, a report taken
twice changing nothing); core.22 (the module typed against the engine
the pinned Claude Code 2.1.296 writes); core.30 (no path and no value of
a variable in status or a refusal); core.32 (the reading is the windows
the program reported in a session flume started, kept in the host's own
documents, asking nothing of the program or its maker and reading no
file a host writes); core.39 (a part built from the program's own
record, or degraded as its profile declares); core.45 (the accounts are
the managed layer's). No area page exists; core.md binds every path this
change touches. The change adds no metric, so core.56 holds as it is.

## 1. The windows, a part of the agent seam (171, 174, 211)

`crates/flume-seams/src/agent.rs` gains:

- `Windows`: `Reported { taken_at, windows, session }`, the windows the
  agent's program last reported for its account, when flume's own part
  of the program took them, by the host's clock, and the session they
  came from where the record names one; `NoneYet`, the record readable
  and nothing reported; `Unknown`, the record unreadable or the part not
  built.
- `Agent::windows(&self, folder, agent) -> Windows`, read in the place's
  own folder as `spent` is, `Unknown` by default.
- `Agent::reports_windows(&self) -> Offered`, whether the profile builds
  the part, `NotOffered` by default. A program whose profile does not is
  degraded there: its accounts have no reading, and status says so
  (171, 174).

The core asks the seam and never the program: a reading reaches the
host's documents only through `windows` (core.2).

## 2. The module that passes the windows (C.4 at bd79555)

`plugins/claude/hooks/usage.ts`, a module of its own beside `line.ts`,
typed against the vendored engine types. The pinned engine loads one
hooks module per plugin, refusing a second entry in `hooks/hooks.json`'s
`modules`, so the one listed is `hooks/flume.ts`, which registers
`line.ts`'s hooks and then `usage.ts`'s and hooks nothing itself; the
path not taken, the handler written into `line.ts`, gives the place's
line a second job.



- Whether flume started the session is read once from its environment,
  `FLUME_PLACE` and `FLUME_AGENT` both set, as `line.ts` reads it.
- On `session.measure`, after `next(e)`, where flume started the
  session, `e.changed` names `rateLimits` and `e.rateLimits` holds a
  window, it runs `flume hook usage` by `$.process.run`, its `stdin`
  `{"rateLimits": e.rateLimits}` as the engine gives them, bounded at
  ten seconds, and does not wait on it. A run that fails changes
  nothing, and the handler never throws into the engine.
- Outside a session flume started it registers nothing that runs a
  process.
- The binary is `const FLUME = "flume";`, which the profile writes as
  the binary's absolute path, as it writes `line.ts`'s
  (`profiles::agent::claude::written`).

The engine's first measurement names every unit it has a figure for, so
the session's first windows pass with it. An empty `rateLimits` is the
engine having no reading, off a subscription or before the first
response, and passes nothing: an account signed in with a key has no
reading, which status says. The path not taken, passing an empty list as
a reading of no windows, would replace a reading kept with nothing.

## 3. `flume hook usage` (the Claude Code profile)

`crates/flume/src/profiles/agent/claude/windows.rs`, beside `hook.rs`:

- **The report read.** The input's `rateLimits`, each window a `kind`,
  a non-empty string, a `percentUsed`, a number of 0 or more, and an
  optional `resetsAt`, which RFC 3339 reads. Each becomes a
  `UsageWindow` named by its `kind`, its use in hundredths, a use past
  100 read as 100, and its reset by the host's clock. A report with a
  window in no such form, or no `rateLimits` list, is no report: it
  keeps nothing, and the reading kept stands. The path not taken,
  keeping the windows that read, could read an account under its
  threshold with the window that holds it left out.
- **Kept beside the start's record.** `windows.json` in the start's
  record's folder (`record::Record`), `taken_at` the time the hook ran,
  by the host's clock, and the windows, written whole beside and renamed
  over. A file there whose `taken_at` is later is kept.
- **Taken by the core.** The hook then asks the core to take the
  reading of the start `FLUME_AGENT` names with `FLUME_SECRET` (section
  4). It prints nothing and exits 0 whatever happened, as every hook
  does; outside a session flume started it reads nothing and writes
  nothing.
- **The part.** `Claude::windows(folder, agent)` reads `windows.json` in
  the start's record: `Reported` with the session the started ack names,
  `NoneYet` where the record is readable and holds none, `Unknown`
  otherwise. `Claude::reports_windows()` is `Offered`.

`hook_events()` gains `usage`, which reads its input
(`hook_reads_input`), and `hook()` routes it before the Claude Code
events. It is the module's word, no hook event of Claude Code's, as
`status-line` is.

## 4. The core takes a reading (153, 184)

`crates/flume-core/src/readings.rs`, `Engine::take_reading(identity,
secret)`:

1. The live current start `identity` names, its secret `secret`, as
   `start_record` finds it; otherwise refused, taking nothing.
2. A start under no account takes nothing: the windows are no
   account's.
3. Its program's agent's `windows(folder, identity)`; anything but
   `Reported` takes nothing.
4. The account's document, `HostKey::Account` by the start's pool and
   account, read, and its `reading` set to the report, `agent` the
   start's identity, by compare-and-set at the version read, trying
   again on a moved version up to eight times, as `record_spent` does.
   A reading kept that was taken at the same second or later is kept,
   and nothing is written. The spent keys are carried as they stood.

No settle takes a reading: the hook is the one way in, as clause 184
says, and a report the hook could not record is followed by the next,
the engine reporting each whole point a window moves.

`record_spent` and `spent_now` read and write the account's document the
same way, carrying its reading. A reading never sets, shortens or
lengthens a spent time (146, 185).

## 5. The account's document (153, C.7)

`crates/flume-seams/src/store.rs`:

- `HostDocument::Account(AccountDocument)`, `AccountDocument { spent:
  Option<AccountSpent>, reading: Option<AccountReading> }`, the spent
  keys flattened at its top as an earlier release wrote them, `until`,
  `words`, `recorded_at` and `unread`, and the reading under `reading`.
  A document an earlier release wrote reads as spent with no reading,
  and one with a reading and no spent time reads with none.
- `AccountReading { taken_at, agent, session, windows }`, in
  `crates/flume-seams/src/host.rs` beside `UsageWindow`. The reading and
  its windows are spelled in C.7's form: `taken_at` and each
  `resets_at` RFC 3339's with their offset, UTC written as `Z`; each
  window's `name` as the program gives it and `used` its share, a number
  from 0 to 100 written from its hundredths, 42 or 23.5; `session`
  left out where none. flume-seams takes the workspace's `chrono`, which
  flume-core and flume already take, for those times.
- `Usage`, `Account::usage` and `Host::usage` go.

A document reads as JSON whole, as today: one that cannot be read reads
as none, which is no spent time and no reading. Only flume writes it.

## 6. The room and status read the document (56, 145, 184, 185)

`crates/flume-core/src/accounts.rs`'s `Reading` becomes `None(why)` or
`Taken(AccountReading)`. The room's read of each account's document in
`room.rs` gives both its spent time and its reading, one read. With no
reading kept, why there is none: `its program's profile gives no
windows` where `reports_windows` is not offered, or no profile binds
the program, and `no session flume started has reported its windows`
otherwise. Freshness, what is left and what holds an account past its
threshold read `taken_at` where they read `read_at`, unchanged
otherwise: fresh for fifteen minutes, a window past its reset read as
nothing used, a window used to the threshold holding its account until
its reset, or while the reading is fresh where it names none.

Status's `ReadingStatus::Read` keeps `read_at`, now when flume took the
reading, and gains `agent`, the start's identity, and `session`, left
out where none; `ReadingStatus::Unread`'s `unread` gives why there is
none, for every account with no reading, where before an account naming
no `usage` gave no `reading`. The JSON form only gains fields
(core.14). The person's form gives each window's use and reset, then
`taken at <time> from <agent>`, with `stale, ` before it where it is not
fresh, or `no reading: <why>`.

## 7. The configuration and the Nix module (C.7; core.45)

`crates/flume/src/host/config.rs` reads an account's `name`, `program`,
`env` and `agents` alone: `usage` is refused as every word an account
does not hold is, `[pools.<pool>.accounts[<n>]] usage is no word of an
account`, refusing every launch and start until the file is mended, with
no alias kept (record 0062). The reading flag that gave `usage` as
`<set>` among the configuration's values goes. `host/usage.rs` and the
real host's `usage` go.

`nix/modules/options.nix` declares no `usage` on an account, so a
module that sets it renders it as written and flume refuses it, as any
undeclared key; `nix/tests/every-key.nix` and
`nix/tests/files/every-key.toml` drop it.

## 8. The fakes and the suite (106)

- The fake host holds no reading: `set_usage` and its `usage` go.
- The fake agent scripts the windows its program reports for a start,
  `report_windows(agent, taken_at, windows)`, kept in the folder the
  start recorded, read by `windows` with the session `set_session`
  gave, logged `windows`; `reports_no_windows()` makes its
  `reports_windows` not offered.
- The agent seam gains a scenario: `windows` reads none yet before a
  report, the report after it, the later of two, and nothing from
  another folder; a profile that reports none reads unknown. The fake
  passes it both ways, and Claude Code's profile passes it, its harness
  running `flume hook usage`'s keeping on the start's record.
- The account pools' scenarios read their readings from the account's
  document: a helper writes a reading there through the store, as one
  taken from a session would stand. The fake host's two reading
  scenarios go, and the scenarios of a file that cannot be read and of
  nothing written become an account no session reported for, an
  account of a program whose profile gives no windows, and nothing of
  the host's asked.
- The take's scenarios: a report taken as the account's reading, from
  the agent and its session; an earlier report dropped; a start under
  no account and a wrong secret taking nothing; a spent time and a
  reading each kept by the other's write.
- The clause list moves to blueprints 55b89a4, the revision that adds
  211: 211 in Part B, `SCENARIOS` 123, and S121 to S123, which that
  revision holds too, named in `unbuilt()` as built by records 0060 and
  0061's changes before the list held them, as the list names S92. The
  path not taken, 211 alone at 31f9f1c, which does not hold it.

## 9. The site

`guides/agent-settings.md` says an account's reading is what its agent
program reports in the sessions flume starts, drops `usage` from its
examples and its section on the file, and says what an account with no
reading does; `reference/configuration.md` drops `usage` and names it
among the words refused; `reference/state.md` gives the account's
document's `reading`, drops the file an account's `usage` names, and
gives status's `reading` with `agent`, `session` and why there is none;
`guides/claude-code.mdx` names the module that passes the windows;
`reference/commands.md` gives `flume hook usage`; the roadmap's row
reads the program's reading. The site names Claude Code as a product
flume integrates with, and no host of ours (core.35).

## 10. Outside this change

- A host whose managed layer names `usage` refuses every launch and
  start once this lands, until the file drops it: deneb's Boomdrive set
  names it, and `packages/swancloud-usage` writes the files. Record
  0062's consequences give both to swancloud and Boomdrive.
- C.4's row is on `design/flume-claude-codes-windows-row` at bd79555,
  not yet on blueprints' main.

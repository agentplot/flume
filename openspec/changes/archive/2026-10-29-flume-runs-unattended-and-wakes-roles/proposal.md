# Flume runs unattended and wakes roles

## Why

Once `flume-runs-a-kind-through` lands, a kind runs from its launch to
its landing on its own, but only while something settles it: under the
printed command a caller's command does, and nothing yet keeps the
terminal's watch (119, 122). And no caller learns how a place ended
without polling status, so crew's roles, which stand no session, can
never be woken by the work they launched (43, 132; records 0009, 0013).
This change makes flume run unattended under herdr, and delivers each
end, hold, stall and made thing to whoever it is for, waking a role
where nobody is there to deliver to.

## What Changes

- **flume's herdr plugin**, `plugins/herdr`, a moon project of its own.
  Its watch, on herdr's server, runs `flume settle` on the bound the
  herdr terminal profile states and at each change of an agent's state,
  so an agent that crashed, hung or was killed and a prompt never
  received are read with no caller asking (122, C.5, S28). Its sidebar
  shows every place in flight from `flume status --json`, with keys that
  open a place's pane, tell, stop and retry through flume's commands
  (section 9).
- **An end is delivered once.** To the recipients the launch named,
  which the earlier change stores (7, 8), the settle that reads a
  stage's end (a stage lost, never begun or stuck among them, with what
  flume read), a submission's outcome or a stall delivers it once, as a notice through the terminal into each
  recipient's pane, and records on the place that it was delivered, to
  whom and when; until then it is pending (43; record 0009). `flume wait`
  waits by place, answering a pending end at once and otherwise
  following the host's state under a timeout, and drives no settle
  where the terminal keeps watch. `flume inbox` collects every end
  pending for a recipient and marks each delivered. A finished place is
  tidied as always, and only its document stays while an end of it is
  pending (62).
- **Holds and stalled places.** `flume hold` puts a place's next stage
  or its submission on hold after the launch, with the caller's words.
  Status gives, for a place waiting on its caller, the move it waits on,
  since when, a hold's words and whether it is stalled. A place waiting
  under no hold's words for five minutes, or the launch's bound, is
  stalled, and each recipient is delivered that once (130; record 0008).
- **What a stage made.** A stage that declares what it makes records
  each thing at `made.jsonl` in the place's folder, which `{{made}}`
  renders. flume carries the lines as written into the stage's end,
  status and the notice's answer, and a declared thing with no line is
  the deliverable not yet made (128, 129; record 0007).
- **Role addresses and waking.** A recipient may be a role's address,
  `<partition>/<role>`. A role's run is a place of the kind the host's
  `[roles]` table names, on its `base` with its optional `brief`, named
  by the role, its agent's session named by
  the address. A settle that finds an end pending for an address with
  no run live or waiting for room launches that kind at its priority,
  and `flume wake <address>` does the same for another tool's carrier;
  one run per address (132, C.7, S38; record 0013).

## Capabilities

### New Capabilities

- `end-delivery`: an end, an outcome or a stall delivered once into
  each recipient's pane and recorded; pending until then; the wait by
  place; the inbox; a tidied place's document kept while its ends are
  pending.
- `caller-waits`: a hold put after the launch with its words; the move a
  place waits on and since when; a stalled place, said once to each
  recipient.
- `made-things`: what a stage made, recorded by its run and carried as
  written; `{{made}}`; a declared thing with no line holding the stage
  undelivered.
- `role-waking`: a role's address as a recipient, the host's `[roles]`
  table, the wake at a settle and by `flume wake`, and one run per
  address.
- `herdr-plugin`: the watch on herdr's server and the sidebar with its
  keys, a plugin outside the core that computes nothing of its own.

### Modified Capabilities

- `place-status`: status gains what waits on the caller, what a stage
  made, and the ends pending and delivered for each recipient.
- `seam-contracts`: the terminal delivers a notice to a recipient's
  session by its name, and opens an agent under a session name its
  settings give.
- `seam-fakes`: the fake terminal holds recipients' sessions and records
  each notice sent.
- `clause-coverage`: the suite's list holds every clause, invariant and
  scenario of the statement at 480dbb4, 128 to 139 and S35 to S49 among
  them, none left unbuilt.

## Impact

- `crates/flume-seams`: the held state gains the ends log
  with its deliveries, holds with words and since when, the waiting
  move, the role address; the terminal seam's recipient send.
- `crates/flume-core`: delivery inside the settle, the wait, the inbox,
  holds, the stall, the made lines, the wake; the tidy kept while ends
  are pending.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake terminal's
  recipients; the engine scenarios for every new requirement, S38 among
  them; the clause list at 480dbb4.
- `crates/flume`: `flume wait`, `flume inbox`, `flume hold`,
  `flume wake`, and `--hold <stage>=<words>` on `flume launch` where the
  earlier change does not take it; the herdr terminal
  profile's recipient send, its session name, and its watch answered
  while the plugin's watch is live; the host configuration's `[roles]`
  table.
- `plugins/herdr` (new) with its moon project, its tools pinned in
  `devenv.nix`, and treefmt claiming its files.
- agentplot/weir: weir-kinds gains `makes` on a stage, landed there
  through sluice, before flume reads it (core.4).
- Depends on `flume-runs-a-kind-through` landed (the profiles, the
  command line, the queue, `sluice inbox`, a launch's recipients), and
  on herdr 0.9.3 or later on the host.

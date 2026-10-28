# Design

## Context

The core landed at 4e8bb15: the seven seam traits and their types in
`flume-seams`, the engine in `flume-core` (launch, the settle, stage
runs, caller acts, status, the tidy), the fakes, and the conformance
suite with its clause coverage at the statement's revision 6246a48.
`flume-runs-a-kind-through` lands before this change and builds on that
core: the profiles of Part C (plain git worktree places, weir's kinds,
OpenSpec, Claude Code with flume's Claude Code plugin, herdr's open,
send, state and close and the printed command, sluice through `sluice
submit --no-wait --json`, `sluice inbox` and `sluice step`), a kind
running through with its fix round (131), the host's queue by priority
read from `[host] agents` in the host's configuration (70, C.7), and the
command line under `crates/flume/src/commands/`. This change is written
against that tree. Where it names a module of that change it names it
by the statement's term, the herdr terminal profile, the host
configuration's reader, the launch command, since its file names are
that change's. Three of that change's choices bear on this one: the
host's terminal is bound by `[host] terminal` in the host's
configuration, and the herdr terminal profile answers no watch until
this change's plugin keeps one; flume's Claude Code plugin is
`plugins/claude/`; and the root moon project runs `openspec validate`
over the changes as its `changes` task, with herdr, sluice and OpenSpec
pinned in `devenv.nix` and the tools' own checks in the `check-tools`
layer.

The statement is read at agentplot/blueprints edf0de2, which carries
the words of record 0014 (delivered, pending, recipient, collect,
notice, stalled, needs an answer or found work). The records behind
this change are 0005 (plugins outside the core, no process of flume's),
0007 (what a stage made), 0008 (a hold and a stalled place), 0009 (an
end delivered once, a wait by place), 0012 (a submission read at the
settles that follow it), 0013 (a role's address) and 0014 (the words).
Record 0014 and the rules in its words are on flume's branch
`design/the-words`; the rules this design cites, core.24, core.25 and
core.28, are cited in those words. The one rules page is
`docs/architecture/core.md`; this change adds no area page and touches
no area another page would own.

## Goals / Non-Goals

**Goals:**

- flume runs unattended under herdr: no flume command after a launch is
  needed for a kind to run from launch to landing (119, 122, 131).
- Every end, outcome and stall reaches each recipient once, survives a
  restart of the recipient, and wakes a role with no live run (43, 130,
  132).

**Non-Goals:**

- Anything a caller decides: which address is a place's recipient, what
  a role's kind does, what a woken run decides (69, 70, 72; core.28).
- Bringing up or releasing a made thing that must run: sluice's, pressed
  by whoever shows the card (129; record 0007).
- The kind format's `makes` field: weir's to add (core.4).
- Telling a person anything: a notice goes into a recipient's session,
  and no surface pushes to a person by itself (72).

## Decisions

### 1. The held state's new parts

`HeldState` in `flume-seams` gains, each serialized and round-tripped as
the core's types are:

- `recipients`: the launch's recipients, each a `Recipient`, a session's
  name or a role's address. A name of the form `<partition>/<role>` is
  an address; any other is a session's name. They are fixed by the
  launch (8).
- `role`: the address a role's run serves, set by the wake (decision
  8).
- `ends`: every end the place has had, in order, each with its key, a
  stage's start (the stage and its start's number), a submission (the
  submit's number) or a stall (its since), when it was read, what 43
  names of it (the head or report and the made lines, the agent's words
  with whether it needs an answer or found work, the commit landed as,
  the merge queue's words, or the stalled move), and its deliveries,
  each a recipient, when, and by what: a notice, a wait or the inbox.
  An end is pending for every recipient with no delivery of it (43).
- `waiting`: the move the place waits on its caller for, since when,
  and whether its stall has been read (130; decision 5).
- `holds`: each hold with its target, a stage or the submission, its
  words where it has any, and since when, the launch's among them
  (decision 4).
- `stall_after`: the launch's bound for a stall, five minutes where it
  sets none (130).

A stage's `StageEnd` gains `made`, the made lines read at its end
(decision 6). The `Kept` reasons gain `Pending`, naming each recipient
an over place's ends wait on (decision 3).

Rejected: ends kept in a host inbox apart from the places, outliving
them. That keeps a record of a place after its tidy, against 62 and
core.16, and makes a second store to keep in step with the first.

### 2. Delivery inside the settle

The settle (46) gains three steps after the existing ones read and move
the place, in this order, each safe to repeat and each written by
compare-and-set (74, 75):

1. **Record.** A stage end, a submission outcome the settle reads, a
   landing, a failure or a conflict from the merge queue's inbox or a
   withdrawal from read-submission (99, 133), or a stall read in this
   settle is appended to `ends`, once, by its key.
2. **Deliver.** For each end pending for a recipient, the notice is
   sent through the terminal seam into the recipient's session (decision
   7). Only a send the terminal answers sent is recorded as a delivery;
   a recipient asking, unknown or not found, or a send refused, leaves
   it pending for the next settle. The delivery is claimed in held state
   before the send, as a prompt's sending is (`deliver.rs`), so two
   settles at once send one notice.
3. **Wake.** For each address with an end pending and no run live or
   waiting for room, the address's run is launched (decision 8).

Under the watch these run within its bound; under the printed command a
caller's acting command runs them (119, 120). No step waits: a notice
not sendable now is sent by a later settle.

**A notice is sent into a working session, never into a question.**
Rule core.11 binds prompts and tells, which wait for a waiting agent and
are read back as received. A notice is neither: it is the end and the
place to read (record 0014), and it is sent as sluice pushes an outcome
(sluice record 0068), into the session whatever it is doing, the agent
program queuing it as its next turn; herdr's send refuses a session at
an approval or question before typing anything, and the core reads the
session's state first. So a notice is never sent while the session is
asking or unknown, never left typed (95), and is recorded delivered on
the terminal's answer, with no read-back. Waiting for the session to be
idle was rejected: a busy conductor is rarely idle when the watch runs,
and the end would lag by the conductor's turns.

**The notice's words** are one line: `flume: <kind>--<name> <what
ended>; read: flume status <kind>--<name>`, where what ended is
`<stage> delivered`, `<stage> needs an answer`, `<stage> found work`,
`<stage> finished`, `submission landed <commit>`, `submission failed`,
`submission withdrawn`, or `stalled: <move>`. The words whole, the head,
the made lines and the commit are in status, the wait's answer and the
inbox's (43, 61).

### 3. An over place is kept while its ends are pending

The tidy (66) of a place whose work is over first asks whether any end
stands pending; if so the place is kept with `Kept::Pending` naming the
recipients, and the first settle after the last delivery tidies it.
This keeps 62 whole, since nothing of a place outlives its tidy, and
gives a restarted recipient the landing it missed. A recipient that
never returns keeps the place: status names it, and the operator's
`flume inbox --recipient <name>` releases it. Rejected: tidying at once
and losing the pending end, which 43 forbids, and the host inbox of
decision 1.

### 4. Holds

`flume hold <place> (--stage <stage> | --submission) <words>` puts a
hold after the launch (8, 130). It is taken for a stage the place has
not started since its current stage began, one not reached or waiting
for room (a hold takes it out of the queue), or for a submission not
yet made; it is refused naming why for a stage started, a submission
made, or a place whose work is over. `flume launch --hold <stage>` keeps
its spelling and gains `--hold <stage>=<words>`, so a launch's hold may
carry words too. A retry naming the stage or the submission ends its
hold (47, 49), the only way one ends (core.24).

### 5. The waiting move and the stall

Each settle computes, from the held state alone, the move the place
waits on its caller for, the first that holds of: the submission on
hold and due; the next stage on hold; the current stage ended short
needing an answer; readiness failed with a stage waiting on it; a
non-landing kind's last stage delivered; the place past its kind's
bound of goings-back (112, 130, 131). A move read for the first time
sets `waiting` with since when; a move that changes resets it; none
clears it. A place waiting under no hold's words for `stall_after` is
stalled: the settle records the stall as an end, delivered once to each
recipient (decision 2). A hold with words never stalls; a stage waiting
for room is no move of the caller's and never stalls (70; core.26).

### 6. What a stage made

The kind's stage, as the kinds seam reads it, gains `makes`, the names
the declaration gives, once weir's kind format holds the field (C.2,
core.4). The store seam's folder (128) holds `made.jsonl`, and
`{{made}}` renders its path for a stage that declares what it makes; a
stage asking for it otherwise is refused naming the part (27). At a
stage's start the core records how many lines the file holds; at its
end it reads the lines written since, each parsed as one JSON object
carrying at least `name`, and keeps each whole as the run wrote it, its
fields untouched (core.23). A stage whose deliverable exists is not
delivered while a declared name has no line, and the stop check names
each missing name beside the deliverable (44, 110). The lines reach the
stage's end, status, a wait and the inbox; the folder goes with the
place (62, 123). A line that is not one JSON object is carried as its
text with no name, so a run's mistake is seen and not dropped.

### 7. The terminal reaches a recipient

The terminal seam (B.5) gains a send to, and a state of, a recipient's
session by its name, answering not found where the terminal holds no
such session, and `open` takes the session name its agent carries, a
role run's address. 43 names the recipient's own pane (95), so a
caller's session is an agent the terminal sends to; the operations are
95 and 96 reached by the name a session carries rather than by flume's
identity, and no operation is added to the statement. The herdr
terminal profile finds a session by its agent's name in herdr and sends
by herdr's agent prompt; the printed command answers every send
refused, so every end stays pending there and is read from status, a
wait or the inbox (C.5).

**Naming a recipient.** `flume launch --recipient <name>`, given any
number of times. A caller's session gives its own name: crew's sessions
and role runs carry it, and flume's Claude Code plugin's `/launch`
passes `$FLUME_RECIPIENT` where the session has it set. A command that
takes a recipient, `flume wait` and `flume inbox`, defaults to
`$FLUME_RECIPIENT`, and within a role's run to the run's address, read
from the place `$FLUME_PLACE` names.

### 8. Roles and the wake

The host configuration's reader gains `[roles."<partition>/<role>"]`
with `repository`, `kind` and `priority` (C.7). An address's run is a
launch of that kind in that repository on its shared branch, named by
the role, at that priority, with the address as its `role` and as its
agent's session name, no brief and no recipient (132). The wake,
`Engine::wake(address)`, answers that the run is live or waiting where
a place of that repository, kind and name stands, and otherwise
launches it; a launch refused for its name standing is the other
settle's run, answered as waiting (3, 75). The settle's wake step
(decision 2) and `flume wake <address>` call the same wake. A role's
kind whose stage asks for `{{brief}}` is refused at the wake, and
status names the refusal beside the pending ends. Delivery to an
address goes to the session named by the address, which is the run's
agent while one is live; between the run's stages, or before its first
agent starts, the end stays pending and the run's `flume inbox`
collects it.

### 9. The wait and the inbox

`flume wait <place> (--stage <stage> | --submission) [--recipient
<name>] [--timeout <duration>] [--json]` reads the place's newest end
of that stage's current start or that submission. Pending for the
recipient, it answers it and records it delivered by the wait; already
delivered, it answers it as a read; none, it follows the host's state,
reading the store every two seconds, until one comes or the timeout
passes. Under a terminal that keeps watch it calls nothing but the
store (core.13, core.25); under one that keeps none it settles every
place before each reading (119). Its exit says the outcome, beside the
command line's 0 for an answer and 1 for a refusal: 0 for a stage
delivered or finished or a submission landed, 2 for a stage that ended
short, lost or never begun, or a submission failed or withdrawn, and 3
for still waiting at the timeout.

`flume inbox [--recipient <name>] [--json]` reads every place on the
host, over places kept for their pending ends among them, and answers
each end pending for the recipient with what 43 names, recording each
delivered by the inbox at the same write. Status answers ends and their
deliveries and records nothing (57).

### 10. flume's herdr plugin

`plugins/herdr` follows sluice's herdr plugin in form: POSIX `sh`
scripts with `jq`, a `herdr-plugin.toml` whose `[[startup]]` runs the
watch with `--ensure`, actions and panes for the sidebar's popup, and a
README with the install command, `herdr plugin install
agentplot/flume/plugins/herdr`, and the keys and sidebar rows to add to
herdr's configuration.

- **The watch** holds a lease in the plugin's state directory, renewed
  on each tick, naming its process, the plugin's directory and version,
  and herdr's server by its socket, so a second `--ensure` exits and a
  reinstall or a new server replaces it. On each short tick it reads
  herdr's agent states for flume's panes, a herdr call that starts no
  process of flume's; when one changed, or when the bound passed since
  its last settle, it runs `flume settle`, then `flume status --json`
  into its state directory for the sidebar. The bound is the one the
  herdr terminal profile states (122), read from status's host part, so
  the plugin and the core name one bound. It runs nothing else of
  flume's.
- **The sidebar** is a herdr sidebar row and a popup pane, both drawn
  from the status the watch last wrote: kind, stage, agent state, tasks
  ticked, submission, and waiting on the caller or stalled. They read
  status and compute nothing (core.14).
- **The terminal's watch.** The herdr terminal profile's `watch`
  answers the bound while the plugin's lease is fresh, read from the
  plugin's state directory, and no watch otherwise, so flume runs
  attended (119) on a host where the plugin is not installed or its
  watch is not running, and never waits on a watch that is not kept
  (122, C.5). The profile reads the lease and starts nothing.
- **The keys** are herdr actions: open focuses the place's tab,
  `<kind>--<name>` (C.5); tell, stop and retry each run one flume
  command, asking for the words or the force in a popup first. No key
  sends to an agent's pane (107).

The plugin is moon project `plugin-herdr`, language `bash`, with
check-fast tasks `manifest` (the manifest as herdr reads it, every
command it names present, its version equal to the cargo workspace's so
a tag installs a matching plugin), `lint` (shellcheck over every
script) and `test` (the scripts against stand-in `herdr` and `flume` on
`PATH`, under the system's `sh` and under `dash`), one check each
(core.21). A `check-tools` task, `install`, links the plugin into the
pinned herdr under a scratch configuration and reads herdr's list of
plugins naming it, so the manifest is proved against herdr itself; the
install from flume's repository at a tag is the README's command, the
same manifest fetched. shellcheck and dash are pinned in `devenv.nix` beside jq
(core.22), and treefmt claims `*.sh` with shfmt, where no earlier change
already does (core.21).

### 11. The checks

Each group of the tasks ends with the moon tasks it touches passing,
run as `sluice exec main -- moon run <project>:<task>`: sluice runs the
command inside the environment its `[tests] toolchain` names, the
devenv shell, under the host's bound, so this is the config's `devenv
shell -- moon run` taken in turn with the host's other agents. The
engine's new scenarios live in `flume-conformance/src/engine/`, one
file per capability (`end_delivery.rs`, `caller_waits.rs`,
`made_things.rs`, `role_waking.rs`), each citing its clauses as data
(106); the terminal's recipient scenarios join `seams/terminal.rs`. The
clause list moves to the statement's revision edf0de2 or later, holding
128 to 133 and S35 to S38, wherever the earlier change has not. The
last group runs `sluice exec main`, the gate's checks with the root's
`changes` task among them, and `openspec validate` on this change.

## Risks / Trade-offs

- [weir's kind format holds no `makes` yet, and weir refuses a stage
  field it does not know] → group 4 waits on weir's release with the
  field; the other groups do not depend on it, and the tasks say so.
- [A notice into a working session is queued by the agent program, not
  read back] → the terminal's send is the record, the end stays readable
  by status, the wait and the inbox, and a later delivery of the same
  end never happens (43).
- [A recipient that never returns keeps an over place, worktree and
  all] → status names the recipient it waits on, and `flume inbox
  --recipient <name>` releases it.
- [A role whose launch is refused is tried again at every settle] → the
  refusal is a launch's, refused before anything is made (core.8); it
  costs a read of the configuration and the kinds per settle while ends
  stand pending, and status names it.
- [A session's name is the caller's to keep stable] → a name that
  changes across a restart loses nothing of flume's, since the inbox for
  the old name still answers; crew names its sessions, and a role run's
  name is its address.

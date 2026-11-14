# Design

The statement is read at boomdrivedev/blueprints main 81288ba: clauses
7, 43, 53 to 55, 56, 121, 132, 193, 214 and 215, C.4's deliver row,
section 9's surfaces and S126 and S128, with C.7's deliveries bullet as
the branch `design/flume-a-message-to-a-role-is-a-delivery-at-its-address`
(9ddd402) amends it: a message's delivery key and the claims kept beside
the records. The decision is record 0069, accepted on 2026-11-14. Where
this design and the statement disagree, the statement wins (core.19).

The rules relied on: core.1 and core.2 (the core names no program or
terminal; the deliver part and `send_to` are reached through the agent
and terminal seams, and Claude Code's mod is its profile's); core.4 and
core.25 (every delivery record, hand-over and confirmation is spelled
by weir-delivery's crate, and a hand-over delivers nothing until its
recipient confirms it); core.11 (a prompt, a message and a notice go
only to a recipient that waits and asks no question, never left typed);
core.13 (no process of flume's: the mod's clock runs in the agent
program's own process and ends with its session, each tick one command
that reads and ends); core.14 (one core behind every surface, the JSON
form gaining fields); core.16 and core.17 (the host's deliveries
document written whole by compare-and-set, every act safe to repeat);
core.28 (a role's run, a standing agent and a carried address as their
source's entry says); core.30 (no variable's value in an answer);
core.35 (the site speaks to any host's operator); core.39 (a profile's
part is checked against the release flume pins, or not offered);
core.54 (a recipient every place's ends reach). No area page exists;
core.md binds every path this change touches. This change amends
core.11 and core.25, and the wording of core.27, core.37 and core.54,
to record 0069 (section 13). It departs from no rule.

## 1. Scope, and the change beside it

This change builds a message as clauses 53 to 55 give it but for a
hibernating stage, a role's deliveries as 214 gives them, the deliver
part of 215 and C.4, a launch from a role's run reporting to its
address (7, 214), status's part of 56 for a role's address that does
not turn on hibernation, and `message`, `launch` and `wake` over MCP
(section 9).

The change `a-stage-whose-kind-hibernates-it-sleeps-and-resumes`, cut
from the same bolt, builds 212 and 213: hibernate and resume, a message
to a hibernating place resuming it (53), a delivery resuming a role
whose place hibernates (214), `flume hibernate` and `flume resume` and
their MCP tools, and S124, S125 and S127. Whichever lands second meets
the other in `crates/flume-core/src/acts.rs`: this change routes every
message through `Engine::message`, whose place branch is where a resume
of a hibernating stage joins, before the refusal of a place with no
live agent. A standing role's place, 193's folder of detached
checkouts, is built by neither; S128's role here is a role's run.

## 2. `message` replaces `tell` (53 to 55)

- `Command::Tell` becomes `Command::Message { target, words, within,
  answers }`, with `tell` a hidden alias clap reads as `message`, so a
  caller that runs `flume tell <place> <words>` gets a message's answer.
- `Act::Tell` becomes `Act::Message`, serialized `message` and read from
  `tell`, so held state that recorded an answer by a tell reads it as a
  message's. The answer's `act` is `message`: a field's value, no field
  removed (core.14).
- `Engine::tell` and `tell_answering` become `message` and
  `message_answering`; `TELL_BOUND` becomes `MESSAGE_BOUND`, ten
  minutes. Every refusal, every word `print.rs` prints and every help
  text that said tell says message.
- The herdr plugin's key `t tell` becomes `m message`, running `flume
  message`.

The path not taken: removing `tell` at once, which would break every
caller that tells a place, crew's roles among them, before each moves
(record 0069, "Elsewhere"). The alias goes in a later chore once no
caller uses it.

## 3. A message's target

`flume message <target> <words>` takes a role's address where `target`
is one, `<stream>/<role>` with each part a word as `is_address` reads
it and neither part `.` or `..`; any other target names a place as
every command names one, its location or `<kind>--<name>` in the
repository. A place whose location is a relative `<word>/<word>` is
named by its absolute location or `./` before it. `--within` and
`--answers` are refused with an address, naming why: a bound is how
long a message waits for a place's agent, and an answer goes to the
place that asked (42, 53).

The path not taken: a flag of its own for an address, `--to`, which
`flume needs` gives a role's name; a message names one target, and an
address is the recipient's own name everywhere else (7, 132).

## 4. A message to a role's address (53, 214, C.7)

The act reads the host's roles as a wake does (132, 193). An address a
source declares once, with a run, a standing agent or carried, takes the
message; one no source declares, two declare, or whose stream the host
does not declare, or whose entry is at fault, is refused naming why, as
`flume wake` refuses it, and nothing is recorded.

A message taken is recorded through weir's `record` in the host's
deliveries document:

- **key** `message/<stream>/<role>/<token>`, the token drawn as a
  launch's is, eight hex characters (`deliveries::token`);
- **readers** the address alone;
- **message** `<from> messages <address>:` and the words as given on
  the lines after it, where `<from>` is the recipient the sending
  command acts as, read as a command that names no recipient reads one
  (`recipient_of`): `$FLUME_RECIPIENT`, the address of the role's run
  `$FLUME_PLACE` names, or the terminal's session; `A message to
  <address>:` where none is read;
- **expires** after `[host] ends_expire`, a week where it is unset
  (weir 104).

Then, within the same command, the act wakes the address where its
source gives it a run and none is live or waiting for room, as a
settle's wake does (132), and hands the address's batch where its
session waits (section 5). It answers at once, with the delivery's key,
the address and what the act found: the run woken, live or waiting for
room, a standing agent, carried, or the batch handed. No bound refuses
it: a message at an address is pending until the address's session
takes it and confirms it (214; weir 102).

A message's delivery is let go of, its records with its hand-overs and
confirmations, once its one reader has confirmed it, at the next settle
of the host's watch or the next command that settles every place; no
document of a place holds it.

The path not taken: a message to an address that waits within a bound,
as a message to a place does; a role whose run is woken or waits for
room would refuse every message sent while it starts.

## 5. What a recipient takes at its next wait (43, 214, 215)

A recipient's **batch** is every delivery whose readers name it, owed
to it, handed to it by nothing and claimed for it by nothing, but a
drop's or a done's end, which no notice carries (43), in the order the
host's deliveries document recorded them. Each item is
`notice_to(message, key, recipient)`, a delivery's message followed by
the way that recipient confirms it, an end's and a message's alike;
items are joined by one blank line, with nothing before or after them.

A batch is handed only while the recipient's session reads waiting,
never working, asking or unknown, and:

- where the recipient is a role's run, only once its current start is
  started and its prompt received (record 0058), and only while its
  stage does not wait on an answer, so a delivery is never taken for the
  answer, which comes by a message to the place, naming its key where
  it likes, or by a retry (42, 53);
- where it is a standing agent, in the one agent of the entry's name its
  stream's terminal finds in its session, as today (132).

**Claims.** The host's deliveries document holds, beside its records,
`claims`: each a delivery's key, a recipient, when it was claimed, and
whether its send is still unanswered (C.7 as amended). A settle claims
a whole batch in one write at the version read; reads the document
again and drops the items another way handed to the recipient
meanwhile; sends; and, on the send's answer, in one write, adds each
item's hand-over through weir's `hand`, by `notice` for an end and by
`message` for a message, at the claim's time, and removes the claims.
A send refused or not found removes the claims and records nothing, so
the next settle hands the batch again. A claim left sending by a settle
ended mid-send is recorded by no settle, as a place's notice claim is
today, and holds its items out of every later batch: the inbox, a wait
or the session's start still hands them. A claim whose delivery was
handed to its recipient by another way, or let go of, is removed by the
next write. A place's `noticing` claims written before this change are
read as before, an answered one recorded once, and none is written.

**Who asks.** A settle of one place hands the batch of each recipient
of its ends pending and, where the place is a role's run, of its own
address, so the run's agent going quiet (121) takes what waited for it.
A settle of every place, the host's watch's among them, hands the batch
of every recipient a delivery is pending for, each once in the pass, so
a message to a standing agent or a session that no place's settle
reaches is handed at the watch's next settle. `deliver_ends` becomes
the module `crates/flume-core/src/batch.rs`'s `hand_batches`.

The paths not taken: a notice of each end apart, as today, which types
two prompts into one session where one would do (214); the claims kept
on each place's document, as today, which leaves a message, of no
place, nowhere to be claimed, and a batch across places claimed by as
many writes.

## 6. Who hands a batch: the deliver part, else the terminal

Where the recipient is a role's run whose live agent's profile offers
deliver (section 7), a settle hands nothing: the agent takes its batch
itself at its next wait. Otherwise the settle hands it by the
terminal's `send_to` into the session it reached, the run's address,
the standing agent's or the session's own name. A standing agent or a
session flume did not start is always handed by the terminal: flume's
plugin acts only in a session flume started, and the session-start
inbox is the only way into the rest (C.4).

herdr's and Rex's `send_to` type into a waiting session only, a working
one refused naming that it works, so a session that began a turn
between the core's reading and the send is typed nothing (54, 214;
core.11). Under the printed command nothing is sent, as before.

## 7. The deliver part, and Claude Code's mod (215, C.4)

**The seam.** The agent seam gains `deliver(&self) -> Offered`,
`NotOffered` by default: whether the program takes words as one prompt
the next time it waits by its own channel, leaving its input as a
person left it. A profile that says nothing of it offers none, as the
common profile, not yet built, will not (170 to 174).

**The take.** `Engine::take(place, agent, secret)` is the core's act for
a deliver part: refused unless `agent` names the live agent of the
place's current stage, `secret` is its start's, and its profile offers
deliver, as `flume needs` is refused (41); nothing while that agent's
asking mark stands, its prompt is not received or its stage waits on
an answer; otherwise the oldest message waiting for that start (section
8), alone, and else, where the place is a role's run, its address's
batch, claimed and recorded handed in one write, with no send between.
It answers the words, or nothing.

**Claude Code.** `flume hook deliver` reads `FLUME_AGENT`,
`FLUME_SECRET` and `FLUME_PLACE`, asks the take, prints its words, and
exits 0 whatever it answers. The deliver part of the plugin's one hooks
module, `hooks/flume.ts`, which pins the place's line beside it since the
pinned engine takes one hooks module per plugin, each event's hook once
in it and `$` into no function of another file, does nothing in a
session whose environment does not name `FLUME_PLACE`, `FLUME_AGENT` and
`FLUME_DELIVER`; in one that does,
it runs `flume hook deliver` at `session.start` and at `turn.complete`,
each after its `next`, and every ten seconds while no turn runs, read
from `turn.start` and `turn.complete`, one run at a time, each bounded
at ten seconds; where the run exits 0 and prints words it submits them
by `$.prompt.submit({ text, asUser: true })`, which the engine runs as a
turn once the session is idle and which leaves the prompt box as it
stands. A run that fails submits nothing. The profile's command sets
`FLUME_DELIVER=1` only where the profile offers deliver; the plugin
carries the part either way, its binary named by its absolute path as
the line's is.

**The pinned release.** The deliver row was read on 2.1.322, and flume
pins 2.1.296, which is sluice's too, so every host runs one engine
(record 0069; `devenv.nix`). No check of this tree runs the pinned
Claude Code in an interactive session but `check-live`, by hand, so
nothing here can show that 2.1.296 runs a hooks module's
`$.prompt.submit` there. Claude Code's profile therefore answers
deliver not offered, its constant naming record 0069 and the release it
was read on, and the terminal hands every batch and message (core.39).
The chore that moves the pin, with sluice's, turns it on once its live
walkthrough reads a batch taken through the mod. Until then the module
runs nowhere: no start sets `FLUME_DELIVER`.

The choices: `asUser`, so the agent reads the words bare as the
terminal would type them, where the framed form reads "The flume plugin
sent a message"; the take recording the hand-over before the mod
submits, since the hook cannot learn whether the submit took, so a
batch the mod failed to submit is handed and unconfirmed, given again by
the inbox (43); and the variable gating the module, where the path not
taken, a module that asks at every idle tick in every session, would
start a process every ten seconds in every agent for a part no release
flume pins offers.

## 8. A message to a place's agent (53, 54)

Where the agent's profile offers no deliver, a message is handed as a
tell is today: once the agent said it started, waits and asks nothing,
within the bound, by the terminal's send, read received within 60
seconds or sent once more.

Where it offers deliver, the act writes the message on the current
start, `messages`: its words, when, who sent it and when it was taken,
none yet; then follows the place's held state. Once taken, it reads
received within 60 seconds of the take or puts it back untaken once, by
compare-and-set, for one more take; a second take not received is
refused, the agent named as not receiving the words. Where the bound
passes untaken, the act takes it back by compare-and-set, only while
untaken, and is refused with nothing typed; one taken meanwhile is read
received as above. An answer handed to an agent kept open (42, 198)
goes this same way.

## 9. A role's run's first prompt carries its batch (214)

Each start of a role's run's stage fills its prompt with the batch
pending at its address when the prompt is filled, after the kind's
template and before the way to stop short (28): the start claims those
items for the address under its identity, in the claims of section 5,
and records them handed once its prompt is read received, removing the
claims in the same write. A start read never begun, or ended before its
prompt was received, lets its claims go and records nothing, so the
next start or the next wait takes them. What comes after the prompt
was filled waits for the run's next wait (record 0058). The run's
`flume inbox` still collects for its address (43).

## 10. A launch from a role's run reports to its address (7, 214)

A launch's recipients, where it names none, are read in this order:
`$FLUME_RECIPIENT`; the address of the role's run `$FLUME_PLACE` names,
`recipients_from` `role-run`; the terminal's session; and the answerer
behind the place's stream. So a place a role's run launches, by the
command line or by MCP, whose server runs in the agent's environment,
reports to the role's address, and its ends join the role's other
deliveries (214).

## 11. Status, the inbox and confirm (43, 56, 214)

- `AddressStatus` gains `messages`, each message pending at the address
  with its key, who sent it, when, the first line of its words, each
  hand-over with its way and when, and whether it is expired; and
  `reporting`, each place whose work is not over that names the address
  among its recipients. Status lists an address a message alone is
  pending for, as one an end is pending for. Both are read at the ask
  and kept by nothing (core.15).
- `flume inbox` gives `messages` beside `ends`, each handed to its
  recipient by the inbox and confirmed by `--confirm`; an inbox naming
  a place gives that place's ends alone, as before.
- `flume confirm` takes a key that begins `message/` as a message's,
  refused where no delivery stands under it or its reader is not the
  recipient the command acts as.
- `flume wait` is by place and waits on no message.

The person's form of status gives under each address `messages:` with
each key, sender and first line, and `reporting:` with each place's
`<kind>--<name>`.

## 12. MCP (section 9)

`flume mcp` serves four tools: `status`, as before; `message`, with
`target`, `words`, `within` and `answers`; `launch`, with `kind`,
`name`, `brief`, and the command's other words and switches by their
long names, `base`, `recipient`, `hold`, `hold_submission`, `mark`,
`priority`, `stall_after`, `agent`, `stream` and `intent`, arrays where
the flag repeats; and `wake`, with `address`. Each call is
turned into the command line's arguments and run through the same
parser and act, so every default, check and refusal is the command
line's; its result is the act's JSON, a refusal marked `isError`. A
launch takes no file over MCP: its brief is text. `hibernate` and
`resume` join with the change that builds them.

The path not taken: an MCP tool calling the core with arguments of its
own, which would hold a second reading of every flag beside the
command line's.

## 13. Rules, record and suite

- **core.11** reads: a prompt or a message is sent only to a waiting
  agent that has said it started, its held input cleared first where
  the terminal types it, read back as received within 60 seconds or
  sent once more, never left typed for a hand; an end's notice and a
  message at an address are handed only to a recipient that waits and
  asks no question, every one pending for it in one prompt, by its
  agent profile's deliver part where its program offers one and
  otherwise the terminal, and count handed over on the take or the
  terminal's answer, delivering nothing by themselves.
- **core.25**'s notice goes "into the recipient's own session at its
  next wait, by its agent profile's deliver part or the terminal".
- **core.27, core.37, core.54** say message for tell.
- Each amendment notes "amended 2026-11-14 to record 0069", and the
  site's copy of the rules takes the same lines.
- **Record 0069** gains a section of the construction agent's choices:
  sections 2, 3, 4, 5, 7, 9 and this one's suite, each with the path
  not taken.
- **The suite.** Its scenarios cite clauses its list holds at its
  revision, 31f9f1c: 7, 43, 53 to 55, 56, 95, 121, 132 and 193, and
  their names give 214 and 215 in words. The chore that moves the
  list's revision past 210 adopts 211 to 224 and finds this change's
  scenarios to cite 214 and 215. The path not taken: moving the
  revision here, which lists fourteen clauses and eleven scenarios that
  four changes on this bolt build.

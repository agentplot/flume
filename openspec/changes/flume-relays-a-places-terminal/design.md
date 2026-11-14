# Design

The statement is read at boomdrivedev/blueprints main 41bd0a2, whose
relay text main's head on 2026-11-14, 81288ba, holds unchanged: 107,
221, 222, C.5's herdr relay row and the paragraph on the printed
command, with the clauses they cite (4, 95, 96, 98, 120, 136, 187,
190). The decision behind them is boomdrivedev/boomdrive record 0004,
accepted by the user on 2026-11-07 on their card, and its terminal
relay spec says what the runner did with herdr itself. Where this
design and the statement disagree, the statement wins (core.19).

The rules relied on: core.1 and core.2 (the core names no terminal and
reaches one through the terminal seam alone; herdr's commands are the
profile's); core.11 (a prompt, a tell and a notice clear what is typed
and are read back, which a relay's keys are not, so core.11 says so,
section 9); core.13 (the relay is no process of flume's: it lives inside
its command, ended with it); core.14 (the relay's lines are one form a
program reads, from the one core); core.15 and core.52 (status and the
list read where facts live and name no relay); core.16 and core.17 (the
relay writes nothing and changes nothing, so it is safe to repeat);
core.29 (every herdr call bounded, every command and field checked
against the pinned herdr); core.30 (no variable's value in an answer or
words); core.35 (help and site speak to any host's operator); core.49
(the relay goes to the terminal the agent's start recorded); core.56
(telemetry carries no words, so nothing of a relay). No area page
exists; core.md binds every path this change touches.

## 1. Scope

This change adds relay to the terminal seam, a relay loop to the core,
the command `flume relay`, herdr's relay, and the refusals of the
printed command and Rex. It changes nothing a place does: no stage,
start, end, hold, settle or status reads a relay, and the agent profile
is untouched, since a prompt a person submits through a relay reaches
the agent's hooks as one typed at the terminal's own view does, its
received ack matching no prompt of flume's (89, 107).

## 2. The command and its lines (221)

`flume relay <place>` names its place as status does, its location or
`<kind>--<name>` in the repository (`--repository` as for every
command). It is for a program: it speaks JSON lines on its standard
input and output whatever `--json` says, each line one object.

Out, on standard output, while it runs:

- `{"screen": "<what the terminal shows>"}`: the whole of what the
  terminal shows of the agent as a person's view would, colours and
  attributes as the terminal gives them (herdr's ANSI). The first at
  once, then each time it differs from the last given.
- `{"unread": "<why>"}`: a read the terminal did not answer, in its
  words, given once until a read answers again.
- `{"untyped": "<why>"}`: one line of input not typed: a line that is
  no input of the form below, a key flume does not name, or the
  terminal's refusal of it, in its words.

The last line is the command's answer, as `--json` gives every answer:
`{"relayed": {"place": …, "agent": "<identity>", "ended": "caller" |
"gone"}}`, exit 0, or the refusal's JSON, exit 1, which is then the
only line. A reader stops at a line holding `relayed` or `refused`.

In, on standard input, one object a line, an empty line ignored:

- `{"text": "<text>"}`: typed as it is.
- `{"keys": ["<key>", …]}`: each key by flume's name (section 3), typed
  in order.

The caller's word to end is closing standard input; a standard output
it no longer reads ends the relay the same way. Nothing else ends it but
the agent's terminal going (221).

`flume relay` settles nothing, as `flume open` does not: it is no act
(221), and a relay of a place whose held state still reads its agent
live after the agent went finds it gone at its terminal at once.

## 3. Key names (222)

A person's keys go in by name, so the caller names no multiplexer's
keys (boomdrive record 0004). flume's names: `enter`, `tab`, `esc`,
`backspace`, `delete`, `insert`, `home`, `end`, `pageup`, `pagedown`,
`up`, `down`, `left`, `right`, `space`, `f1` to `f12`, and any one
printable character; each may be led by `ctrl+`, `alt+` and `shift+`,
in that order, as `ctrl+c`, `alt+b`, `shift+tab`. `escape` is read as
`esc`. A name outside these is untyped, naming it and flume's names.
The seam carries a key as `Key`, parsed from and written as its name,
so a profile maps it to its terminal's own and the core reads no
terminal's spelling (core.1).

## 4. The seam's relay (222)

`Terminal::relay(agent)` answers a `Relay` handle, or a refusal in the
terminal's words. Its default refuses with `NO_RELAY`, "the terminal
offers no relay", so a profile offers a relay only by building one. The
handle:

- `every()`: the bound the profile states within which a change of
  what the terminal shows is given again, the least time between two
  reads (222).
- `screen()`: `RelayScreen::Shows { screen }`, what the terminal shows
  of the agent now; `Gone`, the agent's terminal holds it no more; or
  `Unread { reason }`, the terminal's words where it did not answer.
- `typed(&Typed)`: the input given to the agent as a person typed it,
  nothing cleared first and no receipt read, or the terminal's refusal.

`Typed` is `Text(String)` or `Keys(Vec<Key>)`, serialised as the input
lines of section 2, so the command reads a line straight into it. The
handle holds what the profile found at the ask, a pane, so each read is
one call. Dropping it ends nothing in the terminal: a relay makes and
closes nothing there (221).

The handle's shape is this design's: one call that runs the whole relay
inside the profile would put the loop, its ends and its reading of
state into every profile; reads and types one at a time keep them in the
core, once, over the fakes.

## 5. The core's relay (221)

`Engine::relay(place, link)` takes the caller's side as a `RelayLink`,
which gives a line to the caller (false once the caller takes no more)
and answers `next(every)`: the person's next input as it comes, a tick
once `every` has passed since the last tick, or the caller's close. The
command's link reads standard input on a thread of the command's own,
ended with it (core.13); a test's link is a script.

1. The place's held state read once, no lock taken and nothing written.
   The current stage's start that is live is the agent relayed; with
   none, the relay is refused naming the place and that it has no live
   agent, its state in its line's words (4, 56).
2. The terminal its start recorded, through `terminal_of` (190;
   core.49), whose refusal refuses the relay; then that terminal's
   relay of the agent, whose refusal refuses the relay in the
   terminal's words (4).
3. A read at once and at each tick: a screen that differs from the last
   given is given; `Gone` ends the relay `gone`; `Unread` is given once
   as `unread` and the agent's state read at once.
4. The agent's state read through the terminal's state (96) at the
   first tick of each second of the host's clock and after an unread
   screen; stopped ends the relay `gone`.
5. Between ticks, each input typed as it comes; a refusal is given as
   `untyped`, and the relay goes on.
6. The link's close ends the relay `caller`.

The answer is `Answer::Relayed { place, agent, ended }`, which only
gains fields (core.14). The core holds the last screen given, to compare
with, and nothing else of what passed.

A relay of a stage kept open for its answer, held at a question its
program asks before it starts, or for review, reaches its agent as any
other: the person at the keyboard is the person's to be (35, 107).

## 6. herdr's relay (C.5's relay row)

`relay(agent)` finds the agent's pane as every herdr operation does,
by its recorded id and socket, found again by its herdr name where the
id no longer answers; gone refuses the relay saying herdr holds no pane
of the agent, and unknown refuses it in herdr's words. The handle:

- `every()`: 250 milliseconds.
- `screen()`: `pane read <pane> --source visible --format ansi`; on
  `pane_not_found` the pane found again, read there where it moved and
  `Gone` where it is gone; `server_not_running` is `Gone`.
- `typed`: text by `pane send-text <pane> <text>`, keys by `pane
  send-keys <pane> <key>…` in one call, each flume key name as herdr
  0.9.5 names it; `pane_not_found` found again once and typed there,
  since herdr refuses before it writes a byte.

Every relay call runs with `HERDR_SOCKET_PATH` the pane's server and the
caller's `HERDR_*` stripped, as every call of the profile, and is
bounded at one second, as runs' listing is, since herdr answers in
milliseconds and a read that waits holds the person's screen; one past
it is unread, or untyped, naming the bound (98; core.29). Each command
and field is checked against the pinned herdr in the tools layer, every
key name of section 3 among them.

## 7. The printed command and Rex

The printed command refuses relay, since it holds no part of a terminal
a person can be shown (C.5, 222), calling nothing.

C.5 gives Rex no relay row, so flume's Rex profile offers none: its
relay refuses saying flume's Rex profile offers no relay, with no Rex
call made. A relay of Rex is the statement's to add first, as a Rex
relay row (core.19); this change builds none.

## 8. Nothing kept, no act (221)

The relay writes no held state, no host document, no file, no log line
and no telemetry, and records no act: it is no send (95), no tell, no
answer. Status and the list name no relay (core.15, core.52). What
herdr is given to type travels as its arguments, as C.5's row names
the calls, and no words flume gives repeat what was typed: a refusal of
`send-text` names the call by `unvalued`, as every refusal of the
profile does.

## 9. A person's keys, not a send

core.11 binds a prompt, a tell and a notice. A reviewer reading its
details would point at a relay's text typed into a pane and left there,
so core.11's details gain: not ruled out, a relay's text and keys,
typed as the person typed them with nothing cleared and no receipt
read, left as the person left them (107, 221). And the relay gets a rule
of its own, core.57, the next free number: a relay reaches a place's
live agent only through the terminal its start recorded, gives what the
terminal shows and types what the person typed and nothing of flume's,
keeps nothing it carried but the last screen to compare, writes nothing,
counts as no act, and ends at its caller's close or once the agent's
terminal goes (107, 221, 222). Both cite record 0070, proposed, which
quotes the user's words in boomdrive record 0004 and holds the choices
of sections 2 to 7 as the construction agent's, each with the path not
taken. Both are mirrored on the site's rules page.

## 10. The clause list stays

The suite's clause list reads the statement at 31f9f1c, before 221 and
222. As record 0065's construct read it on 2026-11-14 for S131, moving
the list to main's revision brings 211 to 224 and S121 to S131 with it,
each to be cited or named unbuilt by its change, and is a change of its
own. So the relay's scenarios cite the clauses the list holds, 4, 56,
95, 96, 98, 107, 120, 136 and 190, and name 221 and 222 in their words.

## 11. The fakes and the suite

The fake terminal relays by default, every 250 milliseconds: its screen
is the one `set_screen` gives, then what was typed into the agent, so a
relay sees typing as a terminal echoes it; `enter` submits what is typed
as the harness's `submitted` reads it; a closed or killed agent reads
`Gone`. It scripts a refusal of relay, a screen sequence, an unread
read and a refused input, and logs each relay call, never what was
typed.

The terminal seam's scenarios, generic over a harness that says whether
it relays: where it does, a screen given, text typed onto what was left
typed with nothing cleared and submitted by `enter`, typed text shown on
the screen within a bound, and a closed agent read `Gone`; where it does
not, the relay refused with words. The engine's relay scenarios run the
core's loop over the fakes with a scripted link.

## 12. The construction agent's choices

Each is record 0070's, with the path not taken:

- **JSON lines both ways** (section 2), not raw bytes out and in: a
  caller tells a screen from an end and a refusal, and the form gains
  fields as every answer does. Not taken: `--json` choosing, since no
  person reads a relay's frames.
- **flume's key names** (section 3), not the terminal's: the caller
  stays free of any multiplexer. Not taken: passing names through,
  which ties the runner to herdr's spelling.
- **A handle the core drives** (section 4), not a loop in each profile.
- **State read once a second** (section 5), not at each read: a read
  every 250 milliseconds with a state read beside it would double
  herdr's calls for an end a second later.
- **One second for herdr's relay calls** (section 6), not the profile's
  ten: a read that waits ten seconds holds the person's screen.
- **Rex refuses** (section 7), not a relay built from `capture`, which
  the statement does not name.
- **No settle** (section 2): the relay is no act.

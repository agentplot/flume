# Design

The statement is read at boomdrivedev/blueprints main 786d040: C.5's Rex
rows, the paragraphs beneath them and C.6's stripped variables, with the
clauses they cite (7, 32, 34, 35, 43, 94 to 98, 107, 132, 136 to 139,
143, 187, 190, 199, 201, S91). The research behind them is
`design/research/2026-11-11-rex-as-flumes-second-terminal.md`, read with
flume at e2973d57; this tree is that main and more, and the places it
names stand where it says. Where this design and C.5 disagree, C.5
wins (core.19).

The rules relied on: core.1, core.2 and core.3 (the profile names its
tool alone, is reached through the terminal seam and is chosen by name
alone); core.6 (a submission names no session); core.9 (no stage's end
from Rex's states); core.10 and core.11 (one agent per identity, a
prompt never left typed); core.12 (only the folder-trust question
answered); core.13 (no process of flume's between commands; a session's
server is the terminal's); core.18 (nothing closed that flume did not
make); core.22 (Rex at the version the tree pins); core.29 (every call
bounded and checked against the pinned release, which the record of
section 13 extends to Rex); core.30 (no variable's value in an argument,
a record or words); core.39 (a part the agent profile builds is never
read from the terminal); core.49 (an agent stays in the terminal its
start recorded); core.51 (a test finds room for its sockets). No area
page exists; core.md binds every path this change touches.

## 1. Scope

This change builds the Rex profile's server, open, send, state, screen,
close, tidy, kept, runs, standing, session and caller rows, and the
sending to and reading of a session by its name. It leaves to the bolt's
next unit, `a-person-sees-a-rex-place-in-the-app` (plan.rec, After this
one), C.5's attach for the app, its show row and the paragraph on the
app's Remote Hosts: here attach refuses both views and show shows
nothing, so a stream on Rex runs its places and a person is not yet
shown them. Until that unit lands, `flume open` of a Rex stream's session
is refused in the profile's words (186, 187), and nothing else of a
stream on Rex waits on it.

## 2. The profile and its registration (105, 190; core.1 to core.3)

The profile is `crates/flume/src/profiles/terminal/rex/mod.rs`, named
`rex`, appended to the terminal profiles' module list in
`crates/flume/src/profiles.rs`, listed third by `terminal_profiles()`
(`printed`, `herdr`, `rex`) and built by `terminal()` over
`<state>/terminal/rex`, the calling command's block read from its
environment at the build. Building it runs nothing: every declared
stream's terminal is built and asked `session()`, `session_state()` and
`kept()` at each command, so a host with a Rex stream and no Rex server
running starts none for a command that opens nothing.

The core is not touched. It takes the first terminal whose
`session_state` answers (`crates/flume-core/src/terminals.rs`), so the
Rex profile answers none for every name not its own (section 9).

## 3. Calls (98; core.29)

Every call is `rex --autostart=false -S unix://<socket> <words>`, the
`rex` first on the profile's `PATH` (a test names its own), with the
calling process's `REX_*` taken out of its environment, so a command run
in a Rex block reaches only the server named and no call starts a server
of Rex's choosing. An answer is the JSON on standard output; a refusal is
a nonzero exit, Rex's words those on standard error. Each call runs under
a bound, ten seconds where the call states none; one not answered within
it is killed and read as unknown, or as a refusal naming the bound, in
the words of the row that made it.

The `REX_*` of the caller are stripped: a choice of this design, since C.5
says the socket is passed and says nothing of the caller's variables.
Without it a call whose words the pinned Rex reads from `REX_SESSION` or
`REX_BLOCK` where none is given would act on the caller's block. The path
not taken is passing them on, as a herdr call passing `HERDR_*` would.

## 4. Servers (107, 143, C.5's server row)

Each of the host's terminal sessions is a Rex server of flume's own at
`<state>/terminal/rex/<session>.sock`, its state folder
`<state>/terminal/rex/<session>.state` beside it, flume's own session
named `flume`. The server row runs as C.5 says: `api call server.status`
at the socket; where Rex answers `no Rex server is running at <socket>`,
`server start --state-dir <folder> --restore=false --json`, which answers
the running server where another start came first; an `api_revision`
below 12 refuses in Rex's words. The profile never stops a server and
never names the socket the Rex app runs for its user (C.5; core.13's
"a session's server the terminal's profile starts headless").

A session's name is refused, naming it, where it makes the socket's path
longer than 104 bytes (C.5), and, as this design's choice, where it is
empty, `.` or `..`, or holds `/` or a NUL: the name becomes a file's
name, and such a name would put the socket outside
`<state>/terminal/rex` or nowhere. The path not taken is leaving it to
Rex's bind to fail, which would leave `..` reaching a folder of the
host's.

## 5. Records (98, 137, 138; core.16, core.17)

Under `<state>/terminal/rex`:

- `agents/<digest>.json`, one per identity, the digest the identity's
  SHA-256 as herdr's records name it: the block's id, its Rex session's
  id, its server's socket, the start's folder and the session name a
  role run carries. open claims the identity by creating it only where
  none stands (`create_new`); one whose agent does not read stopped
  refuses the open, and one whose agent reads stopped is replaced (98).
  Rex refuses no label twice, so the labels alone would let two opens
  at once both pass.
- `bases/<digest>.json`, one per server, repository and base: the Rex
  session's id that open made.
- `windows/<digest>.json`, one per server and place: the window's id
  open made, and the base it is in.

A record is written before anything is sent to what it names, and none
holds a variable's value (core.30).

## 6. open (94, 98, 107, 199, C.5's open row)

open refuses where the identity's record claims a live agent, or where a
live agent carries the session name it is given (98, 132). It refuses a
command whose kind is not one whose program reports Program Status to
the pinned Rex, naming the kinds that do (32): `claude` alone, Claude
Code 2.1.322 reporting natively as `app=claude-code`. Codex, OpenCode and
the rest were not read (the research's "Not read yet"), and flume binds
no agent profile but Claude Code's, so no program runs under Rex whose
received ack its terminal would have to read (171).

Where the block opens:

- Beside a named block, `rex:<block>@<socket>`, a split beside it in
  its Rex session, found by `session.list` and `session.view` on that
  server; left where its caller put it, and no window recorded. A name
  in another terminal's form refuses the open naming it.
- Otherwise in the session the core names, or `flume` for the
  terminal's own (107, 143, 199): the base's Rex session the records hold
  for the server where `session.list` still lists it, else
  `session.create` labelled `<repository> <base>` with one window
  `<kind>--<name>` holding the block; in it the place's recorded window,
  split by `session.new_split` beside one of its blocks, else
  `session.new_window` labelled `<kind>--<name>`, unfocused.

The block is the flavor `com.superlogical.terminal.shell`, labelled the
agent's Rex name: `f-<place>-<stage>-<start>` lowercased, each character
outside `[a-z0-9_-]` made `-`, cut to 119 characters, then `-` and the
first eight hex digits of the identity's SHA-256. Its options are
`shell` `none`, `cwd` the place, `exit.on_completion` false, and
`command` the start script.

The start's folder is `<folder>/rex/<digest>`: `bin/<kind>` links to the
program the settings name, and `start` is the script, mode 0700, which
removes itself and execs `/usr/bin/env -i` with Rex's `TERM`,
`COLORTERM`, `TERM_PROGRAM`, `TERM_PROGRAM_VERSION`, `REX_SERVER`,
`REX_SESSION` and `REX_BLOCK` taken from its own environment, each
variable of the host user's login environment read once (29) with the
agent's environment over it and flume's own, `PATH` led by `bin`, then
`bin/<kind>` and the command's arguments, each quoted whole. The values
are in the script's text alone, never in the block's options, so none
stands in the arguments of the block's `/usr/bin/login`, which every
user of the host can list. The command's files (196) are written by the
core before open, as under herdr.

Then, until 120 seconds have passed, every 250 milliseconds, `block call
process` and `program_status`: a record whose owner is the block's child
is Rex's ready, a second signal and never the start (35); the
folder-trust question on the screen is answered as section 7 says and
the block read again; `last_exit` set, or the bound passed with no
record, is the start never begun with the screen (`Opened::NotComeUp`).
Rex holds no start at a question it can read, so open answers no
`Held`: a program that asks anything else first reports nothing and is
never begun with the screen (35).

Once ready, the host's process table is read for the block's program:
the process `process` names as the block's child, or, where that is
`/usr/bin/login`, its child, must be the program the settings name, by
its path or, for a program a script starts, the script's path among its
arguments (32). Anything else closes the block and refuses the open
naming what ran. Every failure of open after the block was made closes
it, removes the start's folder and the identity's record, and leaves
the window and the Rex session to tidy.

## 7. The folder-trust question (34; core.12)

The screen is read for the command's folder-trust question as herdr's
profile reads it: the focus moved to the yes choice with the command's
`next` key, confirmed only there with its `confirm` key, by `send-key`,
which reads `down` and `enter` by herdr's names; read again until the
question is gone, within 30 seconds. `answer_trust` does the same at a
settle for a start whose agent has not said it started. Nothing else on
a screen is answered.

## 8. send, state, screen and close (89, 95 to 97, 136)

- **send** reads the state row; where it reads waiting, `send-key
  ctrl+c` clears held input. Then `send -- <text>` in pieces of at most
  60,000 bytes cut between characters, since Rex refuses a request over
  64 KiB, then `send-key enter`. A working agent is not interrupted.
  Received is read through the agent profile (89); the events listening
  of C.5's send row applies only where the agent's profile reads no
  received ack of its program's, which no program the profile runs is
  (section 6). `sends()` is true.
- **state**: `process` answering `block target "<block>" not found`, or
  no server at the socket, reads stopped, and `last_exit` set reads
  stopped; otherwise `program_status`'s newest record by `updated_at`
  whose `owner.pid` is the block's child's: `working` working, `idle`
  and `done` waiting, `blocked` asking (which the core takes as asking
  only with the agent profile's mark, 90), `error` waiting, none
  unknown. A call not answered within its bound reads unknown (98).
  Nothing of it ends a stage (core.9).
- **screen**: `capture --trim` and `block call size`, its last `rows`
  lines.
- **close**: `block close`, a block not found read as already closed;
  then `process`, gone only on `block target "<block>" not found` or no
  server at the socket; anything else refuses (97, 98).

## 9. Sessions by name (7, 43, 132)

A recipient's session and the calling command's are named
`rex:<block>@<socket>`. `session()` reads `REX_BLOCK` and `REX_SERVER`
of the calling command's environment, both set, and none otherwise.
`send_to` reaches such a block on its server: a block that waits has
held input cleared first, one working is typed into without
interruption, its program queuing the text, one asking or unread is
refused with nothing typed, and one gone is not found. `session_state`
reads it as state does, none where gone.

A role run's address, given to open as its session, is recorded with
its identity and reached in the block that identity's live start holds.
A standing agent is found at each ask by `agents_in`: `server.status` at
the session's socket, a server not running holding none and nothing
started; then `session.list` and `session.view` of each of its Rex
sessions, each block by its `label` at `rex:<block_id>@<socket>`, its
state as state reads it; two of one label both given, for the core to
count. A listing refused or not answered within its bound is not read,
in Rex's words.

Every other name, a herdr session's among them, is none of Rex's:
`send_to` answers not found and `session_state` none, so the core asks
the next terminal (190).

## 10. tidy and kept (137, 138; core.18)

tidy reads the place's recorded window in each server it made one in:
a block in it that flume did not open keeps it, named; else
`session.close_window`, a window already gone read as closed, its
record removed. Then each recorded Rex session of the place's
repository and base: where `session.view` holds no window but those of
live places the profile made and no block it did not open,
`session.destroy`; otherwise kept, and what holds it named. `kept()`
names each window and Rex session kept, with what holds it, from the
records and one `session.view` of each. Nothing flume did not make is
closed.

## 11. runs (200, 201)

runs calls `server.status` at each part's socket, every part at once,
each call bounded at one second: a server that answers runs, under the
name the part has (`flume` for the terminal's own); no server at its
socket does not; anything else is not read, in Rex's words. A name
section 4 refuses is not read, naming why. Nothing is started.

## 12. attach, show and setup (139, 187, 122)

attach refuses a window, since the pinned Rex has no terminal client and
only its app shows a session, and refuses the app's view in the
profile's words until the next unit builds it (section 1). show answers
that it shows none and `shown_at` gives nothing. setup keeps the seam's
default: flume carries no plugin for Rex.

## 13. What herdr's and Rex's profiles read alike (core.2)

Neither profile calls the other. What both read alike moves out of
herdr's module into `crates/flume/src/profiles/terminal/common.rs`,
which is no profile and names no tool: the folder-trust reading of a
screen (whether it asks, whether the yes choice has the focus), the
program check over the host's process table, the base's label, the
identity's digest and the JSON records' read and write. The path not
taken is a second copy in the Rex module, which two profiles would let
drift apart.

## 14. Rex pinned (core.22)

Rex enters the devenv shell from Rex's flake, an input of `devenv.yaml`
locked at an exact revision in `devenv.lock`, on the platforms the flake
packages, aarch64-darwin as read on 2026-11-11, so the shell elsewhere
evaluates without it. The release it pins is 0.1.0-preview.14 or later
at API revision 12, and each command, field and refusal the profile
reads is checked against it as built; a reading the pinned release
answers otherwise than C.5's table says is a finding for the statement,
by `flume needs --found`, never a reading changed here (core.19). The
tools tests that need Rex fail naming it where the shell holds none,
and never pass without it (core.51).

## 15. sluice strips `REX_*` (C.6; core.6)

`a_session` in `crates/flume/src/profiles/merge_queue/sluice/mod.rs`
takes `REX_*` beside `HERDR_*`, `ZELLIJ*`, `TMUX` and `TMUX_PANE`, so a
submission from a process in a Rex block names no session. The tests'
own lists of variables they strip take `REX_` beside `HERDR_`, so a test
run inside a Rex block reads as one run outside it.

## 16. The stand-in reports to Rex

The stand-in for Claude Code (`crates/flume/tests/profiles/stand-in/claude.rs`)
reports its state to herdr in a herdr pane. In a Rex block, `REX_BLOCK`
set, it writes OSC 7501 reports to its terminal instead, `app`
`claude-code`, in the states the pinned Rex names: `idle` once up,
`working` from a prompt's first moment, `done` at its turn's end, as
Claude Code 2.1.322 was read to report. So the walkthroughs run under
Rex with the stand-in as under herdr.

## 17. Tests

- Unit tests in the Rex module, needing no Rex: the Rex name, a session
  name and its socket's path, the `rex:<block>@<socket>` form and the
  names that are not it, the caller's session, the start script's text,
  the state read from Rex's answers, and the pieces a text is sent in.
- The profile against the pinned Rex in
  `crates/flume/tests/profiles/tools/terminal_rex.rs`, in the
  check-tools layer: each test builds the profile over a state folder of
  its own, short enough that every socket fits under 104 bytes, and
  stops every server it started at its end, so no session of the host's
  is touched. The seam's scenarios run there through a harness whose
  stubborn agent is held from ending by stopping its server's process,
  which views none, holds no start, and moves a block only where the
  pinned Rex offers it.
- A walkthrough through the built binary on a host whose stream `trial`
  names Rex, with the stand-in, a fix landing and its window and Rex
  session closed at its tidy.
- A launch from a Rex block, through the binary, naming the block as
  its recipient, needing no Rex.

## 18. An uncarried multiplexer is `cmux` (190, S91)

The statement moved S91's uncarried example to `cmux`. Every scenario
and test that held `rex` uncarried names `cmux`: the resolver's and the
binary's (where `rex` is now carried), and those over the fake resolver
(which carries no `rex` and would keep passing), so no reader of a spec
finds Rex both carried and refused.

## 19. The record and the rules

A record, proposed: Rex is flume's second terminal profile, its calls
bounded, stripped of the caller's `REX_*` and checked against the Rex
the tree pins as herdr's are, its session names refused where they
would leave `<state>/terminal/rex`, and the next unit's attach and show
named. core.29 is amended to bind every call to Rex as to herdr, and
core.1's details name `rex` among the strings no module outside a
profile holds. The record takes the next free number in `docs/adr/`.

## Risks

- Rex is in tester beta, and its preview channel moves weekly: a
  release the flake serves later may read otherwise. The pin holds the
  release the checks ran against, and section 14 sends a difference to
  the statement.
- Not read yet, from the research: the folder-trust question under
  Rex, a `blocked` record with `kind=permission`, which app macOS holds
  responsible for a server flume starts, and an app of one release
  showing a server of another. The first is answered from the screen as
  under herdr; the others do not bear on this change's rows.
- The tools layer runs where the flake packages Rex. A host that runs
  check-tools on another platform fails the Rex tests naming Rex, as
  section 14 says.

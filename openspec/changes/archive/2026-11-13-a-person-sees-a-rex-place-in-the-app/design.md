# Design

The statement is read at boomdrivedev/blueprints main 6557c33: C.5's
Rex rows for attach and show, the paragraphs beneath the Rex terminal
row on the app's Remote Hosts and on the labels, and the readings of Rex
0.1.0-preview.14 with the app's Remote Control on, with the clauses they
cite (107, 139, 186, 187, 190). The research behind them is
`design/research/2026-11-11-rex-as-flumes-second-terminal.md`, "Showing
a session to a person" and "The prototype". Where this design and C.5
disagree, C.5 wins (core.19).

The rules relied on: core.2 (the profile is reached through the
terminal seam, and the core is not touched); core.13 (no process of
flume's between commands; the app told by attach to show a part is the
person's); core.14 (the settle alone shows the clues, from the one
core); core.18 (nothing labelled or closed that flume did not make);
core.29 (every Rex call bounded, names its server and is checked against
the pinned Rex); core.30 (no variable's value in an answer or words);
core.35 (the site speaks to any host's operator); core.48 (a session
shown in the app through its attach alone, flume keeping nothing of what
the app shows); core.49 (attach and show go to the terminal the stream
or the start recorded). No area page exists; core.md binds every path
this change touches.

## 1. Scope

This change builds the Rex profile's attach for the app, its show and
its shown_at, in `crates/flume/src/profiles/terminal/rex/mod.rs`. The
core, the seams and every other profile are untouched: `flume open`
already asks a stream's multiplexer's attach for its app where the
stream's view is `app` (186, 187), and each settle already calls show
where a place's clues changed since it last showed them (139). attach
for a window stays refused, since the pinned Rex has no terminal
client, so a Rex stream names `view = "app"` for `flume open` to show
it.

## 2. attach for the app (187, C.5's attach row)

attach for the app of the part `session`, the terminal's own `flume`
where none is named:

1. The part's server found or started as the server row says, by the
   profile's `server` (`server.status` at its socket, and `server start
   --state-dir <folder> --restore=false --json` where no server
   answers), so a name the profile refuses, or a server below API
   revision 12, refuses the attach in the server row's words.
2. `api call client.list` on that server (`clients[]`, `client_id`,
   `info.kind`, `connection_state`). The app is a client whose
   `info.kind` is `app`, listed once a person has added the server's
   socket under the app's Remote Hosts (section 3).
3. `api call session.list` on that server (`sessions[]`,
   `session_id`), and the newest of the Rex sessions the profile's
   records hold for that server that it still lists (section 4).
4. `-C <client_id> do session.select session_id=<id>` for that
   session. An answer, `null` with exit 0, is `Attached::Shown`: no
   call reads back what the app shows, so a select that returns is as
   far as the profile can see (C.5's readings).

Each call is `rex --autostart=false -S unix://<socket> <words>` under
the profile's bound, ten seconds where the call states none (core.29).
The refusals, each naming what was refused, the part and its socket:

- **The app is no client of the server**: the words name the socket and
  say to add it under the Rex app's Remote Hosts, since only a person
  adds a server the app shows (C.5).
- **The app refuses**: Rex's words from `do`'s standard error, without
  its `Error: ` lead as record 0068 gives Rex's words, as the app's
  "not accepting actions" while its Remote Control setting is off.
- **A call past its bound**: refused naming the bound, as every Rex call
  is (core.29).

attach records nothing and reads nothing back: what the app shows is
the person's, and a second asking tells the app again (core.48,
core.17).

## 3. Which client is the app

`client.list` lists clients in the order they connected. The app is the
last listed whose `info.kind` is `app` and whose `connection_state` is
not `lost`, `connected` or none being read as connected, as the pinned
Rex's `api describe client.list` says to treat a missing or unknown
state. Where every app listed is `lost`, the refusal says the app's
connection to the server is lost, naming the socket. A choice of this
design, since C.5 names the client by `info.kind` alone: a lost client
cannot take an action, and the newest connection is the app a person is
looking at. Not taken: the first app listed, which may be a connection
the app dropped and has not yet taken up again.

## 4. Which Rex session the app is shown

C.5 shows "the newest Rex session the profile made there". The profile
made a Rex session for each repository and base whose first place opened
in that server, and records each in `bases/<digest>.json` by its id and
server (the change a-stream-runs-its-agents-in-rex, design section 5).
The newest is the one whose record was written last: a base's record is
written once, when open makes its Rex session, and nothing rewrites it,
show included (section 6), so its file's time is its making.

Of those records, attach takes the newest whose session `session.list`
still lists, so a Rex session gone with a server started again (which
restores nothing) is passed over for one that stands. A choice of this
design: the select of a session the server no longer holds would be
refused in Rex's words that say nothing of why. Not taken: selecting the
newest record whatever the server holds.

Where no Rex session the profile made stands in the server, attach
refuses, saying the server holds no Rex session of flume's yet and that
one is shown once a place opens there, naming the socket. A choice of
this design, since C.5 presumes a session to select: answering
`Attached::Shown` with nothing selected would say the app was told to
show what it was told nothing of. Not taken: that answer.

## 5. attach for a window

Refused before any call, naming that Rex 0.1.0-preview.14 has no
terminal client and only its app shows a session (C.5).

## 6. show (139, C.5's show row)

At each settle that changes one of a place's clues the core calls show
with them (139). The profile labels:

- **The place's window**, each the records hold for the place on any
  server and that no tidy kept for a block flume did not open, by `api
  call -s <session> session.set_window_label` with its `window_id` and
  its `label` `<kind>--<name> · <line>`, `<kind>--<name>` the place's
  folder's name as open labelled the window, and `<line>` the clues'
  short line, `code 1/2 · run 1 · tasks 2/5 · working`. Clues written
  before with no line give `<kind>--<name>` alone.
- **The base's Rex session**, each the records hold for the place's
  repository and base on any server, by `api call -s <session>
  session.set_label` with its `label` `<repository> <base> · <count>`,
  the count as the seam gives it, `1 running · 0 asking · 0 waiting on
  caller · 0 stalled`.

Each is found by its recorded id, so a label is display only, and a
block keeps its agent's Rex name as its label (C.5). Each label is cut
to at most 128 characters, between characters, keeping its head, the
most a label holds in the pinned Rex (`api describe
session.set_label`). show writes no record.

The answer: `Shown::Shown` where any label was set; `Shown::ShowsNone`
where the records hold neither a window of the place nor a Rex session
of its base on any server; `Shown::Failed` naming each window and Rex
session not labelled, with Rex's words or the bound, where any was not.
The core logs a failure, fails nothing and shows the clues again at the
next settle (139).

A window or a Rex session Rex answers not found, or whose server does
not run, is passed over, neither labelled nor failed. Rex closes a
window with its last block, so a place between stages, its agent's
block closed and its next not yet open, has no window to label. A choice
of this design. Not taken: a failure logged at each settle while the
place waits for room.

## 7. shown_at (139)

From the records alone, asking nothing of Rex:

- `place`: the place's window on the live agent's server, else the
  newest of its windows, as `rex:<window_id>@<socket>`.
- `agent`: the live agent's block, `rex:<block>@<socket>`, the name a
  recipient's session has.
- `base`: the base's Rex session on that window's server, else the
  newest, as `rex:<session_id>@<socket>`.
- `bases`: each of the base's Rex sessions, on every server, the same
  way.

Each is none where the records hold none. A window kept by a tidy for a
block flume did not open is not the place's. A choice of this design,
the form herdr's shown_at gives its tabs and workspaces,
`herdr:<id>@<socket>`: each id is Rex's own and found again whatever
label a person or Rex gives it. Not taken: the labels, which do not
find a window again.

## 8. Tests

- **Without Rex**, in the module's tests: the labels' text and their cut
  at 128 characters; show and shown_at over hand-written records, and
  with no records calling nothing; attach for a window calling nothing;
  and attach for the app over a stand-in `rex`, a script of the test's
  that answers `server.status`, `client.list`, `session.list` and `do`
  as the test sets and logs each call's words, as the module's tests
  already use one. The stand-in gives the choices of sections 3 and 4
  and every refusal of section 2, which no gate's host can make the app
  give.
- **Against the pinned Rex**, in `terminal_rex.rs`: show's labels read
  back by `session.view` and `session.list`, a window Rex closed passed
  over, and a server that does not answer failing show naming it within
  the bound; attach for the app on a server no app is a client of,
  refused naming the socket, the server started, and the fields the
  profile reads of `client.list` read there from the command line's own
  clients. The terminal seam's scenarios run with the harness's
  `shows_clues` true, so the clues scenario reads `Shown`; its views
  stay none, since no app runs on a gate's host and attach for the app
  refuses there.
- **Through the binary**, in `walkthroughs.rs`: the fix under Rex reads
  its window's and its Rex session's labels while its agent works; and
  `flume open trial` on a host whose `trial` names Rex and `view =
  "app"` is refused naming trial's socket, trial's server running after.

The app's own answer to `session.select` is read by no test: the Rex app
is a program a person runs, not a tool the tree pins, and the research
read it on 2026-11-12 with Remote Control on. A release whose `do` or
`client.list` reads otherwise than C.5's table says is a finding for the
statement, by `flume needs --found`, never a reading changed here
(core.19).

## 9. The record

Record 0068, proposed, says the Rex profile's attach for the app and
show are the bolt's next unit. Its decision item for them is rewritten
to say what they do, citing C.5's attach and show rows, and the choices
of sections 3, 4, 6 and 7 join the construction agent's choices, each
with the path not taken. It stays proposed until the operator's yes. No
rule changes: core.48 already holds a session shown in the app through
its attach alone, and core.29 already binds every Rex call.

## 10. The site

The guide's section "Places under Rex" says `flume open <session>` of a
Rex stream whose `view` is `app` brings the newest Rex session flume made
in that session's server to the front of the Rex app, once a person adds
the server's socket under the app's Remote Hosts and turns on its Remote
Control, and that a place's window and its base's Rex session carry the
place's line and the base's count. The roadmap's terminals row says
the Rex app shows a stream's session, and the Rex app leaves its
decided-not-built list. Rex is named only as a product flume integrates
with (core.35).

## Risks

- Rex is in tester beta: a later release may take `session.select` or
  report clients otherwise. The pin holds the release the checks ran
  against, and section 8 sends a difference to the statement.
- The app's own answer is outside the gate (section 8): a regression in
  the app shows only to a person, as `flume open` answering shown while
  the app shows nothing new.
- Not read yet, from the research: an app of one Rex release showing a
  server of another. A refusal from the app in that case reaches the
  person in Rex's words.

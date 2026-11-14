# A person sees a Rex place in the app

## Why

A stream naming Rex runs its places in Rex servers of flume's own
(the change a-stream-runs-its-agents-in-rex), but a person is not yet
shown them: `flume open` of a Rex stream's session is refused, and
flume labels nothing in Rex but each block's agent. The statement now
says how a person is shown one (boomdrivedev/blueprints
`design/flume/requirements.md`, C.5's Rex rows for attach and show and
the paragraph on the app's Remote Hosts, on main since 6557c33). The
research read it with the Rex app's Remote Control on: the app showed a
second server once a person added its socket under Remote Hosts, and
`-C <client> do session.select` on that server brought its session to
the front, answering `null`, with no call that reads back what the app
shows; the app showed no host `rex hosts add` wrote, so only a person
adds one (`design/research/2026-11-11-rex-as-flumes-second-terminal.md`,
"Showing a session to a person"). This unit was planned after
a-stream-runs-its-agents-in-rex on the bolt rex-is-flumes-second-terminal
(afterthought/crew-state `plan.rec` at agentplot/main cb7c26a1).

## What Changes

- **attach for the app (187, C.5's attach row).** For a session of a
  Rex stream whose view is `app`, the profile finds or starts the
  session's server as the server row says, reads `client.list` on it,
  and tells the app, a client of `info.kind` `app`, to show the newest
  Rex session the profile made there, by `-C <client> do
  session.select session_id=<id>`. Where the app is no client of the
  server it refuses, naming the socket a person adds under the app's
  Remote Hosts; where the app refuses, as it does until a person turns
  on its Remote Control, it refuses in Rex's words. attach for a window
  stays refused: the pinned Rex has no terminal client.
- **show (139, C.5's show row and the paragraph beneath the table).**
  At each settle that changes one of a place's clues, the profile
  labels the place's window `<kind>--<name> · <line>`, the short form
  of the place's line, and its base's Rex session `<repository> <base>
  · <count>`, by `session.set_window_label` and `session.set_label` on
  the recorded ids, each label at most 128 characters; a label that
  fails is logged and fails nothing. A block keeps its agent's name.
- **shown_at (139).** Status gives where Rex shows a place's clues: its
  window, its live agent's block and its base's Rex sessions, from the
  records alone.
- **The terminal seam's scenarios** run against the profile with its
  clues shown.
- **The record** of the Rex profile, 0067, proposed, says what attach
  for the app and show do and the choices this change makes.
- **The site** says `flume open` shows a Rex stream's session in the
  Rex app once a person adds its server's socket under Remote Hosts and
  turns on Remote Control, and moves the Rex app out of the roadmap's
  decided-not-built list.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `terminal-profiles`: the Rex profile's attach for the app, its
  labels on a place's window and its base's Rex session, and where it
  shows a place's clues, in place of its attach and show refusing.
- `stream-terminals`: a fix on a stream under Rex carries its line on
  its window and its base's count on its Rex session while it runs, and
  a Rex stream's session opened with no app added is refused naming the
  socket to add.

## Impact

- `crates/flume/src/profiles/terminal/rex/mod.rs`: `attach`, `show`,
  `shown_at`, and the labels' text.
- `crates/flume/tests/profiles/tools/terminal_rex.rs`: the labels and
  attach against the pinned Rex; the seam harness's clues.
- `crates/flume/tests/profiles/tools/walkthroughs.rs`: the fix under
  Rex reads its labels; `flume open` of a Rex stream's session through
  the binary.
- `docs/adr/0067-…`: amended, still proposed.
- `site/src/content/docs/guides/places-and-herdr.mdx`,
  `site/src/content/docs/roadmap.md`.
- A host whose Rex stream names `view = "app"` has `flume open` show
  the stream's session in the Rex app, refused before.

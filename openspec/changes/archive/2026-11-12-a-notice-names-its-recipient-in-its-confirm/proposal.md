# A notice names its recipient in its confirm

## Why

A notice handed to a recipient told it `flume confirm <key>`, with no
recipient, and that confirm was refused: "`reader`:
`herdr:w3:p3@/Users/chuck/.config/herdr/sessions/agentplot/herdr.sock`
is not a reader of
`bolt--claude-codes-telemetry-reaches-the-hosts-collector/c83246f8/submission/1/landed`"
(signal 2026-11-13-agentplot-ops-4b4a402a/02). `flume confirm` confirms
as `$FLUME_RECIPIENT`, a role's run's address, or the terminal's session
where none is named (7, 132), and an agent in a pane reads as its pane's
own name. An end owed to a role's address that a standing agent holds,
or to a session by its name, is owed under another name, so the command
the notice gave confirmed as no reader of it. Clause 43 has the notice
name "the delivery's key by which to confirm it"; a command its
recipient cannot run confirms nothing.

The design ruled that a notice to a recipient says
`flume confirm --recipient <recipient> <key>`, as status' expired line
already does.

## What Changes

- **A notice confirms as its recipient (43; weir 102).** The notice sent
  to a recipient ends `; confirm: flume confirm --recipient <recipient>
  <delivery key>`, the recipient's name quoted for a shell where it holds
  anything but letters, digits and `_@%+=:,./-`.
- **The delivery's message is every reader's (weir 100).** A delivery
  has one message for all its readers, so the message recorded is the
  notice but its way to confirm, and each notice adds its recipient's. A
  message an earlier flume recorded with `; confirm: flume confirm
  <key>` in it is sent with that taken off and the recipient's in its
  stead.
- **The inbox, a wait and the drop's refusal say it the same way (43).**
  The person's form of the inbox and of a wait gives the command with
  the recipient they collected as, and a tidied place's drop refused
  while ends stand pending names `--recipient <recipient>` in the
  commands it gives. One function, `confirm_command`, words the command
  for the notice, the inbox, a wait and status' expired line.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `end-delivery`: a notice names the recipient it is sent to in its
  `flume confirm`, and the delivery's message is every reader's.

## Impact

- `crates/flume-core/src/ends.rs`: `notice` gives the delivery's
  message; `notice_to` and `confirm_command` give the notice sent and
  the command.
- `crates/flume/src/commands/print.rs`: the inbox's, a wait's and the
  expired line's confirm through `confirm_command`.
- `crates/flume-core/src/acts.rs`: the drop's refusal.
- The conformance suite's expected notices name their recipients, and
  the end-delivery spec's "A notice into a standing agent" runs the
  command a notice names.
- The site's guides and reference give the notice and the confirm as
  they now read.
- No field of the JSON form changes (61).

# A notice refused in flight is sent again

## Why

The merge queue failed main's `flume:tools` on #50 with S94's
walkthrough waiting three minutes for the notice of construct's
stop-short in the person's pane: the place waited on its answer and the
notice never came. S94 failed the same gate a run before #50 for
another reason, so the walkthrough is not new to failing under the
gate's load, and the gate's run left no log of flume's to read.

Reading `deliver_ends` finds one way an end's notice is lost for good.
A settle claims the notice on the place and sends it. A second settle,
the caller's watch or an agent's quiet, reads the claim while the send
is in flight and, finding no hand-over in the host's deliveries,
records one by notice at the claim's time, as for a send answered sent
whose record failed. Where the terminal then refuses the send, as a
herdr call that does not answer within its bound under load, the first
settle takes its claim back, but the hand-over stays recorded: the end
reads handed to the recipient by a notice the terminal never typed, and
clause 43 sends no second. The requirement records each hand-over "with
to whom, when and by what"; this one never happened.

## What Changes

- **A claim says whether its send is answered (43, 75).** A notice is
  claimed sending; on the terminal's answer sent the claim is marked
  answered and kept, then its hand-over recorded. A send refused takes
  the claim back, as before.
- **Only an answered claim is recorded by another settle (43, 75; weir
  103).** A settle that reads a claim whose hand-over the host's
  deliveries lack records it only once the claim is answered, as where
  the sender's record failed or has yet to land; a claim still sending
  is its sender's. It still holds the recipient's notice, so two
  settles send one.
- **A claim written before reads answered.** A claim an earlier flume
  wrote has no `sending`, and is recorded as it was.
- **A send a settle never finished stays claimed.** A claim left
  sending by a settle ended mid-send, which the terminal may have
  typed, still holds the recipient's notice and is recorded by no
  settle; the end waits for the recipient's inbox, a wait or its
  session's start, where before it read handed over by a notice that
  may never have been typed.
- **S94's walkthrough says what flume could not do.** A walkthrough
  that never comes to what it settles toward gives the host's own log,
  and the person's session's log, beside each start's.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `end-delivery`: a notice's hand-over is recorded only once its send is
  answered sent, and a send refused while another settle reads its claim
  leaves the end to the next settle's notice.
- `seam-contracts`: `EndHeld::noticing` gives each claim whether its
  send is still unanswered.

## Impact

- `crates/flume-seams/src/held.rs`: `NoticeClaim::sending`, written only
  while true.
- `crates/flume-core/src/ends.rs`: `deliver_ends` claims sending and
  marks the claim answered on the terminal's answer sent;
  `record_claims` passes over a claim still sending.
- `crates/flume-fakes/src/terminal.rs`: `refuse_next_send`, a send
  refused once after what runs while it is in flight.
- The conformance suite's new scenario, "a notice whose send is refused
  while another settle reads its claim", fails without the change with a
  hand-over recorded of a notice never sent.
- `crates/flume/tests/profiles/tools/`: the walkthroughs' logs on
  failure.
- No field of the JSON form changes (61).

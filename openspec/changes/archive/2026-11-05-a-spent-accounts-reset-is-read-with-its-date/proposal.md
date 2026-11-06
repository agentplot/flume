# A spent account's reset is read with its date

## Why

On deneb, 2026-11-05 about 20:40Z, four agents on one account stopped
with "You've hit your weekly limit · resets Nov 8 at 12am
(America/New_York) (rate_limit)". flume recorded the account spent for
an hour, as 146 gives where the words name no reset, since the Claude
Code profile read a date only joined to its time by a comma, a form the
pinned 2.1.296 never writes. An hour later the pool took the account
again with its weekly limit spent, and status gave nothing of why the
pause was an hour (record 0045, accepted by the user on 2026-11-05 at
the fix's review).

## What Changes

- **A reset is read as 2.1.296 writes it (151, C.4).** Within a day, a
  time of day, `5pm`; past a day, a date and a time joined by ` at `,
  `Nov 8 at 12am`, with the year after the day where it is not the
  hook's, `Jan 3, 2027 at 9am`. A limit on a budget names its plan
  limit's reset in `your <limit> resets <when> (<zone>)`, which is read
  too. A date joined by a comma is no longer read.
- **The pinned release holds every fragment the reading rests on
  (core.22).** The budget's part, the formatter's choice of a date past
  a day, and its options for each form join the fragments the
  check-tools test finds in the release.
- **A form flume cannot read is still an hour, and status gives its
  words (56, 146).** The spent document marks the hour `unread`, and each
  account of a pool gives `spent_unread`, its agent's words, while that
  hour stands.
- **A host's reading of an account is left as record 0035 made it
  (185).** A reading never moves a spent pause, and each holds the
  account on its own; this change reads only the pause's time.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `agent-profile`: the usage limit's reset is read in the forms the
  pinned release writes, a date and a budget's plan limit among them,
  and the fragments the check-tools test pins grow by the formatter's.
- `account-pools`: a spent account's hour is marked where no reset was
  read in its words.
- `place-status`: status gives the words of an account spent for that
  hour.

## Impact

- `crates/flume`: the Claude Code profile's `limit`, its fragments and
  tests, and the person's form of a pool's accounts.
- `crates/flume-seams`: `AccountSpent::unread`, read false where a
  document lacks it, and its round trip.
- `crates/flume-core`: `record_spent` marks the hour, the queue's
  `AccountRoom` and `AccountStatus::spent_unread` carry the words.
- `crates/flume-conformance`: the no-reset scenario and the answers'
  field list.
- `site/`: the guide's spent account and threshold, and the state
  reference.
- `docs/`: record 0045, accepted 2026-11-05; rule core.32's details.
- agentplot/blueprints: 146 and C.4's spent row, on main at 2287669
  (core.19).
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool.

# Tasks

The work is on `fix/a-spent-accounts-reset-is-read-with-its-date`. Every
test of the reading uses fixed words and a fixed clock.

## 1. The reading

- [x] 1.1 Have the Claude Code profile's `limit::reset` read the ` · resets <when> (<zone>)` part or the budget's ` · your <limit> resets <when> (<zone>)` part, `<when>` a time of day or a date and a time joined by ` at `, with the year after the day where the release writes one, and read no date joined by a comma; add the budget's part, the formatter's choice of a date past a day off and its options for each form to `limit::fragments` (151, C.4; core.22). True when `sluice exec main -- moon run flume:test flume:tools` passes, with `limit.rs` holding the agent-profile spec's scenarios, deneb's words at 2026-11-05T20:40Z read as 2026-11-08T05:00Z among them, and `tools/claude_words.rs` finding every fragment in the pinned release.

## 2. The hour and status

- [x] 2.1 Add `AccountSpent::unread`, read false where a document lacks it and left out where false; have `record_spent` mark the hour it records where no reset was read and clear it where a later reset replaces the hour; carry the words to `AccountRoom` and `AccountStatus::spent_unread` while the hour stands, and print them under the account's line (56, 146). True when `sluice exec main -- moon run seams:test core:test conformance:test conformance:coverage flume:test` passes, with `round_trip.rs` holding a spent document written before, `engine/account_pools.rs` the account-pools spec's scenarios, `engine/answers.rs` the field in the answers' list, and `print.rs` the place-status spec's account spent for an hour.

## 3. The site and the records

- [x] 3.1 Say in `site/src/content/docs/guides/agent-settings.md` the forms a reset is read in, that status gives the words of an hour's pause, and that a pause and a reading each hold an account on its own; give `unread` and `spent_unread` in `site/src/content/docs/reference/state.md`, to any host's operator (core.35). Record 0045, accepted by the user on 2026-11-05 at the fix's review, its statement amendment cited on agentplot/blueprints main at 2287669, and rule core.32's details in `docs/architecture/core.md`. True when `sluice exec main -- moon run site:prose site:build root:adrs` passes.

## 4. The gate

- [x] 4.1 Run the checks of the projects this change touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test core:format core:lint core:test conformance:format conformance:lint conformance:test conformance:coverage flume:format flume:lint flume:test flume:tools site:prose site:build root:format root:adrs root:changes root:specs` passes.

## 5. The acceptance

- [x] 5.1 Mark record 0045 accepted on the user's words at the fix's review, 'Accept and release it, and push the statement change', with the day, citing agentplot/blueprints main at 2287669 where it cited the design branch, and the same in this proposal and these tasks; archive this change so the specs reconcile (core.19). True when `openspec validate --specs --strict` and `sluice exec main` pass.

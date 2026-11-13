# Tasks

Every command runs from the place's root, inside `devenv shell`.

## 1. A notice confirms as its recipient

- [x] 1.1 In `crates/flume-core/src/ends.rs`, have `notice` give the delivery's message, the notice but its way to confirm, and add `notice_to`, the message with `; confirm: ` and `confirm_command` for the recipient after it, an earlier message's `; confirm: flume confirm <key>` taken off first, and `confirm_command`, `flume confirm --recipient <recipient> <key>…` with the recipient quoted for a shell where it needs it; have `deliver_ends` send `notice_to` for each recipient (43; weir 100, 102; core.11). Verify with `sluice exec main -- moon run core:test`.
- [x] 1.2 In `crates/flume/src/commands/print.rs`, have the inbox's and a wait's confirm, and the host's expired line, give `confirm_command` with their recipient, and in `crates/flume-core/src/acts.rs` have a tidied place's drop refused while ends stand pending give `--recipient <recipient>` in its commands (43). Verify with `sluice exec main -- moon run flume:test`.
- [x] 1.3 In the conformance suite, have every expected notice name its recipient, the stop-short's and the delivery's notice scenarios read `flume confirm --recipient ops`, and add the end-delivery spec's "A notice into a standing agent"; in the walkthrough of a standing agent under herdr, the notice names `w/planner` (43, 132). Verify with `sluice exec main -- moon run conformance:test`.
- [x] 1.4 In the site, `guides/callers-and-holds.md`, `guides/roles-and-wake.md`, `reference/commands.md` and `reference/state.md` give the notice and the confirm as they now read, to any host's operator (core.35). Verify with `sluice exec main -- moon run site:prose`.

## 2. The spec

- [x] 2.1 The delta has the notice name its recipient in its confirm. Verify with `openspec validate a-notice-names-its-recipient-in-its-confirm --strict`, then archive with `openspec archive a-notice-names-its-recipient-in-its-confirm --yes`.

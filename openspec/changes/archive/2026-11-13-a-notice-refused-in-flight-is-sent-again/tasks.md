# Tasks

Every command runs from the place's root.

## 1. A notice's hand-over waits on its send's answer

- [x] 1.1 In `crates/flume-seams/src/held.rs`, give `NoticeClaim` `sending`, read false where a claim written before has none and written only while true, and in `crates/flume-seams/src/round_trip.rs` round-trip a claim answered and one sending, the answered one written with no `sending` (43, 75, 61). Verify with `sluice exec main -- moon run seams:test`.
- [x] 1.2 In `crates/flume-core/src/ends.rs`, have `deliver_ends` claim a notice sending and mark its claim answered on the terminal's answer sent, before it records the hand-over, and `record_claims` pass over a claim still sending (43, 75; weir 103). Verify with `sluice exec main -- moon run core:test`.
- [x] 1.3 In `crates/flume-fakes/src/terminal.rs`, add `refuse_next_send`, and in the conformance suite add the end-delivery spec's "A notice refused while another settle reads its claim", another settle run while the send is in flight and the send then refused; it fails without 1.2 with a hand-over recorded of a notice never sent (43, 75). Verify with `sluice exec main -- moon run conformance:test`.
- [x] 1.4 In `crates/flume/tests/profiles/tools/`, have a walkthrough that never comes to what it settles toward give the host's own log beside each start's, and S94's person's session keep its log among them (S94). Verify with `sluice exec main -- moon run flume:tools`.

## 2. The spec

- [x] 2.1 The deltas record a notice's hand-over only once its send is answered sent, and give a claim's `sending`. Verify with `openspec validate a-notice-refused-in-flight-is-sent-again --strict`, then archive with `openspec archive a-notice-refused-in-flight-is-sent-again --yes`.

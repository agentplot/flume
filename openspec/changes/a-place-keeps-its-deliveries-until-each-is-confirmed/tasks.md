# Tasks

The work is on `fix/main--a-place-keeps-its-deliveries-until-each-is-confirmed`,
cut from `main`. The statement is read at boomdrivedev/blueprints main
6557c33 (43, 56, 62); weir's at the revision the tree pins (101, 102,
107, I9).

## 1. Status names only a recorded key

- [x] 1.1 `EndStatus::of` names `delivery` from the end's recorded delivery alone. Verify with `sluice exec main -- moon run conformance:test`, the end-delivery scenario "a code end whose delivery the host's deliveries refused to record".

## 2. A place keeps its deliveries until each is confirmed

- [x] 2.1 The fake store runs an act inside its next read of every place, between its listing and its reads, and refuses one place's next write once.
- [x] 2.2 `let_go_of_the_gone` takes a place as gone only where a second read of every place lacks it too. Verify with `sluice exec main -- moon run core:test`, `deliveries::tests::a_done_delivery_of_a_document_a_tidy_moves_during_the_read_is_kept`.
- [x] 2.3 A tidy whose kept document cannot be written keeps the place's own document, its log saying why. Verify with the end-delivery scenarios "a fix for ops and boomdrive lands" and "the document kept for its ends cannot be written".

## 3. flume's log names each delivery

- [x] 3.1 The record of a delivery and the let-go of one each write a line to flume's log once the write lands. Verify with the end-delivery scenario "flume's log names each delivery".

## 4. The spec

- [x] 4.1 Validate with `openspec validate a-place-keeps-its-deliveries-until-each-is-confirmed --strict`, then archive with `openspec archive a-place-keeps-its-deliveries-until-each-is-confirmed --yes`.

## ADDED Requirements

### Requirement: A recipient no pane holds collects its ends

A recipient that no pane holds SHALL be sent no notice, and each end
pending for it SHALL stay pending, keeping its tidied place's document,
until it collects the end by `flume inbox` or a wait as itself (43,
62).

#### Scenario: A landing for a recipient no pane holds

- **WHEN** a fix launched with the recipients `ops` and `boomdrive` lands while `ops` waits in the fake terminal and no pane holds `boomdrive`
- **THEN** `ops` gets one notice of the landing and no notice goes to `boomdrive`; the place is tidied, its document stays with code's end and the landing pending for `boomdrive` alone; and once `boomdrive` collects them by `flume inbox` naming the place, the next settle removes the document (43, 62)

### Requirement: A drop or a done is an end no notice carries

A drop, and a done of a place whose kind does not land, SHALL record an
end of the place, keyed `over/dropped` or `over/done`, in the write that
makes its work over. No notice of it SHALL be sent and no role SHALL be
woken for it: it SHALL stand pending for each recipient of the place
that is no role's address, and for no one where every recipient is one
(43, 52, 114, 132).

#### Scenario: A drop for a role and a reader

- **WHEN** the caller drops a live fix launched with the recipients `east/planner`, which the scratch source `acme` declares with a run, and `boomdrive`
- **THEN** status of the tidied document gives the end `over/dropped` pending for `boomdrive` alone; no notice is sent and no run of `east/planner` is launched; and the document stays until `boomdrive` collects the end by `flume inbox` (43, 52, 62)

#### Scenario: A drop for a role alone

- **WHEN** the caller drops a live fix launched with the recipient `east/planner` alone, which `acme` declares with a run
- **THEN** the end `over/dropped` is recorded pending for no one, no notice is sent, no run of `east/planner` is launched, and the tidy keeps no document of the place (43, 52, 62, 132)

#### Scenario: A drop for a session in a pane

- **WHEN** the caller drops a live fix launched with the recipient `ops` while `ops` waits in the fake terminal
- **THEN** `ops` gets no notice of the drop, the end `over/dropped` stays pending for `ops`, and the document stays until `ops` collects it by `flume inbox` (43, 52, 62)

#### Scenario: A done

- **WHEN** a place of a kind that does not land, launched with the recipient `boomdrive`, is declared done
- **THEN** the end `over/done` stands pending for `boomdrive`, and the tidied document stays until it is collected (43, 114)

### Requirement: A tidied place's document stays while an end is pending for a recipient

A place whose work is over SHALL be tidied as always, its worktree and
branch removed (66, 67), and while an end of it stands pending for a
recipient its document alone SHALL stay, naming the recipients it waits
on, until every end is delivered or its caller drops it. A launch of the
same name SHALL not be refused for it (3, 43, 62).

#### Scenario: A landing with its recipient away

- **WHEN** a place lands while its one recipient's session is gone
- **THEN** its worktree and branch are removed, status gives the tidied place pending for that recipient, and once the recipient collects by `flume inbox` the next settle removes the document

#### Scenario: Dropped while pending

- **WHEN** the caller drops a tidied place whose end stands pending
- **THEN** the document and its pending ends are gone, and the inbox gives nothing of it

#### Scenario: Launched again

- **WHEN** a place of the same repository, kind and name is launched while the tidied document stays
- **THEN** the launch makes its place, and the tidied document's ends stay pending as they were

#### Scenario: Kept for a recipient no pane holds

- **WHEN** a place launched with the recipients `ops` and `boomdrive` lands, `ops` has its notice, and no pane holds `boomdrive`
- **THEN** status gives the tidied place pending for `boomdrive` (62)

## MODIFIED Requirements

### Requirement: An end's recipients

An end's recipients SHALL be its place's, as its launch took them. A
stop-short's that named a role SHALL be that address alone, not the
place's. A stall's SHALL be the place's, the address the stop-short it
waits on named, of whichever stream, and the answerer behind the
place's own stream, each once. A drop's or a done's SHALL be the
place's that are no role's address (7, 41, 43, 130, 193).

#### Scenario: A stop-short that named a role

- **WHEN** a construct of a place in `east`, whose recipient is `east/operator`, stops short naming `ops`
- **THEN** the end stands pending for `east/ops` alone, and none of it for `east/operator` (41, S93)

#### Scenario: A stall of a question for another stream

- **WHEN** a construct of a place in `east`, whose recipient is `east/planner`, stopped short naming `west/design`, `west/operator` stands behind `west`, and the place stalls
- **THEN** the stall is delivered to `east/planner`, `west/design` and `east/operator`, once each, and not to `west/operator` (130, 193)

### Requirement: An inbox naming a place

An inbox that names a place SHALL give and record only the ends of the
documents `flume status` reads under that name, the place's own, else
those kept for it once it was tidied, or the one document
`<place>@<at>` names, and SHALL give only that place's entry of the
places that wait on the recipient. It SHALL be refused, naming the
place and marking nothing delivered, where no document stands under
that name (43, 62, core.25).

#### Scenario: An inbox naming a place

- **WHEN** two fixes launched with the recipients `ops` and `boomdrive` have landed and been tidied, no pane holding `boomdrive`, and `boomdrive` runs `flume inbox` naming the first
- **THEN** the answer holds the first's ends alone, its code's end and its landing, delivered to `boomdrive`; the second's stay pending for `boomdrive` and its document stays (43, 62)

#### Scenario: A name under which nothing stands

- **WHEN** an inbox names a place under which no document stands on the host
- **THEN** it is refused naming the place, and nothing is marked delivered (43)

## REMOVED Requirements

### Requirement: The end of a place's work waits for its caller

**Reason**: A launch names no caller (record 0060, rewritten on the user's answer of 2026-11-08): a reader names itself one more recipient, and "A recipient no pane holds collects its ends" keeps its ends.

**Migration**: Name the reader a recipient, `--recipient <name>`, beside the others the launch names, and collect by `flume inbox <place>` as that name.

### Requirement: A drop or a done is an end for the caller alone

**Reason**: Replaced by "A drop or a done is an end no notice carries", pending for the place's recipients that are no role's address.

**Migration**: None: the end's key and its form are as they were.

### Requirement: A tidied place's document stays while an end is pending

**Reason**: Replaced by "A tidied place's document stays while an end is pending for a recipient": a place has no caller (record 0060, rewritten), so the document stays for its recipients alone.

**Migration**: None.

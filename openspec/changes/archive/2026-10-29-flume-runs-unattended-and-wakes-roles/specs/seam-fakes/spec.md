## ADDED Requirements

### Requirement: The fake terminal holds recipients' sessions

The fake terminal SHALL hold sessions by name, each with a settable
state, SHALL record every text sent to each in order, and SHALL answer a
name it holds no session for as not found, so the suite can script a
recipient away, asking, or restarted (43, 95).

#### Scenario: A recipient scripted away and back

- **WHEN** a test removes the session `ops` from the fake terminal, settles a place whose end is for `ops`, then adds `ops` back and settles again
- **THEN** the fake records no notice to `ops` from the first settle and one from the second

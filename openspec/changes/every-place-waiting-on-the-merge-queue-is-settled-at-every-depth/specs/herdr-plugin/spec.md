## ADDED Requirements

### Requirement: The watch settles on its bound with no flume agent in any pane

The watch SHALL run `flume settle`, naming no place, once per bound
whether or not any pane holds a flume agent, so a place with no agent,
a submitted one among them, is settled within the bound; it SHALL read
nothing of a place to decide whether to settle (46, 122, C.5).

#### Scenario: Only a submitted place

- **WHEN** a stand-in herdr lists no pane holding an agent and the stand-in flume's status names one place, submitted, with no agent, for three bounds
- **THEN** the watch runs `flume settle` with no place named once per bound, three times, and the stand-in flume records each (122, S109)

## ADDED Requirements

### Requirement: A role's run takes no notice before its prompt

While the current start of an address's live run is starting, or
started with its prompt not yet received, a settle SHALL send no notice
into the run's session, which its agent would take as its prompt, and
the end SHALL stay pending until a settle reads the start begun or the
run's inbox collects it. A standing agent of the address SHALL be
delivered to as before (37, 38, 43, 132).

#### Scenario: A run not yet given its prompt

- **WHEN** a fix's end wakes `east/planner`'s run, whose agent waits in the session `east/planner` before it says it started, and the fix is settled again
- **THEN** no notice goes into the run's session and the end stays pending; once the agent says it started and its prompt is received, the next settle of the fix delivers the end into the run's pane once (37, 38, 43)

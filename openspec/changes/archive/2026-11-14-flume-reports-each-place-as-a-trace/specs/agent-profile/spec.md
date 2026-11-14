## ADDED Requirements

### Requirement: Claude Code's conversation read from its started word

The Claude Code profile SHALL give a start's conversation as the
session its word that it started names, read from its record in the
place's own folder, and none where the start has said nothing yet; a
profile that reads none SHALL give none (217, 128).

#### Scenario: The conversation of a started agent

- **WHEN** the profile reads a start whose word that it started names the session `s-1`, and a start that has said nothing
- **THEN** it gives `s-1` for the first and none for the second (217)

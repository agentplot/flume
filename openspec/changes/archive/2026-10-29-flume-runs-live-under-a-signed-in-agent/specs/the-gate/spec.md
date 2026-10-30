## ADDED Requirements

### Requirement: The live walkthroughs pass under the pinned Claude Code

The `check-live` layer's walkthroughs SHALL carry a fix and a change
from their launch to their landing with the pinned Claude Code under
herdr, signed in by a test account's token, and SHALL leave a stage
whose pane goes quiet with no deliverable not ended (39, S3); the
operator runs them by hand, outside `sluice exec`.

#### Scenario: The operator's run

- **WHEN** the operator runs `moon run flume:live` from the devenv shell with a test account's token
- **THEN** both walkthroughs land, and the quiet stage with no deliverable is not ended

## ADDED Requirements

### Requirement: herdr keeps a place's stages in two of its stream's sessions

Under the pinned herdr, where the core opens one stage's agent of a
place in one named session and another's in a second, the herdr profile
SHALL open each in the place's tab of its base's workspace in that
session, SHALL send to, read and close each on its own pane and server,
and SHALL close the place's tab in both sessions at its tidy (107, 137,
138, 190, 199, S106).

#### Scenario: A change's design apart, under herdr

- **WHEN** under the pinned herdr, with herdr's sessions under a scratch home, a change's construct agent opens in the session `work-design` and its code agent in `work`, each is sent a prompt, read and closed, and the place is tidied
- **THEN** herdr lists `work-design` and `work` running; each server held the workspace `repository main` with the tab `change--<name>` and that agent's pane; each prompt, state and close went to that agent's pane on its own server; and after the tidy neither server holds the tab or the workspace (107, 137, 138, 190, 199, S106)

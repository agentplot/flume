## ADDED Requirements

### Requirement: A command's pane is named as its agents are given

A terminal profile that gives a part's agents by name SHALL name the
session a command of flume's runs in in the form its `agents_in` gives
an agent's session name, so a command in an agent's pane names that
agent's: `herdr:<pane>@<socket>` under herdr and `rex:<block>@<socket>`
under Rex, each with the socket its `agents_in` gives the part's agents
at, checked against the pinned release (7, 132, C.5; core.29).

#### Scenario: A command in a herdr session's pane

- **WHEN** under the pinned herdr, the session `w` runs the agent `w-planner`, and a command of the test's own runs in another pane of `w`'s server
- **THEN** the session the profile names for that command is `herdr:<that pane>@<socket>`, the socket the profile's `agents_in` of `w` gives `w-planner` at (132, C.5)

#### Scenario: A command in a labelled Rex block

- **WHEN** under the pinned Rex, a block labelled `ops` in the session `east` runs the stand-in, and the profile is given the `REX_BLOCK` and `REX_SERVER` that block's program ran under
- **THEN** the session the profile names for a command run there is the session name its `agents_in` of `east` gives `ops` at (132, C.5)

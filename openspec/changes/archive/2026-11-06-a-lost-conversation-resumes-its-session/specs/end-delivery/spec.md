## ADDED Requirements

### Requirement: A lost conversation's end says flume left an agent in its pane

The end of a conversation's stage read lost while an agent the
terminal brought back stands in its pane SHALL say that flume left that
agent in its pane, in the end held on the place and in its notice to
each recipient (43, 77).

#### Scenario: A lost talk with a bare agent

- **WHEN** after a rebuild the fake terminal brings a talk's agent back bare, with the session `ops` waiting, and the settle reads talk lost
- **THEN** the end gives `left_in_pane`, and the fake terminal records one notice to `ops` naming talk lost and that flume left the agent in its pane (43, 77, S113)

#### Scenario: A lost talk with no agent

- **WHEN** a talk's agent is killed and the settle reads talk lost
- **THEN** the end gives no `left_in_pane`, and the notice says nothing of an agent left (43, 45)

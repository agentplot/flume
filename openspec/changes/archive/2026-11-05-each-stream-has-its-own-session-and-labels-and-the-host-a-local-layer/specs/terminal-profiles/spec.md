## MODIFIED Requirements

### Requirement: Where herdr opens an agent

The herdr profile SHALL open an agent in herdr's session the core
names, its server started headless where none runs, or in flume's own,
`flume`, where the core names the terminal's own: in that session's
workspace of the place's repository and base, labelled `<repository>
<base>`, made when first needed, one tab per place named
`<kind>--<name>`, each recorded with its server; or in a split beside
the pane the settings name (94, 107, C.5; records 0018, 0031).

#### Scenario: A conversation beside the operator's pane

- **WHEN** a stage's settings name the pane `operator`
- **THEN** the agent opens in a split beside that pane, held by its pane and socket, under a herdr name made from its identity

#### Scenario: Places on two bases

- **WHEN** places on `main` and on `bolt/the-surfaces` of one repository open under herdr in flume's own session
- **THEN** each base's places are tabs of a workspace of its own, labelled `<repository> main` and `<repository> bolt/the-surfaces`

#### Scenario: Two streams' sessions under herdr

- **WHEN** under the pinned herdr, with herdr's sessions under a scratch home, a fix's agent opens in the session `work` and another's in `personal`
- **THEN** herdr lists `work` and `personal` running, `work`'s server holds the workspace `<repository> main` with the first fix's tab `fix--<name>` and its agent's pane, and `personal`'s the other's (107, S82)

#### Scenario: A session name herdr refuses

- **WHEN** the core names a session the pinned herdr refuses to start
- **THEN** the open is refused in herdr's words, and nothing of flume's is left open (98; core.29)

## ADDED Requirements

### Requirement: herdr tidies a place in every session

The herdr profile SHALL close a place's tab in every session it made
one in when the place goes, and each of its base's workspaces once it
holds no tab of a live place and no pane flume did not open; and SHALL
read and tidy where they stand the workspaces and tabs it recorded
before it kept them by server (137, 138, C.5; record 0031).

#### Scenario: A place whose agents opened in two sessions

- **WHEN** a place's first stage opened in flume's own session, its stream then names `work`, its next stage opens in `work`, and the place lands and is tidied
- **THEN** its tab in each session is closed, and each base's workspace it was the last place of (137, 138)

#### Scenario: Records written before

- **WHEN** a place whose workspace and tab were recorded before they were kept by server is tidied
- **THEN** its tab and its base's workspace in flume's own session are closed as before (138)

### Requirement: The base's count on each of its workspaces

Status's `bases` SHALL give a repository's base once for each workspace
the terminal holds for it, each with the base's count and where that
workspace is, so the herdr plugin refreshes the count on the workspace
on its own server (139, C.5).

#### Scenario: One base in two sessions

- **WHEN** a repository's base has places in `work` and in `personal`, each session holding a workspace for it
- **THEN** `bases` gives the base twice with one count, each with its own workspace's `herdr:<id>@<socket>` (139)

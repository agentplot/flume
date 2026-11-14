## MODIFIED Requirements

### Requirement: A launch's recipients

A launch naming no recipient SHALL take `$FLUME_RECIPIENT`, or else the
address a source gives the standing agent it runs in, or else the
session that launched it as the terminal names it, or else the answerer
behind the place's stream, the role a source of the host declares to
stand behind it, and SHALL record the place with its priority, its
recipients, where each came from, and its holds' words, which nothing
after the launch changes (7, 8, 43, 70, 132, 193; core.28).

#### Scenario: Recipients at the launch

- **WHEN** one launch names two recipients, another names none with `FLUME_RECIPIENT` set, and a third names none from a session the fake terminal names
- **THEN** status gives the first place both, the second the variable's, the third that session, and nothing after the launch changes them (7, 8)

#### Scenario: A launch from no session takes its stream's answerer

- **WHEN** a launch in the stream `east` names no recipient, `FLUME_RECIPIENT` is unset, the fake terminal names no session, and the scratch source `acme` declares `east/operator` with a run, standing behind `east`
- **THEN** the place's one recipient is `east/operator`, and status gives it as from the stream's answerer, declared by `acme` (7, 193, S93)

#### Scenario: Each stream its own answerer

- **WHEN** `acme` also declares `west/operator` standing behind the stream `west`, and a like launch names `west`
- **THEN** the place's one recipient is `west/operator` (143, 193)

#### Scenario: A launch from a standing agent's pane

- **WHEN** `acme` gives `east/planner` the standing agent `east-planner` in the session `east`, the fake terminal holds it there, and a launch naming no recipient, with `FLUME_RECIPIENT` unset, runs in its pane
- **THEN** the place's one recipient is `east/planner`, and status gives it as from the standing agent `east-planner`, declared by `acme` (7, 132)

## ADDED Requirements

### Requirement: A launch from the pane of an agent standing for several addresses

A launch naming no recipient, with `$FLUME_RECIPIENT` unset, from the pane
of an agent its sources give more than one address SHALL be refused,
naming the agent and each address and asking for `--recipient`, and
SHALL make nothing (7, 132; core.8, core.28).

#### Scenario: A launch from the pane of an agent standing for two addresses

- **WHEN** `acme` gives `east/planner` and `east/ops` each the standing agent `east-planner` in `east`, and a launch naming no recipient, with `FLUME_RECIPIENT` unset, runs in its pane
- **THEN** the launch is refused naming `east-planner`, `east/ops`, `east/planner` and `--recipient`, and no branch, worktree or held state is made (132; core.8)

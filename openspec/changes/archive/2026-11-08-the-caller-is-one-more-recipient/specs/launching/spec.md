## MODIFIED Requirements

### Requirement: A recipient a launch names

A recipient the launch names SHALL be a session's name, whether or not
a pane holds it, or a role's address of any stream written whole,
`<stream>/<role>`, and SHALL be recorded as from the launch, whatever
the place's own stream (7, 132, 193).

#### Scenario: Another stream's role named

- **WHEN** a launch in the stream `east` names the recipient `west/planner`, which the scratch source `acme` declares
- **THEN** the place's one recipient is `west/planner`, from the launch

#### Scenario: A role's address and a reader beside it

- **WHEN** a launch names the recipients `madswan/planner` and `boomdrive`, a name no pane holds
- **THEN** status gives the place's recipients `madswan/planner` and `boomdrive`, from the launch, and a hold and a retry after the launch leave them as they were (7, 8)

## REMOVED Requirements

### Requirement: A launch may name its caller

**Reason**: A launch names no caller (record 0060, rewritten on the user's answer of 2026-11-08); a reader names itself a recipient.

**Migration**: Name the reader a recipient, `--recipient <name>`, beside the others the launch names.

### Requirement: A caller is no recipient and no owner

**Reason**: There is no caller (record 0060); a recipient is no owner either (69).

**Migration**: None.

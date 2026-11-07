## ADDED Requirements

### Requirement: Status gives an agent's effort

Status SHALL give the current agent's effort as its layer wrote it,
with the layer it came from, the value its profile passed, and that it
was not read back; an agent whose settings give no effort SHALL be given
none. Where the agent's account's environment names the variable its
program reads over the effort, status SHALL name that variable as
outranking it, never its value, in the JSON form and the person's (56,
197; core.30).

#### Scenario: An effort from the launch

- **WHEN** status is asked of a place whose agent took `xhigh` from `--agent effort=xhigh`
- **THEN** the agent's settings give the effort `xhigh` from `launch`, passed and not read back, in the JSON and the person's form (S103, 56)

#### Scenario: An account's variable that outranks it

- **WHEN** status is asked of a place whose agent took `high` from `host-kind` and whose account's `env` names the variable the fake names as read over its effort
- **THEN** the agent's effort names that variable as outranking it, and no value of it (56, 197)

#### Scenario: No effort

- **WHEN** status is asked of a place whose agent's settings give no effort
- **THEN** the agent's settings give no effort, and the JSON form holds no `effort` for it (197)

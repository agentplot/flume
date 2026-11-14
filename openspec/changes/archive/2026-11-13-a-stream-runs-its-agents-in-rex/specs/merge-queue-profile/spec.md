## MODIFIED Requirements

### Requirement: Submitted through sluice with no wait

The sluice profile SHALL submit by `sluice submit <base> --no-wait
--json` in the place, its environment stripped of every multiplexer's
session, `HERDR_*`, `REX_*`, `ZELLIJ*`, `TMUX` and `TMUX_PANE`, never
with `--check` and never through `wt submit`, answering with the entry
once sluice holds it, or the refusal with sluice's words (111, 113, 117,
C.6, core.6).

#### Scenario: A submission from a herdr pane's hook

- **WHEN** the profile submits from a process whose environment carries `HERDR_*` words
- **THEN** sluice's entry names the place's worktree as its submitter and no session, and the answer comes before any verdict

#### Scenario: A submission from a Rex block

- **WHEN** the profile submits from a process whose environment carries `REX_SERVER`, `REX_SESSION` and `REX_BLOCK`
- **THEN** sluice's entry names the place's worktree as its submitter and no session, and the answer comes before any verdict (C.6)

## ADDED Requirements

### Requirement: A base sluice lands not

The profile's base SHALL answer that sluice lands no such base where
`.config/sluice.toml` names no table for the base, its own or a
pattern's, or no revision of the base holds a policy, so the core reads
the base at origin; and SHALL answer the local base where the base's
table names no remote, sluice landing it there (126, C.1, C.6).

#### Scenario: A base the policy names no table for

- **WHEN** base is asked of `trial`, a branch made from main whose policy names a table for main alone
- **THEN** it answers that sluice lands no such base, and main's at a revision (126, C.6)

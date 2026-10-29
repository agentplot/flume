## ADDED Requirements

### Requirement: A wrapped Claude Code starts as the program it wraps

The Claude Code agent profile SHALL name as the program the one a nix
wrapper the settings name execs, with the variables the wrapper sets,
so that herdr detects the agent and flume's foreground check takes the
program that runs (32; record 0018; C.5).

#### Scenario: A wrapper exec'ing its program

- **WHEN** the settings name a wrapper `bin/claude` that execs `bin/.claude` with variables of its own
- **THEN** the start runs `bin/.claude` under the name `claude` with those variables, herdr reads it as Claude Code, and the start is taken

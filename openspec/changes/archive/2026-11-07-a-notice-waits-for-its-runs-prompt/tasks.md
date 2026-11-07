# Tasks

The work is on `fix/flume-sessions-start-their-mcp-servers--s93-holds-steady`,
cut from `bolt/flume-sessions-start-their-mcp-servers`. The race is
shown by S93's walkthrough under the pinned herdr with its run's
launch kept from sending the prompt, the hack left out of the commits.

## 1. The delivery

- [x] 1.1 Hold the reach of a role's run for a notice while the run's current start is live and starting or started, its prompt not yet received, so the end stays pending; a standing agent of the address reached as before (37, 38, 43, 132). True when `sluice exec main -- moon run core:test conformance:test conformance:coverage` passes, with `engine/role_waking.rs` holding the role-waking spec's "A run not yet given its prompt", under its requirement "A role's run takes no notice before its prompt",, which fails with the hold taken out.

## 2. The walkthrough

- [x] 2.1 Have `settled_until` give every start's stand-in log with every place's line where it never comes to what it waits on. True when `sluice exec main -- moon run flume:lint flume:tools` passes, with S93 passing with its run's launch kept from sending its prompt, run by hand.

## 3. The records

- [x] 3.1 Record 0058, proposed, and rule core.11's details in `docs/architecture/core.md`. True when `sluice exec main -- moon run root:adrs` passes.

## 4. The gate

- [x] 4.1 Run the checks of the projects this change touched. True when `sluice exec main` passes.

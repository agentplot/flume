# Tasks

The work is on `fix/flume-list-leaves-finished-places-out`.

## 1. The statement and the records

- [x] 1.1 Amend clause 200 and S108 on a blueprints branch, `design/flume-the-list-leaves-finished-places-out`, to the user's word of 2026-11-06; add record 0049's amendment, accepted on that yes, and amend rule core.52 and its details. True when blueprints holds the commit e4fab7b on that branch and `moon run root:adrs` passes.

## 2. The list

- [x] 2.1 Have `Engine::list` leave out a place whose phase says it landed or was dropped, kept, being tidied or tidied, unless every place is asked; give `flume list --all`, its help naming it (63, 200). True when `moon run core:test flume:test` passes, with the core's unit test of each phase and the command line's test of a dropped place listed only with `--all`.
- [x] 2.2 Have the list's conformance scenarios ask for every place where they read a landed or dropped place, and add the scenario of each phase left out (62, 63, 200, S108). True when `moon run conformance:test conformance:coverage` passes.

## 3. The site

- [x] 3.1 Say in the commands reference that the list leaves out a place that landed or was dropped and that `--all` lists every place, and in the state reference that the JSON holds such a place only under `--all`. True when `moon run site:prose site:build` passes.

## 4. The gate

- [x] 4.1 Validate and archive the change, and run the checks of the projects it touched. True when `openspec validate --specs --strict` passes after the archive and `moon run :#check-fast` passes.

---
number: 53
title: A repository whose clone moved is renamed in flume by one act, and no place moves
status: rejected
date: 2026-11-06
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# A repository whose clone moved is renamed in flume by one act, and no place moves

Status: rejected by the user on 2026-11-07, on its card: "I don't think we want to be handling renames like this. A lot of my renaming is just because I'm in flux at an early stage. Once this is a final product, I think you either add a repo or remove a repo. There's no renaming". Clause 209 is taken out of the statement (boomdrivedev/blueprints b0df70b).

Deciders: the user, on proposal 13's card in Pending You, 2026-11-06, ruling on a request written under a kit's former name: "Wouldn't the root cause be that the original rename didn't rename the parts of the data that were necessary? If anything shouldn't the tools expose some sort of rename functionality? I don't want you to be building [...] custom workaround [...]", which settles that a rename renames the data that names what was renamed, that each tool exposes its own rename, and that no reader maps a former name. agentplot-design for flume's shape below.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` clauses 76, 77 and 209 (added 2026-11-06), C.1 and C.4 (an agent's identity names its repository by the id of its common git directory), C.7; records 0027 (a place's id is the location it was made at; no place is moved) and 0055 of afterthought/crew (9a4b965b on teamless/deneb: crew's kit rename renames every record naming the kit, the fleet renames clones in place, and flume 0.13.0 has no command that renames a place's stored repository, which is flume's to design); signal 2026-11-06-agentplot-planner-230ee466/01 on afterthought/crew-state agentplot/main.

## Context and problem statement

When a host's clone of a repository is renamed, Flume still names the old folder for every place it holds of that repository: in the place's repository, in the intents keyed by the repository and base, and in the links git keeps between the clone and each place's worktree. So a place of the renamed repository can no longer be read, readied, submitted or removed against its clone. crew now renames every record of its own that names a renamed kit, and the fleet renames the clone in place. Flume offers no rename of its own.

## Considered options

1. **One act of the host's renames everything flume keeps that names the former folder,** places' repositories, intents, base places and worktree links, while every place stays where it was made.
2. flume reads a place's repository under its former folder too, by a map of former folders. That is the reader working around a stale name, which the user ruled out.
3. flume moves the clone itself and renames its records in the same act. Where a host keeps its clones is the host's, the runner's on Boomdrive's hosts (Boomdrive core.14) and the fleet's under crew record 0055, so flume would take a job that is not its own.
4. flume moves each place under the repository's new id as well. A place's id is the location it was made at (record 0027), and every agent's identity and every stored state document names it, so moving places breaks what option 1 keeps.

## Decision outcome

Option 1.

- **One act, given both folders.** The host, having moved the clone, tells flume the folder it was and the folder it is. flume renames each live place's repository, the intents and base places of the repository's bases, and the links between the clone and each place's worktree, through the place profile, so each place works against the clone where it now is and nothing is read under the former folder again (209).
- **No place moves.** Each place stays at its location and keeps its id (record 0027). A place made after the rename is made under the repository's id as it is now. An agent started before the rename keeps its identity and its stage.
- **Refused when it cannot be right.** The act changes nothing where the folder given holds no clone whose worktrees the places are, or where the former folder still holds a clone, since then there are two copies and no rename. Done again, it changes nothing. Its answer and status name each place it renamed.
- **The host's part stays the host's.** flume moves no clone and changes no configuration of the host's that names the folder, such as a stream mapped to the repository.

### Consequences

- One unit in flume, once this record is accepted, citing clause 209.
- On Boomdrive's hosts, the fleet that renames a clone runs flume's act after it, and the runner's report names the clone's new folder (boomdrivedev/boomdrive, unit `the-runner-reports-each-repository-it-keeps-and-its-clones-folder`).
- sluice keeps a repository's records inside its git directory, so they move with the clone; whether anything sluice keeps on the host names the former folder is sluice's to read, at its own change.

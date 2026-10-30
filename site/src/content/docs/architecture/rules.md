---
title: Rules
description: The rules that bind every change to the repository, each with an id a review can point at.
sidebar:
  order: 2
---

A mirror of the rules in
[docs/architecture/core.md](https://github.com/agentplot/flume/blob/main/docs/architecture/core.md),
which is the binding copy. The rules are written so a reviewer can point
at a diff and say which rule it breaks. The narrative is the statement
in agentplot/blueprints, `design/flume/requirements.md`; a rule names
the clause it enforces and never restates it. The binding copy gives
each rule's details: what it rules out, what it leaves open, and its
sources. Where the code does not yet keep a rule,
[docs/architecture/_open.md](https://github.com/agentplot/flume/blob/main/docs/architecture/_open.md)
says so, and the [roadmap](/flume/roadmap/) names what is decided and
not yet built.

<!-- The rules below are the binding copy's words, mirrored as written. -->
<!-- vale off -->

## Rules

### The core and its seams

- **[core.1]** Code outside a profile names no worktree tool, change tool, agent program, terminal or merge queue.
- **[core.2]** Every seam of Part B is reached through exactly one interface, and a profile implements that interface and nothing of the core.
- **[core.3]** A repository's profiles are read from its `[profiles]` table and from nothing else; a seam the table leaves unbound is the no-tool profile, or unbound, and never guessed from another tool's files.
- **[core.4]** flume defines no field of the kind format: the format is weir's, and flume reads the part clause 23 names and ignores every section it does not know.
- **[core.34]** A kind from any source but flume's own, the repository's and a launch's runs only as a lock records it, its content's hash read at each launch and the launch refused where it differs; a kind's name and version resolve as clause 154 orders and never by a guess between sources; flume fetches nothing and writes no lock at a launch or a settle; and a kind refers to nothing outside its package and names nothing a host owns.
- **[core.29]** No call to herdr is made without a bound, and one that does not answer within it is read as unknown or a refusal in herdr's words; every command and field a profile reads of herdr is checked against the herdr the tree pins.

### Branches, submissions and landings

- **[core.5]** No code path commits, merges, rebases, pushes or lands in a place, and the only branch flume moves is a place's own, at its launch and at its tidy.
- **[core.6]** A place is submitted only by the settle that follows its last stage's delivery, through the merge-queue seam, to its base and to no other branch, with no wait held on it; its outcome is read from the merge queue's record by the settles that follow, never from an agent, and the merge queue removes nothing of the place.
- **[core.7]** A place's branch is cut from the base's current revision where the merge queue's profile says the base lives, and a launch that cannot read it is refused.
- **[core.8]** A launch is refused whole, leaving nothing it made, when anything of it already stands or its kind asks for more than flume runs.

### Stages and agents

- **[core.9]** A stage's end is read from its deliverable at one head of the place's branch, or from a stop-short carrying the start's own secret; no code reads an end from a terminal's state or an agent's exit.
- **[core.10]** Each start of a stage, each repetition of a repeating stage among them, starts exactly one fresh agent, a place has at most one agent at any moment, every agent carries an identity no other live agent on the host carries, an agent is its stage's only on its own word, or, where its profile reads no word of its program's, its launcher's, that it started under that start's identity and the settings it recorded, its mode among them where they give one and its profile reads it back, and no agent that is not quiet is ended unless a caller forced it. (Amended 2026-10-30 by record 0024, proposed on the user's card, and by record 0026, proposed; accepted only on their yes.)
- **[core.11]** A prompt or a tell is sent only to a waiting agent that has said it started, any input it holds typed cleared first, read back as received within 60 seconds or sent once more, and never left typed for a hand; a notice is sent only to a recipient that asks no question, and counts delivered on the terminal's answer.
- **[core.12]** flume answers on an agent's behalf only the question whether it may work in the place, and, in a stage that delivers a plan, declines the agent's ask to leave its planning mode; it never answers one that widens what an agent may do, and never accepts a plan. (Amended 2026-10-30 by record 0024, proposed on the user's card; accepted only on their yes.)
- **[core.30]** An agent's settings are resolved at its start from the six layers clause 32 orders and from nothing else, the host's configuration as it stands then and every other layer as the launch read it, each setting taken whole; its mode is one of those settings, one of flume's four or a program's own value under that program's name, which its profile maps to exactly that setting or refuses, but a plan stage's, which is `plan`, and none is given where no layer gives one, the account's own configuration deciding; its model is its program's own name, passed as written; where no layer names its program and it has a pool, its program is its account's; its environment is the login environment with its account's variables over it, and nothing else gives one; and status names where each setting and each variable came from. (Amended 2026-10-30 by record 0024, proposed on the user's card, and by record 0026, proposed; accepted only on their yes.)
- **[core.31]** A repetition's prompt differs from its stage's other repetitions' only after every part they share: flume adds nothing between a prompt's parts, and flume's own repeating prompts put the group, the notes' path and the words last; flume names a stage's notes by their path and never reads, writes or carries what they hold.
- **[core.32]** An account is chosen at each start of an agent whose stream or kind has a pool, under the room lock: the intent's affinity while that account is usable, else the pool's strategy; a spent account is recorded in the host's state until its reset and skipped by every flume process, its agent's stage started afresh on another account with no caller's word and no going-back counted; and every move of a stage or an intent between accounts is said in status with why.
- **[core.33]** An agent's standing prompt is the goals, the intent's brief, the stage's standing instructions and its settings' instructions, in that order, each as given and joined by one fixed separator; nothing that differs between starts is in the first three, no part is filled in them, and its prompt holds the unit's brief and facts and then its run's; a kind gives only the standing instructions, and the goals and the briefs come from the caller or the repository.
- **[core.37]** A stage that delivers a plan starts in its program's planning mode, ends only on its caller's finish once a plan was put for review, and keeps its agent live while it is for review, so a tell reaches the same session; the finish accepts the newest plan, kept as it stood in the place's own folder for the stages after it. (Proposed 2026-10-30, record 0024, on the user's card; accepted only on their yes.)
- **[core.39]** Every program the host's terminal starts runs under an agent profile, its own or the common one; each part of the agent seam is built for the program from its own record or degraded as its profile declares, never read from the terminal where its profile builds it and never otherwise than clause 171 says, and status names each degraded part with the agent; a profile's entry for its program is flume's own code, checked against the release flume pins; and a mode, a model or a plan its profile cannot honour refuses the launch, or the stage at a later start, naming the program. (Proposed 2026-10-30, record 0026; accepted only on the user's yes.)

### Settling, surfaces and state

- **[core.13]** flume starts no process that outlives a command of its own, and no command of flume's waits on the merge queue: every settle runs inside a command, started by an agent going quiet, by the terminal's watch or by a caller.
- **[core.14]** Every surface answers from one core through one form a program reads, which only ever gains fields, and no surface computes a stage's end, a status, a submission or a tidy of its own.
- **[core.15]** Status reads every fact of a deliverable, a change or a submission where it lives when asked, and flume keeps no record of one between asks.
- **[core.16]** A place's live state is one document on the host, written by compare-and-set on its version, gone when the place is tidied, and nothing of flume's is written into a repository's content.
- **[core.17]** Every act is safe to repeat, and no act leaves a worktree or a branch of flume's that its held state does not name.
- **[core.18]** flume removes only places it made, through the repository's teardown, never a place holding uncommitted or unignored work or a lock, and never a branch holding a commit whose change its base lacks.
- **[core.23]** What a stage made is read from the line the stage's run wrote and carried as written, a durable thing by its address and a thing that must run by the branch and revision that bring it up; flume builds, checks or rewrites no address of its own, a stage starts no server that outlives its place, and a declared thing with no line is a deliverable not yet made.
- **[core.24]** A place waits on its caller only as status says, which move and since when, a hold carrying its caller's words whether put at the launch or after it; a place waiting under no hold's words for five minutes, or the launch's bound, is said stalled and each recipient of it is delivered that once; flume watches no event for a hold and ends none itself.
- **[core.25]** A stage's end or a submission's outcome is delivered once to each recipient of the place, inside the settle that reads it, as a notice through the terminal into the recipient's own pane, and recorded on the place as delivered, to whom and when; a wait is by place, answers a pending end at once and otherwise follows the host's state under a timeout, so a caller whose session restarted loses nothing; a recipient collects what is pending for it by `flume inbox`, and a read marks nothing delivered.
- **[core.26]** A stage starts only while the host's agents running, every caller's counted together, are under the bound the host's configuration states, its stream's are under the stream's, an account of its pool is usable, and, for a place's first start, its stream's points bound and pace leave room for its points, and otherwise waits for room; a settle starts the waiting stages by their streams' priority, then their places', as many as there is room for, passing over one whose stream or pool has no room, and ends no agent for one; a place's priority is its caller's, given at the launch and changed after, a stream's the host's, and flume orders places by nothing else; every launch is taken and none refused for want of room; status says the agents running, the bound, the room and the queue; a host that states no bound of its own and no stream's bound has no queue. (Amended 2026-10-30 by record 0025, proposed on the user's card; accepted only on their yes.)
- **[core.28]** A recipient may be a role's address; an end pending for an address with no run live or waiting, or a carrier's wake, launches the kind the host's configuration names for the address, at most one run of it at once, and flume decides nothing of what the run does.
- **[core.27]** A kind runs through on its own: a stage's delivery starts the next stage and the last stage's delivery submits the place, with no caller's word unless the launch or a hold asks for one; a failed or conflicting submission, and a stop-short that found work from the kind's last stage, start the stage the kind names for a failed submission with the merge queue's or the agent's words, counted together toward the kind's bound; a stop-short says whether it needs an answer or found work, status says which with the words, and a needed answer is given only by a retry whose prompt carries the question with the answer.
- **[core.36]** flume reads no base's checks and launches no place of its own on a base's failure: a fix of a base whose own checks failed after a landing is a fix place a caller launched on that base with the failure's words as its brief, cut from the base's current revision and submitted to it through the merge queue, its submission held where the base's landings wait on a word the merge queue's policy does not ask, and nothing else moved for it.
- **[core.38]** A place's points are its caller's, a number of the Fibonacci scale fixed at the launch, which flume never estimates, changes or counts by a guess; they measure its agents' work, in flight from its first start until its last stage delivers and again through a fix round; a stream's points bound and its pace hold first starts alone, the pace over the rolling span that ends at each start and charged in the host's own documents; and machine time and tokens are reported beside the points and never limited. (Proposed 2026-10-30, record 0025, on the user's card; accepted only on their yes.)

### Design and records

- **[core.19]** A change that bears on a clause cites the clause by number; a clause that no longer holds is changed in the statement first, never worked around here.
- **[core.20]** Nothing commercial, no pricing, tier or customer, appears in this repository.
- **[core.35]** The site, the README, the plugins' commands and the commands' help and output speak to any host's operator: they name no tool, team, host or word of ours as the reader's own, and name a tool only as a product flume integrates with.

### The gate

- **[core.21]** Every file the tree tracks is owned by a formatter the gate runs, or is excluded by name with its reason; every crate, every directory under `plugins/`, and `site/`, is a moon project of its own, listed in `.moon/workspace.yml` with its language declared, whose check-fast tasks validate, lint and test it, one task per check.
- **[core.22]** Every tool a gate task runs is the version the tree pins, on every host that runs the gate.

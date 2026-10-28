# flume rules

The rules that bind wherever a change is made in this repository,
written so a reviewer can point at a diff and say which rule it breaks.
The narrative is the statement in agentplot/blueprints,
`design/flume/requirements.md`; a rule here names the clause it
enforces and never restates it. There are no area pages yet; a rule
that binds only one part of the tree moves to an area page when that
part exists.

## Rules

### The core and its seams

- **[core.1]** Code outside a profile names no worktree tool, change tool, agent program, terminal or merge queue.
- **[core.2]** Every seam of Part B is reached through exactly one interface, and a profile implements that interface and nothing of the core.
- **[core.3]** A repository's profiles are read from its `[profiles]` table and from nothing else; a seam the table leaves unbound is the no-tool profile, or unbound, and never guessed from another tool's files.
- **[core.4]** flume defines no field of the kind format: the format is weir's, and flume reads the part clause 23 names and ignores every section it does not know.

### Branches, submissions and landings

- **[core.5]** No code path commits, merges, rebases, pushes or lands in a place, and the only branch flume moves is a place's own, at its launch and at its tidy.
- **[core.6]** A place is submitted only by the settle that follows its last stage's delivery, through the merge-queue seam, to its base and to no other branch, and its outcome is read from the merge queue's record, never from an agent.
- **[core.7]** A place's branch is cut from the base's current revision where the merge queue's profile says the base lives, and a launch that cannot read it is refused.
- **[core.8]** A launch is refused whole, leaving nothing it made, when anything of it already stands or its kind asks for more than flume runs.

### Stages and agents

- **[core.9]** A stage's end is read from its deliverable at one head of the place's branch, or from a stop-short carrying the start's own secret; no code reads an end from a terminal's state or an agent's exit.
- **[core.10]** Each stage starts exactly one fresh agent, a place has at most one agent at any moment, every agent carries an identity no other live agent on the host carries, and no agent that is not quiet is ended unless a caller forced it.
- **[core.11]** A prompt or a tell is sent only to a waiting agent, read back as received within 60 seconds or sent once more, and never left typed for a hand.
- **[core.12]** flume answers on an agent's behalf only the question whether it may work in the place, and never one that widens what an agent may do.

### Settling, surfaces and state

- **[core.13]** flume starts no process that outlives a command of its own: every settle runs inside a command, started by an agent going quiet, by the merge queue's verdict or by the terminal's watch.
- **[core.14]** Every surface answers from one core through one form a program reads, which only ever gains fields, and no surface computes a stage's end, a status, a submission or a tidy of its own.
- **[core.15]** Status reads every fact of a deliverable, a change or a submission where it lives when asked, and flume keeps no record of one between asks.
- **[core.16]** A place's live state is one document on the host, written by compare-and-set on its version, gone when the place is tidied, and nothing of flume's is written into a repository's content.
- **[core.17]** Every act is safe to repeat, and no act leaves a worktree or a branch of flume's that its held state does not name.
- **[core.18]** flume removes only places it made, through the repository's teardown, never a place holding uncommitted or unignored work or a lock, and never a branch holding a commit whose change its base lacks.
- **[core.23]** What a stage made is read from the line the stage's run wrote and carried as written, a durable thing by its address and a thing that must run by the branch and revision that bring it up; flume builds, checks or rewrites no address of its own, a stage starts no server that outlives its place, and a declared thing with no line is a deliverable not yet made.
- **[core.24]** A place waits on its caller only as status says, which move and since when, a hold carrying its caller's words whether put at the launch or after it; a place waiting under no hold's words for five minutes, or the launch's bound, is said left and whoever waits on it is told once; flume watches no event for a hold and ends none itself.
- **[core.25]** A stage's end or a submission's outcome is told once to each waiter on the place, inside the settle that reads it, through the terminal into the waiter's own pane, and recorded on the place as told, to whom and when; a wait is by place, answers an untold end at once and otherwise follows the host's state under a timeout, so a caller whose session restarted loses nothing; flume lists a caller's untold ends when asked and marks nothing told by a read. (Proposed 2026-10-27, record 0009, on the user's card; accepted only on their yes.)
- **[core.26]** Status says, for the host, how many agents of flume's places are running, every caller's counted together, and, where the host's configuration states how many agents it runs at once, how many more it has room for; a place waiting with no agent counts nothing; flume holds no cap and refuses no launch for want of room, telling a caller once that it launched past it. (Proposed 2026-10-28, record 0010, on the user's card; accepted only on their yes.)
- **[core.27]** A kind runs through on its own: a stage's delivery starts the next stage and the last stage's delivery submits the place, with no caller's word unless the launch or a hold asks for one; a failed or conflicting submission, and a stop-short that finds from the kind's last stage, start the stage the kind names for a failed submission with the merge queue's or the agent's words, counted together toward the kind's bound; a stop-short says whether it asks or finds, status says which with the words, and an ask is answered only by a retry whose prompt carries the question with the answer. (Proposed 2026-10-28, record 0011, on the user's card; accepted only on their yes.)

### Design and records

- **[core.19]** A change that bears on a clause cites the clause by number; a clause that no longer holds is changed in the statement first, never worked around here.
- **[core.20]** Nothing commercial, no pricing, tier or customer, appears in this repository.

### The gate

- **[core.21]** Every file the tree tracks is owned by a formatter the gate runs, or is excluded by name with its reason; every crate, every directory under `plugins/`, and `site/`, is a moon project of its own, listed in `.moon/workspace.yml` with its language declared, whose check-fast tasks validate, lint and test it, one task per check.
- **[core.22]** Every tool a gate task runs is the version the tree pins, on every host that runs the gate.

## Details

- **core.1** Rules out: the strings `wt`, `worktrunk`, `openspec`, `claude`, `herdr`, `sluice` in any module outside a profile directory; an import of a profile from the core. Not ruled out: `git`, which section 9 of the statement fixes; `plugins/`, which no crate imports. Source: statement preamble, clauses 11, 105; record 0004.
- **core.2** Rules out: a profile calling another profile; a core function with a profile-specific branch. Source: clauses 105, 106; record 0004.
- **core.3** Rules out: reading `.config/sluice.toml`, `openspec/` or `.config/wt.toml` to choose a profile; a merge-queue profile assumed because sluice is on the host. Not ruled out: the sluice profile reading `.config/sluice.toml` for where the base lives and the readiness, once bound (C.1, C.6). Source: clauses 11, 20, 105, C.2; record 0004.
- **core.4** Rules out: a field name of the format defined in flume; a stage with several agents or several deliverables run as if it had one; a declaration refused for a section flume does not know. Source: clauses 19, 23, section 9; weir A.8; record 0004.
- **core.5** Rules out: `git commit`, `merge`, `rebase`, `push`, `update-ref` on any branch but the place's own at `make` and `remove-branch`; a tidy that deletes a branch holding a commit its base lacks. Source: clauses 14, 67, I3, I4; record 0002.
- **core.6** Rules out: a submit from a stage; a submit of a place whose kind does not land; `--no-wait`, `--check` or `--no-remove` passed to the merge queue; landed read from the base's history or an agent's words. Source: clauses 15, 63, 71, 111, 113, 124, I7; record 0002.
- **core.7** Rules out: a place cut from the local `main` of a repository whose policy lands main on a remote; a launch that goes on after a failed fetch. Source: clauses 9, C.1.
- **core.8** Rules out: a launch that makes a worktree and then refuses; adopting a worktree, branch or change flume did not make. Source: clauses 2, 3, 4.
- **core.9** Rules out: a stage ended because its pane is idle, its turn ended or its process exited; a stop-short taken without the start's secret or from another place's agent; a deliverable's facts read at two heads. Source: clauses 25, 39, 41, 45, I1; record 0003.
- **core.10** Rules out: an agent handed from one stage to the next; two agents in one place; an identity reused while its holder is live; `close` on a working agent without `--force`. Source: clauses 30, 31, 33, I2; record 0003.
- **core.11** Rules out: text typed into a pane and left for a hand; a prompt sent into an open question; a third sending. Source: clauses 37, 38, 53, 54, I5.
- **core.12** Rules out: answering a permission prompt that widens an agent's reach, or the warning about bypassing permissions. Source: clause 34, C.4.
- **core.13** Rules out: a daemon, a timer or a background loop of flume's; a settle outside a command. Not ruled out: the herdr plugin's watch on herdr's server, which runs `flume settle`. Source: clauses 46, 119, 120, 122, I8; record 0005.
- **core.14** Rules out: a plugin that computes a stage's state from the pane; two surfaces that answer differently; a field removed from the JSON form. Source: clauses 60, 61; record 0005.
- **core.15** Rules out: a cached task count, head or submission state answered without reading; an answer about a change that disagrees with the change's own reader at the same revision. Source: clauses 58, 59; record 0006.
- **core.16** Rules out: a write without the version read; a flume record in a repository's tree or on a branch; state that outlives its place. Source: clauses 62, 73, 75, 103, 104, C.7; record 0006.
- **core.17** Rules out: an act whose repeat changes something; a worktree made before its held state exists. Source: clauses 74, 76.
- **core.18** Rules out: `git worktree remove --force`; removing the repository's main worktree; removing a place whose agent is live. Source: clauses 64 to 68.
- **core.19** Rules out: a code comment that says the statement is wrong and goes on; a profile that satisfies a clause by reading it loosely. Source: record 0001; `design/requirements-style.md`.
- **core.20** Rules out: pricing, a tier or a customer named in code, docs or the site. Source: record 0001; agentplot/strategy's `CLAUDE.md`.
- **core.21** Rules out: a tracked file treefmt neither formats nor excludes; a crate or plugin outside `.moon/workspace.yml`; a check run by a script a task does not name. Source: `.moon/workspace.yml`, `treefmt.nix`, `.config/sluice.toml`; kit-standard's code profile.
- **core.22** Rules out: a tool from the machine's own installation in a task; a version floated by a range. Source: `devenv.nix`, `.prototools`.
- **core.23** Rules out: a link to a branch, a preview or a page computed from a name and a forge's or a server's layout; an address rewritten, shortened or resolved before it is handed on; a stage read as delivered while a declared made thing has no line; a made thing's lines kept by flume after the place is tidied; a line naming the address of a server the stage started, which nothing holds once the stage ends; a daemon of flume's keeping a made thing alive for a card. Not ruled out: carrying a thing the run recorded and the kind did not declare; the caller keeping the lines it was given. Source: clauses 23, 43, 44, 110, 128, 129; record 0007; sluice record 0045.
- **core.26** Rules out: a slot, a cap or a count of places in flume; a launch refused because the host is full; a bound flume computes from the machine, the seat or sluice's run bound; a count that includes a place whose stage waits with no agent; a count of one caller's agents alone. Not ruled out: a caller that reads the room and launches nothing until there is some, crew among them; a host whose configuration states no bound, for which status gives the count and judges no room; sluice's own run bound, which counts runs and is sluice's. Source: clauses 54, 56, 58, 70; C.7; record 0010; crew record 0045. (Proposed 2026-10-28, record 0010.)
- **core.27** Rules out: a stage whose start waits on a caller's word that neither the launch nor a hold asked for; a failed or conflicting submission that leaves the place waiting for a caller to start the fix; a finding from the kind's last stage left waiting for a retry; a finding or a failed submission sent to a stage that cannot deliver on it (change@1's verify as the send-back stage); goings-back counted apart so a place goes back more than its kind allows; a stop-short whose words status shows without saying whether it asked or found; flume reading a tell, a card or a pane for an answer; a tell taken as the retry that restarts an ended stage; a conversation agent of flume's deciding the next step. Not ruled out: a caller that holds a stage or a submission for its word, S7; a caller that drops or retries a place past its bound; crew turning a tell or a card's answer into the retry; a finding from a stage that is not the last, taken as an ask. Source: clauses 22, 40, 41, 42, 47, 49, 56, 111, 112, 130, 131, C.2, C.4; records 0003, 0008, 0009, 0011; afterthought/crew records 0045, 0047. (Proposed 2026-10-28, record 0011.)
- **core.25** Rules out: a caller that learns an end only by polling status; an end pushed by a process of flume's own between its commands (120); an end told to a person by flume itself (72); a wait that drives a settle where the terminal keeps watch; a wait keyed by a process, a pane or a session rather than the place, which a restart would lose; an end marked told because status or the untold list was read; an end told twice to one waiter. Not ruled out: the settle that tells running inside the terminal's watch or a caller's command (119, 120); a caller that registers its wait and is told at the next settle; a caller whose own hook asks for its untold ends at its start, crew's session-start hook among them; the printed-command terminal, where the end is read from status since there is no pane to tell. Source: clauses 43, 46, 56, 72, 95, 119, 120, 121, 122, 129; records 0005, 0006, 0009; sluice record 0068; crew record 0046. (Proposed 2026-10-27, record 0009.)
- **core.24** Rules out: a hold kept in a caller's memory or a conductor's note with nothing in flume's held state; a waiting place shown without since when, or a hold without its words; a left place said to nobody, or said to a person by flume itself (clause 72); flume polling a card, a tell, a landing or another place for a hold's event, or starting a held stage when it judges the event has come. Not ruled out: the caller's own wait on the event, crew's wake on a card or a tell among them, ended by the retry that starts the held stage; a hold whose words name an event flume cannot see; a place ended short, whose agent's words are its reason. Source: clauses 8, 42, 43, 47, 56, 70, 72, 130; record 0008; crew record 0044.

---
number: 69
title: A stage whose kind hibernates it sleeps once its prompt cache no longer pays and resumes its own session at a message, and every message to a role waits at the role's address
status: accepted
date: 2026-11-14
decision-makers:
- Chuck Swanberg
---

# A stage whose kind hibernates it sleeps once its prompt cache no longer pays and resumes its own session at a message, and every message to a role waits at the role's address

Status: accepted by the user on 2026-11-14, on their card. Drawn from their own words and choices of 2026-11-11 and 2026-11-12 in the talk `boomdrivedev/blueprints:design/research/2026-11-11-a-session-sleeps-and-wakes.md`, where it was drafted under the number 0065. Its statement is flume's clauses 212 to 215 and their amendments, merged to boomdrivedev/blueprints' main on the user's word of 2026-11-13.

Deciders: the user, the operator in flume's words. Their words: "Call it hibernate and resume, and keep
it separate from the standard state machine concept. They're two different
ideas." "certain flume kinds will allow hibernation and resume while others
won't." "You're keeping it open as long as you're benefiting from the cache
and once you can't benefit from the cache anymore, you might as well
hibernate it." "if it's stuck waiting for an answer, it should be allowed to
hibernate and then be woken up when the answer comes". "flume can launch
these on demand but can also be configured with predefined standing
agents." On 2026-11-12, of a role's work: "I feel like that will fit a lot
better if those sub-agents are launched through flume", "launched as
related flume kinds, similar to ops", so that "the user would be able to
see everything in the multiplexer the same way they always would." Of
kinds a role's entry does not name: "I don't think we care to enforce
them". Of credentials: "we need to kick the can on this and assume that
the agents are running in environments I'm setting up ahead of time." Of
repositories: "In an ideal world we're not linking a repo to a standing
agent. They should be able to work against more than one", and "why not
just have it work one way for all of them with no special
consideration?" Of the word: "these are agents that we're defining", and
"I don't think we have to call them standing agents." Their
choices: the host keeps transcripts by `cleanupPeriodDays`
and flume keeps no copy; a message to a hibernating session resumes it;
every message to a role waits as a delivery at its address; a delivery
reaches a live session by its program's own channel, a Claude Code mod
first; the verb is `message`, replacing `tell`; flume's MCP server messages,
wakes and reads a role; a role's
label is its room and its column; a standing role is a session flume keeps,
defined by a kind.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md`, merged to main at `786d040`,
clauses 7, 32, 42, 43, 49, 53 to 56, 109, 132, 179, 180, 193, 198, 212 to
215, C.2, C.4, C.5, sections 9 and 10, I5, S6, S17, S124 to S128; records 0003, 0016,
0020 with its amendments, 0039, 0044, 0052 and 0061; rules core.10, core.13,
core.30 and core.32; on deneb on 2026-11-11, every account home's
`projects` a link to `~/.claude/projects`, no home setting
`cleanupPeriodDays`, the oldest transcript there last written 2026-10-13,
one Keychain item per home, and Plannotator 0.27.22's mod handing its
replies by `$.prompt.submit`; Claude Code 2.1.322's `idleCompaction`
setting, its "Resume this conversation?" dialog and its refusal of
`cleanupPeriodDays` below one.

## Context and problem statement

A conversation stays in its herdr pane until its caller finishes it (109),
holding its room and its account all day, and the operator leaves design
talks open for hours. flume already lets an agent kept open for an answer go
after an hour, the life of its prompt cache (198), and carries a lost
conversation at its caller's retry (49, 179; record 0052), but a stop and a
retry are the stage's state machine: a retry after a stop starts afresh,
and "no other retry carries one" (core.32). crew's standing roles, measured
over 30.7 hours, re-read 230,000 to 320,000 tokens of context on every call
and are typed into from several sides at once, half-typed words in their
input boxes cleared by herdr's ctrl+c (C.5).

## Considered options

1. Sleep as a caller's stop, and wake as a retry that carries the
   conversation, loosening core.32.
2. Hibernate and resume as acts of their own beside the state machine, a
   kind declaring the stages they apply to, the idle time the prompt
   cache's life.
3. A caller's act only, flume never hibernating on its own.

For transcripts: the host sets `cleanupPeriodDays` once; flume keeps its own
copy and restores it before a resume; flume owns a configuration home per
account. For a message to a busy session: typed by the terminal as today;
a delivery at the address taken at the session's next wait, by its
program's own channel; the control plane's queue. For a role's own
projects: its program's subagents, as its prompting asks; places it
launches of kinds its entry names. For an agent's credentials: a role's
own layer of variables; variables a launch names; the environment the
operator sets up on the host ahead of time.

## Decision outcome

Option 2, as the operator ruled. A stage whose kind hibernates it is
hibernated once its agent has been quiet for its hibernate time, at its
caller's `flume hibernate`, or at the end of its agent's keep-open wait for
an answer. Its agent is ended and its session kept, and its room and its
account are freed. A caller's `flume resume`, a message, a role's delivery
or the answer resumes that session on the account it ran under (212,
213). The host keeps transcripts (C.4). Every message and end for a role is
a delivery at its address, handed as one prompt at the session's next wait
by its agent profile's deliver part, a mod for Claude Code, the terminal
the fallback (43, 54, 214, 215). A standing role is a role whose kind's
stage is a hibernating conversation (193). A role's entry names the
kinds its session launches its projects as, which its prompt gives. A place a role launches naming no recipient
reports to the role's address (7, 132), so its ends join the role's
deliveries and resume it, and status gives a standing role's live places
(56, 214). An agent launches one by the command line or flume's MCP
`launch`. A standing role's place holds a detached checkout of each
repository its entry names, moved to its base's tip at each resume, and
commits nothing; work that must commit goes to a place it launches
(193, C.7). An agent's environment stays its account's, which the operator
sets up with the credentials its work needs (32, core.30). flume refuses no
kind a role's entry does not name; the entry names each with when it
serves, since a role finds no kind it is not told of (193). `tell` is
`message` (53 to 55).

### Why not the others

Option 1 is what the operator turned down: a stop and a retry restart a
step of a state machine, and a conversation left open all day is another
idea. Option 3 leaves every conversation holding its room and account past
the time its cache pays. A copy of a transcript is a file in a format flume
does not own and two copies that differ (record 0020, option 3), and a
configuration home of flume's writes where core.32 forbids it. Typing into
a busy session interleaves tasks and clears what a person typed; the
control plane's queue would put a second queue beside flume's deliveries
(record 0061). A program's subagents end with the turn that started them,
hold the role's account, and show in no pane, where a project runs past a
turn; they stay the role's for a lookup answered within one. A role's or
a launch's own variables, as the operator put it, wait on a managed
fleet's design: they need a place to keep secrets, a resolver and a rule
for which an agent may pass on, which no host has yet.

### Consequences

- Rules core.10 and core.32 are amended with it: a resume is one fresh
  agent carrying its stage's own session, on the account clause 213
  names, and is no retry.
- The deliver row needs a Claude Code release that runs hooks modules: read
  on 2.1.322, and flume pins 2.1.296.
- Open: a resume's identity, and how a resumed Claude Code agent passes
  the "Resume this conversation?" dialog (section 10); how credentials
  reach an agent on a machine the operator does not set up; what a
  resume does with a file changed in a checkout; whether the agents are
  defined in a source's roles package or the host's configuration; the
  word for a standing role, which the operator wants to be "agent",
  where the statement gives that word to one process of one stage.
- Elsewhere: the host's setup sets `cleanupPeriodDays` in every home
  (boomdrivedev/boomdrive); crew's tells become deliveries, its roles move
  into a roles package, each entry naming its related kinds, and their
  instructions launch their projects as places (boomdrivedev/crew); the
  control plane shows each role under its label's column with its live
  places, and its MCP tools answer later when they wake a role
  (boomdrivedev/boomdrive-cloud).

## Construction choices

Dated 2026-11-14, the code stage's of the change
`every-message-to-a-role-waits-at-its-address`, named for what its
design (`openspec/changes/every-message-to-a-role-waits-at-its-address/design.md`)
builds of this record: a message, a role's deliveries, the deliver part
and a launch from a role's run reporting to its address. Each is the
construction agent's, within what the operator ruled above, and the
record stays accepted. Sources beside those above:
`boomdrivedev/blueprints:design/flume/requirements.md` at main 81288ba,
clauses 7, 43, 53 to 56, 121, 132, 193, 214 and 215, C.4's deliver
row and S126 and S128, and C.7's deliveries bullet as the branch
`design/flume-a-message-to-a-role-is-a-delivery-at-its-address`
amends it at 9ddd402, a message's delivery key and the claims kept
beside the records.

- **`tell` stays an alias of `message`** (design section 2). `flume
  tell` runs as `flume message`, hidden from the help, and an answer's
  act written `tell` reads as `message`. Not taken: removing `tell` at
  once, which would break every caller that tells a place before it
  moves; a later chore drops the alias once no caller uses it.
- **A message's target is an address or a place** (section 3).
  `<stream>/<role>`, each part a word and neither `.` nor `..`, is a
  role's address, and any other target names a place as every command
  does; a place's relative location of that shape is named by its
  absolute location or `./` before it. `--within` and `--answers` are
  refused with an address. Not taken: a flag of its own for an address,
  `--to`, where an address is the recipient's own name everywhere else.
- **A message to an address is pending with no bound** (section 4). It
  is recorded through weir's `record`, keyed
  `message/<stream>/<role>/<token>`, its one reader the address, its
  first line naming who sent it, expiring after `[host] ends_expire`; its
  own act wakes the address's run as a settle does, and answers at once.
  Not taken: a message to an address that waits within a bound, as one
  to a place does, which would refuse every message sent while a role's
  run is woken or waits for room.
- **A batch is claimed beside the host's deliveries** (section 5). A
  recipient's batch, every delivery pending for it and handed by
  nothing, is claimed whole in one write of the host's deliveries
  document, sent while the recipient waits, and recorded handed with the
  claims removed in one write; a claim left sending holds its items out
  of later batches, which the inbox still gives. Not taken: a notice of
  each end apart, which types two prompts where one would do; claims on
  each place's document, which leave a message, of no place, nowhere to
  be claimed.
- **Claude Code's deliver part is built and not offered** (section 7).
  The mod `hooks/deliver.ts` runs `flume hook deliver` at a turn's end
  and on an idle clock, gated by `FLUME_DELIVER`, and submits what it
  prints as the user's words; the take records the hand-over before the
  mod submits, since the hook cannot learn whether the submit took, and
  the inbox gives again what was handed and not confirmed. Claude Code's
  profile answers deliver not offered while flume pins 2.1.296, which no
  check of the tree reads running a hooks module's `$.prompt.submit` in
  an interactive session, so the terminal hands every batch until the
  chore that moves the pin turns it on. Not taken: offering it on the
  pinned release unread, against core.39; a module that asks at every
  idle tick in every session, which would start a process every ten
  seconds in every agent for a part no pinned release offers; the framed
  submit, which reads "The flume plugin sent a message" where the
  terminal types the words bare.
- **A role's run's first prompt carries its batch** (section 9). Each
  start of a role's run fills its prompt with its address's batch after
  the kind's template and before the way to stop short, claimed under
  the start's identity and recorded handed once the prompt is received;
  a start that never began lets its claims go. Not taken: handing the
  batch at the run's first wait after its prompt, which costs the run a
  turn on its own prompt alone.
- **The suite cites the clauses its list holds** (section 13). The
  engine's new scenarios cite clauses the suite's list holds at 31f9f1c
  and give 214 and 215 in their names' words; the chore that moves the
  list past 210 finds them to cite. Not taken: moving the list's
  revision here, which would list fourteen clauses and eleven scenarios
  that four changes on this bolt build.

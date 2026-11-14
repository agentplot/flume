# Design

## Context

### What is decided

Record 0013's amendment of 2026-11-14, boomdrive-design's, is the
design. Where a command names no recipient, `$FLUME_RECIPIENT` names
none and the command runs in no role's run, flume asks the terminal
which agent the command runs in. Where a source declares that agent, by
its name in that session, standing for an address, the command acts as
that address: a confirm, a collect and a wait as that recipient, and a
launch with it as the place's recipient (7). Anywhere else it acts as
the terminal's session. An agent its sources declare standing for more
than one address acts as none by itself: its command is refused, naming
each address, and asks for `--recipient`. `--recipient` and
`$FLUME_RECIPIENT` keep their place ahead of the reading. Its rejected
options were a `$FLUME_RECIPIENT` set by whoever starts the agent, and
every standing agent writing `--recipient` on each command.

Clause 132 says it on boomdrivedev/blueprints' branch
`design/a-standing-agent-acts-as-its-address` at d008a1d: "A command
that names no recipient, run in the standing agent's pane, acts as the
address the agent holds, as a run's command acts as its own (7); an
agent standing for more than one address names which with each
command." Rule core.28 takes it, and its details rule out such a
command acting as the pane, or as one of several addresses.

No clause is added or changed by this change. The comparison it makes
is the core's, between two things each terminal profile already gives
(C.5): the session a command runs in, which is a launch's default
recipient (7), and the session name `agents_in` gives each named agent,
which a notice reaches it by (132).

### What stands in the code

On the bolt `bolt/flumes-places-need-no-hand`, this place's base:

- **The recipient a command acts as.** `recipient_of`
  (`crates/flume/src/commands/mod.rs`) takes `--recipient`, else
  `Engine::recipient` (`crates/flume-core/src/wait.rs`):
  `$FLUME_RECIPIENT` as the host gives it, else the role of the place
  `$FLUME_PLACE` names where that place is a role's run, else
  `caller_session`, the session the first terminal the host binds names
  for the command. `flume wait`, `flume inbox` and `flume confirm` take
  it, refused naming `--recipient` where none is found; `answering`
  records it as the `by` of a retry's or a tell's answer, none where
  none is found.
- **A launch's recipients.** `recipients_of`
  (`crates/flume-core/src/launch.rs`) takes the launch's, else
  `$FLUME_RECIPIENT`, else `caller_session`, else the answerer behind
  the place's stream, recording `RecipientsFrom` (`flume-seams`'s
  `held.rs`) with the place.
- **A standing agent.** `standing.rs` reads the sources' standing
  entries once per delivery or status (`Standing::of`). `reach` finds
  an address's agent in its stream's terminal (`stream_terminal`) and
  its entry's session, else the stream's own, asks that part's
  `agents_in` once and counts the agents of the entry's name: one, at
  its `at`, none, several, or unread.
- **The terminals.** herdr's `session()` is the pane `HERDR_PANE_ID` on
  `HERDR_SOCKET_PATH` names, `herdr:<pane>@<socket>`, and its
  `agents_in` gives each named agent at `herdr:<pane_id>@<socket>`, the
  socket the session listing gives. Rex's `session()` is
  `rex:<block>@<socket>` from `REX_BLOCK` and `REX_SERVER`, and its
  `agents_in` gives each labelled block at `rex:<block>@<socket>`. The
  printed command names no session and gives no agents. The fake
  terminal's `set_session` names the command's session, and
  `add_agent` answers the key it lists the agent at.

### Rules that bind

core.md binds the change; no area page exists. It relies on core.1 and
core.2 (the core names no terminal and compares only what the terminal
seam gives), core.8 (a launch refused makes nothing), core.13 (nothing
kept or watched for the pane), core.14 (the form gains a variant of
`recipients_from` and loses nothing), core.15 (the terminal read at the
command, nothing kept), core.25 (a recipient's confirmation is taken
from the caller that acts as it, and flume confirms for none itself),
core.28 (the rule this builds), core.29 (no new call to herdr or Rex;
`agents_in`'s bounded calls), core.35 (the site names no host, team or
word of ours), core.49 (the agent sought in its stream's terminal) and
core.54 (an answer's giver as the answering command's recipient names
it).

## Goals / Non-Goals

**Goals:**

- A standing agent confirms, collects and waits for the ends handed to
  it with no flag, and a place it launches names its address.
- A pane standing for two addresses says so and names them, and does
  nothing as either.
- A command from any other pane, or one that names its recipient, acts
  as it does now, and asks the terminal for nothing more where no
  standing agent could match.

**Non-Goals:**

- The notice's words. It names `flume confirm --recipient <address>`,
  which still delivers (record 0061's amendment of 2026-11-12).
- SessionStart in a session flume did not start. It collects as the
  session the terminal names, never as a role's address (C.4), and C.4
  is not amended.
- A launch from a role's run. It takes `$FLUME_RECIPIENT`, else the
  session it runs in, as now.
- An identity guarding a standing agent's name. flume did not start the
  agent and trusts its source's name (record 0013, Consequences).

## Decisions

### 1. The agent a command runs in is the one its address's delivery reaches

For each address the sources give a standing agent, the core reaches its
agent as a delivery does (`reach`): in the address's stream's terminal,
in the entry's session, else the stream's own, by the entry's name. An
address whose reach finds one agent, at `at` on the terminal `T`, is
held by the command's pane where `T.session()` is `at`. The comparison
is of the two strings the terminal seam gives, so the core names no
terminal (core.1, core.2).

Not taken: matching the pane's agent by its name alone, where the
session lists two of that name. The delivery reaches neither of them
(132), so the command would act as an address whose ends it was never
handed, and two panes would confirm as one address. Each pane of two
agents of one name acts as its own pane instead. Not taken either: a
terminal operation giving the name of the agent a command runs in.
`agents_in` already lists the panes by name, and one more operation
would be one more call to build, bound and check in every profile for
nothing `agents_in` lacks.

### 2. The order of a command's recipient

`Engine::recipient(here)` answers, in order:

1. `$FLUME_RECIPIENT`, as the host gives it;
2. the role of the place `here` names, where it is a role's run;
3. the address the standing agent the command runs in holds, where the
   pane holds exactly one; a refusal where it holds several;
4. `caller_session`, as now;
5. none.

It answers `Result<Option<String>, String>`: the refusal of step 3 is
`Err`, naming the agent and each address, sorted, and asking for
`--recipient`: "the agent `east-planner` this command runs in stands
for `east/ops` and `east/planner`: name which with --recipient". The
command layer's `recipient_of` passes `--recipient` first, as now, and
gives the refusal as the command's, exit 1. `answering` records as the
answer's `by` what `recipient_of` finds, and none where it is refused,
so a retry or a tell is never refused for its giver: neither takes
`--recipient`, and its answer's words and key are what the place waits
on (42, 49).

### 3. A launch's order

`recipients_of` takes the launch's recipients, else `$FLUME_RECIPIENT`,
else the standing agent's address, else `caller_session`, else the
answerer behind the stream. A pane standing for several addresses
refuses the launch before anything is made, in the words of decision 2
naming `--recipient` (core.8). A role's run's own launch, given its
`role`, still takes no recipient.

The place records `RecipientsFrom::Standing { agent, source }`,
serialized `{"from": "standing", "agent": …, "source": …}`, the agent's
name and the source that declares it standing for the address. The
person's form gives it as "from the standing agent `<agent>` that
launched it, declared by `<source>`". A held state written before reads
as it was (61).

### 4. Read only where something could match

The reading of decision 1 is made only after steps 1 and 2 found
nothing, only where the sources declare a standing agent, and only
where a terminal the host binds names a session for the command. It
reads the sources' roles once and asks each terminal and part for its
agents once, through one `Standing`, and keeps nothing (core.15). An
address whose stream's terminal names no session for the command is
passed over without asking it, so a command from a herdr pane asks no
Rex part, and one from a shell asks nothing.

### 5. A part that cannot be read is passed over

Where an entry's part answers unread, or its stream is not declared,
that entry holds no pane, and the command acts as the other entries or
step 4 give. Not taken: refusing every command that names no recipient
while any standing entry's part cannot be read, which would stop a
person's `flume inbox` in an unrelated pane for a terminal that does not
answer. Status already names the address and why its part was not read
(132).

### 6. The terminals' caller panes

The comparison holds only where a profile names the command's pane in
the form its `agents_in` gives the agent in that pane. Each profile's
tools tests show it against its pinned release: under herdr, a command
run in a pane of a named session names its pane on the socket
`agents_in` gives that session's agents; under Rex, a command run in a
labelled block names the block as `agents_in` gives it. Where the
pinned release gives the socket in another form, the profile gives its
caller's pane in its `agents_in`'s form, and changes nothing it sends
(C.5; core.29). The printed command names no session, so a command
under it is never a standing agent's.

### 7. Record 0013 and core.28

Record 0013 gains "The construct's reading, 2026-11-14", under its
amendment of that day, giving decisions 1 to 6 in short with the paths
not taken. Rule core.28's details gain what they leave open: a command
in the pane of one of two agents of a standing agent's name acting as
its pane, and one whose standing agent's part cannot be read acting as
the other entries or its session give. No rule's line changes.

### 8. The site

`guides/roles-and-wake.md` says, in "An address an agent already
holds", that the agent's own `flume confirm`, `flume inbox`, `flume
wait` and `flume launch` act as its address, and that an agent standing
for two names which. `reference/commands.md` gives the default of
`--recipient` for `wait`, `inbox`, `confirm` and `launch` with the
standing agent's address in its place, and the refusal.
`reference/state.md` gives `recipients_from`'s `standing` with its
`agent` and `source`. `architecture/rules.md` mirrors core.28's line as
`core.md` holds it. Each speaks to any host's operator (core.35).

## Risks / Trade-offs

- A command that names no recipient, in a pane on a host whose sources
  declare a standing agent, asks the terminal for a part's agents: one
  bounded `session list` and one `agent list` under herdr, as a status
  asks. → Decision 4 asks only where the command has a session and a
  standing agent is declared, each part once.
- flume trusts the name: an agent that takes a standing agent's name in
  its session acts as its address, as it would take its ends (record
  0013).
- A refusal for an agent standing for two addresses applies to every
  command it runs with no recipient, a launch among them. → The words
  name each address and `--recipient`, and `$FLUME_RECIPIENT` still
  serves where the agent's starter sets it.

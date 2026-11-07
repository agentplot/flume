---
title: Roles and wake
description: A recipient that is a role rather than a session, the run the host launches to serve it or the agent that already holds it, and flume wake.
sidebar:
  order: 5
---

A recipient is most often a session: the pane that launched the place. A
session can end, and an end delivered to a session nobody runs is
delivered to nobody. A **role** is a recipient that outlives any one
session, such as a planner that reads every landing and plans the next
piece of work. flume launches a run of the role whenever an end is
pending for it and no run stands. Where the host's configuration names
an agent that already runs for the role, flume delivers into that agent
instead ([an address an agent already holds](#an-address-an-agent-already-holds)).

## A role's address

A role's address is two words joined by a slash, `<group>/<role>`,
each of letters, digits, `.`, `_` and `-`, such as `acme/planner`. Any
recipient name of that form is an address; any other name is a
session's. A launch names one like any recipient:

```sh
flume launch --kind fix --name the-parser-reads-tabs \
  --brief "The parser drops a tab after a quote." --recipient acme/planner
```

## The run that serves a role

The host's configuration names, for each address, the work that serves
it, in a `[roles]` table:

```toml
# ~/.config/flume/config.toml
[roles."acme/planner"]
repository = "~/src/acme"
kind = "planner"
priority = 5
base = "main"
brief = "Read the records and plan the next unit."
```

| Key | Meaning | Left out |
|---|---|---|
| `repository` | the repository the run's place is in; a leading `~` is your home | refused |
| `kind` | the kind the run launches, named as a launch's `--kind` is: from that repository's kinds or flume's own, or by its source | refused |
| `priority` | the run's priority in the host's queue | `0` |
| `base` | the base the run's place is cut from | the repository's default branch |
| `brief` | the run's brief, for a kind whose prompt asks for one | none |
| `stream` | the [stream](/flume/guides/agent-settings/#streams) the run's place runs in, one the host declares | as for any launch |

A key the table does not know, or a key that is no address, makes the
host's configuration unreadable, and every command of flume's says so.

A role's `kind` is read as `flume launch --kind` reads one
([where kinds come from](/flume/guides/kinds/#where-kinds-come-from)).
A bare `kind = "planner"` takes the repository's `planner`, else flume's
own. A role names a kind from a source the host declares by its
source, `kind = "beta/planner@1"`, or `kind = "beta/planner"` to take
the version the locks pick, and the kind runs under the same locks as
any launch's.
A role's run carries no `--kinds` file, so a kind that no repository
declares and flume does not carry reaches a role through a source.

A run of the role is a place like any other, `<kind>--<role>`, here
`planner--planner`, named by the role, with the address as
its role and as the session name its agents carry. It has no recipient
of its own. Since the run's name is the role's, at most one run of an
address stands at a time.

## When a role wakes

A role wakes in two ways:

- **An end pending for its address.** A settle that finds an end
  pending for an address with no run live or waiting launches the run,
  inside that settle.
- **`flume wake <address>`**, for another tool that carries word of
  something flume cannot see, such as an answer on a card or a message,
  and wants the role to look.

A wake of an address whose run is live or waiting makes nothing, and
the answer says the run is live or waits. An address an agent already
holds wakes no run either way. Nothing wakes an address the
host's configuration names nothing for: status lists it
under the host's `addresses`, with its pending ends and why no run was
woken.

flume decides nothing of what the run does. Its kind's prompt says
that.

## How a role reads its ends

An end for an address is delivered as a notice into the pane of the
address's live run, found again by its agent's herdr name when the
pane moved. When no run is live the end stays pending, and the wake
launches one. An address an agent already holds takes its ends in
that agent's pane instead.

Inside the run, `flume wait` and `flume inbox` act for the role by
default: an agent flume started carries `FLUME_PLACE`, and a command
run there takes the role that place serves as its recipient. So the
run's first act can be:

```sh
flume inbox --json
```

which collects every end pending for its address, on every place of
the host, and marks each delivered.

## An address an agent already holds

A role may already have an agent of its own: one a person or another
tool started and keeps running in a herdr session, under a name herdr
knows it by. The host's configuration gives the address that agent in
place of a run:

```toml
# ~/.config/flume/config.toml
[roles."acme/planner"]
agent = "acme-planner"
session = "acme"
```

| Key | Meaning | Left out |
|---|---|---|
| `agent` | the name the multiplexer knows the agent by, as `herdr agent start <name>` gives it | the entry is a run's |
| `session` | the session the agent lives in | the session of the entry's stream |
| `stream` | the [stream](/flume/guides/agent-settings/#streams) whose multiplexer the agent is found in, and whose session it lives in where the entry names none; one the host declares | `default` |

With no `session`, flume looks for the agent in the session the
stream's own agents open in. That is the stream's `session`, else the
stream's name, and the multiplexer's own session for a default stream
that names none. The
multiplexer is the stream's, else the host's. So where the stream
`acme` opens its agents in the agent's session, `stream = "acme"`
alone finds it.

An entry gives an agent or a run, never both. An entry giving `agent`
beside `kind` makes the host's configuration unreadable, naming the
address, since two agents would then hold one role. So does `agent`
beside `repository`, `priority`, `base` or `brief`, naming the key, and
`session` in an entry with no `agent`. flume checks no rule of herdr's
on the name: a name herdr cannot hold is one it never lists, and status
says it found none. The `[roles]` table stays the managed layer's, so a
local file that names it is not taken at all.

### What goes in, and when

Each settle that reads an end pending for the address asks the
stream's multiplexer for the agents in the session and looks among them
for the entry's name. Where it finds exactly one, working or waiting,
the end goes into its pane as a notice, as into any recipient's, and
counts delivered on the multiplexer's answer. flume keeps nothing of
where the agent was. It finds the agent again at each delivery, so an
agent started again under the same name in the session takes the next
end.

An end stays pending while:

- the session holds no agent of the name, or more than one;
- the agent is asking a question, or herdr cannot say what it is doing;
- the session does not run, the host declares no such stream, the
  stream's multiplexer is one flume carries no profile for, or the
  multiplexer cannot be read. The printed command holds no agent a name
  finds.

Each end left pending goes in at the first settle that finds the agent,
one notice each, so an agent back after a day away reads a notice for
each end of that day. Meanwhile `flume inbox --recipient acme/planner`
collects them. herdr forgets the name of an agent that starts again
inside its pane, as after a `/clear`, and the address's ends wait until
its keeper names it again.

flume trusts the name. It did not start the agent and checks no
identity for it, so an agent that takes the name in that session takes
the address's ends.

### No run, and the wake

flume opens nothing for an address an agent holds and keeps no watch on
it. A settle launches no run for it, and `flume wake` launches nothing:
it answers that the agent holds the address, with where the agent
lives, what the multiplexer found and the ends pending for it:

```console
$ flume wake acme/planner
acme/planner is held by the standing agent acme-planner in the session acme (the entry's), found waiting, so nothing was launched; 1 end pending
  ~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs: submission landed d185a879b51dba33555c036392b50443f4dfc5e8
```

Status lists every address an agent holds under the host's addresses,
pending or not, read when asked:

```console
  acme/planner: the standing agent acme-planner in the session acme (the entry's), found waiting; nothing pending
  acme/ops: the standing agent acme-ops in the session acme (the stream acme's name), none of that name there; 1 end pending
```

What it found is `found` with the agent's state, `none of that name
there`, `2 of that name there, so none takes a notice`, or `not read:`
with the reason. The session's source is `the entry's`, `the stream
<stream>'s`, `the stream <stream>'s name`, or the terminal's own
session.

### From an agent to a run

Which agent holds an address is the configuration's, read at each
delivery and each wake. An entry moves the role onto a kind by trading
`agent` and `session` for a run's keys:

```toml
[roles."acme/planner"]
repository = "~/src/acme"
kind = "planner"
priority = 5
```

The next end pending for the address wakes a run, and the agent takes
none. An entry that trades back sends the next end to the agent, even
while a run an earlier entry launched stands. Every launch names the
address as before, so none changes.

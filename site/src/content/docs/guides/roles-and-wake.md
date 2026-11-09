---
title: Roles and wake
description: A recipient that is a role of a stream rather than a session, the roles a source of the host declares, the run flume launches for one or the agent that already holds it, the answerer behind a stream, and flume wake.
sidebar:
  order: 5
---

A recipient is most often a session: the pane that launched the place. A
session can end, and an end delivered to a session nobody runs is
delivered to nobody. A **role** is a recipient that outlives any one
session, such as a planner that reads every landing and plans the next
piece of work, or an operator that answers what no other role takes up.
A source of the host's kinds declares its roles, and flume launches a
run of a role whenever an end is pending for it and no run stands.
Where the source names an agent that already runs for the role, flume
delivers into that agent instead ([an address an agent already
holds](#an-address-an-agent-already-holds)).

## A role's address

A role's address is `<stream>/<role>`: the role `<role>` of the stream
`<stream>`, a [stream](/flume/guides/agent-settings/#streams) the
host declares. `east/ops` is ops of the stream `east`. Each part is a
word of letters, digits, `.`, `_` and `-`. Any recipient name of that
form is an address; any other name is a session's.

The stream says where a role's run runs and how much of the host it
takes. It never bounds whom a place may ask: a launch may name any
stream's role as its recipient, and a stage's agent may put its
question to any stream's role ([who answers a
place](/flume/guides/callers-and-holds/#who-answers-a-place)). A
launch, a wake and a recipient write an address whole:

```sh
flume launch --kind fix --name the-parser-reads-tabs \
  --brief "The parser drops a tab after a quote." --recipient east/planner
```

## Roles come from a source

A source of kinds the host declares
([kinds from a source on the host](/flume/guides/kinds/#kinds-from-a-source-on-the-host))
may also declare the roles it answers. Its file in `sources/` names
`roles`, the file holding them, which the source's own tool writes:

```toml
# ~/.config/flume/sources/acme.toml
kinds_dir = "~/.local/share/flume/kinds/acme"
roles     = "~/.local/state/acme/flume/roles.toml"
```

The roles file holds one entry for each address the source answers,
on every stream it runs in:

```toml
# ~/.local/state/acme/flume/roles.toml, written whole by acme's tool
[roles."east/operator"]
kind = "operator"
repository = "~/src/acme-records"
name = "east"
intent = "east-operator"
answers = "Whatever reaches the stream's work from outside it, and a question no role took up."
answerer = true

[roles."east/ops"]
kind = "ops"
repository = "~/src/acme-records"
answers = "A live system: what is deployed, a credential, a deploy, a landing that failed."

[roles."east/ops".agents]
model = "opus"

[roles."west/ops"]
kind = "ops"
repository = "~/src/acme-records"
answers = "A live system of the stream west."

[roles."north/planner"]
carried = true
answers = "The work's scope, for the stream north, which runs on another host."
```

flume reads every source's roles as their files stand at each launch,
wake, start, stall and status, and writes nothing in them. The
[configuration reference](/flume/reference/configuration/#a-sources-roles)
gives every key and fault.

The roles are the source's because every fact of a role's run is: its
kind, the repository it reads, its name, its settings and what it
answers. A source that runs one role on ten streams writes ten
addresses, each run by the same package. What stays the host's is its
own:

- **the source, declared** in `sources/`, saying where its packages and
  its roles lie, since the host decides whose kinds and roles run under
  its accounts;
- **the lock of the source's kinds**, the host's trust in the content
  that drives agents with its accounts
  ([the locks of kinds](/flume/reference/configuration/#the-locks-of-kinds));
- **its streams**, with their room, pools, terminals and labels' reserves.

The host's configuration names no role and no answerer. A
`config.toml` that gives a `[roles]` table cannot be read, and every
command says so, naming the source's roles file as the place a role is
declared.

## The run that serves a role

A run's entry names the kind and the repository, and may give the run's
`priority`, `base`, `brief`, `name`, `intent` and `agents`, its agents'
settings as a launch's `--agent` gives them. Its kind is one of its
source's own, `kind = "ops"` or `kind = "acme/ops"`, launched as
`acme/ops` at the version the locks pick. A wake whose kind no lock
records is refused, as a launch's is.

A run of the role is a place like any other, launched as a caller's
launch would be:

- **its name** is the entry's `name`, else `<stream>-<role>`, so
  `east/ops` runs as the place `ops--east-ops`, and two streams' runs of
  one role are two places;
- **its stream** is its address's, and its label its kind's, so its
  room, its pool and its terminal are that stream's;
- **its intent and settings** are its entry's, standing as a launch's
  in their layers ([agent settings](/flume/guides/agent-settings/));
- **its role** is the address, which its agents carry as their session's
  name. It has no recipient of its own.

Since one address makes one place, at most one run of an address stands
at a time. A run an earlier flume named by its role alone is found by
its role.

A role's run lands nothing. Once its last stage delivers, the settle
that reads it declares the run done, with no caller's word, and tidies
it. The next end pending for the address wakes another run, which
collects what came since.

## When a role wakes

A role wakes in two ways:

- **An end pending for its address that nothing has handed to it.** A
  settle that finds one with no run live or waiting launches the run,
  inside that settle. An end a run was handed, by a notice, a wait or
  its inbox, and did not confirm wakes no run again, so a run that
  never confirms what it reads wakes no run after it; the next run
  another end wakes is handed it by its own inbox.
- **`flume wake <address>`**, for another tool that carries word of
  something flume cannot see, such as a person's answer or a message,
  and wants the role to look.

A wake of an address whose run is live or waiting makes nothing, and
the answer says the run is live or waits. An address an agent already
holds wakes no run either way. Nothing wakes an address that no source
declares, two declare, or whose entry is at fault: its wake is refused
saying why, and status lists it under the host's `addresses`, with its
pending ends and why it wakes nothing:

```console
$ flume wake east/reviewer
refused: east/reviewer wakes nothing: no source of the host declares east/reviewer
```

flume decides nothing of what the run does. Its kind's prompt says
that.

## How a role reads its ends

An end for an address is handed over as a notice into the pane of the
address's live run, found again by its agent's herdr name when the
pane moved. When no run is live the end stays pending, and the wake
launches one. A hand-over delivers nothing: the end stays pending for
the address until the run confirms it by its key.

Inside the run, `flume wait` and `flume inbox` act for the role by
default: an agent flume started carries `FLUME_PLACE`, and a command
run there takes the role that place serves as its recipient. So the
run's first act can be:

```sh
flume inbox --json
```

which collects every end pending for its address, on every place of
the host, each handed to the address, with its delivery's key. Once
the run has acted on one, it confirms it,
`flume confirm <key> --json`, or it collects with `--confirm` where
taking an end is acting on it. An end the run leaves unconfirmed stays
pending for the address, shown expired in status once the host's span
passes. Its `waiting` gives every place that waits on the address,
delivered or not, with the question, its key and any hold on it
([flume inbox](/flume/guides/callers-and-holds/#flume-inbox)).
A run woken after another sees every question still waiting on its
role, whichever stream asked it.

## The answerer behind a stream

A run's or a standing agent's entry may give `answerer = true`: that
role **stands behind** its address's stream, as `east/operator` stands
behind `east`. The answerer is the stream's last word on its places:

- a launch into the stream that names no recipient, sets no
  `$FLUME_RECIPIENT` and runs in no terminal session takes the answerer
  as its recipient;
- every stall of a place in the stream reaches the answerer, besides
  the place's recipients and the role its question named
  ([stalls](/flume/guides/callers-and-holds/#stalls));
- a role's run that stalls has its stall delivered to the answerer, and
  never to its own address.

One role stands behind a stream at most. Where two do, of one source or
two, the stream has none: status names both, and a launch that would
take it is refused naming both. A stream no role stands behind has no
answerer, and its places name their recipients or run in a session.
flume reads the answerer from the sources at each launch and each
stall, and keeps nothing of it.

Status gives each stream's answerer with its source, and each place's
recipients with where they came from:

```text
  recipients: east/operator, from the stream's answerer, declared by acme
```

The answerer's own run has no one behind it. Where it stalls, its stall
reaches no one, and status names it under the host's addresses:

```text
  0 ends pending for east/operator, its run ~/.local/share/flume/places/f5f5e14159a8/operator--east
    its run ~/.local/share/flume/places/f5f5e14159a8/operator--east stalled: a retry of turn, which reached no one
```

## An address on another host

A source may answer a stream that runs on another host. Its entry there
gives `carried = true`, and `answers` where the role answers questions,
and nothing else. flume launches nothing for a carried address. It
takes a launch naming it, lists it by its whole address for a stage's
question, and keeps its ends pending for the tool that carries them to
the role's host. A `flume wake` of it is refused, saying its source
declares it carried, its run on another host. Status says so:

```text
  1 ends pending for north/planner, no run; acme declares it carried, its run on another host
```

## An answering run's way with a question

A role whose entry gives `answers` is offered every question a stage's
agent addresses to it, and every stall of a place it stands behind. flume
reaches the role, gives it the question with its key, takes the hold
that says the role is on it, and takes the answer once. What the role
does is its kind's. A kind that answers can follow this way:

1. **Collect.** Run `flume inbox --json` for its address and read
   `ends` and `waiting`. Confirm each end once acted on, by
   `flume confirm` with its `delivery` key.
2. **Hold what it takes**, before anything else, so a long turn does
   not stall the place:
   `flume hold <place> "east-ops has it" --waiting --for 1h`.
3. **Answer what is its own**, naming the key, so the answer is taken
   once: `flume retry <place> --stage <stage> --answers <key> --words
   "<answer>"`. While the asking agent is kept open, the answer reaches
   its session; otherwise a fresh agent carries it.
4. **Pass what is another role's**, of its own stream or another's,
   by its source's own means, and hold the move naming the address:
   `flume hold <place> "passed to west/ops" --waiting --for 1h`.
5. **Say what is a person's.** Hold first, then hand the question to
   its source's own code, which puts it to the person once, keyed by the
   place and the end's key, so a second asker finds it already put.
   Hold again naming where it waits, with a bound of a day or more. Once
   the person answers, that code carries their words back as the
   place's retry naming the key. No agent rewords the question or
   carries the answer.

The answerer, on a stall, reads the stall's words and any hold that
lapsed. Where nobody had the question, it passes it once to the role
whose it is. Where the person has it, it holds again and puts nothing
to them a second time.

A retry refused for its key means another answered first: the refusal
names who and when. A stage's agent never asks a person itself; it
asks by `flume needs` alone, and whoever answers decides whether the
question is a person's.

## An address an agent already holds

A role may already have an agent of its own: one a person or another
tool started and keeps running in a herdr session, under a name herdr
knows it by. Its source's roles file gives the address that agent in
place of a run:

```toml
[roles."east/planner"]
agent = "east-planner"
session = "acme"
answers = "The work's scope: work outside the goal, the order of work."
```

| Key | Meaning | Left out |
|---|---|---|
| `agent` | the name the multiplexer knows the agent by, as `herdr agent start <name>` gives it | the entry is a run's |
| `session` | the session the agent lives in | the session of the address's stream |
| `answers` | what the role answers, which lists it for a stage's question | not listed |
| `answerer` | `true` where the role stands behind its address's stream | `false` |

The agent is found in its address's stream's multiplexer, the stream's
own, else the host's. With no `session`, flume looks in the session the
stream's own agents open in: the stream's `session`, else the stream's
name, and the multiplexer's own session for a default stream that names
none.

An entry gives an agent or a run, never both. An entry giving `agent`
beside `kind`, or beside a run's other keys, declares nothing for its
address, and status names it. flume checks no rule of herdr's on the
name: a name herdr cannot hold is one it never lists, and status says
it found none.

### What goes in, and when

Each settle that reads an end pending for the address asks the
stream's multiplexer for the agents in the session and looks among them
for the entry's name. Where it finds exactly one, working or waiting,
the end goes into its pane as a notice, as into any recipient's, and
counts handed over on the multiplexer's answer, pending until the agent
confirms it. flume keeps nothing of
where the agent was. It finds the agent again at each delivery, so an
agent started again under the same name in the session takes the next
end.

An end stays pending while:

- the session holds no agent of the name, or more than one;
- the agent is asking a question, or herdr cannot say what it is doing;
- the session does not run, the stream's multiplexer is one flume
  carries no profile for, or the multiplexer cannot be read. The
  printed command holds no agent a name finds.

Each end that no notice has gone in for goes in at the first settle
that finds the agent, one notice each, so an agent back after a day
away reads a notice for each end of that day. Meanwhile
`flume inbox --recipient east/planner` collects them, and an end it
handed over is sent no notice after. herdr forgets the name of an agent that starts again
inside its pane, as after a `/clear`, and the address's ends wait until
its keeper names it again. A question left pending reaches the
answerer behind the asking place's stream at its stall.

flume trusts the name. It did not start the agent and checks no
identity for it, so an agent that takes the name in that session takes
the address's ends.

### No run, and the wake

flume opens nothing for an address an agent holds and keeps no watch on
it. A settle launches no run for it, and `flume wake` launches nothing:
it answers that the agent holds the address, with where the agent
lives, what the multiplexer found and the ends pending for it:

```console
$ flume wake east/planner
east/planner is held by the standing agent east-planner in the session acme (the entry's), found waiting, so nothing was launched; 1 end pending
  ~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs: submission landed d185a879b51dba33555c036392b50443f4dfc5e8
```

Status lists every address an agent holds under the host's addresses,
pending or not, read when asked:

```console
  east/planner: the standing agent east-planner in the session acme (the entry's), found waiting; nothing pending
  east/design: the standing agent east-design in the session east (the stream east's name), none of that name there; 1 end pending
```

What it found is `found` with the agent's state, `none of that name
there`, `2 of that name there, so none takes a notice`, or `not read:`
with the reason. The session's source is `the entry's`, `the stream
<stream>'s`, `the stream <stream>'s name`, or the terminal's own
session.

### From an agent to a run

Which agent holds an address is its source's roles file's, read at each
delivery and each wake. The source moves the role onto a kind by
trading `agent` and `session` for a run's keys in that file:

```toml
[roles."east/planner"]
kind = "planner"
repository = "~/src/acme-records"
answers = "The work's scope: work outside the goal, the order of work."
```

The next end pending for the address wakes a run, and the agent takes
none. An entry that trades back sends the next end to the agent, even
while a run an earlier entry launched stands. Every launch names the
address as before, and nothing of the host's changes.

---
title: Roles and wake
description: A recipient that is a role rather than a session, the run the host launches to serve it, and flume wake.
sidebar:
  order: 5
---

A recipient is most often a session: the pane that launched the place. A
session can end, and an end delivered to a session nobody runs is
delivered to nobody. A **role** is a recipient that outlives any one
session, such as a planner that reads every landing and plans the next
piece of work. flume launches a run of the role whenever an end is
pending for it and no run stands.

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
| `kind` | the kind the run launches, from that repository's kinds or flume's own | refused |
| `priority` | the run's priority in the host's queue | `0` |
| `base` | the base the run's place is cut from | the repository's default branch |
| `brief` | the run's brief, for a kind whose prompt asks for one | none |

A key the table does not know, or a key that is no address, makes the
host's configuration unreadable, and every command of flume's says so.

A run of the role is a place like any other: `<repository>/places/<kind>--<role>`,
here `places/planner--planner`, named by the role, with the address as
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
the answer says the run is live or waits. Nothing wakes an address the
host's configuration names nothing for: status lists it
under the host's `addresses`, with its pending ends and why no run was
woken.

flume decides nothing of what the run does. Its kind's prompt says
that.

## How a role reads its ends

An end for an address is delivered as a notice into the pane of the
address's live run, found again by its agent's herdr name when the
pane moved. When no run is live the end stays pending, and the wake
launches one.

Inside the run, `flume wait` and `flume inbox` act for the role by
default: an agent flume started carries `FLUME_PLACE`, and a command
run there takes the role that place serves as its recipient. So the
run's first act can be:

```sh
flume inbox --json
```

which collects every end pending for its address, on every place of
the host, and marks each delivered.

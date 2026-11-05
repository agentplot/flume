---
number: 33
title: A stream shares its agents among its kinds' labels, by reserves an idle label lends and slots one keeps free, and a label has no pool, points or pace of its own
status: proposed
date: 2026-11-03
decision-makers:
- Chuck Swanberg
---

# A stream shares its agents among its kinds' labels, by reserves an idle label lends and slots one keeps free, and a label has no pool, points or pace of its own

Status: proposed. The user decided it on 2026-11-03 on their Pending You cards, and what a kind's label is called and how a host's MCP servers reach an agent on 2026-11-05 in the Plannotator Inbox, in the words quoted below. It is stated on agentplot/blueprints main at 2c95a70, and the two rulings of 2026-11-05 on blueprints' branch `design/flume-weir-a-kinds-label-and-a-hosts-mcp-servers` at 157e899. The record waits on the user's yes once they have seen it as a record, on their review of the change `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer`.

Deciders: the user, who decided where a label comes from, that a label has no budget of its own, reserves and the slot kept free, where an MCP server names its labels, what a kind's label is called, and how a host's MCP servers reach an agent. The writing agent's: what a label is spelled as, a label that keeps more free than its reserve, how one settle ranks stages as it starts them, which label an agent's share counts under, what status gives, and what this change leaves unbuilt.
Sources: the Plannotator Inbox thread msg_01M6P77HNP8KR9P83WNB98PA4N; `agentplot/blueprints:design/flume/requirements.md` at 2c95a70 and at 157e899, clauses 70 and 143, the vocabulary's label, reserve and room, 180 to 183, C.2's `label`, C.7, section 10's question on MCP servers, S84 and S85; `agentplot/blueprints:design/weir/requirements.md`, clauses 52 and 94 and C.4; records 0004, 0010, 0020 and 0025; rules core.4, core.26, core.32, core.38; `crates/flume-core/src/room.rs` (`Queue`, `StreamRoom`, `Waiting::rank`, `wants`, `must_wait`, `start_waiting`), `crates/flume/src/host/config.rs` (`streams_declared`, `kinds_streams`), `crates/flume-seams/src/kinds.rs` (`Kind`); weir-kinds at the revision `Cargo.toml` pins, whose `Kind` holds no label.

## Context and problem statement

A stream is one bound on agents running, with one priority and one pool (record 0020). Within a stream every place competes by its caller's priority alone. A host's work stream runs several sorts of work: changes being built, fixes and deploys that keep a running system up, research and plans. A burst of changes fills the stream. A fix to a failing deploy waits until an agent ends, and a priority set high only orders it first in the queue. Nothing keeps room for it while it has not come.

Two streams for the two sorts of work would split the pool, the points bound and the pace, the budget the stream's accounts give, and the host would hold two bounds for work that shares one allowance.

## Considered options

1. Priorities alone, as today.
2. A stream per sort of work.
3. Labels within a stream. A kind's label says what sort of work its places are. A stream owes each label a reserve of its agents, lent to the stream's other labels while the label has nothing waiting and its own again at the next agent to end. A label may keep some of its reserve free, never lent. A label has no pool, points or pace of its own.
4. As 3, an agent ended to give a reserve back at once.
5. Weighted shares of the stream, flume counting each label's use over time.

## Decision outcome

Option 3. The user's words, on their cards and in the Inbox:

- Where a label comes from: "The kind declares it; the host may reassign it."
- Whether a label has a pool, points or pace of its own: "No: seats are the stream's budget."
- How a reserve works: "Reserves an idle label lends; it takes the next free slot back." and, of a slot kept empty, "The same, and a label may hold a slot empty (operation)".
- How an MCP server is given to a label's places: "On the server: `labels = [...]`, absent means open to every kind."
- What weir calls a kind's label, on 2026-11-05: "`label` in both formats for the share, with weir 94's display words renamed (for example `title`)".
- How a host's MCP servers reach the agents flume starts, on 2026-11-05: "A: declared in flume's host config under `[mcp.<server>]` with `labels`, passed to each agent at its start".

1. **A place's label (180).** A kind may declare its label, one word, `label` in its table (C.2), in a simple kind's and a statechart kind's alike, a statechart kind's display words being its `title` (weir 52, 94, C.4). The host's configuration may give a kind another, by `label` in `[repos."<repository>".kinds.<kind>]` for one repository, or in `[kinds.<kind>]` for every one, in that order, as it gives a stream (143). A place's label is the host's word for its kind, else its kind's own, else none. It is read at each start from the configuration as it stands (32), so a change takes the next start, and an agent already started keeps the share it started under until it ends. flume's own kinds declare none, so a host gives them theirs. A label names nothing a host owns (158), and flume gives it no meaning but its share of its stream's agents. A label is one word of letters, digits, `-` and `_`.
2. **A reserve (181).** A stream states, for each label, `reserve`, how many of its agents the label's places are owed, and `keep_free`, how many of those it keeps free, in `[streams.<stream>.labels.<label>]`, each zero where it gives none. Within the stream, a stage of a label running fewer agents than its reserve starts before every stage of a label running its reserve or more, whatever their places' priorities. So a reserve lent while its label had nothing waiting is its own again at the next agent to end, and no agent running is ended for it (31). A place of no label shares what no reserve holds.
3. **A slot kept free (181).** A stage starts only while its stream's room passes what the stream's other labels keep free and do not use. A slot kept free stays empty for its own label's next stage and is never lent.
4. **What refuses (181, 182, 4).** A stream whose reserves together pass its bound starts no stage, each refused naming the stream and its reserves, until the configuration is mended. So does a stream with a label keeping more free than its reserve, and one whose label's table holds any key but `reserve` and `keep_free`, the stage refused naming the key. A stream that states no bound has no reserves, and status names them passed over. A reserve is of its stream's agents alone: where the host's bound leaves no room, a stage of any label waits as 70 says.
5. **No budget of its own (182).** A label has no pool, account, points bound or pace. Its stream's pool, points bound and pace are the one budget every label of the stream draws on.
6. **Status (56).** Each place's label and where it came from: the host for its repository and kind, the host for its kind, or the kind. Each stream's labels, each with its reserve, what it keeps free, its agents running and its stages waiting. A stage waiting on room another label keeps free says so.
7. **An MCP server's labels (183).** A server the host's configuration gives its agents may name the labels whose places may have it, `labels = [...]`, open to every label where it names none. The managed layer declares each server, `[mcp.<server>]` with `command`, `args` and `env`, or `url`, and its `labels` (C.7). At each agent's start flume writes the place's share, the servers its label may have, into the place's own folder and hands it to the agent's program, Claude Code by `--mcp-config` (C.4). A program whose profile hands it no list is given none of the host's servers (171). The servers an agent's account and its repository list load beside the share, and flume neither reads nor filters them. Whether flume also shuts those out, Claude Code's `--strict-mcp-config` among the ways, is section 10's question, not decided.

The writing agent's choices: a label that is not one word of letters, digits, `-` and `_` is refused in a kind's declaration as in the host's configuration (weir C.4). A program whose profile hands it no list of servers is given none, rather than slipping past 183, and status gives each agent's servers. A label keeping more free than its reserve breaks "a label's reserve is at least what it keeps free" and is refused as reserves past the bound are, rather than read as its reserve, which would keep free what the host never owed. Within one settle, a label's count rises with each start the settle takes, and the stages left are ranked again, so one settle never starts a label past its reserve ahead of another below its own. The share is counted by the label each agent started under, since 180 says an agent keeps its share.

### Why not the others

Option 1 is today's: a fix's priority orders it first but keeps no room for it, so it waits for an agent to end. Option 2 splits the stream's pool, points and pace, the one budget its accounts give, which the user ruled out: "seats are the stream's budget." Option 4 ends a working agent, which clause 31 rules out. Option 5 has flume count use over time and judge fairness, a plan across places clause 70 keeps from flume. A reserve and a slot kept free are facts the host states and flume keeps to.

For a kind's label's name, a new name in simple kinds would spell one thing two ways across weir's formats, and no kind's own label would take back 180 and C.2. For how servers reach an agent, servers kept in each account's folder with accounts tagged by label would take back 182 and split a pool by label, so a label could wait on its accounts while others sit idle; leaving it open keeps 183 unbuilt. Only a list flume writes can keep 183's promise that a place outside a server's labels never gets it.

### Consequences

- Clauses 70 and 143, the vocabulary's label, reserve and room, C.2 and C.7 are amended, and 180 to 183, section 10's question on MCP servers, S84 and S85 added, on blueprints main at 2c95a70 (core.19).
- Rule core.26 is amended and rule core.46 drawn from this record.
- C.2 puts a kind's `label` in its table. The kind format is weir's, and flume reads a field of it only through weir's crate (core.4). weir 52, 94 and C.4 name `label` in both formats and a statechart kind's display words `title`, on blueprints' branch at 157e899. Until weir-kinds reads it at the revision flume pins, a kind's own label is read as none and only the host gives one. weir's own record of the name, and its crate's `title`, are weir's.
- Clause 183 is built by a change of its own, once the statement at 157e899 lands: the managed layer's `[mcp.<server>]`, each agent's share written into its place's folder and handed to its program, and status giving each agent's servers. Until then the suite names 183 unbuilt, as it reads the statement at 2c95a70.
- The change `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer` builds the rest.

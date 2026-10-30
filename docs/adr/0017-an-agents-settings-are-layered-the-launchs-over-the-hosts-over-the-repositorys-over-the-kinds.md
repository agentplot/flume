---
number: 17
title: An agent's settings are given for a kind or a stage, and layered, the launch's over the host's over the repository's over the kind's
status: accepted
date: 2026-10-28
decision-makers:
- Chuck Swanberg
---

# An agent's settings are given for a kind or a stage, and layered, the launch's over the host's over the repository's over the kind's

Status: accepted by the user, 2026-10-28, in their design session, on how a host chooses the Claude account and configuration its agents run under once a caller's session no longer reaches them.

Deciders: the user, who set the six layers and the example of one account for every change@1 agent; the shape, and the choice of when the host's layers are read, are the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 4d1f88b, clauses 4, 7, 8, 23, 29, 32, 56, C.2, C.4, C.7 and S44, and `design/weir/requirements.md` clauses 52, 57, 58 and C.4, both as amended with this record; records 0004, 0015, 0016; rules core.4, core.30.

## Context and problem statement

Clause 32 resolved an agent's program, model and instructions from the launch's settings for its stage, else the stage's declaration, else flume's default. Nothing gave settings for a whole kind, nothing let a repository change flume's own kinds without declaring them again, and nothing let a host say anything at all.

The host has something to say. Today each herdr session that runs Claude Code sets `CLAUDE_CONFIG_DIR` itself, which picks the configuration home, and with it the account signed in there. Under clause 29 an agent's environment is the host user's login environment with the settings over it, and nothing of the caller's session, so a session's `CLAUDE_CONFIG_DIR` never reaches a flume agent. Without a host layer, the only way to run change@1 under a chosen account is for every caller to pass it in every launch.

## Considered options

1. Keep clause 32, and have each caller carry the environment in its launch.
2. Settings given for a whole kind or for one stage, in six layers, most specific first: the launch's; the host's configuration for the repository and kind; the host's configuration for the kind; the repository's for a kind it does not declare, flume's own among them; the kind's declaration; flume's default.
3. As 2, but every layer, the host's among them, read once at the launch and kept for the place's life.

## Decision outcome

Option 2.

**The layers.** Each setting comes from the first layer that gives it. Within one layer, a stage's own setting comes before one given for every stage. The environment is merged one variable at a time, so the host can name `CLAUDE_CONFIG_DIR` while the launch adds a variable of its own; the program, the model and the instructions are each taken whole. Where in the terminal an agent runs stays the launch's and the stage's alone.

**Where each is written.** The host's are in `~/.config/flume/config.toml`: `[kinds.<kind>.agents]` for every repository, and `[repos."<repository>".kinds.<kind>.agents]` for one, the repository named by its location as a launch names it, each naming a single stage by a further `.stages.<stage>` table. In an `env` value a leading `~` is the host user's home and `$VAR` a variable of the login environment, never of the caller's session. So

```toml
[kinds.change.agents]
env = { CLAUDE_CONFIG_DIR = "~/.claude-work" }
```

runs every change@1 agent on the host under that configuration home and its account, whoever launched it. A repository's are in its `.config/flume.toml`: `[kinds.<kind>.agents]` beside a kind it declares, or alone, holding no stages, to give settings to a kind declared elsewhere, flume's fix@1, chore@1 and change@1 among them. A repository's file gives a program, a model and instructions, and no environment: an `env` there refuses the launch, since what one machine needs is that machine's to say.

**The format is weir's.** A kind-level `agents` table, and a kind table holding only settings, are fields of the kind format, so the weir statement names them (52, 57) and weir's crate reads them; flume defines no field of its own (core.4). Which declaration settings-only tables join is flume's rule (weir 58).

**When the host's layers are read.** The launch's settings, the repository's and the kind's are what the launch read, as clauses 8, 20 and 21 already hold. The host's configuration is read as it stands at each agent's start, as clause 32 says, and an agent already started keeps what it started with. Option 3 would fix the account for a place's whole life. The user's case needs option 2: an account that hits its usage limit, which the agent's last words carry (record 0016), is moved by changing the host's configuration and retrying the stage, with no new launch. Each stage is a fresh agent that carries nothing from the one before (30), so an account changed between stages moves nothing a running agent holds; the next stage just starts under the new one. A place's stages may then run under two accounts, which status shows.

**Refused, not guessed.** A program no agent profile binds, or that the host's terminal cannot run, refuses the launch, naming the program, the stage and its layer, with every stage resolved as the host's configuration stands at the launch (4). A later start that resolves to one, after the host's configuration changed, or that names a variable the login environment lacks, leaves its stage refused, naming it, until a retry (27).

**Seen in status.** Status gives each setting of the current agent with the layer it came from and its value as that layer wrote it, but an environment variable by its name alone, its value `<set>`, since a value may be a token. No log line, refusal's words or record of a start holds an environment value either. The agent's environment reaches it through its command and is otherwise kept only where a later act needs it: the launch's settings, which each later start reads as the launch gave them, in the place's held state; and, under the printed command, the command's environment in a file beside its mark, which the printed line names; each written readable by the host's user alone and printed by no answer. The host's layers need no keeping, read again at each start.

**Why not the others.** Option 1 puts one host's account into every caller, crew's roles and sounding's launches among them, and a caller that forgets it starts an agent under whatever account the login environment holds. Option 3 makes a usage limit cost a new launch, the place's work thrown away, for a guarantee no stage needs.

### Consequences

- Clauses 4, 7, 8, 23, 29, 32, 56, C.2, C.4 and C.7 of flume's statement are amended and S44 added, and weir's 52, 57, 58 and C.4, at blueprints 4d1f88b. Rule core.30 is drawn from it, and core.4's details name the two new fields as weir's.
- weir's crate reads the kind-level `agents` table and settings-only kind tables, and flume's pin of it moves once it does; until then `_open.md` names the wait.
- The OpenSpec change `flume-runs-a-kind-through` carries it in group 8, beside the resolver and the command line: the host's `[kinds]` and `[repos]` tables, the merge of the six layers, the refusals at the launch and at a later start, and status naming each setting's layer.
- Amended 2026-10-29: status named an environment variable by its value as its layer wrote it, so a token written into the host's configuration as it stands, rather than kept in the login environment, was printed whole; a failed `flume:live` walkthrough put a test account's `CLAUDE_CODE_OAUTH_TOKEN` into its log through status. The user asked that no status, log line, test panic or record print an environment value, names only. "Seen in status" above says what stands; core.30's details are amended with it.
- Amended 2026-10-29 by record 0020, as amended: the user ruled out the per-kind environment, "I don't think we want the old per-kind config." Accounts exist only in pools and give an agent its whole environment; no launch, host, repository or kind layer gives a variable. The host's `[kinds.<kind>.agents]` and `[repos."<repository>".kinds.<kind>.agents]` tables give a model and instructions alone, and the six layers keep their order for those and for the program, which the host's tables do not give. Clauses 29, 32, C.7 and S44 are amended at blueprints 87a7458; core.30 with them.

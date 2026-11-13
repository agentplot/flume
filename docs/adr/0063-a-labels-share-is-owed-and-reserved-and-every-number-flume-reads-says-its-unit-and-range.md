---
number: 63
title: A label's share is `owed` and `reserved`, and every number flume reads says its unit and range
status: accepted
date: 2026-11-09
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# A label's share is `owed` and `reserved`, and every number flume reads says its unit and range

Status: accepted. The direction is the user's, in agentplot-design's pane on 2026-11-09, reading deneb's Boomdrive configuration: "when settings are a number liek this one: threshold = 90; a comment should explain the units/options", and "\"keep_free\" - blurgh. don't love this config key. there are better ways to name config keys". The names and the words are agentplot-design's own choice, cheap to change.

Amends record 0033 where it names a label's `reserve` and `keep_free`.

Deciders: the user, 2026-11-09, above; agentplot-design, for the form.
Sources: `boomdrivedev/blueprints:design/principles.md` 13 ("A key is named in its reader's words. A number's unit and range are in its name or in its description, in the module's option and the tool's configuration reference"); `design/flume/requirements.md` 70, 143, 166, 167, 181, 185, C.7 and the vocabulary's reserve; record 0033; boomdrivedev/sluice's `[runs] reserved`, which keeps that many runs of the host's bound for submissions, "an agent's run in its worktree waits rather than take one" (its configuration reference); boomdrivedev/boomdrive's drive option `stream.labels.<label>.reserve`, Flume's `reserve` (its `nix/modules/options.nix` on the bolt `the-boomdrive-flake-configures-the-whole-host`); this repository's `nix/modules/options.nix` and `site/src/content/docs/reference/configuration.md`.

## Context and problem statement

A stream may give each label two numbers (181). `reserve` is how many of the stream's agents the label's places are owed: its stages start ahead of other labels' until it runs that many, and what it does not use is lent. `keep_free` is how many of those it never lends, held empty for its next stage. A reader cannot tell the second from its name, and `reserve` beside sluice's `reserved`, which means the opposite of lending, would tell them wrong.

Other numbers say less than they could. `threshold = 90` does not say it is a whole percent of any usage window, from 1 to 100. A stream's `priority` does not say which way it orders. The module's option descriptions and the configuration reference give some units and not others.

## Considered options

For the label's two numbers:

1. **`owed` and `reserved`.** `owed` is the statement's own word for what a label is first in line for. `reserved` is the family's word for what is held for one use and never lent, as sluice's `[runs] reserved` is.
2. `reserve` kept, `keep_free` renamed `held`. The pair would still read `reserve` and `reserved` apart across flume and sluice, with opposite meanings.
3. `guaranteed` and `dedicated`. Neither is the statement's word, and `guaranteed` promises an agent the host's bound may still withhold (70).

For the numbers:

1. **Each number's unit and range in its option's description and the configuration reference, its name kept where it reads plainly.**
2. A unit in every name, `threshold_percent`, `agents_max`. It renames keys whose description already settles them.

## Decision outcome

Option 1 for each.

- **A label gives `owed` and `reserved`.** `[streams.<stream>.labels.<label>]` holds `owed`, how many of the stream's agents its places are first in line for, lent to the stream's other labels while it has nothing waiting; and `reserved`, how many of those are held for it alone, never lent, empty while its places run fewer. Each is a whole number of agents from 0, 0 where none is given, and `reserved` is at most `owed`. `reserve` and `keep_free` are no words of a label's table: a file that names one refuses the stream's starts, naming the key, as any key the table does not hold (181), with no alias kept.
- **One word across the family.** `reserved` means held for one use and never lent, wherever a tool of the family reads it: sluice's runs for submissions, flume's agents for a label. Boomdrive's drive option for a label's share is named `owed` with it, and a drive that sets what a label holds names it `reserved`.
- **Every number says its unit and range.** Each number flume reads, in the module's option description, in the configuration reference and in the statement's C.7, says what it counts, its range and its default: the host's and a stream's `agents` (agents running at once, a whole number from 1; none states no bound); an account's `agents` (the same, for its agents); a stream's `priority` (a whole number, higher starting first, 0 by default); `points` (points in flight, a whole number from 1) and `pace` (`points` from 1 per `hour`, `day` or `week`); a label's `owed` and `reserved` (above); a pool's `threshold` (a whole percent of any of an account's usage windows, from 1 to 100, past which it takes no new start, 90 by default); and each span, `keep_open` and `ends_expire` (a time, `<n>`, `<n>s`, `<n>m` or `<n>h`). A name that reads plainly with its description, `threshold` among them, keeps it.

Not taken: the other options above, for the reasons given there.

### Consequences

- Good: a reader of a host's file, or of the Boomdrive set's options, reads what each number is without the statement beside it.
- Bad: every host file that names `reserve` or `keep_free` is refused until it names `owed` and `reserved`. A host's file is written by its module, so the module's next release mends it; a hand-written file is mended by hand.
- The statement's vocabulary, 70's room, 181, C.7 and its example say it, at boomdrivedev/blueprints. core.26 and core.46 say it here. Boomdrive's drive option renames with it, which is Boomdrive's to follow.

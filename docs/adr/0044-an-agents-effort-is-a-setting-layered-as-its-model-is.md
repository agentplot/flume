---
number: 44
title: An agent's effort is a setting layered as its model is, its program's own word, which each profile takes or refuses
status: accepted
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# An agent's effort is a setting layered as its model is, its program's own word, which each profile takes or refuses

Status: accepted by the user on 2026-11-06, on agentplot-planner's card (22:42Z), approving the change's design with this record and sending it into code, which answers the design's one question as it recommended: the effort is a setting of flume's, beside the model, and not Claude Code's own `effortLevel` given through the program-settings change. It is stated on boomdrivedev/blueprints main at cd8fd73, which marks it accepted on the user's word. The need is the user's: on 2026-11-05, in the questions design `every-question-a-place-asks-reaches-someone-who-answers-it`, they ruled that the planner and the design agent move onto their kinds only once flume carries an effort ("That order; planner and design agent wait for an effort setting"). The rest is the construct stage's own, which the written rules leave to it and which is cheap to change.

Deciders: the user, on the need and, on 2026-11-06, on the design; the construct stage of `an-agents-effort-is-a-setting-of-flumes-layers`, on 2026-11-05, on the rest of the design.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` on main at cd8fd73, clauses 4, 32, 88, 158, 170, 172, 173, 195, 197, C.2, C.4, C.7 and S103, as amended with this record, and weir's C.4; records 0017, 0024, 0026 and 0043; rules core.4, core.30, core.39; the change `an-agents-effort-is-a-setting-of-flumes-layers`, its design's Context; Claude Code 2.1.296, the release `devenv.nix` pins, and 2.1.312, the fleet's, each read from its binary on 2026-11-05: the option `--effort <level>` and its list `["low","medium","high","xhigh","max"]`, its warning for an unknown word, the settings schema's `effortLevel` and `modelSettings.<model>.effortLevel` of four words and `maxEffortLevel`, the order it reads an effort in, and the plugin engine types' hook input `effort.level`; Codex 0.165.0's `model_reasoning_effort` and `-c`; Gemini CLI 0.66.0's settings schema and `modelConfigs`; afterthought/crew at main f3c5abd4, `plugin/roles/*.md`, `plugin/lib/crew.py` (`EFFORTS`, `crew-role`'s command) and `docs/architecture/teams.md` (teams.6), and on `teamless/deneb`, `plugin/flume/kinds.toml`, `plugin/lib/plan.py`'s `roles.rec` and its README; `crates/flume/src/profiles/kinds/weir/mod.rs` (`map_agent`) and `crates/flume/tests/profiles/kinds_weir.rs` (sounding's `effort = "high"`).

## Context and problem statement

crew's roles each name a model and an effort, and crew starts them with `claude --model <model> --effort <effort>`: `xhigh` for the planner, the design agent and the coder, `high` for most, `medium` for the dispatcher and the operator, and a partition may move one to `max`. flume gives an agent its model in its layers (32, 172) and no effort, so a role run as a flume kind starts at its account's default.

Claude Code takes an effort two ways. `--effort` takes five words, `low` to `max`, as the session's own level, which the release reads before any settings file. Its settings' `effortLevel` takes four, with no `max`, which the release calls session-only, and it drops any other value with no word. An unknown `--effort` word is not refused either: the release warns on stderr and starts at its default. `CLAUDE_CODE_EFFORT_LEVEL` in the environment comes before both, and the release lowers any effort to what the model offers. Codex takes `model_reasoning_effort` by `-c`, in words of its own. Gemini CLI has no option, only a thinking level in its settings' model configurations.

The program-settings change, record 0043, lets every layer give Claude Code's own settings, and leaves `effortLevel` open.

## Considered options

For where the effort lives:

1. Claude Code's `effortLevel`, given through the program-settings change, with nothing added.
2. A setting of flume's beside the model, the program's own word, passed by each profile its own way or refused.
3. A setting of flume's on a scale of its own, mapped by each profile, as a mode is.

For an effort's word:

1. Passed as written, as a model's name is.
2. Taken only where the profile names it among its program's words, read from the release it pins.

## Decision outcome

Option 2 for where, and option 2 for the word.

1. **A setting beside the model.** Clause 32's settings gain the effort, given by the launch, the host's tables, the repository and the kind, a stage's own first, each taken whole from the first layer that gives it, as the model is (32, 197). A kind may give one, by its program's word, since it names nothing a host owns (158).
2. **The program's own word.** One word as written. A table of programs is the statement's, as the model's is (172), and is built with the model's by record 0026's change; until then a table is refused, as the model's is.
3. **Taken or refused.** Each profile names the words its program takes, read from its release, and refuses any other, naming the word, the program, the stage and the layer, at the launch or at a later start (4, 32). A model's names are open; an effort's are a closed list each release holds, and Claude Code would start at its default under an unknown one while status named another. Claude Code's profile takes `low`, `medium`, `high`, `xhigh` and `max` and passes `--effort <word>`, and a `check-tools` test finds the option and the five in the pinned release (C.4; core.22). A profile with no way refuses every word, so every other program refuses until its entry names its way (170).
4. **None where none.** With no effort in any layer, flume passes none and the account's own configuration decides (144).
5. **One owner in Claude Code's settings.** `effortLevel` is reserved for flume's effort, as `model` is for flume's model (195; record 0043). `modelSettings` and `maxEffortLevel` stay open: a model's entry holds more than its effort and reaches the agent only where no layer gives flume's, and a cap is a host's to set.
6. **Status.** The effort, its layer and the value passed, not read back, since the program may lower it to what the model offers; and, where the account's `env` names the variable the program reads over it, Claude Code's `CLAUDE_CODE_EFFORT_LEVEL`, that variable by name, never its value (56; core.30).

Option 1 for where gives `xhigh` to the planner and the design agent with no code, but cannot give `max`, which Claude Code's settings drop silently while status names it, and it names Claude Code's key in a kind, so a kind run on another program's accounts needs one key per program, and one Gemini CLI does not have. Option 3 maps one word to every program, but an effort is a level of one model and programs' levels differ, so no mapping is "no wider and no narrower", as clause 169 asks of a mode. Option 1 for the word lets a word no release takes start an agent at its default while status names the word.

### Consequences

- `Settings::effort`, taken whole as the model is; `Agent::effort`, refusing every word by default; `Agent::effort_variable`, none by default. The kinds profile's `map_agent`, the host's agents tables and `--agent [<stage>.]effort=<word>` read it.
- Claude Code's command passes `--effort <word>` after `--model`; `effortLevel` joins its reserved keys with the owner flume's `effort`.
- A host that gives `programs.claude.settings.effortLevel` has that key refused, and moves the word to `effort`; a named file's is left out and named in status.
- A declaration that writes `effort` today, which flume ignored as the runner's, is read, and a word its program does not take refuses its launch. sounding's elaboration kind writes one; no kind of flume's own does.
- An account usable only where its program takes the effort is 173's, built with the rest of 173 by record 0026's change; every program flume runs today is Claude Code.
- crew's runner gives a role's run `--agent effort=<Effort>` from its `roles.rec` record beside `--agent model=<Model>`. That is crew's change.
- The statement gains clause 197 and S103 and amends section 3, 4, 32, 88, 158, 170, 173, 195, C.2, C.4 and C.7, and weir's C.4 example gives `effort`, on blueprints main at cd8fd73 (core.19). Record 0026's model, its table and its entries, and record 0043's reserved keys are amended with this record. Rules core.30 and core.39 are amended with it.
- The change is built on the program-settings change's eight layers and its reserved keys, on flume main since #128.

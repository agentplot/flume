# Tasks

The work is on `fix/every-stages-agent-starts-whatever-its-arguments-hold`
at d8ec8d1, and record 0038 at 4a5ed8b. No test reaches the host's own
herdr or Claude Code: the tools layer runs the herdr the tree pins on a
scratch server, and the walkthroughs run the stand-in for Claude Code.

## 1. The command

- [x] 1.1 Give the agent seam's `command` in `crates/flume-seams/src/agent.rs` the place's own folder, where a start writes what it carries by a file beside its record, and have the start in `crates/flume-core/src/start.rs` pass it; the fake agent in `crates/flume-fakes/src/agent.rs` and the conformance scenarios in `crates/flume-conformance/src/seams/agent.rs` take it too (148, C.4; core.2). True when `sluice exec main -- moon run seams:test core:test fakes:test conformance:test conformance:coverage` passes.
- [x] 1.2 Have Claude Code's profile in `crates/flume/src/profiles/agent/claude/mod.rs` write the settings' instructions as given to `Record::standing_prompt`, a file beside the start's record in `record.rs`, and pass `STANDING_PROMPT_FLAG`, `--append-system-prompt-file`, naming it, so no argument holds a newline (148, C.4, C.5; record 0038). This is the agent-profile spec's scenario of the command for a code stage. True when `sluice exec main -- moon run flume:test` passes with `the_command_for_a_code_stage` in `crates/flume/tests/profiles/agent_claude.rs` holding instructions of several lines in the file as given and no argument holding a control character.

## 2. herdr and the walkthroughs

- [x] 2.1 Reproduce herdr's refusal and start past it in `crates/flume/tests/profiles/tools/terminal_herdr.rs`: `instructions_of_many_lines_start_under_herdr_by_the_agent_profiles_file` opens a start whose argument holds the instructions against the pinned herdr and reads it refused as `invalid_agent_argument` with no pane left, then opens one from the profile's command with the same instructions, which starts, the file its arguments name holding them as given (C.5; core.29). Have the stand-in in `crates/flume/tests/profiles/stand-in/claude.rs` refuse an `--append-system-prompt-file` naming no file, as 2.1.296 does, so the walkthroughs fail a start whose file was not written. True when `sluice exec main -- moon run flume:tools` passes.

## 3. The site

- [x] 3.1 Name `--append-system-prompt-file` in the command in `site/src/content/docs/guides/claude-code.mdx`, a file beside the start's record holding the instructions as given, and in the `instructions` row of `site/src/content/docs/guides/agent-settings.md`, so the instructions may hold newlines. Write both to any host's operator, naming no host, team or word of ours (core.35). True when `sluice exec main -- moon run site:prose site:build` passes.

## 4. The gate

- [x] 4.1 Run the checks of the projects this change touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test core:format core:lint core:test fakes:format fakes:lint fakes:test conformance:format conformance:lint conformance:test conformance:coverage flume:format flume:lint flume:test flume:tools site:prose site:build root:format root:adrs root:changes root:specs` passes.

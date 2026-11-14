# Verify: a-person-sees-a-rex-place-in-the-app

Verified at `961a276e` on `change/a-person-sees-a-rex-place-in-the-app`,
against its base `bolt/rex-is-flumes-second-terminal` at `6aa676fb`,
which the branch holds.

**Verdict: holds. Archived.** The whole checks pass. A check that
passed meets every requirement and scenario of the change's two specs.
Every task is ticked done, and every check a task names exists and
passed.

## The whole checks

`sluice exec bolt/rex-is-flumes-second-terminal`, the host's sluice
0.93.2, run once: **passed**, `exit 0`. sluice ran the checks `961a276`
affects against the base, the bolt's `#check-fast`: 13 tasks completed
and 12 passed. It recorded the 12 results under their keys for
`961a276`.

The run first asked through `devenv shell -- sluice exec …` stopped
before any check. Inside the shell the sluice the tree pins for the
merge-queue profile's tests, 0.71.0, comes first on `PATH`, and it
refused the host's `~/.config/sluice/config.toml` (`unknown field
credential`, line 21). No check ran under it. The run above is the
host's own sluice, which enters the devenv toolchain itself as
`.config/sluice.toml`'s `[tests] toolchain` says.

The tasks it ran: `site:install`, `site:prose` (0 errors, 0 warnings,
115 suggestions in 18 files), `site:build` (every internal link
valid), `root:format`, `root:adrs` (65 records, each with a number of
its own), `root:changes` (2 passed, this change among them),
`conformance:test`, `flume:format`, `flume:lint`, `flume:test`,
`plugin-claude:test`, `nix:typecheck` and `nix:test`.

The run's warnings are moon's: it could not write its output archives
from the gate's sandbox. No task failed for them.

Of note:

- `conformance:test`: `no_tool_is_named_outside_a_profile` (core.1) ok.
- `flume:test`: every `profiles::terminal::rex::tests::*` ok, the six
  this change adds among them; the library's 286 tests ok.
- `openspec validate a-person-sees-a-rex-place-in-the-app --strict`:
  valid.

## The tasks' own checks outside the bolt's gate

The bolt requires `#check-fast` alone. The Rex profile's tools tests
and the walkthroughs are `#check-tools`, which main requires. The
checks that tasks 3.1 to 3.4, 4.1 and 4.2 name were run as they name
them, once, in the devenv shell with the pinned Rex 0.1.0-preview.14:

`devenv shell -- cargo test -p flume --features tools --test tools --
terminal_rex:: walkthroughs::a_fix_on_a_stream_under_rex_lands_through_the_binary
walkthroughs::flume_open_of_a_rex_streams_session_names_the_socket_to_add`:
**passed**, `exit 0`, 35 passed and 0 failed in 33.90 seconds: the 33
`terminal_rex::*` tests and the two Rex walkthroughs.

## The spec: terminal-profiles

### REMOVED

| Requirement | Migration | Met |
|---|---|---|
| Rex's attach and show | The seven requirements added below, each held | yes |

### ADDED

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| Rex's attach for a window is refused | A window of Rex | `rex::tests::attach_for_a_window_is_refused_and_calls_nothing`: for `w` and for flume's own, no call, nothing recorded | yes |
| Rex's attach tells the app to show the newest Rex session flume made | The app shows the newest Rex session flume made | `rex::tests::attach_for_the_app_selects_the_newest_rex_session_flume_made`: a command line's client, a `lost` app and a connected app; the select names the connected app and the newer session at `w`'s socket; the records unchanged | yes |
| | A Rex session gone with its server | the same test, the newer session unlisted, the older selected | yes |
| Rex's attach for the app refuses naming why | A server no app is a client of | `terminal_rex::attach_for_the_app_names_the_socket_to_add_where_no_app_is_a_client` against the pinned Rex: `w`'s server started, the refusal naming `w.sock` and Remote Hosts; no select, as `rex::tests::attach_for_the_app_is_refused_naming_why` reads in its log | yes |
| | No Rex session of flume's in the server | `rex::tests::attach_for_the_app_is_refused_naming_why`: "holds no Rex session of flume's yet", naming the socket, no `do` | yes |
| | The app refuses | the same test: `client is not accepting actions` without `Error: `, naming the Rex session and the socket | yes |
| | (every app `lost`, the requirement's) | the same test: "lost", naming the socket, no `do` | yes |
| Rex labels a place's window and its base's Rex session | The labels follow the clues | `terminal_rex::show_labels_the_places_window_and_its_bases_rex_session`: the window `fix--one · code · run 1 · working`, the Rex session `repository main · 1 running · 0 asking · 0 waiting on caller · 0 stalled`, the block keeping its Rex name, no record written | yes |
| | A long label | the same test, with 200 `é` set as the first 128 characters; `rex::tests::a_label_is_its_place_and_clues_cut_to_128_characters`, non-ASCII crossing the 128th, cut between characters | yes |
| What Rex's show answers | A window Rex closed | `terminal_rex::show_passes_over_a_window_rex_closed_and_fails_naming_a_server_that_does_not_answer`: the agent closed, the window gone, show `Shown` with the Rex session labelled | yes |
| | A server that does not answer | the same test, the server frozen: `Failed` naming the window and the Rex session and the bound, within two bounds and two seconds | yes |
| | Nothing recorded | `rex::tests::show_and_shown_at_with_nothing_recorded_call_nothing` | yes |
| Where Rex shows a place's clues | A place on two servers | `rex::tests::shown_at_names_the_window_the_block_and_the_bases_rex_sessions`: the window and Rex session on `w`'s server, the live block, both of the base's Rex sessions, a tidied window and another base's session passed over; with no agent, the newest window and its server's Rex session | yes |
| | Nothing recorded for the place | `rex::tests::show_and_shown_at_with_nothing_recorded_call_nothing` | yes |
| Rex carries no plugin | Rex's setup | the same test: `setup` gives nothing, no call | yes |

The terminal seam's scenarios run with the harness's `shows_clues`
true: `terminal_rex::the_rex_profile_passes_the_terminal_scenarios` ok
(task 3.4).

## The spec: stream-terminals

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| A stream naming Rex runs its places in Rex (MODIFIED) | A fix on a stream under Rex | `walkthroughs::a_fix_on_a_stream_under_rex_lands_through_the_binary`: while the agent works, a settle labels the window `fix--one · <short line>` and the Rex session `<repository> main · 1 running · 0 asking · 0 waiting on caller · 0 stalled`; the fix lands, status gives `rex` from `stream`, and the window and Rex session are gone at the tidy with the server running | yes |
| A Rex stream's session is shown in the Rex app (ADDED) | A Rex stream's session with no app added | `walkthroughs::flume_open_of_a_rex_streams_session_names_the_socket_to_add`: `flume open trial` with `view = "app"` refused naming `trial.sock` and Remote Hosts, trial's server running after, no place's held state written | yes |

## The tasks

Every task of `tasks.md`, 1.1 to 5.2, is ticked `[x]`, and each check
it names passed:

| Task | Check | Passed in |
|---|---|---|
| 1.1 to 1.4, 2.1, 2.2 | `rex::tests::*` named | `flume:test` |
| 3.1 to 3.4 | `terminal_rex::*` named | the tools run |
| 4.1, 4.2 | `walkthroughs::*` named | the tools run |
| 5.1 | `root:adrs` | the gate |
| 5.2 | `site:prose`, `site:build` | the gate |

## The rest of the work

- **The record**, 0068: still proposed. Its decision item 6 says what
  attach for the app and show do, citing C.5's attach and show rows at
  blueprints 6557c33. Its deciders and sources name this change's
  design. The choices of design sections 3, 4, 6 and 7 join the
  construction agent's, each with the path not taken.
- **The site**: "Places under Rex" gives `view = "app"`, the app's
  Remote Hosts and Remote Control, and the labels. The roadmap's Rex
  row adds the labels and the app, and the Rex app leaves "Decided, not
  built". Rex is named only as a product flume integrates with
  (core.35).
- **The rules**: the core and the seams are untouched (core.2). Every
  Rex call goes through the profile's bounded `ask` with
  `--autostart=false -S unix://<socket>` (core.29). attach records and
  reads back nothing of the app (core.48). show writes no record and
  relabels no block (core.18).

## Not read by any check

The app's own answer to `session.select`, as design section 8 and
record 0068's consequences say: no app runs on a gate's host. A person
reads it.

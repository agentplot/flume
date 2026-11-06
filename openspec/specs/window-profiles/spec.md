# window-profiles Specification

## Purpose
The window seam's profiles: Ghostty, Terminal.app and the printed
command, each bound for a stream by its `terminal_app`, else the
host's, the printed command where neither names one (105, 120, 188,
189, C.7, C.8).

## Requirements

### Requirement: The window through Ghostty on macOS

On macOS the ghostty profile SHALL run `open -na Ghostty.app`, `--env
<name>=<value>` for each variable the command needs, then `--args
--title=<title> -e <program> <arguments>`, each a word of its own and no
shell between, under a bound of ten seconds, reading exit 0 as opened
and any other exit, or none within the bound, as refused in `open`'s
words (188, 189, C.8).

#### Scenario: The words open is given

- **WHEN** the profile shows `/bin/herdr session attach w`, needing `XDG_CONFIG_HOME`, under the title `Work Herdr`, its `open` a stand-in that records its words
- **THEN** the stand-in records `-na Ghostty.app --env XDG_CONFIG_HOME=<home> --args --title=Work Herdr -e /bin/herdr session attach w`, the title one word, and the answer is opened (188, C.8)

#### Scenario: Ghostty not installed

- **WHEN** the stand-in `open` exits 1 saying `Unable to find application named 'Ghostty.app'`
- **THEN** the answer is refused with those words (188)

#### Scenario: An open that never answers

- **WHEN** the stand-in `open` does not exit within the profile's bound
- **THEN** the answer is refused naming the bound, and nothing waits on it (189)

### Requirement: The window through Ghostty elsewhere

Elsewhere the ghostty profile SHALL start `ghostty --title=<title> -e
<program> <arguments>` in a process group of its own, its input, output
and errors closed, under the host user's login environment with the
command's over it and nothing of the calling command's, not waited on;
a `ghostty` that cannot be started SHALL refuse (120, 188, C.8).

#### Scenario: The words and environment ghostty is given

- **WHEN** the profile, given the login environment `HOME` and `PATH`, shows a command needing `XDG_CONFIG_HOME` while the calling command holds `HERDR_ENV`, its `ghostty` a stand-in that records its words, environment and process group
- **THEN** the stand-in records `--title=Work Herdr -e /bin/herdr session attach w`, an environment of `HOME`, `PATH` and `XDG_CONFIG_HOME` alone, and a process group not the test's, and the answer is opened before the stand-in ends (120, 188)

#### Scenario: No ghostty to run

- **WHEN** the profile's `ghostty` names no program that can be run
- **THEN** the answer is refused naming it (188)

### Requirement: The window through Terminal.app

On macOS the terminal profile SHALL run `osascript` with a script that
tells Terminal to `do script` its first argument in a new window and set
that tab's `custom title` to its second and `title displays custom
title` true, the second the title, under a bound of ten seconds, reading
exit 0 as opened and any other exit, or none within the bound, as
refused in `osascript`'s words; elsewhere it SHALL refuse (188, 189,
C.8).

#### Scenario: The words osascript is given

- **WHEN** the profile, built for macOS, shows `/bin/herdr session attach w` with no variable under the title `Office Herdr`, its `osascript` a stand-in that records its words
- **THEN** the stand-in records the script's lines each after an `-e`, then `exec /bin/herdr session attach w` and `Office Herdr`, each one argument, and the answer is opened (188, C.8)

### Requirement: The line Terminal.app runs

The terminal profile's first argument SHALL be the line `exec
/usr/bin/env <name>=<value> … <program> <arguments>`, `/usr/bin/env`
and its variables left out where none is needed, each word quoted for a
POSIX shell where it holds anything but letters, digits and
`-_./:=@%+,` (188, C.8).

#### Scenario: A command that needs a variable

- **WHEN** the profile shows a command needing `XDG_CONFIG_HOME` set to a home holding a space
- **THEN** the line is `exec /usr/bin/env 'XDG_CONFIG_HOME=<home>' /bin/herdr session attach w`, the home quoted, and the answer names no value of it (188; core.30)

#### Scenario: Terminal's automation not allowed

- **WHEN** the stand-in `osascript` exits 1 saying `Not authorized to send Apple events to Terminal.`
- **THEN** the answer is refused with those words (188)

#### Scenario: Not macOS

- **WHEN** the profile, built for another system, is asked to show a command
- **THEN** it refuses, saying Terminal.app runs only on macOS, and runs nothing (188)

### Requirement: The printed window

A stream and a host that name no terminal app SHALL get the printed
window, whose show runs nothing and answers printed with the line: each
variable the command needs as `<name>=<set>`, then its program and
arguments, each quoted for a POSIX shell where it holds anything but
letters, digits and `-_./:=@%+,` (105, 188; core.30).

#### Scenario: A line a person runs

- **WHEN** the printed window shows `/bin/herdr session attach work` with no variable, and then a command needing `XDG_CONFIG_HOME` under the title `Work Herdr`
- **THEN** the first line is `/bin/herdr session attach work`, and the second begins `XDG_CONFIG_HOME=<set>` and holds no value of it (188; core.30)

### Requirement: The terminal app bound for each stream

`terminal_app` in `[streams.<stream>]`, else in `[host]`, SHALL name a
stream's window profile, `ghostty` or `terminal`, the printed window
where neither names one; a name no profile carries SHALL refuse that
stream's `flume open` alone, naming the profiles, every other command
running as before; and the local layer SHALL not set it (105, 186, C.7,
C.8; record 0032).

#### Scenario: An unknown terminal app

- **WHEN** the host's configuration gives `[host] terminal_app = "kitty"`, and `flume status` and then `flume open` run
- **THEN** status answers, giving each stream's terminal app `kitty` from `host` and its refusal, and open is refused naming `kitty` and the profiles `ghostty`, `terminal` and `printed` (105, C.8)

#### Scenario: A terminal app set by the local layer

- **WHEN** the host's `config.local.toml` sets `[streams.work] terminal_app = "terminal"`
- **THEN** the local layer is not taken, status naming `streams.work.terminal_app`, and open uses the managed layer's terminal app (C.7)

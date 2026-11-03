## ADDED Requirements

### Requirement: Claude Code's command resumes the session a start carries

Where a start carries a conversation, the Claude Code profile's command
SHALL add `--resume` with the session the start resumes, and SHALL add
neither where the start carries none (88, 179, C.4).

#### Scenario: The command of a carried start

- **WHEN** the profile gives the command for a start resuming the session `s-1`, and for a start resuming none
- **THEN** the first holds `--resume s-1` beside `--plugin-dir`, and the second holds no `--resume` (88, 179)

### Requirement: Each start reads only its own part of its transcript

The Claude Code profile SHALL read own work, last words, asking's answer
and a slash command's receipt only from the records its start's
transcript holds past the length the start's first started ack
recorded, and the whole transcript where the file is shorter than that
length (43, 89, 90, 91, C.4).

#### Scenario: A background task the stopped agent left

- **WHEN** a carried start's transcript holds a background shell the stopped agent started and never finished, before the length its started ack recorded, and nothing of the carried agent's own still runs
- **THEN** `own_work` reads no for the carried start (31, 91)

#### Scenario: Last words before the carried agent wrote any

- **WHEN** a carried start's agent stops before it writes a message, and the transcript's last message is the stopped agent's
- **THEN** the carried start's last words are none, and the stopped start's are its own (43)

#### Scenario: A fresh start reads its whole transcript

- **WHEN** a start that carries nothing recorded a length of 0 at its started ack
- **THEN** every reading reads its whole transcript, as before (91)

### Requirement: Claude Code's carry pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source its carry rests on to stand byte for byte: `--resume`'s and
`--fork-session`'s option text, the transcript's folder under the
configuration home's `projects`, and the words it ends with when it
finds no such session (core.22, C.4).

#### Scenario: The pinned release's resume read

- **WHEN** the test reads the program devenv.nix pins, the file its `claude` wrapper runs
- **THEN** each fragment the carry rests on is found in it, and the test names any that is not (C.4)

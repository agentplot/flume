## ADDED Requirements

### Requirement: A pool's strategy and threshold and an account's reading in the configuration

The host's configuration SHALL read a pool's `strategy`, `fill-first`,
`least-busy` or `most-left`, its optional `threshold`, a whole number
from 1 to 100, and an account's optional `usage`, a path, each in the
managed layer alone, and SHALL refuse any other value of them, naming
the key (144, 184, 185, C.7).

#### Scenario: A most-left pool with readings

- **WHEN** the configuration gives `[pools.work] strategy = "most-left"`, `threshold = 85`, and an account `{ name = "work-1", program = "claude", usage = "~/.local/state/usage/work-1.json" }`
- **THEN** the pool reads most-left with a threshold of 85, and work-1's `usage` reads as written, its `~` read against the login environment only when the reading is asked (184, C.7)

#### Scenario: C.7's example with readings

- **WHEN** the configuration is C.7's example, its accounts of `work` each naming `usage`
- **THEN** each of work-1 and work-2 reads its `usage`, and `work` reads fill-first with a threshold of 90 (185, C.7)

#### Scenario: Values that refuse

- **WHEN** a pool gives `threshold = 0`, then `101`, then `"90%"`, an account `usage = 7`, or a pool `strategy = "round-robin"`
- **THEN** every launch and start is refused naming the key, and the strategy's words name fill-first, least-busy and most-left (4, C.7)

### Requirement: An account's reading is read from its file

The real host SHALL read an account's reading from the file at its
path in C.7's form, each time RFC 3339's with its offset and every key
it does not know ignored, and SHALL answer why there is none in words
that name no path (184, C.7).

#### Scenario: The form read

- **WHEN** a scratch file holds C.7's example reading with a `used` of 71.5, a window with no `resets_at`, and a key `login` at the top and `kind` in a window
- **THEN** the host reads `read_at`, both windows, 7150 hundredths and no reset for the second, and ignores both keys (184, C.7)

#### Scenario: A missing file

- **WHEN** the path names no file
- **THEN** the host answers that the file is missing, naming no path (184)

#### Scenario: A document not in the form

- **WHEN** the file is not JSON, or names no `read_at`, or gives a `read_at` with no offset, or a window whose `used` is 120
- **THEN** the host answers why, naming the key or the window by its place in the list (184, C.7)

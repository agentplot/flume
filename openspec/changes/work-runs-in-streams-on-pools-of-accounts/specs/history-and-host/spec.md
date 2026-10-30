## ADDED Requirements

### Requirement: The host's streams and pools

The host's facts SHALL give, as its configuration stands when asked,
its declared streams, the stream and the pool it names for a
repository and kind with the key each came from, each pool by name,
and the settings it gives a kind in a stream; none SHALL be kept
between asks (32, 143, 144, C.7).

#### Scenario: A pool for a stream's kind

- **WHEN** the configuration gives `[streams.work.kinds.chore] pool = "cheap"` and `[streams.work] pool = "work"`
- **THEN** the host names `cheap` for chore in `work` from the stream's kind, and `work` for fix in `work` from the stream (144)

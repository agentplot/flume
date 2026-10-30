## ADDED Requirements

### Requirement: The host's streams and pools

The host's facts SHALL give, as its configuration stands when asked,
its declared streams, the stream it names for a repository and kind
and the pool for a stream, each with the key it came from, and each
pool by name; none SHALL be kept between asks (32, 143, 144, C.7).

#### Scenario: A stream's pool and the host's

- **WHEN** the configuration gives `[streams.work] pool = "work"` and `[host] pool = "personal"`, and declares `personal` with no pool
- **THEN** the host names `work` for `work` from the stream, and `personal` for `personal` and `default` from the host (144)

## ADDED Requirements

### Requirement: herdr's setup read beside a fresh lease

The herdr profile SHALL give as the terminal's setup the words in the plugin's `setup.json` beside each fresh lease of a herdr server's watch, in order and once each, and nothing beside a stale lease or none, calling no herdr command for it (122, C.5; core.29).

#### Scenario: A fresh lease and a stale one

- **WHEN** one server's watch holds a fresh lease with a `setup.json` naming the popup's missing key, and another's lease is stale with a `setup.json` naming the missing `$flume` rows
- **THEN** the profile's setup is the first server's words alone, and no herdr command was run

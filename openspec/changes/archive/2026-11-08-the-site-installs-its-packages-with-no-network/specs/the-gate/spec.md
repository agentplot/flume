## ADDED Requirements

### Requirement: The site installs its packages with no network

`site:install` SHALL install `site/bun.lock`'s packages offline, from
bun's cache as `site/packages.nix` builds it from the integrity the lock
gives each package, which `devenv.nix` builds outside the sandbox and
gives as `FLUME_SITE_PACKAGES`, and SHALL refuse to run where that is
not given (record 0059).

#### Scenario: The install in the gate's sandbox

- **WHEN** `site:install` runs under `sluice exec`, in the gate's sandbox, whose network reaches localhost alone
- **THEN** bun installs every package the lock names for the host from the cache, asking no registry

#### Scenario: No packages given

- **WHEN** `sh scripts/install.sh` runs in `site/` with `FLUME_SITE_PACKAGES` unset
- **THEN** it exits non-zero saying the devenv shell sets it

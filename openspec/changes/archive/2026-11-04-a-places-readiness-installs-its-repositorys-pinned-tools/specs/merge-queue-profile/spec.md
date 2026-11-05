## ADDED Requirements

### Requirement: A moon repository's pins installed before its setup

Where the policy's runner is moon and the base's revision the policy is
read at holds a `.prototools`, the profile SHALL give the place profile
`proto install --config-mode upwards` as a readiness step of its own,
after the toolchain and before `moon setup`, run under the toolchain's
environment, as sluice's moon runner installs before any moon command;
it SHALL read the pins with the policy, never from a place's tree (11,
12, 20, 79, C.1, C.6).

#### Scenario: Pins nothing installed

- **WHEN** a place is made of a repository whose toolchain activates proto's pins and installs none, and whose `.prototools` pins moon, which nothing has installed
- **THEN** its readiness runs the toolchain, the install with its words and `moon setup`, each passing

#### Scenario: An install that fails

- **WHEN** proto cannot install what the repository pins
- **THEN** the readiness stops at the install, named with proto's words, and `moon setup` is not run (12)

#### Scenario: A repository that pins nothing

- **WHEN** the base's revision holds no `.prototools`
- **THEN** no install is run or named, and `moon setup` follows the toolchain

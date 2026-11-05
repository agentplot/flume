## MODIFIED Requirements

### Requirement: Readiness and teardown as the merge queue declares

The place profile SHALL run the readiness step and the teardown the
merge queue's profile declares, handed by the binary's resolver for the
revision the place's launch read, in the place, with an environment
cleared to the host user's login environment; a step's reading SHALL
give the readiness's steps; nothing of readiness SHALL be kept in the
place's git directory; with no merge queue bound nothing SHALL run (11,
12, 20, 66, 79, C.1, C.7, core.2).

#### Scenario: Readiness that fails at its toolchain

- **WHEN** a scratch repository's readiness step answers its toolchain failed with its words
- **THEN** ready gives the toolchain step and its words and no step after it, and nothing of the caller's environment reached the step (S16)

#### Scenario: No merge queue bound

- **WHEN** a place is made in a repository that binds no merge queue
- **THEN** ready runs nothing and remove runs no teardown step

#### Scenario: Readied at the revision the launch read

- **WHEN** a place was cut from main at R, main has since moved on in the repository, and the place is readied again for a retry
- **THEN** its readiness step is asked at R, not at main's revision then (12, 20)

#### Scenario: Nothing kept in the place's git directory

- **WHEN** a place's readiness passes
- **THEN** the place's git directory holds no `flume/toolchain.sh` and nothing else of the readiness's (C.7)

#### Scenario: A fresh clone's pins installed by the pinned sluice

- **WHEN** a scratch moon repository on main, whose `.prototools` pins a stand-in moon nothing has installed and whose toolchain puts a stand-in proto on the path and installs nothing, is cloned fresh, and a place of the clone is readied under the sluice the tree pins
- **THEN** ready gives `toolchain`, `proto install`, `moon --version` and `moon setup`, each passed, and the stand-in moon is installed where the stand-in proto installs (S89, 11, 79, C.1, C.6)

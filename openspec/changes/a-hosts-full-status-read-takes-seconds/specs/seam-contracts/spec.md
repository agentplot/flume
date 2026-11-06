## ADDED Requirements

### Requirement: The merge queue reads several branches at once

The merge queue seam SHALL answer read-submission for several branches
of one repository at once, each branch's newest submission in the
order the branches were asked, from one read of the merge queue; and
read-submission of one branch SHALL be that read of the one branch (99,
C.6).

#### Scenario: Two branches and one with nothing

- **WHEN** the fake merge queue holds a submission of `fix/one` and none of `fix/two`, and both are asked at once
- **THEN** the answer gives `fix/one` its submission and `fix/two` none, in that order, from one read (99)

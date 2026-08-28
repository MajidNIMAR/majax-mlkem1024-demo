# Governance

This repository is maintained as a public engineering project derived from the
research behind Majax Messenger. Its purpose is reproducible cryptographic work,
not disclosure of Majax production architecture.

## Maintainer

Majid NIMAR is the project maintainer and final reviewer for releases, security
corrections and changes to the cryptographic boundary. Additional maintainers
may be named in this file after a sustained record of reviewed contributions.

The maintainer may delegate reviews without transferring release authority.
No contributor receives access to production Majax systems through work on this
repository.

## Decisions

Routine documentation, tests and tooling changes are accepted through normal
pull-request review. A change affecting cryptographic behavior requires all of
the following material.

- a written problem statement and an explicit security boundary
- deterministic tests or external vectors where they exist
- negative and cross-parameter tests where relevant
- performance evidence when execution behavior changes
- an engineering note describing the decision and its limitations
- successful completion of every required continuous-integration job

Algorithm substitutions, new primitives and incompatible public API changes
must begin as a public proposal. Security-sensitive details belong in a private
security advisory until coordinated disclosure is appropriate.

## Review integrity

The author of a cryptographic change must provide reproducible evidence for the
claim being made. When only one maintainer is available, automated evidence and
the absence of an independent reviewer are recorded explicitly. A passing build
is not presented as an independent audit.

## Conflicts and reversals

The maintainer resolves technical disagreements using published evidence,
standards conformance and the narrowest defensible claim. A decision may be
reversed when new test results, standards guidance or security research justify
the change. The reversal is recorded in the changelog and, when substantial,
in an architecture decision note.

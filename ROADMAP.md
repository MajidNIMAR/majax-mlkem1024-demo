# Public roadmap

The ordered development record is preserved in `DEVELOPMENT.md`. The first
eight engineering stages established the library API, three parameter sets,
conformance and negative tests, assurance tooling, native performance backends,
reproducible releases, downstream integration and this maintenance process.

## Current public priorities

- obtain independent review of the typed API and its test boundary
- record a genuine external integration when immutable evidence exists
- expand machine-checked assurance without weakening reproducibility
- evaluate additional platform integrations through isolated consumers
- keep FIPS 203 vectors, dependency pins and release evidence current

## Candidate work

Potential work includes stronger formal models, broader deterministic vectors,
additional constant-time analysis and integration with a recognized security
framework. A candidate becomes committed work only after its scope, evidence
and maintenance cost have been reviewed.

No date is promised for candidate work. Cryptographic correctness and reviewable
evidence take precedence over release volume.

## Out of scope

The public roadmap does not expose Majax production services, operational
configuration, credentials, private protocol material or the five-domain
deployment. This repository remains independently testable without access to
those systems.

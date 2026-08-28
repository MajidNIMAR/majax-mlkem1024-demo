# Step 8, maintenance and transparency

## Scope

This step establishes the public process used after the engineering roadmap. It
does not alter the ML-KEM implementation or its runtime behavior.

## Project decisions

`GOVERNANCE.md` names the release authority and defines the additional evidence
required for cryptographic changes. `CONTRIBUTING.md` turns the existing test
suites into a contribution contract. `MAINTENANCE.md` defines branch support,
semantic versioning, dependency review, issue handling and claim discipline.

The policy records when an independent reviewer is absent. Automated checks are
valuable evidence, but they are not described as an audit.

## Repository controls

GitHub issue forms request reproducible defects, scoped proposals and external
adoption evidence. The pull-request template requires an explicit claim boundary
and test record. `CODEOWNERS` routes sensitive project surfaces to the named
maintainer.

Dependabot proposes pinned Cargo, Actions and Docker updates. Proposals remain
subject to human review and the complete evidence suite. They are never merged
automatically by this configuration.

## Continuing record

`ROADMAP.md` separates current priorities from uncommitted candidate work. The
release checklist defines the evidence expected for each tag. Changelog entries,
engineering notes and workflow results provide the durable public record.

## Evidence boundary

These files prove that a maintenance process is declared and mechanically
supported. They cannot prove future responsiveness or independent adoption.
Those properties require a continuing public history and external evidence.

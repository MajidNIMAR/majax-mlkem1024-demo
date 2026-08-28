# Contributing

Contributions are welcome when they preserve the repository's reproducible and
inspectable scope. Read `SECURITY.md` before reporting behavior that may expose
a vulnerability.

## Before writing code

Search existing issues and pull requests first. Open a proposal before changing
the public API, cryptographic behavior, dependency graph, release process or
supported platform policy. Small corrections and additional tests can proceed
directly to a pull request.

Do not include production credentials, endpoints, user data, private protocol
material or copied proprietary code.

## Required checks

Run the complete source verification.

```sh
sh scripts/test-all.sh
```

Run the Rust consumer contract after changing the library API.

```sh
sh scripts/test-integration.sh
```

Changes affecting assurance, performance or releases must also run the relevant
suite documented in `scripts/README.md`. Every committed shell script must pass
`sh -n`. Rust changes must pass formatting, Clippy with warnings denied and the
locked test suites used by continuous integration.

## Pull requests

Keep one technical purpose per pull request. Explain the observable behavior,
the evidence produced and the claim boundary. Include deterministic regression
coverage for a bug fix. Update `CHANGELOG.md` and the relevant engineering note
when behavior or public documentation changes.

Generated evidence belongs below `artifacts/` and remains outside version
control. Test vectors may be committed only with their origin, version, license
and expected property documented.

## Review outcome

A pull request may be accepted, returned for changes or closed when it conflicts
with the public scope. Acceptance does not certify the contribution for use in a
production protocol. Releases follow `MAINTENANCE.md` and remain an explicit
maintainer decision.

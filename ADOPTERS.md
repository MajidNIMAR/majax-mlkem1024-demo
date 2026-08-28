# Adoption evidence

This file records independently verifiable use of the library. It separates
working integration support from claims about third-party adoption.

## Current status

No independent external adopter has been verified yet.

The repository includes a standalone consumer under `examples/rust-consumer`.
Continuous integration builds it as a separate Cargo package and exercises all
three parameter sets. That fixture proves the integration contract works. It
does not count as external adoption because it is maintained in this project.

## Evidence required for a listing

An adopter is listed only when all of the following material is public or can
be reviewed by the maintainers.

- the consuming project and responsible organization are identifiable
- an immutable source revision shows the dependency and the integration code
- the integration reaches a released product, a recognized framework or a
  maintained security project
- the adopter confirms which library version or commit is in use
- the evidence can be checked without access to a Majax production system

Benchmarks, forks, experiments and copied source are useful feedback, but they
are not presented as adoption. Reviews and citations receive links with their
publication date and immutable identifier.

## Submitting evidence

Open an issue with the project URL, the exact revision and a short description
of the integration. Security-sensitive reports should follow `SECURITY.md`
instead. A maintainer will verify the evidence before changing this register.

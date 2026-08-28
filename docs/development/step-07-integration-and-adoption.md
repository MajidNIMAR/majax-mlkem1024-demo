# Step 7, integration contract and adoption evidence

## Scope

This step makes the Rust library consumable from a separate project and defines
how external adoption claims are verified. It does not alter ML-KEM operations,
parameter sets, backend selection or release artifacts.

## Downstream consumer

`examples/rust-consumer` is intentionally outside the engine package. Its
manifest depends on `majax-mlkem` through the same public boundary available to
another Rust project. One execution performs KeyGen, Encaps and Decaps for all
three FIPS 203 parameter sets and checks reciprocal secret agreement.

`scripts/test-integration.sh` builds the consumer in the digest-pinned Rust
environment. It also generates rustdoc for the public library. Both outputs are
uploaded by a dedicated CI job, which makes an accidental API break visible
without relying on the engine's own unit tests.

## Consumer guidance

`INTEGRATION.md` describes revision pinning, feature selection, algorithm
binding, secret lifetime and error handling. The example keeps protocol duties
outside the KEM boundary. Identity authentication, transcript binding, key
derivation and message encryption remain responsibilities of the consuming
protocol.

## Adoption register

`ADOPTERS.md` defines the evidence required before an external project is named.
The in-repository consumer is reported as integration evidence and never as an
independent adopter. At completion of this step, no external adoption claim is
made.

## Evidence boundary

Passing the consumer check proves that a separate Cargo package can use the
documented API at the tested revision. It does not prove that a third party has
deployed the library, reviewed its cryptography or adopted it in production.
Those statements require their own identifiable and immutable evidence.

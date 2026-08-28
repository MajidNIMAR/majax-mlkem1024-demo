# Step 2, unified ML-KEM parameter sets

## Purpose

The first development milestone created a typed ML-KEM-1024 library boundary.
This second milestone extends that boundary to every parameter set standardized
by FIPS 203 without creating three independent implementations or three
incompatible APIs.

ML-KEM-512, ML-KEM-768 and ML-KEM-1024 now use the same key generation,
encapsulation and decapsulation contract. Algorithm selection is explicit and
is carried by a closed Rust enumeration.

## Unified algorithm model

The new `Algorithm` type contains the three supported values.

- `MlKem512`
- `MlKem768`
- `MlKem1024`

Each value provides its standardized identifier and serialized object
dimensions. Identifier parsing accepts only the exact strings `ML-KEM-512`,
`ML-KEM-768` and `ML-KEM-1024`.

The multi-level API consists of three operations.

- `generate_keypair_for(algorithm)`
- `encapsulate_for(algorithm, public_key)`
- `decapsulate_for(algorithm, ciphertext, secret_key)`

The original `generate_keypair`, `encapsulate` and `decapsulate` functions
remain available and continue to select ML-KEM-1024. Existing callers therefore
retain their original behavior.

## Standardized dimensions

The implementation reports and tests the following serialized dimensions.

| Parameter set | Public key | Secret key | Ciphertext | Shared secret |
| --- | ---: | ---: | ---: | ---: |
| ML-KEM-512 | 800 bytes | 1632 bytes | 768 bytes | 32 bytes |
| ML-KEM-768 | 1184 bytes | 2400 bytes | 1088 bytes | 32 bytes |
| ML-KEM-1024 | 1568 bytes | 3168 bytes | 1568 bytes | 32 bytes |

## Cross-level isolation

Every operation requires the caller to select its parameter set. Serialized
objects are decoded by the corresponding backend type.

The test suite generates material independently for each level and attempts to
use every private key with the other two levels. These cross-level operations
are rejected before decapsulation because the serialized key does not match the
selected parameter set.

This test establishes separation between the three public API domains. It does
not claim that byte length alone is a complete semantic validation of arbitrary
FIPS 203 key material. Full conformance vectors and deeper malformed-key cases
belong to the next milestone.

## Implicit rejection

For every parameter set, the suite modifies a valid ciphertext and performs
decapsulation with the original private key. A correctly sized altered
ciphertext produces a pseudorandom secret that differs from the valid shared
secret. It does not expose a direct validity result through the API.

The authenticated-envelope integration test remains enabled for the historical
ML-KEM-1024 demonstrator path and confirms that the divergent secret cannot
authenticate the protected payload.

## JSON compatibility

The `gen`, `enc` and `dec` commands now accept an optional `algo` value for all
three standardized identifiers. Omitting `algo` continues to select
ML-KEM-1024.

Automated tests execute a complete JSON key generation, encapsulation and
decapsulation round trip for every level. An unknown identifier is rejected.

## Automated evidence

The following validation completed successfully in the digest-pinned Rust
environment.

- seven library tests across the three parameter sets
- three JSON command-contract tests
- one compiled end-to-end documentation example
- Clippy across all targets with warnings treated as errors
- Rust formatting verification
- the six existing Docker integration checks for ML-KEM-1024

The Docker integration path remained backward compatible and reported a
successful complete scenario in 15.622 milliseconds on the development host.
This number is an indicative functional observation rather than a comparative
benchmark.

## Security boundary

This milestone changes only the isolated public development engine. It does not
modify the five Majax production domains or expose any production key,
credential, endpoint or operational configuration.

Passing these tests does not constitute formal verification, FIPS validation or
a security audit. Deterministic vectors, ACVP evidence, fuzzing, constant-time
analysis and higher-assurance work remain separate milestones.

# Step 3 engineering record

## Deterministic conformance and negative tests

The commit is named `development: add deterministic conformance and negative tests`.

## Purpose

This step moves the development line beyond self-generated reciprocal tests.
It adds fixed external evidence for all three ML-KEM parameter sets, introduces
controlled deterministic test entry points and expands the negative matrix
around every encoded object accepted by the public API.

The stable demonstrator remains unchanged. No production Majax code, endpoint,
credential or operational configuration is included.

## Deterministic test interface

The `deterministic-testing` Cargo feature exposes deterministic KeyGen and
Encaps operations for test harnesses. KeyGen accepts the two 32-byte FIPS 203
inputs `d` and `z`. Encaps accepts the 32-byte input `m` together with a public
key. The feature is disabled by default and is not connected to the JSON CLI,
the HTTP service or the stable demonstrator.

The implementation calls the portable backend compiled by the pinned
`pqcrypto-mlkem` dependency. Seed material is supplied through fixed-size Rust
arrays. Temporary combined key-generation input is cleared immediately after
the native call.

Random production-style operations continue to use the normal public API.

## NIST ACVP sample evidence

One official key-generation sample and one official encapsulation sample are
retained for ML-KEM-512, ML-KEM-768 and ML-KEM-1024. They were mechanically
extracted from the NIST ACVP-Server sample files at commit
`975de31eb83d87039ec88934fdc47d8c312b892d`, tagged `v1.1.0.43`.

For KeyGen, the suite supplies `d` and `z` and compares the complete public and
private keys byte for byte. For Encaps, it supplies the official public key and
`m` input and compares the complete ciphertext and shared secret byte for byte.

## Independent fixtures

The repository now retains one implicit-rejection vector for ML-KEM-512,
ML-KEM-768 and ML-KEM-1024. The fixtures originate from the public C2SP CCTV
corpus and are pinned to commit
`1e3d2860d46e94e777e1b17c7a6f2436387e3ecc`.

Each fixture contains a complete decapsulation key, an adversarial ciphertext
and the expected 32-byte rejection secret. The integration test decapsulates
the fixed ciphertext twice and requires both results to match the external
expected value byte for byte. This catches accidental changes to the rejection
path and verifies that embedded zero bytes do not truncate a ciphertext
comparison.

## Negative matrix

The integration suite checks the following conditions for every parameter set.

- public keys one byte shorter and one byte longer than the standardized size
- private keys one byte shorter and one byte longer than the standardized size
- ciphertexts one byte shorter and one byte longer than the standardized size
- public keys presented to a different ML-KEM parameter set
- private keys and ciphertexts presented to a different parameter set
- bit alterations at the beginning, middle and end of a valid ciphertext
- repeatability of the pseudorandom secret produced by implicit rejection
- separation between the valid shared secret and every tested rejection secret

These checks are run through the same public Rust API used by the JSON adapter.
They therefore protect the library boundary instead of testing a duplicate
implementation.

## Evidence boundary

This stage establishes deterministic regression evidence and broad negative
coverage. It does not claim ACVP validation, FIPS 140 certification or complete
coverage of the FIPS 203 key-validation procedures. The retained NIST cases are
sample vectors rather than a complete ACVP campaign.

## Verification

The normal `cargo test --locked` command checks the default public surface. The
`cargo test --locked --features deterministic-testing` command adds NIST ACVP
KeyGen and Encaps comparisons. CI executes both modes together with formatting,
Clippy, JSON contract tests and the containerized source build.

Successful completion demonstrates the following properties within this
repository version.

- all three parameter sets match their pinned external rejection fixture
- deterministic KeyGen outputs match pinned NIST samples byte for byte
- deterministic Encaps outputs match pinned NIST samples byte for byte
- malformed lengths fail before a cryptographic operation is attempted
- encoded objects cannot cross the parameter-set boundary
- altered fixed-length ciphertexts follow a stable implicit-rejection path
- the original ML-KEM-1024 JSON behavior remains covered

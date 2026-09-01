# Changelog

All notable changes to the public Majax ML-KEM project are recorded here.

## Unreleased

### Main branch consolidation

- promoted the eight completed engineering milestones to `main`
- retained every milestone commit and its accompanying engineering note
- made `main` the canonical branch for testing, integration and future work

### Step 8, maintenance and transparency

- documented governance, contribution, support and maintenance policies
- added public roadmap and reproducible release checklist
- added structured defect, proposal and adoption-evidence forms
- added code ownership, pull-request review criteria and dependency monitoring

The complete engineering note is available in
[`docs/development/step-08-maintenance-and-transparency.md`](docs/development/step-08-maintenance-and-transparency.md).

### Step 7, integration contract and adoption evidence

- added a standalone Rust consumer for all three ML-KEM parameter sets
- added revision pinning, secret-lifetime and protocol-boundary guidance
- added generated public API documentation and downstream checks to CI
- added a public evidence policy for external adoption claims

The complete engineering note is available in
[`docs/development/step-07-integration-and-adoption.md`](docs/development/step-07-integration-and-adoption.md).

### Step 6, reproducible releases and provenance

- added byte-for-byte repeatability checks for independent release builds
- added SPDX and CycloneDX SBOMs for the source tree and runtime image
- added x86-64 and AArch64 release artifacts
- added keyless Sigstore bundles for every published artifact
- added GitHub build provenance attestations and consumer verification tooling

The complete engineering note is available in
[`docs/development/step-06-reproducible-releases.md`](docs/development/step-06-reproducible-releases.md).

### Step 5, portable and native performance backends

- exposed portable, x86-64 AVX2 and AArch64 NEON backend identities
- added portable-only and native-dispatch build policies
- added comparative KeyGen, Encaps and Decaps benchmarks
- added x86-64 and AArch64 performance evidence in CI

The complete engineering note is available in
[`docs/development/step-05-performance-backends.md`](docs/development/step-05-performance-backends.md).

### Step 4, fuzzing and higher-assurance evidence

- added three bounded libFuzzer targets for the complete typed API
- added malformed-object and cross-domain fuzz coverage
- added a dedicated Valgrind memory-safety driver
- added DudeCT timing distributions for decapsulation at every parameter set
- isolated the assurance toolchain in a digest-pinned Docker image
- added short CI campaigns and longer operator-controlled campaigns

The complete engineering note is available in
[`docs/development/step-04-fuzzing-and-higher-assurance.md`](docs/development/step-04-fuzzing-and-higher-assurance.md).

### Step 3, deterministic conformance and negative tests

- added feature-gated deterministic KeyGen and Encaps interfaces for testing
- matched pinned NIST ACVP key-generation vectors for all three parameter sets
- matched pinned NIST ACVP encapsulation vectors for all three parameter sets
- added pinned C2SP CCTV implicit-rejection vectors for all three parameter sets
- verified external expected rejection secrets byte for byte
- added short and extended length tests for every encoded object
- added cross-parameter public key, private key and ciphertext rejection tests
- checked altered ciphertexts at multiple positions for deterministic rejection
- documented the evidence boundary without claiming an ACVP validation

The complete engineering note is available in
[`docs/development/step-03-conformance-and-negative-tests.md`](docs/development/step-03-conformance-and-negative-tests.md).

### Step 2, unified ML-KEM parameter sets

- added ML-KEM-512 and ML-KEM-768 beside the existing ML-KEM-1024 path
- introduced one `Algorithm` selector and one typed API for all three levels
- preserved the original ML-KEM-1024 functions and JSON default behavior
- added standardized dimensions and identifier parsing for every level
- added reciprocal, malformed-input, implicit-rejection and cross-level tests
- extended the JSON command contract to accept all three algorithm identifiers

The complete engineering note is available in
[`docs/development/step-02-unified-parameter-sets.md`](docs/development/step-02-unified-parameter-sets.md).

### Step 1, typed ML-KEM-1024 library API

- separated the ML-KEM-1024 core from the JSON command-line adapter
- added typed key generation, encapsulation and decapsulation operations
- introduced explicit errors for malformed public keys, secret keys and ciphertexts
- added secret byte containers that wipe their memory when dropped
- preserved the existing `gen`, `enc` and `dec` JSON behavior
- added unit tests, a compiled documentation example and stricter CI checks

The complete engineering note is available in
[`docs/development/step-01-library-api.md`](docs/development/step-01-library-api.md).

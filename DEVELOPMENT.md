# Engineering record

The eight public development milestones are integrated into `main`. Their
individual commits and engineering notes remain available so reviewers can
inspect each stage independently.

The work started from the exact ML-KEM-1024 behavior verified by the original
demonstrator. Every extension preserves that behavior through automated
non-regression tests while expanding the supported surface.

## Completed milestones

The cryptographic core is exposed as a typed Rust library. One algorithm
selector provides the same operations for ML-KEM-512, ML-KEM-768 and
ML-KEM-1024. The JSON command-line interface remains an adapter over the same
public operations.

- `generate_keypair_for()` creates a fresh key pair for a selected level
- `encapsulate_for()` validates a public key and returns a ciphertext and secret
- `decapsulate_for()` validates its inputs and applies decapsulation
- legacy operations continue to select ML-KEM-1024
- secret byte containers are wiped when dropped
- malformed objects, altered ciphertexts and cross-level keys are tested
- pinned external rejection vectors cover all three parameter sets
- deterministic KeyGen and Encaps test interfaces are feature gated
- pinned NIST ACVP sample outputs are compared byte for byte
- boundary-length and cross-parameter negative matrices run in CI
- libFuzzer targets exercise round trips, malformed objects and domain separation
- Valgrind checks the dedicated assurance driver for memory errors
- DudeCT compares decapsulation timing distributions across secret-key classes
- portable builds exclude architecture-specific code through a feature boundary
- native builds dispatch to AVX2 or AArch64/NEON when the current CPU supports it
- comparative KeyGen, Encaps and Decaps measurements run on x86-64 and AArch64
- independent release builds must produce byte-for-byte identical binaries
- source and image SBOMs are emitted in SPDX and CycloneDX formats
- tagged artifacts receive keyless Sigstore signatures and GitHub provenance
- a separate Cargo consumer verifies the public integration contract
- public API documentation is generated and retained as CI evidence
- external adoption claims follow an evidence-based public register
- cryptographic changes follow a documented governance and review process
- releases, dependencies and public claims follow an explicit maintenance policy

## Ordered priorities

1. Extract and stabilize the typed ML-KEM-1024 library API, completed
2. Add all three parameter sets through one interface, completed
3. Expand deterministic, negative and conformance testing, completed
4. Add fuzzing, constant-time analysis and higher-assurance evidence, completed
5. Introduce portable and architecture-specific performance backends, completed
6. Strengthen reproducible releases, SBOMs and provenance, completed
7. Expand integration documentation and external adoption evidence, integration support completed; independent adoption pending
8. Establish the long-term maintenance and transparency process, completed

No production credential, endpoint, protocol secret or operational Majax
configuration belongs in this repository.

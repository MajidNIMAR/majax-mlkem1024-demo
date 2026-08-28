# Changelog

All notable changes to the Majax ML-KEM development line are recorded here.

The stable demonstrator history remains available on the `main` branch.

## Unreleased

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

# Changelog

All notable changes to the Majax ML-KEM development line are recorded here.

The stable demonstrator history remains available on the `main` branch.

## Unreleased

### Step 1, typed ML-KEM-1024 library API

- separated the ML-KEM-1024 core from the JSON command-line adapter
- added typed key generation, encapsulation and decapsulation operations
- introduced explicit errors for malformed public keys, secret keys and ciphertexts
- added secret byte containers that wipe their memory when dropped
- preserved the existing `gen`, `enc` and `dec` JSON behavior
- added unit tests, a compiled documentation example and stricter CI checks

The complete engineering note is available in
[`docs/development/step-01-library-api.md`](docs/development/step-01-library-api.md).

# Development line

This branch is the development line for the public Majax ML-KEM work. The
stable demonstrator remains available on `main` and retains its narrow,
reproducible scope.

Development starts from the exact ML-KEM-1024 behavior verified by the stable
demonstrator. Changes must preserve that behavior through automated
non-regression tests before extending the supported surface.

## Current milestone

The cryptographic core is now exposed as a typed Rust library. The JSON command
line interface is an adapter over the same public operations.

- `generate_keypair()` creates a fresh ML-KEM-1024 key pair
- `encapsulate()` validates a public key and returns a ciphertext and secret
- `decapsulate()` validates its inputs and applies standardized decapsulation
- secret byte containers are wiped when dropped
- malformed object lengths and altered ciphertext behavior are tested

## Ordered priorities

1. Stabilize the library contract and publish end-to-end API examples
2. Add ML-KEM-512 and ML-KEM-768 through the same typed interface
3. Expand negative, deterministic and conformance testing
4. Add continuous fuzzing and automated constant-time analysis
5. Introduce portable and architecture-specific performance backends
6. Strengthen reproducibility, release provenance and external review evidence

No production credential, endpoint, protocol secret or operational Majax
configuration belongs in this branch.

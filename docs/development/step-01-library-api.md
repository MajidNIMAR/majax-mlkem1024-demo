# Step 1, typed ML-KEM-1024 library API

## Purpose

The stable demonstrator originally kept cryptographic operations, input
validation and JSON command handling in one Rust executable. That structure was
appropriate for a compact demonstration, but it did not provide a clean
integration boundary for continued development.

This step separates those responsibilities while preserving the behavior of
the existing ML-KEM-1024 demonstrator. The cryptographic operations now live in
a typed Rust library. The command-line program remains available as a thin JSON
adapter over the same operations.

## Public contract introduced by this step

The library exposes three primary operations.

- `generate_keypair()` creates a fresh ML-KEM-1024 key pair
- `encapsulate()` validates a serialized public key and creates a ciphertext
  with its corresponding shared secret
- `decapsulate()` validates serialized inputs and derives the corresponding
  shared secret

The library also exposes the algorithm identifier and the standardized byte
dimensions for public keys, secret keys, ciphertexts and shared secrets.

The JSON interface continues to accept the existing `gen`, `enc` and `dec`
commands. Existing demonstrator clients therefore retain the same external
command contract.

## Secret handling

Private keys and shared secrets are held in a dedicated `SecretBytes` type.
When this container is dropped, every byte is overwritten through volatile
writes followed by a compiler fence. This reduces the lifetime of copied secret
material in ordinary process memory.

The explicit `expose()` method makes secret access visible at call sites. Its
documentation states that callers must not log, persist or otherwise disclose
the borrowed bytes.

This mechanism is a targeted memory hygiene measure. It is not a claim that all
copies created by the operating system, allocator, compiler or underlying
third-party implementation can be proven absent.

## Validation and implicit rejection

Malformed serialized objects are rejected before they reach a cryptographic
operation. Separate typed errors identify an invalid public key, secret key or
ciphertext.

A correctly sized but altered ML-KEM ciphertext follows standardized implicit
rejection. Decapsulation returns a pseudorandom secret rather than exposing a
ciphertext validity oracle. The regression test verifies that this result
differs from the secret produced by the original valid encapsulation.

## Automated evidence

The following checks were executed from the digest-pinned Rust and Node Docker
environments used by the repository.

```sh
cargo fmt -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
sh scripts/test-source-build.sh
```

The Rust test suite reports four passing unit tests and one passing compiled
documentation example.

The Docker integration path reports six passing checks.

- correct ML-KEM-1024 algorithm identifier
- standardized object dimensions
- reciprocal encapsulation and decapsulation
- hybrid protected-message round trip
- divergent secret after ciphertext alteration
- rejection of the altered ciphertext by the authenticated envelope

The measured development-host execution used for this milestone produced the
following indicative timings.

| Operation | Time |
| --- | ---: |
| Key generation | 4.222 ms |
| Encapsulation | 3.021 ms |
| Decapsulation | 3.287 ms |
| Complete integration scenario | 16.532 ms |

These measurements are functional observations from one host. They are not a
portable performance claim or a comparative benchmark.

## CI enforcement

The GitHub Actions workflow now checks Rust formatting, runs Clippy with all
warnings treated as errors and executes the unit and documentation tests before
the existing Docker verification paths.

The Rust environment remains selected by an immutable image digest. Dependency
versions and source checksums remain fixed by `engine/Cargo.lock`.

## Security boundary

This milestone contains no production key, credential, endpoint, user record or
Majax operational configuration. It does not expose the five production Majax
cryptographic domains.

The successful tests demonstrate only the properties explicitly exercised by
the repository. They do not constitute a security audit, formal proof, FIPS
validation or certification of Majax Messenger.

Support for ML-KEM-512 and ML-KEM-768, broader conformance vectors, fuzzing,
constant-time analysis and formal assurance remain separate development work.

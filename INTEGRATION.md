# Integrating the Rust library

The main branch exposes ML-KEM-512, ML-KEM-768 and ML-KEM-1024 through
one Rust API. The library is designed for explicit algorithm selection and
short-lived secret handling. It is not published to crates.io at this stage.

## Dependency policy

Production consumers should pin an exact reviewed Git revision and retain the
resulting `Cargo.lock`. A moving branch is suitable for evaluation, not for a
release build.

```toml
[dependencies]
majax-mlkem = { git = "https://github.com/MajidNIMAR/majax-mlkem1024-demo.git", rev = "REVISION_YOU_REVIEWED", package = "majax-mlkem", default-features = false }
```

The `native` default feature enables the upstream AVX2 or AArch64/NEON dispatch
where supported. Keeping `default-features = false` selects the portable
implementation and gives the most predictable deployment boundary.

## Complete exchange

```rust
use majax_mlkem::{decapsulate_for, encapsulate_for, generate_keypair_for, Algorithm};

fn exchange() -> Result<(), majax_mlkem::Error> {
    let algorithm = Algorithm::MlKem1024;
    let recipient = generate_keypair_for(algorithm);
    let sender = encapsulate_for(algorithm, &recipient.public_key)?;
    let recipient_secret = decapsulate_for(
        algorithm,
        &sender.ciphertext,
        recipient.secret_key.expose(),
    )?;

    assert_eq!(sender.shared_secret.expose(), recipient_secret.expose());
    Ok(())
}
```

`Algorithm::ALL` provides the three supported parameter sets. Each algorithm
also reports its standardized public-key, private-key, ciphertext and shared
secret dimensions.

## Secret lifetime

Private keys and shared secrets use `SecretBytes`. Their contents are wiped
with volatile writes when the owner is dropped. `expose()` borrows the bytes
for the immediate operation that consumes them. Applications must avoid logs,
serialization, persistence and unnecessary copies of that slice.

The public key and ciphertext are ordinary byte vectors. The caller owns their
transport encoding, protocol binding and authentication. ML-KEM establishes a
shared secret. It does not by itself authenticate identities or encrypt an
application message.

## Error handling

`encapsulate_for()` rejects a public key whose encoded length does not match the
selected parameter set. `decapsulate_for()` applies the corresponding checks to
the private key and ciphertext. A valid-length altered ciphertext follows the
FIPS 203 implicit-rejection behavior and therefore produces a pseudorandom
secret instead of a validity oracle.

Applications must bind the selected algorithm to their protocol transcript.
They must never infer the parameter set from untrusted input lengths. Keys and
ciphertexts from different parameter sets are deliberately incompatible.

## Reproducible consumer check

`examples/rust-consumer` is a separate Cargo package. It depends only on the
public library surface and completes an exchange with every parameter set. The
following command builds that consumer in the pinned Rust environment and also
generates the public API documentation.

```sh
sh scripts/test-integration.sh
```

Passing this check demonstrates that the documented public API is consumable
outside the engine package. It is integration evidence for this repository. It
is not evidence of independent adoption by another project.

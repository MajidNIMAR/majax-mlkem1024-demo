use majax_mlkem::{decapsulate_for, encapsulate_for, generate_keypair_for, Algorithm, Error};

#[cfg(feature = "deterministic-testing")]
use majax_mlkem::{encapsulate_deterministic, generate_keypair_deterministic};
#[cfg(feature = "deterministic-testing")]
use serde::Deserialize;

struct RejectionVector {
    algorithm: Algorithm,
    source: &'static str,
}

const REJECTION_VECTORS: [RejectionVector; 3] = [
    RejectionVector {
        algorithm: Algorithm::MlKem512,
        source: include_str!("vectors/cctv-ml-kem-512-strcmp.txt"),
    },
    RejectionVector {
        algorithm: Algorithm::MlKem768,
        source: include_str!("vectors/cctv-ml-kem-768-strcmp.txt"),
    },
    RejectionVector {
        algorithm: Algorithm::MlKem1024,
        source: include_str!("vectors/cctv-ml-kem-1024-strcmp.txt"),
    },
];

fn decode_hex(value: &str) -> Vec<u8> {
    assert_eq!(value.len() % 2, 0, "hex input must contain full bytes");
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let text = std::str::from_utf8(pair).expect("fixture contains ASCII hex");
            u8::from_str_radix(text, 16).expect("fixture contains valid hex")
        })
        .collect()
}

fn fixture_value(source: &str, name: &str) -> Vec<u8> {
    let prefix = format!("{name} = ");
    let value = source
        .lines()
        .find_map(|line| line.strip_prefix(&prefix))
        .unwrap_or_else(|| panic!("fixture is missing {name}"));
    decode_hex(value)
}

#[cfg(feature = "deterministic-testing")]
#[derive(Deserialize)]
struct AcvpCorpus {
    source: String,
    revision: String,
    vectors: Vec<AcvpVector>,
}

#[cfg(feature = "deterministic-testing")]
#[derive(Deserialize)]
struct AcvpVector {
    algorithm: String,
    keygen: AcvpKeygen,
    encapsulation: AcvpEncapsulation,
}

#[cfg(feature = "deterministic-testing")]
#[derive(Deserialize)]
struct AcvpKeygen {
    d: String,
    z: String,
    ek: String,
    dk: String,
}

#[cfg(feature = "deterministic-testing")]
#[derive(Deserialize)]
struct AcvpEncapsulation {
    ek: String,
    m: String,
    c: String,
    k: String,
}

#[cfg(feature = "deterministic-testing")]
fn seed(value: &str) -> [u8; 32] {
    decode_hex(value)
        .try_into()
        .expect("ACVP seed must contain exactly 32 bytes")
}

#[cfg(feature = "deterministic-testing")]
#[test]
fn nist_acvp_keygen_and_encapsulation_match_byte_for_byte() {
    let corpus: AcvpCorpus = serde_json::from_str(include_str!("vectors/nist-acvp-fips203.json"))
        .expect("the pinned ACVP fixture must be valid JSON");

    assert_eq!(corpus.source, "NIST ACVP-Server");
    assert_eq!(corpus.revision, "975de31eb83d87039ec88934fdc47d8c312b892d");
    assert_eq!(corpus.vectors.len(), Algorithm::ALL.len());

    for vector in corpus.vectors {
        let algorithm = Algorithm::from_identifier(&vector.algorithm)
            .expect("the ACVP parameter set must be supported");
        let keys = generate_keypair_deterministic(
            algorithm,
            &seed(&vector.keygen.d),
            &seed(&vector.keygen.z),
        )
        .expect("the deterministic backend must generate a key pair");

        assert_eq!(keys.public_key, decode_hex(&vector.keygen.ek));
        assert_eq!(keys.secret_key.expose(), decode_hex(&vector.keygen.dk));

        let acvp_public_key = decode_hex(&vector.encapsulation.ek);
        let encapsulated =
            encapsulate_deterministic(algorithm, &acvp_public_key, &seed(&vector.encapsulation.m))
                .expect("the deterministic backend must encapsulate");

        assert_eq!(encapsulated.ciphertext, decode_hex(&vector.encapsulation.c));
        assert_eq!(
            encapsulated.shared_secret.expose(),
            decode_hex(&vector.encapsulation.k)
        );
    }
}

#[test]
fn cctv_implicit_rejection_vectors_match_for_every_parameter_set() {
    for vector in REJECTION_VECTORS {
        let secret_key = fixture_value(vector.source, "dk");
        let ciphertext = fixture_value(vector.source, "c");
        let expected_secret = fixture_value(vector.source, "K");

        assert_eq!(secret_key.len(), vector.algorithm.secret_key_bytes());
        assert_eq!(ciphertext.len(), vector.algorithm.ciphertext_bytes());
        assert_eq!(
            expected_secret.len(),
            vector.algorithm.shared_secret_bytes()
        );

        let first = decapsulate_for(vector.algorithm, &ciphertext, &secret_key)
            .expect("a fixed-length ciphertext must use implicit rejection");
        let second = decapsulate_for(vector.algorithm, &ciphertext, &secret_key)
            .expect("the same vector must remain decapsulatable");

        assert_eq!(first.expose(), expected_secret);
        assert_eq!(second.expose(), expected_secret);
    }
}

#[test]
fn every_encoded_object_rejects_short_and_extended_lengths() {
    for algorithm in Algorithm::ALL {
        let keys = generate_keypair_for(algorithm);
        let encapsulated =
            encapsulate_for(algorithm, &keys.public_key).expect("generated key is valid");

        for invalid_length in [
            algorithm.public_key_bytes() - 1,
            algorithm.public_key_bytes() + 1,
        ] {
            assert_eq!(
                encapsulate_for(algorithm, &vec![0; invalid_length]),
                Err(Error::InvalidPublicKey)
            );
        }

        for invalid_length in [
            algorithm.secret_key_bytes() - 1,
            algorithm.secret_key_bytes() + 1,
        ] {
            assert_eq!(
                decapsulate_for(
                    algorithm,
                    &encapsulated.ciphertext,
                    &vec![0; invalid_length],
                ),
                Err(Error::InvalidSecretKey)
            );
        }

        for invalid_length in [
            algorithm.ciphertext_bytes() - 1,
            algorithm.ciphertext_bytes() + 1,
        ] {
            assert_eq!(
                decapsulate_for(
                    algorithm,
                    &vec![0; invalid_length],
                    keys.secret_key.expose(),
                ),
                Err(Error::InvalidCiphertext)
            );
        }
    }
}

#[test]
fn parameter_sets_cannot_exchange_encoded_objects() {
    for source_algorithm in Algorithm::ALL {
        let source_keys = generate_keypair_for(source_algorithm);
        let source_encapsulation = encapsulate_for(source_algorithm, &source_keys.public_key)
            .expect("generated key is valid");

        for target_algorithm in Algorithm::ALL {
            if source_algorithm == target_algorithm {
                continue;
            }

            assert_eq!(
                encapsulate_for(target_algorithm, &source_keys.public_key),
                Err(Error::InvalidPublicKey)
            );
            assert_eq!(
                decapsulate_for(
                    target_algorithm,
                    &source_encapsulation.ciphertext,
                    source_keys.secret_key.expose(),
                ),
                Err(Error::InvalidSecretKey)
            );
        }
    }
}

#[test]
fn altered_ciphertexts_produce_stable_rejection_secrets() {
    for algorithm in Algorithm::ALL {
        let keys = generate_keypair_for(algorithm);
        let encapsulated =
            encapsulate_for(algorithm, &keys.public_key).expect("generated key is valid");

        for position in [
            0,
            algorithm.ciphertext_bytes() / 2,
            algorithm.ciphertext_bytes() - 1,
        ] {
            let mut altered = encapsulated.ciphertext.clone();
            altered[position] ^= 0x80;

            let first = decapsulate_for(algorithm, &altered, keys.secret_key.expose())
                .expect("implicit rejection returns a pseudorandom secret");
            let second = decapsulate_for(algorithm, &altered, keys.secret_key.expose())
                .expect("the rejection path must be deterministic");

            assert_ne!(first.expose(), encapsulated.shared_secret.expose());
            assert_eq!(first.expose(), second.expose());
        }
    }
}

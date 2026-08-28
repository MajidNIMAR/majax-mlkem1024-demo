//! Typed ML-KEM operations for all three FIPS 203 parameter sets.
//!
//! ```
//! use majax_mlkem::{decapsulate_for, encapsulate_for, generate_keypair_for, Algorithm};
//!
//! let algorithm = Algorithm::MlKem1024;
//! let keys = generate_keypair_for(algorithm);
//! let encapsulated = encapsulate_for(algorithm, &keys.public_key)?;
//! let decapsulated = decapsulate_for(
//!     algorithm,
//!     &encapsulated.ciphertext,
//!     keys.secret_key.expose(),
//! )?;
//!
//! assert_eq!(encapsulated.shared_secret.expose(), decapsulated.expose());
//! # Ok::<(), majax_mlkem::Error>(())
//! ```

use std::fmt;

use pqcrypto_mlkem::{mlkem1024, mlkem512, mlkem768};
use pqcrypto_traits::kem::{
    Ciphertext as CiphertextTrait, PublicKey as PublicKeyTrait, SecretKey as SecretKeyTrait,
    SharedSecret as SharedSecretTrait,
};

pub const ALGORITHM: &str = "ML-KEM-1024";
pub const PUBLIC_KEY_BYTES: usize = mlkem1024::public_key_bytes();
pub const SECRET_KEY_BYTES: usize = mlkem1024::secret_key_bytes();
pub const CIPHERTEXT_BYTES: usize = mlkem1024::ciphertext_bytes();
pub const SHARED_SECRET_BYTES: usize = mlkem1024::shared_secret_bytes();

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    MlKem512,
    MlKem768,
    #[default]
    MlKem1024,
}

impl Algorithm {
    pub const ALL: [Self; 3] = [Self::MlKem512, Self::MlKem768, Self::MlKem1024];

    pub const fn identifier(self) -> &'static str {
        match self {
            Self::MlKem512 => "ML-KEM-512",
            Self::MlKem768 => "ML-KEM-768",
            Self::MlKem1024 => ALGORITHM,
        }
    }

    pub fn from_identifier(identifier: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|algorithm| algorithm.identifier() == identifier)
    }

    pub const fn public_key_bytes(self) -> usize {
        match self {
            Self::MlKem512 => mlkem512::public_key_bytes(),
            Self::MlKem768 => mlkem768::public_key_bytes(),
            Self::MlKem1024 => mlkem1024::public_key_bytes(),
        }
    }

    pub const fn secret_key_bytes(self) -> usize {
        match self {
            Self::MlKem512 => mlkem512::secret_key_bytes(),
            Self::MlKem768 => mlkem768::secret_key_bytes(),
            Self::MlKem1024 => mlkem1024::secret_key_bytes(),
        }
    }

    pub const fn ciphertext_bytes(self) -> usize {
        match self {
            Self::MlKem512 => mlkem512::ciphertext_bytes(),
            Self::MlKem768 => mlkem768::ciphertext_bytes(),
            Self::MlKem1024 => mlkem1024::ciphertext_bytes(),
        }
    }

    pub const fn shared_secret_bytes(self) -> usize {
        match self {
            Self::MlKem512 => mlkem512::shared_secret_bytes(),
            Self::MlKem768 => mlkem768::shared_secret_bytes(),
            Self::MlKem1024 => mlkem1024::shared_secret_bytes(),
        }
    }
}

/// Cryptographic implementation selected by this build on the current CPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// Portable PQClean C implementation.
    Portable,
    /// PQClean x86-64 implementation using AVX2 instructions.
    Avx2,
    /// PQClean AArch64 implementation using NEON instructions.
    Neon,
}

impl Backend {
    pub const fn identifier(self) -> &'static str {
        match self {
            Self::Portable => "portable-clean",
            Self::Avx2 => "x86_64-avx2",
            Self::Neon => "aarch64-neon",
        }
    }
}

/// Reports the backend that the upstream dispatcher will use on this CPU.
///
/// Builds made without the `native` feature always report and use the portable
/// implementation. Native builds retain runtime AVX2 detection on x86-64.
pub fn active_backend() -> Backend {
    #[cfg(all(feature = "native", target_arch = "x86_64"))]
    if std::is_x86_feature_detected!("avx2") {
        return Backend::Avx2;
    }

    #[cfg(all(feature = "native", target_arch = "aarch64"))]
    {
        return Backend::Neon;
    }

    Backend::Portable
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidPublicKey,
    InvalidSecretKey,
    InvalidCiphertext,
    BackendFailure,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidPublicKey => "invalid public key for the selected ML-KEM level",
            Self::InvalidSecretKey => "invalid private key for the selected ML-KEM level",
            Self::InvalidCiphertext => "invalid ciphertext for the selected ML-KEM level",
            Self::BackendFailure => "the ML-KEM backend rejected the operation",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, PartialEq, Eq)]
pub struct SecretBytes(Vec<u8>);

impl SecretBytes {
    fn new(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    /// Borrows the secret for an immediate cryptographic operation.
    ///
    /// Callers must not log, persist or otherwise expose the returned bytes.
    pub fn expose(&self) -> &[u8] {
        &self.0
    }
}

impl Drop for SecretBytes {
    fn drop(&mut self) {
        for byte in &mut self.0 {
            unsafe { std::ptr::write_volatile(byte, 0) };
        }
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct KeyPair {
    pub algorithm: Algorithm,
    pub public_key: Vec<u8>,
    pub secret_key: SecretBytes,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Encapsulation {
    pub algorithm: Algorithm,
    pub ciphertext: Vec<u8>,
    pub shared_secret: SecretBytes,
}

#[cfg(feature = "deterministic-testing")]
mod deterministic {
    use super::{Algorithm, Encapsulation, Error, KeyPair, SecretBytes};
    use std::ffi::c_int;

    macro_rules! declare_derandomized_backend {
        ($library:literal, $keypair:ident, $encapsulate:ident) => {
            #[link(name = $library)]
            unsafe extern "C" {
                fn $keypair(public_key: *mut u8, secret_key: *mut u8, coins: *const u8) -> c_int;
                fn $encapsulate(
                    ciphertext: *mut u8,
                    shared_secret: *mut u8,
                    public_key: *const u8,
                    coins: *const u8,
                ) -> c_int;
            }
        };
    }

    declare_derandomized_backend!(
        "ml-kem-512_clean",
        PQCLEAN_MLKEM512_CLEAN_crypto_kem_keypair_derand,
        PQCLEAN_MLKEM512_CLEAN_crypto_kem_enc_derand
    );
    declare_derandomized_backend!(
        "ml-kem-768_clean",
        PQCLEAN_MLKEM768_CLEAN_crypto_kem_keypair_derand,
        PQCLEAN_MLKEM768_CLEAN_crypto_kem_enc_derand
    );
    declare_derandomized_backend!(
        "ml-kem-1024_clean",
        PQCLEAN_MLKEM1024_CLEAN_crypto_kem_keypair_derand,
        PQCLEAN_MLKEM1024_CLEAN_crypto_kem_enc_derand
    );

    type KeypairFunction = unsafe extern "C" fn(*mut u8, *mut u8, *const u8) -> c_int;
    type EncapsulationFunction =
        unsafe extern "C" fn(*mut u8, *mut u8, *const u8, *const u8) -> c_int;

    fn keypair_function(algorithm: Algorithm) -> KeypairFunction {
        match algorithm {
            Algorithm::MlKem512 => PQCLEAN_MLKEM512_CLEAN_crypto_kem_keypair_derand,
            Algorithm::MlKem768 => PQCLEAN_MLKEM768_CLEAN_crypto_kem_keypair_derand,
            Algorithm::MlKem1024 => PQCLEAN_MLKEM1024_CLEAN_crypto_kem_keypair_derand,
        }
    }

    fn encapsulation_function(algorithm: Algorithm) -> EncapsulationFunction {
        match algorithm {
            Algorithm::MlKem512 => PQCLEAN_MLKEM512_CLEAN_crypto_kem_enc_derand,
            Algorithm::MlKem768 => PQCLEAN_MLKEM768_CLEAN_crypto_kem_enc_derand,
            Algorithm::MlKem1024 => PQCLEAN_MLKEM1024_CLEAN_crypto_kem_enc_derand,
        }
    }

    pub(super) fn generate(
        algorithm: Algorithm,
        d: &[u8; 32],
        z: &[u8; 32],
    ) -> Result<KeyPair, Error> {
        let mut public_key = vec![0; algorithm.public_key_bytes()];
        let mut secret_key = vec![0; algorithm.secret_key_bytes()];
        let mut coins = [0; 64];
        coins[..32].copy_from_slice(d);
        coins[32..].copy_from_slice(z);

        // The output buffers and fixed-size seed remain valid for the full FFI call.
        let status = unsafe {
            keypair_function(algorithm)(
                public_key.as_mut_ptr(),
                secret_key.as_mut_ptr(),
                coins.as_ptr(),
            )
        };
        coins.fill(0);
        if status != 0 {
            secret_key.fill(0);
            return Err(Error::BackendFailure);
        }

        let secret_key = SecretBytes(secret_key);
        Ok(KeyPair {
            algorithm,
            public_key,
            secret_key,
        })
    }

    pub(super) fn encapsulate(
        algorithm: Algorithm,
        public_key: &[u8],
        m: &[u8; 32],
    ) -> Result<Encapsulation, Error> {
        if public_key.len() != algorithm.public_key_bytes() {
            return Err(Error::InvalidPublicKey);
        }

        let mut ciphertext = vec![0; algorithm.ciphertext_bytes()];
        let mut shared_secret = vec![0; algorithm.shared_secret_bytes()];
        // The key, seed and output buffers remain valid for the full FFI call.
        let status = unsafe {
            encapsulation_function(algorithm)(
                ciphertext.as_mut_ptr(),
                shared_secret.as_mut_ptr(),
                public_key.as_ptr(),
                m.as_ptr(),
            )
        };
        if status != 0 {
            shared_secret.fill(0);
            return Err(Error::BackendFailure);
        }

        Ok(Encapsulation {
            algorithm,
            ciphertext,
            shared_secret: SecretBytes(shared_secret),
        })
    }
}

macro_rules! generate_with {
    ($module:ident, $algorithm:expr) => {{
        let (public_key, secret_key) = $module::keypair();
        KeyPair {
            algorithm: $algorithm,
            public_key: public_key.as_bytes().to_vec(),
            secret_key: SecretBytes::new(secret_key.as_bytes()),
        }
    }};
}

macro_rules! encapsulate_with {
    ($module:ident, $algorithm:expr, $public_key:expr) => {{
        let public_key =
            $module::PublicKey::from_bytes($public_key).map_err(|_| Error::InvalidPublicKey)?;
        let (shared_secret, ciphertext) = $module::encapsulate(&public_key);
        Ok(Encapsulation {
            algorithm: $algorithm,
            ciphertext: ciphertext.as_bytes().to_vec(),
            shared_secret: SecretBytes::new(shared_secret.as_bytes()),
        })
    }};
}

macro_rules! decapsulate_with {
    ($module:ident, $ciphertext:expr, $secret_key:expr) => {{
        let secret_key =
            $module::SecretKey::from_bytes($secret_key).map_err(|_| Error::InvalidSecretKey)?;
        let ciphertext =
            $module::Ciphertext::from_bytes($ciphertext).map_err(|_| Error::InvalidCiphertext)?;
        let shared_secret = $module::decapsulate(&ciphertext, &secret_key);
        Ok(SecretBytes::new(shared_secret.as_bytes()))
    }};
}

pub fn generate_keypair() -> KeyPair {
    generate_keypair_for(Algorithm::MlKem1024)
}

pub fn generate_keypair_for(algorithm: Algorithm) -> KeyPair {
    match algorithm {
        Algorithm::MlKem512 => generate_with!(mlkem512, algorithm),
        Algorithm::MlKem768 => generate_with!(mlkem768, algorithm),
        Algorithm::MlKem1024 => generate_with!(mlkem1024, algorithm),
    }
}

/// Generates an ML-KEM key pair from the FIPS 203 `d` and `z` inputs.
///
/// This deterministic entry point exists for conformance testing only. It is
/// unavailable unless the `deterministic-testing` feature is enabled.
#[cfg(feature = "deterministic-testing")]
pub fn generate_keypair_deterministic(
    algorithm: Algorithm,
    d: &[u8; 32],
    z: &[u8; 32],
) -> Result<KeyPair, Error> {
    deterministic::generate(algorithm, d, z)
}

pub fn encapsulate(public_key: &[u8]) -> Result<Encapsulation, Error> {
    encapsulate_for(Algorithm::MlKem1024, public_key)
}

pub fn encapsulate_for(algorithm: Algorithm, public_key: &[u8]) -> Result<Encapsulation, Error> {
    match algorithm {
        Algorithm::MlKem512 => encapsulate_with!(mlkem512, algorithm, public_key),
        Algorithm::MlKem768 => encapsulate_with!(mlkem768, algorithm, public_key),
        Algorithm::MlKem1024 => encapsulate_with!(mlkem1024, algorithm, public_key),
    }
}

/// Encapsulates from the FIPS 203 deterministic `m` input.
///
/// This deterministic entry point exists for conformance testing only. It is
/// unavailable unless the `deterministic-testing` feature is enabled.
#[cfg(feature = "deterministic-testing")]
pub fn encapsulate_deterministic(
    algorithm: Algorithm,
    public_key: &[u8],
    m: &[u8; 32],
) -> Result<Encapsulation, Error> {
    deterministic::encapsulate(algorithm, public_key, m)
}

pub fn decapsulate(ciphertext: &[u8], secret_key: &[u8]) -> Result<SecretBytes, Error> {
    decapsulate_for(Algorithm::MlKem1024, ciphertext, secret_key)
}

pub fn decapsulate_for(
    algorithm: Algorithm,
    ciphertext: &[u8],
    secret_key: &[u8],
) -> Result<SecretBytes, Error> {
    match algorithm {
        Algorithm::MlKem512 => decapsulate_with!(mlkem512, ciphertext, secret_key),
        Algorithm::MlKem768 => decapsulate_with!(mlkem768, ciphertext, secret_key),
        Algorithm::MlKem1024 => decapsulate_with!(mlkem1024, ciphertext, secret_key),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_standardized_dimensions() {
        let expected = [
            (Algorithm::MlKem512, 800, 1632, 768, 32),
            (Algorithm::MlKem768, 1184, 2400, 1088, 32),
            (Algorithm::MlKem1024, 1568, 3168, 1568, 32),
        ];
        for (algorithm, public, secret, ciphertext, shared) in expected {
            assert_eq!(algorithm.public_key_bytes(), public);
            assert_eq!(algorithm.secret_key_bytes(), secret);
            assert_eq!(algorithm.ciphertext_bytes(), ciphertext);
            assert_eq!(algorithm.shared_secret_bytes(), shared);
        }
    }

    #[test]
    fn identifiers_round_trip() {
        for algorithm in Algorithm::ALL {
            assert_eq!(
                Algorithm::from_identifier(algorithm.identifier()),
                Some(algorithm)
            );
        }
        assert_eq!(Algorithm::from_identifier("ML-KEM-999"), None);
    }

    #[test]
    fn backend_identifier_matches_the_compiled_dispatch_policy() {
        let backend = active_backend();
        assert!(matches!(
            backend,
            Backend::Portable | Backend::Avx2 | Backend::Neon
        ));

        #[cfg(not(feature = "native"))]
        assert_eq!(backend, Backend::Portable);
    }

    #[test]
    fn every_parameter_set_is_reciprocal() {
        for algorithm in Algorithm::ALL {
            let keys = generate_keypair_for(algorithm);
            let encapsulated =
                encapsulate_for(algorithm, &keys.public_key).expect("valid public key");
            let decapsulated = decapsulate_for(
                algorithm,
                &encapsulated.ciphertext,
                keys.secret_key.expose(),
            )
            .expect("valid ciphertext and private key");
            assert_eq!(encapsulated.algorithm, algorithm);
            assert_eq!(encapsulated.shared_secret.expose(), decapsulated.expose());
        }
    }

    #[test]
    fn malformed_objects_are_rejected() {
        for algorithm in Algorithm::ALL {
            assert_eq!(
                encapsulate_for(algorithm, &[0; 12]),
                Err(Error::InvalidPublicKey)
            );
            assert_eq!(
                decapsulate_for(algorithm, &vec![0; algorithm.ciphertext_bytes()], &[0; 12]),
                Err(Error::InvalidSecretKey)
            );
            assert_eq!(
                decapsulate_for(algorithm, &[0; 12], &vec![0; algorithm.secret_key_bytes()]),
                Err(Error::InvalidCiphertext)
            );
        }
    }

    #[test]
    fn cross_parameter_secret_keys_are_rejected() {
        for algorithm in Algorithm::ALL {
            let keys = generate_keypair_for(algorithm);
            let encapsulated =
                encapsulate_for(algorithm, &keys.public_key).expect("valid public key");
            for other in Algorithm::ALL {
                if other == algorithm {
                    continue;
                }
                let other_keys = generate_keypair_for(other);
                assert_eq!(
                    decapsulate_for(
                        algorithm,
                        &encapsulated.ciphertext,
                        other_keys.secret_key.expose(),
                    ),
                    Err(Error::InvalidSecretKey)
                );
            }
        }
    }

    #[test]
    fn altered_ciphertext_uses_implicit_rejection() {
        for algorithm in Algorithm::ALL {
            let keys = generate_keypair_for(algorithm);
            let encapsulated =
                encapsulate_for(algorithm, &keys.public_key).expect("valid public key");
            let mut altered = encapsulated.ciphertext.clone();
            altered[0] ^= 1;
            let rejected_secret = decapsulate_for(algorithm, &altered, keys.secret_key.expose())
                .expect("implicit rejection returns a pseudorandom secret");
            assert_ne!(
                encapsulated.shared_secret.expose(),
                rejected_secret.expose()
            );
        }
    }

    #[test]
    fn legacy_operations_remain_ml_kem_1024() {
        let keys = generate_keypair();
        let encapsulated = encapsulate(&keys.public_key).expect("valid public key");
        let decapsulated = decapsulate(&encapsulated.ciphertext, keys.secret_key.expose())
            .expect("valid ciphertext and private key");
        assert_eq!(keys.algorithm, Algorithm::MlKem1024);
        assert_eq!(encapsulated.algorithm, Algorithm::MlKem1024);
        assert_eq!(encapsulated.shared_secret.expose(), decapsulated.expose());
    }
}

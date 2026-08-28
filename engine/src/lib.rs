//! Typed ML-KEM-1024 operations used by the development CLI.
//!
//! ```
//! use majax_mlkem::{decapsulate, encapsulate, generate_keypair};
//!
//! let keys = generate_keypair();
//! let encapsulated = encapsulate(&keys.public_key)?;
//! let decapsulated = decapsulate(
//!     &encapsulated.ciphertext,
//!     keys.secret_key.expose(),
//! )?;
//!
//! assert_eq!(
//!     encapsulated.shared_secret.expose(),
//!     decapsulated.expose(),
//! );
//! # Ok::<(), majax_mlkem::Error>(())
//! ```

use std::fmt;

use pqcrypto_mlkem::mlkem1024;
use pqcrypto_traits::kem::{
    Ciphertext as CiphertextTrait, PublicKey as PublicKeyTrait, SecretKey as SecretKeyTrait,
    SharedSecret as SharedSecretTrait,
};

pub const ALGORITHM: &str = "ML-KEM-1024";
/// Size in bytes of a serialized ML-KEM-1024 public key.
pub const PUBLIC_KEY_BYTES: usize = mlkem1024::public_key_bytes();
/// Size in bytes of a serialized ML-KEM-1024 secret key.
pub const SECRET_KEY_BYTES: usize = mlkem1024::secret_key_bytes();
/// Size in bytes of a serialized ML-KEM-1024 ciphertext.
pub const CIPHERTEXT_BYTES: usize = mlkem1024::ciphertext_bytes();
/// Size in bytes of an ML-KEM shared secret.
pub const SHARED_SECRET_BYTES: usize = mlkem1024::shared_secret_bytes();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidPublicKey,
    InvalidSecretKey,
    InvalidCiphertext,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidPublicKey => "invalid ML-KEM-1024 public key",
            Self::InvalidSecretKey => "invalid ML-KEM-1024 private key",
            Self::InvalidCiphertext => "invalid ML-KEM-1024 ciphertext",
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
            // Volatile writes prevent this explicit wipe from being optimized away.
            unsafe { std::ptr::write_volatile(byte, 0) };
        }
        std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct KeyPair {
    pub public_key: Vec<u8>,
    pub secret_key: SecretBytes,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Encapsulation {
    pub ciphertext: Vec<u8>,
    pub shared_secret: SecretBytes,
}

/// Generates a fresh ML-KEM-1024 key pair.
pub fn generate_keypair() -> KeyPair {
    let (public_key, secret_key) = mlkem1024::keypair();
    KeyPair {
        public_key: public_key.as_bytes().to_vec(),
        secret_key: SecretBytes::new(secret_key.as_bytes()),
    }
}

/// Encapsulates a fresh shared secret for a serialized public key.
pub fn encapsulate(public_key: &[u8]) -> Result<Encapsulation, Error> {
    let public_key =
        mlkem1024::PublicKey::from_bytes(public_key).map_err(|_| Error::InvalidPublicKey)?;
    let (shared_secret, ciphertext) = mlkem1024::encapsulate(&public_key);
    Ok(Encapsulation {
        ciphertext: ciphertext.as_bytes().to_vec(),
        shared_secret: SecretBytes::new(shared_secret.as_bytes()),
    })
}

/// Decapsulates a ciphertext with a serialized secret key.
///
/// A correctly sized but altered ciphertext follows ML-KEM implicit rejection
/// and yields a pseudorandom secret instead of a validity oracle.
pub fn decapsulate(ciphertext: &[u8], secret_key: &[u8]) -> Result<SecretBytes, Error> {
    let secret_key =
        mlkem1024::SecretKey::from_bytes(secret_key).map_err(|_| Error::InvalidSecretKey)?;
    let ciphertext =
        mlkem1024::Ciphertext::from_bytes(ciphertext).map_err(|_| Error::InvalidCiphertext)?;
    let shared_secret = mlkem1024::decapsulate(&ciphertext, &secret_key);
    Ok(SecretBytes::new(shared_secret.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_standardized_dimensions() {
        assert_eq!(PUBLIC_KEY_BYTES, 1568);
        assert_eq!(SECRET_KEY_BYTES, 3168);
        assert_eq!(CIPHERTEXT_BYTES, 1568);
        assert_eq!(SHARED_SECRET_BYTES, 32);
    }

    #[test]
    fn encapsulation_and_decapsulation_are_reciprocal() {
        let keys = generate_keypair();
        let encapsulated = encapsulate(&keys.public_key).expect("valid public key");
        let decapsulated = decapsulate(&encapsulated.ciphertext, keys.secret_key.expose())
            .expect("valid ciphertext and private key");

        assert_eq!(encapsulated.shared_secret.expose(), decapsulated.expose());
    }

    #[test]
    fn malformed_objects_are_rejected() {
        assert_eq!(encapsulate(&[0; 12]), Err(Error::InvalidPublicKey));
        assert_eq!(
            decapsulate(&[0; CIPHERTEXT_BYTES], &[0; 12]),
            Err(Error::InvalidSecretKey)
        );
        assert_eq!(
            decapsulate(&[0; 12], &[0; SECRET_KEY_BYTES]),
            Err(Error::InvalidCiphertext)
        );
    }

    #[test]
    fn altered_ciphertext_uses_implicit_rejection() {
        let keys = generate_keypair();
        let encapsulated = encapsulate(&keys.public_key).expect("valid public key");
        let mut altered = encapsulated.ciphertext.clone();
        altered[0] ^= 1;
        let rejected_secret = decapsulate(&altered, keys.secret_key.expose())
            .expect("standardized implicit rejection returns a pseudorandom secret");

        assert_ne!(
            encapsulated.shared_secret.expose(),
            rejected_secret.expose()
        );
    }
}

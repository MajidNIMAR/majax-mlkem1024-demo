use majax_mlkem::{decapsulate_for, encapsulate_for, generate_keypair_for, Algorithm, Error};

fn main() {
    for algorithm in Algorithm::ALL {
        let keys = generate_keypair_for(algorithm);
        let encapsulated = encapsulate_for(algorithm, &keys.public_key).expect("valid public key");
        let recovered = decapsulate_for(
            algorithm,
            &encapsulated.ciphertext,
            keys.secret_key.expose(),
        )
        .expect("valid private key and ciphertext");
        assert_eq!(encapsulated.shared_secret.expose(), recovered.expose());

        let mut altered = encapsulated.ciphertext;
        let middle = altered.len() / 2;
        altered[middle] ^= 0x80;
        let rejected = decapsulate_for(algorithm, &altered, keys.secret_key.expose())
            .expect("implicit rejection returns a pseudorandom secret");
        assert_ne!(encapsulated.shared_secret.expose(), rejected.expose());

        assert_eq!(
            encapsulate_for(algorithm, &[0; 31]),
            Err(Error::InvalidPublicKey)
        );
        assert_eq!(
            decapsulate_for(algorithm, &[0; 31], keys.secret_key.expose()),
            Err(Error::InvalidCiphertext)
        );
        assert_eq!(
            decapsulate_for(algorithm, &vec![0; algorithm.ciphertext_bytes()], &[0; 31],),
            Err(Error::InvalidSecretKey)
        );
    }
}

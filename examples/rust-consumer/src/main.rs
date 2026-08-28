use majax_mlkem::{decapsulate_for, encapsulate_for, generate_keypair_for, Algorithm};

fn main() -> Result<(), majax_mlkem::Error> {
    for algorithm in Algorithm::ALL {
        let keys = generate_keypair_for(algorithm);
        let encapsulated = encapsulate_for(algorithm, &keys.public_key)?;
        let decapsulated = decapsulate_for(
            algorithm,
            &encapsulated.ciphertext,
            keys.secret_key.expose(),
        )?;

        assert_eq!(encapsulated.algorithm, algorithm);
        assert_eq!(keys.public_key.len(), algorithm.public_key_bytes());
        assert_eq!(encapsulated.ciphertext.len(), algorithm.ciphertext_bytes());
        assert_eq!(encapsulated.shared_secret.expose(), decapsulated.expose());

        println!(
            "{} integration round trip passed with the {} backend",
            algorithm.identifier(),
            majax_mlkem::active_backend().identifier()
        );
    }

    Ok(())
}

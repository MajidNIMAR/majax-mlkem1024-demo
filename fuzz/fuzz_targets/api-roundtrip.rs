#![no_main]

use libfuzzer_sys::fuzz_target;
use majax_mlkem::{
    decapsulate_for, encapsulate_deterministic, generate_keypair_deterministic, Algorithm,
};

fn algorithm(selector: u8) -> Algorithm {
    Algorithm::ALL[usize::from(selector % 3)]
}

fn expand(data: &[u8], offset: usize) -> [u8; 32] {
    let mut output = [0; 32];
    for (index, byte) in output.iter_mut().enumerate() {
        *byte = data[(offset + index) % data.len()];
    }
    output
}

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    let algorithm = algorithm(data[0]);
    let d = expand(data, 1);
    let z = expand(data, 33);
    let m = expand(data, 65);
    let keys = generate_keypair_deterministic(algorithm, &d, &z).expect("valid seeds");
    let encapsulated =
        encapsulate_deterministic(algorithm, &keys.public_key, &m).expect("valid public key");
    let recovered = decapsulate_for(
        algorithm,
        &encapsulated.ciphertext,
        keys.secret_key.expose(),
    )
    .expect("valid ciphertext and private key");

    assert_eq!(encapsulated.shared_secret.expose(), recovered.expose());
});

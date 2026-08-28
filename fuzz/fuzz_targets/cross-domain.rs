#![no_main]

use libfuzzer_sys::fuzz_target;
use majax_mlkem::{
    decapsulate_for, encapsulate_deterministic, generate_keypair_deterministic, Algorithm, Error,
};

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

    let d = expand(data, 0);
    let z = expand(data, 32);
    let m = expand(data, 64);

    for source in Algorithm::ALL {
        let keys = generate_keypair_deterministic(source, &d, &z).expect("valid seeds");
        let encapsulated =
            encapsulate_deterministic(source, &keys.public_key, &m).expect("valid key");

        for target in Algorithm::ALL {
            if source == target {
                continue;
            }
            assert_eq!(
                decapsulate_for(target, &encapsulated.ciphertext, keys.secret_key.expose()),
                Err(Error::InvalidSecretKey)
            );
        }
    }
});

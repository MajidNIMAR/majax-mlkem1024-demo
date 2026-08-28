#![no_main]

use libfuzzer_sys::fuzz_target;
use majax_mlkem::{decapsulate_for, encapsulate_for, Algorithm};

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    let algorithm = Algorithm::ALL[usize::from(data[0] % 3)];
    let body = &data[1..];
    let public_end = body.len().min(algorithm.public_key_bytes() + 1);
    let ciphertext_end = body.len().min(algorithm.ciphertext_bytes() + 1);
    let secret_end = body.len().min(algorithm.secret_key_bytes() + 1);

    let _ = encapsulate_for(algorithm, &body[..public_end]);
    let _ = decapsulate_for(algorithm, &body[..ciphertext_end], &body[..secret_end]);
});

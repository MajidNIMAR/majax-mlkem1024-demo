use std::hint::black_box;

use dudect_bencher::{ctbench_main, rand::RngExt, BenchRng, Class, CtRunner};
use majax_mlkem::{decapsulate_for, encapsulate_for, generate_keypair_for, Algorithm};

const SAMPLES_PER_LEVEL: usize = 8_000;

fn measure(algorithm: Algorithm, runner: &mut CtRunner, rng: &mut BenchRng) {
    let keys = generate_keypair_for(algorithm);
    let encapsulated = encapsulate_for(algorithm, &keys.public_key).expect("valid public key");
    let mut altered = encapsulated.ciphertext.clone();
    let middle = altered.len() / 2;
    altered[middle] ^= 0x80;

    for _ in 0..SAMPLES_PER_LEVEL {
        let valid_class = rng.random::<bool>();
        let class = if valid_class {
            Class::Left
        } else {
            Class::Right
        };
        let ciphertext = if valid_class {
            &encapsulated.ciphertext
        } else {
            &altered
        };

        runner.run_one(class, || {
            black_box(
                decapsulate_for(algorithm, ciphertext, keys.secret_key.expose())
                    .expect("fixed-size inputs must decapsulate"),
            )
        });
    }
}

fn decapsulation_mlkem512(runner: &mut CtRunner, rng: &mut BenchRng) {
    measure(Algorithm::MlKem512, runner, rng);
}

fn decapsulation_mlkem768(runner: &mut CtRunner, rng: &mut BenchRng) {
    measure(Algorithm::MlKem768, runner, rng);
}

fn decapsulation_mlkem1024(runner: &mut CtRunner, rng: &mut BenchRng) {
    measure(Algorithm::MlKem1024, runner, rng);
}

ctbench_main!(
    decapsulation_mlkem512,
    decapsulation_mlkem768,
    decapsulation_mlkem1024
);

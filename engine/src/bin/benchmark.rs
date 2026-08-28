use std::hint::black_box;
use std::time::{Duration, Instant};

use majax_mlkem::{
    active_backend, decapsulate_for, encapsulate_for, generate_keypair_for, Algorithm,
};
use serde::Serialize;

const DEFAULT_ITERATIONS: u64 = 250;
const WARMUP_ITERATIONS: u64 = 20;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OperationResult {
    operation: &'static str,
    iterations: u64,
    total_nanoseconds: u128,
    nanoseconds_per_operation: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AlgorithmResult {
    algorithm: &'static str,
    keygen: OperationResult,
    encaps: OperationResult,
    decaps: OperationResult,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BenchmarkReport {
    schema: &'static str,
    backend: &'static str,
    architecture: &'static str,
    operating_system: &'static str,
    rust_profile: &'static str,
    results: Vec<AlgorithmResult>,
}

fn measure<F>(operation: &'static str, iterations: u64, mut action: F) -> OperationResult
where
    F: FnMut(),
{
    for _ in 0..WARMUP_ITERATIONS {
        action();
    }

    let started = Instant::now();
    for _ in 0..iterations {
        action();
    }
    let elapsed = started.elapsed();
    result(operation, iterations, elapsed)
}

fn result(operation: &'static str, iterations: u64, elapsed: Duration) -> OperationResult {
    let total_nanoseconds = elapsed.as_nanos();
    OperationResult {
        operation,
        iterations,
        total_nanoseconds,
        nanoseconds_per_operation: total_nanoseconds as f64 / iterations as f64,
    }
}

fn benchmark_algorithm(algorithm: Algorithm, iterations: u64) -> AlgorithmResult {
    let keys = generate_keypair_for(algorithm);
    let encapsulated =
        encapsulate_for(algorithm, &keys.public_key).expect("benchmark public key is valid");

    let keygen = measure("keygen", iterations, || {
        black_box(generate_keypair_for(black_box(algorithm)));
    });
    let encaps = measure("encaps", iterations, || {
        black_box(
            encapsulate_for(black_box(algorithm), black_box(&keys.public_key))
                .expect("benchmark public key remains valid"),
        );
    });
    let decaps = measure("decaps", iterations, || {
        black_box(
            decapsulate_for(
                black_box(algorithm),
                black_box(&encapsulated.ciphertext),
                black_box(keys.secret_key.expose()),
            )
            .expect("benchmark ciphertext and private key remain valid"),
        );
    });

    AlgorithmResult {
        algorithm: algorithm.identifier(),
        keygen,
        encaps,
        decaps,
    }
}

fn main() {
    let iterations = std::env::args()
        .nth(1)
        .map(|value| value.parse::<u64>().expect("iterations must be an integer"))
        .unwrap_or(DEFAULT_ITERATIONS);
    assert!(iterations > 0, "iterations must be greater than zero");

    let report = BenchmarkReport {
        schema: "majax-mlkem-performance-v1",
        backend: active_backend().identifier(),
        architecture: std::env::consts::ARCH,
        operating_system: std::env::consts::OS,
        rust_profile: "release",
        results: Algorithm::ALL
            .into_iter()
            .map(|algorithm| benchmark_algorithm(algorithm, iterations))
            .collect(),
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("benchmark report is serializable")
    );
}

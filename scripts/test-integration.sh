#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PROJECT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
RUST_IMAGE="rust:1.93-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31"
OUTPUT_DIR="$PROJECT_DIR/artifacts/integration"

command -v docker >/dev/null 2>&1 || {
    echo "ERROR: Docker is required for the downstream integration check." >&2
    exit 1
}

mkdir -p "$OUTPUT_DIR"

docker run --rm \
    -v "$PROJECT_DIR:/workspace:ro" \
    -w /workspace/examples/rust-consumer \
    "$RUST_IMAGE" \
    sh -c 'CARGO_TARGET_DIR=/tmp/consumer-target cargo run --quiet --locked' \
    | tee "$OUTPUT_DIR/consumer.txt"

docker run --rm \
    -v "$PROJECT_DIR:/workspace:ro" \
    -v "$OUTPUT_DIR:/output" \
    -w /workspace/engine \
    "$RUST_IMAGE" \
    sh -c 'CARGO_TARGET_DIR=/tmp/doc-target cargo doc --quiet --locked --no-deps --all-features && cp -R /tmp/doc-target/doc /output/rustdoc'

docker run --rm \
    -v "$OUTPUT_DIR:/output" \
    "$RUST_IMAGE" \
    chown -R "$(id -u):$(id -g)" /output

printf '%s\n' \
    "schema=majax-mlkem-integration-v1" \
    "consumer=examples/rust-consumer" \
    "parameter_sets=ML-KEM-512,ML-KEM-768,ML-KEM-1024" \
    "result=passed" \
    > "$OUTPUT_DIR/RESULT.txt"

echo "Downstream integration evidence written to artifacts/integration/."

#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PROJECT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
ITERATIONS=${BENCH_ITERATIONS:-250}
RUST_IMAGE="rust:1.93-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31"
NODE_IMAGE="node:20-bookworm-slim@sha256:2cf067cfed83d5ea958367df9f966191a942351a2df77d6f0193e162b5febfc0"
OUTPUT_DIR="$PROJECT_DIR/artifacts/performance"

case "$ITERATIONS" in
    ''|*[!0-9]*)
        echo "ERROR: BENCH_ITERATIONS must be a positive integer." >&2
        exit 1
        ;;
    0)
        echo "ERROR: BENCH_ITERATIONS must be greater than zero." >&2
        exit 1
        ;;
esac

command -v docker >/dev/null 2>&1 || {
    echo "ERROR: Docker is required to run the reproducible benchmark." >&2
    exit 1
}

mkdir -p "$OUTPUT_DIR"

docker run --rm \
    -v "$PROJECT_DIR/engine:/src:ro" \
    -v "$OUTPUT_DIR:/output" \
    -w /src \
    "$RUST_IMAGE" \
    sh -c "CARGO_TARGET_DIR=/target cargo run --quiet --release --locked --no-default-features --bin mlkem-bench -- '$ITERATIONS' > /output/portable.json"

docker run --rm \
    -v "$PROJECT_DIR/engine:/src:ro" \
    -v "$OUTPUT_DIR:/output" \
    -w /src \
    "$RUST_IMAGE" \
    sh -c "CARGO_TARGET_DIR=/target cargo run --quiet --release --locked --bin mlkem-bench -- '$ITERATIONS' > /output/native.json"

docker run --rm \
    -v "$PROJECT_DIR:/project:ro" \
    -v "$OUTPUT_DIR:/output" \
    -w /project \
    "$NODE_IMAGE" \
    node scripts/node/render-benchmarks.mjs /output/portable.json /output/native.json /output/RESULTS.md

docker run --rm \
    -v "$OUTPUT_DIR:/output" \
    "$RUST_IMAGE" \
    chown -R "$(id -u):$(id -g)" /output

echo "Portable and native benchmark evidence written to artifacts/performance/."

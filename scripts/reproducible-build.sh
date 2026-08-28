#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PROJECT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
RUST_IMAGE="rust:1.93-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31"
OUTPUT_DIR="$PROJECT_DIR/artifacts/reproducibility"
SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH:-0}

command -v docker >/dev/null 2>&1 || {
    echo "ERROR: Docker is required for the reproducibility check." >&2
    exit 1
}

case "$SOURCE_DATE_EPOCH" in
    ''|*[!0-9]*)
        echo "ERROR: SOURCE_DATE_EPOCH must be a non-negative integer." >&2
        exit 1
        ;;
esac

mkdir -p "$OUTPUT_DIR"

build_once() {
    destination=$1
    docker run --rm \
        -e CARGO_INCREMENTAL=0 \
        -e SOURCE_DATE_EPOCH="$SOURCE_DATE_EPOCH" \
        -e 'RUSTFLAGS=-C link-arg=-Wl,--build-id=none -C strip=symbols --remap-path-prefix=/src=.' \
        -v "$PROJECT_DIR/engine:/src:ro" \
        -w /src \
        "$RUST_IMAGE" \
        sh -c 'CARGO_TARGET_DIR=/tmp/target cargo build --quiet --release --locked --bin mlkem-cli && cat /tmp/target/release/mlkem-cli' \
        > "$destination"
    chmod 0555 "$destination"
}

build_once "$OUTPUT_DIR/mlkem-cli.first"
build_once "$OUTPUT_DIR/mlkem-cli.second"

sha256sum "$OUTPUT_DIR/mlkem-cli.first" > "$OUTPUT_DIR/first.sha256"
sha256sum "$OUTPUT_DIR/mlkem-cli.second" > "$OUTPUT_DIR/second.sha256"

if ! cmp -s "$OUTPUT_DIR/mlkem-cli.first" "$OUTPUT_DIR/mlkem-cli.second"; then
    echo "FAIL: independent builds produced different binaries." >&2
    exit 1
fi

cp "$OUTPUT_DIR/mlkem-cli.first" "$OUTPUT_DIR/mlkem-cli"
sha256sum "$OUTPUT_DIR/mlkem-cli" > "$OUTPUT_DIR/REPRODUCIBLE.sha256"
printf '%s\n' \
    "schema=majax-mlkem-reproducibility-v1" \
    "source_date_epoch=$SOURCE_DATE_EPOCH" \
    "rust_image=$RUST_IMAGE" \
    "result=identical" \
    > "$OUTPUT_DIR/RESULT.txt"

echo "PASS: two independent release builds are byte-for-byte identical."

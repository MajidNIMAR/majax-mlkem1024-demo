#!/bin/sh
set -eu

FUZZ_SECONDS="${FUZZ_SECONDS:-20}"
RESULTS="artifacts/assurance"
mkdir -p "$RESULTS"

echo "[1/5] Building the dedicated assurance drivers"
cargo build --release --locked --manifest-path assurance/Cargo.toml

echo "[2/5] Running the API driver under Valgrind"
valgrind \
  --error-exitcode=99 \
  --leak-check=full \
  --errors-for-leak-kinds=definite,indirect \
  --log-file="$RESULTS/valgrind.log" \
  assurance/target/release/memory-safety
cat "$RESULTS/valgrind.log"

echo "[3/5] Measuring decapsulation timing distributions"
assurance/target/release/timing-decapsulation >"$RESULTS/dudect.log" 2>&1
cat "$RESULTS/dudect.log"

echo "[4/5] Building all libFuzzer targets"
cargo fuzz build --fuzz-dir fuzz

echo "[5/5] Running bounded libFuzzer campaigns"
for target in api-roundtrip malformed-objects cross-domain; do
  log="$RESULTS/fuzz-$target.log"
  if ! cargo fuzz run --fuzz-dir fuzz "$target" -- \
    -max_total_time="$FUZZ_SECONDS" \
    -timeout=10 \
    -rss_limit_mb=2048 \
    -print_final_stats=1 >"$log" 2>&1; then
    cat "$log"
    exit 1
  fi
  cat "$log"
done

echo "Stage 4 assurance checks completed. Review the timing evidence before making claims."

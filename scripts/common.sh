#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PROJECT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
BINARY_PATH="bin/linux-x86_64/mlkem-cli"
EXPECTED_SHA256="88b805e34122f91f98f89d30b01ba560c0a0148133753257011900f4ce7d35ca"

cd "$PROJECT_DIR"

require_docker() {
  command -v docker >/dev/null 2>&1 || {
    echo "ERROR: Docker is not installed or is not in PATH." >&2
    exit 1
  }
  docker compose version >/dev/null 2>&1 || {
    echo "ERROR: the Docker Compose plugin is unavailable." >&2
    exit 1
  }
}

verify_binary() {
  [ -f "$BINARY_PATH" ] || {
    echo "ERROR: $BINARY_PATH is missing. See README.md." >&2
    exit 1
  }

  if command -v sha256sum >/dev/null 2>&1; then
    actual=$(sha256sum "$BINARY_PATH" | awk '{print $1}')
  elif command -v shasum >/dev/null 2>&1; then
    actual=$(shasum -a 256 "$BINARY_PATH" | awk '{print $1}')
  else
    echo "ERROR: sha256sum or shasum is required to verify mlkem-cli." >&2
    exit 1
  fi

  [ "$actual" = "$EXPECTED_SHA256" ] || {
    echo "ERROR: mlkem-cli SHA-256 mismatch." >&2
    echo "Expected: $EXPECTED_SHA256" >&2
    echo "Actual:   $actual" >&2
    exit 1
  }
}

wait_for_health() {
  attempts=0
  while [ "$attempts" -lt 30 ]; do
    if docker compose exec -T kem-demo node -e \
      "fetch('http://127.0.0.1:8080/api/health').then(r=>process.exit(r.ok?0:1)).catch(()=>process.exit(1))"; then
      return 0
    fi
    attempts=$((attempts + 1))
    sleep 1
  done
  echo "ERROR: the demonstration did not become healthy within 30 seconds." >&2
  docker compose logs --tail=80 kem-demo >&2 || true
  exit 1
}

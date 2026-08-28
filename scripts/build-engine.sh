#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PROJECT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
PLATFORM=${1:-linux/amd64}
SOURCE_DATE_EPOCH=${SOURCE_DATE_EPOCH:-0}

case "$SOURCE_DATE_EPOCH" in
  ''|*[!0-9]*)
    echo "ERROR: SOURCE_DATE_EPOCH must be a non-negative integer." >&2
    exit 1
    ;;
esac

case "$PLATFORM" in
  linux/amd64)
    OUTPUT_DIR="dist/linux-x86_64"
    ;;
  linux/arm64)
    OUTPUT_DIR="dist/linux-arm64"
    ;;
  *)
    echo "ERROR: supported platforms are linux/amd64 and linux/arm64." >&2
    exit 1
    ;;
esac

cd "$PROJECT_DIR"
command -v docker >/dev/null 2>&1 || {
  echo "ERROR: Docker is not installed or is not in PATH." >&2
  exit 1
}
docker buildx version >/dev/null 2>&1 || {
  echo "ERROR: Docker Buildx is unavailable." >&2
  exit 1
}

rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

docker buildx build \
  --platform "$PLATFORM" \
  --build-arg "SOURCE_DATE_EPOCH=$SOURCE_DATE_EPOCH" \
  --file Dockerfile \
  --target binary \
  --output "type=local,dest=$OUTPUT_DIR" \
  .

chmod 0555 "$OUTPUT_DIR/mlkem-cli"
echo "Binary built for $PLATFORM: $OUTPUT_DIR/mlkem-cli"
if command -v sha256sum >/dev/null 2>&1; then
  sha256sum "$OUTPUT_DIR/mlkem-cli"
fi

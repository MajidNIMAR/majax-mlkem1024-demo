#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PROJECT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
IMAGE=${1:-majax-kem-demo:1.0.0}
OUTPUT_DIR="$PROJECT_DIR/artifacts"
SYFT_IMAGE="anchore/syft@sha256:e86b0ba0b1d2fe8a2e9f96ed9b22033df9781f43b9a7eb27c57e6c89234946bc"

cd "$PROJECT_DIR"
command -v docker >/dev/null 2>&1 || {
  echo "ERROR: Docker is not installed or is not in PATH." >&2
  exit 1
}
docker image inspect "$IMAGE" >/dev/null 2>&1 || {
  echo "ERROR: image $IMAGE does not exist. Run scripts/test-all.sh first." >&2
  exit 1
}

mkdir -p "$OUTPUT_DIR"
docker image inspect "$IMAGE" --format '{{.Id}}' > "$OUTPUT_DIR/image-id.txt"
docker run --rm \
  -v /var/run/docker.sock:/var/run/docker.sock \
  "$SYFT_IMAGE" \
  "$IMAGE" -o spdx-json \
  > "$OUTPUT_DIR/majax-kem-demo.spdx.json"

docker run --rm \
  -v /var/run/docker.sock:/var/run/docker.sock \
  "$SYFT_IMAGE" \
  "$IMAGE" -o cyclonedx-json \
  > "$OUTPUT_DIR/majax-kem-demo.cyclonedx.json"

docker run --rm \
  -v "$PROJECT_DIR:/source:ro" \
  "$SYFT_IMAGE" \
  dir:/source \
  --source-name majax-kem-demo-source \
  --source-version 1.0.0 \
  -o spdx-json \
  > "$OUTPUT_DIR/majax-kem-demo-source.spdx.json"

docker run --rm \
  -v "$PROJECT_DIR:/source:ro" \
  "$SYFT_IMAGE" \
  dir:/source \
  --source-name majax-kem-demo-source \
  --source-version 1.0.0 \
  -o cyclonedx-json \
  > "$OUTPUT_DIR/majax-kem-demo-source.cyclonedx.json"

if command -v sha256sum >/dev/null 2>&1; then
  sha256sum \
    "$OUTPUT_DIR/majax-kem-demo.cyclonedx.json" \
    "$OUTPUT_DIR/majax-kem-demo.spdx.json" \
    "$OUTPUT_DIR/majax-kem-demo-source.cyclonedx.json" \
    "$OUTPUT_DIR/majax-kem-demo-source.spdx.json" \
    > "$OUTPUT_DIR/SBOM-CHECKSUMS.sha256"
fi

echo "Image identity and image/source SPDX and CycloneDX SBOMs written to artifacts/."

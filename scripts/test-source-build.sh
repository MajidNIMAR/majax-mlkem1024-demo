#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
PROJECT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
IMAGE="majax-kem-demo:source-test"
CONTAINER="majax-kem-demo-source-test"

cleanup() {
  docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
}
trap cleanup EXIT INT TERM

cd "$PROJECT_DIR"
command -v docker >/dev/null 2>&1 || {
  echo "ERROR: Docker is not installed or is not in PATH." >&2
  exit 1
}

echo "Building the demonstration from the published Rust source..."
docker build --file Dockerfile --tag "$IMAGE" .

cleanup
docker run -d \
  --name "$CONTAINER" \
  --read-only \
  --tmpfs /tmp:rw,noexec,nosuid,nodev,size=16m \
  --cap-drop ALL \
  --security-opt no-new-privileges:true \
  "$IMAGE" >/dev/null

attempts=0
until docker exec "$CONTAINER" node -e \
  "fetch('http://127.0.0.1:8080/api/health').then(r=>process.exit(r.ok?0:1)).catch(()=>process.exit(1))"
do
  attempts=$((attempts + 1))
  if [ "$attempts" -ge 30 ]; then
    docker logs "$CONTAINER" >&2 || true
    echo "ERROR: source-built demonstration did not become healthy." >&2
    exit 1
  fi
  sleep 1
done

docker exec "$CONTAINER" node /opt/majax-kem-demo/scripts/verify-demo.mjs
echo "Source build verified successfully."

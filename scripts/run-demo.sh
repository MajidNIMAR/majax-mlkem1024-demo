#!/bin/sh
set -eu
. "$(dirname -- "$0")/common.sh"

require_docker
message=${1:-"Majax ML-KEM-1024 demonstration"}
docker compose exec -T -e DEMO_MESSAGE="$message" kem-demo \
  node /opt/majax-kem-demo/scripts/verify-demo.mjs

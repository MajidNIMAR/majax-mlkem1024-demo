#!/bin/sh
set -eu
. "$(dirname -- "$0")/common.sh"

require_docker
docker compose up -d --build
wait_for_health
echo "Majax ML-KEM-1024 demonstration is ready at http://127.0.0.1:6062"

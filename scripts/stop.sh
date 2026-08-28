#!/bin/sh
set -eu
. "$(dirname -- "$0")/common.sh"

require_docker
docker compose down
echo "Majax ML-KEM-1024 demonstration stopped."

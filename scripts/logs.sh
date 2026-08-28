#!/bin/sh
set -eu
. "$(dirname -- "$0")/common.sh"

require_docker
docker compose logs --tail=200 -f kem-demo

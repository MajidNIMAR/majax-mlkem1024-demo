#!/bin/sh
set -eu
. "$(dirname -- "$0")/common.sh"

require_docker

IMAGE="majax-mlkem-assurance:stage-4"
FUZZ_SECONDS="${FUZZ_SECONDS:-20}"

docker build --file Dockerfile.assurance --tag "$IMAGE" .
docker run --rm \
  -e FUZZ_SECONDS="$FUZZ_SECONDS" \
  -v "$PWD:/work" \
  "$IMAGE"

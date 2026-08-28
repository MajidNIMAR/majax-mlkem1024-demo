#!/bin/sh
set -eu
. "$(dirname -- "$0")/common.sh"

require_docker
verify_binary
export COMPOSE_FILE="compose.yml:compose.prebuilt.yml"

echo "[1/4] Building the isolated demonstration with the included x86-64 binary"
docker compose up -d --build

echo "[2/4] Waiting for the health check"
wait_for_health

echo "[3/4] Verifying the single-engine scope"
docker compose exec -T kem-demo node -e \
  "fetch('http://127.0.0.1:8080/api/health').then(async r=>{const j=await r.json();if(!r.ok||j.status!=='ok'||j.algorithm!=='ML-KEM-1024'||j.engine_count!==1)throw new Error('invalid health response');console.log('PASS - one ML-KEM-1024 engine is active')}).catch(e=>{console.error('FAIL - '+e.message);process.exit(1)})"

echo "[4/4] Running encapsulation, decapsulation and negative tests"
docker compose exec -T -e DEMO_MESSAGE="Prebuilt Majax ML-KEM-1024 test" kem-demo \
  node /opt/majax-kem-demo/scripts/verify-demo.mjs

echo "All prebuilt demonstration checks passed."

#!/bin/sh
set -eu
. "$(dirname -- "$0")/common.sh"

require_docker
docker compose exec -T kem-demo node -e \
  "fetch('http://127.0.0.1:8080/api/health').then(async r=>{const j=await r.json();console.log(JSON.stringify(j,null,2));process.exit(r.ok&&j.status==='ok'&&j.algorithm==='ML-KEM-1024'&&j.engine_count===1?0:1)}).catch(e=>{console.error(e.message);process.exit(1)})"

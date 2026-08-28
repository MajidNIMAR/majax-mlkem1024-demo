param(
    [ValidateSet('start', 'health', 'run', 'test', 'test-prebuilt', 'logs', 'stop')]
    [string]$Action = 'test',
    [string]$Message = 'Majax ML-KEM-1024 demonstration'
)

$ErrorActionPreference = 'Stop'
$ProjectDir = Split-Path -Parent $PSScriptRoot
$Binary = Join-Path $ProjectDir 'bin\linux-x86_64\mlkem-cli'
$ExpectedSha256 = '88B805E34122F91F98F89D30B01BA560C0A0148133753257011900F4CE7D35CA'

Set-Location $ProjectDir

if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    throw 'Docker is not installed or is not in PATH.'
}
docker compose version | Out-Null

function Assert-Binary {
    if (-not (Test-Path -LiteralPath $Binary -PathType Leaf)) {
        throw 'bin\linux-x86_64\mlkem-cli is missing. See README.md.'
    }
    $Actual = (Get-FileHash -LiteralPath $Binary -Algorithm SHA256).Hash
    if ($Actual -ne $ExpectedSha256) {
        throw "mlkem-cli SHA-256 mismatch. Expected $ExpectedSha256, received $Actual."
    }
}

function Wait-Health {
    for ($Attempt = 1; $Attempt -le 30; $Attempt++) {
        docker compose exec -T kem-demo node -e "fetch('http://127.0.0.1:8080/api/health').then(r=>process.exit(r.ok?0:1)).catch(()=>process.exit(1))"
        if ($LASTEXITCODE -eq 0) { return }
        Start-Sleep -Seconds 1
    }
    docker compose logs --tail=80 kem-demo
    throw 'The demonstration did not become healthy within 30 seconds.'
}

function Start-Demo {
    docker compose up -d --build
    if ($LASTEXITCODE -ne 0) { throw 'Docker build or startup failed.' }
    Wait-Health
}

function Invoke-Demo {
    docker compose exec -T -e "DEMO_MESSAGE=$Message" kem-demo node /opt/majax-kem-demo/scripts/verify-demo.mjs
    if ($LASTEXITCODE -ne 0) { throw 'The cryptographic verification failed.' }
}

switch ($Action) {
    'start' {
        Start-Demo
        Write-Host 'Majax ML-KEM-1024 demonstration is ready at http://127.0.0.1:6062'
    }
    'health' {
        docker compose exec -T kem-demo node -e "fetch('http://127.0.0.1:8080/api/health').then(async r=>{const j=await r.json();console.log(JSON.stringify(j,null,2));process.exit(r.ok&&j.status==='ok'&&j.algorithm==='ML-KEM-1024'&&j.engine_count===1?0:1)}).catch(e=>{console.error(e.message);process.exit(1)})"
        if ($LASTEXITCODE -ne 0) { throw 'The health check failed.' }
    }
    'run' { Invoke-Demo }
    'test' {
        Write-Host '[1/3] Building and starting the isolated demonstration'
        Start-Demo
        Write-Host '[2/3] Verifying the single-engine scope'
        docker compose exec -T kem-demo node -e "fetch('http://127.0.0.1:8080/api/health').then(async r=>{const j=await r.json();if(!r.ok||j.status!=='ok'||j.algorithm!=='ML-KEM-1024'||j.engine_count!==1)throw new Error('invalid health response');console.log('PASS - one ML-KEM-1024 engine is active')}).catch(e=>{console.error('FAIL - '+e.message);process.exit(1)})"
        if ($LASTEXITCODE -ne 0) { throw 'The single-engine scope verification failed.' }
        Write-Host '[3/3] Running encapsulation, decapsulation and negative tests'
        $Message = 'Automated Majax ML-KEM-1024 test'
        Invoke-Demo
        Write-Host 'All demonstration checks passed.'
    }
    'test-prebuilt' {
        Assert-Binary
        Write-Host '[1/3] Building with the included Linux x86-64 binary'
        docker compose -f compose.yml -f compose.prebuilt.yml up -d --build
        if ($LASTEXITCODE -ne 0) { throw 'Prebuilt Docker build or startup failed.' }
        for ($Attempt = 1; $Attempt -le 30; $Attempt++) {
            docker compose -f compose.yml -f compose.prebuilt.yml exec -T kem-demo node -e "fetch('http://127.0.0.1:8080/api/health').then(r=>process.exit(r.ok?0:1)).catch(()=>process.exit(1))"
            if ($LASTEXITCODE -eq 0) { break }
            if ($Attempt -eq 30) { throw 'The prebuilt demonstration did not become healthy.' }
            Start-Sleep -Seconds 1
        }
        Write-Host '[2/3] Verifying the single-engine scope'
        docker compose -f compose.yml -f compose.prebuilt.yml exec -T kem-demo node -e "fetch('http://127.0.0.1:8080/api/health').then(async r=>{const j=await r.json();if(!r.ok||j.status!=='ok'||j.algorithm!=='ML-KEM-1024'||j.engine_count!==1)throw new Error('invalid health response')}).catch(e=>{console.error(e.message);process.exit(1)})"
        if ($LASTEXITCODE -ne 0) { throw 'The prebuilt single-engine scope verification failed.' }
        Write-Host '[3/3] Running encapsulation, decapsulation and negative tests'
        docker compose -f compose.yml -f compose.prebuilt.yml exec -T -e "DEMO_MESSAGE=Prebuilt Majax ML-KEM-1024 test" kem-demo node /opt/majax-kem-demo/scripts/verify-demo.mjs
        if ($LASTEXITCODE -ne 0) { throw 'The prebuilt cryptographic verification failed.' }
        Write-Host 'All prebuilt demonstration checks passed.'
    }
    'logs' { docker compose logs --tail=200 -f kem-demo }
    'stop' {
        docker compose down
        if ($LASTEXITCODE -ne 0) { throw 'Unable to stop the demonstration.' }
    }
}

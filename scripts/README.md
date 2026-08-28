# Ready-to-use test scripts

These scripts operate the standalone single-engine ML-KEM-1024 demonstration.
They never access Majax production keys or data.

## Requirements

- Docker Engine or Docker Desktop;
- the Docker Compose plugin (`docker compose`);
- for the supplied prebuilt path, a Linux x86-64 host able to execute the
  included `bin/linux-x86_64/mlkem-cli` binary.

No host installation of Node.js, `jq` or cryptographic libraries is required.

## Linux and macOS

Run the complete test in one command.

```sh
sh scripts/test-all.sh
```

This command compiles the Rust engine from source inside Docker before running
the checks.

Other commands are available below.

```sh
sh scripts/start.sh
sh scripts/health.sh
sh scripts/run-demo.sh "Text to protect"
sh scripts/logs.sh
sh scripts/stop.sh
```

## Windows PowerShell

Run the complete test in one command.

```powershell
.\scripts\majax-demo.ps1 test
```

Other actions are available below.

```powershell
.\scripts\majax-demo.ps1 start
.\scripts\majax-demo.ps1 health
.\scripts\majax-demo.ps1 run -Message "Text to protect"
.\scripts\majax-demo.ps1 logs
.\scripts\majax-demo.ps1 stop
```

Test the included Linux x86-64 binary separately with one of these commands.

```sh
sh scripts/test-prebuilt.sh
```

```powershell
.\scripts\majax-demo.ps1 test-prebuilt
```

## What the complete test verifies

1. The service reports exactly one ML-KEM-1024 engine.
2. One ephemeral key pair is generated for the execution.
3. The standardized key, ciphertext and shared-secret dimensions are correct.
4. Encapsulation and decapsulation produce the same shared secret.
5. A hybrid AES-256-GCM envelope can be opened with the derived key.
6. Altering the ML-KEM ciphertext produces a divergent secret.
7. The divergent secret cannot authenticate the protected envelope.
8. No production key or production data is accessed.
9. No private key or shared-secret payload is returned by the API.

The script exits with status `0` only when every check passes.

## Rebuild the engine

The normal test compiles the engine from the published Rust source. The
following explicit command performs the same source build and runs the same
checks.

```sh
sh scripts/test-source-build.sh
```

Export a standalone binary for a specific Linux architecture as shown below.

```sh
sh scripts/build-engine.sh linux/amd64
sh scripts/build-engine.sh linux/arm64
```

On an x86-64 Docker host, the ARM64 command requires QEMU support in the active
Buildx builder. A native ARM64 Docker host can build it directly.

## Development assurance suite

The development branch includes isolated fuzzing, memory and timing analysis.
Run the complete stage 4 suite as shown below.

```sh
sh scripts/run-assurance.sh
```

Set `FUZZ_SECONDS` to extend each libFuzzer campaign.

```sh
FUZZ_SECONDS=3600 sh scripts/run-assurance.sh
```

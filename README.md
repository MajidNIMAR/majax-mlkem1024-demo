<p align="center">
  <img src="docs/majax-logo.png" alt="Majax" width="136">
</p>

<h1 align="center">Majax ML-KEM-1024 Demonstrator</h1>

<p align="center">
  A small, reproducible and inspectable cryptographic experiment extracted from the research behind Majax Messenger.
</p>

<p align="center">
  <a href="https://github.com/MajidNIMAR/majax-mlkem1024-demo/actions/workflows/verify.yml"><img alt="Local verification" src="https://img.shields.io/badge/local_checks-6%20passed-18c995"></a>
  <img alt="Algorithm" src="https://img.shields.io/badge/algorithm-ML--KEM--1024-20b7e8">
  <img alt="Engines" src="https://img.shields.io/badge/demo_engine-1-9ba8b8">
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/badge/license-Apache--2.0-f0a52b"></a>
</p>

---

## Why this repository exists

Majax Messenger uses five specialized ML-KEM-1024 domains inside a broader
security architecture. Publishing the production implementation would expose
operational details that do not belong in a public repository.

This project takes a more useful approach. It isolates one real ML-KEM-1024
engine and turns it into a compact experiment that anyone can build, execute
and inspect without access to Majax infrastructure.

The result is deliberately modest in scope and serious in execution. It is a
working cryptographic demonstrator, not a mock-up and not a marketing
animation.

## What one execution demonstrates

Every run creates fresh material and performs the complete path below.

1. Generate one ephemeral ML-KEM-1024 key pair
2. Encapsulate a shared secret with the public key
3. Decapsulate the ciphertext with the private key
4. Verify reciprocal agreement between both secrets
5. Derive an AES-256-GCM key and protect a user-supplied message
6. Alter the ML-KEM ciphertext and verify that the resulting secret diverges
7. Confirm that the divergent key cannot authenticate the protected envelope
8. Report standardized object dimensions, timings and SHA-256 fingerprints

The HTTP API never returns the private key or either shared secret. Ephemeral
material exists only in memory during the short-lived request execution.

## Run the complete test

Docker is the only required host dependency.

```sh
sh scripts/test-all.sh
```

Windows PowerShell users can run the equivalent command.

```powershell
.\scripts\majax-demo.ps1 test
```

The default image compiles the Rust engine directly from `engine/` inside
digest-pinned Rust and Node environments. The automated verification performs
six cryptographic and integration checks before returning a successful result.

## Explore the live interface

Start the isolated service and open `http://127.0.0.1:6062`.

```sh
sh scripts/start.sh
```

The service binds to loopback by default. Public exposure requires a separate
TLS reverse proxy, explicit rate limiting and normal operational monitoring.

## Build for another Linux architecture

Export the standalone engine for Linux x86-64.

```sh
sh scripts/build-engine.sh linux/amd64
```

Export the same engine for Linux ARM64.

```sh
sh scripts/build-engine.sh linux/arm64
```

Generated binaries are written below `dist/` and remain outside version
control. Building ARM64 on an x86-64 host requires Docker Buildx with QEMU
emulation. A native ARM64 Docker host can build it directly.

## Use the supplied x86-64 binary

The repository also includes a ready-to-run Linux x86-64 binary at
`bin/linux-x86_64/mlkem-cli` for people who want to test first and inspect the
source afterward.

Its expected SHA-256 fingerprint is recorded in `CHECKSUMS.sha256`.

```text
88b805e34122f91f98f89d30b01ba560c0a0148133753257011900f4ce7d35ca
```

Run its independent verification path with the following command.

```sh
sh scripts/test-prebuilt.sh
```

```powershell
.\scripts\majax-demo.ps1 test-prebuilt
```

## Reproducibility and evidence

The repository includes the material needed to inspect the build and its
dependencies.

- `engine/Cargo.lock` records exact Rust dependency versions and checksums
- `Dockerfile` compiles the engine from source using digest-pinned images
- `.github/workflows/verify.yml` repeats the complete test on every change
- `CHECKSUMS.sha256` authenticates the supplied prebuilt binary
- `THIRD_PARTY_NOTICES.md` records cryptographic attributions and licenses
- `scripts/generate-sbom.sh` emits SPDX JSON inventories for the source tree
  and final image

Generate the image identity, both SBOMs and their SHA-256 checksums locally.

```sh
sh scripts/generate-sbom.sh
```

The generated evidence is written below `artifacts/` and remains outside
version control.

## Security boundary

This repository contains no production key, token, credential, user record,
endpoint configuration or operational secret. It does not reproduce the five
Majax cryptographic domains and does not expose the Majax service architecture.

Successful execution proves only the properties exercised by the included
tests. It does not constitute an audit or certification of Majax Messenger.
The complete boundary and private reporting process are documented in
[`SECURITY.md`](SECURITY.md).

## Project identity

This demonstrator is published by Majax to make a concrete part of its
post-quantum engineering independently reproducible. It favors verifiable
execution over architectural disclosure and precise claims over slogans.

The project is distributed under Apache License 2.0. Third-party components
retain their own licenses and notices.

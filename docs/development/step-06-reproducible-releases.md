# Step 6, reproducible releases and verifiable provenance

## Scope

This step makes release artifacts independently identifiable and verifiable. It
does not change the ML-KEM algorithms, typed API or protocol behavior developed
in the preceding steps.

## Repeatable binary build

`scripts/reproducible-build.sh` performs two independent release compilations
inside the same digest-pinned Rust environment. Incremental compilation is
disabled, source paths are remapped and the linker build identifier is removed.
The check fails unless both x86-64 binaries are byte-for-byte identical.
The source Dockerfile and architecture build helper use the same flags, so the
published binary follows the policy exercised by this check.

```sh
sh scripts/reproducible-build.sh
```

The evidence includes both SHA-256 values, the pinned toolchain identity and the
comparison result. Reproducibility applies to the same source revision, target
architecture and declared toolchain. Different processor architectures produce
different binaries by design.

## Software bills of materials

The existing image and source scans now emit both SPDX JSON and CycloneDX JSON.
Every SBOM is covered by `SBOM-CHECKSUMS.sha256`. Syft itself remains pinned by
container digest.

## Tagged releases

Tags beginning with `v` start the release workflow. Native GitHub runners build
x86-64 and AArch64 artifacts. The workflow then creates a deterministic source
archive, produces image and source SBOMs and records SHA-256 checksums.

Every binary, archive, SBOM and checksum file receives a keyless Sigstore bundle.
Authentication comes from GitHub's short-lived OpenID Connect identity, so the
repository stores no release signing private key. GitHub also emits build
provenance attestations for the principal artifacts.

The release workflow is declared in `.github/workflows/release.yml`. Actions,
compiler images, runtime images and SBOM tooling are referenced by immutable
commit or image digests.

## Consumer verification

Downloaded artifacts can be checked with the included helper.

```sh
sh scripts/verify-release.sh release
```

SHA-256 verification always runs. When Cosign is available, the helper also
checks each Sigstore bundle against the expected GitHub workflow identity and
the GitHub Actions OpenID Connect issuer.

## Evidence boundary

These controls establish artifact identity, repeatability and traceable build
origin. They do not replace cryptographic review, platform hardening or an
independent security audit. No production secret or production signing key is
introduced by this release process.

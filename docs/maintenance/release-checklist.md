# Release checklist

## Source review

- the release commit is identified and the working tree is clean
- the changelog describes every public behavior change
- dependency and test-vector origins remain documented
- no production credential, endpoint or private protocol material is present

## Verification

- the complete verification workflow passes at the release commit
- downstream integration and generated rustdoc pass
- assurance checks pass with the recorded campaign duration
- x86-64 and AArch64 performance evidence is retained
- independent release builds are byte-for-byte identical

## Publication

- the semantic version and annotated tag identify the reviewed commit
- x86-64 and AArch64 binaries are attached
- source and image SBOMs are attached in SPDX and CycloneDX formats
- SHA-256 checksums cover every principal artifact
- Sigstore bundles and GitHub provenance attestations are available
- release notes state the security and evidence boundary

## After publication

- the downloaded bundle passes `scripts/verify-release.sh`
- public documentation points to the new supported release
- superseded versions remain available for traceability

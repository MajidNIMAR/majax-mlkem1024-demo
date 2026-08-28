#!/bin/sh
set -eu

RELEASE_DIR=${1:-release}

test -d "$RELEASE_DIR" || {
    echo "ERROR: release directory not found: $RELEASE_DIR" >&2
    exit 1
}
test -f "$RELEASE_DIR/SHA256SUMS" || {
    echo "ERROR: SHA256SUMS is missing." >&2
    exit 1
}

(cd "$RELEASE_DIR" && sha256sum --check SHA256SUMS)

if command -v cosign >/dev/null 2>&1; then
    for bundle in "$RELEASE_DIR"/*.sigstore.json; do
        test -e "$bundle" || continue
        artifact=${bundle%.sigstore.json}
        cosign verify-blob \
            --bundle "$bundle" \
            --certificate-oidc-issuer "https://token.actions.githubusercontent.com" \
            --certificate-identity-regexp '^https://github.com/MajidNIMAR/majax-mlkem1024-demo/.github/workflows/release.yml@refs/tags/v' \
            "$artifact"
    done
else
    echo "NOTE: cosign is unavailable, Sigstore bundles were not verified locally."
fi

echo "Release checks completed."

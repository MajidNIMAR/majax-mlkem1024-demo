# Third-party components

The source build is locked by `engine/Cargo.lock`. The principal cryptographic
component is `pqcrypto-mlkem 0.1.1`, exposed through `pqcrypto-traits 0.3.5`.
Both declare `MIT OR Apache-2.0` licensing in their Cargo metadata. This project
uses them under the Apache License 2.0 option.

The direct Rust dependencies are listed below.

| Component | Version | Declared license |
| --- | --- | --- |
| base64 | 0.21.7 | MIT OR Apache-2.0 |
| pqcrypto-mlkem | 0.1.1 | MIT OR Apache-2.0 |
| pqcrypto-traits | 0.3.5 | MIT OR Apache-2.0 |
| serde | 1.0.228 | MIT OR Apache-2.0 |
| serde_json | 1.0.145 | MIT OR Apache-2.0 |

The complete transitive dependency graph, exact versions and registry checksums
are recorded in `engine/Cargo.lock`. Each dependency remains subject to its own
license.

The `pqcrypto-mlkem` package incorporates ML-KEM implementations from PQClean.
The portable ML-KEM-1024 implementation declares a public-domain dedication.
Architecture-specific source files retain the notices embedded by their
respective authors. The relevant Apache and MIT texts are provided under
`LICENSES/`, together with the PQClean ML-KEM-1024 notice.

This notice does not replace the license information embedded in third-party
source packages.

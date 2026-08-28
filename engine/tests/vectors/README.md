# External ML-KEM rejection vectors

The three fixtures in this directory come from the C2SP CCTV ML-KEM corpus at
commit `1e3d2860d46e94e777e1b17c7a6f2436387e3ecc`.

The upstream files exercise an implicit-rejection edge case that can be missed
when a ciphertext comparison stops at a zero byte. One fixture is retained for
each standardized parameter set. The test checks the complete private key,
ciphertext and expected rejection secret without regenerating any value.

The upstream corpus publishes these vectors under CC0 1.0. The source material
is available at <https://github.com/C2SP/CCTV/tree/1e3d2860d46e94e777e1b17c7a6f2436387e3ecc/ML-KEM/strcmp>.

These fixtures provide independent regression evidence. They do not represent
an ACVP validation or a FIPS 140 certificate.

## Official NIST ACVP fixtures

`nist-acvp-fips203.json` contains one key-generation case and one encapsulation
case for each standardized parameter set. The cases were mechanically
extracted from the NIST ACVP-Server sample prompt and expected-result files at
commit `975de31eb83d87039ec88934fdc47d8c312b892d`, tagged `v1.1.0.43`.

The original files are available in the following directories.

- <https://github.com/usnistgov/ACVP-Server/tree/975de31eb83d87039ec88934fdc47d8c312b892d/gen-val/json-files/ML-KEM-keyGen-FIPS203>
- <https://github.com/usnistgov/ACVP-Server/tree/975de31eb83d87039ec88934fdc47d8c312b892d/gen-val/json-files/ML-KEM-encapDecap-FIPS203>

The retained fields are `d`, `z`, `ek` and `dk` for key generation and `ek`,
`m`, `c` and `k` for encapsulation. Tests enable the dedicated deterministic
feature and compare every generated output byte with the NIST expected result.

Passing these sample cases is regression and conformance evidence. It is not
an ACVP validation or a FIPS 140 certificate.

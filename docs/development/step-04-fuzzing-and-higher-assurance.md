# Step 4, fuzzing and higher-assurance evidence

## Scope

This step adds adversarial and statistical test tooling around the typed ML-KEM
library. It does not alter the stable demonstrator, the normal random API or any
Majax production component.

## Fuzzing surface

Three libFuzzer targets exercise the public library through bounded inputs.

- `api-roundtrip` derives deterministic cases and requires successful recovery
- `malformed-objects` explores encoded object boundaries and parser rejection
- `cross-domain` requires keys and ciphertexts to remain bound to one level

Short campaigns run in continuous integration. Operators can increase
`FUZZ_SECONDS` for longer local or scheduled campaigns. A completed campaign
only establishes that no failure was found in the explored input space.

## Memory analysis

The dedicated memory driver traverses every parameter set, reciprocal
decapsulation, implicit rejection and malformed-length handling. Valgrind runs
the release driver and turns definite or indirect memory errors into a failing
exit status.

This evidence covers the exercised Linux build. It is not a proof for every
compiler, architecture or execution path.

## Timing analysis

The DudeCT harness compares decapsulation timing distributions for a valid
ciphertext and a one-bit alteration processed with the same private key. All
three parameter sets are sampled. Large statistical deviations can reveal a
timing distinguishing signal that requires investigation.

A quiet result cannot prove constant-time execution. Statistical timing tests
depend on the selected classes, sample count, processor, operating system and
noise. Longer campaigns on isolated hardware remain necessary before making a
high-assurance claim.

The suite retains the Valgrind report, DudeCT output and final libFuzzer
statistics under `artifacts/assurance`. CI publishes the same directory as a
downloadable evidence bundle.

## Reproducible execution

The assurance environment uses a digest-pinned Rust nightly image. Cargo-fuzz
is installed at an exact version and Valgrind is installed from the pinned
Debian base repositories. The complete suite starts with one command.

```sh
sh scripts/run-assurance.sh
```

Longer fuzzing campaigns can be requested without changing source files.

```sh
FUZZ_SECONDS=3600 sh scripts/run-assurance.sh
```

## Evidence boundary

This step provides repeatable negative evidence from fuzzing, memory analysis
and timing statistics. It does not constitute formal verification, a proof of
constant-time behavior, an ACVP validation or a FIPS 140 certification.

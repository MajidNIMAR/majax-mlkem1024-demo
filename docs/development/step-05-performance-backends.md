# Step 5, portable and native performance backends

## Scope

This step makes the existing PQClean backend dispatch visible and measurable.
The cryptographic API and standardized ML-KEM behavior remain unchanged.

Two build policies are now explicit.

- `--no-default-features` compiles only the portable clean C implementation
- the default `native` feature compiles AVX2 and AArch64/NEON implementations
  and lets the upstream dispatcher select the supported native path

On x86-64, AVX2 is selected only after runtime CPU detection. A native binary
therefore remains usable on an x86-64 processor without AVX2 and falls back to
the portable implementation. AArch64 native builds use the NEON implementation,
which matches the baseline SIMD capability of the supported target.

## Public backend identity

`active_backend()` reports the implementation selected on the current system.
It returns one of the following stable identifiers.

- `portable-clean`
- `x86_64-avx2`
- `aarch64-neon`

The identifier is diagnostic metadata. It does not alter keys, ciphertexts or
shared secrets and must not be used as protocol input.

## Reproducible comparison

The benchmark builds the same source twice and measures KeyGen, Encaps and
Decaps for ML-KEM-512, ML-KEM-768 and ML-KEM-1024. One build is portable and
the other enables native dispatch.

```sh
sh scripts/run-benchmarks.sh
```

The default campaign uses 250 measured operations after a short warm-up.
Longer local measurements can be requested without editing the source.

```sh
BENCH_ITERATIONS=5000 sh scripts/run-benchmarks.sh
```

Raw JSON and a comparative Markdown table are written below
`artifacts/performance`. Each comparison is made on the same runner so that the
reported speedup does not mix processor families.

## Continuous evidence

GitHub Actions runs the portable/native comparison on Linux x86-64 and Linux
AArch64. The resulting bundles are published separately and include both raw
reports and the rendered comparison.

Wall-clock measurements on shared runners are affected by scheduling,
virtualization, frequency scaling and processor model. They demonstrate that
both backend paths build, execute and remain reciprocal. They do not establish
a universal rank against unrelated implementations.

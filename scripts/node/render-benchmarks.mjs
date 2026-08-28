import { readFile, writeFile } from "node:fs/promises";

const [portablePath, nativePath, outputPath] = process.argv.slice(2);
if (!portablePath || !nativePath || !outputPath) {
  throw new Error("usage: render-benchmarks.mjs <portable.json> <native.json> <RESULTS.md>");
}

const portable = JSON.parse(await readFile(portablePath, "utf8"));
const native = JSON.parse(await readFile(nativePath, "utf8"));

if (portable.schema !== "majax-mlkem-performance-v1" || native.schema !== portable.schema) {
  throw new Error("unsupported benchmark schema");
}
if (portable.architecture !== native.architecture) {
  throw new Error("portable and native reports must come from the same architecture");
}

const format = (value) => Math.round(value).toLocaleString("en-US");
const ratio = (baseline, candidate) => (baseline / candidate).toFixed(2);
const lines = [
  "# ML-KEM backend benchmark",
  "",
  `Architecture, ${native.architecture}`,
  `Portable backend, ${portable.backend}`,
  `Native backend, ${native.backend}`,
  "",
  "Times are wall-clock nanoseconds per operation. Native speedup is relative to",
  "the portable build measured in the same job on the same runner.",
  "",
  "| Parameter set | Operation | Portable ns/op | Native ns/op | Native speedup |",
  "| --- | ---: | ---: | ---: | ---: |",
];

for (let index = 0; index < portable.results.length; index += 1) {
  const portableResult = portable.results[index];
  const nativeResult = native.results[index];
  if (portableResult.algorithm !== nativeResult.algorithm) {
    throw new Error("benchmark reports use different parameter-set ordering");
  }
  for (const operation of ["keygen", "encaps", "decaps"]) {
    const portableTime = portableResult[operation].nanosecondsPerOperation;
    const nativeTime = nativeResult[operation].nanosecondsPerOperation;
    lines.push(
      `| ${portableResult.algorithm} | ${operation} | ${format(portableTime)} | ${format(nativeTime)} | ${ratio(portableTime, nativeTime)}x |`,
    );
  }
}

lines.push(
  "",
  "These measurements are comparative engineering evidence, not a universal",
  "performance claim. Runner load, processor model and virtualization affect results.",
  "",
);

await writeFile(outputPath, lines.join("\n"), "utf8");

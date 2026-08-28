const endpoint = 'http://127.0.0.1:8080/api/run';
const message = process.env.DEMO_MESSAGE || 'Majax ML-KEM-1024 demonstration';

const response = await fetch(endpoint, {
  method: 'POST',
  headers: { 'content-type': 'application/json' },
  body: JSON.stringify({ message }),
});

const result = await response.json();
if (!response.ok) {
  throw new Error(`API returned HTTP ${response.status}: ${JSON.stringify(result)}`);
}

const expectedChecks = [
  'algorithm_identifier_correct',
  'standardized_object_dimensions',
  'encapsulation_decapsulation_reciprocal',
  'hybrid_message_round_trip',
  'altered_ciphertext_secret_diverges',
  'altered_ciphertext_rejected_by_authenticated_envelope',
];

const expectedDimensions = {
  public_key: 1568,
  private_key: 3168,
  ciphertext: 1568,
  shared_secret: 32,
};

const forbiddenPayloadFields = new Set([
  'private_key_b64',
  'privateKey',
  'privateKeyB64',
  'shared_secret_b64',
  'sharedSecret',
  'sharedSecretB64',
]);

function collectKeys(value, keys = []) {
  if (!value || typeof value !== 'object') return keys;
  for (const [key, child] of Object.entries(value)) {
    keys.push(key);
    collectKeys(child, keys);
  }
  return keys;
}

const failures = [];
if (result.overall_result !== 'PASS') failures.push('overall result is not PASS');
if (result.scope?.algorithm !== 'ML-KEM-1024') failures.push('algorithm is not ML-KEM-1024');
if (result.scope?.engine_count !== 1) failures.push('engine count is not one');
if (result.scope?.key_pairs_generated !== 1) failures.push('ephemeral key-pair count is not one');
if (result.scope?.production_keys_accessed !== false) failures.push('production-key isolation is not asserted');
if (result.scope?.production_data_accessed !== false) failures.push('production-data isolation is not asserted');
if (result.scope?.persistent_keys !== false) failures.push('keys are reported as persistent');
if (result.recovered_demo_message !== message) failures.push('protected message did not round-trip');

for (const check of expectedChecks) {
  if (result.checks?.[check] !== true) failures.push(`check failed: ${check}`);
}

for (const [name, expected] of Object.entries(expectedDimensions)) {
  if (result.dimensions_bytes?.[name] !== expected) {
    failures.push(`unexpected ${name} size: ${result.dimensions_bytes?.[name]}`);
  }
}

for (const key of collectKeys(result)) {
  if (forbiddenPayloadFields.has(key)) failures.push(`secret payload field returned: ${key}`);
}

console.log(`Algorithm       ${result.scope?.algorithm}`);
console.log(`Engines         ${result.scope?.engine_count}`);
console.log(`Ephemeral pairs ${result.scope?.key_pairs_generated}`);
console.log('');
for (const check of expectedChecks) {
  console.log(`${result.checks?.[check] === true ? 'PASS' : 'FAIL'} - ${check}`);
}
console.log('');
console.log(`Key generation  ${result.timings_ms?.key_generation} ms`);
console.log(`Encapsulation   ${result.timings_ms?.encapsulation} ms`);
console.log(`Decapsulation   ${result.timings_ms?.decapsulation} ms`);
console.log(`Total           ${result.timings_ms?.total} ms`);

if (failures.length > 0) {
  console.error('\nVerification failed:');
  failures.forEach((failure) => console.error(`- ${failure}`));
  process.exit(1);
}

console.log('\nPASS - the single-engine ML-KEM-1024 demonstration is valid.');

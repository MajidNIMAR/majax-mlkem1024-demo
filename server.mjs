import { createServer } from 'node:http';
import { readFileSync } from 'node:fs';
import { dirname, extname, join, normalize } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';
import {
  createCipheriv,
  createDecipheriv,
  createHash,
  hkdfSync,
  randomBytes,
  timingSafeEqual,
} from 'node:crypto';
import { performance } from 'node:perf_hooks';

const ROOT = dirname(fileURLToPath(import.meta.url));
const PUBLIC_DIR = join(ROOT, 'public');
const CLI = process.env.MLKEM_CLI_PATH || join(ROOT, 'bin', 'mlkem-cli');
const PORT = Number.parseInt(process.env.PORT || '8080', 10);
const ALGORITHM = 'ML-KEM-1024';
const EXPECTED_SIZES = Object.freeze({
  public_key: 1568,
  private_key: 3168,
  ciphertext: 1568,
  shared_secret: 32,
});
const MAX_BODY_BYTES = 8192;
const WINDOW_MS = 60_000;
const MAX_RUNS_PER_WINDOW = 8;
const rateWindows = new Map();
let demoRunning = false;

const MIME = Object.freeze({
  '.html': 'text/html; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
});

function securityHeaders(contentType = 'application/json; charset=utf-8') {
  return {
    'Content-Type': contentType,
    'Cache-Control': 'no-store',
    'Content-Security-Policy': "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; connect-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'; form-action 'none'",
    'Cross-Origin-Opener-Policy': 'same-origin',
    'Cross-Origin-Resource-Policy': 'same-origin',
    'Permissions-Policy': 'camera=(), microphone=(), geolocation=(), payment=(), usb=()',
    'Referrer-Policy': 'no-referrer',
    'X-Content-Type-Options': 'nosniff',
    'X-Frame-Options': 'DENY',
  };
}

function sendJson(response, statusCode, value) {
  response.writeHead(statusCode, securityHeaders());
  response.end(`${JSON.stringify(value)}\n`);
}

function sha256(buffer) {
  return createHash('sha256').update(buffer).digest('hex').toUpperCase();
}

function decodeBase64(value, label) {
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`${label} missing from ML-KEM output`);
  }
  return Buffer.from(value, 'base64');
}

function runCli(command, payload) {
  const started = performance.now();
  const result = spawnSync(CLI, [command], {
    input: JSON.stringify(payload),
    encoding: 'utf8',
    timeout: 20_000,
    maxBuffer: 1024 * 1024,
    env: { PATH: process.env.PATH || '/usr/local/bin:/usr/bin:/bin' },
  });
  const elapsedMs = Number((performance.now() - started).toFixed(3));
  if (result.error || result.status !== 0) {
    throw new Error(`ML-KEM ${command} execution failed`);
  }
  let output;
  try {
    output = JSON.parse(String(result.stdout || ''));
  } catch {
    throw new Error(`ML-KEM ${command} returned invalid JSON`);
  }
  if (output?.ok !== true) {
    throw new Error(`ML-KEM ${command} did not report success`);
  }
  return { output, elapsedMs };
}

function deriveAeadKey(sharedSecret, salt) {
  return Buffer.from(hkdfSync('sha256', sharedSecret, salt, 'majax-kem-demo-v1', 32));
}

function encryptDemoMessage(message, sharedSecret) {
  const salt = randomBytes(32);
  const nonce = randomBytes(12);
  const key = deriveAeadKey(sharedSecret, salt);
  const cipher = createCipheriv('aes-256-gcm', key, nonce);
  const ciphertext = Buffer.concat([cipher.update(message, 'utf8'), cipher.final()]);
  return { salt, nonce, ciphertext, tag: cipher.getAuthTag() };
}

function decryptDemoMessage(envelope, sharedSecret) {
  const key = deriveAeadKey(sharedSecret, envelope.salt);
  const decipher = createDecipheriv('aes-256-gcm', key, envelope.nonce);
  decipher.setAuthTag(envelope.tag);
  return Buffer.concat([
    decipher.update(envelope.ciphertext),
    decipher.final(),
  ]).toString('utf8');
}

function runDemonstration(message) {
  const executionId = randomBytes(12).toString('hex');
  const deviceId = `kem-demo-${executionId}`;
  const totalStarted = performance.now();

  const generatedRun = runCli('gen', { deviceId, algo: ALGORITHM });
  const generated = generatedRun.output;
  const publicKey = decodeBase64(generated.publicKey, 'publicKey');
  const privateKey = decodeBase64(generated.privateKey, 'privateKey');

  const encapsulatedRun = runCli('enc', {
    deviceId,
    algo: ALGORITHM,
    publicKey: generated.publicKey,
  });
  const encapsulated = encapsulatedRun.output;
  const ciphertext = decodeBase64(encapsulated.ciphertext_b64, 'ciphertext_b64');
  const senderSecret = decodeBase64(encapsulated.sharedSecret_b64, 'sharedSecret_b64');

  const decapsulatedRun = runCli('dec', {
    deviceId,
    algo: ALGORITHM,
    privateKey: generated.privateKey,
    ciphertext_b64: encapsulated.ciphertext_b64,
  });
  const decapsulated = decapsulatedRun.output;
  const receiverSecret = decodeBase64(decapsulated.sharedSecret_b64, 'sharedSecret_b64');
  const reciprocal = senderSecret.length === receiverSecret.length
    && timingSafeEqual(senderSecret, receiverSecret);

  const hybridStarted = performance.now();
  const envelope = encryptDemoMessage(message, senderSecret);
  const recoveredMessage = decryptDemoMessage(envelope, receiverSecret);
  const hybridMs = Number((performance.now() - hybridStarted).toFixed(3));

  const alteredCiphertext = Buffer.from(ciphertext);
  alteredCiphertext[Math.floor(alteredCiphertext.length / 2)] ^= 0x01;
  const alteredRun = runCli('dec', {
    deviceId,
    algo: ALGORITHM,
    privateKey: generated.privateKey,
    ciphertext_b64: alteredCiphertext.toString('base64'),
  });
  const alteredSecret = decodeBase64(alteredRun.output.sharedSecret_b64, 'sharedSecret_b64');
  const alteredSecretDiverges = senderSecret.length !== alteredSecret.length
    || !timingSafeEqual(senderSecret, alteredSecret);
  let alteredEnvelopeRejected = false;
  try {
    decryptDemoMessage(envelope, alteredSecret);
  } catch {
    alteredEnvelopeRejected = true;
  }

  const algorithmsCorrect = [generated, encapsulated, decapsulated, alteredRun.output]
    .every((item) => item.algo === ALGORITHM);
  const dimensions = {
    public_key: publicKey.length,
    private_key: privateKey.length,
    ciphertext: ciphertext.length,
    shared_secret: senderSecret.length,
  };
  const dimensionsCorrect = Object.entries(EXPECTED_SIZES)
    .every(([name, expected]) => dimensions[name] === expected);
  const hybridRoundTrip = recoveredMessage === message;
  const passed = algorithmsCorrect
    && dimensionsCorrect
    && reciprocal
    && hybridRoundTrip
    && alteredSecretDiverges
    && alteredEnvelopeRejected;

  return {
    schema_version: 1,
    demonstration: 'single-engine-ephemeral-ml-kem',
    generated_at: new Date().toISOString(),
    execution_id: executionId,
    overall_result: passed ? 'PASS' : 'FAIL',
    scope: {
      algorithm: ALGORITHM,
      engine_count: 1,
      key_pairs_generated: 1,
      production_keys_accessed: false,
      production_data_accessed: false,
      persistent_keys: false,
    },
    checks: {
      algorithm_identifier_correct: algorithmsCorrect,
      standardized_object_dimensions: dimensionsCorrect,
      encapsulation_decapsulation_reciprocal: reciprocal,
      hybrid_message_round_trip: hybridRoundTrip,
      altered_ciphertext_secret_diverges: alteredSecretDiverges,
      altered_ciphertext_rejected_by_authenticated_envelope: alteredEnvelopeRejected,
    },
    dimensions_bytes: dimensions,
    fingerprints_sha256: {
      public_key: sha256(publicKey),
      ciphertext: sha256(ciphertext),
      encrypted_demo_message: sha256(envelope.ciphertext),
    },
    timings_ms: {
      key_generation: generatedRun.elapsedMs,
      encapsulation: encapsulatedRun.elapsedMs,
      decapsulation: decapsulatedRun.elapsedMs,
      authenticated_encryption_round_trip: hybridMs,
      altered_ciphertext_decapsulation: alteredRun.elapsedMs,
      total: Number((performance.now() - totalStarted).toFixed(3)),
    },
    recovered_demo_message: recoveredMessage,
    note: 'All key material existed only in process memory for this execution and was not returned by the API.',
  };
}

function remoteAddress(request) {
  return request.socket.remoteAddress || 'unknown';
}

function consumeRateLimit(address) {
  const now = Date.now();
  const current = rateWindows.get(address);
  if (!current || now - current.startedAt >= WINDOW_MS) {
    rateWindows.set(address, { startedAt: now, count: 1 });
    return true;
  }
  if (current.count >= MAX_RUNS_PER_WINDOW) return false;
  current.count += 1;
  return true;
}

function readBody(request) {
  return new Promise((resolve, reject) => {
    let size = 0;
    const chunks = [];
    request.on('data', (chunk) => {
      size += chunk.length;
      if (size > MAX_BODY_BYTES) {
        reject(new Error('request-too-large'));
        request.destroy();
        return;
      }
      chunks.push(chunk);
    });
    request.on('end', () => resolve(Buffer.concat(chunks).toString('utf8')));
    request.on('error', reject);
  });
}

function serveStatic(request, response) {
  const requested = request.url === '/' ? '/index.html' : request.url;
  const clean = normalize(requested.split('?')[0]).replace(/^(\.\.(\/|\\|$))+/, '');
  const filePath = join(PUBLIC_DIR, clean);
  if (!filePath.startsWith(PUBLIC_DIR)) {
    sendJson(response, 404, { error: 'not-found' });
    return;
  }
  try {
    const content = readFileSync(filePath);
    response.writeHead(200, securityHeaders(MIME[extname(filePath)] || 'application/octet-stream'));
    response.end(content);
  } catch {
    sendJson(response, 404, { error: 'not-found' });
  }
}

const server = createServer(async (request, response) => {
  if (request.method === 'GET' && request.url === '/api/health') {
    sendJson(response, 200, {
      status: 'ok',
      algorithm: ALGORITHM,
      engine_count: 1,
      binary_sha256: sha256(readFileSync(CLI)),
    });
    return;
  }

  if (request.method === 'POST' && request.url === '/api/run') {
    if (!consumeRateLimit(remoteAddress(request))) {
      sendJson(response, 429, { error: 'rate-limit', retry_after_seconds: 60 });
      return;
    }
    if (demoRunning) {
      sendJson(response, 503, { error: 'demonstration-busy' });
      return;
    }
    demoRunning = true;
    try {
      const raw = await readBody(request);
      const body = raw ? JSON.parse(raw) : {};
      const message = typeof body.message === 'string'
        ? body.message.trim().slice(0, 512)
        : '';
      const result = runDemonstration(message || 'Majax cryptographic demonstration');
      sendJson(response, result.overall_result === 'PASS' ? 200 : 500, result);
    } catch (error) {
      console.error(`Demonstration failed: ${error instanceof Error ? error.message : 'unknown error'}`);
      sendJson(response, 500, { error: 'demonstration-failed' });
    } finally {
      demoRunning = false;
    }
    return;
  }

  if (request.method === 'GET' || request.method === 'HEAD') {
    serveStatic(request, response);
    return;
  }
  sendJson(response, 405, { error: 'method-not-allowed' });
});

server.listen(PORT, '0.0.0.0', () => {
  console.log(`Majax KEM demo listening on port ${PORT}`);
});

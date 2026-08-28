use std::env;
use std::io::{self, Read};

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use majax_mlkem::{
    decapsulate as kem_decapsulate, encapsulate as kem_encapsulate, generate_keypair, ALGORITHM,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommonInput {
    #[serde(default)]
    device_id: Option<String>,
    #[serde(default)]
    algo: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EncInput {
    #[serde(flatten)]
    common: CommonInput,
    public_key: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DecInput {
    #[serde(flatten)]
    common: CommonInput,
    private_key: String,
    #[serde(rename = "ciphertext_b64")]
    ciphertext_b64: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GenOutput {
    ok: bool,
    algo: &'static str,
    public_key: String,
    private_key: String,
}

#[derive(Serialize)]
struct EncOutput {
    ok: bool,
    algo: &'static str,
    ciphertext_b64: String,
    #[serde(rename = "sharedSecret_b64")]
    shared_secret_b64: String,
}

#[derive(Serialize)]
struct DecOutput {
    ok: bool,
    algo: &'static str,
    #[serde(rename = "sharedSecret_b64")]
    shared_secret_b64: String,
}

fn read_stdin() -> Result<String, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("unable to read stdin: {error}"))?;
    if input.trim().is_empty() {
        return Err("stdin is empty; a JSON object is required".to_string());
    }
    Ok(input)
}

fn validate_common(input: &CommonInput) -> Result<(), String> {
    if let Some(algo) = input.algo.as_deref() {
        if algo != ALGORITHM {
            return Err(format!("unsupported algorithm: {algo}"));
        }
    }
    let _ = input.device_id.as_deref();
    Ok(())
}

fn decode(value: &str, label: &str) -> Result<Vec<u8>, String> {
    BASE64
        .decode(value)
        .map_err(|error| format!("invalid base64 for {label}: {error}"))
}

fn generate(input: &str) -> Result<GenOutput, String> {
    let common: CommonInput = serde_json::from_str(input)
        .map_err(|error| format!("invalid generation input: {error}"))?;
    validate_common(&common)?;
    let keys = generate_keypair();
    Ok(GenOutput {
        ok: true,
        algo: ALGORITHM,
        public_key: BASE64.encode(keys.public_key),
        private_key: BASE64.encode(keys.secret_key.expose()),
    })
}

fn encapsulate(input: &str) -> Result<EncOutput, String> {
    let request: EncInput = serde_json::from_str(input)
        .map_err(|error| format!("invalid encapsulation input: {error}"))?;
    validate_common(&request.common)?;
    let public_key_bytes = decode(&request.public_key, "publicKey")?;
    let result = kem_encapsulate(&public_key_bytes).map_err(|error| error.to_string())?;
    Ok(EncOutput {
        ok: true,
        algo: ALGORITHM,
        ciphertext_b64: BASE64.encode(result.ciphertext),
        shared_secret_b64: BASE64.encode(result.shared_secret.expose()),
    })
}

fn decapsulate(input: &str) -> Result<DecOutput, String> {
    let request: DecInput = serde_json::from_str(input)
        .map_err(|error| format!("invalid decapsulation input: {error}"))?;
    validate_common(&request.common)?;
    let private_key_bytes = decode(&request.private_key, "privateKey")?;
    let ciphertext_bytes = decode(&request.ciphertext_b64, "ciphertext_b64")?;
    let shared_secret = kem_decapsulate(&ciphertext_bytes, &private_key_bytes)
        .map_err(|error| error.to_string())?;
    Ok(DecOutput {
        ok: true,
        algo: ALGORITHM,
        shared_secret_b64: BASE64.encode(shared_secret.expose()),
    })
}

fn run() -> Result<String, String> {
    let command = env::args()
        .nth(1)
        .ok_or_else(|| "usage: mlkem-cli <gen|enc|dec>".to_string())?;
    let input = read_stdin()?;
    match command.as_str() {
        "gen" => serde_json::to_string(&generate(&input)?)
            .map_err(|error| format!("unable to encode generation output: {error}")),
        "enc" => serde_json::to_string(&encapsulate(&input)?)
            .map_err(|error| format!("unable to encode encapsulation output: {error}")),
        "dec" => serde_json::to_string(&decapsulate(&input)?)
            .map_err(|error| format!("unable to encode decapsulation output: {error}")),
        _ => Err(format!("unsupported command: {command}")),
    }
}

fn main() {
    match run() {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

use hmac::{Hmac, Mac};
use sha2::Sha256;

type Hmac256 = Hmac<Sha256>;

pub fn verify_signature(secret_key: &str, payload: &str, signature: &str) -> bool {
    let sig = signature.trim_start_matches("sha256="); //trim signature from sha256=

    let mut mac = Hmac256::new_from_slice(secret_key.as_bytes()).expect("HMAC engine failed"); //https://docs.rs/hmac/latest/hmac/
    mac.update(payload.as_bytes()); //the Result of this needs to be equal to sig (signature)

    match hex::decode(sig) {
        Ok(decoded) => mac.verify_slice(&decoded).is_ok(),
        Err(_) => false,
    }
}

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use base64;

pub fn validate_license_key(key: &str, secret_key: &[u8]) -> bool {
    let cipher = Aes256Gcm::new_from_slice(secret_key).unwrap();
    let nonce = Nonce::from_slice(b"unique_nonce12");

    match base64::decode(key) {
        Ok(decoded) => match cipher.decrypt(nonce, decoded.as_ref()) {
            Ok(_) => true,
            Err(_) => false,
        },
        Err(_) => false,
    }
}

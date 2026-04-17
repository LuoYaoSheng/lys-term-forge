use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use sha2::{Sha256, Digest};

/// Derive a 256-bit encryption key from machine-specific data.
/// Uses hostname + username as input to SHA-256 with a fixed application salt.
fn derive_key() -> [u8; 32] {
    let hostname = gethostname::gethostname()
        .to_string_lossy()
        .to_string();
    let username = whoami::username();
    let input = format!("TermForge-v1:{}:{}", hostname, username);

    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().into()
}

/// Encrypt a plaintext string to a base64-encoded string.
/// Format: base64(nonce[12] + ciphertext + tag[16])
pub fn encrypt(plaintext: &str) -> anyhow::Result<String> {
    let key = derive_key();
    let cipher = Aes256Gcm::new_from_slice(&key)?;

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    Ok(B64.encode(&combined))
}

/// Decrypt a base64-encoded encrypted string back to plaintext.
pub fn decrypt(encoded: &str) -> anyhow::Result<String> {
    let key = derive_key();
    let cipher = Aes256Gcm::new_from_slice(&key)?;

    let combined = B64
        .decode(encoded)
        .map_err(|e| anyhow::anyhow!("Base64 decode failed: {}", e))?;

    if combined.len() < 12 + 16 {
        return Err(anyhow::anyhow!("Invalid encrypted data length"));
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow::anyhow!("Decryption failed — data may be corrupted or from another machine"))?;

    String::from_utf8(plaintext)
        .map_err(|e| anyhow::anyhow!("Invalid UTF-8 in decrypted data: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let original = "my_secret_password";
        let encrypted = encrypt(original).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_encrypt_produces_different_ciphertexts() {
        let original = "same_password";
        let enc1 = encrypt(original).unwrap();
        let enc2 = encrypt(original).unwrap();
        assert_ne!(enc1, enc2);
        assert_eq!(original, decrypt(&enc1).unwrap());
        assert_eq!(original, decrypt(&enc2).unwrap());
    }
}

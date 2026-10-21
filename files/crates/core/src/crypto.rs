//! Secrets at rest: AES-256-GCM with the `MASTER_KEY` from `.env`, and token helpers.
//!
//! Ciphertext format: `"v1:" + base64(nonce[12] || ciphertext+tag)`. Changing `MASTER_KEY` makes
//! every stored secret unreadable, so it is generated once (`npm run setup`) and kept.

use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use anyhow::{Context as _, anyhow, bail};
use base64::{Engine as _, engine::general_purpose::STANDARD as B64};
use rand::{Rng as _, distributions::Alphanumeric};
use serde::{Serialize, de::DeserializeOwned};
use sha2::{Digest as _, Sha256};

const PREFIX: &str = "v1:";

#[derive(Clone)]
pub struct Crypto {
    cipher: Aes256Gcm,
}

impl std::fmt::Debug for Crypto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Crypto(..)")
    }
}

impl Crypto {
    /// `master_key` is 32 bytes as 64 hex characters or as base64.
    ///
    /// # Errors
    /// When `master_key` is neither.
    pub fn new(master_key: &str) -> anyhow::Result<Self> {
        let master_key = master_key.trim();
        let key = if master_key.len() == 64 && master_key.chars().all(|c| c.is_ascii_hexdigit()) {
            hex::decode(master_key).context("MASTER_KEY must be hex or base64")?
        } else {
            B64.decode(master_key)
                .context("MASTER_KEY must be hex or base64")?
        };
        if key.len() != 32 {
            bail!("MASTER_KEY must decode to 32 bytes, got {}", key.len());
        }
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
        Ok(Self { cipher })
    }

    /// Encrypts bytes into the `v1:` text format.
    ///
    /// # Errors
    /// Practically never (AES-GCM encryption of in-memory data).
    pub fn encrypt(&self, plaintext: &[u8]) -> anyhow::Result<String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext)
            .map_err(|_| anyhow!("encryption failed"))?;
        let mut blob = nonce.to_vec();
        blob.extend_from_slice(&ciphertext);
        Ok(format!("{PREFIX}{}", B64.encode(blob)))
    }

    /// # Errors
    /// When the text is not produced by [`Crypto::encrypt`] with the same key.
    pub fn decrypt(&self, text: &str) -> anyhow::Result<Vec<u8>> {
        let encoded = text
            .strip_prefix(PREFIX)
            .context("unknown ciphertext format")?;
        let blob = B64.decode(encoded).context("ciphertext is not base64")?;
        if blob.len() < 12 {
            bail!("ciphertext too short");
        }
        let (nonce, ciphertext) = blob.split_at(12);
        self.cipher
            .decrypt(Nonce::from_slice(nonce), ciphertext)
            .map_err(|_| anyhow!("decryption failed (wrong MASTER_KEY?)"))
    }

    /// # Errors
    /// See [`Crypto::encrypt`].
    pub fn encrypt_str(&self, plaintext: &str) -> anyhow::Result<String> {
        self.encrypt(plaintext.as_bytes())
    }

    /// # Errors
    /// See [`Crypto::decrypt`]; also when the plaintext is not UTF-8.
    pub fn decrypt_str(&self, text: &str) -> anyhow::Result<String> {
        String::from_utf8(self.decrypt(text)?).context("decrypted value is not UTF-8")
    }

    /// Serializes `value` to JSON and encrypts it (structured secrets).
    ///
    /// # Errors
    /// When serialization fails.
    pub fn encrypt_json<T: Serialize>(&self, value: &T) -> anyhow::Result<String> {
        self.encrypt(&serde_json::to_vec(value)?)
    }

    /// # Errors
    /// When decryption or deserialization fails.
    pub fn decrypt_json<T: DeserializeOwned>(&self, text: &str) -> anyhow::Result<T> {
        Ok(serde_json::from_slice(&self.decrypt(text)?)?)
    }
}

/// A fresh master key (32 random bytes as hex), as written by `npm run setup`.
#[must_use]
pub fn generate_master_key() -> String {
    let mut key = [0u8; 32];
    rand::thread_rng().fill(&mut key);
    hex::encode(key)
}

/// A random token `"<prefix><43 alphanumeric chars>"` (~256 bits), e.g. a session token or API key.
#[must_use]
pub fn random_token(prefix: &str) -> String {
    let body: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(43)
        .map(char::from)
        .collect();
    format!("{prefix}{body}")
}

/// SHA-256 hex of a token. Tokens and API keys are stored only as this hash.
#[must_use]
pub fn hash_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let crypto = Crypto::new(&generate_master_key()).unwrap();
        let sealed = crypto.encrypt_str("sk-secret").unwrap();
        assert!(sealed.starts_with("v1:"));
        assert_ne!(
            sealed,
            crypto.encrypt_str("sk-secret").unwrap(),
            "nonces differ"
        );
        assert_eq!(crypto.decrypt_str(&sealed).unwrap(), "sk-secret");
    }

    #[test]
    fn wrong_key_fails() {
        let a = Crypto::new(&generate_master_key()).unwrap();
        let b = Crypto::new(&generate_master_key()).unwrap();
        assert!(b.decrypt(&a.encrypt_str("x").unwrap()).is_err());
    }

    #[test]
    fn accepts_hex_and_base64_keys() {
        let bytes = [7u8; 32];
        assert!(Crypto::new(&hex::encode(bytes)).is_ok());
        assert!(Crypto::new(&B64.encode(bytes)).is_ok());
        assert!(Crypto::new("short").is_err());
    }

    #[test]
    fn tokens() {
        let t = random_token("sk-");
        assert!(t.starts_with("sk-"));
        assert_eq!(t.len(), 3 + 43);
        assert_eq!(hash_token("a").len(), 64);
    }
}

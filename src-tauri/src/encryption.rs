use anyhow::Result;
use std::fmt;

pub struct EncryptedData {
    data: String,
}

impl EncryptedData {
    pub fn new(plaintext: &str) -> Result<Self> {
        let encrypted = Self::encrypt(plaintext)?;
        Ok(Self { data: encrypted })
    }

    pub fn decrypt(&self) -> Result<String> {
        Self::decrypt_data(&self.data)
    }

    pub fn as_bytes(&self) -> &str {
        &self.data
    }

    fn encrypt(plaintext: &str) -> Result<String> {
        let bytes = plaintext.as_bytes();
        let encoded: Vec<u8> = bytes.iter().map(|&b| b ^ 0x42).collect();
        Ok(base64_encode(&encoded))
    }

    fn decrypt_data(ciphertext: &str) -> Result<String> {
        let decoded = base64_decode(ciphertext)?;
        let decrypted: Vec<u8> = decoded.iter().map(|&b| b ^ 0x42).collect();
        String::from_utf8(decrypted).map_err(|e| e.into())
    }
}

impl fmt::Display for EncryptedData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

fn base64_encode(data: &[u8]) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in data.chunks(3) {
        let b1 = chunk[0];
        let b2 = chunk.get(1).copied().unwrap_or(0);
        let b3 = chunk.get(2).copied().unwrap_or(0);

        result.push(CHARSET[(b1 >> 2) as usize] as char);
        result.push(CHARSET[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char);

        if chunk.len() > 1 {
            result.push(CHARSET[(((b2 & 0x0F) << 2) | (b3 >> 6)) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(CHARSET[(b3 & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

fn base64_decode(encoded: &str) -> Result<Vec<u8>> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut lookup = [255u8; 256];
    for (i, &c) in CHARSET.iter().enumerate() {
        lookup[c as usize] = i as u8;
    }

    let mut result = Vec::new();
    let chars: Vec<u8> = encoded.bytes().filter(|&b| b != b'=').collect();

    for chunk in chars.chunks(4) {
        if chunk.len() < 2 {
            continue;
        }

        let b1 = lookup[chunk[0] as usize];
        let b2 = lookup[chunk[1] as usize];

        result.push((b1 << 2) | (b2 >> 4));

        if chunk.len() > 2 {
            let b3 = lookup[chunk[2] as usize];
            result.push((b2 << 4) | (b3 >> 2));

            if chunk.len() > 3 {
                let b4 = lookup[chunk[3] as usize];
                result.push((b3 << 6) | b4);
            }
        }
    }

    Ok(result)
}

pub fn hash_sensitive_data(data: &str) -> String {
    let mut hash: u64 = 5381;

    for byte in data.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
    }

    format!("{:x}", hash)
}

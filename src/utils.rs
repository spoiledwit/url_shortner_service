use sha2::{Sha256, Digest};

pub fn generate_short_url(original_url: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(original_url.as_bytes());
    hasher.update(salt.as_bytes());
    let result = hasher.finalize(); 
    format!("{:x}", result)[..7].to_string()
}
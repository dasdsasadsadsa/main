use sha2::{Sha256, Digest};

pub fn compute_sha256(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn compute_sha256_string(content: &str) -> String {
    compute_sha256(content.as_bytes())
}

use uuid::Uuid;
use chrono::Utc;
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{Hash, Hasher, BuildHasher};

/// Generate a new UUID v4
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Hash a password using bcrypt-like simple hashing (for demo purposes)
pub fn hash_password(password: &str) -> String {
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    format!("hashed_{:x}", hasher.finish())
}

/// Calculate MD5 hash of string (for demo purposes)
pub fn md5_hash(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Generate random integer between min and max (inclusive)
pub fn random_int(min: i64, max: i64) -> i64 {
    let random_state = RandomState::new();
    let hash = random_state.hash_one(Utc::now().timestamp_nanos_opt().unwrap_or(0));
    
    min + ((hash % ((max - min + 1) as u64)) as i64)
}

/// Generate random float between min and max
pub fn random_float(min: f64, max: f64) -> f64 {
    let rand = random_float_01();
    min + (rand * (max - min))
}

/// Generate random float between 0.0 and 1.0
fn random_float_01() -> f64 {
    let random_state = RandomState::new();
    let hash = random_state.hash_one(Utc::now().timestamp_nanos_opt().unwrap_or(0));
    
    (hash as f64) / (u64::MAX as f64)
}

/// Generate a random string of given length
pub fn random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    
    (0..length)
        .map(|_| {
            let idx = random_int(0, (CHARSET.len() - 1) as i64) as usize;
            CHARSET[idx] as char
        })
        .collect()
}

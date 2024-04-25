use std::hash::{DefaultHasher, Hash, Hasher};

pub fn hash_password(password: &str) -> String {
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    hasher.finish().to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}


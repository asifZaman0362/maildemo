use pbkdf2::{pbkdf2, password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng}, Pbkdf2};

// do we want it as a string?
// I suppose we do
pub fn compute_hash(password: &str) -> Option<String> {
    let salt = SaltString::generate(&mut OsRng);
    if let Ok(hash) = Pbkdf2.hash_password(password.as_bytes(), &salt) {
        Some(hash.to_string())
    } else {
        None
    }
}

pub fn verify_hash(password: &str, hash: &PasswordHash) -> bool {
    Pbkdf2.verify_password(password.as_bytes(), hash).is_ok() // Pbkdf2 is a global static!?
}

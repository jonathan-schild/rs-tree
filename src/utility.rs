use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};

pub fn _hash_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Pbkdf2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn _verify_password(password: &String, hash: String) -> bool {
    match PasswordHash::new(&hash) {
        Ok(parsed_hash) => Pbkdf2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}

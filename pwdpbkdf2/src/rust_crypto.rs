use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Params, Pbkdf2,
};

pub fn hash_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Pbkdf2
        .hash_password_customized(
            password.as_bytes(),
            Some(Algorithm::Pbkdf2Sha256.into()),
            None,
            Params {
                rounds: 600000,
                output_length: 32,
            },
            &salt,
        )
        .unwrap()
        .to_string()
}

pub fn verify_password(password: &String, hash: String) -> bool {
    match PasswordHash::new(&hash) {
        Ok(parsed_hash) => Pbkdf2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}

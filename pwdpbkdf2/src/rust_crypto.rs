use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Params, Pbkdf2,
};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Pbkdf2
        .hash_password_customized(
            password.as_bytes(),
            Some(Algorithm::Pbkdf2Sha256.into()),
            None,
            Params {
                rounds: 600_000,
                output_length: 32,
            },
            &salt,
        )
        .unwrap()
        .to_string()
}

#[must_use]
pub fn verify_password(password: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => Pbkdf2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{hash_password, verify_password};

    #[test]
    fn test() {
        let password1 = "Passw0rt!23";
        let hash1 = hash_password(&password1.to_owned());

        let password2 = "Passw0rt!23X";

        assert!(verify_password(&password1.to_owned(), &hash1));
        assert!(!verify_password(&password2.to_owned(), &hash1));
    }
}

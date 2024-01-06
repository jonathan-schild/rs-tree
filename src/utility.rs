/*
 * Copyright (c) 2024 Jonathan "Nath" Schild - MIT License
 */

use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Params, Pbkdf2,
};
use rand_core::{OsRng, RngCore};
use std::vec::from_elem;

#[must_use]
pub fn hash_password(password: &str) -> Option<String> {
    let salt = SaltString::generate(&mut OsRng);

    //Pbkdf2
    //    .hash_password(password.as_bytes(), &salt)
    //    .unwrap()
    //    .to_string()

    if let Ok(hash) = Pbkdf2.hash_password_customized(
        password.as_bytes(),
        Some(Algorithm::Pbkdf2Sha256.into()),
        None,
        Params {
            rounds: 600_000,
            output_length: 32,
        },
        &salt,
    ) {
        Some(hash.to_string())
    } else {
        None
    }
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

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn build_info() -> String {
    let name = format!(
        "{}: Copyright (c) 2024 Jonathan \"Nath\" Schild - MIT License",
        env!("CARGO_PKG_NAME")
    );

    let version = format!(
        "Version: {}+{}.{} at {}",
        env!("CARGO_PKG_VERSION"),
        env!("BUILD_HASH"),
        env!("BUILD_EPOCH"),
        env!("BUILD_DATE")
    );

    let rustc = format!("Rust Version: {}", env!("BUILD_RUSTC"));

    let repo = format!("Source Code: {}", env!("CARGO_PKG_REPOSITORY"));

    let max = *[name.len(), version.len(), rustc.len(), repo.len()]
        .iter()
        .max()
        .unwrap(); // returns None if iter is empty, which should never occur

    let w = max + 2;
    format!(
        "\n#{}#\n#{}#\n#  {name:w$}#\n#  {version:w$}#\n#  {rustc:w$}#\n#  {repo:w$}#\n#{1}#\n#{0}#",
        "=".repeat(w + 2),
        " ".repeat(w + 2)
    )
}

const SHORT_LINK_CHARACTER: [char; 38] = [
    'a', 'b', 'c', 'd', 'e', 'g', 'h', 'k', 'o', 'p', 'r', 's', 't', 'w', 'x', 'z', 'A', 'C', 'D',
    'E', 'G', 'H', 'L', 'P', 'Q', 'R', 'S', 'T', 'U', 'W', 'X', 'Y', 'Z', '3', '4', '5', '6', '7',
];

#[must_use]
pub fn generate_short() -> String {
    let n = 7;
    let mut rng = from_elem(0, n);

    OsRng.fill_bytes(&mut rng);

    #[allow(clippy::cast_possible_truncation)]
    let mut s = rng
        .iter()
        .map(|r| SHORT_LINK_CHARACTER[(r % SHORT_LINK_CHARACTER.len() as u8) as usize])
        .fold(String::new(), |mut s, c| {
            s.push(c);
            s
        });
    s.insert(3, '-');
    s
}

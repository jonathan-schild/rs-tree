// #![warn(clippy::cargo)]
#![warn(clippy::pedantic)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![warn(missing_docs)]

pub mod ring;
pub mod rust_crypto;

#[must_use]
pub fn hash_password(password: &String) -> String {
    ring::hash_password(password)
}

#[must_use]
pub fn verify_password(password: &String, hash: &String) -> bool {
    ring::verify_password(password, hash)
}

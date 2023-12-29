use std::{num::NonZeroU32, str::from_utf8};

use ring::{
    pbkdf2::derive,
    pbkdf2::verify,
    pbkdf2::PBKDF2_HMAC_SHA256,
    rand::{SecureRandom, SystemRandom},
};

pub fn hash_password(password: &String) -> String {
    let mut salt = [0u8; 16];
    let mut out = [0u8; 32];

    SystemRandom::new().fill(&mut salt).unwrap();

    derive(
        PBKDF2_HMAC_SHA256,
        NonZeroU32::new(600000).unwrap(),
        &salt,
        password.as_bytes(),
        &mut out,
    );

    from_utf8(&out).unwrap().to_owned()
}

pub fn verify_password(password: &String, hash: String) -> bool {
    let mut salt = [0u8; 16];
    SystemRandom::new().fill(&mut salt).unwrap();

    verify(
        PBKDF2_HMAC_SHA256,
        NonZeroU32::new(600000).unwrap(),
        &salt,
        password.as_bytes(),
        hash.as_bytes(),
    )
    .is_ok()
}

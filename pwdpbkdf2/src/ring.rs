use std::num::NonZeroU32;

use base64::{engine::general_purpose, Engine};
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

    let mut ret = "pwdpbkdf$".to_owned();
    general_purpose::STANDARD.encode_string(&salt, &mut ret);
    ret.push('$');
    general_purpose::STANDARD.encode_string(&out, &mut ret);

    ret
}

pub fn verify_password(password: &String, hash: &String) -> bool {
    let spl: Vec<&str> = hash.split('$').collect();

    if spl.len() == 3 && spl[0] == "pwdpbkdf" {
        let salt = general_purpose::STANDARD.decode(spl[1]).unwrap_or_default();
        let hash = general_purpose::STANDARD.decode(spl[2]).unwrap_or_default();

        verify(
            PBKDF2_HMAC_SHA256,
            NonZeroU32::new(600000).unwrap(),
            &salt,
            password.as_bytes(),
            &hash,
        )
        .is_ok()
    } else {
        false
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

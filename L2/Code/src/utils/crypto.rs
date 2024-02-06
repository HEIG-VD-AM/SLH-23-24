use argon2::{password_hash::{
    rand_core::OsRng,
    PasswordHash, PasswordHasher, PasswordVerifier, SaltString
}, Argon2, Algorithm, Version, Params};
use lazy_static::lazy_static;

pub fn hash_password(password: &str) -> Result<String,  argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(64 * 1024, 3, 1, Some(32)).unwrap());
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::new(64 * 1024, 3, 1, Some(32)).unwrap());
    let password_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    argon2.verify_password(password.as_bytes(), &password_hash).is_ok()
}

pub fn default_hash() -> String {
    // This function create a default hash we could use when the user doesn't exist
    lazy_static!(
        static ref DEFAULT_HASH: String = hash_password("").unwrap();
    );
    DEFAULT_HASH.to_string()
}

#[cfg(test)]
mod crypto_tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
    password,
    case(""),
    case("ThisIsASecretPassword"),
    )]
    fn test_hash_and_verify_password(password : String) {
        let hashed_password = hash_password(&password).unwrap();
        assert!(verify_password(&password, &hashed_password));
    }


    #[rstest(
    password,
    case(""),
    case("ThisIsASecretPassword"),
    )]
    fn test_hash_and_verify_wrong_password(password : String) {
        let hashed_password = hash_password(&password).unwrap();
        let wrong_password : &str = "ThisIsNotTheGoodPassword";
        assert!(!verify_password(wrong_password, &hashed_password));
    }
}
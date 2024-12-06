use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};

pub fn password_encrypt(
    password: String,
) -> Result<std::string::String, scrypt::password_hash::Error> {
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let hash: Result<PasswordHash<'_>, scrypt::password_hash::Error> =
        Scrypt.hash_password(password, &salt);

    println!("{:?}", hash);

    match hash {
        Ok(value) => {
            return Ok(value.to_string());
        }
        Err(error) => {
            return Err(error);
        }
    }
}

pub fn password_verify(password: String, hash: String) -> Result<bool, scrypt::password_hash::Error> {
    let parsed_hash;

    match PasswordHash::new(&hash) {
        Ok(value) => {
            parsed_hash = value;
        }

        Err(error) => {
            return Err(error);
        }
    }

    return Ok(Scrypt
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

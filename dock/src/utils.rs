pub fn password_encrypt(password: String) -> String {
    let hash = sha256::digest(password);

    hash
}

pub fn password_verify(password: String, hash: String) -> bool {
    let v_hash = sha256::digest(password);

    v_hash.eq(&hash)
}

#[cfg(test)]
mod tests {
    use crate::utils::{password_encrypt, password_verify};

    #[test]
    fn test_password_encrypte() {
        let password = String::from("rust_in_use");

        let hash = password_encrypt(password.clone());

        // result hash is from third party.
        assert_eq!(
            hash,
            String::from("0bf42167cdf8aad9de92815801f93750fc947819935d3c13fc4877f447a876cd")
        );

        assert!(password_verify(password, hash));
    }
}

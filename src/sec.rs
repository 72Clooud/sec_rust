use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

pub fn hash_password(password: &str) -> BcryptResult<String> {
    hash(password, DEFAULT_COST)
}

pub fn check_password(given_password: &str, saved_password: &str) -> BcryptResult<bool> {
    verify(given_password, saved_password)    
}

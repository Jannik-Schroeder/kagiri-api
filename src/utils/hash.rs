use bcrypt::{DEFAULT_COST, hash, verify};
use crate::utils::db::get_hashed_password;

pub fn hash_password (pwd: String, email: &String) -> String {
    let pepper = &email[0..5];
    let peppered_password = format!("{:?}{}", pwd, pepper);
    let hashed_password = hash(peppered_password, DEFAULT_COST).unwrap();
    return hashed_password;
}

pub fn verify_password (pwd: String, email: &String) -> bool {
    let pepper = &email[0..5];
    let hashed_password = get_hashed_password(email);
    let peppered_password = format!("{:?}{}", pwd, pepper);
    let is_valid = verify(peppered_password, &*hashed_password).unwrap();
    return is_valid;
}
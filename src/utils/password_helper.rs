use std::ptr::hash;
use sha2::{Digest,Sha256};
use sha2::digest::Update;
/*pub fn hash_password(password: &String) -> &String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hashed_password = hasher.finalize();
    hex::encode(hashed_password)
}*/


pub fn hash_password(password: &String) -> String {
    let mut hasher = Sha256::new();
    Update::update(&mut hasher, password.as_bytes());
    let hashed_password = hasher.finalize();
    hex::encode(hashed_password)
}
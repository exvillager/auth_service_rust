use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

pub fn hash_password(string: String) -> Result<String, BcryptError> {
    hash(string, DEFAULT_COST)
}

pub fn verify_pass(password: String, hashed_pass: String) -> Result<bool, BcryptError> {
    verify(password, &hashed_pass)
}

#[derive(Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
const SECRET: &[u8] = b"xpradx";

pub fn generate_access_token(user_id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )?;
    
    Ok(token)
}

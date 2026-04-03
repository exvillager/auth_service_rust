use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::envs::{get_env};

pub fn hash_password(string: String) -> Result<String, BcryptError> {
    hash(string, DEFAULT_COST)
}

pub fn verify_pass(password: String, hashed_pass: String) -> Result<bool, BcryptError> {
    verify(password, &hashed_pass)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn generate_access_and_refresh_token(
    user_id: &Uuid,
) -> Result<Token, jsonwebtoken::errors::Error> {
    let access_exp = Utc::now()
        .checked_add_signed(Duration::minutes(60))
        .unwrap()
        .timestamp() as usize;

    let access_claims = Claims {
        sub: user_id.clone(),
        exp: access_exp,
    };

    let access_token = encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(get_env().access_token_secret.as_bytes()),
    )?;

    let refresh_exp = Utc::now()
        .checked_add_signed(Duration::days(7))
        .unwrap()
        .timestamp() as usize;

    let refresh_claims = Claims {
        sub: user_id.clone(),
        exp: refresh_exp,
    };

    let refresh_token = encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(get_env().refresh_token_secret.as_bytes()),
    )?;

    Ok(Token {
        access_token,
        refresh_token,
    })
}

pub fn decode_token(token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

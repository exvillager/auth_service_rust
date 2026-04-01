use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn hash_password(string: String) -> Result<String, BcryptError> {
    hash(string, DEFAULT_COST)
}

pub fn verify_pass(password: String, hashed_pass: String) -> Result<bool, BcryptError> {
    verify(password, &hashed_pass)
}

#[derive(Serialize,Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}
const SECRET: &[u8] = b"xpradx";
const REFRESH_TOKEN_SECRET: &[u8] = b"refreshbro";

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
        &EncodingKey::from_secret(SECRET),
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
        &EncodingKey::from_secret(REFRESH_TOKEN_SECRET),
    )?;

    Ok(Token {
        access_token,
        refresh_token,
    })
}

pub async fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode(
        token,
        &DecodingKey::from_secret(REFRESH_TOKEN_SECRET),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

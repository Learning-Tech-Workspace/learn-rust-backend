use std::env;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};

pub fn encode_jwt(user_id: Uuid) -> Result<String> {
    let claims = Claims {
        sub: String::from("Login Token"),
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
        user_id,
    };

    let secret =
        env::var("JWT_SECRET").map_err(|_| Error::EnvVarNotFound("JWT_SECRET".to_string()))?;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();

    Ok(token)
}

pub fn decode_jwt(token: String) -> Result<Uuid> {
    let secret =
        env::var("JWT_SECRET").map_err(|_| Error::EnvVarNotFound("JWT_SECRET".to_string()))?;

    let Claims {
        user_id,
        sub: _,
        exp: _,
    } = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| Error::DecodeJwtFailed(e.to_string()))?
    .claims;

    Ok(user_id)
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // the subject of the token
    pub exp: usize,  // the expiry time
    pub user_id: Uuid,
}

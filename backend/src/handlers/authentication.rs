use crate::db::account::*;
use anyhow::Result;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    provider: u32,
    exp: i64,
}

impl Claims {
    fn with_id(id: &str, provider: u32) -> Self {
        Claims {
            id: id.into(),
            provider,
            exp: (Local::now() + Duration::hours(1)).timestamp(),
        }
    }
}

pub async fn create_token(account_info: &AccountInfo, provider: u32) -> Result<String> {
    let id = account_info.id.to_string();
    let claims = Claims::with_id(id.as_str(), provider);
    let encode_data = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret()),
    )
    .unwrap();
    Ok(encode_data.to_owned())
}

pub async fn decode_token(token: &str) -> Result<String> {
    let decode_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret()),
        &Validation::default(),
    )
    .unwrap()
    .claims;
    Ok(decode_data.id)
}

fn get_secret<'a>() -> &'a [u8] {
    dotenv!("JWT_SECRET").as_bytes()
}

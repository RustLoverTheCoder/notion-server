use chrono::{Duration, Utc};
use config::contants::JWT_SECRET;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) exp: i64,
    // 必填（验证中的defaultate_exp默认为true）。到期时间（以UTC时间戳记）
    iat: i64,
    // 可选 签发时间（以UTC时间戳记）
    iss: String,
    // 可选 签发人
    nbf: i64,
    // 可选 生效时间（以UTC时间戳记）
    pub(crate) sub: String, // 可选 用户
}

pub fn decode(token: &str) -> anyhow::Result<TokenData<Claims>> {
    tracing::debug!("token: {:?}", token);
    let secret = JWT_SECRET.get().unwrap();
    let decodeKey = &DecodingKey::from_secret(secret.as_bytes());
    let jwt = jsonwebtoken::decode(
        token,
        decodeKey,
        &Validation::default(),
    )?;

    Ok(jwt)
}

pub fn encode(user_id: String) -> anyhow::Result<(String, String, Duration)> {
    let secret = JWT_SECRET.get().unwrap();
    let secret = &EncodingKey::from_secret(secret.as_bytes());
    let iss = "server".to_string();
    let expires = Duration::days(30);
    let sub = user_id.to_string();
    let header = Header::default();
    let now = Utc::now();
    let exp = now + expires;
    let claims = Claims {
        exp: exp.timestamp(),
        iat: now.timestamp(),
        nbf: now.timestamp(),
        iss,
        sub,
    };
    let access_token = jsonwebtoken::encode(&header, &claims, secret)?;

    let expires = Duration::days(90);
    let exp = now + expires;
    let claims = Claims {
        exp: exp.timestamp(),
        ..claims
    };
    let refresh_token = jsonwebtoken::encode(&header, &claims, secret)?;
    Ok((access_token, refresh_token, expires))
}

use chrono::{Duration, Utc};
use dotenv::dotenv;
use std::env;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm, TokenData};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    iat: i64,
    exp: i64,
}

pub fn create_token(email: &str) -> String {
    dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let now = Utc::now().timestamp();
    let expiration = Duration::hours(24).num_seconds();

    let claims = Claims {
        iss: "kagiri".into(),
        sub: email.into(),
        iat: now,
        exp: now + expiration,
    };

    let header = Header::new(Algorithm::HS256);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())).expect("Failed to encode claims")
}

pub fn validate_token(token: &str) -> TokenData<Claims> {
    dotenv().ok();
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &validation).expect("Failed to validate token")
}
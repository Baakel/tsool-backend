use axum::Json;
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{encode, Algorithm, Header};

use crate::{
    config::CONFIG,
    models::{AuthResponse, Claims, TsoolError},
};

pub mod goals;
pub mod todos;

pub async fn authorize() -> Result<Json<AuthResponse>, TsoolError> {
    let expiration = Utc::now() + TimeDelta::hours(1);
    let claims = Claims {
        aud: "tsool-backend".to_string(),
        sub: "some user?".to_string(),
        company: "tsool@baakel.dev".to_string(),
        exp: expiration.timestamp() as u64,
    };

    let header = Header::new(Algorithm::ES256);
    let token = encode(&header, &claims, &CONFIG.encoding_key)?;

    Ok(Json(AuthResponse { token }))
}

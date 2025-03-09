use std::sync::Arc;

use axum::{extract::State, Json};
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{encode, Algorithm, Header};
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Record;
use tracing::debug;

use crate::{
    config::CONFIG,
    models::{routes::AuthReq, AppAuthReq, AppState, AuthResponse, Claims, TsoolError},
};

pub mod goals;
pub mod todos;

pub async fn authorize(
    State(state): State<Arc<AppState>>,
    Json(req): Json<AuthReq>,
) -> Result<Json<AuthResponse>, TsoolError> {
    let user = state
        .db
        .signin(Record {
            namespace: &CONFIG.namespace,
            database: &CONFIG.database,
            access: &CONFIG.access_method,
            params: AuthParams {
                email: req.user.clone(),
                password: req.pass.clone(),
            },
        })
        .await?;
    debug!(?user, "we got a user");

    Ok(Json(AuthResponse {
        token: user.as_insecure_token().to_string(),
    }))
}

pub async fn authorize_app(
    State(_): State<Arc<AppState>>,
    Json(req): Json<AppAuthReq>,
) -> Result<Json<AuthResponse>, TsoolError> {
    if req.secret != CONFIG.api_key {
        return Err(TsoolError::Unauthorized);
    }
    let expiration = Utc::now() + TimeDelta::days(90);
    let claims = Claims {
        aud: "tsool-backend".to_string(),
        iss: "tsool.xibalba.xyz".to_string(),
        sub: req.app.to_string(),
        company: "tsool@xibalba.xyz".to_string(),
        exp: expiration.timestamp() as u64,
    };

    let header = Header::new(Algorithm::ES256);
    let token = encode(&header, &claims, &CONFIG.encoding_key)?;
    Ok(Json(AuthResponse { token }))
}

pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SignUpParams>,
) -> Result<Json<AuthResponse>, TsoolError> {
    let created_user = state
        .db
        .signup(Record {
            namespace: &CONFIG.namespace,
            database: &CONFIG.database,
            access: &CONFIG.access_method,
            params: SignUpParams {
                name: req.name,
                email: req.email,
                password: req.password,
            },
        })
        .await?;
    Ok(Json(AuthResponse {
        token: created_user.as_insecure_token().to_string(),
    }))
}

#[derive(Debug, Serialize)]
struct AuthParams {
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignUpParams {
    pub name: String,
    pub email: String,
    pub password: String,
}

use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, errors::ErrorKind, Validation};
use surrealdb::{engine::any::Any, Surreal};
use tracing::{debug, info};

use crate::{
    config::CONFIG,
    models::{AppState, Claims, TsoolError},
};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();

    debug!(?parts, "the req parts");
    match parts.uri.path() {
        "/authorize" | "/signup" => {
            let req = Request::from_parts(parts, body);
            return Ok(next.run(req).await);
        }
        _ => (),
    }

    let auth: TypedHeader<Authorization<Bearer>> = match parts.extract().await {
        Ok(r) => r,
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    if let Err(e) = validate_user(&state.db, auth.token()).await {
        debug!(error = ?e, "Error while authenticating");
        return Err(StatusCode::UNAUTHORIZED);
    };
    // validate_token(auth.token())?;

    let req = Request::from_parts(parts, body);

    Ok(next.run(req).await)
}

async fn validate_user(db: &Surreal<Any>, token: &str) -> Result<(), TsoolError> {
    db.authenticate(token).await?;
    Ok(())
}

fn validate_token(token: &str) -> Result<(), StatusCode> {
    let mut validation = Validation::new(jsonwebtoken::Algorithm::ES256);

    // TODO: add a jti to claims to prevent token replay attacks.

    validation.set_audience(&["tsool-backend"]);
    validation.set_issuer(&["tsool.xibalba.xyz"]);
    let token_data = match decode::<Claims>(token, &CONFIG.decoding_key, &validation) {
        Ok(t) => t,
        Err(e) => {
            debug!(error = ?e,"error while decoding token");
            match e.kind() {
                ErrorKind::InvalidIssuer | ErrorKind::InvalidAudience => {
                    return Err(StatusCode::BAD_REQUEST)
                }
                _ => return Err(StatusCode::UNAUTHORIZED),
            }
        }
    };
    debug!("{token_data:?}");
    Ok(())
}

use axum::{
    extract::Request, http::StatusCode, middleware::Next, response::Response, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::config::CONFIG;

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();

    let auth: TypedHeader<Authorization<Bearer>> = match parts.extract().await {
        Ok(r) => r,
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    validate_token(auth.token())?;

    let req = Request::from_parts(parts, body);

    Ok(next.run(req).await)
}

fn validate_token(token: &str) -> Result<(), StatusCode> {
    if token != CONFIG.api_key {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(())
}

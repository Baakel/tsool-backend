use axum::{
    extract::Request, http::StatusCode, middleware::Next, response::Response, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};
use tracing::debug;

use crate::{config::CONFIG, models::Claims};

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();

    debug!(?parts, "the req parts");
    match parts.uri.path() {
        "/authorize" => {
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

    validate_token(auth.token())?;

    let req = Request::from_parts(parts, body);

    Ok(next.run(req).await)
}

fn validate_token(token: &str) -> Result<(), StatusCode> {
    let mut validation = Validation::new(jsonwebtoken::Algorithm::ES256);
    // validation.sub = Some("tsool@baakel.dev".to_string());
    validation.set_audience(&["tsool-backend"]);
    let token_data = match decode::<Claims>(token, &CONFIG.decoding_key, &validation) {
        Ok(t) => t,
        Err(e) => {
            debug!(error = ?e,"error while decoding token");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    print!("{token_data:?}");
    // if token != CONFIG.api_key {
    //     return Err(StatusCode::UNAUTHORIZED);
    // }
    Ok(())
}

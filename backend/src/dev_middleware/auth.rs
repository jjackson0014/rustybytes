use crate::services::auth::verify_jwt;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use tracing::{info, warn};

pub async fn require_auth(req: Request<Body>, next: Next) -> impl IntoResponse {
    let auth_header = req.headers().get("Authorization");

    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                match verify_jwt(token) {
                    Ok(claims) => {
                        info!("JWT verified for user ID: {}", claims.sub);
                        return next.run(req).await;
                    }
                    Err(e) => {
                        warn!("Invalid JWT: {:?}", e);
                    }
                }
            } else {
                warn!("Malformed Authorization header.");
            }
        }
    } else {
        warn!("Missing Authorization header.");
    }

    (StatusCode::UNAUTHORIZED, "Invalid or missing token").into_response()
}

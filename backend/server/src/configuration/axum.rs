use axum::http::{HeaderName, Method};
use tower_http::cors::{AllowHeaders, AllowMethods, CorsLayer};
use crate::configuration::env::FRONTEND_URL;

pub fn get_cors_layer() -> CorsLayer {
    let origins = [FRONTEND_URL.parse().unwrap()];

    CorsLayer::new()
        .allow_methods(AllowMethods::from(vec![
            Method::GET,
            Method::PUT,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ]))
        .allow_headers(AllowHeaders::list(vec![HeaderName::from_static(
            "content-type",
        )]))
        .allow_origin(origins)
        .allow_credentials(true)
}
